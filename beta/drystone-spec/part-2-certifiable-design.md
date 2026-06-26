# Drystone — Part 2: The Certifiable Design

`Status: beta — build-against shape complete; byte-level ENABLING encodings open (Appendix B)`

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
> two layers also differ today in hash function (§4 proven on SHA-256; §7 designed on BLAKE3) — a real
> reconciliation item, surfaced in Appendix B rather than papered over.

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

- **History** — the content peers author, as signed, hash-chained entries. This is the data plane.
- **Governance facts** — signed, append-only entries recording who may do what (admit, expel, grant,
  revoke, amend). Authority is a deterministic fold over these. This is the control plane.

A peer is one or more **devices** (keypairs) acting under a single **lineage**; receivers fold devices
back to one actor so that membership and thresholds are counted by actor, not by device. Peers reach each
other over an encrypted QUIC transport with relay fallback; a relay forwards opaque frames and need not
read content. When two peers' views diverge, **range-based reconciliation** finds the difference and
exchanges the missing entries; **history** converges by hash-chain replay, and **governance** converges
by a timestamp-free causal-and-cryptographic resolution order. Genuine membership contradictions are
**not** auto-merged — they hard-stop and escalate to humans, and a minority's sanctioned exit is a clean,
attributable **re-formation fork**.

The sections below specify each piece. §4 is the data model; §5 is identity, rights, and capabilities;
§6 is transport; §7 is synchronization and governance-conflict resolution; §8 is security; §9 is
interoperability and conformance.

### 3.1. What makes this a system *of peers* — the diagnostic that orders the spec

A network of many nodes with any-to-any connectivity is **not** automatically a distributed system of
peers. The distinction this specification turns on is not topological — it is about **where adjudication
lives.** A **peer is a locus that can *adjudicate*** — it holds genuine authority over some domain that
other peers must respect — not merely a node that can sense, store, and relay. A node that only senses and
relays, with its decisions made elsewhere, is a **sensor**, however well-connected it is.

The cautionary archetype is a national economic-planning network of the 1970s with tens of thousands of
local terminals and any-terminal-to-any-terminal wiring, whose adjudication nonetheless all funneled to a
single apex: thousands of nodes, **zero peers in the sense that matters**. Its terminals sensed, reported,
and received instructions; none adjudicated. **The wires lie; the authority topology tells the truth.**
The diagnostic that follows is mechanical: **count the adjudication loci.** If the answer is one (or a
small apex), the system is a centralized decision apparatus wearing a distributed sensor mesh as a costume,
regardless of how its packets flow.

This is why Drystone specifies **rights separately from capabilities** (§5) and cannot let either be read
off a network diagram: a **capability** ("this node can do X" — sense, store, relay, compute) is visible in
the plumbing, but a **right** ("this node's authority over X must be respected") is visible only in the
governance. It is also why the spec is **ordered** the way it is — define the peer (a locus of
adjudication, §5.2), then peer rights (§5.3), then the mechanics that keep adjudication distributed (§7,
§8) — and why the **label-not-enforce** posture (§8) is load-bearing rather than cosmetic: enforcement
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
**lineage identifier** names a logical actor; multiple devices may act under one lineage (§5.6). A
signature **MUST** verify against the author's published verifying key before any other check, and a valid
signature is **necessary but not sufficient** — standing (§5, §7) is also required.

