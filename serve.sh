#!/usr/bin/env bash
# Serve the workflow walkthrough site. Usage: ./serve.sh [port]
set -euo pipefail

PORT="${1:-8000}"
cd "$(dirname "$0")/site"

echo "Serving the workflow walkthrough at http://localhost:${PORT}"
exec python3 -m http.server "$PORT"
