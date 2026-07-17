#!/usr/bin/env bats
# D5 — the state taxonomy promoted to executable law: no disposable path is ever
# a backup target, and every canonical path has one. Cross-checks manifests
# against the ACTUAL generated litestream.yml + rclone units (not just the
# generated summary), so a generator bug cannot hide.

load helpers

audit() { "$KIT_ROOT/scripts/backup-audit.sh" "$@"; }

@test "audit passes on the real generated tree" {
  run audit --manifests-dir "$KIT_ROOT/services" --generated-dir "$KIT_ROOT/generated"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "audit FAILS when a disposable path is a backup target" {
  fx="$KIT_ROOT/tests/fixtures/audit_disposable_backed"
  run audit --manifests-dir "$fx/services" --generated-dir "$fx/generated"
  [ "$status" -ne 0 ]
  [[ "$output" == *"disposable"* ]]
  [[ "$output" == *"index.db"* ]]
}

@test "audit FAILS when a canonical path lacks a backup target" {
  fx="$KIT_ROOT/tests/fixtures/audit_canonical_unbacked"
  run audit --manifests-dir "$fx/services" --generated-dir "$fx/generated"
  [ "$status" -ne 0 ]
  [[ "$output" == *"canonical"* ]]
  [[ "$output" == *"state.db"* ]]
}
