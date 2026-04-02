# AEQI

## For Every Task

1. `aeqi_recall(project, query)` — gate-enforced before any edit
2. Load a workflow skill from the list below — it contains all steps including task creation
3. Follow the loaded workflow step by step

Trivial fixes (typos, one-line config): `aeqi_recall` → edit → `aeqi_remember` → done.

## Tools

| Tool | Purpose |
|------|---------|
| `aeqi_recall(project, query)` | Search institutional memory |
| `aeqi_remember(project, key, content, category)` | Store learnings |
| `aeqi_skills(action, name)` | Load domain knowledge and workflows |
| `aeqi_graph(action, project, ...)` | Code structure: search, context, impact, file |
| `aeqi_delegate(agent, project, task_id)` | One-call agent delegation (returns prompt for subagent) |
| `aeqi_blackboard(action, project, ...)` | Coordination: post findings, claim files, read context |
| `aeqi_create_task(project, subject)` | Track work |
| `aeqi_close_task(task_id)` | Close task (triggers integration check) |
| `aeqi_agents(action, phase)` | Browse available agents by phase |

## Delegation Rules (MANDATORY)

When spawning subagents for research, review, implementation, or any focused work:

1. **Use `aeqi_delegate`** to get the proper agent template — never spawn a raw Agent with an inline prompt
2. **Subagents MUST post findings to blackboard** — `aeqi_blackboard(action="post", key="task:{id}:{phase}", ...)`
3. **Read blackboard after delegation** — `aeqi_blackboard(action="query", project)` to get subagent findings
4. **Use phase-appropriate agents** — `aeqi_agents(action="list", phase="discover")` to see available templates

```
# Correct delegation pattern:
aeqi_agents(action="list", phase="discover")     # see available agents
aeqi_delegate(agent="researcher", project, task_id)  # get template prompt
# → spawn Agent with the returned prompt
# → subagent posts to blackboard automatically
aeqi_blackboard(action="query", project)          # read findings
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
