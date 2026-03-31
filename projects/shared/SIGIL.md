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

## Delegation Rules (MANDATORY)

When spawning subagents for research, review, implementation, or any focused work:

1. **Use `sigil_delegate`** to get the proper agent template — never spawn a raw Agent with an inline prompt
2. **Subagents MUST post findings to blackboard** — `sigil_blackboard(action="post", key="task:{id}:{phase}", ...)`
3. **Read blackboard after delegation** — `sigil_blackboard(action="query", project)` to get subagent findings
4. **Use phase-appropriate agents** — `sigil_agents(action="list", phase="discover")` to see available templates

```
# Correct delegation pattern:
sigil_agents(action="list", phase="discover")     # see available agents
sigil_delegate(agent="researcher", project, task_id)  # get template prompt
# → spawn Agent with the returned prompt
# → subagent posts to blackboard automatically
sigil_blackboard(action="query", project)          # read findings
```

Do NOT spawn anonymous agents with ad-hoc prompts. The templates include blackboard posting instructions, phase-specific skills, and proper tool scoping.

## Rules

- No comments except `///` on public APIs.
- No backward compat hacks. Change everywhere or don't.
- Same concept = same name everywhere.
- DRY. Extract at two, refactor at three.
- Schema = source of truth.
- Worktrees only. Never edit dev/master.
- No secrets in git.
- Full autonomy. BLOCKED = missing credential, unresolvable failure, or competing paths needing human choice. Nothing else.
