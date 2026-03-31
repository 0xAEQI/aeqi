# Sigil

## For Every Task

1. `sigil_recall(project, query)` — gate-enforced before any edit
2. Load a workflow skill from the list below — it contains all steps including task creation
3. Follow the loaded workflow step by step

Trivial fixes (typos, one-line config): `sigil_recall` → edit → `sigil_remember` → done.

## Tools

| Tool | Purpose |
|------|---------|
| `sigil_recall(project, query)` | Search institutional memory |
| `sigil_remember(project, key, content, category)` | Store learnings |
| `sigil_skills(action, name)` | Load domain knowledge and workflows |
| `sigil_graph(action, project, ...)` | Code structure: search, context, impact, file |
| `sigil_delegate(agent, project, task_id)` | One-call agent delegation (returns prompt for subagent) |
| `sigil_blackboard(action, project, ...)` | Coordination: post findings, claim files, read context |
| `sigil_create_task(project, subject)` | Track work |
| `sigil_close_task(task_id)` | Close task (triggers integration check) |
| `sigil_agents(action, phase)` | Browse available agents by phase |

## Rules

- No comments except `///` on public APIs.
- No backward compat hacks. Change everywhere or don't.
- Same concept = same name everywhere.
- DRY. Extract at two, refactor at three.
- Schema = source of truth.
- Worktrees only. Never edit dev/master.
- No secrets in git.
- Full autonomy. BLOCKED = missing credential, unresolvable failure, or competing paths needing human choice. Nothing else.
