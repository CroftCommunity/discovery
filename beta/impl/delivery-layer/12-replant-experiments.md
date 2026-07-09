# Experiment plan: tree re-plant and atomic group swap

`Status: experiment plan, pre-execution`

`Purpose: demonstrate that Drystone can atomically swap the MLS key-distribution tree at a scheduled boundary without disturbing conversation, membership, or governance continuity, and that a freshly planted tree resets the blank-node cost that an evolved tree accumulates.`

`Companion to: 01-delivery-architecture.md, 08-experiment-methodology.md (fidelity ladder), 10-experiments-round2.md (prior round, mls-rs 0.55.2).`

`Stack: mls-rs 0.55.2 (as validated in round 2), Rust. Fidelity target: Rung A (real library) for the MLS mechanics; Rung B (model-form) acceptable for the governance-chain and dataplane hash structures, which are Drystone's own and not yet built.`

---

## The idea under test

The design demotes the MLS group to a disposable key-distribution layer. Three durable structures carry all continuity and none of them is keyed to the MLS tree's shape: the **dataplane** (conversation history, a hash tree), the **current-governance chain** (membership and roles, a hash tree), and the **history-governance chain** (how governance evolved, a hash tree). They are independent and linked only by application-layer logic.

The claimed operation: at a deterministic boundary (every N messages), read the current membership from the governance chain, instantiate a fresh MLS group seeded with those members, cut down the old group, and carry continuity forward through the three hash structures plus the resumption PSK. No replay, no reconstruction of state from the tree, an atomic swap of the key layer under a continuous conversation.

The claimed payoffs: (1) a fresh tree has no blanks or unmerged leaves, so the O(log n) to O(n) re-key drift from removal-blanking and add-accumulation is reset; (2) because each fresh group draws a fresh KeyPackage per member, the swap rotates every member's leaf key, so it is also a group-wide key refresh, not only a structural reset; (3) history, membership, and governance survive the swap untouched, so the UX can present one seamless conversation across many underlying trees.

## Grounded facts these experiments build on

- Group creation is unilateral and needs a KeyPackage per added member; adds are batched into one Commit with per-member Welcomes. *Verified (RFC 9420 §Protocol Overview).*

- KeyPackages are single-use and SHOULD NOT be reused, except a *last resort* KeyPackage which exists precisely for the case where no fresh package is available. *Verified (RFC 9420 §10, §16.8; mls-extensions).* This is the availability escape hatch for seating an offline member at a boundary.

- MLS ships reinitialization (close old group, same members, new parameters) and branching (new group from a subset, old group untouched), both linked by a resumption PSK that proves membership continuity. *Verified (RFC 9420 §11.2, §11.3, §8.4).*

- mls-rs 0.55.2 exposes group creation, add-members, and reinitialization/branching. *Verified (crate docs); the exact re-init API surface is [confirm] until exercised in E12.1.*

---

## Experiments

### E12.1 Baseline: stamp a fresh group from a member set

**Question.** Can we instantiate a fresh group over N members read from an external source (standing in for the governance chain), and confirm all N can seal and read, with a fresh tree that reports zero blanks?

**Method.** Generate N members each with a fresh KeyPackage. A designated planter creates a group, batch-adds all N-1 others in one commit, distributes Welcomes, and all members process. Assert every member reaches the same epoch, can encrypt and decrypt a round-trip message, and that the tree has no blank nodes and no unmerged leaves at epoch 1.

**Confirms.** The stamp-out operation works and starts pristine. Records the O(N) instantiation cost (commit size, Welcome count and bytes, wall time) as the per-boundary baseline for tuning N.

**Fidelity.** Rung A (real mls-rs).

### E12.2 The atomic swap preserves conversation continuity

**Question.** After a conversation accrues messages in group G1, can we stamp a fresh G2 over the same membership, cut down G1, and have the conversation continue in G2 with the full prior history still verifiable, without replaying anything into the tree?

**Method.** Build G1 over N members. Seal M application messages, appending each to a model-form dataplane hash chain (author-signed records, as in round 2). At the boundary, read the member set, stamp G2 with fresh KeyPackages, and derive a resumption PSK link from G1 to G2. Continue sealing messages in G2, appending to the *same* dataplane chain. Assert: (a) the dataplane chain verifies end to end across the G1 to G2 boundary with no discontinuity; (b) no message content was re-encrypted or replayed during the swap; (c) a member can verify, via the resumption PSK, that G2's membership descends from G1's.

**Confirms.** History continuity is a property of the dataplane hash structure, not the tree. The swap moves the key layer without touching the conversation.

