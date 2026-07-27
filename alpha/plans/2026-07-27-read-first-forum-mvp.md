# Plan: read-first forum MVP — a local-first PWA lens over atproto (phase plan, Pass 1)

date: 2026-07-27
identity: chasemp (`chase@owasp.org`, `github-personal`)
status: **Pass 1–3 complete (2026-07-27).** Ready for execution pending two cheap BLOCKING items (repo/
product name; the D2 probe). See the Review Log + Open Questions + the board-ready Kanban breakdown below. Code home = the **shared Rust
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

### Phase 0 — discovery probe (Discovery Exemption: no TDD/wiring/commit-per-item)
- [ ] **D1: What do the AppView read responses actually look like?** **Probe:** hit `public.api.bsky.app`
  for `getFeed` (a known feed-generator AT-URI), `searchPosts` (a hashtag), `getPostThread` (a known post);
  **print raw JSON.** **Success:** field names/shapes confirmed against returned bytes — `post.record.text`,
  the four counts (`reply/repost/like/quoteCount`), `embed` variants, `cid`/`uri`, `cursor`, CDN thumb URLs,
  the `viewer` block, `#blockedPost`/`#notFoundPost` placeholders. **Disposition:** `keep-as-fixture` (these
  become the `feed-core` test fixtures — consumers get TDD, the fixtures don't).
- [ ] **D2 (BLOCKING for Phase 5): Does r=1/r=2 need OAuth, or is an unauth fetch-by-handle enough?**
  **Probe:** resolve a handle→DID, then call `app.bsky.graph.getFollows` and `getFollowers` against
  `public.api.bsky.app` **unauthenticated**; confirm both return the full public lists paginated.
  **Success:** both resolve unauth (→ radius needs only the viewer's *handle*, no OAuth, no write scope) OR
  they require auth (→ Phase 5 needs a read-only OAuth step). **Disposition:** `throwaway`.
- [ ] **D3 (PHASE-GATED, informational): confirm the CDN thumb/fullsize/avatar URL forms** returned in D1 are
  directly `<img src>`-able cross-origin. **Disposition:** `throwaway`.
**Done when:** D1 fixtures captured; D2 resolved and the Phase-5 approach fixed accordingly; Verified
Assumptions updated with firsthand evidence.

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

## Documentation Impact
- **`<new-repo>/README.md`** — created in Phase 1 (what the client is, how to build the WASM core + serve
  the shell, the "read-only lens, no writes" scope). Phase that makes it stale: Phase 1.
- **`discovery/alpha/thinking/app/client-architecture-adr.md`** — add the forum as the *first realized
  `feed-core + Bluesky port` instance* (it was decided but unbuilt). Phase: 1. (Grep done — the ADR + E19 are
  the only cross-refs to `feed-core`.)
- **`discovery/alpha/ROADMAP_TODO.md` E62/E70** — flip to "MVP build underway" + link the repo once it
  exists. Phase: 1. **`ECOSYSTEM.md` §5j** (TOKIMEKI/SkyFeed) — cite once shell UX patterns are borrowed
  (Phase 2/3).
- This plan doc — the living handoff (Review Log updated each pass).
- Grepped for other stale refs to a "read-first forum" / "Social Tree" build: only E62/§59/§60/§5i/§5j and
  the two fact-check docs — all already consistent.

## Concurrency Map
Sequential spine: **Phase 0 → Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 5 → Phase 6.**
Rationale: each phase reads what the prior produced — Phases 2–6 all consume the Phase-1 `feed-core`/WASM
bindings; the shell app-shell + router is shared mutable state that 2/3/4/6 all edit.
**One flagged parallel candidate (opt-in, user's call): {Phase 2 board-list UI, Phase 3 thread drawer}** —
both depend only on Phase 1 and their *component* write-sets are disjoint (`board-list/*` vs
`thread-drawer/*`), BUT both edit the shell **router + app-shell composition** (shared write-set) → **cannot
be parallel as scoped** unless the router wiring is extracted into a Phase-1.5 seam first. Default: keep
sequential. Not worth worktree isolation for a two-component split.

## Per-phase wiring tests + validation calibration (Pass 3)
Every phase's gate is an **entry-point wiring test**, not isolated unit tests:

| Phase | Wiring test (RED→GREEN, exercises the call chain) | Validation scope |
|---|---|---|
| 1 `feed-core` | port(real adapter) → core → sorted `PostCard[]` from a **live** `getFeed` fixture *and* a `@live` probe; + boundary unit tests (window edges, tie-break, empty/media-only title) | Moderate (real fixtures + `@live` where egress allows) |
| 2 board list | shell → WASM core → **DOM renders N cards** in the selected sort; toggling sort/window re-renders; filter toolbar drives URL state (popstate restores) | Moderate (manual on a mid-range phone; 60fps scroll) |
| 3 thread drawer | tapping a card → `getPostThread` via the port → tree renders; **close preserves exact scroll**; deep-link `/p/:did/:rkey` resolves + highlights | Moderate |
| 4 boards | create a board config → switch → the **correct query executes** and renders; shared `/view?q=…` URL reconstructs it | Moderate |
| 5 radius | enter a handle → fetch public follows/followers (per D2) → **r=1 toggle reduces the rendered set to mutuals** with no re-fetch; a quiet mute drops a DID with zero network calls | Broad (live graph fetch; verify no write scope requested) |
| 6 PWA/a11y | SW registered → **offline reload serves a cached board**; Home-Screen install; axe scan clean; keyboard-only nav reaches every control | Broad (on-device iOS + axe + Lighthouse) |

**Observability:** `feed-core` emits structured diagnostics behind a debug flag — per-fetch counts, cache
hit/miss, filter drop-counts, sort timing, and **429 backoff** events; the shell logs port errors + the
"stale-while-revalidate" banner state. Log levels: WARN for 429/backoff and port errors, DEBUG for
counts/timing. **Rate-limit handling** (p-queue + exponential backoff on 429 `retry-after`) lives in the
Bluesky port (Phase 1), not sprinkled in the shell.

## Open Questions
- **[CONFIRMED: BLOCKING] Repo / product name** (A-series naming decision) — needed before the shell repo is
  created; the `feed-core` crate can grow in the existing croft-group/app workspace first, so this blocks the
  *shell repo*, not Phase 0/1. *User owns naming.*
- **[RECOMMENDED: BLOCKING for Phase 5] D2 — does radius need OAuth or just an unauth graph fetch?** Resolved
  by the Phase-0 D2 probe; it fixes whether Phase 5 adds a read-only OAuth step. *Cheap to resolve now.*
- **[RECOMMENDED: PHASE-GATED (Phase 1)] `feed-core` home:** a new crate in the existing `croft-group`
  workspace (E19) vs a fresh `croft-app` workspace. *Reuses the decided decomposition either way; affects
  where the port lives.*
- **[RECOMMENDED: ADVISORY] Search impl:** Rust crate in-core (tantivy-class) vs a thin JS MiniSearch shim in
  the shell for the first spike. *Core is the default (E48 confirms JS MiniSearch is fine as a stopgap).*
- **[RECOMMENDED: ADVISORY] Shell framework:** vanilla-TS (skylite/arecipe consistency) vs a light framework.
  *Shell is thin either way; deferitable to Phase 2.*

## Kanban / roadmap breakdown

Epics = phases; cards are the trackable units (size S/M/L; each card's Done = its acceptance criterion).
Dependencies are strict unless noted. This is the board-ready decomposition.

**EPIC 0 — Discovery (spike, ~0.5–1 day)**
- `0.1` Probe AppView read shapes, save fixtures (D1). **S.** Done: 3 fixture files committed.
- `0.2` Probe unauth graph fetch by handle (D2). **S.** Done: OAuth-needed? answered; Phase 5 approach fixed.
- `0.3` Confirm CDN img URLs render cross-origin (D3). **S.** Done: one-line note in Verified Assumptions.

**EPIC 1 — `feed-core` (Rust → WASM) [dep: E0]**
- `1.1` Bluesky/AppView **port** trait + real adapter + in-proc fake. **M.** Done: fake serves fixtures;
  real adapter fetches `@live`.
- `1.2` `normalize` response → `PostCard`. **M.** Done: unit tests green on fixtures.
- `1.3` time-window filter + tie-break; sort modes (MostDiscussed/Hot/New/Ratio). **M.** Done: boundary tests.
- `1.4` synthetic-title extraction (incl. media-only/empty). **S.**
- `1.5` native-block/mute + local-keyword filter pass. **S.**
- `1.6` IndexedDB cache + LRU purge (via the shell's storage adapter behind a port). **M.**
- `1.7` `wasm-bindgen` boundary + debug-diagnostics + 429 backoff in the port. **M.** Done: WASM built,
  callable from a smoke shell; wiring test green.

**EPIC 2 — Board list UI (web shell) [dep: E1]**
- `2.1` Virtualized card list (compact/card density). **M.**
- `2.2` Metadata bar + synthetic title + lazy/ThumbHash media. **S.**
- `2.3` Sort + time-window controls wired to the core. **S.**
- `2.4` Responsive filter toolbar per E69 (desktop dropdowns → mobile `<dialog>` sheet; container queries). **L.**
- `2.5` URL state (pushState/replaceState/popstate) + finite-window empty state + skeletons. **M.**

**EPIC 3 — Thread drawer (read) [dep: E1; soft-dep E2 for launch surface]**
- `3.1` Non-destructive slide-over (translate3d, scroll preserved). **M.**
- `3.2` `getPostThread` render: depth colors, collapse+pill, OP badge, load-more, deleted/blocked placeholders. **L.**
- `3.3` Deep-link routing `/p/:did/:rkey` + `/comment/:rkey` (highlight + semantic `<a href>`). **M.**

**EPIC 4 — Boards as saved queries [dep: E2]**
- `4.1` Board config schema (hashtag / feed-URI / multi-query smart view) in IndexedDB. **M.**
- `4.2` Board switcher + per-board default sort/window. **S.**
- `4.3` Shareable `/view?q=…` config (dedup by uri for smart views). **M.**

**EPIC 5 — Read-only identity + radius lens [dep: E1, E2; gated on D2]**
- `5.1` Handle→DID + fetch public follows/followers; compute r=1 (and lazy r=2). **M.**
- `5.2` Radius toggle (Global/Follows/Mutuals) as a core filter (no re-fetch). **M.**
- `5.3` Quiet local mutes + private local user-tags (IndexedDB, DID-keyed). **M.**

**EPIC 6 — PWA + a11y polish [dep: E2–E5]**
- `6.1` Manifest + service-worker app-shell cache + offline cached-board read. **M.**
- `6.2` `storage.persist()` + iOS Home-Screen install prompt. **S.**
- `6.3` a11y pass (WCAG 2.5.8/2.5.1, ARIA disclosure/dialog, reduced-motion) + axe scan. **M.**
- `6.4` Full-parity list view + OLED/typography controls. **S.**

**Suggested milestones:** M1 = E0+E1 (the core proves the thesis is buildable, headless). M2 = E2+E3 (a
usable read client). M3 = E4+E5 (boards + the small-world radius lens — the differentiator). M4 = E6 (ship-
quality PWA). The thesis is *testable at M2*; the *distinctive* value lands at M3.

## Review Log

### Pass 1 — 2026-07-27
Base plan: problem, approach, reasoning, six phases + Phase 0, scope cuts (payments/private/tree out),
verified-primitives list. Corrected mid-session: WASM `feed-core` is the core (E5/E19), not deferred;
Jetstream is a later live path, not "out."

### Pass 2: Gap Analysis — 2026-07-27
**Found:** (a) r=1/r=2 needs the viewer's graph — whether that's OAuth or an unauth fetch-by-handle was
unverified and gates Phase 5 → added Phase-0 **D2**. (b) Rate-limit handling was in Risks but not owned by a
phase → assigned to the Bluesky port (Phase 1). (c) Phase 5's "read-only identity" over-implied OAuth; if D2
passes it's just a handle input (no auth, no write scope). (d) The `feed-core` home (existing croft-group
workspace vs a new croft-app one) was implicit → surfaced as a PHASE-GATED question.
**Concurrency:** added the Concurrency Map (sequential spine); flagged {Phase 2, Phase 3} as a parallel
candidate but **pulled it back to sequential** — both edit the shared shell router/app-shell (shared
write-set), so not safe without a Phase-1.5 router seam.
**Changed:** Phase 0 rewritten as D1/D2/D3 with dispositions; Documentation Impact + Concurrency Map + Open
Questions added; port-level rate-limit ownership noted.
**Confirmed:** the AppView-XRPC read path (no firehose/indexer), the feed-core/shell split, and the scope
cuts all hold.

### Pass 3: Quality Gates — 2026-07-27
**TDD ordering:** every phase now has a named **wiring test** that exercises the call chain (port→core→DOM),
not isolated unit tests; boundary/mutation cases named for the filter/sort/window edges (Phase 1). Phase 0 is
Discovery-Exempt (dispositions declared per task).
**Observability:** `feed-core` structured diagnostics behind a debug flag (fetch counts, cache hit/miss,
filter drop-counts, sort timing, 429 backoff); WARN vs DEBUG levels specified; backoff owned by the port.
**Debugging readiness:** each epic is an independently-committable checkpoint; the milestones (M1–M4) are the
health gates.
**Validation calibration:** per-phase scope set (Moderate for the engine/UI, Broad for radius/live-graph and
PWA/on-device/a11y); a `@live` tier noted as hand-run given the prior sandbox egress block.
**Concurrency honesty:** Map present and accounts for all phases; the one parallel candidate examined and
correctly kept sequential (shared router write-set); shared-state contract is trivial (single sequential
worktree, no daemons/ports).
**Documentation impact:** README + ADR + ROADMAP/ECOSYSTEM updates assigned to the phases that make them
stale (Phase 1/2), not a trailing docs phase.
**Coherence:** still solves the original problem (a read-first lens on verified-green primitives); no scope
creep (the cut layers stayed cut).
**Confirmed ready:** yes, pending the two BLOCKING items (repo name; D2 — both cheap).

## Next
Resolve the two BLOCKING items (repo/product name; run the D2 probe — both cheap), pick the `feed-core` home
(existing `croft-group` workspace vs a new `croft-app` one), then execute **Phase 0 → M1 (E0+E1)**. The
kanban breakdown above is board-ready; import epics 0–6 as columns/swimlanes and the numbered cards as
tickets.
