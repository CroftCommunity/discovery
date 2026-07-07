# Drystone: Quorum Enactment, Membership Ceilings, and Governance Finality

`Status: draft in two tiers. Part A is merge-ready normative specification for Part 2 §7.3.1 and §7.3.3. Part B is a distinct register of the challenges and open questions, kept separate on purpose so that at merge time it is clear what is settled and foldable versus what still owes a decision or an experiment. Nothing in Part A is earned in the "tested in production" sense; the epistemic status is held explicitly in B7.`

`Scope: how a governance decision that requires a k-of-n threshold becomes effective and gets enforced without a central referee, how a removed member's authority is bounded so that honest nodes cannot diverge indefinitely, how a node knows whether its authority view is current enough to enforce on, and how genuine disagreement is handled (operation-type precedence, the three registers of mute/governance/fork, fork placement and announcement, hold-suspends-enactment, permanent-both, and posture dials) without any central adjudication. Extends the conflict-resolution semantics of p10-drystone-fold-semantics.md (R1 through R4) with the quorum, ceiling, enactment, now, finality, and conflict-handling mechanisms walked out after the Stage 1 and Stage 2 convergence runs.`

`Companion to: p10-drystone-fact-and-chain-representation.md (the byte-level substrate under this doc: how facts are encoded and hashed, how the now is derived, and the three signing roles A0/A1/A7 rest on), p10-drystone-fold-semantics.md (R1 is amended by A1 here; the two should merge together into §7.3.1), p10-drystone-scaling-and-ordering.md (whose governance-ordering claim this bears on directly), p10-drystone-asset-keying.md (§H two-phase removal and the retained-copy floor, which A6 depends on), p10-drystone-history-durability.md, p10-drystone-part2-mechanics.md (§7.3.1 and §7.3.3, the merge targets; §7.4 freshness cursor, which A8 reuses; §7.6 fork-as-escalation).`

`Terms: quorum (k concordant votes assembling one decision); vote (a signed fact supporting a slot transition); ceiling (the head as-of which a removed member's authority ends, A3); the now (the materialized current authority state plus in-flight tallies, A7); enactment (applying a decided change to the MLS epoch, A4 through A6); best-known state and final state (A10); fork (lineage divergence at a head, A9); mute (a purely local presentational act, A11); posture dial (a temperament-keyed default over conflict handling, A17). Persona, node, authority, and helper are as defined in p10-drystone-authority-and-complement.md.`

Normative keywords are **MUST / SHOULD / MAY** (BCP 14). Tags: `Verified-RFC` (checked against RFC 9420/9750), `Design` (a Drystone resolution specified here, take as the specified answer), `Load-bearing, unearned` (a property a claim requires that is not yet established), `[confirm]` (a smaller pending item). Part A is the settled tier; Part B is the open tier.

---

# Part A: Specification (merge-ready, normative)

## A0. The governing principle: sign the state, not the authorship

Every durable governance object introduced below (a ceiling, the now, a freshness attestation) is a statement about a **state value**, corroborated by signatures that ride alongside it, and never a claim about **who produced it**.

- A governance object's identity MUST be a function of the state it asserts (it is content-addressed by that state), and MUST NOT include the identity of the party asserting it.

- Signatures over a governance object MUST be treated as a corroborating grow-set attached to that object, unioned across signers, and MUST NOT form part of the object's identity.

Rationale and consequence: this is the single property that keeps concurrency safe across the whole mechanism. If two parties independently assert the same state, the "sign the state" rule yields one object with two vouchers, which union idempotently; the "sign the authorship" alternative would yield two rival objects ("I did this" versus "no, I did this"), which fork. The test to apply to any new governance object is: if two honest parties do this independently, do you get one thing or two. It MUST be one. This principle is why the ceiling (A3), the now (A7), and the freshness attestation (A8) are all concurrency-robust rather than points of contention. `Design.`

## A1. The fold folds quorums, not votes (amends R1)

A change to an authority slot (membership, a role assignment, a threshold) is caused by an assembled threshold of agreement, not by any single signed fact.

- An authority-slot transition MUST require an assembled k-of-n quorum of concordant governance facts for that transition, where the threshold k-of-n is the one in force at the relevant causal position (itself read from the folded state, per R4 threshold ordering).

- A set of fewer than k concordant facts MUST leave the slot unchanged on every node, including nodes authored by signatories. A signatory's own vote does not move the slot for that signatory.

- A node MUST NOT treat an authority change as effective, nor enact it (A4), on fewer than k signatures. A node that enacts on its own sub-threshold signature is deviating unilaterally from the group's decision and is a de facto fork origin.

- A node MUST assemble a quorum against its own current view: concordance and threshold satisfaction are judged against the state that node holds. The fold MUST NOT attempt to reconcile votes cast against divergent baselines into a single forced outcome. Where two nodes hold the same facts and still disagree on whether a quorum was met (a baseline disagreement that survives reconciliation, not mere delivery lag), that disagreement is a fork condition escalated to §7.6, not a fold error and not something the fold adjudicates (B1). Picking a winner would be installing a center.

Rationale: this is the correction the Stage 1 analysis surfaced. Resolving a slot by "the causally-maximal single fact" (R1 as first stated) is right for *which* concordant decision wins, but a decision is not a single fact; it is the k-th corroborating fact crossing the threshold. Folding a lone vote into a change would make any single member a center, which the peer-symmetry principle forbids (p10-drystone-authority-and-complement.md). The fold resolves *assembled quorums* by R1, not individual votes. `Design.`

## A2. Recognizing a met threshold is not an exclusive action

Threshold satisfaction is an objective property of the assembled facts, not a decision any one party makes.

- Any member observing the assembled quorum MAY recognize it and emit the corresponding effect (the ceiling of A3, or the enactment of A4). Recognition MUST NOT be reserved to a single designated party for correctness purposes.

- Multiple concordant recognitions MUST be treated as corroboration (unioned per A0), not as competing facts.