The reference implementation uses **Ed25519** (RFC 8032, deterministic, 64-byte signatures) and
**SHA-256** (32-byte digests). *green-real* (real Ed25519 over live iroh: a forged message is rejected
as a bad signature on every receiver, including a NAT'd peer). A production profile **MAY** select a
different suite, but the suite is part of the versioned wire profile and **MUST NOT** be silently
negotiated down.

### 4.2. Identifiers and derivations

Wire identifiers are a hash over a **tagged** pre-image: `tag = version ‖ domain-separator`, so one
identifier kind can never collide with another's input. An implementation **MUST** derive these
identically; they are the interop anchor.

| identifier | pre-image (structure) |
|---|---|
| lineage genesis | `H(tag["lineage-genesis"] ‖ lineage_id)` |
| group genesis | `H(tag["group-genesis"] ‖ group_id)` |
| group gossip topic | `H(tag["group-topic"] ‖ group_id)` |
| content id | `H(canonical{group_id, regime, author_id, content, timestamp})` |

*green-real* — the three tagged wire derivations are conformance-tested canonical functions, byte-identical
to the proving spike. The reference profile's tag strings are `croft-lineage-genesis:`,
`croft-group-genesis:`, `croft-group-topic:`; **Drystone normatively requires a versioned, domain-separated
tag and does not mandate these strings** (Appendix B, naming reconciliation).

A scope's gossip-topic seed **MUST** be high-entropy / salted, not a guessable human handle — otherwise an
adversary computes the topic and joins or observes. *(Leak bound characterized; see §8.)*

### 4.3. The signed message — the unit of history

The history unit binds author, position, branch, and payload so a message cannot be replayed onto another
branch or position. The signed pre-image (reference profile):

```
signing_bytes = "msg-v1" ‖ branch(32) ‖ seq(LE u64) ‖ author_id_bytes ‖ 0x00 ‖ payload
```

A receiver **MUST** recompute the pre-image and verify the signature against the author's key. *green-real*
— the real message traveled live iroh-gossip and verified against a real backfill import; an honest
member's message is accepted, a forged one rejected.

### 4.4. Integrity-and-ordering vs authorship-and-standing — two distinct guarantees

These are kept strictly separate, and conflating them is the central honesty error the protocol forbids:

- **Integrity + ordering (structural).** A branch is a sequence chained by
  `hash = H(prev ‖ seq(LE) ‖ payload)`; a receiver **MUST** reject a branch with a broken chain or
  non-contiguous sequence numbers. This proves in-transit integrity and contiguous ordering — **not** who
  wrote it.
- **Authorship + standing (authority).** The signature (§4.3) plus standing (§5, §7). A receiver **MUST**
  apply both; **integrity alone MUST NOT be treated as authorization.**

*green-real* — a valid-chain branch from a non-member is accepted by the structural check but rejected by
the authority check as an unauthorized author. This separation is what makes a branch trustworthy: the
hash chain proves it was not tampered in transit; only signature + standing prove it may be there.

### 4.5. Multi-device fold — device-count ≠ actor-count

Devices of one actor share a lineage; each device carries a distinct device identity. Receivers **MUST**
fold absorbed branches by lineage into one actor. A scope's topic carries multiple lineages; the fold is
what every peer computes identically to agree on the member list and on **lineage-counted thresholds**
(§7.2). *green-real* — one actor's two devices fold to one; all peers agree on the folded actor count.

---

## 5. Identity, Rights, and Capabilities

> `Realizes: P-Peer-Equality, P-Local-Truth, P-Knowable-Truth, P-Durable-Enablement`

This section fixes the vocabulary that keeps a rights distinction from being smuggled in as a capability
distinction. It is deliberately small, and it is the section a reviewer presses hardest on, because it is
where `P-Peer-Equality` is enforced by mechanism rather than assumed.

### 5.1. The only canonical state is local

A device can prove exactly one thing: its own local state. Its local store is **canonical for that peer**.
Any belief about another peer's state is **comparative** (range reconciliation) or **asserted** (a signed
fact it accepted), never canonical. There is **no global canonical state**, by design. A lagging peer
computes a stale-but-honest state, never a false one, because it is only ever reading its own store.
*green-real / design* — this is the property §7 relies on to make authority a fold over an append-only
view.

### 5.2. There is one kind of peer — and a peer is a locus of adjudication

A **peer is a locus that can adjudicate** — it holds genuine authority over some domain that other peers
must respect — not merely a node that can sense, store, and relay (§3.1). This is the prior definition the
rest of this section depends on: you cannot reason about *peer rights* until you have said what a peer
*is*, and the answer is about decision rights, not connectivity.

Drystone defines a single participant: a **peer**. No species, no tiers, no lesser participants.
Differences between peers are differences in two separate things, kept apart by §5.3 and §5.4:

- **what a peer is entitled to** — rights, universal; and
- **what a peer has been granted the means to do** — capabilities, additive and delegated.

A peer is **never defined by subtraction.** We never say "a full peer minus a capability"; we say "the
universal floor, plus whatever capabilities were delegated." Every named configuration **MUST** decompose
to `floor + [additive capability set]`. A configuration that cannot be written that way is a smuggled
rights distinction and **MUST** be rejected (§5.5).

### 5.3. Rights: universal, never delegated

A right is a claim a peer is entitled to, not a power it holds and could pass on. Rights **cannot** be
delegated. The base floor, held identically by every peer:

- **Read your own local history.** Unqualified, identical for all peers.
- **Read the history of a scope you are a member of, for the period of your membership.** Begins at join,
  ends at leave; includes what the peer was present for; does not extend to content authored after the
  peer leaves, and does not retroactively vanish for content the peer legitimately held while a member
  (§5.7).
- **A scope holds full history for itself,** independent of any member's tenure.

Two consequences where the floor is most often misread. **A peer's own history is permanent; its window
into a shared scope is bounded by membership** — two different histories, treated as such. **A peer with
no local history of its own still holds the full read-your-own-history right** — it is simply exercised
over an empty set (the pure-availability peer of §5.4 authored nothing, so it satisfies the right
vacuously). Blindness is the *absence of a granted capability*, never a restriction on a right.

