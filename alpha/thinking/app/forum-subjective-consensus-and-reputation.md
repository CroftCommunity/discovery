# The forum pad: bare-link discovery + subjective-consensus reputation on atproto

date: 2026-07-31

source: distilled from two 2026-07 dialogues — the claude.ai bare-link/naming conversation
(`seeds/transcripts/raw/amble-naming-coop-metering-dialogue-2026-07-31.md`, Thread A) and a Gemini
forum-mechanics conversation (`seeds/transcripts/raw/graze-forum-subjective-consensus-gemini-2026-07.md`).
The user's *reasoning* is primary; the Gemini turns supply mechanics.

> **Fact-check discipline.** The Gemini source is a model dialogue → per the source-of-truth
> `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`, atproto/lexicon facts are
> **dialogue-sourced, `[UNVERIFIED]`** until confirmed. Named projects (atwork.place, Sifa,
> `place.atwork.listing`, H3 geo-indexing) are pending-verification — see ECOSYSTEM §5b. "Merkle search
> trees" in the Jetstream passage is **atproto-repo-correct** (atproto repos are MSTs served in CAR
> files), distinct from the FACTCHECK's iroh-docs correction. No REFUTED items reintroduced.

This is the design layer for the forum pad (the mock is **Graze**, working name moving to **Amble by
Croft** pending clearance — `NAMING.md`, `research/amble-name-clearance-2026-07.md`). It bears directly
on the tracked **E80 ↔ E62 architecture fork** (COHESION §63) and is the social-tree thesis made
concrete.

## Problem statement

Two gaps motivate the pad:

1. **Bare-link discovery has no home.** The user wants to reshare links *with no commentary* — the act
   of sharing is the whole signal: not endorsement of everything in it, just "this was worth reading /
   I wish someone would recommend it to me." No forum surfaces bare, ranked, perpetual link-sharing as
   its own view. The tagline *is* the bar: **"this is something I wish someone else would recommend to
   me."**
2. **Reputation is gameable when all content is user-owned.** On a Reddit-shape forum over Bluesky/
   atproto, every metric a user's PDS self-reports (karma, counts) is forgeable. You cannot trust the
   PDS's word; reputation must be computed from *other users'* authenticated interactions.

## Approach

### 1. The bare-link view

A view that does nothing but rank shared links — no commentary, "top / trending / perpetual." Each link
can be rendered at several fidelities depending on what the reader wants: **bare URL** → the **short
form** browsers substitute (Open-Graph) → an **on-device AI summary** (onboard browser AI, no server) →
(speculative, flagged *fraught*) a generated image. The share action itself is the vote; the ranking is
the discovery surface.

### 2. Subjective-consensus reputation (graph-weighted)

The shift is from **Global Consensus** (Reddit: one objective score) to **Subjective Consensus** (a
post's score is relative to *your* social tree). Mechanics:

- **The AppView (or the client) is the source of truth, never the PDS.** Tally valid vote records from
  other DIDs pointing at a post's URI; ignore any self-reported reputation on the author's PDS.
- **Piggyback likes for upvotes; keep downvotes local.** Upvotes = native `app.bsky.feed.like`
  (interoperable — reads as a like across the network, so posts gain reach); downvotes = a custom
  `com.graze.feed.downvote` lexicon (contained — never injected into clients that can't contextualize
  negativity).
- **Weight by graph distance** (the thesis): Depth 0 = self; **Depth 1** (your follows) = highest
  multiplier; **Depth 2** = standard; **Depth 3+** = fractional / globally-Sybil-filtered. Plus
  **tenure multipliers** (DID age / first-valid-interaction) and cheap **diversity proxies** (PDS
  spread, age distribution, cross-labeler variety) instead of expensive true-diversity math. Aligns
  with EigenTrust/PageRank: trust flows from the established trunk; a bot farm forms an isolated cluster
  with **subjective weight ≈ 0** by default — **shadowbanning becomes an organic byproduct**, not a
  system to build.
- **Two-score UI** so subjective scores don't confuse: show a **Network Score** (global baseline, Sybil-
  resisted) beside a **Tree Score** (how *your* graph voted).
- **Reddit mechanics, re-derived subjectively:** Hot/Rising/Controversial become graph-weighted
  (Controversial is the most interesting — a stark split between your trusted graph and the global
  network); moderation is **decentralized Ozone labelers** the viewer subscribes to (drop from the feed
  UI, never delete from the PDS); awards are **graph-amplified** (a scarce weekly token; a Depth-1
  award paints a big highlight).

### 3. Two-mode, local-first architecture (the load-bearing ethos)

The pad is a **PWA/SPA that can operate without a custom AppView** — an AppView is a *convenience and
accelerator, not a necessity*. Two equal modes, both runnable client-side:

- **Global mode** — connect to a public **Jetstream** (lightweight filtered JSON WebSocket over the
  firehose; no CBOR/CAR parsing), tally upvotes/downvotes in IndexedDB → raw global karma, **no backend
  aggregator**.
