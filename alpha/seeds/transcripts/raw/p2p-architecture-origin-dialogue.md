# Raw transcript — P2P architecture origin dialogue (2026-06-02 to 06-03)

source: the originating "could I build a 3-way P2P chat" conversation, re-dropped 2026-06-15.

> **Fidelity:** this is a **preserved-condensed** rendering (a faithful readable rendering of the
> pasted dialogue, not the exact verbatim paste) — the decision arc and the load-bearing
> exchanges are kept; assistant web-search citations and long tangents are compressed. A verbatim
> re-drop can replace this file. This is the earliest seed — it predates the
> `design-dialogue-2026-06-13-to-14.md` and is where the stack (iroh + Automerge + Willow + MLS +
> coop) and the economics (mutual credit + consumer-pull ads + Lightning) were first reasoned out.

---

## The opening question and the stack discovery

**Q: Could I write a 3-way P2P app where chat responses sync rather than going via a central API?**
Yes — well-trodden. WebRTC DataChannels (full mesh for 3 peers) + CRDTs (Yjs/Automerge) for the
sync. The one caveat: a signaling step is still needed to bootstrap WebRTC (can be minimal, even
manual copy-paste). After that, all data flows P2P.

**Q: Android + macOS + web clients? And you said "common pattern" — examples?**
Walked back "common": P2P+CRDT chat is well-explored in research, fewer in production. Real
examples: **Briar** (Android, Bramble/Tor/BLE), **Berty** (libp2p), **Manyverse** (SSB), **Tox**,
**Syncthing** (sync, not chat). Cross-platform native is the hard part — WebRTC unifies browsers
but native Android/macOS needs a shared transport. Candidate stack: libp2p (Rust core + FFI) or
**iroh**; Automerge (Rust core → JS/Kotlin/Swift) as the CRDT; native UIs. Real cost isn't the
protocol — it's NAT traversal, offline sync/reconnection, and maintaining 3 UI codebases.

**Q: Could I use iroh?** Strong fit, maybe better than libp2p. iroh = QUIC P2P keyed by Ed25519
pubkey, near-universal connectivity (direct → hole-punch → relay fallback), E2E encrypted by
default. Maps to chat: `iroh` core (connections), `iroh-gossip` (pub/sub topic = chat room),
`iroh-docs` (eventually-consistent KV for history). FFI bindings (Swift/Kotlin) exist but are
**tier 2**. **The web client is the hard part** — browsers can't do raw QUIC.

## Automerge, examples open-source, and data modeling

The example apps are all open source: Briar (GPL, no iOS planned), Berty, Manyverse, Tox.
**Automerge**: created 2017 (Ink & Switch + Kleppmann), implementing the 2019 *Local-First Software*
essay's philosophy (you own your data). Immutable operations with unique IDs; never deletes
(tombstones); deterministic conflict resolution; scales to millions of changes; Automerge 3.0 cut
memory ~100x via columnar compression. For append-only chat it may be overkill — `iroh-gossip` +
`iroh-docs` could suffice. Use Automerge when you want editable messages / collaborative state.

**Compaction / rolling truncation:** yes, exists — serialize current state without history. The
tradeoff: a compacted doc can't sync with one that lacks a common ancestor (need full state
transfer). **Local-first Instagram → one document per post** is the right instinct (sync
granularity, independent compaction, per-doc access control, smaller memory). Challenge becomes
discovery/indexing (no central DB query) — that's where gossip / a lightweight notify relay fits.

## iroh primitives, then Willow

Walked the iroh stack: `iroh` (Endpoint, EndpointId=Ed25519 pubkey, Router/ALPN, NAT traversal,
DNS/PKARR discovery), `iroh-blobs` (BLAKE3 content-addressed verified streaming, kB→TB),
`iroh-gossip` (HyParView+PlumTree epidemic broadcast, phone-friendly), `iroh-docs` (LWW KV over
blobs+gossip), `iroh-willow` (in construction). Composition for the chat app:
iroh → gossip (realtime) → blobs (attachments) → docs (persistent history).

**Willow** (from Earthstar, implemented by iroh-willow): 3D data model (namespace + subspace +
path + timestamp), **Meadowcap** capabilities (delegatable, scopeable, expirable read/write),
**true deletion** via prefix pruning (the answer to CRDT tombstone bloat), private-intersection
sync, resource budgets for mobile. The big wins over iroh-docs: real access control + real
deletion. **Status: not released** (iroh-willow pinned to iroh 0.34 vs 0.97 current; Confidential
Sync spec back to Proposal). Don't block on it — build on iroh-docs now, design Willow-shaped,
migrate later (nontrivial). For early testing, willow-rs / willow-js (Earthstar) are more active.

## Relays, the SYN-cookie rendezvous, and the superpeer

