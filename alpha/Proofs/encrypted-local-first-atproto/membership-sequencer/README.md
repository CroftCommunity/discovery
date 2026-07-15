# Membership Sequencer / Delivery Service (Phase 10)

Phase 7 proved MLS membership must be totally ordered (a fork otherwise) and used
an ad-hoc tiebreak. This builds the explicit **delivery-service sequencer** that
provides the order — the production answer to "who decides commit order?"

```
cargo run     # found -> sequential adds -> 3 concurrent adds drained -> converge
```

## How it works

The `Sequencer` holds the authoritative epoch counter and an ordered log of
accepted commits. A committer stages an add against its current epoch and submits
the commit:

- **Accepted** iff it targets the sequencer's current epoch → the epoch advances,
  the commit is appended to the canonical log.
- **Rejected** otherwise → the committer catches up (applies the accepted commit)
  and re-submits against the new epoch.

So concurrent commits are linearized: exactly one per epoch, totally ordered by
the sequencer; the group **cannot fork**. It only ever sees commit messages
(membership metadata), never content (which stays E2E-encrypted / CRDT).

## Lifecycle (all 3 checks PASS)

- Alice founds the group; Bob and Carol are added sequentially through the sequencer.
- **Three concurrent adds at one epoch** — Alice+Dave, Bob+Erin, Carol+Frank — are
  submitted together. The sequencer accepts one and rejects two each round; the
  losers re-submit. The queue drains in **3 rounds**, every proposal commits
  (liveness, no starvation).
- All six members converge to the **same epoch (5), membership, and content key** —
  no fork. The sequencer's accepted log (5 commits) is the canonical total order.

## Issues surfaced

1. **The sequencer is a required role and a single point of ordering.** It must be
   available; a malicious sequencer can **censor or reorder membership** (it
   cannot read content). Candidates: the superpeer, or a designated/rotating
   member. Trust + availability of this role is now a first-class design concern.
2. **Wasted work under contention.** Each losing committer stages a commit that is
   discarded and re-staged (with a fresh key package). N concurrent proposals take
   N rounds; throughput is one membership change per epoch.
3. **Ordering policy lives in the sequencer** (here: first-submitted wins). A real
   DS needs an explicit, auditable ordering rule and probably backpressure.
4. **Content sync stays unordered/P2P (CRDT)** — only membership needs the
   sequencer, so the local-first property is preserved for the data path. This is
   the same split Phase 7 identified: order membership, merge content.

## Resolved versions

rustc 1.94.1 · openmls 0.8.1.
