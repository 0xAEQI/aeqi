# Shared Workflow

These rules apply to ALL rigs. Rig-specific AGENTS.md may add to but never contradict these.

## Git Workflow

1. **Always work in worktrees** — never edit `dev` or `master` directly
2. Create worktree: `git worktree add ~/worktrees/feat/<name> -b feat/<name>`
3. Work, test, commit in the worktree
4. Merge to `dev` for auto-deploy to dev environment
5. Test on dev, then merge `dev` → `master` for production
6. Cleanup: `git worktree remove ~/worktrees/feat/<name> && git branch -d feat/<name>`

**Exception:** Sigil framework itself works directly on `master` (no worktree).

## Code Standards

| Rule | Rationale |
|------|-----------|
| NO COMMENTS | Code is self-documenting. `//!` and `///` on public APIs only. |
| NO BACKWARD COMPATIBILITY HACKS | No `_unused`, no `#[deprecated]`, no shims. Change everywhere or don't. |
| CONSISTENT NAMING | Same concept = same name across entire codebase. |
| DRY → SHARED CODE | See a pattern twice? Extract it. Three places = refactor. |
| BEST IMPLEMENTATION ONLY | Find the optimal approach. No "good enough". |
| SCHEMA = SOURCE OF TRUTH | DB changes must update schema files. Fresh setup must work. |

## Worker Protocol

When executing a bead (task), workers must signal their outcome:

- **Completed**: Provide a clear summary of what changed (files, commits, deployments)
- **BLOCKED:** prefix: Need a decision or information. State the specific question. Be precise — it gets passed to another agent or human.
- **FAILED:** prefix: Technical error (build failure, test failure). Include error output and what was tried.

## Sub-Agent Orchestration

Workers have full access to Claude Code's Task tool. Each worker IS an orchestrator.

For complex tasks, follow the **R→D→R pipeline** (Research → Develop → Review):

1. **Research**: Spawn an Explore agent to map relevant code, find patterns, identify constraints
2. **Develop**: Implement based on research findings. Work in worktree, commit.
3. **Review**: Spawn a review agent to check for anti-patterns, security issues, correctness

Simple tasks (single-file fix, config change) don't need the full pipeline — just do the work.

## Escalation

If you genuinely cannot determine something from the codebase:
1. First try harder — check docs, configs, related code, git history
2. If truly stuck, respond with `BLOCKED:` and a specific question
3. The Witness will attempt rig-level resolution (spawn another worker with your question)
4. If still stuck, escalates to Familiar (cross-rig knowledge)
5. If Familiar can't resolve, escalates to human via Telegram

## Safety

- Never commit secrets or API keys to git
- Never edit files in `/var/www/` (auto-deployed, read-only)
- Never deploy to production without testing on dev first
- Never trust client-side values for server-side operations
