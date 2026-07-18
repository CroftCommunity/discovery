#!/usr/bin/env bats
# D11 — mechanical consistency check for the group-tier design brief.
# Verifies GROUPS.md's stated invariants match §1.2 (taxonomy) and §1.4 (the
# gated tier + write-path fork). This is a claims-grep, recorded as such — it
# checks that the brief SAYS the right invariants, not that they are proven.
#
# GROUPS.md is corpus (stays in discovery, excluded from extraction), so this
# lives outside kit/ and is not part of the kit's `make check`.

DIR="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
GROUPS_MD="$DIR/GROUPS.md"
KIT="$DIR/kit"

@test "GROUPS.md link-check passes" {
  run "$KIT/scripts/link-check.sh" "$GROUPS_MD"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "honest-posture language is present (private, not cryptographically confidential)" {
  grep -qi 'roster-gated' "$GROUPS_MD"
  grep -qi 'not.*E2EE\|not cryptographically confidential\|not.*confidential' "$GROUPS_MD"
  grep -qi 'trusted.gatekeeper' "$GROUPS_MD"
  grep -qi 'member.*leak' "$GROUPS_MD"
}

@test "the scale boundary is a deferred parameter with the working number" {
  grep -q 'group_scale_boundary' "$GROUPS_MD"
  grep -q '5000\|5,000' "$GROUPS_MD"
  grep -qi 'defer\|owner' "$GROUPS_MD"
}

@test "both write-path variants are analyzed" {
  grep -qi 'Variant A' "$GROUPS_MD"
  grep -qi 'Variant B' "$GROUPS_MD"
  # scored against the taxonomy dimensions named in §1.4
  grep -qi 'backup' "$GROUPS_MD"
  grep -qi 'restore' "$GROUPS_MD"
  grep -qi 'moderation' "$GROUPS_MD"
  grep -qi 'migration' "$GROUPS_MD"
}

@test "taxonomy classification matches §1.2 (roster/grants canonical, index disposable)" {
  grep -qi 'roster' "$GROUPS_MD"
  grep -qi 'canonical' "$GROUPS_MD"
  grep -qi 'disposable' "$GROUPS_MD"
}

@test "the decision request names variant, boundary, and launch order" {
  grep -qi 'decision' "$GROUPS_MD"
  grep -qi 'variant' "$GROUPS_MD"
  grep -qi 'boundary' "$GROUPS_MD"
  grep -qi 'before.*with\|launch' "$GROUPS_MD"
}

@test "the spec-facing note is staged in proposed-changes, reviewed spec untouched" {
  grep -q 'RUN-15' "$DIR/../../../beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md"
  grep -q 'RUN-15' "$DIR/../../../beta/impl/experiments/drystone-reviews-and-experiments-log.md"
}

# ---------------------------------------------------------------------------
# RUN-16 — the group tier model (v2). The three-tier / two-axis model
# supersedes and extends the D11 two-tier framing above. These invariants are
# extended RED first (asserting GROUPS.md v2's model text), then GROUPS.md v2
# and the staged note land to turn them green. Same claims-grep discipline: a
# check that the model SAYS the right invariants, not that they are proven.
# ---------------------------------------------------------------------------

@test "v2: two policy axes, not one (membership policy + write policy)" {
  grep -qi 'membership policy' "$GROUPS_MD"
  grep -qi 'write policy' "$GROUPS_MD"
  # the three membership values on one axis
  grep -qi 'open.*gated.*sealed' "$GROUPS_MD"
  # a write-policy value unique to the second axis
  grep -qi 'named-set' "$GROUPS_MD"
}

@test "v2: three tiers — open/broadcast, gated/backplane, sealed" {
  grep -qi 'broadcast' "$GROUPS_MD"
  grep -qi 'backplane' "$GROUPS_MD"
  grep -qi 'sealed' "$GROUPS_MD"
}

@test "v2: silence is not a verdict (pending stays pending; decay is presentation)" {
  grep -qi 'silence is not a verdict' "$GROUPS_MD"
}

@test "v2: a role's sequence numbers are delivery cursors, never order" {
  grep -qi 'delivery cursor' "$GROUPS_MD"
  grep -qi 'never order' "$GROUPS_MD"
}

@test "v2: key authority lives in the DID document, delegated by PDS attestation" {
  grep -qi 'DID document' "$GROUPS_MD"
  grep -qi 'attestation' "$GROUPS_MD"
}

@test "v2: sealed-scope helper-index rows are observation-born, not projections" {
  grep -qi 'observation-born' "$GROUPS_MD"
}

@test "v2: delivery roles are separate processes, not one fused primitive" {
  grep -qi 'own process\|separate process' "$GROUPS_MD"
  grep -qi 'primitive' "$GROUPS_MD"
}

@test "v2: history backfill is scoped by membership interval" {
  grep -qi 'membership interval' "$GROUPS_MD"
  grep -qi 'backfill' "$GROUPS_MD"
}

@test "v2: the iroh overlay is loaded only by sealed scopes and governance" {
  grep -qi 'loaded only by sealed' "$GROUPS_MD"
}

@test "v2: the open-topic survival rule is validate before relay" {
  grep -qi 'validate before relay' "$GROUPS_MD"
}

@test "v2: message identity is the hash of the canonical envelope" {
  grep -qi 'hash of the canonical envelope' "$GROUPS_MD"
}

@test "v2: the RUN-16 model note is staged in proposed-changes and reviews-log" {
  grep -q 'RUN-16' "$DIR/../../../beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md"
  grep -q 'RUN-16' "$DIR/../../../beta/impl/experiments/drystone-reviews-and-experiments-log.md"
}

# ---------------------------------------------------------------------------
# RUN-18 — reception completeness (the subscriber-side guarantee) and the
# publications positioning. Extended RED first: the reception checks assert the
# canonical paragraph GROUPS.md does not yet carry; the publications checks
# assert a PUBLICATIONS.md that does not yet exist. Same claims-grep discipline
# as above — the docs SAY the invariants; Part B (tier-proof) proves them.
# ---------------------------------------------------------------------------

PUB_MD="$DIR/PUBLICATIONS.md"

@test "run18: per-author chaining is REQUIRED in write-restricted scopes" {
  grep -qi 'MUST chain their envelopes' "$GROUPS_MD"
  # each envelope carries the author's previous envelope in its antecedents
  grep -qi "author's previous envelope" "$GROUPS_MD"
}

@test "run18: reception completeness is DETECTION up to newest held, never delivery" {
  grep -qi 'DETECTION of incompleteness' "$GROUPS_MD"
  grep -qi 'newest held' "$GROUPS_MD"
  grep -qi 'best-effort' "$GROUPS_MD"
}

@test "run18: the withheld-tail limit is tied to the completeness-ahead doctrine" {
  grep -qi 'withheld TAIL' "$GROUPS_MD"
  grep -qi 'completeness-ahead' "$GROUPS_MD"
  # multimodal delivery is the mitigation, not a closure of the limit
  grep -qi 'multimodal delivery' "$GROUPS_MD"
}

@test "run18: open enrollment never weakens verification" {
  grep -qi 'open enrollment never weakens' "$GROUPS_MD"
  # verification requires no standing, only the envelopes
  grep -qi 'no standing' "$GROUPS_MD"
}

@test "run18: the swarm paragraph notes the second path turns a withheld tail from silent to detected" {
  grep -qi 'silent to detected' "$GROUPS_MD"
}

@test "run18: GROUPS.md and PUBLICATIONS.md point at each other" {
  grep -q 'PUBLICATIONS\.md' "$GROUPS_MD"
  grep -q 'GROUPS\.md' "$PUB_MD"
}

@test "run18: the degeneration principle is stated as binding" {
  grep -qi 'degeneration principle' "$PUB_MD"
  # open/single scopes ride the substrate's own records plus chaining only
  grep -qi 'atproto records' "$PUB_MD"
  grep -qi 'chaining' "$PUB_MD"
  grep -qi 'binding' "$PUB_MD"
}

@test "run18: the single-agent limit and the consent contrast with curator lists" {
  grep -qi 'collective noun' "$PUB_MD"
  grep -qi 'curator' "$PUB_MD"
  grep -qi 'consent' "$PUB_MD"
  # the provable multi-party fact is the one added atom
  grep -qi 'provable multi-party' "$PUB_MD"
}

@test "run18: the delta table distinguishes tamper-evident history from tamper-free current state" {
  grep -qi 'tamper-evident' "$PUB_MD"
  grep -qi 'tamper-free' "$PUB_MD"
  grep -qi 'authorization' "$PUB_MD"
}

@test "run18: the three-way retraction distinction is named" {
  grep -qi 'never-existed' "$PUB_MD"
  grep -qi 'retracted' "$PUB_MD"
  grep -qi 'withheld' "$PUB_MD"
}

@test "run18: the auditable-count claim is present (reach and churn re-derivable)" {
  grep -qi 'auditable' "$PUB_MD"
  grep -qiE 're-deriv|re-fold|refold' "$PUB_MD"
}

@test "run18: the honest scope of 'managing' is stated" {
  grep -qi 'honest scope' "$PUB_MD"
  grep -qi 'manag' "$PUB_MD"
}

@test "run18: PUBLICATIONS.md link-check passes (joins the broken-ref audit)" {
  [ -f "$PUB_MD" ]
  run "$KIT/scripts/link-check.sh" "$PUB_MD"
  if [ "$status" -ne 0 ]; then echo "$output"; fi
  [ "$status" -eq 0 ]
}

@test "run18: PUBLICATIONS.md anchors into GROUPS.md resolve to real headings" {
  anchors=$(grep -o 'GROUPS\.md#[a-z0-9-]*' "$PUB_MD" | sed 's/.*#//' | sort -u)
  [ -n "$anchors" ]
  headings=$(grep -E '^#{1,6} ' "$GROUPS_MD" \
    | sed -E 's/^#+ +//' | tr '[:upper:]' '[:lower:]' \
    | sed -E 's/[^a-z0-9 -]//g; s/ /-/g')
  for a in $anchors; do
    echo "$headings" | grep -qx "$a"
  done
}

@test "run18: the RUN-18 addendum is staged in proposed-changes and reviews-log" {
  grep -q 'RUN-18' "$DIR/../../../beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md"
  grep -q 'RUN-18' "$DIR/../../../beta/impl/experiments/drystone-reviews-and-experiments-log.md"
}
