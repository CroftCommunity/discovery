# Drystone convergence experiment v3: testing the governance-finality mechanisms

## What this adds, and why it is separate

The v1 and v2 briefs test the governance fold's conflict-resolution semantics (R1 through R4 in ../drystone-design/fold-semantics.md): given a set of facts, does it fold order-independently and causally correctly. This v3 brief tests the layer specified on top of that in ../drystone-design/governance-finality.md: how a k-of-n decision becomes effective (quorum-folding, A1), how a removal bounds a member's authority (the ceiling, A3), how the decision is enacted (the enactment dial, A4 through A6), how a node knows its authority view (the now, A7, and the fail-closed finality gate, A8), that a ban and a voluntary fork are one primitive (A9), and how genuine disagreement escalates rather than being adjudicated (B1).

This is a distinct body of behavior from the fold itself, so it is a separate brief and three new stages (4, 5, 6) that build on the v2 crate. Do not begin it until the v2 Stage 1 (properties A through F) and v2 Stage 2 (gap-detection) are green and committed, because every stage here reuses the faithful fold and the seeded simulator from v2.

## The honesty rule still holds, and gains one clause

As in v1 and v2: the first line of every result MUST state what was tested. For v3 there are two facts to state, not one.

- Real versus reference fold, exactly as before. No production fold exists in `croftc/securitypolicy`, so unless one now exists, you are testing a faithful reference implementation.

- **The enforcing-commit model is a reference stand-in for MLS, not real MLS.** The enactment stages (5) model the "enforcing commit" as a single-winner-per-epoch serialized operation that is idempotent in effect. This models the *enactment logic's* convergence and the single-slot contention shape (RFC 9420's one-commit-per-epoch rule), and results speak to that logic. They do **not** exercise real MLS key schedule, ratchet tree, or Welcome/GroupInfo handling. Any result touching enactment MUST say, in its first line, that the epoch model is a reference model of commit serialization, not MLS.

Additionally, several escalation behaviors (Stage 6) depend on the two sub-decisions the spec leaves open in B1 (auto-fork versus flag-and-hold; soft-healable versus durable). Where a test depends on those, RESULTS.md MUST state which sub-decision the harness assumed. The settled detection line (lag heals, same-facts disagreement escalates, gated on completeness) is testable without fixing the sub-decisions; the post-escalation behavior is not.

## Contract extensions

Build on the v2 fold contract. Add the following types and rules. Keep them the faithful minimum; simplicity is the point, as in v2.

