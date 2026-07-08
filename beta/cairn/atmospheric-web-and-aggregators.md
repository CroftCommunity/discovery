# The atmospheric web and the aggregator strategy

`Status: cairn layer (Layer 3, the open field). Register: survey / demand-side argument. Resolution: library —
the prior-art and demand-side grounding for Croft's client-and-aggregator (the "garden of ponds") direction.
External facts carry verification flags inline: the atmospheric-web app roster and the aggregator license-map
were web-verified 2026-06-22 (refresh volatile facts — maturity and licenses drift); the Rust/client-tooling
register is dialogue-sourced and marked [UNVERIFIED] throughout, pending independent verification before
external use.`

## Overview

This document registers the *demand side* of Croft's bet — the apps people already want to build on open
social protocols, and the tooling that would let a small team build a client that aggregates them — and draws
out why each grouping matters to Croft's client-and-aggregator strategy. Three load-bearing conclusions lead,
because they are the "why anyone wants this" case, not a catalogue:

1. **The atmospheric web is a real, named demand surface — and every serious version of it hits the
   public-by-default wall, which is exactly the gap Croft's private-group primitive fills.** *Atmospheric web*
   is the community term (per atproto.com) for the non-social apps built on AT Proto — the "web of docs /
   Neo-GeoCities / open-LinkedIn" vein. The pattern is not hypothetical: a homestead of live apps already
   exists in pieces. But each ambitious version (an open-LinkedIn with private alumni groups, closed
   neighborhoods, B2B verticals) runs into the same limit — AT Proto is public-by-default and has no
   native private-group primitive. That limit *is* Croft's thesis, which makes the atmospheric web an
   adoption argument for the confidentiality layer Croft is building, not merely an adjacent surface.

2. **Open social protocols charge no per-activity gas or rent, so "build a TweetDeck for the open web and
   write your own adapters" is economically viable — and the fork-a-client license-map decides which
   downstream clients you may actually redistribute.** This is what makes the garden-of-ponds aggregator
   strategy affordable, and the license-map is what keeps it legally shippable.

3. **A silently fused timeline is an anti-pattern; per-source columns are the composable unit.** Openvibe
   (one combined Mastodon/Bluesky/Nostr/Threads timeline) is the negative example that confirms Croft's
   honest-seams stance; deck.blue / TweetDeck-style per-source columns are the shape that keeps each source
   legible as itself.

## Charter: what this document covers

- **In scope:** the atmospheric-web demand argument; the aggregator/cross-poster license-map and the
  no-per-activity-fee finding; the fused-timeline anti-pattern; and a register of atmospheric-web apps and
  the Rust/client tooling that would sit behind Croft's transport ports — each with the reason Croft credits,
  learns from, or borrows-and-rejects it.
- **Out of scope (and where it lives):** the ATProto ecosystem *positioning* (public-vs-private, access
  control vs confidentiality, Drystone's empty cell) is in `atproto-ecosystem.md`; the substrate and
  transport prior art (iroh, CRDT, MLS, the routing lineage) is in `substrate-prior-art.md`; MLS itself is in
  `mls-and-mimi.md`; the iroh-native app-pond building blocks (games, realtime media, on-device AI) live in a
  sibling cairn doc. The product framing this feeds — the aggregator ponds and the cold-start problem — is
  `croft/product-the-garden-of-ponds.md`; the demand-side adoption thesis is `socialization/adoption-strategy.md`.
- **Boundary call:** this is the "what people want to build, and how you'd build a client that reaches it"
  register. The confidentiality mechanism that answers the public-by-default wall is documented elsewhere; here
  we carry only the demand-side reason it matters.

## The demand-side argument: the atmospheric web and the public-by-default wall

*Atmospheric web* is the AT-Proto community's term (per atproto.com) for the non-social applications built on
the protocol — the vein sometimes described as a web of docs, a *Neo-GeoCities*, or an *open-LinkedIn*. The
shape is a lightweight document lexicon whose records live in the author's own repository (so the "pages" are
portable and host-independent), rendered by a purpose-built client, with composable moderation (opt-in labeler
"curated web rings") and feed generators for discovery. It is the cozy indie web on self-sovereign rails.

The load-bearing observation is not that this is buildable — it demonstrably is, and a homestead of live apps
already covers pieces of it (see the register below). It is that **every ambitious version of the atmospheric
web hits the same wall: AT Proto is public-by-default, with no native private-group primitive.** An
open-LinkedIn wants private alumni groups and recruiter DMs; closed neighborhoods want members-only spaces; a
B2B vertical wants confidential data. None of those has a native protocol answer — private messaging and
groups on AT Proto are third-party today. That gap is precisely Croft's thesis: the private, member-scoped
group is the primitive the atmospheric web keeps reaching for and cannot get from the base protocol.

