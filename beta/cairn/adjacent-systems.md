# cairn / adjacent-systems: the field Drystone sits among

date: 2026-07-07

**What this doc is.** A grounded landscape survey of the existing privacy-forward,
capture-resistant, credible-exit systems that Drystone learns from and sits among. It is the
inverse of the activism layer: activism indicts the extractive incumbents; this doc credits the
adjacent tech and locates the gap it leaves. Everything here is distilled from the local-authority
collaboration notes' ecosystem survey (Thread D of the 2026-07-07 batch-ten transcript), which
rates systems on two axes: **capture-resistance** (can users leave with their graph and data
intact, without a single operator holding them hostage) and **privacy** (how little the
infrastructure can see).

The finding the survey lands on: the corner where a system is strong on *both* axes is either
niche-and-young or mature-and-metadata-leaky, and the standards bodies took a per-room-hub
shortcut. That is the structural reason the "both" corner is empty, and it is the space Drystone
occupies.

## Two products, two postures

### Roomy (atproto): a two-pivot journey

Roomy is an atproto project whose path is a two-pivot journey, and the shape of that path is the
lesson.

It first reached for peer-to-peer stacks: Willow, then Keyhive, then Jazz. Each was dropped because
the tech was not ready to depend on.

Having exhausted the ready-made p2p options, in late 2025 it built its own off-protocol server:
Leaf, an event-sourced per-space sync layer for private data, realtime, and notifications. This was
off-protocol by necessity, not by preference.

Then in 2026 it swung back atproto-native, once Bluesky's permissioned-data proposal landed. That
swing added **The Arbiter**: a per-community group-membership service with a root DID, eight
cumulative access levels, recursive space-in-space delegation, and an `$admin` space. A detail worth
holding: the access levels govern only the arbiter itself, not the app.

Roomy reached GA on July 1 2026.

### p2panda: one ground-up rewrite, then additive build-out

p2panda took a single ground-up rewrite in late 2024, moving to modular, CRDT-agnostic crates that
reuse iroh. That was the one hard reset.

Everything after was an additive build-out of the hard parts as they ripened: offline networking,
persistence, group and message encryption (roughly a Double-Ratchet adapted for offline groups),
and decentralized access control with a "pull" level that relays without reading. That access-control
work is p2panda-spaces.

p2panda was pre-1.0 as of mid-2026.

### The contrast

The two projects are the same problem approached from opposite sides of the supplier relationship.

Roomy is a **customer changing suppliers** because the tech it wanted to buy was not ready: it tried
three p2p stacks, gave up and built its own server, then switched back to the native option once it
matured.

p2panda is a **supplier doing the research and shipping the hard parts as they ripen**: one rewrite
to the right foundation, then a steady additive release of the difficult pieces.

Both stories carry the same underlying signal that Willow, Keyhive, and Jazz were not yet
dependable, which is the same "not-ready-as-a-dependency" profile the Willow/Meadowcap analysis
notes for its own implementations.

## The standards: MLS, MIMI, and the unstandardized seam

The standards layer is a two-protocol story, and the gap between the two protocols is where Drystone
lives.

**MLS (RFC 9420)** is the group end-to-end-encryption standard, and it is delivery-agnostic: it
standardizes the encryption, not how messages get carried.

**MIMI** (More Instant Messaging Interoperability) is the interop standard, and it reintroduces a
hub: each MIMI room is hosted at a single provider, and that provider orders messages and is trusted
to enforce room policy. The draft accepts this cost "for simplicity."

So the privacy standard exists, the interop standard chose a per-room hub, and the
**capture-resistant delivery layer is exactly what is left unstandardized**. That is the seam
Drystone occupies.

## Implementations on the two axes

The following systems are rated as the survey rates them: capture-resistance and privacy, with the
maturity of the group story noted because that is usually where the weakness lives.

### SimpleX: closest to the spec, achieves privacy by deleting identity

SimpleX is rated closest to the specification of what a graph-blind system looks like. It carries no
user identifiers at all, uses a double ratchet with post-quantum protection, envelope-encrypts
delivery metadata, offers self-hostable relays, and has multiple Trail of Bits audits behind it.

The mechanism worth naming: it achieves graph-blindness by *deleting identity* rather than by hiding
it. There is no persistent account for the infrastructure to see.

Its groups are the least mature part of the offering.

### Briar / Cwtch: strongest capture-resistance, niche

Briar and Cwtch have the strongest capture-resistance in the survey: no servers (or Tor-only),
device-to-device delivery, metadata-resistant by construction, and audited.

The cost is reach: they are niche, small-group tools, not general-purpose community infrastructure.

### Matrix + Element: the most mature "both," but the metadata leaks

Matrix with Element is the most mature system that is genuinely strong on both axes: federated,
self-hostable, and end-to-end-encrypted.

The qualifications are structural, not incidental. Room and membership metadata live on homeservers,
and there is heavy de-facto centralization in practice despite the federated design. It is also worth
noting that the Matrix Foundation co-authors MIMI, which ties it to the per-room-hub choice above.

### Session: decentralized, and the forward-secrecy saga worth learning from

