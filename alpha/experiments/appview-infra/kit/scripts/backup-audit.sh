#!/usr/bin/env bash
# backup-audit.sh — entry point for the state-taxonomy audit (backup-audit.py).
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "$here/backup-audit.py" "$@"