Rationale: because any node that can see the k votes correctly concludes the same true thing, concurrent recognition, up to and including every member independently concluding "my signature completed the quorum," produces many assertions of one identical fact, which collapse. This is the fact-plane concurrency story: recognition is free under concurrency. Its enactment on the epoch plane is where cost reappears (A4). This over-recognition safety holds within a node's own consistent view of the state the votes were assembled against; where nodes hold divergent baselines and the disagreement survives reconciliation, that is not a break in the safety but the escalation case (A1, B1). `Design.`

## A3. The membership ceiling

When a removal crosses its threshold, the crossing fact records the removed member's authority endpoint, so that the member's necessarily-frozen view is correct by construction rather than a divergence to be repaired.

- The fact that crosses the removal threshold MUST stamp a **membership ceiling**: the governance head as-of which the removed member's read and enforcement authority ends. The ceiling MUST be part of the quorum-crossing fact, carrying that fact's k-of-n authority, and MUST be stamped by the completing signer (the party whose signature assembles the quorum, and who is therefore the first party for whom the removal is true).

- The ceiling MUST be a fact about the removed member (a state value under A0), held in the authority-plane state that current members reconcile. It MUST NOT be forgeable by the removed member, who is by definition not the completing signer and cannot assemble a quorum excluding the required others.

- A removed member's authority MUST be treated as ending at its ceiling. Its view beyond the ceiling is correct-by-construction and requires no reconciliation: a member locked out of a scope is expected to have a frozen view, and the protocol owes it no consistency past the ceiling.

- Where concurrent completing facts stamp ceilings at different heads *within one lineage* (all nodes agree the removal occurred, differing only on the exact head), the ceilings MUST union as concordant assertions, and the canonical ceiling head MUST be selected by the R1 concurrent tiebreak. Where nodes hold the same facts but genuinely disagree on whether the removal occurred at all (one lineage stamps a ceiling, another stamps none), the ceilings do not union: the disagreement is real and diverges into separate lineages via the ban/fork unification (A9) and §7.6. So concurrent ceilings union within a lineage and diverge across a fork; forced concordance across a genuine disagreement is never attempted.

Rationale: the ceiling is the completeness anchor for membership. It converts the dangerous case (a still-connected node acting on a stale membership view) into a checkable one (any current member can check an actor's action head against that actor's recorded ceiling, A8). It does not by itself close general completeness (B3); it closes the membership-entitlement question specifically, which is the safety-critical one because the dangerous direction is enforcing a revoked authority. `Design.`

## A4. Decision versus enactment: two concurrency regimes

A removal is two coupled events on two planes that behave oppositely under concurrency, and they MUST be treated separately.

- The **decision** (the quorum-completing fact plus its ceiling) lives on the fact plane. It folds per A1, unions per A0, and is concurrency-safe and cheap. The decision MUST fold independent of, and prior to being conditioned on, the enforcing commit.

- The **enforcement** (the MLS commit removing the member's leaf and rekeying) lives on the epoch chain. Only one commit closes an epoch (`Verified-RFC`, RFC 9420/9750), so concurrent enforcing commits collide and serialize. Multiple concurrent enforcing commits for the same decision MUST converge to the same epoch state, because they are idempotent in effect (each performs the same removal and rekey; a loser rebuilt on the winner's epoch finds nothing left to do).

- To avoid redundant rekeying and epoch churn, a single enactor SHOULD perform the enforcing commit. Correctness MUST NOT depend on enactor uniqueness; only efficiency does.

Rationale: this is where the fact-plane freedom of A2 stops. Recognition is non-exclusive and free; enforcement is exclusive by MLS construction and expensive under contention (each collision triggers a re-key and an inconsistency window, p10-drystone-scaling-and-ordering.md). The epoch/governance decoupling (p10-drystone-asset-keying.md §H) is what lets the decision be agreed cheaply on the fold and the key change be enacted once off the back of it. `Design`, with the single-commit-per-epoch fact `Verified-RFC`.

## A5. The enactment dial

Who fires the enforcing commit is a deployment dial that trades epoch churn against enactment latency, and never affects correctness.

- The completing signer SHOULD be the enactor (the tight setting: fewest commits).

- If no enforcing commit is observed within a configured interval, a fallback enactor MAY act. The fallback set is the dial: the k signers only (small, bounds redundant commits), or any member (fastest under partition, largest potential herd). The dial SHOULD escalate the fallback set over time, so broad enactment is reached only when narrower enactment demonstrably did not occur. Defaults and intervals are Open (B4).

- Because enforcing commits are idempotent in effect (A4), a fallback collision is safe and merely wasteful, never divergent. A node MAY therefore fall back without global certainty that no commit exists.

- If no eligible enactor is present, the removal MUST remain a valid **decided-but-unenacted** state: the ceiling governs authority in the interim (A6), and enactment occurs when an eligible enactor returns. A removal that cannot yet be cryptographically enforced because the relevant members are offline is honest and safe, not a failure.

Rationale: a removal is decided the instant the quorum assembles; enactment is a mechanical follow-on. Because the enactor holds no authority (it executes a decision the quorum already made), designating one, or escalating a fallback, does not create a center and composes with peer-symmetry. The "three-of-three tally with no enforcing commit" state, read from the now (A7), is the signal that a fallback should fire, because it can only mean the completer crossed the threshold and then dropped out before enacting. `Design.`

## A6. The two-phase interval

Between decision and enactment there is a bounded window whose semantics MUST be stated, because authority and cryptographic access diverge within it.

- The ceiling MUST govern authority from the moment the decision folds. From that point the removed member's actions MUST NOT be honored by any node that has the ceiling.

- The enforcing commit governs cryptographic access from enactment. Until it lands, the removed member may retain the ability to decrypt current content.

- The interval is therefore a window in which a governance-revoked member may still decrypt but MUST NOT have any action honored. This is safe under the retained-copy floor (p10-drystone-asset-keying.md): revocation protects the future, not the past, and a bounded window in which a just-revoked member can still read content it would have seen anyway is within the existing threat model.

Rationale: decide first, enforce second, and enforce authority from the decision. This is the correct and safe ordering, and it is the two-phase removal of the asset-keying doc made explicit for the governance plane. `Design.`

## A7. The now: a materialized governance head

To make reads and fallback-enactment decisions cheap, the governance plane maintains a current-state head distinct from, but bound to, the append-only history chain, so that neither routine operation nor fallback logic requires replaying history.

