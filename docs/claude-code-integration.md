# Claude Code + Sigil Integration

Add this section to your project's `CLAUDE.md` to integrate with Sigil.

## Setup

1. Build `sg`: `cd /home/claudedev/sigil && cargo build --release`
2. Symlink: `sudo ln -sf /home/claudedev/sigil/target/release/sg /usr/local/bin/sg`
3. Start daemon: `sg daemon start` (or run in background)

## Commands for Claude Code

When working with tasks and orchestration, use these `sg` commands:

```bash
# Check what work is ready across all rigs
sg ready

# Check work for a specific rig
sg ready --rig algostaking

# Assign a new task to a rig
sg assign "fix the login bug" --rig algostaking --priority high

# Close a completed task
sg done as-001 --reason "fixed in commit abc123"

# Search collective memory
sg recall "how does the auth system work?" --rig algostaking

# Store a learning for future sessions
sg remember "zmq-pattern" "Always use recv_timeout in tokio::select, never recv()" --rig algostaking

# Start a workflow
sg mol pour feature-dev --rig algostaking --var issue_id=as-001

# Check molecule progress
sg mol status as-001

# Query daemon status
sg daemon query status

# Run a skill
sg skill run troubleshooter --rig algostaking --prompt "API returning 500 errors"

# Full system health
sg status
sg doctor
```

## IPC Socket

When the daemon is running, it listens on `~/.sigil/sg.sock` for JSON-line queries.
This allows programmatic access from any process:

```bash
# Example: query daemon status via socat
echo '{"cmd":"status"}' | socat - UNIX-CONNECT:~/.sigil/sg.sock

# Or via sg itself
sg daemon query ping
sg daemon query status
sg daemon query rigs
sg daemon query mail
```

## Mapping Claude Code Agents to Sigil Skills

Existing Claude Code agents can be run as Sigil skills:

| Claude Code Agent | Sigil Skill | Command |
|-------------------|-------------|---------|
| troubleshooter | troubleshooter | `sg skill run troubleshooter --rig algostaking` |
| health-checker | health-checker | `sg skill run health-checker --rig algostaking` |
| deploy-watcher | deploy-watcher | `sg skill run deploy-watcher --rig algostaking` |
| latency-debugger | latency-debugger | `sg skill run latency-debugger --rig algostaking` |
| log-analyzer | log-analyzer | `sg skill run log-analyzer --rig algostaking` |
| metrics-query | metrics-query | `sg skill run metrics-query --rig algostaking` |
| db-inspector | db-inspector | `sg skill run db-inspector --rig algostaking` |
| code-reviewer-hft | code-reviewer | `sg skill run code-reviewer --rig algostaking` |

## Workflow

```
Human (Claude Code)           Sigil Daemon
      |                            |
      | sg assign "task" --rig X   |
      |--------------------------->|
      |                            | Familiar routes to Rig X
      |                            | Witness spawns Worker
      |                            | Worker executes via OpenRouter
      |                            |
      | sg ready --rig X           |
      |<---------------------------|
      |                            |
      | sg done bead-id            |
      |--------------------------->|
      |                            | Updates convoys, unblocks deps
```
