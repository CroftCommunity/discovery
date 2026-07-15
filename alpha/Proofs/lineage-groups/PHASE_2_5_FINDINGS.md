# Phase 2.5 Findings — Adversarial pass

**Date:** 2026-06-14
**Run:** `cargo test -p lineage-core --test adversarial -p lineage-history --test backfill_adversarial`

A short, targeted pass aimed at *falsifying* invariants rather than confirming
them — and two of the three found real gaps, which we then closed with the
capability the thesis claims (not paper over). All 37 workspace tests pass.

## Results

| Exp  | Probes | Outcome |
|------|--------|---------|
| A2.1 | Order-independent convergence (I10/I5) under fuzzed delivery: a branch's forward-only log, delivered in 300×4 random orders with buffering, must converge to the identical head + members | **PASS** — held; no order-dependence found |
| A2.2 | Attributable detection of an admin that **equivocates** (signs two conflicting ops at the same chain position) | **Gap found → closed.** Added `detect_equivocation`; now detected and attributed |
| A2.3 | Backfill of a well-signed but illegitimate branch (author never held standing; or gapped/reordered sequence) | **Gap found → closed.** Strengthened `backfill_import`; now rejected |

## What the gaps were, and how they were closed

### A2.2 — governance equivocation (was undetected)
The thesis calls the governance tree "fork-detecting, attributable," but
`GroupState` only validated each replica's *own* forward-only chain — it had no
way to recognize that an admin had signed two different ops at the same `seq`.
That is the classic equivocation/fork attack, and it was silently
representable.

**Closed** with `gov::detect_equivocation(a, b, dir)`: given two signed ops on
the same lineage at the same `seq` with different op ids, it returns one
attributable `Equivocation { culprit, genesis, seq, op_lo, op_hi }` per DID that
*validly signed both*. It is symmetric, ignores identical ops, and attributes
only real signers (a forged second signature names only the genuine
double-signer). This is the explicit, attributable fork signal — never a heal.

### A2.3 — backfill provenance was signature-deep only (was insufficient)
`backfill_import` verified per-message signatures + shared genesis, but a valid
signature is necessary, not sufficient. Two illegitimate branches slipped
through the original check:

1. **Unauthorized author** — a message validly signed by a DID that never held
   standing on the lineage (e.g. a stranger's perfectly-signed "history").
2. **Tampered ordering** — gapped or reordered `seq`s, each message individually
   well-signed.

**Closed** by strengthening `backfill_import` to additionally require, before
absorbing anything: (a) contiguous sequence (`message[i].seq == i`,
`BackfillError::NonContiguous`), and (b) author standing on the branch's lineage
via `Lineage::standing` (`BackfillError::UnauthorizedAuthor`). The existing E2.6
/ E2.7 / E2.8 legitimate-backfill tests still pass unchanged, so the checks
reject forgeries without rejecting real history. This directly addresses the
plan's open question — "validating an epoch chain you were not present for" —
by binding acceptance to recorded standing + structural integrity, not faith in
a signature.

## What held (A2.1)
Convergence is robust: across 300 seeds × 4 replicas, a branch's signed log
delivered in arbitrary shuffled order (with buffer-until-chained) always reached
the same head and member set as the canonical in-order application. The
forward-only hash chain makes per-branch convergence order-independent, as I10
requires — no negotiation, no divergence.

## Honesty boundaries

- `detect_equivocation` detects a *pairwise* conflict between two ops; wiring it
  into a continuous gossip feed (track each admin's `(seq → op id)` and flag the
  first contradiction) is a small follow-on, not done here.
- Equivocation detection attributes any DID that double-signed; it does not by
  itself decide consequences (eviction, fork promotion) — that remains a
  governance/escalation policy choice, consistent with the escalation-only
  stance from Phase 2.
- The standing check uses *ever-held standing on the lineage*, not standing *at
  the exact epoch a message was written*; per-epoch membership-at-time is a
  stricter check left as future work.
