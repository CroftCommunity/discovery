#!/usr/bin/env bash
# drill/lib.sh — shared local-rehearsal machinery (sourced, never run directly).
#
# Brings up every tenant's stub + api on localhost from the manifests (adapted
# to user-mode: plain supervised processes, not systemd), replicates canonical
# sqlite to a file:// litestream replica and blob dirs to a local rclone dir
# (standing in for R2), and provides destroy/restore + a full assertion loop.
#
# SPEC-DELTA[run15-usermode | stand-in]: user-mode processes + file/local
# replicas stand in for systemd units + R2. The generated config is the same;
# only the execution substrate and replica endpoints are adapted.

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCAL="${LOCAL_ROOT:-$KIT/.local}"
R2_STANDIN="$LOCAL/r2"
REPLICAS="$LOCAL/replicas"
STATE="$LOCAL/state"        # per-tenant data dirs live here
PIDDIR="$LOCAL/pids"
LOGDIR="$LOCAL/logs"
STUB="$KIT/stub/stub.py"
FIXTURE="$KIT/tests/fixtures/groups/g.json"
LS_CONFIG="$LOCAL/litestream.yml"
CADDYFILE="$LOCAL/Caddyfile"

# Backup target: "file" (default — file:// litestream + local rclone dir) or
# "s3" (a local MinIO standing in for R2, exercising the real s3:// code path).
# SPEC-DELTA[run15-s3-local | stand-in]: local MinIO stands in for Cloudflare R2
# (same S3 API); the sandbox's proxy-injected AWS creds are overridden and the
# CA bundle cleared for the plain-HTTP localhost endpoint. Phase 1.5 points the
# same s3:// configs at real R2 by swapping endpoint + credentials only.
DRILL_TARGET="${DRILL_TARGET:-file}"
BUCKET="croft-appview-staging"       # matches the generator's RENDER_BUCKET default
MINIO_ADDR="127.0.0.1:9000"
MINIO_DATA="$LOCAL/minio"            # the "R2" store — survives the box destroy
MINIO_PIDFILE="$LOCAL/minio.pid"     # kept OUT of PIDDIR so stop_stack won't kill it

# DRILL_FAILED is set here (bad()) and read by the calling script (fire-drill /
# local-up). Initialise so shellcheck sees a use and re-runs are clean.
: "${DRILL_FAILED:=0}"

DRILL_DID="did:example:drill"        # canonical-marker owner
MEMBER_DID="did:example:alice"       # in the g1 fixture roster
NONMEMBER_DID="did:example:carol"    # never a member
CADDY_OFFSET=10000                   # caddy listens on port+offset (plain http)

tenants() { python3 "$KIT/scripts/list-services.py" "$KIT/services"; }

auth() { echo "Authorization: Bearer test:$1"; }

# --- s3 target (MinIO stands in for R2) ------------------------------------
# Ephemeral, generated-at-runtime creds — NO static secret lives in the repo.
s3_setup_env() {
  local key="drill" secret
  secret="$(head -c 16 /dev/urandom | od -An -tx1 | tr -d ' \n')"  # 32 hex chars
  export MINIO_ROOT_USER="$key" MINIO_ROOT_PASSWORD="$secret"
  # Override the sandbox's proxy-injected AWS creds and clear the CA bundle so
  # litestream/rclone reach the plain-HTTP localhost MinIO with the right keys.
  export AWS_ACCESS_KEY_ID="$key" AWS_SECRET_ACCESS_KEY="$secret"
  export LITESTREAM_ACCESS_KEY_ID="$key" LITESTREAM_SECRET_ACCESS_KEY="$secret"
  export AWS_CA_BUNDLE=""
  export NO_PROXY="127.0.0.1,localhost,::1${NO_PROXY:+,$NO_PROXY}"
  export no_proxy="$NO_PROXY"
  # the rclone remote name matches the generated units' r2: remote
  export RCLONE_CONFIG_R2_TYPE=s3 RCLONE_CONFIG_R2_PROVIDER=Minio
  export RCLONE_CONFIG_R2_ENDPOINT="http://$MINIO_ADDR"
  export RCLONE_CONFIG_R2_ACCESS_KEY_ID="$key"
  export RCLONE_CONFIG_R2_SECRET_ACCESS_KEY="$secret"
}

