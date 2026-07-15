# Phase 2 Findings — Data Model + Merge Semantics (the conceptual core)

**Date:** 2026-06-14
**Crates:** `lineage-core` (governance, DAG, survivor, conflict, keys),
`lineage-history` (per-branch signed history + backfill).
**Run:** `cargo test -p lineage-core -p lineage-history`

## Gate result: **GO** ✅

The two-tree model is implementable. Governance fork/heal with deterministic
survivor selection and conflict hard-stop is sound under adversarial
reordering; history-as-navigable-tree with verifiable consensual backfill works;
standing is decided from signed/structural data alone. **Proceed to Phase 3.**

## Invariant / experiment results

| Exp  | Proves | Invariant | Result |
|------|--------|-----------|--------|
| E2.1 | Under-threshold remove rejected by all honest clients, deterministically | I2 | **PASS** |
| E2.2 | Adding a member never confers admin standing; forged-genesis op rejected | I1 | **PASS** |
| E2.3 | Non-conflicting partition heals; survivor selection deterministic + symmetric | I10, I5 | **PASS** |
| E2.4 | Contradictory remove/keep hard-stops and escalates; no silent re-admit | I6 | **PASS** |
| E2.5 | Rejected conflict-merge leaves two valid groups, each with intact lineage/standing | I3 | **PASS** |
| E2.6 | Fresh-genesis inherits both parent logs read-only, unreordered | I7 | **PASS** |
| E2.7 | Entitled branch imports; forged signature and foreign genesis rejected | I8, I3 | **PASS** |
| E2.8 | Reconcile yields distinct navigable branches, not a merged scroll | (guard) | **PASS** |
| —    | Fold/unfold lossless and inert | I9 | **PASS** |
| —    | Survivor selection + conflict detection order-independent over 2000 seeded cases | I5 | **PASS** |

All of I1–I3 and I5–I10 hold in simulation. (I4 — forward-key linearity — is an
MLS property established in Phase 1, E1.2/E1.4; the governance layer is
deliberately separate from the key ratchet.)

## Design decisions and honesty boundaries

1. **Governance is fully reproducible.** Ed25519 signatures are deterministic
   (RFC 8032), and identities are seed-derived, so a signed governance op is
   bit-reproducible for a fixed key + message. The governance/history layer is
   therefore bit-reproducible — in contrast to the MLS key material (Phase 1
   boundary). This makes the "every honest client computes the same result"
   claims (I2, I5) directly testable.

2. **Genesis immutability is structural, not enforced at runtime (I1).** There
   is no `OpKind` that mutates the admin set or thresholds. Membership ops
   (`Add`) change *who is a member* but never *who is an admin*; E2.2 proves a
   freshly-added member's signature carries no standing. The "who decides who
   decides" regress is grounded at the root by construction.

3. **Survivor tiebreak uses a per-state head hash.** Two partitions of one
   group share a genesis id, so genesis alone cannot break ties. The experiments
   summarize each side by its current op-log head hash (a stable, signed-data
   derivative), giving a total, symmetric order. Determinism verified over 2000
   seeded cases.

4. **Conflict resolution is escalation-only (I6).** `conflict::reconcile`
   classifies (`Heal` vs `HardStop`) and routes every conflict to an `Escalator`
   callback. It never picks winners or re-admits/excludes anyone — the social
   dispute terminates at a human (or a quorum override built on top), exactly as
   the thesis requires (§1.4). A consensual `Leave` is intentionally *not* a
   conflict trigger; only a `Remove`-then-still-included contradiction is.

5. **No timestamp interleave is structural (E2.8).** `HistoryStore` keys
   branches by genesis and offers no merge-by-timestamp API at all. Reconcile
   can only add a *separate* navigable branch. The "six tapes" mistake is
   unrepresentable, not merely avoided.

6. **Automerge is deferred, deliberately (not assumed).** The thesis names
   Automerge for `lineage-history`. Its value is *in-branch* concurrent-edit
   convergence; none of the falsifiable invariants under test (I7 no-corrupt,
   I8 verifiable backfill, I9 fold, E2.8 no-interleave) require a CRDT — and are
   arguably *stronger* without a merge mechanism, since branches never merge.
   We implemented a signed append-only per-branch log and document that
   Automerge integration for concurrent in-branch editing is a later iteration.
   This keeps the Phase 2 dependency set minimal (matching Phase 0/1 discipline);
   `ed25519-dalek` was already in the lockfile transitively, so Phase 2 added no
   new third-party license surface.

## What this does and does not establish

**Establishes:** governance threshold soundness and genesis immutability;
standing from signed/structural data; deterministic, negotiation-free survivor
selection; conflict detection that hard-stops to a human; verifiable consensual
backfill with no cross-branch interleaving — all in deterministic simulation.

**Does NOT establish:** behavior over a real transport (Phase 3), the binding of
governance enactment to actual MLS commits under live partition/reconnect
(the open seam at the two-tree binding), total-device-loss recovery, scale at
large branch counts, or UX of the fold/escalation flows. Those remain Phase 3
or explicitly beyond this validation.
