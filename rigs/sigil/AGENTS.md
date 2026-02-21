# Operating Instructions

Inherits from `rigs/shared/WORKFLOW.md` for code standards, R→D→R pipeline, and escalation.

## Sigil-Specific Workflow

**Exception to shared workflow**: Sigil works directly on `master` (no worktrees).

1. Run `cargo test` and `cargo clippy` before committing
2. Commit messages: `feat:`, `fix:`, `docs:`, `chore:`
3. Edition: Rust 2024

## Key Paths

- Binary: `sg/src/main.rs`
- Traits: `crates/sigil-core/src/traits/`
- Config: `config/sigil.toml`
- Rigs: `rigs/<name>/`
- Shared: `rigs/shared/`

## Available Skills

### R→D→R Archetypes (rig-specific overrides)
- **researcher**: Framework analysis — trait hierarchies, orchestration flow, config patterns
- **developer**: Rust implementation — trait design, async/tokio, edition 2024
- **reviewer**: Framework review — trait boundaries, async safety, no hardcoded heuristics

## Adding Things

- New tool: implement `Tool` trait in `sigil-tools`, export from lib.rs, add to `build_rig_tools()`
- New provider: implement `Provider` trait in `sigil-providers`, export, add factory
- New channel: implement `Channel` trait in `sigil-channels`, export, wire into daemon
- New rig: create `rigs/<name>/` with SOUL.md + IDENTITY.md + AGENTS.md, add to `config/sigil.toml`

## Critical Rules

- Traits over concrete types — everything through Provider, Tool, Memory, Observer, Channel
- Zero Framework Cognition — agent loop is thin, LLM decides everything
- No hardcoded heuristics in the agent loop
- Shared templates in `rigs/shared/` — never duplicate per-rig what can be shared