- The governance plane MUST maintain a **now**: a materialized current state carrying settled slot values (membership, roles, thresholds) and in-flight quorum tallies (per pending change: target, current count, threshold in force, and whether enactment has occurred).

- The now MUST be bound by reference to the history-chain head from which it is derived, and MUST be verifiably derivable from the chain (a node holding the chain-tail can confirm the now rolls up from it, and a node holding the now can confirm a chain-tail is consistent with it). The now and the relevant chain travel together in the dataplane exchanges that already carry a representative-of-latest. The concrete representation of this binding, the now as a derived-and-replaceable snapshot bound by hash to the history head, with genesis-derivability as the integrity floor and checkpoints as acceleration, is specified in p10-drystone-fact-and-chain-representation.md (§1, §6).

- The now MUST be a replaced current value, not an accumulating structure. The history chain remains the append-only audit record; the now is a moving pointer over it, so it carries no unbounded growth.

- The now MUST advance as governance facts fold and MUST NOT itself trigger an epoch change. Only key-changing operations commit (A4); role, threshold, and tally changes advance the now at zero epoch cost.

- Routine reads and fallback-enactment decisions MUST be answerable from the now without replaying the history chain.

- Nows MUST be comparable by their bound history head. A node reconciling with a peer MUST move toward the causally-later now; nows bound to concurrent heads MUST reconcile by re-derivation from the underlying facts, landing both at the same later now.

- The now MUST be signed as an attestation over its state value per A0. Signatures corroborate and union; they MUST NOT make one signer's now a distinct object from another's. This is what prevents a forked head ("everyone thinks they signed the next one" yields one corroborated now, not many rival nows).

Rationale: this is the authority-plane analogue of the §7.3.3 snapshot-over-history and the §7.4 cursor. The chain is for audit and reconciliation; the now is for operating. The in-flight tally is the specific field the enactment dial (A5) reads. `Design.`

## A8. Finality and enforcement: fail closed on finality, never on reads

A node must be able to keep working while behind, and must refuse to take irreversible authority actions while it cannot establish it is current. These are separated precisely.

- A node MUST distinguish **reading best-known state** (folding what it holds and serving it, possibly with a freshness qualification) from **treating state as final for irreversible enforcement** (both defined precisely in A10). The two are gated differently.

- Reading best-known state MUST NOT be gated. A partitioned node MUST be able to fold what it holds, serve its best-known governance view, and remain fully live on the content plane.

- A node MUST NOT finalize an irreversible authority-enforcing action (honoring or denying access, admitting or removing a member, any action not cheaply reversible) while it cannot establish that its now is current to the group's edge. This is fail-closed on finality: under such uncertainty the enforcement MUST stall, not proceed.

- Enforcement MUST check an actor's action head against that actor's recorded ceiling (A3). An action at or beyond the actor's ceiling MUST be treated as void, regardless of whether the enforcing node personally witnessed the removal as a live fact, because the ceiling is a durable group-held marker.

- Establishing "current to the edge" MUST be done via a quorum of attestations over the now (A0, A7), modeled on the §7.4 freshness cursor: a node is current when it holds a quorum-attested now and observes no attestation referencing a later head it lacks. This is completeness-ahead corroboration, and it is corroborated, not proven (B3): a sufficiently isolated node cannot self-certify that nothing newer exists, and MUST fail closed on finality in that condition.

- The same shared-completeness signal gates the fork trigger of A1, so a group does not over-fork on ordinary delivery lag. A disagreement escalates to §7.6 only once nodes can establish they hold the same facts (the same head) and still disagree; before that, a disagreement is presumed to be lag and MUST heal by convergence, not escalate. So "do not finalize on stale state" and "do not fork on lag" are the same guarantee, both gated on the completeness signal.

Rationale: the danger the whole governance-scaling claim must exclude is honest nodes enforcing divergent authority, and the safety-critical direction is enforcing a stale grant a revocation has not reached (granting access that should have been pulled). Fail-closed on finality turns that window into a stall (delay) rather than a breach (enforcing withdrawn authority), which is the correct bias for a governance layer. Fail-closed on finality but not on reads is what keeps a partitioned node useful (read-only-ish on governance, fully live on content) instead of frozen. `Design`, resting on the read/enforce separation that §7.3.3 must draw (B2).

## A9. Ban and voluntary fork are one primitive: equal in outcome, distinct in artifacts

A removal by the group (a ban) and a member leaving of their own accord (a voluntary fork) are the same underlying primitive, seen from two directions. The principle side of this (the right to fork is inherent and unconfigurable) is stated in p10-drystone-authority-and-complement.md §5; this section specifies the mechanism.

- The primitive is **lineage divergence at a head**: two lineages that share history up to a point and diverge after it, with each side keeping its own state and neither owed reconciliation by the other.

- There are two triggers. A **ban** is the group ceasing to corroborate a member (the quorum stops vouching for them going forward). A **voluntary fork** is a member ceasing to require the group's corroboration (they continue from their own local state). The outcome is identical in both: divergence at a head, both sides intact in their own lineage.

- The two triggers are **equal in outcome but distinct in artifacts**. A ban deposits a quorum-stamped ceiling (A3): a fact carrying k-of-n group authority that says "the group stopped corroborating this member as of head H." A voluntary fork deposits no group-side artifact, because the group performed no act; there may be a self-authored divergence marker in the forker's own lineage, but there is no group-authority stamp, because no quorum acted.

- The distinct artifacts are an **evidentiary** necessity, not a mechanical one. They exist so a third party deciding which lineage to corroborate (the social-utility decision) can read the provenance: a quorum-stamped ceiling says "the group, by its rules, decided this," and its absence says "this party decided this on their own." The system furnishes legible provenance and stays silent on the verdict, consistent with the posture of the now, the freshness attestation, and the fork itself: a fact about state that corroborates, never an authority claim (A0).

- A ban is therefore **not a deletion**. It forks the member off whole, withdrawing the group's corroboration going forward, and the member continues in their own lineage holding everything they had up to the ceiling. The most a concentrated authority can do to a member is force a fork, and a forced fork leaves the member intact. What the member loses is precisely and only the group's corroboration going forward, which is the one thing the group had the authority to withdraw.

