# Drystone: Governance Fold Conflict-Resolution Semantics (§7.3.1 resolutions)

`Status: merge-ready draft for Part 2 §7.3.1. Resolves the four open semantic questions (OQ-1 through OQ-4) surfaced by the Stage 1 convergence harness, which could not build a faithful fold because §7.3.1 did not answer them. With these resolutions a faithful fold can be written and the order-independence property becomes constructive rather than asserted, conditional only on gap-completeness.`

`Scope: specifies how the governance fold resolves conflicts, so that the fold is a deterministic, order-independent function of the set of authorized governance facts. Answers OQ-1 (causal order versus hash tiebreak), OQ-2 (role cascade on member removal), OQ-3 (operations on absent targets), OQ-4 (repeated threshold changes). Feeds the load-bearing convergence property of scaling-and-ordering.md.`

`Companion to: ../../drystone-spec/part-2-certifiable-design.md (§7.3.1, the merge target), governance-finality.md (whose A1 amends R1 below, folding assembled quorums rather than individual votes; the two should merge together into §7.3.1), fact-and-chain-representation.md (which specifies the fact the fold consumes and the observed-head reference R1 walks; concurrency here is derived from the local DAG index defined there), scaling-and-ordering.md (whose governance-ordering claim rests on the order-independence established here), asset-keying.md, history-durability.md.`

Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Tags: `Verified-RFC` (checked against RFC 9420/9750), `Design` (a Drystone resolution proposed here, take as the specified answer), `Load-bearing, unearned` (a property the result requires that is not yet established), `[confirm]` (a smaller pending item).

`Terms: slot (a single governance variable, for example a member's role or a threshold); fact (one signed governance operation); fold (the deterministic function from a set of facts to resolved slot values); causal order (the partial order induced by facts' observed-head references); concurrent (two facts, neither causally after the other); tiebreak (the deterministic order applied only among concurrents); projection (a cross-slot effect computed over final resolved slots); gap (a referenced-but-absent predecessor). OQ-1 through OQ-4 are the four open questions this doc resolves; R1 through R4 are its resolutions.`

## The unifying model

The four questions have one answer. The governance fold is a deterministic function of the **set** of authorized governance facts, defined by three properties that together make it order-independent given a complete causal history.

- **Per-slot causal last-writer-wins.** Each governance quantity is resolved as an independent register over the causal (happens-before) order of the facts that touch it, with a deterministic tiebreak used only among genuinely concurrent facts (R1).

- **Cross-slot effects are projections on the final sets.** Effects that span slots, such as a member removal revoking that member's roles, are computed once as pure functions of the final resolved slots, never applied incrementally during folding (R2).

- **No fold-time semantic-validity rejection.** The fold checks well-formedness and authorization, but never rejects a fact because its semantic target appears absent, because absence is a function of causal completeness a node may lack under a gap (R3).

These make the fold's order-independence a construction, not an assumption: the resolved value of every slot is a pure function of the fact set and its causal DAG, and the projections are pure functions of the resolved slots, so no fold order can change the result. The one input that can vary is the set itself, if a gap hides a fact, which is precisely why gap-detection is the fold's single external dependency and remains the load-bearing open item. `Design` for the model; `Load-bearing, unearned` for the gap-completeness it is conditional on.

**Definitions.**

- A **slot** is an independently resolved governance quantity: the membership status of a member (`member:m`), the assignment of a role to a member (`role:m:r`), the threshold for a role (`threshold:r`). Each fact touches one or more slots.

- The **causal order** is the partial order induced by facts' predecessor references. Each fact references the frontier of facts its author had observed when authoring it; the transitive closure is happens-before. Fact B is *causally after* A iff A is in B's causal history.

- The **causally-maximal facts for a slot** are those touching the slot that are not causally dominated by another fact touching the same slot.

- A fact whose referenced predecessor is not present in the set signals a **gap**: causality cannot be fully determined, and the node MUST treat this as detected incompleteness (R3, and gap-detection generally), not fold onward as if complete.

## R1 (resolves OQ-1): causal order is authoritative; the tiebreak orders only concurrents

For each slot, the resolved value is determined by the causally-maximal facts touching that slot.

- If exactly one fact is causally-maximal for the slot, its operation determines the slot value.

- If more than one fact is causally-maximal (they are mutually concurrent), a deterministic total order over facts breaks the tie, and the greatest under that order determines the value. The tiebreak key is the FactId (a total order over content hashes), or another canonical per-fact key fixed by the spec.

Normative constraints:

- The tiebreak MUST be consulted **only** among mutually concurrent facts, and MUST NOT override causal precedence. A causally-later fact always supersedes a causally-earlier one for the same slot, regardless of their FactId order.

