# atproto self-hosting, AppViews & bridges: the sovereign-index landscape

`Status: cairn layer (Layer 3, the open field). Register: survey / homage. Resolution: library — the
self-hosting, AppView, and bridge grounding for Croft's private-overlay and self-host posture. External
facts were web-verified 2026-06-22; prices and version claims are point-in-time and volatile — treat $
figures as illustrative and refresh before external use. The Blacksky organization is profiled in
`blacksky-and-atproto-community.md`; this document references it and does not repeat it.`

## Overview

The load-bearing conclusion is that **owning the read/index layer inverts atproto's public-by-default
posture**, and that inversion is a buildable shape of Croft's blind-broker / honest-seams stance. atproto
publishes public repositories and lets anyone index them; a self-run **AppView** (the component that reads
the firehose and builds the queryable index a client actually talks to) is the point of leverage where an
operator can add private blocking, keep feeds off-repo, and ingest from multiple sources. That is the same
move Croft makes when it puts a private overlay in front of a public substrate. The strongest single
relative is **Groundmist** (grjte, Ink & Switch): a private, local-first atproto layer that is the closest
local-first-private cousin to Croft's model, carried here with its own caveat that private-by-default is
still intent rather than security.

Underneath the AppView sit two supporting landscapes that Croft's self-host posture depends on. First, **PDS
self-hosting** — the reference Personal Data Server is single-tenant SQLite, and the alternatives (Cocoon,
rsky-pds), managed hosts, and blob-storage backends are the concrete answers to "can this survive as small
self-hosted nodes." Second, **bridges and migration tooling** — Bridgy Fed and Bounce are the
"people not platforms" cousins to Croft's exit / portability thesis, and atproto's decoupling of identity
from host (CAR export/import) is the structural backstop that makes "no data hostage" real.

## Charter: what this document covers

- **In scope:** the atproto AppView / indexer landscape (with the AppView-as-private-gatekeeper pattern as
  the headline), PDS self-hosting implementations, managed hosts, blob-storage backends, cross-protocol
  bridges and migration tooling, and the independent-infrastructure movements — each with the reason Croft
  credits, reuses, or learns from it.
- **Out of scope (and where it lives):** the Blacksky organization, its governance model, and its Community
  Posts source-of-truth inversion (`blacksky-and-atproto-community.md`); the broader ecosystem comparison
  and Drystone's positioning against it (`atproto-ecosystem.md`); NSID / lexicon mechanics
  (`atproto-nsid-and-lexicon-mechanics.md`); content portability and backdating
  (`atproto-content-portability-and-backdating.md`); the substrate / transport prior art
  (`substrate-prior-art.md`); MLS and messaging siblings (`mls-and-mimi.md`).
- **Boundary call:** this is the "how the read/index and hosting layers are self-run, and among whom"
  register. Where a project (rsky-wintermute) belongs to Blacksky, its full profile lives in the Blacksky
  doc; here it appears only as a member of the AppView landscape.

## The AppView-as-private-gatekeeper pattern

atproto's default is public-by-default: repositories are public, the firehose is public, and any operator
can build an index. The AppView is the component that consumes that firehose and serves the queryable view a
client reads. Owning the AppView is therefore the leverage point for privacy: an operator who runs the
read/index layer can enforce private blocking (blocks that are not themselves published), serve feeds that
never live in a public repo, and ingest from more than one source. This *AppView-as-private-gatekeeper*
pattern inverts public-by-default into private-by-index, and it is a buildable shape of Croft's
blind-broker / honest-seams stance — Croft likewise puts a controlled read layer in front of a substrate it
does not want to expose wholesale.