Rationale: this unification means the design carries one primitive rather than two, while preserving the provenance difference that the social layer needs to read a departure. It also makes the humane and structurally-honest reading explicit: even the group's harshest power over a member is bounded by the same floor as everything else (it can withhold corroboration, it cannot reach into what the member holds), which is the mechanism-side confirmation of the §5 principle that the right to fork cannot be configured away. `Design.`

## A10. The read/enforce line: best-known state versus final state (the §7.3.3 addition)

A8's fail-closed-on-finality relies on a distinction the spec must draw explicitly, and §7.3.3 as it stands does not draw it: that section addresses completeness *behind* a checkpoint (verifiable truncation, so old data can be pruned and what remains trusted), not completeness *ahead* of one (whether an unseen newer fact is missing). This section adds the distinction and is the definitional foundation A8 uses. It closes B2 and merges into §7.3.3, extending it from backward-completeness to include forward-completeness for enforcement.

Two states, defined:

- **Best-known state** is the authority state a node computes by folding every governance fact it currently holds. It is always computable and always available.

- **Final state** is best-known state that the node has additionally established is current to the group's leading edge: it can rule out an unseen, causally-later fact that would change the relevant slot. The distinction is forward-completeness: best-known says "given what I hold," final adds "and I hold everything up to the edge that bears on this slot."

The gating rule:

- Reads and content-plane operations MUST use best-known state and MUST NOT be gated. A node MUST always be able to fold, serve its best-known view (with a freshness qualification when that view is not final), and remain live on the content plane, even under partition.

- Irreversible authority-enforcing actions MUST use final state and MUST be gated on it. When a node cannot establish final state for the relevant slot, that action MUST stall (fail closed), and only that action; the stall MUST NOT extend to reads or content-plane liveness.

This is the read/enforce line: reads run on best-known, enforcement runs on final, and fail-closed applies to the enforce side only. That is what keeps a partitioned node useful (it reads and operates on best-known, degrading only to a stall on the specific enforcement it cannot make final) rather than frozen (stalling on everything). Applying fail-closed to reads as well would produce a node that halts on any partition, which is a worse failure than the divergence it would prevent.

How a node establishes final state:

- For a slot generally: the node holds a quorum-attested now (A7) whose bound head covers the slot's relevant facts, and it observes no attestation referencing a later head it lacks (the §7.4 freshness cursor is the model).

- For a removed actor's entitlement specifically: the ceiling (A3) is the durable marker. An action at or beyond an actor's recorded ceiling is void regardless (A8, and the enforcement check there), because the ceiling does not go stale the way a general slot view can.

The load-bearing caveat, flagged:

- A sufficiently isolated node CANNOT establish final state on its own, because it cannot distinguish "no later fact exists" from "I cannot currently reach anyone who would attest one." Such a node MUST fail closed on enforcement (stall) while continuing to read and operate on best-known state. This is the corroborated-not-proven nature of completeness-ahead (B3): final state is established by corroboration reaching the node, never by a lone node's self-certification. The caveat is the boundary of what the read/enforce line guarantees. It makes enforcement safe (never on stale authority) at the cost of enforcement liveness under isolation (a fully isolated node stalls enforcement for as long as it stays isolated), which is the correct trade for a governance layer: delay over breach.

Rationale: naming the two states and gating enforcement, not reads, on the forward-complete one is what makes fail-closed safe rather than paralyzing. `Design` for the distinction and the gating rule; the isolated-node liveness limit is intrinsic (it cannot be closed without corroboration) and is stated, not claimed away.

## A11. Three registers of response: mute, governance, and fork

Not every interpersonal friction needs the group to act. Because all state is local and all presentation is local, there are three registers for responding to a problem, escalating in cost and visibility, and the design pushes each response to the lightest register that suffices.

- **Mute** is a purely local, presentational act. A persona MAY mute another persona or a whole group; this changes no shared state, produces no governance fact, rolls no epoch, and announces nothing. It affects only what the muting persona renders, on their own device, and is freely reversible. Mute is the first reach for "I do not want to see this," and it preserves the group entirely because it touches no one else's state.

- **Governance** (vote, role change, removal) is the shared register: it produces folded facts, and when it changes membership it enacts an epoch and splits the audience (A15). It is for when the *group* must change, not when one persona's view must.

- **Fork and exit** is the floor (p10-drystone-authority-and-complement.md §5): always available, for when a persona cannot accept the group's shape at all.

The principle: push response to the lightest register that suffices. Most interpersonal friction is a presentation problem, not a governance problem, and resolving it by mute is cheaper and less fracturing than a vote. Governance is for group change; fork is for departure. `Design.`

Standing preferences make the lighter registers largely automatic. A persona MAY hold standing, personal, presentational rules, for example "always follow this persona's lineage in a fork," "always join both," "prioritize these personas," or "mute any group without these members-with-standing." These are local, overridable in the moment, and they resolve most fork placements and mutes without any prompt. Mute-as-a-standing-rule and fork-placement-as-a-standing-rule are the same kind of thing: local, personal, presentational defaults that resolve friction without requiring anyone else to act. `Design.`

## A12. Operation-type precedence: the layered fold

The fold resolves governance facts in a fixed precedence over operation *types*, as a layered fold. This is a determinism mechanism first (it removes a class of ordering ambiguity) and a conflict-reducer second (it shrinks the baseline-incoherence surface that generates forks).

The precedence, highest first, each tier resolved against the settled result of the tiers above it:

1. Threshold changes.

2. Membership removals.

3. Role and capability removals.

4. Role and capability delegations (grants).

5. Membership additions.

The organizing principle is **subtractions before additions, at every level**: settle everything that reduces authority or membership before anything that adds it. This biases each intermediate state toward the more restrictive reading (the fail-safe direction, the same instinct as remove-wins and revoke-wins), so an addition is always evaluated against a world where the relevant subtractions have already landed. Membership brackets roles: removals near the top, additions at the very bottom, role changes nested between, so role grants project onto a settled membership (the R2 effective-roles projection is well-defined) and a role grant attaches only to an already-established member. A just-added member starts clean, and a role for them requires a subsequent grant, consistent with the R2 re-add discipline.

Two boundary rules make the precedence unambiguous and MUST be stated:

- **The precedence is the outer loop; R1 is the inner loop.** Type-precedence orders the tiers; two facts of the *same* type still resolve by causal-LWW with the tiebreak (A13), leaving R1's causal core untouched. The fold resolves tier 1 fully by R1, then tier 2 by R1 against tier 1's settled result, and so on. Type-precedence brackets R1; it does not replace it.

- **A removal's cascade rides at the removal's tier.** A membership removal is also the fact that stamps the ceiling (A3) and causally revokes the member's roles (R2). Those cascade consequences MUST be applied when the removal settles (tier 2), not deferred, so that by the time tiers 3 and 4 run a removed member already has no roles to reason about and the effective-roles projection is consistent at every tier boundary, not only at the end. `Design.`

## A13. Within-tier resolution and configurable tiebreak keys

Within a tier, causal order is always authoritative (the determinism core, R1); the tiebreak applies only among genuine concurrents where causal order gives no answer. The tiebreak *key* is configurable within a safe range, and the range is set by one principle.

**The governing principle: a resolution mechanism may be a default only if it is party-neutral; any mechanism that can privilege a party must be opt-in and itself governed.** Type-precedence (A12) defaults because it orders kinds of operation and privileges no party. A tiebreak key that privileges a party, or the facts of a party, must be opt-in and under k-of-n governance.

Three rungs, increasing in power and in how much they privilege:

- **Canonical hash (the party-neutral floor, default).** Among concurrents, resolve by a canonical, total, ungameable key (the FactId hash). It privileges no one; it is a deterministic coin flip, and it is the default precisely because it is perfectly party-neutral.

- **Join-order seniority (default-eligible option, member operations).** A group MAY opt into resolving concurrent member operations by seniority, the longer-standing member's operation winning. "Join order" MUST be defined as a settled causal or logical position in the governance history (order, not time), never a wall-clock stamp, because a wall-clock value differs across nodes and would reintroduce the nondeterminism the tiebreak exists to prevent. Join-order can itself tie (two members added at concurrent positions), so the canonical hash remains underneath as the final total fallback. It is default-*eligible* rather than an unconditional default, because seniority is neutral in mechanism (nobody sets it) but not in effect (it favors the long-standing), and a flat group may prefer the pure-neutral hash. Its value is that a resolved tie becomes explicable ("the longer-standing member's operation won") rather than arbitrary.

- **Instance weighting (opt-in and governed; no default).** A group MAY assign weights so that among concurrents a higher-weighted fact of a type wins, for example marking certain threshold changes "constitutional" so they outrank routine ones, or "always process this specific threshold first." This is a protocol-defined and protocol-honored mechanism with **no default weighting**. It privileges specific facts and through them specific parties, so it MUST be opt-in, and the weight assignments MUST themselves be under the group's k-of-n governance and revocable, never self-asserted, because a settable weight is a lever of authority and a self-assertable one would be a backdoor to controlling the rules by out-weighting rather than out-voting. Under governance it is simply another expressible governance shape, safe by the same revocable-and-exitable logic as any concentration (p10-drystone-authority-and-complement.md §4). `Design`; any non-hash default and per-archetype tiebreak defaults are Open (B9).

## A14. The fork: who lands where, and how it is announced

When a genuine same-facts disagreement escalates to a fork (B1), the population splits into three roles, and placement follows from role.

- **Voters (the dissenters)** are the delta. Their expressed intent is the disputed quantity, so they have effectively placed themselves by voting and land in the lineage their vote implies. Where a voter's landing is ambiguous they MAY be asked to clarify, because their intent is the thing in dispute.

- **Bystanders** expressed no position, so by default they are members of *both* resulting lineages (A16) and are *offered but not forced* a choice of where to remain. Standing preferences (A11) resolve most of these placements automatically and without a prompt, so the in-the-moment choice becomes the genuine exception rather than the rule.

- **The subject of a contested removal** is placed by the *outcome* in each lineage, not by their own preference: in the lineage where the removal succeeded they are out (ceiling stamped), in the lineage where it failed they are in. The subject does not get a "both" option for their own membership, because otherwise a member could dodge a legitimate removal by choosing the lineage that keeps them.

The choice prompt, where one is needed, states the disagreement factually, not editorially, consistent with the provenance principle (A0): for example, "Bob and Bernice voted to remove Tom; Tom and Sarah voted against," with clear shorthand names for each lineage, "both" as an easy default, and "neither" made deliberately harder to select by accident, since leaving both rooms is a real loss and should not be a slip. The fork announcement signed on the governance change that produced the fork MUST carry this factual statement of cause in the dataplane ("this group split on the vote to remove Tom"), so the event is legible as a social event with a stated cause. `Design.`

## A15. Hold, enactment, and the audience split

An epoch roll is a fork at the confidentiality layer. The moment a membership change enacts, the members inside the new epoch can read what those outside it cannot, and "two sets of people who can no longer all read the same thing" is what a fork *is*. So the governance-outcome fork and the epoch or audience split are the same phenomenon at two layers, and the data model MUST honor this: the audience line the epoch draws and the membership the governance decision reaches MUST agree on who is on which side.

This forces the ordering and resolves the tension between a hold posture and eager enactment:

- In a **hold** posture (A17), a detected conflict holds the *decision* (flagged, not finalized), and because the enforcing commit fires only on a finalized decision (A4, A6), **the hold suspends enactment**: no epoch rolls, so the audience is not split cryptographically while the members are still resolving. The hold MUST act at the decision layer, before enactment, or it holds nothing, since an epoch that already rolled has already split the audience.

- The two-phase interval (A6) is what makes this coherent: decide first, enact second, so the audience split always follows a settled decision rather than racing it.

The separation of directive from incidental is a consideration, not a cliff: an epoch roll is *incidentally* an audience split whether or not it was a *directed* fork, so the design treats the confidentiality consequence faithfully (the data model reflects it) while not treating every epoch as a declared schism. Mute and standing preferences (A11) further mean many socially-coherent separations happen with no governance act and no epoch at all, purely in local presentation. `Design.`

## A16. After the fork: being in both is permanent, and merge is cheap, not required

