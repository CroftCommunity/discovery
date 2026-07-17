#!/usr/bin/env bats
# D11 — mechanical consistency check for the group-tier design brief.
# Verifies GROUPS.md's stated invariants match §1.2 (taxonomy) and §1.4 (the
# gated tier + write-path fork). This is a claims-grep, recorded as such — it
# checks that the brief SAYS the right invariants, not that they are proven.
#
# GROUPS.md is corpus (stays in discovery, excluded from extraction), so this
# lives outside kit/ and is not part of the kit's `make check`.

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GROUPS="$DIR/GROUPS.md"
KIT="$DIR/kit"

@test "GROUPS.md link-check passes" {
  run "$KIT/scripts/link-check.sh" "$GROUPS"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "honest-posture language is present (private, not cryptographically confidential)" {
  grep -qi 'roster-gated' "$GROUPS"
  grep -qi 'not.*E2EE\|not cryptographically confidential\|not.*confidential' "$GROUPS"
  grep -qi 'trusted.gatekeeper' "$GROUPS"
  grep -qi 'member.*leak' "$GROUPS"
}

@test "the scale boundary is a deferred parameter with the working number" {
  grep -q 'group_scale_boundary' "$GROUPS"
  grep -q '5000\|5,000' "$GROUPS"
  grep -qi 'defer\|owner' "$GROUPS"
}

@test "both write-path variants are analyzed" {
  grep -qi 'Variant A' "$GROUPS"
  grep -qi 'Variant B' "$GROUPS"
  # scored against the taxonomy dimensions named in §1.4
  grep -qi 'backup' "$GROUPS"
  grep -qi 'restore' "$GROUPS"
  grep -qi 'moderation' "$GROUPS"
  grep -qi 'migration' "$GROUPS"
}

@test "taxonomy classification matches §1.2 (roster/grants canonical, index disposable)" {
  grep -qi 'roster' "$GROUPS"
  grep -qi 'canonical' "$GROUPS"
  grep -qi 'disposable' "$GROUPS"
}

@test "the decision request names variant, boundary, and launch order" {
  grep -qi 'decision' "$GROUPS"
  grep -qi 'variant' "$GROUPS"
  grep -qi 'boundary' "$GROUPS"
  grep -qi 'before.*with\|launch' "$GROUPS"
}

@test "the spec-facing note is staged in proposed-changes, reviewed spec untouched" {
  grep -q 'RUN-15' "$DIR/../../../beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md"
  grep -q 'RUN-15' "$DIR/../../../beta/impl/experiments/drystone-reviews-and-experiments-log.md"
}
