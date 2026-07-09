# Drystone delivery layer: architectural design

`Status: design, for folding into Part 2`

`Realizes: P-Local-Truth, P-Knowable-Truth, P-Peer-Equality, P-Durable-Enablement (Part 1)`

`Depends on (Part 2 spec sections): Part 2 §3.1 (peer-to-peer vs center-free), Part 2 §5 (identity, capabilities, communal namespace), Part 2 §7.3.1 (timestamp-free ordering), Part 2 §7.4 (freshness, catch-up). This doc has its own §7.1 to §7.5; references to Part 2 are always written "Part 2 §X".`

`Substrate: MLS (RFC 9420 / RFC 9750), iroh core 1.0`

`Functionality validated against the real libraries (iroh 1.0.1, iroh-gossip 0.101.0, mls-rs 0.55.2); the experiment plans and results are documented separately (05, 10).`

---

## Terms and definitions

This document self-defines its vocabulary rather than deferring to Part 1 or Part 2, so that the merge into Part 2 can reconcile any discrepancy against a stated definition. Where a term is inherited, the definition below is *this document's working meaning*; if Part 1 or Part 2 differs, that difference is for the merge to resolve, and the source of inheritance is noted.

**Inherited vocabulary (from Part 1 / Part 2):**

- **Persona.** The human proxy: the entity that occupies the social-utility adjudication and escalation layer, and that carries **one weight** in a Group (weight-one is the correct and enforced unit). Cryptographically, a persona is represented by the **root key pair of a lineage**; its devices are leaves descending from that root. A persona is not a node or a device; it is the party a set of devices speaks for. (Part 1.)

- **Principal.** A permission-holding entity: anything that holds a permission set. This is an authorization role, not a synonym for "person," so several kinds of thing are principals, a persona (via its root key pair), a Group, a device, or a helper node such as a push-notify host (which is a principal because it holds credentials). A persona is one *kind* of principal. Used here to mark a distinct permission-holding party, as in metadata leaked "to a separate principal." (Concept from Part 1; stated here as this doc's working meaning.)

- **Group.** The entitlement unit: the set of members who share the sealing keys for a conversation and are thereby entitled to read its content and to reconcile its history. "Group" (capital G) is the concept; MLS is its current realization (§2.3). The routing/entitlement split (§1.1) turns on the Group being the entitlement population, distinct from the larger carrying population.

- **Membership.** The property of belonging to a Group: holding its keys, entitled to read and to reconcile. Membership, not position on the fabric, is what authorizes reading and reconciliation. A member is present in the Group as one or more leaves.

- **Leaf and leaf key.** A **leaf** is a single device's position in a Group; one device is one leaf, and a persona with several devices holds several leaves, all co-equal (no hierarchy among leaves). A **leaf key** is the key material a leaf holds; holding the Group leaf key is what lets a device decrypt the Group's content. Leaves descend, by lineage, from the persona's root key pair. (Realized by MLS's ratchet tree, RFC 9420.)

- **Epoch.** The state of a Group between two membership or key changes. Content is sealed to an epoch; advancing the epoch (an add, remove, or update) rekeys the Group, which is what delivers forward secrecy and post-compromise security. (Realized by MLS, RFC 9420.)

- **MLS mechanics: PrivateMessage / Welcome / Authentication Service (AS) / Delivery Service (DS).** These are the current realization's terms (§2.3). PrivateMessage is the sealed application message. Welcome is the message that admits a new member to an epoch. The AS is the trusted identity authority (validates credentials); the DS is the *untrusted* store-and-forward-and-ordering service, assumed able to drop, delay, or observe metadata but not to forge or decrypt. Drystone keeps the DS's store-and-forward role (as the meer) and declines its ordering role (§3b). (MLS, RFC 9420 / 9750.)

- **Causal fold.** Part 2's single, clock-free ordering structure: a partial order of records linked by what-followed-what, with concurrent branches first-class (not forced into a line). Both governance and the dataplane read it, differently (§7.4). (Part 2 §7.3.1.)

- **(G, D) cursor.** Part 2's catch-up position, a pair: **G**, a position in the governance history, and **D**, the dataplane position. This document's §7.5 establishes that the D half *is* the set of per-author high-water marks (the same object as the gap-detector), and the G half tracks governance acts. A node polls or reconciles "since this cursor." (Part 2 §7.4.)

- **EndpointId.** iroh's stable cryptographic identity for a node (its public key), used to address it; the IP is an ephemeral hint that can change. (iroh.)

- **Topic.** The routing selector on the overlay: the identifier the fabric uses to carry a message to a group's subscribers. Readable by carriers (routing requires it); it is the minimum routable selector, with everything else sealed (§1.2).

**Vocabulary this document defines:**

- **Delivery Fabric.** The carrying population: a blind, content-agnostic overlay (gossip, direct links, relays) that moves sealed messages. Larger than and overlapping the Groups that run over it; carriers see ciphertext and routing metadata at most. Distinct from the entitlement population (the Group). (§1.1.)

- **Meer.** A blind store-and-forward node that holds sealed bytes for offline recipients, never holds a key, and is revocable and redundant. Drystone's instantiation of the store-and-forward half of the MLS DS (§3b).

- **Relay** (vs **gateway**). A *relay* is an iroh-layer handoff-and-tunneling helper that endpoints deliberately use to hole-punch or tunnel; it forwards by EndpointId and sees an EndpointId pair only until a direct connection forms. A *gateway* is an IP-layer topological chokepoint (NAT, uplink, ISP egress) that packets traverse; it sees encrypted packets between ephemeral IPs and no iroh-layer identity. The two are different observers at different layers (§1.2).

