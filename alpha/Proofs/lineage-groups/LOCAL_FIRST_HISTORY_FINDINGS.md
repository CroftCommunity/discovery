# Local-first history — multi-device & group voluntary backfill, demonstrated on real machines

date: 2026-06-15

**Claim under test:** message history is *local-first* — each device/member keeps its own
append-only signed branch, and reconciliation is **voluntary consensual backfill**: another party's
branch is absorbed as a *separate, navigable* branch, **never** spliced into one canonical
transcript. The *same* mechanism serves a single user's multiple devices (separate keys bound to one
ancestor) and a group of distinct people. This is the design's answer to the rigidity of
one-true-transcript systems: divergence stays livable, and sync is a choice, not a forced merge.

**Verdict: demonstrated** on the three real boxes plus the rejection paths.

## Method

Substrate: the (TDD-tested) `lineage-core::dag::Lineage` (shared-ancestry + standing) and
`lineage-history` (`BranchHistory` append/fold, `HistoryStore::backfill_import`). A throwaway,
TDD-exempt `crates/history-harness` wraps them: every party derives the **same** shared root genesis
and its own forked branch deterministically (so all machines agree), writes signed messages, and
serializes the branch. On another machine, `merge` absorbs donor branches with full verification.

- One shared **ancestor** (root genesis `croft-shared-lineage-root-v1`); each party forks its own
  branch off it (`croft-branch:<name>`), with its own Ed25519 key — *separate keys, one ancestor*.
- "Sync" = exchange branch JSON (relayed via the Mac) and `backfill_import` locally — voluntary.

## Results (on the boxes)

**Multi-device (node-1=dev1, node-2=dev2, node-3=dev3 — one user, three devices):** each box wrote
its own branch, exchanged, and merged the other two. Every box ended with **3 separate navigable
branches** (`27d28170` dev1, `33c2700d` dev2, `16c00ab3` dev3), identical branch ids across machines,
**never interleaved**. Fold demo on each: a branch's visible count goes `N → 0 → N` (folding hides it
from the daily view losslessly; unfolding restores it — local-first, no ambient pressure).

**Group (alice/bob/carol — distinct people, same mechanism):** bob voluntarily absorbed alice's and
carol's branches as separate navigable branches — the identical `backfill_import` path, no special
"group" code. Carol could not attend yet still gifted her history; bob chose to take it (voluntary).

**Rejection (verification is load-bearing):** when alice tried to absorb a **tampered** carol branch
(one signature byte flipped) and an **outsider** (mallory, never in the lineage):
- tampered → `REJECTED … message 0 … failed signature verification` (`BadSignature`);
- outsider → `REJECTED … donor branch does not share a genesis with the recipient's lineage`
  (`ForeignGenesis`).
Unverifiable or foreign history is never taken on faith.

Evidence: `local-first-history-evidence/*.json` (sample branches incl. the forged + outsider ones).

## What this means (capabilities)

- **Multi-device with separate keys is the same as group sync.** There is one mechanism, not two:
  "my other phone" and "another person in the group" are both just another branch off the shared
  ancestor that I may choose to absorb. No device-linking ceremony distinct from group membership.
- **Sync is voluntary and additive.** You pull the history you want; nothing is forced on you, and
  absorbing a branch never rewrites what you already hold (I7 — history never corrupts).
- **Divergence is livable, not broken.** Branches coexist as navigable, foldable threads; there is no
  forced interleave that turns concurrent history into "six tapes playing at once," and no canonical
  transcript whose ordering must be agreed. This is the rigidity escape: a fork is a resting state.
- **Trust is structural.** A branch is absorbed only if it shares the lineage's root *and* every
  message verifies against an author who held standing — so backfill can be offered by anyone without
  becoming an attack surface.

## Honesty boundary

The transport is branch-file exchange (relayed via the Mac), not a live sync protocol — the
*computation* (verification, no-interleave absorption, fold) runs on the real separate machines; the
*delivery* is out of scope here (that is the iroh transport work in `experiments/iroh`). Automerge
for in-branch concurrent-edit convergence is deferred (see `PHASE_2_FINDINGS.md`); the properties
shown here deliberately do not require a CRDT.
