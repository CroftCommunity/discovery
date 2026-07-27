# Raw: Bluesky/ATProto "Social Tree" — Reddit-style forum + RSS-reader lens PWA/SPA — design dialogue (2026-07-27)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a claude.ai product-design dialogue pasted 2026-07-27. Light UI render chrome
stripped (no "Added to memory" / duplicated thinking-summary headers were present; a few inline citation
markers left as-authored). No secrets.

**WHOLE-DOCUMENT FACT-CHECK CAVEAT.** This is a long AI product-thinking dialogue. The load-bearing
technical claims (Bluesky search operators; atproto method/field/lexicon names incl. the `site.standard.*`
long-form family; `threadgate` reply-gating semantics; native-E2EE-DM / "Privately Shared Data" claims;
atproto OAuth granular scopes; W3C Digital Credentials API + Chrome/Safari version-timing; WebAuthn PRF;
OPFS / File System Access / `chrome.storage.sync` quotas / Web Locks / share_target / Background Sync /
`navigator.connection`; Privacy Pass RFC number; MLS RFC 9420; SFrame; WebRTC DTLS-SRTP; ThumbHash/BlurHash;
Yjs/Automerge/HLC; OpenID4VP cross-device VP) are verified in
`../../../research/2026-07-27-social-tree-factcheck.md`. **Cite that doc, do not re-verify.** atproto/iroh/iOS
baseline facts defer to the FACTCHECK SoT (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`).

**What this is.** A single, sustained dialogue that grows one coherent product: a **local-first,
client-side PWA/SPA that reframes Bluesky/atproto activity as a Reddit/Discourse-style forum fused with a
Google-Reader-style RSS reader** — the user names it the **"Social Tree."** It starts from a concrete
question ("can I sort a Bluesky feed by most-comments over the last N days?"), establishes that the default
apps cannot but the open atproto APIs make a custom client trivial, then builds outward across: forum-idiom
mapping, a Rust/WASM client engine, moderation layering, long-form (`site.standard.document`) + comment
anchoring, mobile-first UX, deprecated-Reddit-feature restoration (RES/Apollo power features), the
degree-based **radius model (r=0 personal / r=1 mutuals / r=2 mutuals-of-mutuals / r=∞ global)**, quiet
local muting + private local user-tags, the **RSS-reader-as-submission-pipeline** unification, riding
**native browser bookmark/`storage.sync` sync** as a transport, browser credential/VC/passkey integration,
an **anti-extraction / utility-over-attention** thesis (finite feeds, intentionality budgets, continuous
local export, zero-take economics, an ethical WebRTC-relay commercial tier), and a 5-phase build roadmap.
The user's tenet — **"machines can compute provenance, never utility"** — is the philosophical spine.

**Relationship to the corpus (this is NOT net-new territory — it extends existing tracked work).**
- **E39** (aggregator pond, atproto path — a lens that inherits a population, no cold-start) is the *same
  core thesis* the dialogue reaches at "we are a view onto other activity … we don't have the seed
  problem." The Social Tree is E39 taken to full product/UX depth.
- **E24/E42** (sovereign AppView club) already holds: private feeds, blocking-as-experience-shaping,
  cooperative/private Labelers, multi-source AppView (AT+AP+Nostr+**RSS**). The radius model, quiet local
  mutes, mod-list mirroring, and RSS bridge are the client-side face of that.
- **E48** (client-side atproto search — FlexSearch/MiniSearch/SQLite-FTS5-WASM, OPFS VFS, Jetstream/CAR)
  already verified the WASM-search substrate the dialogue leans on. Cite E48, don't re-derive.
- **E44** (account kernel / encrypted repo-mirror / Web-Push / Home-Screen install) holds the PWA
  sync/encrypted-vault/cross-device layer. **S5** (`thinking/social-layer.md`): semi-private-on-a-public-
  plane leaks → the dialogue's own mid-course correction ("Definitely not invisible at all") re-derives S5.

---

## The dialogue (cleaned-paste, content-faithful)

**User:** On bsky, is there a way for me to take a feed and show the entries in the last N (days or weeks)
with most comments?

**Assistant:** Not natively in the default Bluesky app/website — feed generators serve posts by whatever
rules the feed's creator programmed (usually reverse-chronological); there is no toggle to re-sort an
existing custom feed by reply count over the last N days. You can achieve it via external feed builders
(SkyFeed input/filter/sort blocks; Bluesky Feed Creator) which let you filter by time window and sort by
reply count, but they can't ingest an existing 3rd-party feed's rules — you recreate them.

Follow-up ("is there a better way?"): three practical paths — (1) **Native search with date operators**
(`#Treksky since:2026-07-12 until:2026-07-26`, then the **Top** tab, which ranks by blended engagement
likes+replies+reposts, not comment-count strictly); (2) a **Python script** with the `atproto` SDK
(`client.app.bsky.feed.get_feed({'feed': FEED_AT_URI, 'limit': 100})`, filter by `created_at` cutoff, sort
by `reply_count`); (3) a **browser userscript** (Tampermonkey) that intercepts the `app.bsky.feed.getFeed`
response and re-orders by `replyCount`.

**User:** Could I set up a pwa/spa that showed it to me in a Reddit-style post interface? Import existing
feeds? Hashtags?

**Assistant:** Yes — atproto's API is public/open, so a purely client-side React/Vue/Svelte app hosted free
(GitHub Pages/Vercel/Netlify) works with no backend. Fetch via `@atproto/api` (`BskyAgent`, service
`https://bsky.social`); public GET endpoints don't require login. Import feeds via
`agent.app.bsky.feed.getFeed({feed: 'at://…/app.bsky.feed.generator/treksky', limit:100})`; hashtags via
`agent.app.bsky.feed.searchPosts({q:'#Treksky', limit:100})`. Filter client-side by `createdAt` cutoff,
sort by `replyCount`. Reddit-style card: vote UI, thumbnail from `post.embed`, "posted by u/handle in
r/FeedName", comment count opening `agent.getPostThread()`. No backend needed (unauthenticated GET), PWA
manifest + service worker for install, deep-link via post URIs.

**User:** How much can I use the default bsky apps to get top over days/weeks — hearts or replies by feed
field or hashtag?

**Assistant:** In the default UI you're bound to the search bar: `#hashtag since:YYYY-MM-DD until:…` +
**Top** tab; `(from:a OR from:b) since:…` for account sets. Hard limits: can't sort strictly by comments or
strictly by likes (Top is a blended score); can't apply date filters *inside* an existing feed AT-URI; no
UI date pickers (type `since:`/`until:` manually). For strict comment-count sort or feed-URI import you
need the custom SPA/PWA.

**User:** Keep going — activity factors within a feed to order and rank? Better ideas?

**Assistant:** Public activity signals per post: `replyCount` (active conversation), `quoteCount` (strong
intent — quote posts add commentary), `likeCount` (passive), `repostCount` (amplification). Ranking
formulas: **Discussion Velocity** (HN/Reddit gravity) `((replies·3)+(quotes·2)+likes) / (ageHours+2)^1.5`;
**Controversy ratio** `(replies+quotes)/(likes+1)`; **thread-depth / unique-interactor** via
`getPostThread` (count distinct `author.did`). Implement client-side in the PWA (scoring fn over the raw
array), via a SkyFeed engagement-sort feed, or via a firehose feed generator (subscribe
`com.atproto.sync.subscribeRepos`, store rolling counters, recompute rank every 15-30 min).

**User:** Ok assume spa/pwa and then think through other forum-style interfaces/controls that reflect native
bsky usage and stats just in the other interface style.

**Assistant:** Map atproto primitives → forum features. **Boards** = custom feed URI or hashtag (tabbed/
sidebar nav running getFeed/searchPosts); **curated categories** = User Lists (`app.bsky.graph.list`);
**thread title** = synthetic (first sentence/line of `record.text`, truncate ~60 chars); **sticky** =
`app.bsky.actor.profile` `pinnedPost`; **mod badges** = Labelers (`com.atproto.label`). Sort control bar:
🔥 Hot (velocity) / 💬 Most Discussed (replyCount) / ⚡ New / 🎯 Ratio. Post cards in compact
(Reddit/HN) vs card (Lemmy) style, comment-count button expands thread in-line. Threaded view via
`getPostThread` (recursive indentation, collapse subtrees, [OP] badge, deep-link to a comment's at:// URI).
Reply root/parent = `reply.root` / `reply.parent`. Upvote = native `app.bsky.feed.like`; **no native
downvote** (track locally or omit); cross-post = quote post; saved = likes or local bookmark.

**User:** Keep thinking it out.

**Assistant:** Dual navigation (Feeds as `/f/`, Hashtags as `/t/`, Lists as `/c/`, meta-feeds `/f/all`,
`/f/popular`). Advanced controls: timeframe + sort dropdowns; client-side sort switch
(MOST_DISCUSSED/HOT_VELOCITY/CONTROVERSIAL/TOP_ENGAGEMENT/NEW). Handle atproto limits: `getPostThread`
`depth` param + "load more replies" lazy deeper fetches; `#blockedPost` placeholders → render "[Comment
unavailable]"; `RichText.detectFacets` for links/mentions/hashtags when composing.