A durable fork produces two independent, equal groups that share a past. Being a member of both is a permanent, fully-supported end-state, not a transitional condition that must resolve. There is nothing to reconcile, because two rooms that used to be one is a coherent standing state, exactly as a mutual friend of two now-separated people is genuinely still both their friends. A persona in both MAY exit either, mute either, or remain in both indefinitely.

- **Merge is available and cheap, but not required.** The remedy for an *accidental* fork (a timing near-miss) is an *intentional* reconciliation, and reconciliation MUST preserve history continuity and "room" continuity, so a group that forked over a misunderstanding and merged back feels, to its members, like the same room that had a brief disagreement, not a destroyed group and a new one. This continuity is what makes soft-healable forks usable rather than merely possible, and it is a requirement, not a nicety: without it every fork would be experienced as a small death and "forks are cheap" would be false in the way that matters.

- To support recognition and reunion, lineages MUST retain a shared ancestor identity that survives the fork, so a merge can recognize "these two are the same room, reunited" and stitch their histories. Because non-merger is not a problem to fix (it is just two groups now), the merge machinery is a convenience for reconciliation, not a mandatory cleanup. `Design.`

## A17. Posture, dials, and the protocol/product division

There is no single correct configuration, because respecting variety and canonical local state means different groups genuinely want different thresholds. What exists is not right-versus-wrong settings but better-and-worse *matches* between a group's temperament and its configuration, with all the mechanisms safe in every configuration. This is the social-layer expression of the governance-concentration spectrum (p10-drystone-authority-and-complement.md §4): the protocol provides a safe range and lets groups choose their point in it.

The two escalation sub-decisions left open earlier are resolved not by picking one, but as **posture dials with temperament-keyed defaults**:

- **Auto-fork versus hold-on-conflict** is a dial. Hold-on-conflict (flag the disputed slot, suspend its enactment per A15, let members resolve) is the default for high-cohesion archetypes where a disagreement is likely a misunderstanding to talk out; auto-fork (split immediately, no human beat) is available for groups preferring clean automatic splits.

- **Merge-as-routine versus fork-as-durable** is a posture dial; the *capability* to heal is always present (A16), and whether a group treats forks as routine-and-healed or rare-and-meaningful is its posture.

The protocol's job and the product's job divide cleanly, and this is the frame for the whole layer:

> Making it possible faithfully is Drystone. Making it representable and as easy as possible is the product layer.

- **The protocol (Drystone) is composable, unopinionated, and safe in every configuration.** It provides the mechanisms (mute, the three registers, operation-precedence, the tiebreak-key range, voter/bystander/subject placement, hold-suspends-enactment, permanent-both, cheap merge) and imposes no social shape. Composability is why it gets complicated, which is correct at the protocol layer: it must support all social shapes safely, not choose one.

- **The product is opinionated, legible, and defaulted by temperament.** It chooses which dials to expose and defaults them to a coherent, temperament-matched posture on an 80/20 path, so most groups never touch a dial and the default *is* the posture. Defaults SHOULD be keyed to group archetype (friends-and-family, professional association, social or logistics group) and may vary by group size and trust posture, each with its own likely failure modes, and the defaults do double duty: they work out of the box and they demonstrate what the postures mean. The application layer owns keeping this discernible, since more exposed dials is more power and more UX burden.

The through-line, shared with the foundations: **the protocol faithfully represents individual, local, personal choice, and never centrally resolves social conflict, because social conflict has no central resolution, only individual responses that aggregate.** The anchoring example is a literal divorce with a shared friend group. When group A and group B want to merge but A has banned a member of B, there is no technical fix and no correct group-level answer, because the right outcome differs per person: some mutual friends stay close to both, some pick a side, some drift. The resolution is that each persona acts as themselves (join, exit, mute) and the group-level outcome is the aggregate of those choices, a realization of the social adjudication, not an input to it. Trying to make the group decide *for* everyone would be imposing a fiction, the same way a central host resolving a divorce's fallout would. So the default dials SHOULD mostly orient toward preserving the group and letting exit shine as the most empowering right, and the protocol's contribution is faithful possibility, never resolution. `Design.`

---

# Part B: Challenges and open questions (distinct; resolve or test before merge treats Part A as earned)

`This tier is deliberately separate. Part A is specified; the items here are decisions still owed, properties not yet established, or seams a merge must reconcile. None of them is closed by the reasoning above.`

## B1. Escalation, not adjudication: distinguishing lag from genuine disagreement (and two sub-decisions)

Earlier drafts framed this as choosing a vote-coherence rule (strict baseline, loose baseline, or a dependency-scoped middle) that the fold would use to *resolve* baseline-incoherent quorum assembly. That framing is retired. Resolving the ambiguity to a single forced outcome would install a center to pick the winner, which the premise forbids (p10-drystone-authority-and-complement.md). The resolution is instead: each node assembles quorums against its own view (A1), and a baseline disagreement is not resolved by the fold but detected and escalated to §7.6 fork-as-escalation. A group that made itself hard to argue in is handled the same way as any other disagreement: work within the rules, revoke, or fork and exit.

What remains genuinely open is the detection line, which is real work and the same shape as the gap-completeness question: distinguishing a node that is merely *behind* (has not yet seen a threshold-raise or a re-add, and will converge once the fact arrives, benign liveness) from a node that holds the *same facts* and still computes a different quorum outcome (a genuine disagreement that escalates). The rule is: escalate only after nodes can establish they hold the same head and still disagree; before that, presume lag and heal by convergence. This gate reuses the shared-completeness signal of A8, so the "do not over-fork on lag" guarantee is the same guarantee as "do not finalize on stale state." Defining and testing that line precisely is owed. `Design` for the escalate-not-adjudicate resolution; `[confirm]` for the exact lag-versus-disagreement detection threshold.

The two sub-decisions that were open inside escalation are now resolved, not by picking one, but as **posture dials with temperament-keyed defaults** (A17): auto-fork versus hold-on-conflict is a dial defaulting to hold-on-conflict for high-cohesion archetypes, and merge-as-routine versus fork-as-durable is a posture dial with the healing *capability* always present (A16). The escalation mechanism itself (who lands where, the factual announcement, hold-suspends-enactment) is specified in A14 and A15. So B1 is closed on structure; what remains open is only the exact lag-versus-disagreement detection threshold (above) and the per-archetype preset values (B4, B9). `Design` for the resolution; `[confirm]` for the detection threshold and presets.

