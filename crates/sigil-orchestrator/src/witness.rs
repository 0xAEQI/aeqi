use anyhow::Result;
use sigil_beads::{BeadStatus, BeadStore};
use sigil_core::config::ExecutionMode;
use sigil_core::traits::{Provider, Tool};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info, warn};

use crate::executor::ClaudeCodeExecutor;
use crate::mail::{Mail, MailBus};
use crate::rig::Rig;
use crate::worker::{Worker, WorkerState};

/// Max resolution attempts at the rig level before escalating to Familiar.
/// Each attempt spawns a new worker to try to answer the blocker question.
const MAX_RIG_RESOLUTION_ATTEMPTS: u32 = 1;

/// Label prefix for tracking escalation depth on beads.
const ESCALATION_LABEL_PREFIX: &str = "escalation:";

/// Witness: per-rig supervisor. Runs patrol cycles, manages workers,
/// detects stuck/orphaned beads, handles escalation, reports to Familiar.
pub struct Witness {
    pub rig_name: String,
    pub workers: Vec<Worker>,
    pub max_workers: u32,
    pub patrol_interval_secs: u64,
    pub mail_bus: Arc<MailBus>,
    pub beads: Arc<Mutex<BeadStore>>,
    /// Execution mode for this rig's workers.
    pub execution_mode: ExecutionMode,
    // Agent-mode fields (used when execution_mode == Agent).
    pub provider: Arc<dyn Provider>,
    pub tools: Vec<Arc<dyn Tool>>,
    pub model: String,
    pub identity: sigil_core::Identity,
    // ClaudeCode-mode fields (used when execution_mode == ClaudeCode).
    /// Rig's repo path for Claude Code working directory.
    pub repo: Option<std::path::PathBuf>,
    /// Max turns per Claude Code execution.
    pub cc_max_turns: u32,
    /// Max budget per Claude Code execution.
    pub cc_max_budget_usd: Option<f64>,
}

impl Witness {
    pub fn new(rig: &Rig, provider: Arc<dyn Provider>, tools: Vec<Arc<dyn Tool>>, mail_bus: Arc<MailBus>) -> Self {
        Self {
            rig_name: rig.name.clone(),
            workers: Vec::new(),
            max_workers: rig.max_workers,
            patrol_interval_secs: 60,
            mail_bus,
            beads: rig.beads.clone(),
            execution_mode: ExecutionMode::Agent,
            provider,
            tools,
            model: rig.model.clone(),
            identity: rig.identity.clone(),
            repo: None,
            cc_max_turns: 25,
            cc_max_budget_usd: None,
        }
    }

    /// Set execution mode to Claude Code with rig-specific settings.
    pub fn set_claude_code_mode(
        &mut self,
        repo: std::path::PathBuf,
        model: String,
        max_turns: u32,
        max_budget_usd: Option<f64>,
    ) {
        self.execution_mode = ExecutionMode::ClaudeCode;
        self.repo = Some(repo);
        self.model = model;
        self.cc_max_turns = max_turns;
        self.cc_max_budget_usd = max_budget_usd;
    }

    /// Create a worker based on the rig's execution mode.
    fn create_worker(&self, worker_name: String) -> Worker {
        match self.execution_mode {
            ExecutionMode::Agent => Worker::new(
                worker_name,
                self.rig_name.clone(),
                self.provider.clone(),
                self.tools.clone(),
                self.identity.clone(),
                self.model.clone(),
                self.mail_bus.clone(),
                self.beads.clone(),
            ),
            ExecutionMode::ClaudeCode => {
                let workdir = self.repo.clone().unwrap_or_default();
                let executor = ClaudeCodeExecutor::new(
                    workdir,
                    self.model.clone(),
                    self.cc_max_turns,
                    self.cc_max_budget_usd,
                );
                Worker::new_claude_code(
                    worker_name,
                    self.rig_name.clone(),
                    executor,
                    self.identity.clone(),
                    self.mail_bus.clone(),
                    self.beads.clone(),
                )
            }
        }
    }