> **Two open checks before the rights set hardens** (carried from Part 1 §2.3): the proven floor in this
> draft is the read-rights triple above. The fuller four-rights articulation — **tenure** (standing to
> remain a peer), **exit** (the right to fork), **voice** (standing to assert into the record and be
> corroborated or refuted), and **share** (a claim on a scope's commons) — fixes each right by what its
> removal would foreclose. `share` and `tenure` are not yet frozen: whether `share` is fully a right or
> partly a membership-class capability, and whether the §7 survivor/re-key path can strand `tenure`, both
> gate hardening the closed set into normative text. *design* (Appendix B).

### 5.4. Capabilities: additive, delegatable, revocable

A capability is the means to *do* something, sitting on top of the floor. Capabilities are **additive**
(floor plus zero or more), **delegatable** (a holder may grant one it holds), and **revocable** (§5.7).

Delegation has two normative properties:

- **It is always additive and bounded.** A peer **MUST** be able to delegate only a subset of what it
  holds, never a superset (`Realizes: P-Peer-Equality`; the attenuation requirement, §7.2 R2). Granting
  never widens authority.
- **It targets either of two groups with the same primitive.** A peer **MAY** delegate to (a) a peer in
  its **own device group** (trust stays within personal control, leaks to no third party) or (b) a peer in
  the **shared scope** (a cooperative anchor, or another member's always-on node). The mechanism is
  identical; the trust boundary is the user's choice. This two-target flexibility is what lets Drystone
  refuse consolidation onto a single keeper.

**Common capabilities** (the current set, not closed):

- **Availability.** Hold and serve a scope's *encrypted* objects, including buffering for offline members.
  Availability does **not** imply read; an availability-only peer holds ciphertext it cannot decrypt.
- **Read / search-offload.** Decrypt, index, retain, and serve a scope's history so a resource-limited
  peer can exercise its read right without indexing everything itself.
- **Gating.** Act on the distribution or visibility of content within a scope, per the scope's governed
  rules. **The one capability that requires care against the rights floor** (§5.7.1).

### 5.5. Capability, role, and PeerSet — three layers, none touching rights

- **Capability** — an atomic delegated power. Fluid; granting/revoking one is normal, no alarm. Each grant
  is a governance fact (§7).
- **Role** — the *descriptive* set of capabilities a peer holds right now. Composes by union, mutates
  freely, triggers no alarm: change is its nature.
- **PeerSet** — a *named, pinned, group-recognized* bundle with an **enforced composition** that includes
  both a **required** set and a **forbidden** set. Prescriptive: it answers "what is a peer of this name
  supposed to have, and not have." Drift from the pinned composition is an integrity event the group flags.

All three live entirely in the capability plane. A peer carrying any of them holds the complete rights
floor. The mechanical check: every role and PeerSet **MUST** be definable as `floor + [explicit
composition]`; a name meaning "entitled to less" is **forbidden**. **Rights have no presets; only
capabilities do.**

A PeerSet's pinning is enforced by a **drift check**: the group gathers the capability grants in force for
a peer from the governance log and compares them against the declared PeerSet's required-and-forbidden
composition. Mismatch in **either** direction is the alarm — a peer that *acquired* a forbidden capability
(dangerous) or *lost* a required one (failing the job relied on). The check is mechanical, because both
sides are facts already in the log. This is why "it is a meer" is a *trustworthy* statement: the group
enforces the pinned composition.

Worked example — the **meer** (an always-on blind helper):

```
meer (a PeerSet) ::= floor
                   + requires { availability }
                   + forbids  { read, * conferring read }
```

A meer has full standing and the full rights floor; it satisfies read-your-own-history vacuously (it
requires availability and forbids read, so it holds no plaintext of its own). Its blindness is the
*enforced absence* of read, not a special restricted nature. Delete the name and nothing about the peer's
rights changes. *green-real (Tier-0 blind broker / running blind meer); design (PeerSet drift-check
formalism).*

### 5.6. Membership, standing, and revocation authority

**Standing** is decided from recorded, signed data — never the actor's own assertion. A message is
authorized iff its author held standing on a branch sharing the relevant lineage root. *green-real.*

**Revocation** removes a device/actor from the accepted set *going forward*. Survivors **MUST** reject the
revoked party's subsequent branches and **MUST NOT** claw back history contributed before removal (standing
≠ membership; history is not erased). *green-real.*

**Revocation/add authority is a threshold dial** (k-of-n, counted by distinct lineage): default 1-of-any,
up to k-of-any or role-restricted admins. A membership op is authorized iff it carries signatures meeting
the scope's **current, replicated** policy; policy lives in versioned scope state and is itself changed by
governance ops under the current policy. The canonical form is a **co-signed op** — a self-certifying
k-of-n bundle validated locally against the current epoch, freshness-gated (§7.4); proposal-plus-votes is
an optional deliberative mode. *green-real (real k-of-n bundle verified over live transport: an
authorized 2-of-≥2 revoke accepted, an under-threshold revoke rejected).*

**The admin floor is derived from policy, anti-brick only.** A threshold `k_op` **MUST** be ≤ eligible
signers by distinct lineage at the epoch it is set (solo genesis ⇒ `k_op = 1`; a scope **MAY** be born
"create with 10, need 5"); raising above headcount self-bricks and is rejected. Once set, the scope
**MUST** retain `n ≥ k_op`; a membership op whose post-state breaches the floor is **structurally invalid**
(rejected by every verifier from replicated policy alone). `k` **MUST NOT** auto-track `n` downward (a
threshold-downgrade attack). The floor is **anti-brick only**: a legitimate quorum acting within policy —
including self-capture — is accepted; the recourse for an out-voted minority is the §7.6 re-formation fork,
never a structural veto. **Capture ≠ brick.** *design — decided; tests specified, not yet run.*

**Roles are revocable delegations, never impositions.** Every granted role (admin, moderator, a
content-gating role, an always-on meer) **MUST** be a revocable delegation under the same threshold
authority, **MUST** carry only scoped, enumerated, non-creeping rights, and **MUST NOT** be immutable,
forced, or held by structural right. A creator holds **no** structural superuser right: at creation they
receive a bootstrap admin role purely so a one-member scope can function, revocable like any other.
**Anti-entrenchment ladder:** any delegated role is revocable (1) routinely under the policy threshold,
(2) as an always-available backstop by unanimity of the non-holders (a ceiling on revocation difficulty —
a group may set an easier bar, never a harder one), and (3) ultimately by the §7.6 fork. **No grant may
make itself irrevocable.** *green-real (revocation mechanics); design (ladder — decided).*

### 5.7. Revocation reuses governance machinery; it protects the future, not the past

A capability grant is a governance fact (§7); revoking it is a new fact that supersedes the grant,
resolved by the same total order and fold as any other governance conflict. For capabilities that confer
read, revocation **MUST** rotate the scope epoch so the revoked peer cannot read content authored after the
revocation folds in (identical to membership expulsion). Revoking a read-delegate is, formally, an
expulsion-shaped fact.

This inherits an honest limit: **revocation protects the future, not the past.** A peer that held a read
capability while valid may have retained what it read; revocation stops future access, it cannot unmake
what was legitimately held. The plain form, which a non-expert can hold: *you can revoke a delegate's
access to new content at any time and it actually takes effect, but the delegate may keep copies of what
it already saw — true of everyone anyone has ever shared anything with.* Stating it plainly is more honest
than implying otherwise.

#### 5.7.1. Open item — gating against the read right

Availability and read/search-offload are clean additive capabilities. **Gating** is different: it acts on
the distribution or visibility of content, which bears on other peers' ability to exercise their read
right, so the additive framing does not automatically dissolve the tension. The likely resolution, to be
*specified* rather than assumed: gating acts on distribution/visibility within the scope's own governed
rules, **not** on the underlying right to read what one legitimately holds; every gating action is itself a
governed, attributable governance fact; and a gated peer's right to its own local history (including what
it already holds) is untouched.

