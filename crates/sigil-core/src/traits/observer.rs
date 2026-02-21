use async_trait::async_trait;
use serde_json::Value;

/// Event types for observability.
#[derive(Debug, Clone)]
pub enum Event {
    AgentStart { agent_name: String },
    AgentEnd { agent_name: String, iterations: u32 },
    LlmRequest { model: String, tokens: u32 },
    LlmResponse { model: String, prompt_tokens: u32, completion_tokens: u32 },
    ToolCall { tool_name: String, duration_ms: u64 },
    ToolError { tool_name: String, error: String },
    Custom { name: String, data: Value },
}

/// Observability trait for metrics, logging, tracing.
#[async_trait]
pub trait Observer: Send + Sync {
    /// Record an event.
    async fn record(&self, event: Event);

    /// Observer name.
    fn name(&self) -> &str;
}

/// Default observer that logs to tracing.
pub struct LogObserver;

#[async_trait]
impl Observer for LogObserver {
    async fn record(&self, event: Event) {
        match &event {
            Event::AgentStart { agent_name } => {
                tracing::info!(agent = %agent_name, "agent started");
            }
            Event::AgentEnd { agent_name, iterations } => {
                tracing::info!(agent = %agent_name, iterations, "agent completed");
            }
            Event::LlmRequest { model, tokens } => {
                tracing::debug!(model = %model, tokens, "LLM request");
            }
            Event::LlmResponse { model, prompt_tokens, completion_tokens } => {
                tracing::debug!(model = %model, prompt_tokens, completion_tokens, "LLM response");
            }
            Event::ToolCall { tool_name, duration_ms } => {
                tracing::debug!(tool = %tool_name, duration_ms, "tool executed");
            }
            Event::ToolError { tool_name, error } => {
                tracing::warn!(tool = %tool_name, error = %error, "tool error");
            }
            Event::Custom { name, data } => {
                tracing::info!(event = %name, data = %data, "custom event");
            }
        }
    }

    fn name(&self) -> &str {
        "log"
    }
}
