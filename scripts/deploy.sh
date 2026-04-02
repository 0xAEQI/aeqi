#!/usr/bin/env bash
# CI/CD deploy script — build release binary and restart services.
# Called manually or by git post-merge hook.
#
# Usage: ./scripts/deploy.sh [--no-restart]

set -euo pipefail

AEQI_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$AEQI_ROOT"

echo "[deploy] Building release binary..."
cargo build --release -p aeqi 2>&1 | tail -3

# Build and deploy landing page if changed.
LANDING_DIR="$AEQI_ROOT/apps/landing"
if [ -d "$LANDING_DIR" ] && [ -f "$LANDING_DIR/package.json" ]; then
    echo "[deploy] Building landing page..."
    (cd "$LANDING_DIR" && npm run build --silent 2>&1 | tail -3)
    echo "[deploy] Deploying landing page to /var/www/aeqi-ai..."
    sudo rsync -a --delete "$LANDING_DIR/dist/" /var/www/aeqi-ai/
fi

if [[ "${1:-}" == "--no-restart" ]]; then
    echo "[deploy] Build complete (restart skipped)."
    exit 0
fi

echo "[deploy] Restarting aeqi-daemon..."
sudo systemctl restart aeqi-daemon
sleep 3

echo "[deploy] Restarting aeqi-web..."
sudo systemctl restart aeqi-web
sleep 2

# Verify
DAEMON_STATUS=$(systemctl is-active aeqi-daemon 2>/dev/null || echo "failed")
WEB_STATUS=$(systemctl is-active aeqi-web 2>/dev/null || echo "failed")

echo "[deploy] daemon: $DAEMON_STATUS | web: $WEB_STATUS"

if [[ "$DAEMON_STATUS" == "active" && "$WEB_STATUS" == "active" ]]; then
    echo "[deploy] Deploy successful."

    # Reindex graph after deploy
    if command -v aeqi &>/dev/null; then
        aeqi graph index -r aeqi 2>/dev/null &
        echo "[deploy] Graph reindex started in background."
    fi
else
    echo "[deploy] WARNING: One or more services failed to start!"
    exit 1
fi
