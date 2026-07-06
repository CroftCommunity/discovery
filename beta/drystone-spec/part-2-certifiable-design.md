# Drystone, Part 2: The Certifiable Design

`Status: beta. Build-against shape complete; byte-level ENABLING encodings open (Appendix B)`

This part is the "what." It specifies the mechanics an implementation is built and validated against.
Each section names the Part 1 principle(s) it `Realizes`. Normative keywords are **MUST / SHOULD / MAY**
(BCP 14). Each normative claim carries a status flag: `green-real` (real crypto/transport), `green-model`
(reference model), `design` (specified, unproven), `ENABLING` (byte-level encoding required for
interoperation, gates a publication-final release), `[confirm before publish]` (rests on an external
fact not yet independently verified).

> **A note on two maturity layers inside this part.** The message/history/transport layer (§4, §6) is
> matured from a reference implementation that is **green-real** on real crypto and real iroh transport.
> The governance-conflict-resolution layer (§7.3–§7.5) is matured from a **design/ENABLING** draft whose
> reasoning is complete but whose wire encodings and one comparative dependency are not yet pinned. The
> two layers also differ today in hash function (§4 proven on SHA-256; §7 designed on BLAKE3), a real
> reconciliation item, surfaced in Appendix B rather than papered over.

> **Naming: center-free, not serverless.** Drystone is a **center-free peer protocol**. *Peer-to-peer*
> describes its transport (§6) accurately and we use it there. We do **not** call it *serverless*;
> that term now names managed ephemeral compute, nearly the inverse of what is meant. The load-bearing
> property is not topology but **where adjudication lives** (§3.1): no node holds privileged or canonical
> authority. Where precision is needed we say *center-free* or *no node holds privileged or canonical
> state*; where we mean the wiring we say *peer-to-peer*. (This replaces the looser "serverless" usage in
> earlier drafts.)

> **Vendor-neutral naming.** Drystone is the protocol. The reference implementation this part is matured
> from carried the historical brand "Croft" in some signed wire constants (e.g. the domain-separation tag
> namespace `croft-*`). Those values are shown where they are what was proven, but they are **the
> reference profile, not the protocol**: Drystone requires a versioned, domain-separated tag, and does not
> mandate that string. Defining the vendor-neutral `drystone-*` tag namespace and re-proving the rename
> (the tag is signed over, so changing it re-opens the signature proofs) is an Appendix B item.

---

## 3. Protocol Overview

A **peer** holds a local store that is canonical for it (`P-Local-Truth`). Peers participate in
**scopes** (groups holding shared state). Within a scope, two kinds of state move:

- **History**: the content peers author, as signed, hash-chained entries. This is the data plane.

- **Governance facts**: signed, append-only entries recording who may do what (admit, expel, grant,
  revoke, amend). Authority is a deterministic fold over these. This is the control plane.

A peer is one or more **devices** (keypairs) acting under a single **lineage**; receivers fold devices
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

### 3.1. What makes this a system *of peers*: the diagnostic that orders the spec

A network of many nodes with any-to-any connectivity is **not** automatically a distributed system of
peers. The distinction this specification turns on is not topological; it is about **where adjudication
lives.** A **peer is a locus that can *adjudicate***; it holds genuine authority over some domain that
other peers must respect, not merely a node that can sense, store, and relay. A node that only senses and
relays, with its decisions made elsewhere, is a **sensor**, however well-connected it is.

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
governance. It is also why the spec is **ordered** the way it is, define the peer (a locus of
adjudication, §5.2), then peer rights (§5.3), then the mechanics that keep adjudication distributed (§7,
§8), and why the **label-not-enforce** posture (§8) is load-bearing rather than cosmetic: enforcement
relocates adjudication to whoever enforces, quietly converting peers into sensors, while labeling leaves
adjudication with the peer and propagates only information. Each enforcement hook looks locally reasonable,
which is exactly how a peer network can **degrade into a sensor mesh** over time without anyone deciding to
centralize. Keeping adjudication at the edge is the protocol's job; surfacing the hard case to a human
(the algedonic escalation, §7.6) is how it does so without pretending rules can resolve everything.

---

## 4. Data Model

> `Realizes: P-Local-Truth, P-Knowable-Truth`

### 4.1. Cryptographic foundation

A device holds a signature keypair; its public half is its verifying identity, its secret half signs. A
**lineage identifier** names a logical actor; multiple devices may act under one lineage (§5.2). A
signature **MUST** verify against the author's published verifying key before any other check, and a valid
signature is **necessary but not sufficient**; standing (§5, §7) is also required.

The reference implementation uses **Ed25519** (RFC 8032, deterministic, 64-byte signatures) and
**SHA-256** (32-byte digests). *green-real* (real Ed25519 over live iroh: a forged message is rejected
as a bad signature on every receiver, including a NAT'd peer). A production profile **MAY** select a
different suite, but the suite is part of the versioned wire profile and **MUST NOT** be silently
negotiated down. *(The abstract signature and hash requirements, and why Ed25519 / the SHA-256-vs-BLAKE3
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

*green-real*: the three tagged wire derivations are conformance-tested canonical functions, byte-identical
to the proving spike. The reference profile's tag strings are `croft-lineage-genesis:`,
`croft-group-genesis:`, `croft-group-topic:`; **Drystone normatively requires a versioned, domain-separated
tag and does not mandate these strings** (Appendix B, naming reconciliation).

> **Note on the content-id pre-image.** Earlier drafts included a `timestamp` field in the content-id
> canonical pre-image. It is removed: per §2.0.1, a wall-clock value is an uncorroborable assertion and
> must not appear in any identity, ordering, or authority-bearing computation. A content id binds *what*
> and *where* (group, regime, author, content), not *when*. If an application wishes to record an authored
> "claimed time," it does so as ordinary payload content, an explicitly author-asserted value, never a
> protocol fact, and never an input to the content id. *(design; the canonical content-id encoding is
> `ENABLING`, Appendix B.)*

A scope's gossip-topic seed **MUST** be high-entropy / salted, not a guessable human handle, otherwise an
adversary computes the topic and joins or observes. *(Leak bound characterized; see §8.)*

### 4.3. The signed message: the unit of history

The history unit binds author, position, branch, and payload so a message cannot be replayed onto another
branch or position. The signed pre-image (reference profile):

```
signing_bytes = "msg-v1" ‖ branch(32) ‖ seq(LE u64) ‖ author_id_bytes ‖ 0x00 ‖ payload
```

A receiver **MUST** recompute the pre-image and verify the signature against the author's key. *green-real*,
the real message traveled live iroh-gossip and verified against a real backfill import; an honest
member's message is accepted, a forged one rejected. (Note: position is `seq`, a per-branch counter, not a
clock, ordering is structural, not temporal, consistent with §2.0.1.)

### 4.4. Integrity-and-ordering vs authorship-and-standing: two distinct guarantees

These are kept strictly separate, and conflating them is the central honesty error the protocol forbids:

- **Integrity + ordering (structural).** A branch is a sequence chained by
  `hash = H(prev ‖ seq(LE) ‖ payload)`; a receiver **MUST** reject a branch with a broken chain or
  non-contiguous sequence numbers. This proves in-transit integrity and contiguous ordering, **not** who
  wrote it.

- **Authorship + standing (authority).** The signature (§4.3) plus standing (§5, §7). A receiver **MUST**
  apply both; **integrity alone MUST NOT be treated as authorization.**

*green-real*: a valid-chain branch from a non-member is accepted by the structural check but rejected by
the authority check as an unauthorized author. This separation is what makes a branch trustworthy: the
hash chain proves it was not tampered in transit; only signature + standing prove it may be there.

### 4.5. Multi-client fold: client-count and device-count ≠ peer-count

A **peer** (the human as represented, §5.2) is rooted in a **cryptographic key pair**. That key pair is the
root of a **cryptographic lineage**: each of the peer's devices and each **client** (group-member software,
§5.2) hosted on them carries a membership key that **descends from the peer's key pair by signed
credential**. Lineage here is literal, the concrete chain of signatures by which a client's membership key
is provably tied back to its rooting peer, not an abstract tier.

Receivers **MUST** fold by following each client's lineage back to its root: every client whose lineage
roots at the same peer counts as **that one peer**, however many clients run on however many devices. A
scope's topic carries many such lineages; the fold is what every peer computes identically to agree on the
member list and on **lineage-rooted thresholds** (§7.2), which count **one peer per rooting key pair**,
never clients or devices. *green-real*: one human's two devices fold to one peer; all peers agree on the
folded count.

#### 4.5.1. Per-client authorship, per-client logical clock, and the principal-as-self-AS