This reframes the whole register. The atmospheric web is not an adjacent product Croft might also build; it is
a standing, independent demand-side argument for the confidentiality layer Croft is already building. The
"delightful client over rugged self-sovereign data" adoption thesis is the same one Croft holds. When someone
asks *why would anyone want a private-group substrate*, the atmospheric web is the answer: because the open web
of docs, neighborhoods, and profiles that people are already building wants private membership and has nowhere
to get it.

`[The "atmospheric web" term and the public-by-default architecture are web-verified 2026-06-22 against
atproto.com. Refresh before external use — the ecosystem moves fast.]`

## The aggregator strategy: no per-activity fee, and the fork-a-client license-map

Two findings make the aggregator half of the garden-of-ponds strategy viable, and they travel together.

**No per-activity gas or rent.** ActivityPub and AT Protocol carry no per-activity fee, network rent, or gas.
ActivityPub is a standard HTTP POST to a server's API — only the instance admin pays hosting. AT Protocol is an
XRPC push of a small record into a PDS repository, with no blockchain validation layer and therefore no
execution fee. The consequence for a client builder: if most of a user's activity targets these federated
networks, the marginal infrastructure cost per post is effectively bandwidth-only. That is what makes "build a
*TweetDeck for the open web*, and write your own adapters" a viable strategy rather than a cost sink — the
economics of a flat, low-price SaaS (or a free tier subsidized by a small tip cut) actually close. This is the
direct feed into the garden-of-ponds aggregator ponds: aggregator ponds are cheap to run because the protocols
they aggregate do not tax activity. (Web3 protocols like Farcaster and Lens *do* carry rent/gas — Farcaster
storage rent is roughly $7/unit — which is exactly why the aggregator's federated adapters and its Web3
adapters have different cost profiles.)

**The fork-a-client license-map.** If the strategy is "fork or repackage an existing open client and write
adapters," the license each candidate carries decides whether a downstream distribution is legally
shippable. The map matters because a permissive license and a network-copyleft license impose very different
obligations on a hosted service:

| Client / bridge | License | What the license permits downstream |
|---|---|---|
| Bridgy Fed (ActivityPub ↔ AT Proto bridge; Python/Granary) | CC0 / public domain | Total freedom — fork freely, ship a commercial Docker image, no source-share obligation |
| Mixpost (self-hosted Buffer/Hootsuite alternative) | open-core / dual-license | Free Community Edition plus a paid Pro/Enterprise white-label tier; the dual-license is the business model |
| SkyFeed (AT-Proto feed builder; Dart/Flutter) | EUPL-1.2 (copyleft) | Copyleft obligations apply; a downstream distribution must honor the EUPL's share-alike terms |
| Flare (multi-network aggregator; Kotlin Multiplatform; Mastodon/Bluesky/X/Misskey/Nostr/RSS) | AGPL-3.0 | Network copyleft — host it as a service and you must publish your modified source |
| CrossPoster (Next.js cross-poster with AI drafting) | open source | Repackageable as a cross-posting base |
| yup-live (Vue3/Turborepo/Ionic/Tauri multi-protocol aggregator) | open source, unmaintained | Repackageable, but the unmaintained status is the cost to weigh |

The reason to carry the license column and not just the app names is the anti-rollup rule applied to a
distribution strategy: the decision ("fork this client for the aggregator pond") is only trustworthy if the
obligation it incurs (AGPL network-copyleft means publishing modified source; CC0 means none) travels with it.
An AGPL client forked into a hosted Croft pond would obligate publishing the modifications; a CC0 bridge would
not. That difference is load-bearing for the garden-of-ponds strategy and must not be dropped.

`[The no-per-activity-fee finding and the aggregator license-map were web-verified 2026-06-22 (dedicated
research pass). Licenses and maintenance status are volatile — re-check the specific repository before any
downstream distribution decision.]`

## The fused-timeline anti-pattern

One negative example sharpens the composable-unit decision. **Openvibe** presents a single *combined* timeline
that fuses Mastodon, Bluesky, Nostr, and Threads into one stream. That is the anti-pattern: silently fusing
sources erases which network a post came from, its trust and moderation context, and its native affordances —
it launders provenance. It is the exact thing Croft's honest-seams stance rejects.

