#!/usr/bin/env bash
# PostToolUse hook for mcp__sigil__sigil_recall: open the recall gate.
# Writes the recalled project name to the gate file.
# Any subsequent recall for a different project updates the gate.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/hook-log.sh"

# Extract project from the tool input
PROJECT=""
if [ -n "${CLAUDE_TOOL_INPUT:-}" ]; then
    PROJECT=$(printf '%s' "$CLAUDE_TOOL_INPUT" | jq -r '.project // empty' 2>/dev/null) || true
fi
[ -z "$PROJECT" ] && PROJECT="sigil"

printf '%s' "$PROJECT" > "$SIGIL_SESSION_DIR/recall.gate"
log_hook "mark-recall" "touch" "project=$PROJECT"
