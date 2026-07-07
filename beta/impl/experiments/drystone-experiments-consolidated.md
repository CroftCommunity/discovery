# Drystone experiments: consolidated definition, prior results, and open questions

`Status: single coding-agent brief. Supersedes the three convergence-experiment briefs (v1, v2, v3) by merging them into one staged program, and adds the prior results and the full open-question inventory.`

`Scope: what to build and test. The semantics are authoritative in the specs named in the source map (section 9); this file is authoritative for what to test and in what order.`

`Companion to: p10-drystone-fold-semantics.md (R1 through R4), p10-drystone-governance-finality.md (the A-series), and Part 2 Appendix B (the open-seam inventory).`

---

## 0. What this file is

This is one file a coding agent can execute end to end. It collapses the three prior briefs into a single staged program, records what the earlier run already established, and inventories every open seam so nothing is lost between "the experiment" and "the spec's open questions."

The program tests two layers, stacked:

- **The governance fold's conflict-resolution semantics** (R1 through R4): given a set of governance facts, does the fold produce one identical authority state regardless of delivery order or gaps. Stages 1 through 3.

- **The governance-finality mechanisms** (the A-series): how a k-of-n decision becomes effective (quorum-folding), how a removal bounds authority (the ceiling), how a node knows its authority view (the now and the fail-closed finality gate), how a decision is enacted (the enactment dial), that a ban and a voluntary fork are one primitive, and how genuine disagreement escalates rather than being adjudicated. Stages 4 through 6.

What passing establishes and what it does not is stated up front in section 2 and again per stage in section 8. The single most important frame: this program tests the **specified** mechanisms in a **reference** implementation. No production fold and no production governance-finality implementation exist in `croftc/securitypolicy`. A green run is strong evidence about the specified semantics, never a statement about production, and never a proof.

---

## 1. Prior results

The v1 experiment was built and run. What it established, and what it left, is the starting point here.

- **v1 ran and its Stage 1 passed.** A reference stub, built because no production fold existed, passed permutation invariance. The stub used a trivial placeholder rule (highest-FactId-wins) whose order-independence was real but uninformative.

- **v1 surfaced four open semantic questions, OQ-1 through OQ-4,** where the specification did not answer what the fold should do (for example, a role granted to a concurrently-removed member). The stub had to pick placeholders, and flagging those rather than resolving them silently is what let them be found.

- **Those four questions were then resolved into R1 through R4** in `p10-drystone-fold-semantics.md`: OQ-1 to R1 (causal order is authoritative; the tiebreak orders only concurrents), OQ-2 to R2 (cross-slot effects are projections on the final sets), OQ-3 to R3 (no fold-time semantic-validity rejection; idempotent no-ops), OQ-4 to R4 (repeated threshold changes resolve by R1). R1 through R4 were proposed as `Design` resolutions for Part 2 section 7.3.1; the v2 reference fold now demonstrates them, so as reference-model logic they are `Modeled`, while the fold's whole-system order-independence stays a **consequence** conditional on gap-completeness (the beam, section 6.2) and remains `Load-bearing, unearned`.

- **v2 has now run, and Stages 1, 2, and 4 pass on a faithful reference fold** (not a production fold; none exists in the repository). Stage 1 confirms permutation-invariance and the R1 through R4 resolutions under the A12 layered fold, including the discriminating D6 and D7 that a flat id-only fold fails. Stage 2 confirms referenced-gap detection, where a node holding a fact whose referenced predecessor is absent returns a gap error rather than folding an incomplete set as complete, and records the unreferenced-tail case as the documented limit, which is the completeness-ahead beam. Stage 4 confirms the finality mechanics: quorum folding (A1), non-exclusive recognition (A2), the ceiling (A3), and the now (A7). Stage 3, the adversarial scheduler with equivocation-to-fork and bounded model checking, remains specified but unimplemented. No new semantic ambiguity arose during v2, so OQ-1 through OQ-4 stay resolved, and v3's governance-finality layer is subsumed by the Stage 4 results.

- **Three honesty facts carry into every result** and are stated once here so the stages need not repeat them: real-versus-reference fold; the enactment epoch model is a reference stand-in for MLS, not real MLS; and the Stage 6 post-escalation behavior depends on which B1 posture dial (A17) is set (section 6.3), so any test touching it states the dial setting.

---

## 2. The honesty rule, above all

The first line of every result file states what was tested. There can be up to three facts to state:

- **Real versus reference fold.** If a production fold exists in the repository, wire to it and say so. If not, the fold is a reference implementation faithful to R1 through R4, and the result speaks to the specified semantics, not to production.

- **Reference-epoch versus MLS**, for any result touching enactment (Stage 5). The epoch model is a reference stand-in for commit serialization (one commit per epoch, idempotent in effect). It does not exercise real MLS key schedule, ratchet tree, or Welcome and GroupInfo handling.

- **The B1 posture-dial setting**, for any Stage 6 result touching post-escalation behavior.

Two standing rules complete the discipline:

- A green run against a reference implementation proves the harness and the specified semantics are sound and testable. It is a genuine step up from a trivial stub, and no more. Do not let "faithful reference fold" drift into "the protocol is verified."

- If a semantic ambiguity arises that the named specs do not cover, do not resolve it silently. List it in RESULTS as a new open question, because such a choice may itself be where a latent order-dependence or divergence hides. This is the rule that turned the v1 run into R1 through R4.

---

## 3. The fold contract (R1 through R4)

The fold is a pure function from a set of governance facts to an authority state:

```
fold(facts: Set<Fact>) -> AuthorityState
```

Implement it as a per-slot causal last-writer-wins register with cross-slot projection. The full rationale is in `p10-drystone-fold-semantics.md`; what to implement is here.

**Fact model.**

- `id: FactId`: a content hash in production. For the harness, allow ids to be assigned explicitly (a synthetic ordered id), because Property B needs a causally-later fact carrying a smaller id, which is impossible if the id is a hash of the content. Document that production uses content hashes and that R1 must hold for any id assignment, which is what makes explicit test ids legitimate.

