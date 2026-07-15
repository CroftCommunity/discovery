# Phase 2.6 Findings — authority lifetime & governed override

**Date:** 2026-06-14
**Run:** `cargo test -p lineage-core --test authority`

Two follow-on experiments squeezed in after Phase 2.5, both aimed at the
*authority* edges the earlier suites never pushed on: how long admin authority
lasts, and how a hard-stop conflict is allowed to end. One found a real gap (the
code contradicted its own documented invariant); the other built the governed
override the thesis names but never had.

## Results

| Exp  | Probe | Outcome |
|------|-------|---------|
| A2.4 | Does a genesis admin who has been **removed from the group** still govern? | **Gap found → closed.** Authority is now per-epoch: a departed admin's signature no longer counts |
| A2.5 | Can a hard-stop conflict be resolved **only** by an explicit, threshold-meeting signed decision (never automatically)? | **PASS** — built `quorum_override`; below threshold the hard-stop stands |

## A2.4 — departed-admin authority (was unbounded)

`gov.rs` documents **I2** as: an op is enacted iff it carries threshold
signatures "from admins with standing **in the current epoch**." But
`GroupState::validate` only checked `genesis.rules.admins.contains(did)` — the
*immutable* genesis set. Nothing required the signer to still be a member. So a
genesis admin who had been **removed (or had left)** retained full governance
authority forever: a compromised-then-evicted admin key could keep signing
Adds/Removes. The code contradicted its own stated invariant.

**Closed** by making standing two-fold in `validate`: a signer must be in the
genesis admin set (I1, unchanged) **and** be a current member of this branch.
A departed admin is now rejected with a distinct, attributable
`RejectReason::DepartedAdmin(did)` — separate from `SignerLacksStanding` (never
an admin) so the two failure modes don't conflate. Their signatures also stop
counting toward thresholds (`valid_admin_sigs` / `meets_threshold`).

This is *not* a weakening of I1: the admin **set** is still immutable and still
anchored at genesis; membership operations still never expand it (E2.2 holds
unchanged). What changed is that authority is now **scoped to the epoch you are
actually present in** — exactly what the I2 text always claimed. All prior tests
still pass because no earlier scenario had a removed admin sign a later op.

Note this is naturally partition-local: an admin removed on the left branch
loses authority *there*, while still governing on a right branch that never
booted them — consistent with the two-valid-groups resting state from Phase 2.

## A2.5 — quorum override (new capability)

`detect`/`reconcile` only ever classify and escalate; they never change
membership (I6). The thesis says the escalation hook is where "a human (or a
quorum-approved override) decides" — but there was no structured override, only
the human callback. A2.5 builds it as the *disciplined* form of that decision.

`conflict::quorum_override(reasons, decisions, group, dir)` resolves a hard-stop
**only** when, for every contested member, the quorum supplies a `Decision`
whose signed op (a) names that member, (b) is the matching kind
(`Readmit→Add`, `ConfirmRemoval→Remove`), and (c) **meets the genesis
threshold** against the group's current admins-with-standing. It is
all-or-nothing and mutates nothing — it authorizes, it does not apply.

The test pins the properties that matter:

- **No decision → hard-stop stands** (`Unresolved`).
- **Under-threshold decision → hard-stop stands** (a lone `ConfirmRemoval`
  signature when Remove needs two is refused).
- **Threshold-meeting decision → resolved**, recording the governed outcome
  (`ConfirmRemoval` keeps the boot; `Readmit` brings the member back — but only
  via an explicit, signed Add, never automatically).
- **Wrong subject → not authorized.**

The algorithm still never picks a winner. It only verifies that the humans
signed for one, at the strength the genesis rules require. Re-admission is
therefore *possible* but always explicit and attributable — the anti-silent-
re-admit stance of I6 is preserved, not bypassed.

## Honesty boundaries

- `quorum_override` *authorizes* a resolution; it does not append the decision
  to either chain or merge the two branches. Applying it is an ordinary
  governance `apply` on the surviving chain, deliberately left out of scope here
  (the interesting property is the authorization gate, not the bookkeeping).
- The departed-admin check uses *current membership on the branch being
  evaluated*. It does not yet model an admin who is re-Added later regaining
  authority beyond the plain consequence that they are a member again; that
  re-entry path is untested here.
- Threshold evaluation in `meets_threshold` intentionally ignores chain
  position (seq/prev) — it answers "is this decision strong enough," not "does
  it slot into the log here." Chain placement remains `validate`'s job.
