```toml
[skill]
name = "workflow-feature"
description = "Full workflow for new features and multi-file changes"
phase = "workflow"
```

# Feature Workflow

Execute in order. Every step is a tool call.

## 1. Setup
```
sigil_create_task(project, subject)
sigil_recall(project, query)
sigil_skills(action="list") → pick relevant → sigil_skills(action="get", name="<match>")
```

## 2. Research
```
sigil_delegate(agent="researcher", project, task_id)
```
Spawn as a Claude Code Agent subagent with the returned prompt. Wait for it to finish.
Then read its findings:
```
sigil_blackboard(action="read", project, prefix="task:<id>")
```

## 3. Plan
```
sigil_graph(action="impact", project, node_id=<key symbol>) — blast radius
sigil_blackboard(action="post", project, key="task:<id>:plan", content=<your plan>)
```
For complex changes, delegate planning:
```
sigil_delegate(agent="architect", project, task_id)
```
Read the architect's plan from blackboard.

## 4. Implement
Before editing each file:
```
sigil_graph(action="file", project, file_path)
```
Write code. Run tests after each logical change.

## 5. Review
```
sigil_delegate(agent="reviewer", project, task_id)
```
Spawn as subagent. Wait for it to finish. Read verdict:
```
sigil_blackboard(action="read", project, prefix="task:<id>:review")
```
If FAIL: address issues and re-delegate review.

## 6. Close
```
sigil_graph(action="search", project, query="<new symbols>") — verify new code has callers
sigil_remember(project, key, content, category)
sigil_close_task(task_id)
```
