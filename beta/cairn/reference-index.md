# Projects register: the ecosystem Croft builds among

`Status: cairn layer (Layer 3, the open field). Register: the projects directory for the whole cairn layer.
Resolution: index — one row per ecosystem project, protocol, spec, or library named across the cairn docs,
with the relationship Croft holds to it, its status, a resolvable locator, a verification marker, and which
sibling cairn doc(s) cover it. This is the directory; the reasoning for each relationship lives in the doc
named under "Covered in." External facts carry verification markers; volatile version/price/maturity facts
are marked "refresh before external use," and iroh version facts cite the FACTCHECK source of truth and are
not re-verified here.`

Unlike a bibliography, this register is a *directory of the field*: what each thing is, and how Croft stands
toward it. Every project appears exactly once in its primary role; where a project spans roles, its other
homes are named in the "Covered in" column. The register is the map; the docs are the territory.

## Relationship-tag key

- **build-on** — Croft depends on it, reuses it directly, or forks/wraps it as a component.
- **homage** — credited as prior art or a design influence; not a live dependency (includes options considered and set aside).
- **partner** — a movement or organization Croft could align or collaborate with.
- **rebroadcast** — a bridge/dual-write pattern Croft mirrors when it wants both reach and a durable native record.
- **learn↔** — studied as a near neighbor or sibling; a two-way exchange of lessons, including cautionary and negative examples.
- **reject** — a named pattern or mechanism Croft deliberately declines (the reason travels with the rejection).

A single project may carry more than one tag (e.g. *build-on, learn↔*).

## Verification-marker key

- **verified** — confirmed against a primary source (RFC, spec, repo, or the FACTCHECK source of truth for iroh).
- **web-verified DATE** — web-confirmed on the given date; treat as a snapshot and refresh volatile facts before external use.
- **dialogue-sourced** — surfaced in the Croft design dialogue; not yet independently verified.
- **[UNVERIFIED]** — explicitly flagged unverified; needs an independent verification pass before external use or any build decision.
- **[confirm]** — a specific claim the source flagged to re-check against the primary before publish.

"Refresh before external use" is appended wherever a version, price, maturity, or adoption fact is known to drift.

---

## Transport & substrate

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| iroh (n0) | QUIC-first peer-to-peer transport; EndpointId = Ed25519 pubkey, hole-punching, relays, multipath migration | build-on (core dependency) | iroh 1.0; companion crates (iroh-blobs, iroh-gossip, iroh-roq) still pre-1.0 | github.com/n0-computer/iroh | verified — iroh version per FACTCHECK source of truth; refresh before external use | substrate-prior-art; iroh-app-pond-building-blocks |
| Peat / peat-gateway (Defense Unicorns) | Off-grid/denied-environment Rust P2P data-sync middleware assembling iroh + Automerge + MLS; integrates ATAK, resyncs to Okta/Keycloak | build-on, learn↔ | active open source; in production defense/disaster-response/industrial | github.com/defenseunicorns/peat | dialogue-sourced — `[confirm]` against repo before external use | substrate-prior-art |
| libp2p | Modular P2P stack (transports, pubsub, DHT) | homage (set aside as primary — mobile-weak vs iroh) | mature | libp2p.io | verified | substrate-prior-art |
| Veilid | Privacy-first P2P with source-address-free routing; small-record DHT | learn↔ (future metadata-resistant messaging candidate) | live | veilid.com | verified | substrate-prior-art |
| Holochain | Agent-centric P2P, no global consensus; adopting iroh via Kitsune2 | homage (dropped as substrate — runs on iroh anyway, mobile-weak) | mature | holochain.org | verified | substrate-prior-art |
| p2panda | Building blocks for P2P apps; modular CRDT-agnostic crates reusing iroh; p2panda-spaces access control | learn↔ | pre-1.0 (mid-2026) | p2panda.org | dialogue-sourced (substrate); landscape-grounded (adjacent-systems) | substrate-prior-art; adjacent-systems; willow-meadowcap |
| iroh-rings | Relationship-based access control for resources over iroh | learn↔ (neighbor to the capability layer) | — | repo as cited | dialogue-sourced 2026-06-24 | substrate-prior-art |
| RINA (Recursive InterNetwork Architecture, John Day) | Thesis that networking is one recursive layer repeated at scale; bounds routing state by recursion | learn↔, build-on (routing lineage) | research/academic | — | dialogue-sourced 2026-06-20 | substrate-prior-art |
| Named Data Networking (NDN) | Routes on aggregatable hierarchical names | learn↔ (routing lineage) | research | named-data.net | dialogue-sourced 2026-06-20 | substrate-prior-art |
| Yggdrasil | Routes over cryptographic-identity trees, no global table, locality-aware; working small-scale network | learn↔, build-on (federation-routing PoC shape) | working network | yggdrasil-network.github.io | dialogue-sourced 2026-06-20 | substrate-prior-art |
| cjdns | Crypto-identity-tree routing with no global table | learn↔ (routing lineage) | mature | github.com/cjdelisle/cjdns | dialogue-sourced 2026-06-20 | substrate-prior-art |