- `author: AuthorId`: a persona or device subspace id.

- `predecessors: Set<FactId>`: the frontier of facts the author had observed when authoring this one. This is the causal dependency reference. R1's causal order and R3's gap-detection both depend on it; a fact model without it cannot implement these semantics.

- `payload`: one of `AddMember(m)`, `RemoveMember(m)`, `GrantRole(m, r)`, `RevokeRole(m, r)`, `SetThreshold(role, k, n)`.

**Causal order.** Build the DAG from `predecessors`. Fact B is causally after A iff A is in the transitive closure of B's predecessors within the folded set. Two facts are concurrent iff neither is causally after the other. A referenced predecessor not in the set is a detected gap (relevant in Stage 2, not Stage 1, where sets are complete).

**Slots.** `member:m`, `role:m:r`, `threshold:role`.

**Fold algorithm, the layered fold (A12) with R1 as the inner loop.**

1. Build the causal DAG from predecessors.

2. For each slot, find the causally-maximal facts touching it (touching the slot and not causally dominated by another fact touching the same slot).

3. Resolve operation types in a fixed precedence (A12), highest first, each tier resolved against the settled result of the tiers above it: (1) threshold changes, (2) membership removals, (3) role and capability removals, (4) role and capability grants, (5) membership additions. The organizing principle is subtractions before additions at every level: settle everything that reduces authority before anything that grants it. This type-precedence is the outer loop.

4. Within each tier, resolve each affected slot by R1, the inner loop: one causally-maximal fact of that tier wins; several concurrent means the greatest by the tiebreak (FactId order); the tiebreak is consulted only among concurrents and never overrides causal order. Threshold resolution at tier 1 is R4 (threshold last-writer-wins by R1).

5. Membership follows from tiers 2 and 5: `member:m` is a member iff its resolved winning operation is `AddMember` with no settled later `RemoveMember`; a `RemoveMember` resolves it to not-a-member. A membership removal's revocation cascade over all `role:m:*` rides at the removal's tier (tier 2), not as a separate later pass.

6. Effective-roles projection (R2): role `(m, r)` is effective iff `role:m:r` resolved to granted and `member:m` resolved to member. Compute this on the final resolved slots, once, never incrementally.

7. Absent targets (R3): operations on absent targets are idempotent no-ops. A remove of a never-added member leaves it not-a-member; a revoke of a never-granted role leaves it ungranted. Never reject a fact at fold time for an absent target.

**AuthorityState.** `{ members: Set, effective_roles: Set<(m, r)>, thresholds: Map<role, (k, n)> }`. Canonical ordering (BTreeSet, BTreeMap) and a `fingerprint()` (canonical serialization then SHA-256). Two states compare equal iff they represent the same authority, by fingerprint, never by debug string.

---

## 4. The finality contract extensions (the A-series)

Build these on top of the fold contract. Keep them the faithful minimum; simplicity is the point. The rationale is in `p10-drystone-governance-finality.md`.

**Vote and quorum.** A `Vote` is a signed fact expressing one member's support for a specific slot transition (for example `RemoveMember(m)`), carrying its author and its `predecessors` like any fact. A slot transition is caused by an assembled quorum: k concordant votes for the same transition, where k is the threshold in force at the relevant causal position (read from the folded state, per R4). Quorum assembly is evaluated against the assembling node's own view. Fewer than k concordant votes leaves the slot unchanged. The k-th vote, folded against a node's view, is the one that crosses.

**Ceiling.** A `Ceiling` records, for a removed member, the governance head as-of which its authority ends. It is stamped by the completing signer as part of the crossing fact, is a fact about the removed member (content-addressed by that state, not by author), and carries the quorum's authority.

**The now.** A `Now` is a materialized current state: settled slot values (members, roles, thresholds) plus in-flight tallies (per pending transition: target, current count, threshold, enacted-yes-or-no). It carries a reference to the history-chain head it was derived from and is verifiably derivable from that chain. It is a replaced value, not accumulating. It advances as facts fold and never itself triggers an epoch. Attestations over a now are a corroborating signature-set attached to it, and are never part of its identity.

