use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sigil_beads::BeadId;
use anyhow::{Context, Result};
use std::path::Path;
use tracing::info;

/// A convoy tracks work across multiple rigs.
/// It monitors a set of beads from different rigs and
/// auto-closes when all tracked beads are completed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub beads: Vec<ConvoyBead>,
    pub created_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvoyBead {
    pub bead_id: BeadId,
    pub rig: String,
    pub closed: bool,
}

impl Convoy {
    pub fn new(name: &str, beads: Vec<(BeadId, String)>) -> Self {
        let id = format!("convoy-{}", uuid::Uuid::new_v4().as_simple());
        Self {
            id,
            name: name.to_string(),
            beads: beads.into_iter().map(|(bead_id, rig)| ConvoyBead {
                bead_id,
                rig,
                closed: false,
            }).collect(),
            created_at: Utc::now(),
            closed_at: None,
        }
    }

    /// Mark a bead as closed in this convoy.
    pub fn mark_closed(&mut self, bead_id: &BeadId) {
        for b in &mut self.beads {
            if b.bead_id == *bead_id {
                b.closed = true;
            }
        }
    }

    /// Check if all beads in the convoy are closed.
    pub fn is_complete(&self) -> bool {
        self.beads.iter().all(|b| b.closed)
    }

    /// Count completed vs total.
    pub fn progress(&self) -> (usize, usize) {
        let done = self.beads.iter().filter(|b| b.closed).count();
        (done, self.beads.len())
    }
}

/// Persistent convoy store.
pub struct ConvoyStore {
    path: std::path::PathBuf,
    pub convoys: Vec<Convoy>,
}

impl ConvoyStore {
    pub fn open(path: &Path) -> Result<Self> {
        let mut store = Self {
            path: path.to_path_buf(),
            convoys: Vec::new(),
        };

        if path.exists() {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("failed to read convoy store: {}", path.display()))?;
            store.convoys = serde_json::from_str(&content).unwrap_or_default();
        }

        Ok(store)
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self.convoys)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }

    /// Create a new convoy.
    pub fn create(&mut self, name: &str, beads: Vec<(BeadId, String)>) -> Result<&Convoy> {
        let convoy = Convoy::new(name, beads);
        info!(id = %convoy.id, name = %name, beads = convoy.beads.len(), "convoy created");
        self.convoys.push(convoy);
        self.save()?;
        Ok(self.convoys.last().unwrap())
    }

    /// Mark a bead as closed across all active convoys.
    pub fn mark_bead_closed(&mut self, bead_id: &BeadId) -> Result<Vec<String>> {
        let mut completed_convoys = Vec::new();

        for convoy in &mut self.convoys {
            if convoy.closed_at.is_some() {
                continue;
            }
            convoy.mark_closed(bead_id);
            if convoy.is_complete() {
                convoy.closed_at = Some(Utc::now());
                info!(id = %convoy.id, name = %convoy.name, "convoy completed");
                completed_convoys.push(convoy.id.clone());
            }
        }

        if !completed_convoys.is_empty() {
            self.save()?;
        }

        Ok(completed_convoys)
    }

    /// Get a convoy by ID.
    pub fn get(&self, id: &str) -> Option<&Convoy> {
        self.convoys.iter().find(|c| c.id == id)
    }

    /// List active (unclosed) convoys.
    pub fn active(&self) -> Vec<&Convoy> {
        self.convoys.iter().filter(|c| c.closed_at.is_none()).collect()
    }

    /// Remove completed convoys older than the specified days.
    pub fn cleanup(&mut self, max_age_days: i64) -> Result<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(max_age_days);
        let before = self.convoys.len();
        self.convoys.retain(|c| {
            c.closed_at.map(|t| t > cutoff).unwrap_or(true)
        });
        let removed = before - self.convoys.len();
        if removed > 0 {
            self.save()?;
        }
        Ok(removed)
    }
}
