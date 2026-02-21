use anyhow::{Context, Result};
use std::path::PathBuf;
use std::time::Instant;
use tokio::process::Command;
use tracing::{debug, info, warn};

/// Result of a Claude Code CLI execution.
#[derive(Debug)]
pub struct ExecutionResult {
    /// The assistant's final response text.
    pub result_text: String,
    /// Session ID (if returned).
    pub session_id: Option<String>,
    /// Number of agentic turns used.
    pub num_turns: u32,
    /// Total cost in USD.
    pub total_cost_usd: f64,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
}

/// Worker protocol injected into every Claude Code worker's system prompt.
/// Teaches workers how to report completion, signal blockers, and use sub-agents.
pub const WORKER_PROTOCOL: &str = r#"
## Worker Protocol

You are a Sigil worker executing a task (bead). Follow these rules strictly.

### Completion
When you successfully complete the task, provide a clear summary of what you changed.
Include file paths, commit hashes, and any deployment notes.

### Blocked — Need Input
If you cannot complete the task because you need a decision, clarification, or information
that isn't available in the codebase:
- Start your response with exactly: BLOCKED:
- On the next line, state the specific question you need answered
- Then describe what you've done so far and why you're stuck
- Be precise — your question will be passed to another agent or human for resolution

Example:
```
BLOCKED:
Should the new WebSocket endpoint require authentication, or should it be public?
I've implemented the handler and message types in src/ws.rs but need to know
whether to wire it through the auth middleware before proceeding.
```

### Failed — Technical Error
If the task fails due to a build error, test failure, or infrastructure issue you cannot fix:
- Start your response with exactly: FAILED:
- Include the error output and what you tried

### Sub-Agents
You have full access to Claude Code's Task tool for spawning sub-agents. Use them freely:
- Explore agents for parallel codebase research
- Bash agents for running tests and builds
- general-purpose agents for complex multi-step investigations
Each worker IS an orchestrator — swarm when the task is complex.

### Git Workflow
Follow the project's CLAUDE.md for git workflow (worktrees, branches, commits).
"#;

/// Spawns Claude Code CLI instances for bead execution.
///
/// Each execution is ephemeral: no session persistence, no interactive mode.
/// The worker's identity is injected via `--append-system-prompt` and the
/// repo's CLAUDE.md is auto-discovered from the working directory.
///
/// NO tool restrictions — workers get full Claude Code access including
/// Edit, Grep, Glob, Task (sub-agents), Bash, Read, Write, and everything else.
pub struct ClaudeCodeExecutor {
    /// Working directory (rig's repo path).
    workdir: PathBuf,
    /// Claude Code model (e.g., "claude-sonnet-4-6").
    model: String,
    /// Max agentic turns per execution.
    max_turns: u32,
    /// Max budget in USD per execution (None = unlimited).
    max_budget_usd: Option<f64>,
}

impl ClaudeCodeExecutor {
    pub fn new(
        workdir: PathBuf,
        model: String,
        max_turns: u32,
        max_budget_usd: Option<f64>,
    ) -> Self {
        Self {
            workdir,
            model,
            max_turns,
            max_budget_usd,
        }
    }

    /// Execute a bead via Claude Code CLI.
    ///
    /// Spawns `claude -p "<bead_context>"` with the rig's identity + worker protocol
    /// as `--append-system-prompt`. Returns the parsed result from JSON output.
    pub async fn execute(
        &self,
        identity: &sigil_core::Identity,
        bead_context: &str,
    ) -> Result<ExecutionResult> {
        let start = Instant::now();

        let mut cmd = Command::new("claude");

        // Core flags.
        cmd.arg("-p").arg(bead_context);
        cmd.arg("--output-format").arg("json");
        cmd.arg("--permission-mode").arg("bypassPermissions");
        cmd.arg("--model").arg(&self.model);
        cmd.arg("--max-turns").arg(self.max_turns.to_string());
        cmd.arg("--no-session-persistence");

        // Budget cap if configured.
        if let Some(budget) = self.max_budget_usd {
            cmd.arg("--max-budget-usd").arg(budget.to_string());
        }

        // NO --allowedTools — workers get full unrestricted Claude Code access.
        // This means Edit, Grep, Glob, Task (sub-agents), Bash, Read, Write,
        // WebSearch, WebFetch, NotebookEdit — everything.

        // Identity + worker protocol as system prompt appendage.
        let mut system_prompt = identity.system_prompt();
        system_prompt.push_str("\n\n---\n\n");
        system_prompt.push_str(WORKER_PROTOCOL);
        cmd.arg("--append-system-prompt").arg(&system_prompt);

        // Working directory.
        cmd.current_dir(&self.workdir);

        // CRITICAL: Unset CLAUDECODE env var to avoid nested-session block.
        cmd.env_remove("CLAUDECODE");
        cmd.env_remove("CLAUDE_CODE");

        debug!(
            workdir = %self.workdir.display(),
            model = %self.model,
            max_turns = self.max_turns,
            "spawning claude code (unrestricted)"
        );

        let output = cmd
            .output()
            .await
            .context("failed to spawn claude CLI — is it installed?")?;

        let duration_ms = start.elapsed().as_millis() as u64;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            warn!(
                exit_code = ?output.status.code(),
                stderr = %stderr,
                "claude code exited with error"
            );
            anyhow::bail!(
                "claude code failed (exit {}): {}",
                output.status.code().unwrap_or(-1),
                if stderr.is_empty() { &stdout } else { &stderr },
            );
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Self::parse_json_output(&stdout, duration_ms)
    }