    /// Run one patrol cycle: check workers, assign ready work, handle blocked
    /// beads, report status.
    pub async fn patrol(&mut self) -> Result<()> {
        debug!(rig = %self.rig_name, "patrol cycle");

        // 1. Clean up done/failed workers.
        self.workers.retain(|w| {
            !matches!(w.state, WorkerState::Done | WorkerState::Failed(_))
        });

        // 2. Handle blocked beads — attempt resolution or escalate.
        self.handle_blocked_beads().await;

        // 3. Check for ready beads and assign to idle workers.
        let ready_beads = {
            let store = self.beads.lock().await;
            store.ready().into_iter().cloned().collect::<Vec<_>>()
        };

        for bead in ready_beads {
            if self.workers.len() as u32 >= self.max_workers {
                break;
            }

            if bead.assignee.is_some() {
                continue;
            }

            let worker_name = format!("{}-worker-{}", self.rig_name, self.workers.len() + 1);
            info!(
                rig = %self.rig_name,
                worker = %worker_name,
                bead = %bead.id,
                subject = %bead.subject,
                mode = ?self.execution_mode,
                "assigning work"
            );

            let mut worker = self.create_worker(worker_name);
            worker.assign(&bead);
            self.workers.push(worker);
        }

        // 4. Log worker states.
        for worker in &self.workers {
            debug!(
                rig = %self.rig_name,
                worker = %worker.name,
                state = ?worker.state,
                "worker status"
            );
        }

        // 5. Report to Familiar.
        let active = self.workers.iter().filter(|w| w.state == WorkerState::Working).count();
        let pending = {
            let store = self.beads.lock().await;
            store.ready().len()
        };

        if active > 0 || pending > 0 {
            self.mail_bus
                .send(Mail::new(
                    &format!("witness-{}", self.rig_name),
                    "familiar",
                    "PATROL",
                    &format!(
                        "Rig {}: {} active workers, {} pending tasks",
                        self.rig_name, active, pending
                    ),
                ))
                .await;
        }

        Ok(())
    }

    /// Handle blocked beads: attempt rig-level resolution or escalate to Familiar.
    ///
    /// Escalation chain:
    ///   1. Worker BLOCKED → Witness spawns resolver worker (same rig, has full codebase access)
    ///   2. Resolver answers → Witness appends answer to bead, resets to Pending for re-attempt
    ///   3. Resolver also blocked → Witness escalates to Familiar via mail
    ///   4. Familiar tries (has KNOWLEDGE.md + cross-rig context)
    ///   5. Familiar resolves → sends RESOLVED mail back → Witness re-opens bead
    ///   6. Familiar stuck → routes to human via Telegram
    async fn handle_blocked_beads(&mut self) {
        let blocked_beads = {
            let store = self.beads.lock().await;
            store.all().into_iter()
                .filter(|b| b.status == BeadStatus::Blocked)
                .cloned()
                .collect::<Vec<_>>()
        };

        for bead in blocked_beads {
            let escalation_depth = Self::get_escalation_depth(&bead.labels);

            if escalation_depth >= MAX_RIG_RESOLUTION_ATTEMPTS {
                // Already tried rig-level resolution. Escalate to Familiar.
                self.escalate_to_familiar(&bead).await;
            } else {
                // Attempt rig-level resolution: re-open as Pending with resolution context.
                self.attempt_rig_resolution(&bead, escalation_depth).await;
            }
        }
    }

    /// Attempt to resolve a blocker at the rig level.
    ///
    /// Increments escalation depth, appends the blocker question to the bead
    /// description as resolution context, and resets to Pending so a new worker
    /// picks it up with the full context.
    async fn attempt_rig_resolution(
        &self,
        bead: &sigil_beads::Bead,
        current_depth: u32,
    ) {
        let new_depth = current_depth + 1;
        let new_label = format!("{ESCALATION_LABEL_PREFIX}{new_depth}");

        // Extract the blocker question from closed_reason or description.
        let blocker_context = bead.closed_reason.as_deref()
            .unwrap_or("(no blocker details captured)");

        info!(
            rig = %self.rig_name,
            bead = %bead.id,
            depth = new_depth,
            "attempting rig-level resolution"
        );

        // Update bead: append resolution context, increment depth, reset to Pending.
        let mut store = self.beads.lock().await;
        let _ = store.update(&bead.id.0, |b| {
            // Append blocker context to description so the next worker sees it.
            b.description.push_str(&format!(
                "\n\n---\n## Resolution Attempt {new_depth}\n\n\
                 A previous worker was blocked on this task. \
                 Before continuing the original task, first try to answer this question \
                 using the codebase, documentation, and your knowledge. \
                 If you can answer it, proceed with the original task using that answer. \
                 If you genuinely cannot determine the answer, respond with BLOCKED: again.\n\n\
                 **Blocker question:**\n{blocker_context}\n"
            ));

            // Track escalation depth.
            b.labels.retain(|l| !l.starts_with(ESCALATION_LABEL_PREFIX));
            b.labels.push(new_label);

            // Reset to Pending so patrol picks it up for a new worker.
            b.status = BeadStatus::Pending;
            b.assignee = None;
        });
    }

