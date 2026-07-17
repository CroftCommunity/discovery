#!/usr/bin/env bash
# render.sh — thin entry point for the generator (scripts/render.py).
# Runs from the kit root; defaults to services/ -> generated/.
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$here/render.py" "$@"