start_minio() {
  command -v minio >/dev/null 2>&1 || { bad "minio not installed (s3 target)"; return 1; }
  mkdir -p "$MINIO_DATA"
  minio server "$MINIO_DATA" --address "$MINIO_ADDR" \
    --console-address 127.0.0.1:9001 >"$LOGDIR/minio.log" 2>&1 &
  echo $! > "$MINIO_PIDFILE"
  local _
  for _ in $(seq 1 80); do
    curl -fsS "http://$MINIO_ADDR/minio/health/live" >/dev/null 2>&1 && break
    sleep 0.25
  done
  curl -fsS "http://$MINIO_ADDR/minio/health/live" >/dev/null 2>&1 \
    || { bad "minio did not become healthy"; return 1; }
  rclone mkdir "r2:$BUCKET" >>"$LOGDIR/rclone.log" 2>&1 || true
}

stop_minio() {
  [[ -f "$MINIO_PIDFILE" ]] || return 0
  kill "$(cat "$MINIO_PIDFILE")" 2>/dev/null || true
  rm -f "$MINIO_PIDFILE"
}

# P15-3 (local): estimate the S3 write (Class-A) op profile a drill produces and
# surface the free-tier signal. The count is illustrative; the load-bearing
# message is the RATE model, which is what actually decides free-tier fit.
report_op_counts() {
  [[ "$DRILL_TARGET" == s3 ]] || return 0
  local ls_puts blob_puts
  # each litestream WAL segment / snapshot write is ~one S3 PUT
  ls_puts="$(grep -cE 'wal segment written|snapshot written' \
    "$LOGDIR/litestream.log" 2>/dev/null || echo 0)"
  # each blob file synced is ~one S3 PUT (first upload)
  blob_puts="$(find "$STATE" -path '*/blobs/*' -type f 2>/dev/null | wc -l | tr -d ' ')"
  echo
  echo "[drill] --- S3 op estimate (MinIO stand-in; R2 Class-A = writes) ---"
  echo "  litestream WAL/snapshot PUTs (observed): $ls_puts"
  echo "  rclone blob PUTs (observed):             $blob_puts"
  echo "  RATE MODEL (the real free-tier determinant): litestream syncs at"
  echo "  sync-interval=1s, so a CONTINUOUSLY-writing canonical db emits up to"
  echo "  ~1 PUT/s = ~2.6M PUT/mo per db — OVER R2's 1M/mo free Class-A tier."
  echo "  => Free-tier fit depends on WRITE FREQUENCY, not tenant count. Levers:"
  echo "  raise sync-interval, or use a paid tier, for high-write tenants. Idle /"
  echo "  rarely-writing dbs stay well inside 1M/mo. Confirm exact counts against"
  echo "  real R2 in P15-3."
}

log()  { echo "[drill] $*"; }
ok()   { echo "  PASS: $*"; }
bad()  { echo "  FAIL: $*" >&2; DRILL_FAILED=1; }

# ---------------------------------------------------------------------------
mk_workspace() {
  rm -rf "$LOCAL"
  mkdir -p "$R2_STANDIN" "$REPLICAS" "$STATE" "$PIDDIR" "$LOGDIR"
}

# Emit a local litestream config: file:// replica per canonical db.
render_litestream_local() {
  {
    echo "dbs:"
    while IFS=$'\t' read -r name _u _au _p _ap _art _am; do
      [[ -z "$name" ]] && continue
      printf '  - path: %s/%s/state.db\n' "$STATE" "$name"
      printf '    replicas:\n'
      if [[ "$DRILL_TARGET" == s3 ]]; then
        printf '      - type: s3\n        endpoint: http://%s\n        bucket: %s\n        path: %s/state\n        force-path-style: true\n        region: us-east-1\n' \
          "$MINIO_ADDR" "$BUCKET" "$name"
      else
        printf '      - type: file\n        path: %s/%s-state\n' "$REPLICAS" "$name"
      fi
    done < <(tenants)
  } > "$LS_CONFIG"
}

