# Plan: read-first forum MVP — a local-first PWA lens over atproto (phase plan, Pass 1)

date: 2026-07-27
identity: chasemp (`chase@owasp.org`, `github-personal`)
status: **Pass 1 (plan + reasoning) — drafted this pass.** Pass 2 (gap review) and Pass 3 (TDD/diagnostic/
validation gates) deferred; invoke the `phase-plan` skill to run them. Code home = the **shared Rust
`feed-core`** grown in the croft-group/app workspace (the `feed-core + Bluesky port` slot E19 named) **+ a
thin web shell** (repo/name TBD — working name "Social Tree"; the forum name is an open A-series decision);
discovery holds only the plan. Roadmap anchor: **E62** (+ **E19/E5** client architecture, E63/E69/E70/E71);
scope corrections per user 2026-07-27 (WASM `feed-core` is the core not deferred; Jetstream is the later
live-updates path not "out"; E67 payments parked; private plane is Drystone's; mutuals-feed possible).

## Problem statement

The Social-Tree dialogue (E62) proposed a Reddit/Discourse-style forum that is really a **read-first lens over
existing Bluesky/atproto activity** — "a board is a saved query, no cold-start." After the 2026-07-27
fact-check and the user's scope cuts, the *buildable, high-confidence* core is exactly this read lens, with
the uncertain layers removed: **writes, the RSS reader, custom lexicons + a custom indexer, the tree-UI, the
private room (→ Drystone), and payments (parked) are all OUT of the MVP.** The problem this plan solves: ship
the read lens on verified-green primitives, with no unresolved architectural fork inside it, so it *tests the
actual thesis* (does a finite, high-signal forum view of the big world feel better than doomscrolling) at the
lowest possible cost and friction.

## Approach

A client whose **read engine is the shared Rust `feed-core`** — the already-decided `feed-core + Bluesky
port` from the Croft client architecture (`thinking/app/client-architecture-adr.md`; E19/E5), symmetric to
croft-chat-cli's `group-core + Transport port` — **compiled to WASM and driven by a thin web shell.** This is
the WASM the whole product is built on; it is **not** a deferred optimization and it is **not** a one-off — it
is the *same shared-core work* as croft-chat-cli / croft-group, grown into the `feed-core` slot that
decomposition already named. The web shell (TS) owns only platform glue + rendering; all domain logic
(normalize / time-window / sort / synthetic-title / search / graph-intersection / radius) lives in the Rust
core behind a **Bluesky/AppView port**, so the same core later feeds native shells (Tauri/iOS/Android) with no
logic re-fattening. **Read source for the MVP = the existing Bluesky AppView** (`public.api.bsky.app`, unauth
+ CORS — verified) over its **XRPC request/response** endpoints (`getFeed`/`searchPosts`/`getPostThread`): a
board is a *query*, so it is a fetch, not a firehose subscription — **no custom indexer and no firehose are
needed to read a board.** (Live updates and custom lexicons are a *different* concern — see the indexer/
Jetstream reasoning below.) Local-first: IndexedDB cache with aggressive LRU; OPFS only as a speed cache,
never sold as durability (E71). Borrow *interaction/UX* patterns (not the core language) from the good OSS PWA
clients (TOKIMEKI/SkyFeed, E70). Build the `feed-core` first (pure Rust, TDD over real response fixtures),
then the web shell UI, then boards, then the optional read-only-identity radius filter, then PWA polish. Each
phase leaves a working state.

**Verified primitives this rests on** (fact-check `research/2026-07-27-social-tree-factcheck.md` + `-2.md`):
`public.api.bsky.app` unauth CORS reads; `app.bsky.feed.getFeed` / `searchPosts` / `getPostThread(depth)`;
post view has `replyCount`/`repostCount`/`likeCount`/`quoteCount`; `cdn.bsky.app/img/{feed_thumbnail,
feed_fullsize,avatar_thumbnail}/…`; `<dialog>.showModal` + CSS container queries are Baseline-widely; MiniSearch
is client-side; WCAG 2.5.8 (24px) / 2.5.1. `popover` is Baseline-*Newly* (progressive-enhance only).

## Reasoning

- **Why read-first / no auth to browse:** it removes the three biggest cost centers at once — no OAuth before
  value, **no backend** (rides the existing AppView), and **no moderation desk** (read-only, honoring native
  blocks/mutes + local keyword filters). It's the cheapest possible test of the thesis, and it's the part the
  fact-check scored ~9.
- **Three distinct read paths — don't conflate them (this corrects the earlier draft):** (1) **board reads =
  AppView XRPC** (`getFeed`/`searchPosts`/`getPostThread`, request/response, cursor-paginated) — a board is a
  *query*, so no firehose is involved at all; this is the whole MVP read path. (2) **Live updates** ("N new
  comments" pills, live thread growth) = **Jetstream** — and you are right that Jetstream (the filtered JSON
  WebSocket, `wantedCollections=app.bsky.feed.post`) is **the only sane browser-native firehose**; the raw
  `com.atproto.sync.subscribeRepos` CAR/CBOR firehose is not browser-appropriate. A browser can hold a
  Jetstream WS filtered to posts and match against the boards/threads currently open — **no custom indexer
  required.** Jetstream is therefore a *later phase, not "out"* — it was only "out of the guest MVP" because
  request/response reads already populate a board; I overstated it. (3) A **custom indexer AppView** is needed
  *only* for custom `app.socialtree.*` lexicons or global cross-network aggregation the public AppView doesn't
  serve — *that* is the fuller Social Tree, genuinely out of this MVP.
- **Why the shared Rust `feed-core` + a thin web shell (not a vanilla-TS one-off):** the forum is an instance
  of the **Croft client architecture** (E5/E19, `client-architecture-adr.md`), which already decided a
  `feed-core + Bluesky port` symmetric to croft-chat-cli's `group-core + Transport port`. The read engine
  **is** that feed-core, so it should be authored as the shared Rust core (grown in the croft-group/app
  workspace) and compiled to **WASM** for the web shell — this is continuous with the croft-chat-cli work, and
  it is what lets the same logic later drive Tauri/iOS/Android shells without re-fattening. (skylite/arecipe
  are simpler vanilla-TS atproto *pads*; this forum is a richer *client* and belongs in the app-client
  architecture — E70's "borrow from the PWA clients" is about shell UX, not the core language.) Search can be
  a Rust crate inside the core (or a thin JS MiniSearch shim in the shell for the very first spike, E48), but
  the **default is the Rust core** — WASM is the core, not a jank-contingent optimization.
- **Why radius/mutuals is a *logged-in-read* feature, not guest-core:** r=1/r=2 need the *viewer's* social
  graph (follows/followers), which a guest doesn't have. So the guest MVP is r=∞ (public boards); a
  **read-only** identity step unlocks the mutuals/radius lens. Mutuals-only-as-a-feed is possible
  (follows∩followers, client-side — verified; such feeds exist in the wild) and honors S5 (a filter, not
  confidentiality). Writes stay out entirely.
- **Why writes/tree/private/payments are out:** scope discipline ("prove one pond before three"). Writes add
  OAuth + optimistic-UI + a moderation surface; the tree-UI is E63 phase two (list+search+avatars first); the
  private room is **Drystone's** plane (a separate project leverages it); payments are parked (E67).

## Phases

### Phase 0 — discovery probe (no product code; honor "never guess an API shape")
Write a throwaway probe that hits `public.api.bsky.app` for `getFeed` (a known feed-generator AT-URI),
`searchPosts` (a hashtag), and `getPostThread` (a known post), and **prints the raw JSON**. Confirm field
names/shapes actually returned (`post.record.text`, the four counts, `embed` variants, `cid`/`uri`, `cursor`,
CDN thumb URLs, `viewer` block, `#blockedPost`/`#notFoundPost` placeholders). **Save the responses as test
fixtures.** Gate: fixtures captured; field names confirmed against the returned bytes, not docs.

### Phase 1 — the `feed-core` (shared Rust core → WASM; pure, TDD, no UI)
Grow the `feed-core` crate in the croft-group/app workspace (the `feed-core + Bluesky port` slot E19 named),
behind a **Bluesky/AppView port** (real adapter + in-proc fake, mirroring croft-chat-cli's `Transport` port).
RED→GREEN on pure Rust functions over the Phase-0 fixtures (use the real response types, never redefined
schemas); compile to WASM and expose to the shell via `wasm-bindgen`:
(1) **normalize** a feed/search response → an internal `PostCard` model; (2) **time-window filter** (rolling
N-hours, default 4, configurable) with the secondary tie-break (replies → reposts → likes → newest); (3)
**sort modes** — `MostDiscussed` (replyCount), `Hot` (velocity `(replies·3+quotes·2+likes)/(ageHours+2)^1.5`),
`New`, `Ratio` (replies/(likes+1)); (4) **synthetic title** extraction (first line/sentence, media-only →
`[Image/Video Post]`, ~80-char truncate); (5) **local keyword/native-block filter** pass (honor `viewer.
muted`/`blockedBy` + a local keyword list) *before* sort. Then the **IndexedDB cache** (Dexie) with LRU purge
(~25 MB / 24 h unbookmarked; delete orphaned thread nodes). Gate: engine unit tests green on fixtures.

### Phase 2 — the board list UI
Virtualized compact/card feed (a lightweight vanilla virtual scroller; fixed-height cards to avoid CLS),
synthetic title + metadata bar (💬 replies · likes · reposts · quotes), compact↔card density toggle, sort +
time-window controls. The **responsive filter toolbar per E69**: desktop = sticky per-facet disclosure
dropdowns; mobile = a "Filters (n)" button → native `<dialog>.showModal()` bottom sheet; **URL state**
(URLSearchParams + pushState/replaceState, restore on popstate); `container-type: inline-size` on the toolbar.
Skeleton loading (reserved aspect-ratio boxes), ThumbHash/lazy media, and the **finite-window empty state**
("You're caught up on /f/… in this 4-hour window"). Gate: a board renders, sorts, filters, and scrolls at
60fps on a mid-range phone from cached + live data.

### Phase 3 — the thread drawer (read)
Non-destructive slide-over (`translate3d`, background feed stays mounted, Esc/swipe restores exact scroll).
`getPostThread(depth)` → recursive render with depth-colored connector lines, tap-to-collapse subtrees + a
summary pill, `[OP]` badge, "load more replies" for deep sub-threads, graceful `[Comment unavailable]` /
`[deleted]` placeholders. Deep-link routing to `/p/:did/:rkey` and `…/comment/:rkey` (auto-scroll + highlight;
semantic `<a href>` so middle-click opens a real tab). Gate: open/close preserves scroll; deep links resolve.

### Phase 4 — boards as saved queries
A board = a JSON query config in IndexedDB: a hashtag/keyword `searchPosts`, a feed-generator AT-URI `getFeed`,
or a **multi-query "smart view"** (dedup by `uri`). Board switcher + a URL-shareable board config
(`/view?q=…`). Per-board default sort/window. Gate: create/switch/share a board with no server.

### Phase 5 — read-only identity + the radius lens (optional within MVP)
A **read-only** "connect your handle" step (resolve handle→DID, fetch public follows + followers) → compute
`follows∩followers` (r=1) and, if pursued, r=2 (lazy/Bloom, per E62 trade-off A) → a client-side radius filter
(🌐 Global / 👤 Follows / 👥 Mutuals). **Quiet local mutes** + **private local user-tags** (IndexedDB, DID-
keyed, zero network signal). No write scope requested. Gate: radius toggle re-filters instantly with no
re-fetch; mutes/tags are local-only.

### Phase 6 — PWA polish + a11y
Web App Manifest + service-worker app-shell cache; offline read of cached boards/threads;
`navigator.storage.persist()` + Home-Screen-install prompt on iOS (dodge the 7-day eviction); OLED/true-black
+ typography controls; full WCAG pass (2.5.8 24px targets, 2.5.1 swipe-not-only-close, ARIA disclosure/dialog,
`prefers-reduced-motion`); a **full-parity list view** (the tree is explicitly *not* in this MVP). Gate:
Lighthouse PWA + an axe a11y scan clean; installs and reads offline.

## Explicitly OUT of this MVP (tracked elsewhere)
Writes (like/reply/post), the downvote lexicon, the RSS reader + CORS relay, custom `app.socialtree.*`
lexicons **+ a custom indexer AppView**, the **tree-UI** (E63 phase two), the **private room** (Drystone
Part 2 §6 — a separate project leverages it), and **payments** (E67, parked). **NOT out (corrected):** the
**Rust `feed-core`/WASM engine** is the *core* of the MVP, not deferred (see Approach + Reasoning);
**Jetstream live updates** are a *later phase*, not out-on-principle (Jetstream is the correct browser-native
firehose and needs no indexer); only *native shells* (Tauri/iOS/Android) over the same core are later.

## Validation & risks
- **TDD gate:** engine phases are pure functions tested against Phase-0 real-response fixtures (no redefined
  schemas). UI phases: behavior tests + a Playwright harness.
- **Live vs hermetic:** hermetic fixtures are the CI gate; a thin `@live` read tier hits `public.api.bsky.app`
  where network egress allows — note the prior **sandbox browser-egress block** (arecipe), so `@live` is
  hand-run in a networked env, not a hard CI gate.
- **Risks:** cold-feed empty engagement counters (skeleton + client thread-probe); rate-limits on the public
  AppView (p-queue + backoff on 429); AppView view-shape drift (the Phase-0 fixtures + a periodic re-probe);
  DOM/memory bloat on deep threads (virtualize; revoke object URLs on unmount).

## Next
Run phase-plan Pass 2 (gaps/downstream) and Pass 3 (TDD ordering, diagnostic logging, validation
calibration) before execution; settle the repo name (A-series naming decision) and stand up the new
static-PWA repo; then execute Phase 0.