## Group encryption & MLS

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| MLS (RFC 9420) | Group end-to-end encryption; FS + PCS; TreeSync/TreeKEM/TreeDEM decomposition | build-on (subordinate key-distribution backplane) | finished RFC | RFC 9420 | verified | mls-and-mimi; atproto-ecosystem; object-capability |
| MLS Architecture (RFC 9750) | The MLS architecture doc; Delivery Service model, long-term-key protection | build-on | finished RFC | RFC 9750 | verified | mls-and-mimi |
| MIMI (More Instant Messaging Interoperability) | IETF interop standard for MLS identity/transport/addressing; reintroduces a per-room hub | learn↔; reject (the per-room hub) | Internet-Drafts (draft-ietf-mimi-protocol-05, Oct 2025) | datatracker.ietf.org/wg/mimi | verified (draft state) — refresh draft number before external use | mls-and-mimi; adjacent-systems |
| OpenMLS | Rust MLS implementation; basis for the Soler measurement study and DMLS PoC | build-on / reference | active | github.com/openmls/openmls | verified | mls-and-mimi; object-capability |
| AWS mls-rs | Rust, RFC 9420-conformant MLS (no full third-party audit noted) | build-on (candidate) | active | github.com/awslabs/mls-rs | verified — `[confirm]` audit status | mls-and-mimi |
| DMLS / FREEK (Phoenix R&D; Alwen, Mularczyk, Tselekounis) | Decentralized MLS processing out-of-order commits; FREEK (Fork-Resilient CGKA via puncturable PRF) recovers most FS; ~8 kB/PPRF-eval cost | learn↔ (nearest serverless sibling; prices the fork→FS cost) | IETF draft + PoC OpenMLS fork; no production (mid-2026) | IETF draft; FREEK paper | `[confirm before publish]` (web 2026-06-26) | object-capability |
| draft-xue-distributed-mls ("TwoMLS", Naval Postgraduate School) | Serverless MLS giving each member its own "Send Group"; PCS + FS without global ordering | learn↔ (serverless sibling) | IETF draft (presented IETF 124) | datatracker.ietf.org (draft-xue-distributed-mls) | `[confirm before publish]` | object-capability |
| MLS production deployments (adoption signal) | Every shipping MLS deployment is server-ordered: Webex, Wire, Discord "DAVE", Cisco/RingCentral (OpenMLS), GSMA RCS Universal Profile 3.0 (Apple + Google, 2026) | learn↔ (empirical anchor: no production MLS is serverless-ordered) | shipping | vendor sites | verified — Discord "DAVE" `[confirm]`; refresh before external use | mls-and-mimi; object-capability |

## CRDT & local-first

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| Automerge (Ink & Switch) | CRDT; multi-writer-merge-with-history primitive; the CRDT Peat and Groundmist ship | build-on | mature; 3.x line | automerge.org | verified — 3.x memory-reduction claim is version-volatile, refresh before external use | substrate-prior-art; atproto-selfhosting-appviews-and-bridges; atproto-ecosystem |
| "Local-first software" thesis (Kleppmann et al., Ink & Switch, 2019) | The seven ideals; single-user value before network effect, primary copy with the unit | homage, build-on (intellectual root) | published paper | inkandswitch.com/local-first | verified | substrate-prior-art |
| Willow | State-based CRDT (join-semilattice); range-based set reconciliation, content-hash addressing, Entry/subspace model | build-on (Croft is "Willow-shaped"); learn↔ | Data Model + Meadowcap Final; Confidential Sync / Drop Format / Willow'25 took breaking changes into 2026; impls pre-1.0 | willowprotocol.org | verified (survey snapshot) — treat readiness as a snapshot | willow-meadowcap; substrate-prior-art |
| Earthstar | A Willow implementation (11 beta) | learn↔ | beta | earthstar-project.org | verified (snapshot) | willow-meadowcap |
| Loro | CRDT; one of the candidates Roomy builds communities on (Loro or Automerge per differing reports) | learn↔ | active | loro.dev | `[confirm]` which CRDT Roomy uses | atproto-ecosystem |
| Jazz | Local-first framework; one of the three P2P stacks Roomy tried and dropped | learn↔ (cautionary — not-ready-as-a-dependency) | active | jazz.tools | landscape-grounded | adjacent-systems |