The composable unit is the opposite: **deck.blue** (and the TweetDeck lineage generally) shows per-source
*columns*, where each source stays legible as itself. That is the shape an aggregator pond should take — a
column per pond, each honest about what it is, rather than one blended feed. Openvibe confirms the stance by
being the counter-example; deck.blue supplies the unit of composition.

`[Dialogue-sourced 2026-06-20→22, [UNVERIFIED] — the anti-pattern framing is sound but the specific product
behaviors need independent verification before external use.]`

## Register A — atmospheric-web apps (web-verified 2026-06-22)

These are live, non-social (or infrastructure) apps on AT Proto — the homestead the atmospheric-web argument
rests on. All rows web-verified 2026-06-22; refresh volatile maturity facts before external use. Grouped with
the reason Croft credits or learns from each.

**Publishing and documents (the "web of docs" spine).** WhiteWind (Markdown blogging with data on the PDS),
Leaflet (long-form/social publishing, block-editor), and Standard.site (a long-form publishing *lexicon set*,
block-based rather than Markdown-primary) are the working proof that the document vein is tractable. Croft
credits them as the demand evidence for the web-of-docs surface — this is the content people already publish
on portable, host-independent records. Relationship: learn↔, homage.

**Verticals that prove the custom-lexicon pattern.** Tangled (decentralized Git collaboration, with self-host
"Knots" for off-repo bulk data), Semble (a research knowledge network — not a Linktree clone), Smoke Signal
(decentralized events/RSVP, MIT-licensed), Streamplace (livestreaming), Flashes (an Instagram-like photo
client), and npmx (an npm-registry browser with AT-Proto sign-in — the developer-tooling vertical) each show a
distinct vertical riding the same public identity plus a custom lexicon. The lesson Croft
takes: the content plane is tractable and the reuse-plus-custom-lexicon split is standard practice, which is
the shape an aggregator pond consumes. Relationship: learn↔.

**Graysky — the custom-namespace exemplar.** Graysky, an alternative Bluesky client, defined its own
`app.graysky.*` namespace. It is the clean exemplar of a third-party client extending the lexicon space with
its own namespace — the pattern Croft would follow if a pond needs client-specific records. Relationship:
homage, learn↔.

**ATmosphere (Automattic's WordPress plugin) — the dual-write bridge.** The WordPress → AT Proto plugin (v1.0.0,
May 2026) publishes a standard Bluesky post for visibility *and* stores the full article as a Standard.site
lexicon record so other apps can render the long form independently. Croft credits it as a rebroadcast/bridge
pattern — the same dual-write move an aggregator pond makes when it wants both reach and a durable native
record. Relationship: rebroadcast, learn↔.

**Tap — official repo-sync/backfill.** Tap is the official tool for subscribing to a Relay and auto-backfilling
via `getRepo` (events marked `live:false` until backfill completes; SQLite/Postgres storage). Croft credits it
as build-on: if any Croft pond ever needs an AppView, indexer, or backfill path, Tap is the reference for how
to hydrate from the firehose without re-inventing sync. Relationship: build-on.

**Verified Rust AT-Proto tooling (ATrium, bsky_tui).** Two Rust items are web-verified and so belong with this
register rather than the dialogue-sourced tooling in Register B. ATrium (atrium-rs) is the established Rust
AT-Proto framework: atrium-lex plus atrium-codegen generate Rust types directly from lexicons, and a bsky-sdk
sits on top — it is the reference Rust client path, and the baseline that the lower-boilerplate Jacquard
(Register B) positions itself against. bsky_tui is a Rust TUI Bluesky client (Ratatui/Tokio over ATrium) — the
working proof that presentation decouples cleanly from the AT-Proto core, which is exactly the port boundary a
Croft client would draw. Relationship: build-on (ATrium, the Rust client path), homage (bsky_tui, the
decoupled-presentation proof).

`[All Register A rows web-verified 2026-06-22 against atproto.com and the projects' own sites. Maturity and
roadmap facts are a mid-2026 snapshot and will drift — refresh before external use.]`

## Register B — Rust and client tooling behind ports ([UNVERIFIED] — dialogue-sourced, pending verification)

**Every row in this section is dialogue-sourced and NOT independently verified. Treat all of it as
`[UNVERIFIED]` pending an independent verification pass before any external use or reliance.** These surfaced
in the Croft app-design dialogue (2026-06-20→22) and are registered so they are not lost, with the reason each
matters. The grouping is what is load-bearing; the specific facts (crate names, license claims, feature
descriptions) all need confirmation.

