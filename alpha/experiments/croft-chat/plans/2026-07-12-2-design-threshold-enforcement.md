# Design note: k-of-n governance threshold enforcement (V5′)

`Status: proposal. Motivated by the threshold-not-enforced refutation
(threshold_not_enforced.rs). The lineage-counting core it needs is built and tested
(governance::count_personae_by_lineage / threshold_met). The remaining piece — how k
personae approve one act — is a genuine representation fork and wants a decision.`

## The finding

Groups carry k-of-n thresholds (`add_member_threshold`, `remove_member_threshold`,
`role_change_threshold`, `rule_change_threshold`), they are amendable at position (I6),
but they are **not enforced**: `check_authorization` proxies "threshold satisfied" as
"author is Owner/Admin" (`let _ = current_threshold`). A single Owner satisfies a 2-of-n
add. So the values are decorative.

V5′ is really two things, and only the second is novel:
1. **Enforcement** — actually require k approvals for a threshold-k act. A build.
2. **Lineage counting** — the k must be *distinct personae by lineage, never clients*
   (§5.7), so a single persona cannot meet a k-of-n by signing with many devices. This
   is the security property, and the one the spec calls out.

## What is built (low-risk, done)

`governance::count_personae_by_lineage(&[PrincipalId]) -> usize` and
`threshold_met(&[PrincipalId], required) -> bool`. Pure, mutation-scoped, tested: two
personae meet a 2-of-n; one persona signing three times (its three clients) does **not**;
callers must pass principals resolved from devices (the credential resolver does this),
so device count cannot inflate the count. This is layer 2's counting core — independent
of how approvals are represented.

## The representation fork (needs a decision)

How do k distinct personae approve one governance act? Two shapes fit the append-only
fold; they have different wire and UX consequences.

### Option A — approval facts (proposal → quorum)

A governance act is authored once (the proposer). It is **pending** until `threshold_met`
distinct personae have emitted an `Approval` assertion referencing its hash; the fold
applies it at the moment the quorum is reached.

- *Pros:* append-only-native; approvals are ordinary signed facts; naturally async
  (personae approve over time); composes with the antecedent DAG and the existing
  pending/heal machinery (an act below quorum is simply not applied yet, like a held
  fact).
- *Cons:* introduces a new assertion type and a "pending act" state; the derived state
  must track partial approvals; interacts with concurrency (two competing acts each
  gathering approvals — does that re-enter the §7.6.1 contradiction path?).

### Option B — co-signed envelope

One envelope carries k signatures from k distinct personae. Authorization counts the
distinct signing principals against the threshold.

- *Pros:* one fact, atomic — no pending state; simpler derived state.
- *Cons:* a wire-format change (the envelope has one `author`/`signature` today); requires
  collecting all signatures *before* submission (synchronous, worse UX for async groups);
  less natural for the local-first, gather-over-time model.

**Recommendation: Option A** (approval facts). It fits the append-only, async, local-first
model and reuses the pending/heal posture; the "pending until quorum" state is the honest
analogue of the antecedent-guard's "held until complete." But it is the larger change and
introduces a new fact type + partial-approval derived state, so it wants a green light
before touching the mutation-vetted fold.

## Enforcement hook and interactions

- `check_authorization` (or a step after it) consults `threshold_met` over the act's
  approver set instead of the Owner-proxy. The approver set is the distinct principals of
  the act's author + its admitted `Approval`s (Option A), or the envelope's signers
  (Option B).
- **Concurrency:** two competing threshold-k acts each reaching quorum concurrently is a
  §7.6.1 contradiction — the existing detect/resolve machinery should extend to it once
  acts can be "gathering approvals."
- **I6:** the threshold value is already checked at position; enforcement makes that value
  bite.

## Status (2026-07-12) — Option A IMPLEMENTED (approvals-as-antecedents variant)

Chosen: Option A, in the form that reuses the antecedent guard rather than a separate
pending-act table. An `Approval` fact (type `0x000B`; payload = approved act_type ‖
subject principal) approves "an act of this type on this subject". A threshold-k act
references k such approvals as its **antecedents**, so the existing antecedent guard holds
the act until the approvals are present; then `fold_derived` Step 5.6 counts distinct
approver personae (`count_personae_by_lineage` over the act's author + its matching
antecedent-approvals) and rejects with `FoldError::ThresholdNotMet` below quorum. No
pending table, no trigger path — enforcement rides the guard.

- `AssertionType::Approval`; `threshold_for` (rules at position); `act_subject`,
  `approval_matches`, `gather_approvers`; the Step 5.6 gate; an `Approval` authorization
  arm (approver must be Owner/Admin) in both `check_authorization`s; a no-op derived
  effect (approvals are evidence, not state).
- **Verified** (`threshold_enforced.rs`): sole Owner fails a 2-of-n (X absent); Owner + a
  second admin's approval meets it (X present); Owner's own self-approval does not
  (still one persona). The by-device lineage guard is unit-tested in governance.
- **Consequence handled:** four completeness tests used `add_member_threshold → 7` as an
  arbitrary divergence value; now that it *gates* adds, they were switched to
  `remove_member_threshold` (which does not gate the add), preserving each test's mechanism.

**Deferred:** approver-role granularity (currently Owner/Admin, not per-act-bar); the
concurrency interaction (two competing threshold-k acts each reaching quorum → a §7.6.1
contradiction); and revocation of an approval before quorum.

**Update (2026-07-13) — RuleChange enforcement DONE.** The "no principal subject" gap is closed:
a RuleChange's approval subject is a **content hash of its payload** (`rule_change_approval_subject
= blake3(rule_key ‖ new_value)`), so approvers name `(RuleChange, H(payload))` — the rule-keyed
approval this note asked for, generalized to the whole proposed change. `act_subject` returns it and
Step 5.6 enforces via the same distinct-personae path. Proven by `rulechange_threshold_enforced.rs`
(4 cases) + the strengthened substrate `test_i6`. See the master ledger Phase 8 and
`alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` (Reconciled). The concurrency interaction remains deferred.