**User:** Great, keep going through core forum challenges — performance, state, custom lexicons,
moderation/community.

**Assistant:** Infinite scroll + IndexedDB cache (TanStack Query + Dexie, keyed feedUri+timeframe+sortMode;
cursor pagination). Long-form: read-more truncation, `RichText` facets, inline media/embed handling
(`app.bsky.embed.video`, external). **Lexicon choice**: pure `app.bsky.*` = 100% interoperable (comments
visible in the main app) but ~300-char + plain styling; **custom lexicon** (`net.myforum.*`) = unlimited
length/metadata but siloed unless cross-posted. Recommended: stick to `app.bsky.feed.post` for reach, embed
forum metadata in text/facets. Moderation via Ozone/Labelers (`com.atproto.label`), user mutes/blocks
(`app.bsky.graph.getMutes`/`getBlocks`), respect thread-author `app.bsky.feed.hideReply`.

**User:** What if we rendered standard.site lexicons for long-form content? Keep going.

**Assistant:** `site.standard.*` (Standard.site) bridges microblogging with long-form publishing.
`site.standard.publication` = a blog/forum channel → Board header; `site.standard.document` = full article →
thread/main-post body; `site.standard.graph.recommend` = endorsement → upvote; `site.standard.graph.
subscription` = follow → subscribe-to-board. Unify long-form doc + micro reply-tree: write the article as a
`site.standard.document`, simultaneously create an `app.bsky.feed.post` "comment anchor" (teaser + link)
that reply-trees attach to → standard Bluesky users see a post-with-link, forum PWA users see a
Medium/Substack-style article with an embedded Reddit-style comment tree. Hybrid ranking adds
`recommendCount` and a long-form content weight. Route by `record.$type`; local draft autosave in
IndexedDB; cross-platform portability (Whitewind, Leaflet, docs.surf).

**User:** Ok, review the bsky atproto network attributes and abilities and keep overlaying.

**Assistant:** Overlay the network: **Relays/AppViews/Firehose** — connect the client (or a custom indexer)
to a Relay firehose (`com.atproto.sync.subscribeRepos`) for live comment updates, efficient repo sync
(`com.atproto.sync.*`). **Identity/auth** — DIDs (`did:plc`/`did:web`) permanent, handles mutable →
key everything by DID; atproto OAuth + granular scopes; web-based PDS signup in-app. **Moderation** —
Ozone Labelers + mod lists + mutes/blocks, pluggable client-side (subscribe labelers → hide/blur/collapse/
boost). **Multi-lexicon mesh** — render `app.bsky.feed.post` + `site.standard.document` + custom
`net.forum.poll` via a lexicon renderer registry; graceful degradation to `record.text`. Full stack:
React/Svelte PWA + `@atproto/api` + AppView reads + Relay WS + PDS OAuth writes + client-side sort worker.

**User:** Good, keep going, think through UX and mobile-first optimization.

**Assistant:** Mobile IA: bottom nav (Home/Feeds, Discover/Search, Compose FAB, Activity/Inbox, Profile/PDS);
horizontal swipe tabs for sort modes; auto-hiding sticky filter bar. Feed cards: separated tap zones
(body→thread, comment badge→comments, handle→profile drawer); swipe-right upvote, swipe-left save. Thread:
limit indentation to 3 levels on small screens, tap-to-collapse, floating "jump to next root comment".
Long-form reader typography + progress bar; lazy media. PWA integration: Web Share Target, offline-first
(Dexie + SW), Background Sync for offline drafts, app shortcuts. Perf: virtual lists
(`@tanstack/react-virtual`), skeleton UI, Web Worker for scoring/sorting.

**User:** Good, keep thinking.

**Assistant:** Unified cross-lexicon notification inbox (aggregated grouping, deep-link routing, parent-
context). Multi-account switcher (encrypted session tokens in IndexedDB via `@atproto/api` OAuth persistent
clients; in-context posting identity; role-based mod-action unlock). Developer plugin architecture: a
`lexiconRegistry` Map (`$type` → component) with a `DefaultFallbackCard`. End-to-end system blueprint (PWA
client → ATProto repos/DIDs/OAuth + moderation/labelers/mod-lists).

**User:** Spa/pwa only.

**Assistant:** Pure client-side, zero backend. AppView CORS unauthenticated reads (`public.api.bsky.app`);
authenticated writes via PKCE-OAuth straight from browser to the user's PDS
(`app.bsky.feed.post`/`like`/`site.standard.graph.recommend`). In-browser sort engine in a Web Worker
(filter by timeframe, sort by MOST_COMMENTS/HOT). Compact feed view + slide-over/drawer thread viewer
(closing leaves scroll position). Offline: SW app-shell cache + Dexie post/thread cache + Background-Sync
outbox. Stack: React (Vite) / SvelteKit + Tailwind + `@atproto/api` + TanStack Query + Dexie + Web Worker +
Workbox + host on GitHub Pages/Vercel/Cloudflare Pages ($0).

**User:** Good keep going. [several "keep going" turns — condensed]

**Assistant:** Optimistic UI mutations (TanStack Query onMutate/rollback, 0ms). Deep linking & client-side
routing (`/f/`, `/t/`, `/u/`, `/post/at://did/collection/rkey`; URL→AT-URI conversion). PWA protocol
handlers (`protocol_handlers` `web+at` in manifest) + Web Share Target. Client-side community themes via
`site.standard.theme` → inject CSS custom properties. Then: zero-backend local search (MiniSearch/FlexSearch
in-memory inverted index; instant typeahead), dynamic thread-graph visualization (D3/React Flow), reading-
history/unread markers via IntersectionObserver→IndexedDB, multi-PDS session management + anonymity (DID
resolution via `plc.directory` or `/.well-known/did.json`; password never touches a third-party server),
offline drafts + auto-recovery. Then: zero-backend local analytics, client-side rate-limit mitigation
(p-queue, exponential backoff on 429 `retry-after`, batch `getPosts`), local data export/portability
(WebRTC/QR device sync; CAR export via `com.atproto.sync.getRepo`), client-side moderation overlays
(layered filter pipeline).

**User:** Block lists can be shared right? Moderator mirroring — pulling in new ones from other mods and
saving them, letting other users pull.

