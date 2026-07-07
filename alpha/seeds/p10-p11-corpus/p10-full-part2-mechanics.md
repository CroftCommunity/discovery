# Drystone, Part 2: The Certifiable Design

`Status: beta. Build-against shape complete; byte-level [gates-release] encodings open (Appendix B)`

`Resolution: library, the complete reference and the coherence authority for the Drystone set. For shorter tellings see the coffee-shop overview, then the elevator pitch.`

This part is the "what." It specifies the mechanics an implementation is built and validated against.
Each section names the Part 1 principle(s) it `Realizes`. Normative keywords are **MUST / SHOULD / MAY**
(BCP 14). Each normative claim carries one status flag from the canonical ladder (recorded in
`p10-drystone-conventions-and-decisions.md`, the single source both parts defer to): `Verified` (real
crypto or transport, demonstrated against a running reference implementation), `Verified-RFC` (verified
against a normative primary, e.g. RFC 9420 / RFC 9750), `Modeled` (a reference model exists and is
reasoning-complete but is not yet backed by real crypto), `Measured` (an empirical or benchmark
measurement), `Established` (an established literature result or an inherited primitive), `Design`
(specified, unproven), `Synthesis` (a claim assembled across sources rather than resting on a single
citation), `Load-bearing, unearned` (a property the design leans on that is not yet earned),
`[gates-release]` (a byte-level encoding or spec item that must be pinned before a publication-final
release and before two implementations interoperate), and `[confirm]` (rests on an external fact not yet
independently verified).

> **A note on two maturity layers inside this part.** The message/history/transport layer (§4, §6) is
> matured from a reference implementation that is **Verified** on real crypto and real iroh transport.
> Within §6, the MLS-architecture claims (the two identity planes, the two-layer encryption stack, the
> DS/AS trust model, the metadata exposure) are **verified against the RFC 9420 / RFC 9750 primaries**, and
> the iroh transport-plane claims are **verified against iroh 1.0** (wire-and-API-stable as of June 15, 2026:
> public-key identity, direct-first-with-blind-relay-fallback, post-handshake authentication). The items
> still left **[confirm]** are confined to the layers iroh's 1.0 guarantee does **not** cover, the
> separately-versioned `iroh-gossip` crate (pre-1.0: its event surface and overlay tuning) and the
> address-lookup crates (republish/expiry specifics, and the Pkarr-spec record model). The
> governance-conflict-resolution layer (§7.3–§7.5) is matured from a **Design**-stage draft whose
> reasoning is complete but whose wire encodings and one comparative dependency are not yet pinned. The §4
> and §7 layers also differ today in hash function (§4 proven on SHA-256; §7 designed on BLAKE3), a real
> reconciliation item, surfaced in Appendix B rather than papered over.

> **Naming: center-free, not serverless.** Drystone is a **center-free peer protocol**. *Peer-to-peer*
> describes its transport (§6) accurately and we use it there. We do **not** call it *serverless*;
> that term now names managed ephemeral compute, nearly the inverse of what is meant. The load-bearing
> property is not topology but **where adjudication lives** (§3.1): no node holds privileged or canonical
> authority. Where precision is needed we say *center-free* or *no node holds privileged or canonical
> state*; where we mean the wiring we say *peer-to-peer*.

> **Vendor-neutral naming.** Drystone is the protocol. The reference implementation this part is matured
> from carried the historical brand "Croft" in some signed wire constants (e.g. the domain-separation tag
> namespace `croft-*`). Those values are shown where they are what was proven, but they are **the
> reference profile, not the protocol**: Drystone requires a versioned, domain-separated tag, and does not
> mandate that string. Defining the vendor-neutral `drystone-*` tag namespace and re-proving the rename
> (the tag is signed over, so changing it re-opens the signature proofs) is an Appendix B item.

---

## 0. Map

`A per-section index with scope, dependencies, and orthogonality, maintained as sections change. Section numbers run per part, and cross-part references name the part: Part 1 holds §1 through §3 (positioning, the principles, and their corroboration), and Part 2 holds §3 onward (the mechanics), so Part 1 §3 and this Part 2 §3 are distinct. Each entry states what the section covers and, where it matters, what it depends on and what it is orthogonal to, so a reader or a later fold can pull the relevant section without dragging in the unrelated ones.`

- **§3. Protocol Overview.** The running example the rest of the spec demonstrates on (§3.0, the fixed cast and its resources, additive to the normative text), and the system-of-peers diagnostic (§3.1) that orders the whole spec: the test for whether a design is genuinely center-free. Frames everything below.

- **§4. Data Model.** The representation layer: cryptographic foundation (§4.1), identifiers and derivations (§4.2), the signed message as the unit of history (§4.3), integrity-and-ordering versus authorship-and-standing as two distinct guarantees (§4.4), and the multi-client fold, where client-count and device-count are not persona-count (§4.5), and canonical-on-the-wire encoding with free local storage (§4.6). *Depends on:* nothing internal; it is the base. *Orthogonal to:* the governance semantics of §7 (this is representation, §7 is resolution).

- **§5. Identity, Principals, Rights, Roles, and Capabilities.** Who acts and with what standing: the two equalities and two inequalities (§5.0), the only-canonical-state-is-local claim (§5.1), the principal/client/persona model (§5.2), the inherent rights floor of tenure/voice/exit (§5.3), resources versus grants (§5.4), Group Roles and capabilities across the governance and data-access planes (§5.5), weight anchored to personhood (§5.6), membership/standing/revocation (§5.7, §5.8), exitability (§5.9), and the Group as a principal (§5.10), and read-scoped content keys that cryptographically enforce the read role (§5.11). *Depends on:* §4 (identifiers). *Orthogonal to:* transport §6 (standing does not depend on how bytes travel). §5.9 exitability is the backstop §7.6 relies on.

- **§6. Transport and Delivery: the Three Planes.** How bytes move and who can see what: the two identity planes (§6.1), the two-layer encryption stack (§6.2), the Delivery Fabric versus the Group (§6.3), the metadata floor (§6.4), the three planes Carriage/Durability/Presence (§6.5 through §6.7), gap-aware history convergence and the content-blind history store (§6.8), discovery and interaction tiers (§6.9), the gossip overlay realization (§6.10), the two deployment modes (§6.11), and real-time media (§6.12). *Depends on:* §4 (the message), §5 (who is a member). *Orthogonal to:* the governance-conflict resolution of §7 (delivery is mechanism-neutral about what it carries). §6.8 gap-aware convergence is the completeness machinery the §7.3 beam leans on.

- **§7. Synchronization and Governance-Conflict Resolution.** The governance machinery, the largest section and internally the most separable.

  - **§7.1, §7.2.** Data-model commitment, then the mechanism-neutral, normative grant-and-revocation interface.

  - **§7.3. Governance facts are entries, not mutations** (the core). §7.3.1 the total order (causal and cryptographic, never temporal); §7.3.2 what conflicts; §7.3.3 the declarative snapshot as cache and the read/enforce line; §7.3.4 sign-the-state-not-the-authorship; §7.3.5 the membership ceiling; §7.3.6 decision versus enactment; §7.3.7 the now; §7.3.8 the finality gate; §7.3.9 principal recovery and break-glass (a TBD stub). *Depends on:* §5 (standing), §4 (facts). The whole §7.3 run rests on the completeness beam (Appendix B).

  - **§7.4.** Freshness, no false current; §7.4.1 the governed false-positive tolerance; §7.4.2 the two MLS recovery hazards the corroboration model dissolves; §7.4.3 the governance-generation stamp that lets a data-plane entry self-locate against the authority chain.

  - **§7.5.** Attributable acceptance and the regress-free fold; §7.5.2 breaks the authority-ordering regress and closes the resolution input set.

  - **§7.6. The reconcile hard-stop and re-formation fork** (conflict handling). §7.6.1 the two-member escalation set; §7.6.2 one primitive, three arities; §7.6.3 MLS subordinate; §7.6.4 a ban is a fork; §7.6.5 the three registers (mute, governance, fork); §7.6.6 fork placement; §7.6.7 the epoch roll is the audience split; §7.6.8 permanent-both and cheap merge; §7.6.9 posture dials and the protocol/product division; 7.6.10 re-composition as a view and the adjudication rubric. *Depends on:* §7.3 (the order and the ceiling), §5.9 (exitability). *Orthogonal to:* §7.9 scaling.

  - **§7.7, §7.8.** Dataplane history modes (forward-only versus Willow-mutable) and side histories.

  - **§7.9. Scaling: commits, folds, and the relocation of ordering cost.** *Depends on:* §7.3.1 and §7.3.2 (order-independence) and the Appendix B beam; §7.7 and §7.8 (catch-up). *Orthogonal to:* §7.6 conflict-handling; a scaling change rarely touches conflict semantics.

- **§8. Security Considerations.** Forward secrecy and durable history are not in tension (§8.1); the honesty boundaries the spec still carries (§8.2). *Depends on:* §6 (the stack), §7 (the governance model).

- **§9. Interoperability and Conformance.** What two implementations must share to interoperate.

- **§10. Substrate Requirements and Reference Realizations.** Requirement-first mapping onto the messaging backplane (§10.2), transport and overlay (§10.3), and primitives (§10.4), closed by the dependency-versus-realization ledger (§10.5). *Depends on:* §4 through §7 (it states what each layer requires, then names one conforming realization).

- **Appendix A. Alternatives Considered.** A.1 decentralized MLS and the forward-secrecy cost of center-free ordering.

- **Appendix B. Open Questions.** The tracked residuals. Leads with the single `Load-bearing, unearned` beam (completeness-ahead), then the governance `[confirm]` presets (enactment dial, the now's schema, tiebreak/weighting defaults), then the naming, hash, wire-encoding, substrate, and security items. Referenced by §7.3.3, §7.3.7, §7.3.8, and §7.9.

- **Appendix C. Consolidated Prior Art and Positioning.** C.1 the data layer, C.2 the governance/resolution layer (same skeleton, opposite spine), C.3 the cryptographic group-state layer, C.4 the governance-as-protocol frontier, C.5 the cross-disciplinary grounding.

- **Appendix D. Term lattice and invariants** (the vocabulary of record): entities and genus, the hosting chain, MLS-carried terms, lineage and the count, the four properties and the non-property, the bundle/relation/participant senses, the invariants of record, and the delivery-plane vocabulary.

- **Appendix E. The running example, chained.** The §3.0 cast walked through the Group's whole life as one arc (E1 formation to E10 re-composition), each beat listing the sections it demonstrates and linked to the in-body beat there. Additive, no new normative content.

---

## 3. Protocol Overview

A **persona** holds a local store that is canonical for it (`P-Local-Truth`). Peers participate in
**Groups**: entitlement-and-governance units holding shared state, each realized over an MLS group (§6,
§10.2). Within a Group, two kinds of state move:

- **History**: the content personae author, as signed, hash-chained entries. This is the data plane.

- **Governance facts**: signed, append-only entries recording who may do what (admit, expel, grant,
  revoke, amend). Authority is a deterministic fold over these. This is the control plane.

A Group's **scope** is wider than its membership: it is the extent of exposure and processing for the
Group's messaging, the reach of routing and metadata beyond the entitled members. A node that carries or
triggers on a Group's traffic without holding its keys (a relay, a store-and-forward meer, a push-notify
node) sits in the Group's scope but is not a member of the Group. Governance and entitlement facts attach
to the **Group**; exposure and routing-reach facts attach to the **scope**. (The two terms, and the
lowercase-**group** social sense beneath the capital-G Group, are fixed in §5.0 and §5.2.)

A persona is one or more **devices** (keypairs) acting under a single **lineage**; receivers fold devices
back to one actor so that membership and thresholds are counted by actor, not by device. Peers reach each
other over an encrypted QUIC transport with relay fallback; a relay forwards opaque frames and need not
read content. When two peers' views diverge, **range-based reconciliation** finds the difference and
exchanges the missing entries; **history** converges by hash-chain replay, and **governance** converges
by a timestamp-free causal-and-cryptographic resolution order. Genuine membership contradictions are
**not** auto-merged; they hard-stop and escalate to humans, and a minority's sanctioned exit is a clean,
attributable **re-formation fork**.

The sections below specify each piece. §4 is the data model; §5 is identity, rights, and capabilities;
§6 is transport; §7 is synchronization and governance-conflict resolution; §8 is security; §9 is
interoperability and conformance; §10 consolidates the substrate requirements and their reference
realizations (MLS, iroh, and the primitives), separating each *requirement* from its current
*realization*. Appendix C consolidates the prior art each layer builds on.

### 3.0. The running example: a cast the rest of the spec demonstrates on

The sections below carry their definitions and their normative clauses in full, and those stand on their
own. Laid over them is a running example, a small fixed cast that recurs so each mechanism can be shown in
place, and especially so each **MUST** and **MUST NOT** can be seen biting on a named actor rather than on
"a node." The cast is additive: a beat never carries an obligation in a clause's stead, it shows the clause
landing. The scattered beats chain into one continuous arc in **Appendix E**; a reader who wants the whole
journey at once should read that. The cast and its resources are fixed, and no beat introduces an actor or
resource not named here, so the arc stays coherent.

**Personae and their clients.**

- **Alice**, a persona, with two clients, a **phone** and a **laptop**, both under one key lineage. She holds
  a **moderator Group Role Set**. She is the worked case for one-persona-many-clients, for a Group Role Set
  layered over the flat rights floor, and for concentrated-but-revocable authority.

- **Bob**, a persona, with one client, a **phone**. He is the worked case for the bare-node floor, and he is
  the member later banned, who continues in his own lineage (a ban is a forced fork).

- **Dave**, a persona, with two clients, a **phone** and a **tablet**, his phone offline for a stretch. He is
  the worked case for stale-but-honest convergence, for mobile wake, and for a fork bystander who lands where
  his own choice puts him.

- **Erin**, a persona with one client, a **phone**, admitted to the Group alongside Dave. She is the worked
  case for equal branches in individual outcome: at the fork she lands with Bob's lineage where Dave lands
  with the Group, two opposite yet equally legitimate choices the substrate does not make for them; and at
  the later re-composition, having been banned in the meantime by Bob's lineage, she stays excluded by the
  floor while Bob is re-admitted by an explicit governed act.

**The helper.**

- **Carol's node**, an always-on node admitted **by delegation** as a durability-and-search helper. It holds
  clear text, so it can replay for an offline member and answer queries, yet it holds **no standing**: it
  cannot foreclose, remove, or govern, and it is revocable exactly as it was admitted. It is the worked case
  for capability-is-not-authority (§5.4, §5.5, and Part 1 §2.7). Carol is a persona whose device is that
  node, kept parallel to the in-Group personae; the only difference is that her node was invited to serve,
  not to govern.

**Resources.**

- **The Group**: Alice, Bob, and shortly Dave and Erin, with its **append-only governance log**, the fact
  set the fold runs over.

- **The delivery fabric**: the nodes that carry frames, a superset of the Group in which Carol's node sits.

- **A k-of-n threshold** the Group sets for its membership and revocation decisions.

- **Alice's moderator Group Role Set**, which makes `floor + [roles] + [capabilities] + [resources]`
  concrete against Bob, who holds the floor alone and counts for exactly the same one unit of weight.

That is the whole cast. Each section from here reuses it, and Appendix E chains the reuse into the Group's
life, from formation to fork.

### 3.1. What makes this a system *of peers*: the diagnostic that orders the spec

A network of many nodes with any-to-any connectivity is **not** automatically a distributed system of
peers. The distinction this specification turns on is not topological; it is about **where adjudication
lives.** A **principal is a locus that can *adjudicate***: it holds genuine authority over some domain that
other principals must respect, not merely a node that can sense, store, and relay. A node that only senses and
relays, with its decisions made elsewhere, is a **sensor**, however well-connected it is. To say a system
is one *of peers* is to say its principals stand in **peer relation**: each is a locus the others must
respect, none a center the others must obey.

This is also exactly why the protocol is named *center-free* rather than *peer-to-peer*: peer-to-peer is
a true statement about the wiring and a misleading one about the authority. A blockchain is peer-to-peer
and builds a single canonical chain; that global apex is precisely what a center-free design refuses. The
word that names the property is not in the topology.

The cautionary archetype is a national economic-planning network of the 1970s with tens of thousands of
local terminals and any-terminal-to-any-terminal wiring, whose adjudication nonetheless all funneled to a
single apex: thousands of nodes, **zero peers in the sense that matters**. Its terminals sensed, reported,
and received instructions; none adjudicated. **The wires lie; the authority topology tells the truth.**
The diagnostic that follows is mechanical: **count the adjudication loci.** If the answer is one (or a
small apex), the system is a centralized decision apparatus wearing a distributed sensor mesh as a costume,
regardless of how its packets flow.

This is why Drystone specifies **rights separately from resources** (§5) and cannot let either be read
off a network diagram: a **resource** ("this device can do X", sense, store, relay, compute) is visible in
the plumbing, but a **right** ("this principal's authority over X must be respected") is visible only in the
governance. It is also why the spec is **ordered** the way it is, define the **principal** (a locus of
adjudication, §5.2) and the **persona** as its human-representing kind, then the rights floor (§5.3), then the mechanics that keep adjudication distributed (§7,
§8), and why the **label-not-enforce** posture (§8) is load-bearing rather than cosmetic: enforcement
relocates adjudication to whoever enforces, quietly converting principals into sensors, while labeling leaves
adjudication with the principal and propagates only information. Each enforcement hook looks locally reasonable,
which is exactly how a network of peers can **degrade into a sensor mesh** over time without anyone deciding to
centralize. Keeping adjudication at the edge is the protocol's job; surfacing the hard case to a human
(the algedonic escalation, §7.6) is how it does so without pretending rules can resolve everything.

---

## 4. Data Model

> `Realizes: P-Local-Truth, P-Knowable-Truth`

### 4.1. Cryptographic foundation

A device holds a signature keypair; its public half is its verifying identity, its secret half signs. A
**lineage identifier** names a logical actor; multiple devices may act under one lineage (§5.2). A
signature **MUST** verify against the author's published verifying key before any other check, because any
check run on an unverified message would be trusting unauthenticated input, and a valid signature is
**necessary but not sufficient**: standing (§5, §7) is also required.

*Running example: when Bob's laptop receives Alice's post, it verifies her signature against her published
key before any other check, because acting first would trust bytes whose author is not yet established
(walkthrough beat E2).*

The reference implementation uses **Ed25519** (RFC 8032, deterministic, 64-byte signatures) and
**SHA-256** (32-byte digests). *Verified* (real Ed25519 over live iroh: a forged message is rejected
as a bad signature on every receiver, including a NAT'd peer). A production profile **MAY** select a
different suite, but the suite is part of the versioned wire profile and **MUST NOT** be silently
negotiated down, because a silent downgrade is a downgrade attack: an adversary that could force a weaker
suite unnoticed would undo the profile's security guarantees. *(The abstract signature and hash requirements, and why Ed25519 / the SHA-256-vs-BLAKE3
choice are the references, are consolidated in **§10.4**.)*

### 4.2. Identifiers and derivations

Wire identifiers are a hash over a **tagged** pre-image: `tag = version ‖ domain-separator`, so one
identifier kind can never collide with another's input. An implementation **MUST** derive these
identically; they are the interop anchor.

| identifier | pre-image (structure) |
|---|---|
| lineage genesis | `H(tag["lineage-genesis"] ‖ lineage_id)` |
| group genesis | `H(tag["group-genesis"] ‖ group_id)` |
| group gossip topic | `H(tag["group-topic"] ‖ group_id)` |
| content id | `H(canonical{group_id, regime, author_id, content})` |

*Verified*: the three tagged wire derivations are conformance-tested canonical functions, byte-identical
to the proving spike. The reference profile's tag strings are `croft-lineage-genesis:`,
`croft-group-genesis:`, `croft-group-topic:`; **Drystone normatively requires a versioned, domain-separated
tag and does not mandate these strings** (Appendix B, naming reconciliation).

> **What the gossip topic *is*, shown by its consequences: it is a major determinant of scope, and its
> mapping to Groups is a delivery-fabric choice.** The `group gossip topic` row above shows the reference
> profile deriving one gossip topic per Group from that Group's ID. That is a reasonable default and what the
> testing implementation does, but it is a **realization choice, not the model**, and the clearest way to see
> why is to hold the choice as a variable and read off the outcomes. First, a distinction to keep sharp: the
> gossip topic is **not synonymous with scope**. Scope is the whole envelope of exposure and processing, the
> Delivery Fabric as a whole (§6.3, where the Delivery Fabric is defined; and §5.4): gossip, direct
> connections, relays, meers, and push-notify nodes, and the metadata each sees. The gossip topic is **one
> large contributor** to that envelope, because it governs who is in the dissemination swarm and therefore
> who sees the sealed traffic and its routing metadata, but a relay in the path, a meer holding sealed bytes,
> or a push-notify node each adds to scope
> independently of the topic. With that held, the topic seed is a major lever on *how wide* the scope is:
>
> - **Seed the topic from a single Group's ID (the reference default).** The swarm is that Group's members
>   (plus whatever helpers are in the swarm). Scope, on the gossip axis, tracks the Group one-to-one. Metadata
>   exposure over gossip is confined to that Group's delivery, and a member of another Group cannot compute or
>   join this topic. This is the tight default, and it is why it reads, at first, as though "scope is
>   Group-specific."
>
> - **Seed the topic from a value shared across several Groups.** Those Groups now share one dissemination
>   swarm: a node in the swarm sees the sealed traffic of all of them (still ciphertext, but the routing
>   metadata and membership of the shared topic are common). Scope is now *wider* than any one Group, which
>   is exactly the case that shows scope and Group are different objects. A deployment might choose this to
>   pool delivery infrastructure across related Groups, accepting the wider metadata envelope as the cost.
>
> - **Include or exclude a given meer or relay from the topic (or add one to the path).** Adding a helper to
>   the swarm or the delivery path widens the scope to include that helper's exposure (it now sees the sealed
>   envelopes and their routing metadata as they pass); excluding it narrows the scope. Same Group, same
>   gossip topic, different scope, purely a fabric choice, which is the concrete proof that the topic alone
>   does not fix the scope.
>
> So the mapping of scope to Group is a **delivery-fabric decision**, not a fixed coupling, and the topic
> seed is one principal lever on it (alongside which helpers are in the path). What the model *requires* is
> only that the seed be high-entropy (below), so the topic is not guessable; the Group-ID derivation is one
> way to meet that, carried in the reference profile. Reading "the reference profile keys the topic from the
> Group ID" as "scope is Group-specific" is the realization-for-requirement slide §10 exists to prevent.

> **Note on the content-id pre-image.** The content-id pre-image carries **no `timestamp` field**: per
> Part 1 §2.0.1, a wall-clock value is an uncorroborable assertion and must not appear in any identity, ordering,
> or authority-bearing computation. A content id binds *what* and *where* (group, regime, author, content),
> not *when*. If an application wishes to record an authored "claimed time," it does so as ordinary payload
> content, an explicitly author-asserted value, never a protocol fact, and never an input to the content id.
> *(Design; the canonical content-id encoding is `[gates-release]`, Appendix B.)*

A scope's gossip-topic seed **MUST** be high-entropy / salted, not a guessable human handle, otherwise an
adversary computes the topic and joins or observes. *(Leak bound characterized; see §8.)*

### 4.3. The signed message: the unit of history

The history unit binds author, position, branch, and payload so a message cannot be replayed onto another
branch or position. The signed pre-image (reference profile):

```
signing_bytes = "msg-v1" ‖ branch(32) ‖ seq(LE u64) ‖ author_id_bytes ‖ 0x00 ‖ payload
```

A receiver **MUST** recompute the pre-image and verify the signature against the author's key. *Verified*,
the real message traveled live iroh-gossip and verified against a real backfill import; an honest
member's message is accepted, a forged one rejected. (Note: position is `seq`, a per-branch counter, not a
clock, ordering is structural, not temporal, consistent with Part 1 §2.0.1.)

*Running example: Alice's post is the unit of history, and before accepting it Bob's laptop recomputes the
pre-image from these signing fields and checks the signature against her key (walkthrough beat E2).*

### 4.4. Integrity-and-ordering vs authorship-and-standing: two distinct guarantees

These are kept strictly separate, and conflating them is the central honesty error the protocol forbids:

- **Integrity + ordering (structural).** A branch is a sequence chained by
  `hash = H(prev ‖ seq(LE) ‖ payload)`; a receiver **MUST** reject a branch with a broken chain or
  non-contiguous sequence numbers. This proves in-transit integrity and contiguous ordering, **not** who
  wrote it.

- **Authorship + standing (authority).** The signature (§4.3) plus standing (§5, §7). A receiver **MUST**
  apply both; **integrity alone MUST NOT be treated as authorization**, because a well-formed branch from a
non-member would otherwise be honored as if authorized.

*Verified*: a valid-chain branch from a non-member is accepted by the structural check but rejected by
the authority check as an unauthorized author. This separation is what makes a branch trustworthy: the
hash chain proves it was not tampered in transit; only signature + standing prove it may be there.

*Running example: a valid-chain frame reaching Bob from a non-member passes his structural check but fails
the standing check, so integrity alone does not authorize it (walkthrough beat E2).*

### 4.5. Multi-client fold: client-count and device-count ≠ persona-count

A **persona** (the human layer's manifestation, §5.2) is rooted in a **cryptographic key pair**. That key pair is the
root of a **cryptographic lineage**: each of the persona's devices and each **client** (group-member software,
§5.2) hosted on them carries a membership key that **descends from the persona's key pair by signed
credential**. Lineage here is literal, the concrete chain of signatures by which a client's membership key
is provably tied back to its rooting persona, not an abstract tier.

Receivers **MUST** fold by following each client's lineage back to its root: every client whose lineage
roots at the same persona counts as **that one persona**, however many clients run on however many devices. A
scope's topic carries many such lineages; the fold is what every participant computes identically to agree on the
member list and on **lineage-rooted thresholds** (§7.2), which count **one persona per rooting key pair**,
never clients or devices. *Verified*: one human's two devices fold to one persona; all participants agree on the
folded count.

*Running example: Alice's phone and laptop are two clients under one lineage that fold to one persona, so
she is one member counting for one unit of weight however many devices she runs (walkthrough beat E1).*

#### 4.5.1. Per-client authorship, per-client logical clock, and the principal-as-self-AS

Clients are **not** a shared key. Each **client** (§5.2: software on a device that is a member of a Group,
one MLS leaf, one signature key, one credential) is an independent signer with its own signature key, and
an assertion is **authored and signed per-client**: the envelope carries `author_client` (the signing
identity the signature verifies against), `author_device` (the device hosting it), and `author_principal`
(the principal the client is credentialed to). The client→principal binding is established by a
**credential**, not by key-sharing. This is the only model consistent with the underlying group-key layer,
which **represents each client as its own member** (the MLS member is the client/leaf, and a device may run
more than one) and which Drystone requires not to share one signature key across a principal's clients in a
group, so "these clients are the same person" is necessarily an **identity-layer credential policy**, not
a key-layer fact. This is the mechanism behind the governance-integrity spine (§5.2): because each client
is a distinct member but governance counts **principals**, a principal's clients and devices are resolved
by lineage to one persona before any quorum is counted, so more clients or devices never buy more weight.
*(Group-key/MLS facts: confirm against the primary specification, **[confirm]**. The
abstract group-key requirement this rests on, including the no-shared-signature-key-across-a-principal's-
clients property as requirement K5, is consolidated in **§10.2**, where MLS is positioned as the reference
realization rather than the requirement.)*

Because there is no central authority in a center-free system, the **principal key acts as its own
tiny certificate authority for its clients and devices**: the principal signs each client's credential, and
a verifier accepts "this client is principal P" by validating that credential chain. Adding a device or a
client is the deliberate, high-trust act (e.g. scanning a code) that the principal then attests, which is
exactly the trust binding of Part 1 §2.0 (the human act binds existing trust to a key; the credential
records it).

A direct consequence for ordering, and a load-bearing instance of Part 1 §2.0.1 (*time is not a fact*): the
**logical clock is per-client and strictly logical, never a wall-clock.** Each client advances only its own
`lamport` counter, so there are no cross-client collisions and no coordination is required; a wall-clock is
never consulted, because a wall-clock is an uncorroborable assertion and could not order what must
converge. A principal's stream is the **deterministic fold-time merge** across its clients' streams
(ordered by lamport, then a stable cryptographic tiebreak, never by time). `auth_assertions_by_client`
(the per-client causal index used for range-sync) keys on the client identity for exactly this reason.
*(Local-storage detail; see the implementation build spec.)*

### 4.6. Canonical on the wire, free in local storage

Three representation decisions follow from one principle, that agreement is on facts and their hashes and
nothing more. The bytes that are hashed, signed, and reconciled are fixed by the protocol; everything
downstream of a verified fact is a local choice. Getting the first half wrong fails silently, and the second
half is where the local-first advantage lives.

**Facts are encoded canonically, and any JSON rendering is only a lens.** A fact **MUST** be encoded in a
canonical, deterministic binary form (dag-cbor) for hashing and signing, and a JSON or other human-facing
rendering **MUST NOT** be the artifact that is hashed or signed, because hashing and signing require a single
reproducible byte sequence and JSON does not provide one: its map-key ordering is unspecified, whitespace is
free, number formatting varies, and string escaping and Unicode normalization differ across encoders, so the
same logical fact serializes differently, yields a different FactId, and breaks signature verification even
though its meaning never changed. That is cross-node divergence entering through a side door, the same error
class as ordering by wall-clock or deriving causality from a local index: a value that must be identical
across nodes is allowed to vary. dag-cbor closes it because it specifies map-key ordering, a single number
model, and a deterministic encoding, so a given fact has exactly one valid byte sequence and one FactId on
every node. `Established` (the determinism is a defined property of dag-cbor), and `Verified` by ecosystem
use (ATProto encodes its records in dag-cbor and IPLD uses it as a canonical codec). The split to hold onto
is that the canonical form is hashed, signed, stored, and reconciled, while the presentation form is computed
from it as a lens, never hashed or signed, and free to differ. `Design`, resting on that `Established`
determinism. The exact canonical content-id encoding is `[gates-release]` (§4.2).

**The causal reference is one committed reference on the wire, and the DAG is rebuilt locally.** A fact's
causal reference **MUST** be the observed governance frontier, a single reference that transitively commits
to the fact's entire causal past the way a Merkle root commits to a whole tree (§7.3.1, §7.5.2), and a bare
within-branch parent-pointer **MUST NOT** stand in for it, because a bare parent authenticates only
author-local order and discards cross-author causality, whether this author's fact was made in knowledge of
another's, which is exactly what the fold needs to order concurrents and what no local index can recover once
it was never attested. The per-branch sequence of §4.3 carries that author-local order and is additional to
this reference, never a substitute for it. The reference is deliberately a single committed hash rather than
an explicit multi-hash frontier listing every head a fact saw, which trades one property for another: the
committed form is smaller on the wire and preserves full causal fidelity but makes concurrency a *derived*
property, computed by resolving two facts' references against a local DAG rather than read off a fact
directly, while an explicit frontier is self-describing but larger. Local-first settles the trade toward the
committed form, because a node maintains a rebuildable, integrity-checkable DAG index (forward and backward
edges, a frontier index, reverse indexes) as a pure local acceleration, so it pays that index cost once and
thereafter answers walk-the-graph and what-is-concurrent-with-this as a cheap local lookup, while an explicit
frontier would pay off only when reasoning about concurrency without such an index, chiefly during initial
sync. `Design.`

**The reconciliation format is fixed; local storage and derived views are free.** Because agreement is on
facts and their hashes and not on any storage layout, the wire format and each node's local format are
decoupled, which a shared-state chain cannot do, since there every node reads one structure and the
representation must agree. The reconciliation format is fixed by the protocol, signed facts in canonical
dag-cbor carrying the committed causal reference above (§7.3.1), and it **MUST** agree across nodes because the fold
consumes it. Local storage is whatever each node chooses: a node **MAY** prune raw facts behind a verified
checkpoint (§6, §7.3.7), keep only the folded-now plus a tail, and store in any structure it likes, because a
dropped fact is re-requestable and re-verifiable against its signature, so storage depth is bounded only by
how much history the node wishes to serve. Local consumption is equally free: a node **MAY** build any number
of derived views (materialized tables, reverse indexes, the DAG index above) as integrity-checkable,
rebuildable caches, none authoritative on its own, since any one can be rebuilt from the chain and re-verified
if suspect. Three efficiencies are therefore three independent dials: the protocol fixes the wire, and each
node trades its own disk and CPU against query speed and against how much it is willing to serve, with
integrity guaranteed throughout because everything traces to signed facts. `Design.`

---

## 5. Identity, Principals, Rights, Roles, and Capabilities

> `Realizes: P-Peer-Equality, P-Local-Truth, P-Knowable-Truth, P-Durable-Enablement`

This section fixes the vocabulary the rest of the spec runs on. It is the section a reviewer presses
hardest, because it is where `P-Peer-Equality` is enforced by mechanism rather than assumed. The section
asks one precise question, *in what ways may one persona differ from another?*, and answers it with
**exactly four properties, two necessarily equal and two legitimately unequal**, then separates the
**identity layer** (principals and clients) from the **governance layer** (personae and weight).

> **Provenance of this model.** The equality framing and the principal/peer/client vocabulary below are
> Drystone's own synthesis (ours), not sourced from any external spec. Two prior-art vocabularies are
> adopted *as-is* rather than renamed, so Drystone does not fight the systems it builds on:
>
> - **MLS** terms, **client**, **member**, **leaf**, **Authentication Service (AS)**, **Delivery Service
>   (DS)**, keep their RFC 9420 / RFC 9750 meanings (§10.2).
>
> - **Meadowcap's "capability"** is kept verbatim for what it means there: an unforgeable token bestowing
>   **read or write access to data** in a namespace, issued by that data's owner and attenuating under
>   delegation (§5.5, §10.4). Drystone does **not** reuse "capability" for anything else.
>
> The device-facility layer is **resource** (§5.4), and the in-Group governance-authority layer is **Group
> Role** (§5.5). **Capability** is **not** one of the four peer-equality properties below: it is the
> **data-access mechanism a Group Role operates through** (a Group Role may carry the authority to issue
> capabilities), and it lives in the data plane (§7.1, §10.4), one layer below the equality question. So the
> property nouns sit at distinct planes, **resource** (device fact),
> **Group Role** (in-Group governance authority), **capability** (data access, Meadowcap, beneath Group Roles), and none
> collides with the prior art. (Lowercase "role" is the genus, the category of delegated authority; a "Group
> Role" is a concrete grant inside a Group. §5.0 and Part 1 §2.3.)

### 5.0. Two equalities, two inequalities: and the two layers

The question `P-Peer-Equality` actually answers is: **in what ways may one persona differ from another?**
There are exactly four properties, and the whole model is that **two are necessarily equal and two are
legitimately unequal.** "Personae are equal" is shorthand for this four-way split; collapsing it into
one phrase obscures which axes are equal and which are not.

What makes the equality *matter* is what a **persona** is: a persona is the **human layer's manifestation** in the
system, a principal by virtue of its key pair, present through lineage and verification (§4.5), not a node or a device. So peer-equality is **equality of personae as represented**, equality in
expression and in count. The protocol guarantees, by mechanism, that one recognized persona carries equal
rights and one flat unit of weight. Whether a persona corresponds to one distinct human is a separate
question the protocol does **not** answer, and **could not**: that binding has no technical representation
the protocol could read (§5.2), so there is no fact to certify. It is a social-utility judgment the Group
makes at its own standard (Part 1 §2.0, §5.6). The mechanical guarantee is what makes that judgment meaningful; it
does not substitute for it. That is the razor (Part 1 §2.0) applied to the persona concept itself.

**The two equalities**, equal for every persona, always:

- **Right, what a *principal* inherently holds.** The floor: voice, tenure, and exit/fork (§5.3). It is
  **equal for every persona, and unremovable.** The proof that it is a right and not a role is that the last
  of them, exit/fork, survives even when every Group Role is stripped and even when a quorum captures the
  Group: participation persists as the standing to leave with your state and continue. A right is precisely
  the thing that *cannot* be delegated or revoked, standing in the system rather than in any one Group.
  Attaches to the **principal**, flows to its clients.

- **Weight, how much a *persona* counts in governance.** **Flat: one per distinct persona**, where a persona is
  the unit a Group's **members resolve to by lineage** (§4.5), regardless of how many clients, devices,
  resources, or roles carry that lineage. Weight is equal **not by a separate decree but as a consequence of
  the equal right**: if standing-to-participate is equal (the right), then standing-to-be-counted
  is equal (the weight), the second is the governance image of the first, the same commitment in a different
  context, not an independent conclusion. Attaches to the **persona** (§5.6).
  *(A note on what the Group recognizes, and what it does not. A Group recognizes its **members**,
  clients, in MLS terms (§5.2). Lineage then resolves a Group's member-clients to **one persona** (§4.5), and
  it is that resolved persona that is counted once. The system attests this resolution by provenance. What the
  system does **not** attest is whether a persona corresponds to a distinct **person**: that one-persona-one-human
  binding is a **contextual judgment the Group makes** at its own confidence (§5.6). We avoid the phrase
  "personhood-verified": "verified" would imply the system did the verifying, when the protocol guarantees
  provenance and the Group judges personhood.)*

**The two inequalities**, legitimately different between personae:

- **Resource, what a *node* has.** Storage, uptime, reachability, a push token, a radio, "this
  box is willing to relay." A property of the **device or node**, not of identity: **intrinsic**,
  **descriptive**, **expected to be unequal**, and **not delegable**, you cannot hand another node your
  RAM. It is a fact about every node in the system (a persona's clients, but also meers and relays, §5.4),
  not only about personae; it is listed among the persona inequalities because, *across personae*, it is one of the
  two ways they legitimately differ. A resource says what is *possible*, never what is *permitted* and
  never how much a persona *counts* (§5.4). The word is **resource**, not "capability": "capability" is
  Meadowcap's word for data access (§5.5), and "resource" names the device fact without inviting the false
  slide from "able to" to "entitled to."

- **Group Role, what governance authority a *principal* has been *granted*.** Admin, moderator, gating, the
  act-for-the-Group authority, the authority to issue capabilities (§5.5). A Group Role is **granted by member
  consent, scoped, attenuating, and always revocable.** It is the one *operational* inequality the design
  permits: personae may hold different Group Roles, and that is normal. Crucially, **a Group Role rides entirely above the
  two equalities**, granting or revoking one never changes a persona's rights floor or its unit of weight.
  Group Roles are the application-layer construct MLS deliberately leaves undefined (§5.5). (Lowercase "role"
  is the genus, the category of delegated authority; a concrete grant inside a Group is a "Group Role.")

The one-sentence statement of the model: **personae are equal in rights, and therefore equal in weight, and
unequal in resources and revocable Group Roles.** Rights do not vary; Group Roles do. The inequality people
sometimes have in mind when they imagine rights differing is always a *Group Role*, a grant, never a right.

A note on what is **not** on this list. **Capability** (the Meadowcap data-access grant, read/write an
area of a namespace, §5.5) is not a fifth persona-property. It is the **mechanism a Group Role operates
through**: a Group Role may carry the authority to *issue* capabilities, and the capabilities themselves are
data-plane tokens (§7.1, §10.4), one level below the question of how personae differ. It is listed here only
to place it: it sits *under* Group Roles, not beside resources.

The four properties sit across two layers, because the entity that holds rights is not the same granularity
as the device that acts. The **identity layer** is **principals and clients**: a principal is the
permission-holding entity (one key-lineage), realized by one or more clients across one or more devices. The
**governance layer** is **personae and weight**: a persona is the human layer's manifestation, the entity
rights and weight attach to, and the locus at which the social-utility calls the system cannot compute
(Part 1 §2.0) are adjudicated. Both layers are defined in full at §5.2; what matters here is that they are
distinct, the identity layer answers *who is acting and can it be authenticated*, the governance layer
answers *whose standing counts and by how much*.

### 5.1. The only canonical state is local

A client can prove exactly one thing: its own local state. Its local store is **canonical for that
principal**. Any belief about another principal's state is **comparative** (range reconciliation) or
**asserted** (a signed fact it accepted), never canonical. There is **no global canonical state**, by
design. A lagging client computes a stale-but-honest state, never a false one, because it is only ever
reading its own store. *Verified / design*, the property §7 relies on to make authority a fold over an
append-only view.

### 5.2. Principal, client, persona: the identity model

> The consolidated term lattice and the invariants of record (the vocabulary
> a reviewer validates against) are in **Appendix D**; this section is the
> prose source those entries summarize.

A **principal** is a **permission-holding entity, identified by one key-lineage.** This is the genus. It is
defined by its *identity* (one authenticatable lineage), not merely by its function, so that "holds a
permission" does not collapse into "anything at all." A principal is reasoned about through the permissions
it carries, and "permission" spans planes: a Group member holds **Group Roles** (in-Group governance
authority, §5.5), while a non-Group principal like a meer holds **ecosystem permissions** (connectivity and
delivery reach, §5.4) and no Group Role. (Lowercase "role" is the genus of delegated authority; a concrete
grant inside a Group is a "Group Role.") Kinds of principal:

- a **persona**, the principal that **manifests a human** in the system, a principal by virtue of its key
  pair, carrying the rights floor and one unit of weight, and the locus at which the social-utility calls
  the system cannot compute (Part 1 §2.0) are adjudicated because a person stands behind it (the
  common case: one person, one persona per Group, possibly many devices). Its clients run on its devices,
  tied to the persona by lineage (§4.5); the persona is neither a client nor a device nor any node;

- a **Group**, a collective that can hold a Group Role as a single principal (its identity model is an **open
  seam**, see §5.10 and below);

- a **delegate**, not a separate species but a **state**: a persona or Group currently holding a Group Role
  delegated by another principal (§5.5).

A **meer** is **not** a principal in the Group-governance sense and does not appear above: it is a
broad-plane principal holding ecosystem permissions (§5.4), a blind store-and-forward node offering
availability capacity, configured by a persona to serve ciphertext. It holds no Group Role, no right, and no
weight, and it is named by persona-level configuration rather than enrolled as an identity. It is defined in
§5.4 and realized at the transport layer in §6.

A **client** is **software on a device that is a member of a Group**: one MLS **leaf**, one **signature
key**, one **credential**, authenticated as a **member** via the **AS** (§10.2). The term is MLS's, kept
for consistency. A **device** is hardware (a node, §5.4) and may host **more than one client**; a human
may have **more than one device**. So the hosting chain is human → devices → clients, and a principal is
**realized by one or more clients across one or more devices.** MLS addresses clients; Drystone governance
addresses principals, folding a principal's clients and devices, by lineage, to one persona (§4.5).

> **The governance-integrity spine, identity, not client count.** Governance quorums and thresholds count
> **personae (resolved by lineage to one persona per rooting key pair, §4.5), never clients.** Many clients across several devices are one
> identity's worth of standing. This is what makes "you cannot buy your way to shifting the centre of
> gravity" structurally true: adding clients or devices adds **resources**, never **rights** and never
> **weight**, because the count is over principals. The governance layer discerns the **lineage** of each
> client (the MLS leaf) and folds a principal's clients and devices to one persona, so a principal with one
> device and a principal with five are weighted identically. This is the property whose absence made early
> crypto-governance takeovers possible and painful to unwind; Drystone makes identity-not-resource the
> basis of weight by construction.

> **The keystone distinction, a lineage is a provenance object; a persona is the human it manifests, and
> personhood is a social judgment.** These are different *kinds* of thing, and keeping them distinct is the
> identity-layer instance of the spec's founding provenance/utility split (Part 1 §2.0).
>
> - A **lineage** is **technically representable**: it is a cryptographic-provenance chain, a thing the
>   protocol can point at, verify signatures against, and count. The protocol delivers the lineage with
>   certainty.
>
> - A **persona** is the **human that lineage is taken to manifest**. Whether a given lineage corresponds to
>   a distinct person, the one-persona-one-human binding, has **no technical representation at all**, because
>   it was never a fact the system holds. It is a judgment the *group* makes (§5.6), where to *recognize* is
>   to decide to **treat** a lineage as a distinct person for the group's own purposes, never to *verify* it.
>   The system counts lineages; the group decides which lineages it recognizes as distinct persons, and *that*
>   recognition is what turns a counted lineage into a weighted persona.
>
> So the binding between the two, *this lineage is one person*, is **not a lookup but an adjudication**,
> and that is precisely why it is a seat of social-utility judgment rather than something the spec
> resolves. Collapsing persona into personhood is the same category error as "the network can certify truth":
> it asks a provenance system to deliver a utility verdict. This is *why* "persona" and "personhood" are
> separate words in this spec, not loose synonyms, the separation is load-bearing, and how the binding is
> structured is set on the same **per-edge adversarial dial** as all other trust (Part 1 §2.3), because
> the posture toward "is this persona one person" is no more a single global setting than the posture toward
> any other edge.

> **Open seam, the principal that anchors a multi-client lineage.** The cross-device identity is the
> **principal**. The **Group-as-principal identity** takes a concrete shape in §5.10, a Meadowcap
> **communal namespace** rather than a derived central credential; what is designed-not-frozen is its key
> establishment and rotation under membership change. Carried to Appendix B (the collective / federation
> gap).

*Running example: Alice and Bob are personae, principals that hold standing, and each of Alice's two
clients carries a membership key descending from her single lineage (walkthrough beat E1).*

### 5.3. Rights: the inherent, equal floor: never delegated, never unequal

A right is what a principal **inherently holds**, not what it may be *granted* (that is a role, §5.5). The
floor is held **identically by every persona** and **cannot** be delegated away or stripped without degrading
the system (Part 1 §2.4); it is one of the two equalities of §5.0. The base floor:

- **Read your own local history.** Unqualified, identical for every persona.

- **Read the history of a Group you are a member of, for the period of your membership.** Begins at join,
  ends at leave; includes what the persona was present for; does not extend to content authored after the
  persona leaves, and does not retroactively vanish for content the persona legitimately held while a member
  (§5.7).

- **A Group holds full history for itself,** independent of any member's tenure.

Two consequences that keep the floor precise. **A persona's own history is permanent; its window
into a shared Group is bounded by membership**, two different histories, treated as such. **A principal
with no local history of its own still holds the full read-your-own-history right**, exercised over an
empty set (a persona that has joined but authored nothing satisfies the right vacuously). A meer is not the
example here: it is infrastructure, not a principal, so it holds no rights at all (§5.4). Where a principal
is *blind*, that is the absence of a key and of any role conferring read, never a restriction on a right.

> **One open check before the rights set hardens** (carried from Part 1 §2.4): the proven floor in this
> draft is the read-rights triple above. The fuller rights articulation is **three rights**, **tenure**
> (standing to remain a persona), **voice** (standing to assert into the record and be corroborated or
> refuted), and **exit** (the right to fork); each fixed by what its removal would foreclose. A claim on a
> Group's commons is **not** a right: it is not part of the inalienable floor, and where it has substance it
> is a data-layer matter, ownership of a Meadowcap communal namespace (§5.10), handled there rather than in
> the rights set.
>
> The remaining open check is **`tenure` under re-key**: can the §7 survivor / re-key path leave a persona
> formally a member but unable to re-establish its standing after a re-key? If so, tenure is not yet a
> clean right and the set cannot harden. This needs a concrete test (see Appendix B for what to exercise).
> *Design* (Appendix B). The Meadowcap distinction between **communal** namespaces (authority from owning a
> subspace key; horizontal, no apex) and **owned** namespaces (top-down single-owner control) is the
> nearest prior-art cut at the commons-asset question and the model for the group-principal (§5.10).
> *(Meadowcap communal/owned semantics confirmed verbatim against the Willow Protocol spec; see §5.10.)*

*Running example: Bob holds exactly this floor, identical to Alice's, even though Alice also holds a
moderator Group Role Set; the Set sits above the floor and cannot reach into it (walkthrough beat E1).*

### 5.4. Resources: node facilities, descriptive, not delegable

A **resource** is what a **node has**, a facility intrinsic to the hardware and its configured intent.
Resources are a property of **any node in the distributed system**, not only of a persona's clients: a persona's
client devices have resources, and so do meers (the blind store-and-forward nodes, below) and relays. The
persona/meer distinction is drawn in the **identity** model (§5.2), never in resources; resources are a
physical fact about a box, blind to whether that box is an identity. Resources are **descriptive** (they
report what is possible), **unequal across nodes**, and **not delegable**: a node cannot hand another node
its storage, uptime, or radio. A resource enables a principal to *fulfil* a role (§5.5), or makes it
*unsuited* to one, but holding a resource is never itself a grant of authority and never adds governance
weight.

**Common resources** (the current set, not closed):

- **Availability capacity.** The device can hold and serve a Group's *encrypted* objects, including
  buffering for offline members. Availability capacity does **not** imply read: such a device holds
  ciphertext it cannot decrypt, because it is never given a key. A node a persona configures to do only
  this, blindly, is a **meer** (below); it is infrastructure, not a principal.

- **Read / search-offload capacity.** The device can decrypt, index, retain, and serve a Group's history
  (the *facility*; whether the principal is *permitted* to is a read **role**, §5.5).

- **Reachability.** The device is positioned (public reachability, uptime) to forward for others.

The pairing rule (for the resources that matter to *governance*, namely a principal's own clients): a
**role** (the governance authority, §5.5) is only useful to a principal whose **client** has the
**resource** to exercise it. Granting a read role to a device with no decryption capacity is inert. Roles
and resources are matched at the **Group Role Set** layer (§5.5), which is exactly why a Group Role Set bundles a role
set *with an expectation of resources*. The anti-capture consequence is in the words: a node may have more
resources, and that buys it no rights and no weight, only the ability to be *useful*, never to *count for
more*.

**The meer: blind store-and-forward infrastructure.** "Meer" is a **colloquialism**, not a model entity:
it is just a short name for a **blind store-and-forward node**. Such a node accepts, retains, and serves a
Group's encrypted objects, seeing ciphertext plus routing metadata only. It is **not a Group principal, member,
or persona**, holds **no Group Role**, **no rights floor**, and **no weight**. It is not granted a Group Role
and not enrolled as an identity. Its blindness is **structural**: it is never issued a decryption key, so there is
no "decrypt" to forbid and no Group Role to strip. A Tier-0 meer can prove it holds zero payload keys (§8).

**A meer's presence in a scope is a fabric-level fact; a persona's *use* of it is a per-persona decision.**
These are two different layers and must not be collapsed. A meer participates in a Group's delivery scope
the way any swarm node does, it is in the gossip fabric, carrying and seeing the sealed envelope and its
routing metadata as it passes, and that presence is not something a single persona can revoke: it is a
property of the **Delivery Fabric** (the blind carrying population, defined at §6.3), not a per-persona
grant. What *is* a per-persona decision is whether to
**use and rely on** the meer, calling in for held messages, depending on it for durability, respecting its
store-and-forward function, or declining all of that. Because a single meer's exposure sits at the fabric level
and is shared across everyone in scope (and can be in scope for several Groups at once, since scope is
exposure reach, not bounded by any one Group's membership), one persona declining to interact does not
remove the meer from scope or change anyone else's exposure. So the meer is not "adopted per persona"; its
scope presence is a fabric fact, and only the *trust-and-use* decision is the individual's.

**A meer is a principal in the broad sense, though not in the Group-governance sense, and it does hold
permissions.** Saying the meer holds "no Group Role, no rights, no weight" is a claim about the
*Group-governance plane* only. It does not mean the meer is authority-less in the system. The meer is a
**principal** in the broad sense of §5.2 (an actor reasoned about through the permissions it carries), and
those permissions are real and enumerable, they are just of a different kind: **ecosystem permissions**,
connectivity and delivery reach rather than in-Group governance. A meer typically holds permission to be in
a Group's delivery swarm, and often permission to talk to a downstream push-notify node, which in turn may
hold permission to talk to an external third party (an OS push service). This little chain of ecosystem
permissions is what lets the meer function as infrastructure, and it is why "Group Role," "right," and
"weight" are reserved for the governance plane while "permission" spans planes: the meer holds ecosystem
permissions and no Group-governance ones. These permissions are **enumerable** and **blind** (exercising
them never confers sight of content).

**Two revocation planes, and why a persona's recourse against a meer lives on the second.** Revocation in
Drystone is not one mechanism; it acts at two layers with different actors and different authority.

- **Group-governance revocation** (§5.7, §5.8) acts *inside* a Group, on Group facts, a Group Role or
  membership. It runs through the Group's replicated policy and the k-of-n threshold counted per persona,
  and it is global to the Group by construction: once the fold accepts it, the fact holds for every member.

- **Node-local withdrawal of use** acts at the layer of the software an individual persona runs as a
  standalone authoritative node. It is how a persona withdraws its own reliance on a *non-Group* helper
  like a meer or relay: it stops calling the helper, stops depending on it for durability, declines to
  interact. Its authority comes from the standalone-authoritative-node premise (§5.1), so it needs no
  threshold and no governance round. Crucially, this is **not** removal of the helper from scope: the meer
  remains in the Delivery Fabric and still sees whatever routing metadata the fabric exposes. It is the
  **exit right exercised at the client level**, a recognition of the persona's autonomy over its own node,
  not a change to the shared scope. (Architecturally this holds by construction, since a persona controls
  whether its own client calls, answers, or connects to a given helper, even where a dedicated "stop using
  this meer" affordance is not yet implemented.)

The **asymmetry is the tell** they are two planes. A Group Role revocation is global to the Group and
actually changes a Group fact; a node-local withdrawal is local to the one persona and changes only that
persona's behavior, leaving the helper in scope. Whether to withdraw, and the shape of the response
(re-home your own durability to a different meer, lean on more than one, pull the function onto your own
device, or fork the Group entirely if the concern is Group-wide), is a **social-utility judgment** about
trust and tradeoffs, not a mechanical toggle, which is why the protocol makes the response available but
does not compute it (the Part 1 §2.0 razor, one layer down).

**Helper governance and alignment is a first-class concern, treated separately.** Because scope is broader
than a Group and a single helper can be in scope for many Groups, the **operation and governance of helper
nodes (meers, relays, push-notify nodes) meaningfully shapes the scope of exposure across the whole
system**, at a layer no individual persona's non-interaction can dissolve. A persona's node-local
withdrawal is the individual backstop, the exit exercised at the client; it is not a substitute for the
question of who runs these helpers and whether they are ideologically and operationally aligned. That
question is a first-class concern in its own right, not something each Group settles internally, and it is
treated separately (carried as an open item; see Appendix B and the conventions reference).

**A meer is optional, not required.** Clients communicate directly peer-to-peer over the transport (§6);
two clients, or many, can exchange MLS messages with no meer in the path. What a meer adds is **offline
persistence**: because MLS operates asynchronously (no two clients need be online at once), a message for
an absent recipient has to be held somewhere, and a meer is the optional augmentation that holds it, useful
when some node has high-availability resources to spare. In MLS terms a meer is one realization of the
**Delivery Service's store-and-forward function** (§10.2), and MLS does not require a central Delivery
Service: clients on a peer-to-peer network supply the delivery logic themselves. **Distinguish the meer
from an iroh relay** (§6): the relay is a transport-layer blind packet forwarder for reachability (NAT
traversal), holding nothing; the meer is an application-layer store that persists encrypted objects for
later delivery. Both are blind; they sit at different layers and neither is mandatory.

*Running example: Carol's node has vast resources, always on with deep storage, while Bob's phone has
little; the difference is descriptive and confers neither of them any standing (walkthrough beat E3).*

### 5.5. Group Role, capability, Group Role Set, and delegation: the governance and data-access planes

Two distinct kinds of grant sit above the rights floor, at two different planes, and a third construct
bundles them. None touches the inherent rights floor or the flat weight.

- **Group Role, an in-Group governance authority.** A scoped, attenuable authority to *act* in the Group's
  governance: admit or remove members, gate distribution, hold the **act-for-the-Group** authority (§5.10),
  or *issue and revoke capabilities* over the Group's data. A Group Role is **granted to a principal** by member
  consent, is **revocable** (§5.7), composes by union, and mutates freely, granting or revoking one is
  normal governance traffic, no alarm. Each grant is a governance fact (§7). **This is the layer MLS
  deliberately leaves to the application**, RFC 9750 §6.4 (Access Control) states that MLS "does not itself enforce any
  access control on group operations" (any member can add or evict), in contrast to designs with a single
  group controller, and that "MLS-using applications are responsible for setting their own access control
  policies", giving the example that if only an administrator may change members, the application must
  inform members of that policy and who the administrator is (§10.2). Drystone's Group Role layer **is** that
  application policy, enforced not by a server but by the governance fold: a child's `Remove(parent)` is a
  well-formed MLS message that honest peers **reject as unauthorized** because the replicated Group Role policy
  does not grant the child that authority. *(Honest seam: the protection is convergent agreement that the
  op is unauthorized, not cryptographic impossibility of emitting it, §5.7.)*

- **Capability, a data-access grant (Meadowcap's sense, kept verbatim).** An **unforgeable token
  bestowing read or write access to an area of a namespace**, issued by that data's owner, attenuating
  under delegation. A capability is about **reading and writing entries**, nothing else: it does not admit
  members, does not gate, does not carry a vote. Capabilities are *issued under* a Group Role (the authority to
  issue them) and live in the data plane (§7.1, §10.4). Drystone keeps Meadowcap's word because Drystone
  intends Meadowcap (or a Meadowcap-shaped mechanism) as the data-access realization, and renaming it
  would fight the prior art.

- **Group Role Set, a named, pinned, Group-recognized bundle** of Group Roles, the capabilities they imply, and the
  **resources expected to fulfil them**, with an **enforced composition**: a **required** set, a
  **forbidden** set, and optionally **mutually-exclusive** Group Roles (two that may never travel together).
  It serves two functions: it lets a Group **grant or revoke a bundle as one unit** rather than
  Group-Role-by-Group-Role (the human-fatigue reason, people reason about "moderator," not a list of
  individual grants), and it lets the Group **constrain composition** for separation of powers ("a holder
  of this Set may not also hold that Group Role"). Prescriptive; it answers "what is a principal of this
  name supposed to hold, and never hold." Drift from the pinned composition is an integrity event the Group
  flags. *(First-class term, still settling: the name and the two functions are fixed, the full mechanism is
  developed across §5.5 to §5.9 and flagged where it is not yet frozen.)*

**Delegation** is the act of granting a Group Role or passing on a capability. Two normative properties:

- **Always attenuating and bounded.** A principal **MUST** be able to delegate only a subset of what it
  holds, never a superset, because a principal that could delegate more than it holds would mint authority
  from nothing, letting a permission appear without the consent every grant requires (`Realizes:
  P-Peer-Equality`; the attenuation requirement, §7.2 R2; this is also Meadowcap's confinement property for
  capabilities). Delegation **moves or narrows** authority; it never widens or mints it.

- **Two targets, one primitive.** A principal **MAY** delegate to (a) another client in its **own
  principal's device Group** (trust stays within personal control) or (b) another principal in the
  **shared Group** (a cooperative anchor, another member's always-on node). Same mechanism; the trust
  boundary is the user's choice. This is what lets Drystone refuse consolidation onto a single keeper.

All of these live in the **grant planes** and **none alters weight or the rights floor.** A principal
carrying any Group Role or capability still holds its complete inherent floor and its single unit of weight. The
mechanical check: every Group Role Set **MUST** be definable as `floor + [explicit Group Role set] + [implied
capabilities] + [expected resources]`; a name meaning "entitled to fewer **rights**" is **forbidden**;
that would be a smuggled rights distinction. **Rights have no presets; Group Roles, capabilities, and Group Role Sets
do.**

A Group Role Set's pinning is enforced by a **drift check**: the Group gathers the Group Role grants in force for a
principal from the governance log and compares them against the declared Group Role Set's required / forbidden /
mutually-exclusive composition. Mismatch in **any** direction is the alarm, a principal that *acquired* a
forbidden Group Role (dangerous), *lost* a required one (failing the job relied on), or *combined* two
mutually-exclusive Group Roles (a restricted combination). The check is mechanical, because every side is a fact
already in the log. (Separately, "this node is blind" is trustworthy for a structural reason, not a pinned
Group Role set: a Tier-0 store node can prove it holds zero payload keys, §8.)

> **Trust-dynamics changes fail loud, by design.** Two cases route to a noisy hard-stop rather than a
> silent rejection. The first is a governance action placing two mutually-exclusive Group Roles on one principal.
> The second is a reconfiguration that would make a Group's blind store-and-forward node (a meer, §5.4)
> receive decryption keys, converting blind infrastructure into a reading party. In either case the action
> surfaces to the affected Group ("a restricted change was attempted; your communications may be at risk,
> fork without the principals who voted it?") and routes to human adjudication (§7.6). The reasoning: a
> quorum that votes to de-blind a node everyone relied on as blind has *changed the trust dynamics of the
> Group*, which is precisely the kind of standing contradiction that is a utility judgment, not a
> computation. *Design, decided; the loud-failure rung ties to §7.6.*

Worked example, a **moderator** (a Group Role Set):

```
moderator (a Group Role Set) ::= floor                            // full inherent rights, unchanged
                        + requires role { admit, remove }     // governance authority granted by consent
                        + expects  resource { reachability }  // device facts that help fulfil it
                        + forbids  role { act-for-the-Group }  // kept separate from Group-signing authority
                        + holds    capability { }              // no standing data-access grant by default
```

A moderator is a **persona** holding a moderation Group Role: its rights floor and unit of weight are unchanged by
the grant, and revoking the Group Role returns it to a bare persona. The Group Role Set pinning makes drift an integrity
event: acquiring the forbidden `act-for-the-Group` Group Role, or losing a required one, is flagged. Delete the
Group Role Set name and nothing about any persona's rights changes. *Design (Group Role Set drift-check and mutual-exclusion
formalism).*

A **meer** is *not* a Group Role Set, because it is not a principal in the Group-governance sense: it is blind store-and-forward infrastructure
a persona configures for its own durability (§5.4), with no Group Role to pin and no rights to bundle.

*Running example: Alice delegates Carol's node a read capability, a subset of what she holds and never a
superset, so the grant moves access without minting authority (walkthrough beat E3).*

### 5.6. Weight: flat by default, conserved under delegation, anchored to personhood

**Weight** is the second of the two equalities (§5.0): how much a persona counts when the Group decides
something. It attaches to the **persona**, and its default is **flat, one per distinct persona**,
regardless of how many clients, devices, resources, capabilities, or roles that persona holds. It is equal
**not by a separate decree but as a consequence of the equal rights floor** (§5.3): equal
standing-to-participate is the same fact as equal standing-to-be-counted, the same commitment in a
different context, not an independent conclusion. Resources and Group Roles are the two
legitimate inequalities (§5.0); rights and weight are the two equalities, and weight is the governance
image of the right.

**The default model is one-persona-one-vote.** Other governance models are explicit variations layered on top
of the flat default, not separate primitives (the multi-model intent from the open-thread review,
governance at scale):

- **Liquid delegation.** A persona **MAY** delegate the *exercise* of its weight to a delegate-principal,
  revocably. The delegate then exercises several personae's weight, but every unit still traces to a distinct
  persona (by lineage).

- **Elected admins.** A Group **MAY** vest decision Group Roles in a small elected set (closer to forum
  moderation), with personae retaining equal weight to elect, recall, and ultimately fork.

- **Broadcast-only.** A Group **MAY** define a rights model where most principals receive rather than
  decide; weight is near-vestigial in such a Group, but the floor (voice, exit) is retained.

**The conservation invariant, which holds across every model:** weight is **allocated one-per-recognized-persona
at the source and is never minted, only moved.** A delegate exercising five personae's weight still reduces to
five distinct personae the Group recognizes. The total weight in a Group equals the count of its recognized
personae, no matter how delegated, pooled, or elected. **Delegation moves weight; it never creates it.** This
is the anti-capture property, and it is *stronger* and *more honest* than "everyone votes equally" (some
Groups won't): the claim is not equal exercise, it is **non-inflatable total over the personae the Group
recognizes.**

> **What the protocol guarantees vs what the Group judges, and why this split is the honest one.** The
> anti-capture property has two parts that live at two layers. Collapsing them into a single claim that
> "personhood is unforgeable" would be the provenance/utility collapse the spec exists to prevent (Part 1 §2.0):
> the protocol proves provenance, but personhood is a Group judgment, and the two must stay distinct.
>
> 1. *Protocol guarantee (provenance, technical, airtight):* messages from a key-lineage are provably from
>    that lineage; governance weight is **flat per recognized persona and conserved under delegation**, never
>    minted by adding clients or resources. Adding devices adds resources, never weight. This Drystone
>    guarantees by mechanism.
>
> 2. *Group judgment (personhood, social, contextual):* whether a recognized persona corresponds to a
>    distinct person is a **utility judgment the Group makes at its own confidence**, on the same
>    trust-to-do gradient as every other delegation. The protocol does **not** guarantee
>    one-lineage-one-human, and **could not**: there is no fact for it to deliver (the binding has no
>    technical representation, §5.2) and no authority tier above the Group from which to impose it (§3.1, §8).
>    Both impossibilities are the same Part 1 §2.0 limit, seen from the delivery side and the enforcement side. So
>    the judgment does not get handed to the Group; it **necessarily falls** to it. (This is distinct from a
>    Group **gating its own entry** to its own standard, which is legitimate and often desirable: gating is
>    the Group's recognition dial operating at the door, not the protocol enforcing a binding from above.)
>
> So the load-bearing claim is **not** "you cannot forge personhood." It is: *given the Group's recognition
> of who its personae are, weight is flat and uninflatable by resources.* The equality holds over the personae
> the Group recognizes, and the recognition is the Group's own.

> **Sybil resistance is contextual, not global, stated honestly rather than overclaimed.** Multiplicity has
> two cases that must not be conflated. *Across discrete systems*, one human holding many personae (one per
> Group) is the intended design, not a flaw. *Within a single Group*, the intent is one persona per human;
> the protocol cannot enforce that binding (above), so two personae for one human is **possible**, and its
> consequence is **degraded governance**: weight that should be one unit counts as two, so per-persona
> equality stops corresponding to per-human equality. Whether this is tolerated is the Group's call, and the
> **strength of binding a Group requires before recognizing a persona as a distinct human is proportional to
> the Group's function and goals**: a Group with access to financials sets a tighter standard than a casual
> messaging Group, which sets a tighter one than a public-but-registered event invite. This proportionality
> is on **recognition**, never on weight: a stronger binding requirement, not a heavier vote; once recognized,
> weight is flat, one per persona, regardless of how strong the binding was. Sybil resistance is supplied by
> the Group's chosen personhood-confidence mechanism, not by the protocol, and it ranges across the trust
> gradient:
>
> - **High**, a family Group where a partner scans a QR code to join: the binding of social identity to
>   key provenance is high-confidence, so flat-per-persona is flat-per-person in practice.
>
> - **Medium, and anonymous**, an activist or privacy-sensitive Group that delegates the personhood check
>   to a verifiable-credential service which enforces "one personhood per government ID" *without ever
>   revealing the ID*: one-persona-per-person **and** real-world anonymity, simultaneously. Provenance is
>   guaranteed (these messages came from one root key chain); real-world identity is never disclosed.
>
> - **Low**, an open broadcast Group where binding is loose and Sybil resistance is weak, accepted as the
>   property of that context.
>
> Delegating the personhood check to a credential service is **itself a utility judgment to accept**, and
> if that service turns adversarial the remedy is the same as for any standing contradiction: **withdraw
> trust and fork** (§7.6). "Revocable" here does not mean a technical off-switch, *the fork is the
> revocation.* Treating the credential service as a structural dependency needing a protocol primitive
> would re-collapse utility into provenance; it is a valuation edge (Part 1 §2.3), trusted by choice and
> exited by fork.

> **Why declining to solve global personhood is faithfulness, not a cop-out, the variety argument applied
> to identity.** A protocol that *enforced* one-key-one-human would make legitimate **multiple
> presentation** impossible: the same person as a parent in one Group, a pseudonymous activist in another,
> an anonymous participant in a third. That plurality is part of the social substrate, and collapsing it
> to a single global identity would prune variety, the Ashby argument (Part 1 §2.3) applied to identity
> itself. The Spritely Institute (Christine Lemmer-Webber, Executive Director and lead author of W3C
> ActivityPub) names the relevant principles directly. On contextual identity: There is no "global town square", and we are deeply concerned about context collapse. Communication and collaboration should happen from contextual flows. On
> not overclaiming guarantees: Much harm is caused by giving people the impression that we provide features and guarantees that we cannot provide. And on trust as
> contextual and revocable rather than binary, their capability-security framing holds that trust to be something fundamental to cooperation rather than all-or-nothing, consent-granted
> mechanisms that are intentional, granted, contextual, accountable, and revocable. Drystone
> takes the same posture: guarantee provenance, leave personhood to contextual group judgment, build trust
> as a contextual revocable edge, and do not overclaim a global guarantee it cannot honestly provide.
> *(Source: Spritely Institute "Technical Values and Design Goals," spritely.institute/about; W3C
> ActivityPub lineage. Quotations verified verbatim against the primary page.)*

> **This is not a Drystone quirk, cryptographic trust always grounds out in a social judgment. Three
> irrefutable cases.** The persona/personhood split restates a pattern every deployed cryptographic-trust
> system already exhibits: the math guarantees *continuity, integrity, and authorship*; it never
> guarantees what a key *means*; that final binding is always social.
>
> - **TLS / X.509 certificate authorities.** The signature chain is mathematically airtight; what it is
>   anchored to is the CA's *process and reputation*, an institutional trust that the CA verified the
>   entity before signing. The browser trust-store is a curated list of "CAs we have collectively decided
>   to trust," a social artifact, not a cryptographic one. *(Seam, marked: a CA is a **centralized** anchor,
>   the very thing Drystone refuses structurally. The analogy holds at "cryptographic certainty
>   terminates in a social trust decision about an attester"; it breaks if pushed to "so Drystone has CAs."
>   Drystone's attesters are chosen, per-edge, and **forkable**, a valuation edge you can exit, not a root
>   you are stuck under. Drystone distributes and makes-exitable what PKI centralizes.)*
>
> - **PGP web-of-trust.** The cleanest case, because the social judgment is explicit and user-set. Validity
>   is inferred by transitivity of *human attestations*, a user signs another's key to assert it belongs
>   to its claimed owner, and the trust threshold is the user's own policy (accept a key only if signed by
>   N trusted others; loosen it at your own risk). The GnuPG trust-level guidance is itself explicitly
>   social and discretionary: it gives verifying a fingerprint in person against a hard-to-forge photo ID
>   such as a passport as an *example* of high-confidence certification, while stating that in the end it
>   is up to the user to decide what casual and extensive verification mean. And the proof-of-personhood
>   literature is explicit that web-of-trust establishes only *that two parties agree on the correct key*,
>   and was **not** designed to verify unique personhood or resist Sybil attacks, exactly Drystone's
>   provenance-yes / personhood-no line, drawn by the canonical decentralized-trust system thirty years
>   ago.
>
> - **SSH trust-on-first-use.** The quietly universal one. On first connection SSH cannot verify the key
>   against anything; it shows a fingerprint and asks the human to decide whether to trust it. Thereafter
>   the cryptography guarantees *continuity* (same key as last time) with certainty, but the founding act,
>   *is this the right host*, is a human decision with no cryptographic basis, made by verifying the
>   fingerprint out-of-band. (The TOFU mechanism is the same one GnuPG documents: a key is memorized on
>   first sight, and a later conflicting key for the same identity is flagged for manual confirmation.)
>
> The throughline: cryptography can prove *this key signed this*, *this is the same key as before*, *the
> chain is valid*; it can never prove *this is the right person / a distinct human / a trustworthy party*.
> Every real system either makes a human decide (SSH, PGP) or delegates to an institution whose authority
> is itself social (TLS CA). Drystone's persona/personhood split is that universal pattern, stated honestly,
> with the binding made an explicit **per-group dial** instead of a hidden default. *(PGP web-of-trust and
> the GnuPG TOFU / trust-level descriptions verified against GnuPG documentation and the proof-of-personhood
> literature. **[confirm, TLS/X.509 CA wording against RFC 5280, and the SSH
> known-hosts/TOFU wording against the OpenSSH man pages.]**)*

> **What remains genuinely open** (carried, not resolved): the **mechanisms a group may plug in for its
> personhood-confidence dial**, QR-scan binding, verifiable-credential services, social-graph,
> web-of-trust, are application-layer choices Drystone enables but does not mandate or fully specify, and
> the wire shape of delegating a personhood check to a (forkable, valuation-edge) credential service is
> `[gates-release]`. This is **not** a threat-model hole the protocol must plug; it is a contextual judgment the
> protocol deliberately leaves to groups. The AS (§10.2) is the architectural seam where a binding is
> checked; its *content* is the group's contextual call. (Appendix B.)

*Running example: Alice runs two clients but counts for one unit of weight, the same one as Bob, because
weight is anchored to the persona, not to the device count (walkthrough beat E1).*

### 5.7. Membership, standing, and revocation authority

**Standing** is decided from recorded, signed data, never the actor's own assertion. A message is
authorized iff its author held standing on a branch sharing the relevant lineage root. *Verified.*

**Revocation** removes a client/principal from the accepted set *going forward*. Survivors **MUST** reject
the revoked party's subsequent branches, because revocation removes it from the accepted set going forward,
and **MUST NOT** claw back history it contributed before removal, because that history was authored with
standing and revocation protects the future, not the past (§5.8): unmaking it would corrupt the append-only
record rather than withdraw an authority (standing is not membership; history is not erased). *Verified.*

**Revocation/add authority is a threshold dial** (k-of-n, **counted by distinct persona (by lineage),
never by client**): default 1-of-any, up to k-of-any or role-restricted admins. A membership op is
authorized iff it carries signatures meeting the Group's **current, replicated** policy; policy lives in
versioned Group state and is itself changed by governance ops under the current policy. The canonical form
is a **co-signed op**, a self-certifying k-of-n bundle validated locally against the current epoch,
freshness-gated (§7.4); proposal-plus-votes is an optional deliberative mode. *Verified (real k-of-n
bundle verified over live transport: an authorized 2-of-≥2 revoke accepted, an under-threshold revoke
rejected).*

> **Threshold counts personae, not clients, the same spine as §5.2/§5.6.** A k-of-n membership threshold is
> evaluated over **distinct personae (by lineage)**, so a single persona's multiple clients cannot
> together satisfy a multi-signer threshold. This is the mechanical face of "more devices does not grant
> more rights": the lineage of each signing key is resolved to its principal before the count, and
> co-signatures from clients of the same principal count once.

**The admin floor is derived from policy, anti-brick only.** A threshold `k_op` **MUST** be ≤ eligible
signers by distinct persona (by lineage) at the epoch it is set (solo genesis ⇒ `k_op = 1`; a Group
**MAY** be born "create with 10, need 5"); raising above headcount self-bricks and is rejected. Once set,
the Group **MUST** retain `n ≥ k_op`; a membership op whose post-state breaches the floor is **structurally
invalid** (rejected by every verifier from replicated policy alone). `k` **MUST NOT** auto-track `n`
downward (a threshold-downgrade attack). The floor is **anti-brick only**: a legitimate quorum acting
within policy, including self-capture, is accepted; the recourse for an out-voted minority is the §7.6
re-formation fork, never a structural veto. **Capture ≠ brick.** *Design, decided; tests specified,
partially run (see §7.3 capped-root note and Appendix B for the coverage Drystone claims).*

> **Capture ≠ brick is also Drystone's answer to the uncapped-root steelman.** Matrix, under adversarial
> review, prevents room-capture by granting the creator uncapped, permanent power (an apex). Drystone's
> design philosophy is the opposite and is the one consistent with `P-Peer-Equality`: a legitimate quorum
> **may** capture a Group (that is permitted), what is forbidden is **bricking** it (rendering it
> inoperable / unrecoverable), and the remedy against capture is **exit, the §7.6 fork**, not an apex
> that prevents capture in advance. So the comparison with Matrix is not only "can a capped root match
> their soundness" but "is exit-as-remedy-for-capture sound where apex-prevents-capture was their
> choice." See §7.3. *(MSC4289 creator-power verified against the Matrix Project Hydra disclosure, Aug
> 2025.)*

**Group Roles are revocable delegations, never impositions.** Every granted Group Role (admin, moderator, a
content-gating Group Role) **MUST** be a revocable delegation under the same threshold
authority, **MUST** carry only scoped, enumerated, non-creeping authority, and **MUST NOT** be immutable,
forced, or held by structural right, because a Group Role that could not be revoked or that attached by
structural right would be an entrenched authority indistinguishable from a right, a center no quorum could
reach, which is exactly the inequality `P-Peer-Equality` forbids at the role layer. A creator holds **no**
structural superuser right: at creation they
receive a bootstrap admin Group Role purely so a one-member Group can function, revocable like any other.
**Anti-entrenchment ladder:** any delegated Group Role is revocable (1) routinely under the policy threshold,
(2) as an always-available backstop by unanimity of the non-holders (a ceiling on revocation difficulty,
a Group may set an easier bar, never a harder one), and (3) ultimately by the §7.6 fork. **No grant may
make itself irrevocable.** *Verified (revocation mechanics); design (ladder, decided).*

*Running example: the Group admits Dave and sets a k-of-n threshold, and no grant in that configuration can
make itself irrevocable, because the fork remains the floor beneath every threshold (walkthrough beat E4).*

### 5.8. Revocation reuses governance machinery; it protects the future, not the past

A Group Role grant is a governance fact (§7); revoking it is a new fact that supersedes the grant, resolved by
the same total order and fold as any other governance conflict. For Group Roles that confer read, revocation
**MUST** exclude the revoked principal from reading content authored after the revocation folds in (the R5
forward-read exclusion, §7.2), which the MLS realization delivers by advancing to a new epoch whose keys the
revoked principal does not hold (identical to membership expulsion). Revoking a read-delegate is, formally, an
expulsion-shaped fact.

This inherits an honest limit: **revocation protects the future, not the past.** A principal that held a
read Group Role while valid may have retained what it read; revocation stops future access, it cannot unmake what
was legitimately held. The plain form, which a non-expert can hold: *you can revoke a delegate's access to
new content at any time and it actually takes effect, but the delegate may keep copies of what it already
saw, true of everyone anyone has ever shared anything with.* Stating it plainly is more honest than
implying otherwise.

#### 5.8.1. Open item: gating against the read right

The availability and read/search-offload roles are clean additive permissions. **Gating** is different: it
acts on the distribution or visibility of content, which bears on other personae's ability to exercise their
read right, so the additive framing does not automatically dissolve the tension. The likely resolution, to
be *specified* rather than assumed: gating acts on distribution/visibility within the Group's own governed
rules, **not** on the underlying right to read what one legitimately holds; every gating action is itself a
governed, attributable governance fact; and a gated persona's right to its own local history (including what
it already holds) is untouched.

> `[gates-release]:` The precise relationship between a gating action and the read right MUST be specified, what
> it can and cannot affect, how it relates to content already held locally, and how it is bounded by the
> Group's rules so it cannot become a backdoor for suppressing a right under the guise of a Group Role. This is
> the one Group Role that, specified carelessly, could re-introduce a rights distinction through the permission
> layer, and therefore the one most needing an explicit forbidden clause wherever it appears in a Group Role Set.

A content-visible gating Group Role also weakens the system's "cannot comply" property (compellability): a
principal that has seen content cannot un-see it on revocation. The default **MUST** therefore remain blind
and any such Group Role **MUST** be strictly per-Group opt-in, disclosed, scoped to the least-invasive rung, and
accountable, and it is a policy/legal question, not only an engineering one (gates a real deployment).

### 5.9. Exitability: the backstop that makes flexibility real

> `Realizes: P-Durable-Enablement`

A delegated Group Role is only meaningfully different from a captive structural dependency if the delegation can
be withdrawn and restructured **without loss of rights.** Therefore:

Any default delegation a principal or Group adopts **MUST** be revocable and restructurable down to the
rights floor (§5.3) at any time, with **no loss of rights** and **only graceful degradation of capacity**,
never loss of function or standing. Two different things are in play here and must be kept apart. The
**read / search-offload Group Role** (the authority to decrypt, index, and serve, §5.4) is a governance
grant, so like every Group Role it **MUST** be revocable under the Group's threshold authority (§5.7);
revoking it excludes the ex-delegate from new content, realized by an epoch advance (§5.8). **Availability**, by
contrast, is **not a Group Role at all**, it is a *resource* (§5.4): a node's blind capacity to hold and
serve ciphertext, needing no grant because there is nothing to permit.
A principal serving these functions (a cooperative anchor or single operator-principal) **MUST** be able to move
the read Group Role to a different principal, split it, or drop it, and members **MUST** be able to shift
their blind-availability reliance to a different node, in every case continuing to function. What degrades
is capacity (deep search may slow or need a member's own device online); never rights or the ability to
communicate and govern.

Material reversibility is normative, not formal: (a) a helper holds only **encrypted** state and the keys
are held by the Group's members (§5.4), never by the helper, so no helper can hold data hostage; (b) because
whether to *use* a given meer is a per-persona decision at the client (§5.4), each member can shift its own
durability to a different store-and-forward node by withdrawing use of the incumbent and calling another,
with no governance round, and this per-member freedom aggregates to a Group-level property: the Group as a
whole is never hostage to any one helper, because no member depends on it by structural necessity; (c) the
§7.6 re-formation fork remains the adversarial backstop for the Group-governance plane, where a concern is
Group-wide rather than one member's. *Verified, a meer's encrypted store was
exported, imported into a different replacement meer, and a member re-homed and converged identically;
losing a meer costs availability, never data.*

**The asymmetry of expressible range** (a checkable claim, not a quality judgment): a flexible model can
present as the rigid one, but the rigid one cannot present as the flexible one. Drystone can be configured
to behave like a single-keeper deployment; a server-shaped system cannot be configured to behave like the
exitable, per-Group-Role-delegated model, because its central dependency is structural rather than granted. The
design question this hands forward, and the thing to press the spec on, is whether the exit path is
genuinely lossless-in-rights *in the default case* and not only at the unused margins (§5.7 admin floor,
§7.4 freshness, the no-helper-path obligation of `P-Durable-Enablement`).

*Running example: when Bob is banned he leaves with his full rights intact in his own lineage, which is what
makes the Group's flexibility real rather than a captive dependency (walkthrough beat E7).*

### 5.10. The Group as a principal: communal ownership, composition, and acting on a Group's behalf

> `Realizes: P-Peer-Equality, P-Local-Truth, P-Durable-Enablement`
>
> Cross-references: §5.2 (kinds of principal), §5.5 (roles and capabilities), §7.1 (data model), §7.6
> (fork), Part 1 §2.3 (recursive principal-is-a-Group), Appendix C (Meadowcap communal/owned).

A **Group is a principal** (§5.2): a collective that can hold a Group Role, own artifacts, be granted to, and be
referred to, a composable unit, not only a key-agreement context. This subsection fixes how that works,
because it answers a question a single-layer model cannot: **when a Group forks, who owns the shared
artifacts?**

**The Group-principal lives above MLS, in the artifacts, not in the key layer.** The MLS group is the
**communication-and-safety substrate** (a set of clients sharing a key, with forward secrecy and
post-compromise security). The **Group-principal** is an **application-layer identity**, a Meadowcap
**communal namespace** (§7.1); that *corresponds to* an MLS group for communication but is **not defined
by it**. They share a membership set; they are different objects at different layers. MLS answers "who
shares the key and can read"; the Group-principal answers "who owns this, whom may this Group grant to,
what does this Group's authority cover." This separation is deliberate and is the only one consistent with
MLS's own design, which provides no notion of a Group acting as a grantor of access (§10.2).

**Authority is communal, not apex, which is what lets it survive a fork.** Meadowcap offers two models:
**communal** namespaces, where authority comes from owning a subspace key and all members hold equal
authority with no one holding the whole, and **owned** namespaces, where a single keyholder has total
top-down control. Drystone's Group-principal is **communal by default**, because communal authority *is*
`P-Peer-Equality` expressed at the data layer: horizontal, no apex, each member writing into their own
subspace. The **owned** model is the apex Drystone rejects for Group governance, though it remains
legitimate for the narrow case of a single author owning a sub-namespace of content they alone created
(the boundary between governing-the-Group and owning-your-own-data: Group governance is communal, while a
single author's own sub-content may be owned, §5.3).

**Worked mechanism, the forked artifact.** Three personae collaborate on a document by automatic merge
(a convergent, monotonic data structure, §7.1). They disagree in a way that is a genuine standing
contradiction, and the Group forks (§7.6). Who owns the document?

- **Both layers own it, and that is why it survives.** The artifact lives in the Group's **communal
  namespace**: the Group-principal owns it *as a collective*, and each contributing persona owns its own
  **subspace** of contributions *as an individual*. Ownership was never solely at either layer.

- **At the fork, both descendant Groups carry the whole artifact**: exactly as an open-source fork
  carries the full repository, not a fraction. Because communal authority was distributed across the
  members all along, the fork does not orphan the artifact (there was no center to sever) and does not
  shatter it into private fragments (the communal namespace is the shared object). Each fork continues
  with a complete copy and full standing to evolve it independently.

- This is the data-layer face of *fork-not-verdict* (§7.6): the system does not adjudicate who "keeps"
  the document. Both do. The fork is the dignified exit, and the artifact comes along on both sides.

**Composition, a Group-principal can be a member of another Group-principal.** Because a Group is a
principal, the structure nests: a **persona's own devices are a Group of clients**, a **community is a group
of people** (whose personae, or whose sub-Groups acting as principals, are the members), a **federation is a
grouping of communities**. (The case follows §5.0 and Part 1 §2.3: the social body is lowercase group, the
same body acting as an in-system principal is the capital-G Group.) Each layer is a communal namespace
with a referable identity, each ownable and grantable, each forkable with its artifacts intact. A
Group-principal can therefore be granted a capability (§5.5), hold a Group Role, or be referred to as a unit,
which is what makes "stand on a Group as a composable identity" concrete rather than aspirational.

**Acting on a Group's behalf is itself a governed Group Role.** A Group cannot sign; some principal must act for
it. The authority to **act-for-the-Group**, to issue a capability, delegate, or make a grant in the
Group's name, is a **Group Role** (§5.5) granted by member consent, scoped, and **revocable** under the same
threshold authority as any other Group Role (§5.7), with the §7.6 fork as the ultimate backstop. This is the
same anti-entrenchment discipline as admin, one layer up: no principal holds the act-for-the-Group Group Role by
structural right, and no grant of it may make itself irrevocable.

**The recursion bottoms out, and that is what keeps weight honest.** Composition could otherwise be a
laundering path for governance weight, a principal pooling many sub-Groups to manufacture standing. It
cannot, because **weight (§5.6) is anchored at the leaves: flat, one per distinct persona, never
minted, only delegated.** However deep the composition, the total weight in any Group reduces to the count
of distinct personae (by lineage) at the bottom. A Group-principal's weight in a parent Group is
*defined by its members' delegated weight*, conserved through every layer, never inflated by the act of
composing. So the Group-as-principal model gives Drystone composable collective identity **without**
opening the door composition would otherwise open, because the personhood-anchored leaf is the floor of
the recursion.

> **Two seams left open here, stated rather than smoothed.** *(1)* The precise **identity construction for
> a Group-principal**, the communal namespace key and how it is established and rotated as membership
> changes, is designed-not-frozen (the §5.2 open seam): the shape is a communal namespace rather than a
> derived central credential. The motivation is concrete: when forking and merging are cheap
> but a Group is collaborating on a shared asset, honoring a fork requires the asset to be owned jointly by
> the clients (and so the personae) *and* the Group, so both forks carry the whole thing like an open-source
> repository fork. A Meadowcap **communal namespace** fits that model well, which is why the Group-principal
> is shaped as one. What is unworked is the **key rotation scheme** (how the Group and its members jointly
> own the namespace and how the key rotates under churn) and whether the communal namespace is **primary**
> (the Group-principal *is* a communal namespace at all times) or **secondary** (established only at a fork
> or merge, when joint ownership has to be made explicit). The decisive next step is to **dig into Meadowcap
> and check its alignment with MLS**, whether Group-associated assets can fork and merge sanely across the
> two layers, before committing the construction. This is a Drystone construction question, not a gap in
> Meadowcap: Meadowcap's communal-namespace semantics are confirmed (below). *(2)* **Cross-Group** grants
> and references (one Group-principal granting to or composing another across trust boundaries) are sketched
> here as the composition model but their wire encoding and the valuation-vs-composition edge (Part 1 §2.3)
> are `[gates-release]`. Both carried to Appendix B.
>
> *Meadowcap grounding, confirmed verbatim against the Willow Protocol spec (willowprotocol.org/specs/meadowcap):
> a capability is "an unforgeable token that bestows read or write access for some data to a particular
> person, issued by the owner of that data"; a **communal** namespace is one where "authority is derived
> from ownership of a given subspace key pair... a horizontal model where all members of a namespace have
> equal authority, and no-one has authority to all data in the namespace"; an **owned** namespace is "a
> top-down model where the owner of the namespace key pair has total control over all data." The spec
> itself frames the choice in governance terms, communal for those to whom centralized control "sounds
> like an uncomfortable level of control and power," owned for those whom a privileged shaping group "makes
> feel safe", which is exactly Drystone's peer-equal-vs-apex mapping.*

---

### 5.11. Read-scoped content keys: how a read role is cryptographically enforced

A read Group Role (§5.5) is a permission; this subsection is how that permission is enforced on durable
content. It is needed because MLS enforces no application access control (§5.5, RFC 9750 §6.4), so the read
role has no cryptographic teeth on its own, and because forward secrecy and readability-by-future-holders
cannot come from one key system, so the plane that serves read scope must be keyed on its own terms.

**Content moves on three keying planes, deriving membership from one identity.** A Group's content has three
distinct key needs, and treating them as one is the failure this split prevents. Live-latest delivery is
linear, forward-moving, and forward-secret, and its single agreed current epoch is what MLS's ratchet
provides. History convergence does no key agreement at all; it reconciles already-sealed bytes (§6.8.1).
Read-scoped decryption is the plane the asset key serves: a persona granted a read Role after content was
authored **MUST** be able to read that content, and a persona removed from the Role **MUST** be excluded
from content authored after the removal folds. The read-scoped plane **MUST** be keyed separately from the
live plane, because the live plane wants a later joiner to gain no old keys while the read-scoped plane
wants exactly the opposite for durable assets that are often un-renderable without their predecessors, and
no single key system delivers forward-secret-linear and readable-by-future-holders at once. All three planes
**MUST** derive their membership from the one truth, the persona lineages and the governance fold that
issues capabilities and evaluates thresholds (§5.5, §5.7), so that the split is three key derivations over
one roster, never two rosters. `Synthesis.`

**The per-scope asset key, and why scopes are siblings, not a hierarchy.** Read-scoped content **MUST** be
sealed under an asset key wrapped to each current holder of its read-granting Group Role. Each read scope
**MUST** have its own asset key wrapped to its own folded set, and a narrower scope's key **MUST NOT** be
derived from a wider scope's, because a derivation any wider-scope holder could reproduce would make the
narrower scope a fiction: every member holds the all-members communal key, so if an admin-log key were
`KDF(communal_key, ...)` every member could compute it. A Group's communal key and a narrower moderation-log
key therefore coexist as siblings, each enforced only by wrapping to fewer holders. `Design.`

**The asset key gates read, not write.** The asset key seals payload, so it governs who may decrypt and
nothing else. Write authority is separate: it is a capability issued under the read-granting Role (§5.5),
held per persona, with each contributor authoring into its own subspace under its own key. A read-scoped
asset is thus the union of per-author, individually attributable entries sealed under one shared read key,
and **MUST NOT** be a shared authoring secret, because a shared authoring key would collapse attribution and
defeat §7.2 R6, could not be revoked from one holder without re-keying all of them, and would let a removed
holder who retained it forge entries indistinguishable from a current holder's. `Design.`

**Provisioning is fold-gated, and the two membership operations are asymmetric.** Key wrapping and rotation
**MUST** be a downstream convergent consequence of the governance fold (§7.3), delivered out of band through
the same channel as history convergence, and **MUST NOT** be an inline act at the moment of a grant or
revocation, because at fold time the Role-holder set is deterministic and every key operation is computed
against it rather than a partial view. An eager scheme that rotates at the moment of the act races the fold
and fails under concurrency in two ways: a member added on a branch that later loses a same-subject conflict
keeps a key it should never have held, and two concurrent removals resolved by naively picking one winner
silently drop the losing rotation, leaving a just-removed member able to derive the current key. The two
operations are asymmetric, and that asymmetry is the mechanism. An **add** wraps the current asset key to
the new holder, with no rotation, commutative with concurrent operations. A **remove** **MUST** mint a fresh
asset key carrying new entropy the removed party lacks, wrapped only to the folded remaining set, and **MUST
NOT** derive the new key from the old, because the removed party held the old key, so a derivation of the
form `K' = KDF(K_old, frontier)` is computable by them and breaks §7.2 R5. `Design.`

**Quiescence is a predicate, not a timer, and concurrent re-keys are benign.** The re-key that follows a
removal fires when a membership frontier is quiesced for a node, which holds when the node has both
completeness, every operation causally concurrent with the batch with no nameable gap (checked via §6.8.1),
and freshness, the same frontier head corroborated by at least the threshold of distinct persona lineages
(§7.4, §5.7). Neither term is a duration, and local elapsed time enters only as a private input to
freshness, never as a shared clock and never to order anything. Two remaining members may both mint a fresh
key at quiescence; this is benign, because every candidate is wrapped to the same deterministic folded set
and so already excludes the correct members, the Group converges on one candidate by the §7.3.1
content-address tiebreak, and content sealed under a superseded candidate stays readable by the folded set
and is never re-sealed. `Design.`

**The re-key is triggered by the fold, never by the epoch, and delay under partition is safe.** A
read-scoped asset re-key **MUST** be initiated by a governance-fold membership change and **MUST NOT** be
initiated by an MLS epoch advance, because the epoch tracks key-change need, so a post-compromise rotation
advances it, while authority is read only from the governance chain, and triggering on the epoch would both
re-key wastefully on a routine rotation and, worse, fail to re-key on a governance removal not yet coincident
with an epoch tick, leaving a removed holder still provisioned. For the same reason a node **MUST NOT** infer
authority state from the epoch, because epoch equality does not imply authority equality. When the quiescence
predicate does not hold, under partition or isolation, no re-key fires and authors continue under the last
stable key, and that delay is safe rather than a breach because R5 (§7.2) is causal, not temporal: the only
content a removed holder can read is content causally concurrent with its own removal, which it could already
read, so however long a partition lasts it can never expose content causally after the removal. The cost of
partition is therefore liveness, the forward-exclusion taking effect later, and never safety. `Design.`

**No continuous group key agreement is required here.** The read-scoped plane needs no separate continuous
key-agreement protocol beyond MLS's key schedule for the live plane and this fold-gated asset key for the
read-scoped one, because the live plane's forward secrecy is already MLS's ratchet and the read-scoped
plane's requirement is the opposite, readability by future holders, which the wrapped, fold-gated asset key
delivers directly. Adding a continuous group-key-agreement layer would be a second key system solving a
problem the plane split has already dissolved, at the cost of a second soundness dependency. `Synthesis.`

**The security meaning of durability.** The read-scoped plane is deliberately not forward-secret for the
assets it holds: a granted Role-holder holds the asset key and can read all content under it, and a
later-compromised key exposes that history. This is intrinsic to being entitled to history, since a durable
asset is often un-renderable without its causal predecessors. The consequence to state where a reader would
assume otherwise is that forward secrecy is meaningful only for content that stays on the live plane, so a
Group's choice of what is durable-and-read-scoped versus ephemeral is a security decision and not only a
storage one, and it maps onto the §7.7 mode axis. `Synthesis.` `[confirm: the causal-history-access model
against Ink & Switch Keyhive and the p2panda access-control model.]`

*Running example: the Group seals its shared album under one communal asset key wrapped to every member;
when Bob is removed, the next photo is sealed under a fresh key wrapped only to the folded remaining set, so
Bob keeps what he could already read and is shut out of everything authored after his removal folds
(walkthrough beat E5).*

---

## 6. Transport and Delivery: the Three Planes, Identity, and the Encryption Stack

> `Realizes: P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement`
>
> Cross-references: §3.1 (where adjudication lives), §4.5.1 (per-client membership), §5.4 (the meer and the
> relay, two distinct blind roles), §5.9 (the meer is optional; exitability), §7.4 (freshness and the
> returning-member catch-up), §7.5 (attributable acceptance), §8 (the relay/meer as blind forwarders),
> §10.2 (the MLS requirement), §10.3 (the transport/overlay requirement). External transport facts (iroh
> core / QUIC) are verified against the released iroh 1.0 primaries this round.
>
> **Requirement vs realization.** This section reasons about the *shape, handling, and requirements* of
> transport and delivery independently of the named protocols that currently realize them, then names the
> realization (iroh, MLS, HyParView/PlumTree) with enough specificity that a reader can tell what is
> load-bearing from what is incidental. The abstract transport requirement (T1–T7) and the abstract
> Group-key requirement (K1–K8), with the compliance bars a non-iroh or non-MLS candidate must meet, are
> consolidated in §10.3 and §10.2. A protocol meeting the same requirements is swappable for the named one.
> The clean demonstrative of why this separation matters, carried through the section: MLS's own spec
> includes an *ordering* function at its Delivery Service, and Drystone references MLS heavily and
> appropriately, yet **declines** that function, because ordering is already carried inside the sealed
> messages (§6.6, §7.4). "Present in the realization" is not "required by the model," and the DS ordering
> role is the standing example.
>
> **Verification legend.** *Verified*, checked against the cited primary this round; **[confirm]**,
> load-bearing and version-dependent or not yet pulled from the cited primary this round. **iroh reached
> 1.0 (June 15, 2026), and the version split is itself a differentiator this section relies on.** iroh core
> (the `Endpoint` / `Connection` / `Router` / ALPN surface, the QUIC + TLS 1.3 transport, the relay,
> key-based addressing) is wire-and-API-stable under the 1.0 guarantee, so the transport-plane claims here
> are pinned, not provisional. The overlay and discovery layers Drystone uses are **separately-versioned
> crates that the 1.0 guarantee does not cover**, `iroh-gossip` (pre-1.0, its own repo and release cadence)
> and the address-lookup crates (`iroh-mainline-address-lookup`, `iroh-mdns-address-lookup`); their internal
> specifics are therefore still **[confirm]** against each crate's pinned version even though iroh core is
> stable. Each flag below says which layer it belongs to.

This section specifies how Drystone peers find each other, how sealed messages move between them, and what
each layer does and does not protect. It is built against the two substrates §10 names: **MLS** (RFC 9420 /
RFC 9750) for Group key agreement and message protection, and **iroh** for transport, discovery, and the
gossip overlay.

The organizing idea, and the spine of this section, is that **delivery is not one choice but three
independent questions**, and treating them separately is what lets mechanisms a flatter model would cast as
rivals be *combined*, each doing the job it is good at:

- **Carriage (Plane C):** by what path does a sealed message travel from author toward recipient? A direct
  dial, the gossip overlay, or a relay. (§6.5.)

- **Durability (Plane D):** when the recipient is not reachable right now, where do the sealed bytes persist
  so they can be pulled later? The participants themselves, a store-and-forward node, or fellow members.
  (§6.6.)

- **Presence (Plane P):** who learns that a sealed message for a given recipient exists, so they can prompt
  that recipient to fetch it? Nobody (the recipient polls), a carrier, a holder, or a push service. (§6.7.)

A delivery arrangement is then a *pairing*: one carriage path, one durability source, one presence source.
The planes are **independent axes** paired freely (any carriage with any durability with any presence),
**except where one mechanism serves two planes by construction**, which the text flags at each occurrence.
Within a plane the sources are **non-exclusive**: more than one can be active at once, racing, because a
sealed MLS message is byte-identical whichever path carried it and duplicates deduplicate on their content
hash (§6.6.4). When Bob is briefly offline, the swarm may carry a message live (carriage) while a meer holds
the durable copy (durability) and a wake nudges his phone when he is reachable (presence): three answers to
three questions, not one coupled decision.

Keeping the planes separate lets the spec state each plane's real character without contradiction: the
gossip swarm is a *carriage* path that provides no durability, and separating carriage from durability is
what lets that be said plainly (the swarm carries; it does not persist), instead of the plane's nature being
obscured by filing it under durability. Where points *are* fused by construction, the discipline is to say
so: **D-self is also C-direct** (participants who hold the buffer also deliver it, one act on two planes),
and **the meer is one node on three planes** (it persists as D-meer, is carry-fetched from, and can poke as
P-meer); **C-swarm, by contrast, is fused with nothing durable**. Naming the fusions keeps "these two planes
happen to be one mechanism here" distinct from "these are independent choices."

The section is ordered so the substrate precedes what it produces. **Identity (§6.1) and the encryption
stack (§6.2) come first, and the Delivery Fabric (§6.3) and the observer picture (§6.4) follow as their
consequences**: a carrier is blind *because* content is sealed to the Group's epoch, and observers see so
little *because* identity is split into two planes and the payload interior is sealed. **Discovery** (§6.9),
resolving a peer key to a location, is a distinct concern given its own section after the planes; it is the
one part of this section the spec treats more lightly than it will ultimately need, and §6.9 says so.

### 6.1. Two planes of identity

Drystone has two identity planes, and they answer two different questions. Keeping them separate is not
pedantry; it is the mechanism realization of the composition-vs-valuation distinction (Part 1 §2.3), and it
is the same seam the whole delivery design rests on: the plane that authenticates a *channel* is not the
plane that authorizes an *actor*.

#### 6.1.1. Peer-level identity (the transport plane)

A peer is addressed by a long-lived public key, an iroh **`EndpointId`** (the public half of an Ed25519
keypair). This key is the peer's network identity: you dial a key, not an address, and iroh resolves the
key to a current network location on demand (discovery, §6.9). *(Verified against iroh 1.0: each endpoint
holds an Ed25519 keypair whose public half is the `EndpointId`, and that key is also the endpoint's TLS
identity, so it cannot be impersonated. `EndpointId` is the iroh 1.0 term; iroh's own pre-1.0 material uses
`NodeId` for the same thing.)*

What this plane establishes is **provenance of the channel**: when a connection opens to an `EndpointId`,
the transport's TLS 1.3 handshake authenticates that the peer on the other end holds the private key for
that `EndpointId`. This is a certain, computable fact (Part 1 §2.0): the connection either authenticates
the key or it does not.

What this plane does **not** establish is *who that key belongs to in human terms*. The binding
"this `EndpointId` is my co-organizer Dana" is a **utility judgment** supplied by a human act (a QR scan, an
in-person exchange), exactly as Part 1 §2.0 argues, and exactly the principal-as-its-own-CA act of §4.5.1.
The transport plane carries that binding forward once a human has made it; it never computes it.

#### 6.1.2. Group-level identity (the MLS plane)

Membership in a governed Group is a separate fact from peer reachability. In MLS, a member is a **leaf** in
the group's ratchet tree, holding the group's continuously-rotated key material. RFC 9420 §2 defines a
member as a client included in the shared state of a group, with access to the group's secrets, and an
epoch as the state of a group in which a specific set of authenticated clients hold shared cryptographic
state. *(Verified-RFC, RFC 9420 §2 terminology.)* A peer's `EndpointId` says it can be *reached*; its MLS leaf
says it is a *member of this Group at this epoch*. This is the wire form of the §4.5.1 model: each
**client** is its own MLS member (one leaf, one signature key, one credential), and a principal's clients
are folded to one persona by lineage (§4.5), never by sharing a key.

These planes are deliberately decoupled. A single principal may hold several `EndpointId`s (a phone, a
laptop, the device-pool composition edge of Part 1 §2.3) that together act through several clients in one
Group, and the same person may be a member of many Groups under different presentations (the
multiple-presentation argument of Part 1 §2.3 / §5.6). Peer identity is per-device-key; Group identity is
per-membership. The recursion of Part 1 §2.3 (device → user → community) lives here: composition at the
MLS plane (shared Group key) is distinct from valuation across planes (one Group weighting another's
assertions with no shared key).

> **The seam, stated plainly.** Peer identity authenticates a *channel*; Group identity authorizes an
> *actor in a Group*. A correctly-authenticated channel from a peer who is not a member of Group S grants
> nothing in S. A member of S whose channel cannot currently be authenticated is still a member; you simply
> cannot talk to them right now. **Reachability and membership fail independently**, and the design **MUST
> NOT** treat "I have a verified connection" as "this peer may act here." This is the §4.4 separation
> (integrity-and-ordering vs authorship-and-standing) seen from the transport side, and it is why the
> Delivery Fabric (§6.3) can be a blind carrying commons: carrying is a channel fact, reading is a
> membership fact, and the two never collapse into each other.
>
> The reference transport reinforces the seam by construction: in iroh the remote `EndpointId` is known
> only *after* the mutual-TLS handshake completes, so a peer is admitted to the channel before any
> Group-membership question is asked, and the membership check is necessarily a *later, application-layer*
> step. There is no point at which "channel authenticated" and "may act in S" are the same event. *(Verified
> against iroh 1.0: a `Connection` can only be constructed after successful handshake and authentication,
> and `Connection::remote_id()` is infallible precisely because the connection it is called on is already
> authenticated; the application then decides separately whether that identity may act in the Group. This is
> stable 1.0 core API, not a provisional detail.)*

### 6.2. The encryption stack: two layers, different jobs

Calling this "double encryption" is close but imprecise. There are two encryption layers, but they are not
redundant wrappings of one secret; they protect **different things**, and the security argument depends on
knowing which protects what. This is the substrate the rest of the section is an outcome of: it is because
Layer B seals content to the Group's epoch that the Delivery Fabric (§6.3) can carry blind and the observer
exposure (§6.4) stays narrow.

*Running example: Alice's message to Bob rides the transport layer, which hides it on the wire, inside the
message layer, which keeps it unreadable to anyone outside the Group (walkthrough beat E2).*

RFC 9750 states the division of labor directly, and it is sharper than "the transport adds security":
*the security guarantees of MLS do not depend on the transport*, MLS is designed to hold even against a
compromised transport, especially a compromised Delivery Service. The transport is there for a
**complementary** job: RFC 9750 §8 puts it as the transport keeping metadata private from network
observers, while MLS provides confidentiality, integrity, and authentication for the application data
itself, which may pass through multiple untrusted systems. *(Verified-RFC, RFC 9750 §8, primary text this
round.)* So the layers do not stack one security claim on another; each covers a gap the other leaves
open.

#### 6.2.1. Layer A, transport encryption (iroh QUIC / TLS 1.3)

**Scope:** the *link* between two peers, hop by hop.

**Protects:** the metadata of a single connection, which `EndpointId` is talking to which, and (with
padding) message sizes, from a *network observer on that link*. It also authenticates the peer-level
identity of §6.1.1.

**Does not protect:** anything Layer A would have hidden, once the bytes reach an intermediary you handed
them to. The two intermediary roles see different amounts. A **meer** you dial directly *is* a TLS
endpoint of your connection, so it sees everything Layer A protects on that link (your `EndpointId`, the
timing and sizes of what you deposit or drain), though never Layer B content. An **iroh relay** does
**not** terminate the peer-to-peer QUIC/TLS session; it routes encrypted packets by `EndpointId` and
cannot decode them, so it sees the `EndpointId`-to-`EndpointId` envelope and timing but not even the
Layer A record contents. Either way, the point holds: transport encryption is hop-by-hop, so its metadata
protection is against a *network observer*, not against the intermediary in the path (§6.4 draws the full
by-layer picture). This is exactly why transport encryption alone is insufficient for a center-free design
where messages pass through other peers, and exactly why Layer B is not optional.

RFC 9750's own recommendation names this layer: use transports providing reliability and metadata
confidentiality, such as TLS or QUIC, for carrying MLS messages. *(Verified-RFC, RFC 9750 §8 recommendation,
primary text this round.)* iroh's transport is QUIC with TLS 1.3, so the recommendation is satisfied by
construction.

#### 6.2.2. Layer B, message encryption (MLS PrivateMessage)

**Scope:** the *content*, end-to-end across the whole Group, independent of how many hops or relays it
traverses.

**Protects:** confidentiality, integrity, and authenticity of the application payload, and (when
configured) the handshake, to the Group's current epoch. RFC 9420 §2 defines a `PrivateMessage` as signed,
authenticated as coming from a member in a particular epoch, and encrypted so that it is confidential to
the members of the Group in that epoch; an application message is a `PrivateMessage` carrying application
data. *(Verified-RFC, RFC 9420 §2 terminology.)* A `PrivateMessage` is AEAD-sealed under per-sender,
per-message keys derived from the ratchet. Anyone who is not a member at that epoch, including every relay,
meer, and gossip-forwarding non-member, sees ciphertext only.

**Provides what transport cannot:** end-to-end protection that survives passing through untrusted
intermediaries. This is the layer that makes the blind-relay (§6.5.2) and blind-meer (§6.6.2) roles safe.
A meer holds `PrivateMessage` bytes it cannot read precisely because Layer B sealed them before they ever
reached Layer A.

**Provides, uniquely:** forward secrecy and post-compromise security across epochs (§10.2 K1, K2), because
MLS actively deletes and replaces keying material as the Group advances. Transport encryption gives
session forward secrecy; MLS gives *Group-history* forward secrecy that survives membership changes.

**Carried, not bound.** Drystone rides its content structures, the sealed envelope and the content-addressed history entries, inside `application_data` as opaque bytes, and does not bind them into MLS's own authenticated group state. MLS keeps two internal trees, the ratchet tree and the transcript-hash chain (§8.2) with its epoch authenticators (§8.7), and those commit to the handshake history of Proposals and Commits, not to application content, so a content root cannot be folded into the transcript through the framing path. *(Verified-RFC, RFC 9420 §8.2.)* The design does not need that binding: what it takes from MLS is confidential, integrity-protected, member-attributed, epoch-scoped exchange of arbitrary bytes, which is exactly the `PrivateMessage` guarantee over `application_data`, and content order is supplied by the content-addressed chains themselves (§7.1, §7.7), not by MLS, which gives no total order over application messages *(Verified-RFC, RFC 9420 §15.3)*. The `authenticated_data` field is authenticated but not encrypted, so the envelope rides in the sealed `application_data`, never in that field. MLS's exporter (§8.5) remains available should a later construction need a value cryptographically bound to group state, but the current design does not use it. *(Design for the choice not to bind; the carriage path itself is `Verified-RFC`.)*

#### 6.2.3. Why both, and what neither gives

The layers compose because they cover complementary gaps:

- **Transport-only (Layer A alone)** would expose content to every intermediary peer, fatal in a
  center-free topology where peers relay for each other.

- **Message-only (Layer B alone)** would protect content end-to-end but leak the full contact graph and
  timing to any network observer, because the ciphertext would travel over an observable channel.

- **Together:** Layer B keeps content secret from intermediaries and the network; Layer A keeps the *fact
  and shape* of a given link's traffic secret from a network observer on that link. Content secrecy is
  end-to-end; metadata secrecy is hop-by-hop.

The by-layer observer picture (who sees what, at the overlay, iroh, and IP layers) is drawn in §6.4; the
two blockquotes below are the complementary MLS-specific residue, what the *seal itself* does and does not
conceal regardless of transport.

> **The honest limits, neither layer defeats traffic analysis, and metadata is only hop-private.** Three
> cautions the design carries rather than papers over.
>
> *First, MLS minimizes but does not eliminate metadata.* RFC 9420 §16.4 lists fields it does not protect:
> the unencrypted header fields of a `PrivateMessage`, the lengths of encrypted messages, anything sent as
> a `PublicMessage`, and the `KeyPackage` / `GroupInfo` / `Welcome` distribution. From these a party can
> infer the Group ID, the current epoch and the frequency of epoch changes, message frequency within an
> epoch, Group extensions, and membership. §16.4.1 is explicit that MLS provides no mechanism to protect
> the Group ID and epoch from the Delivery Service, so those and the change frequencies are not protected
> against DS inspection, though any modification to them causes decryption failure. *(Verified-RFC, RFC 9420
> §16.4 / §16.4.1.)* The named adversary is the DS and a party observing the unprotected header fields, not
> a generic observer of the ciphertext.
>
> Note what is **not** exposed: the per-sender `generation` counter rides inside `SenderData`, which is
> AEAD-encrypted (RFC 9420 §6.3.2), and RFC 9420 §16.3's purpose is precisely to conceal which member sent
> a message. So sender identity within the Group is protected in the framing, and there is no RFC-sourced
> claim that an outside observer can detect a missed message from a counter gap. *(Verified-RFC, RFC 9420
> §6.3.2 and §16.3.)*
>
> *Second, transport metadata privacy is hop-by-hop, not end-to-end.* A relay carrying the fallback path,
> or a meer you dial directly, sits *inside* Layer A and sees the contact metadata for the traffic it
> handles. It still cannot read content (Layer B), but "metadata-private" means private from the *network*,
> not from the *intermediary you handed the bytes to*. This is inherent to hop-by-hop encryption, and it is the
> §6.4 relay/meer exposure seen from the encryption side.
>
> *Third, neither layer conceals packet timing, sizes, rate, and bursts from a determined traffic-analysis
> adversary.* TLS 1.3 provides a length-concealment *mechanism* (record padding) but no guarantee against
> traffic analysis. *(Verified this revision: RFC 8446 §5.4 states TLS records may be padded to obscure
lengths and improve protection against traffic-analysis techniques but does not hide transmitted data
length; the traffic-analysis residue is corroborated by arXiv:2406.15686 §6.2, which restates exactly this
RFC 8446 limitation.)* For the
> high-threat dials of Part 1 §2.3 (activist, journalist), this is the residue that needs additional
> measures (cover traffic, mix routing) outside the scope of this section.

> **Membership inference, and the mitigation the RFC names.** Of the metadata exposures, Group membership
> is the most consequential for the high-threat dial, and RFC 9420 §16.4.3 is precise about it: membership
> is represented directly by the ratchet tree, so exposing the tree leaks membership; `Add` / `Remove`
> proposals sent as `PublicMessage` leak membership changes, and a party seeing all changes can reconstruct
> the membership; and a party holding a pool of `KeyPackage`s who observes a `Welcome` can identify the new
> member. *(Verified-RFC, RFC 9420 §16.4.3.)* The critical caveat dovetails with the multiple-presentation
> argument of Part 1 §2.3: these leaks reveal membership only to the degree a member's `LeafNode` is
> identifying, and the RFC offers the explicit mitigation of pseudonymous credentials with frequently
> rotated encryption and signature keys. So the protocol-level defense for the activist/journalist dial is
> to keep the `LeafNode` minimally identifying, a deployment choice the spec sanctions, not a gap. (This is
> the same client-correlation cost §10.2 logs against K5, named there as an accepted tradeoff.)

### 6.3. Two populations: the Delivery Fabric and the Group

With sealing established (§6.2), the first outcome can be stated: a separation orthogonal to the three
planes organizes the rest of this section, and it is *made possible* by the seal. **The set of nodes that
carry sealed messages is independent from, and can be larger than, the set entitled to read them.**

- The **Delivery Fabric (DF)** is the *carrying* population: a blind, content-agnostic overlay (gossip,
  direct links, relays) that moves sealed messages. It can be larger than any one Group and can overlap
  many Groups, a shared carrying commons. A node on the fabric sees ciphertext and routing metadata at
  most, never content, and it sees that little precisely because Layer B sealed the payload before it
  entered the fabric. This is where a non-member's devices can sit: nodes that carry, holding no key.

- The **entitlement population** is the **Group** (§5): who holds the leaf keys to read, and who may be
  asked to reconcile history. **Membership, not position on the fabric, is what authorizes reading and
  reconciliation.**

The Delivery Fabric is the concept the earlier sections point to when a helper is said to "sit in the
Group's scope but not in the Group" (§5.4). Stated precisely now that the fabric has a name: a Group's
**scope** (§5.4) is its *exposure envelope measured over the Delivery Fabric*, the whole reach of routing
and metadata across the carrying population, of which the gossip topic is one large contributor and each
helper in the path (relay, meer, push-notify node) is another (§4.2). The fabric is the substrate; the
scope is how much of that substrate a given Group's sealed traffic touches. This is also why a single meer
or relay can sit in the scope of several Groups at once: its position is a fact about the shared carrying
fabric, not a grant from any one Group (§5.4, and the fabric-level-fact vs per-persona-use distinction
there).

This split lets the carrying network scale for robustness without ever widening who can read. A larger
fabric means more paths, more redundancy, better reach to a hard-to-reach recipient, and none of those
extra carriers gains any ability to read, because reading is gated by holding the Group leaf key, not by
fabric position. A non-member node can relay for a Group all day and learn nothing, because it holds no
leaf key for that Group.

The **sealed message** is the single object that crosses between the two populations, and it does three
jobs at once: it is **payload** for those entitled to read it, **delivery** for its recipient, and
**gap-definition** for an entitled holder who reads it, because inside the seal it carries its author's
signed sequence index (§6.8, §7.5). One artifact, three jobs. This is why the design needs no separate
"a message exists" announcement channel: the message's own arrival is the announcement, and its sealed
index, read by members, is the record of what should exist. (The correctness reason a *separate* announce
channel would be worse, not merely redundant, is the time-of-check-to-time-of-use argument in §6.7 under
P-gossip.)

This decoupling is not new: Hyperledger Fabric ships the same shape (membership-scoped channels riding a
larger gossip overlay), and the epidemic-dissemination lineage goes back to Demers et al. 1987. What
Drystone does differently is keep it center-free in ordering and governance, and make entitlement
cryptographic rather than a routing policy. (Appendix C / the provenance companion traces the full lineage
and is explicit about where the novelty is and is not.)

### 6.4. What observers can see (the metadata floor)

The second outcome of the substrate: because content is sealed (§6.2) and identity is split into two planes
(§6.1), the routing envelope and the sealed interior are different objects, and the difference bounds the
entire metadata exposure.

*Running example: a relay carrying Alice's frame sees the endpoints, timing, and size, never the content,
and is never required to read it (walkthrough beat E2).* For the fabric to carry a message to a Group's subscribers, the **topic
identifier must be readable**. This is not a Drystone concession; it is the irreducible floor of *any*
routed delivery system: a flat server reads recipient addresses, a federated server routes by domain, a
broadcast overlay reads a topic. You cannot route to a destination you cannot name. Drystone exposes the
*minimum* routable selector (a topic identifier) and seals everything else; the only way to expose less is
to route every message to everyone, which general messaging does not ship.

The **author identity and the author-signed sequence index are sealed inside the payload**, readable only
by members after decryption. This ties two claims together: because the index is sealed, a member reads it
to detect gaps (§6.8, §7.5), and a blind carrier, including a meer, cannot read it to order or attribute
the blobs it holds (§6.6.2). "The meer cannot order" and "a carrier cannot infer per-author gaps" are one
consequence of one decision, the index lives inside the seal.

The adversarial picture is best drawn **by observer**, because observers sit at different layers, see
different things, and a single attacker rarely occupies more than one:

- A **swarm member** observes at the **overlay layer**: a subscribed node forwarding along the broadcast
  tree, seeing the traffic through *its* part of that tree, the topic identifier and the sealed blobs on it.
  An epidemic overlay gives no node the global roster or flow, so this is a *partial, local* picture: the
  shape of activity in its neighborhood (volume, timing, sizes, distinct-blob counts via the content-hash
  dedup key, §6.6.4). It knows *which Group* but sees only its slice, and it is content-blind (sealed) and
  attribution-blind (author and index sealed).

- A **relay** observes at the **iroh layer**, as a handoff-and-tunneling helper endpoints deliberately use
  to hole-punch or to carry traffic when no direct path exists. iroh's relay forwards datagrams addressed to
  a destination EndpointId, so it can see that one EndpointId is talking to another and how many bytes pass,
  but, by iroh's own account, only until the two endpoints establish a direct connection, and a direct
  hole-punch succeeds roughly 90% of the time (§6.5.2). So the relay sees a graph of **durable EndpointId
  pairs, but transiently**: it drops out once a direct connection forms, leaving only the pre-hole-punch
  window and the residual fraction of pairs that never go direct, and even then it sees the *pair and byte
  count*, never the topic (sealed in QUIC) and never the content (MLS-sealed inside that). The relay is a
  chosen helper, not a network position. *(Verified against iroh 1.0 / crate docs.)*

- A **gateway** observes at the **IP layer, below iroh**, as a topological chokepoint packets traverse
  because of where it sits (a NAT, a mesh uplink, an ISP egress). It sees encrypted QUIC packets between
  **IP addresses** and nothing above that: not the EndpointId (an iroh-layer identity inside the connection,
  not an IP header), not the topic, not the content. Its graph is over **ephemeral IPs**, which rotate with
  mobility and NAT, are shared behind CGNAT, and are reassigned, so it does not durably attribute to a
  persona or device and it decays as addresses churn. This is the same weak handle any network observer has
  on any encrypted traffic, the universal floor, not a Drystone-specific concession.

These do not combine cheaply, and they fail in opposite ways: the relay sees durable identities but
transiently and pair-only, blind to Group and content; the gateway holds a persistent position but sees
only churning, identity-blind IP flows. The frightening observer, persistent *and* identity-bearing *and*
Group-aware, would have to be the relay for a pair that never hole-punches *and* a swarm member at that
Group's tree positions *and* correlate the two across layers, which a healthy redundant swarm and the 90%
direct-connection rate actively work against. A separate surface is **discovery** (§6.9): resolving an
EndpointId to its address tells the resolver that someone looked up EndpointId X, which is the one place the
default deployment leans on an operated service and which a deployment can opt out of. All of this
transport- and discovery-level metadata is out of scope for what *this* layer undertakes to defeat
(Drystone seals content and in-payload metadata and routes on the topic); it is addressed, if at all, by
lower-layer countermeasures (a private relay and resolver, padding, cover traffic, mixing), which the
design notes rather than provides. The strongest honest summary: a swarm member can build a partial view of
the *shape* of a Group's activity, never its content and never its attribution; broader views require a
chokepoint position that a redundant fabric is designed to prevent.

### 6.5. Carriage (Plane C): the path the message travels

The Delivery Fabric (§6.3) offers three carriage paths. **None of them, by itself, persists anything for an
absent recipient**; persistence is the durability plane's job (§6.6). Carriage is about reaching whoever can
be reached *now*. The three paths are non-exclusive: the selector (§6.8) may attempt several at once and let
the first delivery win, because duplicates deduplicate on the content hash (§6.6.4).

*Running example: Alice's post reaches Bob directly when a path forms, by relay when a direct path will not,
and by swarm when it fans out to the whole Group (walkthrough beat E2).*

A note on discovery, alluded to here and treated in full at §6.9: every carriage path presupposes that the
recipient's key can be resolved to a current location (or that a shared topic is already joined). That
resolution is a distinct concern, handled by §6.9, and this section assumes it has happened.

#### 6.5.1. C-direct (direct peer-to-peer, the most center-free path)

Suppose Alice (a persona running two devices, a phone and a laptop) and Bob (a persona on one device, the
recurring "briefly offline" case below) are both present, on the same network, no internet needed. They
exchange MLS messages directly over QUIC, routed by `EndpointId`, no helper in the loop. This is the most
center-free path in the design and is **first-class, not a degraded mode**. It is also the path half of the
no-helper floor: when participants carry their own traffic directly, carriage and durability are **fused by
construction** into D-self (§6.6.1), one act that both delivers and (with queue-and-retry) persists.
*Verified (direct QUIC
path; the two peers exchange sealed messages with no intermediary).*

The reference transport is **iroh**: encrypted QUIC with TLS 1.3, addressed by `EndpointId`, attempting a
direct hole-punched path first. In iroh's own production figures the direct path succeeds for the large
majority of connections and carries the bulk of data volume, so the relay (§6.5.2) is the exception path,
not the common one. *(Verified against iroh 1.0: direct-first with relay fallback. The success-rate
percentages are iroh's reported production numbers, not a Drystone guarantee, and are cited as corroboration
only.)*

#### 6.5.2. C-relay (relay-assisted, when a direct path will not form)

When a direct path will not form (NAT, mobility, firewall), an **iroh relay** forwards the encrypted packets
so carriage still succeeds. The relay is a **carriage assist, content-blind**: it does not terminate the
peer-to-peer QUIC/TLS session, routes by `EndpointId`, and cannot decode what it forwards. What it can
observe, and for how long (an `EndpointId` pair and byte counts, transiently, until a direct connection
forms), is drawn in §6.4. A relay process **SHOULD** meter and isolate per tenant and **MUST** degrade
*visibly* under stress, never silently. *Verified (NAT path via relay; the relay sees only ciphertext plus
routing metadata).*

The relay is one of the three resource-asymmetry roles (§6.6.2 for the meer, §6.7.1 for push-notify); it
supplies **reachability** and nothing else, is redundant and revocable, and is never an authority. It is a
chosen helper, not a network position: endpoints deliberately use it, and a Group that wants no relay can
run direct-only on a local network (§6.11 Mode 2).

#### 6.5.3. C-swarm (the gossip overlay, for live fan-out with no central store)

Now suppose Alice and Bob are in a larger Group on a local network with no internet, or simply want live
fan-out to many recipients. The **gossip overlay** carries each message across the **swarm** (the population
of nodes subscribed to the topic) along a **spanning tree** built over that swarm: a message is eager-pushed
along the tree's edges and the tree self-repairs as nodes come and go. It is **not a flooded mesh**; the
tree is what keeps the broadcast cheap. This is genuine, efficient delivery, and the right tool when there
is no central store or a Group deliberately wants none.

The mechanism, HyParView for membership and PlumTree for broadcast, is developed in full at §6.10 as the
current *realization* of C-swarm; the requirement C-swarm places on any overlay (an epidemic broadcast tree,
no node holding global membership, blind to content) is stated at §6.10 and consolidated in §10.3, and a
different overlay meeting that requirement is swappable for the named one.

C-swarm's nature, stated plainly because separating the planes lets the spec say it without contradiction:
**gossip carries but does not persist.** It keeps no replay log, so a node entirely offline during a
message's live window recovers nothing from the swarm itself. *Verified (an online node received every
broadcast; a late joiner recovered none).* C-swarm is therefore a carriage path **with no durability-plane
partner of its own** (the one plane-fusion that does *not* happen), so an offline recipient needs a separate
D- source, a meer or peer reconciliation, to recover what the swarm carried past.

> **A Group may run on C-swarm with weak or no durability, a legitimate choice under the dial discipline.**
> Under the Part 1 §2.3 dial discipline, a Group may select C-swarm to avoid an internet-link requirement or
> to tilt away from any central store (P-Durable-Enablement). The one non-negotiable: **a Group may accept
> message *loss*, but the protocol must never allow *invisible* loss.** Loss-tolerance is a Group dial and
> may be loose; loss-*visibility* is floored by P-Knowable-Truth (Part 1 §2.2) and the freshness rule
> (§7.4). The mechanism that keeps loss visible even with no store to query is **gap-aware history
> convergence** (§6.8): each message's sealed signed index lets a member see precisely what it is missing
> from the messages it does hold. A swarm-only Group leans entirely on that, which is why the sealed index
> is load-bearing rather than optional.

### 6.6. Durability (Plane D): where the bytes wait for an offline recipient

Three sources, presented **most-center-free outward**, because the floor is the point: everything above it
is an optimization, never a dependency. Within the plane the sources are non-exclusive and the selector
(§6.8) may race them.

*Running example: while Bob's phone is offline, Carol's node holds the bytes so he can converge them when he
returns, and his own device remains the floor beneath that help (walkthrough beat E3).*

#### 6.6.1. D-self: the floor

Suppose Bob steps away mid-conversation. Alice, or any node on the path, buffers the sealed bytes and
retries when Bob returns. **Durability is supplied by the participants themselves.** Paired with C-direct
this is the fused no-helper cell (one mechanism carrying and persisting), and it is the path Part 1 §2.4
requires to stay real and routinely exercised: a conversation can happen on bare nodes with no purchased
infrastructure (P-Durable-Enablement). Everything else in this plane is an optimization over D-self, and
naming it the floor is what lets every other source be removed without ending the conversation. *Verified
(exactly-once delivery holds down to the D-self floor under every combination of surviving paths).*

#### 6.6.2. D-meer: the default optimization

Now suppose Bob is offline for hours and Alice does not want to stay online holding his mail. A **meer**
(§5.4) is a **blind store-and-forward node** that holds the sealed bytes for Bob and hands them over when he
returns. It is never given a leaf key, so it stores ciphertext and learns nothing; it is revocable and
redundantly held, never a structural dependency. It is Drystone's realization of the **store-and-forward
half** of MLS's Delivery Service. One well-connected meer can fan out to many recipients, cheaper than many
direct dials, so this is the sensible default for an internet deployment. *Verified (Tier-0 meer holds
sealed bytes and serves them on dial-home; admission denies a non-listed node).*

The meer is the standing example of the requirement-vs-realization separation (§6 header note). **It does
store-and-forward but does not order messages, and ordering is not lost by this; it is sourced elsewhere.** A
conventional delivery service orders by acting as a central sequencer clients must trust. Drystone does not,
because ordering is already carried *inside the messages*, in the author-signed causal structure and
per-author index (§7.3.1, §7.4), sealed in the payload where only members can read it (§6.4). So the meer
has no ordering job to do, and could not do it: the index it would need to sequence the blobs is sealed
against it. **Being blind and not ordering are the same fact, and that fact is the sealed index.** Order is
needed, present, and intrinsic; it is simply not the meer's to provide, nor within its power to provide.
This is precisely the MLS DS *ordering* function that Drystone declines while keeping the DS's
store-and-forward function: present in the realization, not required by the model.

Because the meer holds only ciphertext, every MLS guarantee holds against it exactly as against any
untrusted DS. *(Verified-RFC, RFC 9750.)*

> **Design rule, the meer stores byte-identical sealed messages, never re-sealed copies.** Dual delivery (a
> member receiving the same commit once via gossip and once via meer drain) is benign because gossip
> deduplicates beneath MLS and MLS applies commits forward-only and idempotently: a duplicate commit no
> longer matches current state and is dropped, the monotonic-fold property of Part 1 §2.2 / §7.3. But this
> safety holds **only if the meer returns the identical `PrivateMessage` bytes.** A meer that re-encrypts or
> re-frames a message could induce ratchet key/nonce reuse, which MLS does not permit. The meer **MUST**
> store and forward the sealed bytes unchanged. *(This is why §5.4 insists the meer is never issued a key:
> there is no decrypt, so there is nothing to re-seal.)*

The meer is one node appearing on **three planes**, kept distinct on purpose: it persists here as D-meer,
you carry-fetch from it (a carriage act), and it can poke as P-meer (§6.7). Naming the three keeps "one node
happens to do three jobs" separate from "these are three independent choices."

#### 6.6.3. D-peer: the Group as its own replay buffer

Now suppose Bob was offline for a message Alice caught, there is no meer, but Alice is reachable. Bob
recovers it **from Alice directly.** Members are already entitled to all the Group's messages, so two
members reconciling their held history **leaks nothing across the entitlement boundary and adds no new
reader.** Because a real Group has many members, the Group becomes its own **distributed replay buffer**: if
anyone caught it, anyone else can recover it, with no central store. This is the same reconciliation
machinery the device-Group uses (§6.6.5) and the same gap-aware convergence C-swarm leans on (§6.8), pointed
at fellow members. *Verified (a gap detected from a fabric-delivered message is filled by reconciling with
a member reached off the fabric; forged or tampered records are rejected whatever their source).*

D-peer is sound only under **hard invariants**, each keeping the convenience from leaking trust into
authority (Part 1 §2.0 razor, Part 1 §2.3, Part 1 §2.5):

- **Self-verifying records only.** A member accepts a reconciled record only when it verifies on its own
  author signature and folds into the hash structure, never on the partner's say-so. Reconciliation moves
  provenance-carrying records, never derived or asserted state.

- **Dataplane only, never governance.** Reconciliation carries message history. Governance state (who is a
  member, the current epoch) is read from the member's own authoritative MLS view, never accepted as a
  peer's assertion.

- **Corroboration is alignment, not truth.** That several members hold the same record affects *coverage*
  (how likely you are to find it), never *validity* (which is the signature's job alone). Treating a
  holding-count as a truth signal would build the quorum apex the razor forbids.

- **Current membership only, exit is final.** A member may reconcile only the history of the Group it is
  currently in; departure ends eligibility. This is the Part 1 §2.4 exit right read correctly, you leave
  *with* the history you hold and gain no standing to pull more afterward, which **dissolves** the
  former-member history-pull attack surface rather than having to defend against it.

Participation is **voluntary per member**: acting as a sync peer is a service a member opts into, not a duty
the Group imposes. The **Group** sets whether D-peer is allowed at all; within that, members enroll
individually. The one cost D-peer carries that the blind roles do not is that **a sync partner is not
blind**: reconciling reveals a coarse activity pattern to a fellow member, so whether D-peer defaults on is
a **per-Group threat-model dial** (on for small high-trust Groups, opt-in for large contested ones, §6.11,
and the open item in Appendix B).

#### 6.6.4. Racing the sources, for free

Because an MLS message is **byte-identical no matter which path carried it**, the same message arriving by
two routes is recognized as one and **deduplicated on its content hash**. So a recipient can have several
paths and sources attempting at once across both planes, C-direct, C-swarm, D-meer, and D-peer, first
delivery wins, duplicates drop. This is the maximum-robustness posture and it costs nothing in principle: a
message the swarm carried past Bob's offline window is one the meer still holds; a message one member
withholds is one several others are corroborated to hold; a message the meer is slow on may arrive first
over a live swarm path. *Verified (one seal relayed by two paths deduplicates to a single entry; the
selector delivers exactly once down to the D-self floor).*

#### 6.6.5. The device-Group: a persona's own devices, kept in sync

Now suppose Alice's laptop caught a message her phone missed. Her devices should converge so her history is
complete everywhere, without a server syncing them. Drystone handles this with a **second Group**: Alice's
own devices form their own Group whose job is reconciling history among them. It is the durability plane
applied to a persona's own device-pool, and it is a **durability amplifier**, more places any given message
survives, entirely within one entitlement boundary.

**An ordinary Group, scoped by lineage.** Alice's devices are each, independently and equally, leaves in
whatever conversations she is in; MLS treats them as co-equal members, and the design adds no leaf-to-leaf
hierarchy, because inventing one would step outside the flat membership MLS's security analysis assumes.
Their reconciliation Group is **not a special construct or a sub-tier**. It is an **ordinary first-order
Group**, distinguished only by *scope*: its admission is lineage-restricted (below), and all its leaves
belong to one owner. Same machinery, same security analysis. The apt description is a **secondary
history-convergence backplane with a stronger membership story**: a backplane, because its job is
reconciling history across Alice's devices alongside her primary conversations; stronger, because the bar to
join is verifiable cryptographic descent rather than an asserted invitation.

**What actually moves.** The device-Group moves **sealed bytes** among its leaves, exactly as every Group
does. Its leaves decrypt what they receive because they hold the leaf key, but that is simply what
membership is; there is no special "plaintext channel" and so nothing extra to secure. It widens entitlement
to **no one**, which is the entire safety story: every leaf is the same owner, already entitled on each of
her devices, so syncing among them moves nothing across any entitlement boundary. A device *not* in a
conversation holds no leaf key for it and, even handed the sealed bytes, reads nothing. The two convergence
invariants that matter, trust a record only on its own signature and its fold into the hash structure (never
the partner's word), and reconcile dataplane history only (never governance, read from the device's own MLS
view), hold here for the ordinary reason they hold for every Group. *Verified (two co-entitled devices
each decrypt and converge; a non-member handed the same sealed bytes cannot decrypt; a tampered record is
rejected even from a trusted sibling device, and a forged governance assertion over the device channel is
ignored).*

**Admission is where control is worth the most.** A device joins the device-Group like any MLS member,
under a **lineage-restricted policy** by default: the joining leaf must prove cryptographic descent from the
persona's rooting key (§5, and the credential-validation hook RFC 9750 names as application policy). Part 1
Part 1 §2.3 already makes lineage the thing that collapses a person's many devices into one persona of weight one,
and that descent is verifiable. The reason to put the strong check precisely *here* is that a history
holder, once admitted, cannot be made to un-read what it received: **admission is the last moment control is
fully effective**, so it earns the strongest available control, a verifiable proof beats an asserted "this
device is mine." *Verified (a non-lineage credential is rejected by the real credential-validation hook at
commit-build time, before the commit reaches the network).* The honest scope limit: the lineage check
governs *which devices become holders*; it does nothing about a holder compromised later, and nothing about
Alice exporting her own content. It protects the admission decision, not the aftermath, because nothing can
protect content already read (claiming otherwise would be the Part 1 §2.0 failure of trusting math where the real
safeguard was human judgment). The standing defense against a later-seized device is the ongoing ability to
cut *future* convergence.

### 6.7. Presence (Plane P): who tells the recipient to look

Durability decides *where* Bob's mail waits. Presence decides *who tells Bob it is there*. The recipient's
situation sets which applies. The presence paths are **not rivals for one slot**; they compose as
**detector plus actuator** (below).

*Running example: a minimal push is what wakes Dave's phone to come and look for what it missed while it was
offline (walkthrough beat E9).*

- **P-none.** Nobody but sender and recipient. Bob's device catches up on its own schedule, foreground,
  periodic fetch, manual refresh, by **polling a meer** with its cursor ("anything since this position?").
  No wake mechanism and no device-token binding: the most center-free presence posture, and **always
  available as the floor.**

- **P-gossip.** Suppose one of a non-member's devices is a fabric carrier on the topic. The sealed message
  arrives at that node via gossip; it makes no attempt to read it (it holds no leaf key for the Group),
  takes the bare fact of arrival as the signal "a message exists for this Group," fires a content-free wake
  toward Bob, and discards the bytes. **The node read nothing and stored nothing, yet Bob's sleeping phone
  gets nudged.** This works on the stock gossip layer with no extra channel, because presence does not
  require a content-free signal: holding an unreadable blob *is* a perfectly good arrival signal, and the
  carrier simply does not decrypt it.

- **P-meer.** A meer that holds mail for Bob knows it holds it, and can poke his device to fetch. This is
  the meer on its third plane (§6.6.2). In a meer-backed deployment this is the usual detector.

- **P-push.** A **byte-free wake to a sleeping mobile device**, via the platform push service, because a
  backgrounded mobile OS forbids a live connection and only a push can wake the app (§6.7.1). It carries
  nothing of substance; it is an actuator, not a carriage path.

> **Why the design wants no separate "announcement" channel, and it is a correctness point, not a
> preference.** Splitting "a message exists" onto a second channel from "here is the message" creates a
> window where the two can disagree: an announcement with no message behind it (a node stuck waiting), or a
> message with no announcement (a node that thinks nothing arrived). That is the classic
> time-of-check-to-time-of-use race, the same reason it is safer to attempt an operation and handle failure
> than to check-then-act. Letting the message's own arrival be the signal (P-gossip) is **atomic**: there is
> no gap to disagree across. The cost is that a carrier receives a blob it only needed to know existed and
> then throws it away, a little wasted bandwidth, the cheapest thing in the system to spend, in exchange for
> eliminating an entire class of race and an entire second overlay to secure. The inefficiency is the clean
> choice.

**Detector plus actuator.** Something *notices* the message exists (P-gossip: a carrier saw it arrive;
P-meer: the holder knows it stored it; P-none: nobody, the recipient polls), and something *reaches* the
recipient (P-push wakes a sleeping device; a live device needs no actuator; polling is self-actuated). "A
carrier detects, push actuates" is one coherent path. **Which detector applies is set by deployment** (a
meer-backed deployment uses P-meer; a swarm-only deployment uses P-gossip; they are interchangeable where
both exist), and **the actuator is forced by the recipient** (only a push can wake a backgrounded phone, and
poll-a-meer is always the non-push fallback beneath it). *Verified (a backgrounded device presents
identical, complete, ordered history on waking, whether woken by push or by foreground poll).*

#### 6.7.1. The push-notify role, and why it is designed to be minimal

The push-notify node exists because a backgrounded mobile OS forbids a live connection, and only a push can
wake the app. It is one of the three resource-asymmetry roles (relay, §6.5.2; meer, §6.6.2; push-notify
here): all blind to content, revocable, redundant, never authorities, each an instance of the Part 1 §2.3
resource inequality (what a node happens to *have*, here APNs/FCM credentials and a device-token registry).
It is deliberately the **smallest** such role:

It learns only "Endpoint X has a waiting message; wake X," and sends a **content-free wake**, not the
message. Two reasons converge on byte-free. The rights model wants the smallest metadata footprint, so no
ciphertext should pass through a third party that does not need it. And the platform forces it anyway: the
reliable push is the user-facing alert, the invisible silent push is throttled and droppable (§6.7.2), so
any design that needs the push to *carry* content is fragile, whereas wake-then-fetch degrades cleanly to
"catch up on next foreground."

It is **doubly removable.** A foregrounded app holds a live connection and needs no push host at all (the
no-helper path, kept real). A polling device pulls from a meer on its own schedule with no wake, needing no
token binding. The single irreducible cost is the **device-token-to-`EndpointId` binding**, which is
inherently identifying, and because polling avoids it entirely, paying it is **opt-in**: a user who declines
push declines the binding. Keeping the host byte-free is both a smaller footprint and a sharper revocation
story, removing it costs a wake optimization and nothing else, because durability lived in the meer or in
D-self all along.

#### 6.7.2. Mobile wake as a conditional accommodation (not a requirement)

Unlike the transport, overlay, and sealing requirements (§10.2, §10.3), mobile wake is **not a requirement
of the design**. It is a conditional accommodation for one class of node. Most nodes participate in the
overlay and see arrivals directly (P-gossip), so they need no wake; but a backgrounded mobile device cannot
hold constant presence (the OS drops it from the swarm), and for that class an **external** wake lets it
come back and fetch.

The wake **must be able to carry nothing of substance** (the design uses it only as a content-free nudge),
because the wake channel is operated by a platform vendor the design does not trust with content, and
because the channel is throttled and unreliable enough that no design should depend on it carrying anything.
This is why the wake is an accommodation and not a requirement: it is unreliable by nature, so it can only
ever be an *optimization* over "catch up on next foreground," and the polling fallback (P-none) always
suffices without it.

The reference implementation is **APNs and FCM**, the platform-mandated push services. Payload cap is 4 KB
(Apple); FCM matches at 4096 bytes and states its transport is not end-to-end encrypted, so applications
must supply their own E2E, which is exactly why content sealing (§6.2) is what makes riding a push provider
safe: the provider carries, at most, a content-free wake. *(Verified, Apple / Firebase primaries.)*
Silent/background push is throttled and not guaranteed. **[confirm: only Apple's "a few per hour, dynamic"
is pinnable; specific secondary numbers conflict, and the device-side measurement awaits real credentials.]**
The reliable channel is the user-facing alert; the invisible silent channel is the throttled one, which is
exactly why the wake is only an optimization: wake-and-fetch degrades correctly, and the durable fetch path
(§6.6) exists regardless.

### 6.8. Gap-aware history convergence, plane fusions, and the adaptive selector

#### 6.8.1. Gap-aware history convergence (one mechanism, three uses)

Three separate features in this section, C-swarm hole-visibility (§6.5.3), D-peer recovery (§6.6.3), and
device-Group sync (§6.6.5), are **one mechanism at different scopes**, and it is defined once here so the
three point at a single description rather than three that could drift.

*Running example: Dave's returning phone computes a stale-but-honest view from what it holds, then closes
the gap by pulling the entries it missed, never presenting a partial state as the whole current one
(walkthrough beat E9).* Its natural home is the **durability
plane**: it is how a member recovers history it missed, distinct from the meer's store-and-forward (the meer
holds sealed bytes *for* you; convergence is members reconciling what they each *already hold*). But it runs
over the same Delivery Fabric as everything else and reaches across planes, it is also the *detector* that
makes loss visible on a swarm-only Group (a presence-adjacent touchpoint), which is why it is stated here
rather than buried inside one plane.

The mechanism has two beats:

- **Detect a nameable gap.** Each sealed message carries, *inside the seal*, its author's signed sequence
  index (§6.4, §7.5). A member reading its held messages therefore knows each author's high-water mark and
  can name precisely which positions it is missing, from the messages it *does* hold, with no store to
  query and no announcement channel. This is why loss stays *visible* even on C-swarm with no durable source
  (§6.5.3): the record of what should exist is the set of sealed indices the member already has. A blind
  carrier cannot compute this, because the index is sealed against it, the same fact that denies the meer
  ordering (§6.6.2).

- **Fill it from a self-verifying source.** The member reconciles the missing positions with any source that
  can supply them, its own other devices first (§6.6.5), then fellow members (§6.6.3), over any transport,
  via **range-based set reconciliation (RBSR)**: an efficient exchange that converges two holders' record
  sets by comparing summaries of ranges rather than shipping everything. Every filled record is accepted
  only on its own author signature and its fold into the hash structure (the §6.6.3 self-verifying
  invariant), never on the partner's word.

**No transport gate; the gate is membership.** A message arriving over any path updates the high-water mark,
which is pure detection, requiring no trust decision and working for member and blind carrier alike. *Then*,
if the node is an entitled member with a gap and the Group allows it, the application reconciles with an
entitled member over any transport, and that member need not be on the fabric at all. Whether you reached a
partner via the fabric or a direct dial is irrelevant; whether they are a verified current member is
everything. *Verified (a gap detected from a fabric-delivered message is filled by reconciling with a
member reached off the fabric; the records verify source-agnostically, and tampered or forged records are
rejected whatever their source).*

The RBSR production construction (Willow 3d-range vs Negentropy) is a §5 decision; the scaling shape is
Verified, the specific construction is not yet chosen (Appendix B).

#### 6.8.2. The plane fusions, gathered

Stated once each at their occurrence and gathered here for reference, because knowing which combinations are
*forced by construction* and which are *free choices* is the whole point of separating the planes:

- **D-self is C-direct** (§6.5.1, §6.6.1): participants who buffer their own traffic also deliver it, one
  mechanism on the carriage and durability planes at once.

- **The meer is one node on three planes** (§6.6.2): D-meer (it persists), carriage (you carry-fetch from
  it), and P-meer (it can poke). Three jobs, one node, kept distinct on purpose.

- **C-swarm is fused with nothing durable** (§6.5.3): gossip carries but keeps no replay log, so it has no
  durability-plane partner and an offline recipient needs a separate D- source. This *non*-fusion is as
  important to name as the fusions: the swarm's lack of durability is a property to state outright, not one
  to discover after filing it under the wrong plane.

Everything else pairs freely: any carriage with any durability with any presence.

#### 6.8.3. The adaptive selector

The **selector** is the runtime policy that picks a (carriage, durability, presence) combination **per
recipient, per moment**, and races the non-exclusive sources within a plane. It is **not itself a delivery
model**; it is the control layer above the three planes. It chooses by connectivity and degrades
gracefully: a direct local link when both peers are present; swarm, meer, and member-corroboration racing
when the recipient is intermittent; meer plus push for a backgrounded phone; poll-a-meer when push is
declined. Where D-peer is enabled, it prefers reconciling with **several** members over one, so no single
member can withhold unnoticed (the §5.4 / Part 1 §2.4 no-single-dependency floor at the sync layer).

**Every cell has a named no-helper fallback**, because Part 1 §2.4 requires the floor to stay real: it is
always **D-self with P-none**, direct carriage, participant durability, recipient-polls presence. *Verified
(exactly-once delivery holds under every combination of surviving paths down to the D-self floor, and a
backgrounded device presents identical, complete, ordered history on waking).*

#### 6.8.4. Payload classes the selector distinguishes

Speed and durability are independent (carriage and durability are separate planes), so a fast path does not
imply a non-durable one. **The application classifies each payload deliberately; the selector never
guesses**, and in particular a real message is never silently treated as throwaway (that would be invisible
loss, which §6.5.3 / Part 1 §2.2 forbids).

- **Live-durable.** Both parties present, a low-latency path for a snappy real-time feel, *and* a full
  signed record that persists into history like any other message. This is the right shape for real-time
  chat in a provenance-first system: fast *and* remembered. Real-time here is just fast delivery of durable
  messages, not a separate ephemeral lane.

- **Intrinsic-ephemeral.** Non-utterance state whose value is bound to the moment: typing indicators, cursor
  positions, presence beacons. No durability source and no wake engage, because there is nothing to keep and
  nobody to wake; **loss is *correct***, and a suppressed one is a non-event, not a gap. Because these cost
  battery and radio for zero durable value, each carries a resource-default that can scale with Group size
  (typing indicators, whose value falls and whose cost rises as a Group grows, default off above a
  small-Group threshold, working value ~7 to 10, tunable). This is a battery setting, not a privacy or
  correctness one.

- **Chosen-ephemeral (disappearing messages).** A real, durable message carrying a signed "do not retain
  past T" disposition that honest clients apply. This is a *retention policy on a normal message*, not a
  transport mode, and it carries a hard honesty caveat identical to refusing a fake unsend: it is
  **cooperative non-retention, never enforced deletion.** A node cannot prove it expunged, and a modified
  client simply keeps what it read. So it may promise "honest clients stop showing this and drop their
  copy," never "this message is gone." (This is the §8 label-not-enforce posture applied to retention.)

- **Self-destruct (time-bound sensitive value).** A distinct payload class (the guest-wifi password shown
  until Monday) whose achievable semantics depend on the history mode and whose strength is bounded by node
  fidelity, not cryptography. It is the one class that deliberately chooses *less* durability and *less*
  replication, opting out of the meer and out of D-peer so no blind store holds the sealed value past its
  window, the inverse of every other class here. It is framed, not settled (Appendix B; the history-modes
  treatment in §7).

#### 6.8.5. The history store: content-blind history convergence via a mirror group

Gap-aware convergence (§6.8.1) runs between parties that hold keys, but availability often wants a node that
stores and serves durable history without being trusted to read it. That node is the **history store**, and
it answers the second reconciliation question (history convergence, "what am I missing") while the meer and
D-peer (§6.6) answer the first (live delivery, "what just happened"). There are two node tiers on one
primitive: **members**, who hold the Group's keys and read content, and **history stores**, which read
nothing. Both are helpers with capability, not standing (§5.4): a persona or a k-of-n group admits a store,
it holds and serves, and it can be removed the same way, so it holds no authority over any persona and is not
a center however much it holds. A content-blind store and a clear-text helper admitted to a scope for search
or indexing are two points on one spectrum, capability delegated without authority, differing only in how
much the peers chose to reveal. `Synthesis.`

**The mirror group and nested sealing.** A history store **MUST** be a member of a separate mirror group
G-hist for transport and **MUST NOT** be a member of the content group G, because MLS membership implies the
ability to decrypt (a member is a client with access to the group's secrets, and only members can decrypt an
application message's payload; there is no MLS notion of a member who cannot decrypt), so a store that held
G's key would read all content. Membership in G-hist gives the store only the mirror group's transport
secrets, an authenticated, forward-secret channel over iroh with no IP exposure, and never a content key.
`Verified-RFC` for the MLS property (RFC 9420, RFC 9750), `Design` for the nested construction. Content
**MUST** be double-sealed: a member takes an entry already sealed under G's per-scope asset key (§5.11) and
sends it as the opaque payload of a G-hist message, so the store decrypts only the G-hist transport layer,
obtaining the reconciliation envelope in clear and the asset-key-sealed content as opaque bytes; on retrieval
it returns those bytes over G-hist and the member peels the G-hist layer and then the asset-key layer. That
is what makes content-blindness hold, the store never holds the asset key. `Design.` The reconciliation
envelope **MUST** ride inside the encrypted G-hist payload and **MUST NOT** ride in MLS additional
authenticated data, because AAD is authenticated but sent unencrypted (RFC 9420 framing, RFC 9750), so a
relay would see it, whereas the envelope must be visible to the store as a G-hist member but not to iroh or
any relay. `Design.` G-hist membership **MAY** be cryptographically bound to G membership via an MLS
resumption pre-shared key linking the epochs, so a persona's presence in G-hist is tied to its presence in G
rather than tracked as a separate roster; the roster is in any case derived from the same governance chain.
`[confirm: the exact resumption-PSK binding construction.]`

**The store's Group Role Set.** A history store **MUST** be admitted with a Group Role Set granting only a
carriage-and-durability role and excluding every content and every governance Group Role, because the
exclusion, folded from the governance chain as an attributable fact, forecloses silent escalation: granting
the store any further role would either violate its Role Set's mutual-exclusion (rejected by the fold) or
require a visible, governed change every member folds. So the store can hold no read grant, no content-write
grant, no governance vote, and no capability-issuing authority. `Design.` The Role Set and key-withholding
are complementary and both required, because MLS enforces no application access control (§5.5, RFC 9750
§6.4), so the Role Set is the folded, auditable authorization statement and does not cryptographically
prevent decryption of a layer whose key a node holds, while key-withholding (the store holds G-hist's key,
never the asset key) is the cryptographic enforcement. A compromised store is therefore bounded by what it
holds, sealed blobs and envelopes, so its residual exposure is metadata, never content. `Synthesis.`

**The reconciliation envelope.** A history store **MUST** be able to detect gaps and serve the missing spans
of a member's history over MLS transport without holding any content key and without learning path
structure, wall-clock time, or the access-control structure, and it **MUST** expose no more than a blind meer
already does, namely which chains exist, their lengths, their approximate append timing, and padded
per-entry sizes. Each stored item is a pair inside a G-hist message, the cleartext-to-G-hist envelope and the
asset-key-sealed content blob. The envelope's minimal fields are:

- **subspace_id**: the author device-subspace as a hash or pseudonym rather than in the clear (Willow e2e
  guidance), which lets the store bucket entries by chain.

- **predecessor_digest**: the content digest of this entry's predecessor in its chain, or a genesis or
  checkpoint marker, the hash link that makes a gap nameable.

- **entry_digest**: the content digest of this entry and the address of its sealed blob, the reconciliation
  identity.

- **counter**: the per-subspace logical counter, monotonic and single-writer per device-subspace, which
  enables range reconciliation over (subspace, counter) and orders the chain with no wall-clock.

- **size_hint**: the padded byte length of the sealed blob, padded so the true payload length does not leak.

The envelope **MUST NOT** carry the Willow path (the hierarchy stays inside the sealed content, reconstructed
by members after decrypt), any wall-clock timestamp (a display time lives in the sealed payload as content,
§5.11 and §7.7), any Meadowcap capability or authorization token (verification is deferred to member
endpoints), or the raw subspace_id. The stated leak profile is meer-level: the store learns which
device-subspaces are active, each chain's length and append rate via counters, and padded per-entry sizes,
and it does not learn paths, content, wall-clock, or access structure. `Synthesis.` The envelope's byte
layout is `[gates-release]` (Appendix B).

**Why not a structure-aware replica.** A third tier was considered, a structure-aware replica reconciling at
path level over end-to-end-encrypted entries via Willow Confidential Sync, and it is rejected, because it
would be a second cryptographic primitive, so soundness would then rest on both MLS and Willow-e2e, and a
second threat boundary with its own envelope and failure modes to reason about. The mirror-group store
reaches the same availability goal on one primitive and one boundary, at the cost of coarser chain-level
rather than path-level reconciliation, which is an acceptable trade because path-level reconciliation by an
untrusted node is not a requirement. `Synthesis.`

*Running example: Dave's returning phone (E9) pulls the entries it missed from a member, or from a history
store that held them all along without ever being able to read them, the gap named by the envelope's
predecessor and entry digests and the spans served sealed over the mirror group (walkthrough beat E9).*

### 6.9. Discovery, connection establishment, and interaction tiers

Discovery is a concern distinct from the three planes, placed after the delivery model because carriage
(§6.5) presupposes it but does not depend on its internals. It is the one part of §6 the specification treats
more lightly than it will ultimately need: the fuller treatment, multiple resolvers, privacy of the lookup
itself, rendezvous, and revocation of stale records, is **under-developed** and owed future work (Appendix
B).

#### 6.9.1. Connection establishment and interaction tiers

The transport is **iroh**: encrypted QUIC with relay fallback for NAT'd peers, routed by `EndpointId`
(§6.5). A Group's membership maps to a gossip **topic** (§4.2, and the topic-is-one-contributor-to-scope
note there); a relay forwards opaque frames and **MUST NOT** be required to read content, routing by
endpoint, not by topic, because a relay that had to read content would need the Group's keys and could
censor or profile by what it carried.

**Co-location** (reference deployment): two peers reach each other over relay fallback only if they share a
home relay (no relay-to-relay mesh); relay placement is server-published and authoritative, keyed on the
rendezvous/namespace, not on identity. A relay process **SHOULD** meter and isolate per tenant and **MUST**
degrade *visibly* under stress, never silently. *Verified (measured).*

**Interaction tiers** are chosen at **Group creation**, not toggled at runtime: **interactive** (prompt
delivery + real failure signal), **quiet-large** (eventual, "it will arrive or you will be told it did
not"), and **broadcast** (best-effort rolling log). The broadcast tier **MUST** disable the embedded
Group-key ratchet tree (O(N) commits) and ship the tree out of band. *Design (tiers) + Verified (the O(N)
ratchet-tree cost is measured).*

#### 6.9.2. Resolving a key to a location

Discovery is the lookup that turns a bare `EndpointId` (identity) into a current network location so a peer
can be dialed at all. It is distinct from the gossip overlay's *internal* peer-learning (§6.10): discovery
is key-to-address resolution; the gossip overlay's internal mechanism is intra-swarm peer sampling.
Discovery gets you to the door; gossip is the room.

One integrity property is constant across all three mechanisms below and worth stating once: discovery
records are **self-signed by the endpoint's own key**, and the lookup key *is* that public key, so no
resolver can forge a peer's address; a tampered record fails signature verification against the key being
looked up. **What varies between mechanisms is who can observe or withhold lookups, never whether they can
lie.** Two notes on status after iroh 1.0: the three mechanisms are now **separately-versioned
address-lookup crates** (`iroh-mainline-address-lookup` for the DHT path, `iroh-mdns-address-lookup` for
mDNS) rather than options inside the core crate, so they are real shipped components, not
"wire-it-yourself" gaps; the self-signed-record integrity model itself is **[confirm against the Pkarr
record-signing spec]**, a Pkarr-level primary, not an iroh-version question.

**DNS / Pkarr (default; a soft center).** Self-signed records served through DNS servers operated by n0.
Fast (globally cached) and zero-config. The soft-center caveat: n0's servers cannot forge a record
(integrity holds) but can observe which keys are looked up and published, and can withhold or go down (an
availability and metadata-concentration point). A center for observation, never for integrity, and the one
discovery-layer surface the observer picture (§6.4) flags as an operated dependency.

**Pkarr-on-mainline-DHT (center-free, global).** The same self-signed records published into the no-owner
BitTorrent mainline DHT. This removes the single observation/censorship point: no party sees all lookups or
can globally withhold a record. Costs: higher lookup latency, lower reliability than cached DNS, and records
expire and require periodic republishing. This is the discovery path that keeps an internet-scale Drystone
deployment from depending on n0. Post-1.0 this path ships as the `iroh-mainline-address-lookup` crate.
**[confirm, the mainline-DHT publishing and republish-interval behavior against that crate's pinned
version.]**

**mDNS (center-free, local / airgapped).** Local-link multicast resolution: no internet, no server, no DHT.
Every member on the LAN segment is discoverable without publishing anywhere, so there is no bootstrap seed
at all (§6.10.2). Bounded to the broadcast domain (does not cross routers), a feature for airgapped and
family-LAN Groups, a hard limit for cross-network reach. Same key-authenticated integrity: a malicious LAN
responder returning a wrong address causes a failed dial, never an impersonation. Post-1.0 this ships as the
`iroh-mdns-address-lookup` crate, so it is a turnkey discovery mechanism, not something a deployment must
build itself. **[confirm, the crate's exact behavior, e.g. whether it republishes on interface change,
against its pinned version.]**

### 6.10. The gossip overlay: the current realization of C-swarm (HyParView + PlumTree)

This section develops the mechanism behind C-swarm (§6.5.3), and it is the clearest place to see the
requirement-vs-realization discipline the whole section rests on. **The requirement comes first, and it is
the durable thing**; the named algorithms are one way to meet it.

**The requirement C-swarm places on any overlay.** An **epidemic broadcast overlay**: a swarm in which
nodes forward each message to their neighbors along a **self-healing spanning tree**, so a message injected
anywhere reaches every currently-live subscriber, **without any node holding global membership** and
without a central broadcaster. Two capabilities follow: a **membership** mechanism keeping each node a
small, self-repairing neighbor set (the swarm survives churn without anyone knowing the whole roster), and a
**broadcast** mechanism pushing content along the tree while lazily repairing breaks (delivery both cheap
and resilient). The overlay carries **sealed bytes only**; it is one population of the Delivery Fabric
(§6.3) and is blind by construction. This requirement is consolidated in §10.3, and **a different overlay
meeting it is swappable for the named one**: if a future swarm protocol met these capabilities with some
added advantage, it could replace HyParView/PlumTree without disturbing anything above the overlay, which is
exactly the point of reasoning about the requirement independently of the manifestation.

What the requirement does **not** ask of the overlay, stated because a naive design would over-build it:
**durable storage** (the swarm keeps no replay log, which is why a fully-offline node recovers nothing from
it, §6.5.3, and durability is a separate plane, §6.6), and a **separate content-free "a message exists"
signal** riding the overlay (a node in the swarm already sees each arrival, so presence for that node rides
the arrival of the sealed bytes, §6.7 P-gossip).

**Current conforming implementation: iroh-gossip.** Once a peer can reach one swarm member (via §6.9), the
gossip layer disseminates messages and maintains a fresh neighbor picture with no coordinator. iroh-gossip
stacks two algorithms, each from its own 2007 paper by Leitão, Pereira & Rodrigues: **HyParView** for
membership ("HyParView: A Membership Protocol for Reliable Gossip-Based Broadcast," DSN 2007) and
**PlumTree** for broadcast ("Epidemic Broadcast Trees," SRDS 2007, pp. 301–310). *(Verified, attribution and
the two-paper split against the primaries this round; the SRDS paper is PlumTree, the DSN paper is
HyParView, they are not one paper, and iroh-gossip's own crate docs name both papers as its basis.)*

**A version-status note that matters here.** Unlike iroh core, **`iroh-gossip` is a separately-versioned
crate on its own pre-1.0 release line**, outside the iroh 1.0 wire-and-API stability guarantee. Its
mapping of HyParView/PlumTree, its event surface, and its tuning constants can still change between
releases. The flags in this subsection are therefore scoped to `iroh-gossip`'s own pinned version, not to
iroh core, which is stable. **[confirm, iroh-gossip internals against its pinned (pre-1.0) version.]** This
version split is itself an instance of the requirement-vs-realization separation: the requirement is pinned,
the realization is a moving pre-1.0 crate, and saying so is what keeps a reader from mistaking a tuning
constant for a design commitment.

#### 6.10.1. HyParView, the membership layer

Each node keeps two views: a small **active view** (the live connections messages flow over) and a larger
**passive view** (a reserve address book, not connected). Periodic background shuffles swap view samples
between nodes, keeping the passive view stocked. When an active link dies, a passive peer is promoted to
refill it, so the overlay reheals locally with no central coordinator. **[confirm, the active/passive
view-size constants are illustrative; verify iroh-gossip's configured values, the SRDS paper used a small
fanout, e.g. 4, but iroh may differ.]**

This is the source of the "gossip has its own internal discovery" property: within a swarm, a node learns of
*other members* from peers it already knows, via shuffles. This is intra-swarm peer learning, **not**
key-to-address resolution (§6.9); it cannot bootstrap a node into a swarm from nothing, which is why
`subscribe` takes a bootstrap-peer set (§6.10.2). *(Verified against the iroh-gossip crate API: `subscribe`
is called with a `TopicId` and a set of bootstrap peers, confirming a swarm cannot be joined from a topic id
alone.)*

#### 6.10.2. Bootstrap and the seed function

To join a topic, a node hands `subscribe` one or more bootstrap `EndpointId`s already in the swarm;
HyParView grows its view from that seed. The seed shares peer *knowledge* (samples from its views), not its
live connections; the newcomer forms its own active view by dialing peers directly. The seed is a phonebook,
not a switchboard.

> **The seed is a join-time center if singular, make it plural and rotating.** A swarm runs fine without the
> seed once members are woven in (PlumTree and HyParView need no bootstrap thereafter). But a *new* member,
> or one that lost its passive view while offline, cannot re-enter if the only discoverable peer is down.
> Per Part 1 §2.4 this is the structural-dependency hazard relocated to the join path, and the fix mirrors
> the meer fix (§5.9): several members publish as seeds, newcomers bootstrap from any, the set rotates. In
> Mode 2 (mDNS, §6.11.2) this is automatic, every member is discoverable, so there is no single seed. A
> starved active view (e.g. "1 active, many passive") makes a seed especially fragile and is not recommended
> for a peer others rely on to join.

#### 6.10.3. PlumTree, the broadcast layer

Each node splits neighbors into **eager-push** peers (which get the full message immediately) and
**lazy-push** peers (which get only an `IHAVE(message-id)` digest). Eager links self-organize into a
spanning tree; lazy links are the redundant edges held in reserve. **[confirm, eager/lazy semantics against
the SRDS primary and the iroh-gossip implementation.]**

Pruning is the efficiency engine: a duplicate arriving over an eager link (already received from the
tree-parent) triggers a `PRUNE` that demotes the link to lazy. After a few messages the eager links collapse
to a tree carrying one copy per node, and the redundant links carry only cheap digests. Healing is why the
lazy links exist: if a tree branch breaks, downstream nodes still receive `IHAVE` digests over lazy links,
notice a digest for a message they never received, and send a `GRAFT` to pull it and re-attach the tree.
*(The payload-via-tree-branches, recover-via-remaining-links design is verified against the SRDS primary;
the exact `PRUNE`/`GRAFT`/`IHAVE` wire behavior in iroh-gossip is* **[confirm].**)

#### 6.10.4. What gossip does and does not guarantee

Delivery is best-effort and probabilistic: the tree usually reaches everyone and healing usually repairs
breaks, but a node that is *entirely offline* during a broadcast is not in the tree and receives neither
eager pushes nor `IHAVE` digests; there is no branch to heal. This is the structural reason gossip cannot be
the durability layer: PlumTree repairs *transient* breaks in a live tree; it offers nothing to an absent
node. Offline durability is the durability plane's job (§6.6), and catch-up validity is the returning
member's local check (§7.4).

The event surface a subscriber sees at the network layer is `NeighborUp`, `NeighborDown`, `Received`, and
`Lagged`, note the absence of any delivery confirmation. *(Verified against a recent iroh-gossip release:
the net-layer subscriber event set is exactly these four, and `Lagged` carries no information about *what*
was missed. The protocol-layer `ProtoEvent` is narrower still, `NeighborUp` / `NeighborDown` / `Received`,
with no `Lagged` at all.)* The exact enum variants vary across `iroh-gossip` releases, so they remain
**[confirm]** against the pinned release, as does the send-side `broadcast` return semantics. The
load-bearing property is version-stable regardless of the enum's exact shape: there is **no per-recipient
ack**, so "what to drop" decisions cannot read a gossip confirmation; they ride MLS epoch acknowledgement
and dial-home acks instead (§6.6.2, §7.4). `Lagged` tells a receiver it fell behind but not what it lost, a
reason to drain from a meer, not a data-loss signal in itself, and the precise "what did I miss" answer
comes from gap-aware convergence (§6.8.1), not from the overlay.

### 6.11. The two deployment modes

The modes are **named bundles of plane choices**: they run the **same protocol**, and the differences are
entirely in the discovery substrate (§6.9) and the presence of the meer (§6.6.2). MLS sealing, the gossip
overlay, local commit verification, and the cursor catch-up are identical in both. This is the payoff of the
whole three-plane treatment: a "deployment mode" is not a different architecture but a selection across
Carriage, Durability, and Presence.

#### 6.11.1. Mode 1, relay/meer (internet-scale, members scattered, often offline)

- **Discovery:** DNS/Pkarr or Pkarr-on-DHT (§6.9.2).

- **Carriage:** direct hole-punched QUIC where possible (C-direct, §6.5.1); relay-assisted fallback
  (C-relay, §6.5.2); the swarm spans the internet (C-swarm, §6.5.3).

- **Durability:** returning members dial a redundant, rotating meer (D-meer, §6.6.2), report the `(G, D)`
  cursor, drain missed governance commits and retained dataplane backlog, then verify forward (§7.4).

- **Presence:** P-meer or P-push over a poll-a-meer floor (§6.7).

- **Meer-absent behavior:** liveness and cursor comparison still work peer-to-peer (D-peer, §6.6.3);
  governance validity is still locally checkable; only long-gap backlog (a member offline while its
  corroborating peers were also away) is lost. Governance currency degrades to a corroborated estimate,
  which is safe because incompleteness can only **under-authorize** (§7.4).

#### 6.11.2. Mode 2, direct P2P, local network (LAN, airgapped, family-tablet, field)

- **Discovery:** mDNS (§6.9.2), every member discoverable, no seed.

- **Carriage:** pure direct QUIC on the LAN (C-direct); no relay (no NAT problem on a flat segment); the
  swarm is small and local (C-swarm).

- **Durability:** peer-to-peer (D-peer, §6.6.3), a briefly-absent member rejoins the topic, asks reachable
  members for cursors, pulls missed records from any member that has them via gap-aware convergence
  (§6.8.1), verifies locally. No meer.

- **Presence:** P-gossip and P-none; no push host.

- **Missing vs Mode 1:** durability across a *total-absence* window only. If all members cycled off, no one
  holds the backlog. Governance stays safe (under-authorize); the dataplane may gap.

> **The through-line.** Mode 2 is Mode 1 minus offline durability, with every other function preserved,
> `P-Durable-Enablement` made concrete. The meer and the relay are **additive conveniences, not structural
> requirements** (§5.9): discovery swaps substrate; the cryptographic and governance machinery is invariant.
> This is the §5.9 exitability claim seen at the transport layer: the rigid single-keeper deployment is one
> *setting* of this model, never a different architecture.

### 6.12. Real-time media

Real-time media (voice/video/stage) rides the **same iroh transport** as messaging but over **QUIC
datagrams** (unreliable, no retransmit) carried as RTP-over-QUIC. It is, in the payload-class vocabulary of
§6.8.4, an intrinsic-latency-over-reliability class with its own carriage needs. Media frames **MUST** use
the datagram flow (latency over reliability) and **MUST** be end-to-end encrypted via per-sender keys
derived from the Group key epoch, so a forwarding helper stays blind. A Group-scale call **SHOULD** use a
**blind forwarding helper** (header-only routing) rather than full mesh past a handful of peers; server-side
mixing that requires plaintext is **forbidden**. Media keys rotate on membership change exactly as messages
do.

Two media congestion-control rules are **normative**: (1) the media engine's bitrate estimator **MUST** be
authoritative and back off on the path-RTT trend (plus per-stream loss and jitter); it **MUST NOT** rely on
datagram-send back-pressure (the transport silently drops oldest, never errors) nor on receiver-side loss
alone (a delayed prefix shows none). (2) Real-time media and bulk reliable transfers **MUST** run on
separate flows/connections, or the bulk transfer starves the media. *Verified (both rules measured: a
delay-based estimator backs off 64→8 kbps in under a second; co-located bulk drove media RTT to seconds,
separate flows left the call untouched). The video engine and real-codec/RTP path are design.*

## 7. Synchronization and Governance-Conflict Resolution

> `Realizes: P-Knowable-Truth, P-Local-Truth, P-Peer-Equality`
>
> Reasoning complete; several wire encodings are `[gates-release]` (Appendix B). The **Willow / Meadowcap** claims
> are confirmed against the spec; the comparative claims about **Matrix** (State Resolution, MSCs, CVEs)
> and **Keyhive** remain **[confirm]**, web-verified in source dialogues and consolidated
> in Appendix C, not yet in the FACTCHECK SoT.

### 7.1. Data-model commitment

Governance facts live in a **namespace / subspace / path** structure addressed and reconciled by
**range-based set reconciliation** (a Willow-shaped data model; the three-dimensional namespace/subspace/path
model is confirmed against the Willow spec, Appendix C). Drystone implements this *shape* directly in early
phases rather than depending on a Willow implementation, so a later transition is a substitution, not a
redesign; Drystone is built Willow-*shaped*, not Willow-*dependent*.

### 7.2. The grant-and-revocation interface (mechanism-neutral, normative)

> **Scope (carried from §5.5):** this interface governs both kinds of grant that sit above the rights
> floor, **Group Roles** (in-Group governance authority) and **capabilities** (Meadowcap data-access grants).
> Both are unforgeable, attenuating, revocable governance facts, so they share one interface. "Capability"
> here is Meadowcap's data-access sense, kept verbatim; the requirements are the standard object-capability
> guarantees.

Whatever Group-Role / capability mechanism Drystone adopts **MUST** provide:

- **R1, Unforgeable grant.** A Group Role or capability cannot be fabricated by anyone not entitled to issue it.

- **R2, Attenuating delegation.** A holder may delegate a subset of held authority, never a superset
  (`Realizes: P-Peer-Equality`; this is Meadowcap's confinement property for capabilities).

- **R3, Convergent revocation expression.** A revocation **MUST** be expressible as a governance fact
  (§7.3) that folds deterministically, so all synced honest peers agree the grant is void.

- **R4, Bounded stale-authority exposure.** For a holder that refuses to sync a revocation, the protocol
  **MUST** bound the window in which third parties accept the revoked grant, a finite, stated bound
  (epoch boundary or membership-graph generation; **not** a wall-clock interval, per Part 1 §2.0.1, a bound
  expressed in time would rest on the same uncorroborable clock).

- **R5, Forward read exclusion.** After expulsion, the member **MUST NOT** read entries authored after
  the expulsion folds in (past entries out of scope, §7.5), because honoring a removed member's reads would
  let the expulsion be undone in practice, the member continuing to see the Group it was removed from.

- **R6, Attributable acceptance.** A participant accepting a write under a capability **MUST** record the causal
  frontier of governance facts it had synced at acceptance, so a later-synced revocation makes the stale
  acceptance **detectable and attributable** rather than silent (§7.5).

R3 and R6 are what defeat a silent state-reset failure mode and hold regardless of the mechanism. The
mechanism itself (Track A vs Track B) is **deferred** to the richer-access-control phase; see Appendix A.
No normative text here assumes a track. *Design.*

*Running example: the Group admits Dave through this one grant-and-revocation interface, the same interface a
later revocation of him would use (walkthrough beat E4).*

### 7.3. Governance facts are entries, not mutations

A governance decision (admit / expel / grant / revoke / amend) is a **signed, append-only entry**. Entries
are never modified or deleted; a reversal is a new entry referencing the one it reverses. There is no
"current state" to reset, only a monotonically growing fact set and a deterministic **left fold** from it
to an effective authority state. A participant that has seen fewer facts computes a **stale** authority state,
never a wrong one, and never one another participant could weaponize by replaying old entries (`Realizes:
P-Knowable-Truth`). *Design, the append-only-fold property is the load-bearing invariant the rest of §7
depends on; if it is relaxed, termination (§7.5.2), no-state-reset, and attributability all fall together.*

> **The deployed evidence for this invariant, and the steelman against it.** This is the central design
> bet of §7, so it is stated with both the supporting case and the strongest objection.
>
> *Supporting case.* Matrix State Resolution instead recomputes a mutable key-value state map per
> resolution. In 2025 that produced **CVE-2025-49090**, a state-reset class in which room state could
> revert to an earlier value with no event validly producing that reversal. Per Matrix's own disclosure
> the root cause was the **starting state** the replay built on and the **scope of events replayed**, not
> the tiebreak; the fix (State Res v2.1 / MSC4297) was to begin the iterative auth checks from the empty
> set rather than the unconflicted set, and to replay the full *conflicted subgraph* between conflicting
> facts (see §7.5.2, which adopts that closure). Drystone's append-only monotonic fold makes this
> reversion class **structurally impossible**: a lagging node under-authorizes, it never reverts to a
> prior value. Drystone's founding-fact base case (§7.5.2) is also already the empty-equivalent
> unconflictable base that v2.1 moved toward. *Two distinct claims, kept separate so neither overreaches:*
> the monotonic fold defends against the **reset** class; the timestamp-free order (§7.3.1) defends
> against the **backdating** surface. These are different defenses against different failure modes, not
> one claim. *(Verified this revision: the CVE-2025-49090 root cause, a state reset to an earlier/incorrect
> value absent a validly-producing event, and the MSC4297 fix, begin the iterative auth checks from the
> empty set and replay the full conflicted subgraph between conflicting facts, are both confirmed against
> the Matrix State Res v2.1 implementer's guide and the CVE record.)*
>
> *Steelman against.* Under the same 2025 review, Matrix concluded that sound decentralized resolution
> requires an **uncapped root**, room creators with permanent "infinite" power (MSC4289), reasoning
> that an attacker who can backdate events already wields de facto apex control, so the apex must be made
> explicit to be bounded. Drystone's capped, delegable, revocable-by-succession root (below) contradicts
> that conclusion. Drystone's wager is that the conclusion was **forced by their inputs, not by the
> problem**: their order consumes a wall-clock, so backdating manufactures authority, so they pin
> authority to an apex; Drystone removes the wall-clock from the order (§7.3.1), so that specific attack
> has no purchase. **This is an argument, not yet a proof, and the gap is named precisely in Appendix B.**
> *(The MSC4289 facts, creator infinite power and the backdating rationale, are verified against the Matrix
> Project Hydra disclosure; what remains open is Drystone's soundness claim, not the Matrix fact.)*

**The unconflictable root.** Each Group has a founding fact establishing initial authority. Its authority
over the genesis of the Group is **not** subject to the §7.3.1 ordering; it is the base case of the
authority-rank computation, not a competitor within it, because a conflict at the root is the one ambiguity
an attacker could convert into total capture. Drystone's root authority is **capped, delegable, and
revocable-by-succession** (`Realizes: P-Peer-Equality`), not infinite. The root forgery vector itself is
closed structurally: the Group's genesis id is `H(tag ‖ group_id)` (§4.2) and the founding fact is the fold's
unconflictable base, so there is no second create event to smuggle in (the structural analogue of the fix
Matrix shipped as MSC4291, room-id-as-hash-of-create-event). *Design; the Matrix MSC4291 fix is verified
against the v1.16 release notes.*

> `[gates-release]:` Root-authority transfer is not a special operation: a planned handoff is reassignment
> (a revoke-then-grant that folds) and a contested one is a fork (§7.6). The residual, recovering a principal
> whose keys are lost, is stubbed at §7.3.9 as a bounded recovery primitive (restore, never seize). A permanently
> fixed root contradicts the cooperative model. The capped-vs-uncapped-root soundness question (above and
> Appendix B) must be resolved before this is frozen.

*Running example: admitting Dave and setting the k-of-n threshold are new append-only facts in the log, not
edits to a mutable state, and the current authority is the fold over them all (walkthrough beat E4).*

#### 7.3.1. The total order over conflicting facts: causal and cryptographic only, never temporal

When two facts conflict (§7.3.2), every honest persona **MUST** resolve the conflict the same way, because
two personae that resolved the same conflict differently would hold divergent canonical views, which is
exactly the silent split the fold exists to prevent.
Resolution has a **precondition** (authorization, established structurally) and then a layered **order**
over the authorized decisions that remain.

**The precondition: authorization is a gate, never a ranking.** A governance change is an assembled
**k-of-n quorum** of concordant facts, not a single fact, and each contributing fact counts only if its
author held the required standing *at that fact's causal position*, computed by the §7.5.2 forward pass
(which settles authority without consulting the contested outcome, breaking the authority-ordering
regress). A contributing vote is counted at its own causal position and a later removal of its author does
not retract it, because a cast vote is a durable governance act: the only fact that un-counts a vote is a
ban causally prior to it, meaning the author already lacked standing when they voted and the vote was never
valid, so un-counting merely corrects an over-count, whereas a ban that lands after a valid vote removes the
author going forward but does not reach back to void it. Making a crossing instead depend on every
contributor still holding standing at the moment it completes would void valid votes of members removed in
the interim, which is a social-utility judgment the protocol does not make, and would enlarge the set of
concurrent facts a crossing must rule out, binding every crossing more tightly to the completeness beam. A
contested removal in that window resolves as an explicit social fork, the removed persona leaving with its
current state, never as the protocol voiding a valid act. A set of fewer than k concordant facts leaves the slot unchanged everywhere, including for the
signatories themselves: a persona's own vote does not move the slot. So authorization admits a decision
into the order or excludes it; it does **not** tip a conflict, and no decision wins *because* its author
holds more authority. This is deliberate, and it is the earlier draft's "issuer authority rank" corrected:
ranking conflicting decisions by their author's standing would let authority decide an outcome, which is a
center living in the comparator, the same move Drystone refuses for the wall-clock and for sender power in
the Matrix contrast below, and the same move the configurable-tiebreak principle forbids (a
party-privileging mechanism must be opt-in and governed, never a silent default). Standing decides *who
may act*; it never decides *whose otherwise-valid decision wins*. `Design.`

Among the authorized decisions that conflict, Drystone resolves in this order:

1. **Operation-type precedence, the layered fold: subtractions before additions.** Resolve highest tier
   first, each tier against the settled result of the tiers above it: threshold changes, then membership
   removals, then role and capability removals, then role and capability grants, then membership additions.
   This biases every intermediate state toward the more restrictive reading (the fail-safe direction, the
   instinct behind remove-wins and revoke-wins), and membership brackets roles so a grant always projects
   onto an already-settled membership (§7.3.2). A removal's cascade, its ceiling stamp and the revocation
   of the member's roles, rides at the removal's tier, not deferred to a later one. Type-precedence
   privileges no party (it orders *kinds* of operation, not parties), which is why it is a safe default
   rather than an opt-in.

2. **Causal precedence, within a tier.** For two facts of the same type, the causally-later fact
   supersedes the causally-earlier one for the same slot. Each fact references the frontier its author had
   observed; the transitive closure is happens-before, and that observed-frontier reference is a single
   committed hash rather than an explicit frontier (§4.6). Causal precedence is authoritative and is never
   overridden by the tiebreak below, because the tiebreak exists only where no causal answer exists;
   overriding a genuine happens-before with an arbitrary key would discard the provenance the order is meant
   to honor.

3. **The concurrent tiebreak, only among genuine concurrents.** Where two same-type facts are mutually
   concurrent, so no causal answer exists, a deterministic total order decides, and any total order
   identical on every node is correct, since by definition no causal truth is being overridden. The default
   key is the **content address**, the digest of the canonical fact encoding: party-neutral, ungameable,
   and identical everywhere. A group **MAY** govern this key upward within a safe range (join-*order*
   seniority for member operations, defined strictly as a settled causal or logical position and never a
   wall-clock; or governed instance weighting), under the rule that any party-privileging key is opt-in and
   itself under k-of-n governance; the content address remains the total fallback beneath any such choice.

**Timestamps appear nowhere in this order, and the reason is structural, not merely defensive.** Earlier
framings justified excluding the wall-clock by noting it is "trivially gamed by a peer lying about its
clock." That is true but it is the *weak* reason (it describes only deliberate deception). The *strong*
reason, from Part 1 §2.0.1: **a timestamp is not a fact even from a perfectly honest node.** Time
discernment is a shared construct, locally represented; a node can be objectively wrong about its own clock
and have no way to know it, and **no node can ever prove when an event occurred on another node**, there
is no shared clock to appeal to, only each node's local proxy. A timestamp therefore fails corroboration at
the root and is an **assertion**, never **provenance**, so it cannot order what must converge.

The security consequence is sharper than "don't trust attackers": **a wall-clock in the ordering is a
social-engineering vector even with membership fully gated.** Gating answers *who* acts; it says nothing
about *whether their clock is real*. An authorized, honest member with a skewed clock manufactures a
favorable resolution with no malice; a malicious member does it deliberately while passing every membership
check. The order is therefore **causal and cryptographic only**, causal precedence and per-device logical
clocks (corroborable) plus a content-address tiebreak (deterministic), never a wall-clock (uncorroborable).
*Design.*

> `[gates-release]:` The canonical byte encoding fed to the content-address hash, and the wire format of a
> governance fact, must be specified to byte level before two implementations interoperate. This gates a
> publication-final release.

> **Contrast with Matrix State Resolution (the closest neighbor), stated precisely and confirmed against
> the primary sources.** Matrix uses the same DAG-plus-Kahn's-algorithm skeleton Drystone uses, but it
> **folds sender power into the ordering** and breaks ties by **mainline position and `origin_server_ts`**
> (the sender's claimed wall-clock). The State-Resolution-v2 sort orders events by `power_level`, then
> `origin_server_ts`, then a lexicographic `event_id` fallback, so a forgeable timestamp is a live
> discriminator in the path that decides *which governance event wins*. The Matrix authors are explicit
> about the cost in their own design discussion: timestamps are used as a tiebreaker whenever the auth DAG
> implies no ordering, which (this is my characterization, not a verbatim Matrix claim) is the **common
> case**, so the resolution effectively relies on servers being honest about the time. The MSC1442
> discussion makes the conditional explicit, framing state resolution as correct for servers that do not
> lie about the time. Damage is mitigated only **after the
> fact** by an admin starting a new epoch (de-opping or banning the offender). That is the precise design
> choice Drystone refuses: Drystone keeps the topological-sort skeleton and removes *both* power and
> wall-clock from the ordering spine, checking authority only in the forward pass (§7.5.2) and breaking
> ties by content-address, so a lied-about clock has **no purchase to mitigate**, rather than a purchase
> that must be patched. **This is a difference of architecture, not of correctness.** Matrix's tolerance is
> *rational for Matrix*: it has an authority tier (homeservers, room admins) that can override a bad
> resolution cheaply and in place, so it can absorb the occasional gamed result with an administrative
> correction. Drystone has no such tier, every node holds its own canonical view, and its only remedy is
> the fork (§7.6), exit rather than in-place override, so it cannot afford a routinely-gameable ordering
> input the way a system with cheap correction can. The two are one coin: no-authority-tier-to-appeal-to
> and fork-as-only-remedy together *force* the exclusion here, and would force it in any system sharing
> those two properties, but they do not make it a universal law (Part 1 §2.0.1). The reason for rejecting
> power-in-the-comparator is separate from the reason for rejecting timestamps: folding power into the
> comparator drew an apparent-cycle objection in Matrix's own review, while the timestamp tiebreak is
> uncorroborable per Part 1 §2.0.1. Note carefully: Matrix's 2025 state-reset CVE was rooted in
> starting-state/replay-scope, **not** in the timestamp tiebreak, so this contrast is about ordering-spine
> design, and the CVE is cited in §7.3 for the *separate* monotonic-fold point. *(State-Resolution-v2
> tiebreak fields and the authors' "trusting servers not to lie about the time" admission confirmed against
> the Matrix.org stateres-v2 description and the MSC1442 discussion. See Appendix A and Appendix C.)*

*Running example: the Alice/Bob mutual removal, being genuinely concurrent, is decided by the deterministic
content-address tiebreak, so every node selects the same survivor (walkthrough beat E6).*

#### 7.3.2. What conflicts

Two governance facts conflict when applying both, in either order, yields a different effective authority
state than applying them in the order §7.3.1 selects, concretely, two facts targeting the same subject
where at least one removes or narrows authority the other depends on. The **mutual-expulsion** case (A
expels B while B expels A, equal standing) resolves by the concurrent tiebreak (§7.3.1 key 3): both are
membership removals of the same type and, being concurrent, admit no causal answer, so the content-address
tiebreak selects exactly one. Exactly one survives, never both and never neither, and the loser's fact
remains in the log as a valid-but-superseded entry, visible for audit.

**Cross-slot effects are projections on the final resolved slots, not incremental mutations.** Slots
resolve independently by §7.3.1; effects that span slots are then computed once, as pure functions of the
resolved values, and **MUST NOT** be applied by mutating state mid-fold, because a fold that mutated the state it is still reading would
become order-dependent, and two nodes folding the same facts could then reach different results. Two rules govern roles against
membership. First, **removal revokes roles**: a `RemoveMember(m)` acts as a revoke on every one of m's role
slots at its own causal position, so a role granted causally before a removal is revoked by it, and
re-adding m later does not silently restore those roles (a fresh grant causally after the re-add is
required). Second, the **effective-roles projection**: a role is effective only if its slot resolves to
granted **and** m's membership slot resolves to member, so a removed member holds no effective role even if
a concurrent grant happened to win that role's tiebreak. Computing both as projections on the final sets
makes them order-independent by construction, where an incremental cascade (delete on the removal, recreate
on a later grant) would be order-dependent, the exact latent divergence the fold exists to exclude. In the
layered order (§7.3.1 key 1), a removal's cascade rides at the removal's tier, so by the time role grants
resolve a removed member already has no roles to reason about, and the projection is consistent at every
tier boundary, not only at the end. `Modeled.`

**The fold checks well-formedness and authorization, but never rejects a fact because its target appears
absent.** Semantic validity (does the target exist) is a function of the complete causal history, which a
node may lack under a gap, so rejecting on it would make the resolved state depend on which facts happened
to arrive. Instead, an operation on an absent target is an **idempotent no-op** with respect to the
resolved state: `RemoveMember(m)` with no observed `AddMember(m)` leaves m not-a-member; `RevokeRole(m, r)`
with no observed grant leaves the role ungranted. Membership "never mentioned" and "explicitly removed" are
therefore indistinguishable, both are not-a-member, which is intended. A fact whose referenced predecessor
is **absent** is a different matter: it is a detected **gap** (the causal references of §7.3.1, and the
completeness dependency §7.3.3 turns on), which the node marks as incompleteness and does not fold onward
as if complete. Absence of a target is a no-op; absence of a predecessor is a gap; neither is ever a
fold-time rejection. `Modeled.`

> **Boundary with the §7.6 hard-stop.** §7.3.2 resolves conflicts where the ordering key yields a
> determinate, non-arbitrary winner. The mutual-expulsion case above is resolvable *only because* a
> content-address tiebreak is non-arbitrary-by-construction (deterministic on every peer); it is a
> provenance tiebreak, not a utility judgment about who *should* win. Where a contradiction is a genuine
> membership contradiction that the fold cannot determinately resolve without manufacturing a utility
> verdict (Part 1 §2.5), §7.6 applies instead: the protocol hard-stops and escalates. The line between
> "deterministically tiebroken" and "must escalate" is itself partly a per-Group tolerance, see §7.6.

*Running example: Alice's removal of Bob and Bob's removal of Alice target the same subject at equal
standing, so they conflict and enter the concurrent case (walkthrough beat E5).*

#### 7.3.3. The declarative snapshot is a cache; truncation is verifiable

The governance log is the **imperative** source of truth; "current state" (membership, roles, the rules
in force) is a **declarative snapshot**, a deterministic fold of the log carrying the governance head it
was computed from. The snapshot **MUST** be treated as a cache: it is **never authoritative, never
independently writable, never synced as truth, never trusted from a participant**, and it is **valid only while
its recorded head equals the Group's current governance head** (otherwise re-fold the tail). It is not
"latest values" but "latest values that passed authorization at each step", the log is **self-validating
under replay**, since each fact is admitted only if authorized under the rules in force at its position.
Peers reconcile by exchanging the **imperative log** and each deriving the snapshot independently;
agreement is verified by reaching the same state from the same head, and disagreement is explicit, there
is **no point at which a participant accepts another's declared state without local validation.**

To bound replay cost without breaking that discipline, the log **MAY** be truncated by a **roll-up**: a
signed checkpoint committing to `(governance_head_hash, state_commitment)`. Because the head is hash-linked,
committing to it transitively commits to the whole prefix, so a roll-up is a **re-expandable, back-verifiable
truncation**, not a trusted summary. The sound posture (and the one that needs no quorum to stay live):
**each participant independently folds and self-checkpoints**; where roll-ups are co-signed, a co-signature is
*corroboration of an independent identical fold*, never a substitute for local validation. Compaction is set
at genesis in **two tiers**, the **governance spine is permanent and uncompacted** (it is exactly what a
returning/dormant node needs to reconstruct the authorized signer set and validate everything else), while
**content is compactable** into head-committed, Merkle-rooted checkpoints. Roll-up is **built-in but off by
default** and catch-up never *depends* on it. *(Design; the byte-level checkpoint encoding is `[gates-release]`,
Appendix B. Local snapshot/rollback mechanics, e.g. savepoint cadence, are an implementation detail.)*

**Backward-completeness and forward-completeness are different, and enforcement turns on the second.**
Everything above concerns completeness *behind* a checkpoint: a roll-up lets a node prune old facts and
still trust what remains, because the hash-link back-verifies the prefix. It says nothing about
completeness *ahead*, whether an unseen, causally-later fact is missing that would change a slot the node
is about to act on. Reads tolerate that uncertainty; irreversible enforcement does not. Two states,
defined. **Best-known state** is what a node computes by folding every governance fact it currently holds;
it is always computable and always available. **Final state** is best-known state that the node has
additionally established is current to the group's leading edge: it can rule out an unseen, causally-later
fact that would change the relevant slot. The distinction is exactly forward-completeness, best-known says
"given what I hold," final adds "and I hold everything up to the edge that bears on this slot."

The gating rule is the **read/enforce line**, stated as a single referenceable normative gate in §7.3.8; the treatment here gives the underlying backward-versus-forward distinction, and §7.3.8 gives the clause the ceiling and enactment sections point at. Reads and content-plane operations **MUST** use best-known
state and **MUST NOT** be gated: a node **MUST** always be able to fold, serve its best-known view (with a
freshness qualification when that view is not final, §7.4), and stay live on the content plane, even under
partition. An irreversible authority-enforcing action (honoring or denying access, admitting or removing a
member, anything not cheaply reversible) **MUST** use final state and **MUST** be gated on it; when a node
cannot establish final state for the relevant slot, that action **MUST** stall (fail closed), and only that
action, the stall **MUST NOT** extend to reads or content-plane liveness. Applying fail-closed to reads as
well would produce a node that halts on any partition, a worse failure than the divergence it would
prevent. A node establishes final state for a slot by holding a quorum-attested *now* (the materialized
governance head, added below) whose bound head covers the slot's relevant facts, and observing no
attestation referencing a later head it lacks (the §7.4 freshness cursor is the model); for a removed
actor's entitlement specifically, the membership *ceiling* (added below) is the durable marker, since an
action at or beyond an actor's recorded ceiling is void regardless, the ceiling not going stale the way a
general slot view can.

The load-bearing caveat, stated rather than claimed away: a sufficiently isolated node **cannot** establish
final state on its own, because it cannot distinguish "no later fact exists" from "I cannot currently reach
anyone who would attest one." Such a node **MUST** fail closed on enforcement (stall) while continuing to
read and operate on best-known state. This is the corroborated-not-proven nature of completeness-ahead
(the single remaining load-bearing beam, Appendix B): final state is established by corroboration reaching
the node, never by a lone node's self-certification. It makes enforcement safe (never on stale authority)
at the cost of enforcement liveness under isolation, which is the correct trade for a governance layer:
**delay over breach.** `Design` for the distinction and the gating rule; the isolated-node liveness limit
is intrinsic and is stated, not closed.

#### 7.3.4. The governing principle for governance objects: sign the state, not the authorship

Every durable governance object introduced in the subsections below, the membership ceiling (§7.3.5), the
now (§7.3.7), and the freshness attestation (§7.3.8, §7.4), is a statement about a **state value**,
corroborated by the signatures that ride alongside it, and never a claim about **who produced it**. Two
rules make this precise:

- A governance object's identity **MUST** be a function of the state it asserts (it is content-addressed by
  that state) and **MUST NOT** include the identity of the party asserting it, because binding identity to
  the asserter is what turns two independent assertions of the same state into two rival objects instead of
  one, the fork this section exists to prevent.

- Signatures over a governance object **MUST** be treated as a corroborating grow-set attached to that
  object, unioned across signers, and **MUST NOT** form part of the object's identity, because a signature
  that changed the object's identity would make each additional voucher a new object rather than added
  corroboration of the same one, so agreement would manufacture divergence.

This is the single property that keeps concurrency safe across the whole mechanism. If two parties
independently assert the same state, "sign the state" yields **one** object with two vouchers, which union
idempotently; the "sign the authorship" alternative would yield two rival objects ("I did this" versus "no,
I did this"), which fork. The test to apply to any new governance object: if two honest parties do this
independently, do you get one thing or two. It **MUST** be one. This is why the ceiling, the now, and the
freshness attestation are concurrency-robust rather than points of contention, and it is the
mechanism-level face of the principle that the protocol furnishes corroborable state and never an
authorship claim (Part 1 §2.0, the razor: provenance, not a verdict about who spoke). `Design.`

#### 7.3.5. The membership ceiling

When a removal crosses its threshold (the revocation-authority dial of §5.7), the crossing fact records the removed member's authority endpoint,
so that the member's necessarily-frozen view is correct by construction rather than a divergence to be
repaired.

- The fact that crosses the removal threshold **MUST** stamp a **membership ceiling**: the governance head
  as-of which the removed member's read and enforcement authority ends. The ceiling **MUST** be part of the
  quorum-crossing fact, carrying that fact's k-of-n authority, and **MUST** be stamped by the completing
  signer (the member whose signature assembles the quorum, and who is therefore the first for whom the
  removal is true).

- The ceiling is a governance object per §7.3.4: it **MUST** be a fact about the removed member (a state
  value), held in the authority-plane state that current members reconcile, because that is what lets any
  current member check an actor's action head against the actor's own recorded ceiling without consulting
  the removed member or any central record. It **MUST NOT** be forgeable by the removed member, because a
  forgeable ceiling would let that member move its own authority-endpoint forward and re-authorize actions
  the group had already cut off, the enforce-a-revoked-authority failure seen from the record side; it is
  unforgeable by construction, since the removed member is not the completing signer and cannot assemble a
  quorum excluding the required others.

- A removed member's authority **MUST** be treated as ending at its ceiling. Its view beyond the ceiling is
  correct-by-construction and requires no reconciliation: a member locked out of a scope is expected to
  hold a frozen view, and the protocol owes it no consistency past the ceiling.

- Where concurrent completing facts stamp ceilings at different heads **within one lineage** (all nodes
  agree the removal occurred, differing only on the exact head), the ceilings **MUST** union as concordant
  assertions (§7.3.4), and the canonical ceiling head **MUST** be selected by the §7.3.1 concurrent
  tiebreak. Where nodes hold the same facts but genuinely disagree on whether the removal occurred at all
  (one lineage stamps a ceiling, another stamps none), the ceilings do **not** union: the disagreement is
  real and diverges into separate lineages via the ban/fork unification (§7.6.4). So concurrent ceilings
  union within a lineage and diverge across a fork; forced concordance across a genuine disagreement is
  never attempted.

The ceiling is the completeness anchor for membership. It converts the dangerous case (a still-connected
node acting on a stale membership view) into a checkable one: any current member can check an actor's
action head against that actor's recorded ceiling (the enforcement check of §7.3.8), and an action at or
beyond the ceiling is void. It does not by itself close general completeness (that remains the single
load-bearing beam, Appendix B); it closes the membership-entitlement question specifically, which is the
safety-critical one because the dangerous direction is enforcing a revoked authority. `Design.`

*Running example: once Bob's removal crosses its threshold he cannot be re-admitted above the ceiling the log
records, so his necessarily-frozen view stays correct by construction (walkthrough beat E7).*

#### 7.3.6. Decision versus enactment: recognition is free, enforcement is exclusive

A removal, or any authority change that also changes keys, is two coupled events on two planes that behave
oppositely under concurrency, and they **MUST** be treated separately. Conflating them, enforcing on the
epoch chain as if it were as cheap and concurrency-safe as folding the decision, would reintroduce
epoch-commit contention into what should be a cheap fold, and would make a lost or duplicated enforcing
commit look like a lost or duplicated *decision*, which it is not.

**Recognition is not exclusive.** Threshold satisfaction is an objective property of the assembled facts,
not a decision any one party makes. Any member observing the assembled quorum **MAY** recognize it and emit
the corresponding effect (the ceiling of §7.3.5, or the enactment below). Recognition **MUST NOT** be
reserved to a single designated party for correctness, because reserving it would make the group depend on
that one party to notice a fact any member can independently and correctly conclude: if the designated
recognizer drops out, an objectively-complete decision stalls with no one permitted to act on it, a single
point of failure manufactured for no reason. Multiple concordant recognitions **MUST** be treated as
corroboration, unioned per §7.3.4, not as competing facts, because they assert one identical state, and
treating them as rival facts would fork the group *on agreement*, the exact failure §7.3.4 exists to
prevent. Recognition is therefore free under concurrency, up to every member independently concluding "my
signature completed the quorum"; its cost reappears only at enactment.

**The decision folds; the enforcement commits.** The **decision** (the quorum-crossing fact plus its
ceiling) lives on the fact plane: it folds per §7.3.1, unions per §7.3.4, and is concurrency-safe and
cheap. It **MUST** fold independent of, and prior to being conditioned on, the enforcing commit, because
binding the decision to its enforcement would drag epoch-commit serialization back into the fold and stall
an already-agreed decision behind a key operation that is only a mechanical follow-on. The **enforcement**
(the MLS commit that removes the member's leaf and rekeys) lives on the epoch chain, where only one commit
closes an epoch (`Verified-RFC`, RFC 9420/9750). Concurrent enforcing commits therefore collide and
serialize; multiple concurrent commits for the same decision **MUST** converge to the same epoch state, and
they do, because they are idempotent in effect (each performs the same removal and rekey; a loser rebuilt
on the winner's epoch finds nothing left to do). A single enactor SHOULD perform the commit to avoid
redundant rekeying and epoch churn, but correctness **MUST NOT** depend on enactor uniqueness, because the
commits are idempotent in effect: were correctness to depend on uniqueness, a benign duplicate commit (two
enactors acting on the same decision) would become a divergence instead of a harmless retry, converting an
efficiency concern into a safety one. Only efficiency turns on uniqueness. `Design`, with the
single-commit-per-epoch fact `Verified-RFC`.

**The enactment dial.** Who fires the enforcing commit is a deployment dial that trades epoch churn against
enactment latency and never affects correctness, and the reason it is safe is the capability/authority
split: the enactor holds no authority, it executes a decision the quorum already made, so designating one
or escalating a fallback composes with peer-symmetry and creates no center. The completing signer SHOULD be
the enactor (fewest commits). If no enforcing commit is observed within a configured interval, a fallback
enactor MAY act, and the fallback set is the dial: the k signers only (bounds redundant commits), or any
member (fastest under partition, largest potential herd), escalating over time so broad enactment is
reached only when narrower enactment demonstrably did not occur. The signal a fallback reads is the now's
in-flight tally (§7.3.7): a completed threshold with no enforcing commit can only mean the completer
crossed it and then dropped out before enacting. A fallback collision is safe and merely wasteful
(idempotence again), so a node MAY fall back without global certainty that no commit exists. If no eligible
enactor is present, the removal **MUST** remain a valid **decided-but-unenacted** state with the ceiling
governing authority in the interim, because discarding a decided removal merely because it cannot yet be
cryptographically enforced would let the revoked member's authority silently revive, which is the very
divergence the decision was meant to close; a removal awaiting an offline enactor is honest and safe, not a
failure. Defaults and intervals are Open (Appendix B).

**The two-phase interval.** Between decision and enactment there is a bounded window whose semantics
**MUST** be stated, because authority and cryptographic access diverge within it and an unstated window is
where a node would guess. The ceiling governs authority from the moment the decision folds: from that point
the removed member's actions **MUST NOT** be honored by any node that holds the ceiling, because honoring
them would enforce an authority the group has revoked, the single safety-critical failure (enforcing a
withdrawn grant) that the ceiling and the finality gate exist to exclude. The enforcing commit governs
cryptographic access from enactment; until it lands, the removed member may retain the ability to decrypt
current content. So the interval is a window in which a governance-revoked member may still decrypt but has
no action honored, which is safe under the retained-copy floor:
revocation protects the future, not the past, and a bounded window in which a just-revoked member can still
read content it would have seen anyway is within the existing threat model. The ordering is deliberate,
decide first, enforce second, and enforce authority from the decision. `Design.`

#### 7.3.7. The now: the materialized current state over the chain

The history chain is the append-only audit record; reading it end to end to answer "who is a member right
now, what thresholds are in force, what is pending" would be prohibitive. The governance plane therefore
**MUST** maintain a **now**: the materialized current state (the settled slot values, membership, roles,
thresholds) together with the in-flight quorum tallies (per pending change, its target, its current count,
the threshold in force, and whether enactment has occurred). It **MUST** be maintained because routine
reads and fallback-enactment decisions must be answerable without replaying history, which is the now's
entire purpose and the authority-plane analogue of the declarative snapshot (§7.3.3): the chain is the
record, the now is the operating pointer over it.

- The now **MUST** be bound, by reference, to the history-chain head it is derived from, and **MUST** be
  verifiably derivable from the chain, because a current-state that floats free of the chain is trusted
  rather than checked, the exact posture §7.3.3 forbids: a holder of the chain tail can confirm the now
  rolls up from it, and a holder of the now can confirm the chain tail is consistent with it. A now and the
  relevant chain travel together in dataplane exchanges, so that what is offered as current always arrives
  with the means to check it.

- **Genesis-derivability is the integrity floor.** A now **MUST** be re-derivable from the history chain
  from genesis, folding every fact from the beginning and checking that it arrives at this now, because
  that is the one check that needs nothing external: a node can self-certify a now in complete isolation
  from raw signed history alone, and so is never required to trust a state it did not itself verify.
  Checkpoints are the acceleration on that floor, not a substitute for it. A node that held the history
  records a checkpoint over its own verified fold and **MAY** thereafter discard raw history before it
  (§7.3.3 truncation) and fold incrementally from the checkpoint rather than from genesis, which is
  caching its own verified result, not trusting another's summary. Accepting another node's
  quorum-corroborated checkpoint is an explicit, optional trust decision a fresh node **MAY** take to skip
  the genesis fold when corroboration is available, and **MUST NOT** be a base-case dependency, because
  under isolation or partition the node must fall back to the always-possible genesis fold. This settles
  derivation and is orthogonal to completeness: genesis-derivability establishes whether a now is a
  correct fold of the history a node holds, not whether that history is complete to the group's leading
  edge, which remains the completeness-ahead beam (§7.3.3, Appendix B). `Design`; the byte-level encoding
  of the now and its checkpoint is `[gates-release]`.

- The now **MUST** be a replaced current value, not an accumulating one, because the append-only audit is
  already the chain's job, and a now that grew without bound would duplicate the chain while adding no
  auditability. The now advances as facts fold; the chain, not the now, is what lengthens.

- Advancing the now **MUST NOT** itself trigger an epoch change, because if it did, every role edit,
  threshold change, and tally increment would force a rekey and split the audience, collapsing the
  decoupling of epoch from governance that keeps cryptographic commits rare (the scaling property, §7.9).
  Only a key-changing operation commits (§7.3.6); a governance fact that changes no key advances the now at
  zero epoch cost.

- Concurrent nows derived from divergent chain heads **MUST** reconcile by re-derivation to the same later
  now, because two nodes acting on divergent current-states is exactly the divergence the fold exists to
  prevent: a reconciling node folds the facts it was missing and re-derives, and because the fold is
  order-independent (§7.3.1), both nodes land on one now. Nows are comparable by the chain head each is
  bound to, so "which is later" is decided by causal precedence, never by wall-clock or arrival.

- The now **MUST** be signed as an attestation over its state value per §7.3.4, and concurrent signatures
  over the same now **MUST** union as corroboration and **MUST NOT** be treated as one distinct object per
  signer, because a now-per-signer would manufacture rival current-heads out of agreement, the forked-head
  failure §7.3.4 exists to exclude: two honest nodes independently attesting the same current state must
  yield one signed now, not two.

The now is what makes the authority plane operable at speed without giving up checkability: fast to read,
cheap to advance, bound to the record that proves it, and convergent under concurrency. What it
deliberately does not provide is a guarantee that it reflects every committed fact that exists somewhere
unseen; that is the completeness question the freshness cursor (§7.4) and the finality gate (§7.3.8)
address, and the single load-bearing beam (Appendix B) that remains. `Design.`

#### 7.3.8. The finality gate: irreversible action fails closed

§7.3.3 draws the read/enforce line: reads run on best-known state, enforcement runs on final state. This
section states the gate that line implies as a single normative clause, so the sections that invoke it (the
ceiling of §7.3.5, the enactment of §7.3.6) have one place to point rather than restating it.

- A node **MUST NOT** treat a governance decision as final, for the purpose of an irreversible action,
  unless it can establish that it has seen every committed fact bearing on that decision up to the relevant
  point. If it cannot establish this, it **MUST** fail closed, stalling that one action, and **MUST NOT**
  fail open by proceeding on possibly-incomplete state. The grounding is the asymmetry of the two errors:
  proceeding on incomplete state can enforce an authority the group has already revoked or overturned (the
  enforce-a-revoked-authority failure, the safety-critical direction), which is irreversible once the
  action lands, whereas stalling is recoverable the moment the missing facts arrive. Where the two errors
  are not symmetric, the gate MUST resolve toward the recoverable one. This is "delay over breach."

- Finality for the action is established by the freshness cursor (§7.4): the cursor is what lets a node
  conclude that no later head bearing on this decision exists that it has not seen. For a membership action
  specifically, the relevant point is the actor's ceiling (§7.3.5): an action at or beyond a recorded
  ceiling is void regardless of finality, and an action below it is subject to this gate. The gate
  therefore composes with the ceiling check and does not replace it.

- The gate binds only the irreversible action, and **MUST NOT** be extended to reads or to the content
  plane, because a node that halted all activity on any completeness doubt would fail far more often, and
  far more destructively, than one that keeps reading and serving while holding back only the unrecoverable
  step. §7.3.3 already establishes that reads run on best-known state; this clause narrows the fail-closed
  posture to exactly the actions that warrant it.

What the gate cannot do is turn corroborated completeness into proven completeness: a node establishes "I
have seen all committed facts to this point" by the freshness mechanism, which corroborates but does not
prove the absence of an unseen committed fact in the general case. That residual is the single load-bearing
beam the design still carries (Appendix B). The gate is shaped so that the beam bears only on liveness (an
over-cautious stall), never on safety (a wrongful enforcement), which is the most the design can honestly
claim until the beam is discharged. `Design.`

*Running example: an irreversible action premised on Bob's removal fails closed until that removal is final,
never acting on a merely probable state (walkthrough beat E7).*

#### 7.3.9. Principal recovery and break-glass (TBD)

`Design.` This subsection is a stub: it fixes the shape and the one binding invariant, and defers the mechanism.

Succession is not a distinct primitive. A Group Role is not intrinsic to its holder (§5.2), so a lost or vacated role, including the root, is reassigned by the ordinary fold (a revoke-then-grant that folds and converges) or, if contested, becomes a fork with full history on both sides (§7.6). A vacant role is not a brick, because survivors can always fork and re-establish authority among themselves (the exit and fork floor, §5.3). What remains is one primitive: recovering a principal whose key material is lost (death, lost keys, incapacitation), without letting the recovery path become a center.

Recovery is a ladder by strictness. From a limited-read custodian over a shared recovery secret (a capability, not authority, the standing of §2.7), through a k-of-n guardian threshold, to a time-delayed contestable break-glass, with a survivor fork as the floor when no recovery quorum remains. The strictness rungs trade availability against resistance to abuse.

The one binding invariant, which any recovery mechanism specified here MUST satisfy: recovery restores or reassigns exactly the lost principal's authority and never more, because a recovery path that could grant more than the lost holder held would be an authority no quorum bounded and no fork escaped, which is a center (§2.7); with a capped root, even root recovery is so bounded. The recovery delegate is itself revocable and forkable, like any role, so it is not a center either.

Deferred: the concrete trigger, the guardian set and threshold defaults, the break-glass delay and contest window, and the recovery secret's encoding. This is a known pattern (social and threshold recovery), not open research. The properties a specified mechanism must pass are enumerated in the consolidated experiments file, Stage 8 (recovery delegation and break-glass).

### 7.4. Freshness: no false "current"

A participant/helper **SHOULD** periodically emit a signed, **content-free** tip beacon
`{scope_id, epoch, head, seq_high, sig}` (head/epoch/routing only, safe for a meer). A participant
**MUST** track time-since-last-heard *locally* (liveness is a local measurement, never trust in another
participant's wall-clock) and **MUST NOT** display a view as "current" unless it is both caught up to the
best-seen tip **and** has heard a beacon within the tier's freshness horizon; otherwise the view **MUST**
surface as "behind" or "unverified." Silence **MUST NOT** be rendered as currency, because presenting the absence of new data as an
up-to-date view is exactly the false current this section exists to prevent. *Modeled.*

> **What freshness can and cannot establish, and why this is a Part 1 §2.0.1 consequence.** No node can know
> what is *most current* in a center-free design, "most current" presumes a global vantage no node has,
> and it would have to rest on the very wall-clock Part 1 §2.0.1 rules out. But a node *can* establish two things
> that **are** corroborable provenance: **liveness over a window** (which peers emitted beacons recently,
> measured by the node's own local elapsed time as a private input, never as a shared clock) and **causal
> independence** (whether two diverging facts are concurrent or one references the other). The protocol
> therefore never asserts "this is the latest"; it asserts "I have heard from these peers within my own
> measured window, and these facts stand in this causal relationship"; both of which a peer can defend.
> Currency is not provenance; liveness-over-a-window and causal-independence are. This rule is also the
> mechanism realization of the **legibility** property of field-integrity (Part 1 §2.6): refusing to
> render silence or a stale view as currency is, at the relational layer, the protocol declining to
> present a partial slice as the whole and current field, which is what keeps a persona's **voice** a right
> rather than a center-shaped capacity.

**Membership/governance acts require strict CURRENT + corroboration.** To originate or co-sign an
add/remove/policy-change, a participant **MUST** be (a) caught up and (b) corroborated-fresh, agreement on the
same head from ≥k distinct lineages observed stable, and after any unverified lapse re-checked at signing.
Ordinary content has no such precondition (it MAY be authored from a behind/unverified view, honestly
labeled). This **narrows, does not close**, the fresh-but-wrong-partition window; the residual is the §7.6
hard-stop's, by design. *Design, decided; tests specified, not yet run.*

**The returning-member catch-up: the `(G, D)` cursor and the verify-fold-escalate sequence (Fig. 2).** A
member that was offline rejoins and reconciles against whatever it can reach (a meer in Mode 1, §6.6.2;
reachable peers in Mode 2, §6.11.2). The returning member reports a **`(G, D)` cursor**, the position it
last held on the **governance** stream (`G`) and on the **dataplane** stream (`D`), and drains what it
missed from that cursor forward. The source of the commits **does not matter to validity**: a commit
arriving from a meer, a peer, or the gossip overlay is checked identically, because a forged one will not
verify. The local sequence, drawn in Fig. 2, is:

- **Verify each commit against group state** (signature + binding to the prior epoch) and **apply
  forward-only**. An invalid commit is **dropped and logged attributably**; a validity failure is the
  boring case and is **not** an escalation (§7.5.1).

- **Fold the valid, non-conflicting commits** by the monotonic merge (§7.3): no clock, no human.

- **Test concurrency** for any pair of valid operations (neither in the other's causal history, §7.3.1).
  Sequential operations are not the residue; only genuinely concurrent, mutually-exclusive-over-standing
  operations are.

- For a concurrent contradiction, apply the per-Group **benign-vs-dispute tolerance** (§7.4.1). A
  provably-benign sync artifact is auto-reconciled or held as a **stale-but-honest** view pending
  corroboration; a case that cannot be proven benign **escalates to humans with full provenance** (§7.6),
  where the machine annotates and does not decide.

The safety property the cursor inherits from the monotonic fold (§7.3): a returning member that cannot
reach a complete backlog **under-authorizes** (acts on less) but never **mis-authorizes** (acts on a
reverted or forged state). Incompleteness is therefore always safe, which is why meer-absent and
total-absence windows (§6.11) degrade governance currency to a corroborated estimate rather than to a
hazard. *(Design; the byte-level `(G, D)` cursor and checkpoint encodings are `[gates-release]`, Appendix B. The
catch-up validity check reuses the §7.3 fold and §7.5 attributable-acceptance machinery, adding no new
authority-bearing input.)*

*Running example: Dave's stale view is never presented as the current one; freshness is what keeps his
catch-up honest until it completes (walkthrough beat E9).*

#### 7.4.1. The false-positive tolerance is a governed utility judgment, not a constant

A genuine concurrent contradiction (a real social dispute the fold cannot resolve, §7.6) and a benign
**sync artifact** (two peers momentarily diverged because neither had yet seen the other's facts) can look
identical at the moment of detection, §7.5.1 notes backdating cannot be defeated by cryptography alone,
and the same ambiguity applies to honest concurrency. If every concurrent divergence escalated, the §7.6
human channel would drown in false alarms and lose the trust the entire algedonic posture depends on
(Part 1 §3, Beer). If escalation were too lax, a real contradiction would be silently auto-reconciled,
which would re-open the manufactured-resolution surface §7.3.1 closes.

The resolution follows the razor (Part 1 §2.0, Part 1 §2.5). The **machine computes the provenance signals**,
concurrency vs causal-dependence, liveness-over-window (§7.4), and the magnitude/shape of frontier
divergence, all corroborable. **Whether a given concurrent contradiction is treated as benign-and-safely-
auto-reconcilable versus escalate-to-humans is a per-Group governed tolerance over those signals, not a
hardcoded constant**, because the benign-vs-deception distinction is ultimately a utility judgment and is
vulnerable to alarm-fatigue, so it must be tunable to the Group's threat model, temperament, and need.

Two normative guardrails keep the tolerance honest:

- The tolerance governs only **when to escalate versus when to auto-reconcile a *provably-benign*
  concurrent case**. The boundary of "provably benign" **MUST** stay cryptographic/causal, never
  heuristic, a tolerance **MUST NOT** auto-resolve a case it cannot prove benign, because a too-loose
  tolerance is an attacker's instrument.

- Where a case is genuinely ambiguous (cannot be proven benign and cannot be proven a real contradiction),
  the safe default **MUST** be to **escalate** (§7.6): a false alarm costs attention; a silent false
  resolution costs the integrity the protocol exists to protect.

The setting itself is a threat-model judgment; this specification states the axes (fatigue-risk vs
silent-false-resolution-risk) and **declines to pick the default value**; that is a Group's call, and a
per-Group governed policy fact like any other. What the spec leaves to implementation is twofold: the
**granularity of the knobs** exposed and the **shipped defaults**; both are tuning decisions to settle
against a real deployment, not protocol constants. *Design.*

*Running example: whether the Alice/Bob contradiction is treated as a benign sync artifact or a genuine
dispute is the Group's own governed tolerance, not a fixed constant (walkthrough beat E6).*

#### 7.4.2. Two MLS recovery hazards the corroboration model dissolves

Two hazards MLS names for a recovering node lose their force here, and it is worth showing why, because in
each the MLS hazard is real and the Drystone answer is not a patch but a consequence of the fold already
specified.

**A stale `GroupInfo` cannot defeat post-compromise security.** When a client cannot process a commit, a
common MLS recovery is to rejoin by external commit using a published `GroupInfo`, and stock MLS treats
that `GroupInfo` as authoritative. An adversary who feeds a victim an unprocessable commit and then serves
a stale `GroupInfo` can drive the victim to rejoin into a superseded (possibly compromised) epoch,
defeating PCS, or serve a corrupted one to block rejoin (a denial of service). *(Verified-RFC, RFC 9750 §8.1.4,
§5.3.)* In a center-free mesh this is sharper, because any peer, not one identifiable DS, can serve the
`GroupInfo`. The hazard exists because in stock MLS the `GroupInfo` is the only thing the rejoining client
can check against. **In Drystone the `GroupInfo` is not the authority; the governance chain is.** A
rejoining node treats a `GroupInfo` as a *claim* and corroborates it against the authoritative chain it
already holds; a `GroupInfo` pointing at a membership or epoch the chain has moved past is detectable
locally, with no trusted third party. Two distinct defenses discharge the two halves: the **monotonic fold**
(§7.3) means a forged governance assertion cannot roll the group back, it only fails to advance in step, so
the asserter forks itself out (a fork of one) rather than corrupting anyone; and the **per-Group threshold**
(§7.2) quantifies the attack MLS leaves unquantified, mounting it costs compromising a quorum of cosigners,
not one member, and the threshold is the same dial Part 1 §2.3 sets by context (a family joined in person by
QR runs low; an activist group runs strict). The residual carried to Appendix B: a rejoining node far enough
behind on the governance chain that its own view has not advanced past the epoch a recent-but-superseded
`GroupInfo` points at. The §7.4 under-authorize-never-mis-authorize property suggests this holds, but it is
reasoned, not proven. **[confirm.]**

**Insider replay and nonce-reuse-on-restore are isolated by out-of-band convergence.** MLS does not protect
against replay by insiders (a member can re-inject an old application message; the per-sender counter
detects a gap but does not prevent replay), and it carries a `reuse_guard` because a client that reverts to
earlier state can reuse a nonce and break AEAD. *(Verified-RFC, RFC 9750 §8.6; RFC 9420 §6.3.1, §16.7.)* Both
hazards are the same shape, old bytes re-entering the live protocol stream, and both are dangerous only if
the durability-and-history layer shares a stream with live protocol operation. It does not: **gap-aware
history convergence runs out of band** (§6.8.1), reconciling the durable hash tree by anti-entropy and never
re-injecting into the live MLS stream. A replayed old message therefore arrives as a content-addressed
history-tree entry to be reconciled, idempotent and non-advancing (the monotonic fold, Part 1 §2.2), not as a live
MLS message to process, so it is inert. The same separation covers nonce reuse: recovery restores
history-tree state on the out-of-band layer, never a live group's ratchet or secret-tree state resumed in
place. The residual carried to Appendix B: this holds only if the recovery model is always "re-plant or
re-join fresh, converge history out of band" and never resurrects a live group's epoch secrets in place. If
any path does the latter, the hazard returns. **[confirm.]**

#### 7.4.3. The governance-generation stamp: every data-plane entry self-locates against the authority chain

The beacon and the `(G, D)` cursor let a node measure freshness from signals it seeks out. A complementary
mechanism lets any data-plane entry a node receives announce, by itself, whether the node holds the
governance state that entry was authored against, so a reader can detect a nameable governance gap and
locate a re-key boundary without consulting any central record.

- **The stamp is a governance-generation counter.** Each data-plane entry **MUST** carry a
  governance-generation stamp: a monotonic integer advanced by one with each governance fact on the
  authority chain, recording the generation the author held when authoring the entry. A counter is used
  rather than a wall-clock or a Lamport timestamp because the authority chain is a hash-linked chain of
  monotonic decisions, self-ordered by its own linking, so it needs no Lamport clock to order concurrent
  authors, and a monotonic integer is sufficient, cheaper, and maps onto the MLS epoch integer already
  carried by delivery. The MLS epoch is the key-currency anchor carried by delivery; the generation stamp
  is the authority-currency anchor carried by the entry; a node consults both. `Synthesis.`

- **The currency check is an integer comparison.** A node folded to governance generation `n_local`,
  reading an entry stamped `n_msg`, compares the two. If `n_msg` is at or below `n_local`, the node holds
  at least the governance state the entry was authored against and **MAY** interpret and enforce against the
  entry with no governance gap. If `n_msg` is above `n_local`, the node has a nameable gap of known size and
  **MUST** converge the missing governance before treating the entry as authoritative or re-keying against
  it. The entry thereby self-announces the governance facts the reader is missing. `Synthesis.`

- **What it closes, and the residual it does not.** The stamp closes the detectable-from-traffic case: a
  governance fact that any received data-plane entry depended on is announced by that entry, turning an
  invisible gap into a sized, named one that must be filled before acting. It does **not** close general
  completeness, because a governance fact that no data-plane entry has yet depended on can still be silently
  absent, which is the completeness-ahead beam (§7.3.3, Appendix B). So the stamp converts the
  behind-via-traffic case from undetectable to detected and leaves exactly the unreferenced tail. `Design.`

- **A locator, never an authorization input.** A stamped generation is the author's own assertion about
  their governance frontier and is forgeable, so it **MUST** be used only to locate, to detect a gap, and to
  read the re-key boundary, each then confirmed against the authority chain the node folds independently. It
  **MUST NOT** authorize on its own and **MUST NOT** rest on a wall-clock, because a stamped value that
  authorized would be a forgeable fast path, which the §2.0.1 razor rejects. `Synthesis.`

- **Encoding.** The entry header **MUST** carry the generation as a currency locator, and **SHOULD**
  delta-encode it: an entry carries the value explicitly only when it differs from the author's previous
  stamped value and inherits it otherwise, because governance frontiers are stable across long runs of
  content, so the explicit stamp costs bytes only at the transitions where the re-key boundary is actually
  consulted. Whether the wire value is the counter alone or the full governance head hash is an encoding
  choice deferred to Appendix B: the counter suffices for the currency check, while the full head hash would
  additionally let a reader verify the governance content from the entry alone. `Design`; the byte-level
  encoding is `[gates-release]`.

*Running example: a message from a member several generations ahead arrives stamped above the reader's own
generation, so instead of being read as current it names a governance gap of known size that the reader
fills before enforcing against it.*

### 7.5. Attributable acceptance and the regress-free fold

#### 7.5.1. Attributable acceptance (R6)

The guarantee is **detection and attribution, never prevention.** The acceptance record is a governance
fact signed by the accepting participant, whose signed body includes (1) the accepted entry's content digest
(binding *what* was accepted, via the authorized-write hook), (2) a **frontier commitment**, a commitment
over the set of governance-fact digests the participant claims as its synced frontier, signed as part of the
acceptance body (signing over prior signed state rather than a mutable timestamp), and (3) the participant's own
prior acceptance-record digest, chaining its acceptances.

Against the attack of lying about one's knowledge state: **frontier omission** is defeated cryptographically
(the commitment pins the set; the omitted revocation is provably in or out), and **equivocation** is
defeated by the per-persona acceptance chain (two signed chain heads with the same predecessor are
non-repudiable proof). **Backdating** cannot be defeated by cryptography alone, there is no trustworthy
internal clock (Part 1 §2.0.1, again: a node's own clock is not corroborable), so the bound is **causal**: if the
revocation is in the causal history of any fact the participant's frontier includes, the "didn't have it" claim is
refuted; the only residual is a participant genuinely causally independent of the revocation, which is the
legitimate concurrent-partition case R4 exists to bound (by epoch/generation, not by time). Every stale
acceptance therefore resolves into exactly one of two categories, **knowingly stale** (full attribution)
or **concurrently stale** (no fault, R4-bounded), with no third category where a participant silently escapes both
prevention and attribution. *Design.*

> `[gates-release]:` The frontier-commitment construction (a Merkle root over sorted governance-fact digests), the
> acceptance-record wire format, and the per-persona chain linkage must be byte-specified before interoperation.

#### 7.5.2. Breaking the authority-ordering regress, and closing the resolution input set

Ordering conflicting facts by "issuer authority rank" appears circular, resolving authority would require
already-resolved authority. Drystone breaks the regress by **computing the order from causal structure
alone, then evaluating authority in a single forward pass** along that fixed order; authority is never
consulted to *produce* the order, only checked against the partial state the order has already built.

- **The ordering spine is causal, not authoritative.** Every governance fact references its causal
  predecessors (the frontier it was issued against). This forms a finite acyclic DAG; the resolution order
  is a topological sort over it (Kahn's algorithm), requiring no authority judgment.

- **The tiebreak is cryptographic, not temporal and not authoritative.** Ties break solely by the digest of
 the canonical fact encoding (no power, no clock) deterministic on every peer (§7.3.1).

- **The forward pass checks authority against partial state.** Folding left from the unconflictable founding
  fact (§7.3), each fact is admitted iff authorized by the authority accumulated so far. A grant that was
  itself never admitted cannot authorize a later fact.

It **terminates** (a single linear pass over a finite sorted list; the accumulator only grows; no fixpoint
iteration) and **converges** (every step is a deterministic function of order-independent inputs grounded
in an unconflictable base case; a lagging participant under-authorizes rather than diverging). *Design.*

**The resolution input set MUST be closed under two distinct relations before sorting. These guard two
different properties and neither subsumes the other.**

- **Frontier-closure (authorization-guard).** The input set **MUST** be closed under "facts named in any
  included fact's causal frontier", i.e. include, recursively, every grant that *authorizes* a fact in the
  set. Without this, an implementation can admit a fact whose authorizing grant it failed to include. This
  is backward-reachability (ancestors), and it guards **authorization soundness**.

- **Conflicted-subgraph-closure (convergence-guard).** The input set **MUST** additionally be closed under
  the *conflicted subgraph*: every governance fact lying on a causal path **between** two conflicting facts.
  Equivalently, contract the conflicting facts into a single node and take the strongly-connected component
  containing it; or compute it directly as the **intersection of the backward-reachable and
  forward-reachable sets** of the conflicting facts. Without this, two honest peers that saw a different
  in-between fact can fold to different authority states even with identical frontier-closure and identical
  sort rules. This guards **convergence**.

> `[gates-release]:` **Frontier-closure-and-subgraph-closure before sorting** is the **single most likely place
> for two implementations to disagree** and therefore the place the spec must be most exact. The
> conflicted-subgraph-closure rule is **adopted from Matrix State Resolution v2.1 (MSC4297)**, whose
> "conflicted state subgraph" mechanism added exactly this closure to fix CVE-2025-49090; the
> strongly-connected-component characterization and the forward-backward-intersection computation are
> theirs (Matrix's Change 1, begin from the empty/unconflictable base, Drystone already satisfies via
> the §7.3 founding-fact base case). **Adopting their closure does not adopt their ordering**: Drystone
> keeps its own content-address tiebreak and rejects power-in-the-comparator and the wall-clock tiebreak
> (§7.3.1). The two are separable, the closure is a convergence prerequisite independent of how ties
> break. *(Verified this revision against the Matrix State Res v2.1 implementer's guide: the two changes
> are exactly (1) start the iterative auth checks from the empty set and (2) replay the conflicted state
> subgraph, characterized as the strongly-connected component containing the contracted conflicted
> supernode and computed by intersecting the forward-reachable and backward-reachable sets.)*

> **A failure mode specific to Drystone's monotonic fold.** If a participant omits an in-between fact (incomplete
> subgraph-closure), Drystone's monotonic fold will **not** produce Matrix's *reversion*; it produces two
> honest peers stuck at **different heads**, which §7.6 would then surface as a *contradiction*. So an
> incomplete-closure bug manifests as a **false trip of the human-escalation channel**, firing the
> algedonic alarm on what is actually a sync artifact, which is exactly the alarm-fatigue risk §7.4.1
> governs. This is why subgraph-closure is normative and byte-specified, not best-effort: the cost of
> getting it wrong is paid in the trustworthiness of the escalation channel, the one thing §7.6 cannot
> afford to erode.

### 7.6. The reconcile hard-stop and re-formation fork

When two histories merge, an implementation **MUST** detect membership contradictions (e.g.
removed-then-included) and **hard-stop**; it **MUST NOT** silently auto-resolve (last-writer-wins or
otherwise). Resolution is a social/governance input, not an automatic merge. The sanctioned exit for a
minority is a clean, attributable **re-formation fork**: a differently-shaped Group that preserves history
and provenance to the point of departure and legitimizes/erases nothing retroactively. Stripping a helper
operated by a cooperative or external operator simply **detaches** the Group into a differently-shaped one
(unpreventable anyway, the operator can always leave); the protocol only preserves history and provenance
to the detachment. *Verified (contradiction hard-stop; identical reformed genesis across independent
peers).*

This hard-stop is Drystone's **algedonic channel** (Part 1 §3), and it is the realization of the **forced
terminus** of Part 1 §2.5. Rather than letting an automatic rule resolve a case it cannot safely resolve,
the protocol raises the hard case to the humans who hold the context, the formal version of "specify only
somewhat, then escalate the residue." It is a *designed* channel, not a failure path: the protocol commits
in advance that genuine contradiction is a human-adjudicated event, because no merge rule can be trusted to
absorb it silently. Crucially, the escalation keeps **both the signal and the authority local**; it
surfaces the conflict to the affected Group rather than relocating the decision to a center that lacks the
context that made the conflict legible.

**Why the terminus is a fork and not a verdict** (Part 1 §2.5, made mechanical). The residue that reaches
this hard-stop is the set where **provenance is fully determined and utility is still open**: both
contradicting facts verify, the causal structure is symmetric, and "who should remain" is not a question
with a discoverable truth-value; it is a question about what the people want. A verdict would presuppose
an answer the loser should accept; the fork presupposes there was none, and lets divergence persist as two
communities. When even the humans cannot agree, irreducibly wanting different things, both legitimately;
the protocol's last service is to make the split **clean** (history and provenance preserved to the point
of departure), not to manufacture a consensus that was never available. The machine's job ends at
*surfacing* the contradiction with full provenance; the humans supply utility; and the fork is what honesty
requires when utility itself is contested. *(This is also why the §8 posture is label-not-enforce: the same
move, one layer down.)*

*Running example: where Alice and Bob genuinely disagree, the protocol hard-stops rather than auto-merging,
because manufacturing one answer would impose a fiction (walkthrough beat E6).*

#### 7.6.1. The escalation set has two members, not one

The residue Part 1 §2.5 forces to humans has **two shapes**, and a mechanism that watches for only the
first misses the second. Both share the defining property, provenance fully settled and utility open, the
razor's seam (Part 1 §2.0); they differ only in how they are detected.

- **Contradiction: too many valid claims.** Concurrent conflicting operations that all verify: Carol and
  Bob, two personae at equal standing, each committing the other's removal against the same epoch; or a
  removed-then-included merge. Detected as concurrent commits that will not linearize. This is the case the
  hard-stop above catches.

- **Under-determination: too few valid claims.** A required Group Role vacant with no valid grant
  available. Suppose Dave holds the sole admin Group Role and his only node goes stale and is pruned
  (§7.4); the survivors need the Group Role filled and cannot agree who fills it. There is no conflicting
  act to order, there is an *absence* no cryptographic operation should fill. Detected as a required Group
  Role vacant with no admissible successor.

Under-determination is **expected, and deliberately given no technical resolution**, because filling the
vacancy by a tree operation the survivors did not agree on is a center certifying utility (Part 1 §2.0). What the
protocol supplies is the conditions for cheap human resolution, not the resolution: **legibility** (every
node deterministically computes the same picture, Group Role vacant, these survivors, this governance
posture, per P-Knowable-Truth, Part 1 §2.2), **cheap exits in every direction** (re-delegate the Group Role in one
grant, run with it unfilled since the Group persists Group-Role-less by the no-helper floor of Part 1 §2.4,
or fork), and **refusal to manufacture the missing authority**. The judgment stays with the humans; the
instantiation of whatever they choose is Drystone's, and it is cheap in every direction.

*Running example: the escalation set is both Alice and Bob, not one of them, so a mechanism watching only for
a contradiction would miss the case where a required role is left vacant (walkthrough beat E5).*

#### 7.6.2. The mechanism the fork and heal run on: one primitive, three arities

A fork is not a distinct mechanism from a heal or a routine re-key; all three are the **same operation at
different arity**, which the delivery layer calls **re-plant** (§6): read current membership from the
governance chain, instantiate a fresh MLS group over it, and atomically repoint the conversation to it.

- **Legitimate governance fork** (the split is real, so two conversations are correct): plant two fresh
  groups, one per branch, each seeded from its branch's membership. One conversation lineage becomes two.

- **Accidental fork** (a benign concurrency artifact, no governance divergence): plant one fresh group over
  the reconciled membership and repoint both former branches to it. Valid **only** when governance did not
  diverge, both branches carry identical authoritative membership; any governance divergence, even one
  contested act, disqualifies the auto-heal, because healing it would manufacture the verdict Part 1 §2.5 forbids.

- **Routine re-key**: repoint one to one.

The choice of *which arity* to apply is the governed classifier of §7.4.1 (is this concurrent contradiction
a real dispute or a benign artifact), a per-Group tolerance over verifiable provenance signals, never a
constant. The mechanism is identical whichever way the classifier decides; only the decision is governed,
and it must stay governed.

#### 7.6.3. MLS is subordinate here; the conversation outlives the key layer

The reason none of this is asked of MLS is a division of layers. **The MLS group is key-distribution
infrastructure with a lifespan; the conversation persists across a sequence of MLS groups**, carried by the
application-layer structures (the dataplane history and the governance chain), and the MLS group identity is
not the conversation identity. MLS's transcript hash can represent exactly one linear commit sequence, with
no representation for a branch or a merge (RFC 9420 §8.2), so a fork cannot be expressed inside an MLS group
at all. *(Verified-RFC, RFC 9420 §8.2: each epoch chains as `confirmed_transcript_hash[n] =
Hash(interim_transcript_hash[n-1] || ConfirmedTranscriptHashInput[n])`, a strict single-predecessor chain
seeded from a zero-length string at genesis, which is exactly the "one linear sequence" property, no branch
or merge is representable.)* Rather than defeat that linearity, Drystone
keeps it where it is harmless (the monotonic dataplane, one linear sequence is correct) and declines it
where it is not (the governance residue, where forcing a linear order over concurrent non-monotonic
operations is the coordinate horn of the CALM dilemma Part 1 §2.5 rules out).

The three arities are not bespoke: they map onto MLS's own **ReInit** (the re-key arity, and the heal over
reconciled membership) and **branching** (the fork arity) operations, linked by a **resumption PSK** that
proves co-membership at the source epoch (RFC 9420 §11.2, §11.3, §8.6). This is strong external validation,
the standards body converged on the same shape, close the old group and re-form over the membership, linked
by a PSK. The PSK carries the **entitlement** thread across a re-plant; the **content** thread is carried
separately by the dataplane history (the PSK touches no content), and the two are kept distinct. *(Verified,
RFC 9420 §8.6: the resumption PSK proves prior co-membership, not content.)*

> **The one MLS-named hazard the re-plant inherits: ReInit is not atomic.** Committing a ReInit immediately
> freezes the existing group, but creating the new group and sending its Welcomes is a separate step, so a
> member can commit the ReInit and then go offline before completing the re-form, stranding the group in a
> window where the old group is dead and the new one does not yet exist. *(Verified-RFC, RFC 9750 §6.1, §7.)* In
> a center-free mesh the committer is an ordinary peer, so this freeze-then-strand window is a routine risk,
> not an edge case. The freeze-first order is a deliberate tradeoff: it purchases replay-immunity (a committed
> ReInit cannot be duplicated by a conflicting commit), at the cost of the stranding window. The candidate resolution is that the **governance chain
> records the re-plant intent (re-planting to membership M) before the freeze**, so any member can complete
> a stranded re-plant from the authoritative instruction rather than needing the original committer.
> Whether the ordering is genuinely intent-recorded-before-freeze is the open question that decides whether
> this hazard is discharged or merely bounded. **[confirm against the delivery and governance-chain
> ordering; carried to Appendix B.]**

#### 7.6.4. A ban is a fork seen from one side: one primitive, distinct artifacts

A removal by the group (a **ban**) and a member leaving of its own accord (a **voluntary fork**) are the
same underlying primitive seen from two directions. The principle side, that the right to fork is inherent
and cannot be configured away, is Part 1 §2.5 and the exitability floor (§5.9); this section specifies the
mechanism and the one place the two directions differ.

The primitive is **lineage divergence at a head**: two lineages that share history up to a point and
diverge after it, each keeping its own state, neither owed reconciliation by the other (the
diverge-across-a-fork case §7.3.5 names from the ceiling side). It has two triggers. A ban is the group
ceasing to corroborate a member, the quorum stops vouching for them going forward. A voluntary fork is a
member ceasing to require the group's corroboration, it continues from its own local state. The outcome is
identical, divergence at a head with both sides intact in their own lineage, and the mechanism is the
re-plant of §7.6.2 in its fork arity whichever trigger fired.

The two triggers are equal in outcome but **distinct in artifacts**, and the distinction **MUST** be
preserved, because it is the only thing that lets a third party read the provenance of a departure when it
later decides which lineage to corroborate (the social-utility decision). A ban deposits a quorum-stamped
ceiling (§7.3.5): a fact carrying k-of-n group authority that says "the group, by its rules, stopped
corroborating this member as of head H." A voluntary fork deposits no group-side artifact, because the
group performed no act; there may be a self-authored divergence marker in the forker's own lineage, but no
group-authority stamp, because no quorum acted. Erasing that difference would strip the social layer of its
only basis for telling "the group decided this" from "this party decided this alone," which is exactly the
provenance the system exists to furnish while staying silent on the verdict (§7.3.4). The distinction is
therefore evidentiary, not mechanical: the mechanism is one primitive, and the artifacts differ so the
record stays legible.

A ban is therefore **not a deletion**, and **MUST NOT** be implemented as one, because the group's
authority reaches only to withdrawing its own corroboration going forward, never into what the member
already holds; implementing a ban as deletion would claim an authority the group does not have and cannot
be granted (the inherent-fork floor, §5.9), and would break the guarantee that a forced fork leaves the
departing member intact. A ban forks the member off whole: it continues in its own lineage holding
everything it had up to the ceiling, and what it loses is precisely and only the group's corroboration
going forward, which is the one thing the group had the standing to withdraw. The most a concentrated
authority can do to a member is force a fork, and a forced fork leaves the member whole. This is the
mechanism-side face of the Part 1 §2.5 commitment that the terminus is a fork and not a verdict, and of the
peer-equality floor: even the group's harshest power over a member is bounded by the same limit as every
other authority, it can withhold its corroboration, it cannot reach what the member holds. `Design.`

*Running example: banning Bob is a forced fork, so it ends the Group's corroboration of him but leaves him
whole in his own lineage holding everything he had (walkthrough beat E7).*

#### 7.6.5. Three registers of response: mute, governance, fork

Not every interpersonal friction needs the group to act. Because all state is local and all presentation is
local, there are three registers for responding to a problem, escalating in cost and visibility, and the
design pushes each response to the lightest register that suffices.

- **Mute** is a purely local, presentational act. A persona **MAY** mute another persona or a whole group;
  this changes no shared state, produces no governance fact, rolls no epoch, and announces nothing. It
  **MUST NOT** produce a governance fact or roll an epoch, because a mute that changed shared state or
  announced itself would convert a unilateral presentational preference into a coercive act on everyone
  else's state, which is precisely what the governance register's quorum exists to gate. Mute affects only
  what the muting persona renders, on its own device, and is freely reversible; it preserves the group
  entirely because it touches no one else's state. It is the first reach for "I do not want to see this."

- **Governance** (a vote, a role change, a removal) is the shared register: it produces folded facts, and
  when it changes membership it enacts an epoch and splits the audience (§7.6.7). It is for when the *group* must
  change, not when one persona's view must.

- **Fork and exit** is the floor (§5.9): always available, for when a persona cannot accept the group's
  shape at all. This is the register §7.6.4 specifies as the ban-or-departure primitive.

The principle is to push each response to the lightest register that suffices, because most interpersonal
friction is a presentation problem rather than a group problem, and resolving it by mute is cheaper and
less fracturing than a vote: governance is for group change, and fork is for departure. Reaching for a
heavier register than the problem needs spends shared epoch cost and audience-splitting on what one persona
could have resolved on its own device.

Standing preferences make the lighter registers largely automatic. A persona **MAY** hold standing,
personal, presentational rules, for example "always follow this persona's lineage in a fork," "always join
both," "prioritize these personas," or "mute any group without these members-with-standing." These rules
are local, overridable in the moment, and resolve most fork placements and mutes with no prompt, because
keeping the light registers automatic is what makes proportionate response the path of least resistance
rather than a decision imposed each time. Mute-as-a-standing-rule and fork-placement-as-a-standing-rule are
the same kind of thing: local, personal, presentational defaults that resolve friction without requiring
anyone else to act. `Design.`

*Running example: Alice mutes Bob's stream as noise while Dave keeps it visible, and both are purely local
view choices that leave the shared group state identical for each of them, so from one situation two people
sit with different views and neither is the wrong one, because mute changes only the muter's presentation
and the substrate takes no group-wide stance on how a person is seen (walkthrough beat E2).*

#### 7.6.6. The fork: who lands where, and how it is announced

When a genuine same-facts disagreement escalates to a fork, the population splits into three roles, and
placement follows from role.

- **Voters, the dissenters,** are the delta. Their expressed intent is the disputed quantity, so they have
  effectively placed themselves by voting and land in the lineage their vote implies. Where a voter's
  landing is ambiguous they **MAY** be asked to clarify, because their intent is the very thing in dispute
  and no other signal settles it more truthfully.

- **Bystanders** expressed no position, so by default they are members of *both* resulting lineages
  (§7.6.8) and are offered, but not forced, a choice of where to remain. Standing preferences (§7.6.5)
  resolve most of these placements automatically and without a prompt, so an in-the-moment choice is the
  exception rather than the rule.

- **The subject of a contested removal** is placed by the *outcome* in each lineage, not by their own
  preference: in the lineage where the removal succeeded they are out (the ceiling is stamped, §7.3.5), and
  in the lineage where it failed they are in. The subject **MUST NOT** be given a "both" option for their
  own membership, because otherwise a member could dodge a legitimate removal simply by choosing the
  lineage that keeps them, which would make removal unenforceable at the one moment it matters.

Where a choice prompt is needed it **MUST** state the disagreement factually, not editorially, consistent
with the provenance principle (§7.3.4): for example, "Bob and Bernice voted to remove Tom; Tom and Sarah
voted against," with clear shorthand names for each lineage, "both" as an easy default, and "neither" made
deliberately harder to select by accident, because leaving both rooms is a real loss and should not be a
slip. The fork announcement signed on the governance change that produced the fork **MUST** carry this
factual statement of cause in the dataplane ("this group split on the vote to remove Tom"), because a fork
is a social event and is legible as one only if its cause travels with it; a fork with no stated cause
would leave members to reconstruct or misattribute why their group divided. `Design.`

*Running example: Dave and Erin, the same dispute behind them, land on opposite sides, Carol's node keeps
serving whichever side still admits it, and Bob is in his own lineage, each where their own choice puts them
and none of it a verdict the substrate rendered (walkthrough beat E8).*

#### 7.6.7. Hold, enactment, and the audience split

An epoch roll is a fork at the confidentiality layer. The moment a membership change enacts, the members
inside the new epoch can read what those outside it cannot, and "two sets of people who can no longer all
read the same thing" is what a fork is. So the governance-outcome fork and the epoch or audience split are
the same phenomenon at two layers, and the data model **MUST** honor this: the audience line the epoch
draws and the membership the governance decision reaches **MUST** agree on who is on which side, because if
they disagreed a party would be cryptographically inside the room while governance places them outside it,
or the reverse, and the crypto boundary would no longer mean what the authority record says it means.

This forces the ordering and resolves the tension between a hold posture and eager enactment:

- In a **hold** posture (§7.6.9), a detected conflict holds the *decision* (flagged, not finalized), and
  because the enforcing commit fires only on a finalized decision (§7.3.6), the hold **suspends
  enactment**: no epoch rolls, so the audience is not split cryptographically while the members are still
  resolving. The hold **MUST** act at the decision layer, before enactment, because an epoch that already
  rolled has already split the audience, so a hold that fired after enactment would hold nothing.

- The two-phase interval (§7.3.6) is what makes this coherent: decide first, enact second, so the audience
  split always follows a settled decision rather than racing it.

The separation of directed from incidental is a consideration, not a cliff: an epoch roll is *incidentally*
an audience split whether or not it was a *directed* fork, so the design treats the confidentiality
consequence faithfully (the data model reflects it) while not treating every epoch as a declared schism.
Mute and standing preferences (§7.6.5) further mean many socially-coherent separations happen with no
governance act and no epoch at all, purely in local presentation. `Design.`

*Running example: the epoch roll that excludes Bob is the mechanical form of the audience split, and a hold
can suspend enactment while the members decide (walkthrough beat E8).*

#### 7.6.8. After the fork: being in both is permanent, and merge is cheap, not required

A durable fork produces two independent, equal groups that share a past. Being a member of both is a
permanent, fully-supported end-state, not a transitional condition that must resolve. There is nothing to
reconcile, because two rooms that used to be one is a coherent standing state, exactly as a mutual friend
of two now-separated people is genuinely still both their friends. A persona in both **MAY** exit either,
mute either, or remain in both indefinitely.

- **Merge is available and cheap, but not required.** The remedy for an *accidental* fork (a timing
  near-miss) is an *intentional* reconciliation, and reconciliation **MUST** preserve history continuity
  and "room" continuity, because a group that forked over a misunderstanding and merged back should feel to
  its members like the same room that had a brief disagreement, not a destroyed group replaced by a new
  one. This is a requirement, not a nicety: without it every fork would be experienced as a small death,
  and "forks are cheap" would be false in exactly the way that matters to the people in the room.

- To support recognition and reunion, lineages **MUST** retain a shared ancestor identity that survives the
  fork, because a merge has to be able to recognize "these two are the same room, reunited" in order to
  stitch their histories, and without a surviving shared identity there would be nothing for the merge to
  match on. Because non-merger is not a problem to fix (it is simply two groups now), the merge machinery is
  a convenience for reconciliation, not a mandatory cleanup. `Design.`

*Running example: Bob's lineage and the Group can both persist permanently, and a later merge between them
is cheap but never required (walkthrough beat E8).*

#### 7.6.9. Posture, dials, and the protocol/product division

There is no single correct configuration, because respecting variety and canonical local state means
different groups genuinely want different thresholds. What exists is not right-versus-wrong settings but
better-and-worse *matches* between a group's temperament and its configuration, with every mechanism safe
in every configuration. This is the social-layer expression of Part 1's governance-concentration spectrum
(authority consolidates freely because it stays revocable and exitable): the protocol provides a safe range
and lets groups choose their point in it.

The two escalation sub-decisions left open earlier are resolved not by picking one, but as **posture dials
with temperament-keyed defaults**:

- **Auto-fork versus hold-on-conflict** is a dial. Hold-on-conflict (flag the disputed slot, suspend its
  enactment per §7.6.7, let members resolve) is the default for high-cohesion archetypes where a
  disagreement is likely a misunderstanding to talk out; auto-fork (split immediately, no human beat) is
  available for groups that prefer clean automatic splits.

- **Merge-as-routine versus fork-as-durable** is a posture dial: the *capability* to heal is always present
  (§7.6.8), and whether a group treats forks as routine-and-healed or rare-and-meaningful is its posture.

The protocol's job and the product's job divide cleanly, and this is the frame for the whole layer:

> Making it possible faithfully is Drystone. Making it representable and as easy as possible is the product
> layer.

- **The protocol (Drystone) is composable, unopinionated, and safe in every configuration.** It provides
  the mechanisms (mute, the three registers, operation-type precedence, the tiebreak-key range,
  voter/bystander/subject placement, hold-suspends-enactment, permanent-both, cheap merge) and imposes no
  social shape. Composability is why it gets complicated, which is correct at the protocol layer: it must
  support all social shapes safely rather than choose one, and a protocol that chose one shape would be
  unable to serve the groups whose temperament differs.

- **The product is opinionated, legible, and defaulted by temperament.** It chooses which dials to expose
  and defaults them to a coherent, temperament-matched posture on an 80/20 path, so most groups never touch
  a dial and the default *is* the posture. Defaults **SHOULD** be keyed to group archetype
  (friends-and-family, professional association, social or logistics group) and may vary by group size and
  trust posture, because a default does double duty, it works out of the box and it demonstrates what the
  posture means, so a mismatched default miseducates as well as misfits. The application layer owns keeping
  this discernible, since more exposed dials is more power and more UX burden.

The through-line, shared with the foundations: **the protocol faithfully represents individual, local,
personal choice, and never centrally resolves social conflict, because social conflict has no central
resolution, only individual responses that aggregate.** The anchoring example is a literal divorce with a
shared friend group. When group A and group B want to merge but A has banned a member of B, there is no
technical fix and no correct group-level answer, because the right outcome differs per person: some mutual
friends stay close to both, some pick a side, some drift. The resolution is that each persona acts as
themselves (join, exit, mute) and the group-level outcome is the aggregate of those choices, a realization
of the social adjudication rather than an input to it. Trying to make the group decide *for* everyone would
impose a fiction, the same way a central host resolving a divorce's fallout would. So the default dials
**SHOULD** mostly orient toward preserving the group and letting exit shine as the most empowering right,
because when in doubt the safe bias is the one that keeps the room intact while leaving every individual
free to leave, and the protocol's contribution is faithful possibility, never resolution. `Design.`

#### 7.6.10. Re-composition is a view, and adjudication is a sliding scale

§7.6.8 established that reunion after a fork is optional and cheap; this subsection states how a reunion is
expressed and how much of its terms may be computed rather than hand-decided. The organizing move is that a
re-composition is a **view**, a governed declaration of which rules and thresholds apply and how, and not a
merge of two authority states, which §7.6.2 already establishes MLS cannot represent. The framing is
load-bearing because the alternative regresses: if the reconciliation rules themselves had to be agreed in
the abstract before any reunion, there would be no base case, only agreeing about how to agree. Binding the
rules to the concrete act dissolves that regress, because the view is the agreement. It is recorded and
visible, a view over shared facts computes one membership, and a persona that rejects its terms exercises
exit or voice rather than negotiating a prior rulebook into existence.

- A re-composition **MUST** be expressed as a re-plant over a named membership and **MUST NOT** be expressed
  as a blend of two authority states, because MLS has no representation for a branch or a merge (§7.6.2, RFC
  9420 §8.2) and a blended authority state is exactly the silent contradiction this section exists to
  exclude: there is no state to blend, only a membership to constitute. The two structural modes are
  **subsume**, one lineage's members re-planted into the other so one history continues and absorbs, and
  **fresh**, both re-planted into a third as a new base; these are the §7.6.2 re-plant arities named for
  reunion rather than for schism. `Design.`

- The rules, thresholds, and their application **MUST** travel as a single governed view bound to the
  re-composition, not as free-standing prior agreement, because requiring the reconciliation rules to be
  settled in the abstract before any reunion regresses with no base case, whereas binding them to the
  concrete act makes the view itself the agreement, legible to everyone it affects. A persona that rejects a
  view's terms **MUST** retain exit and voice up to declining the re-composition and forking, because a view
  imposed with no such recourse would be a center dictating terms, which peer-equality forbids. `Design.`

- A view together with the facts a node holds **MUST** compute exactly one membership, because a
  re-composition that could yield different memberships on different nodes from identical inputs would
  reintroduce the silent divergence this section excludes; single-valued computation is what makes the
  outcome transparent, since every persona can see why it is in or out, and convergent, since every node
  lands identically. `Design.`

- The rubric is a sliding scale of adjudication that a group **MAY** adopt in part or not at all, and each
  rung **MUST** be independently selectable, because a group's tolerance for automated reconciliation is a
  governed-utility judgment that varies by archetype and shifts as the group grows, the same continuum
  §7.4.1 and §7.6.9 already govern, so an all-or-nothing rule would deny that variety. The rungs are given
  floor first below; higher rungs are more specific and more automated, and the top of the ladder is no rung
  at all, the re-forming authority hand-deciding the membership. `Design.`

- The floor rung, exclude-banned: a view **SHOULD** by default exclude any persona banned on either side,
  applying the §7.3.5 ceiling across the re-composition, because it is the cheapest and most legible rung
  and the one nearly every view keeps, and a banned persona keeps its own state and **MAY** re-form, so
  exclusion withholds nothing it is owed. Re-admitting such a persona is permitted but **MUST** be an
  explicit governed act rather than an automatic carry-across, because silently carrying a banned persona
  over a reunion would let a merge undo a ban that no one re-authorized. `Design.`

- The threshold rung: where the two sides carry different k-of-n thresholds, a view's default **SHOULD** be
  the more restrictive, re-derived against the combined membership, because the more restrictive reading is
  the fail-safe direction, the same bias §7.3.1's layered fold uses within a single history, and a threshold
  silently loosened by a reunion would be a downgrade attack in a merge's clothing. `Design.`

- The slot rung: where the two sides disagree on a role, capability, or other slot, a view's default
  **SHOULD** be the more restrictive reading, computed as an absolute count or a ratio as the view names,
  because the same fail-safe bias extends to the reunion and naming the metric keeps the computation
  deterministic rather than implementation-dependent. `Design.`

- The rubric **MUST NOT** be presented as a replacement for adjudication, only as the portion of it a group
  has chosen to automate, because codifying a rung removes none of the social judgment; it automates the
  part a group is comfortable automating, and every rung a group declines stays a human call. Presenting it
  as a replacement would smuggle in the center move of a codified rule settling a social-utility question
  the persona layer owns. `Design.`

- When a view cannot compute a membership from the facts a node holds, the node **MUST** defer to authority
  rather than compute a provisional one, because a re-composition run on an incomplete view could produce a
  membership a fuller view contradicts, the silent-disagreement failure; deferring keeps the completeness
  beam on the liveness side, where a reunion waits, and off the safety side, where it would otherwise
  re-form a wrong membership. This is the §7.3.8 delay-over-breach posture applied to re-composition.
  `Design.`

The surface is two axes: the mode, subsume or fresh, and the view, which rungs and which thresholds and
whether by absolute count or ratio, both governed, recorded, and exitable. Together they make
re-composition social-utility-centered, since the group sets its own tolerance; transparent, since the view
and its inputs are visible; composable, since rungs stack independently; and deterministic for the common
case with a clean hand-back to authority for the rest. What the protocol furnishes is the possibility and
the provenance; which reunion happens, and on what terms, stays the persona layer's to decide.

*Running example: long after the split, Bob's lineage and the Group reunite by re-planting a fresh Group
over both memberships under a view that takes the more restrictive threshold and excludes anyone banned on
either side; from the same banned beginning Bob is re-admitted by an explicit governed act while Erin is
not, two equally legitimate outcomes the substrate does not choose; every node computes the same membership,
and where the view cannot settle a contested role the re-forming authority decides rather than the machine
guessing (walkthrough beat E10).*

### 7.7. Dataplane history: two modes

The governance fold above (§7.3) is one structure; a Group's **dataplane history** (the conversation
content itself) is managed in one of **two mutually exclusive modes**, chosen at Group creation. A Group
runs one or the other, not both, because their convergence semantics and validity rules differ. The split
exists because two things a Group might want, unbounded scale and coordinated mutability, pull the data
model in incompatible directions, and no single structure has both properties.

- **Forward-only mode (large-scale Groups).** An append-only, hash-linked causal fold, the same §7.3.1
  ordering spine applied to content: provenance accrues by accumulation, entries are never overwritten or
  deleted in place. It scales because no node needs the whole history and the structure is a simple
  verifiable chain, which fits large, broadcast-like, or loosely-coupled Groups where coordinated mutation
  would not converge cheaply. The only "cleanup" available is a coordinated hash roll-up (§7.7.2), not
  per-entry mutation.

- **Willow-mutable mode (bounded-size Groups).** Entries are path-addressed; a newer entry overwrites an
  older one at the same path, and deletion is expressed as **prefix pruning**, so mutation and deletion are
  first-class, convergent, capability-gated operations. It fits bounded-size Groups (families, teams, small
  communities) that want genuine coordinated edit and delete, and it carries per-entry overwrite-tracking
  overhead that only pays below some practical size. *(The size bound is an engineering estimate,
  parameter- and backend-dependent, not yet measured; Appendix B.)*

Being **Willow-shaped already** makes the mutable mode cheap to reach: the delivery layer chose range-based
set reconciliation and content-hash addressing (§6.8.1, §7.1) for reasons that hold in forward-only mode
regardless, and those choices put the substrate close to Willow's data model already. So adopting
Willow-mutable mode for a bounded Group later is an *evolution of an already-Willow-shaped store, not a
rewrite*, and it costs nothing in the forward-only case. This is a deliberate hedge: shape for the harder
mode, pay only for the easier one until a Group needs the harder one.

**Concurrent edits under mutation take a semantic read-merge, never a timestamp overwrite.** Forward-only
mode has no in-place overwrite, so concurrent writes accumulate and this question does not arise.
Willow-mutable mode does overwrite by path, and Willow's own newer relation for a same-path collision is an
unverified writer timestamp, then payload digest, then length, which orders by a self-assigned number
rather than by what any editor intended *(Verified, Willow data-model spec)*. Drystone does not let that
relation decide a shared resource. Because authority in a communal namespace comes from writing one's own
subspace (§5.10), concurrent editors of one logical resource write distinct subspaces whose entries coexist
under the union rather than one silently overwriting the other, and reconciling them to a single value is a
read-time merge that **MUST** be semantic, a CRDT in the sealed payload or a mutation routed through the
causal governance fold (§7.3), and **MUST NOT** be an application-level timestamp last-writer-wins, because
a self-assigned clock choosing the winner reintroduces the silent loss of a concurrent edit that per-writer
subspaces otherwise remove. The within-subspace overwrite that remains is one writer replacing its own
earlier entry, which is intentional sequential mutation and not a dropped conflict.

#### 7.7.1. What prefix pruning buys, and the wall it does not move

Prefix pruning is a real capability gain and a bounded one, and both halves matter. Willow gives payloads
hierarchical path names and deletes by overwriting a prefix (like overwriting a directory with an empty
file); a delete is itself a convergent, synced, Meadowcap-capability-gated operation, so *who may delete
what* is an authorization question. *(Verified, Willow data-model spec; Meadowcap.)* This is qualitatively
beyond the forward-only fold, where removing a mid-history entry breaks every downstream hash (a fork from
that point): Willow expresses coordinated, convergent, authorization-gated deletion as a normal synced
operation, which the fold cannot without forking.

The wall it does **not** move, stated plainly because a reader will otherwise assume erasure. Deletion is
**metadata-convergent, not existence-erasing**, the prune leaves a tombstone, so "a value was here and was
removed" is itself a propagated fact (honest, but not secret). And **convergence governs honest nodes; it
cannot reach a copy already taken** by export, screenshot, or a modified client. This is the same
irreversibility wall as cooperative chosen-ephemeral (§6.8.4): Willow upgrades deletion from an
uncoordinated, unprovable, chain-breaking act to a coordinated, convergent, tombstoned, capability-gated
one, but it does not eliminate the cooperative-non-retention nature of the guarantee. Deletion remains
cooperative against honest nodes, never enforced against an adversary, and never corroborable.

#### 7.7.2. Coordinated history roll-up (a governance item, not deletion)

Distinct from per-entry deletion is **coordinated history roll-up**: a Group collapsing or summarizing old
history to bound storage. It is a **Group governance decision** with a Group default for local execution and
corroboration, and it is named here only to keep it separate from the two ideas it is often confused with,
it is **not** chosen-ephemeral (a per-message retention disposition, §6.8.4) and **not** self-destruct
(§6.8.4). In forward-only mode it is the *only* cleanup available, taking the shape of a coordinated hash
roll-up rather than mutation. The mechanism (how the Group decides, the default, how local execution is
corroborated) is its own governance treatment, referenced here rather than specified.

#### 7.7.3. Self-destruct is bounded by node fidelity, and mode changes what it can mean

Self-destruct (the §6.8.4 payload class: a time-bound sensitive value all members would prefer not persist,
"show the guest wifi password until Monday") carries a **modest, deliberately-named threat model**: a
good-enough honest auto-clean that beats passwords living forever in SMS, not anti-forensic erasure against
a motivated adversary. Its strength is **not cryptographic**, because you cannot prove a recipient did not
copy the value nor that every device expunged; it is a **trust-and-fidelity property**, the Part 1 §2.3 question of
which nodes are in play and whom you trust to honor the disposition. This makes it the **inverse of
provenance**: everything else in Drystone maximizes durability and replication, and self-destruct
deliberately minimizes both, to keep the value within a boundary of nodes trusted to honor its removal.

Three consequences follow, and the last is the cross-cutting one:

- **It opts out of blind-store durability on principle.** A self-destruct value must not sit sealed in a
  meer past its window, because a meer is *outside the fidelity boundary*, it is blind, honors nothing, and
  cannot be extended removal-trust. So it is delivered live-durable to member devices and skips D-meer and
  D-peer (§6.6). The opt-out is principled, not a knob.

- **The fidelity boundary is the membership, and must be legible.** The sender is trusting the member
  devices in scope to honor "mask or remove after T," so the system should show that boundary (this reaches
  these N member devices, none a blind store, honest clients will expunge), letting the sender make the
  Part 1 §2.3 judgment informed.

- **Achievable semantics differ by history mode, and this must be surfaced.** In **Willow-mutable** mode,
  prefix pruning is a real convergent removal of the stored value. In **forward-only** mode, the sealed
  record is in the append-only fold and cannot be excised without a fork, so the value can be masked from
  display but the sealed entry remains. The same self-destruct request therefore yields different honest
  outcomes by mode, and in forward-only mode "removal" honestly means "masked from display, not excised from
  the fold." Self-destruct remains an open thread (Appendix B): framed here with its honest envelope and its
  mode-dependence, deliberately not fully specified pending dedicated investigation.

### 7.8. Side histories: one conversation, more than one history

A conversation may carry more than one history, and the requests that look like one feature are **three
mechanisms at three costs**. The single question that selects the tier: *does the side history need
different keys, or just different structure?* Reaching for a heavier tier than the answer requires
overbuilds the common case; reaching for a lighter one underbuilds the rare one.

- **Tier 1, threading (a subid).** A thread is a field on a message, a subid pointing into the Group's
  existing dataplane hash tree. Nothing new is created, the messages are already sealed under the Group's
  keys and already in the one history; the thread is a grouping over existing content, a UI/UX function, not
  a protocol one. This is the common case, nearly free, and the expected default for ordinary in-conversation
  threads. *Settled: no new mechanism.*

- **Tier 2, a separate-but-inherited side history.** Some side conversations want their own history, not
  just a display grouping (a "2026 vacation" collection kept aligned but separate and exportable; a
  guestbook off a Group). The defining property is that **everyone in the parent Group may read it**, the
  separation is structural, not access. So it is a *second dataplane hash tree sealed under the same Group's
  keys*, addressable and convergeable on its own, but **not a new MLS group**; entitlement is inherited from
  the parent because there is no new key layer. Its cost is another hash tree to converge and nothing more,
  no O(N) instantiation, no ratchet tree, no Welcomes, no freeze-then-strand hazard. *Candidate.*

- **Tier 3, a subgroup with its own entitlement.** When the side history must have *different* entitlement,
  a subset who may read it and others who may not, cryptographically enforced, only a real MLS branch will
  do: a new group over the subset with its own key layer, linked to the parent by resumption PSK. This is
  the **fork arity of the re-plant family** (§7.6.2) and carries its full cost, O(N) instantiation over the
  subset and the ReInit/branch freeze-then-strand hazard (§7.6.3). *Candidate.*

The load-bearing consequence: **entitlement inheritance is what makes tier 2 cheap, and also what
disqualifies it the moment access must narrow.** In tier 2 a member's right to the side history is
definitionally the parent-Group right, because there is no separate key layer, so the instant the design
wants "only some of us see this," inheritance breaks and the case is forced up to tier 3. The tier-2/tier-3
boundary is not a UX preference; it is a hard line drawn by whether entitlement diverges from the parent,
and the entire cost cliff sits there. Whether tier 2 deserves a first-class named construct at all, or is
simply an emergent use of the data model (a Group hosting more than one dataplane hash tree), is the central
undecided question, carried to Appendix B.

### 7.9. Scaling: commits, folds, and the relocation of ordering cost

This section states what a Drystone deployment forces through MLS's two scaling bottlenecks, commit
serialization and the Delivery Service, why most of Drystone's ordering happens off the epoch chain, and
where the cost removed from the chain actually goes. It is analytical, so its discipline is faithful
epistemic tagging and honest residual (§8.2, Appendix B) rather than normative grounding, and the strength
of the argument is entirely in scoping and relocating cost, not in any conjured efficiency. The central
caveat is stated once here and again where it bites: the governance half of the bottleneck-two claim is
conditional on the governance fold being order-independent, which is constructive given a single property,
gap-completeness, that is not yet earned (the beam below, Appendix B).

**What this section does not claim.** Leading with the disclaimers, because the argument is scoping, not a
conjured efficiency.

- It does not beat MLS cryptographically. TreeKEM key agreement is already cheap and near-logarithmic in
  the good case; the cryptography was never the limiter, so there is no crypto win to claim. `Verified-RFC.`

- It does not offer a free lunch on ordering. The ordering work is not eliminated, it is relocated from
  central slot-contention to distributed per-node deterministic computation plus convergence bandwidth.
  That relocation scales out and has no center, but it is real work, so this section treats the relocation
  as the claim, not as a cost avoided. `Design.`

- It does not solve the giant single-group case. A single MLS group still pays a ratchet-tree cost linear
  in its membership; the topology below mitigates this by keeping groups scoped, but that mitigation is
  conditional, not a solution to arbitrarily large single groups. `Verified-RFC.`

- It does not offload MLS cryptographic-state catch-up, only content-history catch-up. A joining member
  must still ingest the group's ratchet tree, carried in the Welcome and GroupInfo objects and scaling with
  member count; the tiered delivery (§7.7, §7.8) offloads the content half of catch-up to a content-blind
  store, and the cryptographic-state half remains MLS's problem. `Verified-RFC.`

#### 7.9.1. Bottleneck one: commit serialization

MLS advances the group key through a strict chain of epochs, and only one commit may close each epoch, so
concurrent commits collide and all but one must rebuild and retry. The load on this single slot is not
activity, it is specifically the rate of *key-changing* operations, because only those become commits;
application messages never touch the epoch chain. `Verified-RFC.`

The question for any deployment is what fraction of its events are key changes, and Drystone's answer
differs sharply from a messenger's by design.

- The content plane is not commits. Posts, comments, likes, replies, profile edits, moderation-log entries,
  and votes are per-author writes converged out of band (§7.7, §7.8), not MLS commits and not even MLS
  application messages, so the highest-volume traffic contributes nothing to the slot. This follows from
  RFC once the content plane is kept off MLS, which is the `Design` decision.

- Authority-only governance is not commits. Granting a role, changing a threshold, amending policy: these
  fold as governance facts and advance the governance generation, not the epoch (the now advances at zero
  epoch cost, §7.3.7), so they are not key changes and not commits. This is the epoch/governance
  decoupling, `Design`, with the consequence, following by `Verified-RFC`
  reasoning once the decoupling is granted, that they never hit the slot.

What remains that actually hits the slot is only genuine key changes: a member added, a member removed, a
post-compromise rotation. So a Drystone group's commit rate is its membership-change-and-rotation rate, not
its activity rate and not even its governance rate: a large group in vigorous discussion with constant
moderation and role changes is, from the epoch chain's view, nearly silent. `Verified-RFC` for the
reduction, given the two `Design` decisions above.

One reduction is Drystone-internal: a membership removal is both a governance fact and an enforcing commit,
and the two-phase removal (§7.3.6) separates them so concurrent removals resolve by the governance order
without slot collision, colliding only if their enforcing commits land on the same epoch. This narrows the
collision surface to the key-enforcement step. `Design`; load-bearing only for the "concurrent removals
rarely collide" sub-claim, not for the main reduction.

#### 7.9.2. Bottleneck two: the Delivery Service, split into its two jobs

The DS does two separable things, and Drystone's relationship to each differs: it *orders* concurrent
commits (picks the single winner per epoch), and it *fans out and delivers* messages. RFC 9750 leaves the
DS unstandardized and describes fan-out as client-side, server-side, or mixed, so how it scales is the
application's engineering problem, not a protocol guarantee. `Verified-RFC.`

**The ordering job: relocated, and resting on the beam.** For the high-volume planes, Drystone needs no
single orderer, because ordering is a property of the data rather than a decision imposed by a referee: the
content plane converges by set union over content-addressed chains (order-insensitive, `Design`), and the
governance plane converges by the deterministic monotonic fold whose result is a function of the folded
set, not delivery order (§7.3.1, `Design` for the specification). If those hold, concurrent governance and
content events do not contend for a slot, they fold identically on every node, and the DS's ordering job
shrinks to the small residue of actual commits.

Here is the load-bearing beam, stated plainly and not softened. The governance half of this claim is
conditional on the fold being genuinely order-independent. Under the §7.3.1 and §7.3.2 resolutions (a
per-slot causal last-writer-wins register, a tiebreak restricted to genuine concurrents, cross-slot effects
as projections on final sets, and no fold-time validity rejection), the fold's order-independence is
*constructive*: it follows from the model rather than being asserted, because every slot's value is a pure
function of the fact set and its causal DAG, and the projections are pure functions of the resolved slots.
The remaining condition is a single one, **gap-completeness**: a node being able to tell whether it holds
the complete causal set to fold, since an undetected gap could hide a causally-later fact and change a
slot's resolved value. That single condition is `Load-bearing, unearned`. The correct reading is that given
gap-completeness, the governance ordering cost is relocated off the chain and order-independence follows by
construction; gap-completeness itself is corroborated, not proven (the dataplane checkpoint and
completeness-ahead corroboration, §7.3.3), and until the convergence experiment demonstrates gap-detection
it remains conditional and unearned, not a result. If gap-completeness fails, honest nodes can diverge and
a referee is needed to reconcile them, which puts a DS decision back in the governance loop and collapses
the "no DS ordering for governance" claim. The full statement and discharge path are in Appendix B.

The content half is on firmer ground, because union convergence is order-insensitive by a much weaker
argument than the governance fold requires, but it inherits the same dependency: a node must know whether
it holds all the entries, or it can present an incomplete union as complete. `Design` for union
convergence; `Load-bearing, unearned` for gap-detectability. This is the same beam as the governance
fold's, not a second one, and one mechanism discharges both: completeness *behind* a checkpoint is provable
from the hash-linked content-addressed chains directly, and completeness *ahead* to the frontier is
corroborated by the same quorum the now uses (§7.3.3). The only difference is the setting at which the
signal is applied. Governance enforcement fails closed on it, while the content plane qualifies freshness
on it and never gates reads (§7.4, the read and enforce line), so the weaker guarantee the union needs is
met by the weaker application of the one mechanism.

The relocation, stated as the claim: Drystone does not remove the ordering work, it moves it from central
slot-contention to distributed per-node deterministic computation plus the bandwidth of convergence. That
scales out and has no center, but it is real work and shows up as convergence traffic and per-node fold
cost rather than slot contention. This is the honest form of deterministic-yet-scaled-out ordering, a
relocation and not a free lunch.

**The fan-out and delivery job: the window, entered rarely, and the catch-up seam.** A commit is done not
when a CPU finishes it but when the DS has ordered and delivered it to everyone, and during that interval
the group sits in a transient inconsistent state. One measurement study observed that window reaching up to
about two seconds, dominated by the DS and network rather than the millisecond-scale cryptography.
`Measured` (Soler et al. 2025, OpenMLS, groups up to about 5,000 members): one study under specific
conditions, not a protocol constant. Any system delivering a key change to N members over a network pays
some such window; Drystone inherits it per commit, and because commits are rare in its traffic mix
(membership churn only), it enters that window rarely rather than constantly. `Verified-RFC` for the
inheritance; the rarity follows from §7.9.1.

Content delivery does not go through the commit-ordering path: a post propagates by the convergence layer
(a live MLS application message for freshness, out-of-band history convergence for completeness), so
content latency is decoupled from commit latency. `Design.`

The catch-up seam, stated precisely because an MLS-literate reader will look for it, has two halves.
Content-history catch-up (posts, comments, governance facts) offloads to the content-blind store by chain
reconciliation, off the MLS DS (`Design`, §7.7, §7.8). Cryptographic-state catch-up does not: a joining
member must ingest the group's ratchet tree, which travels in the Welcome and GroupInfo objects, is MLS
group state rather than content, and scales linearly in member count because the tree has a node per member
(`Verified-RFC`). So the tiering offloads the unbounded-over-time half (content history) and not the
membership-linear half (the tree): a content-blind store can serve sealed history blobs, and cannot serve
or reason about the ratchet tree.

#### 7.9.3. Topology: many small groups, and the cross-group cost it trades against

The ratchet-tree cost being per-group and linear in that group's membership is a reason to prefer many
scoped groups (a group per family, club, or channel) over one enormous group, since the linear cost is then
bounded by each group's smaller membership rather than by a single large one. `Verified-RFC` for the
per-group linear cost.

The topology has a cross-group cost that must be named, or the win is asserted rather than shown. Turning
one large group into many small ones means something spans them: a persona in fifty groups is in fifty MLS
groups, so fan-out to that persona is fifty groups' worth of delivery, and catching that persona up is
fifty trees rather than one. The cost moves from one tree of size M to N trees of smaller size, and whether
that is a win depends on whether the sum of the small trees is smaller than the one large tree. It usually
is, because trees are per-group and most personae are not in most groups, so the sum is sparse. This
section states the small-groups advantage as conditional on groups staying scoped and personae not being in
pathologically many groups, and does not assert it unconditionally, because Drystone's actual
group-membership distribution is not established here. `Design` for the topology; the win is `[confirm]`
pending a realistic membership distribution.

#### 7.9.4. Honest costs, collected

- Membership-change bursts still serialize as commits and still enter the inconsistency window. Two hundred
  joins in a minute are two hundred key-affecting events. The intended handling matches production systems:
  a designated committer that *batches* adds and removes into far fewer commits, so a join storm becomes a
  handful of batched commits rather than a storm of collisions. This composes with peer-symmetry because
  the committer is a mechanical role that needs no authority and cannot foreclose or govern, so it is a
  delegated-capability helper, not a center (the capability/authority split, §7.6.9). `Design` for the
  composition; the industry practice (Webex, Cloudflare) is secondary-sourced.

- The ratchet-tree half of catch-up is linear in group membership and is not offloaded. The mitigation is
  the scoped-groups topology, itself conditional (§7.9.3).

- The relocated ordering cost is real: it appears as convergence bandwidth and per-node fold computation,
  scales out, has no center, and is not zero.

- The governance scaling claim is conditional and currently unearned, with the risk reduced to one
  condition. With the §7.3.1 and §7.3.2 resolutions the fold's order-independence is constructive rather
  than asserted, so the remaining unearned beam is gap-completeness alone: a node being able to tell whether
  it holds the complete causal set. That condition is corroborated, not proven, is the largest single
  caveat, and is the one most likely to be probed. Its discharge path is the extended gap-detection
  experiment (Appendix B).

#### 7.9.5. Summary posture

| Claim | Status | Rests on |
|---|---|---|
| Commit rate is key-change rate, not activity rate | `Verified-RFC` | RFC 9420 |
| Content plane and authority-only governance are not commits | `Design` plus `Verified-RFC` consequence | §7.7, §7.8 |
| DS splits into ordering and fan-out, unstandardized | `Verified-RFC` | RFC 9750 |
| Content ordering needs no referee (order-insensitive union) | `Design`, with gap-detectability `Load-bearing, unearned` | §7.3.1, Appendix B |
| Willow last-writer-wins carries no silent loss (per-writer subspaces, union, semantic read-merge) | `Design`, Willow prune-scope `Verified` | §7.7, §5.10 |
| Governance ordering needs no referee | `Load-bearing, unearned` | constructive given gap-completeness (§7.3.1, §7.3.2); gap-completeness corroborated not proven, Appendix B |
| Ordering cost is relocated, not eliminated | `Design` | the convergence architecture |
| ~2s inconsistency window, entered per commit | `Measured`, rarity by reduction | Soler et al. 2025 |
| Content-history catch-up offloads to a blind store | `Design` | §7.7, §7.8 |
| Ratchet-tree catch-up does not offload, is linear in membership | `Verified-RFC` | RFC 9420 Welcome/GroupInfo |
| Small-groups topology bounds the tree cost | `Design`, win `[confirm]` | membership distribution, unestablished |
| Membership bursts handled by a batched designated committer | `Design` composition, secondary industry practice | Webex, Cloudflare |

---

## 8. Security Considerations

> `Realizes: P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement`

**Threat model (summary).** Drystone defends against: forged authorship (signature + standing, §4.4);
in-transit tampering (hash-chain integrity, §4.4); unauthorized membership/governance acts
(threshold authority + replicated policy + the admin floor, §5.7); silent authority reversal / state reset
(append-only fold, §7.3); manufactured-resolution via clock-lying *and* via honest-clock-skew (timestamp-
free order, §7.3.1, note this is now framed as a structural property, not only an anti-attacker measure);
and a helper accreting control by holding state (blind-by-construction helpers + material reversibility +
exitability, §5.5/§5.8). It explicitly does **not** defend against: instant revocation (exposure is
bounded, not zero, §7.2 R4); retroactive confidentiality (past reads by a since-expelled member are not
unmade, §5.7); or *prevention* of stale-authority writes (only their detection and attribution, §7.5).
These non-guarantees are the honest price of an eventually-consistent, center-free architecture; none of
them lets an attacker capture a Group or silently revert a decision.

**The wall-clock as an attack surface (cross-reference).** Because Part 1 §2.0.1 establishes that a timestamp is
an uncorroborable assertion, any place a wall-clock entered an authority-bearing computation would be a
social-engineering surface exploitable **even by an authorized, membership-gated member**, honestly (clock
skew) or maliciously (deliberate skew). The protocol's response is categorical: the wall-clock appears in
no ordering, identity, authority, or bound (§4.2, §4.5.1, §7.2 R4, §7.3.1). Liveness is measured locally as
a private input only (§7.4). This closes the surface by construction rather than by detection.

**Visibility and the social layer.** A Group's regime and visibility class are **born in at genesis and
immutable** (part of the signed genesis); there is **no silent regime crossing**, a republish is a
distinct authored act carrying a reference plus author-chosen content, never the original. Outward
propagation depth is enforced by every verifier. An implementation **MUST NOT** offer a structure-only
share (topology revealed, identities withheld), graph topology is re-identifying; the only safe share is
consented-distance/resolution-scoped. *Modeled (visibility regimes; the structure-only share is shown
unrepresentable, a modelled target's connection shape has anonymity set 1).*

**Metadata is the residual surface.** A meer still observes join-metadata (digest, length,
namespace) and connection attempts; this surface **MUST** be surfaced, not hidden, and a
Tier-0 helper **MUST** hold no payload key and **MUST be able to prove it** (assert-and-log
`payload_keys_held = 0`). *Verified (Tier-0 meer proves zero payload keys; admission denies a non-listed
node).* What a relay observes in this surface is **arrival order at the relay**, its own local observation,
not an authored timestamp; per Part 1 §2.0.1 the metadata accounting names it "arrival ordering as locally
observed," never "timestamp."

**Failed-operation response.** Detection of an invalid op is deterministic; the *response* is a governance
dial, **loud** (signed, corroborated rejection → Group immune memory), **silent** (reject, no signal), or
**blackhole** (tarpit). A serious auto-response **SHOULD** require k-observer corroboration. Note "silent"
is application-layer: the relay still observes the connection attempt. *Design.*

**Label, not enforce, a personahood-preserving primitive.** Where content moderation or social adjudication
is involved, the protocol's posture is to **label** (attach advisory, attributable metadata) and leave the
*action* to Group governance or each persona's own client, rather than to **enforce** (act unilaterally and
irreversibly on the network's behalf). This is not only a safety choice; it is what keeps the system *made
of peers* (§3.1). Enforcement relocates adjudication to whoever enforces, quietly converting peers into
sensors by stripping their decision rights, whereas labeling leaves adjudication with the principal and
propagates only information. It is the same algedonic move as the §7.6 hard-stop and the same razor as Part
1 Part 1 §2.5: **surface the signal, don't seize the decision.** Each enforcement hook tends to look locally
reasonable, which is exactly how a network of peers can degrade into a centrally-adjudicated sensor mesh over
time; the label-not-enforce default is the precommitment against that drift.

### 8.1. Forward secrecy and durable history are not in tension

A center-free design that keeps durable history invites the worry that durability undoes forward secrecy.
It does not, and the reason is what forward secrecy is defined against. **Forward secrecy assumes the
adversary already holds all the ciphertext**; it delivers its guarantee by requiring the *keys* to that
ciphertext be deleted on schedule. So a durability node (a meer, §6.6.2) holding sealed bytes indefinitely
is squarely inside the scenario forward secrecy is built to survive, provided keys were deleted on
schedule. The deletion schedule operates on keys, never on ciphertext. *(Verified-RFC, RFC 9750: FS holds
against access to all encrypted traffic history combined with current keying material, provided keys are
deleted after use.)*

The real friction lives in **keys, not ciphertext**, and there are two seams:

- **Key retention to process reordering.** A reorder-prone center-free delivery layer may require members to
  retain key material to process commits out of order, which violates the deletion schedule and reduces
  forward secrecy. This is the seam a decentralized delivery layer creates, and it is the cost side of the
  staleness calculus (§7.4). *(Verified against draft-kohbrok-mls-decentralized-mls-00.)*

- **The persistently-offline node** (§7.4): FS and PCS rely on active deletion and replacement, so an offline
  node holding old keys is a residual hazard MLS itself admits it cannot fully close. *(Verified-RFC, RFC 9750.)*

These seams are also where a member's ability to *read* durable history lives, and naming that keeps the claim honest. A blind durability node holding sealed bytes is no forward-secrecy cost, because it never held the keys, which is the argument above. A member reading content from an epoch it has already advanced past is a different matter: MLS forward secrecy is scoped to the live epoch and transport schedule, so reading old durable content needs key material the deletion schedule would otherwise remove. Drystone does not re-seal history under a separate archival key at the store, since the meer keeps byte-identical sealed bytes (§6.6.2), so long-term readability of same-Group history rests either on a member retaining the relevant epoch keys, the reorder-and-offline seam above and an explicit forward-secrecy cost, or on the content living in a separately-keyed layer, a subgroup or mirror-group history store with its own key schedule (§7.8), whose key is then itself a retained secret carrying the same offline-node posture. Either way the durable layer does not inherit the epoch layer's forward secrecy, and this specification does not claim it does: the guarantee is over the live key schedule, and durability trades forward secrecy for readability by whichever of those two paths a deployment takes.

Self-deleting content is a **different object at a different layer**, and the two must not be conflated. MLS
has no disappearing-message feature; its deletion schedule is a *key* mechanism producing forward secrecy
and says nothing about content. Drystone's chosen-ephemeral retention (§6.8.4) is a *content*-layer policy,
a signed "do not retain past T" disposition honest clients apply. MLS's key deletion neither implements nor
conflicts with it; they touch different objects. And chosen-ephemeral retention is **cooperative
non-retention, never enforced deletion**, a node cannot prove it expunged, and a modified client keeps what
it read (the §8 label-not-enforce posture applied to retention).

*Running example: Carol's node holds durable history for the Group, and forward secrecy over the live key
schedule is untouched by that, because the two answer different questions (walkthrough beat E3).*

#### 8.1.1. Decentralized MLS (DMLS / FREEK): a research pointer, not a dependency

The key-retention seam above is the subject of active work worth tracking without depending on. DMLS
(draft-kohbrok-mls-decentralized-mls-00, March 2025), built on FREEK (Alwen, Mularczyk, Tselekounis;
eprint 2023/394), modifies the MLS key schedule to derive multiple init secrets via a puncturable PRF, so
retained key material loses less forward secrecy, and adds content-derived epoch identifiers so forks off
one integer epoch are uniquely identifiable. *(Verified against the draft.)* It is **preliminary and not a
dependency**: its introduction and security-considerations sections are empty, and its state-consolidation
procedure assumes two coordinating servers to prevent forks, the inverse of Drystone's premise. The posture
is to track it as the most relevant work on the *cost* side of the staleness calculus, and to consider
adopting two ideas independently of the whole protocol, **content-derived epoch identifiers** and the
**PPRF approach to forward-secure retained init secrets**, while **declining its consolidation procedure**,
because Drystone's consolidation is the governed fork/heal of §7.6, above the key layer rather than inside
it. If DMLS matures on the cost side it widens the "briefly offline" window of §7.4 without weakening
forward secrecy as much; it does not change the §7.6 escalation posture, which the principles force
regardless of key-layer efficiency. *Design.*

### 8.2. Honesty boundaries this specification still carries

Stated plainly so the spec does not over-claim: (a) freshness (§7.4) is proven in the model, not yet over
live transport; (b) the failed-op leak/immune dial is design-only; (c) a content-visible gating Group Role's
**compellability** tradeoff is an unresolved policy/legal question, not an engineering one (gates any such
deployment); (d) the video media engine and real-codec/RTP path are design; (e) the membership-op
freshness threshold and admin-floor rule are decided-but-not-yet-test-run; (f) the false-positive
escalation tolerance (§7.4.1) is design-only and its *value* is deliberately left to Group policy; (g) the
capped-root soundness claim against the Matrix uncapped-root steelman (§7.3, Appendix B) is argued, not
proven, and the coverage of what has actually been tested must be surfaced before it hardens.

---

## 9. Interoperability and Conformance

> `Realizes: P-Knowable-Truth, P-Peer-Equality`

Two implementations are **Drystone-compatible** when they agree on every normative section that forces
agreement: the identifier derivations (§4.2), the signed message pre-image and verification (§4.3), the
integrity-vs-authority separation (§4.4), the lineage fold (§4.5) and lineage-counted thresholds (§5.7),
the rights floor and the `floor + roles + capabilities + resources` decomposition (§5), the governance-fact
append-only fold and total order (§7.3), and, once its `[gates-release]` encodings are pinned, the
frontier-closure-and-
subgraph-closure rule (§7.5.2), which is where two implementations are most likely to diverge. **Peer
equality is shown to be enforced by mechanism, not convention, exactly here:** a conformant implementation
cannot grant a persona a rights difference, because §5 makes every configuration `floor + roles +
capabilities + resources` and rejects anything that decomposes to fewer rights than the floor.

A conformant implementation **MUST** pass the conformance vectors and must-reject cases. The reference
conformance suite is **built and passing (66/0)**, derived by running the real implementation: derivations
(incl. the tagged wire forms), signed pre-images, the fold and lineage-counted thresholds, revocation
mechanics and k-of-n revoke-authority, the reconcile corpus, the adversarial cases, and the visibility and
freshness vectors. *Verified (suite), note the suite covers the §4/§5/§6 proven layer; the §7.3–§7.5
governance-resolution vectors depend on the `[gates-release]` encodings in Appendix B and are not yet in the
suite.*

---

## 10. Substrate Requirements and Reference Realizations

> `Realizes: P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement`

This section exists to enforce, for the bundled substrate, the same discipline the rest of the spec
applies to everything else: **separate the requirement from the realization.** For each substrate
component Drystone names a concrete reference (MLS, iroh, Ed25519, and so on), but the reference is *the
current best realization of a stated requirement*, never the requirement itself. This matters for
provenance, a reader can see exactly what Drystone *depends on* versus what it *currently uses*, and for
durability: a reference can be replaced by any candidate that satisfies the requirement, and the spec
should make the compliance bar explicit rather than implicit in a name.

The mechanism-neutral treatments elsewhere carry the same separation and are cross-referenced here: the
capability interface (§7.2 R1–R6) and the Willow-shaped data-model commitment (§7.1) already separate
requirement from realization, and §10.3 and §10.4 consolidate and complete that treatment. Where §10 and an
in-place section appear to overlap, the in-place section is normative for its mechanism and §10 is the
consolidated requirement view.

A note that frames the whole section. Drystone is a **local-first, center-free** system: there is **no
shared global state by design** (`P-Local-Truth`). It suits a specific class of application, group
messaging, collaborative state, membership and governance, anything where
local-canonical-plus-reconciliation is the honest model, and it is deliberately not built for applications
requiring a single authoritative global ordering (a central ledger, a globally-serialized auction close),
which should use a coordinator. The human-adjudication layer (§7.6) is part of the design, not a gap in it:
**every convergent system, of any architecture, is ultimately managed by humans**, and where a centralized
system places those humans behind an operator, Drystone makes the layer explicit and keeps it at the edge.
The substrate requirements below are chosen to serve that class well.

### 10.1. How to read this section

Each component below is given as: **(R)** the abstract requirement Drystone places on the component; **a
compliance table** of what any candidate must satisfy; **the disqualifiers** that would make a candidate
non-compliant; and **the reference realization** with a note on *why it currently wins*. A candidate that
meets the table and avoids the disqualifiers is **substrate-compliant** even if it is not the reference.

### 10.2. Messaging backplane: group key agreement and the secure-messaging properties

> Cross-references: §4.5.1 (per-client authorship, client-as-member), §5.7 (revocation rotates epoch),
> §6.2 (the two-layer encryption stack and the MLS metadata treatment), §6.12 (media keys from the Group
> epoch), Appendix A.1 (decentralized-MLS forward-secrecy cost), Appendix C.3.

**(R) Requirement.** Drystone requires a **group key agreement** mechanism that lets a dynamic set of
devices derive shared symmetric secrets, asynchronously (members need not be online together), with the
following security properties holding **under Drystone's center-free, fork-tolerant ordering**, which is
the part that makes this harder than the standard setting.

The named properties, defined precisely so the requirement is unambiguous (FS, PCS, membership agreement,
and asynchronous operation per RFC 9420 / RFC 9750, confirmed; FS-under-concurrency is Drystone's own
addition, below):

- **Forward secrecy (FS).** Compromise of current key material **MUST NOT** expose past messages. An
  attacker who learns the state at epoch N cannot decrypt messages from epochs before N. *(RFC 9420
  provides FS via the key schedule and hash ratchet.)*

- **Post-compromise security (PCS).** After a compromise, the group **MUST** be able to *heal*: once a
  compromised member performs a key update / the group commits a fresh representation, the attacker loses
  the ability to decrypt subsequent messages. *(RFC 9420 achieves this efficiently by removing and
  replacing a member's leaf via the ratchet tree, `log(N)` work, not `N`.)*

- **Membership agreement.** All honest members **MUST** compute the *same* group state (e.g. the same tree
  hash) for a given epoch, or detect that they have not. Disagreement must be detectable, not silent.

- **Asynchronous operation.** Key establishment **MUST** work for members not simultaneously online: the
  group advances by committed changes, not by a synchronous handshake.

- **FS-under-concurrency (the Drystone-specific addition).** Because Drystone has **no Delivery Service to
  totally order Commits** (§7.6 forks are first-class), concurrent commits will occur, and processing
  commits out of order forces clients to **retain key material**, which degrades FS. A compliant mechanism
  **MUST** either (a) prevent the FS loss, or (b) bound and minimize it, or (c) make the retention cost
  explicit and tunable. This is the property standard server-ordered MLS does **not** have to satisfy and
  Drystone does.

**Compliance table, a candidate group-key mechanism MUST:**

| # | Requirement | Why it is required |
|---|---|---|
| K1 | Provide FS as defined above | past content stays sealed after compromise (`P-Knowable-Truth`) |
| K2 | Provide PCS as defined above | the group can recover from compromise, not just detect it |
| K3 | Compute deterministic, checkable per-epoch group state with detectable disagreement | a member can verify it agrees, never silently diverge (§7.3.3 discipline at the key layer) |
| K4 | Operate asynchronously (no all-online handshake) | members are devices that come and go (`P-Durable-Enablement`) |
| K5 | Represent each **client** as its own member (a device may host several); do not require one signature key to be shared across a principal's clients in a group | this is what makes "same person" an identity-layer credential, not a key-layer fact (§4.5.1) |
| K6 | Rotate keys on membership change so a removed member is excluded going forward | revocation actually takes effect (§5.7, R5) |
| K7 | Either prevent, bound, or make-explicit the FS loss from out-of-order commit processing under forks | the center-free, fork-tolerant setting demands it (§7.6) |
| K8 | Scale PCS/commit cost sub-linearly in group size (target `log(N)`) | linear-per-update cost makes real groups impractical |

**Disqualifiers, a candidate is non-compliant if it:**

- Requires a **trusted, online ordering authority** to provide FS/PCS at all (it would re-introduce the
  center `P-Local-Truth` forbids, note this is exactly the line Drystone draws against MLS's *Delivery
  Service* assumption, see below).

- Uses **shared sender keys** as its only mechanism such that PCS costs scale as `O(N²)` and a leaked
  sender key permits indefinite passive eavesdropping (the explicit anti-pattern RFC 9420 was built to
  beat).

- Cannot represent per-client membership (forces key-sharing across a principal's clients), breaking K5 and the
  §4.5.1 model.

- Provides no detectable membership disagreement (silent divergence at the key layer), breaking K3.

- Bakes in a **wall-clock** dependency for epoch validity or ordering (violates Part 1 §2.0.1).

**Reference realization: MLS (RFC 9420), with the architecture of RFC 9750, and exactly why it wins, and
exactly where Drystone diverges from its assumptions.**

MLS is the reference because it is the only standardized, widely-reviewed mechanism that satisfies K1–K6
and K8 directly: it provides FS and PCS for groups from two to thousands, via a **ratchet tree** that
makes member replacement `log(N)`, and it computes a deterministic per-epoch group state (the tree hash)
that members check. *Verified at the data/transport layers it underpins; the FS/PCS and group-state
properties are per RFC 9420 / RFC 9750, confirmed.*

The critical qualification, and the reason MLS is named as *realization* and not as *requirement*: **MLS
assumes a trusted Authentication Service (AS) and an untrusted but ordering Delivery Service (DS).** RFC
9420's design leans on *some* DS to order Commits. The DS plays two roles, and Drystone splits them:
Drystone keeps an optional **store-and-forward** node (the meer, §5.4, a blind store that retains and
serves ciphertext for offline delivery, never required) but has **no DS in its *ordering* role**; it is center-free by construction (§3.1),
and ordering is the timestamp-free causal-cryptographic fold (§7.3.1) with forks first-class (§7.6). This
is precisely the gap K7 names and the reason MLS *as standardized* is necessary but **not sufficient** for
Drystone: the moment you remove the *ordering* DS, concurrent commits fork and FS degrades (Appendix A.1).
So Drystone's relationship to MLS is:

- It **adopts** MLS's tree-based group-key core (K1–K6, K8) as the reference.

- It **diverges** from MLS's DS *ordering* assumption; that ordering authority is exactly the center
  Drystone refuses, though the DS's blind store-and-forward role is retained as the meer (§5.4), and
  therefore must satisfy **K7** by other means: either a **FREEK-shaped puncturable-PRF mechanism**
  (`draft-kohbrok-mls-dmls`) that bounds the FS loss at a storage cost scaling with fork frequency, or the
  **`draft-xue` "Send Groups"** operating-constraints approach, or an explicit retention-cost budget. This
  is tracked as an open binding (Appendix A.1, Appendix B).

- The **AS** role, in Drystone, is filled by the **user-principal-as-its-own-CA** (§4.5.1), not by a
  trusted third party, another point where Drystone instantiates an MLS-architecture role center-free.

> **A named, accepted cost, client-level metadata exposure (K5's price).** Representing each **client** as
> its own member (K5) is what makes governance count identities not clients (§5.2, §5.6), but it carries
> a real privacy cost the architecture documents acknowledge: because base MLS operates each client as a
> distinct member, other members can identify **which of a principal's clients sent each message** (and,
> since clients run on devices, often which device was in use), and can detect **when a principal adds or
> removes a client or device**, which RFC 9750 §8.2.4 ("Associating a User's Clients") notes may, for some
> applications, be an unacceptable breach of privacy. Per that section, the risk arises specifically when
> the clients' leaf nodes carry data that can correlate them, and is mitigated by careful handling of that
> leaf-node/credential data. Drystone's identity layer (the principal-as-CA, §4.5.1) is the separate
> principal-level mechanism layered over the per-client members, so the design is internally consistent,
> but the client-correlation metadata is a
> cost Drystone accepts, not one it eliminates, and a deployment with a stricter metadata-privacy threat
> model must mitigate it explicitly (e.g. constraining what leaf-node data is exposed). *Stated as a known
> tradeoff rather than hidden; RFC 9750 §8.2.4 confirmed.*

So MLS is the realization, the requirement is K1–K8, and the honest statement is: *Drystone is built on
the MLS group-key core but is not MLS-as-deployed, because every production MLS deployment is
server-ordered and Drystone is not.* A future mechanism that satisfies K1–K8, including K7 natively,
could replace MLS without changing any Drystone requirement. **[confirm, the DS/AS trust
model and deployment-status claims against RFC 9420/9750 and the decentralized-MLS drafts.]**

#### 10.2.1. Concept alignment, and where MLS is subordinate

Where a Drystone concept already has a native MLS representation, the two are folded rather than built
twice, and the correspondence strength dictates what to do: **Direct** (the Drystone concept *is* the MLS
construct, build on it), **Partial** (they align on one layer only, use the MLS piece for its layer and
supply the rest, keep the boundary explicit), **Drystone-only** (MLS has no representation, Drystone builds
it), **Underused** (MLS offers a construct Drystone should adopt rather than reinvent).

- **Direct.** The **re-plant family** (§7.6.2) is MLS's **ReInit** and **branching** generalized, linked by
  **resumption PSK**; this is the strongest alignment in the suite and lowers implementation risk, the
  primitives exist. **Entitlement continuity** across a re-plant is the resumption PSK, which proves
  co-membership at the source epoch. **State-loss recovery (entitlement half)** is MLS's rejoin-with-PSK.
  *(Verified-RFC, RFC 9420 §11.2, §11.3, §8.6; RFC 9750 §6.6.)*

- **Partial.** **Conversation continuity** is only partly the PSK: the PSK carries entitlement, the
  **dataplane history** carries content, kept separate. **State-loss recovery (content half)** has no MLS
  representation, MLS rejoin proves prior membership but does not restore missed messages, so recovery is
  MLS-for-entitlement and Drystone-for-content, two separate exchanges. **Staleness removal** uses MLS's
  *intent* ("eventually remove non-updating members," RFC 9420 §16.6) as the requirement, and supplies the
  *mechanism* MLS omits (the deterministic staleness predicate plus governed response of §7.4).

- **Drystone-only, by design.** **Out-of-band history convergence** (§6.8.1), the **monotonic governance
  fold** (§7.3, MLS enforces no access control on group operations, any member may commit, which is
  precisely why the external-join hazard is unquantified in MLS and quantifiable here), and
  **fork-not-verdict / the escalation set** (§7.6, MLS's linear transcript cannot express a fork at all).
  MLS staying out of history and access-control is deliberate on both sides; there is nothing to fold in.

- **Underused (candidate to adopt).** The **epoch_authenticator**: MLS derives a per-epoch value members
  compare out of band to confirm they share the same state (two members on different branches compute
  different authenticators), which is close to Drystone's whole-group-consistency check; whether the check
  can use it directly rather than a separately-built comparison, and how it relates to the governance
  chain's own consistency signal, is worth resolving so the two are folded rather than parallel. **[confirm
  against RFC 9420 §8.7 and the delivery-layer consistency design; carried to Appendix B.]** Separately, the
  resumption PSK's cross-group PCS-carrying property may be underused for linking healing across a persona's
  parallel groups. *(Not yet examined; Appendix B.)*

The whole set reduces to one posture, applied case by case: **MLS supplies a local cryptographic check
almost everywhere it needs agreement and assumes a coordinator supplies the global agreement; Drystone does
not re-add the coordinator, and for each case decides whether the global agreement is unnecessary,
reconstructible without coordination, or a social-utility judgment that must escalate to humans** (§7.6).
The third kind is the designed terminus of Part 1 §2.5, not a failure mode.

| Hard case | What MLS assumes | Drystone posture | Forcing principle |
|---|---|---|---|
| Linear epoch chain | DS orders commits, one wins per epoch | MLS subordinate; continuity in app-layer hash structures (§7.6.3) | Part 1 §2.0 |
| Concurrent commits / fork | one commit wins per epoch | fork/heal/re-key are one primitive, three arities, never represented in MLS (§7.6.2) | Part 1 §2.5 |
| Role under-determination | not addressed | second escalation class; expected; escalates to humans; cheap instantiation of their choice (§7.6.1) | Part 1 §2.4, Part 1 §2.5 |
| Stale / offline node | should eventually be removed | mechanical staleness detector, governed response; rights untouched, capacity degrades (§7.4) | Part 1 §2.3, Part 1 §2.4 |
| External-join recovery | GroupInfo authoritative to a rejoining client | GroupInfo is a claim corroborated against the governance chain; threshold dial quantifies attack cost; bad assertions self-fork (§7.4.2) | Part 1 §2.2, Part 1 §2.3 |
| Insider replay / nonce reuse on restore | no replay protection; reuse_guard on revert | isolated by out-of-band history convergence; live epoch secrets never restored in place (§7.4.2) | Part 1 §2.2 |
| FS versus durable bytes | keys deleted on schedule | no tension; FS assumes ciphertext retention; real risk is key retention under reordering (§8.1) | Part 1 §2.4 |
| Self-deleting messages | no such feature | application-layer content policy; cooperative non-retention (§6.8.4, §8.1) | Part 1 §2.4 |
| Decentralized operation | strongly consistent DS | track DMLS/FREEK for the cost side; adopt epoch-id and PPRF ideas, not the consolidation (§8.1.1) | engineering |
| ReInit non-atomicity | committer completes the re-form, or another member does | native ReInit/branching shape; freeze-then-strand window closed if governance chain records intent before freeze (§7.6.3) | Part 1 §2.4 |

*Table sourced from the MLS-hard-cases analysis; each row's mechanism is developed in the cited section. The RFC citations behind the "what MLS assumes" column are verified against RFC 9420 / RFC 9750 this round; the Drystone-posture cells are design, with the load-bearing residuals (external-join far-behind node, in-place secret restore, re-plant intent ordering) carried as **[confirm]** in Appendix B.*

### 10.3. Transport and overlay: point-to-point reachability, and the topology question

> Cross-references: §6 (transport and delivery), §6.9 (discovery), §6.5 (carriage), §6.6 (durability,
> incl. the meer), §6.10 (the gossip overlay), §6.11 (deployment modes), §6.12 (media datagrams), §3.1
> (where adjudication lives), §8 (the relay and meer as blind forwarders), Appendix C.

**(R) Requirement.** Drystone requires a **transport** that lets two peers, identified by **public key**
(not by IP or location), establish a **mutually-authenticated, end-to-end-encrypted** channel, with a
**fallback path** when a direct connection is impossible (NAT/firewall), where **any intermediary is a
blind forwarder** that cannot read content and routes by endpoint, not by topic. It additionally requires
an **overlay** by which scope members find and exchange with each other, and an **unreliable datagram**
mode for real-time media (§6.12).

**Compliance table, a candidate transport/overlay MUST:**

| # | Requirement | Why it is required |
|---|---|---|
| T1 | Identify peers by public key; the key is the channel's authenticated identity (no impersonation) | identity is cryptographic, not positional (`P-Knowable-Truth`); ties to §4.1 |
| T2 | Provide end-to-end encryption such that intermediaries see ciphertext + routing metadata only | a relay/meer stays blind (§8); supports the meer (§5.4) |
| T3 | Attempt a direct peer-to-peer path, with a **fallback relay** when direct fails | participation on a bare node behind a NAT must be real (`P-Durable-Enablement`) |
| T4 | Ensure any relay/intermediary is a **blind packet forwarder** (no session-content knowledge, routes by endpoint) | an intermediary must not become a center or an adjudicator (§3.1) |
| T5 | Provide a reliable stream mode **and** an unreliable datagram mode | messaging needs reliability; real-time media needs latency-over-reliability (§6.12) |
| T6 | Provide (or admit) an overlay for scope-member discovery/exchange keyed on the scope topic (§4.2) | members must find each other without a central directory |
| T7 | Not require a wall-clock for any authority-bearing decision | Part 1 §2.0.1 |

**Disqualifiers, a candidate is non-compliant if it:**

- Routes or authenticates by **IP/location** as identity (breaks T1; IPs are ephemeral and
  impersonable).

- Requires a **content-reading** intermediary, any relay/SFU/mixer that must see plaintext to function
  (breaks T2/T4; server-side media mixing on plaintext is already **forbidden**, §6.12).

- Makes the fallback path a **mandatory permanent hop** (a relay that cannot be bypassed by a direct
  connection is a structural center, breaking T3/T4).

- Offers only reliable streams, forcing real-time media through retransmit (breaks T5, starves media,
  the measured failure in §6.12).

**Reference realization: iroh (1.0), and why it currently wins.**

iroh is the reference because it satisfies T1–T5 directly and cleanly. It establishes **peer-to-peer QUIC
connections between endpoints identified by public key** (the `EndpointId` is the TLS identity, so it
cannot be impersonated, T1), all connections are **end-to-end encrypted with concurrent streams and a
datagram transport** (T2, T5), it **attempts a direct hole-punched connection and falls back to a relay**
when that fails (T3), and, the property that matters most for §3.1, its **relay is a stateless blind
forwarder**: it carries encrypted packets and routes them by `EndpointId` without being able to decode
them, and the payload is always encrypted to the destination endpoint (T4). *(Verified against iroh 1.0:
public-key TLS identity that cannot be impersonated, direct-first with stateless relay fallback, and a
relay that sees only endpoint identifiers and not content. These are stable 1.0 core properties.)* The
overlay (§6.10) and discovery (§6.9) layers it pairs with are separately-versioned crates outside the 1.0
guarantee, tracked in Appendix B.

**The topology question, stated surgically, because "p2p" does not cleanly capture the design.** This is
the point that motivated dropping *serverless* and reserving *peer-to-peer* for the transport (naming
note, §3.1). The requirement above asks for **point-to-point reachability with a blind fallback**; it
does **not** require, and does not forbid, any particular *overlay topology*. So:

- **What is required:** that any two scope members *can* reach each other directly when the network
  permits, that no intermediary is a content-reading center, and that the fallback is bypassable. This is
  a **reachability** requirement, not a **topology** requirement.

- **What is a legitimate, compliant divergence:** a **pure peer-to-peer mesh** (every member maintains a
  direct path to every other) is fully Drystone-compliant, nothing in the spec prevents it. So is a
  relay-assisted star, a gossip overlay (HyParView/Plumtree, the iroh-gossip reference, §6.10), a
  hub-and-spoke through a cooperative anchor, or any hybrid. All satisfy T1–T7 so long as the anchor/relay
  stays blind and bypassable.

- **Why the reference is point-to-point-with-relay-fallback and not pure mesh:** pure mesh is **not
  required** because Drystone does not need all-to-all connectivity to function, members reconcile
  pairwise and via the overlay, and a member need only reach *enough* peers to sync, not *every* peer.
  Pure mesh is also, critically, **not tolerant and expensive**: a full mesh is `O(N²)` connections,
  degrades poorly as members churn, and its connection-maintenance cost grows with group size for no
  benefit Drystone's reconciliation model can use. The point-to-point-with-blind-relay-fallback reference
  (iroh) gives the reachability the requirement needs at far lower cost and far better churn-tolerance.
  Mesh is *permitted* (an application that wants it for its own reasons is compliant); it is simply not
  the reference because it pays a cost the design does not need to pay.

So the honest statement: **Drystone requires point-to-point reachability with a blind, bypassable
fallback, and iroh is the reference realization; pure-mesh and other topologies are compliant divergences,
permitted but not required, and the reference deliberately avoids mesh because mesh is expensive and
churn-intolerant without giving the reconciliation model anything it needs.**

#### 10.3.1. On "local-first peer-to-peer" as a category

This is framing, not a normative requirement, included because the category Drystone sits in is
under-defined in the literature and naming it sharpens the positioning.

"Local-first" (Kleppmann et al., Appendix C.1) names a **data-ownership** property: the primary copy lives
with the unit. "Peer-to-peer" names a **transport** property. Their intersection, call it **local-first
peer-to-peer**, is a category with a precise defining trait that neither term alone captures: **no shared
global state by design.** Each unit holds canonical local state; the system reconciles between units; and
there is deliberately no global authoritative copy or ordering (`P-Local-Truth`).

This category is **not suitable for every application**, anything that genuinely needs a single global
serialization (a clearing house, a globally-ordered ledger close) is the wrong fit and should use a
coordinator, honestly. But the classes it *is* suited to are significant: group messaging, shared
documents, membership and governance, presence, collaborative tools, anywhere local-canonical plus
reconciliation is the truthful model of how the data actually lives. And the apparent "limitation"; that
such systems need a human layer for the irreducible residue (§7.6), is **not a deficiency of the
cryptographic-systems lineage but a property of all convergent systems**: every one of them is ultimately
managed by humans; the centralized designs merely relocate the humans to an operator and hide them.
Drystone's contribution is to keep that human layer explicit and at the edge. Naming the category this way
**reinforces** the value of the crypto-style convergent systems rather than devaluing them: they are the
honest substrate for the large and real class of applications whose data is genuinely local-first, and the
human-adjudication requirement is shared by every alternative, not unique to them.

### 10.4. Primitives: signature, hash, capability, data model

> Cross-references: §4.1 (suite), §4.2 (hash in derivations), §7.1 (data model), §7.2 (grant
> interface), Appendix B (hash reconciliation), Appendix C.1.

These are given more briefly because several are already requirement-first in the in-place text; §10.4
consolidates the compliance bar.

**Signature.** **(R)** A signature scheme providing existential unforgeability, deterministic verification,
and public-key-as-identity, fixed in the versioned wire profile and **never silently negotiated down**
(§4.1). *Disqualifier:* any scheme whose identity is not a public key, or that permits silent downgrade.
*Reference:* **Ed25519** (RFC 8032), deterministic, 64-byte signatures, ubiquitous and well-reviewed;
wins on maturity and the fact that the iroh and MLS layers already use it as the endpoint/credential
identity, so a single key type serves transport, group-key, and governance.

**Hash.** **(R)** A collision-resistant hash used over **tagged, domain-separated** pre-images (§4.2), the
same function across all parties for the interop anchor. *Disqualifier:* untagged pre-images (cross-kind
collision), or two implementations using different functions for the same derivation. *Reference and open
item:* §4 is proven on **SHA-256**; §7 is designed on **BLAKE3**; the single committed suite is an open
reconciliation (Appendix B), with mild evidence favoring BLAKE3 because the Willow data model Drystone is
built toward uses it (the Earthstar instantiation fixes the payload hash as BLAKE3 with a 256-bit digest,
confirmed against the Willow spec; note Willow flags it as "to be replaced by WILLAM3," so this is a moving
target worth re-checking). A length-extension check of the §4 hash uses confirms none is a secret-prefix MAC, the one construction length extension breaks: the identifier and content-id derivations are collision-resistance and preimage uses over public, domain-separated pre-images (§4.2), the branch hash chain over prior-hash, sequence, and payload is a collision-resistance use (§4.4), and authorship is a separate Ed25519 signature over a pre-image (§4.3), never a keyed hash. So no §4 proof depends on a SHA-256-specific property, and none implicitly assumes a length-extension resistance SHA-256 lacks; re-basing §4 on BLAKE3 removes a footgun rather than adds one, since BLAKE3 is length-extension resistant where SHA-256 is not. *(Verified-source for the BLAKE3 property; the construction-level check is `Reviewer-judgment`, to be formalized as a per-context note at freeze.)* The suite pin and the `croft-*` to `drystone-*` rename (§4.2) are both signed-encoding changes that re-open the §4 signature proofs, so they belong in one wire-freeze, re-proven together and re-run against the §9 conformance suite on the committed function, not applied piecemeal.

**Capability mechanism** (Meadowcap's data-access sense; §5.5). The compliance bar **is** §7.2 R1–R6
(unforgeable grant, attenuating delegation, convergent revocation expression, bounded stale-authority
exposure, forward read exclusion, attributable acceptance), the standard object-capability guarantees.
*References:* Track A (Meadowcap-shaped delegated tokens) or Track B (Keyhive-shaped convergent membership
graph), Appendix A; both satisfy R1/R2/R3/R6; they differ only on revocation immediacy. No Drystone
requirement assumes a track. The **Group Role** layer (in-Group governance authority, §5.5) sits *above* this:
Group Roles are granted by the governance fold and may carry the authority to issue capabilities, but the
capability mechanism itself is the data-access primitive.

**Data model / sync.** Already requirement-first: a **namespace/subspace/path** addressable store
reconciled by **range-based set reconciliation** (§7.1). *Reference:* the **Willow** data model, built
**Willow-shaped not Willow-dependent**, so a later transition is substitution, not redesign. *(Willow's
namespace/subspace/path model confirmed against the spec; see Appendix C.1.)*

### 10.5. Summary: the dependency-vs-realization ledger

| Component | Requirement (the bar) | Reference realization | Why reference / divergence note |
|---|---|---|---|
| Group key agreement | K1–K8 (§10.2), incl. FS-under-concurrency | **MLS** (RFC 9420/9750) core | wins on standardization + `log(N)` PCS; **diverges** from MLS's Delivery-Service ordering (Drystone has no *ordering* DS; the blind store-and-forward role is kept as the meer, §5.4), so K7 met via FREEK-shaped / Send-Groups / explicit-retention; AS role filled by user-principal-CA (§4.5.1) |
| Transport / overlay | T1–T7 (§10.3): point-to-point reachability, blind bypassable fallback | **iroh** (1.0) | wins on key-identity + blind stateless relay + datagram mode; **pure mesh and other topologies are compliant divergences**, not required, avoided as reference because mesh is `O(N²)` and churn-intolerant |
| Signature | unforgeable, deterministic, key-as-identity, no silent downgrade | **Ed25519** (RFC 8032) | maturity + shared key type across transport/key/governance |
| Hash | collision-resistant, tagged pre-images, single agreed function | **SHA-256** (§4) / **BLAKE3** (§7) | open reconciliation (Appendix B); leaning BLAKE3 to match Willow |
| Capability (Meadowcap data-access) | §7.2 R1–R6 | Track A (Meadowcap) / Track B (Keyhive) | mechanism-neutral; differ only on revocation immediacy; the in-Group **Group Role** layer (governance authority) sits above this (§5.5) |
| Data model / sync | namespace/subspace/path + range reconciliation (§7.1) | **Willow**-shaped | shaped-not-dependent; transition is substitution |

The single sentence for this section: **Drystone depends on a stated bar for each substrate component and
currently realizes each with the best-reviewed available mechanism, MLS for group keys, iroh for
transport, Ed25519, and a Willow-shaped store, and in every case the reference is replaceable by any
candidate that meets the bar, with the one persistent divergence being that Drystone removes the
*ordering* center (MLS's Delivery Service in its Commit-ordering role, any mandatory relay hop) that the
standard realizations assume, while keeping the DS's blind store-and-forward function available as the
optional meer (§5.4).**

---

## Appendix A. Alternatives Considered

- **Capability mechanism, Track A (delegated-token, Meadowcap-shaped) vs Track B (convergent
  membership-graph, Keyhive-shaped).** Track A satisfies unforgeable-grant and attenuation natively but has
  no native revocation, so R4 is met via bounded expiry (revoke = decline-to-renew) and R5 via per-Group
  epoch keys; revocation latency is bounded by the expiry interval (expressed as an epoch/generation bound,
  not a wall-clock interval, Part 1 §2.0.1). Track B makes removal and re-encryption first-class convergent
  operations (stronger revocation immediacy) at materially higher complexity and a dependency on research
  still in flight. Both satisfy R1/R2/R3/R6 identically, so the state-reset-avoidance guarantee does not
  depend on the choice, only revocation immediacy does. The choice is deferred to the richer-access-control
  phase, decided on whether expiry-bounded revocation is operationally adequate for real expulsion cadence.
  *Author's current lean:* Track A's Meadowcap term is preserved deliberately because Willow and Meadowcap
  are the foreshadowed data-layer path, so not colliding with that vocabulary is the stronger default;
  Keyhive (Track B) is the better mechanism on revocation immediacy and is not ruled out. The next step that
  would actually settle this is to **define Drystone's revocation needs concretely** (expulsion cadence,
  acceptable stale-authority window, complexity budget), then test each track against them. Until that needs
  definition exists, the deferral stands and no normative text assumes a track.
  *(Meadowcap's capability semantics and attenuation confirmed against the Willow Protocol spec. **[confirm
  before publish, Keyhive convergent-membership-graph and revocation claims, which rest on research still
  in flight; see Appendix C.]**)*

- **Co-signed op vs proposal-plus-votes** for membership authority: the self-certifying co-signed k-of-n
  bundle is canonical (one broadcast, validated locally against the current epoch); proposal-plus-votes is
  an optional deliberative mode, not built for v0.

- **Reverse-topological-power ordering (Matrix-style)** for governance resolution: rejected, for **two
  separate reasons that must not be conflated**: (1) folding sender power into the comparator produced an
  apparent-cycle objection in that protocol's own review; and (2) the timestamp tiebreak is uncorroborable
  (Part 1 §2.0.1), not merely gameable. Drystone keeps power and clock out of the ordering spine entirely
  (§7.5.2). **Separately**, Drystone *adopts* Matrix State Resolution v2.1's conflicted-subgraph closure
  (§7.5.2), taking their convergence fix without taking their ordering. The CVE-2025-49090 state-reset
  class is cited in §7.3 as evidence for the monotonic-fold choice and was rooted in starting-state/replay-
  scope, **not** in the rejected tiebreak, keep the two points distinct. **[confirm,
  Matrix State Resolution v2 / v2.1, MSC4289/4291/4297; see Appendix C.]**

### A.1 Related work: decentralized MLS (and the forward-secrecy cost of center-free ordering)

Drystone's §7 (center-free, timestamp-free deterministic ordering with fork-by-construction and a
deterministic tie-break) sits in the same design space as the active **decentralized-MLS** work, and shares
its central problem: with no Delivery Service to order Commits, concurrent commits fork, and **processing
commits out-of-order degrades forward secrecy** because clients must retain key material. There are **two
distinct drafts**, and earlier Drystone drafts conflated them; they are separate authors and separate
contributions:

- **`draft-kohbrok-mls-dmls`** (Kohbrok): "Decentralized MLS," based on the **Fork-Resilient Continuous
  Group Key Agreement** protocol **FREEK** (Alwen et al.), recovers most lost forward secrecy with a
  **puncturable PRF** at a **storage cost that scales with fork frequency**. Even this only improves FS for
  the init secret; members must still delay deletion of other ratchet-tree secrets.

- **`draft-xue-distributed-mls`** (Xue et al.): "Distributed MLS," whose contribution is the
  operating-constraints / "Send Groups" framing for PCS+FS without global ordering consensus, and which
  declines to resolve Commit collisions, leaving members to agree on strategies (in their words, the full
  weight of the CAP theorem is levied on the members).

Both are **drafts / proof-of-concept as of mid-2026; every production MLS deployment is server-ordered.**
The relevance to this spec: the fork/reconcile model (§7.6) and the survivor/re-key path are **not free of
an FS cost**, reconciling a fork means retaining (and ideally puncturing) key material, the price of
keeping the tie-break window open. Drystone should either design against or adopt a FREEK-shaped mechanism;
tracked as an open binding alongside the MLS↔governance-log consistency item. Note also that the
`draft-xue` posture, "members must agree on strategies for collisions", is precisely the gap Drystone
fills at the protocol level: the §7.6 hard-stop is a protocol-level answer to "what do the members do,"
which the MLS drafts hand back to the members. **[confirm, draft-kohbrok-mls-dmls / FREEK,
draft-xue-distributed-mls, and the MLS-deployment-status claims; see Appendix C.]**

A distinct point corroborates the caution rather than the cost. The external operations, external commits and proposals, received no computational security analysis until 2025: ETK (External-Operations TreeKEM) is the first cryptographic analysis of RFC 9420 to cover them, appearing roughly two years after the RFC *(Verified against the primary source: Cremers, Günsay, Wesselkamp, and Zhao, "ETK: External-Operations TreeKEM and the Security of MLS in RFC 9420," IACR ePrint 2025/229, 2025; EUROCRYPT 2026)*. So Drystone's wariness about the external-join and external-commit surfaces (§7, and the stale-`GroupInfo` hazard) is grounded in the literature's own late arrival at those operations, not merely a defensive posture.

## Appendix B. Open Questions (forming; not weakening the normative sections)

These are known-incomplete and tracked so they are not mistaken for settled:

- **The completeness-ahead beam: the single load-bearing property (§7.3.3, §7.3.7, §7.3.8, §7.9).** This is
  the one `Load-bearing, unearned` property the governance and scaling claims rest on, distinct from the
  `[confirm]` items below, which are smaller pending checks. The ceiling (§7.3.5) closes the
  membership-entitlement tail-gap: whether an actor is still entitled is checkable against a durable marker.
  It does not close the general tail-gap for role and threshold slots, where a current member can be behind
  on a grant or threshold change with no ceiling-equivalent to check against; those rest on the now, the
  finality gate, and the freshness attestation (§7.3.7, §7.3.8, §7.4). The freshness attestation is itself
  corroboration: a node in a partition fundamentally cannot self-certify that nothing newer exists, so
  completeness-ahead is established only when attestations reach it, and its absence forces a fail-closed
  stall rather than a proof of currency. Stated honestly: for membership, the dangerous permanent
  divergence is converted to a benign catch-up on a durable ceiling marker; for general governance,
  currency rests on quorum-attested freshness that an isolated node cannot manufacture. Order-independence
  of the fold is constructive given this property (§7.3.1, §7.3.2, §7.9), so the whole
  governance-ordering-needs-no-referee claim narrows to it. Its discharge path is the convergence experiment
  (permutation-invariance of the fold and, the load-bearing half, gap-detection: does a node with a gap
  detect it rather than fold an incomplete set as complete). Until that passes, the claim is conditional and
  unearned, not a result. `Load-bearing, unearned.`

  The theorem underneath this is CALM (Part 1 §3): a predicate has a consistent, coordination-free implementation if and only if it is monotonic, and "has my node seen every committed governance event" is non-monotonic, since one late arrival can flip it from true to false, so no purely local coordination-free completeness detector can exist. This does not sink center-freedom, because center-free is not coordination-free: a checkpoint that any member may propose and that converges without a privileged center is admissible, provided the single non-monotonic step, declaring completeness, is quorum-witnessed or causally-sealed while everything else stays monotonic (a grow-only chain of checkpoints, each a semilattice join over the events causally prior to it). Earning the property therefore means exhibiting four things, and a green convergence run alone does not: (1) a precise statement of the completeness predicate with a proof that it is either monotonic, hence exempt, or non-monotonic with the exact coordination the checkpoint requires stated; (2) a liveness argument under partition, that the fail-closed stall degrades safely rather than deadlocking governance, with the §7.3.3 read-and-enforce line and the §7.4 no-false-current property as the degraded-but-safe mode; (3) a safety argument that a late-arriving pre-checkpoint event cannot silently reverse already-enforced authority, the state-reset class the Matrix resolver admitted before its 2.1 hardening; and (4) a fork-composition argument, that two honest partitions produce checkpoints that either merge cleanly or fork explicitly, never silently disagree, whose home is the §7.6 reconcile-and-re-formation machinery (a ban is a fork seen from one side §7.6.4, the fork announces who lands where §7.6.6, and being in both is permanent while merge is cheap and never required §7.6.8). These are the discharge obligations the convergence experiment and the checkpoint specification must jointly meet. The concrete freshness primitive the ahead-tail still needs is a corroboration-on-latest request answered by a governed standard-of-care over head-attestations from distinct personae; §7.4.3's generation stamp already closes the behind-via-traffic half of detection, leaving this primitive to bound the unreferenced tail.

- **Enactment dial and posture presets (§7.3.6, §7.6.9).** The enactment dial's default rung and fallback
  intervals are unspecified (convergence-timeframe-dependent policy, §7.3.6), and the conflict-posture dials
  (auto-fork versus hold-on-conflict, merge-as-routine versus fork-as-durable) default by group archetype
  (§7.6.9). What is owed is the concrete presets: the enactment intervals and default rung, and the
  per-archetype posture defaults for friends-and-family, professional association, and social or logistics
  groups, on an 80/20 path. The structure is settled; the preset values are open. `[confirm.]`

- **The now's concrete wire schema (§7.3.7).** §7.3.7 specifies the now's contents and invariants but not
  its wire schema: how in-flight tallies are represented, how the binding-to-chain-head reference is
  encoded, and how the corroborating signature-set is attached. This is a serialization detail, but the
  tally representation in particular must be pinned so the fallback-enactment signal (§7.3.6) is
  unambiguous. `[confirm.]`

- **Tiebreak-key and instance-weighting defaults (§7.3.1).** §7.3.1 sets the safe range of within-tier
  tiebreak keys and the principle that party-neutral mechanisms may default while party-privileging ones
  must be opt-in and governed. Open is the default *choice* within that range: the canonical content-address
  hash is the party-neutral floor and the safe default, but whether any archetype should default to
  join-order seniority instead, and for which operation types, is unresolved. Instance weighting has no
  default weighting at all at present; it is a protocol-defined, protocol-honored, opt-in mechanism whose
  weights sit under k-of-n governance, and whether any archetype ships a default weighting scheme (for
  example a "constitutional threshold" preset) is open. The mechanism and its safety rule are settled; the
  defaults are not. `[confirm.]`

- **Vendor-neutral naming reconciliation.** The reference implementation's signed domain-separation tags
  use the historical `croft-*` namespace (§4.2). Drystone, the protocol, requires a versioned
  domain-separated tag but should define a vendor-neutral namespace (`drystone-*`). Because the tag is
  signed over, the rename is a real wire change that re-opens the §4 signature proofs; it must be defined
  and re-proven, not silently swapped.

- **Hash-function reconciliation.** The proven message/history layer (§4) is Verified on **SHA-256**; the
  designed governance-resolution layer (§7) specifies **BLAKE3** for canonical fact content-addressing.
  Note that Willow/Earthstar uses BLAKE3 (256-bit) for payload hashing (confirmed against the Willow spec's
  Earthstar instantiation), so the §7 choice is convergent with the data model Drystone is built toward,
  mild evidence the single committed suite should be BLAKE3 and §4's SHA-256 is the legacy side. The single
  suite the production profile commits to (and any transition) must be pinned, and Willow notes BLAKE3 is
  "to be replaced by WILLAM3," which the pin should track.

- **`[gates-release]` wire encodings** (gate a publication-final DOI): canonical governance-fact byte encoding
  (§7.3.1, the base all others extend); the canonical content-id pre-image now that `timestamp` is removed
  (§4.2); frontier-commitment construction and acceptance-record format (§7.5.1);
  **frontier-closure-and-subgraph-closure before sort** (§7.5.2, the highest-risk divergence point); the
  gating-vs-read relationship (§5.8.1); the capability/membership-graph wire format (gated on the Track A/B
  decision); the **returning-member `(G, D)` cursor and checkpoint encoding** (§6.6.2, §7.4), the
  governance-position and dataplane-position a returning member reports on dial-home; and the **data-plane
  governance-generation stamp** (§7.4.3), specifically whether its wire value is the generation counter
  alone or the full governance head hash, with the stamp delta-encoded across stable frontiers.

- **iroh substrate confirmation, post-1.0 (§6.5, §6.9, §6.10, §10.3).** iroh core is **1.0 (June 2026)**,
  wire-and-API-stable, which covers the transport-plane facts: public-key (`EndpointId`/Ed25519) TLS
  identity that cannot be impersonated, the post-handshake point at which the remote identity becomes known
  (the §6.1 seam), direct-first hole-punch with stateless blind relay fallback, and the relay routing by
  endpoint without decoding content, all **verified against iroh 1.0 core**. What **remains [confirm]** is
  the set the 1.0 guarantee does **not** cover, because those components are **separately-versioned
  crates**:

  - **`iroh-gossip` (pre-1.0, own release line):** the `Event` surface and the absence of any per-recipient
    delivery confirmation (the four net-layer events are verified at a recent version, but the enum has
    changed shape across versions, so the exact variants are version-pinned); the send-side `broadcast`
    return semantics; the HyParView active/passive view-size constants; and the PlumTree
    `PRUNE`/`GRAFT`/`IHAVE` wire behavior as iroh-gossip configures it (§6.10.1, §6.10.3, §6.10.4). The
    `subscribe(TopicId, bootstrap_peers)` shape is verified against the crate API.

  - **The address-lookup crates (`iroh-mainline-address-lookup`, `iroh-mdns-address-lookup`):** these now
    exist as shipped, maintained crates (which resolves the earlier "is mDNS mature / turnkey" question);
    what remains is their exact republish-and-expiry behavior against each crate's pinned version (§6.9.2,
    §6.9.2), plus the Pkarr self-signed-record integrity model, which is a **Pkarr-spec** primary, not an
    iroh-version question (§6.9.2).

  The *algorithm attribution* (the two 2007 Leitão, Pereira & Rodrigues papers) and the *RFC 9420 / RFC 9750
  architecture claims* in §6 are verified against primaries and are **not** in this set. The honest summary:
  iroh core stability is now load-bearing for the transport plane and the overlay/discovery crates'
  pre-1.0 status is the remaining surface to pin.

- **Root-authority succession and principal recovery** (§7.3): planned handoff is reassignment (a revoke-then-grant that folds) and contested handoff is a fork (§7.6), so succession is not a distinct primitive. The residual is the bounded recovery primitive for lost key material, stubbed at §7.3.9 (`Design`), a known social-and-threshold-recovery pattern rather than a whole deferred Lifecycle section.

- **The capped-vs-uncapped-root soundness question** (§7.3, and Part 1 §2.3): **the priority open
  security item.** The Matrix facts are now confirmed against primaries, and they sharpen the framing rather
  than just supporting it. Matrix's MSC4289 gives the room creator "infinitely high" power level, and its
  stated reason is precise: *the creator's server can already effectively control a room by backdating
  events, because access control requires a hierarchy and the creator sits at its top*, so MSC4289
  formalizes a control that backdating already made real. *(Verified against MSC4289 and the Matrix Project
  Hydra disclosure, Aug 2025.)* This is not "decentralized resolution needs an uncapped root in the
  abstract"; it is "an apex is forced *when backdating is possible*." Drystone's claim is therefore that a
  *capped, revocable-by-succession* root is sound *because* its ordering is timestamp-free (§7.3.1), which
  removes the very backdating surface that forced Matrix's apex. The work to close this:

  - **State the coverage, not a bare "tested/open."** The MSC4289 attack class has at least three
    components: (a) backdating to manufacture favorable causal/authority position; (b) the
    create-event-uniqueness / root-forgery vector (Matrix's CVE-2025-54315, fixed in room v12 / MSC4291 by
    making the room ID the hash of the creation event, which Matrix rates High-not-Critical and reports with
    *no known exploitation path*); and (c) the self-demotion / promote-others entrenchment trap (MSC4289
    also adds creator self-demotion and multiple creators as the escape). *(All three verified against the
    Matrix v1.16 release notes and Project Hydra disclosure.)* Drystone's timestamp-free order addresses (a);
    the `H(tag ‖ group_id)` genesis and unconflictable-base fold address (b), and note this is the *same
    fix* Matrix reached, the room/group ID as a hash of the genesis; the anti-entrenchment ladder and
    anti-brick floor (§5.7) address (c). **Identify which of (a), (b), (c), and which of their
    compositions, the existing tests actually exercised**, and state that coverage. If composition
    (a)+(b)+(c) was tested under adversarial conditions with must-reject vectors, the claim is closer to
    "tested" than "argued" and should say so; if only the components individually, refine the claim to
    "components proven, composition open."

  - **Frame it as the design-philosophy difference, which is the cleaner thing to test** (§5.7 note):
    Matrix *prevents* capture with an apex; Drystone *permits* capture and makes the §7.6 fork the remedy.
    So the question is not only "can a capped root match their soundness" but "**is exit-as-remedy-for-
    capture sound where apex-prevents-capture was their choice**", and the latter is what the test suite
    should target. *(The MSC4289 / MSC4291 / CVE-2025-54315 facts this rests on are verified this revision;
    what remains open is the Drystone-side question of which coverage the test suite actually exercises, a
    design/validation item, not a fact to confirm.)*

- **The open rights check** (§5.3): does the §7 survivor/re-key path strand `tenure` (leave a persona
  formally a member but unable to re-establish standing after a re-key)? This gates freezing the rights set
  (now three: tenure, voice, exit) into normative text. The candidate fourth right `share` has been
  **dropped**: a claim on shared assets is not part of the inalienable floor; where it has substance it is
  ownership of a Meadowcap communal namespace (§5.10), a data-layer matter, not a right. The concrete test
  to run: take a persona with valid standing, drive a survivor re-key of the Group, and check whether that
  persona can still re-establish its membership standing from its retained lineage and local state, or whether
  the re-key leaves it unable to rejoin its own Group. If the latter, tenure is not yet clean.

- **The false-positive escalation tolerance** (§7.4.1): the signals are corroborable provenance; the
  tolerance over them is a governed utility judgment whose *value* this spec declines to fix. The open work
  is the byte-level definition of the exposed signals and the policy-fact format for the tolerance, plus
  guidance (not a default) on tailoring it to use case without inviting either fatigue or silent false
  resolution.

- **What grounds a persona's authority, and what makes a right cost something to violate?** §3.1/§5.2 define
  a principal as a locus of adjudication, and a persona as its human kind. The working position (refined from earlier candidate groundings): a
  persona's authority is grounded in the **rights floor being variety-enabling, and therefore
  system-sustaining**. The floor is what lets the system hold plurality; negating a persona's rights lowers
  the variety available to resist the next negation, so rights-negation is the self-amplifying move toward
  collapse (Part 1 §2.3). That is precisely what makes a persona's authority *necessary* rather than granted:
  violating a right is not a local wrong against one persona, it degrades the system's capacity to absorb the
  next shock, and the cost is borne by everyone, not only by the persona whose right was negated. This is the
  companion to "where do decision rights sit": rights cost something to violate because the cost is
  systemic, which is the early detector of principal→sensor rot.

  Distinct from *why authority is necessary* is *what binds a human to a persona in the first place*. That
  binding (mint-and-bind: tying a human identity to a fixed cryptographic persona identity) is **contextual**,
  and the spec should say so explicitly rather than seek a single mechanism. A family group is simpler and
  higher-trust to bind (a QR scan in person suffices); a large group of disconnected strangers is harder
  and may need a credential service or a weaker, accepted binding (§5.6). The grounding of the binding is
  therefore the same contextual trust gradient as personhood itself, not a protocol primitive. This couples
  to the §5.8 exitability backstop and the exit-as-remedy-for-capture framing above.

- **MLS hard-case residuals** (the [confirm] items the §7/§8/§10.2 fold carries, each a specific stress-test, not a design gap):

  - **External-join far-behind node** (§7.4.2): whether a rejoining node far enough behind on the governance chain can always distinguish a recent-but-superseded `GroupInfo` from a current one. The §7.4 under-authorize-never-mis-authorize property suggests yes; reasoned, not proven. **[confirm.]**

  - **In-place secret restore** (§7.4.2): whether the recovery design ever restores a live group's epoch secrets in place (which would reopen the insider-replay and nonce-reuse hazards) versus always re-planting or re-joining fresh. The isolation argument holds only if recovery never resurrects live ratchet/secret-tree state in place. **[confirm.]**

  - **Re-plant intent ordering** (§7.6.3): whether the governance chain records the re-plant intent (re-planting to membership M) *before* the ReInit freeze, so any member can complete a stranded re-plant from the authoritative instruction. If yes, the freeze-then-strand window is discharged; if not, it needs a different completion mechanism. **[confirm against the delivery and governance-chain ordering.]**

  - **KeyPackage-exhaustion seating trilemma** (§7.4, §7.6): for an offline member that has exhausted pre-published KeyPackages at a boundary, at most two of three hold, fresh-KeyPackage forward secrecy, offline seatability, and avoiding the external-join path. Reusable last-resort KeyPackages buy offline seatability at an FS cost (RFC 9420 §16.8); refusing them forces late joining or the external-join path (whose safety depends on the §7.4.2 `GroupInfo`-corroboration defense holding). The resolution is likely another posture dial. *Open thread.*

  - **Re-plant seating default at a boundary** (§7.6.2): Welcome-seating versus external-commit-seating, which moves the KeyPackage-availability burden between planter and joiner. §7.4.2 reweights this: the external-commit path carries a PCS-integrity hazard the Welcome path does not, even with the governance-chain defense. *Undecided.*

  - **epoch_authenticator overlap** (§10.2.1 underused row): whether Drystone's whole-group consistency detection can use the MLS epoch_authenticator directly rather than a separately-built comparison, and how it relates to the governance chain's own consistency signal. **[confirm against RFC 9420 §8.7 and the delivery-layer consistency design.]**

  - **Resumption-PSK cross-group linking** (§10.2.1 underused row): whether the re-plant family already exercises, or could use, the PSK's cross-group PCS-carrying property to link healing across a persona's parallel groups. *Not yet examined.*

  - **Transcript-hash construction formula** (§7.6.3): *resolved this revision.* The §8.2 construction was read verbatim: `confirmed_transcript_hash[n] = Hash(interim_transcript_hash[n-1] || ConfirmedTranscriptHashInput[n])` and `interim_transcript_hash[n] = Hash(confirmed_transcript_hash[n] || InterimTranscriptHashInput[n])`, seeded from zero-length strings at genesis. This is a strict single-predecessor chain with no branch or merge representation, exactly the "one linear commit sequence" property §7.6.3 relies on. *(Verified-RFC, RFC 9420 §8.2.)*

  - **Epoch-number metadata leak versus re-plant frequency** (§6.4, §7.6.2): *the leak itself is confirmed; the re-plant interaction is the open part.* RFC 9750 states verbatim that MLS header metadata is an opaque `group_id` plus a numerical epoch (the number of changes made to the group), and that a network observer correlating this with other data may reconstruct sensitive information, with the recommended mitigation being to carry the metadata over a secure channel. *(Verified-RFC, RFC 9750 §8.1.2.)* The open design question is the re-plant interaction: re-plant is cheap and freely used, but each re-plant reads as activity to an observer, so the cheapness that makes re-plant attractive also makes it a metadata emitter. Two mitigations to weigh: the DMLS content-derived epoch identifier (§8.1.1), which leaks differently than a monotonic count, and the metadata-confidential transport RFC 9750 itself recommends (the delivery layer's concern, §6.4). *Open thread; interacts with the §6.4 metadata posture.*

- **Dataplane history modes and side histories** (§7.7, §7.8): the open threads from the history-structure design.

  - **Mode migration** (§7.7): whether a Group can migrate between forward-only and Willow-mutable modes, or whether the mode is fixed at Group creation. Suspected fixed-at-creation given the differing convergence semantics; verify. **[confirm.]**

  - **The forward-only / Willow-mutable size bound** (§7.7): the practical size separating the two modes is an engineering estimate, parameter- and backend-dependent, not yet measured. The specific Willow instantiation parameters and Meadowcap capability shapes for the mutable mode are a §5 decision. **[confirm.]**

  - **Self-destruct specification** (§7.7.3, §6.8.4): framed as a node-fidelity-bounded, inverse-of-provenance payload class with its honest envelope and mode-dependent achievable semantics, deliberately not fully specified pending dedicated investigation. Open questions: selector signaling for "do not durably store / do not enter D-peer," delivery semantics when a member device is offline past T (likely never deliver an already-expired secret), whether "mask" and "remove" are distinct dispositions, and whether a client's build profile can be part of the legibility surface. *Open thread.*

  - **Mutable-mode read-merge mechanism** (§7.7): the mutable mode requires that concurrent edits to one shared resource reconcile by a semantic read-merge, stated normatively as a **MUST** for a CRDT in the payload or a governance-routed merge and a **MUST NOT** for an application-level timestamp last-writer-wins. The positive mechanism is undecided: which CRDT family (for example a last-writer-wins register versus an observed-remove set versus a sequence CRDT) fits which resource class, and where the boundary sits between a payload-embedded CRDT and routing the mutation through the causal governance fold (§7.3). *Open thread; the normative guard holds regardless of which mechanism is chosen.*

  - **Tier-2 side history: feature or data-model note** (§7.8): whether a separate-but-inherited side history deserves a first-class named construct with its own lifecycle (creation, export, garbage collection, convergence scope), or is simply an emergent use of the data model (a Group hosting more than one dataplane hash tree). This is the central undecided question and determines whether "side history" is a feature or a note. If named: how it is addressed and discovered, how its convergence scope relates to the parent's, and how it is exported as a standalone tree while sealed under the parent's keys. **[confirm.]**

  - **Tier-2 to tier-3 promotion** (§7.8): whether a tier-2 side history can be promoted to a tier-3 subgroup later (access narrows after the fact) without losing history, and what that migration costs. This is the mirror of the re-plant family (§7.6.2) and likely reuses it. Also whether tier-1 subids and tier-2 side histories should share an addressing scheme so a thread can be losslessly promoted. *Candidate, not required.*

- **External-fact confirmation** (§7, and the Beer/Cybersyn/OGAS grounding in Part 1 §3): the comparisons
  consolidated in Appendix C (CALM, Willow/Meadowcap/Keyhive, Matrix State Resolution and the 2025 CVEs,
  decentralized-MLS, Modular Politics) and the Beer quotes / Cybersyn-OGAS history are web-verified in
  source dialogues only and **[confirm]** against primary sources before they harden.

## Appendix C. Consolidated Prior Art and Positioning

This appendix gathers, in one place, the prior art each layer builds on and the precise relationship to
each, so the novelty claim can be stated honestly and a reviewer can see the landscape at once. **Status
of grounding:** the **MLS** (RFC 9420/9750), **Meadowcap/Willow**, and **Spritely/ActivityPub** claims
have been confirmed verbatim against their primary sources. The **Matrix Project Hydra** facts are now
confirmed against Matrix's own primaries (the Aug 2025 Project Hydra disclosure, the v1.16 release notes,
and the CVE records): **CVE-2025-54315** (rooms before v12 lack cryptographic create-event uniqueness, High,
no known exploitation path), **MSC4289** (creator gets "infinitely high" power, with Matrix's stated reason
that backdating already gives the creator's server de facto control, plus creator self-demotion and multiple
creators), **MSC4291** (room ID becomes the hash of the creation event), **MSC4297** (State Resolution
v2.1: start the iterative auth checks from the empty set, and replay the conflicted state subgraph between
conflicting facts, the closure Drystone *adopts* at §7.5.2), and **CVE-2025-49090** (the state-reset class
v2.1 fixes). What still remains `[confirm]` is narrower: the **CRDT/
local-first** attributions and **Beer / Cybersyn / OGAS** history (still web-research grounding, not yet the
FACTCHECK SoT). The **CALM theorem** attribution and statement are confirmed against Hellerstein & Alvaro
("Keeping CALM," arXiv 1901.01930 / CACM 2020, conjectured PODS 2010) this revision. The honest novelty
claim is **synthesis and terminus, unoccupied against the closest published
neighbors**, *not* "first ever," and *not* novelty of the underlying mechanisms.

### C.1 The data layer: borrowed deliberately

- **CALM theorem** (Hellerstein & Alvaro; conjectured PODS 2010, proven by Ameloot, Neven & Van den
  Bussche). A problem has a consistent, coordination-free distributed implementation **iff** it is
  monotonic. This is the formal boundary underneath §7: the monotonic governance operations fold without
  coordination; the non-monotonic residue cannot be totally ordered coordination-free and is exactly what
  §7.6 escalates. Drystone should cite CALM as the proof that its escalation boundary is *necessary*, not a
  design preference.

- **CRDTs / local-first** (Shapiro et al.; Kleppmann, Wiggins, van Hardenberg, McGranaghan, Ink & Switch
  2019). The premise that local state is primary and replicas converge without a coordinator. This is
  `P-Local-Truth` at the data layer. Local-first is the closest *named movement* to Drystone's values, and
  it deliberately stops at data ownership; it has no governance/adjudication layer, which is where
  Drystone continues.

- **Willow data model + Meadowcap + range-based set reconciliation** (Earthstar/Willow project; Meyer's
  range reconciliation). Drystone is built **Willow-shaped** (namespace/subspace/path, range reconciliation)
  with Meadowcap-style attenuating capabilities (§5.5, §7.1, §7.2 Track A), but Willow-*independent*.
  Meadowcap's **communal vs owned** namespace distinction is the nearest prior-art cut at the §5.3
  commons-asset question (where a claim on shared assets lives, now that `share` is dropped as a right) and
  the model for the group-principal (§5.10). Willow uses BLAKE3
  (256-bit), relevant to the §7 hash choice (Appendix B). *(Meadowcap capability definition and
  communal/owned semantics confirmed verbatim against willowprotocol.org/specs/meadowcap; see §5.10.)*

### C.2 The governance / resolution layer: same skeleton, opposite spine

- **Matrix State Resolution v2 / v2.1** (MSC1442; MSC4297; the 2025 Project Hydra disclosures). The closest
  neighbor at the resolution layer. **Same** machinery: causal/auth DAG, Kahn's-algorithm topological sort.
  **Opposite** spine: Matrix folds sender power into the ordering and breaks ties by mainline/timestamp;
  Drystone removes power and wall-clock from the spine and breaks ties by content-address (§7.3.1).
  **Adopted from v2.1:** the conflicted-subgraph closure (§7.5.2) and the empty/unconflictable-base start
  (Drystone's §7.3 founding-fact base). **Cited as cautionary evidence:** CVE-2025-49090 (state-reset class,
  rooted in starting-state/replay-scope) supports Drystone's monotonic-fold choice (§7.3); CVE-2025-54315 /
  MSC4291 (create-event uniqueness) corresponds to Drystone's structural root-id closure (§7.3). **Cited as
  the steelman against `P-Peer-Equality`:** MSC4289 (uncapped "infinite" creator power) is the opposite of
  Drystone's capped root; the unresolved soundness comparison is the priority open item (Appendix B).

- **Blockchain / DLT governance.** Reaches global consensus on a canonical chain and treats forks as
  failures to avert. Drystone is the inverse on both counts: no canonical global chain (`P-Local-Truth`),
  and the fork as a first-class designed *good* (Part 1 §2.5, §7.6), not a failure. The DAO-hard-fork literature
  documents that human intervention/forking is *forced* when code cannot resolve a value dispute; Drystone
  makes that forced move a designed primitive rather than a crisis.

### C.3 The cryptographic group-state layer: the cost Drystone inherits

- **MLS (RFC 9420) and decentralized-MLS drafts** (`draft-kohbrok-mls-dmls` / FREEK; `draft-xue-distributed-
  mls`). The drafts establish that center-free Commit ordering forces retention of key material, degrading
  forward secrecy. Drystone's fork/reconcile path inherits this cost (Appendix A.1). The `draft-xue`
  posture, collisions handed back to "the members", is precisely the gap Drystone's §7.6 fills at the
  protocol level.

### C.4 The governance-as-protocol frontier: the nearest neighbor in *intent*, and the instructive gap

- **Modular Politics** (Schneider, De Filippi, Frey, Tan, Zhang; CSCW 2021) and the surrounding Metagov
  work, plus **Ostrom's** Institutional Analysis and Development / polycentric governance that grounds it.
  Modular Politics explicitly calls for governance as an **open, portable, interoperable protocol
  standard**, the same ambition as Drystone's §5–§7. **Three load-bearing differences:**

  - It agrees computation cannot capture human governance ("at best, computational tools can facilitate
    those non-computational processes") but treats this as a **humility disclaimer**, not a design driver.
    Drystone makes the same impossibility the **generative principle**: §7.6 is *derived from* "utility
    cannot be computed."

  - Its authority **roots in a platform operator** ("all permissions derive from those the platform
    administrators specify at the level of the Instance"). Drystone's roots in no node above the principal
    (`P-Peer-Equality`, §3.1). Modular Politics is governance-tooling for a hosted world; Drystone is
    governance for a center-free one.

  - It **brackets the resolution mechanics and wire encodings as future work** ("does not consider matters
    such as security and database structures"). That bracketed layer is Drystone's entire Part 2. Modular
    Politics drew the map; Drystone built the territory.

  This is the strongest single piece of positioning evidence: the most credible group building "governance
  as an open protocol" shares Drystone's premises and stops at exactly the door Part 2 walks through.

### C.5 The cross-disciplinary grounding (Part 1 §3)

Ashby (requisite variety), Beer (algedonic channel; Cybersyn/OGAS), Hayek (dispersed knowledge), Popper
(finitude/corroboration), Mill (the dissenter), Ostrom (commons without a sovereign). These supply the
*principles* (Part 1 §3 corroboration) but stop at the level of theory or values, none converts the value
into a byte-level wire obligation with conformance vectors. The **value-to-mechanism gap** is part of what
Drystone fills, and citing them is corroboration of the values, not of the mechanics. The
intrinsically-personal nature of the Part 1 §2.5 residue is *why* this gap is legitimate and not a failure of
nerve: the protocol technicalizes only the provenance layer and **mechanizes the refusal** to technicalize
the utility layer (the §7.6 hard-stop is a primitive whose content is "a human decides").

## Appendix D. Term lattice and invariants (the vocabulary of record)

This appendix is the single place that walks out every load-bearing noun and
the exact relation each holds, so usage can be validated and misses reasoned
about reliably. The prose definitions are in §5 (identity model) and §5.0/§5.5
(properties); this is the consolidated lattice over them, not a second source
of truth. Where this appendix and the §5 prose appear to differ, §5 governs
and this appendix is in error and should be corrected to match.

### D.1 Entities and genus

**principal** is the genus: a **permission-holding** entity identified by one
key-lineage, reasoned about through the permissions it carries. Exactly three kinds:

- **persona** is-a principal. The kind that manifests a human. Carries the
  rights floor and one unit of weight. The locus at which non-computable
  social utility is adjudicated, because a human stands behind it (§5.2).

- **Group** is-a principal. A collective that can hold a Group Role as a single
  principal (key-establishment identity is an open seam, §5.10, Appendix B).
  (Capital-G Group is the collective as an in-system principal; lowercase group
  is the social body it manifests, §5.0, Part 1 §2.3.)

- **delegate** is-a principal as a **state**, not a species: a persona or
  Group currently holding a Group Role delegated by another principal (§5.5).

Not principals in the Group-governance sense: **meer** (blind store-and-forward node, infrastructure; holds
no Group Role, right, or weight, though it is a broad-plane principal holding **ecosystem permissions**;
§5.4) and **relay** (iroh transport-layer blind forwarder; holds nothing; §6). Distinct layers; neither is
a Group-governance principal.

### D.2 Hosting / realization chain

human manifests-as persona (1:N across systems; one per Group is the norm).
persona is rooted-in exactly one root key pair at a time. The root key pair
descends-to membership keys by signed credential (the lineage, §4.5). persona
is realized-by 1..N clients; clients run-on devices; a device is-a node and
may host 1..N clients; a human has 1..N devices. One line:
human → devices → clients, folded by lineage to one persona.

### D.3 MLS-carried terms (RFC 9420/9750, verbatim)

**client** = software on a device that is a member of a Group: one **leaf**,
one **signature key**, one **credential**, authenticated as a **member** via
the **AS**. **member** = the client as enrolled in a Group; the Group
recognizes members (clients), and lineage folds a Group's member-clients to
one persona. Counting members ≠ counting personae; the fold is the bridge.

### D.4 Lineage and the count

**lineage** is a provenance object: the signature chain from a root key pair
to each membership key; technically representable, verified and counted with
certainty. The **fold** resolves all clients/devices of one lineage to **one
persona per rooting key pair**. Governance counts personae by lineage, never
clients or devices. The binding "this lineage is one human" has **no technical
representation**; it is the Group's judgment (§5.6).

### D.5 The four properties and the non-property

Asked of personae, *in what ways may one persona differ from another?*

- **right** (EQUAL): inherent floor (voice, tenure, exit/fork). Attaches to
  the **principal**, flows to clients. Unremovable, never delegated. Standing
  in the system, not in any one Group.

- **weight** (EQUAL as a consequence of the equal right): how much a persona counts. Flat, one per
  distinct persona by lineage, non-inflatable. Attaches to the **persona**.

- **resource** (UNEQUAL, legitimate): what a node has. Attaches to the
  **node/device**. Descriptive, not delegable.

- **Group Role** (UNEQUAL, legitimate): in-Group governance authority granted to a
  **principal** by consent. Scoped, attenuating, revocable. Rides above the
  equalities. (Lowercase "role" is the genus; a "Group Role" is a concrete
  grant inside a Group.)

- **capability** (NOT a fifth property): a Meadowcap data-access grant,
  issued **under a Group Role**, living in the data plane, one level below the
  equality question. Under Group Roles, not beside resources.

### D.6 The bundle, the relation, and the participant/node senses

**Group Role Set** (capital S, first-class) = a named, pinned, Group-recognized bundle of Group Roles and
implied capabilities, bindable to **any principal**. Definable as
`floor + [Group Roles] + [implied capabilities] + [expected resources]`. Two
functions: grant/revoke as one unit, and mutual-exclusion constraints for
separation of powers. It is a set of **Group Roles**, not of principals.
Still-settling: name and functions fixed, full mechanism developing across §5.

**peer** = the **relation**: symmetric standing between principals. Also kept
for transport (peer-to-peer) and the consensus sense ("every honest peer
agrees"). Never a noun for the entity. **participant** = a single
sync-protocol actor performing a mechanical step. **node** = the hardware box;
device is-a node.

### D.7 Invariants of record

- **I1.** Only a Group-governance principal (a persona, or a Group acting as
  one) holds a **Group Role**, a **right**, or **weight**. A meer, relay,
  node, device, or client holds none of these. (A meer is still a *broad-plane*
  principal, reasoned about through the **ecosystem permissions** it carries,
  §5.4; "holds no Group Role/right/weight" is a claim about the
  Group-governance plane, not a claim that the meer is authority-less.)

- **I2.** weight→persona; right→principal (flows to clients);
  resource→node/device; Group Role→granted-to-principal (in-Group governance
  authority); ecosystem permission→held-by-non-Group-principal (meer, relay).

- **I3.** The fold counts personae per rooting key pair, never clients or
  devices; thresholds and quorums count personae by lineage.

- **I4.** capability is issued-under-a-Group-Role and lives in the data plane;
  never one of the four equality-properties.

- **I5.** The locus of adjudication is the principal (persona where the human
  kind is specifically meant); never a node, and never the relation-sense
  "peer."

- **I6.** The persona-to-human binding is a group judgment, never a protocol
  fact; to *recognize* is to decide to *treat as*, never to *verify*.

Sanctioned exceptions (checked, deliberately kept): §3.1 "zero peers in the
sense that matters" reads as the relation (absence of peer-standing in a
sensor mesh), not the entity; the consensus-sense "peer"; and the compounds
peer-to-peer / peer-governed / of peers / P-Peer-Equality.

### D.8 Delivery-plane vocabulary (§6 coinages)

**Delivery Fabric (DF)** = the *carrying* population: a blind, content-agnostic
overlay (gossip, direct links, relays) that moves sealed messages. Larger than
and overlapping the Groups that run over it; a node on it sees ciphertext and
routing metadata at most, never content (§6.3). Distinct from the **Group**,
the *entitlement* population (who holds leaf keys to read). The single object
crossing between the two is the **sealed message**, which is payload, delivery,
and gap-definition at once (§6.3).

**scope** relates to the fabric thus: a Group's **scope** (§5.4) is its
*exposure envelope measured over the Delivery Fabric*, of which the gossip
topic is one large contributor and each helper in the path (relay, meer,
push-notify node) is another (§4.2). Fabric is the substrate; scope is how much
of it a Group's sealed traffic touches.

**The three planes** (a delivery arrangement is a pairing, one of each;
independent axes except where fused by construction):

- **Carriage (C-)**, the path a message travels: **C-direct**, **C-swarm**
  (the gossip overlay), **C-relay** (§6.5).

- **Durability (D-)**, where sealed bytes persist for an offline recipient:
  **D-self** (the floor, participants themselves), **D-meer** (a blind
  store-and-forward node), **D-peer** (fellow members re-serve held history)
  (§6.6).

- **Presence (P-)**, who learns a message exists and prompts the fetch:
  **P-none** (poll), **P-gossip** (a carrier's arrival-signal), **P-meer**,
  **P-push** (byte-free mobile wake) (§6.7). Composes as **detector + actuator**.

**Plane fusions of record** (§6.8.2): **D-self is C-direct**; **the meer is one
node on three planes** (D-meer, carry-fetch, P-meer); **C-swarm is fused with
nothing durable** (the load-bearing *non*-fusion).

**gap-aware history convergence** = the one mechanism (detect a nameable gap via
the sealed per-author index; fill it from a self-verifying source via RBSR)
behind C-swarm hole-visibility, D-peer, and device-Group sync (§6.8.1).
Durability-homed but cross-plane. **device-Group** = a persona's own devices as
an ordinary first-order Group, lineage-restricted admission, a durability
amplifier (§6.6.5).

**The three resource-asymmetry roles** (blind, revocable, redundant, never
authorities; instances of the Part 1 §2.3 resource inequality, holders of
ecosystem permissions not Group Roles): **relay** (reachability, §6.5.2),
**meer** (blind storage, §6.6.2), **push-notify** (wake, §6.7.1).

## Appendix E. The running example, chained: a Group over its life

This appendix chains the running example (§3.0) into one continuous arc, from the Group's formation through
a fork to convergence and, later, a re-composition. Each beat is numbered (E1 to E10) and lists the sections
it demonstrates, and the in-body beat in each of those sections points back to the beat here, so the local
demonstration and the whole journey reference each other. The narrative adds nothing normative; it shows the
clauses those sections already carry, landing on the cast.

**E1. Formation.** Alice creates the Group. Her phone and her laptop are two clients under one key lineage,
and receivers fold them to one persona, so Alice is one member with one unit of weight however many devices
she runs. She holds a moderator Group Role Set, a named bundle of authority layered over the same inherent
rights floor every member holds equally: the Set changes what Alice may do, never what she is entitled to
or how much she counts. Bob joins with a single phone, holding that same floor and nothing more. The Group's
founding membership is recorded as signed, append-only governance facts, and authority over the Group is the
deterministic fold over them. *(Demonstrates: §5.2, §4.5, §5.3, §5.6, §7.3.)*

**E2. The first message.** Alice posts, and the entry travels as a signed, hash-chained message inside two
encryption layers: the transport layer that hides it on the wire, and the message layer that keeps it
unreadable to anyone outside the Group. When Bob's laptop receives it, the laptop verifies Alice's signature
against her published key before acting on the message at all, because acting first would be trusting bytes
whose author is not yet established; and a valid signature is necessary but not sufficient, since the laptop
also checks that Alice has the standing to say what she said. A relay that helps carry the frame sees only
endpoints, timing, and size, never the content, and is never required to read it. *(Demonstrates: §4.1,
§4.3, §4.4, §6.2, §6.5, §6.4.)*

**E3. Admitting Carol's node.** The Group wants deep history a phone cannot hold and a search a phone cannot
run, so it admits Carol's always-on node as a helper. Alice delegates it a read capability, and the grant is
attenuating: it conveys a subset of what Alice holds, never a superset, because a delegation exceeding the
delegator would mint authority from nothing. The node now holds clear text, which widens the confidentiality
surface to it, but it gains no standing whatever: it cannot foreclose, remove, or govern, and the Group can
revoke it exactly as it admitted it. Forward secrecy over the live key schedule is untouched by the node
holding durable history, because the two answer different questions. *(Demonstrates: Part 1 §2.7, §5.4,
§5.5, §6.6, §8.1.)*

**E4. Adding Dave and Erin, and setting the threshold.** The Group admits Dave, who runs a phone and a
tablet, and Erin, who runs a phone, and sets a k-of-n threshold for its membership and revocation decisions.
Each is recorded as a new append-only governance fact, not as an edit to a mutable state: nothing is
overwritten, the prior state remains in the log, and the current authority is read as the fold over the
whole fact set. *(Demonstrates: §7.2, §7.3, §5.7.)*

**E5. The concurrent removal.** A dispute breaks. Alice moves to remove Bob at the same time Bob moves to
remove Alice, both at equal standing, neither observing the other's act first. There is no causal answer,
because the two facts are genuinely concurrent, and there is no fact of the matter about who should remain:
this is the residue the razor names, where provenance is fully settled and utility is still open, and it is
a value the members hold, not a truth a machine can compute. *(Demonstrates: Part 1 §2.5, §7.3.2, §7.6.1.)*

**E6. Resolution: tiebreak, or hard-stop.** Two paths part here by what the contradiction actually is. Where
it is a benign sync artifact, the deterministic tiebreak decides among the genuine concurrents by content
address, party-neutral and identical on every node, so exactly one survives, never both and never neither.
Where it is a genuine social dispute, the protocol refuses to auto-merge: the reconcile hard-stop fires and
escalates to the people, because manufacturing a single answer would be imposing a fiction. Which of the two
a given contradiction is, is itself a governed tolerance the Group tunes, not a hardcoded constant.
*(Demonstrates: §7.3.1, §7.6, §7.4.1.)*

**E7. The ban as a forced fork.** The Group bans Bob. A ban is not an erasure: it is a forced fork, the same
lineage-divergence primitive as a voluntary departure, differing only in its artifacts. Bob continues whole
in his own lineage, holding everything he had; the Group's harshest power over him ends its corroboration of
him going forward and reaches no further. He cannot be re-admitted above the ceiling the log establishes,
and any irreversible action premised on his removal fails closed until that removal is final, never on a
merely probable state. *(Demonstrates: §7.6.4, §7.3.5, §7.3.8, §5.9, Part 1 §2.5.)*

**E8. Who lands where.** The split resolves as individual choices, not a group verdict. Dave and Erin sat
through the same dispute and land on opposite sides: Dave stays with the Group, Erin goes with Bob's
lineage, and neither landing is the correct one, because the substrate does not pick a side for a person.
Carol's node, a bystander, keeps serving whichever side still has it admitted. Bob, the subject, is in his
own lineage. The epoch roll is the mechanical form of the audience split, a hold can suspend enactment while
people decide, and being in both lineages is a permanent, legitimate state, with a later merge cheap but
never required. The group-level outcome is the aggregate of these individual choices, which is faithful
representation, not a resolution the substrate imposed. *(Demonstrates: §7.6.6, §7.6.7, §7.6.8, Part 1
§2.8.)*

**E9. Convergence.** Dave's phone, offline through the dispute, comes back. It first computes a
stale-but-honest view from the facts it already holds, correct as far as it has seen and never presenting a
partial state as the whole current one, then closes the gap by gap-aware history convergence, pulling the
entries it missed from Carol's node or a peer and folding them in. A minimal push is what woke it to look.
When it has caught up its view matches everyone else's, reached with no node having held a privileged copy
the others deferred to. *(Demonstrates: Part 1 §2.1, §6.8, §7.4, §6.7.)*

**E10. Re-composition.** Long after the split, Bob's lineage and the Group choose to reunite. There is no MLS
merge; the re-forming authority instead declares a view, re-planting a fresh Group over both memberships,
taking the more restrictive of the two thresholds, and excluding anyone banned on either side. Two personae
begin this beat from the same condition, banned: Bob, whom the Group banned in E7, and Erin, who went with
his lineage at the fork and whom it banned during the years apart. They reach opposite outcomes, and both
are legitimate. Re-admitting
Bob is an explicit governed act in the view rather than an automatic carry-across, so he returns; no one
moves to re-admit Erin, so the floor holds and she stays out. The difference is not the substrate's to make
but the reuniting parties', and the protocol only furnishes that both moves were possible. Every node
computes the same membership from the view and the shared facts, and where the view leaves a contested role
unsettled the authority decides rather than the machine guessing. Being in both lineages was legitimate and
permanent; reuniting is one available move, chosen and governed, never owed. *(Demonstrates: §7.6.10,
§7.6.2, §7.3.5, §7.3.8.)*

---

## References

**Normative:** BCP 14 (RFC 2119 / RFC 8174); the signature and hash suites of the committed wire profile
(§4.1); QUIC (RFC 9000) and TLS 1.3 (RFC 8446); the iroh transport (iroh core **1.0**, wire-and-API-stable,
June 2026), with the overlay and discovery layers supplied by the separately-versioned `iroh-gossip`,
`iroh-mainline-address-lookup`, and `iroh-mdns-address-lookup` crates (pinned versions tracked in Appendix
B); RTP-over-QUIC for media (§6.12). The FACTCHECK SoT remains the internal cross-check of record for these
external facts.

**Informative (and [confirm] where load-bearing; consolidated in Appendix C):** the CALM
theorem (Hellerstein & Alvaro) as the formal boundary for the escalation cut; CRDTs / local-first
(Shapiro; Kleppmann et al.) as the data-layer premise; the Willow data model (namespace / subspace / path;
range-based set reconciliation; authorized-write hook), Meadowcap (delegated **capabilities**, read/write
data-access grants, kept verbatim as Drystone's data-access term, §5.5; attenuation by subsetting;
communal/owned namespaces) and Keyhive (convergent capabilities / membership graphs) as the two
data-access **capability** tracks, with the in-Group **Group Role** layer (governance authority) sitting above
them (§5.5); Matrix State Resolution v2 / v2.1 (MSC1442 / MSC4297), the 2025 Project Hydra disclosures
and CVE-2025-49090 / CVE-2025-54315, and MSC4289 / MSC4291 as the rejected-ordering, adopted-closure, and
uncapped-root-steelman references that motivate §7; decentralized-MLS (`draft-kohbrok-mls-dmls` / FREEK;
`draft-xue-distributed-mls`) and MLS (RFC 9420) for the forward-secrecy cost of center-free ordering;
Modular Politics (Schneider, De Filippi, Frey, Tan, Zhang, CSCW 2021) and Ostrom's IAD/polycentric work as
the governance-as-protocol neighbor; and Sigstore's signature-transparency model (Rekor: an append-only
Merkle log with signed tree-head checkpoints) as the nearest deployed analogue for the sign-over-prior-
signed-state shape of the frontier commitment (§7.5.1), noting that Sigstore's own primitive is transparency
via signed checkpoints, not a thing it calls "countersigning."

**Transport, secure-messaging architecture, and overlay (the §6 mechanism lineage):** MLS (RFC 9420, July
2023) for the group-key core and the `PublicMessage`/`PrivateMessage` framing; the MLS Architecture (RFC
9750, April 2025) for the AS/DS trust model and the transport/MLS division of labor (§6.2, §6.5, §6.6,
verified against the primaries this round); QUIC (RFC 9000) and TLS 1.3 (RFC 8446) for the transport and
its record-padding mechanism; and, for the iroh-gossip overlay (§6.10), the two source algorithms, each its
own 2007 paper by Leitão, Pereira & Rodrigues: **PlumTree** ("Epidemic Broadcast Trees," *Proc. 26th IEEE
SRDS*, 2007, pp. 301–310) for the eager/lazy spanning-tree broadcast, and **HyParView** ("HyParView: A
Membership Protocol for Reliable Gossip-Based Broadcast," *Proc. IEEE/IFIP DSN*, 2007) for the
active/passive-view membership layer. The iroh-implementation specifics of all of the above are
**[confirm]** against the pinned release (Appendix B); the RFC architecture claims and the algorithm
attribution are verified.

---

## Upstream reference links (versioned)

This section pins each external dependency and citation to a canonical, version-specific source, so a reader or implementer resolves the *exact* version this specification was written against rather than a moving "latest." Where a source was read against its primary this revision, it is marked *(verified this revision)*; otherwise the link is the canonical location to confirm against before publication. Version-sensitive dependencies carry their pinned version; where the production profile has not yet pinned a version (the pre-1.0 iroh crates), that is stated and the pin is tracked in Appendix B.

### Secure-messaging core (MLS)

- **RFC 9420, The Messaging Layer Security (MLS) Protocol** (Standards Track, July 2023). https://www.rfc-editor.org/rfc/rfc9420.html . The group-key core, the `PublicMessage`/`PrivateMessage` framing (§6), the transcript-hash chain (§8.2), the resumption PSK (§8.6), the epoch authenticator (§8.7), ReInit and branching (§11.2, §11.3). *(§8.2, §8.6 verified verbatim this revision; §8.7 epoch-authenticator adoption still an open candidate, §10.2.1.)*

- **RFC 9750, The Messaging Layer Security (MLS) Architecture** (Standards Track, April 2025). https://www.rfc-editor.org/rfc/rfc9750.html . The AS/DS trust model, the transport/MLS division of labor (§6.2), the header-metadata leak (`group_id` plus numerical epoch, §16.4.1 / the metadata section), the KeyPackage-reuse tradeoff (§16.8), and the interoperability-parameter list (out-of-order tolerance, resumption-PSK retention). *(Metadata-leak and DS-ordering claims verified verbatim this revision.)*

- **draft-kohbrok-mls-decentralized-mls** (DMLS) and **FREEK** (Alwen, Mularczyk, Tselekounis, IACR ePrint 2023/394). https://datatracker.ietf.org/doc/draft-kohbrok-mls-decentralized-mls/ and https://eprint.iacr.org/2023/394 . The PPRF-based forward-secure retained init secrets and content-derived epoch identifiers (§8.1.1). *Preliminary, tracked not depended on.* **[confirm against the current draft revision before publish.]**

### Transport and overlay (iroh, QUIC, gossip)

- **iroh, version 1.0.0** (wire-and-API-stable, June 2026). Crate: https://crates.io/crates/iroh/1.0.0 . Docs: https://docs.rs/iroh/1.0.0 . Source: https://github.com/n0-computer/iroh . The public-key (`EndpointId`/Ed25519) transport, dial-by-key with TLS-identity authentication, direct-first hole-punch with blind relay fallback (§6.1, §6.5). *(1.0 release and the EndpointId/dial-by-key API verified this revision; the 1.0 wire-and-API stability guarantee is load-bearing for the transport plane.)*

- **`iroh-gossip`** (separately versioned, pre-1.0 own release line, outside the iroh 1.0 guarantee). Crate: https://crates.io/crates/iroh-gossip . Source: https://github.com/n0-computer/iroh-gossip . The HyParView/PlumTree overlay realization (§6.10). **The production profile must pin a version; the pinned version and its `Event` surface, `broadcast` semantics, and view-size constants are tracked in Appendix B. [confirm against the pinned version.]**

- **`iroh-mainline-address-lookup` and `iroh-mdns-address-lookup`** (separately versioned, pre-1.0). Source: https://github.com/n0-computer . The discovery layer (§6.9.2). **Republish/expiry behavior is [confirm] against each crate's pinned version; the Pkarr self-signed-record model is a Pkarr-spec primary (below).**

- **Pkarr (Public-Key Addressable Resource Records).** https://github.com/pubky/pkarr . The self-signed-record integrity model underneath the DNS/Pkarr discovery path (§6.9.2). **[confirm against the Pkarr spec.]**

- **RFC 9000, QUIC: A UDP-Based Multiplexed and Secure Transport** (Standards Track, May 2021). https://www.rfc-editor.org/rfc/rfc9000.html . The transport iroh is built on (§6.1).

- **RFC 8446, The Transport Layer Security (TLS) Protocol Version 1.3** (Standards Track, August 2018). https://www.rfc-editor.org/rfc/rfc8446.html . The handshake authenticating the `EndpointId` and the record-padding mechanism (§6.2, §6.4).

- **RTP-over-QUIC (RoQ), `draft-ietf-avtcore-rtp-over-quic`** (Internet-Draft, not yet a published RFC; -14 as of this revision). https://datatracker.ietf.org/doc/draft-ietf-avtcore-rtp-over-quic/ . The media-transport reference for the real-codec path (§6.12). *(Corrected this revision: RoQ has no RFC number yet, the draft's own text states that only implementations of the final published RFC may use the "roq" ALPN token; treat the media path as riding a not-yet-final draft.)*

- **PlumTree**, Leitão, Pereira & Rodrigues, "Epidemic Broadcast Trees," *Proc. 26th IEEE SRDS*, 2007, pp. 301–310. https://doi.org/10.1109/SRDS.2007.27 . The eager/lazy spanning-tree broadcast (§6.10). *(Algorithm attribution verified.)*

- **HyParView**, Leitão, Pereira & Rodrigues, "HyParView: A Membership Protocol for Reliable Gossip-Based Broadcast," *Proc. IEEE/IFIP DSN*, 2007, pp. 419–429. https://doi.org/10.1109/DSN.2007.56 . The active/passive-view membership layer (§6.10). *(Algorithm attribution verified.)*

### Data model and capabilities (Willow family, Keyhive)

- **Willow Data Model.** https://willowprotocol.org/specs/data-model/index.html . The namespace/subspace/path structure and the `is_authorised_write` hook Drystone is built shaped-toward (§7.1). *(Data model verified this revision.)*

- **Meadowcap.** https://willowprotocol.org/specs/meadowcap/index.html . The delegated data-access capability system: attenuation by subsetting, communal vs owned namespaces (§5.5, §5.10, the Track A capability option). *(Capability model and attenuation verified this revision.)*

- **Range-Based Set Reconciliation**, Aljoscha Meyer, arXiv:2212.13567 (v2, Feb 2023); peer-reviewed as *Proc. 42nd IEEE SRDS*, 2023, pp. 59–69. https://arxiv.org/abs/2212.13567 . The RBSR sync technique underneath gap-aware convergence (§6.8.1, §7.1). *(Identifier and venue verified this revision; also the sync algorithm iroh-docs cites.)*

- **Willow reference implementation (Rust).** https://codeberg.org/worm-blossom/willow_rs (formerly github.com/earthstar-project/willow-rs). Noted for version-finding; the spec, not the implementation, is normative.

- **Keyhive** (Ink & Switch), the convergent-capability / membership-graph alternative (Track B, §7.2 capability-mechanism decision). https://www.inkandswitch.com/keyhive/ . **[confirm the current canonical location and the convergent-capability claims before publish.]**

### Governance-conflict resolution (Matrix State Resolution, the neighbor Drystone contrasts with)

- **Matrix State Resolution v2**, MSC1442. https://github.com/matrix-org/matrix-spec-proposals/blob/main/proposals/1442-state-resolution.md . The DAG-plus-Kahn's-algorithm skeleton Drystone shares but re-orders (§7.5.2, Appendix C).

- **Matrix Project Hydra disclosure** (August 2025). https://matrix.org/blog/2025/08/project-hydra-improving-state-res/ . The 2025 state-resolution security work. *(Verified this revision.)*

- **CVE-2025-49090** (State Resolution 2.0 state-reset class). https://www.cve.org/CVERecord?id=CVE-2025-49090 . Cited as cautionary evidence for the monotonic-fold choice (§7.3). *(Verified this revision: state reset to an earlier/incorrect value absent a validly-producing event, exploitable by a malicious homeserver via a crafted event/API sequence; fixed by State Res v2.1.)*

- **CVE-2025-54315** (rooms before v12 lack cryptographic create-event uniqueness; High; no known exploitation path). https://www.cve.org/CVERecord?id=CVE-2025-54315 . Corresponds to Drystone's structural root-id closure (§7.3). *(Verified this revision.)*

- **MSC4289** (explicitly privilege room creators; creator "infinite" power, with the backdating rationale; creator self-demotion; multiple creators). https://github.com/matrix-org/matrix-spec-proposals/blob/main/proposals/4289-explicitly-privilege-room-creators.md . The uncapped-root steelman (§5.7, §7.3). *(Verified this revision against MSC4289 and the Hydra disclosure.)*

- **MSC4291** (room ID as the hash of the create event) and **MSC4297** (conflicted-subgraph closure, State Resolution v2.1). https://github.com/matrix-org/matrix-spec-proposals/blob/main/proposals/4291-room-ids-as-hashes.md , https://github.com/matrix-org/matrix-spec-proposals/blob/main/proposals/4297-consistent-state-res.md , with the implementer's guide at https://matrix.org/docs/spec-guides/state-res-2.1/ . MSC4291 corresponds to the genesis-id closure (§7.3); MSC4297's conflicted-subgraph mechanism is *adopted* at §7.5.2. *(Both verified this revision: MSC4291 against the v1.16 notes; MSC4297's two changes, empty-set start and conflicted-subgraph replay via the forward-backward SCC computation, against the State Res v2.1 implementer's guide.)*

- **Matrix spec v1.16 release notes** (the room-version-12 changes above, landed). https://matrix.org/blog/2025/09/17/matrix-v1.16-release/ . *(Verified this revision.)*

### Formal distributed-systems spine

- **CALM theorem**, Hellerstein & Alvaro, "Keeping CALM: When Distributed Consistency is Easy," *CACM* 63(9), 2020. https://cacm.acm.org/research/keeping-calm/ (arXiv preprint https://arxiv.org/abs/1901.01930 ). Conjectured at PODS 2010; proof for queries by Ameloot, Neven & Van den Bussche, *J. ACM* 60(2), 2013. The consistent-coordination-free-iff-monotonic boundary underneath the §7.6 escalation cut. *(Attribution and statement verified this revision.)*

- **CRDTs**, Shapiro, Preguiça, Baquero & Zawirski, "Conflict-free Replicated Data Types," *SSS 2011* (LNCS 6976), pp. 386–400, https://doi.org/10.1007/978-3-642-24550-3_29 ; companion technical report "A Comprehensive Study of Convergent and Commutative Replicated Data Types," INRIA RR-7506, January 2011, https://hal.inria.fr/inria-00555588 . The convergence-without-consensus premise for the data plane (§4, §7.1). *(Venue, DOI, and RR-7506 report number verified this revision; note RR-7506 is the "comprehensive study," distinct from the later RR-7686.)*

### Countersigning pattern

- **Sigstore signature transparency (Rekor).** https://docs.sigstore.dev/logging/overview/ . The nearest deployed analogue for the frontier commitment's sign-over-prior-signed-state shape (§7.5.1): Rekor is an append-only Merkle transparency log that periodically signs its tree head (a signed checkpoint), so an entry proves data existed at a point in a verifiable, non-mutable order. *(Mechanism verified this revision; note Sigstore's primitive is transparency via signed checkpoints and RFC 3161 timestamps, not "countersigning", so Drystone draws the checkpoint/Merkle-consistency analogy, not a literal Sigstore feature.)*

Normative dependencies (BCP 14 keyword usage, RFC 2119 / RFC 8174) are at https://www.rfc-editor.org/info/bcp14 . Everything marked **[confirm]** here is consolidated with its resolution status in Appendix B; the versioned pins for the pre-1.0 iroh crates are the production profile's to fix and are tracked there.