**Enactment and epoch model (reference stand-in).** Model an `Epoch` chain where an `EnforcingCommit` (for example, removing m's leaf) closes one epoch and opens the next, at most one commit per epoch (losers rebuild on the new epoch). All `EnforcingCommit`s for the same decision are idempotent in effect: whichever wins, the resulting epoch state is identical, and a loser rebuilt on the winner's epoch is a no-op. This is a reference model of commit serialization, not MLS.

**Lineage and divergence.** A `Lineage` is a chain of facts from a shared history. The divergence primitive is two lineages sharing history up to a head and diverging after it. Two triggers produce it: a ban (a quorum ceases corroborating a member, stamping a ceiling) and a voluntary fork (a member continues from local state, no group artifact). Both leave each side intact.

---

## 5. The staged experiment

Build in stages. Each stage must be green and committed before the next begins. Do not skip ahead. Stages 1 through 6 are the core dependency chain; Stages 7 through 10 each build on named earlier stages rather than strictly on the one before and state their own prerequisite; the integration track is a separate crate against real libraries and can proceed in parallel. Use `proptest` for the invariance properties and hand-written unit cases for the specific scenarios, in the style Stage 1 establishes.

### Stage 1: permutation invariance and the R1-through-R4 properties

**The critical seam: real fold versus stub.** Determine which applies and state it in every result. If a real fold exists in the repository, find it (search for "fold", "governance", "authority state", or "7.3.1"), wire the harness to it, and report the path. If none exists, build the reference fold faithful to section 3 in the test crate and label it a reference fold, not production. This distinction is load-bearing and must never be blurred.

Properties (A is retained from v1; B through F target the R1-through-R4 resolutions and are discriminating, meaning a wrong fold fails them):

- **Property A, permutation invariance.** For any complete valid fact-set S and any permutation, `fold(S) == fold(permutation)` by fingerprint. Generate causally-consistent sets that deliberately include concurrency (facts from different authors with no causal order) and conflicting pairs (two `SetThreshold` on one role; an `AddMember` and `RemoveMember` of one member with no causal order). Support proptest shrinking so any failure reports a minimal counterexample; commit the regression file.

- **Property B, causal precedence beats id order (R1, the discriminating one).** Construct a causally-later fact carrying the smaller id, conflicting on the same slot; assert the causally-later fact wins. Minimal hand case: `Add(m)` [id 5, no preds]; `GrantRole(m, r)` [id 4, preds {5}]; `RevokeRole(m, r)` [id 1, preds {4}]. The revoke is causally last with the smallest id; assert `(m, r)` is not effective. Confirm a deliberately id-only fold fails this, then remove that variant. State that the property must hold for every id assignment (R1's guarantee).

- **Property C, concurrent tiebreak determinism (R1).** Construct genuinely concurrent conflicting facts on one slot; assert the resolved value is the tiebreak winner (greatest id among the concurrent maximal facts) and is permutation-invariant.

- **Property D, role cascade via projection (R2), each also checked permutation-invariant.** D1 causal grant then remove: assert m not a member and `(m, r)` not effective. D2 concurrent grant and remove (siblings on the Add): assert not effective regardless of which wins the role-slot tiebreak, because the membership projection filters it. D3 re-add does not restore: `Add`, `Grant`, `Remove`, `Add` in a chain; assert m a member and `(m, r)` not effective. D4 re-grant after re-add works: extend D3 with a causally-later `Grant`; assert effective. D5: for D1 through D4, every permutation yields identical effective roles. D6 operation-type precedence (A12): construct concurrent facts of different types (a threshold change, a membership removal, a role grant, and a membership addition) on related targets; assert they resolve in the fixed precedence, subtractions before additions, with R1 only as the inner loop within a tier. **Discriminating** against a flat per-slot fold that ignores type precedence. D7 removal-cascade-at-tier: a membership removal's role and capability revocations resolve at the removal's tier (tier 2), so a concurrent grant at tier 4 cannot survive the removal; assert the grant does not take effect.

- **Property E, idempotent no-ops (R3).** E1 fold `{ Remove(m) }` with no `Add(m)`: m not a member, equal to folding empty for m. E2 fold `{ Revoke(m, r) }` with no grant: not effective, no error. E3 `fold({ Remove(m) })` and `fold({ Add(m), Remove(m) })` (causal) agree on m's membership.

- **Property F, threshold LWW (R4).** F1 causal: `SetThreshold(role, 2, 3)` then causally-later `SetThreshold(role, 3, 5)`; resolved is `(3, 5)`. F2 concurrent: two concurrent `SetThreshold`; resolved is the tiebreak winner's value, permutation-invariant.

**Stage 1 acceptance.** The faithful fold passes A through F over a large proptest count; Property B is confirmed to fail an id-only fold and D6 to fail a flat fold that ignores type precedence (variants then removed); hand cases for B, D2, D3, and D6 are explicit unit tests; the regression file is committed. RESULTS states real-versus-reference and faithful-to-R1-through-R4-and-the-A12-layered-fold.

### Stage 2: interleaving and gap-fill

A seeded deterministic simulation, not a network test: N node-models each holding a subset of the fact stream, delivered in different orders with some facts late (a gap that later fills) and some never delivered to a given node. Nodes reconcile pairwise then fold. All order, gaps, and timing come from a seeded RNG (single-threaded, reproducible from the seed), in the style of FoundationDB and TigerBeetle seeded simulation. Run many seeds.

- **Convergence.** Any two nodes that have exchanged the same set of facts hold the identical folded state.

- **Referenced-gap detection (MUST pass).** Give a node a fact G whose `predecessors` include a fact F it does not hold, where F is a causally-later operation on some slot than what the node has folded. Assert the node detects the gap (G references an absent predecessor, per R3) rather than folding its incomplete set and confidently emitting the stale slot value. A silent confident wrong fold under a gap is the failure that would reintroduce a central referee, so this is the highest-value check in Stage 2.

- **Unreferenced-tail gap (documents the limit; expected NOT detectable by references alone).** Give a node a complete-looking set missing a pure-tail fact F that nothing it holds points to. Assert reference-based detection alone does not catch this, and record it in RESULTS as the case the completeness-ahead corroboration and the dataplane checkpoint must handle. Do not fake a pass; the point is to show precisely where reference-based detection stops and the open item (section 6.2) begins.

- **Convergence after fill.** After the missing fact is delivered and reconciled, the previously-gapped node converges to the identical fingerprint as a node that always had the complete set.

**Stage 2 acceptance.** Convergence and referenced-gap detection pass over many seeds; the unreferenced-tail case is recorded as a documented limit, not a pass; the convergence-after-fill check passes; the seed corpus is committed.

### Stage 3: adversarial and bounded-exhaustive schedules (specify only)

Recorded so the staging is legible; do not build yet. Make the scheduler adversarial (search for an order plus gaps and equivocations that maximizes divergence between honest nodes); assert honest nodes still converge once they hold the same facts, and that equivocation is detected (both conflicting facts retained and surfaced as a fork per section 7.6, not one silently overwriting the other). For small models (3 to 5 nodes), add bounded exhaustive model checking (a Rust model checker such as `stateright`, or TLA+ checked with TLC) for a much stronger statement than sampling.

### Stage 4: quorum-folding, ceilings, and the now (A1 through A3, A7)

Pure fold-plane and now behavior; no epoch model yet. Do not start until Stage 2 is green and committed.

- **Group G, quorum-folding (A1).** G1 a sub-threshold set of concordant votes leaves the slot unchanged (discriminating against a "count any concordant signature" fold that flips below k). G2 the k-th concordant vote, assembled against a node's view, flips the slot; fewer does not. G3 a signatory's own single vote does not move the slot for that signatory (discriminating against treating the author's own vote as sufficient). G4 quorum assembly is permutation-invariant: any order of the k votes yields the same decision and crossing head (up to the tiebreak on concurrent completing votes). G5 unilateral sub-k enactment is detectable as a fork origin: an enactment that crossed on fewer than k votes is not silently accepted as quorate; it is detectable and marks a fork origin (the tie to Stage 6 escalation). **Discriminating** against a fold that accepts a sub-k enactment as if it were quorate.

- **Group H, non-exclusive recognition and concurrent completion (A2).** H1 N members who each fold the same k-1 prior votes and add their own each conclude "quorum met," concordantly. H2 the unanimous case (every member believes it is the completer) yields one canonical decision and one canonical ceiling head by idempotent union plus tiebreak, not N rival decisions; permutation-invariant; discriminating against treating "who completed" as an exclusive slot.

- **Group I, ceiling (A3).** I1 on crossing, a ceiling is stamped at the crossing head. I2 concurrent ceilings at different heads within one lineage union; the canonical head is selected by the R1 tiebreak; permutation-invariant. I3 a removed member's authority ends at its ceiling (enforcement tested in group L).

- **Group K, the now (A7).** K1 the now is verifiably derived from the chain (a tampered now fails verification). K2 the now is replaced, not accumulated (advancing it does not grow an unbounded structure). K3 fold-plane changes advance the now and never trigger an epoch; only membership key-changes produce an `EnforcingCommit` (discriminating against committing on every governance change). K4 nows are comparable by bound head; a node reconciling moves to the causally-later now; nows bound to concurrent heads re-derive to the same later now. K5 sign-the-state-not-authorship (the nine-fives test, highest-value in Stage 4): N members attesting the same now produce one now with N corroborating signatures, not N rival nows, and the head does not fork; discriminating against making a signer's now a distinct object by author. K6 in-flight tally correctness: the now reflects "2 of 3" for a pending removal, and a "3 of 3 with no enacting commit" state is representable and detectable (the fallback signal read in Stage 5).

**Stage 4 acceptance.** G, H, I, K pass over a large proptest count; the discriminating cases (G1, G3, H2, K3, K5) each confirmed to fail a stated naive variant (then removed); hand cases for H2 and K5 are explicit unit tests; regression file committed. RESULTS states real-versus-reference and faithful-to-A1-A3-and-A7.

### Stage 5: enactment, the two-phase interval, and the finality gate (A4 through A6, A8)

Needs the reference epoch model; build on the seeded simulator for delivery-timing parts. Do not start until Stage 4 is green and committed. Any result here states, first line, that the epoch model is a reference stand-in for MLS.

- **Group J, decision versus enactment and the dial (A4 through A6).** J1 N concurrent `EnforcingCommit`s for one decision converge to one epoch state; losers rebuilt on the winner's epoch are no-ops. J2 single-enactor path produces exactly one epoch. J3 signer-fallback: the completer drops out before enacting; after the interval a fallback signer enacts; the group converges, redundant commits bounded by signer count. J4 any-member (aggressive) fallback converges; redundant-commit count may be larger, epoch state identical. J5 no-enactor-present: the removal remains a valid decided-but-unenacted state (the ceiling governs authority, no commit yet) and enacts correctly when an eligible enactor returns; discriminating against a design where a decision is lost if not immediately enacted. J6 two-phase interval: after the decision folds but before the enforcing commit, the removed member's actions are not honored by a node that holds the ceiling (authority ends on the fact), though the member is modeled as key-holding until the commit (access ends on enactment). Authority-void is immediate; access-revocation is on enactment.

- **Group L, the finality gate (A8).** L1 an action at or beyond an actor's ceiling is void (enforcement checks the action head against the ceiling). L2 a current member behind on a ceiling marker heals on propagation (honors a now-void action before it holds the ceiling, voids it after, nodes converge): the benign liveness case. L3 fail-closed on finality (highest-value in Stage 5): when a node cannot establish its now is current to the edge (no quorum-attested now, or an attestation referencing a later head it lacks), it must not finalize an irreversible authority-enforcing action; it stalls. Discriminating against a fail-open design that finalizes on stale state or annotates-and-proceeds. L4 reads and content-plane liveness are never gated: a partitioned node still folds, serves best-known state with a freshness qualification, and stays live on the content plane; discriminating against a design where fail-closed freezes the node entirely. L5 completeness-ahead is corroborated, not proven: currency is established only via a quorum of attestations over the now, and in their absence the node stalls; test that the stall happens, not that currency is proven.

**Stage 5 acceptance.** J and L pass; discriminating cases (J5, L3, L4) confirmed against stated naive variants; the first line of RESULTS states the epoch model is a reference stand-in for MLS.

### Stage 6: ban/fork equivalence and escalation (A9, B1)

Needs the lineage/divergence primitive; build on the seeded simulator for the same-facts-versus-lag distinction. Do not start until Stage 5 is green and committed.

- **Group M, ban/fork equivalence (A9).** M1 a ban and a voluntary fork of the same member produce the same lineage-divergence structure (divergence at a head, both sides intact). M2 distinct artifacts: the ban lineage carries a quorum-stamped ceiling, the voluntary fork carries no group-side artifact; a third party can tell them apart by the presence or absence of the ceiling. M3 a banned member's lineage is intact, not deleted (it retains all state up to the ceiling and can continue in its own lineage); discriminating against modeling a ban as deletion or state loss.

- **Group N, escalation detection (B1).** N1 a delivery-lag disagreement heals by convergence and does not escalate (a node merely behind converges once the fact arrives). N2 a same-facts disagreement escalates: two nodes holding the same head that still compute different quorum outcomes trigger a fork, not a silent forced resolution; discriminating against a fold that silently picks a winner (which would reintroduce a center). N3 escalation is gated on the shared-completeness signal: a fork fires only after equal-head is established; before that the disagreement is presumed lag and heals; the same signal as L3 and L5; discriminating against forking on transient delivery differences. N4 (posture-dial-dependent, state the dial setting): the two escalation sub-decisions are resolved as posture dials (A17), so this tests each dial value's specified behavior, not an open choice. Under hold-on-conflict, the disputed slot enters a held state with enactment suspended without fragmenting the group; under auto-fork, the lineage splits immediately; under merge-as-routine, a re-vote against the current baseline reconciles the diverged lineages, preserving history and room continuity; under fork-as-durable, it does not. The healing capability is present under both merge dials (A16).

**Stage 6 acceptance.** M and N1 through N3 pass; N2 and N3 confirmed against stated naive variants; N4 tested per posture-dial setting (A17), each a specified behavior, with the setting recorded in RESULTS.

### Stage 7: capped-versus-uncapped root soundness (Part 1 2.3 and 2.7; the priority open question)

The priority open item. A center is authority that is irrevocable and inescapable, and the substrate's whole claim is that no such authority exists, because a member can always fork away. A root authority tests that claim at its sharpest: does a root, capped or uncapped, preserve the center-free invariant, or does an uncapped root become a center. Build on the fold, the ceiling (Stage 4), and the divergence primitive (Stage 6). This stage's job is partly to inform the design decision, not only to check a fixed one, so it is expected to produce a finding, not only a pass.

Model a `Root` as a principal holding a distinguished top Group Role Set. Test both a capped root (authority bounded by an enumerated, revocable scope) and an uncapped root (authority unbounded).

- **Group P, the escape-hatch invariant (the center test).** P1 under a capped root, any member can still fork away into an intact lineage (the divergence primitive fires, both sides intact); the fork is unblockable regardless of the root's acts. P2 under an uncapped root, the same must hold; a model where an uncapped root can prevent or invalidate a member's fork fails the center-free invariant, and this is the discriminating result the priority question turns on. P3 a capped root is itself revocable by the normal quorum path (it can be bounded or removed by quorum at its causal position); an uncapped root that no quorum can bound is authority that is irrevocable, the other half of the center definition.

- **Group Q, soundness under root change.** Q1 a change to the root (bounding, rotation, or removal) folds order-independently and converges, as any other governance fact. Q2 concurrent root-change facts resolve by the R1 tiebreak and never fork the head. Q3 a member out-voted on a root change has the fork as its remedy, not an override (the same forced terminus as any governance loss).

**Stage 7 acceptance.** P and Q pass over a large proptest count; P2 and P3 are each confirmed to fail a stated uncapped-root variant that blocks a fork or resists all quorum bounding (the failure the analysis predicts, recorded as evidence bearing on the design decision); RESULTS states which root model each result used and reports plainly whether the center-free invariant held under each, and whether an uncapped root can be sound, and if not, why the cap is load-bearing.

### Stage 8: recovery delegation and break-glass (the recovery ladder)

Succession is not a special primitive. A Group Role is not intrinsic to its holder, so a lost or vacated role, including the root, is reassigned by normal governance (planned handoff is a revoke-then-grant that folds and converges, Stages 4 through 7) or, if contested, becomes a fork with full history on both sides (Stage 6). A vacant root is not a brick, because survivors can always fork and re-establish authority among themselves (the floor). The earlier framing of this stage assumed a continuity guarantee, that the top role must never be unheld, and that assumption was itself a smuggled center: an authority whose absence bricks the group is exactly the inescapable authority the design forbids. It is dropped.

What remains is one real primitive: recovering a principal whose key material is lost (death, lost keys, incapacitation), without letting the recovery path become a center. Recovery is general over any principal, not special to the root. The mechanisms form a ladder by strictness (how hard to invoke, hence how resistant to abuse), all instances of one shape, a recovery-scoped role or quorum that restores or reassigns a lost principal's authority, bounded so it can never grant more than the lost principal held (the Stage 7 tie: restore, never seize).

- **Rung 1, custodial read over a shared secret** (least strict, most available). A recovery delegate holds a limited read role over a shared recovery secret (a held share or an encrypted recovery blob). This is a capability, not authority (the Part 1 2.7 distinction, the same standing Carol's node has): the custodian can surface the secret to re-establish the lost holder's key lineage, but holds no Group Role, cannot act as the principal, and cannot govern. The read is attributable and the role revocable. Best where availability matters most and the custodian is trusted.

- **Rung 2, threshold recovery** (k-of-n guardians). No single party recovers; k concordant guardian votes re-establish or reassign the lost authority, on the existing quorum machinery. Abuse requires k colluding guardians. The default for most groups.

- **Rung 3, contestable break-glass** (strictest, least available). The governance quorum reassigns the role only after an announced, time-delayed, contestable break-glass; a break-glass fired while the holder is in fact fine is detectable and revocable or out-forkable before it takes effect. For the highest-value roles.

- **Backstop, survivor fork** (the floor, no delegate). If even a recovery quorum is gone, survivors fork with full history and re-establish authority among themselves. Always available, no pre-provisioning; the cost is a lineage split.

Properties, each rung against the same invariants:

- **Group U, bounded recovery (the center test, highest-value).** U1 recovery restores or reassigns exactly the lost principal's authority, never more; a rung that lets the recoverer end with more authority than the lost holder had fails, and with a capped root (Stage 7) even root recovery is bounded. **Discriminating** against a break-glass that grants unbounded authority (a seize). U2 the recovery delegate is itself revocable and forkable like any role; it is not a center. U3 rung 1 is capability-not-authority: reading the secret does not move any governance slot for the custodian; a model where the custodian gains standing by holding the secret fails.

- **Group V, invocation and abuse-resistance by rung.** V1 rung 2 requires k concordant guardians; k-1 does not recover (the Stage 4 quorum property, reused). V2 rung 3's break-glass is time-delayed and contestable: a false break-glass is detected, and a contest before the delay elapses blocks it or forks; assert the fired-while-fine case does not silently take effect. V3 the recovery decision folds order-independently and converges; concurrent recovery claims resolve by the R1 tiebreak and by quorum, never as rival holders (the Stage 4 group H shape). V4 the backstop always terminates: even with no recovery delegate and no quorum, survivors can fork and continue (the floor, the Stage 6 and Stage 7 fork remedy). This is the property that replaces the dropped continuity guarantee: a vacant role is recoverable-or-forkable, never a brick.

**Stage 8 acceptance.** U and V pass over a large proptest count; U1 and V2 are each confirmed to fail a stated variant (a seizing break-glass, a silent fired-while-fine break-glass); the rung-1 custodial-read case and the rung-3 contest case are explicit hand cases; RESULTS states which rung each result exercised and that recovery is bounded (restore, never seize). Design note: the only new spec object is the recovery-delegation-and-break-glass primitive (the three rungs plus the backstop), a bounded and known pattern (social and threshold recovery), not a from-scratch Lifecycle section. Build once that primitive is specified; it builds on Stages 4, 6, and 7.

### Stage 9: tenure and the survivor and re-key path (5.3)

Builds on the finality machinery (Stages 4 and 5). Tenure is one of the three inherent rights of the floor. The open question is whether the survivor path, where a subset re-keys after a loss, strands tenure: leaves a persona holding a right it can no longer exercise.

- **Group R, no stranded right.** R1 after a survivor re-key, every persona that holds `tenure` in the folded state can still exercise it (can still act and be honored on the surviving lineage); a persona left with `tenure` in state but unable to exercise it is a stranded right and fails this. **Discriminating** against a re-key path that advances the group's key material without carrying the surviving members' rights forward. R2 a persona correctly removed before the re-key does not retain tenure (the ceiling governs, Stage 4). R3 the re-key folds order-independently and converges; a member behind on the re-key heals on propagation (the Stage 5 L2 benign case). R4 tenure and access are distinguished across the re-key exactly as authority and access are distinguished at enactment (Stage 5 J6): a surviving member's tenure is continuous even if its access is re-established by the re-key.

**Stage 9 acceptance.** R passes; R1 confirmed to fail a stated re-key variant that drops surviving rights; RESULTS reports whether tenure is stranded under the modeled survivor path, and any residual the spec's re-key description leaves open.

### Stage 10: dataplane history modes and side histories (7.7, 7.8)

Builds on the fold and the seeded simulator. This is the data plane, largely orthogonal to the governance-conflict resolution of Stages 4 through 6. Two history modes are specified with open threads, forward-only append and Willow-mutable, and side histories are additional threads off a main history.

- **Group S, mode consistency.** S1 forward-only: history is append-only; any two nodes holding the same set converge to the identical history fingerprint; a rewrite attempt is rejected or detected, never silently applied. S2 Willow-mutable: a mutation resolves by the specified newer-supersedes rule, converges order-independently, and a superseded entry is not resurrected by delivery order. **Discriminating** against a mutable-mode fold whose result depends on delivery order. S3 side histories: a side history off a main head is consistent on its own and does not corrupt the main history's convergence; merging a side history back converges. S4 concurrent cross-writer edits: two writer-subspaces edit one logical resource concurrently; assert their entries coexist under the union rather than one silently superseding the other (per-writer subspaces, 5.10), that an application-level timestamp last-writer-wins read-merge is shown to drop one edit, and that a semantic merge (a payload CRDT or a governance-routed mutation) preserves both. **Discriminating** against the timestamp last-writer-wins read-merge, the silent concurrent-edit loss 7.7 forbids.

- **Group T, the open threads.** T1 record, do not resolve, the history-structure design's open threads: for each, a test that pins current behavior and a note in RESULTS marking it as a design thread, not a settled result. Do not fake a pass on a thread the design has not closed.

**Stage 10 acceptance.** S passes; S2 confirmed to fail a stated order-dependent mutable variant; S4 confirmed to lose a concurrent edit under a timestamp last-writer-wins read-merge and to preserve both edits under a semantic merge; the group T threads are recorded as pinned-behavior-plus-open-note, not passes; regression file committed.

### Integration track: real substrate

The stages above test the specified mechanisms in a reference implementation. This track is different in kind: it tests real external libraries, so a green run here speaks to the real substrate, not only the specified model. Build it as a separate crate from `drystone-convergence`, because it links real dependencies and its honesty status differs: results here close specific reference-model gaps rather than exercising a reference model.

**Integration experiment I1: real iroh 1.0 (6.5, 6.9, 6.10, 10.3).** iroh core is 1.0 as of June 2026, so this is buildable now. Test that iroh behaves as the transport design assumes:

- I1a: a relay routes by `EndpointId` and does not decode content (a relay given a sealed frame cannot read it and forwards by endpoint only). **Discriminating** against assuming a relay that must read content.

- I1b: direct-first connection with hole-punch, and a stateless blind relay as fallback (a connection succeeds directly where possible and falls back to the relay without the relay holding session state).

- I1c: the 6.1 identity-plane assumption behaves as specified against iroh's `EndpointId` model. State, first line, that this is real iroh 1.0, and record which assumptions were confirmed and which remain `[confirm]` against a later iroh.

**Integration experiment I2: real MLS (7, 8, 10.2).** Use a real MLS implementation (for example OpenMLS), because the Stage 5 epoch model is a reference stand-in that explicitly does not exercise real MLS. Test the MLS hard-case residuals the fold carries:

- I2a: the external-join hazard behaves as the mitigation assumes.

- I2b: insider replay is prevented as assumed.

- I2c: ReInit non-atomicity is handled as the design states.

- I2d: epoch_authenticator overlap behaves as assumed (the whole-group consistency-detection question).

- I2e: the real key schedule, ratchet tree, and Welcome and GroupInfo handling are exercised, so the enactment logic Stage 5 tested against a stand-in is confirmed against real MLS for the single-commit-per-epoch shape.

State, first line, that this is real MLS, not the reference-epoch stand-in, and record which documented hard cases the mitigations hold against and which remain open. This track is what turns the Stage 5 honest-scope caveat into a confirmed result, one hard case at a time.

---

## 6. Open questions

The complete open-seam inventory is Part 2 Appendix B. Each seam below is mapped to how this experiment program relates to it, so a reader can see at a glance what a green run touches and what it does not. Every seam now maps to a reference-harness stage, the real-substrate integration track, a design prerequisite it is gated on, or a non-experimental resolution; none is left unaddressed.

### 6.1 Seam-coverage matrix

| Seam | Spec ref | Flag | Coverage in this program |
| --- | --- | --- | --- |
| Completeness-ahead beam | 7.3.3, 7.3.7, 7.3.8, 7.9 | Load-bearing, unearned | Partial. Stage 2 referenced-gap and Stage 5 L5 stall exercise it; the proof-ahead residual stays open. See 6.2. |
| Enactment dial and posture presets | 7.3.6, 7.6.9 | [confirm] | Behavior in Stage 5 J3, J4. The default rung and preset choice are non-experimental. |
| The now's concrete wire schema | 7.3.7 | [gates-release] | Behavior in Stage 4 K. The byte schema itself is non-experimental (pinning). |
| Tiebreak-key and instance-weighting defaults | 7.3.1 | [confirm] | Behavior in Stage 1 C and Stage 4 G4 (holds for any tiebreak). The key and weighting choice are non-experimental. |
| Vendor-neutral naming reconciliation | impl | [confirm] | Non-experimental (naming reconciliation). |
| Hash-function reconciliation | 4 vs governance | [confirm] | Non-experimental (choice). The harness is hash-parameterized and runs under any hash. |
| [gates-release] wire encodings | publication | [gates-release] | Non-experimental (byte-encoding pinning for a publication-final DOI). |
| iroh substrate confirmation, post-1.0 | 6.5, 6.9, 6.10, 10.3 | [confirm] | Integration track I1 (real iroh 1.0). |
| Root succession and principal recovery | 7.3 | Design | Stage 8 (recovery ladder); planned handoff is reassignment via Stages 4 through 7, only the bounded recovery primitive is new. |
| Capped-versus-uncapped-root soundness | 7.3, P1 2.3 | open, priority | Stage 7 (the priority; expected to yield a design finding, not only a pass). |
| The open rights check (tenure stranding) | 5.3 | [confirm] | Stage 9 (the survivor and re-key path). |
| False-positive escalation tolerance | 7.4.1 | [confirm] | Behavior in Stage 6 N (lag heals, same-facts escalates, gated on completeness). The tolerance default is non-experimental. |
| What grounds a persona's authority | 3.1, 5.2 | Design, foundational | Non-experimental (foundational design; the proof-of-personhood question). |
| MLS hard-case residuals | 7, 8, 10.2 | [confirm] | Integration track I2 (real MLS; confirms the Stage 5 stand-in one hard case at a time). |
| Dataplane history modes and side histories | 7.7, 7.8 | Design, open | Stage 10 (data plane; open threads recorded as pinned behavior, not passes). |
| Mutable-mode read-merge | 7.7 | Design, open | Stage 10 S4 tests the guard (a timestamp last-writer-wins read-merge loses a concurrent edit; a semantic merge preserves both); the positive mechanism, which CRDT or governance-routed, is open in the spec's Appendix B. |
| External-fact confirmation (Beer, Cybersyn, OGAS) | 7, P1 3 | [confirm] | Non-experimental (primary-source confirmation of the cross-disciplinary grounding). |

Summary of the buckets:

- **Exercised by a reference-harness stage**: the beam (partial, Stages 2 and 5), the enactment dial (Stage 5), the now (Stage 4), the tiebreak (Stages 1 and 4), the escalation tolerance (Stage 6), the capped-versus-uncapped-root soundness question (Stage 7, the priority), tenure stranding (Stage 9), and the dataplane history modes (Stage 10). A stage tests behavior; any default, schema, or key choice inside a covered seam is still a separate non-experimental choice.

- **Confirmed by the real-substrate integration track**: iroh (I1) and real MLS (I2), each a separate crate against a real library, closing the reference model's stand-in gaps one at a time.

- **Defined, pending a small spec**: recovery delegation and break-glass (Stage 8). It needs the bounded recovery primitive specified, a known social-and-threshold-recovery pattern, not a whole Lifecycle section. Succession itself is not separate: planned handoff is reassignment and contested handoff is a fork, both already in Stages 4 through 7.

- **Non-experimental** (a design decision, a byte encoding to pin, or a primary-source confirmation): naming reconciliation, hash choice, the wire encodings, the now's byte schema, the authority-grounding question, and the external-fact grounding.

### 6.2 The load-bearing beam (special)

Completeness-ahead is the single `Load-bearing, unearned` property the governance and scaling claims rest on. Completeness *behind* a known checkpoint is provable; completeness *ahead* is only corroborated. Stage 2's referenced-gap check and Stage 5's L5 stall exercise the mechanisms that lean on it, at the level of "a referenced gap is detected" and "in the absence of a quorum-attested now the node stalls." They do not prove completeness ahead. A green run must not upgrade this property's status. The discharge path is the dataplane checkpoint plus the completeness-ahead corroboration, and it remains open until that mechanism is built and shown, not merely exercised.

The closure must prove four things, none of which a green run substitutes for, and these match the spec's Appendix B beam entry. By CALM (Hellerstein and Alvaro, Theorem 1), a completeness predicate is non-monotonic and so has no coordination-free detector, and center-free is not coordination-free, so the one non-monotonic step, declaring completeness, must be quorum-witnessed or causally-sealed while all else stays monotonic. The four obligations: (1) state the completeness predicate and prove it monotonic, hence exempt, or non-monotonic with the exact coordination named; (2) a liveness argument that the fail-closed stall degrades safely under partition rather than deadlocking governance; (3) a safety argument that a late pre-checkpoint event cannot silently reverse enforced authority, the state-reset class; (4) a fork-composition argument that two honest partitions' checkpoints merge cleanly or fork explicitly, never silently disagree. Stage 2's gap-detection and Stage 5's stall exercise the mechanisms but discharge none of the four; the checkpoint-and-corroboration specification is what must meet them.

### 6.3 The B1 escalation posture dials

B1 resolves the two sub-decisions once open inside escalation not by picking a value but as posture dials with temperament-keyed defaults (A17): auto-fork versus hold-on-conflict, and merge-as-routine versus fork-as-durable, with the healing capability always present (A16). Both dial values are specified behavior, so Stage 6 N4 tests each value rather than assuming one open choice. What remains genuinely open, carried as `[confirm]`, is only the exact lag-versus-disagreement detection threshold (the same shape as the gap-completeness question) and the per-archetype preset values (B4, B9). The settled detection line, that lag heals and a same-facts disagreement escalates gated on shared completeness, is what N1 through N3 test.

---

## 7. Deliverables and layout

A single Rust crate, `drystone-convergence/` (or a `tests/` module in the existing workspace), containing:

- The fold wiring or the clearly-labeled reference stub, faithful to section 3.

- The finality types of section 4: `Vote` and quorum, `Ceiling`, `Now`, the reference `Epoch` and `EnforcingCommit`, and `Lineage`.

- The `proptest` generators and the properties A through T, with the hand-written unit cases and the stated naive variants for the discriminating properties.

- The committed proptest regression file and the seeded-simulator seed corpus.

- The seeded deterministic simulator (Stage 2 onward), single-threaded and reproducible from its seed.

- The Stage 7 through 10 additions: a `Root` model (Stage 7), the survivor and re-key path (Stage 9), and the forward-only, Willow-mutable, and side-history models (Stage 10), with the properties P, Q, R, S, and T and their discriminating variants. The succession types (Stage 8) are added once the Lifecycle section specifies succession.

- A separate integration crate for the real-substrate track: the iroh 1.0 tests (I1) and the real-MLS tests (I2, for example against OpenMLS), each with its own README and RESULTS stating it tests a real library and recording which assumptions were confirmed and which remain open.

- A README stating what was tested, how to run it, the case counts, and the honest-scope paragraph of section 8.

- A RESULTS file whose first line states real-versus-reference fold, and for enactment results that the epoch model is a reference stand-in for MLS, and any assumed B1 sub-decision; then the case counts, what passed, the discriminating-variant confirmations, the outcome of the referenced-gap test (expected pass) and the unreferenced-tail case (expected documented limit), and any new open question the run surfaced.

---

## 8. Honest scope, consolidated

State this plainly so no green run is overclaimed:

- Property-based testing samples the input space; it does not exhaust it. Bounded model checking (Stage 3) exhausts only the bounded case. All stages passing is strong evidence, not a theorem.

- A green run against a reference implementation proves the harness and the specified semantics, not the production protocol. No production fold and no production governance-finality implementation exist in `croftc/securitypolicy`. Only a green run against a real implementation would speak to production.

- The enactment epoch model is a reference stand-in for commit serialization, not MLS. Stage 5 results speak to the enactment logic's convergence and the single-commit-per-epoch shape, not to the MLS key schedule, ratchet tree, or Welcome and GroupInfo handling.

- The integration track is the exception to the reference-versus-production caveat: I1 tests real iroh 1.0 and I2 tests a real MLS implementation, so a green run there speaks to the real substrate for exactly the assumptions it exercises. I2 is what turns the Stage 5 stand-in caveat into a confirmed result, one MLS hard case at a time, scoped to the cases it actually runs.

- Per stage: Stage 1 establishes the specified conflict-resolution model (R1 through R4) is order-independent and causally correct in the reference fold. Stage 2 adds referenced-gap detection and convergence-after-fill, and marks the unreferenced-tail limit. Stage 4 establishes quorum-folding, the ceiling, and the now are order-independent and concurrency-safe, including that concurrent and unanimous completion do not fork the head or the decision. Stage 5 establishes the enactment logic converges under the reference epoch model and the finality gate fails closed on finality while never gating reads or content liveness. Stage 6 establishes a ban and a fork are structurally one primitive with distinct artifacts, and that escalation distinguishes lag from genuine disagreement gated on completeness; its post-escalation behavior is tested per posture-dial setting (A17), each a specified behavior. Stage 7 establishes, or refutes, that the center-free invariant (any member can fork away, no authority is irrevocable) survives under a capped and an uncapped root, and is expected to yield a design finding on whether an uncapped root can be sound. Stage 9 establishes whether the survivor and re-key path strands tenure. Stage 10 establishes the two history modes are convergent and order-independent, with the history-structure design's open threads recorded, not resolved. Stage 8 establishes that recovery is bounded across the ladder (each rung can restore or reassign but never seize) and that a vacant role is recoverable or forkable rather than a brick; its only prerequisite is specifying the bounded recovery primitive, not a whole Lifecycle section.

- The whole order-independence result remains conditional on gap-completeness, per section 6.2 and the fold-semantics doc. Passing is strong evidence for the specified mechanisms, not a proof and not a statement about production.

---

## 9. Source map

- Fold contract and its resolutions: `p10-drystone-fold-semantics.md` (R1 through R4, resolving OQ-1 through OQ-4).

- Governance-finality mechanisms: `p10-drystone-governance-finality.md` (the A-series: quorum-folding, ceiling, now, enactment, finality gate, ban and fork, escalation).

- Scaling and the completeness-ahead beam: `p10-drystone-scaling-and-ordering.md`.

- The open-seam inventory: `p10-full-part2-mechanics.md`, Appendix B.

- Superseded source briefs, kept for history: `drystone-convergence-experiment-brief.md` (v1), `-v2.md`, `-v3.md`. This file consolidates all three.