**Fidelity.** Rung A for MLS G1/G2 and the PSK link; Rung B for the dataplane hash chain (Drystone's own structure).

### E12.3 Byte-nondeterminism across planters is irrelevant

**Question.** If two members independently stamp G2 from the same governance-chain member set, they may produce different tree bytes (different KeyPackage selection). Does anything downstream break?

**Method.** Two planters each stamp a G2 candidate from the identical member set. Confirm the trees differ in bytes (or confirm they can). Then show that the dataplane chain, the membership set, and the governance reference are identical regardless of which candidate is adopted, and that the content-hash tie-break (as used for delivery dedup in round 1) selects one candidate deterministically. Assert that a member who adopted candidate A and a member who adopted candidate B, once converged on the tie-break winner, reach an identical working group.

**Confirms.** The tree carries no downstream-load-bearing state, so planter divergence is a dedup, not a fork. This is the core correctness claim of the whole approach.

**Fidelity.** Rung A for the two trees; Rung B for the tie-break (reuses the round-1 dedup mechanism).

### E12.4 Fresh tree resets blank-node cost

**Question.** Does a re-plant actually reset the re-key cost that an evolved tree accumulates through removals?

**Method.** Build a group, then drive it through a churn sequence (repeated add/remove without intervening full-path commits) to accumulate blanks and unmerged leaves. Measure the re-key cost signal (number of distinct public keys a commit must encrypt to, i.e. the resolution size along the path, or the UpdatePath ciphertext count as a proxy) as it drifts upward. Then perform a re-plant and measure the same signal on the fresh tree. Assert the fresh-tree cost returns to the pristine O(log n) baseline from E12.1.

**Confirms.** The re-plant is a genuine reset of the blank/unmerged pathology, quantifying the payoff that motivates the whole move.

**Fidelity.** Rung A. Note: if mls-rs does not expose resolution size directly, use UpdatePath ciphertext count or Welcome/commit byte size as the observable proxy, and say so.

### E12.5 Fresh tree rotates every member's leaf key (PCS side)

**Question.** Does stamping from fresh KeyPackages rotate every member's leaf key material, so the swap is a group-wide key refresh and not only a structural reset?

**Method.** Record each member's leaf public key in G1. Stamp G2 from fresh KeyPackages. Assert every member's leaf key in G2 differs from G1. Separately, seat one member in G2 from a *last-resort* (reused) KeyPackage and confirm that member's key does *not* rotate, documenting the honest exception: the offline-member escape hatch trades key freshness for availability.

**Confirms.** The favorable answer to the earlier PCS question, with its one honest exception named. Feeds the governance decision about how often to force fresh-package publication versus tolerate last-resort seating.

**Fidelity.** Rung A.

### E12.6 KeyPackage availability at the boundary (the center-free constraint)

**Question.** What happens at a boundary when a member has no fresh KeyPackage fetchable? Confirm the last-resort package seats them, and characterize the cost.

**Method.** Stamp G2 where K of N members have a fresh package and N-K have only a last-resort package. Assert all N are seated and functional. Record which members got key rotation (the K) and which did not (the N-K), tying back to E12.5. Model the availability question: in a center-free setting a member's fresh package must have been pre-published and be reachable at boundary time; the last-resort package is the floor that guarantees seating regardless.

**Confirms.** The one new constraint the center-free setting adds is bounded by an existing MLS mechanism, so the swap never blocks on an unreachable member; it only trades that member's PCS refresh for availability until they republish.

**Fidelity.** Rung A for seating; Rung B for the "reachable at boundary" modeling (transport is out of scope here).

### E12.7 Governance continuity across the swap (Drystone-side proof)

**Question.** The MLS resumption PSK proves *membership* continuity. Does the governance state (roles, weights, whatever the current-governance chain holds) survive the swap verifiably, not just by assertion?

**Method.** Model-form: attach a governance-chain position (a hash) to G1's context, carry it into G2's initial context (via a group context extension or the app-layer binding), and assert a member can verify G2 references the same governance-chain head that authorized the swap at boundary N. Show that tampering with the carried governance reference is detectable.

**Confirms.** The seam is two independent proofs, MLS membership continuity (PSK) and Drystone governance continuity (chain reference), and both hold. This is the load-bearing Drystone-side claim, and it is explicitly Rung B because the governance chain is not yet built.

**Fidelity.** Rung B (Drystone's own governance structure, modeled).

---

## What these do and do not show

**Do.** That the atomic swap is buildable on the validated stack; that continuity lives in the three hash structures and the PSK, not the tree; that planter byte-divergence is a dedup not a fork; that a fresh tree resets both the structural cost and the per-member key staleness; and that the one center-free constraint (KeyPackage availability) is bounded by the last-resort mechanism.

**Do not.** Prove the governance chain's own correctness (E12.7 is model-form, pending the real governance layer); settle the optimal N (these measure the per-boundary cost that *feeds* that tuning, they do not choose it); or address transport-level reachability of KeyPackages at a boundary (out of scope for this layer, noted in E12.6).

## Open questions to resolve during execution

- Whether mls-rs exposes reinitialization as a first-class operation that emits the resumption PSK, or whether the swap is better built as a plain fresh-group creation plus a manually derived resumption PSK. E12.1 and E12.2 will surface which is cleaner on this stack. **[confirm]**

- Whether mls-rs surfaces resolution size or blank/unmerged counts directly, or whether E12.4 must use a byte-size or ciphertext-count proxy. **[confirm]**

- Whether a group context extension is the right carrier for the governance-chain reference (E12.7), or whether the binding should live entirely in the app layer above MLS. This is a design choice, not a library limit.
