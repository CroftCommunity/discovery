#!/usr/bin/env bats
# D13 — the local full-stack rehearsal + fire drill (the pre-purchase capstone).
# Brings up every tenant's stub + api on localhost from the generated config
# (adapted to user-mode), plants canonical + blob markers, backs up to a
# file:// litestream replica and a local rclone dir, DESTROYS local state,
# RESTORES from the replicas, restarts, and runs the full per-tenant assertion
# loop. Zero credentials, zero spend.
#
# SPEC-DELTA[run15-usermode | stand-in]: user-mode supervised processes stand in
# for systemd units; litestream file:// + a local dir stand in for R2.

load helpers

@test "drill scripts are shellcheck-clean" {
  run shellcheck "$KIT_ROOT/drill/fire-drill.sh" "$KIT_ROOT/drill/lib.sh" \
    "$KIT_ROOT/drill/local-up.sh"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "fire-drill --variant local: destroy, restore, and the full assertion loop pass" {
  run "$KIT_ROOT/drill/fire-drill.sh" --variant local
  echo "$output"
  [ "$status" -eq 0 ]
  [[ "$output" == *"DRILL PASS"* ]]
  # every tenant's full assertion loop reported
  [[ "$output" == *"stellin-appview"* ]]
  [[ "$output" == *"croft-groups"* ]]
  # the five assertion classes each appear
  [[ "$output" == *"healthz"* ]]
  [[ "$output" == *"canonical marker"* ]]
  [[ "$output" == *"blob marker"* ]]
  [[ "$output" == *"api self-scoping"* ]]
  [[ "$output" == *"gated-group"* ]]
}