**Assistant:** Yes — natively `app.bsky.graph.list` with a `modlist` purpose + `app.bsky.graph.listitem`
entries; subscribing mutes/blocks all DIDs in the list. Client-side aggregation/mirroring: passive
"follow/subscribe" (fetch a mod's list via `getList`, treat DIDs as blocked for the viewer) and active
"consolidated mirror" (diff external source lists vs your own, batch-write new `listitem` records to your
PDS). Web-of-trust: consensus thresholds ("block if on ≥2 of 3 subscribed lists"), background sync, audit
trails (each `listitem` has timestamp + author DID).

**User:** Keep going? What are we missing? [production-edge-cases turn]

**Assistant:** PDS bloat/quotas (thousands of `listitem` records → chunk into a custom
`net.forum.modlist.chunk` storing arrays of ~500 DIDs). Cross-device read-state without a server (a custom
`net.forum.user.readState` PDS record). Cold-start AppView performance (empty engagement counters on brand-
new feeds → client-side thread-probing `getPostThread depth:1`). Rich embed security (sandboxed iframes
`sandbox="allow-scripts allow-same-origin"`, prefer `app.bsky.embed.external` pre-sanitized cards). Sticky
announcements (`pinnedPost` + client-side pin extraction). Full feature checklist table.

**User:** (1) fair but also this is on top of existing robust protections like local user blacklist which we
should already honor and keyword exclusions and all the default-app-design stuff. (2) single client for now.
(3) we have to make the experience work with populated timelines even if a little slower — today or last 6
hours, maybe 3 at best.

**Assistant:** Layered moderation, native protections FIRST: Layer 1 native mutes/blocks
(`getBlocks`/`getMutes` → silent drop at parse stage before DOM/sort), Layer 2 local keyword/hashtag
exclusions (hide/collapse with a "[Muted Keyword]" placeholder), Layer 3 community/web-of-trust lists
(flag/collapse). Single-client: no multi-PDS CRDT sync; all local state in IndexedDB; instant boot from
local storage. Dense timelines (N=3-6h): rolling-time-window fetch (batches of 50 until crossing the cutoff,
safety cap ~250 posts); compact-list default; for short windows weight `replyCount`/`quoteCount` over
time-decay; virtualize with `@tanstack/react-virtual`.

**User:** Ok what else needs figuring out?

**Assistant:** Live firehose updates without reorder-jank (don't auto-reorder; show a "12 new comments •
click to refresh" pill). Title-extraction edge cases (empty text/media-only → "[Image Post]"; first
sentence/line; truncate ~80). Empty/low-engagement fallback (secondary tie-break reposts→likes→newest).
Media perf & data-savers (lazy images, IntersectionObserver muted autoplay, low-data text-only toggle).
Dual-link share sheet (forum link + native bsky link). Then: comment-tree depth pagination
(`getPostThread depth`, "show N more replies"), DOM memory bloat (virtualization + object-URL revoke on
unmount), graceful error/stale states ("[Comment deleted by author]", offline banner), RichText facet
parsing (`RichText.detectFacets`). Then: preserve scroll (keep-alive overlay routes), iOS WebKit handling
(safe-area insets, `overscroll-behavior-y: none`), a recommended Vite + src/ codebase structure.

**User:** How about rust and wasm?

**Assistant:** Rust→WASM for the hot paths (JSON parse, title/regex extraction, layered moderation filter,
in-memory search index, velocity sorting) in a Web Worker; keep React/Svelte for UI. `wasm-bindgen` +
`serde-wasm-bindgen` (or SharedArrayBuffer/TypedArrays to avoid marshalling overhead). Crates:
`aho-corasick` (multi-keyword ban), `tantivy`/minisearch (in-memory search), `linkify`/
`unicode-segmentation` (title parsing), `ring`/`k256` (client crypto). `rayon` + Web Workers for
multithreading (SharedArrayBuffer). Feasibility rating **9.2/10** — data access 10/10 (CORS open AppView, no
keys), stack 9.5, hosting 10 ($0 static), mobile/PWA 8.5 (iOS WebKit quirks), perf envelope 9 (3-6h window
caps ~100-300 items, <1ms WASM). Only real risks: WASM-boundary marshalling (pass TypedArrays) and cold-feed
AppView latency (skeleton + background hydrate). Greenlight.

**User:** What else could we do here that would really push the experience but still be great?

**Assistant:** (1) Direct browser **Jetstream** live subscriptions
(`wss://jetstream1.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post`) — live-inject into
the timeline. (2) Client-side sentiment/"vibe" meter (tiny quantized WASM model → thread vibe badges). (3)
Progressive offline-first indexing (persistent Tantivy/MiniSearch in IndexedDB → offline full-text search).
(4) Client-side custom embed parsers (YouTube/Spotify/SoundCloud sandboxed iframes). (5) Keyboard-first
desktop shortcuts (J/K, Enter, Shift+↑ upvote, `/` search).

**User:** 4 and 5 there are esp great, 4 even needed. What else? Other bsky idioms to tie in? Your activity
in each interface still translates.

**Assistant:** Map bsky culture into forum idioms with 1:1 network translation: quote posts → crossposts;
Lists → multireddits/curated boards; Starter Packs → "Join this Community" board onboarding; Labelers →
flair/auto-mod; and a bi-directional 1:1 mapping table (upvote→`app.bsky.feed.like`, comment→
`app.bsky.feed.post` reply, new board post→top-level post with hashtags, save→local/pinned AT-URI). The PWA
is a custom lens on the open network — start a discussion in the forum, read replies on iOS, jump to the
official app without missing a reply.

**User:** Awesome, keep going, think about user niceties.

**Assistant:** Apollo/RES-grade QoL: color-coded thread lines + tap-to-collapse-with-haptics + floating
jump-to-next-root + scroll anchoring; lightbox swipe-to-dismiss + inline muted autoplay + PiP; OLED true-
black + reading-density toggle + adjustable typography; customizable swipe actions + long-press peek-and-pop
+ `navigator.vibrate` haptics; stale-while-revalidate status bar + instant draft restore + native share
target.

**User:** Great, deep linking for content, comments and others? Keep going.

**Assistant:** URL routing table (`/f/`, `/t/`, `/u/`, `/p/:did/:rkey`, `/p/:did/:rkey/comment/:commentRkey`,
`/l/:did/:listRkey`). Targeted comment deep-link (fetch thread `depth:10`, traverse to target rkey, auto-
scroll + highlight glow, dim parents, sticky "jump to parent/OP"). `web+at://` protocol handlers +
client-side resolver. Dynamic client-side OpenGraph meta injection for external preview cards. Unified
"copy share link" menu (forum link / bsky link / comment-direct / raw AT-URI).

**User:** Awesome, what else are we missing?

**Assistant:** Web Push (SW + Background/Periodic Sync polling `app.bsky.notification.listNotifications`,
high-priority filter for direct replies/mentions). DID/handle resolution caching (DID-first internal model,
in-memory handle resolver, handle-change protection). Automated storage eviction (LRU; request
`navigator.storage.persist()`; monitor `navigator.storage.estimate()`). Bandwidth/CDN media optimization
(`cdn.bsky.app` variants: `feed_thumbnail`/`avatar_thumbnail`/`feed_fullsize`). Silent OAuth token refresh
(store encrypted refresh token, proactive refresh, read-only cached-mode fallback).

**User:** We should fetch from the cdn directly? And it's fair to consider this very usable but a limit for
mass of local history is fine — purge older contents faster, be pragmatic.

**Assistant:** Yes — AppView JSON already returns fully-qualified pre-signed CDN URLs (`thumb`/`fullsize`);
bind `<img src={thumb} loading="lazy">` in lists, swap to `fullsize` on lightbox. Pragmatic LRU: cap ~25MB /
~1000 threads, 24h expiry for unbookmarked posts, purge on boot/after fetch (delete old + orphaned
comments). Benefits table (image latency <50ms edge CDN, storage locked <25MB, IndexedDB <2ms, instant
launch).

**User:** What kinds of reddit-style top-N and type queries would translate and be performant?

**Assistant:** Reddit-sort equivalents: Top(24h) = `searchPosts(since=24h, sort=top)` (API-driven); Hot =
searchPosts + WASM gravity score; Most Discussed = WASM sort by replyCount; Rising = <1h high reply/like
ratio; Controversial = R/L>1.5. Five best-performing queries (top-N discussions in past X hours; hot
media/crossposts via `domain:`/`url:`; community multi-board; sub-thread deep-dive `getPostThread depth:6`;
rising velocity). Execution bounds: cap `limit=100` (≤2 pages/200 items), WASM 3-pass (moderation →
dedup+time-window → sort), virtualized render. ~400KB memory / <0.5ms WASM / ~80KB compressed JSON per
fetch.

**User:** I feel like on reddit you need a great new feed; here we are a view onto other activity and we
don't have as much of the seed problem.

**Assistant:** Spot on — this shifts the problem from *community building* to *signal curation*. A board is a
saved query, not a blank DB table: "Create Board → Query Network → 200 live threads → instant activity." No
cold start; a post through the forum UI publishes a standard `app.bsky.feed.post` with the tag → the whole
network's replies aggregate back into the forum thread → global reach, forum organization. Eliminates
ghettoization + moderator cold-start burnout; instant value for hyper-specific personal boards; the secret
sauce is converting the microblog firehose into a structured forum via client-side WASM.

**User:** Yes with this framing keep going.

**Assistant:** Zero-cold-start "instant communities" (a board = an aggregation filter over the firehose,
rendered <500ms). Long-form "article submissions" via `site.standard.document` (native in-app reader +
standard atproto reply thread). "Custom lens" per-user algorithmic rules (local WASM slider weights: Debate
Finder / Deep Dives Only / Media Gallery). Seamless crossposting (multi-hashtag → appears on multiple
boards, unified underlying thread). Read-only guest experience (CORS `public.api.bsky.app`, login only to
interact). Summary table (data ownership / cold-start / reach / algorithm control / infra cost).

**User:** This means a flexible follow-style reader interface is effectively free — importing a feed or query
or just curating a local view.

**Assistant:** Precisely — no feed-ingestion backend, no crawler, no DB. A "feed"/"view" is a JSON config in
IndexedDB (or a lightweight PDS record); load it via a direct `fetch()` to the public AppView/search; WASM
sorts <1ms. Unlocks modular "smart views" (composite boards), a long-form newsletter/RSS reader (via
`site.standard.document`), portable view configs (share a board as a URL), 500 custom feeds with $0 infra.

**User:** Yes, awesome keep going, really think about what we can do that is greater than Reddit. We can
prioritize users in a way reddit cannot.

**Assistant:** Because it's a client-side lens (no ad incentive), prioritize agency/focus/time. Walk three
scenarios — long-form deep-dive (unified reader + docked comments vs Reddit's webview-eviction), the "quiet
night" (strict 3-6h window, finite feed, local vibe/density filters vs doomscroll), power search/research
(multi-hashtag aggregation + local WASM indexing + velocity sort). Four structural wins: monetization drive,
data ownership, algorithmic control (transparent client-side WASM), community creation (instant density).
Positioning: a **High-Signal, Zero-Noise Discussion Reader**.

**User:** You could still follow ppl, see feeds on more content types, and a deep discussion view that is a
public back-and-forth — announce/pin-style comms on a topic. Ok so we looked at aligned/translated idioms —
what great things can we do with a custom lexicon, nsid, or PDS functionality? And how do we make the PWA as
good as possible with local content but backup and safety — back up settings to the PDS, even encryption?

**Assistant:** Custom NSIDs beyond `app.bsky.feed.post`: **`app.forum.announcement`** (title/body/isPinned/
targetBoard → prominent pinned header cards, with an `app.bsky.feed.post` cross-reference for interop);
**`app.forum.discussion`** (turn-based debate/interview layout, audience Q&A in a side-drawer). PDS state
sync: **`app.forum.userSettings`** (layout density, default window, custom boards) fetched on OAuth login →
instant cross-device hydration. E2EE for local state: Web Crypto AES-256-GCM before pushing to
**`app.forum.encryptedVault`**; key derived from DID + passphrase or WebAuthn/Passkey; PDS stores only
opaque ciphertext.

**User:** We could do an AMA and pin the AMA participants. And keep going — user-first features. I'd love a
forum that is only my mutual follows, and they can quietly hide each other if there is contentious crossover.

**Assistant:** **`app.forum.ama`** (hosts[] DIDs, start/end time) → host answers sticky-pinned with [HOST
ANSWER] badge, Q&A-mode toggle (only host-answered top questions), all-questions mode. **Mutuals-only board**
(fetch follow+follower lists on boot → in-memory mutuals Set; WASM discards non-mutual authors). **Quiet
soft-hiding** (`app.forum.quietHide` local record: "hide @mutualA on /f/politics", "collapse subthreads where
@mutualB replies to @mutualA"; zero public signal, no notification). More: Ghost Threads (IndexedDB
autosave + encrypted PDS `app.forum.drafts`), Discussion Time-Capsules (freeze thread → offline HTML/JSON),
noise-gated notification inbox.

**User:** Look at the current reddit site and user experience and come up with your top 15 ideas for doing it
better for users in our different model.

**Assistant:** 15 ideas — (1) Finite Feed (anti-doomscroll, rolling window, "you're caught up"); (2)
zero-latency WASM vibe/density sliders; (3) quiet soft-hiding; (4) native in-app reader for long-form
(`site.standard.document`); (5) instant curation-over-creation boards; (6) transparent open-source algorithm
presets; (7) interactive media + custom embed sandboxes + PiP; (8) Apollo-style gesture/thread nav polish;
(9) native AMA mode with pinned host answers; (10) multi-board feed synthesis (free multireddits); (11)
complete offline reading + local search; (12) portable view configs (zero-server share links); (13)
encrypted PDS state sync (zero-knowledge); (14) Ghost Threads + auto-draft recovery; (15) read-only
frictionless guest experience. Summary contrast table.

**User:** Great list, can you see what users are requesting / most complained about that we can do better?
No repeats.

**Assistant:** (1) accidental swipe/tap-loss recovery (layered viewport stack + shake-to-restore); (2)
zero-state & pre-populated search bars (persistent query state + local history); (3) native OP-comment
highlighting & author trackers ("OP Only" filter, jump-to-author bar); (4) direct CDN edge media vs Reddit's
buggy transcode (native HTML5 + IndexedDB pre-cache); (5) multi-account zero-reload context switching
(client-side OAuth session registry).

**User:** How can we integrate creating content in a friendly native way by content type? What user
reactions has reddit gotten rid of that users miss? What types of content can we effectively render?

**Assistant:** (1) Modular composer by intent (Quick Discussion `app.bsky.feed.post` / Long-Form
`site.standard.document` / Pinned `app.forum.announcement` / AMA `app.forum.ama`), each with friendly
touches (auto board-context tags, live markdown preview, companion teaser post, host-selection wizard). (2)
Missed Reddit reactions restored as local/zero-cost: free custom awards via Labelers; crosspost indicators;
custom community upvote icons; granular controversial/Q&A sort. (3) Renderable content: code snippets +
LaTeX; long-form essays; multimedia galleries; interactive web widgets (Spotify/YT/SoundCloud sandboxed);
structured data cards (GitHub/ArXiv/polls/events).

**User:** Do more research on features reddit has deprecated to great user complaint (enshittification).

**Assistant:** Restorable deprecated features: (1) chronological "Following" + custom-feed isolation
(zero recommended injection); (2) full granular comment sorting (Controversial/Old/Q&A/Depth); (3) native
RSS/JSON export (client-side generator, no proxy); (4) customizable "next comment" jump-button position;
(5) unrestricted multireddits/feed limits (infinite IndexedDB folders); (6) public upvote/downvote & karma
visibility settings; (7) legacy compact-text layout; (8) non-destructive multi-tab search (slide-over
drawer); (9) transparent moderation logs (labeler transparency). Contrast table.

**User:** We could do likes as upvotes and make a downvote lexicon and allow ORing both.

**Assistant:** Dual-write: upvote = standard `app.bsky.feed.like` (visible network-wide, contributes to
global algos); downvote = custom **`app.forum.feed.dislike`** (subject uri+cid; ignored by standard clients
→ no demotivating notification; PWA indexes it locally, `Score = Likes − Dislikes`). Three score modes
(user/board choice): A Reddit (Likes−Dislikes), B Positive-Only (likes only, dislikes ignored), C
Controversial (high dislike ratio / R/L). Benefits: zero lock-in, no dislike-spam on the main firehose, local
privacy over who downvoted.

**User:** Users can decide if downvotes or shaping and account activity sync to PDS or stay local entirely.

**Assistant:** Three storage tiers, per-feature: **A Local-Only** (IndexedDB, zero network trace, no cross-
device); **B PDS Encrypted Sync** (AES-256-GCM via WebAuthn/Passkey or master passphrase → `app.forum.vault.
encrypted`, cross-device zero-knowledge); **C PDS Public Records** (plaintext lexicons, network-indexable).
Concrete use cases (downvotes private-curation vs public-accountability; board shaping personal-lens vs
shared-community; soft-hiding/mutual-mutes must stay Local-Only or Encrypted). Graceful migration ("encrypt
& sync" / "export JSON"), guest-mode accumulation → one-tap PDS hydration on login with encryption choice.

**User:** Give me more user-experience rollbacks and start thinking through UI reflecting Reddit's peak-UX
era and better but still simple in that 2010s best-of-web-2.0 way.

**Assistant:** More rollbacks: (1) native "open in true new tab" (semantic `<a href>` middle-click); (2)
local "mark as read" + dimmed thread states (IntersectionObserver→IndexedDB); (3) true one-tap "hide post"
(instant local array removal, no layout shift); (4) custom CSS/theme injectors (JSON/CSS-var manifest,
sandboxed, no arbitrary script); (5) deterministic "oldest first"/linear-time reading modes. Peak Web-2.0
UI: high-density compact layout, native system-font stack (zero CLS), 8px padding, 15-16px semibold titles,
12px muted meta; compact-text vs card toggle; classic light (#FFFFFF surface / #FF4500 accent) / slate / OLED
palettes. 3 viewport zones (sticky 44px header / virtualized stream / non-destructive slide-over drawer).

**User:** The big trap is removing genuine UX improvements just because they came after the era, rather than
integrating them in a way that smoothly extends the prior idiom.

**Assistant:** Bridge, don't retro-trap. (1) Modern galleries as classic inline `[+]` toggles (inline
accordion → swipeable pinch-zoom carousel); (2) rich text = WASM WYSIWYG-Markdown hybrid (inline styling,
syntax visible, toolbar for power users); (3) gesture navigation over tiny text links (swipe→action +
haptics, keep desktop text buttons); (4) non-destructive slide-over drawers (`translate3d`, background
mounted); (5) smart thread collapsing with contextual summary pills + locked scroll anchor. Synthesis matrix.

**User:** That's great, give me more of the same.

**Assistant:** (1) saved-search filter → permanent live custom tab (IndexedDB query rule, local+AppView
concurrent); (2) contextual parent-chain preview on hover/long-press (floating card, up to 2 ancestors, no
scroll change); (3) keyboard-first RES/Vim suite (J/K/X/A/Z/C/O/`/`, left-accent focus indicator); (4)
non-intrusive thread-activity badges (`💬 52 (+12 new)` via cached replyCount diff + highlight glow); (5)
multi-column deck view (desktop >1200px, independent WASM virtualized columns + shared inspector drawer).

**User:** Look at reddit enhancement suite and other better clients for ideas users created for themselves.

**Assistant:** RES/Apollo/Sync/Relay/Narwhal features to adapt: user tagging + notes (local IndexedDB /
encrypted `app.forum.userTags`); high-contrast role highlighting ([OP]/[HOST]/[MUTUAL]/[MOD]); seamless
multi-account switcher; RES comment-navigator + sibling-skip; parent-on-hover; rainbow indentation depth;
`[l+c]` link+comments opener; Apollo jump bar (`/` typeahead); Vim keys; hold-to-peek media; scrubbable
GIFs/video speed; filteReddit (local keyword/domain/regex → WASM); hide-read-on-scroll.

**User:** Oh really good list, let's do like 15 more.

**Assistant:** 15 more: (1) share comment-thread as image (html2canvas); (2) auto-expanding parent context in
notifications; (3) mark-read-on-scroll + floating vacuum button; (4) smart domain/sub-feed favicons; (5)
advanced formatting bar + live preview; (6) un-collapsed search in deep trees (WASM raw-tree search + auto-
expand + highlight); (7) per-board default sorting rules; (8) volume-aware muted video (waveform indicator +
fade-in); (9) custom text-expansion snippets; (10) collapse-all-child-comments; (11) readability/dyslexic/
bionic text filters; (12) auto-mute contentious keywords (48h temporary spoiler mutes); (13) high-speed
gallery-grid mode; (14) local read-later offline queue; (15) local private karma/impact analytics.

**User:** Great, that's actually a very achievable risk. Can you keep walking out design guidelines organized
by user experience with an eye towards leveraging newest techniques with 2010s crisp UX?

**Assistant:** Design system by UX area: (1) information density & scannability (native system fonts, tight
rhythm, deterministic layout modes); (2) navigation & viewport preservation (non-destructive slide-over
drawers, semantic `<a href>` anchors, DOM virtualization + scroll-anchoring); (3) high-speed interaction &
power-user controls (Vim keys, haptic gesture mapping, floating comment navigator); (4) content creation &
dual-engine editing (context-aware micro-composer, WASM hybrid markdown, ghost-thread autosave); (5) local
privacy & zero-knowledge sync (three-tier data model, quiet soft-hiding). Architecture summary matrix.

**User:** Good, describe the design more, in user experience terms.

**Assistant:** [Detailed UX-feel walkthrough] high-contrast zero-whitespace hierarchy; tactile feedback &
gesture response (haptic swipe upvote/save/collapse, instant-over-animation); thread reading flow (rainbow
depth, 1-tap branch collapse + summary pill, floating comment navigator); the drawer context switch
(background dims 30%, hardware-accelerated slide, Esc/swipe-away restores exact scroll); content-creation
experience (auto-context composer, WYSIWYG-markdown hybrid, formatting rail, draft-safety indicator).

**User:** Great, keep going.

**Assistant:** Zero-latency state transitions (optimistic UI 0ms → WASM/IndexedDB → background PDS sync);
contextual focus & "Zen reading mode" (auto-hiding chrome, focus dimming, 1-tap zen); high-signal
notifications & "quiet inbox" (parent-context previews, inline quick-reply, batching/noise-gating);
accessibility & inclusive design (WCAG AAA 7:1 contrast, semantic ARIA trees, `prefers-reduced-motion`).

**User:** Can you describe design in technical terms so we can keep consistency across user flows and keep
going?

**Assistant:** System design spec: CSS custom-property tokens (spatial 4px grid, type scale, color themes
light/dark-OLED); layout physics (3 compositor layers, `translate3d` GPU-promoted drawers, cubic-bezier
easing, `overscroll-behavior-y: contain`); atomic component interfaces (TypeScript `PostCardProps`,
`ThreadDrawerProps`); state-management pipeline (Zustand optimistic store ↔ WASM Web Worker ↔ Dexie
IndexedDB ↔ async PDS sync); keyboard-navigation engine spec (KeyMap, `data-focused`, `scrollIntoView`).

**User:** Let's keep going on design specifics — usable, maintainable, and friendly. But also, should we
natively empower theming and custom UI/UX?

**Assistant:** Yes — "Theme-as-Data" (safe): a `app.forum.board.theme` record of pure JSON tokens mapped to
sanctioned CSS custom properties inside a sandboxed scope (no arbitrary CSS/JS → no XSS). User granularity
(strict universal / allow board personality / community theme bazaar via URL import). Maintainability: 4
core layout primitives (Box/Stack/Text/Interactive), isolated feature modules (feed/discussion/reader/
composer/settings). Friendly design language: human micro-copy, non-destructive undo toasts, transparent
status badges, warm empty states, zero-setup guest mode + contextual OAuth prompt.

**User:** What are we not thinking of that we need for a first-class daily-driver experience?

**Assistant:** (1) multi-tab/multi-window sync (`BroadcastChannel`); (2) background sync & subway/flight
transitions (idle prefetch via `requestIdleCallback`, offline outbox, SW Background Sync); (3) instant image
uploads + client-side WASM/Canvas WebP compression; (4) deep-linking & universal URL routing (dual-route
resolution, un-authenticated previews); (5) keyboard/touch accessibility edge cases (text-selection vs
swipe threshold, layered Esc stack); (6) granular notification control (thread-level mute, custom mention
sounds). Then media lightbox/PiP, cache-first SW + ThumbHash/BlurHash placeholders, high-velocity search
(`Cmd+K` command palette), cross-device reading-position memory, graceful degradation + bandwidth-aware
throttling (`navigator.connection`).

**User:** Think about things we need to consider upfront that will only bite down the road if not.

**Assistant:** (1) storage quotas & IndexedDB eviction (`navigator.storage.persist()`/`estimate()`, LRU
>14d); (2) ATProto lexicon schema versioning & migrations (mandatory `$version`, WASM migration pipeline);
(3) DOM node accumulation in deep threads (flatten tree to a 1D array in WASM, virtualize <30 nodes); (4)
rate-limiting & PDS commit batching (300ms debounce write queue); (5) Web Crypto key management & loss
recovery (WebAuthn/Passkey PRF, exportable 12-word recovery). Then: local search index size & WASM memory
(two-tier hot/archive index, worker indexing); DID→handle stale-cache (DID-first + stale-while-revalidate
24h TTL); moderation/labeler stacking fail-safes (WASM bitmask matrix, cached fallback); deep-link cold-start
hydration (single-flight `Promise.all`, instant app-shell). Then the feast-or-famine loading fixes:
watermark prefetch (80% threshold), micro-chunked DOM hydration (`requestIdleCallback`), ThumbHash
placeholders, single-flight request coalescing, stale-while-revalidate, network-aware adaptive throttling.

**User:** What else could be cooler than reddit with a bluesky feature otherwise not possible — go live
integration? What else?

**Assistant:** (1) decentralized "go live" (`app.forum.live.stream` record with WHIP/HLS URL, chat as reply-
tree over Jetstream); (2) open audio rooms (WebRTC mesh/SFU + `app.forum.audio.stage` for stage perms,
auto-archived podcast transcript); (3) live match/event multi-streams; (4) interactive live polling /
reputation-only prediction markets (`app.forum.poll`); (5) multi-user live collaborative whiteboards /
in-browser WASM code execution.

**User:** 2 is a cool idea, predictions is dicey without moderation figured out first. Those are great Phase 2
ideas — what else?

**Assistant:** Phase 2 (no video/prediction risk): (1) cross-network "bridge boards" (ActivityPub/Lemmy +
RSS ingestion, unified threading); (2) client-side AI summarization/TL;DR (WebGPU/WebLLM/Transformers.js
local, zero API cost, debate anchors); (3) open custom algo-feeds marketplace (pluggable WASM/JS sorting);
(4) collaborative curation & multi-maintainer/forkable boards (labeler multi-sig pins, fork a board config);
(5) native AMA/Q&A host mode.

**User:** I was thinking of integrating a Google Reader type thing where you read articles and then post them
to the forum for discussion in one stream, and make posting really just a smooth way to post it for
discussion.

**Assistant:** Unified stream — RSS reader + forum as two views of one stream. "Promote to Forum" = inline
discussion sheet (expands under the RSS card; empty → "be the first"; typing one sentence publishes an
`app.bsky.feed.post` with the canonical URL embed → the RSS item transitions to an active thread app-wide;
if already promoted, shows the live comment count via canonical-embed matchmaking). Three data layers: local
RSS engine (Web Worker XML→IndexedDB), url→AT-URI matchmaker, forum thread layer. Keyboard-first reader-mode
Vim suite. Beats Feedly+Reddit (1-tap start-discussion, unified around canonical link AT-URIs, local RSS
parsing, offline queue).

**User:** Oh that's awesome, and you could post it and request to only see replies to some user scope —
global, mutuals, follows or followers — and then it is the same but as if you can tune your experience even
in place.

**Assistant:** In-Place Scope Tuner — a segment control docked in every thread: 🌐 Global / 👥 Mutuals /
👤 Follows / 🔒 Local. Each transforms the comment-tree filter instantly (WASM graph-set bitmask over the
locally-held social graph; non-destructive — switching back restores all replies, zero re-fetch). Post
creators can *gate* replies via native `app.bsky.feed.threadgate` (enforced at the PDS: mutuals-only rejects
non-mutual reply commits). Contextual scope-shift experience (read → mutuals warm-up → global deep-dive).

**User:** Think on that one and how to integrate it nicely a bit more.

**Assistant:** Five integration patterns: (1) inline discussion sheet (expand under card, zero-submit thread
creation, instant transition); (2) in-place highlight & quote-posting (text-selection popover → composer
pre-populated with `> blockquote` + link); (3) the scope tuner as personal-notes vs public-discussion
(Private Note → IndexedDB/encrypted; Mutuals → threadgate; Public → `/f/`); (4) read-later offline stacking
(full-text readability snapshot + top-30 comments in IndexedDB, offline replies queued); (5) keyboard-first
navigation (J/K/Space/C/Q/S/A/Z/M). Architectural synthesis diagram.

**User:** And you could say "mutuals of mutuals" to define the radius of discussion you personally
participate in, and you can easily tag someone as who they are to you (local only), and mute someone without
any fanfare or notification locally, and poll for updates outside of the stream to scope your experience to
the "small web" or small tree. A tree has a seed; a web is a web really only in aggregate.

**Assistant:** Degree-based radius control (r=0 Personal / r=1 Mutuals / r=2 Extended "mutuals-of-mutuals" /
r=3 Network / r=∞ Global); r=2 is the golden ratio (friends-of-friends, civil because of a shared bridging
connection). Private local-only user tagging (keyed to immutable DID; colored pills only on your screen).
Fanfare-free quiet muting (DID → local blocklist; zero protocol trace, no notification; WASM strips before
DOM). Polling the "small web" (disconnect the firehose WS; scheduled batch fetch every 4h → a finite "Daily
Edition" → "caught up for the day"). "Trees vs aggregates" diagram.

**User:** I am calling this the 'social tree' — what else can we do that reinforces this as a first-class
experience?

**Assistant:** Five "Social Tree" concepts: (1) visual structural thread maps (branch summaries via WASM,
radius canopy trimming, 1-tap branch jumping); (2) "grafting" threads (cross-post a sub-branch to another
board with a bi-directional dual-rooted pointer); (3) "dormancy & seasons" (threads go read-only 48-72h
after inactivity, index to a local Personal Archive, "seed a new tree" to revive); (4) "roots & soil"
(RSS-source trust/soil-quality metrics computed locally, quiet source pruning); (5) the "Personal Garden"
dashboard (utility counters — active trees / dormant archive / local tags — not vanity follower tallies).
Paradigm-shift table (Social Feed vs Social Tree).

**User:** Now take in this whole conversation and look at what we're missing and what we clearly need to do
for design and experience now for the PWA/spa.

**Assistant:** Restates the Social Tree vision (seed → canopy(r) → tree structure). Missed gaps: PWA offline
shell + background sync; local-first graph engine (WASM bitmask radius filter over IndexedDB DID sets); local
privacy & quiet moderation (local mutes never leave IndexedDB; encrypted vault). UX/UI architecture matrix +
4-layer breakdown (unified stream / in-place scope control / visual tree nav + grafting / finite seasons +
personal garden) + technical implementation checklist.

**User:** What else, and what can we settle now that will allow us to build this for real later?

**Assistant:** Settle the foundational contracts: (1) custom ATProto lexicon schemas
(`app.socialtree.feed.threadgate` with a `radius` enum r0/r1/r2/rInf + `allowGrafting`;
`app.socialtree.vault.encrypted` ciphertext/nonce/version); (2) client-side Dexie/IndexedDB schema
(`rssItems`/`posts`/`identityGraph` (DID + distanceDegree)/`userTags`/`offlineOutbox`); (3) WASM worker
contract (`COMPUTE_RADIUS_TREE`/`FLATTEN_TREE_NODES`/`SEARCH_LOCAL_INDEX`); (4) single-view component tree
(`AppShell`→header/stream-virtualizer/thread-drawer→reader/scope-tuner/tree-map); (5) 4-phase dev roadmap.

**User:** Think through building this in these stages with the features we've discussed and then note the
reasoned undecided parts and specify with specific technical and UX/UI.

**Assistant:** Staged plan (P1 Local Reader & Engine → P2 ATProto integration & 1-tap seeding → P3 Social
Tree radius engine → P4 offline sync, vault & personal garden). Reasoned open trade-offs: (A) r=2 graph
resolution at scale — RESOLUTION **hybrid two-tier graph cache** (r=1 fully in IndexedDB; r=2 hydrated lazily
via `requestIdleCallback` / a Bloom filter; UI fallback `[?] Check Degree`); (B) reply gating native vs
custom — RESOLUTION **dual-write** (`app.bsky.feed.threadgate` for broad interop mapping r=1→"mutuals-ish" +
custom `app.socialtree.threadgate` for exact r=2); (C) RSS CORS — RESOLUTION **stateless CORS relay worker**
(a Cloudflare Worker that only appends `Access-Control-Allow-Origin: *`, logs nothing). UX/UI spec (3 zones,
token table, high-density card wireframe).

**User:** You could almost ride the bookmark syncing functionality to transcend devices by using that
mechanism natively.

**Assistant:** [First interpreted as PDS-native sync] The PDS as a sovereign state bus (write bookmark/
readState records → firehose WS → secondary device receives commit instantly). Multi-device handoff +
scroll-memory (`app.socialtree.readState` — rootUri/lastReadCommentUri/scrollPercentage). Native "read
later" cross-client interop (dual-write standard + rich records). Privacy dual-path (public records vs
client-side WebCrypto encrypted vault). Architectural benefits table ($0 DB, native firehose sync, data
sovereignty, Merkle-tree offline replay).

**User:** No I meant integrating literal bookmarks in the local user browser and the various browser-based
sync backends to move offline between devices.

**Assistant:** [Corrected — native browser bookmark sync as transport] Bookmarks as a data pipeline (encode
app state into URL fragments/params inside a dedicated bookmark folder; Chrome Sync / iCloud Safari / Firefox
Sync push it cross-device). Web-platform boundary: a pure PWA CANNOT access `chrome.bookmarks`; two paths —
**A Native Share Target** (`share_target` manifest → "Add to Social Tree" from any share sheet) and **B a
browser-extension companion** (`chrome.bookmarks` permission, `onCreated` watcher on a "Social Tree" folder,
`BroadcastChannel` to the PWA). Bookmark-handoff flow (desktop save → OS sync → mobile → PWA ingestion).
Comparison table (custom cloud vs ATProto PDS vs browser bookmark sync).

**User:** Any other browser sync ability we can leverage for our users directly?

**Assistant:** (1) extension `chrome.storage.sync` (real-time low-latency tunnel; ~100KB total / ~8KB per
key; store active reading pointer / draft-queue state / session-handoff signal); (2) Web App Manifest
`file_handlers` + OPFS sync (register as OS file handler; OPFS desktop-class disk speed; drop an OPFS backup
into a cloud-synced folder to hydrate another device); (3) Web Share Target (`share_target` → OS share
sheet); (4) Web Locks API (`navigator.locks` leader-election: one master tab holds the Jetstream WS +
BroadcastChannel to passive tabs, 0ms failover). Capability-stack table.

**User:** What other primitives do browsers store? Can we allow integration with the user's password store
for login to platforms that require subscriptions/accounts? Let the user create their own world and leverage
tools they have at hand.

**Assistant:** (1) Credential Management API (`navigator.credentials.get({password:true})` → native password
manager/Keychain → unlock paywalled RSS/paper session tokens; `store()` to save the PDS app-password); (2)
WebAuthn PRF extension (hardware-backed deterministic seed → AES-256-GCM key to decrypt the local vault;
Passkeys sync via iCloud/Google); (3) W3C Digital Credentials API (`navigator.credentials.get({digital})` →
OS wallet verifiable credentials, selective disclosure / zero-knowledge; e.g. `/f/journalism` proof-of-press
without real name); (4) File System Access API (export/import offline archives); Clipboard API; Periodic
Background Sync. "Sovereign client stack" diagram.

**User:** This could allow viewing modes that relate to VC creds — attested as a person by xyz as an
offering.

**Assistant:** Sovereign attestation-based viewing modes. Boards define attestation prerequisites; users
present ZK proofs locally to unlock views/post privileges. Modes: "Accredited Canopy" (r_attested — a filter
tier beside r=1/r=2); vouched-personhood (humanity proofs — mDL/World ID boolean claim); peer-attested
web-of-trust (any org as VC issuer, e.g. `app.socialtree.attestation.contributor`). Technical flow
(`navigator.credentials.get({digital})` → OS wallet selective disclosure → client-side WASM ZK-proof
verification against issuer DID → ephemeral local `isAttested` token). Ephemeral thread-bound badges (zero
cross-thread correlation). Identity-tier matrix.

**User:** I think native apps would follow potentially, but the PWA should be the floor for the use case.

**Assistant:** PWA as the floor (open web standards, no gatekeeping, can't be de-platformed). Baseline
capabilities + defensive strategies (WebKit storage pressure → `storage.persist()` + PDS/exported-file as
source of truth; feature-detection matrix + graceful degradation to social-graph attestation when
`DigitalCredential` unsupported). Native-app progression (Swift/Kotlin wrappers add background refresh,
Secure-Enclave/Keystore, native wallet). Shared "WASM Core" (Rust graph/ZK/search/migration logic compiled
once, run in the PWA Web Worker AND native via FFI). Zero lock-in (data on PDS + local IndexedDB).

**User:** Let's think about real platform design with integrated components and how to map primitives from
one domain into another — likes as upvotes and saved-for-later as bookmarks in the browser itself.

**Assistant:** Cross-domain primitive map (Social Tree feature × ATProto lexicon × browser primitive × local
hardware/state). Deep dives: likes → upvotes + global signal (`app.bsky.feed.like`); saved-for-later →
dual-mode bookmarks (browser bookmark tree via extension + local IndexedDB offline snapshot); crossposting →
branch grafting (`app.bsky.feed.repost` + custom `graftContext`); paywalled content → native Credential
Management retrieval; moderation/verification → Digital Credentials API ZK-VCs. Unified three-tier local
system (Browser/OS layer / local PWA client engine / decentralized protocol layer).

**User:** What else? (Search, Economics, Lock-in.)

**Assistant:** (1) zero-cost local semantic search (WebGPU + Transformers.js MiniLM embeddings → local
IndexedDB vector cosine search over your Personal Garden + r=2, offline, zero external AI); (2) frictionless
value exchange (W3C Web Monetization / Interledger streaming to RSS authors; micro-payment paywall bypass;
peer tipping); (3) the "glass escape hatch" — total portability (OPFS → OS filesystem 1-tap "Export Personal
Garden" as Markdown; native PKM integration — Obsidian/Logseq/Notion). Complete-stack table.

**User:** I've seen snippets of this but not fully formed in any direction — the pieces are all there, I
wonder why that is.

**Assistant:** Four structural reasons no one has assembled it: (1) incentive inversion (VCs won't fund a
product that structurally prevents attention/data monetization — zero data moat, zero server compute, finite
engagement); (2) the native "app store" blindspot (teams default to native, ignore bleeding-edge Web
Platform APIs — OPFS/Web Locks/WebAuthn PRF/Digital Credentials only recently matured); (3) the "unbundling"
phase of ATProto (isolated single-purpose tools; now reaching a re-bundling phase); (4) the high barrier to
local-first engineering (WASM/Rust + cryptography + distributed systems in one team). The paradigm is
shifting; it just needs a team willing to cross the domain boundaries.

**User:** Ok what are we missing? [operational realities]

**Assistant:** Four critical missing infra pieces: (1) the "Jetstream chasm" — a browser can't hold the raw
global firehose → deploy a lightweight stateless **Social Tree AppView** (Rust indexer that pre-digests
Jetstream, indexes only `app.socialtree.*` + canonical URL embeds, serves paginated tree slices; WASM still
does r=2 + ZK locally); (2) composable moderation & spam (`com.atproto.label` labeler integration — blur/
drop before DOM); (3) split-brain offline sync (CRDTs — Yjs/Automerge + Hybrid Logical Clocks merge, never
overwrite); (4) push notification delivery (zero-knowledge Web Push relay — stores subscription + r=1 DID
whitelist, wakes the SW). Adjusted blueprint (client sovereignty + delegate heavy aggregation to atproto
infra).

**User:** What else?

**Assistant:** Security/authorization/privacy layer (enabled by recent updates): (1) granular ATProto OAuth
**auth scopes** (read/write only `app.bsky.feed.post` + `app.socialtree.*`, deny DMs/blocklists — a crypto-
enforced sandbox); (2) cross-device wallet handoff via CTAP/BLE (Digital Credentials API "stable in Chrome
141 and Safari 26, late 2025"; QR + BLE handoff to a phone wallet; OpenID4VP + ISO mdoc 18013-7); (3) native
E2EE and group-private data (claims "ATProto's latest architecture introduces native Privately Shared Data
and E2EE DMs"; r=1 mapped to a "Group-Private Data Enclave" never broadcast to Jetstream). Final synthesis.

**User:** I might have other ideas for the private-comms-on-a-public-backplane problem — discuss a public
article in a private room and have it feel equal to the public experience — and I'm hesitant to bend the
encrypted content in the PDS too far atm.

**Assistant:** Justified hesitation — a PDS is a public Git-like repo; E2EE blobs on a public ledger make one
future key-compromise catastrophic. Decouple identity from transport: (1) "sidecar" protocol (ATProto DID as
SSO/identity; route the actual private messages over MLS or Matrix; the RSS canonical URL is the room
subject; UI identical to the public thread); (2) "Dark PDS" (a non-federating ATProto server = a private
Discord-like instance; write standard posts to the shared Dark PDS, not the personal public one; rely on TLS
+ access control, no custom encryption); (3) ephemeral WebRTC (DID handshakes exchange signaling tokens →
direct browser-to-browser data channels → zero-storage room that evaporates on tab close). The UI illusion:
public backplane and private room as interchangeable TABS on the same object (same renderer/parser/
shortcuts; the article's canonical URI is the anchor; query public AppView + sidecar private transport
concurrently).

**User:** An online-signal and WebRTC-supporting infra is interesting potential for a helper ethical
commercial view.

**Assistant:** The "ethical commercial helper" model — sell utility (bandwidth/latency/relay reliability),
not attention/data; WebRTC is E2EE by default (DTLS-SRTP), so the relay operator can't read payloads. The
infra problem (NAT-traversal fails ~15-20%; mesh topology wall beyond 4-5 peers) → legitimate demand for
TURN relays + SFUs. The "Privacy-Pass" anonymous metering model (blind signatures / RFC 9152 — buy a token
pack via Stripe → cryptographically blinded usage tokens unlinkable to sessions). Tiered offerings (free
floor: open STUN + P2P mesh <4 peers + self-host signaling; ethical helper $3-5/mo: global TURN edge relays,
high-capacity SFUs, E2EE SFrame media, dedicated signaling). Why it works long-term (aligned incentives,
zero lock-in via open WebRTC standards, legal defense as a stateless encrypted utility relay).

**User:** This is really the whole vision — users pay a small fee for real bandwidth and low-latency servers
while retaining 100% ownership of their identity, data, and social graph.

**Assistant:** It flips the software model back to its original promise — treating people as customers buying
a high-quality tool rather than products farmed for attention. Decouple utility (relays/bandwidth/sync) from
state (identity/graph/annotations); a clean honest contract. The convergence of ATProto/WebRTC/WASM/OPFS/
Digital Credentials means this is a viable, executable engineering roadmap.

**User:** One of my tenets is: machines can compute provenance, never utility. Partially because humans can
only discern value, and value and worth are both subjective and travel together.

**Assistant:** That tenet diagnoses why algorithmic feeds feel alienating — computing "utility" substitutes
measurable proxies (engagement/outrage/CTR) and flattens human experience. Strict boundary: **the machine's
domain = provenance** (cryptographic signatures / ZK proofs / packet routing / graph mathematics — the
guarantee of *what happened, who did it, how it reached you*); **the human's domain = utility** (subjective
labeling via private local tags that never leave IndexedDB; relational context via r=2; source trust). The
ultimate separation of concerns: mixing the two in a central DB = the modern web (algorithms deciding what's
true based on what's popular). Sell the provenance engine (the encrypted pipe); the user brings the utility.

**User:** Yes. But critically create the bridge to the big world at user discretion — that's the tie-in to
the interconnected-trees view as critique, yes, but the seed grows outward first and foremost.

**Assistant:** Corrects the inverted model (traditional social starts at r=∞ and makes you shrink). The
Social Tree mirrors organic growth: every interaction begins as a local seed (r=0) and grows outward; the
big world is a forest of other trees you *bridge* to. Outward-growth progression table (Seed r=0 IndexedDB/
vault → Roots r=1 shared-enclave/WebRTC → Canopy r=2 custom threadgate → Bridge r=∞ standard post).
[CLAIMED but self-corrected next turn] the discretionary bridge via lexicon separation — writing to
`app.socialtree.garden.note` is "invisible to the algorithmic big world"; "Graft to Global" dual-writes to
`app.bsky.feed.post`. Interconnected trees (inbound foraging / outbound grafting) protect from context
collapse.

**User:** Definitely not invisible at all — `app.socialtree.garden.note` … "it exists on your server but is
invisible to the algorithmic big world." The graft analogy is grand though.

**Assistant:** [Accepts the correction — a critical catch] The `com.atproto.sync.subscribeRepos` firehose
does NOT care about lexicon namespaces; any unencrypted record written to a public PDS is broadcast in
plaintext to every scraper/indexer. The AppView may ignore it (not in the main feed) but the network sees
it 100%. So Seed (r=0) & Roots (r=1) CANNOT live on the public PDS in plaintext — they live in local
IndexedDB, and if synced across your own devices they are WebCrypto AES-256-GCM encrypted first
(`app.socialtree.vault.encrypted` — opaque to the firehose). The "Graft to Global" is a literal
cryptographic threshold: pull the private local thought out of IndexedDB/decrypt → translate to a plaintext
`app.bsky.feed.post` → commit → THAT is the moment it hits the firehose. (Re-derives invariant S5.)

**User:** What else?

**Assistant:** Elevate from "social app" to a true User Agent: (1) the "browser of lexicons" (sandboxed
extensibility — unknown lexicons render via a decentralized registry of Web Component renderers in a Shadow
DOM/sandboxed iframe; the network distributes the data AND the UI); (2) sovereign AI (WebNN/WebGPU local
small-parameter LLM nightly synthesizes the unread queue against a private local vector profile → a morning
briefing; reads r=0/r=1 without leaking a byte); (3) the cold-start bridge / universal namespace (normalize
Substack RSS / YouTube / Reddit JSON / atproto DID into one local data structure; seamless grafting across
the legacy web and atproto). Capstone: a fortified, encrypted, intelligent OS for your digital life.

**User:** The other killer feature is native user time-limits and self-control/awareness functionality as a
totally optional but integrated component — something they literally will not mimic unless forced.

**Assistant:** Ad-driven platforms structurally cannot (revenue = impressions × time). Because the model is
utility/infra-based, reducing mindless scroll lowers bandwidth cost and raises retention → time management
is a native design choice. (1) the "Finite Edition" (discrete morning/evening batches, feed literally ends,
DOM unmounts the stream); (2) scope-based time budgets (cap r=∞, uncap r=0/1; client-enforced friction gate
unmounts the global feed when the budget expires); (3) private local telemetry (creation-vs-consumption,
radius distribution, context-switching — all IndexedDB, zero surveillance). Why competitors can't copy
(business-model matrix).

**User:** Awesome, what other ideas in this same vein?

**Assistant:** Anti-dopamine / friction-as-a-feature: (1) "Proof of Context" friction gates (block a
broadcast reply to a long article if local read-state shows <N seconds on page); (2) "Synthesize or Stop"
checkpoint (after N consecutive passive nodes, unmount the feed, demand a one-sentence synthesis or close);
(3) local composting (aggressive client-side auto-delete of low-utility r=∞ chatter — intentional amnesia,
the machine forced to forget); (4) the "Open Equalizer" (real-time sliders mapped to local WebGPU AI +
per-post "why am I seeing this?" receipt); (5) batched asynchronous briefings (SW intercepts pings except
r=1 WebRTC rings; a single 5pm synthesized briefing).

**User:** Interesting — other ways we could actualize the utility-vs-extraction advantage; we are genuinely
aligned with the user here in a way online social has not been.

**Assistant:** (1) pluggable feeds / "bring your own algorithm" (ATProto Feed Generators + an algorithm
store — Mozilla/university/independent; the infra fee is unchanged regardless of your chosen sort); (2)
continuous export / "packing your bags daily" (File System Access API → SW silently writes latest notes/
bookmarks/threads to a chosen local/iCloud/Drive folder as Markdown; data already on your desktop if the
servers vanish); (3) zero-take economics / direct-to-creator (Web Monetization / native tipping — 100% to
the creator wallet, the platform abstains); (4) "read-it-later" as the default state (primary CTA is
"Queue", not "Reply" — intentional delay against real-time-engagement anxiety). The ultimate advantage:
institutional trust proven by the physics of the software.

**User:** This is possible? ["Continuous export" — please verify.]

**Assistant:** Yes, technically. Two pillars: **OPFS** (sandboxed high-performance local disk allocated to
the PWA; sync access handles in Web Workers; used by Photoshop-on-web and local WASM SQLite) and the **File
System Access API** (`showDirectoryPicker()` — hook into a user-visible folder, e.g. an Obsidian vault or a
cloud-synced dir). Continuous-export background pipeline (SW wake / session end → WASM extracts latest
IndexedDB → map to Markdown + YAML frontmatter → stream to the chosen OS folder). Real-time mirroring; zero
lock-in (your local file tree stays current independent of the servers). Legacy platforms keep data trapped
in proprietary server schemas + 72h zip friction as a dark pattern.

**User:** Ok what else?

**Assistant:** Protocol-level interoperability as anti-rivalry. Attention is rivalrous in ad social (they
must steal yours to sell it); local-first + decentralized makes the digital experience anti-rivalrous (your
local AI runs on your WebGPU silicon, your storage mirrors to your own FS, your graph lives on your PDS).
Creator and user finally pull the same direction: machine computes provenance, user defines utility, helper
infra keeps the pipes clean/fast/open.

**User:** Think through building this and lay out a prospective ordering roadmap.

**Assistant:** 5-phase engineering roadmap: **P1 Sovereign Client & Local Core (mo 1-3)** — Rust→WASM core;
Dexie IndexedDB layer; OPFS + File System Access continuous-export; UI skeleton + keyboard-first finite-feed
layout. **P2 ATProto Integration & Auth (mo 4-6)** — ATProto OAuth + granular scopes; Jetstream/AppView
indexer pipeline; dual-write & graft engine; non-destructive slide-over drawer. **P3 Helper Infra & P2P
Rooms (mo 7-9)** — sidecar encrypted messaging (MLS/WebRTC); TURN/SFU edge deploy (SFrame); Privacy-Pass
metering. **P4 Local Intelligence & Advanced Primitives (mo 10-12)** — WebGPU semantic search; Digital
Credentials + CTAP/BLE; intentionality budgets & friction gates. **P5 Ecosystem Open-Sourcing &
Extensibility (mo 13+)** — sandboxed lexicon registry; pluggable algorithm marketplace; public beta + self-
hosted infra docs.

---

## Load-bearing claims flagged for the fact-check (see the research doc)

- **`site.standard.*` long-form lexicon family** (`document`/`publication`/`graph.recommend`/
  `graph.subscription`) — is this a real published namespace or a fabrication? A lot of the long-form design
  rests on it.
- **`app.bsky.feed.threadgate` semantics** — does it natively support a **mutuals/bidirectional-follow**
  rule (the dialogue maps r=1 → threadgate "mutuals only")? Enumerate the actual allowed rules + whether
  hidden-replies is a field vs. an `app.bsky.feed.hideReply` method.
- **`quoteCount` on the post view; `getMutes`/`getBlocks`; `cdn.bsky.app` image variant paths;
  `public.api.bsky.app` unauthenticated CORS; `pinnedPost`; Jetstream endpoint shape.**
- **"ATProto native E2EE DMs / Privately Shared Data"** — very likely overstated; verify current DM/private-
  data state.
- **atproto OAuth granular per-lexicon scopes** — shipped or aspirational?
- **W3C Digital Credentials API + "Chrome 141 / Safari 26, late 2025"** timing.
- **WebAuthn PRF** feature-detection shape; **Privacy Pass "RFC 9152"** (likely the wrong RFC number);
  **MLS RFC 9420**; **SFrame** status; **`chrome.storage.sync` 100KB/8KB quotas**; **File System Access /
  OPFS / Background Sync / `navigator.connection` / share_target** browser-support caveats (Safari/Firefox);
  **Web Monetization / Interledger** status; **OpenID4VP** cross-device VP; **WebGPU/WebNN**; **ThumbHash/
  BlurHash**; **Yjs/Automerge/HLC**.
