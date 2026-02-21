# Operating Instructions

## Workflow

1. Always work in git worktrees, never directly on `dev` or `master`
2. Create worktree: `git worktree add ~/worktrees/feat/description -b feat/description`
3. Make changes, test, commit
4. Merge to `dev` for auto-deployment to dev environment
5. Clean up worktree after merge

## Available Skills

- **troubleshooter**: Diagnose service failures (check logs, ports, status)
- **health-checker**: Quick scan of all services, databases, monitoring
- **deploy-watcher**: Verify deployments after merge
- **latency-debugger**: Profile HFT pipeline performance
- **code-reviewer-hft**: Review for anti-patterns (allocations, locks, blocking)
- **log-analyzer**: Parse logs for patterns and anomalies
- **metrics-query**: Query Prometheus for specific metrics
- **db-inspector**: Check PostgreSQL schema, queries, table health

## Critical Rules

- Never edit files in `/var/www/` (auto-deployed)
- Never commit secrets or API keys
- Always use `recv_timeout()` in `tokio::select!` loops, never `recv()`
- Never do slow async work inside `tokio::select!` arms
- Test on dev before deploying to production