> `ENABLING:` The precise relationship between a gating action and the read right MUST be specified — what
> it can and cannot affect, how it relates to content already held locally, and how it is bounded by the
> scope's rules so it cannot become a backdoor for suppressing a right under the guise of a capability.
> This is the one capability that, specified carelessly, could re-introduce a rights distinction through
> the capability layer, and therefore the one most needing an explicit forbidden clause wherever it appears
> in a PeerSet.

A content-visible gating role also weakens the system's "cannot comply" property (compellability): a peer
that has seen content cannot un-see it on revocation. The default **MUST** therefore remain blind and any
such role **MUST** be strictly per-scope opt-in, disclosed, scoped to the least-invasive rung, and
accountable — and it is a policy/legal question, not only an engineering one (gates a real deployment).

### 5.8. Exitability — the backstop that makes flexibility real

> `Realizes: P-Durable-Enablement`

A delegated capability is only meaningfully different from a captive structural dependency if the
delegation can be withdrawn and restructured **without loss of rights.** Therefore:

Any default delegation a peer or scope adopts **MUST** be revocable and restructurable down to the rights
floor (§5.3) at any time, with **no loss of rights** and **only graceful degradation of capacity**, never
loss of function or standing. Concretely, a scope that delegated availability and search-offload to some
peer (including a cooperative anchor or single operator) **MUST** be able to move that delegation to a
different peer, split it across several, pull it into its own device groups, or drop it entirely — and in
every case continue to function. What degrades is capacity (deep search may slow or need a member's own
device online); never rights or the ability to communicate and govern.

Material reversibility is normative, not formal: (a) a helper holds only **encrypted** state and the scope
holds the keys, so the scope **MUST** be able to re-host on or migrate to a different holder (no data
hostage); (b) the scope **MUST** be able to stand up a different helper and elect it in place of the
incumbent (the role is a re-issuable grant, not bound to a box); (c) the §7.6 re-formation fork remains the
adversarial backstop. *green-real — a meer's encrypted store was exported, imported into a different
replacement meer, and a member re-homed and converged identically; losing a meer costs availability, never
data.*

**The asymmetry of expressible range** (a checkable claim, not a quality judgment): a flexible model can
present as the rigid one, but the rigid one cannot present as the flexible one. Drystone can be configured
to behave like a single-keeper deployment; a server-shaped system cannot be configured to behave like the
exitable, per-capability-delegated model, because its central dependency is structural rather than granted.
The design question this hands forward — and the thing to press the spec on — is whether the exit path is
genuinely lossless-in-rights *in the default case* and not only at the unused margins (§5.6 admin floor,
§7.4 freshness, the no-helper-path obligation of `P-Durable-Enablement`).

---

## 6. Transport

> `Realizes: P-Local-Truth, P-Peer-Equality, P-Durable-Enablement`
>
> External transport facts (iroh / QUIC) cite the FACTCHECK SoT — iroh `1.0.0` — do not re-verify.

### 6.1. Connection establishment and routing

The transport is **iroh**: encrypted QUIC with relay fallback for NAT'd peers, routed by `EndpointId`
(public-key endpoint identity; renamed from `NodeId`, per FACTCHECK SoT). Scope membership maps to a
gossip **topic** (§4.2), carried over **iroh-gossip** (HyParView/Plumtree, per FACTCHECK SoT). A relay
forwards opaque frames and **MUST NOT** be required to read content; it routes by endpoint, not by topic.
*green-real — NAT path via relay; blind broker sees only ciphertext plus routing metadata.*

**Co-location** (reference deployment): two peers reach each other over relay fallback only if they share
a home relay (no relay-to-relay mesh); relay placement is server-published and authoritative, keyed on the
rendezvous/namespace, not on identity. A relay process **SHOULD** meter and isolate per tenant and
**MUST** degrade *visibly* under stress, never silently. *green-real (measured).*