    /// Escalate a blocked bead to the Familiar for cross-rig resolution.
    ///
    /// The Familiar has KNOWLEDGE.md with operational learnings and cross-rig
    /// awareness. If it can't resolve either, it routes to human via Telegram.
    async fn escalate_to_familiar(&self, bead: &sigil_beads::Bead) {
        // Only escalate once — check if we already sent an ESCALATE mail for this bead.
        if bead.labels.iter().any(|l| l == "escalated_to_familiar") {
            return;
        }

        info!(
            rig = %self.rig_name,
            bead = %bead.id,
            "escalating to familiar — rig-level resolution exhausted"
        );

        // Mark bead as escalated.
        {
            let mut store = self.beads.lock().await;
            let _ = store.update(&bead.id.0, |b| {
                b.labels.push("escalated_to_familiar".to_string());
            });
        }

        // Send escalation mail to Familiar with full context.
        self.mail_bus
            .send(Mail::new(
                &format!("witness-{}", self.rig_name),
                "familiar",
                "ESCALATE",
                &format!(
                    "Rig {} needs help resolving a blocker.\n\n\
                     Bead: {} — {}\n\
                     Priority: {}\n\n\
                     Full description:\n{}\n\n\
                     This bead has been blocked after {} resolution attempt(s) at the rig level. \
                     Please try to resolve using your cross-rig knowledge (KNOWLEDGE.md). \
                     If you can answer the blocker question, send a RESOLVED mail back to \
                     witness-{} with the answer. If you cannot resolve it, escalate to the \
                     human operator via Telegram.",
                    self.rig_name,
                    bead.id,
                    bead.subject,
                    bead.priority,
                    bead.description,
                    Self::get_escalation_depth(&bead.labels),
                    self.rig_name,
                ),
            ))
            .await;
    }

    /// Process a RESOLVED mail from the Familiar: re-open the blocked bead
    /// with the answer appended to the description.
    pub async fn handle_resolution(&self, bead_id: &str, answer: &str) {
        info!(
            rig = %self.rig_name,
            bead = %bead_id,
            "received resolution from familiar"
        );

        let mut store = self.beads.lock().await;
        let _ = store.update(bead_id, |b| {
            b.description.push_str(&format!(
                "\n\n---\n## Resolution (from Familiar)\n\n{answer}\n\n\
                 **Now proceed with the original task using this answer.**\n"
            ));
            b.status = BeadStatus::Pending;
            b.assignee = None;
            // Remove escalation labels — fresh start with the answer.
            b.labels.retain(|l| {
                !l.starts_with(ESCALATION_LABEL_PREFIX) && l != "escalated_to_familiar"
            });
        });
    }

    /// Get escalation depth from bead labels.
    fn get_escalation_depth(labels: &[String]) -> u32 {
        labels.iter()
            .filter_map(|l| l.strip_prefix(ESCALATION_LABEL_PREFIX))
            .filter_map(|n| n.parse::<u32>().ok())
            .max()
            .unwrap_or(0)
    }

    /// Execute all hooked workers. Returns the number of workers that ran.
    pub async fn execute_workers(&mut self) -> usize {
        let mut executed = 0;
        for worker in &mut self.workers {
            if worker.state == WorkerState::Hooked {
                match worker.execute().await {
                    Ok(outcome) => {
                        debug!(
                            rig = %self.rig_name,
                            worker = %worker.name,
                            outcome = ?std::mem::discriminant(&outcome),
                            "worker finished"
                        );
                    }
                    Err(e) => {
                        warn!(
                            rig = %self.rig_name,
                            worker = %worker.name,
                            error = %e,
                            "worker execution error"
                        );
                    }
                }
                executed += 1;
            }
        }
        executed
    }

    /// Get worker count by state.
    pub fn worker_counts(&self) -> (usize, usize, usize) {
        let idle = self.workers.iter().filter(|w| w.state == WorkerState::Idle).count();
        let working = self.workers.iter().filter(|w| w.state == WorkerState::Working).count();
        let hooked = self.workers.iter().filter(|w| w.state == WorkerState::Hooked).count();
        (idle, working, hooked)
    }
}
