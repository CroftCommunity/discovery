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
standing correction (updated 2026-06-22): **iroh is now released at `1.0.0`** (the relay lab
pins `=1.0.0`; first-party Swift bindings `iroh-ffi` shipped with 1.0 mid-June 2026). Earlier
"`1.0.0-rc.0`" / "pre-1.0 / v0.97" notes are superseded. Companion crates remain pre-1.0
(iroh-docs 0.100, iroh-gossip 0.100, iroh-blobs 0.102). The endpoint identity type was renamed
`NodeId`→`EndpointId` (0.94).

---

## 1. Transport & substrate

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| n0 | iroh | QUIC-first P2P networking (Rust) | EndpointId (Ed25519), hole-punching, relays (ex-"DERP"), QUIC-multipath migration; iroh-gossip (HyParView/Plumtree), iroh-docs (range-based set reconciliation, LWW), iroh-blobs (BLAKE3); `unstable-custom-transports` (0.97+, ≥1,200-byte datagrams); first-party `iroh-ffi` Swift/Kotlin/Py/JS bindings | **`1.0.0`** (relay lab pins `=1.0.0`, API verified vs source); Swift bindings mid-June 2026; in production in Delta Chat, Nous Research (distributed LLM training), Paycode (POS) [verified: relay-lab + web 2026-06-22] | build-on, partner, learn↔ |
| community | iroh custom transports | pluggable QUIC-over-anything | `mcginty/iroh-ble-transport` (BLE, community; + `blew` crate, `BlewChat` **unencrypted** demo); `n0-computer/iroh-tor`, `n0-computer/iroh-nym` (metadata privacy); `iroh-pkarr-node-discovery` | early/experimental; BLE is community not core; `iroh-webrtc-transport` claimed but not found [verified: web 2026-06-22] | learn↔ (future off-grid/anonymity transports) |
| Defense Unicorns | **Peat** (+ `peat-gateway`) | Off-grid/denied-environment P2P data-sync middleware (Rust) | **Iroh** transport (QUIC/BLE) + **Automerge** CRDTs + **MLS** group security; stitches servers/Android/RPi/drones/ESP32 into a self-healing mesh; ATAK integration; `peat-gateway`→Okta/Keycloak when a link returns | active open-source; production defense/disaster/industrial use [verified: web 2026-06-22 — github.com/defenseunicorns/peat] | **build-on, learn↔ — strongest prior art for Croft's exact substrate bet (Rust+iroh+CRDT+MLS), proven in denied/degraded** |
| — | libp2p | Modular P2P stack | Transports, pubsub, DHT | mobile-weak vs iroh; rejected as primary [verified: dossier] | homage |
| Veilid team | Veilid | Privacy-first P2P with source-address-free routing | Ed25519/x25519/XChaCha20/BLAKE3/Argon2; DHT (small mutable records) | demoted to future metadata-resistant messaging-layer candidate; no large-blob primitive [verified: dossier] | learn↔ (future) |
| — | Holochain | Agent-centric P2P (no global consensus) | source chains, rrDHT, validation rules, membrane proofs | dropped as substrate (uses iroh transport anyway; mobile-weak) [verified: dossier] | homage (borrow patterns) |
| Earthstar / Willow team | Willow protocol / willow-rs | Local-first data model with true deletion | 3D data model, Meadowcap capabilities, prefix-pruning | not shippable in 2026; design "Willow-shaped," migrate later [UNVERIFIED current] | homage, build-on (later) |
| John Day / community | RINA (Recursive InterNetwork Architecture) | Recursive scoped-addressing networking | "networking is one recursive layer repeated at scale"; bounds routing state by recursion, not a flat global table | research/academic [dialogue-sourced 2026-06-20] | learn↔ (the closest formalization of Croft's recursive-federation routing) |
| — | Named Data Networking / Yggdrasil / cjdns | Hierarchical-name / cryptographic-identity overlay routing | NDN routes on aggregatable hierarchical names; Yggdrasil/cjdns route over crypto-identity trees with no global table, locality-aware | NDN research; Yggdrasil a working small-scale net [dialogue-sourced 2026-06-20] | learn↔, build-on (federation routing PoC prior art; Yggdrasil ≈ the PoC target shape) |
| community | **p2panda** | Building blocks for peer-to-peer applications | local-first app primitives over a p2p substrate; listed in `awesome-iroh` as a conceptual neighbor | [dialogue-sourced 2026-06-24, pending verification] | learn↔ (closest "p2p app building blocks" neighbor; read its peer-equality framing) |
| community | **iroh-rings** | Relationship-based access control for resources over iroh | RBAC-shaped capability/relationship layer on the iroh substrate | [dialogue-sourced 2026-06-24, pending verification] | learn↔ (direct neighbor to Drystone's peer-equality / capability layer — compare before §5 hardens) |

## 2. Group encryption & crypto primitives

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| IETF MLS WG | MLS / RFC 9420 | Standard group key agreement | per-epoch rekey, forward secrecy, post-compromise security; assumes a delivery/ordering service | published RFC; the standard we build group encryption on [verified] | homage, build-on |
| Phoenix R&D / community | openmls | Rust MLS implementation | external-commit builder, reinit, fork_resolution module | `0.8.1`; proven to express survivor re-key with PCS [verified: Proofs/lineage-groups PR #8] | build-on, learn↔ |
| Phoenix R&D | **DMLS / FREEK** (decentralized MLS) | Serverless MLS ordering without a privileged delivery service | extends MLS to process out-of-order Commits with **reduced FS loss** via **FREEK** (Fork-Resilient CGKA; Alwen/Mularczyk/Tselekounis) — a **puncturable-PRF** recovers most forward secrecy at a **storage cost scaling with fork frequency** | IETF draft + **PoC OpenMLS fork** (no production deployment as of mid-2026); a Matrix-side DMLS fork (uhoreg) also exists [web 2026-06-26, **confirm before publish**] | **learn↔ — the closest decentralized-MLS sibling to Drystone's serverless §7 ordering; quantifies the fork→FS cost (T29/T22)** |
| (Germ / NPS — see §6) | **`draft-xue-distributed-mls`** ("TwoMLS") | Serverless MLS via per-member "Send Groups" | PCS+FS **without global ordering consensus**, for P2P/partitioned topologies | IETF draft (IETF 124) [also listed §6 Germ DM] | learn↔ (second independent decentralized-MLS approach) |
| IETF MLS WG (deployment status) | MLS in production | What actually ships | **every production MLS deployment is server-ordered** (centralized delivery service): Webex, Wire, Discord; Google/Apple **RCS MLS-E2EE (May 2026)** | the serverless escapes above are drafts/PoC — so "no *production* deployment is serverless-ordered" is the airtight claim [web 2026-06-26, **confirm before publish**] | the empirical anchor for "different, not weaker" (03) |
| Cryspen / community | hpke-rs | HPKE for MLS | RFC 9180 HPKE | MPL-2.0 (mandatory for RFC 9420; no permissive substitute) — our open license-gate item [verified: PR #8] | build-on |
| Signal Foundation | Signal Protocol | 1:1 + group E2EE benchmark | X3DH, Double Ratchet, sender-keys, sealed sender | the UX/security gold standard; centralized [verified: research] | homage, learn↔ |
| RustCrypto | k256 / DAG-CBOR / CID crates | secp256k1, content addressing | low-S secp256k1, DAG-CBOR, multibase | `k256 0.13` (stable, not RC) [verified: PR #4] | build-on |

## 3. CRDT / local-first data

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| Ink & Switch / community | Automerge | CRDT for shared mutable state | multi-writer merge, change history | Automerge 3.0 claimed ~10× (up to 100×) memory reduction vs v2 [UNVERIFIED version] | build-on, learn↔ |
| Ink & Switch | "Local-first software" (Kleppmann et al. 2019) | The local-first thesis | 7 ideals; single-user value before network effects | foundational essay [verified: dossier] | homage |
| M. Kleppmann et al. | BFT-CRDTs / Blocklace | The formal underpinning of Croft's fork/merge plane | "Making CRDTs Byzantine Fault Tolerant" (PaPoC 2022); "Byzantine Eventual Consistency…" (2020); Blocklace (2024) — hash-DAG CRDTs immune to tampering/sybil; equivocation is the residual hashes can't catch | papers [dialogue-sourced 2026-06-20, links verified in-session] | build-on, learn↔ (the productive tension: his SEC auto-converges; Croft gates reconvergence per-plane) |

## 4. Identity, trust & capabilities

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| Bluesky PBC | AT Protocol | Federated public social protocol | DIDs, handles, PDS, Jetstream firehose, lexicons, labelers | live; custom NSIDs propagate w/o registration; labelers pull-only; writes via entryway, reads via PDS [verified: PR #4/#6 live] | build-on (public path), partner, rebroadcast, learn↔ |
| Bluesky PBC | did:plc | DID method | signed, auditable op log (PLC audit) | live; flagged centralization vector + consumer-scale recovery unpolished [verified: research/dossier] | build-on, learn↔ |
| W3C | DIDs / Verifiable Credentials | SSI standards, blockchain-free | self-certifying identifiers, VC issuance | standards; our identity favors these + did:web [UNVERIFIED current] | homage, build-on |
| Spritely Institute | Goblins / OCapN / CapTP | Object-capability distributed programming | "designation is authorization," POLA, petnames | 501(c)(3), NLnet/NGI-funded, no VC/tokens — a funding/governance model too [UNVERIFIED current] | homage, partner, learn↔ |
| — | BrightID / petname systems | Proof-of-personhood / human-readable local naming | web-of-trust, Sybil resistance | [UNVERIFIED current] | learn↔ |
| Trust over IP Foundation | ToIP Decentralized Trust Graph WG | Trust-graph standardization | transitive trust, Merkle proofs | no de-facto standard yet [verified: dossier] | partner, learn↔ |
| DIF / spec authors | did:webvh (fka did:tdw) | "web + verifiable history" DID method | SCID-anchored append-only log; pre-rotation (`nextKeyHashes`); `portable:true` (genesis-only) for "credible exit"; `/whois` LinkedVP | spec; **not a blessed atproto method** — Newbold: adoption conditionally on the table, portability explicitly does NOT fit atproto's immutable-DID model [dialogue-sourced 2026-06-20, pending independent verification] | build-on (the portable provenance root), learn↔ |
| — | didwebvh-rs / didtoolbox | did:webvh implementations / validators | log-chain validation, SCID continuity, pre-rotation key provisioning | named as the build-it-today tooling [dialogue-sourced, pending verification] | build-on |
| Bluesky PBC | plc.directory | central did:plc directory/registry | resolve `GET /{did}`; audit log `/{did}/log/audit`; 12M+ ops; self-certifying (transparency-log-not-CA) | live; the known centralization soft spot; governance handoff to a nonprofit planned, **not done** [dialogue-sourced; aligns with `plc-identity-resilience.md`] | build-on (Bluesky spoke), learn↔ |
| bluesky-social | goat (Go AT CLI) | atproto account/identity CLI | `account plc recommended → edit → request-token → sign → submit` (email-token gated); PDS signs/forwards | named as the real PLC-op flow [dialogue-sourced, pending verification] | build-on |
| Hive community | Hive | on-chain social blockchain | owner/active/posting/memo key hierarchy; `json_metadata` / `custom_json` arbitrary fields | live; **no DID, no `alsoKnownAs`** — provenance only via signed custom metadata (bespoke) | learn↔ (a spoke; the weakest-linkage case) |
| Google / academia | Certificate Transparency (RFC 6962) + CT gossip; CONIKS | append-only transparency logs; per-principal monitoring | the equivocation-*detection* model — each principal monitors its own binding, gossip cross-check forces non-equivocation, no trusted center | CT in production; CONIKS research (USENIX Sec 2015) [dialogue-sourced 2026-06-20] | build-on, learn↔ (the four-substrate-guarantees / capture-detection model) |
| DIF | DIDComm Mediator Coordination / Pickup | hold-and-forward for offline DID controllers | near-exact prior art for the capability-only, offline-principal **delegate** | spec [dialogue-sourced 2026-06-20] | build-on, learn↔ |
| Ink & Switch | Keyhive | Convergent capabilities + membership graph for local-first | signed membership/capability graph syncing alongside content; **removal + re-encryption as first-class convergent operations** (generation-bounded revocation, not expiry) | research, in flight [dialogue-sourced 2026-06-24, pending independent verification] | learn↔, **candidate Track B** for Drystone's capability layer (§X.5.3; vs Meadowcap-shaped Track A) |
| Earthstar / Willow team | Meadowcap | Capability layer of the Willow stack | unforgeable delegated tokens; attenuation by subsetting; delegate by signing with receiver's key, verified recursively; **no native revocation** (only expiry / namespace deletion — "you cannot un-sign") | spec [dialogue-sourced 2026-06-24, pending verification] | learn↔, **candidate Track A** for Drystone's capability layer (§X.5.2; tokens + epoch keys, expiry-bounded revocation) |

## 5. Social protocols & federation

| Org | Project | Purpose | Capabilities | Current state | Relationship |
|---|---|---|---|---|---|
| W3C | ActivityPub | Federated social standard | server-to-server, actor model | dropped for v1 (conflicts with local custody behind NAT); federation priority #1 for later [verified: dossier] | rebroadcast, homage |
| Nostr community | Nostr | Simple relay-based protocol | signed events, relays | federation priority #3 [UNVERIFIED current] | rebroadcast, learn↔ |
| Ryan Barrett | Bridgy Fed | AP ↔ ATProto bridge | cross-protocol federation | the bridge reference [verified: dossier] | build-on, partner |
| Matrix.org Foundation | Matrix | Federated E2EE group chat | Olm/Megolm; MLS migration in progress | MLS still in design (MSC4256/4244, arewemlsyet.com); 25+ govt deployments [verified: research] | homage, learn↔ |
| Solid / Inrupt (Berners-Lee) | Solid + WebID + Solid-OIDC | User-owned **Pods** (general-purpose web storage); apps read/write **directly** to the Pod | RDF/Turtle/JSON-LD over HTTP; **WebID** profile URL; **Solid-OIDC** (issuer discovery from the WebID, `solid:oidcIssuer`) + **DPoP** (RFC 9449) token binding; granular per-file ACLs; private-by-default | live; W3C-standards-based; Inrupt commercializes [verified: web 2026-06-22] | homage, learn↔ (the *private-by-default, direct-access* pole vs atproto's public indexed pipeline — Croft sits between, w/ an E2EE private layer that is neither) |
| Project Liberty (F. McCourt) | DSNP + Frequency | Social graph as a **public utility** (decouple graph/data from app layer) | identity = keypair (platform-unrevokable); social graph = portable on-chain events; content = on-chain announcements → off-chain media; **no built-in crypto token in the core protocol**; **delegation** to user-agents w/o surrendering master keys; reference chain = **Frequency** (Polkadot parachain, capacity/staking not per-tx fees) | live; whitepaper + Frequency parachain [verified: web 2026-06-22 — dsnp.org, CoinDesk] | homage, learn↔ (the *blockchain-consensus-layer* social-graph pole; Croft rejects the chain but shares the unbundle-the-social-web + delegation goals) |

## 5b. AT Proto "atmospheric web" apps & Rust tooling

The community term for non-social apps built on AT Proto is the **"atmospheric web"** (per
atproto.com). All rows below verified via web 2026-06-22 (see
`seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`). Relevant because the
"web of docs / Neo-GeoCities / open-LinkedIn" product vein (see `thinking/atproto-atmospheric-web.md`)
would build alongside these.

| Org/Author | Project | Purpose | Current state | Relationship |
|---|---|---|---|---|
| — | Tangled (tngl.sh / tangled.org) | Decentralized Git collaboration on ATProto | live; `user.tngl.sh` handles, federated PRs, self-host "Knots" [verified: web] | learn↔, build-on (Smoke Signal hosts here) |
| — | WhiteWind (whtwnd.com) | Markdown blogging, data on PDS | live, OSS [verified: web] | homage, learn↔ |
| Leaflet team | Standard.site | Long-form publishing lexicon set on PDS | live; block-based, **not** Markdown-only [verified: web] | learn↔ |
| — | Leaflet (leaflet.pub) | Long-form/social publishing on PDS | live; block editor [verified: web] | learn↔ |
| — | Semble (semble.so) | Research knowledge network (NOT a Linktree clone) | live, on ATProto [verified: web] | learn↔ |
| — | Smoke Signal (smokesignal.events) | Decentralized Meetup/Eventbrite | live, MIT OSS, 1yr 2025-07-14 [verified: web] | homage, learn↔ |
| — | npmx | npm registry browser w/ ATProto sign-in | live [verified: web] | learn↔ |
| Livepeer-funded | Streamplace | Livestreaming over ATProto | live, OSS [verified: web] | learn↔ |
| S. Vogelsang | Flashes | Instagram-like photo client on ATProto | live (3rd-party) [verified: web] | homage |
| Automattic | ATmosphere (WordPress plugin) | Bridges WordPress → ATProto (publishes site.standard.* to PDS) | v1.0.0 May 2026 [verified: web] | rebroadcast, learn↔ |
| @mozzius | Graysky | Alt Bluesky client; defined `app.graysky.*` | live; the custom-namespace exemplar [verified: web] | homage, learn↔ |
| Rudy Fraser | Blacksky | Independent ATProto infra (own relay, Ozone, Rust "rsky") | live; AppView in dev [verified: web] | homage, learn↔, partner |
| zeppelin-social | Zeppelin AppView | Independent full-network Bluesky AppView | ~16 TB / ~$200-mo Hetzner; **decommissioned** Fall 2025 [verified: web] | learn↔ (the full-mirror cost lesson) |
| sugyan | ATrium (atrium-rs) | Rust AT-Proto framework | live; atrium-lex + atrium-codegen (lexicon→Rust), bsky-sdk [verified: web] | build-on (Rust client path) |
| @ksk001100 | bsky_tui | Rust TUI Bluesky client (Ratatui+Tokio+atrium) | live [verified: web] | homage (decoupled-presentation proof) |
| Bluesky / community | Tap | Official Go repo-sync/backfill tool: subscribe to a Relay + auto `getRepo` backfill (events marked `live:false` → live), SQLite/Postgres | live, OSS [verified: web 2026-06-22 — atproto.com/blog/introducing-tap] | build-on (if Croft builds any AppView/indexer/backfill) |

Private-groups/E2EE on AT Proto are **third-party**: **Germ DM** (MLS, §6 below) and the
**XMTP↔Bluesky bridge** (XMTP Labs `bluesky-chat`). This gap is what Croft's lineage-groups MLS proof
answers — see COHESION §17. **Nuance (2026-06-22, COHESION §26):** the *fictional* "AT Messaging /
MLS-standardizing working group" remains REFUTED, but a **real, community-led ATProto Private Data
Working Group** does exist (atproto.wiki / discourse.atprotocol.community, Boris Mann; GitHub #3363
"Namespaces"→"buckets/realms", #121 "Encryption for private content"; Paul Frazee *informally*). It is
converging on **access-controlled, PDS-gated** private data (PDS as a trusted agent); **true E2EE /
zero-knowledge is explicitly deferred** — so native-in-protocol E2EE still doesn't exist, and Croft's
host-untrusted MLS answer is *more* differentiated, not less.

## 5c. App-layer tooling & clients (from the 2026-06-20→22 app dialogue — pending independent verification)

These surfaced in the Croft app design dialogue and are recorded here so they are not lost. **Not
independently re-verified this session** — treat as dialogue-sourced `[UNVERIFIED]` until checked
(ROADMAP §14 follow-on). ATrium, Graysky, Blacksky/rsky, the full-mirror cost lesson, and iroh are
already covered above (§5b, §1).

| Org/Author | Project | Purpose | Relationship |
|---|---|---|---|
| — | Jacquard (jacquard-rs) | Rust AT-Proto crates; zero-copy borrowed deserialization, ergonomic OAuth; the lower-boilerplate alternative to ATrium | build-on (Bluesky module, behind a port) |
| h3poteto | megalodon-rs | Multi-server fediverse client (Mastodon/Pleroma/Friendica/Firefish/GoToSocial/Pixelfed) one interface; Apache-2.0 | build-on (the "AP" module targets the Mastodon client API, not AP C2S) |
| LemmyNet | lemmy-client-rs | Official Rust Lemmy client; WASM-aware (browser fetch), manages version skew | build-on (Lemmy pond) |
| Red Badger | Crux | Hexagonal Rust app framework: side-effect-free core, effects-as-data, WASM+native | homage, learn↔ (adopt the *pattern* slim, not the pre-1.0 framework) |
| Tauri / WRY | Tauri v2 | Rust shell, web frontend, all 5 platforms incl. Android; webview-per-OS | build-on (the desktop/mobile shell) |
| — | Leptos | Rust fine-grained-reactive web UI → WASM (same-memory boundary with the core) | build-on (the web shell render path) |
| — | Dioxus | Rust cross-platform UI (web/desktop/mobile) | homage (the Path-B alternative not chosen) |
| @cheeaun | Phanpy | Open web Mastodon client; deliberately de-emphasizes engagement actions | homage, learn↔ (closest values-aligned client; multi-column) |
| — | deck.blue | TweetDeck-style column Bluesky client | homage (per-source column = the composable unit) |
| Openvibe | Openvibe | Combined-timeline multi-network app (Mastodon/Bluesky/Nostr/Threads) | learn↔ (the *fused-timeline* anti-pattern — confirms honest-seams) |
| Apps incub. | Fedilab | Fediverse client, simple-by-default + more-on-demand | homage (progressive disclosure shipped) |
| Merlin (n0/community) | webxdc + Delta Chat mini-apps | Small web-bundle apps over iroh realtime; the topic+ticket handoff; the WebRTC-transport-swap porting recipe | homage, learn↔ (the **pads/games pond** reference) |
| Tencent / W3C | WeChat mini-programs · W3C MiniApp | Super-app guest-app model; permission scopes; the gatekeeper trap to reject | learn↔ (borrow the grammar, reject the central distribution/observation) |
| — | kbone · uni-app · Re.Pack/react-native-sandbox | Open mini-app runtimes: web-in-mini-app shim; one-codebase-many-hosts; guest crash-containment/isolation | learn↔ (guest-isolation + permission patterns for pads) |
| Zed / Excalidraw / Spacedrive | (crafted-app references) | Renderer-spectrum + craft-discipline references; Spacedrive = closest Rust-core-shared-web-UI twin (cautionary: shell-demos-easy, finishing-hard) | learn↔ |

## 5c-2. Digital-garden / personal-space projects (from the 2026-07-10 Croft.ing dialogue — pending independent verification)

Surfaced in the Croft.ing website / plot-tender design dialogue
(`seeds/transcripts/raw/croft-website-plot-tender-design-dialogue-2026-07-10.md`). **Not independently
re-verified this session** — treat as dialogue-sourced `[UNVERIFIED — dialogue-sourced 2026-07-10]` until
checked. They matter as the closest prior art for the Croft.ing personal-plot surface (the "atmospheric web"
product vein, §5b / `thinking/atproto-atmospheric-web.md`, ROADMAP_TODO E1). Standard.site (§5b, verified)
is the adoptable blogging lexicon (`site.standard.document`) and is not repeated here.

| Org/Author | Project | Purpose / relevance | Current state | Relationship |
|---|---|---|---|---|
| Hypha Co-op | spores.garden | Personal-website / digital-garden builder on ATProto; palette + isoline background + **flower identicon derived from the immutable DID**; users "plant" flowers in each other's gardens | `[UNVERIFIED — dialogue-sourced 2026-07-10]` (named live in-dialogue) | learn↔ (closest live prior art for the plot surface + DID-derived personalization) |
| — | Neocities | Modern static personal-page host, the GeoCities successor (raw HTML/CSS creative freedom) | `[UNVERIFIED — dialogue-sourced 2026-07-10]` (widely known; not verified this session) | homage, learn↔ (the "persist the prior great things" reference) |
| — | Linkat | Link-in-bio on ATProto | `[UNVERIFIED — dialogue-sourced 2026-07-10]` | learn↔ (the profile-card built on public reads) |
| — | Bio.blue | Link-in-bio on ATProto | `[UNVERIFIED — dialogue-sourced 2026-07-10]` | learn↔ |
| — | kibun.social | Status / `/now` update via a custom status lexicon | `[UNVERIFIED — dialogue-sourced 2026-07-10]` | learn↔ (the live-status widget reference) |
| Cohost team | Cohost (defunct) | Modern ad-free cooperative social platform; beloved for in-post "CSS crimes" (interactive games/puzzles); **centralized, no ads/VC → shut down 2025-01-12 [confirm before publish]** | defunct | cautionary prior-art (the financial-gravity rug-pull the serverless/sovereign design answers; alongside GeoCities/MySpace) |

## 5c-3. Croft-owned live properties (shipped 2026-07)

These are not prior art — they are Croft's own shipped surfaces, cross-repo under the `CroftCommunity` org. Recorded here so the register reflects what exists rather than only what was surveyed. Verified live via their deploys in the source sessions; treat exact feature state as `[UNVERIFIED — session-sourced 2026-07]` until re-checked.

| Property | Repo | Purpose | Relationship |
|---|---|---|---|
| arecipe.app | `CroftCommunity/arecipe` | The first working crop: recipe box + meal planner, backendless PWA on ATProto/Bluesky | the reference product surface |
| arecipe.croft.ing | `CroftCommunity/arecipe_treatise` | One-page treatise: the ENS-resistance / four-preconditions argument for the backendless design | the "why it is built this way" long-form |
| croft.ing | `CroftCommunity/crofting_site` | The landing/guide site (Plot/Wall/Valley pillars, five-tier depth, guide pages, ENS terms page) | the homestead surface |
| skylite.croft.ing | `CroftCommunity/skylite` | Tended Bluesky window (sponsor-curated, no-algorithm view; kids/grandparents/quiet-sky use) | the "gate in the wall" surface |
| pdsview.croft.ing | `CroftCommunity/pdsview` | Standalone zero-dependency SPA/PWA browser for public ATProto PDS content (browse collections/records/blobs + export); layer-honest about CORS | the public-repo inspector |

## 5d. Games & app-pond building blocks (from the ponds/games dialogue 2026-06-21)

Surfaced and partly verified in the ponds/games deep-dive (`thinking/app/ponds/`); verification status
is per the artifacts (licenses checked via GitHub mirror; some Codeberg-hosted licenses want a final
glance at bundle time). iroh-blobs/docs/gossip themselves are §1.

| Org/Author | Project | Purpose / relevance | Relationship |
|---|---|---|---|
| n0 | sendme | iroh-blobs file-transfer reference (the device-to-device drop pad, near-free) | build-on |
| sunbeam-stdio | **libmarathon / Marathon** | Offline-first multiplayer framework on **Bevy + iroh + iroh-gossip + CRDTs**; 3D cube-sim demo (macOS/iOS) — the closest "iroh-gossip + CRDT game" prior art | learn↔, build-on [verified: web 2026-06-22 — crates.io/crates/libmarathon] |
| Chad Fowler | **ascii-royale** | 16-player terminal ASCII battle-royale in Rust over iroh; host prints a ticket, no servers — proves direct-QUIC twitch play | homage, learn↔ [verified: github chad/ascii-royale] |
| rustonbsd | **iroh-lan** | Hamachi-like encrypted virtual-LAN over iroh → legacy LAN-only games (Minecraft Java, StarCraft, CS 1.6) + **emulator netplay (Snes9x/RetroArch)** over the internet, no port-forwarding | learn↔, build-on (the "tunnel localhost over iroh" pattern) [verified: github rustonbsd/iroh-lan] |
| community | **godot-iroh** | Godot Asset-Library extension swapping Godot's default multiplayer socket for an iroh endpoint (connect by Node ID, no port-forwarding) | build-on [verified: web 2026-06-22] |
| vinay-winai / schollz / n0 | **DataBeam** (croc + sendme) | Desktop GUI uniting **croc** (schollz; code-phrase P2P file transfer) convenience with **sendme** (iroh) speed/resumability — convenience-meets-iroh exemplar | learn↔ [verified: croc + sendme real; project live] |
| adbenitez / ArcaneCircle | webxdc game catalog (chess, wonster word puzzle, many) | the wrappable catalog; mixed licenses (several GPL-3.0; chess piece art CC-BY-SA-3.0 — flagged trap); wonster MIT | wrap (via webxdc-compat shim), homage, learn↔ |
| rameshvarun | netplayjs | Rollback netcode + WebRTC; host-authoritative state doubles as a hidden-info dealer; swap signaling for iroh | port substrate (confirm license at bundle) |
| gschup / community | GGRS + matchbox | Rollback netcode in Rust + WebRTC signaling; more architecture-aligned than netplayjs for the Rust core | build-on (twitch-tier games) |
| community | Curvytron (Curve Fever / Achtung die Kurve) | MIT fork base for the standout 2-8p twitch game | port/wrap (MIT) |
| nicolodavis | boardgame.io | MIT turn-based engine with a transport abstraction; JS/React (architecture-fit tension with the Rust core) | learn↔ (turn-based patterns) |
| Cure53 / OpenTechFund | webxdc security audit | The audit proving CSP alone doesn't contain a webview (WebRTC + DNS-prefetch exfiltration; FILL500 fix) | learn↔ (security model; **disable webview WebRTC**) |
| Apple | Foundation Models framework | On-device ~3B model (iOS/macOS 26), guided generation (`@Generable`) constrains output to a real catalog (anti-hallucination); macOS=M-series; needs Swift↔Rust bridge | build-on (optional assistant; macOS target) |
| Google | Gemini Nano (AICore + ML Kit GenAI) | On-device model; strong privacy isolation but steep device cliff (~flagship-only) + weaker structured output | build-on (optional assistant; Android target, fallback-heavy) |
| — | Bond Touch (and similar) | The "thinking-of-you" bracelet — built a business/account/cloud-relay around ~50 bytes; the anti-pattern the free perpetual ping rebukes | learn↔ (negative example) |

## 5e. AT Proto PDS self-hosting: implementations, hosts & blob-storage backends (from the 2026-06-22 atproto/PDS dialogue)

Surfaced and web-verified 2026-06-22 (see
`seeds/transcripts/raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22-FACTCHECK.md`).
Relevant because Croft's substrate stance ("must survive as small self-hosted nodes," the cooperative
/ non-extractive hosting question — ROADMAP_TODO E20/E22) is exactly the choice these projects answer.
**Pricing is point-in-time/volatile — treat $ figures as illustrative, not current.** The official
reference PDS (`@atproto/pds`, TypeScript) is **single-tenant SQLite** (per-user `.sqlite` repos +
PDS-wide DBs, local-FS-bound); the alternatives below add Postgres.

| Org/Author | Project | Purpose / relevance | Current state | Relationship |
|---|---|---|---|---|
| haileyok | Cocoon | Alternative PDS in **Go** with a **PostgreSQL** backend (shares an existing DB cluster, unlike official SQLite) | live; self-described "highly experimental, not production-ready" [verified: web 2026-06-22] | learn↔ (the Postgres-PDS path) |
| Blacksky (Rudy Fraser) | rsky-pds | Alternative PDS in **Rust** (Postgres + S3 blobs + Mailgun); part of the `rsky` workspace (§5b Blacksky row) | live [verified: web 2026-06-22] | build-on, learn↔ (Rust-PDS path; closest to Croft's stack) |
| ElfHosted | Managed Bluesky PDS | Fully-managed PDS hosting (provision/HTTPS/updates; point your domain) | live; ~$9/mo cited but store shows a $1/7-day intro trial — **price unconfirmed** [verified: web 2026-06-22] | learn↔ (the managed-host model; cooperative-vs-SaaS tension, E20) |
| DigitalOcean | BlueSky Social PDS 1-Click app | Official Marketplace 1-Click PDS droplet (bundles Caddy); the **genuine** 1-click PDS host | live (slug `blueskysocialpds`); droplet ~$4-6/mo (volatile) [verified: web 2026-06-22] | learn↔ |
| Hostinger | Bluesky PDS VPS template | One-click Docker VPS template w/ the official PDS image | live; ~$6.49/mo (volatile) [verified: web 2026-06-22] | learn↔ |
| Vultr | (PDS installer target — **not** a Marketplace app) | Supported VPS target for the official `bluesky-social/pds` installer (`pdsadmin` works); Gemini's "1-Click Marketplace PDS app" claim was **REFUTED** | n/a (no marketplace app) [verified: web 2026-06-22] | note (correction, not a partner) |
| Backblaze | B2 (+ Cloudflare Bandwidth Alliance) | S3-compatible blob backend; ~$6/TB, **free egress** when served via Cloudflare — strong PDS blob store | live [verified: web 2026-06-22; price volatile] | build-on (blob backend) |
| Cloudflare | R2 | S3-compatible, **zero egress fees** (~$15/TB); best for high-traffic public media | live [verified: web 2026-06-22; price volatile] | build-on (blob backend) |
| iDrive / Hetzner / Wasabi | e2 / Object Storage / Hot Cloud | Low-cost S3-compatible backends (iDrive e2 ~$4/TB; Hetzner ~$5.99/TB; Wasabi ~$6.99→7.99/TB flat, **90-day min-retention trap**) | live [verified: web 2026-06-22; prices volatile] | build-on (blob backend) |
| AWS / Azure / GCP | Glacier Deep Archive / Blob Archive / Archive | Deep-cold tiers (~$1/TB) for rarely-touched data; steep retrieval + egress penalties — *not* for active blobs | live [verified: web 2026-06-22; prices volatile] | learn↔ (cold-tier economics; the retrieval-penalty trap) |

**Two corrections worth keeping visible:** MinIO (often cited as the self-host S3 backend) had its
community-edition repo **archived Feb 2026** — Garage/SeaweedFS are the maintained alternatives. And
atproto **decouples identity from host** (CAR repo export/import → migrate PDS without losing
followers), which is the structural backstop that makes "no data hostage" real for any of the above.

## 5f. AT Proto AppView / client / bridge tooling (from the 2026-06-22 sovereign-AppView dialogue)

Surfaced and web-verified 2026-06-22 (see
`seeds/transcripts/raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22-FACTCHECK.md`;
analysis in `research/atproto-sovereign-appview-club.md`). Relevant because the **AppView-as-private-
gatekeeper** pattern is a buildable shape of Croft's blind-broker / honest-seams-ponds stance (own the
read/index layer → private blocking, off-repo feeds, multi-source ingestion). All rows real and correctly
attributed (Gemini was unusually accurate here).

| Org/Author | Project | Purpose / relevance | Current state | Relationship |
|---|---|---|---|---|
| A New Social (Ryan Barrett / "snarfed" + ~2) | **Bridgy Fed** | The main **bi-directional AP↔AT bridge** (Mastodon/Threads ↔ Bluesky); `@bsky.brid.gy` / `@ap.brid.gy` routing | live; `snarfed/bridgy-fed`, **CC0**, Python (Granary + Arroba); ~150k users; cost re-architected $0.15→$0.03/user/mo [verified: web 2026-06-22] | learn↔ (the credible-exit / "people not platforms" cousin), partner |
| A New Social | **Bounce** | Cross-protocol account **migration / "credible exit"** (move into a native PDS without losing bridged followers) | live [verified: web 2026-06-22] | learn↔ (no-data-hostage instrument) |
| — | RSS Parrot · Pinhole · Fedisky | Independent narrower bridges (RSS/Atom→Fediverse; Bluesky→AP one-way; AP-extension for Bluesky PDS) | live/niche [verified: web 2026-06-22] | learn↔ |
| grjte | **Groundmist** | **Private, local-first ATProto layer** — a "Personal Sync Server (PSS)" analogous to a PDS but private; Automerge CRDT sync; WhiteWind-lexicon publish | prototype (sync server ships with **auth disabled** — "private by default" is intent, not yet security) [verified: web 2026-06-22] | learn↔, build-on (closest local-first-private relative to Croft's model) |
| alnkesq | **AppViewLite** | Lean **AppView** (C#), ~2.2 GB disk/day to index the firehose; Jetstream/Relay; memory-mapped columnar | live [verified: web 2026-06-22] | build-on (lean self-host AppView; inject private-filter logic) |
| Blacksky | **rsky-wintermute** AppView | TS bsky appview + **Rust** indexer; ~10k records/sec; **"Private Community Posts" scaffolding on the AppView layer** | live [verified: web 2026-06-22] | build-on (closest to Croft's stack; the private-community reference) |
| zeppelin-social | **bluesky-appview** | Docker-Compose **full reference AppView stack** (bsky, indexer, labelmuncher, palomar search, postgres/redis/opensearch) | live [verified: web 2026-06-22] | build-on (reference clone + custom proxy middleware) |
| colibri.social | **Colibri** | Discord-like community chat on ATProto; sparked the public-by-default-history debate; `verdverm` building permissioned-DB prototypes | live/experimental [verified: web 2026-06-22] | learn↔ (the permissioned-data debate) |
| — | **Jetstream** | Official tool converting the binary firehose → compressed filterable JSON (the lightweight ingestion path) | live [verified: web 2026-06-22] | build-on (ingestion) |
| pdelfan / tunjid / mary-ext / Bluesky | **Ouranos · Heron · atcute · social-app** | Client bases to extend: Ouranos (Next.js web), **Heron** (offline-first Kotlin Multiplatform + Compose/Room — mesh sandbox), atcute (from-scratch TS protocol utils), social-app (official) | live [verified: web 2026-06-22] | build-on (client upstreams) |
| bluesky-social / pdsmoover | **goat** · **PDS MOOver** | Account-migration tooling (CAR export/import + signed PLC ops); returning-user-only inbound to bsky.social | live [verified: web 2026-06-22] | build-on (migration) |
| #IndieSky · Eurosky (Modal Foundation) · Free Our Feeds | independent-infra movements | Non-corporate Relays / private-group / localized-moderation infra for the open atproto web | active [verified: web 2026-06-22 — NB "AT Community Fund" is an imprecise name; the real funder is **Free Our Feeds**] | homage, learn↔, partner |

## 6. P2P / decentralized messengers (the field)

Detailed competitive analysis lives in `research/messaging-solutions-landscape.md`. Relational summary:

| Org | Project | Purpose | Current state | Relationship |
|---|---|---|---|---|
| — | Secure Scuttlebutt / Manyverse | Pure-P2P local-first social | declined; Manyverse lead stepped away Apr 2024; fusion-identity & partial-replication specs archived [verified: research] | homage, learn↔ (the canonical cautionary tale) |
| Merlin / community | Delta Chat | E2EE over email + iroh | Rust core, chatmail, iroh realtime + Add-Second-Device; multi-device = transfer-then-diverge [verified: research] | homage, learn↔ (closest Rust+iroh cousin), partner |
| Briar Project | Briar | Tor-based P2P, high-risk threat model | no multi-device, no recovery (by design); Mailbox async relay [verified: research] | homage, learn↔ |
| Session / OPTF→Swiss foundation | Session (Oxen) | No-phone decentralized messaging | Protocol V2 (Dec 2025) re-added PFS + ML-KEM; mnemonic recovery [verified: research] | homage, learn↔ |
| Open Privacy Research Society | Cwtch | Metadata-resistant group chat over Tor onion services | needs an (untrusted) server/host node to anchor group state — if the host drops, the group goes dark (the "unequal peer" made explicit) [verified: web 2026-06-22] | learn↔ |
| SimpleX Chat Ltd | SimpleX Chat | No-identifier messaging (no user IDs / phone #s) | unidirectional message queues on relay servers (server-mediated, not pure P2P); QR-code contact setup; closest "honest hybrid" attempt [verified: web 2026-06-22] | learn↔ (the no-persistent-identifier + honest-server-mediation lesson) |
| Holepunch (Tether-backed, El Salvador) | Keet / Pear / Hypercore | Mass-market P2P calls & file transfer; `Pear` P2P dev platform on the `Bare` runtime | Keet uses **Hypercore** (append-only feeds); fast, no file-size limits, **no PFS**, **needs an active internet DHT** so it's not air-gap-capable [verified: web 2026-06-22] | learn↔ (the perf-first camp; the "DHT dependency" limit) |
| Berty / Weshnet (French non-profit) | Berty / **Wesh** protocol | Mobile-first pure-P2P w/ offline BLE mesh | Wesh on libp2p+IPFS, Go core + React Native UI; the "adaptive online/offline" attempt that **stalled** under the Go-daemon/RN-bridge weight + Apple watchdog churn [verified: web 2026-06-22] | learn↔ (cautionary: ideological purity → too heavy for consumer phones) |
| Matrix.org Foundation | Matrix (Olm / Megolm) | Federated E2EE messaging | Olm (1:1, Double-Ratchet) + Megolm (group, shared ratchet → weaker PCS); DAG room state forks badly in a split mesh; Rust crypto lib = **Vodozemac** (note: "Voskop" is a Gemini fabrication) [verified: web 2026-06-22]. **Governance/UX findings (2026-06-24 dialogue, the close-cousin cautionary reference for Drystone §X — web-verified in-session, pending our SoT confirmation):** State Resolution v2 over a finality-free consensus-free partial order → **state resets** (**CVE-2025-49090**, fixed v2.1; Karlsruhe SACMAT-2020 analysis); **room v12 / "Project Hydra" / MSC4289** gives creators infinite-immutable power + `additional_creators` (the unconflictable-root lesson); a multi-week **matrix.org Postgres state-group-corruption outage** (2025-07 postmortem); **Megolm/UTD** key-custody gap ("Unable to decrypt"); **encryption is a one-way latch** (can't be disabled once on — defeats MITM downgrade); **encrypted-room search is Seshat desktop-only** (web/mobile can't); **Element X vs Element Classic** three-codebase split (Spaces/Threads parity only late-2025). | homage, learn↔ (the close cousin: shared eventual-consistency ancestry, opposite choices) |
| Defense Unicorns | Peat | (off-grid Rust+iroh+CRDT+MLS data-sync — see §1) | the protocol-toolkit answer to the consumer-app graveyard: ship the substrate, not a "P2P WhatsApp" | build-on, learn↔ |
| — | XMTP / Keybase teams | web3 messaging / team key management | per-device-as-member prior art (Keybase) [UNVERIFIED current] | learn↔ |
| Germ Network | Germ DM | MLS E2EE messenger on atproto identity; launches from Bluesky profiles | **First native-launched private messenger from a Bluesky profile (2026-02-18)**; iOS; MLS, multi-identity ("cards"/burner cards), no-phone; cofounder/CTO **Mark Xue** (ex-Apple iMessage/FaceTime). Open-source **Autonomous Communicator (AC) Protocol** on MLS (MIT); IETF **`draft-xue-distributed-mls`** (IETF 124, "TwoMLS", Naval Postgraduate School); Protocol Labs **Cypherpunk Fellowship**; identity bound via an **"Anchor Key" published in the atproto profile**; external **mailbox services**; Germ Inc. runs routing (no self-host server yet). [verified: web 2026-06-22 — Gemini drift: `ger.mx`, `/android-waitlist`, draft name "distributed-mls-id" all wrong/unverified; see `croft-atproto-pds-germ-privatedata-dialogue-2026-06-22-FACTCHECK.md`] | homage, learn↔ (closest atproto+MLS cousin), partner |
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
| C2PA (Adobe/Arm/BBC/Intel/Microsoft/Truepic et al.) | Content Credentials — cryptographic media-provenance standard (is-this-synthetic at the asset layer) | learn↔ (media-provenance complement to Croft's authorship-provenance; relevant only if Croft renders external media) [verified: web 2026-06-22] |
| Atlantic Council DFRLab | "Scaling Trust on the Web" (Task Force for a Trustworthy Future Web, Jun 2023) — T&S as public infrastructure; **middleware** (user-chosen moderation); C2PA provenance | homage, learn↔ (the "middleware" rec rhymes w/ atproto composable labelers + Croft's moderation-as-a-chosen-lane) [verified: date/publisher; exact recs UNVERIFIED] |

## 8. Cooperative / governance prior art

The co-op vertical's lineage (detail in the dossier §3, §8):

| Example | What it proves | Relationship |
|---|---|---|
| Elinor Ostrom's commons work | Communities sustain shared resources for centuries (Törbel, Valencia, Bali Subak, Maine lobster); polycentric governance + subsidiarity = the scale answer | homage (governance DNA) |
| Crofting / Crofters' Holdings Act 1886 | Secure tenure + common grazing = the rights-floor + commons made literal; the form that survived the Highland Clearances (the monoculture-by-optimization disaster) | homage (the name is the thesis) |
| Commons-DAO research (De Filippi, Rozas et al.) | "DAO design for the commons" (Frontiers in Blockchain, 2023): forking as pressure on the powerful; "none is essential"; Ostrom-grounded — vs. the code-is-law / auto-executed-legitimacy mainstream Croft rejects | homage, learn↔ [dialogue-sourced, verified Frontiers DOI 10.3389/fbloc.2023.1287249] |
| Liquid Feedback / liquid democracy (German Pirate Party; Google Votes) | Instantly-revocable per-topic delegation = "cheap exit at scale"; real-world failure is delegation *concentration* (super-delegates) — the antidotes are decay/caps/bounded-chains/expiry/visibility | learn↔ [dialogue-sourced 2026-06-20] (D9 governance-at-scale) |
| Green Bay Packers | Only community-owned major US sports team; the model was banned after | homage |
| Mondragon | Worker-owned federation at scale | homage |
| Credit-union lineage | Schulze-Delitzsch → Raiffeisen → Desjardins → Filene; "not for profit, not for charity, but for service" | homage (institutional model) |
| Informal Systems (Ethan Buchman), Subvert, Patio.coop, USFWC | Modern tech-worker co-op references / structures (LCA + PBC) | partner, learn↔ |
| Platform Cooperativism Consortium (Trebor Scholz, The New School, 2018) | The movement + research hub for user/worker-owned platforms; Scholz's 2014 essay "Platform Cooperativism vs. the Sharing Economy" is the founding text | homage, learn↔ [verified 2026-06-22] |
| Stocksy United (Colorado LCA) | The gold-standard platform co-op: ex-Getty/iStock founders, multi-class membership (founder/staff/artist), profitable, quality-density over scale — the model for progressive-decentralization + multi-class | homage, learn↔ [verified] |
| The Drivers Cooperative (NYC) | Driver-owned ride-share; drivers keep ~85% (co-op takes ~15%); proves you can out-margin a VC monopoly by paying contributors | homage [verified] |
| Resonate | Music-streaming co-op; "stream-to-own" micro-payments convert to ownership/governance — a non-token internal-economy precedent | homage, learn↔ [verified] |
| Social.coop | Member-owned, member-governed Mastodon instance funded by Open Collective dues — the closest living relative to the "Social Union" (single-protocol, no progressive-decentralization sunset) | homage, learn↔ [verified] |
| The Purpose Foundation / steward-ownership; Haferkater; Start.coop | Revenue-Based Financing / "Demand Dividend" capped-return investor models (financial rights, no governance, stake vanishes at cap) — the seed-capital mechanism | learn↔ [verified] |
| 501(c)(3) tech fiscal sponsors — SPI, Software Freedom Conservancy, Code for Science & Society | The grant-funded-open-source-labor path for the nonprofit half of a co-op/501(c)(3) tandem (host Debian/Arch/PostgreSQL; Git/Inkscape; public-interest tech). NB Open Collective *Foundation* dissolved end-2024; Open Source Collective is a 501(c)(6) | partner, learn↔ [verified] |
| **Software in the Public Interest (SPI)** — the trademark-stewardship proof-of-concept for Croft's exact plan | SPI *owns* the Debian trademark (registered US mark), *managed by* the Debian project, and has licensed it to outside orgs; also holds ArduPilot's mark. The asset-holding-shell / community-runs-it model = exactly Croft's foundation-holds-marks/coop-operates intent. Best long-term asset-holding match. See `thinking/foundation-and-ip-stewardship.md` | build-on, learn↔ [dialogue-sourced 2026-06-23] |
| **Software Freedom Conservancy** — permanent neutral mark-holder (FOSS-purist) | 501(c)(3); can hold copyrights, trademarks, domains + provides trademark registration/policy/licensing/enforcement (word-for-word Croft's plan), plus GPL-enforcement legal muscle (Vizio). Strict OSI/DFSG bar, slow rolling review, "not a service provider". The Phase-2+ permanent home if Croft becomes a major FOSS effort | learn↔, partner [dialogue-sourced 2026-06-23] |
| **Aspiration** — Croft's recommended **interim foundation** (Phase 1) | 501(c)(3) fiscal sponsor; short-term (1–2 yr) **grantor/grantee** model → project keeps its own board + IP (friendly to holding your own marks + graduating to your own 501(c)(3)). Sponsors "mission-aligned technology, data, and digital-rights projects, focusing on open/equitable community, governance, and IP" — near-exact fit; fees 5–15%; $150 + 30-min interview. Open item: verify it sponsors a *coop-operated* project. Projects incl. Coko, Tahoe-LAFS, Open Referral | partner, learn↔ [dialogue-sourced 2026-06-23] |
| Anti-patterns (learn↔, what-not-to-do) — Ello (VC capture), Ampled (volunteer burnout, shut 2023), Steemit (hyper-financialization), Diaspora (architectural naïveté), Coomappa (white-label dependency) | The verified failure lineage the cooperative model is designed to answer — see `thinking/cooperative-social-union-model.md`, COHESION §33 | learn↔ [verified 2026-06-22] |

(Most rows above from the 2026-06-22 cooperative "Social Union" dialogue; fact-checked — see
`seeds/transcripts/raw/cooperative-social-union-governance-dialogue-2026-06-22-FACTCHECK.md`. The
SPI/SFC/Aspiration rows are enriched from the 2026-06-23 foundation/IP dialogue —
`seeds/transcripts/raw/croft-foundation-coop-ip-naming-dialogue-2026-06-23.md`, **dialogue-sourced,
verify before reliance** — see COHESION §35.)

## 9. Publication, preservation & prior-art infrastructure

The vehicles for putting the Drystone spec out as a defensive publication (the *external* IP twin of §8's
internal stewardship; detail in `thinking/drystone-publication-and-defensive-disclosure.md`). **All rows
dialogue/web-sourced 2026-06-24, NOT independently re-verified — verify before reliance.**

| Org | Project | Purpose | Capabilities | Relationship |
|---|---|---|---|---|
| CERN | **Zenodo** | Free open-access repository that mints permanent **DOIs** | third-party (CERN) custody + timestamp; Concept-DOI vs version-DOI; mandatory per-upload license; GitHub-release auto-archive (`.zenodo.json`); reserve-DOI-before-publish; ~30-day post-publish file-edit window then new version; 50GB/100-file caps | **build-on — the chosen priority/prior-art vehicle** for the Drystone spec (CERN-witnessed beats a fakeable local repo; better fit than gatekept arXiv). [dialogue-sourced 2026-06-24] |
| OpenTimestamps | **OpenTimestamps** | Cryptographic existence-by-date proof | hashes a document, anchors the hash into the Bitcoin blockchain; proves existence-before-a-block (nothing about authorship/content); free | **build-on — pairs with Zenodo** as the strongest "this exact text existed by this date" anchor. [dialogue-sourced 2026-06-24] |
| IETF / IETF Trust | Internet-Draft process | Standards-track spec input format | draft→RFC lifecycle; rough-consensus + 2-interop-impls culture; **drafts are IETF-Trust-encumbered** (reuse-restricted, "cite only as work in progress") | learn↔ / **later destination, not first** — wrong first venue and *more* encumbered than a self-licensed repo for timestamping. [dialogue-sourced 2026-06-24] |
| Malleable Systems | Malleable Systems community / forum | User-controlled, independent digital-spaces movement | low-rigor, philosophy-tolerant feedback venue (Willow was discussed there on release) | learn↔ — soft early-shaping venue for the design-principles layer. [dialogue-sourced 2026-06-24] |

(iroh GitHub Discussions/Discord, the Willow maintainers, and the local-first community are the primary
*feedback* venues — see `thinking/drystone-publication-and-defensive-disclosure.md` §3; iroh/Willow already
registered in §1/§3. This refines beta `07` Pillar C's prior-art-vehicle posture — COHESION §38.)

---

## Refresh discipline

Volatile facts (versions, ship dates, org changes) drift. Before any external use, run a
verification pass on every `[UNVERIFIED]` row and re-confirm the `[verified]` ones older than
a few months. The provenance-debt caveat from the dossier applies here too: framings are
sound; specific numbers and dates need primary-source confirmation.
