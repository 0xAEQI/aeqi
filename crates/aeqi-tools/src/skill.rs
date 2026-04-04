use aeqi_core::frontmatter;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// A prompt file — MD with YAML frontmatter. Unified format for skills, agents, and all prompts.
///
/// The frontmatter holds metadata, the body is the system prompt.
/// Skills and agents are both just prompt files differentiated by `inject`:
/// - `inject: session` — loaded into system prompt at session start
/// - `inject: turn` — re-injected each message
/// - (omitted) — invoked on-demand
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    /// Injection mode: `"session"` or `"turn"`. None = on-demand.
    #[serde(default)]
    pub inject: Option<String>,
    #[serde(default)]
    pub phase: String,
    #[serde(default)]
    pub triggers: Vec<String>,
    /// Natural-language description of when to auto-invoke.
    #[serde(default)]
    pub when_to_use: Option<String>,
    /// `"fork"` = subagent, `"inline"` = expand into current context.
    #[serde(default)]
    pub context: Option<String>,
    /// Named arguments for `$arg` substitution in the body.
    #[serde(default)]
    pub arguments: Vec<String>,
    #[serde(default)]
    pub argument_hint: Option<String>,
    /// Model override (e.g. `"anthropic/claude-haiku-4-5"`).
    #[serde(default)]
    pub model: Option<String>,
    /// Enable `!`backtick`` shell expansion in the body at load time.
    #[serde(default)]
    pub allow_shell: bool,
    // ── Tools ──
    /// Allowed tools (empty = all allowed).
    #[serde(default)]
    pub tools: Vec<String>,
    /// Denied tools.
    #[serde(default)]
    pub deny: Vec<String>,
    // ── Prompt ──
    /// The prompt body (system prompt). Populated from the MD body, not frontmatter.
    #[serde(skip)]
    pub body: String,
    /// Prefix prepended to user messages when invoking this skill.
    #[serde(default)]
    pub user_prefix: String,
    // ── Execution ──
    /// Number of parallel agents for fan-out. None = sequential.
    #[serde(default)]
    pub parallel: Option<u32>,
    /// Run in an isolated git worktree.
    #[serde(default)]
    pub worktree: bool,
    /// Max budget per agent (USD).
    #[serde(default)]
    pub max_budget_usd: Option<f64>,
    // ── Verification ──
    /// Commands to run for verification.
    #[serde(default)]
    pub verify: Vec<String>,
    /// Expected patterns in verification output.
    #[serde(default)]
    pub verify_patterns: Vec<String>,
}

