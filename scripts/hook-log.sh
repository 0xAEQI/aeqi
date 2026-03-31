#!/usr/bin/env bash
# Telemetry helper — sourced by other hook scripts.
# Provides log_hook() with elapsed-time tracking and error resilience.

SIGIL_TELEMETRY_FILE="/tmp/.sigil_hook_telemetry"
SIGIL_SESSION_DIR="${HOME}/.sigil/session"
SIGIL_CONFIG="${SIGIL_CONFIG:-/home/claudedev/sigil/config/sigil.toml}"

mkdir -p "$SIGIL_SESSION_DIR" 2>/dev/null || true

# Capture start time (nanoseconds) at source time — every hook gets a timer.
_HOOK_START_NS=$(date +%s%N 2>/dev/null || echo 0)

log_hook() {
    local hook_name="$1" decision="$2" detail="${3:-}"
    local now_ns elapsed_ms=""
    now_ns=$(date +%s%N 2>/dev/null || echo 0)
    if [ "$_HOOK_START_NS" != "0" ] && [ "$now_ns" != "0" ]; then
        elapsed_ms=$(( (now_ns - _HOOK_START_NS) / 1000000 ))
    fi
    if [ -n "$elapsed_ms" ]; then
        printf '%s %s %s %sms %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S)" "$hook_name" "$decision" "$elapsed_ms" "$detail" >> "$SIGIL_TELEMETRY_FILE" 2>/dev/null || true
    else
        printf '%s %s %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S)" "$hook_name" "$decision" "$detail" >> "$SIGIL_TELEMETRY_FILE" 2>/dev/null || true
    fi
}
