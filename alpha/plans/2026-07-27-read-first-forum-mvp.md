# Plan: read-first forum MVP — a local-first PWA lens over atproto (phase plan, Pass 1)

date: 2026-07-27
identity: chasemp (`chase@owasp.org`, `github-personal`)
status: **Pass 1 (plan + reasoning) — drafted this pass.** Pass 2 (gap review) and Pass 3 (TDD/diagnostic/
validation gates) deferred; invoke the `phase-plan` skill to run them. Code home = a **new static-PWA repo**
(name TBD — working name "Social Tree"; the forum name is an open A-series decision), not this repo; discovery
holds only the plan. Roadmap anchor: **E62** (+ E63/E69/E70/E71); scope corrections per user 2026-07-27
(E67 payments parked; private plane is Drystone's; mutuals-feed possible).

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

A pure client-side static PWA that rides the **existing Bluesky AppView** (`public.api.bsky.app`,
unauthenticated + CORS-enabled — verified) as its only read source. **No backend, no custom indexer, no
custom lexicon for the MVP** — those belong to the fuller Social Tree, not here. Vanilla-TS static PWA to
match the house pattern (skylite/arecipe) and to let us borrow AppView-interaction patterns from the good
open-source PWA clients (TOKIMEKI/SkyFeed, E70). Local-first: IndexedDB cache with aggressive LRU; OPFS only
as a speed cache, never sold as durability (E71). Build the testable engine first (pure functions over real
response fixtures), then the UI, then boards, then the optional read-only-identity radius filter, then PWA
polish. TDD throughout; each phase leaves a working state.

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
- **Why no custom indexer for the MVP:** the "Jetstream chasm" (a browser can't filter the raw global
  firehose) only bites if you run live global aggregation or custom `app.socialtree.*` lexicons. The MVP reads
  *finished* views from Bluesky's own AppView, so the indexer is a *later* concern, not an MVP dependency.
- **Why vanilla-TS static PWA:** consistency with skylite/arecipe (same deploy story, $0 static hosting) and
  E70's "borrow from the PWA clients." A framework + WASM engine (the transcript's React/Svelte+Rust) is
  **deferred** — MiniSearch-in-JS is verified fine at MVP scale (E48); WASM earns its place only if profiling
  shows jank.
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

### Phase 1 — the read engine (pure, TDD, no UI)
RED→GREEN on pure functions over the Phase-0 fixtures (use the real response types, never redefined schemas):
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
lexicons + a custom indexer AppView, live Jetstream, the **tree-UI** (E63 phase two), the **private room**
(Drystone Part 2 §6 — a separate project leverages it), **payments** (E67, parked), and the **WASM engine**
(deferred until MiniSearch-in-JS demonstrably janks).

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
