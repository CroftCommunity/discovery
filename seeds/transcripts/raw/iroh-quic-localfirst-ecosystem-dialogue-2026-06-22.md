# Raw transcript — Iroh / QUIC / local-first stack & ecosystem (Gemini)

source: design-exploration dialogue, pasted 2026-06-22 · **cleaned-paste, content-faithful —
NOT a byte-pristine export** (PLAYBOOK §4). One movement of a longer continuous Gemini session;
filed as four topic files (this = Body 1). Companion bodies: `opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md`,
`cooperative-social-union-governance-dialogue-2026-06-22.md`,
`atproto-pds-architecture-technical-dialogue-2026-06-22.md`.

> **Provenance & verification status (READ FIRST).** This is an AI-assistant (**Gemini**) dialogue;
> the user has flagged Gemini as sometimes unreliable. Every substantive assertion is being
> **fact-checked 2026-06-22** at the user's request — see companion
> **`iroh-quic-localfirst-ecosystem-dialogue-2026-06-22-FACTCHECK.md`** for verdict tables with
> source URLs. **Do not read the dialogue's assertions as fact without consulting the fact-check.**
> Gemini's failure mode here, as in the existing atproto dialogue, is *provenance/packaging drift*:
> invented codenames, mis-attributed or fabricated crate/project names, first-party-vs-community
> confusion, overstated specifics around a largely-accurate skeleton. Treat every crate name,
> codename, "Project X is built on iroh" claim, and exact API as **suspect until the FACTCHECK
> confirms it.** Do **not** copy code snippets as working code.
>
> **For atproto / iroh / iOS-P2P facts, cite the project source of truth
> (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`), do not re-derive** — notably iroh `1.0.0`,
> iroh-docs = range-based set reconciliation + LWW (**not** Merkle Search Trees), `NodeId`→
> `EndpointId`, relays formerly "DERP", no native AT-Proto E2EE working group (third-party only).
>
> **Overlap note:** the Delta Chat / Chatmail / Peat material at the end of this body also appears
> (from a *different* Gemini session) in `crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md`.
> Distill once; cross-reference rather than duplicate. UI render chrome (citation chips like
> "Iroh", "GitHub", "Crates.io", "+1"; "AI generated" image captions) stripped per §4.

---

**[user]** Tell me about what makes QUIC cool for use with iroh p2p

**[assistant]** Building a peer-to-peer (P2P) network is notoriously difficult because the modern
internet was designed for a client-server architecture. By building Iroh entirely on top of QUIC
(via their customized QUIC stack, noq), the developers bypassed years of technical debt that older
P2P frameworks had to handle manually. Here is what makes QUIC uniquely powerful when paired with
Iroh:

1. **Seamless NAT Traversal (Hole Punching).** The primary goal of Iroh is to establish direct
   connections between devices, even behind strict home routers or firewalls. The UDP advantage:
   QUIC operates over UDP rather than TCP; UDP is vastly easier for hole punching. Native fallback:
   if a direct connection fails, Iroh routes through a relay server. Because it's all QUIC/UDP,
   transitioning from a relayed path to a direct hole-punched path happens cleanly without
   interrupting the upper application layers.

2. **Zero Head-of-Line Blocking.** In older TCP P2P architectures, one dropped packet freezes the
   whole connection (HoL blocking). QUIC stream multiplexing lets Iroh open thousands of
   independent concurrent streams over a single connection; a dropped packet in an iroh-blobs
   fetch has zero impact on a parallel iroh-gossip stream.

3. **Built-In, Zero-Overhead Security.** QUIC bakes TLS 1.3 directly into its packet structure;
   every Iroh connection is E2E encrypted and authenticated by default. Public keys as IDs: Iroh
   maps a node's cryptographic public key directly to its identity; QUIC secures the transport
   using these exact keys, removing the need for external SSL/TLS certificates.

4. **Connection Migration & Multipath.** TCP is tied to a static 4-tuple; switching Wi-Fi→cellular
   kills TCP connections. QUIC uses Connection IDs independent of IP, so an Iroh node changing
   networks migrates seamlessly. Iroh's team has heavily contributed to QUIC Multipath, allowing
   nodes to use multiple paths (Wi-Fi and 5G) simultaneously.

5. **QUIC Address Discovery (QAD).** Most P2P relies on STUN (primitive, unencrypted UDP). Iroh
   leverages native QUIC frames (OBSERVED_ADDRESS): when you connect to a peer/relay they securely
   tell you what your IP looks like from outside, inside the QUIC layer — inheriting QUIC's
   congestion control, reliability, and encryption.

6. **Streams & Datagrams (best of both).** QUIC natively supports reliable streams AND unreliable
   datagrams over the same socket. Iroh can stream a DB state update reliably while firing
   fire-and-forget telemetry without secondary connections.

**[user]** Like the contributed to the standard itself? "Iroh's team has heavily contributed to
QUIC Multipath" — Wow that sounds like a great leap forward actually

**[assistant]** To clarify, the Iroh team (n0) aren't the primary editors drafting the official
IETF specification text — that is spearheaded by engineers from Alibaba, UCLouvain, and internet
pioneers like Christian Huitema. Instead, the Iroh team's contribution is on the engineering /
implementation side. The launch of their dedicated, general-purpose QUIC stack **noq** (Number 0
QUIC) is a leap forward for the ecosystem.

How they pushed the standard forward: an IETF draft (e.g. draft-ietf-quic-multipath) is only as
good as its real-world implementations. The Iroh team built it in production-grade Rust. Why this
matters: (a) shaking out protocol bugs (e.g. PATH_STATUS frame interactions with handshakes); (b)
elevating the relay architecture — before native multipath Iroh ran a "sleight of hand" where the
QUIC layer thought it talked to one IP while lower layers shuffled packets between direct paths and
relays; with noq they mapped these P2P fallbacks onto the official Multipath spec, so the relay is
a first-class "path" with its own native congestion control; (c) validation from spec authors —
launching noq caught the attention/praise of core IETF QUIC WG members including multipath authors
like Christian Huitema.

**[user]** Talk to me about iroh and whether it actually needs a relay and how things work with and
without

**[assistant]** Short answer: No, Iroh does not strictly need a relay to function, but practically
it needs them to survive on the public internet. Same LAN, or a node with a public IP + open ports,
can bypass relays. Behind home routers / corporate firewalls / cellular, relays become essential.

WITH a relay (default): (1) Finding a home base — on boot a node pings known relays, picks fastest,
designates a "home relay," keeps a lightweight connection. (2) Instant connection / staging ground
— Node A only knows Node B's NodeID + home-relay URL; A sends encrypted packets to B's home relay
which forwards immediately (zero waiting). (3) Hole punching in background — relay does STUN-like
ops telling both nodes their public IP/ports; A and B try to punch a direct UDP path. (4) The clean
break — ~9/10 times punching succeeds; QUIC connection migration shifts the stream from relay to
direct path mid-stream without a dropped packet; the relay steps back. If punching fails (strict
symmetric firewalls), the relay remains as an encrypted fallback pipe.

WITHOUT a relay (pure direct mode): you cannot rely on just a NodeID; you must pass a NodeAddr with
explicit direct addresses. LAN / public IP nodes connect instantly. Vulnerabilities of relay-less:
no hole punching (residential NAT both sides → never connect), no network agility (IP change → 
connection dies permanently), no fallback. Key takeaway: Iroh treats the relay as a disposable
utility, not a central broker. Because QUIC enforces TLS 1.3, the relay is blind (sees scrambled
bytes + destination NodeID). Iroh relays are stateless — no DBs, no accounts; if one crashes, nodes
connect to the next closest and keep moving data.

**[user]** Talk to me about iroh and automerge

**[assistant]** Combining Iroh and Automerge gives you the foundational stack for local-first
software. Automerge is the brain (what the data is, how to merge without conflicts); Iroh is the
muscle (find the other machine, securely move bytes without a central server).

1. **CRDTs meet P2P transport.** Automerge is a CRDT tracking changes to a JSON-like structure as
   an append-only log; two devices edit offline, reconnect, merge automatically, converge to the
   same state. But Automerge has no networking code — it generates binary sync messages. Iroh
   delivers them via QUIC, punching through firewalls peer-to-peer.

2. **Wire protocol harmony (QUIC streams).** Automerge uses a stateful sync protocol (Peer A sends
   a fingerprint, B figures out missing edits, loop until synced). Iroh maps via ALPN
   (Application-Layer Protocol Negotiation) strings to route traffic — e.g. a custom
   `/iroh-automerge/1` protocol handler — and bidirectional QUIC streams passing length-prefixed
   sync messages until done.

3. **Zero-trust security by default.** Automerge doesn't encrypt; raw sync over TCP/UDP is
   sniffable. Iroh mandates TLS 1.3, so documents are E2E encrypted, and NodeIDs = public keys
   guarantee you sync only with trusted peers.

Architecture:
```
+-------------------------------------------------------+
|                   Your Application                    |
|  (Interacts with a local, mutable JSON-like document) |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|                       Automerge                       |
|   (Tracks local changes, generates binary sync frames)|
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|                      Iroh Router                      |
| (Opens QUIC streams, bypasses NATs, falls back to relay)|
+-------------------------------------------------------+
```
Payoff: 100% offline-first; serverless collaboration; turnkey infrastructure (ecosystem tools like
the open-source peat-mesh project wire Iroh + Automerge out of the box).

**[user]** Tell me more about these [multipath]. And tell me if there are other alternatives to
automerge for use with iroh

**[assistant]** The **noq** ("number 0 QUIC") launch by n0 is a pure-Rust, general-purpose QUIC
implementation built to support bleeding-edge internet standards; it's the primary transport under
Iroh. The noq breakthrough — making multipath native: before noq, multiple routes required
"sleight of hand" (single static IP illusion while Iroh shuffled packets across IPv4/IPv6/relays —
a tiny custom NAT layer). With noq: first-class paths (Wi-Fi / 5G / relay each a native distinct
path); per-path congestion control (independent congestion states + packet number spaces per path);
beyond multipath — custom QUIC NAT Traversal (QNT) and QUIC Address Discovery (QAD), bypassing STUN.

Alternatives to Automerge for Iroh apps:
1. **iroh-docs (built-in KV store).** Distributed multi-dimensional key-value store, entries signed
   by cryptographic authors; uses range-based set reconciliation to identify disagreements;
   separates metadata (keys, author sigs, BLAKE3 hashes) from content (raw bytes pulled via
   iroh-blobs).
2. **Loro (loro).** Ultra-fast production-grade CRDT in Rust; built-in time-travel version control;
   advanced rich-text algorithms; tight binary update blocks → minimal QUIC framing overhead.
3. **Y-CRDT (y-crdt).** Native Rust port of Yjs; deep integration with ProseMirror/Monaco/Quill/
   Slate; uses state vectors to diff; pair with iroh-gossip for transient cursor data.
4. **Diamond Types.** Hyper-optimized text CRDT in Rust; fastest processing / lowest memory; good
   for embedded/low-power text editing.

**[user]** I meant tell me more about this — "the open-source peat-mesh project [wires] Iroh and
Automerge together out of the box"

**[assistant]** The project is **Peat** (developed by Defense Unicorns); its core engine peat-mesh
is a prime example of the maturing local-first ecosystem. Wiring Iroh + Automerge yourself requires
non-trivial boilerplate (protocol handler, bidirectional QUIC streams, length-prefixing sync
messages, state-vector loops, disk persistence). peat-mesh collapses that into one abstraction:
```rust
// 1. Open a local, persistent Automerge document store
let store = Arc::new(AutomergeStore::open(&db_path)?);
// 2. Fire up the Iroh P2P transport layer
let transport = Arc::new(IrohTransport::from_seed_at_addr("my-node-seed", bind_addr).await?);
// 3. Glue them into a single turnkey sync backend
let backend = AutomergeBackend::with_transport(store, transport);
// 4. Start the background sync loop
backend.start_sync()?;
```
Infrastructure headaches it solves: automatic topology & discovery (Iroh wide-area + local mDNS, so
two devices in a room find each other over Wi-Fi even with no internet); abstracting the network
lifecycle (wakes on local change, isolates changed keys, opens a QUIC stream, pushes diffs); tactical
& denied (DIL) environments — gracefully drops from Iroh QUIC to a local Bluetooth Low Energy mesh
(peat-btle) without losing Automerge context; hardened security — shifted default QUIC TLS stack
from `ring` to `aws-lc-rs` for FIPS-compliant crypto.

**[user]** Are there other value add projects like this that are good to know about?

**[assistant]** The ecosystem is shifting from low-level plumbing to high-level abstractions:
1. **Decentralized AI & tensor orchestration.** Prime Intellect (prime-vllm / prime-iroh) — modded
   pipeline-parallel vLLM; uses Iroh to pass intermediate tensors/gradients P2P between distributed
   GPUs over the public internet. Tandemn (tandemn-vllm / tensor-iroh) — distributed LLM inference
   over mixed low-cost hardware; streams PyTorch activations down Iroh QUIC streams.
2. **Zero-config game netcode.** Bones engine (native P2P gaming) & godot-iroh (bridge into Godot);
   out-of-the-box zero-server multiplayer via a cryptographic ticket.
3. **Reactive event streaming.** cross.stream — local-first P2P alternative to Kafka/RabbitMQ; event
   streaming for reactive workflows, sharing topics between devices via Iroh.
4. **Drop-in developer utilities.** iroh-ssh — SSH into machines by NodeID without public IP/DDNS/
   VPN. Obsiroh — decentralized Obsidian vault sync over an encrypted Iroh stream.

**[user]** Other value add projects in the iroh ecosystem? Open source apps that use iroh
effectively? Especially collaboration or social

**[assistant]** Visual & text collaboration: **Hubris** — offline-first P2P note-taking for visual
thinkers, freeform canvas + local AI, multi-user collaboration via Iroh, no artificial bandwidth
caps. **Teamtype** — editor-agnostic real-time collaborative text editing; syncs local workspace
files over an Iroh stream; deltas propagate over QUIC. **Zeco** — securely share active Zellij
terminal sessions across the internet by tunneling to another dev's NodeID (alternative to
SSH/reverse-proxy/tmate). Social & secure chat: **Dash Chat** — "Secure Chat for Internet
Shutdowns," P2P mesh; uses Iroh localized mDNS + wide-area; upgrades to wide-area P2P when internet
returns. **Tauri + Automerge + Iroh Chat (PoC)** — boilerplate combining Tauri v2, React, Automerge,
Iroh; chat history as an immutable Automerge CRDT timeline, Iroh as transport. Massive frameworks:
**Holochain (Kitsune2 transport)** — the Holochain core team is rewriting its networking component
(Kitsune2) to run on top of the Iroh transport layer, offloading global connectivity / fallback
relay routing / NAT punching to Iroh.

**[user]** I'm confused about the relationship of delta chat and iroh networking and chatmail

**[assistant]** Think of Delta Chat as the main application engine; Chatmail and Iroh are two
different pipelines it uses to move data. Delta Chat feels like an IM (Signal/WhatsApp) but uses the
global email network to send messages. Because email can be slow/rigid/metadata-leaky, Chatmail was
built as a specialized fast email server for the app. But email is inefficient for instant tasks
(linking a new phone, real-time games), so Iroh is embedded to let devices talk directly, skipping
the email server when direct speed is required.

1. **Delta Chat (app architecture).** Decentralized front-end chat client; translates messages into
   E2E-encrypted emails (Autocrypt + OpenPGP) over IMAP/SMTP; works with any standard email address,
   though consumer providers throttle/flag chat traffic.
2. **Chatmail (optimized email engine).** Minimalist OSS email server config (trimmed Postfix +
   Dovecot); instant onboarding via QR/link (anonymous profile); sub-second delivery; spam immunity
   (forbids unencrypted outgoing mail → can't be blacklisted).
3. **Iroh (P2P stack).** Multi-device syncing (link phone↔desktop, transfer keys/history P2P);
   real-time WebXDC channels (mini isolated web apps — games/pads/tools inside a chat — powered by
   live direct P2P streams).

**[user]** Are there any chat apps that only use p2p and iroh that are mature?

**[assistant]** Short answer: no. No mainstream, fully mature consumer chat app relies exclusively
on P2P/Iroh for all messaging. Iroh only recently hit stable 1.0-RC milestones; most implementations
are developer toolkits, tech demos, or hybrid integrations. Why Delta Chat uses a hybrid model: (a)
the offline problem (pure P2P has nowhere to hold a message for an offline recipient; Chatmail is a
decentralized mailbox); (b) mobile battery & push notifications (OSes kill background processes; a
constant P2P daemon drains battery; servers wake phones via Apple/Google push); (c) discovery
latency (hole-punching per message from a cold start adds lag). What exists now: iroh-gossip tech
demos (official protocol example, CLI/terminal chat over a P2P swarm); community GitHub experiments
(e.g. iroh-lab, indie Rust + Iced GUIs — PoC-grade); Delta Chat's WebXDC real-time apps (spin up a
pure Iroh P2P connection to stream keystrokes/game moves, skipping email for that session).

**[user]** How mature and supported is delta chat? How did it get started? Where does it get its
funding? Is chatmail just for it?

**[assistant]** Maturity & support: highly mature, stable, production-ready across Android, iOS,
Windows, macOS, Linux. Supported by open-source devs + digital-rights orgs; hard for governments to
block without shutting down corporate email. Front Line Defenders and eQualitie recommend it for
high-risk users/journalists/activists. Security vetting: multiple independent crypto audits (e.g.
Include Security) + academic protocol reviews of OpenPGP/Autocrypt. How it started: ~2017, major
stable launches 2019; started by Holger Krekel and Merlinux GmbH (Freiburg, Germany). Core insight:
instead of building a new centralized network, use email — the largest, oldest decentralized comms
network — with a beautiful frontend that hides Subject/Signature and automates E2EE via Autocrypt.
Funding: no VC/ads; relies on public-interest tech grants — Open Technology Fund (OTF); the EU (NGI/
Next Generation Internet); the U.S. Bureau of Democracy, Human Rights, and Labor (DRL). Is Chatmail
just for Delta Chat? For all practical purposes, yes — minimalist Postfix/Dovecot config by the
Delta Chat team; speaks IMAP/SMTP but is restricted (encrypted-only; anti-spam crippling; instant QR
provisioning) so it's useless as a normal email account.

**[user]** How wide spread is it?

**[assistant]** Decentralized, so no master count (plugging in your own email address makes you
invisible to the team). From default Chatmail + push infra: mid-tier privacy footprint, expanding —
especially in censorship regions. 2025/2026 surges in the US and Cuba; peak windows onboarding up to
5,000 new users/hour on default chatmail infra. Push traffic ~1.8 million notifications/day (excludes
Desktop, F-Droid, classic-email users). High density among digital-rights activists, journalists,
tech groups migrating off Matrix, and citizens where mainstream apps are blocked. Maturing ecosystem
of third-party clients/forks: **ArcaneChat** (low-bandwidth-optimized client); **Privitty**
(enterprise fork on Play Store adding message revocation + time-based access). Bottom line: active
global base in the hundreds of thousands (likely low millions across all private domains).

**[user]** Why was arcanechat created?

**[assistant]** ArcaneChat was created by **Asiel Diaz Benitez**, a core Delta Chat developer. Fully
compatible with standard Delta Chat users (same engine). Marketed as a "private messenger for the
family"; a streamlined, optimized flavor that tests advanced features before they hit the main app.
Four goals: (1) absolute mandatory encryption — own hardened registration server (arcanechat.me)
blocking any unencrypted traffic; (2) aggressive bandwidth/data saving — profile disabling (zero
background bandwidth), extreme audio compression (low-quality voice notes), strict download caps
(media auto-download hard-capped at 640 KB default); (3) quality-of-life UI — markdown formatting,
Telegram animated stickers (.tgs) + SVG previews, "last-seen" status, auto-looping videos; (4) an
upstream sandbox — server-hardening + experimental features tested in ArcaneChat, then merged back
into mainstream Delta Chat.
