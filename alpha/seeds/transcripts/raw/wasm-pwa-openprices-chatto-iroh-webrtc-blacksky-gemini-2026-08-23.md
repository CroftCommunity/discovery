# Raw: client-side WASM · openprices idea · Chatto/LiveKit · iroh+WebRTC two-tier relay · Blacksky/PDS mechanics (Gemini, pasted 2026-08-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a **Gemini** session (owner-confirmed "yes to gemini for each of these"),
dialogue date unknown, pasted 2026-08-23. Citation chips ("mfyz", "GitHub +1", "Futuristic Bug",
"CNET", "Wikipedia", "Bluesky", "IETF", "The Go Programming Language", "sifa.id", "Foodkg",
"Blacksky Algorithms", "Will Schenk", "atproto.blue", "Docs.rs", "Tangled") reproduced as
bracketed markers — unfollowable from a paste. Owner turns include voice-dictation (reproduced
as-dictated). No secrets.

**WHOLE-DOCUMENT FACT POSTURE.**

- **THE LOAD-BEARING PROBLEM (Body D):** the two-tier browser-calling architecture is built on
  **`iroh-webrtc-transport`** — a crate the FACTCHECK SoT
  (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`, crate-name hygiene) records as **likely
  nonexistent**. Gemini's own in-session "critical review" demotes it to "experimental,
  unmaintained third-party software" while continuing to build on it, and also lets slip that
  STUN may be stale ("`enable_stun = true` **or QAD flags**" — iroh 1.0 moved address discovery
  to QAD). The `[access.http]` config block, `web-transport-iroh`, and every other named knob are
  **`[UNVERIFIED]`**. The corpus has *real* ground truth for iroh-relay gating: **croft-stack's
  own gated relay + croft-admit admission machinery** — verify against that and against real
  iroh/iroh-relay source, never against this transcript. The architecture's *shape* (thin
  browser WebRTC tier + gated tunnel tier + honest failure) is sound and matches the owner's
  stated requirement; the shape is what E131 carries, not the knobs.
- Body A/E numbers (WASM project claims, PDS census ~3,100–3,500 active / ~16,000 all-time,
  Bluesky's ~80–100 mushroom-named shards behind an entryway, Blacksky ~35k repos / July 2026
  beta, rate-limit points tables, plc.directory latency figures) are **all dialogue-sourced
  `[UNVERIFIED]`**. Known baseline facts (Jetstream, requestCrawl, DID→PDS resolution shape,
  MST repos, blob CID references) are consistent with the corpus's verified atproto material.
- **Chatto (chatto.run)** — a product this corpus cannot confirm exists; treat the whole Body C
  stack description as `[UNVERIFIED]`.
- Body B's external references (Open Prices / Open Food Facts, GasBuddy's proprietary posture,
  NHTSA VIN API, Open Products Facts, FoodKG) are plausible and mostly consistent with §5l's
  earlier caveats; licensing/API specifics `[UNVERIFIED]`.

**What this is.** Six bodies in one session: (A) a three-round client-side-WASM survey
(showcase → open-source only → everyday-user apps → hardware/CRDT/ZK/audio/accessibility) plus a
libp2p/WebRTC clarification; (B) **the owner's openprices idea** — a public, user-owned price log
on atproto ("cars.com without the bullshit") → draft `org.openprices.observation` lexicon,
polymorphic identifiers, trust design, prior-art landscape (→ **E130**); (C) the Chatto stack and
its fully-brokered LiveKit SFU calls; (D) **the owner's browser-calling requirement and the
two-tier iroh+WebRTC candidate architecture** (→ **E131**, COHESION §77 — extends §73's
browser-relay-only constraint); (E) Bluesky/Blacksky federation, blob storage, relay crawling,
PDS census, and the full handle→DID→PDS→record lookup walkthrough.

---

## Body A — client-side WASM in SPA/PWA (three rounds)

**Owner:**

> What are some awesome and amazing things done with client side wasm esp in an spa/pwa type
> scenario

**Gemini** *(round 1, condensed-faithful; citation chips as markers)*:

> Client-side WebAssembly has reshaped SPA and PWA architectures by enabling tasks previously
> restricted to native desktop apps or heavy server-side infrastructure — near-native C/C++/
> Rust/Go/.NET in the browser; zero-latency, offline-capable, privacy-first apps. *[mfyz +1]*
>
> 1. **Full embedded databases & local-first architectures:** PGlite (Postgres compiled to Wasm)
>    and sql.js (SQLite) run relational databases entirely in client memory backed by
>    IndexedDB/OPFS; offline-first PWAs execute joins/aggregations/vector searches locally and
>    sync asynchronously; Pyodide brings the Python scientific stack (NumPy, Pandas, SciPy,
>    scikit-learn) to the client, JupyterLite runs notebooks fully in-browser.
> 2. **Desktop-grade media editing (zero uploads):** Adobe Photoshop Web (decades of C++
>    compiled to Wasm); Figma (C++/Rust, 60 FPS vector canvas); FFmpeg.wasm tools (Clipchamp,
>    Modfy, Mastershot) slicing/transcoding multi-GB 4K video offline; OpenCascade.js for 3D
>    CAD via WebGL/WebGPU.
> 3. **Client-side AI/ML:** Picovoice offline voice recognition; ONNX Runtime Web + SIMD +
>    WebGPU for background segmentation (Meet/Zoom blur), Whisper speech-to-text, lightweight
>    LLMs; in-browser background removal, face recognition, OCR — documents never leave the
>    sandbox.
> 4. **Full runtimes & emulation:** Blazor WebAssembly (.NET runtime in Wasm); Ruffle
>    (Rust Flash emulator); v86 (x86 emulator booting Linux/Windows in a tab).
> 5. **Architectural benefits:** zero server compute; instant offline (SW caches the .wasm);
>    strict data privacy; C/C++/Rust code reuse.

**Owner:**

> Talk about in the opensource space only

**Gemini** *(round 2, condensed-faithful)*:

> 1. **Local-first databases:** **PGlite** (ElectricSQL; official Postgres → Wasm, in-memory or
>    OPFS/IndexedDB persistence, pgvector for local RAG); **DuckDB-Wasm** (columnar analytics
>    over multi-GB Parquet/CSV/JSON on static CDNs, in the browser thread); **SQLite3-Wasm**
>    (official SQLite-team build; OPFS pairing for ACID near-native local storage).
> 2. **On-device ML:** **Transformers.js** (Hugging Face; Whisper, Segment Anything, BERT via
>    ONNX Runtime Wasm/WebGPU); **WebLLM** (MLC; Llama 3/Phi/Mistral in-tab via Wasm+WebGPU);
>    **ONNX Runtime Web** (Microsoft; SIMD + multi-threading).
> 3. **Zero-server media/docs:** **FFmpeg.wasm**; **Typst** (Rust typesetting → Wasm, instant
>    PDF-as-you-type, no LaTeX toolchain); **Squoosh** (Google; MozJPEG/AVIF/WebP/OxiPNG codecs
>    in Wasm).
> 4. **Runtimes & emulation:** **Pyodide & JupyterLite**; **Ruffle**; **v86**; **Godot Engine**
>    web export (Wasm + WebGPU, installable offline PWA games).
> 5. **Native-language UI frameworks:** Rust reactive frameworks (**Leptos, Yew, Dioxus**);
>    **Blazor WebAssembly**.

**Owner:**

> Keep going, let's think on some really useful applications for everyday users that could be
> done with wasm and pwa

**Gemini** *(round 3, condensed-faithful)*:

> 1. **Zero-trust local utilities:** no-upload PDF mergers/image resizers/video trimmers;
>    privacy-first passport/document photo makers (ONNX background removal + landmark
>    detection); universal archive extractors (libarchive → 7z/RAR/TAR in a PWA); FFmpeg.wasm
>    trimmers/extractors.
> 2. **On-device personal AI/knowledge:** offline semantic search over personal PDFs/notes/
>    receipts (PGlite+pgvector or DuckDB-Wasm + Transformers.js); in-browser transcription
>    (Whisper.cpp/Picovoice via Wasm SIMD).
> 3. **Private personal finance:** DuckDB-Wasm/Pyodide dashboards over years of bank CSVs —
>    categorize, forecast, pivot — fully offline; deterministic privacy via SW + local Wasm.
> 4. **Offline creative/media:** Wasm DAWs (EQ, noise removal, stem separation, podcast
>    stitching); photo restoration/vectorizing/upscaling (Squoosh, Rust/WebGPU editors).
> 5. **Offline field & travel:** GPX/topo analysis (GEOS/OpenCascade spatial engines in
>    airplane mode); offline OCR + translation (Tesseract.js + local matrices).
>
> Architectural synergy: SW shell/cache → OPFS + SQLite/PGlite storage → Wasm compute →
> WebGPU/SIMD/threads hardware acceleration.

**Owner:**

> Keep going

**Gemini** *(round 4, condensed-faithful)*:

> 1. **WebHardware (WebUSB/WebSerial + Wasm):** in-browser 3D slicing (PrusaSlicer/Cura → Wasm,
>    G-code streamed to the printer over WebSerial); microcontroller/firmware flashing (ESP Web
>    Tools, Via/Vial keyboards; Arduino/ESP32/STM32 from a PWA).
> 2. **Local-first P2P & CRDT collaboration:** Rust CRDT engines (**Automerge, Loro, Yjs/
>    Y-Rust**) — offline edits, direct WebRTC connect, merge without a central server;
>    browser-native peer nodes (**libp2p-wasm**) as light nodes for off-grid messaging/mesh/
>    file sharing.
> 3. **Client-side crypto & ZKPs:** in-browser ZK-SNARK generation (Arkworks, snarkjs);
>    memory-hard KDFs (Argon2/PBKDF2) in Wasm workers for local vaults.
> 4. **Real-time WebAudio DSP:** Wasm DAWs (JUCE/Faust) in AudioWorklet threads; real-time
>    guitar FX/amp modeling/pitch correction in an offline PWA.
> 5. **Local accessibility/assistive:** offline sign-language translation (client-side CV on
>    WebGPU/Wasm); offline eye-tracking/hands-free control from standard webcams.

**Owner:**

> Is [quoting the libp2p-wasm item] … Webrtc?

**Gemini:**

> Yes, WebRTC is the primary underlying transport protocol that enables libp2p to establish
> direct browser-to-browser peer connections. Browsers do not expose raw TCP or UDP socket APIs
> to JavaScript or WebAssembly, so Wasm cannot open arbitrary network listeners on its own.
> libp2p isn't a transport itself — it is a modular P2P framework (peer identity via crypto
> keys, discovery, NAT traversal, protocol negotiation, stream multiplexing) that wraps
> browser-supported transports: **WebRTC / WebRTC-Direct** (true browser-to-browser data
> channels — encrypted streams with no central server in the data path); **WebSockets/WSS**
> (browser-to-server: bootstrap, signaling, relay nodes — a browser cannot accept incoming WS
> connections); **WebTransport** (UDP/QUIC to native HTTP/3 nodes); **Circuit Relaying** (when
> both browsers are behind strict NATs, traffic routes through an intermediate relay). In a
> browser Wasm PWA, WebRTC is the raw wire, and libp2p is the stack on top of it.

## Body B — the openprices idea (owner project concept → E130)

**Owner** *(voice-dictated)*:

> Thinking about building an application on AT Proto that allows sharing of prices, you know,
> like a public log, I guess by default, but also uh an, you know, public aggregate. I think it
> would be really interesting if there was like a lexicon and an application that allowed people
> to submit prices and it like mapped it, you know, like do the thing that like a cars.com
> purports to do, but without the bullshit, you know, but I'm talking about even broader. Like,
> I don't know what how you would tag them. Maybe by UPC doesn't come with a price. Like, I
> don't know what the right thing to do would be. But it's a really interesting concept in terms
> of kind of user empowerment.

**Gemini:**

> Building an open price-logging protocol on AT Proto is a compelling architecture for
> user-owned economic data. Because AT Proto decouples user storage (PDS) from aggregation
> (AppViews), users retain full ownership of their submission history, while indexers can
> process the global firehose to build transparent, manipulation-resistant aggregate price maps.
>
> 1. **The Core Identifier Problem.** No single schema covers every transaction: a car has a
>    VIN, milk has a UPC/GTIN, an apartment has an address, a local service only a
>    context-dependent description. Solution: a **Polymorphic Identifier** pattern in the
>    Lexicon, separating observation metadata from item identity. Categories: **GTIN/UPC/EAN**
>    (cross-reference open databases like Open Food Facts / Open Products Facts for titles/
>    brands/images); **Geographic / OSM node IDs** (services, housing, gas — OpenStreetMap
>    place/way IDs as the vendor-neutral location entity); **domain-specific IDs** (`vin` —
>    decodable via the free NHTSA API, `isbn`, `us_npi` for medical pricing); **fallback
>    free-text** (brand + model/name + unit_quantity).
> 2. **Draft Lexicon (`org.openprices.observation`)** — record keyed `tid`; required
>    `price` (number), `currency` (ISO 4217), `item` `{ idType: gtin|upc|vin|isbn|osm|custom,
>    idValue, name?, brand? }`, `createdAt`; optional `unit` (gallon/kg/month/flat_rate),
>    `condition` (new / used_like_new / used_fair / refurbished), `location`
>    `{ latitude?, longitude?, osmPlaceId?, vendorName? }`, `proof` (photo blobs of receipt or
>    shelf tag, `image/*`).
> 3. **Architecture:** user app signs & writes → user's PDS → relay firehose → open AppView /
>    indexer → spatial DB (PostGIS price maps) + time-series analytics (inflation/trends).
>    Anyone can host an AppView or run local queries; if one aggregator suppresses data or
>    inflates averages, a competitor can spin up an alternative frontend on the same public
>    ledger.
> 4. **Trust, spam, manipulation:** cryptographic accountability (every price point signed by a
>    DID — spam DIDs ignorable, web-of-trust filters); **web-of-trust weighting** ("only
>    aggregate prices from users within 3 hops of my follow graph", labeler-verified);
>    **proof blobs** (receipt/shelf-tag photos, OCR-verifiable by AppViews); **statistical
>    robustness** (trimmed medians, IQR bounds, sample volume — never bare averages).

**Owner:**

> Does something like this already exist?

**Gemini:**

> Yes, parts of this concept exist across different domains, but a universal, protocol-native
> price ledger on AT Proto does not exist as a unified standard yet.
>
> 1. **Open Food Facts "Open Prices"** (prices.openfoodfacts.org) — the closest spiritual
>    equivalent for consumer goods: users upload receipt/shelf-tag photos; OCR + crowd
>    validation extract GTINs, OSM store locations, prices, dates; fully open data dumps. Falls
>    short of the vision in being a **centralized database** rather than a federated protocol,
>    and grocery-scoped.
> 2. **Vertical crowdsourced platforms:** **GasBuddy** (proved users will log location prices
>    millions of times a day — but ad-supported, sells location data, fuel-only);
>    **BrickSeek / Keepa / CamelCamelCamel** (scraping/affiliate APIs — retailers can cut
>    access at any time); **TrueCar / Cars.com** (purport real paid prices, but dealer-lead
>    business models incentivize hiding true out-the-door prices).
> 3. **AT Proto today:** momentum toward niche custom lexicons (movies, books, workouts, coffee
>    brews) proves the PDS-to-AppView pattern; an `org.openprices`-shaped broad standard is
>    **still an uncolonized space**. Why AT Proto fixes the structural flaws: no API cutoff
>    (data in user PDSs + open firehose); separation of incentives (the consumer owns the
>    record; AppViews can be run by nonprofits, consumer advocates, local communities).

**Owner:**

> Gas buddy records are public?

**Gemini:**

> GasBuddy's dataset is strictly proprietary, not open or public data. *[CNET]* Anyone can view
> nearby prices in the app, but the underlying database, raw submission logs, and historical
> feeds are locked behind a private, monetized platform: B2B data sales (enterprise market
> intelligence, fleet software, price-trend analytics), no public API (scrapers violate ToS and
> are blocked), and consumer monetization via location tracking sold to ad brokers. The classic
> Web2 crowdsourcing dynamic: users contribute free, one corporation claims exclusive ownership
> of the compiled database. An AT Proto approach flips it: records published to individual user
> repositories keep the data stream public at the protocol level — no one can paywall the
> database, cut API access, or quietly sell location trails.

## Body C — the Chatto stack (`[UNVERIFIED]` product)

**Owner:**

> Explain the stack behind chatto and how they do audio and video calls

**Gemini:**

> Chatto (chatto.run) is a lightweight, self-hostable team chat built around an event-sourced
> architecture, structured as a single Go binary with horizontal scaling. Backend: **Go**,
> ConnectRPC + Protocol Buffers over HTTP/1.1–2; **NATS JetStream + NATS KV** as primary store
> and messaging backbone (embedded NATS in single-binary mode; no Postgres/MySQL); event
> sourcing (immutable domain events in JetStream `EVT` streams; operational state in NATS KV
> with optimistic concurrency); **Authling** (OIDC) for identity. Frontend: **Svelte 5**,
> Tailwind v4, TinyBase client state.
>
> Calls delegate real-time media to **LiveKit**, an open-source WebRTC SFU: the Chatto backend
> verifies room permissions and mints scoped LiveKit JWT join tokens; media never routes
> through the Chatto server or NATS — the Svelte client connects directly to the LiveKit SFU;
> LiveKit handles adaptive bitrate, signaling, Opus/VP8/VP9/H.264/AV1, multi-party
> distribution. Voice/video requires running LiveKit alongside the Chatto binary.

**Owner:**

> So is it a brokered experience or a handoffs and direct between clients after or neither?

**Gemini:**

> It is a brokered (server-routed) experience; there is no direct P2P handoff. Two-stage
> brokering: (1) the Chatto backend is the signaling/authorization broker (tokens, per-call
> E2EE key, webhook-tracked participant state); (2) the LiveKit SFU is the media broker —
> each participant uploads one track, the SFU forwards streams by subscription/active-speaker/
> bandwidth; with client-side E2EE the SFU routes packets it cannot decrypt. Why: P2P mesh
> costs every client N−1 uploads, which collapses in group calls; an SFU keeps the client at 1
> upload + N−1 downloads with better loss handling, simulcast, and scaling.

## Body D — the browser-calling requirement and the two-tier iroh+WebRTC candidate (→ E131)

**Owner:**

> Is there an existing livekit or webrtc model that does pairing and handoff like an iroh relay
> style so users talk directly?

**Gemini:**

> LiveKit itself does not support direct P2P handoffs, but the pattern — pair via a relay, hand
> off to direct P2P — is native to standard WebRTC ICE and is implemented by platforms like
> Jitsi Meet and libraries like PeerJS.
>
> **Why LiveKit doesn't:** strictly an SFU, even 1-on-1 routes through the server —
> intentionally, for server-side recording/egress, voice-AI agents, uniform telemetry and
> adaptive bitrate.
>
> **How WebRTC natively implements relay/P2P handoff** (iroh's relay design — derived from
> Tailscale's DERP — was heavily inspired by this): signaling exchanges SDP; STUN discovers
> public IP:port; TURN relays as fallback; ICE tests candidate pairs and hands traffic to the
> direct path the moment one succeeds.
>
> **Existing implementations:** **Jitsi Meet** — hybrid handoff: 1-on-1 starts as direct P2P
> (TURN fallback), and when a third participant joins, both streams hand off seamlessly to the
> Jitsi Videobridge SFU without dropping the call. **PeerJS / Simple-Peer / libp2p-webrtc** —
> lightweight signaling for pairing, STUN/TURN traversal, then direct sockets. **Iroh + WebRTC
> data channels / custom pipelines** — some developers build custom voice/video on iroh
> transport (WebRTC DataChannels or RTP over QUIC/iroh streams), using iroh's relay for handoff
> rather than STUN/TURN.

**Owner:**

> Explain this with sources — "Iroh's relay design (which is derived from Tailscale's DERP
> architecture) was heavily inspired by native WebRTC networking" — And sure but iroh is the
> browser is not really functional in this way is it? — "using Iroh's relay for handoff rather
> than standard STUN/TURN servers." — And telle more about PeerJS / Simple-Peer / libp2p-webrtc

**Gemini** *(condensed-faithful)*:

> 1. **DERP/iroh ← WebRTC lineage:** STUN → relay STUN query (learn public IP:port); TURN →
>    DERP/iroh relay (encrypted packet fallback); ICE → hole-punching + handoff. Tailscale DERP
>    and iroh relays act as both STUN servers and TURN-like relays: traffic flows through the
>    relay immediately while background hole punching executes; once a direct QUIC/WireGuard
>    path opens, nodes shift traffic off the relay — matching ICE candidate switching. Unlike
>    TURN, DERP/iroh relays only ever route packets encrypted to the peers' public keys — an
>    untrusted wire.
> 2. **Is iroh functional in browsers?** Yes, with major sandbox limitations: no raw UDP in
>    the sandbox → iroh-in-Wasm cannot hole-punch natively; default is **relayed mode** over
>    WebSockets/HTTP through an iroh relay. For direct browser connections the ecosystem uses
>    transport crates: **`iroh-webrtc-transport`** (wraps browser WebRTC DataChannels under
>    iroh-like semantics) and **`web-transport-iroh`** (WebTransport semantics over iroh
>    streams, client-to-server). **[CORPUS FLAG: `iroh-webrtc-transport` is
>    FACTCHECK-flagged as likely nonexistent; both crate names `[UNVERIFIED]` — see header.]**
>    If your app lives entirely in the browser, you still depend on WebRTC (or an iroh relay)
>    to traverse firewalls.
> 3. **The libraries:** **PeerJS** — high-level wrapper + optional PeerServer;
>    `peer.call('user-a', stream)`; server exchanges SDP/ICE, then media/data flows directly.
>    **Simple-Peer** — RTCPeerConnection as a Node-style duplex stream; BYO signaling
>    (`peer.on('signal')` / `peer.signal(data)`); handles STUN/TURN gathering and hole
>    punching. **libp2p-webrtc (js-libp2p)** — cryptographic Peer IDs; **Circuit Relay V2**
>    instead of signaling servers; relay-exchanged SDP then a **WebRTC-Direct upgrade** to a
>    direct socket.

**Owner** *(the requirement — verbatim, load-bearing for E131)*:

> Basically, I want browser to browser equivalent relay hole punch and handoff without full
> tunneling and I know it can only do 9/10 cases without tunneling and that's fine
>
> But I use iroh elsewhere and would prefer to stick with it so iroh + webrtc direct with a
> relay handoff or a failure message
>
> Then the other case is members/insiders who really can use the tunneled relay CPU and bw with
> a more iroh native exp but I would like both cheaply and the browser to browser as a thin
> best effort Alice and to host only one relay

**Gemini** *(the architecture — shape sound, knobs `[UNVERIFIED]`, condensed-faithful)*:

> This architecture is feasible using the iroh ecosystem on a single hosted iroh-relay
> instance. iroh-relay serves two roles simultaneously: a STUN / WebRTC signaling broker
> (near-zero cost) and an encrypted packet-forwarding relay (heavy cost). Combining
> `iroh-webrtc-transport` on the client with iroh-relay's HTTP access-control callouts, you
> enforce direct-only WebRTC P2P for standard browser users while allowing full relay fallback
> for authenticated insiders.
>
> **Tier 1 (standard users, thin B2B):** browser Wasm client; signaling + STUN via the relay;
> direct RTCDataChannel attempt; **direct-only enforcement** — a strict connection watchdog
> (~5 s); on failure (the ~10% symmetric-NAT case) the client aborts and shows an explicit
> error ("Direct connection failed. Upgrade to Insider for relayed connectivity."). Server
> cost: kilobytes of signaling JSON + STUN queries; zero media touches the relay.
>
> **Tier 2 (insiders):** short-lived bearer token / key at endpoint init; direct WebRTC first;
> on failure, fall back to tunneling through the iroh-relay over WebSockets/TLS (or native
> QUIC from desktop/mobile binaries).
>
> **Single-relay config (claims `[UNVERIFIED]`):** `[server] enable_stun = true`;
> `[access.http] url = "https://api.yourdomain.com/v1/iroh-relay-auth"` — on each tunnel
> attempt the relay POSTs the client's endpoint id + token to your auth endpoint; 200 = relay
> forwards (insider), 403 = tunnel refused (guest stays direct-or-fail).

**Owner:**

> The end result is browser to browser opportunistically where it's nearly 0 cost and a single
> server process manage? And the b2b can do realtime bidirectional communication?

**Gemini:**

> Yes: for the ~90% where hole punching succeeds, the relay handles only the STUN query and
> SDP swap (fractions of a millisecond, a few KB), then steps out of the data path; one
> iroh-relay process is the sole control plane (STUN + signaling + tiered access control); the
> direct path is RTCDataChannel (SCTP over DTLS/UDP) — fully bidirectional, low-latency,
> ordered or unordered binary/text — and native WebRTC media tracks (Opus, VP8/VP9/AV1 over
> SRTP) can ride the same peer connection.

**Owner:**

> Explain the stack in detail focusing on the browser elements, the relay additions needed, and
> the parts iroh doesn't have natively bc I didn't know iroh could do this

**Gemini** *(condensed-faithful; the honest-parts table is the useful residue)*:

> Native iroh compiled to Wasm cannot hole-punch — sandbox forbids raw UDP; standard
> iroh-in-Wasm is 100% relayed. The hybrid needs: browser-native `RTCPeerConnection` (the only
> API allowed to send UDP hole-punching probes); a WebRTC transport facade
> (`iroh-webrtc-transport`-shaped: maps NodeIds to WebRTC peers, exposes dial/accept/streams)
> **[UNVERIFIED crate]**; opportunistic handoff + watchdog logic branching
> connected(~90%)/failed(~10% → guest error / insider Wasm-iroh WebSocket tunnel).
>
> Relay side: (a) a **WebRTC signaling broker endpoint** — iroh-relay does NOT natively parse
> SDP; a lightweight HTTP/WS route must be added alongside; (b) **STUN** (`enable_stun = true`
> claim); (c) **auth hook** (`access.http` claim) gating tunnel allocation.
>
> Native-vs-custom summary: NodeId identity, encrypted tunneling, relay auth hooks, STUN —
> claimed native; browser UDP hole punching — impossible (browser API only);
> iroh↔WebRTC abstraction — NOT in core iroh (third-party crate); SDP/ICE signaling broker —
> NOT in iroh-relay (custom route).

**Owner:**

> Tell me about this — `enable_stun = true` — Why it exists at all, and what exactly it does —
> If as we say this is true — Iroh-to-WebRTC Abstraction — Bc this all seems like it's
> pluggable capability rather than forked

**Gemini** *(condensed-faithful)*:

> `enable_stun = true` (claimed) spins up an RFC 5389 STUN UDP listener on the relay: a node
> sends a STUN request, the relay echoes back the public IP:port it arrived from. It exists so
> native iroh endpoints get server-reflexive candidates for QUIC hole punching without
> third-party STUN infrastructure. Because STUN is a standard, a browser's WebRTC engine can
> point at the same listener (`iceServers: [{ urls: 'stun:your-relay:3478' }]`). The
> architecture is modular composition, not forking: upstream unmodified iroh-relay (STUN +
> auth hook), unmodified browser WebRTC APIs, and a pluggable adapter crate implementing
> iroh's connection traits over browser WebRTC. **[CORPUS FLAG: iroh 1.0 reportedly moved
> address discovery to QAD — whether a STUN listener still exists in current iroh-relay is
> exactly the kind of knob E131's verification gate must check.]**

**Owner:**

> Review this conversation and critically review for feasability

**Gemini** *(the in-session critical review — the most corpus-useful turn)*:

> The proposed architecture is conceptually sound and economically smart, but it oversimplifies
> several critical integration boundaries.
>
> **What feasibly works:** zero-cost B2B via native STUN (`enable_stun = true` **or QAD
> flags**); gated relay fallback via `access.http` (403 guests / 200 insiders); the browser
> constraint model (no raw UDP in Wasm; RTCPeerConnection is the only hole-punching API).
>
> **Critical gaps:** (1) **iroh-relay is a STUN server, NOT a TURN server** — its forwarding
> engine speaks iroh's DERP/QUIC protocol, not RFC 5766; a browser's ICE agent cannot list it
> as a `turn:` fallback; on direct-connect failure the frontend must explicitly tear down
> RTCPeerConnection and spin up a separate iroh-Wasm WebSocket connection. (2)
> **`iroh-webrtc-transport` is experimental third-party software** — a community experiment,
> not an n0-computer core crate; official browser support focuses on WebSocket tunneling and
> web-transport; relying on it is significant maintenance risk. (3) **Missing signaling
> layer** — iroh-relay does not route SDP; a custom HTTP/WS signaling hub is required. (4)
> **Wasm client complexity** — two distinct networking pathways (browser WebRTC + iroh Wasm)
> means bundle bloat and explicit transport-switching state.
>
> **Pragmatic path:** keep the transports explicitly separated (no leaky unifying
> abstraction); explicit fallback circuit (guests: direct-or-error after ~5 s; insiders:
> direct, then catch failure and open the iroh WebSocket); embed the signaling route in the
> main backend.

**Owner:**

> I meant, do we need to fork iroh, I already need it for other things

**Gemini:**

> No. Standard upstream iroh and iroh-relay, completely unmodified: the (claimed) native STUN
> listener serves browser WebRTC engines as-is; the (claimed) `access.http` hook carries the
> insider gating; everything custom is application glue — a WebRTC SDP signaling route, an
> auth endpoint answering the relay's callouts, and frontend state logic (direct attempt →
> guest error / insider tunnel). Using upstream keeps you compatible with future n0-computer
> updates and your other iroh workloads.

## Body E — Bluesky/Blacksky federation, storage, relays, census, lookups

**Owner:**

> So are bluesky and blacksky like federated? Can you login to one from the other?

**Gemini:**

> Yes — both run on the AT Protocol, one open interconnected network: follow/like/repost/reply
> across servers regardless of where data is hosted; Bluesky operates the default bsky.social
> PDS, Blacksky runs its own PDS infrastructure and moderation within the network. You can
> sign into Blacksky-compatible clients with your Bluesky credentials or an app password — no
> new account needed for a different frontend — and full **PDS account migration** can move
> your data, posts, followers, and handle to Blacksky's servers without losing your graph.

**Owner:**

> How much does blacksky give you vs bluesky in terms of storage and rate limits

**Gemini** *(all figures `[UNVERIFIED]`, condensed-faithful)*:

> Storage: Bluesky bsky.social caps media blobs at 50 MB/file; Blacksky manages its own blob
> limits with automated S3 backups and migration tooling (PDS MOOver). Rate limits: Bluesky's
> points system — 5,000 points/hour, 35,000/day per account (create=3, update=2, delete=1 →
> ~1,666 creations/hour, 11,666/day); 3,000 requests per 5 min per IP; 30 session creations
> per 5 min (max 300/day). Blacksky: independent limits on its Rust (rsky) stack; the
> rsky-wintermute indexer is built for >10k records/sec.

**Owner:**

> I keep seeing conflicting accounts of bluesky storing blobs in users PDS natively vs
> offloading to an object store

**Gemini:**

> Both claims are true at different layers. **Protocol authority:** the PDS is the single
> source of truth for an account's blobs (repo MST records reference blobs by CID; the PDS
> serves raw blobs). **Implementation:** storage sits behind a pluggable `BlobStore` interface
> — `DiskBlobStore` for small self-hosted instances, `S3BlobStore` (S3/R2) for production
> including bsky.social. **Delivery:** end users mostly fetch via AppView/CDN caches
> (cdn.bsky.app) that resize/transcode from the originating PDS.

**Owner:**

> Tell me about the blacksky operation, history, size, values

**Gemini** *(all specifics `[UNVERIFIED]`, condensed-faithful)*:

> Created by Rudy Fraser in 2021, launched June 2023 as a custom feed generator + moderation
> network for Black users migrating from Twitter/X; Blacksky Algorithms founded to build
> dedicated infrastructure; July 2026: open public beta + iOS/Android apps as a standalone
> frontend and network host. Runs a parallel Rust implementation of the AT Protocol (**rsky**):
> own PDS (atproto.africa), relay, indexer (rsky-wintermute, >10k records/sec), AppView;
> rsky-pds swaps SQLite for PostgreSQL + native S3. Values: digital public infrastructure —
> mitigating anti-Black harassment, community self-governance ("middleware agency"),
> open-source reference model for affinity groups. Size: tens of thousands of active users;
> founder/CEO Rudy Fraser, core engineering team + community moderation council; one of the
> most complete independent atproto stacks.

**Owner:**

> So does blacksky relay only connect to blacksky PDS? Or all PDS?

**Gemini:**

> All public PDSs — a relay's job is "big-world" networking: rsky-relay connects to the event
> streams of any reachable PDS (bsky.social, self-hosted, independent), verifies commit
> signatures, and pools them into a single stream feeding Blacksky's indexer, labelers, and
> feed generators. Blacksky-only crawling would blind its users to the rest of the network.

**Owner:**

> I though the PDS called home and then the relay setup a feed tunnel — If so, how does
> blacksky make it work? Was their relay put into the PDS config?

**Gemini:**

> Slightly the reverse: the PDS is a publisher, the relay a subscriber. Every PDS runs an open
> WebSocket firehose (`com.atproto.sync.subscribeRepos`); `requestCrawl` is just an HTTP ping
> ("I exist at this URL — please subscribe"), after which the relay opens the WebSocket.
> Blacksky's relay discovers PDSs three ways: (1) PDS config (`PDS_CRAWL_HOSTS` /
> `PDS_CRAWLERS` — operators can add Blacksky's relay alongside bsky.network); (2)
> **plc.directory crawling** — monitoring the DID audit log for new accounts and parsing each
> DID document's PDS service URL, then connecting without waiting for a ping; (3)
> relay-to-relay subscription to an upstream firehose, re-verifying signatures.

**Owner:**

> So how long would a new relay take to see that event from all PDS? And how does it send it?
> Explain — "rsky-relay constantly monitors the plc.directory audit log for new accounts."

**Gemini** *(figures `[UNVERIFIED]`, condensed-faithful)*:

> Latency: already-connected PDS ≈ 100 ms–2 s (WebSocket push on post); new PDS via
> requestCrawl ≈ 1–5 s; new-account discovery via plc.directory polling ≈ 5–30 s;
> relay-to-relay peering < 500 ms. A relay is a WebSocket fan-in/fan-out multiplexer: ingest
> hundreds of PDS firehoses; verify signatures, dedupe, assign a unified sequence number;
> serve one outbound `subscribeRepos` stream of CAR frames to indexers/search/feeds/moderation.
>
> plc.directory monitoring: the directory exposes an append-only audit log at `/export` (every
> create/handle-change/key-rotation/migration appends a JSON operation carrying
> `services.atproto_pds.endpoint`). rsky-relay cursor-polls `/export?after=<last_seen>`,
> parses new operations, checks its connection table for each referenced PDS, opens WebSockets
> to unknown hosts, and maintains an identity cache (DID → PDS) so migrations and handle
> changes propagate to downstream consumers as identity events.

**Owner:**

> So a PDS is effectively a data store with an API and an oauth server — How many of them are
> there? How many user repos do they usually host?

**Gemini** *(figures `[UNVERIFIED]`, condensed-faithful)*:

> Yes — repository (MST records) + blob store, XRPC API, OAuth identity endpoint. Roughly
> **3,100–3,500 active reachable PDS hosts** (16,000+ all-time URLs in plc.directory, many
> short-lived). Distribution is an extreme power law: Bluesky corporate shards
> (~80–100 mushroom-named nodes behind an entryway, 100k–500k+ repos each, 62M+ total);
> community/regional hosts (1k–50k repos — Blacksky ~35k, Tangled ~21k, Eurosky ~7k);
> self-hosted/family nodes (1–10 repos — the vast majority of hosts). bsky.network caps new
> unverified third-party PDSs at 100 accounts until a limit increase is requested.

**Owner:**

> Explain the blaky PDS and directory relationship and explain the bluesky PDS setup them the
> blacksky then compare and contrast

**Gemini** *(condensed-faithful)*:

> **Blacksky ↔ directory:** account creation on a Blacksky PDS generates a keypair and
> registers a DID on plc.directory mapping DID → public signing key + service endpoint
> (atproto.africa); resolution runs through rsky-identity; because the directory entry lives
> outside both companies, migration = update the entry, keep graph/handle/history.
>
> **Bluesky PDS (@atproto/pds):** TypeScript/Node reference implementation; SQLite + local
> disk in dev, PostgreSQL/CockroachDB + S3/R2 in production; dozens of sharded worker nodes
> behind an Entryway proxy; defaults wired to bsky.network relay and Bluesky AppView/
> moderation.
>
> **Blacksky PDS (rsky-pds):** Rust; PostgreSQL and S3 native out of the box; Mailgun for
> account flows; containerized cloud-portable deployments; natively wired to rsky-relay,
> rsky-wintermute, rsky-labeler.
>
> Similarities: full protocol compliance, cross-server interaction, plc.directory/did:web
> portability. Differences: monolithic corporate host optimization (TS shards) vs
> low-cost self-governing community hosting (Rust); default service stacks (PBC pipeline vs
> the rsky suite).

**Owner:**

> Define how lookups work with examples and analogies and tech ical detail about tech stack,
> operation and workflows

**Gemini** *(condensed-faithful)*:

> A lookup resolves a human-readable handle into a persistent cryptographic identity, locates
> the hosting server, and retrieves records. Analogy ladder: handle = vanity phone number; DID
> = ID number (permanent); PLC directory = the registry mapping ID → current home address; PDS
> = the address; record = the document in the drawer.
>
> **Stage 1 — handle → DID:** DNS TXT at `_atproto.<handle>` (`"did=did:plc:…"`) or HTTPS
> `https://<handle>/.well-known/atproto-did`; bidirectional verification via the DID doc's
> `alsoKnownAs` (`at://<handle>`), else `handle.invalid`.
> **Stage 2 — DID → PDS:** GET `https://plc.directory/<did>` → DID document with
> `verificationMethod` (Multikey) and `service[#atproto_pds].serviceEndpoint`.
> **Stage 3 — record fetch:** XRPC
> `GET /xrpc/com.atproto.repo.getRecord?repo=<did>&collection=app.bsky.feed.post&rkey=<rkey>`
> → `{ uri, cid, value }` from the PDS's MST.
>
> Implementations: TypeScript `@atproto/identity` (IdResolver; dns.promises + fetch; LRU/Redis
> caching); Rust `atproto-identity`/`rsky-identity` (Tokio, reqwest, hickory-dns, storage_lru;
> `resolve_subject(&http, &dns, "alice.com")`). Spec table: DNS TXT / well-known text/plain /
> plc.directory JSON-LD / did:web well-known / XRPC getRecord.

*(End of paste.)*
