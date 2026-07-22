# Raw: Gemini — client-side search for serverless atproto SPAs + Heardle-shaped music pond + Kudoboard (2026-07-22)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a pasted **Gemini** session 2026-07-22. UI render chrome stripped ("Opens in a new
window", "Gemini is AI and can make mistakes", inline citation superscripts folded into the Sources block
at the end). No secrets. Part of the 2026-07-22 batch (plan:
`alpha/plans/2026-07-22-transcript-intake-ponds-kernel-ens.md`).

> **⚠ WHOLE-DOCUMENT `[UNVERIFIED]`.** This is Gemini output the user explicitly flagged **"needs more
> fact-checking."** Every version number, bandwidth figure, benchmark, size limit, browser-behaviour
> claim, and API detail below is UNVERIFIED and must be checked against a primary source before any of it
> is promoted into a research or beta doc (per the plan, the *full* fact-check runs at outcome-processing
> time). Gemini's known failure mode (per the atproto FACTCHECK) is provenance/packaging drift — invented
> codenames, mis-attributed crate/library names. Treat library names (`atcute`, `Tap`, `SearchFn`,
> `coi-serviceworker`), the VFS names, and the "Sync v1.1 / 200 GB/day / 60–150 KB/s" figures as claims to
> verify, not facts. Where it touches atproto/iroh/iOS, the FACTCHECK
> (`atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) is the source of truth. Note: it uses "Merkle Search
> Tree (MST)" for atproto's repo structure — that specific usage is FACTCHECK-confirmed correct.

---

## Part A — Architectural blueprints for serverless, client-side search in atproto apps (Gemini research report)

Premise: a serverless SPA/PWA that talks directly to atproto, with no indexing backend, must push
ingestion, schema validation, index generation, and query execution into the browser.

### Ingestion
- **Real-time via Jetstream.** Subscribing directly to the relay firehose (`subscribeRepos` WebSocket)
  is impractical in-browser — high bandwidth (claimed >200 GB/day raw) and CPU to parse compressed
  DAG-CBOR. **Jetstream** is an open-source streaming proxy that converts the binary firehose to
  lightweight JSON over a standard WebSocket, with connection-time filters (by collection NSID, e.g.
  `app.bsky.feed.post`, or by DID). Claimed bandwidth for a posts-filtered global subscription: 60–150
  KB/s — manageable on broadband, strains mobile/battery. Mitigation: an edge-hosted aggregation layer
  (e.g. a Cloudflare Worker + Durable Object) holds one upstream Jetstream connection and streams a
  compressed, filtered payload to clients.
- **Historical via CAR archives.** Fetch a user's repo from their PDS with `com.atproto.sync.getRepo` →
  a **Content-Addressable Archive (CAR)**: a signed root commit block + MST nodes + records in
  DRISL-CBOR (a deterministic CBOR profile). Parse in-browser with lightweight libs (`atcute` —
  `@atcute/car`, `@atcute/cbor`) to decode headers, walk the MST, deserialize records. Reconstructing the
  tree uses per-key depth `depth = floor(leadingZeros(SHA-256(key)) / 2)` to place keys in the Merkle
  tree and verify cryptographic integrity.
- **Backfill + live reconciliation via the `Tap` daemon pattern.** New DID → backfill phase (full repo
  history from the authoritative PDS) → transition to live WebSocket events; use the per-account revision
  (`rev`) to de-duplicate, discard out-of-order events, drop stale updates by sequence boundary.
  "Collection Signal Mode" uses profile records (`app.bsky.actor.profile`) as discovery markers to
  start/stop backfill.

### Client-side search engine comparison (all figures UNVERIFIED)

| Engine | Bundle (min+gz) | Latency @10k | Memory @10k | Persistence | Best use |
|---|---|---|---|---|---|
| **FlexSearch** | ~6 KB light / ~15 KB bundle | sub-ms | ~15–40 MB (varies by tokenizer) | manual keyed block exports | high-throughput real-time keyword; Web Worker parallel; **returns only doc IDs** (no scores/offsets → custom highlight layer) |
| **MiniSearch** | ~6 KB | 1.5–5 ms | ~3–8 MB | JSON import/export | dynamic timelines; radix-tree compact index; incremental add/remove; returns BM25 scores + token offsets |
| **Orama** | ~2 KB core | 1–4 ms | ~12–25 MB (>400 MB @100k → tab-crash risk) | JSON / dpack plugin | hybrid lexical+vector; AVL/BKD trees for facets/geo; `@orama/plugin-data-persistence` needs Node stream polyfill (+500 KB Vite risk) |
| **uFuzzy** | ~3 KB | 2–10 ms (linear O(N)) | ≤1 MB (indexless) | none (raw array) | autocomplete / short lists <10k; compiles queries to regex over raw strings; 3-phase filter→info→sort |
| **SQLite FTS5 (WASM)** | ≥1 MB WASM | 0.5–12 ms | configurable page cache | OPFS / IndexedDB VFS | large historical archives; native FTS5 virtual tables; ~100–500 ms init latency |
| **SearchFn** | — | — | — | IndexedDB + in-mem term cache | FlexSearch-compatible adapter; browser-first index-in-IndexedDB |

Relevance scoring is standard **BM25** (non-linear TF saturation + doc-length normalization; params
`k1≈1.2–2.0`, `b≈0.75`; IDF `= ln(1 + (N − n(qi) + 0.5)/(n(qi) + 0.5))`). Typo tolerance via FTS5
**trigram tokenization** (e.g. `bluesky` → `{blu, lue, ues, esk, sky}`).

### Browser storage / VFS
- **LocalStorage** ~0.017 ms write / 0.005 ms read but 5 MB cap + synchronous (blocks main thread).
- **IndexedDB** async, high capacity, transaction overhead.
- **OPFS** high-perf, synchronous access handles inside dedicated Web Workers.
- **SQLite VFS choices:** `IDBBatchAtomicVFS` (maps pages to IndexedDB; good <100 MB; works in Safari
  private mode; degrades larger, stack-depth risk on Safari) vs `OPFSCoopSyncVFS` / `AccessHandlePoolVFS`
  (sync access handles in dedicated workers; scales >1 GB; **not** in Safari incognito; Chrome incognito
  100 MB cap). Fallback pattern: detect OPFS → OPFSCoopSyncVFS else IDBBatchAtomicVFS.
- **Tuning:** `PRAGMA journal_mode=truncate` faster than delete/wal on OPFS; `PRAGMA synchronous=normal`;
  larger page cache (`-DSQLITE_DEFAULT_CACHE_SIZE=-8192` = 8 MB). For very large datasets, **HTTP Range
  Requests** via `sql.js-httpvfs` / `sqlite-wasm-http` fetch only needed pages from a static CDN
  (`requestChunkSize: 4096` to match SQLite page size).

### Deployment / security
- **`SharedArrayBuffer` needs cross-origin isolation** (Spectre mitigation): headers
  `Cross-Origin-Opener-Policy: same-origin` + `Cross-Origin-Embedder-Policy: require-corp`. Missing → no
  multi-threaded WASM, fallback to single-threaded in-memory.
- **Static-host workaround:** a service worker (`coi-serviceworker.js`) intercepts fetches and injects
  COOP/COEP in-browser to establish cross-origin isolation on GitHub Pages / Cloudflare Pages.
- **Multi-tab:** route all OPFS access through a shared SharedWorker/ServiceWorker that serializes
  read/writes; use the Web Locks API to transfer the connection/lock to another active tab if a worker
  tab is suspended.

### Recommended patterns
- **Real-time feed:** uFuzzy + filtered Jetstream (min memory + startup; trending/live).
- **Scale-out archive:** SQLite FTS5 + OPFS in a Web Worker (offloads heap → disk; scales large).
- **Hybrid:** MiniSearch + IndexedDB (or SearchFn) — incremental + rich match metadata + typo-tolerant.

---

## Part B — Heardle as a cautionary tale + a local-first PWA music-guessing pond (Gemini)

### Why Heardle is a cautionary tale
Heardle: viral Wordle spin-off (early 2022; guess a song from its opening seconds in 6 tries; tens of
millions of visitors). Spotify acquired it (July 2022), shut it down (May 2023). The three lessons:
1. **Acquisition misalignment / value extraction** — bought to funnel users into the main app; forced
   Spotify player integration broke the lightweight web mechanics.
2. **Platform fragility / broken trust** — licensing conflicts, regional blocks, broken streaks →
   retention collapsed.
3. **Data & product ephemerality** — shutdown vanished all player histories, stats, streaks, leaderboards.

*(This is a literal ENS/ephemerality case — ties to the enshittification thread and the "10-year
survivability" product promise.)*

### Local-first PWA + atproto PDS architecture
Local state primary (IndexedDB/OPFS, 100% offline); stats sync to the user's atproto PDS so they survive
even if hosting dies.

```
┌───────────────── Local Device / PWA ─────────────────┐
│  [UI / Game Engine] ──▶ [LocalDB (IndexedDB)]         │
└──────────────────────────────────┬───────────────────┘
                                   │ Sync (Online)
                                   ▼
                        [ User ATProto PDS ]
```

Custom lexicon (public game results), local-first storage handler (idb/Dexie), a PDS publisher
(`@atproto/api`, `com.atproto.repo.createRecord`), and a Workbox service worker caching the app shell +
audio.

### How Heardle was played, and why it fits
Daily global puzzle (same song for everyone/24h). Incremental unlocks: 1s → 2s → 4s → 7s → 11s → 16s on
wrong guess/skip; 6 attempts; constrained autocomplete search pool; emoji-grid share (🔊🟩⬜…) + browser
streak counter. Fit: (1) **deterministic daily seed** — client derives today's song from a date-hash / tiny
static JSON manifest, no backend game engine; (2) **audio caching** via SW (16s low-bitrate < ~500 KB);
(3) **local state as source of truth** (kills the fragility that killed Heardle); (4) **decoupled
publishing** — on finish, write an `app.heardle.result` record to the user's PDS; other apps / feeds /
firehose listeners read it for leaderboards without any DB server.

### Licensing analysis (legal — treat as candidate/backlog per the plan)
No minimum threshold for music copyright infringement (a 1-second sample can infringe if it captures the
"heart"). Two separate copyrights per track: **composition** (publisher/songwriter) and **master recording**
(label). Original Heardle dodged early suits by embedding third-party players (SoundCloud, later Spotify) —
hosting zero audio, offloading licensing to the platform — which introduced regional-block fragility;
Spotify's Web Playback SDK bound it to Spotify's regional licenses and drove the regional shutdowns.

Four distribution models for a local-first build:
- **A — Embedded official APIs** (Spotify Web Playback SDK / YouTube IFrame / Apple MusicKit): full
  mainstream catalog; requires third-party auth; breaks offline.
- **B — Free Music Archive / Creative Commons** (FMA, Jamendo, ccMixter): offline-cacheable + redistributable;
  no chart-toppers.
- **C — Video-game / chiptune / open audio**: micro-sized (<50 KB); still risky for modern AAA.
- **D — Bring-your-own-media**: engine accepts a playlist JSON / local uploads; app hosts zero copyrighted
  assets (pure-utility legal shield).

### User decisions (this thread)
- **Hybrid licensing** — "a few genuine notable open-licensed tunes, a few likely-fine game tunes, and
  integrations." **Guest mode first** ("so you can get a feel for it before you need to login"), and
  **challenges per backend**.
- On reusing Super Mario Bros 1-1 / Sonic Green Hill / Mario 3 Bowser's Castle: **No, not safely** —
  Nintendo (Koji Kondo) defends aggressively; Sonic 1/2 compositions were licensed from Masato Nakamura /
  Dreams Come True and are managed tightly (JASRAC). Even re-arranging the melody infringes the
  composition. → use **stylistic pastiche ("sounds-like")** or CC/public-domain.
- On pastiche worry ("style like is too off"): use a **"sounds-like" rather than an identity** approach —
  players guess archetypes/tropes, not brand names.

### The 6-channel "sounds-like" starter set (Gemini proposal)
1. **"Plumber's Run"** (sounds-like Mario 1-1) — bright ragtime-calypso 4/4, NES 2A03 pulse arps.
   Title e.g. "Mushroom Bounce"; pool: Platformer Level 1, Retro Jump'n'Run, Chiptune Overworld.
2. **"Zone 1 FM Slap"** (sounds-like Sonic Green Hill) — 16-bit Genesis YM2612 FM, metallic slap-bass.
   Title e.g. "Emerald Hills"; pool: 16-Bit Speed Run, Genesis FM Jam.
3. **"Lava Citadel"** (sounds-like Mario 3 Bowser) — dark low chromatic bassline, triangle arps, noise
   percussion. Title e.g. "Koopa Keep"; pool: 8-Bit Boss Castle, Final Dungeon Theme.
4. **In the Hall of the Mountain King** — CC0 8-bit chiptune of Grieg (1876); ramps in speed (perfect for
   the 1s→16s ladder); composition public-domain globally.
5. **Pixel Rush** — 80s arcade chiptune (OpenGameArt / Kevin MacLeod, CC-BY/CC0).
6. **8-Bit Canon in D** — NES Pachelbel; recognizable in ~1s → good tutorial "1-second win"; PD composition.

Served via a per-channel daily manifest, e.g.:

```json
{
  "date": "2026-07-22",
  "channels": [
    { "id": "starter-pastiche-01", "name": "Level 1 Overworld", "requiresAuth": false,
      "provider": "html5", "audioUrl": "/audio/past-overworld.mp3",
      "metadata": { "title": "Mushroom Bounce", "artist": "Open Chiptune Studio",
                    "searchPool": ["Mushroom Bounce", "Platformer Level 1", "Plumber's Run"] } }
  ]
}
```

### Multi-backend architecture + lexicon
A unified `AudioPlayer` interface (`play`/`pause`/`seek`) with adapters: HTML5 (local/CC, offline,
no-login), Spotify Web Playback SDK (Premium auth), YouTube IFrame. Backend-driven daily manifests define
"channels/packs" (`requiresAuth`, `provider`, `spotifyUri`/`audioUrl`, metadata + searchPool). Extended
lexicon `app.mygame.stats` / `app.heardle.result` records `gameDate`, `channelId`, `provider`
(`html5|spotify|youtube`), `score` (1–6), `won`, `guessCount`, `createdAt`, `prevResultCid`. Onboarding:
guest → optionally connect Spotify (music) and/or atproto/Bluesky (publish); on connect, batch-sync guest
IndexedDB history up to the PDS.

### How guessing + reporting works
Controlled autocomplete (pick from a dropdown, not freeform) → standardized ID matched against the day's
answer (`guessedId === targetId`); prevents typos, makes validation trivial and client-side. Two reports:
(A) **visual grid** (🔊/🟩/🟥/⬛/⬜) to `navigator.clipboard.writeText()`; (B) **PDS record**
(`app.heardle.result`) published to the user's repo, readable by leaderboards/feeds/firehose listeners with
no DB server.

### Hidden issues (Gemini "gotchas")
1. **Autoplay block** (esp. iOS Safari) — first clip can't autoplay; need an explicit "Tap to Play" gesture
   that calls `audioCtx.resume()`. *(User: "yes I would want that anyway.")*
2. **"Time-traveler" cheat** — `new Date()` is gameable (set clock forward, reveal tomorrow's song). Fixes:
   a server `/api/time` check, OR atproto record `createdAt` validated by external leaderboards.
   *(User chose: crypto monotonic ordering — see below.)*
3. **Safari 7-day storage eviction** — IndexedDB/CacheStorage cleared if the PWA isn't visited for 7 days.
   Fix: "Add to Home Screen" (persistent-storage exemption) + early atproto login so state restores from
   the PDS. *(User: "heard.")*
4. **atproto auth** — App Passwords (easy MVP) vs OAuth + DPoP (production; resolves handle→custom PDS).
   *(User: "fine, we have examples.")*
5. **Audio clipping pop** — abrupt cut at 1.000s pops; apply a ~50 ms GainNode exponential fade-out.
   *(User: "good call.")*
6. **Lexicon schema drift** — once `app.heardle.result` is published to user repos, changing/deleting a
   non-optional field breaks network indexers. Fix: make almost all fields optional; wrap payload in an
   extensible `metadata` object.
7. **Service-worker audio cache invalidation** — generic filenames serve yesterday's cached track. Fix:
   content-addressed / date-stamped filenames + CacheFirst with explicit expiration.
8. **Search-pool spoiling** — a small local search index lets users inspect memory to deduce the answer.
   Fix: a large decoy pool (1,000+ titles) or derive from an open DB (MusicBrainz).
9. **Multi-device monotonic sequence divergence** — playing Game #142 offline on two devices creates
   competing records pointing at #141. Fix: CRDT or LWW merge; append `deviceId` to record keys.

### User decision — cryptographic monotonic ordering for anti-cheat (Part 2 answer)
Instead of trusting the local clock, establish verifiable sequence/causal ordering three ways:
- **A. Lamport/vector clocks + monotonic counters.** Each result carries a strict incrementing counter
  (`Cn = Cn-1 + 1`) plus `prevResultHash` — can't produce a valid Game #143 state without the chain
  anchored in #142. Example: `{ "sequenceNumber": 142, "gameDate": "2026-07-22", "prevResultHash":
  "bafyreib2kw3…", "won": true, "score": 2 }`.
- **B. Content-addressed DAG (Merkle).** atproto records are IPLD-linked nodes identified by CIDs;
  modifying a past result changes the CID and breaks the chain; the PDS commit tree naturally rejects
  backdated insertions. `[#140]◄─[#141: prev=140]◄─[#142: prev=141]`.
- **C. Optional decentralized time anchoring (proof of existence).** A daily seed from a trusted public
  entropy source (Bitcoin block hash / drand beacon); the client publishes `hash(userDID + gameId + score +
  dailySeed)` — since the seed wasn't known yesterday, the result couldn't have been precomputed.

---

## Part C — Kudoboard (Gemini explainer)

**Kudoboard** — an online platform for digital group greeting cards / appreciation boards. One person
creates a board and shares a link; multiple people add messages to a shared online wall (instead of passing
a physical card). Features: rich media (text, photos, GIFs, memes, short video); optional group gift fund
(pooled money → digital gift card); delivery as email link, live slideshow, or printable poster/book.
Uses: workplace (farewell, work anniversaries, promotions, shout-outs) and personal (group birthday,
get-well, baby showers, sympathy). *(Direct incumbent for the collaborative-card / webxdc-group-card thread
in `croft-card-maker-webxdc-packaging-and-push-2026-07-22.md`.)*

---

## Sources (Gemini's cited links, condensed from the UI citation list — UNVERIFIED provenance)

docs.bsky.app (AT Protocol docs; Firehose; Repository Exports; Tap; Relay Transition), atproto.com
(Repository; Streaming Data; Data Model), chjh.nl (Hartgerink Bluesky technical), vickiboykis.com
(hack on ATProto), news.ycombinator.com (Jetstream >99%; uFuzzy; SQLite FTS5), dasl.ing (DASL), sdk.blue,
github.com/mary-ext/atcute, github.com/nextapps-de/flexsearch, lucaongaro.eu + github.com/lucaong/minisearch,
docs.orama.com, github.com/leeoniya/uFuzzy, sqlite.org (FTS5; Virtual Table), developer.chrome.com (SQLite
Wasm + OPFS), powersync.com (SQLite persistence on the web, May 2026), rxdb.info (storage comparison),
recca0120.github.io (sql.js-httpvfs), notion.com (WASM SQLite in the browser), smashingmagazine.com
(local-first web dev), and others. **All to be re-verified at fact-check time.**
