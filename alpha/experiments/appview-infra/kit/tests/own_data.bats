#!/usr/bin/env bats
# D10 — the own-data API: self-scoping (caller A never sees B), unauthenticated
# 401, paginated export, statement-timeout, and OS-level write-incapability of
# the api process on the data dir.

load helpers

A="did:example:alice"
B="did:example:bob"

setup() {
  DATADIR="$(mktemp -d)"
  SVC_PORT="$(free_port)"; API_PORT="$(free_port)"
  # writer (service) + reader (api, shared-wal) on the SAME data dir
  spawn_proc SVC "$DATADIR" "$SVC_PORT" --canonical state.db --disposable index.db
  spawn_proc API "$DATADIR" "$API_PORT" --api --api-mode shared-wal \
    --canonical state.db --page-size 2 --stmt-timeout-ms 200
}
teardown() {
  kill_pid "${SVC_PID:-}"; kill_pid "${API_PID:-}"
  rm -rf "$DATADIR"
}

rec() {  # rec <did> <payload>
  curl -s -H "$(auth_hdr "$1")" -X POST \
    "http://127.0.0.1:$SVC_PORT/xrpc/app.stub.recordMyRow?payload=$2" >/dev/null
}

@test "getMyRows returns only the caller's rows (A never sees B)" {
  rec "$A" a1; rec "$A" a2; rec "$B" b1
  run curl -s -H "$(auth_hdr "$A")" "http://127.0.0.1:$API_PORT/xrpc/app.stub.getMyRows"
  [ "$status" -eq 0 ]
  [[ "$output" == *"a1"* && "$output" == *"a2"* ]]
  [[ "$output" != *"b1"* ]]
  # and B sees only b1
  run curl -s -H "$(auth_hdr "$B")" "http://127.0.0.1:$API_PORT/xrpc/app.stub.getMyRows"
  [[ "$output" == *"b1"* ]]
  [[ "$output" != *"a1"* ]]
}

@test "getMyRows unauthenticated is 401" {
  run curl -s -o /dev/null -w "%{http_code}" \
    "http://127.0.0.1:$API_PORT/xrpc/app.stub.getMyRows"
  [ "$output" = "401" ]
}

@test "export streams paginated pages with a cursor" {
  rec "$A" p1; rec "$A" p2; rec "$A" p3
  # page size is 2 -> first page returns 2 rows + a next_cursor
  run curl -s -H "$(auth_hdr "$A")" "http://127.0.0.1:$API_PORT/xrpc/app.stub.export"
  [ "$status" -eq 0 ]
  cursor="$(echo "$output" | python3 -c 'import sys,json;print(json.load(sys.stdin)["next_cursor"])')"
  [ "$cursor" != "None" ]
  n="$(echo "$output" | python3 -c 'import sys,json;print(len(json.load(sys.stdin)["rows"]))')"
  [ "$n" -eq 2 ]
  # second page returns the remaining row and a null cursor
  run curl -s -H "$(auth_hdr "$A")" "http://127.0.0.1:$API_PORT/xrpc/app.stub.export?cursor=$cursor"
  last="$(echo "$output" | python3 -c 'import sys,json;print(json.load(sys.stdin)["next_cursor"])')"
  [ "$last" = "None" ]
}

@test "a deliberately slow query hits the statement timeout (503, fast)" {
  start=$(date +%s)
  run curl -s -o /dev/null -w "%{http_code}" \
    -H "$(auth_hdr "$A")" "http://127.0.0.1:$API_PORT/xrpc/app.stub.slowQuery"
  end=$(date +%s)
  [ "$output" = "503" ]
  [ $((end - start)) -lt 5 ]     # timed out fast, did not hang
}

@test "generated api unit is ReadOnlyPaths, never ReadWritePaths (containment fields)" {
  u="$KIT_ROOT/generated/systemd/stellin-appview-api.service"
  grep -q '^ReadOnlyPaths=/var/lib/stellin-appview' "$u"
  run grep -q '^ReadWritePaths=/var/lib/stellin-appview' "$u"
  [ "$status" -ne 0 ]
}

@test "under an OS read-only mount a write is blocked but a read-only query still works" {
  # SPEC-DELTA[run15-sandbox-unshare | stand-in]: no PID-1 systemd here, so we
  # enforce the unit's ReadOnlyPaths with an equivalent mount-namespace ro bind.
  # This is the same guarantee the api runs under: OS-incapable of writing the
  # data dir, while read-only opens keep serving.
  if ! unshare -m true 2>/dev/null; then skip "no mount-namespace support"; fi
  rec "$A" ro1
  run unshare -m /bin/sh -c "
    mount --bind '$DATADIR' '$DATADIR' &&
    mount -o remount,ro,bind '$DATADIR' '$DATADIR' &&
    ( echo x > '$DATADIR/breach' 2>/dev/null && echo WROTE || echo WRITE_BLOCKED ) &&
    python3 -c \"import sqlite3; c=sqlite3.connect('file:$DATADIR/state.db?mode=ro',uri=True); print('READ_OK', len(c.execute('select 1 from my_rows').fetchall())>=1)\"
  "
  [ "$status" -eq 0 ]
  [[ "$output" == *"WRITE_BLOCKED"* ]]
  [[ "$output" == *"READ_OK True"* ]]
  [ ! -e "$DATADIR/breach" ]
}
