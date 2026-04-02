# AEQI Architecture

## Summary

AEQI is a Rust workspace with two practical execution planes:

- The internal agent plane, used by `aeqi run` and `aeqi skill run`
- The orchestration plane, used by `aeqi daemon start`

On top of those planes, the main native operator surface is now `aeqi monitor`, which combines daemon IPC state with local task-board inspection.

The codebase already contains more orchestration primitives than the top-level CLI exposes directly. This document focuses on the runtime paths operators can use today.

## Crate Layers

```text
aeqi-cli
  -> aeqi-core
  -> aeqi-tasks
  -> aeqi-memory
  -> aeqi-providers
  -> aeqi-tools
  -> aeqi-orchestrator
  -> aeqi-gates
```

- `aeqi-core`: traits, config loading, identity assembly, internal agent loop, secret store
- `aeqi-tasks`: task DAG, missions, dependency inference, JSONL persistence
- `aeqi-memory`: SQLite memory with FTS5, embedding cache, hybrid ranking
- `aeqi-providers`: OpenRouter, Anthropic, Ollama clients plus model pricing
- `aeqi-tools`: shell, file, git, task, skill, and other agent tools
- `aeqi-orchestrator`: daemon, worker pools, workers, dispatch bus, cost ledger, audit log, blackboard, schedules
- `aeqi-gates`: channel adapters such as Telegram

## Runtime Path 1: One-Shot CLI Execution

`aeqi run` and `aeqi skill run` both use the internal `aeqi-core::Agent` loop.

```text
CLI command
  -> load config
  -> build provider
  -> build tool set
  -> load identity
  -> attach optional memory
  -> provider chat loop with tool execution
```

Key properties:

- Provider selection follows the project runtime for project-scoped one-shot runs, or the standalone leader/runtime when no project is selected
- Tool execution happens inside AEQI, not through Claude Code
- Memory recall is injected into the system prompt before the loop starts
- This path is the simplest way to work on providers, tools, identity, and memory behavior

## Runtime Path 2: Daemon Orchestration

`aeqi daemon start` builds a long-running registry and patrol loop.

```text
daemon start
  -> load config + merge agents from disk
  -> init dispatch bus, cost ledger, audit log, blackboard
  -> register projects
  -> register advisor agents as supervised task owners
  -> patrol ready work
  -> launch workers
  -> persist state + serve IPC
```

The daemon owns:

- Project registration and per-project worker pools
- Advisor-agent registration
- Audit log, blackboard, dispatch bus, cost ledger, schedule store
- Telegram ingress and council routing
- IPC queries on `~/.aeqi/rm.sock`

Useful daemon probes today:

- `aeqi monitor`: operator-focused summary plus recommended interventions
- `aeqi daemon query status`: broad inventory of projects, budgets, pulses, and dispatch state
- `aeqi daemon query readiness`: stricter control-plane readiness, including skipped registrations, worker capacity, and budget exhaustion

`aeqi monitor` is not a separate runtime. It is a CLI aggregation layer over:

- daemon readiness / dispatch / budget state when the daemon is live
- local project task boards and repo presence checks even when the daemon is down

## Worker Execution Modes

Projects and advisor agents can run in either mode:

- `agent`: internal provider + tool loop
- `claude_code`: external Claude Code subprocess managed by `ClaudeCodeExecutor`

In practice, the daemon code is set up to use Claude Code for the long-running worker path when configured.

### Claude Code Flow

```text
WorkerPool
  -> AgentWorker
  -> ClaudeCodeExecutor
  -> external `claude` process
  -> stream-json events
  -> TaskOutcome (Done, Blocked, Failed, Handoff)
```

Important details:

- Workers run with `--permission-mode bypassPermissions`
- State is not session-persistent
- Checkpoints are recorded outside the worker from repository state
- Cost is only reliably available on the final result event from Claude Code

## Identity and Context Assembly

AEQI separates agent identity from project context.

Agent-side files:

