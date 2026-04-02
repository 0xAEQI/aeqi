# Architecture Overview

AEQI has three main layers:

1. Agent runtime
2. Orchestration and persistence
3. Operator interfaces

## System Map

```text
Operator
  |-> CLI (`aeqi`)
  |-> Web UI (`apps/ui`)
  |-> API / WebSocket (`aeqi-web`)
                |
                v
        Orchestrator (`aeqi-orchestrator`)
                |
                +-> agent runtime and tools
                +-> task boards and missions
                +-> memory and retrieval
                +-> audit, dispatches, watchdogs, lifecycle
                |
                v
        Providers (`aeqi-providers`)
```

## Runtime Layer

The runtime is responsible for direct agent execution: prompt assembly, tools, middleware, retries, and outcome parsing.

## Orchestration Layer

The orchestration layer routes work, supervises workers, tracks task state, persists memory, and decides when to retry, escalate, or learn from outcomes.

## Interface Layer

- `aeqi` is the CLI and daemon entrypoint
- `aeqi-web` exposes the API and browser-facing surface
- `apps/ui` is the operator control plane

## Repository Shape

```text
apps/ui/                 frontend
crates/aeqi-core/       shared config and traits
crates/aeqi-memory/     retrieval and persistence
crates/aeqi-orchestrator/
crates/aeqi-providers/
crates/aeqi-tasks/
crates/aeqi-tools/
crates/aeqi-web/
aeqi-cli/
```