Session is decentralized, and its forward-secrecy history is a correction the messaging-solutions
landscape research pins down precisely. Session began as a Signal fork, then moved off the Signal
Protocol to its own Session Protocol, and in that move **dropped perfect forward secrecy and strong
deniability** — shipping a decentralized product for years without PFS, which drew sustained
criticism, because without forward secrecy a compromised long-term key can decrypt all past
intercepted messages. It then reversed course: Session **announced Protocol V2 on 1 December 2025** as a
design/roadmap disclosure that will **re-introduce perfect forward secrecy** and **integrate ML-KEM**
post-quantum key exchange, so a stolen device holding current keys will no longer decrypt old messages.
`[confirm — the V2 announcement is verified (2026-07-10) against the getsession.org V2 post and Privacy
Guides, but V2 is still in the design phase: PFS re-implementation and ML-KEM integration are planned,
with detailed specifications expected in 2026, not shipped as of the announcement.]`

The load-bearing lesson, not the timeline, is why Session sits in this survey: **do not trade forward
secrecy for decentralization convenience.** The trade was tempting precisely because Session was
optimizing for anonymity and metadata resistance, and it still cost the property for years before the
reversal. This vindicates Drystone's choice to compose MLS rather than roll a custom protocol — MLS
gives forward secrecy and post-compromise security by design, so the property is a structural
guarantee rather than a decision a team can quietly defer. `Synthesis.`

### Nostr: capture-resistant and credible exit, with a serious MLS privacy answer

Nostr is capture-resistant and offers a credible exit, and it is public-by-default at its base layer,
where its legacy direct messaging is bolted-on and leaky. But the "merely leaky" reading understates
where Nostr now is, and the open-social Nostr dialogue (and its FACTCHECK companion, which confirms
the projects and NIP numbers) supplies the counter-evidence. Nostr today has two private-group paths.
The first is **relay-enforced** privacy (**NIP-29**): a group lives on a specific relay that checks an
authenticated allow-list (**NIP-42**) and refuses to serve data to non-members — Discord-like, and it
trusts the operator. The second is **true cryptographic end-to-end encryption**: **Marmot**, the glue
between Nostr and **MLS**, layers RFC-9420 group encryption onto the substrate, and it pairs with
**NIP-59** ("gift wrapping") to hide sender, recipient, and topic metadata.

That second path already ships. **White Noise** is a live, privacy-focused MLS group messenger on
Nostr + Marmot — metadata-hidden group chat on a decentralized, credible-exit social substrate — and
**Amethyst** carries Marmot-compatible encrypted group channels. White Noise is, in other words, a
shipping cousin of Drystone's exact thesis: MLS group confidentiality with hidden metadata riding an
open substrate. So Nostr does not simply fail the privacy axis; on the messaging plane it reaches the
same corner Drystone targets. What remains distinct is the same distinction the ecosystem survey draws
for Germ and SimpleX — White Noise is metadata-hidden *messaging and groups*, not a durable social
data model with peer-symmetric governance — but the honest reading is that Nostr's private-group story
is a serious neighbor to learn from, not a leak to dismiss. `Synthesis.`

## Conclusion: why the "both" corner is empty

Reading the two axes together, the pattern is clean:

The systems strong on both privacy and capture-resistance are either **niche-and-young** (SimpleX
groups still maturing; Briar and Cwtch confined to small-group use) or **mature-and-metadata-leaky**
(Matrix with metadata on homeservers and de-facto centralization).

The systems that pick one axis pay on the other, with two qualifications the corrections above carry:
Nostr is capture-resistant and public-by-default, but its Marmot/White Noise MLS path shows the
privacy axis is reachable on that substrate for messaging; and Session traded forward secrecy for
decentralization for years before announcing a V2 restoration of PFS (December 2025, design-phase), the exact trade
Drystone refuses by composing MLS.

And the standards bodies did not fill the corner either: MLS standardized the encryption but left
delivery to the deployer, and MIMI standardized interop by taking the per-room-hub shortcut.

That per-room-hub shortcut is the **structural reason the corner is empty**. The capture-resistant
delivery layer, the part that would let a mature "both" system exist without a trusted per-room
provider, is precisely what no one standardized. That unstandardized seam is the space Drystone
occupies.

## Provenance

Distilled from Thread D of `../../alpha/seeds/transcripts/raw/mls-scaling-willow-ecosystem-and-cairn-2026-07-07.md`
(the local-authority collaboration notes' ecosystem survey). The Session forward-secrecy correction is
grounded in the messaging-solutions landscape research (Session Protocol V2 / ML-KEM restoration); the
Nostr private-group counter-evidence (NIP-29, NIP-59, Marmot, White Noise, Amethyst) is grounded in the
open-social Nostr dialogue and its FACTCHECK companion, which confirms the projects and NIP numbers.
Both added sources are registered in `reference-index.md`. Companion cairn docs cover MLS/MIMI as a
building block (`mls-and-mimi`), Willow/Meadowcap (`willow-meadowcap`), and the atproto ecosystem
(`atproto-ecosystem.md`). See `../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md` for
provenance status.
