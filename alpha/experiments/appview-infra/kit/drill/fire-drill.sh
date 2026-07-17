#!/usr/bin/env bash
# fire-drill.sh --variant local
#
# The pre-purchase capstone. Brings up the full local stack, plants canonical +
# blob markers, backs them up (litestream file:// replica + local rclone dir),
# DESTROYS local state, RESTORES from the replicas, restarts, and runs the full
# per-tenant assertion loop. Proves the drill logic, generator output, and
# restore choreography end to end with zero credentials and zero spend.
#
# (--variant reinstall / second-box are Phase 2, P2-6; only 'local' runs here.)
set -uo pipefail

VARIANT="local"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --variant) VARIANT="$2"; shift 2 ;;
    *) echo "usage: fire-drill.sh --variant local|s3-local" >&2; exit 2 ;;
  esac
done

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=drill/lib.sh
source "$HERE/lib.sh"

case "$VARIANT" in
  local)    DRILL_TARGET="file" ;;                     # file:// litestream + local dir
  s3-local) DRILL_TARGET="s3" ;;                        # local MinIO stands in for R2
  *) echo "variant '$VARIANT' is Phase 2 (P2-6); discovery runs 'local' or 's3-local'" >&2
     exit 2 ;;
esac
export DRILL_TARGET

DRILL_FAILED=0

cleanup() { stop_stack; stop_minio; }
trap 'cleanup' EXIT

log "== fire drill (variant: $VARIANT, target: $DRILL_TARGET) =="
log "1/6 bring up the local stack from the generated config"
mk_workspace
if [[ "$DRILL_TARGET" == s3 ]]; then
  s3_setup_env
  start_minio   # the "R2" store — started ONCE, survives the box destroy below
fi
start_stack

log "2/6 plant canonical + blob markers per tenant"
while IFS=$'\t' read -r name _u _au port _ap _art _am; do
  [[ -z "$name" ]] && continue
  plant_markers "$name" "$port"
done < <(tenants)

if [[ "$DRILL_TARGET" == s3 ]]; then
  log "3/6 back up (litestream + rclone -> MinIO, the R2 stand-in over the S3 API)"
else
  log "3/6 back up (litestream + rclone -> local file/dir R2 stand-in)"
fi
sleep 3   # let litestream replicate the planted rows (1s sync interval)
while IFS=$'\t' read -r name _u _au _p _ap _art _am; do
  [[ -z "$name" ]] && continue
  backup_blobs "$name"
done < <(tenants)

log "4/6 stop services + DESTROY all local state"
# stop_stack kills the stubs/litestream/caddy; the MinIO "R2" store (s3 target)
# is NOT in PIDDIR, so it survives — exactly like a real off-box backup.
stop_stack
while IFS=$'\t' read -r name _u _au _p _ap _art _am; do
  [[ -z "$name" ]] && continue
  destroy "$name"
done < <(tenants)

log "5/6 RESTORE from replicas and restart"
while IFS=$'\t' read -r name _u _au _p _ap _art _am; do
  [[ -z "$name" ]] && continue
  restore "$name"
done < <(tenants)
start_stack

log "6/6 full per-tenant assertion loop"
while IFS=$'\t' read -r name _u _au port api_port _art _am; do
  [[ -z "$name" ]] && continue
  assert_tenant "$name" "$port" "$api_port"
done < <(tenants)

cleanup
trap - EXIT

echo
if [[ "$DRILL_FAILED" -eq 0 ]]; then
  echo "DRILL PASS — destroy→restore→assert green for every tenant (variant $VARIANT)."
  exit 0
fi
echo "DRILL FAIL — see failures above." >&2
exit 1
