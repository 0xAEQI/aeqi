#![allow(clippy::too_many_arguments)]
//! Agent orchestration engine — the operational heart of AEQI.
//!
//! Coordinates worker execution ([`AgentWorker`]), agent routing ([`AgentRouter`]),
//! global scheduling ([`Scheduler`]), agent registry ([`agent_registry::AgentRegistry`]),
//! dispatch bus ([`DispatchBus`]), cost ledger ([`CostLedger`]), Prometheus metrics
//! ([`AEQIMetrics`]), and session storage.

pub mod agent_registry;
pub mod agent_router;
pub mod agent_worker;
pub mod audit;
pub mod checkpoint;
pub mod claude_code;
pub mod context_budget;
pub mod cost_ledger;
pub mod daemon;
pub mod delegate;
pub mod escalation;
pub mod event_store;
pub mod execution_events;
pub mod executor;
pub mod expertise;
pub mod failure_analysis;
pub mod hook;
pub mod intent;
pub mod message;
pub mod message_router;
pub mod metrics;
pub mod middleware;
pub mod notes;
pub mod operation;
pub mod pipeline;
pub mod preflight;
pub mod progress_tracker;
pub mod prompt_assembly;
pub mod runtime;
pub mod scheduler;
pub mod session_manager;
pub mod session_store;
pub mod template;
pub mod tools;
pub mod trigger;
pub mod verification;
pub mod vfs;

pub use agent_registry::Agent;
pub use agent_router::{AgentRouter, RouteDecision};
pub use agent_worker::{AgentWorker, WorkerState};
pub use audit::{AuditEvent, AuditLog, DecisionType};
pub use checkpoint::AgentCheckpoint;
pub use context_budget::ContextBudget;
pub use cost_ledger::CostLedger;
pub use daemon::Daemon;
pub use event_store::EventStore;
pub use execution_events::{EventBroadcaster, ExecutionEvent};
pub use executor::TaskOutcome;
pub use expertise::ExpertiseLedger;
pub use hook::Hook;
pub use message::{Dispatch, DispatchBus, DispatchHealth, DispatchKind};
pub use message_router::MessageRouter;
pub use metrics::AEQIMetrics;
pub use notes::{AgentVisibility, Notes};
pub use operation::{Operation, OperationStore};
pub use pipeline::{Pipeline, PipelineStep};
pub use progress_tracker::ProgressTracker;
pub use runtime::{
    Artifact, ArtifactKind, RuntimeExecution, RuntimeOutcome, RuntimeOutcomeStatus, RuntimePhase,
    RuntimeSession, RuntimeSessionStatus, VerificationReport,
};
pub use scheduler::{Scheduler, SchedulerConfig};
pub use session_store::SessionStore;
pub use template::Template;
pub use trigger::{EventPattern, Trigger, TriggerStore, TriggerType};
