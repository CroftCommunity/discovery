#!/usr/bin/env bash
# local-up.sh [--down]
#
# Bring up the full local stack (every tenant's stub + api, litestream to a
# file:// replica, rclone to a local R2 stand-in, Caddy on high ports over plain
# HTTP) and leave it running for manual inspection. `--down` tears it down.
#
# This is the `make local-up` target. The fire drill (fire-drill.sh) uses the
# same lib.sh machinery for its destroy/restore cycle.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=drill/lib.sh
source "$HERE/lib.sh"

if [[ "${1:-}" == "--down" ]]; then
  stop_stack
  echo "local stack stopped."
  exit 0
fi

DRILL_FAILED=0
mk_workspace
start_stack

echo "== local stack up =="
while IFS=$'\t' read -r name _u api_user port api_port _art _am; do
  [[ -z "$name" ]] && continue
  echo "  $name          http://127.0.0.1:$((port + CADDY_OFFSET))/healthz  (caddy) -> :$port"
  [[ "$api_user" != "-" ]] && echo "  $name-api      http://127.0.0.1:$api_port/healthz"
done < <(tenants)
echo
echo "litestream -> $REPLICAS   rclone -> $R2_STANDIN   logs -> $LOGDIR"
echo "tear down with: make local-down"
[[ "$DRILL_FAILED" -eq 0 ]] || { echo "one or more components failed to start" >&2; exit 1; }