- `PERSONA.md`
- `IDENTITY.md`
- `OPERATIONAL.md`
- `PREFERENCES.md`
- `MEMORY.md`
- `EVOLUTION.md`
- shared `agents/shared/WORKFLOW.md`

Project-side files:

- `AGENTS.md`
- `KNOWLEDGE.md`
- `HEARTBEAT.md`

System prompt order from `aeqi-core/src/identity.rs`:

1. Shared workflow
2. Persona
3. Identity
4. Evolution
5. Operational instructions
6. Project operating instructions
7. Project knowledge
8. Preferences
9. Persistent memory

Claude Code workers receive that identity plus the worker protocol that defines `DONE`, `BLOCKED:`, `FAILED:`, and `HANDOFF:`.

### Organization Kernel

`aeqi-core::AEQIConfig` now also supports a first-class organization graph:

- `organizations`
- `units`
- `roles`
- `relationships`
- `rituals`

This is deliberately more general than a corporate title tree. A AEQI organization can represent a company, open-source maintainer group, incident cell, research lab, or any other operating structure.

At runtime, AEQI resolves:

- org-linked project teams through `team.org` and `team.unit`
- per-agent org context through roles, unit membership, relationships, and ritual participation
- hierarchy-aware prompt context via the identity `operational` section

If an agent belongs to multiple organizations, AEQI now prefers the explicitly bound project org, otherwise the default organization when that agent is a member, and otherwise omits single-org context instead of picking an arbitrary first match.

That gives the current runtime a native way to express leaders, peers, direct reports, advisors, reviewers, and escalation paths without hardcoding a specific org chart into the core.

## State and Persistence

Global state under `~/.aeqi/`:

- `rm.pid`: daemon PID
- `rm.sock`: daemon IPC socket
- `audit.db`: decision audit trail
- `blackboard.db`: blackboard entries
- `cost_ledger.jsonl`: cost accounting
- `dispatches/`: persisted dispatch queue state
- `fate.json`: cron jobs
- `operations.json`: cross-project operation state
- `memory.db`: global memory

Per-project or per-agent state:

- `.tasks/<prefix>.jsonl`: task streams for each prefix
- `.tasks/_missions.jsonl`: mission storage
- `.aeqi/memory.db`: project memory database

## Shared Assets

Shared reusable assets live under `projects/shared/`.

- `projects/shared/skills/*.toml`
- `projects/shared/pipelines/*.toml`

The CLI now merges shared assets with project-local ones:

- shared assets load first
- project-local assets override on name collisions

This is the intended place for reusable workflows that should not be copied into each project directory.

## Public Surface vs Internal Capability

Some orchestration capabilities exist in code but are not yet first-class CLI commands.

Examples:

- Council mode exists in the daemon message path and Telegram flow, not as `aeqi council`
- Budget inspection exists through daemon IPC, not as `aeqi cost`
- `aeqi monitor` is the current native operator summary, but not yet a full TUI or web console
- Worker/provider runtime presets now select OpenRouter, Anthropic, or Ollama per project or agent
- Organization graphs are first-class config and identity input, but per-role chat surfaces are not a dedicated top-level CLI yet
- Agent routing and usage-credit inspection still assume OpenRouter in a few control-plane paths

When documenting or extending AEQI, treat the daemon and CLI entrypoints as the source of truth for what operators can use today.

For the proposed terminal operator shell that builds on these primitives, see [docs/chat-interface.md](/home/claudedev/aeqi/docs/chat-interface.md).

## Best Places To Extend

- Provider routing: `aeqi-cli/src/helpers.rs`
- New tools: `crates/aeqi-tools/` plus the relevant tool builders
- Worker behavior: `crates/aeqi-orchestrator/src/agent_worker.rs` and `executor.rs`
- Orchestration policy: `crates/aeqi-orchestrator/src/worker_pool.rs` and `registry.rs`
- Identity assembly: `crates/aeqi-core/src/identity.rs`
- CLI surface: `aeqi-cli/src/cli.rs` and `aeqi-cli/src/cmd/`