**Groundmist** (grjte, associated with Ink & Switch) is the headline relative. It is a private, local-first
atproto layer — a *Personal Sync Server* (PSS) that is analogous to a PDS but private, using Automerge CRDT
sync and publishing through the WhiteWind lexicon. It is the closest local-first-private relative to Croft's
model: private, local-first, CRDT-backed, atproto-native. The caveat travels with the credit and must not be
dropped: as of verification the sync server ships with authentication disabled, so private-by-default is the
project's *intent*, not yet its security posture. `[web-verified 2026-06-22; prototype status and the
auth-disabled caveat are point-in-time — refresh before external use.]`

Why load-bearing for Croft: Groundmist is the existing effort that most directly demonstrates that a
private, local-first layer over atproto is buildable, and its honest caveat is itself the lesson — a
private-by-default claim is not security until the auth story is finished, which is exactly the bar Croft's
own overlay has to clear rather than assert.

## AppViews and indexers to build among

- **rsky-wintermute** — Blacksky's Rust indexer plus *Private Community Posts* AppView scaffolding. This is
  the closest existing stack to Croft's own (Rust, high-throughput indexing, private-community posts held at
  the AppView layer rather than in the public repo), which is why it is the reference to study before
  Croft's private-index layer hardens. The full Blacksky profile — including the source-of-truth inversion
  that Community Posts represent and the operational postmortems — lives in
  `blacksky-and-atproto-community.md` and is not repeated here.
  `[web-verified 2026-06-22; ~10k records/sec throughput figure is point-in-time.]`
- **AppViewLite** (alnkesq) — a lean AppView in C#; roughly 2.2 GB of disk per day to index the firehose,
  reading from Jetstream / a relay with a memory-mapped columnar store. Relevance: the lean self-host AppView
  where private-filter logic could be injected, i.e. a lighter starting point than a full reference clone.
  `[web-verified 2026-06-22; disk-per-day figure is point-in-time and volatile.]`
- **zeppelin bluesky-appview** (zeppelin-social) — a Docker-Compose full reference AppView stack (the bsky
  appview, indexer, label ingestion, a Palomar-style search, and Postgres / Redis / OpenSearch). Relevance:
  the reference clone to fork and wrap with custom proxy middleware when a full self-hosted index is wanted.
  `[web-verified 2026-06-22.]`
- **Jetstream** (official) — converts atproto's binary firehose into compressed, filterable JSON, the
  lightweight ingestion path. Relevance: the on-ramp every self-run index above depends on; the cheap way to
  consume the firehose without decoding the binary protocol directly. `[web-verified 2026-06-22.]`
- **Client bases** — **Ouranos** (pdelfan, Next.js web), **Heron** (tunjid, offline-first Kotlin
  Multiplatform with Compose / Room — the mesh-sandbox-shaped one), **atcute** (mary-ext, from-scratch TS
  protocol utilities), and Bluesky's official **social-app**. Relevance: the client upstreams a private
  AppView would sit behind; Heron's offline-first shape is the nearest to Croft's local-first client
  posture. `[web-verified 2026-06-22.]`

Why load-bearing: these establish that self-running the read/index layer is not aspirational — there is a
spectrum from lean (AppViewLite) to full reference (zeppelin) to the Rust private-community reference
(rsky-wintermute), all fed by a standard ingestion path (Jetstream), and clients ready to sit in front of
them. Croft's private-overlay direction inherits a working template rather than a blank sheet.

## PDS self-hosting: implementations, hosts, and blob backends

The official reference PDS (`@atproto/pds`, TypeScript) is **single-tenant SQLite** — per-user `.sqlite`
repos plus PDS-wide databases, bound to the local filesystem. That shape answers the hobby-deployment case
but not the shared-cluster or object-storage cases, which is where the alternatives matter to Croft's
"survive as small self-hosted nodes" requirement.

- **Cocoon** (haileyok) — an alternative PDS in **Go** with a **PostgreSQL** backend, so it can share an
  existing database cluster rather than being pinned to local SQLite. Self-described as highly experimental,
  not production-ready. Relationship: learn↔ (the Postgres-PDS path). `[web-verified 2026-06-22.]`
- **rsky-pds** (Blacksky) — an alternative PDS in **Rust** (Postgres, S3 blobs, Mailgun), part of the `rsky`
  workspace. This is the closest to Croft's own stack of the PDS implementations and the natural
  Rust-PDS reference. Relationship: build-on, learn↔. `[web-verified 2026-06-22.]`
- **Managed hosts** — **ElfHosted** (fully-managed PDS: provisioning, HTTPS, updates; point your domain);
  **DigitalOcean**'s BlueSky Social PDS 1-Click Marketplace app (slug `blueskysocialpds`, bundles Caddy —
  the genuine 1-click host); **Hostinger**'s one-click Docker VPS template on the official PDS image.
  Relevance: the managed-host model, and the cooperative-vs-SaaS tension it puts pressure on. Illustrative
  $ figures (ElfHosted cited near $9/mo but with an unconfirmed intro trial; DigitalOcean droplet roughly
  $4-6/mo; Hostinger roughly $6.49/mo) are point-in-time and volatile — refresh before external use.
  `[web-verified 2026-06-22; prices illustrative only.]`
- **Vultr — a correction, not a partner.** Vultr is a supported VPS target for the official
  `bluesky-social/pds` installer (`pdsadmin` works there), but the claim that it offers a 1-Click Marketplace
  PDS app was refuted on verification — there is no such marketplace app. Recorded here as a correction so
  the imprecise claim does not propagate. `[web-verified 2026-06-22.]`

**Blob-storage backends** (where a PDS keeps media, S3-compatible unless noted). Every figure below is
illustrative and volatile:

- **Backblaze B2** — roughly $6/TB, with **free egress** when served via Cloudflare (Bandwidth Alliance); a
  strong PDS blob store on cost. `[web-verified 2026-06-22; price illustrative.]`
- **Cloudflare R2** — **zero egress fees** (roughly $15/TB storage); best for high-traffic public media.
  `[web-verified 2026-06-22; price illustrative.]`
- **Hetzner / Wasabi / iDrive** — low-cost S3-compatible backends (iDrive e2 roughly $4/TB; Hetzner roughly
  $5.99/TB; Wasabi roughly $6.99-7.99/TB flat). Carry the **Wasabi 90-day minimum-retention trap**: deleted
  objects are still billed for 90 days, which distorts the flat-rate economics for churny data.
  `[web-verified 2026-06-22; prices illustrative.]`
- **Deep-cold tiers** (AWS Glacier Deep Archive / Azure Archive / GCP Archive) — roughly $1/TB, but with a
  **retrieval-penalty trap**: steep retrieval and egress penalties make them wrong for active blobs and
  right only for rarely-touched data. `[web-verified 2026-06-22; prices illustrative.]`

**Two corrections worth keeping visible.** MinIO — often cited as the self-host S3 backend — had its
community-edition repository **archived in February 2026**; **Garage** and **SeaweedFS** are the maintained
self-host alternatives. And atproto **decouples identity from host**: CAR repository export/import lets an
account migrate PDS without losing followers, which is the structural "no data hostage" backstop that makes
any of the hosting choices above non-captive. `[web-verified 2026-06-22.]`

Why load-bearing: the hosting economics are the concrete form of Croft's self-host requirement. The Postgres
and Rust PDS paths show the single-SQLite reference is not the ceiling; the blob-backend traps (Wasabi
retention, cold-tier retrieval) are the failure modes a self-hoster hits in practice; and the CAR
export/import backstop is the structural guarantee Croft's own portability thesis leans on.

## Bridges and credible exit

If owning the index is how you keep data private, bridging and migration are how you keep people free to
leave — the exit / portability half of Croft's thesis.

- **Bridgy Fed** (A New Social; Ryan Barrett / "snarfed" and a small team) — the main **bi-directional
  ActivityPub↔atproto bridge** (Mastodon / Threads ↔ Bluesky, via `@bsky.brid.gy` / `@ap.brid.gy` routing).
  It is `snarfed/bridgy-fed`, **CC0**, Python (Granary plus Arroba), with a re-architected per-user cost
  (cited moving from roughly $0.15 to $0.03 per user per month). Relevance: the "people not platforms"
  cousin to Croft's exit posture — the credible path between protocol islands rather than a walled reach.
  `[web-verified 2026-06-22; user count and per-user cost figures are point-in-time.]`
- **Bounce** (A New Social) — cross-protocol account **migration**, framed as *credible exit*: move into a
  native PDS without losing bridged followers. Relevance: a no-data-hostage instrument, the account-level
  counterpart to the repo-level CAR export/import backstop. `[web-verified 2026-06-22.]`
- **Narrower bridges** — **RSS Parrot** (RSS/Atom → Fediverse), **Pinhole** (Bluesky → ActivityPub, one-way),
  **Fedisky** (an ActivityPub extension for a Bluesky PDS). Relevance: the niche one-directional bridges that
  round out the interop surface. `[web-verified 2026-06-22.]`
- **Migration tooling** — **goat** and **PDS MOOver** (`pdsmoover`): CAR export/import plus signed PLC
  operations for moving accounts (with returning-user-only inbound to bsky.social). Relevance: the operator
  tooling behind Bounce-style migration. `[web-verified 2026-06-22.]`
- **Independent-infrastructure movements** — **#IndieSky**, **Eurosky** (Modal Foundation), and
  **Free Our Feeds**: non-corporate relays, private-group and localized-moderation infrastructure for the
  open atproto web. Note the funder precisely: the real funder is **Free Our Feeds**; the label
  "AT Community Fund" is imprecise and should not be used as the funder's name. Relevance: homage and
  potential partners — the movement-level evidence that a non-corporate atproto infrastructure layer is
  being actively built, which is the field Croft's cooperative hosting question sits in.
  `[web-verified 2026-06-22.]`

Why load-bearing: bridges and migration are the credible-exit machinery. Croft's portability thesis is only
honest if leaving is real, and Bridgy Fed / Bounce demonstrate that cross-protocol exit is buildable and
already carrying real users; the CC0 license on Bridgy Fed makes it reusable rather than merely admirable.

## What this establishes (and does not)

Establishes that owning the AppView / read-index layer is a real, buildable inversion of atproto's
public-by-default posture, and that this inversion is the same shape as Croft's private-overlay / honest-
seams stance; that a private, local-first atproto layer already exists in prototype (Groundmist) as the
closest cousin to Croft's model, with its own honest not-yet-security caveat; that self-hosting a PDS has a
real spectrum beyond the single-SQLite reference (Cocoon, rsky-pds, managed hosts) with checkable
blob-backend traps; and that credible exit is buildable and shipping (Bridgy Fed, Bounce), backstopped
structurally by CAR export/import.

Does **not** re-profile the Blacksky organization or its Community Posts source-of-truth inversion (see
`blacksky-and-atproto-community.md`), does **not** position Drystone against the wider ecosystem (see
`atproto-ecosystem.md`), and does **not** certify current prices or versions — all $ figures are
illustrative and every external fact here was web-verified only as of 2026-06-22 and must be refreshed
before external use.