Clients are **not** a shared key. Each **client** (§5.2: software on a device that is a member of a group,
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
by lineage to one peer before any quorum is counted, so more clients or devices never buy more weight.
*(Group-key/MLS facts: confirm against the primary specification, **[confirm before publish]**. The
abstract group-key requirement this rests on, including the no-shared-signature-key-across-a-principal's-
clients property as requirement K5, is consolidated in **§10.2**, where MLS is positioned as the reference
realization rather than the requirement.)*

Because there is no central authority in a center-free system, the **principal key acts as its own
tiny certificate authority for its clients and devices**: the principal signs each client's credential, and
a verifier accepts "this client is principal P" by validating that credential chain. Adding a device or a
client is the deliberate, high-trust act (e.g. scanning a code) that the principal then attests, which is
exactly the trust binding of Part 1 §2.0 (the human act binds existing trust to a key; the credential
records it).

A direct consequence for ordering, and a load-bearing instance of §2.0.1 (*time is not a fact*): the
**logical clock is per-client and strictly logical, never a wall-clock.** Each client advances only its own
`lamport` counter, so there are no cross-client collisions and no coordination is required; a wall-clock is
never consulted, because a wall-clock is an uncorroborable assertion and could not order what must
converge. A principal's stream is the **deterministic fold-time merge** across its clients' streams
(ordered by lamport, then a stable cryptographic tiebreak, never by time). `auth_assertions_by_client`
(the per-client causal index used for range-sync) keys on the client identity for exactly this reason.
*(Local-storage detail; see the implementation build spec.)*

---

## 5. Identity, Principals, Rights, Roles, and Capabilities

> `Realizes: P-Peer-Equality, P-Local-Truth, P-Knowable-Truth, P-Durable-Enablement`

This section fixes the vocabulary the rest of the spec runs on. It is the section a reviewer presses
hardest, because it is where `P-Peer-Equality` is enforced by mechanism rather than assumed, and where a
single overloaded word ("peer") previously hid several independent ideas. The fix is to ask one precise
question, *in what ways may one peer differ from another?*, and to answer it with **exactly four
properties, two necessarily equal and two legitimately unequal**, then to separate the **identity layer**
(principals and clients) from the **governance layer** (peers and weight).

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
> The device-facility layer that an earlier draft mislabelled "capability" is renamed **resource** (§5.4),
> and the in-group governance-authority layer is **role** (§5.5). **Capability** is **not** one of the four
> peer-equality properties below: it is the **data-access mechanism a role operates through** (a role may
> carry the authority to issue capabilities), and it lives in the data plane (§7.1, §10.4), one layer below
> the equality question. So the property nouns sit at distinct planes, **resource** (device fact),
> **role** (group governance authority), **capability** (data access, Meadowcap, beneath roles), and none
> collides with the prior art.

### 5.0. Two equalities, two inequalities: and the two layers

The question `P-Peer-Equality` actually answers is: **in what ways may one peer differ from another?**
There are exactly four properties, and the whole model is that **two are necessarily equal and two are
legitimately unequal.** "Peers are equal" was always shorthand for this four-way split; collapsing it into
one phrase is what produced the earlier confusion.

What makes the equality *matter* is what a peer **is**: a peer is the **representation of a human** in the
system, not a node or a device. So peer-equality is **equality of humans as represented**, equality in
expression and in count. This holds **as long as the human-to-peer binding is one-to-one**, and that
binding is a **social-utility judgment the group makes, not a fact the system can attest** (§2.0, §5.6).
The protocol guarantees provenance and runs governance over peers as represented; it does **not** certify
that one peer is one person. That is the razor (§2.0) applied to the peer concept itself.

**The two equalities**, equal for every peer, always:

- **Right, what a *principal* inherently holds.** The floor: voice, tenure, and exit/fork (§5.3). It is
  **equal for every peer, and unremovable.** The proof that it is a right and not a role is that the last
  of them, exit/fork, survives even when every role is stripped and even when a quorum captures the
  group: participation persists as the standing to leave with your state and continue. A right is precisely
  the thing that *cannot* be delegated or revoked. Attaches to the **principal**, flows to its clients.

- **Weight, how much a *peer* counts in governance.** **Flat: one per distinct peer**, where a peer is
  the unit a group's **members resolve to by lineage** (§4.5), regardless of how many clients, devices,
  resources, or roles carry that lineage. Weight is equal **by necessity, not by separate decree**: it
  *follows from* equal rights. If standing-to-participate is equal (the right), then standing-to-be-counted
  is equal (the weight), the second is the governance image of the first. Attaches to the **peer** (§5.6).
  *(A note on what the group recognizes, and what it does not. A group recognizes its **members**,
  clients, in MLS terms (§5.2). Lineage then resolves a group's member-clients to **one peer** (§4.5), and
  it is that resolved peer that is counted once. The system attests this resolution by provenance. What the
  system does **not** attest is whether a peer corresponds to a distinct **person**: that one-peer-one-human
  binding is a **contextual judgment the group makes** at its own confidence (§5.6). We avoid the phrase
  "personhood-verified": "verified" would imply the system did the verifying, when the protocol guarantees
  provenance and the group judges personhood.)*

**The two inequalities**, legitimately different between peers:

- **Resource, what a *node* has.** Storage, uptime, reachability, a push token, a radio, "this
  box is willing to relay." A property of the **device or node**, not of identity: **intrinsic**,
  **descriptive**, **expected to be unequal**, and **not delegable**, you cannot hand another node your
  RAM. It is a fact about every node in the system (a peer's clients, but also meers and relays, §5.4),
  not only about peers; it is listed among the peer inequalities because, *across peers*, it is one of the
  two ways they legitimately differ. A resource says what is *possible*, never what is *permitted* and
  never how much a peer *counts* (§5.4). *(This is the layer an earlier draft called "capability";
  renamed because "capability" is Meadowcap's word for data access, and because "resource" names the
  device fact without inviting the false slide from "able to" to "entitled to.")*

- **Role, what governance authority a *principal* has been *granted*.** Admin, moderator, gating, the
  act-for-the-group authority, the authority to issue capabilities (§5.5). A role is **granted by member
  consent, scoped, attenuating, and always revocable.** It is the one *operational* inequality the design
  permits: peers may hold different roles, and that is normal. Crucially, **a role rides entirely above the
  two equalities**, granting or revoking one never changes a peer's rights floor or its unit of weight.
  Roles are the application-layer construct MLS deliberately leaves undefined (§5.5).

So the sentence that replaces every earlier formulation: **peers are equal in rights and (by necessity)
weight, and unequal in resources and revocable roles.** The old phrase "equal in rights, not capabilities"
was wrong twice over: it used "capabilities" for device facts (now *resources*), and it implied rights
could be unequal when the inequality it had in mind was always a *role*. Rights do not vary. Roles do.

A note on what is **not** on this list. **Capability** (the Meadowcap data-access grant, read/write an
area of a namespace, §5.5) is not a fifth peer-property. It is the **mechanism a role operates through**: a
role may carry the authority to *issue* capabilities, and the capabilities themselves are data-plane tokens
(§7.1, §10.4), one level below the question of how peers differ. It is listed here only to place it: it
sits *under* roles, not beside resources.

And two layers, because the entity that holds rights is not the same granularity as the device that acts:

- **Identity layer, principals and clients.** A **principal** is a role-holding entity identified by
  exactly one authenticatable identity (one key-lineage). A **client** is **software on a device that is a
  member of a group**, one MLS leaf, one signature key, one credential (the term is carried over from MLS
  for consistency). A **device** is the hardware (a node, §5.4); a device may host **more than one
  client**, and a principal is **realized by one or more clients across one or more devices**.

- **Governance layer, peers and weight.** A **peer** is the **representation of a human** in the system:
  the principal behind which there is a person, and the entity rights and weight attach to *because* a
  person is there. A peer is **not a node** (a node is a box, with resources, §5.4); it is the
  human-as-represented, rooted in a **cryptographic key pair** from which its devices' and clients'
  membership keys descend by signed credential (its **lineage**, §4.5). Its clients run on its devices; the
  peer is neither. It carries the rights floor and is the source of one unit of governance weight, and the
  fold counts **one peer per rooting key pair** however many clients and devices carry that lineage. The
  binding "one peer is one human" is what makes that weight meaningful, and that binding is a **social
  judgment the group makes, never something the system attests** (§5.6, §2.0): the protocol attests
  provenance and runs governance over peers as represented; whether a peer corresponds to a distinct person
  is the group's contextual call.

### 5.1. The only canonical state is local

A client can prove exactly one thing: its own local state. Its local store is **canonical for that
principal**. Any belief about another principal's state is **comparative** (range reconciliation) or
**asserted** (a signed fact it accepted), never canonical. There is **no global canonical state**, by
design. A lagging client computes a stale-but-honest state, never a false one, because it is only ever
reading its own store. *green-real / design*, the property §7 relies on to make authority a fold over an
append-only view.

### 5.2. Principal, client, peer: the identity model

A **principal** is a **role-holding entity, identified by one key-lineage.** This is the genus. It is
defined by its *identity* (one authenticatable lineage), not merely by its function, so that "holds a
role" does not collapse into "anything at all." Kinds of principal:

- a **peer**, the principal that **represents a human** in the system, carrying the rights floor (the
  common case: one person, one identity, possibly many devices). Rights and weight attach to the peer
  because a person is behind it. Its clients run on its devices, tied to the peer by lineage (§4.5); the
  peer is neither a client nor a device nor any node;

- a **group**, a collective that can hold a role as a single principal (its identity model is an **open
  seam**, see below);

- a **delegate**, not a separate species but a **state**: a peer or group currently holding a role
  delegated by another principal (§5.5).

A **meer** is **not** a principal and does not appear above. "Meer" is a colloquialism for a blind
store-and-forward node: infrastructure, defined in §5.4 (a node offering availability capacity, configured
by a scope to serve ciphertext) and §6 (transport). The legacy labels "mere-peer," "blind member," and
"blind peer" are all wrong: a meer is neither a member nor a peer, and holds no role. It is named by scope
configuration, not enrolled as an identity.

A **client** is **software on a device that is a member of a group**: one MLS **leaf**, one **signature
key**, one **credential**, authenticated as a **member** via the **AS** (§10.2). The term is MLS's, kept
for consistency. A **device** is hardware (a node, §5.4) and may host **more than one client**; a human
may have **more than one device**. So the hosting chain is human → devices → clients, and a principal is
**realized by one or more clients across one or more devices.** MLS addresses clients; Drystone governance
addresses principals, folding a principal's clients and devices, by lineage, to one peer (§4.5).

> **The governance-integrity spine, identity, not client count.** Governance quorums and thresholds count
> **peers (resolved by lineage to one peer per rooting key pair, §4.5), never clients.** Many clients across several devices are one
> identity's worth of standing. This is what makes "you cannot buy your way to shifting the centre of
> gravity" structurally true: adding clients or devices adds **resources**, never **rights** and never
> **weight**, because the count is over principals. The governance layer discerns the **lineage** of each
> client (the MLS leaf) and folds a principal's clients and devices to one peer, so a principal with one
> device and a principal with five are weighted identically. This is the property whose absence made early
> crypto-governance takeovers possible and painful to unwind; Drystone makes identity-not-resource the
> basis of weight by construction.

> **The keystone distinction, a lineage is a provenance object; a peer is the human it represents, and
> personhood is a social judgment.** These are different *kinds* of thing, and keeping them distinct is the
> identity-layer instance of the spec's founding provenance/utility split (§2.0).
>
> - A **lineage** is **technically representable**: it is a cryptographic-provenance chain, a thing the
>   protocol can point at, verify signatures against, and count. The protocol delivers the lineage with
>   certainty.
>
> - A **peer** is the **human that lineage is taken to represent**. Whether a given lineage corresponds to
>   a distinct person, the one-peer-one-human binding, has **no technical representation at all**, because
>   it was never a fact the system holds. It is a judgment the *group* makes (§5.6). The system counts
>   lineages; the group decides which lineages it recognizes as distinct persons, and *that* recognition is
>   what turns a counted lineage into a weighted peer.
>
> So the binding between the two, *this lineage is one person*, is **not a lookup but an adjudication**,
> and that is precisely why it is a seat of social-utility judgment rather than something the spec
> resolves. Collapsing peer into personhood is the same category error as "the network can certify truth":
> it asks a provenance system to deliver a utility verdict. This is *why* "peer" and "personhood" are
> separate words in this spec, not loose synonyms, the separation is load-bearing, and how the binding is
> structured is set on the same **per-edge adversarial dial** as all other trust (Part 1 §2.3), because
> the posture toward "is this peer one person" is no more a single global setting than the posture toward
> any other edge.

> **Open seam, the principal that anchors a multi-client lineage.** The cross-device identity is the
> **principal**; "operator" and "user" were both rejected (one too effusive of a privileged actor, one too
> imprecise). `principal` is the chosen term. The **group-as-principal identity** is now given a concrete
> shape in §5.10 (a Meadowcap **communal namespace** rather than a derived central credential); what
> remains designed-not-frozen is its key establishment and rotation under membership change. Carried to
> Appendix B (the collective / federation gap).

### 5.3. Rights: the inherent, equal floor: never delegated, never unequal

A right is what a principal **inherently holds**, not what it may be *granted* (that is a role, §5.5). The
floor is held **identically by every peer** and **cannot** be delegated away or stripped without degrading
the system (Part 1 §2.4); it is one of the two equalities of §5.0. The base floor:

- **Read your own local history.** Unqualified, identical for every peer.

- **Read the history of a scope you are a member of, for the period of your membership.** Begins at join,
  ends at leave; includes what the peer was present for; does not extend to content authored after the
  peer leaves, and does not retroactively vanish for content the peer legitimately held while a member
  (§5.7).

- **A scope holds full history for itself,** independent of any member's tenure.

Two consequences where the floor is most often misread. **A peer's own history is permanent; its window
into a shared scope is bounded by membership**, two different histories, treated as such. **A principal
with no local history of its own still holds the full read-your-own-history right**, exercised over an
empty set (a peer that has joined but authored nothing satisfies the right vacuously). A meer is not the
example here: it is infrastructure, not a principal, so it holds no rights at all (§5.4). Where a principal
is *blind*, that is the absence of a key and of any role conferring read, never a restriction on a right.

> **One open check before the rights set hardens** (carried from Part 1 §2.4): the proven floor in this
> draft is the read-rights triple above. The fuller rights articulation is **three rights**, **tenure**
> (standing to remain a peer), **voice** (standing to assert into the record and be corroborated or
> refuted), and **exit** (the right to fork); each fixed by what its removal would foreclose. (An earlier
> draft floated a fourth, `share`, a claim on a scope's commons. It is **dropped as a right**: a claim on
> shared assets is not part of the inalienable floor. Where it has substance it belongs in the data layer
> as ownership of a Meadowcap communal namespace (§5.10), not in the rights set. What survives of that idea
> is the communal-asset model, not a right.)
>
> The remaining open check is **`tenure` under re-key**: can the §7 survivor / re-key path leave a peer
> formally a member but unable to re-establish its standing after a re-key? If so, tenure is not yet a
> clean right and the set cannot harden. This needs a concrete test (see Appendix B for what to exercise).
> *design* (Appendix B). The Meadowcap distinction between **communal** namespaces (authority from owning a
> subspace key; horizontal, no apex) and **owned** namespaces (top-down single-owner control) is the
> nearest prior-art cut at the commons-asset question and the model for the group-principal (§5.10).
> *(Meadowcap communal/owned semantics confirmed verbatim against the Willow Protocol spec; see §5.10.)*

### 5.4. Resources: node facilities, descriptive, not delegable

A **resource** is what a **node has**, a facility intrinsic to the hardware and its configured intent.
Resources are a property of **any node in the distributed system**, not only of a peer's clients: a peer's
client devices have resources, and so do meers (the blind store-and-forward nodes, below) and relays. The
peer/meer distinction is drawn in the **identity** model (§5.2), never in resources; resources are a
physical fact about a box, blind to whether that box is an identity. Resources are **descriptive** (they
report what is possible), **unequal across nodes**, and **not delegable**: a node cannot hand another node
its storage, uptime, or radio. A resource enables a principal to *fulfil* a role (§5.5), or makes it
*unsuited* to one, but holding a resource is never itself a grant of authority and never adds governance
weight.

**Common resources** (the current set, not closed):

- **Availability capacity.** The device can hold and serve a scope's *encrypted* objects, including
  buffering for offline members. Availability capacity does **not** imply read: such a device holds
  ciphertext it cannot decrypt, because it is never given a key. A node configured by a scope to do only
  this, blindly, is a **meer** (below); it is infrastructure, not a principal.

- **Read / search-offload capacity.** The device can decrypt, index, retain, and serve a scope's history
  (the *facility*; whether the principal is *permitted* to is a read **role**, §5.5).

- **Reachability.** The device is positioned (public reachability, uptime) to forward for others.

The pairing rule (for the resources that matter to *governance*, namely a principal's own clients): a
**role** (the governance authority, §5.5) is only useful to a principal whose **client** has the
**resource** to exercise it. Granting a read role to a device with no decryption capacity is inert. Roles
and resources are matched at the **PeerSet** layer (§5.5), which is exactly why a PeerSet bundles a role
set *with an expectation of resources*. The anti-capture consequence is in the words: a node may have more
resources, and that buys it no rights and no weight, only the ability to be *useful*, never to *count for
more*.

**The meer: blind store-and-forward infrastructure.** "Meer" is a **colloquialism**, not a model entity:
it is just a short name for a **blind store-and-forward node**. Such a node accepts, retains, and serves a
scope's encrypted objects, seeing ciphertext plus routing metadata only. It is **not a principal, member,
or peer**, holds **no role**, **no rights floor**, and **no weight**. It is named by a scope's
configuration (the scope records which store-and-forward endpoints it uses), not granted a role and not
enrolled as an identity. Its blindness is **structural**: it is never issued a decryption key, so there is
no "decrypt" to forbid and no role to strip. A Tier-0 meer can prove it holds zero payload keys (§8).
Electing, replacing, or dropping a meer is a configuration fact about the scope's infrastructure (§5.9),
not a grant to a principal.

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

### 5.5. Role, capability, PeerSet, and delegation: the governance and data-access planes

Two distinct kinds of grant sit above the rights floor, at two different planes, and a third construct
bundles them. None touches the inherent rights floor or the flat weight.

- **Role, an in-group governance authority.** A scoped, attenuable authority to *act* in the group's
  governance: admit or remove members, gate distribution, hold the **act-for-the-group** authority (§5.10),
  or *issue and revoke capabilities* over the group's data. A role is **granted to a principal** by member
  consent, is **revocable** (§5.7), composes by union, and mutates freely, granting or revoking one is
  normal governance traffic, no alarm. Each grant is a governance fact (§7). **This is the layer MLS
  deliberately leaves to the application**, RFC 9750 §6.4 (Access Control) states that MLS "does not itself enforce any
  access control on group operations" (any member can add or evict), in contrast to designs with a single
  group controller, and that "MLS-using applications are responsible for setting their own access control
  policies", giving the example that if only an administrator may change members, the application must
  inform members of that policy and who the administrator is (§10.2). Drystone's role layer **is** that
  application policy, enforced not by a server but by the governance fold: a child's `Remove(parent)` is a
  well-formed MLS message that honest peers **reject as unauthorized** because the replicated role policy
  does not grant the child that authority. *(Honest seam: the protection is convergent agreement that the
  op is unauthorized, not cryptographic impossibility of emitting it, §5.7.)*

- **Capability, a data-access grant (Meadowcap's sense, kept verbatim).** An **unforgeable token
  bestowing read or write access to an area of a namespace**, issued by that data's owner, attenuating
  under delegation. A capability is about **reading and writing entries**, nothing else: it does not admit
  members, does not gate, does not carry a vote. Capabilities are *issued under* a role (the authority to
  issue them) and live in the data plane (§7.1, §10.4). Drystone keeps Meadowcap's word because Drystone
  intends Meadowcap (or a Meadowcap-shaped mechanism) as the data-access realization, and renaming it
  would fight the prior art.

- **PeerSet, a named, pinned, group-recognized bundle** of roles, the capabilities they imply, and the
  **resources expected to fulfil them**, with an **enforced composition**: a **required** set, a
  **forbidden** set, and optionally **mutually-exclusive** roles (two that may never travel together).
  Prescriptive; it answers "what is a principal of this name supposed to hold, and never hold." Drift
  from the pinned composition is an integrity event the group flags.

**Delegation** is the act of granting a role or passing on a capability. Two normative properties:

- **Always attenuating and bounded.** A principal **MUST** be able to delegate only a subset of what it
  holds, never a superset (`Realizes: P-Peer-Equality`; the attenuation requirement, §7.2 R2; this is
  also Meadowcap's confinement property for capabilities). Delegation **moves or narrows** authority; it
  never widens or mints it.

- **Two targets, one primitive.** A principal **MAY** delegate to (a) another client in its **own
  principal's device group** (trust stays within personal control) or (b) another principal in the
  **shared scope** (a cooperative anchor, another member's always-on node). Same mechanism; the trust
  boundary is the user's choice. This is what lets Drystone refuse consolidation onto a single keeper.

All of these live in the **grant planes** and **none alters weight or the rights floor.** A principal
carrying any role or capability still holds its complete inherent floor and its single unit of weight. The
mechanical check: every PeerSet **MUST** be definable as `floor + [explicit role set] + [implied
capabilities] + [expected resources]`; a name meaning "entitled to fewer **rights**" is **forbidden**;
that would be a smuggled rights distinction. **Rights have no presets; roles, capabilities, and PeerSets
do.**

A PeerSet's pinning is enforced by a **drift check**: the group gathers the role grants in force for a
principal from the governance log and compares them against the declared PeerSet's required / forbidden /
mutually-exclusive composition. Mismatch in **any** direction is the alarm, a principal that *acquired* a
forbidden role (dangerous), *lost* a required one (failing the job relied on), or *combined* two
mutually-exclusive roles (a restricted combination). The check is mechanical, because every side is a fact
already in the log. (Separately, "this node is blind" is trustworthy for a structural reason, not a pinned
role set: a Tier-0 store node can prove it holds zero payload keys, §8.)

> **Trust-dynamics changes fail loud, by design.** Two cases route to a noisy hard-stop rather than a
> silent rejection. The first is a governance action placing two mutually-exclusive roles on one principal.
> The second is a reconfiguration that would make a scope's blind store-and-forward node (a meer, §5.4)
> receive decryption keys, converting blind infrastructure into a reading party. In either case the action
> surfaces to the affected scope ("a restricted change was attempted; your communications may be at risk,
> fork without the principals who voted it?") and routes to human adjudication (§7.6). The reasoning: a
> quorum that votes to de-blind a node everyone relied on as blind has *changed the trust dynamics of the
> group*, which is precisely the kind of standing contradiction that is a utility judgment, not a
> computation. *design, decided; the loud-failure rung ties to §7.6.*

Worked example, a **moderator** (a PeerSet):

```
moderator (a PeerSet) ::= floor                               // full inherent rights, unchanged
                        + requires role { admit, remove }     // governance authority granted by consent
                        + expects  resource { reachability }  // device facts that help fulfil it
                        + forbids  role { act-for-the-group }  // kept separate from group-signing authority
                        + holds    capability { }              // no standing data-access grant by default
```

A moderator is a **peer** holding a moderation role: its rights floor and unit of weight are unchanged by
the grant, and revoking the role returns it to a bare peer. The PeerSet pinning makes drift an integrity
event: acquiring the forbidden `act-for-the-group` role, or losing a required one, is flagged. Delete the
PeerSet name and nothing about any peer's rights changes. *design (PeerSet drift-check and mutual-exclusion
formalism).*

A **meer** is *not* a PeerSet, because it is not a principal: it is blind store-and-forward infrastructure
configured by the scope (§5.4), with no role to pin and no rights to bundle.

### 5.6. Weight: flat by default, conserved under delegation, anchored to personhood

**Weight** is the second of the two equalities (§5.0): how much a peer counts when the group decides
something. It attaches to the **peer**, and its default is **flat, one per distinct peer**,
regardless of how many clients, devices, resources, capabilities, or roles that peer holds. It is equal
**by necessity, not by separate decree**; it follows from the equal rights floor (§5.3): equal
standing-to-participate is the same fact as equal standing-to-be-counted. Resources and roles are the two
legitimate inequalities (§5.0); rights and weight are the two equalities, and weight is the governance
image of the right.

**The default model is one-peer-one-vote.** Other governance models are explicit variations layered on top
of the flat default, not separate primitives (the multi-model intent from the open-thread review,
governance at scale):

- **Liquid delegation.** A peer **MAY** delegate the *exercise* of its weight to a delegate-principal,
  revocably. The delegate then exercises several peers' weight, but every unit still traces to a distinct
  peer (by lineage).

- **Elected admins.** A scope **MAY** vest decision roles in a small elected set (closer to forum
  moderation), with peers retaining equal weight to elect, recall, and ultimately fork.

- **Broadcast-only.** A scope **MAY** define a rights model where most principals receive rather than
  decide; weight is near-vestigial in such a scope, but the floor (voice, exit) is retained.

**The conservation invariant, which holds across every model:** weight is **allocated one-per-recognized-peer
at the source and is never minted, only moved.** A delegate exercising five peers' weight still reduces to
five distinct peers the group recognizes. The total weight in a scope equals the count of its recognized
peers, no matter how delegated, pooled, or elected. **Delegation moves weight; it never creates it.** This
is the anti-capture property, and it is *stronger* and *more honest* than "everyone votes equally" (some
scopes won't): the claim is not equal exercise, it is **non-inflatable total over the peers the group
recognizes.**

> **What the protocol guarantees vs what the group judges, and why this split is the honest one.** The
> anti-capture property has two parts that live at two layers, and conflating them (as an earlier draft
> did, by asserting "personhood is unforgeable") is exactly the provenance/utility collapse the spec
> exists to prevent (§2.0).
>
> 1. *Protocol guarantee (provenance, technical, airtight):* messages from a key-lineage are provably from
>    that lineage; governance weight is **flat per recognized peer and conserved under delegation**, never
>    minted by adding clients or resources. Adding devices adds resources, never weight. This Drystone
>    guarantees by mechanism.
>
> 2. *Group judgment (personhood, social, contextual):* whether a recognized peer corresponds to a
>    distinct person is a **utility judgment the group makes at its own confidence**, on the same
>    trust-to-do gradient as every other delegation. The protocol does **not** attempt to guarantee
>    one-lineage-one-human, **by design**, because the binding between a key and a breathing person is
>    precisely the kind of truth the system structurally cannot and should not certify (§2.0), and because
>    identity-presentation variety is part of the social substrate worth preserving (below).
>
> So the load-bearing claim is **not** "you cannot forge personhood." It is: *given the group's recognition
> of who its peers are, weight is flat and uninflatable by resources.* The equality holds over the peers
> the group recognizes, and the recognition is the group's own.

> **Sybil resistance is contextual, not global, stated honestly rather than overclaimed.** In a
> low-binding context (an open broadcast scope), one human may hold many peer-lineages, and that is an
> **accepted property of that context**, not a bug. Sybil resistance is supplied by the group's chosen
> personhood-confidence mechanism, not by the protocol, and it ranges across the trust gradient:
>
> - **High**, a family scope where a partner scans a QR code to join: the binding of social identity to
>   key provenance is high-confidence, so flat-per-peer is flat-per-person in practice.
>
> - **Medium, and anonymous**, an activist or privacy-sensitive scope that delegates the personhood check
>   to a verifiable-credential service which enforces "one personhood per government ID" *without ever
>   revealing the ID*: one-peer-per-person **and** real-world anonymity, simultaneously. Provenance is
>   guaranteed (these messages came from one root key chain); real-world identity is never disclosed.
>
> - **Low**, an open broadcast scope where binding is loose and Sybil resistance is weak, accepted as the
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
> presentation** impossible: the same person as a parent in one scope, a pseudonymous activist in another,
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
> irrefutable cases.** The peer/personhood split restates a pattern every deployed cryptographic-trust
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
> is itself social (TLS CA). Drystone's peer/personhood split is that universal pattern, stated honestly,
> with the binding made an explicit **per-group dial** instead of a hidden default. *(PGP web-of-trust and
> the GnuPG TOFU / trust-level descriptions verified against GnuPG documentation and the proof-of-personhood
> literature. **[confirm before publish, TLS/X.509 CA wording against RFC 5280, and the SSH
> known-hosts/TOFU wording against the OpenSSH man pages.]**)*

> **What remains genuinely open** (carried, not resolved): the **mechanisms a group may plug in for its
> personhood-confidence dial**, QR-scan binding, verifiable-credential services, social-graph,
> web-of-trust, are application-layer choices Drystone enables but does not mandate or fully specify, and
> the wire shape of delegating a personhood check to a (forkable, valuation-edge) credential service is
> `ENABLING`. This is **not** a threat-model hole the protocol must plug; it is a contextual judgment the
> protocol deliberately leaves to groups. The AS (§10.2) is the architectural seam where a binding is
> checked; its *content* is the group's contextual call. (Appendix B.)

### 5.7. Membership, standing, and revocation authority

**Standing** is decided from recorded, signed data, never the actor's own assertion. A message is
authorized iff its author held standing on a branch sharing the relevant lineage root. *green-real.*

**Revocation** removes a client/principal from the accepted set *going forward*. Survivors **MUST** reject
the revoked party's subsequent branches and **MUST NOT** claw back history contributed before removal
(standing ≠ membership; history is not erased). *green-real.*

**Revocation/add authority is a threshold dial** (k-of-n, **counted by distinct peer (by lineage),
never by client**): default 1-of-any, up to k-of-any or role-restricted admins. A membership op is
authorized iff it carries signatures meeting the scope's **current, replicated** policy; policy lives in
versioned scope state and is itself changed by governance ops under the current policy. The canonical form
is a **co-signed op**, a self-certifying k-of-n bundle validated locally against the current epoch,
freshness-gated (§7.4); proposal-plus-votes is an optional deliberative mode. *green-real (real k-of-n
bundle verified over live transport: an authorized 2-of-≥2 revoke accepted, an under-threshold revoke
rejected).*

> **Threshold counts peers, not clients, the same spine as §5.2/§5.6.** A k-of-n membership threshold is
> evaluated over **distinct peers (by lineage)**, so a single peer's multiple clients cannot
> together satisfy a multi-signer threshold. This is the mechanical face of "more devices does not grant
> more rights": the lineage of each signing key is resolved to its principal before the count, and
> co-signatures from clients of the same principal count once.

**The admin floor is derived from policy, anti-brick only.** A threshold `k_op` **MUST** be ≤ eligible
signers by distinct peer (by lineage) at the epoch it is set (solo genesis ⇒ `k_op = 1`; a scope
**MAY** be born "create with 10, need 5"); raising above headcount self-bricks and is rejected. Once set,
the scope **MUST** retain `n ≥ k_op`; a membership op whose post-state breaches the floor is **structurally
invalid** (rejected by every verifier from replicated policy alone). `k` **MUST NOT** auto-track `n`
downward (a threshold-downgrade attack). The floor is **anti-brick only**: a legitimate quorum acting
within policy, including self-capture, is accepted; the recourse for an out-voted minority is the §7.6
re-formation fork, never a structural veto. **Capture ≠ brick.** *design, decided; tests specified,
partially run (see §7.3 capped-root note and Appendix B for the coverage Drystone claims).*

> **Capture ≠ brick is also Drystone's answer to the uncapped-root steelman.** Matrix, under adversarial
> review, prevents room-capture by granting the creator uncapped, permanent power (an apex). Drystone's
> design philosophy is the opposite and is the one consistent with `P-Peer-Equality`: a legitimate quorum
> **may** capture a scope (that is permitted), what is forbidden is **bricking** it (rendering it
> inoperable / unrecoverable), and the remedy against capture is **exit, the §7.6 fork**, not an apex
> that prevents capture in advance. So the comparison with Matrix is not only "can a capped root match
> their soundness" but "is exit-as-remedy-for-capture sound where apex-prevents-capture was their
> choice." See §7.3. **[confirm before publish, MSC4289 / Matrix creator-power.]**

**Roles are revocable delegations, never impositions.** Every granted role (admin, moderator, a
content-gating role) **MUST** be a revocable delegation under the same threshold
authority, **MUST** carry only scoped, enumerated, non-creeping authority, and **MUST NOT** be immutable,
forced, or held by structural right. A creator holds **no** structural superuser right: at creation they
receive a bootstrap admin role purely so a one-member scope can function, revocable like any other.
**Anti-entrenchment ladder:** any delegated role is revocable (1) routinely under the policy threshold,
(2) as an always-available backstop by unanimity of the non-holders (a ceiling on revocation difficulty,
a group may set an easier bar, never a harder one), and (3) ultimately by the §7.6 fork. **No grant may
make itself irrevocable.** *green-real (revocation mechanics); design (ladder, decided).*

### 5.8. Revocation reuses governance machinery; it protects the future, not the past

A role grant is a governance fact (§7); revoking it is a new fact that supersedes the grant, resolved by
the same total order and fold as any other governance conflict. For roles that confer read, revocation
**MUST** rotate the scope epoch so the revoked principal cannot read content authored after the revocation
folds in (identical to membership expulsion). Revoking a read-delegate is, formally, an expulsion-shaped
fact.

This inherits an honest limit: **revocation protects the future, not the past.** A principal that held a
read role while valid may have retained what it read; revocation stops future access, it cannot unmake what
was legitimately held. The plain form, which a non-expert can hold: *you can revoke a delegate's access to
new content at any time and it actually takes effect, but the delegate may keep copies of what it already
saw, true of everyone anyone has ever shared anything with.* Stating it plainly is more honest than
implying otherwise.

#### 5.8.1. Open item: gating against the read right

The availability and read/search-offload roles are clean additive permissions. **Gating** is different: it
acts on the distribution or visibility of content, which bears on other peers' ability to exercise their
read right, so the additive framing does not automatically dissolve the tension. The likely resolution, to
be *specified* rather than assumed: gating acts on distribution/visibility within the scope's own governed
rules, **not** on the underlying right to read what one legitimately holds; every gating action is itself a
governed, attributable governance fact; and a gated peer's right to its own local history (including what
it already holds) is untouched.

> `ENABLING:` The precise relationship between a gating action and the read right MUST be specified, what
> it can and cannot affect, how it relates to content already held locally, and how it is bounded by the
> scope's rules so it cannot become a backdoor for suppressing a right under the guise of a role. This is
> the one role that, specified carelessly, could re-introduce a rights distinction through the permission
> layer, and therefore the one most needing an explicit forbidden clause wherever it appears in a PeerSet.

A content-visible gating role also weakens the system's "cannot comply" property (compellability): a
principal that has seen content cannot un-see it on revocation. The default **MUST** therefore remain blind
and any such role **MUST** be strictly per-scope opt-in, disclosed, scoped to the least-invasive rung, and
accountable, and it is a policy/legal question, not only an engineering one (gates a real deployment).

### 5.9. Exitability: the backstop that makes flexibility real

> `Realizes: P-Durable-Enablement`

A delegated role is only meaningfully different from a captive structural dependency if the delegation can
be withdrawn and restructured **without loss of rights.** Therefore:

Any default delegation a principal or scope adopts **MUST** be revocable and restructurable down to the
rights floor (§5.3) at any time, with **no loss of rights** and **only graceful degradation of capacity**,
never loss of function or standing. Concretely, a scope that delegated the availability and search-offload
roles to some principal (including a cooperative anchor or single operator-principal) **MUST** be able to
move that delegation to a different principal, split it across several, pull it into its own device groups,
or drop it entirely, and in every case continue to function. What degrades is capacity (deep search may
slow or need a member's own device online); never rights or the ability to communicate and govern.

Material reversibility is normative, not formal: (a) a helper holds only **encrypted** state and the scope
holds the keys, so the scope **MUST** be able to re-host on or migrate to a different node (no data
hostage); (b) the scope **MUST** be able to stand up a different store-and-forward node and reconfigure to
use it in place of the incumbent (the node is named by scope configuration, not bound to a box); (c) the
§7.6 re-formation fork remains the adversarial backstop. *green-real, a meer's encrypted store was
exported, imported into a different replacement meer, and a member re-homed and converged identically;
losing a meer costs availability, never data.*

**The asymmetry of expressible range** (a checkable claim, not a quality judgment): a flexible model can
present as the rigid one, but the rigid one cannot present as the flexible one. Drystone can be configured
to behave like a single-keeper deployment; a server-shaped system cannot be configured to behave like the
exitable, per-role-delegated model, because its central dependency is structural rather than granted. The
design question this hands forward, and the thing to press the spec on, is whether the exit path is
genuinely lossless-in-rights *in the default case* and not only at the unused margins (§5.7 admin floor,
§7.4 freshness, the no-helper-path obligation of `P-Durable-Enablement`).

### 5.10. The group as a principal: communal ownership, composition, and acting on a group's behalf

> `Realizes: P-Peer-Equality, P-Local-Truth, P-Durable-Enablement`
>
> Cross-references: §5.2 (kinds of principal), §5.5 (roles and capabilities), §7.1 (data model), §7.6
> (fork), Part 1 §2.3 (recursive peer-is-a-group), Appendix C (Meadowcap communal/owned).

A **group is a principal** (§5.2): a collective that can hold a role, own artifacts, be granted to, and be
referred to, a composable unit, not only a key-agreement context. This subsection fixes how that works,
because it answers a question a single-layer model cannot: **when a group forks, who owns the shared
artifacts?**

**The group-principal lives above MLS, in the artifacts, not in the key layer.** The MLS group is the
**communication-and-safety substrate** (a set of clients sharing a key, with forward secrecy and
post-compromise security). The **group-principal** is an **application-layer identity**, a Meadowcap
**communal namespace** (§7.1); that *corresponds to* an MLS group for communication but is **not defined
by it**. They share a membership set; they are different objects at different layers. MLS answers "who
shares the key and can read"; the group-principal answers "who owns this, whom may this group grant to,
what does this group's authority cover." This separation is deliberate and is the only one consistent with
MLS's own design, which provides no notion of a group acting as a grantor of access (§10.2).

**Authority is communal, not apex, which is what lets it survive a fork.** Meadowcap offers two models:
**communal** namespaces, where authority comes from owning a subspace key and all members hold equal
authority with no one holding the whole, and **owned** namespaces, where a single keyholder has total
top-down control. Drystone's group-principal is **communal by default**, because communal authority *is*
`P-Peer-Equality` expressed at the data layer: horizontal, no apex, each member writing into their own
subspace. The **owned** model is the apex Drystone rejects for group governance, though it remains
legitimate for the narrow case of a single author owning a sub-namespace of content they alone created
(the boundary between governing-the-group and owning-your-own-data: group governance is communal, while a
single author's own sub-content may be owned, §5.3).

**Worked mechanism, the forked artifact.** Three peers collaborate on a document by automatic merge
(a convergent, monotonic data structure, §7.1). They disagree in a way that is a genuine standing
contradiction, and the scope forks (§7.6). Who owns the document?

- **Both layers own it, and that is why it survives.** The artifact lives in the group's **communal
  namespace**: the group-principal owns it *as a collective*, and each contributing peer owns its own
  **subspace** of contributions *as an individual*. Ownership was never solely at either layer.

- **At the fork, both descendant groups carry the whole artifact**: exactly as an open-source fork
  carries the full repository, not a fraction. Because communal authority was distributed across the
  members all along, the fork does not orphan the artifact (there was no center to sever) and does not
  shatter it into private fragments (the communal namespace is the shared object). Each fork continues
  with a complete copy and full standing to evolve it independently.

- This is the data-layer face of *fork-not-verdict* (§7.6): the system does not adjudicate who "keeps"
  the document. Both do. The fork is the dignified exit, and the artifact comes along on both sides.

**Composition, a group-principal can be a member of another group-principal.** Because a group is a
principal, the structure nests: a **user is a group of clients**, a **community is a group of users (peer
or group principals)**, a **federation is a group of communities**. Each layer is a communal namespace
with a referable identity, each ownable and grantable, each forkable with its artifacts intact. A
group-principal can therefore be granted a capability (§5.5), hold a role, or be referred to as a unit,
which is what makes "stand on a group as a composable identity" concrete rather than aspirational.

**Acting on a group's behalf is itself a governed role.** A group cannot sign; some principal must act for
it. The authority to **act-for-the-group**, to issue a capability, delegate, or make a grant in the
group's name, is a **role** (§5.5) granted by member consent, scoped, and **revocable** under the same
threshold authority as any other role (§5.7), with the §7.6 fork as the ultimate backstop. This is the
same anti-entrenchment discipline as admin, one layer up: no principal holds the act-for-the-group role by
structural right, and no grant of it may make itself irrevocable.

**The recursion bottoms out, and that is what keeps weight honest.** Composition could otherwise be a
laundering path for governance weight, a principal pooling many sub-groups to manufacture standing. It
cannot, because **weight (§5.6) is anchored at the leaves: flat, one per distinct peer, never
minted, only delegated.** However deep the composition, the total weight in any scope reduces to the count
of distinct peers (by lineage) at the bottom. A group-principal's weight in a parent scope is
*defined by its members' delegated weight*, conserved through every layer, never inflated by the act of
composing. So the group-as-principal model gives Drystone composable collective identity **without**
opening the door composition would otherwise open, because the personhood-anchored leaf is the floor of
the recursion.

> **Two seams left open here, stated rather than smoothed.** *(1)* The precise **identity construction for
> a group-principal**, the communal namespace key and how it is established and rotated as membership
> changes, is designed-not-frozen (the §5.2 open seam, now given a concrete shape: a communal namespace
> rather than a derived central credential). The motivation is concrete: when forking and merging are cheap
> but a group is collaborating on a shared asset, honoring a fork requires the asset to be owned jointly by
> the clients (and so the peers) *and* the group, so both forks carry the whole thing like an open-source
> repository fork. A Meadowcap **communal namespace** fits that model well, which is why the group-principal
> is shaped as one. What is unworked is the **key rotation scheme** (how the group and its members jointly
> own the namespace and how the key rotates under churn) and whether the communal namespace is **primary**
> (the group-principal *is* a communal namespace at all times) or **secondary** (established only at a fork
> or merge, when joint ownership has to be made explicit). The decisive next step is to **dig into Meadowcap
> and check its alignment with MLS**, whether group-associated assets can fork and merge sanely across the
> two layers, before committing the construction. This is a Drystone construction question, not a gap in
> Meadowcap: Meadowcap's communal-namespace semantics are confirmed (below). *(2)* **Cross-group** grants
> and references (one group-principal granting to or composing another across trust boundaries) are sketched
> here as the composition model but their wire encoding and the valuation-vs-composition edge (Part 1 §2.3)
> are `ENABLING`. Both carried to Appendix B.
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

## 6. Transport

> `Realizes: P-Local-Truth, P-Peer-Equality, P-Durable-Enablement`
>
> External transport facts (iroh / QUIC) cite the FACTCHECK SoT, iroh `1.0.0`, do not re-verify.
> The transport layer is the one place *peer-to-peer* is the precise word (§3.1, naming note).
>
> **Requirement vs realization:** this section specifies the iroh-based reference. The abstract transport
> requirement (T1–T7), the compliance bar a non-iroh candidate must meet, and the topology question
> (why point-to-point-with-blind-relay is the reference and why pure mesh is a permitted-but-not-required
> divergence) are consolidated in **§10.3**.

### 6.1. Connection establishment and routing

The transport is **iroh**: encrypted QUIC with relay fallback for NAT'd peers, routed by `EndpointId`
(public-key endpoint identity; renamed from `NodeId`, per FACTCHECK SoT). Scope membership maps to a
gossip **topic** (§4.2), carried over **iroh-gossip** (HyParView/Plumtree, per FACTCHECK SoT). A relay
forwards opaque frames and **MUST NOT** be required to read content; it routes by endpoint, not by topic.
*green-real, NAT path via relay; the meer (blind store-and-forward node) sees only ciphertext plus routing metadata.*

**Co-location** (reference deployment): two peers reach each other over relay fallback only if they share
a home relay (no relay-to-relay mesh); relay placement is server-published and authoritative, keyed on the
rendezvous/namespace, not on identity. A relay process **SHOULD** meter and isolate per tenant and
**MUST** degrade *visibly* under stress, never silently. *green-real (measured).*

### 6.2. Interaction tiers

Interaction tiers are chosen at scope creation, not toggled at runtime: **interactive** (prompt delivery +
real failure signal), **quiet-large** (eventual, "it will arrive or you will be told it did not"), and
**broadcast** (best-effort rolling log). The broadcast tier **MUST** disable the embedded group-key
ratchet tree (O(N) commits) and ship the tree out of band. *design (tiers) + green-real (the O(N)
ratchet-tree cost is measured).*

### 6.3. Real-time media

Real-time media (voice/video/stage) rides the **same iroh transport** as messaging but over **QUIC
datagrams** (unreliable, no retransmit) carried as RTP-over-QUIC. Media frames **MUST** use the datagram
flow (latency over reliability) and **MUST** be end-to-end encrypted via per-sender keys derived from the
group key epoch, so a forwarding helper stays blind. A group-scale call **SHOULD** use a **blind
forwarding helper** (header-only routing) rather than full mesh past a handful of peers; server-side
mixing that requires plaintext is **forbidden**. Media keys rotate on membership change exactly as messages
do.

Two media congestion-control rules are **normative**: (1) the media engine's bitrate estimator **MUST** be
authoritative and back off on the path-RTT trend (plus per-stream loss and jitter); it **MUST NOT** rely on
datagram-send back-pressure (the transport silently drops oldest, never errors) nor on receiver-side loss
alone (a delayed prefix shows none). (2) Real-time media and bulk reliable transfers **MUST** run on
separate flows/connections, or the bulk transfer starves the media. *green-real (both rules measured: a
delay-based estimator backs off 64→8 kbps in under a second; co-located bulk drove media RTT to seconds,
separate flows left the call untouched). The video engine and real-codec/RTP path are design.*

---

## 7. Synchronization and Governance-Conflict Resolution

> `Realizes: P-Knowable-Truth, P-Local-Truth, P-Peer-Equality`
>
> Reasoning complete; several wire encodings are `ENABLING` (Appendix B). The **Willow / Meadowcap** claims
> are confirmed against the spec; the comparative claims about **Matrix** (State Resolution, MSCs, CVEs)
> and **Keyhive** remain **[confirm before publish]**, web-verified in source dialogues and consolidated
> in Appendix C, not yet in the FACTCHECK SoT.

### 7.1. Data-model commitment

Governance facts live in a **namespace / subspace / path** structure addressed and reconciled by
**range-based set reconciliation** (a Willow-shaped data model; the three-dimensional namespace/subspace/path
model is confirmed against the Willow spec, Appendix C). Drystone implements this *shape* directly in early
phases rather than depending on a Willow implementation, so a later transition is a substitution, not a
redesign; Drystone is built Willow-*shaped*, not Willow-*dependent*.

### 7.2. The grant-and-revocation interface (mechanism-neutral, normative)

> **Scope (carried from §5.5):** this interface governs both kinds of grant that sit above the rights
> floor, **roles** (in-group governance authority) and **capabilities** (Meadowcap data-access grants).
> Both are unforgeable, attenuating, revocable governance facts, so they share one interface. "Capability"
> here is Meadowcap's data-access sense, kept verbatim; the requirements are the standard object-capability
> guarantees.

Whatever role/capability mechanism Drystone adopts **MUST** provide:

- **R1, Unforgeable grant.** A role or capability cannot be fabricated by anyone not entitled to issue it.

- **R2, Attenuating delegation.** A holder may delegate a subset of held authority, never a superset
  (`Realizes: P-Peer-Equality`; this is Meadowcap's confinement property for capabilities).

- **R3, Convergent revocation expression.** A revocation **MUST** be expressible as a governance fact
  (§7.3) that folds deterministically, so all synced honest peers agree the grant is void.

- **R4, Bounded stale-authority exposure.** For a holder that refuses to sync a revocation, the protocol
  **MUST** bound the window in which third parties accept the revoked grant, a finite, stated bound
  (epoch boundary or membership-graph generation; **not** a wall-clock interval, per §2.0.1, a bound
  expressed in time would rest on the same uncorroborable clock).

- **R5, Forward read exclusion.** After expulsion, the member **MUST NOT** read entries authored after
  the expulsion folds in (past entries out of scope, §7.5).

- **R6, Attributable acceptance.** A peer accepting a write under a capability **MUST** record the causal
  frontier of governance facts it had synced at acceptance, so a later-synced revocation makes the stale
  acceptance **detectable and attributable** rather than silent (§7.5).

R3 and R6 are what defeat a silent state-reset failure mode and hold regardless of the mechanism. The
mechanism itself (Track A vs Track B) is **deferred** to the richer-access-control phase; see Appendix A.
No normative text here assumes a track. *design.*

### 7.3. Governance facts are entries, not mutations

A governance decision (admit / expel / grant / revoke / amend) is a **signed, append-only entry**. Entries
are never modified or deleted; a reversal is a new entry referencing the one it reverses. There is no
"current state" to reset, only a monotonically growing fact set and a deterministic **left fold** from it
to an effective authority state. A peer that has seen fewer facts computes a **stale** authority state,
never a wrong one, and never one another peer could weaponize by replaying old entries (`Realizes:
P-Knowable-Truth`). *design, the append-only-fold property is the load-bearing invariant the rest of §7
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
> one claim. **[confirm before publish, CVE-2025-49090 root cause against the MSC4297 primary text; the
> blog/implementer-guide summary is the current source, not the MSC itself.]**
>
> *Steelman against.* Under the same 2025 review, Matrix concluded that sound decentralized resolution
> requires an **uncapped root**, room creators with permanent "infinite" power (MSC4289), reasoning
> that an attacker who can backdate events already wields de facto apex control, so the apex must be made
> explicit to be bounded. Drystone's capped, delegable, revocable-by-succession root (below) contradicts
> that conclusion. Drystone's wager is that the conclusion was **forced by their inputs, not by the
> problem**: their order consumes a wall-clock, so backdating manufactures authority, so they pin
> authority to an apex; Drystone removes the wall-clock from the order (§7.3.1), so that specific attack
> has no purchase. **This is an argument, not yet a proof, and the gap is named precisely in Appendix B.**
> **[confirm before publish, MSC4289.]**

**The unconflictable root.** Each scope has a founding fact establishing initial authority. Its authority
over the genesis of the scope is **not** subject to the §7.3.1 ordering; it is the base case of the
authority-rank computation, not a competitor within it, because a conflict at the root is the one ambiguity
an attacker could convert into total capture. Drystone's root authority is **capped, delegable, and
revocable-by-succession** (`Realizes: P-Peer-Equality`), not infinite. The root forgery vector itself is
closed structurally: the scope id is `H(tag ‖ group_id)` (§4.2) and the founding fact is the fold's
unconflictable base, so there is no second create event to smuggle in (the structural analogue of the fix
Matrix shipped as MSC4291, room-id-as-hash-of-create-event). *design.* **[confirm before publish,
MSC4291.]**

> `ENABLING:` Root-authority succession (how founding authority transfers when founders leave) is the most
> dangerous operation in the system and is deferred to a Lifecycle section (Appendix B). A permanently
> fixed root contradicts the cooperative model. The capped-vs-uncapped-root soundness question (above and
> Appendix B) must be resolved before this is frozen.

#### 7.3.1. The total order over conflicting facts: causal and cryptographic only, never temporal

When two facts conflict (§7.3.2), every honest peer **MUST** agree which takes effect. Drystone orders
conflicting facts by this key, compared lexicographically, lowest wins:

1. **Issuer authority rank at the causal frontier** (computed structurally, not by consulting the
   contested outcome, §7.5).

2. **Governance precedence class**, authority-reducing facts (expulsion, revocation) outrank
   authority-expanding facts of equal issuer rank (a conservative default: when valid decisions collide,
   the one reducing exposure wins).

3. **Causal length**, a more-informed decision (longer referenced causal history) outranks a less-informed
   one.

4. **Content-address tiebreak**, the digest of the canonical fact encoding, as the final deterministic
   discriminator.

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
*design.*

> `ENABLING:` The canonical byte encoding fed to the content-address hash, and the wire format of a
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
> uncorroborable per §2.0.1. Note carefully: Matrix's 2025 state-reset CVE was rooted in
> starting-state/replay-scope, **not** in the timestamp tiebreak, so this contrast is about ordering-spine
> design, and the CVE is cited in §7.3 for the *separate* monotonic-fold point. *(State-Resolution-v2
> tiebreak fields and the authors' "trusting servers not to lie about the time" admission confirmed against
> the Matrix.org stateres-v2 description and the MSC1442 discussion. See Appendix A and Appendix C.)*

#### 7.3.2. What conflicts

Two governance facts conflict when applying both, in either order, yields a different effective authority
state than applying them in the order §7.3.1 selects, concretely, two facts targeting the same subject
where at least one removes or narrows authority the other depends on. The **mutual-expulsion** case (A
expels B while B expels A, equal standing) resolves by steps 3–4; exactly one survives, never both and
never neither, and the loser's fact remains in the log as a valid-but-superseded entry, visible for audit.

> **Boundary with the §7.6 hard-stop.** §7.3.2 resolves conflicts where the ordering key yields a
> determinate, non-arbitrary winner. The mutual-expulsion case above is resolvable *only because* a
> content-address tiebreak is non-arbitrary-by-construction (deterministic on every peer); it is a
> provenance tiebreak, not a utility judgment about who *should* win. Where a contradiction is a genuine
> membership contradiction that the fold cannot determinately resolve without manufacturing a utility
> verdict (Part 1 §2.5), §7.6 applies instead: the protocol hard-stops and escalates. The line between
> "deterministically tiebroken" and "must escalate" is itself partly a per-scope tolerance, see §7.6.

#### 7.3.3. The declarative snapshot is a cache; truncation is verifiable

The governance log is the **imperative** source of truth; "current state" (membership, roles, the rules
in force) is a **declarative snapshot**, a deterministic fold of the log carrying the governance head it
was computed from. The snapshot **MUST** be treated as a cache: it is **never authoritative, never
independently writable, never synced as truth, never trusted from a peer**, and it is **valid only while
its recorded head equals the group's current governance head** (otherwise re-fold the tail). It is not
"latest values" but "latest values that passed authorization at each step", the log is **self-validating
under replay**, since each fact is admitted only if authorized under the rules in force at its position.
Peers reconcile by exchanging the **imperative log** and each deriving the snapshot independently;
agreement is verified by reaching the same state from the same head, and disagreement is explicit, there
is **no point at which a peer accepts another's declared state without local validation.**

To bound replay cost without breaking that discipline, the log **MAY** be truncated by a **roll-up**: a
signed checkpoint committing to `(governance_head_hash, state_commitment)`. Because the head is hash-linked,
committing to it transitively commits to the whole prefix, so a roll-up is a **re-expandable, back-verifiable
truncation**, not a trusted summary. The sound posture (and the one that needs no quorum to stay live):
**each peer independently folds and self-checkpoints**; where roll-ups are co-signed, a co-signature is
*corroboration of an independent identical fold*, never a substitute for local validation. Compaction is set
at genesis in **two tiers**, the **governance spine is permanent and uncompacted** (it is exactly what a
returning/dormant node needs to reconstruct the authorized signer set and validate everything else), while
**content is compactable** into head-committed, Merkle-rooted checkpoints. Roll-up is **built-in but off by
default** and catch-up never *depends* on it. *(design; the byte-level checkpoint encoding is `ENABLING`,
Appendix B. Local snapshot/rollback mechanics, e.g. savepoint cadence, are an implementation detail.)*

### 7.4. Freshness: no false "current"

A peer/helper **SHOULD** periodically emit a signed, **content-free** tip beacon
`{scope_id, epoch, head, seq_high, sig}` (head/epoch/routing only, safe for a meer). A peer
**MUST** track time-since-last-heard *locally* (liveness is a local measurement, never trust in another
peer's wall-clock) and **MUST NOT** display a view as "current" unless it is both caught up to the
best-seen tip **and** has heard a beacon within the tier's freshness horizon; otherwise the view **MUST**
surface as "behind" or "unverified." Silence **MUST NOT** be rendered as currency. *green-model.*

> **What freshness can and cannot establish, and why this is a §2.0.1 consequence.** No node can know
> what is *most current* in a center-free design, "most current" presumes a global vantage no node has,
> and it would have to rest on the very wall-clock §2.0.1 rules out. But a node *can* establish two things
> that **are** corroborable provenance: **liveness over a window** (which peers emitted beacons recently,
> measured by the node's own local elapsed time as a private input, never as a shared clock) and **causal
> independence** (whether two diverging facts are concurrent or one references the other). The protocol
> therefore never asserts "this is the latest"; it asserts "I have heard from these peers within my own
> measured window, and these facts stand in this causal relationship"; both of which a peer can defend.
> Currency is not provenance; liveness-over-a-window and causal-independence are.

**Membership/governance acts require strict CURRENT + corroboration.** To originate or co-sign an
add/remove/policy-change, a peer **MUST** be (a) caught up and (b) corroborated-fresh, agreement on the
same head from ≥k distinct lineages observed stable, and after any unverified lapse re-checked at signing.
Ordinary content has no such precondition (it MAY be authored from a behind/unverified view, honestly
labeled). This **narrows, does not close**, the fresh-but-wrong-partition window; the residual is the §7.6
hard-stop's, by design. *design, decided; tests specified, not yet run.*

#### 7.4.1. The false-positive tolerance is a governed utility judgment, not a constant

A genuine concurrent contradiction (a real social dispute the fold cannot resolve, §7.6) and a benign
**sync artifact** (two peers momentarily diverged because neither had yet seen the other's facts) can look
identical at the moment of detection, §7.5.1 notes backdating cannot be defeated by cryptography alone,
and the same ambiguity applies to honest concurrency. If every concurrent divergence escalated, the §7.6
human channel would drown in false alarms and lose the trust the entire algedonic posture depends on
(Part 1 §3, Beer). If escalation were too lax, a real contradiction would be silently auto-reconciled,
which would re-open the manufactured-resolution surface §7.3.1 closes.

The resolution follows the razor (Part 1 §2.0, §2.5). The **machine computes the provenance signals**,
concurrency vs causal-dependence, liveness-over-window (§7.4), and the magnitude/shape of frontier
divergence, all corroborable. **Whether a given concurrent contradiction is treated as benign-and-safely-
auto-reconcilable versus escalate-to-humans is a per-scope governed tolerance over those signals, not a
hardcoded constant**, because the benign-vs-deception distinction is ultimately a utility judgment and is
vulnerable to alarm-fatigue, so it must be tunable to the scope's threat model, temperament, and need.

Two normative guardrails keep the tolerance honest:

- The tolerance governs only **when to escalate versus when to auto-reconcile a *provably-benign*
  concurrent case**. The boundary of "provably benign" **MUST** stay cryptographic/causal, never
  heuristic, a tolerance **MUST NOT** auto-resolve a case it cannot prove benign, because a too-loose
  tolerance is an attacker's instrument.

- Where a case is genuinely ambiguous (cannot be proven benign and cannot be proven a real contradiction),
  the safe default **MUST** be to **escalate** (§7.6): a false alarm costs attention; a silent false
  resolution costs the integrity the protocol exists to protect.

The setting itself is a threat-model judgment; this specification states the axes (fatigue-risk vs
silent-false-resolution-risk) and **declines to pick the default value**; that is a scope's call, and a
per-scope governed policy fact like any other. What the spec leaves to implementation is twofold: the
**granularity of the knobs** exposed and the **shipped defaults**; both are tuning decisions to settle
against a real deployment, not protocol constants. *design.*

### 7.5. Attributable acceptance and the regress-free fold

#### 7.5.1. Attributable acceptance (R6)

The guarantee is **detection and attribution, never prevention.** The acceptance record is a governance
fact signed by the accepting peer, whose signed body includes (1) the accepted entry's content digest
(binding *what* was accepted, via the authorized-write hook), (2) a **frontier commitment**, a commitment
over the set of governance-fact digests the peer claims as its synced frontier, signed as part of the
acceptance body (signing over prior signed state rather than a mutable timestamp), and (3) the peer's own
prior acceptance-record digest, chaining its acceptances.

Against the attack of lying about one's knowledge state: **frontier omission** is defeated cryptographically
(the commitment pins the set; the omitted revocation is provably in or out), and **equivocation** is
defeated by the per-peer acceptance chain (two signed chain heads with the same predecessor are
non-repudiable proof). **Backdating** cannot be defeated by cryptography alone, there is no trustworthy
internal clock (§2.0.1, again: a node's own clock is not corroborable), so the bound is **causal**: if the
revocation is in the causal history of any fact the peer's frontier includes, the "didn't have it" claim is
refuted; the only residual is a peer genuinely causally independent of the revocation, which is the
legitimate concurrent-partition case R4 exists to bound (by epoch/generation, not by time). Every stale
acceptance therefore resolves into exactly one of two categories, **knowingly stale** (full attribution)
or **concurrently stale** (no fault, R4-bounded), with no third category where a peer silently escapes both
prevention and attribution. *design.*

> `ENABLING:` The frontier-commitment construction (a Merkle root over sorted governance-fact digests), the
> acceptance-record wire format, and the per-peer chain linkage must be byte-specified before interoperation.

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
in an unconflictable base case; a lagging peer under-authorizes rather than diverging). *design.*

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

> `ENABLING:` **Frontier-closure-and-subgraph-closure before sorting** is the **single most likely place
> for two implementations to disagree** and therefore the place the spec must be most exact. The
> conflicted-subgraph-closure rule is **adopted from Matrix State Resolution v2.1 (MSC4297)**, whose
> "conflicted state subgraph" mechanism added exactly this closure to fix CVE-2025-49090; the
> strongly-connected-component characterization and the forward-backward-intersection computation are
> theirs (Matrix's Change 1, begin from the empty/unconflictable base, Drystone already satisfies via
> the §7.3 founding-fact base case). **Adopting their closure does not adopt their ordering**: Drystone
> keeps its own content-address tiebreak and rejects power-in-the-comparator and the wall-clock tiebreak
> (§7.3.1). The two are separable, the closure is a convergence prerequisite independent of how ties
> break. **[confirm before publish, MSC4297 conflicted-subgraph mechanism against the primary text.]**

> **A failure mode specific to Drystone's monotonic fold.** If a peer omits an in-between fact (incomplete
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
minority is a clean, attributable **re-formation fork**: a differently-shaped scope that preserves history
and provenance to the point of departure and legitimizes/erases nothing retroactively. Stripping a helper
operated by a cooperative or external operator simply **detaches** the scope into a differently-shaped one
(unpreventable anyway, the operator can always leave); the protocol only preserves history and provenance
to the detachment. *green-real (contradiction hard-stop; identical reformed genesis across independent
peers).*

This hard-stop is Drystone's **algedonic channel** (Part 1 §3), and it is the realization of the **forced
terminus** of Part 1 §2.5. Rather than letting an automatic rule resolve a case it cannot safely resolve,
the protocol raises the hard case to the humans who hold the context, the formal version of "specify only
somewhat, then escalate the residue." It is a *designed* channel, not a failure path: the protocol commits
in advance that genuine contradiction is a human-adjudicated event, because no merge rule can be trusted to
absorb it silently. Crucially, the escalation keeps **both the signal and the authority local**; it
surfaces the conflict to the affected scope rather than relocating the decision to a center that lacks the
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
them lets an attacker capture a scope or silently revert a decision.

**The wall-clock as an attack surface (cross-reference).** Because §2.0.1 establishes that a timestamp is
an uncorroborable assertion, any place a wall-clock entered an authority-bearing computation would be a
social-engineering surface exploitable **even by an authorized, membership-gated member**, honestly (clock
skew) or maliciously (deliberate skew). The protocol's response is categorical: the wall-clock appears in
no ordering, identity, authority, or bound (§4.2, §4.5.1, §7.2 R4, §7.3.1). Liveness is measured locally as
a private input only (§7.4). This closes the surface by construction rather than by detection.

**Visibility and the social layer.** A scope's regime and visibility class are **born in at genesis and
immutable** (part of the signed genesis); there is **no silent regime crossing**, a republish is a
distinct authored act carrying a reference plus author-chosen content, never the original. Outward
propagation depth is enforced by every verifier. An implementation **MUST NOT** offer a structure-only
share (topology revealed, identities withheld), graph topology is re-identifying; the only safe share is
consented-distance/resolution-scoped. *green-model (visibility regimes; the structure-only share is shown
unrepresentable, a modelled target's connection shape has anonymity set 1).*

**Metadata is the residual surface.** A meer still observes join-metadata (digest, length,
namespace) and connection attempts; this surface **MUST** be surfaced, not hidden, and a
Tier-0 helper **MUST** hold no payload key and **MUST be able to prove it** (assert-and-log
`payload_keys_held = 0`). *green-real (Tier-0 meer proves zero payload keys; admission denies a non-listed
peer).* (Note: earlier drafts listed "timestamp" among observed join-metadata; a relay observes *arrival
order at the relay*, which is the relay's own local observation, not an authored timestamp, the distinction
matters per §2.0.1 and the metadata accounting should say "arrival ordering as locally observed," not
"timestamp.")

**Failed-operation response.** Detection of an invalid op is deterministic; the *response* is a governance
dial, **loud** (signed, corroborated rejection → group immune memory), **silent** (reject, no signal), or
**blackhole** (tarpit). A serious auto-response **SHOULD** require k-observer corroboration. Note "silent"
is application-layer: the relay still observes the connection attempt. *design.*

**Label, not enforce, a peerhood-preserving primitive.** Where content moderation or social adjudication
is involved, the protocol's posture is to **label** (attach advisory, attributable metadata) and leave the
*action* to scope governance or each peer's own client, rather than to **enforce** (act unilaterally and
irreversibly on the network's behalf). This is not only a safety choice; it is what keeps the system *made
of peers* (§3.1). Enforcement relocates adjudication to whoever enforces, quietly converting peers into
sensors by stripping their decision rights, whereas labeling leaves adjudication with the peer and
propagates only information. It is the same algedonic move as the §7.6 hard-stop and the same razor as Part
1 §2.5: **surface the signal, don't seize the decision.** Each enforcement hook tends to look locally
reasonable, which is exactly how a peer network can degrade into a centrally-adjudicated sensor mesh over
time; the label-not-enforce default is the precommitment against that drift.

### 8.1. Honesty boundaries this specification still carries

Stated plainly so the spec does not over-claim: (a) freshness (§7.4) is proven in the model, not yet over
live transport; (b) the failed-op leak/immune dial is design-only; (c) a content-visible gating role's
**compellability** tradeoff is an unresolved policy/legal question, not an engineering one (gates any such
deployment); (d) the video media engine and real-codec/RTP path are design; (e) the membership-op
freshness threshold and admin-floor rule are decided-but-not-yet-test-run; (f) the false-positive
escalation tolerance (§7.4.1) is design-only and its *value* is deliberately left to scope policy; (g) the
capped-root soundness claim against the Matrix uncapped-root steelman (§7.3, Appendix B) is argued, not
proven, and the coverage of what has actually been tested must be surfaced before it hardens.

---

## 9. Interoperability and Conformance

> `Realizes: P-Knowable-Truth, P-Peer-Equality`

Two implementations are **Drystone-compatible** when they agree on every normative section that forces
agreement: the identifier derivations (§4.2), the signed message pre-image and verification (§4.3), the
integrity-vs-authority separation (§4.4), the lineage fold (§4.5) and lineage-counted thresholds (§5.7),
the rights floor and the `floor + roles + capabilities + resources` decomposition (§5), the governance-fact
append-only fold and total order (§7.3), and, once its `ENABLING` encodings are pinned, the
frontier-closure-and-
subgraph-closure rule (§7.5.2), which is where two implementations are most likely to diverge. **Peer
equality is shown to be enforced by mechanism, not convention, exactly here:** a conformant implementation
cannot grant a peer a rights difference, because §5 makes every configuration `floor + roles +
capabilities + resources` and rejects anything that decomposes to fewer rights than the floor.

A conformant implementation **MUST** pass the conformance vectors and must-reject cases. The reference
conformance suite is **built and passing (66/0)**, derived by running the real implementation: derivations
(incl. the tagged wire forms), signed pre-images, the fold and lineage-counted thresholds, revocation
mechanics and k-of-n revoke-authority, the reconcile corpus, the adversarial cases, and the visibility and
freshness vectors. *green-real (suite), note the suite covers the §4/§5/§6 proven layer; the §7.3–§7.5
governance-resolution vectors depend on the `ENABLING` encodings in Appendix B and are not yet in the
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

The earlier mechanism-neutral text stands and is cross-referenced: the capability interface (§7.2 R1–R6)
and the Willow-shaped data-model commitment (§7.1) already separate requirement from realization, and
§10.3 and §10.4 simply consolidate and complete that treatment. Where §10 and an in-place section appear
to overlap, the in-place section is normative for its mechanism and §10 is the consolidated requirement
view.

A note that frames the whole section, and answers a question the design keeps raising. Drystone is a
**local-first, center-free** system: there is **no shared global state by design** (`P-Local-Truth`). That
is not suitable for every application, anything requiring a single authoritative global ordering (a
central ledger, a globally-serialized auction close) is the wrong fit and should use a coordinator. But
the application classes it *is* suited to, group messaging, collaborative state, membership and
governance, anything where local-canonical-plus-reconciliation is the honest model, are significant, and
the fit there is not a weakness of the cryptographic-systems lineage but its point. The recurring
observation that "you still need a human-adjudication layer" (§7.6) is not a defect of this class either:
**every convergent system, of any architecture, is ultimately managed by humans**, the centralized ones
simply hide the humans behind the operator. Drystone makes the human layer explicit and keeps it at the
edge rather than at a center. So the substrate requirements below are chosen to serve that class well, not
to chase a universal applicability the design deliberately declines.

### 10.1. How to read this section

Each component below is given as: **(R)** the abstract requirement Drystone places on the component; **a
compliance table** of what any candidate must satisfy; **the disqualifiers** that would make a candidate
non-compliant; and **the reference realization** with a note on *why it currently wins*. A candidate that
meets the table and avoids the disqualifiers is **substrate-compliant** even if it is not the reference.

### 10.2. Messaging backplane: group key agreement and the secure-messaging properties

> Cross-references: §4.5.1 (per-client authorship, client-as-member), §5.7 (revocation rotates epoch),
> §6.3 (media keys from the group epoch), Appendix A.1 (decentralized-MLS forward-secrecy cost),
> Appendix C.3.

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

- Bakes in a **wall-clock** dependency for epoch validity or ordering (violates §2.0.1).

**Reference realization: MLS (RFC 9420), with the architecture of RFC 9750, and exactly why it wins, and
exactly where Drystone diverges from its assumptions.**

MLS is the reference because it is the only standardized, widely-reviewed mechanism that satisfies K1–K6
and K8 directly: it provides FS and PCS for groups from two to thousands, via a **ratchet tree** that
makes member replacement `log(N)`, and it computes a deterministic per-epoch group state (the tree hash)
that members check. *green-real at the data/transport layers it underpins; the FS/PCS and group-state
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
could replace MLS without changing any Drystone requirement. **[confirm before publish, the DS/AS trust
model and deployment-status claims against RFC 9420/9750 and the decentralized-MLS drafts.]**

### 10.3. Transport and overlay: point-to-point reachability, and the topology question

> Cross-references: §6 (transport), §6.3 (media datagrams), §3.1 (where adjudication lives), §8 (the
> relay is a blind forwarder), Appendix C.

**(R) Requirement.** Drystone requires a **transport** that lets two peers, identified by **public key**
(not by IP or location), establish a **mutually-authenticated, end-to-end-encrypted** channel, with a
**fallback path** when a direct connection is impossible (NAT/firewall), where **any intermediary is a
blind forwarder** that cannot read content and routes by endpoint, not by topic. It additionally requires
an **overlay** by which scope members find and exchange with each other, and an **unreliable datagram**
mode for real-time media (§6.3).

**Compliance table, a candidate transport/overlay MUST:**

| # | Requirement | Why it is required |
|---|---|---|
| T1 | Identify peers by public key; the key is the channel's authenticated identity (no impersonation) | identity is cryptographic, not positional (`P-Knowable-Truth`); ties to §4.1 |
| T2 | Provide end-to-end encryption such that intermediaries see ciphertext + routing metadata only | a relay/meer stays blind (§8); supports the meer (§5.4) |
| T3 | Attempt a direct peer-to-peer path, with a **fallback relay** when direct fails | participation on a bare node behind a NAT must be real (`P-Durable-Enablement`) |
| T4 | Ensure any relay/intermediary is a **blind packet forwarder** (no session-content knowledge, routes by endpoint) | an intermediary must not become a center or an adjudicator (§3.1) |
| T5 | Provide a reliable stream mode **and** an unreliable datagram mode | messaging needs reliability; real-time media needs latency-over-reliability (§6.3) |
| T6 | Provide (or admit) an overlay for scope-member discovery/exchange keyed on the scope topic (§4.2) | members must find each other without a central directory |
| T7 | Not require a wall-clock for any authority-bearing decision | §2.0.1 |

**Disqualifiers, a candidate is non-compliant if it:**

- Routes or authenticates by **IP/location** as identity (breaks T1; IPs are ephemeral and
  impersonable).

- Requires a **content-reading** intermediary, any relay/SFU/mixer that must see plaintext to function
  (breaks T2/T4; server-side media mixing on plaintext is already **forbidden**, §6.3).

- Makes the fallback path a **mandatory permanent hop** (a relay that cannot be bypassed by a direct
  connection is a structural center, breaking T3/T4).

- Offers only reliable streams, forcing real-time media through retransmit (breaks T5, starves media,
  the measured failure in §6.3).

**Reference realization: iroh (1.0), and why it currently wins.**

iroh is the reference because it satisfies T1–T5 directly and cleanly. It establishes **peer-to-peer QUIC
connections between endpoints identified by public key** (the `EndpointId` is the TLS identity, so it
cannot be impersonated, T1), all connections are **end-to-end encrypted with concurrent streams and a
datagram transport** (T2, T5), it **attempts a direct hole-punched connection and falls back to a relay**
when that fails (T3), and, the property that matters most for §3.1, its **relay is a stateless blind
forwarder**: it carries encrypted packets without knowing whether they contain application data or
coordination, and the payload is always encrypted to the destination endpoint (T4). *External transport
facts cite the FACTCHECK SoT, iroh `1.0.0`.* **[confirm before publish, iroh relay-blindness and
public-key-identity claims against iroh 1.0 primary docs.]**

**The topology question, stated surgically, because "p2p" does not cleanly capture the design.** This is
the point that motivated dropping *serverless* and reserving *peer-to-peer* for the transport (naming
note, §3.1). The requirement above asks for **point-to-point reachability with a blind fallback**; it
does **not** require, and does not forbid, any particular *overlay topology*. So:

- **What is required:** that any two scope members *can* reach each other directly when the network
  permits, that no intermediary is a content-reading center, and that the fallback is bypassable. This is
  a **reachability** requirement, not a **topology** requirement.

- **What is a legitimate, compliant divergence:** a **pure peer-to-peer mesh** (every member maintains a
  direct path to every other) is fully Drystone-compliant, nothing in the spec prevents it. So is a
  relay-assisted star, a gossip overlay (HyParView/Plumtree, the iroh-gossip reference, §6.1), a
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
target worth re-checking).

**Capability mechanism** (Meadowcap's data-access sense; §5.5). The compliance bar **is** §7.2 R1–R6
(unforgeable grant, attenuating delegation, convergent revocation expression, bounded stale-authority
exposure, forward read exclusion, attributable acceptance), the standard object-capability guarantees.
*References:* Track A (Meadowcap-shaped delegated tokens) or Track B (Keyhive-shaped convergent membership
graph), Appendix A; both satisfy R1/R2/R3/R6; they differ only on revocation immediacy. No Drystone
requirement assumes a track. The **role** layer (in-group governance authority, §5.5) sits *above* this:
roles are granted by the governance fold and may carry the authority to issue capabilities, but the
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
| Capability (Meadowcap data-access) | §7.2 R1–R6 | Track A (Meadowcap) / Track B (Keyhive) | mechanism-neutral; differ only on revocation immediacy; the in-group **role** layer (governance authority) sits above this (§5.5) |
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
  no native revocation, so R4 is met via bounded expiry (revoke = decline-to-renew) and R5 via per-scope
  epoch keys; revocation latency is bounded by the expiry interval (expressed as an epoch/generation bound,
  not a wall-clock interval, §2.0.1). Track B makes removal and re-encryption first-class convergent
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
  (§2.0.1), not merely gameable. Drystone keeps power and clock out of the ordering spine entirely
  (§7.5.2). **Separately**, Drystone *adopts* Matrix State Resolution v2.1's conflicted-subgraph closure
  (§7.5.2), taking their convergence fix without taking their ordering. The CVE-2025-49090 state-reset
  class is cited in §7.3 as evidence for the monotonic-fold choice and was rooted in starting-state/replay-
  scope, **not** in the rejected tiebreak, keep the two points distinct. **[confirm before publish,
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
which the MLS drafts hand back to the members. **[confirm before publish, draft-kohbrok-mls-dmls / FREEK,
draft-xue-distributed-mls, and the MLS-deployment-status claims; see Appendix C.]**

## Appendix B. Open Questions (forming; not weakening the normative sections)

These are known-incomplete and tracked so they are not mistaken for settled:

- **Vendor-neutral naming reconciliation.** The reference implementation's signed domain-separation tags
  use the historical `croft-*` namespace (§4.2). Drystone, the protocol, requires a versioned
  domain-separated tag but should define a vendor-neutral namespace (`drystone-*`). Because the tag is
  signed over, the rename is a real wire change that re-opens the §4 signature proofs; it must be defined
  and re-proven, not silently swapped.

- **Hash-function reconciliation.** The proven message/history layer (§4) is green-real on **SHA-256**; the
  designed governance-resolution layer (§7) specifies **BLAKE3** for canonical fact content-addressing.
  Note that Willow/Earthstar uses BLAKE3 (256-bit) for payload hashing (confirmed against the Willow spec's
  Earthstar instantiation), so the §7 choice is convergent with the data model Drystone is built toward,
  mild evidence the single committed suite should be BLAKE3 and §4's SHA-256 is the legacy side. The single
  suite the production profile commits to (and any transition) must be pinned, and Willow notes BLAKE3 is
  "to be replaced by WILLAM3," which the pin should track.

- **`ENABLING` wire encodings** (gate a publication-final DOI): canonical governance-fact byte encoding
  (§7.3.1, the base all others extend); the canonical content-id pre-image now that `timestamp` is removed
  (§4.2); frontier-commitment construction and acceptance-record format (§7.5.1);
  **frontier-closure-and-subgraph-closure before sort** (§7.5.2, the highest-risk divergence point); the
  gating-vs-read relationship (§5.8.1); the capability/membership-graph wire format (gated on the Track A/B
  decision).

- **Root-authority succession** (§7.3): deferred to a Lifecycle section; the most dangerous operation in
  the system.

- **The capped-vs-uncapped-root soundness question** (§7.3, and Part 1 §2.3): **the priority open
  security item.** Matrix concluded under adversarial review that sound decentralized resolution needs an
  *uncapped* root (MSC4289). Drystone claims a *capped, revocable-by-succession* root is sound *because* its
  ordering is timestamp-free (§7.3.1), closing the backdating surface that forced their apex. The work to
  close this:

  - **State the coverage, not a bare "tested/open."** The MSC4289 attack class has at least three
    components: (a) backdating to manufacture favorable causal/authority position; (b) the
    create-event-uniqueness / root-forgery vector (Matrix's CVE-2025-54315 / MSC4291); and (c) the
    self-demotion / promote-others entrenchment trap. Drystone's timestamp-free order addresses (a); the
    `H(tag ‖ group_id)` genesis and unconflictable-base fold address (b); the anti-entrenchment ladder and
    anti-brick floor (§5.7) address (c). **Identify which of (a), (b), (c), and which of their
    compositions, the existing tests actually exercised**, and state that coverage. If composition
    (a)+(b)+(c) was tested under adversarial conditions with must-reject vectors, the claim is closer to
    "tested" than "argued" and should say so; if only the components individually, refine the claim to
    "components proven, composition open."

  - **Frame it as the design-philosophy difference, which is the cleaner thing to test** (§5.7 note):
    Matrix *prevents* capture with an apex; Drystone *permits* capture and makes the §7.6 fork the remedy.
    So the question is not only "can a capped root match their soundness" but "**is exit-as-remedy-for-
    capture sound where apex-prevents-capture was their choice**", and the latter is what the test suite
    should target. **[confirm before publish, MSC4289 / MSC4291 / CVE-2025-54315.]**

- **The open rights check** (§5.3): does the §7 survivor/re-key path strand `tenure` (leave a peer
  formally a member but unable to re-establish standing after a re-key)? This gates freezing the rights set
  (now three: tenure, voice, exit) into normative text. The candidate fourth right `share` has been
  **dropped**: a claim on shared assets is not part of the inalienable floor; where it has substance it is
  ownership of a Meadowcap communal namespace (§5.10), a data-layer matter, not a right. The concrete test
  to run: take a peer with valid standing, drive a survivor re-key of the scope, and check whether that
  peer can still re-establish its membership standing from its retained lineage and local state, or whether
  the re-key leaves it unable to rejoin its own scope. If the latter, tenure is not yet clean.

- **The false-positive escalation tolerance** (§7.4.1): the signals are corroborable provenance; the
  tolerance over them is a governed utility judgment whose *value* this spec declines to fix. The open work
  is the byte-level definition of the exposed signals and the policy-fact format for the tolerance, plus
  guidance (not a default) on tailoring it to use case without inviting either fatigue or silent false
  resolution.

- **What grounds a peer's authority, and what makes a right cost something to violate?** §3.1/§5.2 define
  a peer as a locus of adjudication. The working position (refined from earlier candidate groundings): a
  peer's authority is grounded in the **rights floor being variety-enabling, and therefore
  system-sustaining**. The floor is what lets the system hold plurality; negating a peer's rights lowers
  the variety available to resist the next negation, so rights-negation is the self-amplifying move toward
  collapse (Part 1 §2.3). That is precisely what makes a peer's authority *necessary* rather than granted:
  violating a right is not a local wrong against one peer, it degrades the system's capacity to absorb the
  next shock, and the cost is borne by everyone, not only by the peer whose right was negated. This is the
  companion to "where do decision rights sit": rights cost something to violate because the cost is
  systemic, which is the early detector of peer→sensor rot.

  Distinct from *why authority is necessary* is *what binds a human to a peer in the first place*. That
  binding (mint-and-bind: tying a human identity to a fixed cryptographic peer identity) is **contextual**,
  and the spec should say so explicitly rather than seek a single mechanism. A family group is simpler and
  higher-trust to bind (a QR scan in person suffices); a large group of disconnected strangers is harder
  and may need a credential service or a weaker, accepted binding (§5.6). The grounding of the binding is
  therefore the same contextual trust gradient as personhood itself, not a protocol primitive. This couples
  to the §5.8 exitability backstop and the exit-as-remedy-for-capture framing above.

- **External-fact confirmation** (§7, and the Beer/Cybersyn/OGAS grounding in Part 1 §3): the comparisons
  consolidated in Appendix C (CALM, Willow/Meadowcap/Keyhive, Matrix State Resolution and the 2025 CVEs,
  decentralized-MLS, Modular Politics) and the Beer quotes / Cybersyn-OGAS history are web-verified in
  source dialogues only and **[confirm before publish]** against primary sources before they harden.

## Appendix C. Consolidated Prior Art and Positioning

This appendix gathers, in one place, the prior art each layer builds on and the precise relationship to
each, so the novelty claim can be stated honestly and a reviewer can see the landscape at once. **Status
of grounding:** the **MLS** (RFC 9420/9750), **Meadowcap/Willow**, and **Spritely/ActivityPub** claims
have been confirmed verbatim against their primary sources this revision. The **Matrix State Resolution**
mechanics (MSC and CVE specifics), the **CALM/CRDT** attributions, and the **Beer / Cybersyn / OGAS**
history remain `[confirm before publish]` against primary sources, current grounding for those is web
research from the design dialogues, not yet the FACTCHECK SoT. The honest novelty claim is **synthesis and
terminus, unoccupied against the closest published neighbors**, *not* "first ever," and *not* novelty of
the underlying mechanisms.

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
  and the fork as a first-class designed *good* (§2.5, §7.6), not a failure. The DAO-hard-fork literature
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
    administrators specify at the level of the Instance"). Drystone's roots in no node above the peer
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
intrinsically-personal nature of the §2.5 residue is *why* this gap is legitimate and not a failure of
nerve: the protocol technicalizes only the provenance layer and **mechanizes the refusal** to technicalize
the utility layer (the §7.6 hard-stop is a primitive whose content is "a human decides").

## References

**Normative:** BCP 14 (RFC 2119 / RFC 8174); the signature and hash suites of the committed wire profile
(§4.1); QUIC (RFC 9000) and the iroh transport (iroh `1.0.0`, FACTCHECK SoT); RTP-over-QUIC for media (§6.3).

**Informative (and [confirm before publish] where load-bearing; consolidated in Appendix C):** the CALM
theorem (Hellerstein & Alvaro) as the formal boundary for the escalation cut; CRDTs / local-first
(Shapiro; Kleppmann et al.) as the data-layer premise; the Willow data model (namespace / subspace / path;
range-based set reconciliation; authorized-write hook), Meadowcap (delegated **capabilities**, read/write
data-access grants, kept verbatim as Drystone's data-access term, §5.5; attenuation by subsetting;
communal/owned namespaces) and Keyhive (convergent capabilities / membership graphs) as the two
data-access **capability** tracks, with the in-group **role** layer (governance authority) sitting above
them (§5.5); Matrix State Resolution v2 / v2.1 (MSC1442 / MSC4297), the 2025 Project Hydra disclosures
and CVE-2025-49090 / CVE-2025-54315, and MSC4289 / MSC4291 as the rejected-ordering, adopted-closure, and
uncapped-root-steelman references that motivate §7; decentralized-MLS (`draft-kohbrok-mls-dmls` / FREEK;
`draft-xue-distributed-mls`) and MLS (RFC 9420) for the forward-secrecy cost of center-free ordering;
Modular Politics (Schneider, De Filippi, Frey, Tan, Zhang, CSCW 2021) and Ostrom's IAD/polycentric work as
the governance-as-protocol neighbor; Sigstore countersigning as the sign-over-prior-state pattern adopted
for the frontier commitment (§7.5.1).