- **The three planes, and their prefixes.** A delivery arrangement is built from three independent axes, each a question with its own named answers, distinguished by a one-letter prefix:

  - **Carriage (C-):** by what path does a message travel? **C-direct** (direct dial), **C-swarm** (the gossip overlay: a broadcast along a self-healing spanning tree built over the swarm of topic subscribers, not a flooded mesh), **C-relay** (via an iroh relay). This is the Delivery Fabric's job (§1.1). (§3a.)

  - **Durability (D-):** where do the sealed bytes persist for a recipient who is offline now? **D-self** (participants queue and retry), **D-meer** (a meer stores them), **D-peer** (members re-serve held history). (§3b.)

  - **Presence (P-):** who learns a message exists and prompts the fetch? **P-none** (recipient polls), **P-gossip** (a carrier takes a blob's arrival as a wake signal and discards it), **P-meer** (the holder pokes), **P-push** (a byte-free wake to a sleeping mobile device). (§4.)

- **Planes are independent axes, but some points are fused by construction**, and the doc flags which. The planes are paired freely (any carriage with any durability with any presence) except where one mechanism inherently serves two at once: **D-self** is also **C-direct** (participants who hold the buffer also deliver it, one act on two planes), and **the meer** is one node on three planes (it persists as D-meer, you carry-fetch from it, and it can poke as P-meer). **C-swarm**, by contrast, is fused with *nothing* durable: gossip carries but keeps no replay log, so it has no durability-plane partner and an offline recipient needs a separate D- source. Naming the fusions is what keeps "these two planes happen to be one mechanism here" distinct from "these are independent choices."

- **The three resource-asymmetry roles.** **Relay** (reachability), **meer** (storage), **push-notify** (wake). All blind, revocable, redundant, never authorities. (§5.)

- **Gap-aware history convergence.** The one mechanism behind C-swarm hole-detection, D-peer, and device-group sync: detect a nameable gap (via the per-author high-water mark) and fill it from a self-verifying source (via range-based set reconciliation). (§7.5.)

- **Device group.** A persona's own devices formed into their own Group to reconcile history, an ordinary first-order Group distinguished only by lineage-restricted admission and single ownership (§6).

---

## Verification legend

*Verified*: checked against a primary source.

**[confirm]**: load-bearing and to be confirmed against the cited primary before normative text.

*Synthesis*: the claim is the design's own reasoning, labeled as such.

---

## A running example

This document follows one small cast, growing the scenario as each mechanism is introduced, so the trust relationships stay concrete.

**Alice and Bob** are members of a conversation: a Group, each present as a leaf, each holding the leaf key that lets them seal and read the Group's messages.

**Alice has two devices**, a laptop and a phone, each an independent member leaf.

**Carol** is a persona who has network connectivity but is *not* a member of Alice and Bob's Group. Like Alice, she is a persona with her own devices, and those devices are nodes on the network. Some of them may sit on the Delivery Fabric and help carry sealed traffic, but because Carol is not in the Group, none of her devices holds a leaf key for it, so they relay what they cannot read. The difference between Carol and Alice is membership, not structure: both are personas with devices; only one is in the Group.

A **larger group** (Alice, Bob, and others) appears where scale matters.

Every mechanism below is a "now suppose" applied to this cast: Bob goes offline, Bob is on a local network with no internet, Alice's phone missed what her laptop caught, one of Carol's nodes relays a message it cannot read, a member departs. The named roles carry the trust meaning throughout.

---

## 1. What this layer does, and the one idea that organizes it

The job: get a sealed message from its author to every entitled recipient, including recipients who are offline, on a flaky link, or on a mobile platform that forbids a live background connection. This layer sits above MLS (which seals and authenticates content to a Group's epoch) and above iroh (which carries bytes between endpoints).

The organizing idea is to stop treating "how a message is delivered" as a single choice and split it into three independent questions:

- **Carriage (Plane C):** by what path does the message travel from author toward recipient? A direct dial, the gossip overlay, or via a relay.

- **Durability (Plane D):** when the recipient is not reachable right now, where do the sealed bytes persist so they can be pulled later?

- **Presence and wake (Plane P):** who learns that a sealed message for a given recipient exists, so they can prompt that recipient to fetch it?

A delivery "model" is then a *pairing*: one carriage path, one durability source, one presence source. The three planes are independent axes, and within each plane the sources are non-exclusive, more than one can be active at once, racing. This is what matters about the split: it lets a gossip overlay and a store-and-forward node be *combined*, each doing the job it is good at, instead of being treated as rival whole-system designs you must choose between. When Bob is briefly offline, the swarm may carry the message live (carriage) while a meer holds the durable copy (durability) and a wake nudges his phone when he is reachable (presence); three answers to three questions, not one coupled decision.

Keeping the planes separate is what fixes a confusion that a flatter model invites: the gossip swarm is a *carriage* path that happens to provide no durability, and only by separating carriage from durability can we say that plainly (the swarm carries; it does not persist) instead of filing it as a durability option and then apologizing that it is not durable. Some points *are* fused by construction, one mechanism serving two planes, and the doc flags those where they arise (D-self is also the direct-carriage path; the meer persists, is carried from, and can poke). The discipline is to say when a combination is forced by construction and when it is a free choice.

### 1.1 Two populations: who carries, and who may read

A second separation, orthogonal to the planes, organizes the rest of the design: the set of nodes that **carry** sealed messages is independent from, and can be larger than, the set **entitled to read** them.

- The **Delivery Fabric** is the carrying population: a blind, content-agnostic overlay (gossip, direct links, relays) that moves sealed messages around. It can be larger than any one group and can overlap many groups, a shared carrying commons. A node on the fabric sees ciphertext and routing metadata at most, never content. This is where Carol's devices can sit: nodes that carry, holding no key.

- The **entitlement population** is the Group: who holds the leaf keys to read, and who may be asked to reconcile history. Alice and Bob live here. Membership, not position on the fabric, is what authorizes reading and reconciliation.

This matters because it lets the carrying network scale for robustness without ever widening who can read. A larger fabric means more paths, more redundancy, better reach to a hard-to-reach recipient, and none of those extra carriers gains any ability to read, because reading is gated by holding the Group leaf key, not by fabric position. One of Carol's nodes can relay for Alice and Bob all day and learn nothing, because Carol is not in their Group, so her devices hold no leaf key for it.

The sealed message is the single object that crosses between the two populations, and it does three jobs at once: it is **payload** for those entitled to read it, **delivery** for its recipient, and **gap-definition** for an entitled holder who reads it, because inside the seal it carries its author's signed sequence index (§7.5). One artifact, three jobs. This is why the design needs no separate "a message exists" announcement channel: the message's own arrival is the announcement, and its index, read by members, is the record of what should exist.

This decoupling is not new. Hyperledger Fabric ships the same shape (membership-scoped channels riding a larger gossip overlay), and the epidemic-dissemination lineage goes back to Demers et al. 1987. What Drystone does differently is keep it center-free in ordering and governance, and make entitlement cryptographic rather than a routing policy. The provenance companion (`09`) traces the full lineage and is honest about where the novelty is and is not.

The decoupling has a metadata consequence worth treating on its own, because separating carrying from reading is precisely what determines who can observe what. That is §1.2.

### 1.2 What observers can see

The routing envelope and the sealed interior are different, and the difference bounds the entire metadata exposure. For the fabric to carry a message to a group's subscribers, the **group/topic identifier must be readable**. This is not a Drystone concession; it is the irreducible floor of *any* routed delivery system, a flat server reads recipient addresses, a federated server routes by domain, a broadcast overlay reads a topic. You cannot route to a destination you cannot name. The honest position is that Drystone exposes the *minimum* routable selector (a topic identifier) and seals everything else; the only way to expose less is to route every message to everyone, which general messaging does not ship.

The **author identity and the author-signed sequence index are sealed inside the payload**, readable only by members after decryption. This ties two claims together: because the index is sealed, a member reads it to detect gaps (§7.5), and a blind carrier, including a meer, cannot read it to order or attribute the blobs it holds (§3b). "The meer cannot order" and "a carrier cannot infer per-author gaps" are one consequence of one decision, the index lives inside the seal.

The adversarial picture is best drawn by observer, because they sit at *different layers* and see different things, and a single attacker rarely occupies more than one.

- A **swarm member** observes at the **overlay layer**. It is a subscribed node forwarding along the broadcast tree, so it sees the traffic through *its* part of that tree: the topic identifier and the sealed blobs on it. An epidemic overlay gives no node the global roster or flow, so this is a *partial, local* picture: the shape of activity in its neighborhood (volume, timing, sizes, distinct-blob counts via the content-hash dedup key, §3c). It knows *which group* but sees only its slice, and it is content-blind (sealed) and attribution-blind (author and index sealed).

- A **relay** observes at the **iroh layer**, as a handoff-and-tunneling helper the endpoints deliberately use to hole-punch or to carry traffic when no direct path exists. iroh's relay forwards datagrams addressed to a destination EndpointId, so it can see that one EndpointId is talking to another and how many bytes pass, but, by iroh's own account, only for as long as the two endpoints have not yet established a direct connection, and a direct hole-punch succeeds roughly 90% of the time (§2.1). *Verified (iroh FAQ / crate docs).* So the relay sees a graph of **durable EndpointId pairs, but transiently**: it drops out of the path once a direct connection forms, leaving only the pre-hole-punch window and the residual fraction of pairs that never go direct. Even then it sees the *pair and byte count*, never the topic (sealed in QUIC) and never the content (MLS sealed inside that). The relay is a chosen helper, not a network position.

- A **gateway** observes at the **IP layer, below iroh**, as a topological chokepoint that packets traverse because of where it sits (a NAT, a mesh uplink, an ISP egress). It sees encrypted QUIC packets between **IP addresses**, and nothing above that: not the EndpointId (an iroh-layer identity inside the connection, not an IP header), not the topic, not the content. Its graph is over **ephemeral IPs**, which rotate with mobility and NAT, are shared behind CGNAT, and are reassigned, so it does not durably attribute to a persona or device and it decays as addresses churn. This is the same weak handle any network observer has on any encrypted traffic, the universal floor, not a Drystone-specific concession.

These do not combine cheaply, and they fail in opposite ways: the relay sees durable identities but transiently and pair-only, blind to group and content; the gateway holds a persistent position but sees only churning, identity-blind IP flows. The frightening observer, persistent *and* identity-bearing *and* group-aware, would have to be the relay for a pair that never hole-punches *and* a swarm member at that group's tree positions *and* correlate the two across layers, which a healthy redundant swarm and the 90% direct-connection rate actively work against.

One further, separate surface is the **discovery** layer: resolving an EndpointId to its current address (Pkarr/DNS, n0-hosted under the default preset) tells the resolver that someone looked up EndpointId X. *Verified (iroh-dns primary).* This is distinct from both relay and gateway, and it is the soft-center a deployment can opt out of (run a private resolver, or distribute addressing out of band). It is named here because it is the one place the default deployment leans on an n0-operated service for convenience.

All of this is out of scope for what *this* layer undertakes to defeat: Drystone seals content and in-payload metadata (author, index, gap structure) and routes on the minimum selector (topic). Transport-level and discovery-level metadata are addressed, if at all, by lower-layer countermeasures (a private relay and resolver, traffic-shaping such as padding, cover traffic, or mixing), which this design notes rather than provides.

So the strongest honest summary: a swarm member can build a partial view of the *shape* of a Group's activity, never its content and never its attribution; broader views require a chokepoint position that a redundant fabric is designed to prevent.

---

## 2. Substrate facts this design rests on

Each subsection states the capability the layer *requires*, in technique-neutral terms, then names the current conforming implementation with its verified specifics. The requirement is the durable thing the spec commits to; the named implementation is one satisfying instance, replaceable without disturbing the requirement. The first three (transport, overlay, content sealing) are genuine requirements; the fourth (mobile wake) is a conditional accommodation for one class of node, called out as such rather than dressed as a requirement. Items subject to version drift carry **[confirm]**.

### 2.1 Transport

**Requirement.** The layer needs a transport that moves bytes between two endpoints over the open internet, attempting a direct peer-to-peer path first and falling back to a relay that forwards without reading. Endpoints must be addressed by a stable cryptographic identity rather than by a network location, so that a device remains reachable as its address changes, and the relay fallback must carry only ciphertext, so that reachability help never becomes a reading capability.

**Current conforming implementation: iroh core 1.0** (reached 1.0 on 2026-06-15, wire-and-API stable for the core surface: Endpoint, Connection, Router, the ALPN handler pattern, QUIC over UDP with TLS 1.3, relay, key-based addressing). *Verified (n0 primary).* Devices are addressed by an endpoint public key; the IP is an ephemeral hint. A direct QUIC hole punch is tried first (succeeding roughly 90% of the time), with a stateless encrypted relay as fallback that routes encrypted packets and does not see content. *Verified.* The endpoint key is Ed25519, 32-byte. *Verified against iroh-base 1.0.1.*

### 2.2 Overlay

**Requirement.** The layer needs an epidemic broadcast overlay: a swarm in which nodes forward each message to their neighbors along a self-healing spanning tree, so that a message injected anywhere reaches every currently-live subscriber, without any node holding global membership and without a central broadcaster. Two capabilities follow from that. A **membership** mechanism that keeps each node a small, self-repairing set of neighbors (so the swarm survives churn without anyone knowing the whole roster), and a **broadcast** mechanism that pushes content along the tree while lazily repairing breaks (so delivery is both cheap and resilient). The overlay carries sealed bytes only; it is one population of the Delivery Fabric (§1.1) and is blind by construction.

What the layer does *not* require of the overlay: durable storage (the swarm keeps no replay log, which is why a fully-offline node recovers nothing from it, §3a), and a separate content-free "a message exists" signal riding the overlay itself (a node participating in the swarm already sees each arrival, so presence for that node rides the arrival of the sealed bytes, §4). Stating these non-requirements matters because they are where a naive overlay design would over-build a second announce channel. Note the scope carefully: this says the *overlay* needs no presence channel for nodes *in* the swarm. A node that has dropped out of the swarm (a backgrounded phone the OS has disconnected) is a different case, handled not by the overlay but by an external wake (§2.4), and that wake is a conditional accommodation, not an overlay feature.

**Current conforming implementation: iroh-gossip**, a separate pre-1.0 crate (not covered by the iroh core 1.0 stability guarantee) whose current version builds and runs against stable iroh 1.x. *Verified.* It realizes the two capabilities with **HyParView** (the membership mechanism: a small active view for dissemination, a larger passive view for repair) and **PlumTree** (the broadcast mechanism: eager push along the tree, lazy hash-announce along the rest for repair). *Verified (crate docs, citing the two papers).* Default HyParView views: active size 5, passive size 30. **[confirm: version-dependent.]** Treat gossip internals as version-dependent; the requirement above is the stable thing, iroh-gossip is the current way to meet it.

### 2.3 Content sealing

**Requirement.** The layer needs content sealed and authenticated to a group of entitled readers such that only current members can read or forge, the seal is independent of transport (so any blind carrier can move it), and the scheme provides forward secrecy and post-compromise security across membership changes. Entitlement must be a cryptographic property of holding a key, not a routing or policy decision, this is what makes a blind carrier safe rather than merely trusted to behave. The scheme must also validate member credentials against a trusted authority at the moment of a membership change, and must permit an application policy for treating several credentials as one user's devices.

**Current conforming implementation: MLS** (RFC 9420 / 9750). MLS realizes the Group, its Membership, and leaf-keying, and MLS PrivateMessage provides end-to-end confidentiality, integrity, and authenticity to the Group's epoch, plus forward secrecy and post-compromise security, independent of transport. *Verified.* MLS assumes a trusted Authentication Service but a largely untrusted Delivery Service: a DS can drop, delay, or observe metadata, but cannot forge or decrypt. *Verified (RFC 9750).* This is precisely the "carrier is untrusted by assumption" property the requirement names, a Drystone meer or fabric carrier is just an untrusted DS. Credential validation happens member-side, when a member processes an Add, Update, or external join, validated against the AS. *Verified (RFC 9420 §5.3.1).* RFC 9750 explicitly leaves two things to application policy: when a member may add or remove others, and the treatment of several credentials as one user's devices:

> a policy for when two credentials represent the same client ... when there are multiple devices for a given user

*Verified (RFC 9750).* This is the hook the device-group lineage check uses (§6), so multi-device-same-user is a designed extension point, not an improvisation.

### 2.4 Mobile wake (a conditional accommodation, not a requirement)

Unlike §2.1 to §2.3, this is **not** a requirement of the design. It is a conditional accommodation for one class of node. Most nodes participate in the overlay and see arrivals directly (§2.2), so they need no wake. But some nodes, backgrounded mobile devices in particular, cannot hold constant presence: the OS forbids a live background connection and drops them from the swarm. For that class, an **external** wake (distinct from any overlay presence signal) lets the device come back and fetch.

**Requirement, when it applies.** For a node that cannot maintain presence, the wake must be able to carry nothing of substance (the design uses it only as a content-free nudge), because the wake channel is operated by a platform vendor the design does not trust with content, and because the channel is throttled and unreliable enough that no design should depend on it carrying anything. This is why the wake is an accommodation and not a requirement: it is unreliable by nature, so it can only ever be an *optimization* over "catch up on next foreground," and a polling fallback (P-none, §4) always suffices without it. Optional by necessity, in both directions: beneficial for a resource-constrained node, and never depended upon by the design.

**Current conforming implementation: APNs and FCM**, the platform-mandated push services. Payload cap is 4 KB (Apple); FCM matches at 4096 bytes and states its transport is not end-to-end encrypted, so applications must supply their own E2E. *Verified (Apple, Firebase primaries).* This is exactly why content sealing (§2.3) is what makes riding a push provider safe: the provider carries, at most, a content-free wake. Silent/background push is throttled and not guaranteed, Apple's guidance is no more than a few per hour, dynamic limits, and possible silent non-delivery even after APNs accepts the request. *Verified (Apple primary).* **[confirm: only "a few per hour, dynamic" is pinnable; specific secondary numbers conflict.]** The reliable channel is the user-facing alert; the invisible silent channel is the throttled one, which is exactly why the wake is only an optimization: wake-and-fetch degrades correctly, and the durable fetch path (§3) exists regardless.

---

## 3. Carriage and durability: getting Bob's message there, and holding it until he can fetch

Two planes live here, kept separate because they answer different questions. **Carriage** (§3a) is the path a message travels; **durability** (§3b) is where it persists for a recipient who is offline. They are paired freely, except where one mechanism serves both by construction, which the text flags. Within each plane the sources are non-exclusive and the selector (§8) may race them.

### 3a. Carriage (Plane C): the path the message travels

The Delivery Fabric (§1.1) offers three carriage paths. None of them, by itself, persists anything for an absent recipient; persistence is the durability plane's job (§3b). Carriage is about reaching whoever can be reached now.

**C-direct.** Suppose Alice and Bob are both present, on the same network, no internet needed. They exchange MLS messages directly over QUIC, no helper in the loop. This is the most center-free path in the design and is first-class, not a degraded mode. It is also the path half of the no-helper floor: when participants carry their own traffic directly, carriage and durability are *fused by construction* into D-self (§3b), one act that both delivers and (with queue-and-retry) persists.

**C-swarm.** Now suppose Alice and Bob are in a larger group on a local network with no internet, or simply want live fan-out. The gossip overlay carries each message across the **swarm** (the population of nodes subscribed to the topic) along a **spanning tree** built over that swarm: a message is eager-pushed along the tree's edges and the tree self-repairs as nodes come and go. It is not a flooded mesh; the tree is what keeps the broadcast cheap. This is genuine, efficient delivery, and the right tool when there is no central store or a Group deliberately wants none. Its nature, stated plainly because separating the planes lets us say it without contradiction: gossip carries but does **not** persist. It keeps no replay log, so a node entirely offline during a message's live window recovers nothing from the swarm itself. *Validated: an online node received every broadcast; a late joiner recovered none.* C-swarm is a carriage path with no durability-plane partner of its own, so an offline recipient needs a separate D- source (a meer, or peer reconciliation) to recover what the swarm carried past.

That a Group may run on C-swarm with weak or no durability is a **legitimate choice** under the Part 1 §2.3 dial discipline, to avoid internet-link requirements or to tilt away from any central store (P-Durable-Enablement, and the field-integrity concern of §2.6). The one non-negotiable: a Group may accept message **loss**, but the protocol must never allow **invisible** loss. Loss-tolerance is a Group dial and may be loose; loss-*visibility* is floored by P-Knowable-Truth (Part 1 §2.2) and the freshness rule (Part 2 §7.4). The mechanism that keeps loss visible even with no store to query is gap-aware history convergence (§7.5): each message's sealed signed index lets a member see precisely what it is missing from the messages it does hold. A swarm-only Group leans entirely on that, which is why the index is load-bearing rather than optional.

**C-relay.** When a direct path will not form (NAT, mobility), an iroh relay forwards the encrypted packets so carriage still succeeds (§2.1). The relay is a carriage assist, content-blind; what it can observe and for how long is §1.2.

### 3b. Durability (Plane D): where the bytes wait for an offline Bob

Three sources, presented most-center-free outward, because the floor is the point: everything above it is an optimization, never a dependency.

#### D-self: the floor

Suppose Bob steps away mid-conversation. Alice, or any node on the path, buffers the sealed bytes and retries when Bob returns. Durability is supplied by the participants themselves. Paired with C-direct this is the fused no-helper cell (one mechanism carrying and persisting), and it is the path Part 1 §2.4 requires to stay real and routinely exercised: a conversation can happen on bare nodes with no purchased infrastructure (P-Durable-Enablement). Everything else in this plane is an optimization over D-self, and naming it the floor is what lets every other source be removed without ending the conversation.

#### D-meer: the default optimization

Now suppose Bob is offline for hours, and Alice does not want to stay online holding his mail. A **meer** is a blind store-and-forward node that holds the sealed bytes for Bob and hands them over when he returns. It is never given a leaf key, so it stores ciphertext and learns nothing; it is revocable and redundantly held, never a structural dependency. It is Drystone's realization of the store-and-forward half of MLS's Delivery Service. One well-connected meer can fan out to many recipients, cheaper than many direct dials, so this is the sensible default for an internet deployment. (The meer is one node on three planes: it persists here as D-meer, you carry-fetch from it, and it can poke as P-meer (§4); the three roles are kept distinct on purpose.)

The meer does store-and-forward but does **not** order messages, and this is worth being precise about because it is easy to misread as a sacrifice. It is not. Ordering is not given up; it is sourced elsewhere. A conventional delivery service orders by acting as a central sequencer that clients must trust. Drystone refuses that, because ordering is already carried *inside the messages*, in the author-signed causal structure and per-author index (§7.4), sealed in the payload where only members can read it (§1.2). So the meer has no ordering job to do, and in fact *could not* do it: the index it would need to sequence the blobs is sealed against it, the same sealing that lets a member detect gaps denies the meer the ability to order. Being blind and not ordering are the same fact, and that fact is the sealed index. Order is needed, present, and intrinsic; it is simply not the meer's to provide, nor within its power to provide.

Because the meer holds only ciphertext, every MLS guarantee holds against it exactly as against any untrusted DS. *Verified (RFC 9750).*

#### D-peer: the Group as its own replay buffer

Now suppose Bob was offline for a message that Alice caught, and there is no meer, but Alice is reachable. Bob recovers it from Alice directly. Members are already entitled to all the Group's messages, so two members reconciling their held history leaks nothing across the entitlement boundary and adds no new reader. Because a real Group has many members, the Group becomes its own distributed replay buffer: if anyone caught it, anyone else can recover it, with no central store. This is the same reconciliation machinery the device group uses (§6, §7.5), pointed at fellow members.

D-peer is sound only under hard invariants, each of which keeps the convenience from leaking trust into authority (Part 1 §2.0 razor, §2.3, §2.5):

- **Self-verifying records only.** A member accepts a reconciled record only when it verifies on its own author signature and folds into the hash structure, never on the partner's say-so. Reconciliation moves provenance-carrying records, never derived or asserted state.

- **Dataplane only, never governance.** Reconciliation carries message history. Governance state (who is a member, the current epoch) is read from the member's own authoritative MLS view, never accepted as a peer's assertion.

- **Corroboration is alignment, not truth.** That several members hold the same record affects *coverage* (how likely you are to find it), never *validity* (which is the signature's job alone). Treating a holding-count as a truth signal would build the quorum apex the razor forbids.


- **Current membership only, exit is final.** A member may reconcile only the history of the Group it is currently in; departure ends eligibility. This is the Part 1 §2.4 exit right read correctly, you leave *with* the history you hold, and gain no standing to pull more afterward, which dissolves the former-member history-pull attack surface rather than having to defend against it.

Participation is **voluntary per member**: acting as a sync peer is a service a member opts into, not a duty the Group imposes. The scope sets whether D-peer is allowed at all; within that, members enroll individually. The one cost D-peer carries that the blind roles do not is that a sync partner is *not* blind, reconciling reveals a coarse activity pattern to a fellow member, so whether D-peer defaults on is a per-scope threat-model dial (on for small high-trust scopes, opt-in for large contested ones).

### 3c. Racing the sources, for free

Because an MLS message is byte-identical no matter which path carried it, the same message arriving by two routes is recognized as one and deduplicated on its content hash. So Bob can have several paths and sources attempting at once across both planes, C-direct, C-swarm, D-meer, and D-peer, first delivery wins, duplicates drop. This is the maximum-robustness posture and it costs nothing in principle: a message the swarm carried past Bob's offline window is one the meer still holds; a message one member withholds is one several others are corroborated to hold; a message the meer is slow on may arrive first over a live swarm path. *Validated: one seal relayed by two paths dedups to a single entry; the selector delivers exactly once down to the D-self floor.*

---

## 4. Plane P, presence and wake: who tells Bob's phone to look

Durability decides where Bob's mail waits. Presence decides who tells Bob it is there. The recipient's situation sets which applies.

- **P-none.** Nobody but sender and recipient. Bob's device catches up on its own schedule, foreground, periodic fetch, manual refresh, by polling a meer with its cursor ("anything since this position?"). No wake mechanism and no device-token binding, the most center-free presence posture, and always available as the floor.

- **P-gossip.** Suppose one of Carol's devices is a fabric carrier on the topic, though Carol is not a member. The sealed message arrives at that node via gossip; it makes no attempt to read it (Carol holds no leaf key for the Group), takes the bare fact of arrival as the signal "a message exists for this Group," fires a content-free wake toward Bob, and discards the bytes. The node read nothing and stored nothing, yet Bob's sleeping phone gets nudged. This works on the stock gossip layer with no extra channel, because presence does not require a content-free signal: holding an unreadable blob *is* a perfectly good arrival signal, and the carrier simply does not decrypt it.

  This is why the design wants no separate "announcement" channel, and the reasoning is a correctness point, not a preference. Splitting "a message exists" onto a second channel from "here is the message" creates a window where the two can disagree, an announcement with no message behind it (a node stuck waiting), or a message with no announcement (a node that thinks nothing arrived). That is the classic time-of-check-to-time-of-use race, the same reason it is safer to attempt an operation and handle failure than to check-then-act. Letting the message's own arrival be the signal is atomic: there is no gap to disagree across. The cost is that a carrier receives a blob it only needed to know existed and then throws it away, which is a little wasted bandwidth, the cheapest thing in the system to spend, in exchange for eliminating an entire class of race and an entire second overlay to secure. The inefficiency is the clean choice.

- **P-meer.** The meer already holds Bob's mail, so it already knows mail is waiting and can poke him. Operationally the simplest (one node), content-blind like always. It pairs with P-push to reach a sleeping phone.

- **P-push.** A dedicated push-notify node fires a content-free wake to Bob's mobile device via APNs/FCM. It holds no sealed bytes, learns only "Bob has mail, wake Bob," and sends a wake, not content (§5). It is the **actuator** the detectors hand off to when the recipient is a sleeping device that nothing else can reach. *Validated: the wake-then-fetch path recovers all buffered messages even with the wake fully suppressed, so push is a pure optimization, and a guard rejects any attempt to put ciphertext in the wake payload.*

The presence paths are not rivals for one slot; they compose as **detector plus actuator**. Something notices the message exists (P-gossip: a carrier saw it arrive; P-meer: the holder knows it stored it; P-none: nobody, the recipient polls), and something reaches the recipient (P-push wakes a sleeping device; a live device needs no actuator; polling is self-actuated). "A carrier detects, push actuates" is one coherent path. Which detector applies is set by deployment: a meer-backed deployment uses P-meer, a swarm-only deployment (no meer) uses P-gossip, and they are interchangeable where both exist. The actuator is forced by the recipient: only a push can wake a backgrounded phone, and poll-a-meer is always the non-push fallback beneath it.

---

## 5. The three resource-asymmetry roles

Every helper in this design is an instance of the Part 1 §2.3 resource inequality, what a node happens to *have*: storage, uptime, reachability, credentials. All three roles are blind to content, revocable, and redundantly held. None is an authority; none touches any persona's rights floor or weight.

- **Relay** supplies reachability (NAT traversal, hole-punch assist), provided by iroh core. It forwards encrypted packets and, while it is in the path, can see an EndpointId pair and byte counts but never topic or content; what it sees and for how long is detailed in §1.2.

- **Meer** supplies durable blind storage (D-meer). Holds sealed bytes, never a key.

- **Push-notify** supplies the wake signal (P-push). Holds APNs/FCM credentials and a device-token registry; holds no sealed bytes.

### 5.1 Why the push-notify role is designed to be minimal

The push-notify node exists because a backgrounded mobile OS forbids a live connection, and only a push can wake the app. It is deliberately the smallest such role:

It learns only "Endpoint X has a waiting message; wake X," and sends a content-free wake, not the message. Two reasons converge on byte-free. The rights model wants the smallest metadata footprint, so no ciphertext should pass through a third party that does not need it. And the platform forces it anyway: the reliable push is the user-facing alert, the invisible silent push is throttled and droppable (§2.4), so any design that needs the push to *carry* content is fragile, whereas wake-then-fetch degrades cleanly to "catch up on next foreground."

It is doubly removable. A foregrounded app holds a live connection and needs no push host at all (the §2.4 no-helper path, kept real). A polling device pulls from a meer on its own schedule with no wake, needing no token binding. The single irreducible cost is the device-token-to-EndpointId binding, which is inherently identifying, and because polling avoids it entirely, paying it is opt-in: a user who declines push declines the binding. Keeping the host byte-free is both a smaller footprint and a sharper revocation story, removing it costs a wake optimization and nothing else, because durability lived in the meer or in D-self all along.

---

## 6. The device group: Alice's laptop and phone, kept in sync

Now suppose Alice's laptop caught a message her phone missed. Her devices should converge so her history is complete everywhere, and they should do it without a server syncing them. Drystone handles this with a second Group: Alice's own devices form their own Group whose job is reconciling history among them.

### 6.1 An ordinary Group, scoped by lineage

Alice's devices are each, independently and equally, leaves in whatever conversations she is in; MLS treats them as co-equal members, and the design adds no leaf-to-leaf hierarchy, because inventing one would step outside the flat membership MLS's security analysis assumes.

Their reconciliation group is not a special construct or a sub-tier. It is an ordinary first-order Group, distinguished only by *scope*: its admission is lineage-restricted (§6.3), and all its leaves belong to one owner. Same machinery, same security analysis. The right description is a **secondary history-convergence backplane with a stronger membership story**: a backplane, because its job is reconciling history across Alice's devices alongside her primary conversations; stronger, because the bar to join is verifiable cryptographic descent rather than an asserted invitation.

### 6.2 What actually moves, stated precisely

The device group moves **sealed bytes** among its leaves, exactly as every Group does. Its leaves can decrypt what they receive because they hold the leaf key, but that is simply what membership is; readable content exists only locally, after a device decrypts, the ordinary state of any member of any Group. There is no special "plaintext channel," and so nothing extra to secure: the content travels sealed and is read at rest, like all sealed content.

It widens entitlement to no one, which is the entire safety story, and it needs no special machinery. Every leaf is the same owner, Alice, already entitled on each of her devices, so syncing among them moves nothing across any entitlement boundary, there is no boundary between Alice's own devices. A device that is *not* in a conversation has no leaf key for it and so, even handed the sealed bytes, can read nothing. *Validated: two of Alice's co-entitled devices each decrypt the Group's messages and converge; a non-member handed the same sealed bytes cannot decrypt them.* The two convergence invariants that matter, trust a record only on its own signature and on folding into the hash structure (never the partner's word), and reconcile dataplane history only (never governance, which each device reads from its own MLS view), hold here for the ordinary reason they hold for every Group's convergence. *Validated: a tampered record is rejected even from a trusted sibling device, and a forged governance assertion offered over the device channel is ignored, the roster read from the device's own MLS view.*

### 6.3 Admission is where control is worth the most

A device joins the device group like any MLS member, under a lineage-restricted policy by default: the joining leaf must prove cryptographic descent from the persona's rooting key. Part 1 §2.3 already makes lineage the thing that collapses a person's many devices into one persona of weight one, and that descent is verifiable. The reason to put the strong, verifiable check precisely *here* is that a history holder, once admitted, cannot be made to un-read what it received. Admission is the last moment control is fully effective, so it earns the strongest available control, a verifiable proof beats an asserted "this device is mine." *Validated: a non-lineage credential is rejected by the real credential-validation hook at commit-build time, before the commit reaches the network.*

The honest scope limit: the lineage check governs *which devices become holders*; it does nothing about a holder compromised later, and nothing about Alice exporting her own content. It protects the admission decision, not the aftermath, because nothing can protect content already read, and claiming otherwise would be the §2.0 failure of trusting math where the real safeguard was human judgment. The standing defense against a later-seized device is the ongoing ability to cut *future* convergence, even though already-synced history is gone.

### 6.4 A lineage-scoped durability amplifier

Because the device group is an ordinary Group, it is eligible for the full delivery stack, including fabric sync, not only direct local links. This is the durability payoff. Earlier, Alice's devices could only heal each other when near each other; as a fabric-eligible group they reconcile asynchronously, through whichever device comes online next, without ever being co-present. The resulting guarantee, stated with its honest bounds: **if any one of Alice's devices receives a message, every enrolled device of hers eventually sees it.** "Eventually" is eventual consistency, in the limit of reachability, not instant and not under permanent partition, and each device always knows from its high-water mark what it still lacks, so the gap stays visible while it persists (§2.2). "Enrolled" is a voluntary per-device choice, so the spread is Alice's decision, never automatic to every device that ever touched her lineage.

This is safe for the same reason all device-group sync is safe: the carriers are blind, and every leaf is the same already-entitled owner, so fabric-wide convergence exposes nothing a smaller-scope sync would not. Its one added cost over local-link-only sync is the metadata any Group's fabric traffic exposes, that *some* lineage is reconciling, never what. So whether the device group rides the fabric is a §2.3 dial, default on (durability is what most multi-device users want), with direct-link-only as the tightening for a user who would rather their device sync never touch the fabric.

The device group is, in the end, the tightest-scope instance of the same gap-aware convergence as D-peer (§3b, §7.5), a single owner's lineage. The only differences are in the user's favor: no metadata leak to a separate principal (the partners are her own devices), and the lineage-governed admission above. Reconciling across both her own devices and other members simply adds independent sources.

---

## 7. Catch-up: ordering and convergence

### 7.1 Range-based set reconciliation

When two parties need to converge on a shared set of records (Alice's two devices, or two members), they use **range-based set reconciliation (RBSR)**: exchange a fingerprint over a range of the ordered history; if the fingerprints match, that range is already in sync and is skipped; if not, split the range and recurse, exchanging actual records only at the small leaves where they differ. The cost scales with the *difference* between the two sides, not the size of the history, in logarithmic rounds. *Validated: transfer volume stays flat as history grows for a fixed difference, and grows with the difference.* This is modern, range-based anti-entropy, the reconciliation half of the 1987 epidemic lineage (`09`).

### 7.2 The order it reconciles on is clock-free

RBSR needs *an* order to partition, but any clock-free monotonic criterion serves; a wall-clock is merely the obvious (and here, forbidden) example. The reconciliation order reuses the Part 2 §7.3.1 timestamp-free causal fold, so RBSR never smuggles a clock back in. *Verified.*

### 7.3 The backend obligation

RBSR is cheap only if the store can summarize an arbitrary range, split it, and enumerate small residuals without rescanning, a range-summarizable structure (a tree with subtree fingerprints), not a flat log. *Verified.* The lean adopted: a monotonic storage index as the canonical, range-summarizable order, with the wall-clock timestamp kept only as a display attribute, never the sort key. **[confirm: the RBSR construction, Willow 3d-range vs Negentropy, is a Part 2 §5 decision; both descend from the same primitive.]**

### 7.4 One order, read two ways

The Part 2 §7.3.1 causal fold is the single ordering structure, primary and clock-free. Two layers read it differently, and the difference is deliberate:

- **Governance** does not linearize concurrent mutually-exclusive operations; it lets them fork and escalate, because telling a real social dispute from a benign artifact is a human judgment, not a clock's (fork-not-verdict, §2.5). A wall-clock here would manufacture a verdict the razor forbids.

- **The dataplane** treats ordering as a UI service. Two concurrent messages need *some* order to display in, and being "wrong" carries no risk because nothing that converges or resolves authority depends on it. So the dataplane safely linearizes, and RBSR partitions on that linearization.

The tiebreak between concurrent messages is the **content hash of the sealed message**: deterministic, clock-free, already canonical (it is the dedup key of §3c). Every device computes the same hash from the same bytes and sorts identically, with no clock and no coordination; the timestamp is shown only as a human-friendly label. *Validated: the (index, content-hash) key produces an identical total order on independent devices, concurrent messages tiebroken identically, over real sealed bytes.* For items the fold has already declared concurrent there is no true order between them, so an arbitrary-but-consistent one is the honest choice, and the label still gives a human a sense of "around when."

### 7.5 Gap-aware history convergence: one mechanism, three relationships

C-swarm hole-detection, D-peer, and device-group sync are not three features. They are one: **detect a nameable gap, then fill it from a self-verifying source.** This is anti-entropy (Demers 1987) refined to RBSR with a clock-free order. Naming it once is simpler than describing reconciliation three times.

**Detection is the per-author high-water mark, and that mark is the cursor.** Each message carries its author's monotonic, signed index *inside the seal*, so this detection is something a member does after decrypting, never something a blind carrier does in transit (§1.2). The highest index a member holds from Alice asserts, by Alice's own signature, the complete range that should exist below it, so any index in that range the member lacks is a *nameable* gap, "I am missing Alice's #14," not a vague worry. The set of these per-author high-water marks *is* the dataplane half of the catch-up cursor; the cursor and the gap-detector are the same object, not two. A single fresh message from Alice over any path updates the mark and re-establishes the whole expected range, which is why even gossip's lossy, no-replay delivery is useful for detection: one recent message tells a member how much it is missing, even though gossip will not itself replay the rest. This is also why the gossip layer not exposing a separate "exists" signal costs nothing here, detection rides the sealed index read by members, not anything the overlay surfaces.

The honest non-goal: this detects every gap *below a known mark*, but cannot reveal an author never heard from at all, there is no mark to anchor on, and that case is indistinguishable from the dataplane between never-sent, withheld, and partitioned. The UI may honestly show "no messages from Carol" (cheap, true), but the system does not hunt for history that may not exist. Missing *governance* acts are caught separately, by the governance half of the cursor.

**Fill is RBSR of self-verifying records, ordered by metadata leak, not by validity.** A record proves itself by signature regardless of who hands it over, so the only thing that varies between sources is how much the *act of asking* reveals:

- **Own devices first.** Zero leak to any separate principal, often over a local link. Alice's devices heal each other before anyone else is involved, which quietly handles the common case (she was online on *some* device) and leaves only genuinely-everyone-missed-it residue to escalate.

- **Members next.** Bounded leak (a fellow member learns a coarse sync pattern, acceptable since they are entitled anyway), reconciling with several members where possible so no single member can withhold unnoticed.

- **Never the fabric at large.** Carriers are unentitled; revealing your gaps to them would leak to non-members for no benefit, since an entitled member supplies the same self-verifying record. The fabric delivers and defines gaps; it is never a reconciliation partner.

**No transport gate; the gate is membership.** A message arrives over any path and updates the mark, that is pure detection, requiring no trust decision and working for member and blind carrier alike. *Then*, if the node is an entitled member with a gap and the scope allows it, the application looks up an entitled member and reconciles with them, over any transport, and that member need not be on the fabric at all. Whether you reached a partner via the fabric or a direct dial is irrelevant; whether they are a verified member is everything. *Validated: a gap detected from a fabric-delivered message is filled by reconciling with a member reached off the fabric; the records verify source-agnostically, and tampered or forged records are rejected whatever their source.*

---

## 8. The adaptive selector

The selector is the runtime policy that picks a (carriage, durability, presence) combination per recipient per moment and races the non-exclusive sources within a plane. It is not itself a delivery model; it is the control layer above them. It chooses by connectivity and degrades gracefully: a direct local link when both peers are present; swarm, meer, and member-corroboration racing when the recipient is intermittent; meer plus push for a backgrounded phone; poll-a-meer when push is declined. Where D-peer is enabled, it prefers reconciling with several members over one, so no single member can withhold unnoticed (the §2.4 no-single-dependency floor at the sync layer). Every cell has a named no-helper fallback, because §2.4 requires the floor to stay real: it is always D-self with P-none. *Validated: exactly-once delivery holds under every combination of surviving paths down to the D-self floor, and a backgrounded device presents identical, complete, ordered history on waking.*

### 8.1 Payload classes the selector distinguishes

Speed and durability are independent (carriage and durability are separate planes), so a fast path does not imply a non-durable one. The application classifies each payload deliberately; the selector never guesses, and in particular a real message is never silently treated as throwaway (that would be invisible loss, which §2.2 forbids).

- **Live-durable.** Both parties present, low-latency path for a snappy real-time feel, *and* a full signed record that persists into history like any other message. This is the right shape for a real-time chat in a provenance-first system: fast *and* remembered. Real-time here is just fast delivery of durable messages, not a separate ephemeral lane.

- **Intrinsic-ephemeral.** Non-utterance state whose value is bound to the moment: typing indicators, cursor positions, presence beacons. No durability source and no wake engage, because there is nothing to keep and nobody to wake; loss is *correct*, and a suppressed one is a non-event, not a gap. Because these cost battery and radio for zero durable value, each carries a resource-default that can scale with group size, typing indicators, whose value falls and whose cost rises as a group grows, default off above a small-group threshold (working value ~7 to 10, tunable). This is a battery setting, not a privacy or correctness one.

- **Chosen-ephemeral (disappearing messages).** A real, durable message carrying a signed "do not retain past T" disposition that honest clients apply. This is a *retention policy on a normal message*, not a transport mode, and it carries a hard honesty caveat identical to refusing a fake unsend: it is cooperative non-retention, never enforced deletion. A node cannot prove it expunged, and a modified client simply keeps what it read. So it may promise "honest clients stop showing this and drop their copy," never "this message is gone."

- **Self-destruct (time-bound sensitive value).** A distinct payload class (the guest wifi password shown until Monday), whose achievable semantics depend on the history mode and whose strength is bounded by node fidelity, not cryptography. It is the one class that deliberately chooses *less* durability and *less* replication, opting out of the meer and out of D-peer so no blind store holds the sealed value past its window, the inverse of every other class here. It is framed, not settled; see the history-modes companion (`07`).

---

## 9. The default deployment, and the stance behind it

Center-free standing is delivered *beneath* an experience at parity with an ordinary messaging app; it is not a cost the user is asked to feel, and not the selling point. Few users will accept a worse experience for ideology, so the principles ride underneath a default that simply works. This is the same shape as P-Durable-Enablement: the no-helper path is guaranteed by being unconditionally *available* to the minority who exercise it, not by being forced on the majority who do not. The testable consequence: under the default, a backgrounded phone behaves indistinguishably from a normal messaging app, parity-default and removable-helpers are the same design seen from two ends.

The default pairing: **D-meer over a D-self floor** for durability (a backgrounded phone still receives, because the meer held the mail), and **P-push as an opt-in convenience with poll-a-meer always beneath it** for presence, foreground being the zero-helper path. All helpers blind, revocable, redundant; the push host byte-free. A loss-tolerant or swarm-only group may instead select C-swarm, accepting visible-but-real loss as a deliberate dial.

---

## 10. Requirements placed on Part 2

This design states requirements and defers mechanism to existing Part 2 sections.

- **Part 1 §2.4 (the exit right) needs a clarification.** Make explicit that the exit right is "leave with the history you hold at the moment of departure," satisfied by the local copy in hand, conferring *no* standing to pull history after leaving. This is load-bearing: it makes D-peer's current-membership-only eligibility (§3b) a consequence of principle rather than an imposed restriction, and it dissolves the former-member history-pull attack surface.

- **Part 2 §5 (identity, capabilities, namespace):** the device-group construction; the lineage-restricted admission policy and its credential encoding; the RBSR construction choice and range-summarizable backend; the D-peer invariant that reconciliation moves only self-verifying author-signed records, never derived state; the placement of the author-signed sequence index *inside the seal* (so it is gap-definition for members and not author/sequence metadata for blind carriers, §1.2); and the dataplane history mode (forward-only vs Willow-mutable), with the mutable mode's Willow and Meadowcap parameters (see `07`).

- **Part 2 §7.3.1 (timestamp-free ordering):** the single causal fold, read by both governance and dataplane; this design requires the dataplane linearization and content-hash tiebreak to be the dataplane *reading* of that one fold, not a parallel order (this doc's §7.4), and the same fold's per-author index to serve gap detection (this doc's §7.5).

- **Part 2 §7.4 (freshness, catch-up):** the (G, D) cursor, whose dataplane half is the per-author high-water mark (§7.5); gap-visibility that must work from fabric-local state in standalone C-swarm; the reconcile-own-devices-first-then-several-members ordering before reporting surviving holes; and the membership gate that confirms a sync partner is an entitled current leaf before any reconciliation.

---

## 11. Open items

The functional claims above are validated against the real libraries; the experiment plans and results live in the companion docs (`05`, `10`), and the methodology that governs them in `08`. What remains genuinely open is a short list:

- **The D-peer default dial** per scope: on for small high-trust scopes, opt-in for large contested ones. A threat-model judgment (§2.3), not a fact to retrieve; the dial must be representable either way.

- **The dataplane history mode** (forward-only vs Willow-mutable) and whether a scope may migrate between them (`07`).

- **The RBSR production construction** (Willow 3d-range vs Negentropy), a Part 2 §5 decision; the scaling shape is validated, the specific construction is not yet chosen.

- **APNs silent-push rate:** only Apple's "a few per hour, dynamic" is pinnable; concrete secondary numbers conflict, and the device-side measurement awaits real credentials.

- **Self-destruct** semantics (`07`), bounded by node fidelity, framed but not settled.
