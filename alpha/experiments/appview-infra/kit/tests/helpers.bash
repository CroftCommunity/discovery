#!/usr/bin/env bash
# Shared bats helpers for the appview-infra kit tests.
# Pure stdlib + curl; no network, no discovery-repo dependency.

KIT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export KIT_ROOT

# Pick a free TCP port by binding :0 and reading it back.
free_port() {
  python3 - <<'PY'
import socket
s = socket.socket()
s.bind(("127.0.0.1", 0))
print(s.getsockname()[1])
s.close()
PY
}

# start_stub <datadir> <port> [extra args...]
# Launches stub/stub.py in the background, records PID in $STUB_PID, waits for
# /healthz to answer 200 (up to ~5s).
start_stub() {
  local datadir="$1" port="$2"; shift 2
  # SPEC-DELTA[run15-local-root | stand-in]: the deployed service runs as a
  # dedicated non-root user (systemd User=); the contract stub refuses root.
  # This CI/rehearsal container is root-only, so we set the documented override.
  # On the box, User= (not this flag) provides the non-root guarantee.
  STUB_ALLOW_ROOT=1 python3 "$KIT_ROOT/stub/stub.py" \
    --data-dir "$datadir" --listen "127.0.0.1:$port" "$@" \
    >"$datadir/.stub.log" 2>&1 &
  STUB_PID=$!
  export STUB_PID STUB_PORT="$port"
  local i
  for i in $(seq 1 50); do
    if curl -fsS "http://127.0.0.1:$port/healthz" >/dev/null 2>&1; then return 0; fi
    if ! kill -0 "$STUB_PID" 2>/dev/null; then
      echo "stub died on startup; log:" >&2; cat "$datadir/.stub.log" >&2; return 1
    fi
    sleep 0.1
  done
  echo "stub did not become healthy on :$port" >&2
  cat "$datadir/.stub.log" >&2
  return 1
}

stop_stub() {
  [[ -n "${STUB_PID:-}" ]] && kill "$STUB_PID" 2>/dev/null || true
  [[ -n "${STUB_PID:-}" ]] && wait "$STUB_PID" 2>/dev/null || true
  STUB_PID=""
}

# A stub auth token for caller DID $1 (stand-in for a real service-auth JWT).
auth_hdr() { echo "Authorization: Bearer test:$1"; }
