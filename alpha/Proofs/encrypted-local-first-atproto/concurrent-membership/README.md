# Concurrent Membership + CRDT Fork Resolution (Phase 7)

The highest-risk unknown, open since Phase 1: what happens when two members act
concurrently? It splits into two very different problems, and the answer is a
clean design split.

```
cargo run     # concurrent commits -> resolve; concurrent edits -> merge; compose
```

## The two concurrency stories

### MLS membership is single-writer per epoch (must be ordered)

Two members committing against the same epoch **fork** the group, and MLS does
*not* self-heal: openmls rejects a commit that references a superseded epoch. So
a **total order** (a delivery service / sequencer) is required.

This phase models it: Alice stages "add Carol" and Bob stages "add Dave", both
against epoch 1 (neither merges). A deterministic tiebreak — the
lexicographically-least commit message — picks the winner (a stand-in for a DS's
ordering). Then:

- **Winner** merges its own pending commit → epoch 2.
- **Loser** aborts (`clear_pending_commit`), applies the winning commit (→ epoch
  2), and **re-proposes** its add against epoch 2 → epoch 3.
- The newly-added members join via the surviving Welcomes; the loser's original
  Welcome is discarded as stale.

Result: all four members (Alice, Bob, Carol, Dave) converge to **the same epoch
(3), the same membership, and the same exporter-derived content key**.

### Automerge content is a CRDT (no ordering needed)

Concurrent content edits **merge automatically and deterministically** — no
fork, no lost writes. From a shared base, Alice appends "from alice" and Bob
appends "from bob" concurrently; after exchanging changes both converge to the
identical document (`["base","from bob","from alice"]` on both sides, same heads).

### They compose

The merged content is opaque to epoch churn: it is encrypted under the resolved
epoch-3 key, and Dave — added via the re-proposal — reads it back. **Order
membership (MLS); merge content (CRDT).**

## Lifecycle (all 3 steps PASS)

1. Concurrent commits at epoch 1 → deterministic winner → loser aborts + re-proposes → all 4 converge to epoch 3, identical membership, identical key.
2. Concurrent CRDT edits → auto-merge to identical state on both peers, all 3 writes preserved.
3. Converged content encrypts under the resolved key; the newly-added member reads it.

## Issues surfaced

1. **MLS forks are not self-healing.** Two commits on one epoch diverge; a commit
   from a superseded epoch is rejected. A **total order is mandatory** — a real
   deployment needs a delivery service / sequencer (or a single designated
   committer). The lexicographic tiebreak here is a stand-in for that order.
2. **The loser must abort and re-propose.** `clear_pending_commit`, then apply
   the winner, then re-issue the proposal against the new epoch. Its Welcome and
   any proposals/app-messages from the losing epoch are **invalidated** and must
   be re-sent — real bookkeeping for clients.
3. **KeyPackages are single-use.** A re-proposed add needs a fresh key package
   (handled here by generating one per add); blindly reusing the original fails.
4. **The design split is the takeaway.** Membership needs ordering; content does
   not. Putting CRDT content under MLS-keyed AEAD composes cleanly precisely
   because the two layers have different concurrency models.
5. **Epoch-bound ciphertext vs. epoch-agnostic CRDT.** Post-rotation writes must
   re-encrypt under the new key. And MLS forward secrecy means a *removed* member
   can still decrypt content encrypted under epochs it belonged to — so "remove
   member" is not "redact their access to past content". Anything requiring true
   revocation of past data needs re-encryption + re-distribution, not just a
   membership change.

## What this implies for the architecture

- A **sequencer/delivery-service role** is not optional for membership — it must
  exist (the superpeer, or a designated committer per group). This was noted as
  out-of-scope in earlier phases; Phase 7 shows it is load-bearing.
- Content sync can stay **fully peer-to-peer and unordered** (CRDT), which keeps
  the local-first property intact even though membership is ordered.

## Resolved versions

rustc 1.94.1 · automerge 0.7.4 · openmls 0.8.1 · chacha20poly1305 0.10.1.
