# Convergence finding — `idx_nodes` card identity was ingest-order-sensitive (2026-06-26)

Surfaced while strengthening the Stage-7 property suite (multi-device generators
+ rebuild-equality assertions) during the PR #11 → `experiments` integration.
This is the storage-layer twin of the protocol's ordering problem, so it is
cross-referenced from `discovery/beta/OPEN-THREADS.md` (T29 / convergence).

---

## Invariant at stake

- **I2 / I5 — order-insensitive convergence.** Two peers that hold the *same set*
  of governance facts must derive the *same* projection, regardless of the order
  in which those facts arrived.
- **I3 / I6 — rebuildability.** `rebuild()` re-folds the authoritative log in
  canonical order and must reproduce the live projection byte-for-byte.

I3 is the cheap local detector for an I2 violation: if a value depends on ingest
order, then the live fold (arrival order) and `rebuild()` (canonical order)
disagree, and the rebuild check fails.

## The bug

In the derived fold, a node card's *identity* fields — `created_at` and
`created_by` on `NodeCard` — were written **first-touch-wins**: whichever
assertion happened to be folded first stamped them, and later assertions left
them untouched.

```
peer A folds:  e_create(node N, t=…001, by=P)  then  e_update(node N, t=…003, by=Q)
               → card.created_at = …001   (create seen first)

peer B folds:  e_update(node N, t=…003, by=Q)  then  e_create(node N, t=…001, by=P)
               → card.created_at = …003   (update seen first; create can't lower it)
```

Same two facts, two arrival orders, **two different cards** → divergence. The
diagnostic pinned a Principal card with `created_at` LIVE=…003 vs REBUILD=…001.

This is exactly the distributed-ordering tension: with no clock and no server,
"first one I saw" is not a function of the *facts*, only of the *network*.

## The fix

Make the identity fields a **canonical, commutative reduction** instead of
first-touch. `(created_at, created_by)` becomes the lexicographic **MIN** over
all assertions touching the node — monotonic-down, order-independent, and
identical under every fold order and under `rebuild()`'s canonical order.

- `upsert_node_stub` — if a card exists, lower `(created_at, created_by)` to the
  min; never raise it.
- `upsert_node_full` — compute `(eff_at, eff_by)` as the min of the incoming and
  existing identity before writing.
- `present` stays **monotonic-up** (a node that has ever been observed present
  stays present); only the *identity* fields changed semantics.

This mirrors the protocol's own sequencer: where the wire layer breaks ties by
content hash, the derived layer breaks identity ties by `(created_at,
created_by)` min. Same medicine, different layer.

## Why the tests missed it before

The pre-existing scale tests folded with a **single device verifier**, so
multi-device causal interleavings (the orders that expose the bug) were never
generated, and the rebuild-equality assertion on `idx_nodes` was disabled. The
fix to the *harness* (a `MultiVerifier` that delegates per device + diverse
multi-member/multi-device generators) is what made the bug reproducible, and the
re-enabled `prop_diverse_rebuild` / `prop_diverse_partial_prefix` rebuild checks
are what now guard it.

## Regression guards (committed)

- `tests_stage7.rs::prop_diverse_rebuild` — full-log rebuild equality over diverse
  multi-device scenarios (un-ignored).
- `tests_stage7.rs::prop_diverse_partial_prefix` — rebuild equality on any causal
  prefix (I9 + I3).
- `tests_stage7.rs::diag_rebuild_divergence` — per-table live-vs-rebuild check
  (was the repro; now a fast deterministic regression test).
- `proptest-regressions/tests_stage7.txt` — saved failing seeds (seed=22;
  seed=0,k=7; seed=21) re-run before novel cases.

## Takeaway for the spec

Every derived field must be defined as a **commutative, canonical reduction** of
the facts, not an artifact of fold order. "Last/first writer wins" is a clock in
disguise, and Drystone has no clock. This is a concrete, code-level confirmation
of the Part 2 §7 ordering discipline and couples T29 (MLS↔log binding under
concurrent commits).
