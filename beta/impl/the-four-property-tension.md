# The four-property tension: why a chosen Delivery Service is honest engineering

`Status: impl layer. Register: engineering argument. Resolution: library — the argument that licenses Croft's
acceptance of an MLS Delivery Service / superpeer, kept in one place so the reasoning that permits the
centralization travels with it. The decentralized-MLS siblings that neighbor this argument live in cairn
(`object-capability-and-decentralized-mls-prior-art.md`); the delivery mechanics the argument permits live in
the delivery-layer pitch (`delivery-layer/04-pitch-technical.md`). Deployment-status claims carry a `[confirm]`
flag and must be checked against primary sources before external use.`

## Overview

Croft accepts an unequal helper: a store-and-forward node (the *meer*) that keeps the MLS Delivery Service's
carriage-and-durability function, and a push-notify node that wakes a sleeping device. A design whose first
claim is peer equality owes an account of why an unequal, privileged-by-resources node is present at all. This
document supplies that account. The account is an engineering argument, not an apology: four properties that
consumer messaging wants together — group moderation, multi-device, forward secrecy, and offline-mesh
operation — cannot all be held at full strength without either an unequal peer or the degradation of one of
them. That is *the four-property tension*. Naming it, and stating its true shape precisely, is what turns
Croft's acceptance of a Delivery Service from a compromise of the equality claim into a deliberate, bounded
engineering choice.

The load-bearing move in this document is a correction to how the tension is usually stated. It is routinely
described as an *impossibility*. It is not one. It is a real, well-documented engineering tension with a
quantifiable forward-secrecy cost, and the ordering function it turns on can be a deterministic, distributed
protocol role rather than a privileged peer. Stating the tension as an impossibility overclaims and is
falsifiable; stating it as a tension with a measured cost is both true and stronger. The honest framing
survives scrutiny better than the strict one, which is the argument's central irony and its reason for
existing in its own file.

## Charter: what this document covers

- **In scope:** the four-property tension, the field evidence for it (by name: MLS, Briar, SSB, Keet), the
  secondary dials that surround it, the correction from *impossibility* to *engineering tension*, and the
  conclusion that this licenses a chosen, revocable, blind Delivery Service.
- **Out of scope (and where it lives):** the decentralized-MLS siblings that quantify the serverless cost —
  DMLS / FREEK and `draft-xue-distributed-mls` — are credited in cairn
  (`object-capability-and-decentralized-mls-prior-art.md`), not re-filed here; the delivery mechanics the
  argument permits (the three planes, the *meer*, the push-node) live in the delivery-layer pitch
  (`delivery-layer/04-pitch-technical.md`); the per-hard-case MLS posture lives in
  `mls/mls-hardcases-and-posture.md`.
- **Boundary call:** the cairn siblings doc carries the *prior art* — the named research relatives and the
  measured cost curve. This document carries the *argument* — the engineering tension that licenses accepting a
  superpeer at all. The tracking of that superpeer as a standing centralization risk lives with the delivery
  design; this doc supplies the reasoning that permits it, not the risk register that watches it.

## The four properties, and why they collide

Four properties that a consumer-grade secure messenger is expected to hold at once:

- **Group moderation** — an authority in the group can add and, load-bearing, *remove* a member, and the
  removal takes effect for everyone.
- **Multi-device** — one identity, several devices, with history and membership consistent across them.
- **Forward secrecy** — keys ratchet forward and old keys are deleted on schedule, so a later compromise
  cannot read earlier traffic.
- **Offline-mesh operation** — the group functions without continuous internet, healing across partitions
  when devices reconnect over local links.

Held pairwise these are routine. Held all four at once over a pure peer-to-peer mesh, they actively break each
other, and the breakage has two named shapes.

**Forward secrecy versus offline-mesh (the key-retention seam).** Forward secrecy requires deleting old keys
on schedule. An offline mesh is asynchronous and fragmented: if one member advances the group key while
another is in a dead zone, the histories fork. To let the offline member catch up later without a server to
impose a single order, the protocol must retain old key material long enough to process the out-of-order
state. Retaining keys past their deletion schedule is exactly what forward secrecy forbids. So the mesh's
healing need and forward secrecy's deletion need pull against each other directly.

**Group moderation versus offline-mesh (the no-master-clock seam).** A removal has to take effect globally to
mean anything. In a partitioned mesh there is no master clock: if a moderator issues a removal on one side
while the removed member keeps posting on the other, no coordination-free fact settles who is *in* the group
at that instant. Without some node empowered to arbitrate the order, a member can ignore the removal and keep
broadcasting to peers who have not yet seen it.

The common resolution to both seams is a node that is *unequal by design* — one that orders commits, arbitrates
membership, or holds the group state others depend on. That is the tension in one sentence: the fourth property
tends to demand the very inequality the other three can live without.

## The field, by name

