//! Skill promotion from recurring memory patterns.
//!
//! When agents repeatedly solve similar problems, the system accumulates
//! pattern-category memories.  The [`SkillPromoter`] detects clusters of 3+
//! similar patterns and promotes them into formal [`SkillDefinition`]s —
//! rendered as SKILL.md files that are injected into future worker context
//! when a task matches the trigger conditions.
//!
//! This is the evolution layer: Sigil learns from experience and codifies
//! that knowledge into reusable skills.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

// ── Source Types ────────────────────────────────────────────────────────────

/// A memory entry of category "pattern" or "procedure" that feeds the
/// skill-promotion pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMemory {
    /// Memory ID in the store.
    pub id: String,
    /// Semantic key (e.g. "deploy/rollback-procedure").
    pub key: String,
    /// Full content text.
    pub content: String,
    /// Category — should be "pattern" or "procedure".
    pub category: String,
}

// ── Candidate ──────────────────────────────────────────────────────────────

/// A skill that has been detected from a cluster of similar patterns but
/// not yet promoted to a formal definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCandidate {
    /// Human-readable skill name derived from the cluster.
    pub name: String,
    /// One-sentence description of what this skill does.
    pub description: String,
    /// Conditions under which this skill should be triggered.
    pub trigger_conditions: Vec<String>,
    /// Ordered workflow steps extracted from the patterns.
    pub workflow_steps: Vec<String>,
    /// How to verify the skill was applied correctly.
    pub verification: String,
    /// IDs of the source pattern memories that formed this cluster.
    pub source_memory_ids: Vec<String>,
    /// Confidence score in `[0.0, 1.0]` based on cluster tightness.
    pub confidence: f32,
}

// ── Definition ─────────────────────────────────────────────────────────────

/// A promoted skill — the final artifact that gets stored as SKILL.md and
/// injected into worker context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    /// Skill name.
    pub name: String,
    /// Full SKILL.md markdown content.
    pub content: String,
    /// When this skill was promoted.
    pub promoted_at: DateTime<Utc>,
    /// Number of source patterns that contributed.
    pub source_count: u32,
}

// ── Promoter ───────────────────────────────────────────────────────────────

/// Detects recurring patterns in memory and promotes them to formal skills.
pub struct SkillPromoter {
    /// Minimum number of similar patterns required before promotion (default 3).
    pub pattern_threshold: u32,
}

impl Default for SkillPromoter {
    fn default() -> Self {
        Self {
            pattern_threshold: 3,
        }
    }
}

impl SkillPromoter {
    /// Create a promoter with a custom pattern threshold.
    pub fn new(pattern_threshold: u32) -> Self {
        Self { pattern_threshold }
    }

    /// Detect skill candidates by clustering similar patterns.
    ///
    /// Similarity heuristic: two patterns are "similar" when they share
    /// more than 60% of their significant words (words with length >= 3)
    /// across both key and content.  Clusters of `pattern_threshold` or
    /// more become candidates.
    pub fn detect_candidates(&self, patterns: &[PatternMemory]) -> Vec<SkillCandidate> {
        if patterns.len() < self.pattern_threshold as usize {
            return Vec::new();
        }

        // Tokenize each pattern into a word set.
        let word_sets: Vec<std::collections::HashSet<String>> = patterns
            .iter()
            .map(|p| tokenize(&format!("{} {}", p.key, p.content)))
            .collect();

        // Build adjacency: which patterns are similar to which.
        let n = patterns.len();
        let mut adjacency: Vec<Vec<bool>> = vec![vec![false; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if word_overlap_ratio(&word_sets[i], &word_sets[j]) > 0.6 {
                    adjacency[i][j] = true;
                    adjacency[j][i] = true;
                }
            }
        }

        // Greedy clustering: pick unvisited node, grow cluster via adjacency.
        let mut visited = vec![false; n];
        let mut clusters: Vec<Vec<usize>> = Vec::new();

        for i in 0..n {
            if visited[i] {
                continue;
            }
            let mut cluster = vec![i];
            visited[i] = true;

            // BFS from i.
            let mut queue = vec![i];
            while let Some(curr) = queue.pop() {
                for j in 0..n {
                    if !visited[j] && adjacency[curr][j] {
                        visited[j] = true;
                        cluster.push(j);
                        queue.push(j);
                    }
                }
            }

            if cluster.len() >= self.pattern_threshold as usize {
                clusters.push(cluster);
            }
        }

        debug!(
            cluster_count = clusters.len(),
            "skill promoter detected pattern clusters"
        );

        clusters
            .into_iter()
            .map(|indices| self.cluster_to_candidate(patterns, &indices))
            .collect()
    }