# Emit a local Caddyfile: plain HTTP on high ports proxying to each stub.
render_caddy_local() {
  {
    while IFS=$'\t' read -r name _u _au port _ap _art _am; do
      [[ -z "$name" ]] && continue
      printf ':%s {\n\treverse_proxy 127.0.0.1:%s\n}\n' \
        "$((port + CADDY_OFFSET))" "$port"
    done < <(tenants)
  } > "$CADDYFILE"
}

wait_health() {
  local port="$1" _
  for _ in $(seq 1 50); do
    curl -fsS "http://127.0.0.1:$port/healthz" >/dev/null 2>&1 && return 0
    sleep 0.1
  done
  return 1
}

start_tenant() {
  local name="$1" port="$2" api_user="$3" api_port="$4" api_mode="$5"
  local dd="$STATE/$name"
  mkdir -p "$dd"
  local gated=()
  # stellin/croft-groups are gated_groups tenants -> seed the fixture roster
  if grep -q 'gated_groups *= *true' "$KIT/services/$name.toml"; then
    gated=(--gated-groups --group-fixture "$FIXTURE")
  fi
  STUB_ALLOW_ROOT=1 python3 "$STUB" --data-dir "$dd" --listen "127.0.0.1:$port" \
    --canonical state.db --disposable index.db --blobs blobs/ "${gated[@]}" \
    >"$LOGDIR/$name.log" 2>&1 &
  echo $! > "$PIDDIR/$name.pid"
  wait_health "$port" || { bad "$name service did not become healthy"; return 1; }
  if [[ "$api_user" != "-" ]]; then
    # snapshot mode: the api serves serve/state.db, normally refreshed by the
    # VACUUM INTO timer. No systemd timers here, so produce the snapshot the way
    # the generated snapshot.service would (atomic swap).
    if [[ "$api_mode" == "snapshot" ]]; then
      mkdir -p "$dd/serve"
      sqlite3 "$dd/state.db" "VACUUM INTO '$dd/serve/state.db.tmp'"
      mv -f "$dd/serve/state.db.tmp" "$dd/serve/state.db"
    fi
    STUB_ALLOW_ROOT=1 python3 "$STUB" --api --api-mode "$api_mode" \
      --data-dir "$dd" --listen "127.0.0.1:$api_port" --canonical state.db \
      --page-size 100 --stmt-timeout-ms 300 \
      >"$LOGDIR/$name-api.log" 2>&1 &
    echo $! > "$PIDDIR/$name-api.pid"
    wait_health "$api_port" || { bad "$name api did not become healthy"; return 1; }
  fi
}

start_stack() {
  render_litestream_local
  render_caddy_local
  while IFS=$'\t' read -r name _u api_user port api_port _art api_mode; do
    [[ -z "$name" ]] && continue
    start_tenant "$name" "$port" "$api_user" "$api_port" "$api_mode"
  done < <(tenants)
  # litestream replicate (daemon) for all canonical dbs
  litestream replicate -config "$LS_CONFIG" >"$LOGDIR/litestream.log" 2>&1 &
  echo $! > "$PIDDIR/litestream.pid"
  # caddy (plain http, high ports) — validated then run
  if command -v caddy >/dev/null 2>&1; then
    caddy validate --config "$CADDYFILE" --adapter caddyfile \
      >"$LOGDIR/caddy-validate.log" 2>&1 || bad "caddy validate failed"
    caddy run --config "$CADDYFILE" --adapter caddyfile \
      >"$LOGDIR/caddy.log" 2>&1 &
    echo $! > "$PIDDIR/caddy.pid"
    sleep 1
  fi
}