    /// Parse the `--output-format json` response from Claude Code.
    fn parse_json_output(stdout: &str, duration_ms: u64) -> Result<ExecutionResult> {
        let v: serde_json::Value = serde_json::from_str(stdout)
            .context("failed to parse claude code JSON output")?;

        let result_text = v.get("result")
            .and_then(|r| r.as_str())
            .unwrap_or("")
            .to_string();

        let session_id = v.get("session_id")
            .and_then(|s| s.as_str())
            .map(String::from);

        let num_turns = v.get("num_turns")
            .and_then(|n| n.as_u64())
            .unwrap_or(0) as u32;

        let total_cost_usd = v.get("total_cost_usd")
            .and_then(|c| c.as_f64())
            .unwrap_or(0.0);

        info!(
            turns = num_turns,
            cost_usd = total_cost_usd,
            duration_ms = duration_ms,
            result_len = result_text.len(),
            "claude code execution complete"
        );

        Ok(ExecutionResult {
            result_text,
            session_id,
            num_turns,
            total_cost_usd,
            duration_ms,
        })
    }
}

/// Parsed outcome from a worker's result text.
#[derive(Debug, Clone)]
pub enum WorkerOutcome {
    /// Task completed successfully.
    Done(String),
    /// Worker is blocked and needs input to continue.
    Blocked {
        /// The specific question or information needed.
        question: String,
        /// Full result text including work done so far.
        full_text: String,
    },
    /// Task failed due to a technical error.
    Failed(String),
}

impl WorkerOutcome {
    /// Parse a worker's result text into a structured outcome.
    pub fn parse(result_text: &str) -> Self {
        let trimmed = result_text.trim();

        if trimmed.starts_with("BLOCKED:") {
            let after_prefix = trimmed.strip_prefix("BLOCKED:").unwrap_or("").trim();
            // The question is everything up to the first blank line (or all of it).
            let question = after_prefix
                .split("\n\n")
                .next()
                .unwrap_or(after_prefix)
                .trim()
                .to_string();
            Self::Blocked {
                question,
                full_text: result_text.to_string(),
            }
        } else if trimmed.starts_with("FAILED:") {
            Self::Failed(result_text.to_string())
        } else {
            Self::Done(result_text.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_output() {
        let json = r#"{
            "type": "result",
            "result": "I fixed the bug in main.rs",
            "session_id": "abc-123",
            "num_turns": 3,
            "total_cost_usd": 0.08
        }"#;

        let result = ClaudeCodeExecutor::parse_json_output(json, 5000).unwrap();
        assert_eq!(result.result_text, "I fixed the bug in main.rs");
        assert_eq!(result.session_id, Some("abc-123".to_string()));
        assert_eq!(result.num_turns, 3);
        assert!((result.total_cost_usd - 0.08).abs() < f64::EPSILON);
        assert_eq!(result.duration_ms, 5000);
    }

    #[test]
    fn test_parse_minimal_json() {
        let json = r#"{"type": "result", "result": "done"}"#;
        let result = ClaudeCodeExecutor::parse_json_output(json, 100).unwrap();
        assert_eq!(result.result_text, "done");
        assert_eq!(result.num_turns, 0);
        assert_eq!(result.total_cost_usd, 0.0);
    }

    #[test]
    fn test_worker_outcome_done() {
        let outcome = WorkerOutcome::parse("I fixed the bug and committed to feat/fix-pms.");
        assert!(matches!(outcome, WorkerOutcome::Done(_)));
    }

    #[test]
    fn test_worker_outcome_blocked() {
        let text = "BLOCKED:\nShould auth be JWT or session-based?\n\nI've set up the middleware but need to know the auth strategy.";
        let outcome = WorkerOutcome::parse(text);
        match outcome {
            WorkerOutcome::Blocked { question, .. } => {
                assert_eq!(question, "Should auth be JWT or session-based?");
            }
            _ => panic!("expected Blocked"),
        }
    }

    #[test]
    fn test_worker_outcome_failed() {
        let outcome = WorkerOutcome::parse("FAILED:\ncargo build returned 3 errors in pms/src/main.rs");
        assert!(matches!(outcome, WorkerOutcome::Failed(_)));
    }
}