**Q: Do I need long-running public relays for NAT'd/mobile peers?** iroh hardcodes free n0 public
relays (fine for small scale); relays do NAT traversal + encrypted fallback, never see plaintext.
For 3 people the public relays are likely fine indefinitely; a relay is a cheap single binary
(DERP-derived).

**Q: Could a static site (WASM) do stateless rendezvous — devices converge on a "quick meet
space" instead of an active relay? (like SYN cookies — encode state into the message so the server
holds none)** The analogy lands. A stateless rendezvous = signed self-expiring "rendezvous tokens"
(STUN-discovered address + timestamp + pubkey, signed) published at a deterministic key derived
from a shared group secret, to a bulletin board you don't operate (Nostr relay, BitTorrent DHT,
PKARR/DNS — iroh already uses PKARR this way). STUN reflection is the one unavoidable stateless
exchange. The genuine wall: **all-symmetric-NAT** simultaneously → someone must forward packets.

**Q: What about public peers in the mesh as larger always-online members — lovely-to-have, not
have-to?** Yes — the **superpeer / SSB "pub" / Briar "mailbox"** pattern. It speaks the full app
protocol (not a dumb relay), holds the full replica, is the always-awake rendezvous + durable
queue, and degrades gracefully if it goes offline. More aligned with P2P than the relay model. In
iroh it's just another Endpoint running gossip+blobs+docs on a public IP. → SSB and Briar deep-dive
(append-only feeds vs Bramble delay-tolerant DAG; pubs vs mailbox). → broader lineage map
(BitTorrent → IPFS/libp2p → iroh; SSB → Earthstar → Willow; Ink&Switch → Automerge; Tailscale DERP
→ iroh relay; Nostr; CouchDB/Replicache/sync-engines).

## The tiered HA-peer backup, and PDS convergence

**Q: A peer with a "taint" that only peers with my devices — my remote backup + content server,
weak peers keep only limited media.** This is personal-cloud topology: home node (full replica,
public, serves blobs to friends so your phone can sleep), weak peers (recent + thumbnails), others
pull media from your home node not your phone. Maps onto iroh-blobs lazy fetch + gossip topic
segmentation (private device-sync topic + group topic) + Willow Meadowcap scoping. The "taint" =
connection filter / owned namespace.

**Q: Could the HA peer also be my Bluesky PDS?** Yes — natural convergence. A PDS already does what
the home node does (hosts your repo, manages keys, represents you when devices are offline). Run
both on one box, sharing storage; an AI agent could bridge private P2P ↔ public atproto (draft
posts, surface replies). ⚠️ flagged: the PDS holds your signing key and is internet-reachable; an
agent with write access widens the blast radius — isolate capabilities from the key.

## Encrypted-at-rest, mule modes, and Signal's standard

**Q: Encrypted at rest on the HA peer; Willow backing up namespaces it isn't a participant in;
multi "mule" modes — straight encrypted-block replication through an HA node, destroyed on
endpoints.** Three trust modes: (1) **encrypted-at-rest on your own node** — encrypt before
iroh-blobs (hash is of ciphertext, lose dedup), works today; (2) **Willow blind-relay** — Willow's
own analysis: payloads/paths/subspace-IDs can be encrypted/pseudonymized, **timestamps must stay
plaintext** for join computation (bucket/jitter them), nobody's built encrypted Willow sync yet
(research); (3) **pure mule mode** — opaque content-addressed blocks, endpoints hold all sync
metadata, HA node is a dumb block store (can't do smart sync). Hybrid: tag data by trust tier.
⚠️ traffic analysis still possible — needs padding / cover traffic / onion routing (why Briar uses
Tor).

**Q: What would it take to meet Signal's standard?** Signal = X3DH + Double Ratchet (new key per
message), forward secrecy, post-compromise security, deniability, async via prekeys, PQXDH. Gap:
iroh gives session-level TLS, not per-message ratcheting. To close: integrate Double Ratchet
(libsignal/vodozemac) at the payload layer; HA node serves prekeys; QR key verification; MLS or
Sender Keys for groups; encrypted-at-rest; sealed-sender equivalent; cover traffic; formal audit.
**The honest framing:** wire-level Signal-equivalence is integration of audited libraries; the
P2P-specific parts (serverless prekeys, out-of-order gossip delivery, metadata protection) are
where the real engineering lives. And — Signal's weakness is **organizational** (one company, one
jurisdiction, one app-store listing, compellable). A no-central-operator, transparently-offline
system is a **different and in several dimensions stronger** posture, not weaker.

## The economic layer: credit, reputation, consumer-pull ads

