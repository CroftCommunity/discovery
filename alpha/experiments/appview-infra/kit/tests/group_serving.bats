#!/usr/bin/env bats
# D12 — roster-gated large-group serving, fork-agnostic (behind GroupStore).
# Matrix mirrors RUN-14 EXP-A: member gets content, non-member 403, anonymous
# 401, removed member 403 on next call. Verifier is the stubbed interface (real
# service-auth is RUN-14 EXP-A). Roster/grants are canonical.

load helpers

ALICE="did:example:alice"   # roster member
BOB="did:example:bob"       # roster member (to be removed)
CAROL="did:example:carol"   # never a member

setup() {
  DATADIR="$(mktemp -d)"
  PORT="$(free_port)"
  start_stub "$DATADIR" "$PORT" \
    --canonical state.db --disposable index.db \
    --gated-groups --group-fixture "$KIT_ROOT/tests/fixtures/groups/g.json"
}
teardown() { stop_stub; rm -rf "$DATADIR"; }

gc() {  # gc <did> ; content GET as that did
  curl -s -o /dev/null -w "%{http_code}" -H "$(auth_hdr "$1")" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.getGroupContent?group=g1"
}

@test "a roster member gets the group content (200)" {
  run curl -s -H "$(auth_hdr "$ALICE")" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.getGroupContent?group=g1"
  [ "$status" -eq 0 ]
  [[ "$output" == *"hello-group"* ]]
  [[ "$output" == *"second-post"* ]]
}

@test "a non-member is 403" {
  run gc "$CAROL"
  [ "$output" = "403" ]
}

@test "an anonymous caller is 401" {
  run curl -s -o /dev/null -w "%{http_code}" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.getGroupContent?group=g1"
  [ "$output" = "401" ]
}

@test "a removed member is 403 on the next call" {
  run gc "$BOB"
  [ "$output" = "200" ]
  curl -s -X POST -H "$(auth_hdr "$ALICE")" \
    "http://127.0.0.1:$PORT/xrpc/app.stub.removeRosterMember?group=g1&did=$BOB" >/dev/null
  run gc "$BOB"
  [ "$output" = "403" ]
}

@test "roster/grants are classed canonical in the gated manifests" {
  for m in stellin-appview croft-groups; do
    f="$KIT_ROOT/services/$m.toml"
    grep -q 'gated_groups *= *true' "$f"
    # roster/grants live in state.db, which must be canonical
    grep -Eq 'canonical *= *\[.*state.db' "$f"
  done
}

@test "the generated gated-tenant unit enables gated-groups serving" {
  grep -q -- '--gated-groups' "$KIT_ROOT/generated/systemd/stellin-appview.service"
}

@test "CONTRACT documents offering-vs-reading as NOT applying to this tier" {
  grep -qi 'offering-vs-reading' "$KIT_ROOT/CONTRACT.md"
  grep -qi 'reads by design' "$KIT_ROOT/CONTRACT.md"
}
