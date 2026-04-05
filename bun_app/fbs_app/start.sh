#!/usr/bin/env bash
# FBS Stock App launcher
# Usage: bash bun_app/fbs_app/start.sh [--kill]
#
# Options:
#   --kill    Kill existing server on port 3200 before starting
#   --stop    Just stop the server, don't restart
set -euo pipefail

PORT="${FBS_PORT:-3200}"
ENV_FILE="$(dirname "$0")/../../.env"
APP_DIR="$(cd "$(dirname "$0")" && pwd)"
PID=""

find_pid() {
  PID=$(netstat -ano 2>/dev/null | grep ":${PORT}.*LISTENING" | awk '{print $5}' | head -1)
}

stop_server() {
  find_pid
  if [ -n "$PID" ]; then
    echo "Killing server on port $PORT (PID $PID)..."
    taskkill //F //PID "$PID" 2>/dev/null || true
    sleep 1
  else
    echo "No server on port $PORT"
  fi
}

if [ "${1:-}" = "--stop" ]; then
  stop_server
  exit 0
fi

if [ "${1:-}" = "--kill" ]; then
  stop_server
fi

# Check if already running
find_pid
if [ -n "$PID" ]; then
  echo "Server already running on port $PORT (PID $PID)"
  echo "Open: http://localhost:$PORT"
  exit 0
fi

echo "Starting FBS Stock App on port $PORT..."
cd "$APP_DIR"
bun --env-file="$ENV_FILE" src/index.ts &
SERVER_PID=$!
cd - > /dev/null

# Wait for server
for i in $(seq 1 10); do
  if curl -s -o /dev/null "http://localhost:$PORT" 2>/dev/null; then
    echo "Server ready: http://localhost:$PORT (PID $SERVER_PID)"
    exit 0
  fi
  sleep 1
done

echo "ERROR: Server failed to start within 10s"
exit 1
