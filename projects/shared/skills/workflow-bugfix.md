```toml
[skill]
name = "workflow-bugfix"
description = "Workflow for bug fixes and single-issue patches"
phase = "workflow"
```

# Bugfix Workflow

Execute in order.

## 1. Setup
```
sigil_create_task(project, subject)
sigil_recall(project, query) — check if this bug was seen before
```

## 2. Diagnose
```
sigil_graph(action="search", project, query) — find the relevant symbol
sigil_graph(action="context", project, node_id) — callers, callees, understand the flow
sigil_skills(action="list") → load relevant domain skill if available
```

## 3. Fix
```
sigil_graph(action="file", project, file_path) — understand the file before editing
```
Write the fix. Run tests.

## 4. Verify
```
sigil_graph(action="impact", project, node_id) — verify fix doesn't break callers
```
Run full test suite: `cargo test` / `npm test`.

## 5. Close
```
sigil_remember(project, key, content, category) — store what caused the bug and how it was fixed
sigil_close_task(task_id)
```