**Vote and quorum.** A `Vote` is a signed fact expressing one member's support for a specific slot transition (for example `RemoveMember(m)`), carrying its author and its `predecessors` like any fact. A slot transition is caused by an assembled **quorum**: k concordant votes for the same transition, where k is the threshold in force at the relevant causal position (read from the folded state, per R4). Quorum assembly MUST be evaluated against the assembling node's own view. A set of fewer than k concordant votes leaves the slot unchanged. The completing vote (the k-th, folded against a node's view) is the one that crosses the threshold.

**Ceiling.** A `Ceiling` records, for a removed member, the governance head as-of which its authority ends. It is stamped by the completing signer as part of the crossing fact, is a fact about the removed member (content-addressed by that state per A0, not by author), and carries the quorum's authority.

**The now.** A `Now` is a materialized current state: settled slot values (members, roles, thresholds) plus in-flight tallies (per pending transition: target, current count, threshold, enacted-yes/no). It carries a reference to the history-chain head it was derived from and MUST be verifiably derivable from that chain. It is a replaced value, not accumulating. It advances as facts fold and MUST NOT itself trigger an epoch. Attestations over a `Now` are a corroborating signature-set attached to it (A0), and MUST NOT be part of its identity.

**Enactment / epoch model (reference stand-in).** Model an `Epoch` chain where an `EnforcingCommit` (for example, removing m's leaf) closes one epoch and opens the next, at most one commit per epoch (losers rebuild on the new epoch). All `EnforcingCommit`s for the same decision are idempotent in effect: whichever wins, the resulting epoch state is identical, and a loser rebuilt on the winner's epoch is a no-op. This is a reference model of commit serialization, not MLS.

**Lineage and divergence.** A `Lineage` is a chain of facts from a shared history. The **divergence primitive** is two lineages sharing history up to a head and diverging after it. Two triggers produce it: a ban (a quorum ceases corroborating a member, stamping a ceiling) and a voluntary fork (a member continues from local state, no group artifact). Both leave each side intact.

## Stage 4: quorum-folding, ceilings, and the now

Pure fold-plane and now behavior; no epoch model needed yet. Use `proptest` for the invariance properties and hand cases for the specific scenarios, exactly as v2 Stage 1.

Property group G, quorum-folding (A1):

- G1: a sub-threshold set of concordant votes leaves the slot unchanged, on every node. **Discriminating**: a naive "count any concordant signature" implementation that flips the slot below k fails this.

- G2: the k-th concordant vote, assembled against a node's own view, flips the slot; fewer than k does not.

- G3: a signatory's own single vote does not move the slot for that signatory (a member enacting on their own signature is a deviation). **Discriminating** against a fold that treats the author's own vote as sufficient.

- G4: quorum assembly is permutation-invariant: any order of the k votes yields the same completed decision and the same crossing head (up to the tiebreak on genuinely concurrent completing votes).

Property group H, non-exclusive recognition and concurrent completion (A2):

- H1: N members who each independently fold the same k-1 prior votes and add their own each conclude "quorum met"; their results are concordant (all "removed").

- H2: the unanimous case (every member believes it is the completer) yields one canonical decision and one canonical ceiling head by idempotent union plus tiebreak, not N rival decisions. Permutation-invariant. **Discriminating** against any design that treats "who completed the quorum" as an exclusive slot.

Property group I, ceiling (A3):

- I1: on quorum crossing, a ceiling is stamped at the crossing head.

- I2: concurrent ceilings at different heads *within one lineage* union; the canonical head is selected by the R1 tiebreak; permutation-invariant.

- I3: a removed member's authority ends at its ceiling (the enforcement consequence is tested in L).

Property group K, the now (A7):

- K1: the now is verifiably derived from the chain (it rolls up from the fact set; a tampered now fails verification).

- K2: the now is replaced, not accumulated: advancing it across many fold-plane changes does not grow an unbounded structure.

- K3: fold-plane changes (role, threshold, tally) advance the now and MUST NOT trigger an epoch; only membership key-changes produce an `EnforcingCommit` (Stage 5). **Discriminating** against a design that commits on every governance change.

- K4: nows are comparable by bound head; a node reconciling with a peer moves to the causally-later now; nows bound to concurrent heads re-derive to the same later now (convergence).

- K5: **sign-the-state-not-authorship (the "nine-fives" test, highest-value in Stage 4).** N members attesting the same now produce one now with N corroborating signatures, not N rival nows, and the head does not fork. **Discriminating**: an implementation that makes a signer's now a distinct object by author fails this and forks the head. This is the property that validates A0 for the now.

- K6: in-flight tally correctness: the now reflects "2 of 3" for a pending removal, and a "3 of 3 with no enacting commit" state is representable and detectable (this is the fallback signal read in Stage 5).

Stage 4 acceptance: G, H, I, K pass over a large proptest count; the discriminating cases (G1, G3, H2, K3, K5) are each confirmed to fail a stated naive variant, which is then removed; hand cases for H2 and K5 are explicit unit tests; regression file committed. RESULTS states real-versus-reference and faithful-to-A1-A3-and-A7.

## Stage 5: enactment, the two-phase interval, and the finality gate

Needs the reference epoch model. Build on the seeded simulator from v2 Stage 2 for the delivery-timing parts.

Property group J, decision versus enactment and the dial (A4 through A6):

- J1: N concurrent `EnforcingCommit`s for the same decision converge to one epoch state (idempotent in effect); losers rebuilt on the winner's epoch are no-ops.

- J2: single-enactor path produces exactly one epoch (no redundant rekeying).

- J3: signer-fallback: the completer drops out before enacting; after the interval a fallback signer enacts; the group converges, with the number of redundant commits bounded by the signer count.

- J4: any-member (aggressive) fallback converges; redundant-commit count may be larger but the epoch state is identical.

- J5: no-enactor-present: the removal remains a valid decided-but-unenacted state (the ceiling governs authority, no commit yet), and enacts correctly when an eligible enactor returns. **Discriminating** against a design where a decision is lost if not immediately enacted.

- J6: two-phase interval: after the decision folds but before the enforcing commit, the removed member's actions are not honored by a node that holds the ceiling (authority ends on the fact), even though the member is still modeled as key-holding until the commit (cryptographic access ends on enactment). Authority-void is immediate; access-revocation is on enactment.

Property group L, the finality gate (A8):

- L1: an action at or beyond an actor's ceiling is void; enforcement checks the actor's action head against the ceiling.

- L2: a current member behind on a ceiling marker heals on propagation: before it holds the ceiling it may honor a now-void action, after receiving the ceiling it voids it, and nodes converge. This is the benign liveness case.

- L3: **fail-closed on finality (highest-value in Stage 5).** When a node cannot establish its now is current to the edge (it holds no quorum-attested now, or observes an attestation referencing a later head it lacks), it MUST NOT finalize an irreversible authority-enforcing action; it stalls. **Discriminating**: a fail-open design that finalizes on stale state, or that annotates-and-proceeds, fails this by enforcing possibly-withdrawn authority.

- L4: reads and content-plane liveness are never gated: a partitioned node still folds, serves best-known state (with a freshness qualification), and stays live on the content plane. **Discriminating** against a design where fail-closed freezes the node entirely.

- L5: completeness-ahead is corroborated, not proven: a node establishes currency only via a quorum of attestations over the now; in their absence it stalls (it does not manufacture currency). Test that the stall happens, not that currency is proven.

Stage 5 acceptance: J and L pass; discriminating cases (J5, L3, L4) confirmed against stated naive variants; the first line of RESULTS states the epoch model is a reference stand-in for MLS. Do not start Stage 5 until Stage 4 is green and committed.

## Stage 6: ban/fork equivalence and escalation

Needs the lineage/divergence primitive; build on the v2 Stage 2 simulator for the same-facts-versus-lag distinction.

Property group M, ban/fork equivalence (A9):

- M1: a ban and a voluntary fork of the same member produce the same lineage-divergence structure (divergence at a head, both sides intact).

- M2: distinct artifacts: the ban lineage carries a quorum-stamped ceiling; the voluntary fork carries no group-side artifact. A third-party model reading the two can tell them apart by the presence or absence of the ceiling.

- M3: a banned member's lineage is intact, not deleted: the member retains all state up to the ceiling and can continue in its own lineage. **Discriminating** against a design that models a ban as deletion or state loss.

Property group N, escalation detection (B1):

- N1: a delivery-lag disagreement heals by convergence and does NOT escalate: a node merely behind (it has not yet seen a threshold-raise or a re-add) converges once the fact arrives.

- N2: a same-facts disagreement escalates: two nodes holding the same head that still compute different quorum outcomes trigger a fork, not a silent forced resolution. **Discriminating** against a fold that silently picks a winner (which would reintroduce a center).

- N3: escalation is gated on the shared-completeness signal: a fork fires only after equal-head is established; before that, the disagreement is presumed lag and heals. This is the do-not-over-fork-on-lag guarantee, and it is the same signal as L3/L5. **Discriminating** against a design that forks on transient delivery differences.

- N4 (sub-decision-dependent, state the assumption): if flag-and-hold is assumed, the disputed slot enters a held state with enforcement suspended, without fragmenting the group; if auto-fork is assumed, the lineage splits immediately. If soft-healable is assumed, a re-vote against the current baseline reconciles the diverged lineages; if durable, it does not. Test whichever the harness assumes, and state it.

Stage 6 acceptance: M and N1 through N3 pass; N2 and N3 confirmed against stated naive variants; N4 tested only under a stated assumed sub-decision. Do not start Stage 6 until Stage 5 is green and committed.

## Honest scope (put in README, extends v2)

- Passing Stage 4 establishes that the specified quorum-folding, ceiling, and now mechanisms are order-independent and concurrency-safe as implemented in the reference fold, including that concurrent and unanimous completion do not fork the head or the decision (K5, H2). It does not establish gap-completeness (still per v2 and the fold-semantics doc) and does not touch enactment.

- Passing Stage 5 establishes that the enactment logic converges under the reference epoch model and that the finality gate fails closed on finality while never gating reads or content liveness. It does **not** establish anything about real MLS, because the epoch model is a reference stand-in for commit serialization.

- Passing Stage 6 establishes that a ban and a fork are structurally one primitive with distinct artifacts, and that the escalation line distinguishes lag (heals) from genuine disagreement (escalates) gated on completeness. Post-escalation behavior is tested only under a stated assumed B1 sub-decision, and is not settled until that sub-decision is fixed.

- Across all stages: property-based testing samples, it does not exhaust; the epoch model is not MLS; no production fold or production governance-finality implementation exists in this repository. Passing is strong evidence for the **specified** mechanisms, not a proof and not a statement about production.

## Deliverables

Extend the existing `drystone-convergence/` crate. Add the Vote/quorum, Ceiling, Now, reference epoch, and lineage types; the G through N properties with their hand cases and stated naive variants for the discriminating ones; the Stage 5 and Stage 6 additions to the seeded simulator; updated README and RESULTS. RESULTS.md must state, in its first line, real-versus-reference and (for enactment results) that the epoch model is a reference stand-in for MLS, then report case counts, what passed, the discriminating-variant confirmations, and any assumed B1 sub-decision.

## The one rule above all (unchanged, extended)

State what was tested, in the first line: real-versus-reference fold, and for enactment, reference-epoch-versus-MLS. Do not resolve a spec ambiguity silently; if the escalation sub-decisions or any unspecified semantics force a choice, state the choice in RESULTS as an assumption, because such a choice may itself be where a latent divergence hides.
