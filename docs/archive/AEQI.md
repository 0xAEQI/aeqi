# AEQI Reference

This is the live reference for the AEQI workspace as it exists today.

## Workspace Snapshot

- Cargo workspace with 8 crates
- CLI binary in `aeqi-cli/`
- 26 top-level CLI commands
- 217 unit tests currently passing

## Command Surface

### Core

- `aeqi init`
- `aeqi setup`
- `aeqi doctor [--fix]`
- `aeqi status`
- `aeqi monitor [--project NAME] [--watch] [--json]`
- `aeqi config show`
- `aeqi config reload`
- `aeqi team`
- `aeqi agent list`
- `aeqi agent migrate`
- `aeqi secrets set|get|list|delete`

### One-Shot Execution

- `aeqi run "prompt" [--project NAME]`
- `aeqi skill list [--project NAME]`
- `aeqi skill run NAME --project NAME [prompt]`

Notes:

- `aeqi run` uses the internal agent loop and selects the project runtime when `--project` is set
- `aeqi skill run` also uses the internal agent loop, but filters tools by the selected skill policy

### Task and Mission Flow

- `aeqi assign "subject" --project NAME`
- `aeqi ready [--project NAME]`
- `aeqi tasks [--project NAME] [--all]`
- `aeqi close TASK_ID`
- `aeqi hook WORKER TASK_ID`
- `aeqi done TASK_ID`
- `aeqi mission create|list|status|close`
- `aeqi deps --project NAME [--apply THRESHOLD]`

Task IDs use a prefix-based hierarchy:

- root task: `mp-001`
- child task: `mp-001.1`
- grandchild task: `mp-001.1.1`

Mission IDs use `prefix-mNNN`, for example `mp-m001`.

### Pipelines and Operations

- `aeqi pipeline list [--project NAME]`
- `aeqi pipeline pour TEMPLATE --project NAME --var key=value`
- `aeqi pipeline status TASK_ID`
- `aeqi operation create NAME TASK_ID...`
- `aeqi operation list`
- `aeqi operation status OP_ID`

Pipeline discovery order:

1. `projects/shared/pipelines/`
2. `projects/shared/rituals/`
3. `projects/<name>/pipelines/`
4. `projects/<name>/rituals/`

Project-local pipeline names override shared names.

### Memory and Observability

- `aeqi recall "query" [--project NAME]`
- `aeqi remember KEY CONTENT [--project NAME]`
- `aeqi monitor`
- `aeqi audit [--project NAME] [--task TASK_ID] [--last N]`
- `aeqi blackboard list|post|query`

### Daemon and Scheduling

- `aeqi daemon start`
- `aeqi daemon stop`
- `aeqi daemon status`
- `aeqi daemon query CMD`
- `aeqi cron add|list|remove`

Useful daemon IPC commands:

- `ping`
- `status`
- `projects`
- `mail`
- `dispatches`
- `metrics`
- `cost`
- `audit`

### Operator Monitor

`aeqi monitor` is the native operator-facing summary for the current CLI.

- It merges daemon readiness data with local task-board inspection.
- It works even when the daemon is down, but then reports local state only.
- It emits recommended interventions for stalled work, critical ready work, dispatch failures, budget pressure, and missing repos.

Use:

- `aeqi monitor`
- `aeqi monitor --watch`
- `aeqi monitor --project NAME`
- `aeqi monitor --json`

## Directory Layout

```text
aeqi/
  config/
    aeqi.toml
    aeqi.example.toml
  agents/
    shared/WORKFLOW.md
    <agent>/
      agent.toml
      PERSONA.md
      IDENTITY.md
      OPERATIONAL.md
      PREFERENCES.md
      MEMORY.md
      EVOLUTION.md
      .tasks/
  projects/
    shared/
      skills/*.toml
      pipelines/*.toml
    <project>/
      AGENTS.md
      KNOWLEDGE.md
      HEARTBEAT.md
      skills/*.toml
      pipelines/*.toml
      rituals/*.toml
      .tasks/
      .aeqi/memory.db
```

## Configuration Model

Top-level sections in `aeqi.toml`:

- `[aeqi]`: workspace name, data dir, patrol interval
- `[providers.*]`: OpenRouter, Anthropic, Ollama configs
- `[aeqi].default_runtime`: default runtime preset name
- `runtime` on `[[projects]]` and `[[agents]]`: per-owner runtime override
- `[security]`: autonomy mode, workspace restriction, daily budget
- `[memory]`: backend and ranking parameters
- `[heartbeat]`: periodic heartbeats and reflections
- `[team]`: leader, advisor roster, router model, background budget
- `[[organizations]]`: reusable org graphs with units, roles, relationships, and rituals
- `team.org` / `team.unit` on `[[projects]]`: bind a project to an org unit
- `[session]`, `[context_budget]`, `[lifecycle]`, `[orchestrator]`: orchestration tuning
- `[repos]`: named repository pool
- `[[projects]]`: project definitions
- `[[watchdogs]]`: event-driven automation rules

Provider reality:

- Runtime presets now drive provider selection in the CLI and daemon for OpenRouter, Anthropic, and Ollama
- A few control-plane paths are still OpenRouter-oriented, especially advisor routing and usage-credit inspection

## Identity Assembly

Identity comes from files, not a large TOML schema.

Loaded agent-side files:

- `PERSONA.md`
- `IDENTITY.md`
- `OPERATIONAL.md`
- `PREFERENCES.md`
- `MEMORY.md`
- `EVOLUTION.md`
- `agents/shared/WORKFLOW.md`

Loaded project-side files:

- `AGENTS.md`
- `KNOWLEDGE.md`
- `HEARTBEAT.md`

This is the main prompt-building path for both the internal agent loop and Claude Code workers.

AEQI now also appends organizational context when an agent is mapped into an `[[organizations]]` graph. That context includes:

- organization and unit
- role title, mandate, goals, permissions, and budget authority
- manager, direct reports, peers, delegates, reviewers, advisors, escalation targets
- recurring rituals the agent owns or participates in

The intent is to keep the core modular: AEQI models graphs of responsibility and communication, not just a fixed CEO/CTO hierarchy.

When an agent participates in more than one organization, AEQI resolves org context in this order:

- explicit project binding through `team.org`
- default organization, if the agent belongs to it
- otherwise no single org context is injected, rather than guessing

## Persistence

Global data dir contents:

- `~/.aeqi/rm.pid`
- `~/.aeqi/rm.sock`
- `~/.aeqi/audit.db`
- `~/.aeqi/blackboard.db`
- `~/.aeqi/cost_ledger.jsonl`
- `~/.aeqi/dispatches/`
- `~/.aeqi/fate.json`
- `~/.aeqi/operations.json`
- `~/.aeqi/memory.db`

Task storage:

- `.tasks/<prefix>.jsonl`: append-only task records
- `.tasks/_missions.jsonl`: append-only mission records

## Extension Points

Traits live under `crates/aeqi-core/src/traits/`.

- `Provider`
- `Tool`
- `Memory`
- `Observer`
- `Channel`
- `Embedder`

Useful extension targets:

- New provider: `crates/aeqi-providers/`
- New tool: `crates/aeqi-tools/`
- New daemon surface: `aeqi-cli/src/cmd/daemon.rs` and `crates/aeqi-orchestrator/src/daemon.rs`
- New orchestration policy: `crates/aeqi-orchestrator/src/worker_pool.rs`

## Current Boundaries

- No dedicated `aeqi council` subcommand
- No dedicated `aeqi cost` subcommand
- Council routing is daemon-driven, most visibly from Telegram `/council ...`
- Cost inspection is daemon-driven via `aeqi daemon query cost`
- Readiness inspection is daemon-driven via `aeqi daemon query readiness`
- `aeqi monitor` is available as a native operator summary, but there is still no first-class TUI or web console
- Daemon service install/print/uninstall is available via `aeqi daemon ...`
- `aeqi team`, `aeqi status`, and `aeqi agent list` expose the current organization model and resolved org context
- Claude Code recursion requires an external `claude` installation and authentication

For the planned full-screen terminal shell, see [docs/chat-interface.md](/home/claudedev/aeqi/docs/chat-interface.md).

## Recommended Validation

```bash
aeqi doctor --strict
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
