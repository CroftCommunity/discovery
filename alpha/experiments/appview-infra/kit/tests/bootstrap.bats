#!/usr/bin/env bats
# D7 — bootstrap.sh brings a fresh Debian 12 to a serving state, idempotently.
#
# This environment is not a fresh Debian box and must not be mutated, so the
# check exercises --plan (dry-run) idempotence and content. Real double-apply
# idempotence is Phase 2 (P2-3).
# SPEC-DELTA[run15-bootstrap-dryrun | stand-in]

load helpers

BS() { "$KIT_ROOT/bootstrap/bootstrap.sh" "$@"; }

@test "bootstrap.sh is shellcheck-clean" {
  run shellcheck "$KIT_ROOT/bootstrap/bootstrap.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "plan lists per-manifest service and api users" {
  run BS --plan
  [ "$status" -eq 0 ]
  [[ "$output" == *"stellin-appview"* ]]
  [[ "$output" == *"stellin-appview-api"* ]]
  [[ "$output" == *"croft-groups"* ]]
  [[ "$output" == *"croft-groups-api"* ]]
  [[ "$output" == *"deploy"* ]]           # the deploy user (D8)
}

@test "plan covers firewall, upgrades, ssh, caddy, backup tools, units" {
  run BS --plan
  [ "$status" -eq 0 ]
  [[ "$output" == *"nftables"* ]]
  [[ "$output" == *"22"* && "$output" == *"80"* && "$output" == *"443"* ]]
  [[ "$output" == *"unattended-upgrades"* ]]
  [[ "$output" == *"ssh"* || "$output" == *"SSH"* ]]
  [[ "$output" == *"caddy"* || "$output" == *"Caddy"* ]]
  [[ "$output" == *"litestream"* ]]
  [[ "$output" == *"rclone"* ]]
  [[ "$output" == *"systemctl"* || "$output" == *"enable"* ]]
}

@test "plan pins litestream and rclone versions from versions.env" {
  grep -q 'LITESTREAM_VERSION=' "$KIT_ROOT/bootstrap/versions.env"
  grep -q 'RCLONE_VERSION=' "$KIT_ROOT/bootstrap/versions.env"
}

@test "plan is idempotent (two runs identical)" {
  a="$(BS --plan)"
  b="$(BS --plan)"
  [ "$a" = "$b" ]
}

@test "plan mutates nothing on this host" {
  before="$(id stellin-appview 2>/dev/null || echo none)"
  BS --plan >/dev/null
  after="$(id stellin-appview 2>/dev/null || echo none)"
  [ "$before" = "$after" ]
}
