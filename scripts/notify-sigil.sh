#!/usr/bin/env bash
# Notification hook: posts Claude Code notifications to Sigil blackboard.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/hook-log.sh"
source "$SCRIPT_DIR/detect-project.sh"

SOCK="${SIGIL_DATA_DIR:-$HOME/.sigil}/rm.sock"
if [ ! -S "$SOCK" ]; then
    log_hook "notify-sigil" "skip" "daemon-down"
    exit 0
fi

PROJECT=$(detect_project)
[ -z "$PROJECT" ] && PROJECT="sigil"

TIMESTAMP=$(date -u +%Y%m%d-%H%M%S)

printf '{"cmd":"post_blackboard","project":"%s","key":"signal:notification:%s","content":"Claude Code session notification in project %s","tags":["notification","claude-code"],"durability":"transient"}' \
    "$PROJECT" "$TIMESTAMP" "$PROJECT" \
    | socat -t2 - UNIX-CONNECT:"$SOCK" >/dev/null 2>&1 || true

log_hook "notify-sigil" "posted" "project=$PROJECT"