The field is the evidence. Every shipping system that reaches for all four resolves the tension the same way —
by spending one property or seating an unequal peer.

- **MLS (RFC 9420)** holds group moderation, multi-device, and forward secrecy at full strength, and pays with
  offline-mesh: it assumes a Delivery Service to order commits so the epoch chain stays linear. Remove the
  ordering service and concurrent same-epoch commits fork the group with no protocol-level operation to stitch
  them back. MLS does not degrade a property so much as name the unequal peer explicitly and require it.

- **Briar** holds offline-mesh, multi-device, and forward secrecy, and pays with moderation: its group chats
  are append-only democratic logs with no arbiter, so a ban on one side of a Bluetooth partition does not
  reach the other side until the two merge. Moderation is the property Briar declines.

- **SSB (Secure Scuttlebutt)** holds offline-mesh, and pays with the rest: single-writer-per-feed append-only
  logs owned by one key mean multi-device is a workaround (the feed cannot be written from a second device
  without sharing the key) and per-message forward secrecy is absent. SSB buys resilience with a simplified
  security model.

- **Keet (Holepunch)** holds moderation and multi-device over large P2P streams, and pays twice: no per-message
  forward secrecy, and a hard dependency on an online DHT for live routing, so it is non-functional in an
  air-gapped local mesh. Keet's unequal element is the always-online coordinating infrastructure.

The pattern is uniform: reach for all four and you either seat an unequal peer (MLS's Delivery Service, Keet's
DHT) or drop a property (Briar and SSB drop moderation, SSB also drops forward secrecy). No shipping system
holds all four at full strength with every peer equal.

## The secondary dials

The primary tension is not the whole cost surface. Six secondary dials sit alongside it, each one a place where
cranking one property degrades another. They are the reason "just decentralize it" is not free even once the
primary tension is understood.

- **Wire overhead.** MLS's TreeKEM keeps computation at log(N) but inflates per-message metadata: a handshake
  or Welcome carries key material and tree hashes measured in kilobytes. Invisible on broadband; on a BLE or
  LoRa mesh link where bandwidth is scarce, the header size forces protocols to strip protections to fit the
  radio MTU.

- **Metadata-versus-scale.** Hiding *who* talks to *whom* needs mixnets or onion routing with cover traffic and
  deliberate delay. Cranked to maximum, per-message latency moves from milliseconds to seconds, and the cover
  traffic drains battery and data. Metadata privacy trades directly against real-time usability and scale.

- **Eviction lag.** Moderation in an asynchronous mesh is itself a dial. Strict signed access lists give clean
  authority but, when the moderator is partitioned away, an evicted member keeps seeing local traffic until
  they sync back — eviction lag. Give up strict ordering for conflict-free reconciliation and eviction lag
  disappears, but the conversation fragments into divergent views that reorder when partitions merge.

- **Energy-depletion.** A mesh routing node cannot ignore an inbound packet; it must wake and verify the
  signature. An attacker who floods junk handshakes forces every device to burn CPU and battery validating
  garbage. Robust cryptography makes each verification costlier; a lightweight handshake saves battery but
  weakens spoofing resistance.

- **Sybil.** With no central gatekeeper, fabricating identities is cheap, and an adversary can surround a device
  with fake peers. The defenses are both bad: proof-of-work melts phone batteries and penalizes low-end
  devices, and invitation chains reintroduce exactly the hierarchy pure peer-to-peer set out to remove.

- **Traffic analysis.** Even unbreakable content leaks through packet-size and timing fingerprints. Defeating
  it needs constant-rate padding and chaff — a uniform data stream transmitted whether idle or active — which
  spikes data usage and congests the network.

These dials do not change the primary conclusion; they widen it. The four-property tension is the load-bearing
one, and the dials are why the surrounding engineering stays hard even after it is accepted.

## The honest correction: a tension, not an impossibility

The tension is frequently escalated into a claim that a moderated, multi-device, forward-secret, offline-mesh
messenger is a *mathematical impossibility*. That escalation is wrong, and correcting it is the load-bearing
work of this document.

**It is quantitative, not binary.** The forward-secrecy-versus-mesh seam has a measured cost, not an infinite
one. Retaining key material to process out-of-order commits does reduce forward secrecy — but the reduction can
be bought back incrementally. The decentralized-MLS research family (DMLS / FREEK, credited in the cairn
siblings doc `object-capability-and-decentralized-mls-prior-art.md`) recovers most of the lost forward secrecy
by selectively puncturing retained key material rather than retaining it wholesale, at a storage cost that
scales with fork frequency. The cost is real and it is paid in storage and complexity — but a quantifiable cost
is precisely what an impossibility is not.

**The ordering role need not be a privileged peer.** MLS already resolves forks without a trusted referee: when
a commit for a past epoch arrives, clients apply a deterministic tie-breaking policy to keep or revert. A
deterministic, content-derived tie-break is a *protocol role that every peer runs identically*, not an unequal
node that arbitrates for the others. The phrase "unequal, privileged peer" smuggles a centralization conclusion
into a place where the cryptography only requires a deterministic ordering function. Ordering can be
distributed; it does not have to be seated in one node.

