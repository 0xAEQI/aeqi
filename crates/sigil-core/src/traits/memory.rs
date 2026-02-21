use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A stored memory entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub key: String,
    pub content: String,
    pub category: MemoryCategory,
    pub created_at: DateTime<Utc>,
    pub session_id: Option<String>,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Fact,
    Procedure,
    Preference,
    Context,
    Evergreen,
}

/// Query for memory retrieval.
#[derive(Debug, Clone)]
pub struct MemoryQuery {
    pub text: String,
    pub top_k: usize,
    pub category: Option<MemoryCategory>,
    pub session_id: Option<String>,
}

impl MemoryQuery {
    pub fn new(text: impl Into<String>, top_k: usize) -> Self {
        Self {
            text: text.into(),
            top_k,
            category: None,
            session_id: None,
        }
    }
}

/// Memory backend trait.
#[async_trait]
pub trait Memory: Send + Sync {
    /// Store a memory entry.
    async fn store(&self, key: &str, content: &str, category: MemoryCategory) -> anyhow::Result<String>;

    /// Search memories by query.
    async fn search(&self, query: &MemoryQuery) -> anyhow::Result<Vec<MemoryEntry>>;

    /// Delete a memory by ID.
    async fn delete(&self, id: &str) -> anyhow::Result<()>;

    /// Backend name for logging.
    fn name(&self) -> &str;
}