Rationale: content hashes are uncorrelated with causal order, so a hash-only rule (the Stage 1 stub's placeholder) can let a causally-earlier decision supersede a causally-later one, undoing a newer governance action because its hash happened to sort lower. Causal precedence is authoritative; the hash breaks only genuine ties, where by definition no causal answer exists and any deterministic total order is acceptable because it is identical on every node. `Design.`

Amendment (see governance-finality.md A1): the "facts touching a slot" that R1 resolves are, for authority-slot transitions, *assembled k-of-n quorums* of concordant votes, not individual votes. A single vote is one signature toward a change and does not move the slot; the causally-maximal resolution here applies to completed quorums. A node assembles quorums against its own view, and a disagreement that survives reconciliation escalates to a fork (§7.6) rather than being resolved by the fold. R1 and A1 should merge together into §7.3.1. `Design.`

Amendment (see governance-finality.md A12, A13): R1 is the *inner loop* of a layered fold. The fold resolves facts in a fixed precedence over operation *types* (thresholds, then membership removals, then role removals, then role grants, then membership additions: subtractions before additions), and R1 resolves each tier against the settled result of the tiers above it. Type-precedence brackets R1; it does not alter R1's causal core. Additionally, the tiebreak *key* used among genuine concurrents is configurable within a safe range: the canonical hash is the party-neutral default; a group may opt into join-*order* seniority for member operations, defined strictly as a settled causal or logical position (order, not wall-clock time) with the hash still underneath as the total fallback; and a group may opt into governed instance weighting (weights themselves under k-of-n, never self-asserted). The principle: a resolution mechanism may default only if it is party-neutral; any that privileges a party must be opt-in and governed. `Design.`

## R2 (resolves OQ-2): cross-slot effects are projections on final sets

Slots are resolved independently by R1. Cross-slot effects are then computed as projections over the final resolved slot values, once, and MUST NOT be applied by mutating state during the fold.

Two specific rules for role-versus-membership:

- **Removal revokes roles.** A `RemoveMember(m)` fact acts as a revoke operation on every `role:m:*` slot at its own causal position. So a role granted causally before a removal is revoked by it, and re-adding m later does not silently restore roles a prior removal revoked; the role must be granted again by a fact causally after the removal.

- **Effective-roles projection.** A role assignment is *effective* iff its `role:m:r` slot resolves to granted **and** m's `member:m` slot resolves to member. A removed member therefore holds no effective roles even if a concurrent grant happened to win its slot tiebreak.

Amendment (see governance-finality.md A12): in the layered fold, a removal's cascade (the ceiling stamp and the role revocation above) rides at the removal's tier (membership removals, tier 2), not deferred to a later tier. So by the time role grants (tier 4) resolve, a removed member already has no roles to reason about, and the effective-roles projection is consistent at every tier boundary, not only at the end. `Design.`

Normative constraint:

- Both the role-revocation effect of removal and the effective-roles projection MUST be computed as pure functions of the final resolved slots, so the result is independent of fold order.

Rationale: an incremental cascade (delete a member's roles when folding the removal, re-create them when folding a later grant) is order-dependent, which is exactly the latent divergence this experiment exists to prevent. A projection on the final sets is order-independent by construction. The combination of the two rules gives intuitive semantics: removal revokes roles; a concurrent grant to a removed member has no effect; re-adding a member does not restore old roles; a fresh grant after re-add works. `Design.`

## R3 (resolves OQ-3): no fold-time semantic-validity rejection; idempotent no-ops

The fold MUST check well-formedness (signature, structure) and authorization (the fact met its k-of-n or capability requirement), but MUST NOT reject a fact at fold time on the grounds that its semantic target appears absent.

- An operation on an absent target is an idempotent no-op with respect to the resolved state. `RemoveMember(m)` with no observed `AddMember(m)` leaves m not-a-member; `RevokeRole(m, r)` with no observed grant leaves the role ungranted.

- A fact whose referenced predecessor is absent is a detected gap (see Definitions), not a rejection. The node marks incompleteness and does not fold onward as if complete.

Rationale: semantic validity ("the target exists") is a function of the complete causal history, which under a gap a node may not hold. Rejecting on incomplete history would make the folded state depend on which facts happened to have arrived, reintroducing both order- and completeness-dependence. Resolving to the causally-correct state, and treating an absent-target operation as leaving the target absent, is gap-safe and order-independent. In effect, membership "never mentioned" and "explicitly removed" are indistinguishable, both are not-a-member, which is intended. `Design.`

## R4 (resolves OQ-4): repeated threshold changes resolve by R1

Two `SetThreshold` facts on the same role are two operations on the `threshold:r` slot, resolved by R1: the causally-later supersedes the earlier, and genuinely concurrent ones are ordered by the tiebreak. No additional validity rule is imposed, and there is no requirement that an author's threshold changes be causally linked.

Rationale: an author writing from a single device produces a causally linked chain (the per-device subspace is hash-linked), so that author's sequential threshold changes are already causally ordered and the later wins. Concurrency across one persona's several devices is legitimate under the device-subspace model, and is resolved by the tiebreak like any concurrency. OQ-4 is thus a special case of R1 and needs no separate rule. `Design.`

## Order-independence, now constructive

With R1 through R4, the fold's order-independence is a consequence, not an assumption:

- Each slot's resolved value is a pure function of the causally-maximal facts and the tiebreak, both determined by the set and its causal DAG, not by fold order. So per-slot resolution is order-independent. `Design.`

- The cross-slot projections (R2) are pure functions of the final resolved slots, so they are order-independent. `Design.`

- No-fold-time-rejection (R3) means the set of facts that determines the result is not itself dependent on arrival order or path. `Design.`

Therefore the fold is order-independent, **conditional on the causal order being fully known, that is, on the fact set being causally complete.** This is the one remaining beam. An undetected gap could hide a causally-later fact, changing the causally-maximal set for a slot and thus its resolved value, so completeness is what the property rests on. `Load-bearing, unearned` for completeness.

The order-independence half of this property is no longer only constructive. A separate proof ran the reconcile computation (fork detection, deterministic survivor selection, contradiction hard-stop with both branches preserved and attributed, no auto-resolve) on three genuinely separate machines with no superpeer and no orderer, and the verdict was byte-identical across all three, invariant across every merge order tested, including a three-way fork that converged regardless of order. That demonstrates the permutation-invariance of the fold-reconcile on real hardware with real Ed25519 governance, on a different library stack (openmls 0.8.1 with automerge 0.7.4) than this suite's delivery experiments, so the order-independence this section is conditioned on is now `Verified` for the reconcile computation rather than asserted. It does *not* discharge the gap-completeness beam: the proof models the partition as op-log exchange, not a live transport partition, so completeness stays `Load-bearing, unearned` and the two halves must not be conflated. See the reference-index (Proof: cross-machine deterministic reconcile). `Verified` (reconcile computation, Proofs Part A); `Load-bearing, unearned` (gap-completeness).

This is a real strengthening of the claim in scaling-and-ordering.md. Before these resolutions, "the fold is order-independent" was asserted with unspecified internal structure, and was itself the risk. Now the commutativity follows by construction from a specified causal-LWW-plus-projection model, and the only remaining conditionality is gap-completeness, which was always present as a separate open item. The resolutions do not close the completeness question, but they move the fold from "asserted commutative" to "constructively commutative given completeness," which narrows the risk to the single completeness dependency. The scaling note's governance claim should be updated to reflect this narrowing, while still not being called earned until the completeness experiments pass.

## What this unblocks

- A faithful reference fold can now be written from R1 through R4, so the convergence harness can test the actual specified semantics rather than a placeholder whose commutativity was trivial. The extended experiment (drystone-convergence-experiment-brief-v2.md) specifies the tests that target each resolution.

- The fact model now requires explicit causal predecessor references (the dependency frontier each fact observed), because R1's causal order and R3's gap-detection both depend on them. Any fact schema that omitted causal references is insufficient for these semantics. `Design.`

## Open items

- **Gap-completeness remains the load-bearing open item.** R1 through R4 make the fold commutative given a complete set; establishing that a node can reliably *detect* an incomplete set (rather than fold it as complete) is the dataplane-checkpoint and completeness-ahead-corroboration work, and is what the extended Stage 2 experiment targets. `Load-bearing, unearned.`

- **The canonical tiebreak key.** R1 uses FactId (content hash) as the concurrent tiebreak. Whether the spec prefers the raw content hash or a canonical composite (for example author key then per-author counter then hash) is a detail to fix; any total order identical on all nodes satisfies the correctness constraint, so this is a preference, not a correctness question. `[confirm.]`

- **Whether any governance conflict should surface rather than resolve.** R1 resolves concurrent conflicts silently by tiebreak. Part 2 §7.6 fork-as-escalation may be the intended path for some conflicts (a genuine governance schism), rather than a silent tiebreak. Which conflict classes escalate to a fork versus resolve by tiebreak is a policy question left to §7.6 and not resolved here; the fold's tiebreak is the default where no escalation is specified. `[confirm.]`

## Changelog

`Working draft; transitions recorded here per the suite's doc-method.`

- **Draft, resolving OQ-1 through OQ-4.** Specifies the per-slot causal-LWW model with the tiebreak restricted to concurrents (R1), cross-slot effects as projections on final sets with the role-revocation and effective-roles rules (R2), no fold-time semantic-validity rejection with idempotent no-ops and predecessor-absence as gap-detection (R3), and threshold changes as a special case of R1 (R4). Establishes order-independence as constructive given gap-completeness.

- **Provenance.** R1 through R4 are `Design` resolutions proposed for §7.3.1; the order-independence is a constructive `Design` consequence conditional on gap-completeness, which is `Load-bearing, unearned`. The tiebreak key and the escalate-versus-resolve boundary are `[confirm]`. Nothing here is `green-real`; the extended experiment is the path to exercising it against a faithful fold.

- **Doc-model standard pass.** Added a Terms block up front for spec-readiness; no semantic change to R1 through R4.
