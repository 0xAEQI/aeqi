```toml
[skill]
name = "workflow-refactor"
description = "Workflow for restructuring and refactoring existing code"
phase = "workflow"
```

# Refactor Workflow

Execute in order.

## 1. Setup
```
sigil_create_task(project, subject)
sigil_recall(project, query)
sigil_skills(action="list") → load architecture skills
```

## 2. Analyze Impact
```
sigil_graph(action="search", project, query) — find all symbols to change
sigil_graph(action="impact", project, node_id) — blast radius for each
sigil_graph(action="context", project, node_id) — all callers that need updating
```

## 3. Plan
```
sigil_blackboard(action="post", project, key="task:<id>:plan", content=<refactoring plan with affected files>)
sigil_delegate(agent="architect", project, task_id) — validate the approach
```

## 4. Implement
```
sigil_graph(action="file", project, file_path) — before each file
```
Change all call sites. Run tests after each file.

## 5. Verify
```
sigil_graph(action="impact", project, node_id) — re-check blast radius post-change
sigil_delegate(agent="reviewer", project, task_id) — verify consistency
```
Full test suite.

## 6. Close
```
sigil_remember(project, key, content, category)
sigil_close_task(task_id)
```