### 6.2. Interaction tiers

Interaction tiers are chosen at scope creation, not toggled at runtime: **interactive** (prompt delivery +
real failure signal), **quiet-large** (eventual — "it will arrive or you will be told it did not"), and
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
> Reasoning complete; several wire encodings are `ENABLING` (Appendix B). The comparative claims about
> Matrix / Willow / Meadowcap / Keyhive are **[confirm before publish]** — web-verified in the source
> dialogue, not yet in the FACTCHECK SoT.

### 7.1. Data-model commitment

Governance facts live in a **namespace / subspace / path** structure addressed and reconciled by
**range-based set reconciliation** (a Willow-shaped data model **[confirm before publish]**). Drystone
implements this *shape* directly in early phases rather than depending on a Willow implementation, so a
later transition is a substitution, not a redesign — Drystone is built Willow-*shaped*, not
Willow-*dependent*.

### 7.2. The capability-and-revocation interface (mechanism-neutral, normative)

Whatever capability mechanism Drystone adopts **MUST** provide:

- **R1 — Unforgeable grant.** A capability cannot be fabricated by anyone not entitled to issue it.
- **R2 — Attenuating delegation.** A holder may delegate a subset of held authority, never a superset
  (`Realizes: P-Peer-Equality`).
- **R3 — Convergent revocation expression.** A revocation **MUST** be expressible as a governance fact
  (§7.3) that folds deterministically, so all synced honest peers agree the capability is void.
- **R4 — Bounded stale-authority exposure.** For a holder that refuses to sync a revocation, the protocol
  **MUST** bound the window in which third parties accept the revoked capability — a finite, stated bound
  (time interval, epoch boundary, or membership-graph generation).
- **R5 — Forward read exclusion.** After expulsion, the member **MUST NOT** read entries authored after
  the expulsion folds in (past entries out of scope, §7.5).
- **R6 — Attributable acceptance.** A peer accepting a write under a capability **MUST** record the causal
  frontier of governance facts it had synced at acceptance, so a later-synced revocation makes the stale
  acceptance **detectable and attributable** rather than silent (§7.5).

R3 and R6 are what defeat a silent state-reset failure mode and hold regardless of the capability
mechanism. The mechanism itself (Track A vs Track B) is **deferred** to the richer-access-control phase;
see Appendix A. No normative text here assumes a track. *design.*

### 7.3. Governance facts are entries, not mutations

A governance decision (admit / expel / grant / revoke / amend) is a **signed, append-only entry**. Entries
are never modified or deleted; a reversal is a new entry referencing the one it reverses. There is no
"current state" to reset — only a monotonically growing fact set and a deterministic **left fold** from it
to an effective authority state. A peer that has seen fewer facts computes a **stale** authority state,
never a wrong one, and never one another peer could weaponize by replaying old entries (`Realizes:
P-Knowable-Truth`). *design — the append-only-fold property is the load-bearing invariant the rest of §7
depends on; if it is relaxed, termination (§7.5.4), no-state-reset, and attributability all fall together.*

**The unconflictable root.** Each scope has a founding fact establishing initial authority. Its authority
over the genesis of the scope is **not** subject to the §7.3.1 ordering — it is the base case of the
authority-rank computation, not a competitor within it, because a conflict at the root is the one ambiguity
an attacker could convert into total capture. Drystone's root authority is **capped, delegable, and
revocable-by-succession** (`Realizes: P-Peer-Equality`), not infinite. *design.*

> `ENABLING:` Root-authority succession (how founding authority transfers when founders leave) is the most
> dangerous operation in the system and is deferred to a Lifecycle section (Appendix B). A permanently
> fixed root contradicts the cooperative model.

#### 7.3.1. The total order over conflicting facts

When two facts conflict (§7.3.2), every honest peer **MUST** agree which takes effect. Drystone orders
conflicting facts by this key, compared lexicographically, lowest wins:

1. **Issuer authority rank at the causal frontier** (computed structurally, not by consulting the
   contested outcome — §7.5).
2. **Governance precedence class** — authority-reducing facts (expulsion, revocation) outrank
   authority-expanding facts of equal issuer rank (a conservative default: when valid decisions collide,
   the one reducing exposure wins).
3. **Causal length** — a more-informed decision (longer referenced causal history) outranks a less-informed
   one.
4. **Content-address tiebreak** — the digest of the canonical fact encoding, as the final deterministic
   discriminator.

**Timestamps appear nowhere in this order.** A wall-clock tiebreak is trivially gamed by a peer lying about
its clock — precisely how an attacker would manufacture a favorable resolution. The order is **causal and
cryptographic only**. *design.*

> `ENABLING:` The canonical byte encoding fed to the content-address hash, and the wire format of a
> governance fact, must be specified to byte level before two implementations interoperate. This gates a
> publication-final release.

#### 7.3.2. What conflicts

Two governance facts conflict when applying both, in either order, yields a different effective authority
state than applying them in the order §7.3.1 selects — concretely, two facts targeting the same subject
where at least one removes or narrows authority the other depends on. The **mutual-expulsion** case (A
expels B while B expels A, equal standing) resolves by steps 3–4; exactly one survives, never both and
never neither, and the loser's fact remains in the log as a valid-but-superseded entry, visible for audit.

### 7.4. Freshness — no false "current"

