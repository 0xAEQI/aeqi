```toml
[skill]
name = "workflow-synthesis"
description = "Synthesize improvements from external sources into Sigil — for both Claude Code harness and native runtime paths"
phase = "workflow"
```

# Synthesis Workflow

For taking an external source (codebase, paper, tool, framework) and synthesizing improvements into Sigil.

## Context

Sigil has two parallel improvement paths:
- **Path A (harness):** Claude Code + Sigil MCP integration — improve the productivity setup used NOW
- **Path B (runtime):** Sigil's native agent orchestrator — the destination that replaces Claude Code

External sources may inform either or both paths. Do not blindly copy. Understand, compare, synthesize.

## Steps

### 1. Understand the External Source
```
Read the external source thoroughly (code, docs, architecture).
Post findings: sigil_blackboard(action="post", project, key="task:<id>:external-analysis", content=<structured analysis>)
```
Extract: architecture decisions, novel patterns, tool interactions, coordination mechanisms.

### 2. Understand Sigil's Current State
```
sigil_recall(project="sigil", query=<relevant area>)
sigil_graph(action="search", project="sigil", query=<relevant symbols>)
sigil_skills(action="get", name="rust-architect")
```
Map Sigil's existing equivalent for every feature found in the external source.

### 3. Gap Analysis
For each capability in the external source, classify:
- **Already better in Sigil** — skip (don't regress)
- **Missing in Sigil, valuable** — candidate for adoption
- **Present in Sigil but weaker** — candidate for improvement
- **Present in external but unnecessary** — skip (don't bloat)

Post analysis: `sigil_blackboard(action="post", project, key="task:<id>:gap-analysis", content=<classification>)`

### 4. Classify by Path
For each improvement candidate:
- **Path A only** — improves Claude Code harness (hooks, primer, settings)
- **Path B only** — improves native runtime (agent loop, middleware, supervisor)
- **Both paths** — improves shared infrastructure (skills, graph, memory, blackboard)

### 5. Prioritize
Order by: impact × effort. Prefer changes that improve BOTH paths over single-path improvements.

### 6. Implement
Load the appropriate workflow for each improvement:
- `workflow-feature` for new capabilities
- `workflow-refactor` for restructuring existing code
- `workflow-bugfix` for fixing identified weaknesses

### 7. Verify
```
sigil_delegate(agent="reviewer", project, task_id)
```
Review must confirm: no regression in existing features, new code is integrated (not dead), both paths still work.