stop_stack() {
  # Kill only the stack processes (stubs/api/litestream/caddy) and reap ONLY
  # those PIDs — never a bare `wait`, which would block on the MinIO "R2" child
  # that we deliberately keep alive across the destroy/restore (s3 target).
  local f p pids=()
  for f in "$PIDDIR"/*.pid; do
    [[ -e "$f" ]] || continue
    p="$(cat "$f")"
    kill "$p" 2>/dev/null || true
    pids+=("$p")
    rm -f "$f"
  done
  for p in "${pids[@]}"; do wait "$p" 2>/dev/null || true; done
}

# --- markers ----------------------------------------------------------------
plant_markers() {
  local name="$1" port="$2"
  # canonical marker: a my_rows row owned by DRILL_DID (proves litestream restore)
  curl -fsS -X POST -H "$(auth "$DRILL_DID")" \
    "http://127.0.0.1:$port/xrpc/app.stub.recordMyRow?payload=CANON-$name" >/dev/null
  # blob marker: a file in the blob dir (proves rclone restore)
  echo "BLOB-$name" > "$STATE/$name/blobs/DRILL-BLOB"
}

blob_remote() {  # where blobs are mirrored, per target
  local name="$1"
  if [[ "$DRILL_TARGET" == s3 ]]; then echo "r2:$BUCKET/$name/blobs"
  else echo "$R2_STANDIN/$name/blobs"; fi
}

backup_blobs() {
  local name="$1"
  rclone sync "$STATE/$name/blobs" "$(blob_remote "$name")" \
    >>"$LOGDIR/rclone.log" 2>&1
}

destroy() {
  local name="$1"
  rm -f "$STATE/$name/state.db" "$STATE/$name/state.db-wal" \
        "$STATE/$name/state.db-shm" "$STATE/$name/index.db"
  rm -rf "$STATE/$name/blobs"
}

restore() {
  local name="$1"
  litestream restore -config "$LS_CONFIG" -o "$STATE/$name/state.db" \
    "$STATE/$name/state.db" >>"$LOGDIR/restore.log" 2>&1
  mkdir -p "$STATE/$name/blobs"
  rclone copy "$(blob_remote "$name")" "$STATE/$name/blobs" \
    >>"$LOGDIR/rclone.log" 2>&1
}

# --- the full per-tenant assertion loop -------------------------------------
assert_tenant() {
  local name="$1" port="$2" api_port="$3"
  echo "-- asserting tenant: $name"
  # 1. healthz (through Caddy if present, else direct)
  local hport="$port"
  if [[ -f "$PIDDIR/caddy.pid" ]]; then hport=$((port + CADDY_OFFSET)); fi
  if curl -fsS "http://127.0.0.1:$hport/healthz" | grep -q '^ok$'; then
    ok "healthz"; else bad "$name healthz"; fi
  # 2. canonical marker restored (litestream): getMyRows via api as DRILL_DID
  if [[ "$api_port" != "-" ]]; then
    if curl -fsS -H "$(auth "$DRILL_DID")" \
        "http://127.0.0.1:$api_port/xrpc/app.stub.getMyRows" | grep -q "CANON-$name"; then
      ok "canonical marker"; else bad "$name canonical marker missing after restore"; fi
    # 4. api self-scoping: a different DID must NOT see DRILL_DID's row
    if curl -fsS -H "$(auth "$NONMEMBER_DID")" \
        "http://127.0.0.1:$api_port/xrpc/app.stub.getMyRows" | grep -q "CANON-$name"; then
      bad "$name api self-scoping breach"; else ok "api self-scoping"; fi
  else
    ok "canonical marker (no api; skipped)"; ok "api self-scoping (no api; skipped)"
  fi
  # 3. blob marker restored (rclone)
  if [[ -f "$STATE/$name/blobs/DRILL-BLOB" ]] && \
     grep -q "BLOB-$name" "$STATE/$name/blobs/DRILL-BLOB"; then
    ok "blob marker"; else bad "$name blob marker missing after restore"; fi
  # 5. gated-group serving: member 200 + content, non-member 403
  if grep -q 'gated_groups *= *true' "$KIT/services/$name.toml"; then
    if curl -fsS -H "$(auth "$MEMBER_DID")" \
        "http://127.0.0.1:$port/xrpc/app.stub.getGroupContent?group=g1" \
        | grep -q 'hello-group'; then ok "gated-group member"; else bad "$name gated member"; fi
    local code
    code=$(curl -s -o /dev/null -w "%{http_code}" -H "$(auth "$NONMEMBER_DID")" \
      "http://127.0.0.1:$port/xrpc/app.stub.getGroupContent?group=g1")
    if [[ "$code" == "403" ]]; then ok "gated-group non-member 403"; else bad "$name gated non-member ($code)"; fi
  else
    ok "gated-group (not a gated tenant; skipped)"
  fi
}