    /// Render a SKILL.md formatted string from a candidate.
    pub fn generate_skill_md(candidate: &SkillCandidate) -> String {
        let mut md = format!("# {}\n\n", candidate.name);

        md.push_str("## When to use\n\n");
        for condition in &candidate.trigger_conditions {
            md.push_str(&format!("- {condition}\n"));
        }

        md.push_str("\n## Workflow\n\n");
        for (i, step) in candidate.workflow_steps.iter().enumerate() {
            md.push_str(&format!("{}. {step}\n", i + 1));
        }

        md.push_str(&format!(
            "\n## Verification\n\n{}\n",
            candidate.verification
        ));

        md
    }

    /// Promote a candidate into a full skill definition with rendered markdown.
    pub fn promote(candidate: &SkillCandidate) -> SkillDefinition {
        let content = Self::generate_skill_md(candidate);
        SkillDefinition {
            name: candidate.name.clone(),
            content,
            promoted_at: Utc::now(),
            source_count: candidate.source_memory_ids.len() as u32,
        }
    }

    /// Convert a cluster of pattern indices into a skill candidate.
    fn cluster_to_candidate(
        &self,
        patterns: &[PatternMemory],
        indices: &[usize],
    ) -> SkillCandidate {
        let cluster_patterns: Vec<&PatternMemory> = indices.iter().map(|&i| &patterns[i]).collect();

        // Derive name from the most common key prefix.
        let name = derive_skill_name(&cluster_patterns);

        // Extract trigger conditions from keys.
        let trigger_conditions: Vec<String> = cluster_patterns
            .iter()
            .map(|p| format!("Task involves: {}", p.key))
            .collect();

        // Extract workflow steps from content (first sentence of each pattern).
        let workflow_steps: Vec<String> = cluster_patterns
            .iter()
            .filter_map(|p| {
                p.content
                    .split(['.', '\n'])
                    .find(|s| !s.trim().is_empty())
                    .map(|s| s.trim().to_string())
            })
            .collect();

        let source_memory_ids: Vec<String> =
            cluster_patterns.iter().map(|p| p.id.clone()).collect();

        // Confidence based on cluster size relative to threshold.
        let confidence = (indices.len() as f32 / (self.pattern_threshold as f32 * 2.0)).min(1.0);

        SkillCandidate {
            name,
            description: format!(
                "Skill derived from {} recurring patterns",
                indices.len()
            ),
            trigger_conditions,
            workflow_steps,
            verification: "Verify that the workflow steps were followed and the outcome matches expected results".to_string(),
            source_memory_ids,
            confidence,
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

/// Tokenize text into a set of lowercase words (length >= 3).
fn tokenize(text: &str) -> std::collections::HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric() && c != '-')
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() >= 3)
        .collect()
}

/// Compute the overlap ratio between two word sets using Jaccard-like metric.
///
/// Returns `intersection.len() / min(a.len(), b.len())` so that a small
/// pattern contained within a larger one still scores high.
fn word_overlap_ratio(
    a: &std::collections::HashSet<String>,
    b: &std::collections::HashSet<String>,
) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }
    let intersection = a.intersection(b).count();
    let min_size = a.len().min(b.len());
    intersection as f64 / min_size as f64
}