## Identity, trust & capabilities

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| Meadowcap | Willow's capability system; unforgeable grants with attenuation-by-subsetting | build-on / learn↔ (Track A capability candidate) | Final | willowprotocol.org/specs/meadowcap | verified | willow-meadowcap; object-capability |
| Keyhive | Capability system; Track B capability candidate; one of the stacks Roomy tried | learn↔ (capability-mechanism decision A11) | pre-1.0 | repo as cited | dialogue-sourced 2026-06-24 | object-capability; adjacent-systems |
| Solid (WebID / Solid-OIDC / DPoP) | Private-by-default personal Pod store; apps read/write directly; per-file ACLs | homage, learn↔ (the private-store pole) | W3C-standards-based; live | solidproject.org | web-verified 2026-06-22 — Inrupt commercial arrangement volatile, refresh before external use | identity-and-data-ownership-poles |
| DPoP (RFC 9449) | Demonstrating Proof-of-Possession; binds tokens to the client so a stolen bearer token cannot be replayed | build-on / homage (Solid's token binding) | finished RFC | RFC 9449 | verified | identity-and-data-ownership-poles |
| DSNP + Frequency (Project Liberty) | Social-graph-as-public-utility; keypair identity, portable on-chain graph, no core token, delegation without master keys; Frequency is the reference Polkadot parachain | homage, learn↔; reject (the chain) | live; published whitepaper | dsnp.org; frequency.xyz | web-verified 2026-06-22 — parachain economics volatile; `[confirm]` "no core token" against current spec | identity-and-data-ownership-poles |
| did:webvh (formerly did:tdw) | Portable root-anchor DID method; SCID over an append-only key-history log, `nextKeyHashes` pre-rotation, genesis-only `portable:true` | build-on (the hub in hub-and-spoke) | spec + implementations | did:webvh spec (identity.foundation) | dialogue-sourced 2026-06-20 | cross-platform-identity-provenance |
| did:plc / plc.directory | atproto's transparency-log DID method (self-certifying ops; 12M+ operations); a log, not a CA | build-on (the atproto spoke) | operated by a single party; nonprofit handoff planned, not done | plc.directory | dialogue-sourced; native-method-set per FACTCHECK | cross-platform-identity-provenance |
| did:web | One of atproto's two blessed DID methods | build-on | standard | w3c-ccg.github.io/did-method-web | verified — per FACTCHECK for atproto method set | cross-platform-identity-provenance; atproto-nsid-and-lexicon-mechanics |
| didwebvh-rs / didtoolbox | Rust implementations/validators for did:webvh (log-chain validation, SCID continuity, pre-rotation) | build-on (makes the hub buildable in Rust) | — | repos as cited | dialogue-sourced | cross-platform-identity-provenance |
| DIDComm Mediator Coordination / Pickup | Hold-and-forward messaging for offline DID controllers | learn↔ (offline-principal delegate prior art) | spec | identity.foundation (DIDComm) | dialogue-sourced 2026-06-20 | cross-platform-identity-provenance |
| Certificate Transparency (RFC 6962) + CT gossip | Equivocation-*detection* lineage: make an inconsistent log entry detectable, no trusted center | learn↔ (keep a single-operator directory honest) | published standard | RFC 6962 | verified | cross-platform-identity-provenance |
| CONIKS | End-user key directory where each user monitors their own binding so a directory cannot substitute a key unnoticed | learn↔ | published (USENIX Security 2015) | coniks.cs.princeton.edu | verified (paper) | cross-platform-identity-provenance |
| Spritely Institute — Goblins / OCapN / CapTP | Object-capability distributed programming; "designation is authorization," POLA, petnames; 501(c)(3), NLnet/NGI-funded, no VC/token | homage, learn↔ (formal frame for capability-not-authority; funding model as sustainability prior art) | active | spritely.institute | `[UNVERIFIED current]` — refresh funding/project facts before external use | object-capability |
| Petname systems (Stiegler) | Locally-assigned human-readable names each party binds for itself; readable naming without a global registry | homage (naming without a central allocator) | published essay | "An Introduction to Petname Systems" | verified (concept) | object-capability |

## Social protocols & federation

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| atproto (AT Protocol) | Public-by-default federated social protocol; DID + handle identity, per-author PDS repos, public firehose, lexicon-typed records | build-on, complement-not-competitor | mature, fast-moving | atproto.com | verified (snapshot) — refresh before external use | atproto-ecosystem; atproto-selfhosting-appviews-and-bridges; atproto-nsid-and-lexicon-mechanics; atproto-content-portability-and-backdating; atmospheric-web-and-aggregators |
| ActivityPub | W3C federation protocol; standard HTTP POST, no per-activity fee | build-on / learn↔ (bridge target; the no-gas finding) | W3C Recommendation (2018) | w3.org/TR/activitypub | verified | atmospheric-web-and-aggregators; atproto-selfhosting-appviews-and-bridges |
| Nostr | Capture-resistant, credible-exit protocol; public-by-default with bolted-on, leaky private messaging | learn↔ (picks capture-resistance, pays on privacy) | mature | nostr.com | landscape-grounded | adjacent-systems; atmospheric-web-and-aggregators |
| Farcaster | Web3 social protocol; carries storage rent (~$7/unit) | learn↔ (the rent/gas cost contrast) | mature | farcaster.xyz | web-verified 2026-06-22 — rent figure volatile, refresh before external use | atmospheric-web-and-aggregators |
| Lens | Web3 social protocol carrying rent/gas | learn↔ (rent contrast) | mature | lens.xyz | web-verified 2026-06-22 | atmospheric-web-and-aggregators |

## atproto self-hosting, AppViews & bridges

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| Blacksky (Rudy Fraser / Black Sky Algorithms) | Black-community atproto infrastructure: own relay, PDS, AppView, moderation, feeds, client; People's Assembly on Polis; ~$6.5k/mo subscription-funded; 0→2M users; full-network index | learn↔, build-on (best evidence community-governed atproto infra scales) | active; full-network AppView | blacksky.app; github.com/blacksky-algorithms | verified (transcript-grounded) — throughput/scale figures point-in-time | blacksky-and-atproto-community; atproto-selfhosting-appviews-and-bridges; atproto-ecosystem |
| rsky-wintermute (Blacksky) | Rust indexer + Private Community Posts AppView scaffolding (~10k records/sec target) | build-on, learn↔ (closest stack to Croft's private-index layer) | active | github.com/blacksky-algorithms/rsky | web-verified 2026-06-22 — throughput point-in-time | blacksky-and-atproto-community; atproto-selfhosting-appviews-and-bridges |
| rsky-pds (Blacksky) | Alternative PDS in Rust (Postgres, S3 blobs, Mailgun) | build-on, learn↔ (nearest Rust-PDS reference) | active | github.com/blacksky-algorithms/rsky | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Community Posts (`community.blacksky.feed.*`) | Private, membership-gated posts resident at the AppView, not the PDS — a deliberate, disclosed source-of-truth inversion | learn↔ (disclosure discipline for inverting an invariant) | shipping | (Blacksky lexicon) | verified (transcript-grounded) | blacksky-and-atproto-community |
| Groundmist (grjte, Ink & Switch) | Private, local-first atproto layer — a Personal Sync Server using Automerge sync, publishing via the WhiteWind lexicon | learn↔ (closest local-first-private cousin) | prototype; ships with auth disabled (intent, not yet security) | grjte / Groundmist | web-verified 2026-06-22 — prototype + auth-disabled caveat point-in-time | atproto-selfhosting-appviews-and-bridges |
| @atproto/pds (official reference PDS) | TypeScript, single-tenant SQLite reference Personal Data Server | build-on / homage (the reference baseline) | reference | github.com/bluesky-social/atproto | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Cocoon (haileyok) | Alternative PDS in Go with a PostgreSQL backend | learn↔ (the Postgres-PDS path) | highly experimental, not production-ready | github.com/haileyok/cocoon | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| AppViewLite (alnkesq) | Lean AppView in C#; ~2.2 GB/day to index the firehose; memory-mapped columnar store | learn↔ (lean self-host AppView for private-filter injection) | active | github.com/alnkesq/AppViewLite | web-verified 2026-06-22 — disk-per-day figure volatile | atproto-selfhosting-appviews-and-bridges |
| zeppelin bluesky-appview (zeppelin-social) | Docker-Compose full reference AppView stack (bsky appview, indexer, label ingest, search, Postgres/Redis/OpenSearch) | build-on (reference clone to fork + wrap) | live reference stack | github.com/zeppelin-social | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Jetstream (official) | Converts atproto's binary firehose into compressed, filterable JSON | build-on (the ingestion on-ramp) | official | github.com/bluesky-social/jetstream | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Tap (official) | Repo-sync/backfill tool: subscribe to a Relay and auto-backfill via `getRepo` (SQLite/Postgres) | build-on (the sync/backfill reference) | official | (bluesky-social) | web-verified 2026-06-22 | atmospheric-web-and-aggregators |
| Peergos | Deployed E2EE PDS; private blockstore of access-controlled CHAMP writing-spaces; offered to standardize its PDS protocol for atproto | learn↔ (the one deployed E2EE-everything design) | deployed; standardization sought | peergos.org | verified/press (discussions #3363) | atproto-ecosystem |
| Bluesky Permissioned Data | Published proposal for non-public data; distinct protocol; explicitly access control, not confidentiality; "spaces" + space authority | learn↔ (the durable-model-without-confidentiality answer) | proposal | bluesky-social/proposals PR #94 | verified (proposal) | atproto-ecosystem |
| The Arbiter (Muni Town) | Per-community group-membership XRPC service: root DID, cumulative access levels, recursive space-in-space delegation, owner-authority | learn↔ (closest existing governance layer; owner-authority, not peer-symmetric) | alpha/directional | Muni Town | press/proposal | atproto-ecosystem; adjacent-systems |
| Northsky, Habitat | Teams building non-public/permissioned-data atproto extensions alongside Blacksky | learn↔ (parallel permissioned-data efforts) | active | — | verified (named by Bluesky) | atproto-ecosystem; blacksky-and-atproto-community |
| Bridgy Fed (A New Social; snarfed) | Bi-directional ActivityPub↔atproto bridge (Granary + Arroba); CC0 | build-on, rebroadcast (credible-exit machinery; CC0 makes it reusable) | live, carrying real users | github.com/snarfed/bridgy-fed | web-verified 2026-06-22 — per-user cost figures point-in-time | atproto-selfhosting-appviews-and-bridges; atmospheric-web-and-aggregators; atproto-content-portability-and-backdating |
| Bounce (A New Social) | Cross-protocol account migration framed as credible exit (moves the graph, not posts) | learn↔ (no-data-hostage instrument) | live | A New Social (anew.social) | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges; atproto-content-portability-and-backdating |
| PDS MOOver (`pdsmoover`) / goat | CAR export/import + signed PLC operations for account migration; goat is the Go atproto CLI demonstrating the did:plc op flow | learn↔, build-on (migration + PLC-op tooling) | active | (goat: bluesky-social) | web-verified 2026-06-22; goat dialogue-sourced | atproto-selfhosting-appviews-and-bridges; cross-platform-identity-provenance; atproto-content-portability-and-backdating |
| RSS Parrot / Pinhole / Fedisky | Narrower one-directional bridges (RSS→Fediverse; Bluesky→ActivityPub; AP extension for a Bluesky PDS) | learn↔ (round out the interop surface) | active | repos as cited | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| #IndieSky / Eurosky (Modal Foundation) / Free Our Feeds | Independent-infrastructure movements: non-corporate relays, private-group and localized-moderation infra (real funder is Free Our Feeds) | partner, homage | active | freeourfeeds.com; eurosky | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Self-host blob storage (MinIO / Garage / SeaweedFS) | S3-compatible self-host object stores; MinIO community edition archived Feb 2026; Garage + SeaweedFS the maintained alternatives | learn↔ (PDS blob backends) | MinIO archived; Garage/SeaweedFS maintained | garagehq.deuxfleurs.fr; github.com/seaweedfs/seaweedfs | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |

*Managed hosts (ElfHosted, DigitalOcean 1-Click, Hostinger — and Vultr, which is a supported installer target but has no 1-Click app) and cloud blob backends (Backblaze B2, Cloudflare R2, Hetzner, Wasabi, iDrive e2, cold-tier archives) are commercial services rather than ecosystem projects; their price/trap details and the Wasabi 90-day and cold-tier retrieval traps live in `atproto-selfhosting-appviews-and-bridges.md`. All $ figures there are illustrative and volatile — refresh before external use.*

## atproto apps, clients & aggregators (the content plane)

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| Frontpage | Hacker-News-style link aggregator on a custom lexicon (post/comment/vote); single global feed, groups later | learn↔ (content-plane-first build order) | early, active | frontpage.fyi | verified | atproto-ecosystem; social-lexicon-group-research-brief |
| WhiteWind | Longform Markdown blogging (`com.whtwnd.blog.*`), data on the PDS | learn↔, homage (web-of-docs demand) | early | whtwnd.com | web-verified 2026-06-22 | atmospheric-web-and-aggregators; atproto-ecosystem; atproto-nsid-and-lexicon-mechanics |
| Leaflet | Docs/publishing with block-composition (each block may be a different lexicon) | learn↔ | early, active | leaflet.pub | web-verified 2026-06-22 | atmospheric-web-and-aggregators; atproto-ecosystem |
| Standard.site / ATmosphere (Automattic WordPress plugin) | Longform publishing lexicon set; the WordPress→atproto plugin dual-writes a bsky post + a full Standard.site record | rebroadcast, learn↔ (the dual-write bridge pattern) | new (v1.0.0, May 2026) | (Automattic) | web-verified 2026-06-22 | atmospheric-web-and-aggregators; atproto-ecosystem |
| Smoke Signal (Nick Gerakines) | Events/RSVP app on atproto; records live in the user's PDS; drove the Lexicon Community namespace; Rust/Postgres/Redis, MIT | learn↔ (the NSID/lexicon worked example) | relaunched + open-sourced ~July 2025 | smokesignal.events | verified (repo/schema) — self-limiting-AppView + HTTP framework left open | atproto-nsid-and-lexicon-mechanics; atproto-ecosystem; atmospheric-web-and-aggregators |
| Lexicon Community | Self-governed, Bluesky-independent lexicon namespace (`community.lexicon.*`: calendar, bookmarks, locations, reactions) with a volunteer TSC | learn↔ (governed-namespace hedge against domain loss) | active | github.com/lexicon-community/lexicon | verified (schema JSON) | atproto-nsid-and-lexicon-mechanics |
| AIP (ATProtocol Identity Provider, Gerakines) | OAuth 2.0 / OpenID Connect service authenticating via atproto PDS OAuth | learn↔ | active | (Gerakines) | verified (talk/repo) | atproto-nsid-and-lexicon-mechanics |
| recipe.exchange | Community recipe sharing reusing atproto identity + Bluesky image CDN; custom-lexicon records | learn↔ (content-plane-first, custom-lexicon-on-public-identity) | early, active (265 recipes 2026-07-08) | recipe.exchange | verified live 2026-07-08 — storage locus `[confirm]` | atproto-ecosystem |
| Tangled | Decentralized Git collaboration; "knots" (headless repo hosts) + "spindles" (Nix CI); `sh.tangled.*` records, per-repo DID | learn↔ (adjunct off-PDS storage pattern) | alpha, active | tangled.org | verified | atproto-ecosystem; atmospheric-web-and-aggregators |
| Semble / Streamplace / Flashes / npmx | Atmospheric-web verticals: research knowledge network / livestreaming / Instagram-like photos / npm-registry browser with atproto sign-in | learn↔ (each proves the custom-lexicon-on-public-identity pattern) | live | project sites | web-verified 2026-06-22 — maturity snapshot | atmospheric-web-and-aggregators |
| Graysky | Alternative Bluesky client defining its own `app.graysky.*` namespace | homage, learn↔ (the custom-namespace exemplar) | live | graysky.app | web-verified 2026-06-22 | atmospheric-web-and-aggregators |
| ATrium (atrium-rs) | Established Rust atproto framework; atrium-lex + atrium-codegen generate types from lexicons; bsky-sdk on top | build-on (the Rust client path) | active | github.com/atrium-rs | web-verified 2026-06-22 | atmospheric-web-and-aggregators |
| bsky_tui | Rust TUI Bluesky client (Ratatui/Tokio over ATrium) | homage (decoupled-presentation proof) | active | (repo as cited) | web-verified 2026-06-22 | atmospheric-web-and-aggregators |
| Jacquard | Low-boilerplate Rust atproto crates (zero-copy deserialization, ergonomic OAuth) | build-on (the lower-boilerplate ATrium alternative) | — | repo as cited | dialogue-sourced 2026-06-20→22 **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| megalodon-rs | One Rust interface over many fediverse servers (Mastodon, Pleroma, Friendica, Firefish, GoToSocial, Pixelfed); Apache-2.0 | build-on (multi-fediverse adapter) | — | repo as cited | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| lemmy-client-rs | Official Rust Lemmy client, WASM-aware | build-on (the Lemmy pond adapter) | — | repo as cited | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| Openvibe | Aggregator presenting one *combined* Mastodon/Bluesky/Nostr/Threads timeline | reject, learn↔ (the fused-timeline anti-pattern) | live | openvibe | dialogue-sourced 2026-06-20→22 **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| deck.blue | TweetDeck-lineage per-source *columns* client | learn↔, homage (the composable unit) | live | deck.blue | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| SkyFeed / Flare / Mixpost / CrossPoster / yup-live | Aggregators & cross-posters spanning the fork-a-client license map (EUPL-1.2 / AGPL-3.0 / open-core / open source / unmaintained) | build-on / learn↔ (fork candidates; license decides shippability) | mixed maturity | project repos | web-verified 2026-06-22 — licenses/maintenance volatile, re-check before distribution | atmospheric-web-and-aggregators |
| Phanpy / Fedilab | Values-aligned fediverse clients: de-emphasized engagement (Phanpy); simple-by-default progressive disclosure (Fedilab) | homage, learn↔ (engagement-restraint design references) | shipping | phanpy.social; fedilab.app | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| Ouranos / Heron / atcute / Bluesky social-app | Client bases: Next.js web / offline-first Kotlin Multiplatform / from-scratch TS protocol utils / the official app | build-on / learn↔ (client upstreams a private AppView sits behind; Heron nearest to local-first) | active | project repos | web-verified 2026-06-22 | atproto-selfhosting-appviews-and-bridges |
| Crux (Red Badger) | Hexagonal Rust app framework: side-effect-free core, effects-as-data, WASM + native | homage, learn↔ (adopt the pattern slim, not the framework) | pre-1.0 | redbadger.github.io/crux | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| Tauri v2 / Leptos / Dioxus | Client shell + render paths: Rust shell + webview (Tauri); fine-grained-reactive Rust WASM UI (Leptos); Dioxus as the Path-B alternative not chosen | build-on (Tauri/Leptos), homage (Dioxus) | active | tauri.app; leptos.dev; dioxuslabs.com | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| Spacedrive | Rust core sharing a web UI across platforms — closest architectural twin | learn↔ (cautionary craft reference: demo is easy, finishing is hard) | active | spacedrive.com | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |

## App-pond building blocks (games, file drop, media, on-device AI)

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| libmarathon / Marathon (sunbeam-stdio) | Offline-first multiplayer framework on Bevy + iroh + iroh-gossip + CRDTs; 3D cube-sim demo | build-on, learn↔ (the exact substrate combo, assembled) | active | crates.io/crates/libmarathon | web-verified 2026-06-22 | iroh-app-pond-building-blocks |
| ascii-royale (Chad Fowler) | 16-player terminal ASCII battle-royale in Rust over iroh; ticket-based join, no servers | homage, learn↔ (direct-QUIC twitch play) | demo | github.com/chad/ascii-royale | verified | iroh-app-pond-building-blocks |
| iroh-lan (rustonbsd) | Hamachi-like encrypted virtual-LAN over iroh for legacy LAN games + emulator netplay | learn↔, build-on (tunnel-localhost-over-iroh pattern) | active | github.com/rustonbsd/iroh-lan | verified | iroh-app-pond-building-blocks |
| godot-iroh | Godot Asset-Library extension swapping Godot's multiplayer socket for an iroh endpoint | build-on (engine integration drops onto iroh cleanly) | active | Godot Asset Library | web-verified 2026-06-22 | iroh-app-pond-building-blocks |
| GGRS + matchbox (gschup / community) | Rollback netcode in Rust + WebRTC signaling | build-on (architecture-aligned rollback stack) | active | github.com/gschup/ggrs | verified — license glance at bundle time | iroh-app-pond-building-blocks |
| netplayjs (rameshvarun) | JS rollback netcode + WebRTC; host-authoritative hidden-info dealer | reject-as-primary / port substrate (swap signaling for iroh) | active | github.com/rameshvarun/netplayjs | verified — `[confirm]` license at bundle | iroh-app-pond-building-blocks |
| Curvytron (Curve Fever / Achtung die Kurve) | MIT fork base for a 2–8-player twitch game | build-on (port/wrap the marquee twitch pad) | fork base | (Curvytron) | MIT — verify at bundle | iroh-app-pond-building-blocks |
| boardgame.io (nicolodavis) | MIT turn-based engine with a transport abstraction (JS/React) | learn↔ (turn-based patterns; JS/React in fit-tension with the Rust core) | mature | boardgame.io | MIT | iroh-app-pond-building-blocks |
| webxdc game catalog (adbenitez / ArcaneCircle) | Catalog of small games wrappable via one webxdc-compat shim | wrap, homage, learn↔ | active | (webxdc) | **LICENSE TRAP** — mixed GPL-3.0; chess art CC-BY-SA-3.0; "wonster" MIT; hand-pick + glance at bundle | iroh-app-pond-building-blocks |
| sendme (n0) | iroh-blobs file-transfer reference; the near-free device-to-device drop pad | build-on (the drop-pad reference) | shipping | github.com/n0-computer/sendme | verified — iroh-blobs version per FACTCHECK | iroh-app-pond-building-blocks |
| DataBeam (vinay-winai / schollz / n0) / croc | Desktop GUI uniting croc's code-phrase convenience with sendme's iroh speed/resumability | learn↔ (friendly drop-pad UX over iroh) | live | (croc: github.com/schollz/croc) | verified | iroh-app-pond-building-blocks |
| Cure53 webxdc security audit (Cure53 / OpenTechFund) | Audit establishing that a Content-Security-Policy alone does not contain a webview (WebRTC + DNS-prefetch exfiltration; FILL500) | learn↔ (security model — disable webview WebRTC in pads) | published audit | cure53.de | verified | iroh-app-pond-building-blocks |
| Apple Foundation Models | On-device ~3B model with guided generation (`@Generable`) constraining output to a real catalog | build-on (optional assistant; Apple target; needs Swift↔Rust bridge) | vendor-shipped (mid-2026) | developer.apple.com | vendor-documented — model sizes/device lists volatile, re-check at build | iroh-app-pond-building-blocks |
| Google Gemini Nano (AICore + ML Kit GenAI) | On-device Android model; strong isolation, steep device cliff, weaker structured output | build-on (optional assistant; Android target, fallback-heavy) | vendor-shipped | developer.android.com | vendor-documented — device coverage volatile, re-check at build | iroh-app-pond-building-blocks |
| callme / iroh-roq (n0) | Peer-to-peer audio over iroh with no WebRTC (iroh-roq datagrams + Opus, cpal capture) | build-on (the proven audio floor) | shipping | github.com/n0-computer (callme) | corroborated web 2026-06 | iroh-app-pond-building-blocks |
| iroh-live / MoQ (n0; moq-rs / moq-dev) | Media-over-QUIC broadcast over iroh (h264 + Opus, room-ticket connect); pub/sub, lazy; optional relay bridging to browsers via WebTransport | build-on (the broadcast ceiling) | shipping (n0 work) | github.com/n0-computer/iroh-live; github.com/moq-dev/moq | corroborated web 2026-06 | iroh-app-pond-building-blocks |
| Rave | Watch-party app shipping iroh + MoQ for video (chosen over libp2p and WebRTC) | learn↔ (adoption signal for MoQ-over-iroh) | shipping | Rave | corroborated — per-relay connection ceilings **[UNVERIFIED]** | iroh-app-pond-building-blocks |
| str0m | Sans-IO WebRTC media engine (codecs/jitter/FEC/echo-cancellation) | build-on / port substrate (WebRTC-over-iroh, the "str0m fold") | production as server-side SFU | github.com/algesten/str0m | server-SFU corroborated; **video maturity [UNVERIFIED]** — audio-first | iroh-app-pond-building-blocks |
| webxdc + Delta Chat mini-apps | Small web-bundle apps over iroh realtime (topic+ticket handoff; WebRTC-transport-swap porting recipe) | learn↔ (the pads/games grammar) | shipping | webxdc.org; delta.chat | dialogue-sourced 2026-06-20→22 **[UNVERIFIED]** | iroh-app-pond-building-blocks; atmospheric-web-and-aggregators |
| WeChat mini-program / W3C MiniApp (+ kbone, uni-app, Re.Pack) | Super-app guest-app grammar: permission scopes, guest isolation | learn↔ (borrow the grammar); reject (central distribution + observation) | mature / standard | w3.org/TR/miniapp | dialogue-sourced **[UNVERIFIED]** | atmospheric-web-and-aggregators |
| Bond Touch | "Thinking-of-you" bracelet wrapping a ~50-byte partner ping in accounts, a cloud relay, and a subscription | reject, learn↔ (the extraction the free perpetual ping rebukes) | commercial | bond-touch | verified (negative example) | iroh-app-pond-building-blocks |

## P2P messengers & community systems

| Project | What it is | Rel. | Status / maturity | Locator | Marker | Covered in |
|---|---|---|---|---|---|---|
| SimpleX | Graph-blind messenger achieving privacy by *deleting identity* (no user identifiers); double ratchet + PQ, envelope-encrypted metadata, self-hostable relays | learn↔ (closest to the graph-blind spec; groups least mature) | audited (multiple Trail of Bits), shipping | simplex.chat | landscape-grounded | adjacent-systems |
| Briar / Cwtch | Strongest capture-resistance: no servers or Tor-only, device-to-device, metadata-resistant by construction, audited | learn↔ (niche, small-group; not general-purpose infra) | shipping | briarproject.org; cwtch.im | landscape-grounded | adjacent-systems |
| Session | Decentralized messenger, reportedly dropped forward secrecy | learn↔ (picks decentralization, reportedly pays on FS) | shipping | getsession.org | landscape-grounded — "dropped FS" is a *reported* detail, not independently confirmed | adjacent-systems |
| Matrix + Element | Federated, self-hostable, E2EE; the most mature "both-axes" system — but room/membership metadata lives on homeservers and there is heavy de-facto centralization; Matrix Foundation co-authors MIMI | learn↔ (mature-but-metadata-leaky) | mature | matrix.org; element.io | landscape-grounded | adjacent-systems |
| Germ | MLS-based E2EE messenger using atproto identity (no phone number); a separate encrypted layer atop atproto; Bluesky added a Germ profile badge Feb 2026; investors include an MLS co-author | learn↔ (Croft's architecture scoped down to DMs; the one production MLS-on-atproto deployment) | beta (iOS) | (see TechCrunch 2026-02-18) | press | atproto-ecosystem |
| Roomy (Muni Town) + Leaf | Discord-like communities on atproto + CRDT (Loro/Automerge); a two-pivot journey (tried Willow/Keyhive/Jazz → built the Leaf off-protocol sync layer → swung back atproto-native with The Arbiter); encrypts messages but leaks conversation metadata | learn↔ (nearest neighbor: durable + encrypted + communities, but experimental, service-mediated, metadata-leaky) | GA July 1 2026 | muni.town / roomy | press/landscape-grounded | adjacent-systems; atproto-ecosystem; willow-meadowcap; substrate-prior-art |

---

## Coverage / gaps

**Coverage.** Every discrete project, protocol, spec, or library named across the fifteen cairn content docs
has a home row above, grouped by its primary role. Standards appear under their RFC/draft numbers (MLS RFC
9420, MLS Architecture RFC 9750, DPoP RFC 9449, Certificate Transparency RFC 6962; MIMI and the two
decentralized-MLS drafts as Internet-Drafts). The three cairn "poles/positioning" docs
(identity-and-data-ownership-poles, atproto-ecosystem, adjacent-systems) contribute the comparison anchors
(Solid, DSNP, Peergos, Germ, Roomy, SimpleX, Briar/Cwtch, Session, Matrix); the survey/homage docs contribute
the build-among rows.

**Deliberately not registered (out of scope, by the docs' own boundary calls).** The protocol-level CRDT and
fork/merge formal underpinnings — Byzantine-fault-tolerant CRDTs, Blocklace, the CALM boundary — are deferred
to the Drystone spec's prior-art appendix and are not ecosystem-directory rows. iroh's and MLS's own
*mechanics* live in the impl transport/MLS notes; here they are directory entries, not mechanism write-ups.
Commercial hosting/blob-storage vendors are folded into a note under atproto self-hosting rather than given
project rows (they are services, not projects). Governance/deliberation tooling named only inside a profile
(Polis, Open Collective for Blacksky) is carried in that profile, not promoted to a row.

**Rows to treat with caution.**

- **Dialogue-sourced / [UNVERIFIED] — verify before any external use or build decision.** The entire Rust and
  client-tooling adapter layer (Jacquard, megalodon-rs, lemmy-client-rs, Crux, Tauri/Leptos/Dioxus, Spacedrive,
  Phanpy/Fedilab, Openvibe, deck.blue, the webxdc/Delta-Chat and mini-app grammar rows) is dialogue-sourced and
  unverified — crate names, licenses, and feature claims all need an independent pass. The routing lineage (RINA,
  NDN, Yggdrasil, cjdns), the did:webvh tooling (didwebvh-rs/didtoolbox), iroh-rings, Keyhive, and the DIDComm
  delegate are dialogue-sourced 2026-06-20→24.
- **`[confirm before publish]`.** The decentralized-MLS rows (DMLS/FREEK, draft-xue "TwoMLS") and the
  "no production MLS is serverless-ordered" adoption anchor carry the source's confirm-before-publish flag; the
  Spritely funding/project facts are `[UNVERIFIED current]`; Discord "DAVE" as an MLS adopter is `[confirm]`.
- **Volatile — refresh before external use.** Every web-verified-2026-06-22 row (atproto self-hosting, AppViews,
  bridges, atmospheric-web apps, DSNP/Solid/Farcaster/Lens facts) describes a fast-moving ecosystem; prices,
  versions, licenses, and maturity drift. iroh version facts (iroh 1.0; companion crates pre-1.0) cite the
  FACTCHECK source of truth and are not re-verified here. Groundmist's auth-disabled caveat, Rave's per-relay
  ceilings, and str0m's video maturity are the specific point-in-time unknowns.

**Most notable unhomed candidate.** No named project lacks a home row. The nearest thing to a gap is *the
atmospheric web itself* — the community demand surface (per atproto.com) that every atproto content app in the
register rides — which is a named *concept*, not a project, and so is intentionally not a row; it is the
demand-side argument in `atmospheric-web-and-aggregators.md` rather than a directory entry.
