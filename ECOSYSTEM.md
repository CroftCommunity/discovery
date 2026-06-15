# Open Ecosystem Register: prior art, integrations, partners, and two-way learning

date: 2026-06-15

status: living register — refresh current-state periodically.

purpose: track the related open-source / open-ecosystem work this effort stands on. These
are prior art we owe homage to, projects we build on or want to integrate, orgs we'd want to
partner with or rebroadcast to, and work we want to learn from in both directions. This is
the "we are part of a movement, not inventing in a vacuum" record.

This complements `research/messaging-solutions-landscape.md` (which judges messaging systems
*against our design* along usability/security/capability) — here the framing is relational,
not competitive.

## Relationship tags

- **homage** — prior art / influence we owe credit to.

- **build-on** — we use it (or plan to) as a dependency or substrate.

- **partner** — org-level collaboration worth pursuing.

- **rebroadcast** — we want to bridge/republish to or from it.

- **learn↔** — mutual learning; our work may inform theirs and vice versa.

current-state legend: facts marked **[verified <source>]** are confirmed from this corpus's
research or live experiments; **[UNVERIFIED]** needs a refresh pass before external use. The
dossier's standing correction: iroh is at `1.0.0-rc.0`, not "pre-1.0 / v0.97."

---

## 1. Transport & substrate

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| n0 | iroh | QUIC-first P2P networking (Rust) | NodeId (Ed25519), hole-punching, relays; iroh-gossip, iroh-docs, iroh-blobs (BLAKE3 content-addressed) | `1.0.0-rc.0`; in production in Delta Chat across 100k+ devices [verified: research + dossier] | build-on, partner, learn↔ |
| — | libp2p | Modular P2P stack | Transports, pubsub, DHT | mobile-weak vs iroh; rejected as primary [verified: dossier] | homage |
| Veilid team | Veilid | Privacy-first P2P with source-address-free routing | Ed25519/x25519/XChaCha20/BLAKE3/Argon2; DHT (small mutable records) | demoted to future metadata-resistant messaging-layer candidate; no large-blob primitive [verified: dossier] | learn↔ (future) |
| — | Holochain | Agent-centric P2P (no global consensus) | source chains, rrDHT, validation rules, membrane proofs | dropped as substrate (uses iroh transport anyway; mobile-weak) [verified: dossier] | homage (borrow patterns) |
| Earthstar / Willow team | Willow protocol / willow-rs | Local-first data model with true deletion | 3D data model, Meadowcap capabilities, prefix-pruning | not shippable in 2026; design "Willow-shaped," migrate later [UNVERIFIED current] | homage, build-on (later) |