**AT-Proto and multi-fediverse client crates (the adapter layer behind a transport port).** Jacquard offers
low-boilerplate AT-Proto Rust crates with zero-copy borrowed deserialization and ergonomic OAuth — the
lower-boilerplate alternative to ATrium (the established Rust AT-Proto framework, web-verified in Register A)
for the Bluesky adapter. megalodon-rs gives one interface over many fediverse servers
(Mastodon, Pleroma, Friendica, Firefish, GoToSocial, Pixelfed) under Apache-2.0 — the multi-fediverse adapter.
lemmy-client-rs is the official Rust Lemmy client, WASM-aware and version-skew-managing — the Lemmy pond
adapter. Croft credits these as build-on candidates, each sitting behind a transport/source port so a pond can
speak to its network without leaking network specifics into the core. Relationship: build-on.

**The hexagonal-core pattern (adopt slim, not the framework).** Crux (Red Badger) is a hexagonal Rust app
framework: a side-effect-free core with effects-as-data, running on WASM and native. Croft's interest is the
*pattern* — adopt the effects-as-data, port-driven core discipline slim, rather than adopting the pre-1.0
framework wholesale. Relationship: homage, learn↔.

**Shells and render paths.** Tauri v2 (Rust shell + web frontend across all five platforms including Android,
webview-per-OS) is the desktop/mobile shell candidate; Leptos (fine-grained-reactive Rust web UI compiled to
WASM, sharing a same-memory boundary with the core) is the web render path; Dioxus is registered as the Path-B
alternative not chosen. Among crafted-app references (Zed, Excalidraw), Spacedrive is the closest twin to
this architecture — a Rust core sharing a web UI across platforms — and carries the load-bearing caution that
a shell demo is easy while finishing is hard, which is the honest cost estimate for the client-shell path.
Croft credits Tauri and Leptos as build-on for the client shell. Relationship: build-on (Tauri/Leptos),
homage (Dioxus), learn↔ (Spacedrive, the cautionary craft reference).

**Values-aligned client references.** Phanpy (an open web Mastodon client that deliberately de-emphasizes
engagement actions) is the closest values-aligned, multi-column client — the design reference for a client that
does not optimize for engagement. Fedilab (a fediverse client that is simple-by-default with more-on-demand) is
the shipped proof of progressive disclosure. Croft credits both as design references. Relationship: homage,
learn↔.

**Mini-app / pad references (borrow the grammar, reject central distribution).** webxdc plus Delta Chat
mini-apps (small web-bundle apps over iroh realtime, using a topic-plus-ticket handoff and a WebRTC-transport-swap
porting recipe) are the pads/games-pond reference. The WeChat mini-program and W3C MiniApp models supply the
super-app guest-app *grammar* — permission scopes, guest isolation — but Croft explicitly **rejects** their
central distribution and observation (the gatekeeper trap). Related open mini-app runtimes (kbone, uni-app,
Re.Pack/react-native-sandbox) contribute guest-isolation and permission patterns. Croft borrows the grammar and
rejects the central store. Relationship: learn↔ (borrow), reject (central distribution).

`[Every Register B row is dialogue-sourced 2026-06-20→22 and [UNVERIFIED]. Independent verification of crate
names, licenses, feature claims, and maintenance status is required before external use or any build decision.]`

## What this establishes (and does not)

Establishes that the atmospheric web is a real, named (per atproto.com) demand surface whose every ambitious
version hits AT Proto's public-by-default wall, making it a standing demand-side argument for Croft's
private-group primitive rather than merely an adjacent product; that the aggregator strategy is economically
viable because ActivityPub and AT Proto charge no per-activity gas or rent, with the fork-a-client license-map
(CC0 → AGPL network-copyleft) deciding which downstream distributions are shippable; that the silently fused
timeline (Openvibe) is an anti-pattern and per-source columns (deck.blue) are the composable unit; and it
registers the atmospheric-web app homestead and the Rust/client tooling that would sit behind Croft's ports,
each with the reason it matters to the garden-of-ponds strategy.

Does **not** re-document the ATProto public-vs-private positioning (that is `atproto-ecosystem.md`), the
substrate/transport prior art (`substrate-prior-art.md`), MLS (`mls-and-mimi.md`), or the iroh-native app-pond
building blocks (its sibling cairn doc); does **not** resolve the garden-of-ponds product decisions or the
cold-start problem (`croft/product-the-garden-of-ponds.md`); and does **not** certify the Register B tooling
rows — those are dialogue-sourced `[UNVERIFIED]` and require an independent verification pass before external
use. The Register A rows and the aggregator license-map are web-verified as of 2026-06-22 but describe a
fast-moving ecosystem; refresh volatile facts before relying on them.