/// Derive a skill name from a cluster of patterns.
///
/// Uses the most common key prefix (before the first '/') and appends "Skill".
fn derive_skill_name(patterns: &[&PatternMemory]) -> String {
    let mut prefix_counts: HashMap<String, usize> = HashMap::new();
    for p in patterns {
        let prefix = p.key.split('/').next().unwrap_or(&p.key).to_string();
        *prefix_counts.entry(prefix).or_insert(0) += 1;
    }

    let best_prefix = prefix_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(prefix, _)| prefix)
        .unwrap_or_else(|| "unknown".to_string());

    // Title-case the prefix.
    let title = best_prefix
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{upper}{}", chars.as_str())
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    format!("{title} Skill")
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_pattern(id: &str, key: &str, content: &str) -> PatternMemory {
        PatternMemory {
            id: id.to_string(),
            key: key.to_string(),
            content: content.to_string(),
            category: "pattern".to_string(),
        }
    }

    #[test]
    fn detect_candidates_with_similar_patterns() {
        let promoter = SkillPromoter::default(); // threshold = 3

        let patterns = vec![
            make_pattern(
                "p1",
                "deploy/rollback",
                "Stop the service, restore backup, restart service, verify health",
            ),
            make_pattern(
                "p2",
                "deploy/rollback-fix",
                "Stop the service, apply hotfix, restart service, verify health check",
            ),
            make_pattern(
                "p3",
                "deploy/rollback-db",
                "Stop the service, restore database backup, restart service, verify health",
            ),
        ];

        let candidates = promoter.detect_candidates(&patterns);
        assert_eq!(candidates.len(), 1, "should detect one cluster");
        assert_eq!(candidates[0].source_memory_ids.len(), 3);
        assert!(candidates[0].confidence > 0.0);
    }

    #[test]
    fn no_candidates_below_threshold() {
        let promoter = SkillPromoter::default(); // threshold = 3

        let patterns = vec![
            make_pattern("p1", "deploy/rollback", "Stop service, restore, restart"),
            make_pattern("p2", "auth/jwt", "Rotate JWT tokens every 24 hours"),
        ];

        let candidates = promoter.detect_candidates(&patterns);
        assert!(
            candidates.is_empty(),
            "should not detect candidates with fewer than 3 patterns"
        );
    }

    #[test]
    fn no_candidates_dissimilar_patterns() {
        let promoter = SkillPromoter::default();

        let patterns = vec![
            make_pattern("p1", "deploy/rollback", "Stop the service and restore"),
            make_pattern("p2", "auth/jwt", "Rotate JWT tokens with refresh mechanism"),
            make_pattern("p3", "pricing/tiers", "Three pricing tiers for enterprise"),
        ];

        let candidates = promoter.detect_candidates(&patterns);
        assert!(
            candidates.is_empty(),
            "dissimilar patterns should not cluster"
        );
    }

    #[test]
    fn generate_skill_md_format() {
        let candidate = SkillCandidate {
            name: "Deploy Rollback".to_string(),
            description: "Rollback deployment procedure".to_string(),
            trigger_conditions: vec![
                "Service deployment fails".to_string(),
                "Health check fails after deploy".to_string(),
            ],
            workflow_steps: vec![
                "Stop the service".to_string(),
                "Restore from backup".to_string(),
                "Restart the service".to_string(),
                "Verify health check passes".to_string(),
            ],
            verification: "All health checks pass and service responds correctly".to_string(),
            source_memory_ids: vec!["p1".to_string(), "p2".to_string(), "p3".to_string()],
            confidence: 0.8,
        };

        let md = SkillPromoter::generate_skill_md(&candidate);

        assert!(md.starts_with("# Deploy Rollback\n"));
        assert!(md.contains("## When to use"));
        assert!(md.contains("- Service deployment fails"));
        assert!(md.contains("- Health check fails after deploy"));
        assert!(md.contains("## Workflow"));
        assert!(md.contains("1. Stop the service"));
        assert!(md.contains("2. Restore from backup"));
        assert!(md.contains("3. Restart the service"));
        assert!(md.contains("4. Verify health check passes"));
        assert!(md.contains("## Verification"));
        assert!(md.contains("All health checks pass and service responds correctly"));
    }

    #[test]
    fn promote_creates_valid_definition() {
        let candidate = SkillCandidate {
            name: "Auth Rotation".to_string(),
            description: "JWT rotation procedure".to_string(),
            trigger_conditions: vec!["Token expiry approaching".to_string()],
            workflow_steps: vec![
                "Generate new token pair".to_string(),
                "Rotate secrets".to_string(),
            ],
            verification: "New tokens validate correctly".to_string(),
            source_memory_ids: vec!["m1".to_string(), "m2".to_string(), "m3".to_string()],
            confidence: 0.75,
        };

        let definition = SkillPromoter::promote(&candidate);

        assert_eq!(definition.name, "Auth Rotation");
        assert_eq!(definition.source_count, 3);
        assert!(definition.content.contains("# Auth Rotation"));
        assert!(definition.content.contains("## When to use"));
        assert!(definition.content.contains("## Workflow"));
        assert!(definition.content.contains("## Verification"));
        // promoted_at should be recent.
        let age = Utc::now() - definition.promoted_at;
        assert!(age.num_seconds() < 5);
    }

    #[test]
    fn custom_threshold() {
        let promoter = SkillPromoter::new(2);

        let patterns = vec![
            make_pattern(
                "p1",
                "deploy/rollback",
                "Stop the service, restore backup, restart service",
            ),
            make_pattern(
                "p2",
                "deploy/rollback-fix",
                "Stop the service, apply hotfix, restart service",
            ),
        ];

        let candidates = promoter.detect_candidates(&patterns);
        assert_eq!(
            candidates.len(),
            1,
            "threshold of 2 should detect a cluster from 2 patterns"
        );
    }

    #[test]
    fn word_overlap_ratio_works() {
        let a = tokenize("deploy service backup restart verify");
        let b = tokenize("deploy service restore restart verify");
        let ratio = word_overlap_ratio(&a, &b);
        // 4 shared words out of 5 = 0.8
        assert!(ratio > 0.7, "overlap ratio = {ratio}");
    }

    #[test]
    fn word_overlap_ratio_empty_sets() {
        let a = tokenize("");
        let b = tokenize("hello world");
        assert!((word_overlap_ratio(&a, &b) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn tokenize_filters_short_words() {
        let words = tokenize("a an the deploy to fix service ok");
        assert!(words.contains("deploy"));
        assert!(words.contains("fix"));
        assert!(words.contains("service"));
        assert!(!words.contains("a"));
        assert!(!words.contains("an"));
        assert!(!words.contains("to"));
        assert!(!words.contains("ok"));
    }
}