## B2. The read/enforce line (now drawn in A10; closed, pending merge into §7.3.3)

`Resolved this cycle.` A8's fail-closed-on-finality was coherent only if the spec separated best-known-state from final-state-for-enforcement, and §7.3.3 as it stood did not, since it addressed completeness behind a checkpoint, not ahead of one. That line is now drawn in A10: best-known state (always readable, never gated) versus final state (forward-complete, required for irreversible enforcement), with fail-closed applying to the enforce side only so a partitioned node reads and operates rather than freezing. The only action remaining is mechanical: fold A10 into §7.3.3 at merge, extending that section from backward-completeness (verifiable truncation) to forward-completeness (finality for enforcement). The residual substantive limit, that an isolated node cannot self-certify final state, is not a gap in this line but the intrinsic corroborated-not-proven property tracked in B3.

## B3. Completeness-ahead is corroborated, not proven, and the ceiling is only the membership half

The ceiling (A3) closes the membership-entitlement tail-gap: whether an actor is still entitled is checkable against a durable marker. It does not close the general tail-gap for role and threshold slots, where a current member can be behind on a grant or threshold change with no ceiling-equivalent to check against. Those rely on the now plus the finality gate plus the freshness attestation (A7, A8). And the freshness attestation itself is corroboration: a node in a partition fundamentally cannot self-certify that nothing newer exists, so completeness-ahead is established only when attestations reach it, and its absence forces a fail-closed stall rather than a proof of currency. The residual, honestly stated: for membership, the dangerous permanent divergence is converted to a benign catch-up on a durable ceiling marker; for general governance, currency rests on quorum-attested freshness that an isolated node cannot manufacture. This is the single remaining load-bearing property. `Load-bearing, unearned.`

## B4. The enactment dial and posture presets

A5 leaves the enactment dial's default rung and fallback intervals unspecified (convergence-timeframe-dependent policy), and A17 establishes that the conflict-posture dials (auto-fork versus hold-on-conflict, merge-as-routine versus fork-as-durable) default by group archetype. What is owed is the concrete presets: the enactment intervals and default rung, and the per-archetype posture defaults for friends-and-family, professional association, and social or logistics groups, on an 80/20 path. The structure is settled (A17); the preset values are Open. `[confirm.]`

## B5. The now's concrete schema

A7 specifies the now's contents and invariants but not its wire schema: how in-flight tallies are represented, how the binding-to-chain-head reference is encoded, and how the corroborating signature-set is attached. This is a serialization detail, but the tally representation in particular must be pinned so the fallback signal (A5) is unambiguous. `[confirm.]`

## B6. The concurrent tiebreak key

A3 and A7 canonicalize concurrent heads by the R1 tiebreak, which p10-drystone-fold-semantics.md leaves as FactId versus a canonical composite. Any total order identical on all nodes satisfies correctness, so this is a preference inherited from the fold-semantics open item, not a new question. `[confirm.]`

## B7. Epistemic status ledger (hold these lines exactly; do not let the recent runs nudge them up)

- **Order-independence of the fold** is `constructive` given gap-completeness (from R1 through R4), and has been `exercised` against discriminating tests (the v2 run: causal-precedence, cascade-by-projection, and their permutation-invariance all passed against a faithful reference fold). It is **not** established in production, because no production fold exists in the repository tested. Status: constructive-and-exercised, not-production. Do not upgrade.

- **Gap-completeness** is `Load-bearing, unearned`, now narrowed. Referenced gaps are detected and recover (verified). The membership half is addressed by the ceiling construction (A3, specified here, not yet tested). The general half rests on quorum-attested freshness (A8, B3). Tail gaps that hide a causally-later fact with no held reference to it remain undetectable by references alone. Status: narrowed and mechanized, not earned. Do not upgrade.

- **What the tail-gap experiment "passing" meant.** The Stage 2 tail-gap test's success criterion was faithfully reproducing the gap. "Passed" there means "correctly exhibited an unsolved problem," the opposite of "solved it." The mechanisms in Part A (ceiling, now, finality gate) are the intended resolution, specified here and **not yet tested**. This sentence exists so a later reader cannot misread "the test passed" as "tail gaps are handled." `Design` for the resolution; `Load-bearing, unearned` for its verification.

## B8. Experiment properties this specification implies (for the next convergence-brief extension)

Recorded here so the verification path is legible, not built in this document:

- Quorum-folding: a sub-threshold vote set leaves the slot unchanged on every node; the k-th concordant vote, assembled against a node's own view, flips it. Unilateral (sub-k) enactment is detectable as a fork origin.

- Ceiling-on-completion and its idempotence: the completing signer stamps the ceiling at the crossing head; concurrent and unanimous completion produce concordant ceilings that union to one canonical head, permutation-invariant.

- Assemble-against-own-view: a node's quorum decision is a function of its own current view; the fold does not reconcile divergent baselines into a forced outcome. Test that two nodes with different observed baselines each decide against their own view, and that a disagreement surviving equal heads escalates rather than being silently resolved.

- Enactment dial: a single enactor produces one epoch; signer-fallback on completer-dropout converges with bounded redundant commits; any-member mode converges; no-enactor-present leaves a decided-but-unenacted removal that enacts correctly on return; all enactment paths reach the same epoch state.

- The now: verifiably derived from the chain; a stale now is caught by the finality gate rather than producing wrong enforcement; concurrent-head nows reconcile to the same later now; corroborating signatures union without forking the head.

- Finality gate: an action at or beyond an actor's ceiling is void; a current member behind on a ceiling marker heals on propagation (benign); enforcement fails closed (stalls) when currency cannot be established, and never gates best-known reads or content-plane liveness.

- Ban/fork equivalence (A9): a ban and a voluntary fork produce the same lineage-divergence outcome; the ban carries a quorum-stamped ceiling artifact and the solo fork carries none; a banned member's lineage remains intact (not deleted) holding state up to the ceiling.