- **Subjective mode** — on login, `app.bsky.graph.getFollows` establishes Depth 1; as votes stream via
  Jetstream, weight each by whether the voter's DID is in your tree. The "server" is the user's device.

Offering both as a **toggle** teaches the protocol's power (watch your graph filter the raw firehose).
Bot/spam filtering can lean on **default Bluesky labelers** immediately (no bespoke moderation engine).

### 4. Better-than-Reddit (only possible non-extractively)

Stackable **BYO-mod** labelers (subscribe to many; unsubscribe from a rogue mod team, posts survive on
PDSes); **algorithmic sovereignty** (literal weight sliders → export as a shareable custom feed);
**zero-cut P2P tipping** (WebLN/Solana/Stripe links in profile metadata → a native Tip button, Graze
takes 0%); **frictionless community forking** (`f/x` is a metadata tag, not a server-locked destination
— forking is a new filter, the archive isn't lost); **true offline archives** (IndexedDB: sync a
community incl. comment trees, read/draft/vote offline, flush on reconnect).

### 5. Bridging microblog ↔ forum (titleless posts)

`app.bsky.feed.post` has no title. Graze-authored posts use the **Markdown convention** (title as a `#`
first line + body, one plain `app.bsky.feed.post` — no lexicon other clients can't read). External
Bluesky posts get **smart title extraction** (first sentence / newline / 80-char cap; or the
`app.bsky.embed.external` Open-Graph title). Very short, embed-less posts render as a **titleless card**
(no forced header).

### 6. Delight extras (prior art + a Tamagotchi)

Third-party-client research (RES, Apollo, RIF, Slide, Infinity, RedReader, Geddit) names the features
power-users miss and the pad can offer natively given atproto + IndexedDB: custom user tags, color-coded
comment threads, auto-collapse / auto-hide-read, account-age highlighting, offline caching, config
export, cross-device state, keyboard nav, snappy ad-free UI. A **Pixel Pals**-style PWA pet is feasible
in-window (CSS sprites / canvas + a timer state machine + Service Worker offline + Web Push), with the
honest caveat that PWAs cannot break out of the browser window (no Dynamic Island / lock-screen / true
background execution — simulate elapsed time on reopen).

## Reasoning — why this matters to the corpus

- **It is the social-tree thesis made concrete.** "Anchor the online experience in our own social tree —
  the closer someone is to me, the more their weight matters" is exactly graph-weighted subjective
  consensus. The design turns the thesis into a shippable reputation model that is *inherently*
  Sybil-resistant (the bot farm has no inbound trust from your trunk).
- **It bears on the E80 ↔ E62 architecture fork (COHESION §63) — and softens it toward E62.** The user
  pulls both ways in one breath: the bare-link view "may make sense to build an AppView side" (E80-ward,
  the Next.js+Postgres large tier) *and* "social tree is my preference" (E62-ward, the read-first lens
  on public Bluesky). The **decisive new data point:** Gemini's design shows **subjective consensus
  works entirely client-side on Jetstream + `getFollows`** — no mandatory AppView. That means the
  read-first E62 approach *can* carry the karma/forum vision, with an AppView as an optional accelerator
  (global aggregation, heavy graph caching) rather than a necessity. This does **not** resolve the fork
  (a large tier may still want an AppView for global scale) — but it lowers the stakes: the pad ships and
  behaves correctly with **no** custom backend, and the AppView becomes a scaling convenience. Surface,
  don't resolve (PLAYBOOK §5); tracked in COHESION §63/§66 and E80.
- **The two-plane privacy and the "meter the boundary" economics connect it to the co-op.** The forum's
  content lives on user PDSes (the co-op's metered hosting, `cooperative-social-union-model.md`);
  zero-cut tipping and the non-extractive posture are the same ethos as the metered-billing "slow-lane
  users are nearly free to serve."

## Open items / gates (surface, don't resolve)

- **[decision] E80 ↔ E62 backend fork** — still the user's; this doc adds the "subjective consensus needs
  no AppView" data point but does not decide the large tier. Tracked COHESION §63, ROADMAP_TODO E80/E62.
- **[verify] atproto/lexicon facts** — Jetstream/`getFollows`/Ozone/label-header mechanics and the
  atwork.place/Sifa/`place.atwork.listing`/H3 projects are dialogue-sourced; confirm against a live
  source before any build leans on them (ECOSYSTEM §5b, flagged pending-verification).
- **[open] custom-lexicon commitment** — `com.graze.feed.downvote` / `com.graze.feed.award` /
  `com.graze.feed.vote` are proposals; NSID choice + whether upvote stays a plain like are build-time
  calls (and interact with the Amble rename — don't mint `com.graze.*` NSIDs into records until the name
  holds).
- **[fraught] generated-image link representations** — flagged by the user as fraught; treat as
  research, not a committed feature.
