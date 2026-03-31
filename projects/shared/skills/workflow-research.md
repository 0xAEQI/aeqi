```toml
[skill]
name = "workflow-research"
description = "Workflow for investigation and research tasks — no code changes"
phase = "workflow"
```

# Research Workflow

Execute in order.

## 1. Setup
```
sigil_create_task(project, subject)
sigil_recall(project, query)
sigil_skills(action="list") → load domain knowledge
```

## 2. Investigate
```
sigil_graph(action="search", project, query) — find relevant symbols
sigil_graph(action="context", project, node_id) — understand relationships
sigil_delegate(agent="researcher", project, task_id) — delegate deep research
```

## 3. Synthesize
```
sigil_blackboard(action="read", project, prefix="task:<id>") — gather all findings
sigil_blackboard(action="post", project, key="task:<id>:research", content=<synthesis>)
```

## 4. Close
```
sigil_remember(project, key, content, category) — store findings as institutional knowledge
sigil_close_task(task_id)
```
