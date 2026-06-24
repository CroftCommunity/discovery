# Messaging & Group-Chat Solutions Landscape

author: ISaT / Product Security

date: 2026-06-13

status: draft for internal review

scope: competitive analysis along usability, security, and capability axes, mapped to our planned encrypted local-first group-messaging stack

---

## Executive summary

This document positions our planned stack (iroh P2P transport plus optional always-on superpeer broker, AT Protocol DIDs for identity, MLS/RFC 9420 via openmls for group encryption, Automerge CRDTs for shared state, iroh-blobs for encrypted media) against the real field of deployed messaging systems. The field runs from Signal's centralized excellence to Secure Scuttlebutt's pure-P2P struggles, with Delta Chat sitting as our closest technical cousin because it is already Rust-plus-iroh in production.

The central finding is that **no system in the field delivers usability, decentralization, and metadata protection simultaneously**. Every solution buys one or two of those by spending the third. Signal buys world-class UX with centralization and phone-number identity. SSB bought pure decentralization and paid with multi-device hell, identity-recovery dead-ends, and unbounded log growth that the project never solved. Briar buys a strong threat model and pays with single-device-only operation and no recovery at all. Delta Chat buys serverless-ish operation by riding email, and inherits email's metadata leakage.

Our architecture's whole bet is that a *thin, optional, cryptographically blind superpeer broker* plus modern CRDTs and MLS escapes SSB's usability collapse without surrendering to Signal's centralization. The evidence partially supports the bet and partially challenges it. The strongest support: MLS is purpose-built for the group-rekey problem SSB never solved, and a rendezvous/queue broker genuinely does dissolve the "both peers must be online" trap that makes pure-P2P painful. The strongest challenge: **multi-device and device-loss recovery remain unsolved across nearly the entire field**, and our DID-multi-device story currently inherits the same hard problem that defeated SSB's fusion-identity effort and that Briar simply refuses to attempt. Our metadata claim is realistic at the broker layer but, like everyone else's, weakest at the network/IP layer.

The highest-value sections are the SSB cautionary tales and the multi-device/recovery synthesis. Those are where our design is most exposed.

---

## Per-solution investigation

For each system: current state (verified June 2026 where possible), then what it does better than our plan, what it gets wrong that we must avoid, and what it validates or challenges in our design. Protocol-level facts are distinguished from client/product-level facts throughout.

### Signal