## 2. Group encryption & crypto primitives

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| IETF MLS WG | MLS / RFC 9420 | Standard group key agreement | per-epoch rekey, forward secrecy, post-compromise security; assumes a delivery/ordering service | published RFC; the standard we build group encryption on [verified] | homage, build-on |
| Phoenix R&D / community | openmls | Rust MLS implementation | external-commit builder, reinit, fork_resolution module | `0.8.1`; proven to express survivor re-key with PCS [verified: Proofs/lineage-groups PR #8] | build-on, learn↔ |
| Cryspen / community | hpke-rs | HPKE for MLS | RFC 9180 HPKE | MPL-2.0 (mandatory for RFC 9420; no permissive substitute) — our open license-gate item [verified: PR #8] | build-on |
| Signal Foundation | Signal Protocol | 1:1 + group E2EE benchmark | X3DH, Double Ratchet, sender-keys, sealed sender | the UX/security gold standard; centralized [verified: research] | homage, learn↔ |
| RustCrypto | k256 / DAG-CBOR / CID crates | secp256k1, content addressing | low-S secp256k1, DAG-CBOR, multibase | `k256 0.13` (stable, not RC) [verified: PR #4] | build-on |

## 3. CRDT / local-first data

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| Ink & Switch / community | Automerge | CRDT for shared mutable state | multi-writer merge, change history | Automerge 3.0 claimed ~10× (up to 100×) memory reduction vs v2 [UNVERIFIED version] | build-on, learn↔ |
| Ink & Switch | "Local-first software" (Kleppmann et al. 2019) | The local-first thesis | 7 ideals; single-user value before network effects | foundational essay [verified: dossier] | homage |

## 4. Identity, trust & capabilities

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| Bluesky PBC | AT Protocol | Federated public social protocol | DIDs, handles, PDS, Jetstream firehose, lexicons, labelers | live; custom NSIDs propagate w/o registration; labelers pull-only; writes via entryway, reads via PDS [verified: PR #4/#6 live] | build-on (public path), partner, rebroadcast, learn↔ |
| Bluesky PBC | did:plc | DID method | signed, auditable op log (PLC audit) | live; flagged centralization vector + consumer-scale recovery unpolished [verified: research/dossier] | build-on, learn↔ |
| W3C | DIDs / Verifiable Credentials | SSI standards, blockchain-free | self-certifying identifiers, VC issuance | standards; our identity favors these + did:web [UNVERIFIED current] | homage, build-on |
| Spritely Institute | Goblins / OCapN / CapTP | Object-capability distributed programming | "designation is authorization," POLA, petnames | 501(c)(3), NLnet/NGI-funded, no VC/tokens — a funding/governance model too [UNVERIFIED current] | homage, partner, learn↔ |
| — | BrightID / petname systems | Proof-of-personhood / human-readable local naming | web-of-trust, Sybil resistance | [UNVERIFIED current] | learn↔ |
| Trust over IP Foundation | ToIP Decentralized Trust Graph WG | Trust-graph standardization | transitive trust, Merkle proofs | no de-facto standard yet [verified: dossier] | partner, learn↔ |

## 5. Social protocols & federation

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| W3C | ActivityPub | Federated social standard | server-to-server, actor model | dropped for v1 (conflicts with local custody behind NAT); federation priority #1 for later [verified: dossier] | rebroadcast, homage |
| Nostr community | Nostr | Simple relay-based protocol | signed events, relays | federation priority #3 [UNVERIFIED current] | rebroadcast, learn↔ |
| Ryan Barrett | Bridgy Fed | AP ↔ ATProto bridge | cross-protocol federation | the bridge reference [verified: dossier] | build-on, partner |
| Matrix.org Foundation | Matrix | Federated E2EE group chat | Olm/Megolm; MLS migration in progress | MLS still in design (MSC4256/4244, arewemlsyet.com); 25+ govt deployments [verified: research] | homage, learn↔ |

## 6. P2P / decentralized messengers (the field)

Detailed competitive analysis lives in `research/messaging-solutions-landscape.md`. Relational summary:

| Org | Project | Purpose | Current state | Relationship |
|---|---|---|---|---|
| — | Secure Scuttlebutt / Manyverse | Pure-P2P local-first social | declined; Manyverse lead stepped away Apr 2024; fusion-identity & partial-replication specs archived [verified: research] | homage, learn↔ (the canonical cautionary tale) |
| Merlin / community | Delta Chat | E2EE over email + iroh | Rust core, chatmail, iroh realtime + Add-Second-Device; multi-device = transfer-then-diverge [verified: research] | homage, learn↔ (closest Rust+iroh cousin), partner |
| Briar Project | Briar | Tor-based P2P, high-risk threat model | no multi-device, no recovery (by design); Mailbox async relay [verified: research] | homage, learn↔ |
| Session / OPTF→Swiss foundation | Session (Oxen) | No-phone decentralized messaging | Protocol V2 (Dec 2025) re-added PFS + ML-KEM; mnemonic recovery [verified: research] | homage, learn↔ |
| — | Cwtch / SimpleX / Tox | Metadata-resistant / no-identifier messaging | SimpleX "no persistent identifiers" lesson [UNVERIFIED current] | learn↔ |
| — | XMTP / Keybase teams | web3 messaging / team key management | per-device-as-member prior art (Keybase) [UNVERIFIED current] | learn↔ |
| Germ Network | Germ DM | MLS E2EE messenger on atproto identity; launches from Bluesky profiles | iOS public beta; MLS, 1:1 text today, multi-identity ("cards"), no-phone; small (~4-person) team [verified: research/germ-xchat-features.md] | homage, learn↔ (closest atproto+MLS cousin), partner |
| X (Twitter) | X Chat / XChat | Mass-market messaging with server-held keys | Juicebox PIN-recoverable server-held keys; **no forward secrecy** (X's own admission); E2EE claims disputed by cryptographers; seamless multi-device is the headline [verified: research] | learn↔ (the anti-pattern: convenience bought with encryption integrity; the multi-device bar) |
| Bluesky PBC | Bluesky native DMs / group chats | Built-in messaging, not E2EE | native group chats launched 2026-06-11, up to 50, no media at launch; distinct from Germ [verified: research] | homage (expectation-setter) |
| Juicebox | Juicebox protocol | PIN-recoverable distributed key storage | the mechanism X Chat uses for server-held key recovery; relevant to our recovery-anchor decision | learn↔ |

## 7. Funders, standards bodies & movements

| Org | What | Relationship |
|---|---|---|
| NLnet / NGI (EU) | Funds Spritely, willow, and similar non-extractive infra | partner (funding model + grants) |
| IETF MLS WG / W3C | Standards we conform to (MLS, DIDs, VCs, ActivityPub) | homage, build-on |
| Mike Masnick | "Protocols, Not Platforms" (2019) | homage (framing) |
| Jay Graber / Bluesky | "Mundus sine caesaribus" | homage, learn↔ |

## 8. Cooperative / governance prior art

The co-op vertical's lineage (detail in the dossier §3, §8):

| Example | What it proves | Relationship |
|---|---|---|
| Elinor Ostrom's commons work | Communities sustain shared resources for centuries (Törbel, Valencia, Bali Subak, Maine lobster) | homage (governance DNA) |
| Green Bay Packers | Only community-owned major US sports team; the model was banned after | homage |
| Mondragon | Worker-owned federation at scale | homage |
| Credit-union lineage | Schulze-Delitzsch → Raiffeisen → Desjardins → Filene; "not for profit, not for charity, but for service" | homage (institutional model) |
| Informal Systems (Ethan Buchman), Subvert, Patio.coop, USFWC | Modern tech-worker co-op references / structures (LCA + PBC) | partner, learn↔ |

---

## Refresh discipline

Volatile facts (versions, ship dates, org changes) drift. Before any external use, run a
verification pass on every `[UNVERIFIED]` row and re-confirm the `[verified]` ones older than
a few months. The provenance-debt caveat from the dossier applies here too: framings are
sound; specific numbers and dates need primary-source confirmation.
