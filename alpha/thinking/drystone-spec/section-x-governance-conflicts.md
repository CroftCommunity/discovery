# Drystone Protocol — §X. Concurrent and Conflicting Governance Changes

`Status: DRAFT / ENABLING`

`Realizes: P-Peer-Equality, P-Local-Truth, P-Knowable-Truth`

`Capability mechanism: UNCOMMITTED — see X.5`

`References: Matrix State Resolution v2 (CVE-2025-49090); MSC1441; MSC4289; Willow Data Model; Meadowcap; Keyhive; Sigstore countersigning`

> **Provenance.** Working spec draft, filed 2026-06-25 from the design dialogue
> `../../seeds/transcripts/raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md`.
> DRAFT/ENABLING status. The Matrix/Willow/Meadowcap/Keyhive facts cited below were
> web-verified *in the source dialogue* (2026-06-24) and are **not yet in the corpus FACTCHECK SoT**;
> the load-bearing ones (CVE-2025-49090, room v12 / MSC4289, Meadowcap "no native revocation",
> Willow's unenforceable plaintext timestamp, Seshat-class encrypted-search limits) are flagged for
> independent confirmation before they harden into beta (`ROADMAP_TODO.md` E30). §2 is filed
> alongside as `section-2-peers-rights-capabilities.md`. Beta integration is a separate pass.

---

## X.0 Status and dependencies

This is a self-contained working draft of a single section of the Drystone specification. Everything in it is drafted; it does not stub the surrounding spec. It depends on sections not yet written, named here so a reader can see the seams:

- `P-Peer-Equality`, `P-Local-Truth`, `P-Knowable-Truth` are the named design principles defined in §1 (Design Principles). This section assumes their plain meaning: peers hold authority by right not by capacity; local state is the unit of truth; the system's history is auditable rather than silently mutable.

- §Y (Lifecycle) is referenced by X.3.3 for root-authority succession. It does not yet exist.

- §Z (Capability Wire Format) is referenced by X.5.4 and is gated on a decision this section deliberately defers.

This section is publishable as a defensive prior-art record on its own, once the `ENABLING` markers (byte-level encodings) are filled, and independent of the capability-mechanism decision. The novel and defensible content (the append-only governance log, the timestamp-free causal resolution order, the R1–R6 capability interface, the attributable-acceptance dichotomy, and the regress-breaking termination construction) does not depend on whether Drystone ultimately adopts a Meadowcap-shaped or Keyhive-shaped capability layer.

### Load-bearing dependency chain (read before editing)

The following subsections form a single logical chain. Weakening an earlier link silently breaks the guarantees of later ones. A future editor must not treat any of these as locally editable:

- **X.3.1** establishes that governance facts are append-only entries and authority is a monotonic fold over them.

- **X.6** depends on X.3.1 for the guarantee that authority state never moves backward (no state reset).

- **X.7** depends on X.6 for the attribution dichotomy (knowingly-stale versus concurrently-stale).

- **X.8** depends on X.6's monotonicity for fold termination, and on X.7's frontier commitments for its closure step.

If the append-only property in X.3.1 is ever relaxed, termination (X.8), no-state-reset (X.6), and attributability (X.7) all fall together.

---

## X.1 Premise

A governance system distributed across vaults has no global clock and no consensus round. Two vaults may issue contradictory authority facts concurrently, each valid against the state its issuer had locally observed. The protocol must define a single deterministic resolution that every honest vault computes identically, and must bound the damage a vault can do by acting on authority it no longer holds but has not yet learned it lost.

This is the hardest problem in the governance layer. It is the problem Matrix addresses with State Resolution, and the problem whose edge cases produced a state-reset CVE (CVE-2025-49090) a decade into that protocol's life. Drystone does not inherit Matrix's mechanism, but it inherits the same underlying impossibility, and this section is where it either pays the cost cleanly or hides it.

The design intent throughout is to pay openly. Where Drystone cannot provide a guarantee, the non-guarantee is stated (X.6) rather than buried, because Matrix's experience shows that an unstated version of exactly this hazard is what becomes a CVE.

---

## X.2 Dependency status of the data model

This section commits to two things and defers one.

**Committed: the Willow-shaped data model.** Governance facts live in a namespace / subspace / path structure, addressed and reconciled by range-based set reconciliation. Drystone implements this shape directly in early phases rather than depending on a Willow implementation, so that a later transition to Willow proper is a substitution rather than a redesign. Drystone is built Willow-shaped, not Willow-dependent.

**Committed: the append-only governance log.** Defined in X.3, this is independent of any capability system and holds regardless of how X.5 resolves.

**Deferred: the capability mechanism.** Drystone has not adopted Meadowcap, Keyhive, or a bespoke construction. X.5 specifies the revocation requirements the mechanism must satisfy, then presents the two candidate tracks and the criteria that decide between them in a later phase. No normative text in this section may assume one track.

---

## X.3 Drystone's resolution model

Drystone does not resolve conflicts by recomputing a global state. It defines governance facts as entries in an append-only governance log, itself a Willow-shaped namespace per scope, and resolves conflicts between facts by a total order that any vault can compute from the facts alone.

### X.3.1 Governance facts are entries, not mutations

A governance decision (admit, expel, grant, revoke, amend) is written as a signed entry to the governance namespace of the affected scope. Entries are never modified or deleted. A reversal is a new entry referencing the one it reverses.

This means the governance log has no "current state" to reset to an earlier value. There is only the set of facts, which grows monotonically, and a deterministic function (a left fold) from that set to an effective authority state. A vault that has seen fewer facts computes a stale authority state, never a wrong one, and never one that another vault could weaponize by replaying old entries.

This is the deliberate divergence from Matrix. Matrix's room state can move backward (the state reset) because state is a resolved value recomputed on merge. Drystone's authority can only move forward, because authority is a fold over an append-only fact set. `Realizes: P-Knowable-Truth`.

### X.3.2 The total order over conflicting facts

When two facts conflict (defined in X.4), every honest vault must agree which one takes effect. Drystone orders conflicting facts by the following key, compared lexicographically, lowest wins as the survivor of an exclusive conflict:

1. **Issuer authority rank at the causal frontier.** A fact issued under higher standing authority outranks one issued under lower. (The apparent circularity here is resolved in X.8; rank is computed structurally, not by consulting the contested outcome.)

2. **Governance precedence class** (X.4.2). A fact's type carries a precedence so that, for example, an expulsion outranks a concurrent grant by the expelled party.

3. **Causal length.** The number of governance facts in the issuer's referenced causal history, so a more-informed decision outranks a less-informed one.

4. **Content-address tiebreak.** The BLAKE3 digest of the canonical fact encoding, as a final deterministic discriminator.

The critical design rule, taken from the Matrix MSC1441 review: timestamps appear nowhere in this order. A wall-clock tiebreak is trivially gamed by a vault lying about its clock, which is precisely how an attacker would manufacture a favorable resolution. Matrix's own state resolution used sender power then timestamp as its tiebreak, and "how do we check that a server is not lying about the time" was raised against it in review. Willow reinforces the point from the other direction: a Willow timestamp is an unenforceable 64-bit integer that the protocol explicitly declines to assign meaning to. Drystone's order is therefore causal and cryptographic only.

> `ENABLING:` The canonical byte encoding fed to BLAKE3, and the wire format of a governance fact, must be specified to byte level before two independent implementations can interoperate. This gates the Zenodo DOI for this section.

### X.3.3 The unconflictable root

Each scope has a founding fact, the Drystone analogue of Matrix's `m.room.create` event, that establishes the initial authority. Per the Matrix room v12 lesson (MSC4289), the founding fact's authority over the genesis of the scope is not itself subject to the X.3.2 ordering. It is the base case of the authority-rank computation, not a competitor within it.

Matrix made room creators formally hold infinite, immutable power level precisely because the root of an access-control hierarchy is the one place a resolution ambiguity is catastrophic, since it can hand the entire room to an attacker who manufactures a conflicting root. Drystone takes the structural lesson while rejecting the mechanism: this root authority is **capped, delegable, and revocable-by-succession** rather than infinite (`Realizes: P-Peer-Equality`), but like Matrix it is defined so that it can never be the losing side of a conflict, because a conflict at the root is the one ambiguity an attacker can convert into total capture.

> `ENABLING:` Succession of root authority (how founding authority transfers when founders leave, the cooperative's analogue of Matrix's clumsy upgrade-to-move-creator) is specified in §Y, Lifecycle. It must exist, because a permanently fixed root contradicts the cooperative model, and Matrix's own experience shows that moving the root is the single most dangerous operation in the system.

---

## X.4 Conflict definition

### X.4.1 What conflicts

Two governance facts conflict when applying both, in any order, yields a different effective authority state than applying them in the order X.3.2 selects. Concretely: two facts targeting the same capability subject, where at least one removes or narrows authority the other depends on.

### X.4.2 Precedence classes

Authority-reducing facts outrank authority-expanding facts of equal issuer rank. Expulsion and revocation occupy the highest precedence class. This encodes a conservative default: when two valid decisions collide, the one that reduces exposure wins.

The mutual-expulsion case (A expels B while B expels A, each with equal standing) resolves by X.3.2 steps 3 and 4, and the protocol guarantees exactly one survives, never both and never neither. The losing party's expulsion fact remains in the log as a valid-but-superseded entry, visible for audit, never silently dropped.

---

## X.5 The capability and revocation interface

This is the deferred decision. The section specifies what the mechanism must do, then the two tracks that could do it, then the criteria for choosing.

### X.5.1 Requirements (mechanism-neutral, normative)

Whatever capability mechanism Drystone adopts MUST provide:

- **R1 — Unforgeable grant.** A capability cannot be fabricated by anyone other than an authority entitled to issue it.

- **R2 — Attenuating delegation.** A holder may delegate a subset of held authority, never a superset. `Realizes: P-Peer-Equality`.

- **R3 — Convergent revocation expression.** A revocation MUST be expressible as a governance fact (X.3.1) that folds deterministically into authority state, so that all honest vaults that have synced it agree the capability is void.

- **R4 — Bounded stale-authority exposure.** For a holder that refuses to sync a revocation, the protocol MUST bound the window during which third parties accept the revoked capability. The bound MAY be a time interval, an epoch boundary, or a membership-graph generation, but it MUST be finite and stated.

- **R5 — Forward read exclusion.** After a member is expelled, that member MUST NOT be able to read scope entries authored after the expulsion folds in. Past entries are out of scope (see X.6).

- **R6 — Attributable acceptance.** A vault accepting a write under a capability MUST record the causal frontier of governance facts it had synced at acceptance, so that a later-synced revocation makes the stale acceptance detectable and attributable rather than silent. (Defended in X.7.)

R3 and R6 are the requirements that defeat the Matrix state-reset failure mode, and they hold on either track below. R4 and R5 are where the tracks differ.

### X.5.2 Track A — Meadowcap-shaped (delegated tokens plus epoch keys)

In this track, capabilities are unforgeable delegated tokens, verified by recursive signature checking, attenuated by restriction to a more restrictive area. This satisfies R1 and R2 natively.

The defining constraint: a Meadowcap-shaped token has no native revocation operation. You cannot un-sign a delegation. Revocation must therefore be constructed:

- **R4 via expiry.** Every delegated capability carries a bounded validity window (Willow's time dimension makes this expressible). Maximum exposure from a malicious non-syncing holder is one expiry interval. Short intervals with renewal-on-good-standing convert "revoke" into "decline to renew," which is natively expressible.

- **R5 via epoch keys.** Read access is gated by a per-scope epoch key. Expulsion rotates the epoch; subsequent entries are encrypted to the new key, which the expelled member never receives. This mirrors Matrix's Megolm session rotation on membership change.

Cost: revocation latency is bounded by the expiry interval, not zero. Re-keying is required on every membership reduction. Conceptually simple, well-matched to Willow, weaker revocation immediacy.

### X.5.3 Track B — Keyhive-shaped (convergent membership graph)

In this track, capabilities and membership are a signed graph that syncs alongside content, with removal and re-encryption as first-class convergent operations. This is the direction the membership-graph thinking already pointed.

- **R4 via graph generation.** A removal is a graph operation that, once folded, voids the member's authority at a generation boundary rather than a clock interval. Exposure is bounded by sync propagation of the graph update, not by a fixed expiry window.

- **R5 via convergent re-encryption.** The membership change drives re-encryption of forward content as a designed operation rather than a bolted-on epoch rotation. This is Keyhive's central research claim.

Cost: materially more complex. Depends on convergent-capability research that is itself in flight, so adopting it early couples Drystone to a moving dependency, the same risk profile that motivated building Willow-shaped rather than Willow-dependent. Stronger revocation semantics, higher implementation and stability risk.

### X.5.4 The deferred decision and its criteria

Drystone does not choose between Track A and Track B in this draft. The choice is made in the richer-access-control phase, on these criteria:

- If expiry-bounded revocation (Track A, R4) proves operationally adequate for the cooperative's actual expulsion cadence, prefer Track A for its simplicity and clean Willow fit.

- If governance requires near-immediate revocation (an expelled bad actor must lose write acceptance in seconds, not an expiry interval), and Keyhive-style convergent capabilities have stabilized enough to depend on, prefer Track B.

- Both tracks satisfy R1, R2, R3, and R6 identically, so the state-reset-avoidance guarantee (X.6) does not depend on the choice. Only the revocation-immediacy guarantee does.

> `ENABLING:` The capability-token / membership-graph wire format is specified in §Z once this decision is made. It is not required for the prior-art record, which covers only the data model, the governance log, and the resolution order. This section can be published with the capability mechanism marked UNCOMMITTED.

---

## X.6 What Drystone guarantees, and what it does not

**Guarantees (both tracks):**

- Every honest vault computes the same effective authority state from the same set of governance facts (deterministic convergence).

- Authority state never moves backward. Stale vaults under-authorize, never mis-authorize (no state reset).

- Exactly one side of any exclusive conflict survives, deterministically, without a manipulable clock.

- Every superseded or stale-accepted decision remains in the log, attributable (`P-Knowable-Truth`).

**Does not guarantee (both tracks):**

- Instant revocation. Exposure is bounded (X.5.1 R4), not zero. The size of the bound depends on the track.

- Retroactive confidentiality. Past reads by a since-expelled member are not unmade (X.5.2, X.5.3).

- Prevention of stale-authority writes, only their detection and attribution (R6, defended in X.7).

These non-guarantees are the honest price of the architecture. They are smaller than Matrix's, because none of them allows an attacker to capture a scope or silently revert a decision, which are the two failure modes that earned Matrix a CVE and the room v12 emergency release. Drystone trades instant revocation for the elimination of state resets, on both tracks. That is the trade, stated so a reader can reject it if they disagree.

---

## X.7 Attack-resistance of attributable acceptance (R6)

### X.7.1 The claim under attack

R6 requires that a vault accepting a write under a capability records the causal frontier of governance facts it had synced at acceptance, so that a later-synced revocation makes a stale acceptance detectable and attributable. The guarantee is detection and attribution, never prevention.

The whole value of R6 is the clean contrast with Matrix. Matrix's state reset silently reverts a change and cannot tell you why; Drystone never reverts but always tells you who acted on stale authority. If R6 is forgeable, that contrast collapses and Drystone inherits a quieter version of the same unattributability that made the Matrix reset a CVE. So this is load-bearing, not a detail.

### X.7.2 The attack

A vault V accepts a write from holder H under a capability that has been revoked by a governance fact R. V wants to later claim it had not yet synced R at acceptance time, so the acceptance looks honest rather than negligent or collusive. The attack is a lie about V's own knowledge state at the moment of acceptance.

Three variants:

- **A1 — Backdating.** V claims an earlier frontier than it actually held, to exclude R.

- **A2 — Frontier omission.** V records a frontier that omits R while including R's causal neighbors, hoping the gap is unnoticed.

- **A3 — Equivocation.** V presents one frontier to H and the network at acceptance, and a different frontier later when challenged.

### X.7.3 Why Willow's native entry does not defend this

Stated plainly so the spec does not over-claim: Willow's entry carries a plaintext, unenforceable timestamp. An acceptance record that relied on that timestamp to establish when V accepted would be defeated by A1 trivially, because nothing binds that integer to reality and Willow itself disclaims any meaning for it.

The defense therefore cannot use Willow's time dimension at all. It must be built in the authorisation token and the governance-log structure, over which signatures are taken. Willow's `is_authorised_write(entry, token)` hook is the sound part to build on: a token that signs over the entry's content digest binds what was accepted. The frontier (what V had seen) must be constructed and signed over separately.

### X.7.4 The defense

The acceptance record is a governance fact (X.3.1), signed by V, whose signed body includes:

1. **The accepted entry's content digest** (BLAKE3), binding what was accepted. This uses Willow's `is_authorised_write` hook honestly: the token signs over the entry, which is sound.

2. **A frontier commitment**: the set of governance-fact digests V claims as its synced frontier, represented as a commitment over those digests (a Merkle root over the sorted digest set; exact construction `ENABLING`). V signs this commitment as part of the acceptance body, in the manner of Sigstore countersigning, where the signature is taken over prior signed state to bind it to the event rather than relying on a mutable external timestamp.

3. **V's own prior acceptance-record digest**, chaining V's acceptances into a per-vault hash chain.

Against the three variants:

**Against A2 (omission).** The frontier commitment is a single hash over the whole claimed set. V cannot later expand what it "meant" by the set, because the commitment pins it. Either R's digest is provably in the committed set (V admits it had R, so the acceptance was knowingly stale: full attribution) or it provably is not (V committed to a frontier lacking R). The omission is not hideable inside the commitment; it is the commitment.

**Against A1 (backdating), the hard one, and the honest bound.** A frontier commitment proves what set V committed to, not when. A vault with no honest clock can still claim its frontier-lacking-R commitment reflects its real knowledge at a real-but-unspecified time. Drystone cannot defeat pure backdating with cryptography alone, for the same reason Willow's timestamp is meaningless: there is no trustworthy internal clock. The defense is causal, not temporal, and it is bounded:

- If R is in the causal history of any fact V's frontier commitment includes, then V provably had a path to R and the "didn't have it" claim is refuted. This catches every backdating attempt where V's claimed frontier transitively references R.

- The residual escape is V committing to a frontier that is genuinely causally independent of R, that is, V and R's issuer were concurrent and unaware of each other. This is not a lie Drystone needs to defeat, because it is true: V really was in a concurrent partition. R6's guarantee is correctly stated as "detectable when V had causal access to R," and concurrent-partition acceptance is the legitimate stale-authority case that R4's exposure bound exists to cover. The two mechanisms compose: R6 attributes the cases V could have known, R4 time-bounds the cases V genuinely could not.

**Against A3 (equivocation).** The per-vault acceptance hash chain (item 3) makes equivocation detectable: V cannot present two different frontier histories without forking its own chain, and the fork is itself evidence. Two signed chain heads from V with the same predecessor are non-repudiable proof of equivocation.

### X.7.5 What this honestly does and does not buy

**Defeats:** frontier omission (A2) and equivocation (A3) cryptographically; backdating (A1) wherever V's frontier had causal access to the revocation.

**Does not defeat:** acceptance during a genuine concurrent partition, because that is not misbehavior. It is the real stale-authority case, attributed honestly as "concurrent, not negligent," and bounded by R4 rather than by R6.

The result is the defensible version of the Matrix contrast. Drystone does not claim to prevent stale-authority writes (no eventually-consistent system can). It claims that every stale acceptance resolves into exactly one of two categories: knowingly stale (V had causal access to R: full attribution), or concurrently stale (V did not: no fault, R4-bounded). There is no third category where a vault silently escapes both prevention and attribution. That third category is precisely where Matrix's state reset lives, and closing it is the point.

> `ENABLING:` The frontier-commitment construction (Merkle root over sorted governance-fact digests), the acceptance-record wire format, and the per-vault chain linkage must be byte-specified before interoperation. These are additive to the X.3.2 encoding and share its BLAKE3 dependency.

---

## X.8 Termination and convergence of the resolution fold

### X.8.1 The regress

X.3.2 step 1 orders conflicting facts by "issuer authority rank at the causal frontier." Authority rank is set by governance facts. Those facts can conflict, and resolving their conflict invokes X.3.2 step 1 again, which again asks for authority rank. Naively, resolving authority requires already-resolved authority. The fold must be shown to terminate, and to converge to the same result on every honest vault regardless of sync order.

This is not hypothetical. It is the exact problem Matrix State Resolution v2 exists to solve, and the worked case is canonical: Alice grants Bob power (fact A), Bob grants Charlie power (fact B), Charlie changes the ban level (fact C); on a fork where A is known but B is not, Charlie never received power and C is unauthorized. Whether C stands depends on resolving A and B first. Drystone has the same dependency and must break it the same way in principle, with two deliberate departures.

### X.8.2 The break: separate the ordering spine from the authority judgment

The regress only exists if computing the order requires evaluating authority. Drystone breaks it by computing the order from structure alone, then evaluating authority in a single forward pass along that fixed order. Authority is never consulted to produce the order; it is only checked against the partial state the order has already built.

#### X.8.2.1 The ordering spine is causal, not authoritative

Every governance fact references its causal predecessors (the frontier it was issued against, X.7.4). This forms a DAG. The resolution order is a topological sort of the conflicting facts over this causal DAG, computed by Kahn's algorithm. A fact's predecessors always sort before it. This requires no authority judgment whatsoever; it is pure graph structure over BLAKE3 references, and it terminates because the causal DAG is finite and acyclic (a fact cannot reference its own digest, since the digest is computed over the references).

This is the deliberate divergence from Matrix's reverse-topological-power ordering. Matrix folds sender power level into the comparator, which is what produced the apparent-cycle objection raised in MSC1441 review (three power-level events where the power comparison and the auth-chain comparison disagree). Drystone keeps power out of the spine entirely. The spine is causal-topological only.

#### X.8.2.2 The tiebreak is cryptographic, not temporal and not authoritative

Topological sort is not unique; ties must break identically on all vaults. Matrix broke ties with sender power then timestamp, and the timestamp choice drew the "how do we check a server isn't lying about time" objection in the same review, the same flaw X.3.2 already rejects. Drystone breaks ties solely by BLAKE3 digest of the canonical fact encoding. No power, no clock. Deterministic on every vault, ungameable without a hash preimage.

#### X.8.2.3 The forward pass checks authority against partial state

Having fixed the order, Drystone folds left, starting from the scope's founding fact (X.3.3, the grounded base case). For each fact in order, it is admitted if and only if it is authorized by the authority state accumulated so far, exactly as Matrix's iterative auth checks admit or reject each event against the partial resolved state rather than the final state.

A fact granting authority that was itself never admitted cannot authorize a later fact. This is how the Alice/Bob/Charlie case resolves without circularity: B is checked against the state after A, and C against the state after B, each as-of its predecessors, never as-of the contested outcome.

### X.8.3 Why it terminates

The fold is a single left pass over a finite, topologically sorted list. Each fact is visited once, and its authorization is checked against an accumulator that only grows monotonically (authority state is the X.3.1 fold, which never moves backward, X.6). No fact is revisited; there is no fixpoint iteration.

Termination is the termination of Kahn's algorithm (linear in facts plus causal edges) followed by one linear pass. The base case (founding fact) is unconflictable by X.3.3, so the accumulator is never empty at the first authority check.

### X.8.4 Why it converges

Two honest vaults with the same set of governance facts compute the same result because:

- The causal DAG is identical (facts carry their own predecessor references, integrity-protected by BLAKE3).

- The topological sort is made unique by the digest tiebreak.

- The forward pass is a deterministic function of the ordered list and the fixed base case.

No input to any step depends on sync order, wall-clock, or vault-local state. Two vaults with different fact sets compute results that differ only by the facts one lacks, and by X.6 the lagging vault under-authorizes rather than diverging, so convergence is monotone as facts propagate.

### X.8.5 The residual subtlety, stated honestly

There is one case worth naming rather than hiding. Matrix needs its "auth difference" (pulling in auth-chain events that appear in only some forks) because an authority-granting event might exist on one fork's history but not another's, and resolution must consider it even if it is not itself in conflict. This is the Alice/Bob/Charlie case from the other side: A is known on one fork so Bob has power, but B was not received on the other fork, so Charlie never received power and was not authorised.

Drystone's X.7.4 frontier commitments make this tractable but not free. A fact's referenced frontier names the authority-granting facts it relied on, so the forward pass can detect when a relied-upon grant is absent and refuse the dependent fact. But this requires that the resolution input set be closed under "facts named in any included fact's frontier," the Drystone analogue of Matrix's auth difference. The fold must first compute this closure, then sort, then pass.

> `ENABLING:` The closure computation (gather all facts transitively named in the frontier commitments of the conflicted set before sorting) must be specified precisely, because an implementation that sorts without first closing the set can admit a fact whose authorizing grant it failed to include, producing divergence. This is the single most likely place for two implementations to disagree, and therefore the place the spec must be most exact. It is the direct analogue of the bug class Matrix spent room versions 2 through 6 hardening.

### X.8.6 What this subsection establishes

The regress is broken: ordering is computed from causal structure with a cryptographic tiebreak, requiring no authority judgment, so authority is never needed to order the facts that determine authority. The fold terminates (single linear pass over a finite acyclic structure) and converges (every step is a deterministic function of order-independent inputs grounded in an unconflictable base case). The one hard requirement for interoperation is frontier-closure before sorting, flagged `ENABLING` as the highest-risk divergence point.

---

## X.9 Open items (for the next pass)

These are known-incomplete and tracked here so they are not mistaken for settled:

- **Frontier-closure specification (X.8.5).** Highest-risk divergence point. Must be byte-exact before two implementations interoperate.

- **Canonical fact encoding (X.3.2).** Gates the prior-art DOI. All other `ENABLING` encodings are additive to this one.

- **Root-authority succession (X.3.3, deferred to §Y).** A permanently fixed root contradicts the cooperative model; the transfer mechanism is the most dangerous operation in the system and is not yet drafted.

- **Capability-mechanism decision (X.5.4).** Track A versus Track B, deferred to the richer-access-control phase, decided on the revocation-immediacy criterion.

- **Frontier-commitment construction (X.7.4).** Merkle root over sorted digests; the per-vault acceptance chain linkage needs byte-level definition.

---

## Appendix X-A. Reference map

The Matrix mechanisms cited above, for the reader who wants to verify the comparisons against primary sources:

- **State Resolution v2** — the reverse-topological-power ordering, mainline ordering, and iterative auth checks. Primary: MSC1441 ("State Resolution: Reloaded"); the room v2 specification; the independent Karlsruhe analysis (Jacob et al., SACMAT 2020).

- **CVE-2025-49090** — the state-reset vulnerability in State Resolution before v2.1, addressed by room version 12 (Project Hydra).

- **MSC4289** — formal privileging of room creators with infinite, immutable power level; the source of the "unconflictable root" lesson in X.3.3.

- **MSC1441 review** — the apparent-cycle objection to power-in-the-comparator, and the "lying about time" objection to timestamp tiebreaks; the source of the two subtractions in X.8.2.

- **Willow Data Model** — namespace / subspace / path structure; `is_authorised_write`; the plaintext, unenforceable Timestamp that X.7.3 declines to rely on.

- **Meadowcap** — unforgeable delegated capabilities with attenuation by subsetting and no native revocation; Track A.

- **Keyhive** — convergent capabilities and membership graphs with first-class removal and re-encryption; Track B.

- **Sigstore countersigning** — signing over prior signed state rather than a mutable timestamp; the pattern X.7.4 adopts for the frontier commitment.