impl Skill {
    /// Load a skill from an MD file with YAML frontmatter.
    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read prompt: {}", path.display()))?;
        let mut skill = Self::parse(&content)
            .with_context(|| format!("failed to parse prompt: {}", path.display()))?;
        // Default name from filename if not in frontmatter.
        if skill.name.is_empty() {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                skill.name = stem.to_string();
            }
        }
        Ok(skill)
    }

    /// Parse an MD string with YAML frontmatter into a Skill.
    pub fn parse(content: &str) -> Result<Self> {
        let (mut skill, body): (Self, String) = frontmatter::load_frontmatter(content)?;
        skill.body = body;
        Ok(skill)
    }

    /// Discover all skills (`.md` files) in a directory.
    pub fn discover(dir: &Path) -> Result<Vec<Self>> {
        let mut skills = Vec::new();
        if !dir.exists() {
            return Ok(skills);
        }
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md") {
                match Self::load(&path) {
                    Ok(skill) => skills.push(skill),
                    Err(e) => {
                        tracing::warn!(path = %path.display(), error = %e, "skipping invalid prompt");
                    }
                }
            }
        }
        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(skills)
    }

    /// Build the full system prompt for this skill.
    /// If `allow_shell` is true, executes `!`backtick`` blocks.
    pub fn system_prompt(&self, base_identity: &str) -> String {
        let mut prompt = self.body.clone();

        if self.allow_shell {
            prompt = Self::expand_shell_commands(&prompt);
        }

        if base_identity.is_empty() {
            prompt
        } else {
            format!(
                "{}\n\n---\n\n# Skill: {}\n\n{}",
                base_identity, self.name, prompt
            )
        }
    }

    /// Execute `!`backtick`` blocks in a prompt string and replace with stdout.
    fn expand_shell_commands(prompt: &str) -> String {
        let mut result = String::with_capacity(prompt.len());
        let mut remaining = prompt;

        while let Some(start) = remaining.find("!`") {
            result.push_str(&remaining[..start]);
            let after_marker = &remaining[start + 2..];

            if let Some(end) = after_marker.find('`') {
                let command = &after_marker[..end];
                let output = std::process::Command::new("bash")
                    .arg("-c")
                    .arg(command)
                    .output();

                match output {
                    Ok(out) if out.status.success() => {
                        let stdout = String::from_utf8_lossy(&out.stdout);
                        result.push_str(stdout.trim_end());
                    }
                    Ok(out) => {
                        let stderr = String::from_utf8_lossy(&out.stderr);
                        result.push_str(&format!("[shell error: {}]", stderr.trim()));
                    }
                    Err(e) => {
                        result.push_str(&format!("[shell exec failed: {e}]"));
                    }
                }

                remaining = &after_marker[end + 1..];
            } else {
                result.push_str("!`");
                remaining = after_marker;
            }
        }
        result.push_str(remaining);
        result
    }

    /// Whether this skill runs as a forked subagent.
    pub fn is_fork_context(&self) -> bool {
        self.context.as_deref().is_some_and(|c| c == "fork")
    }

    /// Whether this skill has auto-invocation criteria.
    pub fn has_auto_trigger(&self) -> bool {
        self.when_to_use.is_some()
    }

    /// Substitute `$arg_name` placeholders in the body.
    pub fn substitute_args(&self, args: &std::collections::HashMap<String, String>) -> String {
        let mut prompt = self.body.clone();
        for (key, value) in args {
            prompt = prompt.replace(&format!("${key}"), value);
        }
        prompt
    }

    /// Check if a tool is allowed by this skill's policy.
    pub fn is_tool_allowed(&self, tool_name: &str) -> bool {
        if !self.deny.is_empty() && self.deny.contains(&tool_name.to_string()) {
            return false;
        }
        if !self.tools.is_empty() {
            return self.tools.contains(&tool_name.to_string());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill() {
        let md = r#"---
name: deploy
description: Deploy a service
phase: workflow
when_to_use: Use when the user wants to deploy to production
context: fork
arguments: [service, env]
argument_hint: "<service> <env>"
tools: [shell, read_file]
parallel: 3
worktree: true
max_budget_usd: 1.50
---

Deploy $service to $env"#;

        let skill = Skill::parse(md).unwrap();
        assert_eq!(skill.name, "deploy");
        assert_eq!(
            skill.when_to_use.as_deref(),
            Some("Use when the user wants to deploy to production")
        );
        assert_eq!(skill.context.as_deref(), Some("fork"));
        assert!(skill.is_fork_context());
        assert!(skill.has_auto_trigger());
        assert_eq!(skill.arguments, vec!["service", "env"]);
        assert_eq!(skill.argument_hint.as_deref(), Some("<service> <env>"));
        assert_eq!(skill.parallel, Some(3));
        assert!(skill.worktree);
        assert!((skill.max_budget_usd.unwrap() - 1.50).abs() < f64::EPSILON);
    }

    #[test]
    fn test_minimal_skill() {
        let md = r#"---
name: health-check
description: Check health
phase: autonomous
tools: [shell]
---

Check health"#;

        let skill = Skill::parse(md).unwrap();
        assert_eq!(skill.name, "health-check");
        assert!(!skill.is_fork_context());
        assert!(!skill.has_auto_trigger());
        assert!(skill.arguments.is_empty());
        assert!(skill.parallel.is_none());
    }

    #[test]
    fn test_argument_substitution() {
        let md = r#"---
name: test
description: test
arguments: [name, target]
---

Deploy $name to $target environment"#;

        let skill = Skill::parse(md).unwrap();
        let mut args = std::collections::HashMap::new();
        args.insert("name".to_string(), "myapp".to_string());
        args.insert("target".to_string(), "production".to_string());

        let result = skill.substitute_args(&args);
        assert_eq!(result, "Deploy myapp to production environment");
    }

    #[test]
    fn test_shell_expansion_in_prompt() {
        let md = r#"---
name: test
description: test
allow_shell: true
---

Date: !`echo 2026-04-01` and host: !`echo testhost`"#;

        let skill = Skill::parse(md).unwrap();
        let prompt = skill.system_prompt("");
        assert!(prompt.contains("2026-04-01"), "got: {prompt}");
        assert!(prompt.contains("testhost"), "got: {prompt}");
        assert!(!prompt.contains("!`"), "shell markers should be replaced");
    }

    #[test]
    fn test_shell_expansion_disabled_by_default() {
        let md = r#"---
name: test
description: test
---

Should not expand: !`echo danger`"#;

        let skill = Skill::parse(md).unwrap();
        let prompt = skill.system_prompt("");
        assert!(prompt.contains("!`echo danger`"));
    }

    #[test]
    fn test_model_override() {
        let md = r#"---
name: cheap-task
description: Uses a cheaper model
model: anthropic/claude-haiku-4-5
---

Do something cheap"#;

        let skill = Skill::parse(md).unwrap();
        assert_eq!(
            skill.model.as_deref(),
            Some("anthropic/claude-haiku-4-5")
        );
    }

    #[test]
    fn test_tool_allowed() {
        let md = r#"---
name: test
description: test
tools: [shell, read_file]
deny: [write_file]
---

test"#;

        let skill = Skill::parse(md).unwrap();
        assert!(skill.is_tool_allowed("shell"));
        assert!(skill.is_tool_allowed("read_file"));
        assert!(!skill.is_tool_allowed("write_file"));
        assert!(!skill.is_tool_allowed("edit_file"));
    }

    #[test]
    fn test_inject_field() {
        let md = r#"---
name: primer
description: Session primer
inject: session
---

You are the system."#;

        let skill = Skill::parse(md).unwrap();
        assert_eq!(skill.inject.as_deref(), Some("session"));
    }
}