A peer/helper **SHOULD** periodically emit a signed, **content-free** tip beacon
`{scope_id, epoch, head, seq_high, sig}` (head/epoch/routing only — safe for a blind helper). A peer
**MUST** track time-since-last-heard *locally* (liveness is a local measurement, never trust in another
peer's wall-clock) and **MUST NOT** display a view as "current" unless it is both caught up to the
best-seen tip **and** has heard a beacon within the tier's freshness horizon; otherwise the view **MUST**
surface as "behind" or "unverified." Silence **MUST NOT** be rendered as currency. *green-model.*

**Membership/governance acts require strict CURRENT + corroboration.** To originate or co-sign an
add/remove/policy-change, a peer **MUST** be (a) caught up and (b) corroborated-fresh — within the tier
horizon, and after any unverified lapse, agreement on the same head from ≥k distinct lineages observed
stable — re-checked at signing. Ordinary content has no such precondition (it MAY be authored from a
behind/unverified view, honestly labeled). This **narrows, does not close**, the fresh-but-wrong-partition
window; the residual is the §7.6 hard-stop's, by design. *design — decided; tests specified, not yet run.*

### 7.5. Attributable acceptance and the regress-free fold

#### 7.5.1. Attributable acceptance (R6)

The guarantee is **detection and attribution, never prevention.** The acceptance record is a governance
fact signed by the accepting peer, whose signed body includes (1) the accepted entry's content digest
(binding *what* was accepted, via the authorized-write hook), (2) a **frontier commitment** — a commitment
over the set of governance-fact digests the peer claims as its synced frontier — signed as part of the
acceptance body (signing over prior signed state rather than a mutable timestamp), and (3) the peer's own
prior acceptance-record digest, chaining its acceptances.

Against the attack of lying about one's knowledge state: **frontier omission** is defeated cryptographically
(the commitment pins the set; the omitted revocation is provably in or out), and **equivocation** is
defeated by the per-peer acceptance chain (two signed chain heads with the same predecessor are
non-repudiable proof). **Backdating** cannot be defeated by cryptography alone — there is no trustworthy
internal clock — so the bound is **causal**: if the revocation is in the causal history of any fact the
peer's frontier includes, the "didn't have it" claim is refuted; the only residual is a peer genuinely
causally independent of the revocation, which is the legitimate concurrent-partition case R4 exists to
time-bound. Every stale acceptance therefore resolves into exactly one of two categories — **knowingly
stale** (full attribution) or **concurrently stale** (no fault, R4-bounded) — with no third category where
a peer silently escapes both prevention and attribution. *design.*

> `ENABLING:` The frontier-commitment construction (a Merkle root over sorted governance-fact digests), the
> acceptance-record wire format, and the per-peer chain linkage must be byte-specified before interoperation.

#### 7.5.2. Breaking the authority-ordering regress

Ordering conflicting facts by "issuer authority rank" appears circular — resolving authority would require
already-resolved authority. Drystone breaks the regress by **computing the order from causal structure
alone, then evaluating authority in a single forward pass** along that fixed order; authority is never
consulted to *produce* the order, only checked against the partial state the order has already built.

- **The ordering spine is causal, not authoritative.** Every governance fact references its causal
  predecessors (the frontier it was issued against). This forms a finite acyclic DAG; the resolution order
  is a topological sort over it (Kahn's algorithm), requiring no authority judgment.
- **The tiebreak is cryptographic, not temporal and not authoritative.** Ties break solely by the digest of
  the canonical fact encoding — no power, no clock — deterministic on every peer.
- **The forward pass checks authority against partial state.** Folding left from the unconflictable founding
  fact, each fact is admitted iff authorized by the authority accumulated so far. A grant that was itself
  never admitted cannot authorize a later fact.

It **terminates** (a single linear pass over a finite sorted list; the accumulator only grows; no fixpoint
iteration) and **converges** (every step is a deterministic function of order-independent inputs grounded
in an unconflictable base case; a lagging peer under-authorizes rather than diverging). *design.*

> `ENABLING:` **Frontier-closure before sorting** — the resolution input set must first be closed under
> "facts named in any included fact's frontier," or an implementation can admit a fact whose authorizing
> grant it failed to include, producing divergence. This is the **single most likely place for two
> implementations to disagree** and therefore the place the spec must be most exact.

### 7.6. The reconcile hard-stop and re-formation fork

When two histories merge, an implementation **MUST** detect membership contradictions (e.g.
removed-then-included) and **hard-stop** — it **MUST NOT** silently auto-resolve (last-writer-wins or
otherwise). Resolution is a social/governance input, not an automatic merge. The sanctioned exit for a
minority is a clean, attributable **re-formation fork**: a differently-shaped scope that preserves history
and provenance to the point of departure and legitimizes/erases nothing retroactively. Stripping a helper
operated by a cooperative or external operator simply **detaches** the scope into a differently-shaped one
(unpreventable anyway — the operator can always leave); the protocol only preserves history and provenance
to the detachment. *green-real (contradiction hard-stop; identical reformed genesis across independent
peers).*

This hard-stop is Drystone's **algedonic channel** (Part 1 §3): rather than letting an automatic rule
resolve a case it cannot safely resolve, the protocol raises the hard case to the humans who hold the
context — the formal version of "specify only somewhat, then escalate the residue." It is a *designed*
channel, not a failure path: the protocol commits in advance that genuine contradiction is a
human-adjudicated event, because no merge rule can be trusted to absorb it silently. Crucially, the
escalation keeps **both the signal and the authority local** — it surfaces the conflict to the affected
scope rather than relocating the decision to a center that lacks the context that made the conflict
legible.

---

## 8. Security Considerations

> `Realizes: P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement`

**Threat model (summary).** Drystone defends against: forged authorship (signature + standing, §4.4);
in-transit tampering (hash-chain integrity, §4.4); unauthorized membership/governance acts
(threshold authority + replicated policy + the admin floor, §5.6); silent authority reversal / state reset
(append-only fold, §7.3); manufactured-resolution via clock-lying (timestamp-free order, §7.3.1); and a
helper accreting control by holding state (blind-by-construction helpers + material reversibility +
exitability, §5.5/§5.8). It explicitly does **not** defend against: instant revocation (exposure is
bounded, not zero, §7.2 R4); retroactive confidentiality (past reads by a since-expelled member are not
unmade, §5.7); or *prevention* of stale-authority writes (only their detection and attribution, §7.5).
These non-guarantees are the honest price of an eventually-consistent, center-free architecture; none of
them lets an attacker capture a scope or silently revert a decision.

**Visibility and the social layer.** A scope's regime and visibility class are **born in at genesis and
immutable** (part of the signed genesis); there is **no silent regime crossing** — a republish is a
distinct authored act carrying a reference plus author-chosen content, never the original. Outward
propagation depth is enforced by every verifier. An implementation **MUST NOT** offer a structure-only
share (topology revealed, identities withheld) — graph topology is re-identifying; the only safe share is
consented-distance/resolution-scoped. *green-model (visibility regimes; the structure-only share is shown
unrepresentable — a modelled target's connection shape has anonymity set 1).*

**Metadata is the residual surface.** A blind helper still observes join-metadata (digest, length,
timestamp, namespace) and connection attempts; this surface **MUST** be surfaced, not hidden, and a
Tier-0 helper **MUST** hold no payload key and **MUST be able to prove it** (assert-and-log
`payload_keys_held = 0`). *green-real (Tier-0 meer proves zero payload keys; admission denies a non-listed
peer).*

**Failed-operation response.** Detection of an invalid op is deterministic; the *response* is a governance
dial — **loud** (signed, corroborated rejection → group immune memory), **silent** (reject, no signal), or
**blackhole** (tarpit). A serious auto-response **SHOULD** require k-observer corroboration. Note "silent"
is application-layer: the relay still observes the connection attempt. *design.*

**Label, not enforce — a peerhood-preserving primitive.** Where content moderation or social adjudication
is involved, the protocol's posture is to **label** (attach advisory, attributable metadata) and leave the
*action* to scope governance or each peer's own client, rather than to **enforce** (act unilaterally and
irreversibly on the network's behalf). This is not only a safety choice; it is what keeps the system *made
of peers* (§3.1). Enforcement relocates adjudication to whoever enforces — quietly converting peers into
sensors by stripping their decision rights — whereas labeling leaves adjudication with the peer and
propagates only information. It is the same algedonic move as the §7.6 hard-stop: **surface the signal,
don't seize the decision.** Each enforcement hook tends to look locally reasonable, which is exactly how a
peer network can degrade into a centrally-adjudicated sensor mesh over time; the label-not-enforce default
is the precommitment against that drift.

### 8.1. Honesty boundaries this specification still carries

Stated plainly so the spec does not over-claim: (a) freshness (§7.4) is proven in the model, not yet over
live transport; (b) the failed-op leak/immune dial is design-only; (c) a content-visible gating role's
**compellability** tradeoff is an unresolved policy/legal question, not an engineering one (gates any such
deployment); (d) the video media engine and real-codec/RTP path are design; (e) the membership-op
freshness threshold and admin-floor rule are decided-but-not-yet-test-run.

---

## 9. Interoperability and Conformance

> `Realizes: P-Knowable-Truth, P-Peer-Equality`

Two implementations are **Drystone-compatible** when they agree on every normative section that forces
agreement: the identifier derivations (§4.2), the signed message pre-image and verification (§4.3), the
integrity-vs-authority separation (§4.4), the lineage fold and lineage-counted thresholds (§4.5, §5.6),
the rights floor and the `floor + capabilities` decomposition (§5), the governance-fact append-only fold
and total order (§7.3), and — once its `ENABLING` encodings are pinned — the frontier-closure-before-sort
rule (§7.5.2), which is where two implementations are most likely to diverge. **Peer equality is shown to
be enforced by mechanism, not convention, exactly here:** a conformant implementation cannot grant a peer
a rights difference, because §5 makes every configuration `floor + capabilities` and rejects anything
that decomposes otherwise.

A conformant implementation **MUST** pass the conformance vectors and must-reject cases. The reference
conformance suite is **built and passing (66/0)**, derived by running the real implementation: derivations
(incl. the tagged wire forms), signed pre-images, the fold and lineage-counted thresholds, revocation
mechanics and k-of-n revoke-authority, the reconcile corpus, the adversarial cases, and the visibility and
freshness vectors. *green-real (suite) — note the suite covers the §4/§5/§6 proven layer; the §7.3–§7.5
governance-resolution vectors depend on the `ENABLING` encodings in Appendix B and are not yet in the
suite.*

---

## Appendix A. Alternatives Considered

- **Capability mechanism — Track A (delegated-token, Meadowcap-shaped) vs Track B (convergent
  membership-graph, Keyhive-shaped).** Track A satisfies unforgeable-grant and attenuation natively but has
  no native revocation, so R4 is met via bounded expiry (revoke = decline-to-renew) and R5 via per-scope
  epoch keys; revocation latency is bounded by the expiry interval. Track B makes removal and
  re-encryption first-class convergent operations (stronger revocation immediacy) at materially higher
  complexity and a dependency on research still in flight. Both satisfy R1/R2/R3/R6 identically, so the
  state-reset-avoidance guarantee does not depend on the choice — only revocation immediacy does. The
  choice is deferred to the richer-access-control phase, decided on whether expiry-bounded revocation is
  operationally adequate for real expulsion cadence. **[confirm before publish — Meadowcap/Keyhive claims]**
- **Co-signed op vs proposal-plus-votes** for membership authority — the self-certifying co-signed k-of-n
  bundle is canonical (one broadcast, validated locally against the current epoch); proposal-plus-votes is
  an optional deliberative mode, not built for v0.
- **Reverse-topological-power ordering (Matrix-style)** for governance resolution — rejected: folding sender
  power into the comparator produced an apparent-cycle objection in that protocol's own review, and a
  timestamp tiebreak is clock-gameable. Drystone keeps power out of the ordering spine entirely (§7.5.2).
  **[confirm before publish — Matrix State Resolution claims]**

## Appendix B. Open Questions (forming; not weakening the normative sections)

These are known-incomplete and tracked so they are not mistaken for settled:

- **Vendor-neutral naming reconciliation.** The reference implementation's signed domain-separation tags
  use the historical `croft-*` namespace (§4.2). Drystone, the protocol, requires a versioned
  domain-separated tag but should define a vendor-neutral namespace (`drystone-*`). Because the tag is
  signed over, the rename is a real wire change that re-opens the §4 signature proofs — it must be defined
  and re-proven, not silently swapped.
- **Hash-function reconciliation.** The proven message/history layer (§4) is green-real on **SHA-256**; the
  designed governance-resolution layer (§7) specifies **BLAKE3** for canonical fact content-addressing. The
  single suite the production profile commits to (and any transition) must be pinned.
- **`ENABLING` wire encodings** (gate a publication-final DOI): canonical governance-fact byte encoding
  (§7.3.1, the base all others extend); frontier-commitment construction and acceptance-record format
  (§7.5.1); **frontier-closure-before-sort** (§7.5.2, the highest-risk divergence point); the gating-vs-read
  relationship (§5.7.1); the capability/membership-graph wire format (gated on the Track A/B decision).
- **Root-authority succession** (§7.3) — deferred to a Lifecycle section; the most dangerous operation in
  the system.
- **The two open rights checks** (§5.3): is `share` fully a right or partly a membership-class capability;
  does the §7 survivor/re-key path strand `tenure`. Both gate freezing the four-rights closed set into
  normative text.
- **What grounds a peer's authority — and what makes a right cost something to violate?** §3.1/§5.2 define
  a peer as a locus of adjudication, but in a system with no central allocator, peerhood-as-authority must
  bottom out in something. Three candidate groundings, implying *different* enforcement primitives: (1)
  **cryptographic-fact** authority (the protocol cannot route around a keyholder) — self-enforcing, but
  only covers key-shaped domains; (2) **consensus-conferred** authority (others agree to respect it) —
  circular, and a peer can be demoted to a sensor by collective non-recognition with no topological change
  (the silent-rot path; needs the very enforcement the design avoids); (3) **exit-backed** authority (a
  peer holds authority insofar as it can credibly leave and the system degrades without it) — ties
  authority to variety and needs *legibility of exit*. The working definition to pressure-test: *a peer is
  a locus whose adjudication others must respect because the cost of not respecting it is borne by them,
  not only by the peer.* The spec needs a companion to "where do decision rights sit": **"what makes those
  rights cost something to violate?"** — without it there is no early detector of the peer→sensor rot. This
  couples to the wolf test and the §5.8 exitability backstop.
- **External-fact confirmation** (§7, and the Beer/Cybersyn/OGAS grounding in Part 1 §3): the Matrix /
  Willow / Meadowcap / Keyhive comparisons and the Beer quotes / Cybersyn-OGAS history are web-verified in
  source dialogues only and **[confirm before publish]** against primary sources before they harden.

## References

**Normative:** BCP 14 (RFC 2119 / RFC 8174); the signature and hash suites of the committed wire profile
(§4.1); QUIC (RFC 9000) and the iroh transport (iroh `1.0.0`, FACTCHECK SoT); RTP-over-QUIC for media (§6.3).

**Informative (and [confirm before publish] where load-bearing):** the Willow data model (namespace /
subspace / path; range-based set reconciliation; authorized-write hook); Meadowcap (delegated capabilities,
attenuation by subsetting) and Keyhive (convergent capabilities / membership graphs) as the two capability
tracks; Matrix State Resolution v2 and the room-creator-privileging lesson as the rejected mechanism whose
failure modes motivate §7; Sigstore countersigning as the sign-over-prior-state pattern adopted for the
frontier commitment (§7.5.1).