**Q: A credit-building system aligned to PDS identity — value exchange via an existing respected
crypto, plus a reputation/points layer feeding a consumer-driven ad exchange where users opt into
ad types and get paid directly, inverting Google.** Three separable systems sharing the existing
Ed25519/DID identity (no new identity, no new token):
1. **Direct value** = Lightning (Bitcoin) — Nostr "zaps" prove identity-linked micropayments; HA
   node runs LND/CLN.
2. **Reputation** = locally-computed Web-of-Trust (node uptime attestations, content contribution,
   social vouching) — subjective, local, no global consensus/blockchain.
3. **Consumer-pull ad exchange** — advertisers publish offers to a namespace; the user's client
   matches **locally** against *declared* (not surveilled) preferences; viewing emits a signed
   attestation; advertiser pays the user directly via Lightning. ⚠️ proof-of-attention is the
   unsolved-without-a-controlled-runtime problem; reputation-gating is the main defense.

**Q: Could Holochain-style accounting help?** Yes — borrow patterns, not the runtime.
Agent-centric source chains (= Willow subspaces), **countersigning** (both parties sign an
attestation → solves ad-fraud), per-namespace validation rules, fork detection (= double-spend),
**mutual credit** (value created by the transaction, no minting — storage/relay/attention credits,
bridge to Lightning to cash out). No blockchain, no global consensus, no new token.

**Q: A non-central ad broker = a set of Holo-ish nodes; many possible; choice is consumer-side
(inversion); unpopular gating providers get ignored; Grandma uses a list curated by her adult
kids; namespace-scoped recognition (fantasy football, kudos) with no rewind.** Yes — the broker is
a **replaceable service, not a platform** (choose your ad provider like your SMTP server). Family
trust chain = Meadowcap delegation. Namespace-scoped recognition systems (fantasy league, workplace
kudos, community reputation, OSS contribution) as countersigned append-only entries. "No rewind" =
recognition is honest, attributable, unfalsifiable. The philosophical inversion: users own and sell
their attention on their terms; the long pole is more ethical *by structure*, not idealism.

## The cooperative and the governance lifecycle

**Q: Pair this with a coop — not a typical nonprofit — that offers HA PDS for friends/family/
namespaces; member like a credit union, not a club; a social utility structured not to extract but
to reinforce.** Credit-union analogy is precise. Not nonprofit (grant-dependent), not startup
(investor extraction). A **cooperative**: member-owned, self-sustaining on dues, surplus
reinvested. Operates: managed HA nodes, OSS dev, an (optional, competing) ad-broker node,
onboarding/support. Legal: LCA + wholly-owned PBC (the Subvert pattern) or a consumer co-op.
Members own data/keys; the coop operates hardware. Ad revenue flows to members, coop takes a small
transparent fee.

**Q: Hard-limited early stewardship (founders drive until ~10yr / quantified threshold / 80%
break-glass); a capped founder return so the typical exit isn't there but early stewards aren't
crushed by opportunity cost; enshittification-proof with real UX-stability guarantees; surplus past
an endowment cap can only be spent on xyz or returned to members.** This is a **governance
lifecycle**, not just a structure. Phase 1 stewardship (founders with board authority, advisory
members, transition trigger compound: earlier of N years OR member+self-sufficiency threshold, 80%
break-glass), Phase 2 transition (board expands, founder seats convert to elected terms).
Anti-enshittification as **binding charter** (6-month deprecation cycles, 18-month LTS, no dark
patterns, data portability, endowment cap, ad-revenue transparency). Separation of concerns:
technical / product / governance / mission decisions have different authority models so early
agility isn't strangled by democracy.

**Q (founder return):** settled on a **royalty model** — small % of gross revenue, paid only after
a sustainability trigger, never squeezing (3% of nothing is nothing), proportional to success but
muted, possibly capped/expiring. Legally framed as a **founder's royalty / technology licensing
fee** (an operating expense), not a return on capital — compatible with cooperative limited-return
norms, transparent and legible to members. ($5k→$20k base; long-tail acknowledgment, not
get-rich.)

## The honest tech-feasibility pass

**Q: Walk back through and find where we're too optimistic; biggest unknowns; MVP/MLP ordering;
how realistic.** Accurate: iroh core/gossip/blobs (production), self-hosted PDS, coop legal
structures, Lightning. **Too optimistic:** cross-platform iroh (FFI tier-2, **web client = no
clean path, browsers can't do QUIC**), iroh-willow maturity (can't ship on it in 2026), encrypted
mule mode (research, no reference impl), Signal-grade ratchet-over-gossip (out-of-order/duplicate
delivery edge cases), Holochain-style countersigning (build from scratch), ad exchange
(proof-of-attention unsolved). **Genuine unknowns:** will iroh reach 1.0 / will Willow ship; mobile
NAT on carrier-grade NAT; mobile battery/background. **MVP order:** Phase 0 prove the pipe (CLI, 2
peers + 1 HA) → Phase 1 basic chat (Android+macOS native, gossip+blobs+docs, HA relay) = the MLP →
Phase 2 security (Double Ratchet, prekeys, QR verify, encrypted-at-rest) → Phase 3 PDS → Phase 4
Willow migration → Phase 5 economic layer → Phase 6 metadata-blind/sealed-sender (ongoing).
**Realistic:** core chat (0-3) = 9-12 months, 2-3 engineers; full vision = 3-5 years. It's not much
*new* tech — it's the **alignment of pieces from separate worlds**. Only genuine research piece is
metadata-blind encrypted sync, and it can be deferred.