- Escalation detection (B1): a disagreement that is mere delivery lag heals by convergence and MUST NOT escalate; a disagreement that survives nodes holding the same head escalates to a fork; the detection is gated on the shared-completeness signal so the group does not over-fork on lag. Under a hold posture, the disputed slot's enactment is suspended (no epoch rolls) while members resolve; under an auto-fork posture the lineage splits immediately; a soft-healable fork re-merges when a re-vote against the current baseline reconciles the lineages, preserving history and room continuity.

## B9. Tiebreak-key and instance-weighting defaults

A13 sets the safe range of within-tier tiebreak keys and the principle that party-neutral mechanisms may default while party-privileging ones must be opt-in and governed. What is Open is the default *choice* within that range: the canonical hash is the party-neutral floor and the safe default, but whether any archetype should default to join-order seniority instead (and for which operation types) is unresolved. Instance weighting has **no default weighting at all** at present; it is a protocol-defined, protocol-honored, opt-in mechanism whose weights sit under k-of-n governance, and whether any archetype ships a default weighting scheme (for example, a "constitutional threshold" preset) is Open. The mechanism and its safety rule are settled (A13); the defaults are not. `[confirm.]`

---

## Changelog

`Working draft; transitions recorded here per the suite's doc-method.`

- **Draft, first consolidation of the quorum, ceiling, enactment, now, and finality mechanisms.** Splits into a settled normative tier (Part A) and a distinct open-questions tier (Part B), so a merge can fold Part A into §7.3.1 and §7.3.3 while Part B remains the checklist of decisions and tests still owed. A0 states the sign-the-state-not-authorship principle that makes the mechanism concurrency-safe; A1 amends R1 to fold quorums rather than votes; A3 introduces the membership ceiling; A4 through A6 separate decision from enactment with the enactment dial and the two-phase interval; A7 specifies the now; A8 specifies fail-closed-on-finality with the read/enforce separation.

- **Escalation and the ban/fork unification.** A1 reframed so a node assembles quorums against its own view and a surviving disagreement escalates to §7.6 rather than being adjudicated by the fold; A3's concurrency rule refined to union-within-a-lineage, diverge-across-a-fork; A8 extended so the fork trigger reuses the shared-completeness signal (do not over-fork on lag is the same guarantee as do not finalize on stale state); A9 added stating ban and voluntary fork as one primitive, equal in outcome and distinct in artifacts, with a ban being a forced fork that leaves the member intact rather than a deletion. B1 rewritten from "pick a coherence rule" to "escalate, not adjudicate," retiring the strict/loose/dependency-scoped candidates and flagging the two remaining sub-decisions (auto-fork versus flag-and-hold; soft-healable versus durable) with leans. B8 extended with the ban/fork and escalation-detection properties. The mechanism side of the inherent right to fork now aligns with p10-drystone-authority-and-complement.md §5, and the two docs cross-reference on that identity.

- **The read/enforce line (A10), closing B2.** Added A10 drawing the distinction A8 relies on: best-known state (folded from what a node holds, always readable, never gated) versus final state (forward-complete, established via a quorum-attested now with no later-head attestation, required for irreversible enforcement). The gating rule is stated: reads and content-plane liveness never gated, enforcement gated on final and fails closed only for the specific action, so a partitioned node degrades to a stall on unmakeable enforcement rather than freezing. The isolated-node-cannot-self-certify limit is flagged as intrinsic (tracked in B3), not a gap. B2 is updated from an open prerequisite to closed-pending-merge: A10 folds into §7.3.3, extending it from backward-completeness (verifiable truncation) to forward-completeness (finality for enforcement). This unblocks A8 from merging.

- **The conflict-handling layer (A11 through A17), closing B1 on structure.** Added the three registers of response with mute as a first-class local presentational primitive and standing preferences that collapse most fork-placement to no-prompt defaults (A11); the operation-type precedence as a layered fold, subtractions-before-additions with membership bracketing roles, with the two boundary pins that the precedence is the outer loop over R1 and that a removal's cascade rides at the removal's tier (A12); the configurable within-tier tiebreak keys (party-neutral hash default, opt-in join-*order* seniority defined as causal position not wall-clock, opt-in-and-governed instance weighting with no default) under the principle that party-neutral mechanisms may default while party-privileging ones must be opt-in and governed (A13); the fork's voter/bystander/subject placement, the factual fork announcement, and the choice prompt (A14); hold-suspends-enactment and the epoch-roll-is-an-audience-split truth (A15); permanent membership of both post-fork lineages with continuity-preserving cheap-but-not-required merge (A16); and the posture dials with temperament-keyed defaults plus the protocol-composable/product-opinionated division framed by "making it possible faithfully is Drystone, making it representable and easy is the product," anchored by the divorce example and the through-line that the protocol faithfully represents individual local choice and never centrally resolves social conflict (A17). B1's two sub-decisions are resolved as posture dials; B4 now covers posture presets; B9 added for the tiebreak-key and instance-weighting defaults (no default weighting at present). This layer is the social-layer expression of the authority-concentration spectrum in p10-drystone-authority-and-complement.md §4.

- **Representation substrate linked.** The byte-level representation this doc rests on (fact encoding, FactId, the two linked chains, the now's derivation, and the three signing roles behind A0, A1, and A7) is now specified in the new companion p10-drystone-fact-and-chain-representation.md, added to the companion line and referenced from A7. No normative change here; the linkage makes explicit that A0's sign-the-state, A1's quorum-as-k-facts, and A7's derivable now are grounded in canonical dag-cbor facts with a single committed-head causal reference, and that the now's genesis-derivability floor leaves B3 (completeness-ahead) untouched.

- **Relationship to the fold-semantics doc.** A1 amends R1 there; the two should merge together into §7.3.1. The order-independence result of that doc is unchanged in status by this document (B7).

- **Status discipline.** Part A is `Design` throughout unless a clause is `Verified-RFC` (the single-commit-per-epoch facts). The load-bearing risk is isolated in B3 and the status ledger B7 holds the order-independence and gap-completeness lines exactly where the prior runs left them, explicitly refusing to upgrade them on the strength of specification alone. Verification is B8's job, not this document's.

- **Doc-model standard pass.** Added a Terms block up front for spec-readiness; Part B continues to serve as the distinct open-items register. No normative change.