**Current state.** Signal is the benchmark for secure-messaging UX. The Signal Protocol combines X3DH key agreement with the Double Ratchet for forward secrecy and post-compromise security. Group messaging uses pairwise Double Ratchet fan-out via "sender keys" layered on the pairwise sessions. Registration still requires a phone number, but since early 2024 [usernames are live](https://signal.org/blog/phone-number-privacy-usernames/) as an optional contact-discovery layer: someone needs your exact unique username to start a chat, and you can hide your phone number from people who do not already have it saved. The phone number is still the root account identifier even when hidden.

Multi-device is a primary-device-plus-linked-devices model. The phone is the primary; desktop and iPad are linked. Sealed sender reduces the metadata the server learns about who is sending to whom, though Signal itself frames sealed sender as an incremental step, with traffic correlation via timing and IP addresses an area of ongoing work. Recovery is handled by a PIN plus registration lock.

**What it does better than us.** Onboarding and "does it just work" are in a different league. Contact discovery via phone number is effortless for mainstream users. The linked-device flow gives genuine phone-plus-desktop use with synced sending. Its metadata story at the *application* layer (sealed sender) is mature and shipping, where ours is still a design claim.

**What it gets wrong that we must avoid.** Centralization is the obvious cost. A single operator can be compelled, blocked, or can change policy. Phone-number-rooted identity is a privacy and censorship liability in exactly the high-risk contexts where decentralization matters most. We must avoid making any single party (including a superpeer operator) a chokepoint of that kind.

**Validates / challenges our design.** Signal validates the thesis that centralization buys UX, which is precisely the cost our optional-broker model is trying to avoid paying in full. It challenges us by setting a UX bar that pure-P2P never meets. The honest question Signal poses: can our optional broker deliver linked-device-quality multi-device without becoming a Signal-style mandatory primary? We do not yet have a shipped answer.

### Delta Chat (our closest technical cousin)

**Current state.** Delta Chat is encrypted messaging over the existing email/SMTP network, with a Rust core, an audited subset of OpenPGP (via rPGP, also audited), and no central server of its own. Identity is an email address; with "chatmail" servers, account creation is effectively a one-tap QR scan with no phone number. The relevant fact for us: Delta Chat integrated iroh in production. Delta Chat uses iroh for multi-device setup and realtime P2P communication across hundreds of thousands of devices.

Crucially, the iroh usage is scoped, and verifying the scope matters because it is easy to overstate. The realtime P2P path powers webxdc apps via a `joinRealtimeChannel()` API. Delta Chat 1.48 shipped peer-to-peer networking with hole punching and forward-secret end-to-end encryption, establishing private P2P gossip networks between users who start a webxdc app that uses the realtime API. The realtime identities are deliberately throwaway: Delta Chat uses ephemeral cryptographic identities for P2P messaging, and a new ephemeral identity is created on each start; iroh's QUIC layer provides forward secrecy. Implementation-wise, it introduces an IrohGossipTopic message header carrying a random 32-byte TopicId generated on the sender's device, sent encrypted in the same message as the webxdc app, and P2P connectivity is established lazily only when a realtime channel is actually used.

The other iroh use is "Add Second Device." This is **not** continuous live sync. It is a one-time transfer: on the old device you choose Add Second Device, on the new device you scan a QR code, and iroh moves the account over a direct connection. After a successful transfer both devices are completely independent, and one device is not needed for the other to work — unlike many messengers. The transfer is local-network-oriented and is a documented friction point: it depends on both devices being reachable on the same network, which trips firewalls and VLAN segmentation, and community threads complain that requiring a local network is itself an obstacle for users who rarely have one. State and location sync between a user's own devices then happens over SMTP, not P2P, precisely because a device could be offline for days and state needs to sync reliably regardless.

A security caveat worth carrying forward: the iroh connection does not respect proxy settings even when connecting to a relay, so it should not be relied on for anonymization, a point Delta Chat community members flagged as unexpected and dangerous for users who assume proxy coverage.

**What it does better than us.** No-phone-number identity that mainstream users tolerate, riding infrastructure (email) that already has universal reach, offline queuing for free (email is store-and-forward), and a shipped, audited crypto path. Its "ride existing infrastructure" model means it never had to bootstrap a network. The split it discovered — use P2P only for ephemeral realtime, use durable store-and-forward for anything that must survive a multi-day offline gap — is a directly transferable design lesson for us.

**What it gets wrong / finds painful that we must avoid.** The email substrate leaks metadata: your email provider sees who you message and when. Multi-device is transfer-then-diverge, not unified history with live sync, and the local-network requirement is a real onboarding stumbling block. Group semantics inherit email's awkwardness. The proxy-bypass gotcha is a concrete anti-pattern: our blind-broker claim must not be silently undermined by a transport that ignores the user's network privacy settings.

**Validates / challenges our design.** This is the most important validation in the document: a Rust-plus-iroh production system confirms iroh's NAT traversal and QUIC-based forward secrecy work at scale, and that the gossip-topic-with-shared-key pattern is a viable realtime primitive. It validates our optional-realtime-over-P2P plus durable-state-elsewhere split. It challenges us where Delta Chat *declined* to use P2P for durable multi-device sync, choosing store-and-forward (SMTP) instead. Our superpeer broker is meant to be that durable-queue layer, so Delta Chat's choice is evidence our instinct is right — but it also means the broker is load-bearing, not optional, for the multi-device experience specifically.

### Secure Scuttlebutt (the canonical cautionary tale — read most closely)

**Current state.** SSB is the purest prior attempt at the thing we are philosophically closest to: fully local-first, P2P, identity-as-keypair. An identity is simply an Ed25519 key pair, there is no worldwide store of identities, and each identity has exactly one append-only signed feed. Gossip replication spreads feeds peer-to-peer, optionally via "pubs" or "rooms" as rendezvous points.

The current state is also a decline story, and that is itself a finding. The lead developer of Manyverse (the main mobile client) and the proposed next-generation PPPPP protocol [announced in April 2024](https://www.manyver.se/blog/2024-04-05/) that his time building up SSB, Manyverse, and the new PPPPP protocol was over. Patchwork, the original desktop client, was deprecated years earlier as essentially unmaintainable for a small group of volunteers. The ecosystem has shifted and contracted. Several of the spec repos that were meant to fix SSB's core limitations are now archived read-only.

**The two failure modes that are our specific risks.**

*Multi-device.* This is the canonical SSB pain and it is structural, not incidental. Each device must use its own identity; there is no avoiding that, because the append-only feed cannot be safely written from two devices. The "fusion identity" effort tried to paper over this by joining multiple per-device feeds into one logical identity. Read the spec's own framing carefully: the Fusion ID protocol exists because in Scuttlebutt users have multiple devices but each device must have its own feed due to synchronization issues, and fusion IDs are an attempt to mitigate the user-experience impact of this technical limitation. It was never finished. The spec itself lists severe v1 limitations: only new members can be added to a fusion, there is no removal of members, only tombstoning; redirection and attestation were deferred to a future version once more real-world experience was gained. The mechanism also requires sharing the fusion private key across devices, which an audit flagged as a weaker security posture.

*Identity recovery.* Because identity is one keypair tied to one device's append-only log, losing the device or key is close to fatal. If a user loses their secret key or has it stolen, they must generate a new identity and tell people to use the new one instead. There is no recovery, only re-introduction to the social graph.

*Scaling and log growth.* The append-only-forever design means feeds grow without bound and replication gets heavier over time. Partial replication and meta-feeds (bendy-butt format) were designed specifically to let clients avoid downloading entire feeds, but these arrived late and the relevant spec repositories are now archived. SSB users have long described choosing how much to download via "hops" and blob-storage limits as a manual coping mechanism for a problem the protocol could not solve cleanly. Forward secrecy is also largely absent: an audit noted that most modern secure communication protocols provide forward secrecy, but in the Scuttlebutt ecosystem very few components satisfy it.

**What it does better than us.** Genuine zero-infrastructure operation. No server is required at all in the steady state; co-present peers gossip directly. Identity-as-keypair is clean and permissionless: because identities are long and random, no coordination or permission is required to create one. Offline-first is real, not aspirational. The social-graph-bounded replication model (you only pull data from friends and friends-of-friends) is an elegant spam and scaling control in principle.

**What it gets wrong that we must avoid.** Everything in the two-failure-modes section. Specifically: single-device-tied identity, no recovery path, unbounded log growth, and forward-secrecy gaps. Also the offline-first-but-unusably-so trap: the ideal of pure local-first produced real-world friction (slow initial sync, confusing onboarding, "where are my messages") severe enough to contribute to ecosystem decline.

**Validates / challenges our design.** SSB is the test case for our entire bet. It validates the *diagnosis* — that pure-P2P with device-tied keypair identity and append-only logs leads exactly to multi-device hell, recovery dead-ends, and log-scaling pain. Our design's response to each:

- *Multi-device:* DIDs decouple identity from any one device's log. A DID can in principle authorize multiple device keys without a shared-private-key hack. This is genuinely better than fusion identity *if* we implement DID key rotation and device authorization properly. We have not yet.

- *Recovery:* a DID with rotatable keys and an always-on broker that can re-attest device membership is a real recovery story that SSB structurally could not have. But see the synthesis: we have not specified the recovery flow, and DID-method choice (did:plc vs did:web vs did:key) determines whether recovery is actually possible or just theoretically so.

- *Log scaling:* Automerge CRDT documents with snapshotting are designed to compact state rather than replay an unbounded log. The broker holding snapshots means a new or returning device syncs a compacted document, not a full history replay. This directly attacks SSB's log-growth problem — *provided* Automerge's own history/change growth is managed, because CRDTs accumulate change metadata too.

- *Onboarding:* the optional broker as rendezvous removes SSB's "both peers must be co-present or find a pub" friction. This is the cleanest win.

The honest caution: SSB's failures were not from lack of cleverness. The fusion-identity and partial-replication specs were thoughtful and still did not ship into a usable product. Our advantages (DID, MLS, CRDT, broker) are real, but they are advantages *on paper* until the multi-device and recovery flows are actually built and tested.

### Matrix (production decentralized-encrypted-group-chat benchmark)

**Current state.** Matrix is federated (many homeservers, any can talk to any) with E2EE via Olm (1:1, a Signal Protocol implementation) and Megolm (groups, a sender-keys scheme). On encryption properties, Matrix's own rebuttal to critics is worth quoting precisely: Olm provides perfect forward secrecy as an implementation of the Signal Protocol, a captured current key cannot decrypt previous messages, and a given Megolm key is used for up to 100 messages by default but is reset whenever group membership changes or after a week. Critics counter that Olm/Megolm do not natively conceal who is in a conversation or which devices are involved, and federation activity — who is talking to whom and when — is exposed between homeservers, leaking structural metadata even when content is encrypted.

MLS is in progress, not shipped. The work is tracked publicly (MSC4256 and MSC4244, and the arewemlsyet.com tracker), and as of a [2025 Matrix Conference talk](https://cfp.2025.matrix.org/matrix-conf-2025/talk/BAKSEA/) the integration is still being designed, with the hard open problem being how to keep a federated ecosystem working with an encryption standard not designed for complete distribution, and how to keep MLS state in sync with Matrix room state. This is directly relevant to us: we are adopting MLS into a decentralized topology, and Matrix is discovering that MLS-in-a-distributed-system is genuinely hard.

**What it does better than us.** Production-grade group chat with history, rich clients (Element), real moderation/roles, and a federation model that already works at scale. Multi-device with cross-signing and key backup is more mature than anything pure-P2P offers. History-on-join with backfill is solved (it keeps Megolm keys precisely so users can read back, which most clients do by default).

**What it gets wrong that we must avoid.** Megolm's metadata exposure and the federation-level who-talks-to-whom leakage are exactly what our blind broker is meant to prevent, so Matrix is the cautionary benchmark for "encrypted content is not enough." Megolm's keep-keys-for-history default is in tension with forward secrecy in practice. Homeserver-as-trust-anchor means your server operator sees a lot.

**Validates / challenges our design.** Validates that MLS is the right direction (the whole ecosystem is migrating toward it) and that per-epoch rekey on membership change is the correct group model — Megolm already does a coarse version of this. Challenges us by demonstrating, in real time, that integrating MLS into a non-client-server topology is unsolved engineering, and that history-on-join and forward secrecy pull against each other. We will face the same MLS-state-versus-document-state sync problem that Matrix is wrestling with between MLS state and room state.

### Briar (P2P, Tor, strong threat model — directly relevant to our P2P/offline ambitions)

**Current state.** Briar is purely peer-to-peer with a high-assurance threat model for at-risk users. No SIM, phone number, email, or messenger ID is required; communication runs over Tor and needs no central server, and Briar can also connect directly over Bluetooth or Wi-Fi to nearby devices. Contacts are added by exchanging a random ID, in person via QR or remotely via a time-limited briar:// link. A "Briar Mailbox" (run on your own or a trusted friend's device) provides asynchronous delivery when peers are not simultaneously online. It is feature-rich beyond 1:1 — groups, threaded forums, and blogs.

Two facts are decisive for our analysis. First, multi-device: Briar essentially does not do it. There is no way to transfer your account to another device; if you get a new device or reinstall, you start fresh with a new ID. Second, and relatedly, recovery: there is none, and Briar frames this as a security feature — there is no account recovery phrase to lose. Briar also explicitly does not hide your identity from your own contacts; it provides unlinkability, not anonymity — others cannot find out who your contacts are, but your contacts can find out who you are.

**What it does better than us.** Best-in-class metadata protection via Tor onion routing, true mesh operation with no internet at all (Bluetooth/Wi-Fi direct), and a coherent threat model for adversaries who monitor the network. The Mailbox concept — a user-run, trusted asynchronous relay — is conceptually close to our superpeer broker and worth studying as prior art for "thin always-on relay that is not a central server."

**What it gets wrong / finds painful that we must avoid.** Single-device-only and zero-recovery are usability cliffs. Tor-everything is slow and battery-hungry. Both-peers-online (absent a Mailbox) reintroduces the pure-P2P presence problem. Briar's stance is "we will not compromise the threat model for convenience," which is correct for its high-risk audience and wrong for a mainstream local-first product.

**Validates / challenges our design.** Validates the user-run-relay pattern (Mailbox ≈ superpeer as queue) and that a strong metadata story is achievable if you are willing to pay in latency. Challenges our multi-device ambition starkly: the system in the field with the strongest threat model simply refuses to do multi-device, which is a signal that multi-device and strong metadata/security are in deep tension. Our claim to offer both is the claim Briar declined to make.

### Session (no-phone-number, decentralized — and a corrected premise)

**Current state.** Session routes messages over the decentralized Oxen/Session node network (onion-routing-like, roughly 1,500–2,200 community-run staked nodes) and requires no phone number or email; identity is a keypair, and the "recovery password" is a mnemonic seed. Account recovery is therefore possible in a way Briar's and SSB's are not: because there is no central server storing identity, the recovery password is a mnemonic seed that restores your Account ID to a new device.

The brief describes Session as "Signal-protocol-derived," and that needs correcting as of 2026. Session started as a Signal fork but **moved off the Signal Protocol to its own Session Protocol**, and in doing so **dropped perfect forward secrecy and deniability**. The shift away from the Signal Protocol, especially dropping perfect forward secrecy and strong cryptographic deniability, stirred significant debate; without PFS, a compromised long-term key can decrypt all past intercepted messages. Then it reversed course: Session announced Protocol V2 on December 1, 2025, adding ML-KEM quantum-resistant encryption and bringing back Perfect Forward Secrecy, so a stolen device with current keys cannot decrypt old messages. Stewardship also moved from the Australian OPTF to a Swiss foundation in 2024.

**What it does better than us.** No-phone-number identity with a real, mnemonic-based multi-device and recovery story — arguably the cleanest recovery model in the decentralized set, and directly instructive for our DID recovery design. Decentralized routing with no single operator to subpoena.

**What it gets wrong that we must avoid.** The PFS saga is the lesson: Session shipped a decentralized product for years *without* forward secrecy, took sustained criticism, and only restored it in late 2025. The takeaway for us is non-negotiable: do not trade away forward secrecy for decentralization convenience. MLS gives us PFS and PCS by design, so we should not repeat Session's mistake — but we should note how tempting the trade was for a team optimizing for anonymity and metadata.

**Validates / challenges our design.** Validates that mnemonic-seed recovery can coexist with keypair identity and no central server, which is encouraging for our DID-recovery design. Challenges nothing in our crypto choice; if anything it vindicates choosing MLS over rolling our own, since Session's custom protocol is precisely where it lost (then regained) PFS.

### Mainstream baseline: WhatsApp / Telegram (the expectation-setter)

**Current state, briefly.** WhatsApp is E2EE by default (Signal Protocol, sender-keys for groups) with effortless phone-contact discovery, cloud backup, and multi-device. Telegram is *not* E2EE by default (only opt-in "secret chats" are, and those are 1:1 only); its appeal is UX, large groups/channels, and bots. Both set the mass-market bar: instant onboarding, reliable push notifications, synced multi-device, big groups, rich media, and "it just works."

**Why include it.** Not for protocol lessons but to fix the UX expectation our users arrive with. Any friction we add relative to this bar — QR-scan device pairing, no phone-contact discovery, manual key verification — is friction users will feel. The baseline is the implicit comparison against which "decentralization tax" is measured.

**Inclusions/exclusions justification.** Included Matrix (production decentralized-encrypted-group benchmark), Briar (our P2P/offline cousin with the strongest threat model), and Session (distinct lesson on the PFS-versus-decentralization trade). Excluded Status and XMTP: both are web3-adjacent and would mostly repeat Session's "decentralized keypair identity" lesson without adding a distinct axis insight, so including them would be padding. Mentioned Threema and Cwtch only in passing for the same reason. SimpleX would arguably add a distinct "no persistent identifiers at all" metadata lesson; flagging it as a follow-up if this analysis needs a deeper metadata section. [UNVERIFIED: SimpleX current state not researched for this draft.]

---

## Comparison tables

Columns are the covered solutions plus a final **Our Stack** column with a one-line architectural reason.

### Axis 1 — Usability

| Sub-point | Signal | Delta Chat | SSB | Matrix | Briar | Session | WhatsApp/TG baseline | Our Stack |
|---|---|---|---|---|---|---|---|---|
| Onboarding | Phone number required; very easy | Email/chatmail QR; easy, no phone | Keypair auto-gen; notoriously confusing | Account on a homeserver; moderate | Local nickname+password; easy account, hard contacts | Keypair+mnemonic; easy, no phone | Phone number; trivial | DID provisioning + first device; aim for QR-simple, **unproven** because DID UX is new |
| Multi-device | Primary phone + linked devices, synced send | Transfer-then-diverge via QR on local net; not live sync | Structurally one device per feed; "hell" | Cross-signing + key backup; mature | None; new device = new ID | Mnemonic restores ID to new device | DID authorizes multiple device keys + broker syncs CRDT snapshot; **our hardest unsolved problem** |
| Recovery (device/key loss) | PIN + registration lock | Backup export, or just keep any one device | Identity effectively lost; re-introduce | Key backup recovers history | None (framed as a feature) | Mnemonic seed restores account | DID key rotation + broker re-attestation; **designed but not specified** |
| Day-to-day feel | Excellent, reliable | Good; email latency for some paths | Slow sync, rough | Good (Element); occasional key errors | Slow (Tor), battery-heavy | Slower network, rougher UX | Gold standard | Target Signal-class when broker present; degraded but functional when peers co-present only |
| Contact discovery | Phone contacts + username | Email address / QR invite | Out-of-band pubkey exchange | Matrix ID / directory | QR in person or time-limited link | Account ID / QR | Phone contacts | DID handle + QR; atproto handle for public identity, no phone-contact graph |

### Axis 2 — Security

| Sub-point | Signal | Delta Chat | SSB | Matrix | Briar | Session | Our Stack |
|---|---|---|---|---|---|---|---|
| E2EE by default | Yes | Yes (PGP subset) | Encrypted DMs; feeds are public-signed | Yes (must enable per room historically) | Yes | Yes | Yes (MLS for groups) |
| Forward secrecy | Yes (Double Ratchet) | Realtime path yes (QUIC); PGP mail path no | Largely absent | Olm yes; Megolm coarse (rekey on membership/week) | Yes | Dropped, then restored in V2 (Dec 2025) | Yes (MLS per-epoch rekey) — **the deliberate non-repeat of Session's mistake** |
| Post-compromise security | Yes | Limited | No | Olm yes; Megolm partial | Yes | V2 reintroduces | Yes (MLS PCS on epoch change) |
| Group encryption approach | Sender keys over pairwise | PGP to recipients | Per-feed; no modern group scheme | Megolm sender keys | Group keys | Sender keys | **MLS RFC 9420, per-epoch rotation on membership change** |
| Metadata exposure | Sealed sender (app layer); IP/timing ongoing | Email provider sees who/when | No central server, but feeds are widely replicated | Homeserver + federation see graph | Best in class (Tor) | Onion-routed, minimal | Broker is cryptographically blind by design; **IP/timing layer is our weakest point, like everyone's** |
| Threat model / who you trust | Signal's servers (metadata) + protocol | Your email provider + protocol | Your peers/pubs | Your homeserver + federation | Tor + your contacts | Oxen node network | Broker operator for availability only, not content; DID method anchor for identity |
| Identity authenticity | Safety numbers | QR / verified keys | TOFU on pubkey | Cross-signing / SAS | QR / link | Account ID exchange | DID + key verification (QR/SAS-style); **verification UX TBD** |
| Openness / audited | Open source, audited | Open source, multiply audited (incl. rPGP) | Open protocol, partial audits | Open, audited | Open, audited | Open, audited (Quarkslab) | Open intent; **no audit yet — must plan one** |

### Axis 3 — Capability

| Sub-point | Signal | Delta Chat | SSB | Matrix | Briar | Session | Our Stack |
|---|---|---|---|---|---|---|---|
| Group chat / size | Yes, large groups | Yes (email-based) | Channels/social, not strong group chat | Yes, very large, roles/moderation | Yes, plus forums | Yes, smaller | Yes; MLS scales logarithmically with membership |
| Membership-change handling | Sender-key rotation | Email recipient list | Weak | Megolm rekey on change | Group re-key | Group re-key | **MLS epoch rotation — the core strength** |
| History on join | Forward-only | Email history present | Full feed backfill | Backfill (keys kept) | Limited | Limited | CRDT snapshot defines join state; configurable backfill via broker snapshot |
| Media / rich content | Full (voice, video, reactions) | Images, files, webxdc apps | Images/blobs | Full | Files, limited media | Files, voice, video | Encrypted content-addressed blobs via iroh-blobs, referenced from document |
| Offline / local-first | Weak (server-mediated) | Strong (email store-and-forward) | Strong (designed for it) | Server-mediated | Strong (mesh + Mailbox) | Node-network-mediated | Strong by design: CRDT local writes, broker queues, P2P when co-present |
| Realtime (presence/typing/receipts) | Yes | Mostly no (deliberate) | No | Yes | Limited | Limited | Optional over iroh realtime; metadata-light by choice |
| Scale / performance concern | Server-scaled | Email-scaled | **Unbounded log growth — cautionary** | Megolm/federation cost at huge scale | Tor latency | Node-network latency | CRDT change growth + snapshot compaction; **must actively manage, this is SSB's trap** |

### Summary table — "Lesson for us"

| Solution | Key strength to learn from | Key failure/pain to avoid | Validates or challenges our design |
|---|---|---|---|
| Signal | World-class onboarding + linked-device sync; shipped sealed-sender metadata story | Centralization; phone-number-rooted identity | Validates "centralization buys UX"; challenges us to match linked-device quality without a mandatory primary |
| Delta Chat | No-phone identity on existing infra; iroh in production; P2P-for-realtime + store-and-forward-for-durable split | Email metadata leak; transfer-then-diverge multi-device on local net; iroh proxy-bypass gotcha | Validates iroh + our realtime/durable split; confirms a durable queue (our broker) is needed, not optional, for multi-device |
| SSB | True zero-infra P2P; clean permissionless keypair identity; real offline-first | Single-device-tied identity; no recovery; unbounded logs; weak FS; onboarding friction | The whole bet's test case — validates our diagnosis, and our DID/MLS/CRDT/broker answers each failure *on paper* |
| Matrix | Production group chat with history, roles, mature multi-device | Megolm metadata leak; federation graph exposure; history-vs-FS tension | Validates MLS direction + per-epoch rekey; challenges us that MLS-in-distributed-topology is unsolved engineering |
| Briar | Best metadata protection; true mesh; user-run Mailbox relay | Single-device only; zero recovery; Tor latency; both-peers-online | Validates user-run-relay pattern (≈ superpeer); challenges our claim to do multi-device AND strong security, which Briar refused |
| Session | No-phone identity with mnemonic recovery + multi-device | Dropped PFS for years before restoring it | Validates mnemonic-style recovery for keypair identity; vindicates choosing MLS over a custom protocol |
| WhatsApp/TG | The mainstream UX bar (instant onboarding, sync, push, big groups) | Centralization; TG not E2EE by default | Sets the "decentralization tax" yardstick our users will feel |

---

## Synthesis — lessons mapped to our stack

### The local-first / P2P cautionary tales (most important)

SSB paid in full for the lessons we are about to face. Mapping each concretely to our design, and being honest where we inherit the same risk:

**Multi-device hell.** SSB's identity is one keypair writing one append-only feed, and a feed cannot be safely written from two devices, so every device is a separate identity and "fusion identity" was a bolt-on that never finished. Our escape is that **a DID is not a feed**. A DID can authorize multiple device keys without any device "owning" the log, and Automerge is multi-writer by construction, so concurrent writes from a user's two devices merge rather than fork. This is a real structural advantage SSB never had. **The risk we still inherit:** authorizing, listing, and *de-authorizing* device keys under a DID is exactly the operation fusion identity botched (it could add but only tombstone, never cleanly remove). If our DID method does not support clean key revocation with propagation, we reproduce the fusion-identity dead-end. This is unspecified in our design today.

**Identity-recovery dead-ends.** SSB: lose the key, lose the identity. Briar: same, by choice. Our escape is DID key rotation plus a broker that can re-attest a new device into existing MLS groups. Session proves a mnemonic-seed recovery is viable even without a central server, which is a model we should study directly. **The risk we still inherit:** recovery is only as good as the DID method's key-rotation and the social/group re-attestation flow, and we have specified neither. If a user loses all devices and we have no recovery anchor (a seed, a social-recovery quorum, or a broker-held encrypted backup), we are back to SSB's dead-end. **We do not yet have a real answer to total-device-loss.** Flagging this as the single largest open question.

**Log / replication scaling.** SSB feeds grow forever and replication gets heavy; partial replication and meta-feeds came late and are now archived. Our escape is Automerge snapshots: the broker holds a compacted document, so a returning device pulls a snapshot, not a full-history replay. **The risk we still inherit:** CRDTs accumulate change/operation metadata, and Automerge documents can grow if not compacted. We must treat snapshot/compaction as a first-class requirement, not an afterthought — this is precisely the corner SSB cut.

**Offline-first-but-unusably-so.** SSB's purity produced slow sync and confusing "where are my messages" states. Our escape is the always-on broker as rendezvous and queue, which means a peer does not need a co-present friend or a pub to make progress. This is the cleanest win in the whole design. **The risk we still inherit:** when the broker is absent and only co-present P2P is available, we degrade toward SSB's experience, so the "superpeer-optional" framing must be honest that the optional case is the degraded case.

The bet, tested: **a thin always-on broker plus Automerge plus MLS does plausibly escape SSB's specific failure modes — but only multi-device and recovery, the two hardest, remain genuinely unsolved in our design, not just SSB's.** The broker and CRDT clearly fix onboarding friction and log-replay pain. MLS clearly fixes the group-rekey and forward-secrecy gaps. DIDs clearly *enable* a better multi-device and recovery story than a device-tied keypair. But "enable" is not "implement," and the gap between the fusion-identity spec and a working fusion identity is exactly the gap we have not yet crossed.

### The usability-vs-decentralization frontier

Plotting the field on the centralization-to-P2P spectrum against UX:

- Signal / WhatsApp: centralized, top UX.

- Matrix / Session: federated or node-decentralized, good-but-rougher UX.

- Delta Chat: infrastructure-riding (email), good UX with a multi-device asterisk.

- Briar / SSB: pure P2P, UX cliff.

Our superpeer-optional model aims to sit where Delta Chat and a hypothetical "Matrix without homeserver metadata leak" overlap: decentralized substrate, but a thin always-on element that buys back the UX that pure-P2P loses. **Is that position achievable, or are we underestimating the UX cost?** The evidence says the position is achievable *when the broker is present* — Delta Chat shows infra-assisted decentralization can feel fine. The risk is the "optional" claim. Every system that achieved good UX did so by making *some* always-on element effectively mandatory (Signal's servers, Delta's email provider, Briar's Mailbox for async, Matrix's homeserver). The honest read: our broker will be *de facto* mandatory for mainstream-acceptable UX, and "optional" describes a graceful-degradation mode, not the common path. That is fine, but we should say it plainly rather than imply pure-P2P-with-no-broker is the normal experience.

### The metadata / blind-broker comparison

Best metadata stories in the field, and their cost:

- **Briar (Tor):** strongest, paid in latency and battery and single-device-only.

- **Signal (sealed sender):** strong at the app layer, shipped, but Signal itself admits IP/timing correlation is unsolved.

- **SSB (no server):** no central observer, but feeds are widely replicated, so "who follows whom" is public.

- **Session (onion routing):** strong, paid in network speed.

Our blind-broker claim — the broker sees ciphertext and routing metadata but not content or group semantics — is realistic at the *application/broker* layer and comparable to sealed sender in spirit. **Where it is not realistic, and where we must be honest:** the broker still sees IP addresses, connection timing, and message sizes/volumes unless we add Tor-style routing or padding. That is the same residual leak Signal openly acknowledges. Delta Chat's iroh-ignores-proxy gotcha is a concrete warning that a transport can silently undermine a privacy claim. So our claim should be scoped: "blind to content and group membership; not, by itself, resistant to network-level traffic analysis." Achieving Briar-grade metadata protection would cost us Briar-grade usability, which contradicts our UX goal — that trade is real and we cannot claim both.

### Multi-device & recovery (the hardest usability × security problem)

The field's approaches, synthesized:

- **Primary + linked (Signal, WhatsApp):** works well, but needs a primary device and server mediation.

- **Cross-signing + key backup (Matrix):** mature, history recoverable, complex.

- **Transfer-then-diverge (Delta Chat):** simple, but devices become independent, no unified live state, local-network requirement.

- **Mnemonic seed (Session):** clean recovery, keypair identity, no central server.

- **None (Briar, SSB):** new device = new identity.

Our plan — DID authorizing multiple device keys, MLS adding each device as a group member, broker syncing CRDT snapshots — is, on paper, the most capable of these: unified multi-writer state (better than transfer-then-diverge), no mandatory primary (better than Signal), real recovery (better than Briar/SSB). **Assessed honestly against the field, we have a strong design and no proven implementation.** The two unspecified pieces are (1) DID device-key add/revoke with propagation, the exact thing fusion identity failed at, and (2) total-device-loss recovery, where we should adopt either Session's mnemonic model, a social-recovery quorum, or a broker-held encrypted-backup model — and we have chosen none. Until those are specified and tested, multi-device and recovery are our two biggest open risks, and we should not claim them as solved.

### Delta Chat as our closest technical cousin

Transferable, in priority order:

1. **The realtime/durable split.** Delta uses iroh (ephemeral, forward-secret, QUIC) only for realtime channels, and store-and-forward (SMTP) for anything that must survive multi-day offline gaps. Our analogue: iroh realtime for presence/typing/live editing, broker queue for durable message and snapshot delivery. This validates putting durable state on the broker, not on opportunistic P2P.

2. **Ephemeral-identity realtime pattern.** Delta's `joinRealtimeChannel()` with a random 32-byte topic id, key shared in-band, lazy P2P bootstrap, is a clean pattern for our realtime layer that we can mirror closely.

3. **No-phone identity that mainstream users accept.** Delta proves email-as-identity works; we are betting DID-as-identity can clear the same bar. The lesson is that the *contact-add flow* (QR invite) matters more than the underlying identifier.

Where we diverge and why: Delta declined P2P for durable multi-device sync (chose SMTP), confirming our instinct that the broker should own durable sync; Delta's group model inherits email's awkwardness, whereas MLS gives us proper group semantics; and Delta's transfer-then-diverge multi-device is explicitly *not* what we want — we want unified live state via CRDT, which is harder but better. Also carry forward the proxy-bypass warning: verify our iroh configuration honors the user's network-privacy settings, or document clearly that it does not.

### Differentiators — what we could do genuinely better than all of them

- **CRDT-based conflict-free concurrent group state.** No system in the field gives a shared, concurrently-editable group document with automatic merge. This is a capability beyond chat — shared lists, collaborative state, offline edits that merge — that none of Signal/Matrix/SSB/Briar/Session offers.

- **Public/private content lifecycle via atproto.** Public social content flows through AT Protocol while private group messaging stays on the encrypted P2P path. A single identity (DID) spanning both is a model no competitor has.

- **True serverless operation when peers are co-present.** Like SSB/Briar but without their UX cliff, because the broker is there for the non-co-present case.

- **User-run infrastructure.** Like Briar's Mailbox but as a richer rendezvous/queue/snapshot store, lowering the barrier to self-hosting the always-on element so it is not operator-controlled the way Signal's servers are.

---

## Prioritized conclusions

**Top design lessons to adopt:**

1. Keep the realtime/durable split (Delta Chat): iroh for ephemeral realtime, broker queue for durable state and snapshots.

2. Make Automerge snapshot/compaction a first-class, non-negotiable requirement (SSB's log-growth trap).

3. Use MLS's per-epoch rekey as the group-security core, and never trade away forward secrecy for decentralization convenience (Session's mistake).

4. Adopt a mnemonic-seed or equivalent recovery anchor for keypair/DID identity (Session's clean recovery model).

5. State plainly that the broker is de-facto-mandatory for mainstream UX and "optional" means graceful degradation, not the common path.

**Top failure modes to actively defend against (drawn especially from SSB):**

1. Device-tied identity with no clean key revocation — specify DID device add/revoke now, or repeat the fusion-identity dead-end.

2. Total-device-loss with no recovery anchor — SSB's and Briar's dead-end; choose a recovery model.

3. Unbounded state growth — Automerge change metadata is the new append-only log if not compacted.

4. Silent transport-level privacy leaks — verify iroh honors network-privacy settings (Delta's proxy-bypass gotcha).

5. Overclaiming metadata protection — scope the blind-broker claim to content and group membership, not network traffic analysis.

**Top unresolved questions this analysis surfaced:**

1. **Multi-device:** what is the concrete DID device-key authorization, listing, and revocation flow, and does our chosen DID method support clean revocation with propagation? (The fusion-identity failure point.)

2. **Recovery:** what happens on total device loss — mnemonic seed, social-recovery quorum, or broker-held encrypted backup? We have chosen none.

3. **MLS in a distributed topology:** how do we keep MLS group state in sync with Automerge document state, the exact problem Matrix is still solving for MLS-in-federation?

4. **Metadata at the network layer:** do we add padding/mixing/onion-routing, accepting the latency cost, or do we scope the privacy claim to exclude traffic analysis?

5. **Audit:** none of our crypto integration has been audited; every credible system in the field has been. Plan one before any production claim.

---

## Sources

Inline citations above link to primary sources. Principal references:

- Signal: usernames and phone-number privacy (signal.org/blog), sealed sender (signal.org/blog/sealed-sender), Signal support docs on protection and linked devices.

- Delta Chat: iroh solutions page (iroh.computer/solutions/delta-chat), realtime P2P announcement (delta.chat/en/2024-11-20-webxdc-realtime), Rust API docs for peer_channels (rs.delta.chat), FAQ on Add Second Device and backups (delta.chat/en/help), community threads on iroh proxy behavior and local-network requirements (support.delta.chat), third-party technical write-up (blog.feld.me).

- SSB: protocol guide and ssb-db (ssbc.github.io), fusion-identity-spec and meta-feeds-spec (github.com/ssbc), NGI Pointer audit report on partial replication and fusion identity, Manyverse "My last update" (manyver.se/blog/2024-04-05), Patchwork deprecation (github.com/ssbc/patchwork).

- Matrix: E2EE implementation guide and "Dispelling myths" (matrix.org/blog/2025/06/dispelling-myths), MLS integration talk (Matrix Conf 2025), arewemlsyet.com.

- Briar: project site (briarproject.org), freie-messenger and Satscryption write-ups on threat model and contact/recovery behavior, moddedbear on single-device limitation.

- Session: getsession.org FAQ and protocol pages, Wikipedia "Session (software)", Protocol V2 announcement coverage (Dec 2025), OSINT Team technical review.

- Mainstream baseline: general knowledge of WhatsApp (Signal Protocol, default E2EE) and Telegram (opt-in secret chats only); not separately researched for this draft.

[UNVERIFIED] markers in text indicate where SimpleX and a few product-level details were not researched for this draft and should be confirmed before relying on them.