## The cross-platform reckoning (the hard part)

**Q: The web thing seems inevitable — a browser can't be a full persistent peer; maybe the coop
provides web access / a bridge.** Right — accept the bridge. Browser ↔ HA node over WebSocket/
WebTransport; HA node bridges to the iroh mesh; the browser holds its own ephemeral key so E2E
holds and the bridge stays blind. Park as a coop service, phase 3-4.

**Q: iOS has to be on the table — and Android background. Is iroh enough or do we think smaller /
look elsewhere?** Honest: **neither iroh nor libp2p has a production-ready iOS story.** iroh-ffi
Swift is ~0.35 vs 0.97 core (deprioritized); swift-libp2p is explicitly experimental; go-libp2p on
iOS (Berty/gomobile-ipfs) was archived. Both iOS (APNs) and Android (FCM/Doze) **require push** for
background delivery — the HA node is the natural push relay (content-free wake signal, E2E intact).
This is another argument for the coop (centralizes the Apple/Firebase operational overhead).
**Recommendation:** write a thin Rust core (quinn QUIC + your own minimal gossip/sync, borrowing
iroh's designs), compile to iOS/Android via UniFFI (the RustDesk / Mullvad / libsignal pattern),
add iroh/libp2p later if it matures. Control over the mobile surface > ecosystem support, when
mobile reliability is the critical path.

## The project survey (alternatives, health, alignment)

**Q: Reticulum?** Python, extreme transport-agnosticism (LoRa/packet-radio/BLE/TCP/I2P, 5 bps
floor), no source addresses on packets (strong metadata protection), LXMF messaging +
**propagation nodes = your HA peer**, Sideband/MeshChat ship (iOS TestFlight). Concerns: Python on
mobile is heavy/Kivy-UI; **founder (Mark Qvist) departed Dec 2025**; no structured sync / blobs.
Best as the extreme-offline transport tier or a reference architecture.

**Q: Comprehensive search for others.** Surfaced — **Veilid** (cDc; Rust; iOS+Android+WASM from one
codebase; Tor-like private routing + DHT; **the cross-platform gap-closer on paper**, but slow
post-DEF-CON velocity, ~1-2 people, flagship still a PoC), **Keet/Holepunch** (1M+ downloads,
**ships on iOS at scale**, JS/Bare, Tether-funded), **Offline Protocol/Fernweh** (300k users,
mesh+identity+payments, VC-funded), **Waku** (Status, store-and-forward, Web3-coupled), **XMTP**
(MLS groups, **Rust-core → Kotlin/Swift SDK = the template to copy**, Web3), **Quiet** (Tor+OrbitDB,
insular-communities = namespaces), **Spritely/OCapN** (object-capability + Hyptis financial
primitives = the capability/economic layer, Scheme — watch the spec), **BitChat** (Dorsey, BLE
mesh + Nostr = validates local-mesh+relay), **Meshtastic** (LoRa hardware tier).

**Health/feasibility verdict:** **build your own thin Rust core** (quinn + minimal gossip/sync,
UniFFI to native — study XMTP's LibXMTP binding architecture). **Study deeply:** Veilid
(architecture), Keet (proof P2P chat ships on iOS), Reticulum/LXMF (propagation nodes = HA peer),
OCapN spec (economic layer). **Watch** Veilid's velocity. **Consider** Reticulum as the
extreme-offline tier.

## The closing note

**"Every revolution has a maintenance phase"** (read on a change-management slide, unattributed).
The survey is littered with maintenance-phase casualties — Tarr moved on, Qvist departed,
gomobile-ipfs archived, the Earthstar/Manyverse maintainers walked away, Veilid's velocity stalled.
**The cooperative is not the revolution — it is the maintenance plan.** The charter, dues, royalty,
governance lifecycle, and endowment cap all exist to answer the question that kills OSS projects:
*"who's still here in year seven, and why?"* (Quote provenance unverified; closest kin: Kafka's
"the Revolution evaporates and leaves behind only the slime of a new bureaucracy," Brinton's
Thermidor/convalescence, Weber's routinization of charisma, Kotter's step 8.)
