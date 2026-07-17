#!/usr/bin/env bats
# D3 — the generator turns manifests into the exact expected set of systemd
# units, Caddy vhosts, litestream.yml, and rclone timers, and rejects
# structurally invalid manifests (port collision, canonical/disposable overlap).

load helpers

render() {  # render <services-dir> <out-dir> [extra args]
  "$KIT_ROOT/scripts/render.sh" --services-dir "$1" --out-dir "$2" "${@:3}"
}

setup() {
  OUT="$(mktemp -d)"
  FIX="$KIT_ROOT/tests/fixtures"
}
teardown() { rm -rf "$OUT"; }

@test "emits the exact file set for two fixture services" {
  run render "$FIX/render_good" "$OUT"
  [ "$status" -eq 0 ]
  # svc-a: service + api + blob timer + two vhosts
  [ -f "$OUT/systemd/svc-a.service" ]
  [ -f "$OUT/systemd/svc-a-api.service" ]
  [ -f "$OUT/systemd/svc-a-blob-0.service" ]
  [ -f "$OUT/systemd/svc-a-blob-0.timer" ]
  [ -f "$OUT/caddy/a.example.test.caddy" ]
  [ -f "$OUT/caddy/api.a.example.test.caddy" ]
  # svc-b: service only, no api, no api vhost
  [ -f "$OUT/systemd/svc-b.service" ]
  [ ! -f "$OUT/systemd/svc-b-api.service" ]
  [ ! -f "$OUT/caddy/api.b.example.test.caddy" ]
  # aggregate backup configs
  [ -f "$OUT/litestream.yml" ]
  [ -f "$OUT/backup-map.json" ]
}

@test "service unit carries the required hardening directives" {
  render "$FIX/render_good" "$OUT"
  u="$OUT/systemd/svc-a.service"
  grep -q '^NoNewPrivileges=yes' "$u"
  grep -q '^ProtectSystem=strict' "$u"
  grep -q '^Restart=always' "$u"
  grep -q '^PrivateTmp=yes' "$u"
  grep -q '^StateDirectory=svc-a' "$u"
  grep -q '^ReadWritePaths=/var/lib/svc-a' "$u"
  grep -q '^User=svc-a' "$u"
}

@test "api unit is ReadOnlyPaths on the data dir and never ReadWritePaths" {
  render "$FIX/render_good" "$OUT"
  u="$OUT/systemd/svc-a-api.service"
  grep -q '^ReadOnlyPaths=/var/lib/svc-a' "$u"
  grep -q '^CPUQuota=' "$u"
  grep -q '^IOSchedulingClass=' "$u"
  run grep -q '^ReadWritePaths=/var/lib/svc-a' "$u"
  [ "$status" -ne 0 ]   # MUST NOT be able to write the data dir
}

@test "snapshot-mode api gets a VACUUM INTO timer; shared-wal does not" {
  # svc-a is shared-wal in the fixture => no snapshot unit
  render "$FIX/render_good" "$OUT"
  [ ! -f "$OUT/systemd/svc-a-snapshot.timer" ]
  # a snapshot-mode fixture
  local sdir; sdir="$(mktemp -d)"
  cat > "$sdir/snap.toml" <<'EOF'
name = "snap"
fqdn = "snap.example.test"
port = 9101
api_port = 9102
serve_api = true
api_mode = "snapshot"
[data_profile]
canonical = ["state.db"]
disposable = ["index.db"]
EOF
  render "$sdir" "$OUT"
  [ -f "$OUT/systemd/snap-snapshot.service" ]
  [ -f "$OUT/systemd/snap-snapshot.timer" ]
  grep -q 'VACUUM INTO' "$OUT/systemd/snap-snapshot.service"
  rm -rf "$sdir"
}

@test "port collision across service and api ports is rejected" {
  run render "$FIX/render_portcollision" "$OUT"
  [ "$status" -ne 0 ]
  [[ "$output" == *"port"* ]]
}

@test "canonical/disposable overlap is rejected" {
  run render "$FIX/render_overlap" "$OUT"
  [ "$status" -ne 0 ]
  [[ "$output" == *"overlap"* || "$output" == *"both"* ]]
}

@test "litestream.yml lists every canonical db and no disposable db" {
  render "$FIX/render_good" "$OUT"
  grep -q '/var/lib/svc-a/state.db' "$OUT/litestream.yml"
  grep -q '/var/lib/svc-b/state.db' "$OUT/litestream.yml"
  run grep -q 'index.db' "$OUT/litestream.yml"
  [ "$status" -ne 0 ]   # disposable never appears in a backup config
}

@test "immutable blob dir gets --immutable; mutable blob dir is flagged" {
  render "$FIX/render_good" "$OUT"
  # svc-a blobs/ is content-addressed => --immutable
  grep -q -- '--immutable' "$OUT/systemd/svc-a-blob-0.service"
  # svc-b media/ is mutable => no --immutable, and a FLAGGED comment
  run grep -q -- '--immutable' "$OUT/systemd/svc-b-blob-0.service"
  [ "$status" -ne 0 ]
  grep -qi 'FLAGGED' "$OUT/systemd/svc-b-blob-0.service"
}