**The empirical anchor, stated precisely.** What is true, and airtight, is the deployment claim — not the
theory claim. Every MLS deployment that actually ships orders commits through a central Delivery Service:
Webex, Wire, Discord, and the Google/Apple RCS MLS-E2EE rollout are all server-ordered `[confirm]`. The
serverless variants that break the privileged-peer dependency — DMLS / FREEK and `draft-xue-distributed-mls` —
exist only as drafts and proof-of-concept code as of mid-2026 `[confirm]`. So the privileged ordering peer is
empirically universal in deployment, theoretically escapable, and nobody has shipped the escape. The precise,
defensible framing is therefore **"no production deployment we are aware of"** does serverless-ordered MLS —
*not* "it is impossible." The strict-negative "impossible" is falsifiable and overclaims; the deployment
framing is exact, and it is the one to use externally.

The irony worth keeping: the honest framing is the stronger one. "Impossible" invites a single counterexample
to demolish it. "A real tension with a measured cost that no shipping system has yet paid" invites no such
demolition, because it is simply what the field shows.

## Why this licenses a chosen, revocable, blind Delivery Service

The argument above is what permits Croft's *meer* and push-node without conceding the peer-equality claim. The
reasoning has three moves.

First, the tension is real, so pretending the fourth property is free would be dishonest. A design that claimed
full moderation, multi-device, forward secrecy, and offline-mesh with every peer perfectly equal and no measured
cost would be making exactly the overclaim this document rejects. Accepting a helper is the honest posture, not
the compromised one.

Second, the helper Croft accepts is the *bounded* form of the unequal peer, not the structural one. It keeps the
Delivery Service's carriage-and-durability function and sheds its ordering authority, because Croft's ordering
is sourced deterministically from the messages' own signed indices — the deterministic-protocol-role escape the
correction above establishes. The *meer* is a resource-asymmetry role: unequal only in what it *has* (uptime,
storage), never in what standing or authority it *holds*. It is content-blind (it moves sealed bytes it cannot
read), revocable (removing it costs convenience, never function or standing), and redundantly held. The
push-node — the *P-push* role in the delivery-layer pitch (`delivery-layer/04-pitch-technical.md`) — is the same
kind of thing: a content-free wake signal, doubly removable, that could have become a structural dependency and
deliberately does not.

Third, the inequality is chosen and tracked, not smuggled. The superpeer is registered as a standing
centralization risk with the delivery design, where the risk is watched; this document is the other half of
that pair — the argument that says accepting it is honest engineering rather than a failure of nerve. A tracked,
bounded, revocable, blind helper answering a real and measured tension is a deliberate choice. A hidden,
authority-bearing, non-removable one would be the compromise. Croft accepts the first and refuses the second,
and the four-property tension is why the first is defensible.

## What this establishes (and does not)

Establishes that four properties consumer messaging wants together — group moderation, multi-device, forward
secrecy, and offline-mesh — stand in a real engineering tension that the field resolves uniformly by seating an
unequal peer or dropping a property (MLS seats a Delivery Service; Briar and SSB drop moderation; SSB drops
forward secrecy; Keet needs an online DHT); that the tension is surrounded by secondary dials (wire overhead,
metadata-versus-scale, eviction lag, energy-depletion, Sybil, traffic analysis) that keep the surrounding
engineering hard; that the tension is honestly a *quantifiable engineering tension*, not a mathematical
impossibility, with a forward-secrecy cost that is measured and partly recoverable and an ordering role that can
be deterministic and distributed rather than privileged; that the airtight claim is the deployment one — no
production deployment we are aware of orders MLS without a server — stated as such and not as "impossible"; and
that this argument is what licenses Croft's acceptance of a chosen, revocable, blind Delivery Service (the
*meer* and the push-node) as honest engineering rather than a breach of the peer-equality claim.

Does **not** re-file the decentralized-MLS prior art — DMLS / FREEK and `draft-xue-distributed-mls` are credited
in cairn (`object-capability-and-decentralized-mls-prior-art.md`), which carries the measured cost curve this
argument relies on; does **not** specify the delivery mechanics the argument permits (the three planes, the
*meer*, the *P-push* node), which live in the delivery-layer pitch (`delivery-layer/04-pitch-technical.md`);
does **not** maintain the centralization-risk register that tracks the superpeer, which lives with the delivery
design; and does **not** certify the deployment-status claims — the "every shipping MLS is server-ordered" and
"the serverless escapes are drafts / proof-of-concept as of mid-2026" claims carry a `[confirm]` flag and need a
check against primary sources (the IETF production-user list, the MLS architecture RFC, the DMLS and
distributed-MLS drafts) before external use.
