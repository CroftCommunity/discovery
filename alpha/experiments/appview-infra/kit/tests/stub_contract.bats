#!/usr/bin/env bats
# D2 — the stub satisfies the core contract: healthz, data-dir files per the
# passed profile, and 401 on unauthenticated authed routes.

load helpers

setup() {
  DATADIR="$(mktemp -d)"
  PORT="$(free_port)"
  start_stub "$DATADIR" "$PORT" \
    --canonical state.db --disposable index.db --blobs blobs/
}

teardown() {
  stop_stub
  rm -rf "$DATADIR"
}

@test "healthz returns 200 ok" {
  run curl -fsS "http://127.0.0.1:$PORT/healthz"
  [ "$status" -eq 0 ]
  [ "$output" = "ok" ]
}

@test "canonical and disposable sqlite files are created under the data dir" {
  [ -f "$DATADIR/state.db" ]
  [ -f "$DATADIR/index.db" ]
  # created as real SQLite databases, not empty files
  run sqlite3 "$DATADIR/state.db" "pragma journal_mode;"
  [ "$status" -eq 0 ]
}

@test "blob dir is created under the data dir" {
  [ -d "$DATADIR/blobs" ]
}

@test "authed route without a token is 401" {
  run curl -s -o /dev/null -w "%{http_code}" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.echo?msg=hi"
  [ "$output" = "401" ]
}

@test "authed route with a valid token echoes 200" {
  run curl -s -H "$(auth_hdr did:example:alice)" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.echo?msg=hi"
  [ "$status" -eq 0 ]
  [[ "$output" == *"hi"* ]]
  [[ "$output" == *"did:example:alice"* ]]
}

@test "authed route with a malformed token is 401 (never a degraded 200)" {
  run curl -s -o /dev/null -w "%{http_code}" \
    -H "Authorization: Bearer garbage" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.echo?msg=hi"
  [ "$output" = "401" ]
}
