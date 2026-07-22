# Client-side search for serverless atproto SPAs (fact-checked)

date: 2026-07-22

status: research deliverable. This doc **distills and fact-checks Part A** of a 2026-07-22 Gemini
research report on client-side, serverless search for atproto apps. Raw source (whole-document
`[UNVERIFIED]`, Gemini output the user flagged "needs more fact-checking"):
`../seeds/transcripts/raw/gemini-atproto-clientside-search-heardle-pond-kudoboard-2026-07-22.md`
(Part A). Parts B (Heardle pond) and C (Kudoboard) are out of scope here.

Every technical claim below was checked against a primary source (official docs, the library's own
repo/npm, the SQLite/DASL specs). Verdict tags are used consistently: **CONFIRMED** (exists and as
described), **PARTLY** (real but mis-described or a figure is off), **REFUTED** (false / no such
thing), **UNVERIFIED** (no credible source found, or a benchmark/figure not independently checkable).
For atproto/iroh/iOS facts the source of truth is the FACTCHECK
(`../seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`); this doc cites it
rather than re-verifying (notably: MST is atproto's repo structure; the relay is non-archival under
Sync v1.1; "Tap" is a real official Bluesky tool).

Gemini's known failure mode is **provenance/packaging drift** (invented codenames, mis-attributed
library names). The main casualty in Part A: **"SearchFn" - no such library was found** (see verdicts).

---

## Headline

Part A is **substantially accurate on architecture and library identity** - a much stronger showing
than a typical hallucination-heavy transcript. Every named search engine, VFS, and tooling package is
real and roughly as described, and the load-bearing atproto mechanics (Jetstream, CAR/getRepo,
DRISL/DAG-CBOR, the MST key-depth formula, Tap) check out. The corrections are narrow:

1. **"SearchFn" does not appear to exist** as a named library (UNVERIFIED, likely invented / mis-named).
2. **The Jetstream "60-150 KB/s filtered posts" figure is overstated** by roughly an order of magnitude
   versus the official compressed headline (~850 MB/day ≈ 10 KB/s for all posts).
3. **Several bundle-size / memory / latency numbers are Gemini's own benchmarks** and are UNVERIFIED;
   one spot-check failed (uFuzzy is ~7.5 KB min, not ~3 KB).

Otherwise the blueprint is sound and buildable.

---

## Verified architecture

Premise (accepted): a serverless SPA/PWA that talks directly to atproto with no indexing backend must
push ingestion, schema validation, index generation, and query execution into the browser.

### Ingestion

**Real-time via Jetstream. [CONFIRMED, one figure PARTLY]**
Subscribing in-browser to the raw relay firehose (`com.atproto.sync.subscribeRepos`) is impractical:
the firehose is high-bandwidth and delivers compressed DAG-CBOR. **Jetstream** is a real,
Bluesky-operated open-source proxy (hosted under the `bluesky-social` GitHub org) that consumes
`subscribeRepos` and re-emits **lightweight JSON over a plain WebSocket, no auth**, with
**connection-time filters**: `wantedCollections` (collection NSIDs, prefix globs like `app.bsky.*`,
max 100) and `wantedDids` (max 10,000). [CONFIRMED: docs.bsky.app/blog/jetstream; jazco.dev Jetstream post]

Bandwidth figures:
- Raw firehose **">200 GB/day"** - **[CONFIRMED]**. The Jetstream post cites **over 232 GB/day**
  during the 2024 Brazil-exodus surge.
- Filtered posts **"60-150 KB/s"** - **[PARTLY / overstated]**. Bluesky's own headline is that you can
  "live tail all posts for as little as **~850 MB/day**" (≈ **10 KB/s**) with zstd compression. Gemini's
  60-150 KB/s (≈ 5-13 GB/day) is roughly 6-15x high - likely an uncompressed / peak-events estimate, not
  the compressed posts-only figure. Use ~850 MB/day compressed as the planning number.

The "edge aggregation layer (Cloudflare Worker + Durable Object holding one upstream connection,
fanning out a compressed filtered payload)" is a reasonable design pattern, not a claim to verify.

**Historical via CAR archives. [CONFIRMED]**
`com.atproto.sync.getRepo` returns a **CAR (Content-Addressable aRchive)**: a signed root commit block
plus MST nodes plus records. Records are **DAG-CBOR**, which atproto aligns with **DRISL**
("Deterministic Representation for Interoperable Structures & Links", the DASL project's deterministic
CBOR profile, codec `0x71`, aka dag-cbor). So Gemini's "DRISL-CBOR (a deterministic CBOR profile)" is
**correct terminology**, not an invention. [CONFIRMED: atproto.com/specs/repository; dasl.ing/drisl.html]

Parse in-browser with **atcute** (`mary-ext/atcute`, a real TypeScript monorepo) - the
**`@atcute/car`** (CAR codec) and **`@atcute/cbor`** (deterministic CBOR codec) packages both exist on
npm. [CONFIRMED: github.com/mary-ext/atcute; npmjs.com/package/@atcute/cbor]

The MST key-depth formula **`depth = floor(leadingZeros(SHA-256(key)) / 2)`** is **exactly right**: the
atproto repository spec computes a key's tree layer by SHA-256-hashing the key and counting leading
binary zeros in 2-bit chunks (fanout 4). [CONFIRMED: atproto.com/specs/repository]

**Backfill + live reconciliation via Tap. [CONFIRMED tool; some specifics UNVERIFIED]**
**Tap** is a real, **official Bluesky repo-synchronization tool** (`@atproto/tap`, blog
"Introducing Tap: Repository Synchronization Made Simple", Dec 2025). It does exactly what Gemini
describes as a pattern: on a new DID it **backfills full repo history from the authoritative PDS via
`getRepo`, then transitions to live firehose events**, and handles MST integrity checks, identity/
signature verification, desync recovery, and **filtered JSON output by repo and collection**.
[CONFIRMED: atproto.com/blog/introducing-tap; FACTCHECK addendum 2026-06-22]
- **`rev`-based dedup / out-of-order discard** - plausible and consistent with atproto's per-commit
  `rev`, but not confirmed as Tap's documented mechanism. **[UNVERIFIED]**
- **"Collection Signal Mode"** (using `app.bsky.actor.profile` records as discovery markers to
  start/stop backfill) - **[UNVERIFIED]**, reads like a Gemini-coined name; Tap does support
  collection filtering, but this specific named mode was not found.

### Client-side search engine comparison

All five named engines are real and roughly as described. **The "SearchFn" sixth row is not.**
Bundle/memory/latency numbers are Gemini's own and are **[UNVERIFIED benchmarks]** except where noted.

- **FlexSearch** (`nextapps-de/flexsearch`) - **[CONFIRMED]**. Real, high-throughput, first-class
  **Web Worker** offload, and returns arrays of **document IDs** by default (highlighting/scoring is a
  layer you add). Modern versions also support persistent indexes (IndexedDB, SQLite, etc.). Bundle
  figures UNVERIFIED. [github.com/nextapps-de/flexsearch]
- **MiniSearch** (`lucaong/minisearch`) - **[CONFIRMED, one nuance]**. Radix-tree (compressed prefix
  tree) index, incremental add/remove, JSON import/export, prefix + fuzzy search. Scoring is
  **BM25+** (Gemini said "BM25" - close; it is the BM25+ variant). "Returns token offsets" is
  partly right: it returns per-result match metadata (matched terms/fields). [github.com/lucaong/minisearch]
- **Orama** (`oramasearch/orama`) - **[CONFIRMED]**. "Complete search engine ... full-text, vector,
  and hybrid search in less than 2kb" (the **<2 KB core** and **hybrid lexical+vector** claims both
  check out). **`@orama/plugin-data-persistence`** is a real package. The "needs a Node stream polyfill
  under Vite (+500 KB risk)" detail is **[UNVERIFIED]** (environment-specific, not independently
  confirmed). [github.com/oramasearch/orama]
- **uFuzzy** (`@leeoniya/ufuzzy`) - **[CONFIRMED, size wrong]**. Indexless fuzzy matcher, compiles
  queries to **non-unicode regex over a raw array** (a "more forgiving String.indexOf"), linear scan.
  Size correction: the repo states **~7.5 KB min**, not Gemini's ~3 KB - **[PARTLY]**. Perf example:
  a 3-term search over 162,000 phrases in ~5 ms. [github.com/leeoniya/uFuzzy]
- **SQLite FTS5 (WASM)** - **[CONFIRMED]**. Native FTS5 virtual tables in a WASM SQLite build,
  persisted via an OPFS/IndexedDB VFS. The **trigram tokenizer** is a real FTS5 tokenizer that enables
  substring matching (`bluesky` → `blu, lue, ues, esk, sky`). Init-latency figures UNVERIFIED.
  [sqlite.org/fts5.html]
- **"SearchFn"** - **[UNVERIFIED / likely invented]**. No npm package or repo by this name surfaced
  matching the description ("FlexSearch-compatible adapter, index-in-IndexedDB"). This is Gemini's
  packaging-drift signature. Do not cite it as a real dependency. If a FlexSearch-compatible
  IndexedDB-backed adapter is wanted, FlexSearch's own IndexedDB persistence (or RxDB's flexsearch
  plugin) is the real path.

**BM25 relevance scoring - [CONFIRMED].** The description (non-linear TF saturation + doc-length
normalization; `k1 ≈ 1.2-2.0`, `b ≈ 0.75`) and the IDF form
`IDF = ln(1 + (N − n(qi) + 0.5)/(n(qi) + 0.5))` are the standard Okapi/Lucene BM25. k1 controls TF
saturation; b controls length normalization (0.75 is the usual default). [multiple IR references]

### Browser storage / VFS

- **LocalStorage / IndexedDB / OPFS** trade-offs as described - LocalStorage is synchronous and
  ~5 MB-capped; IndexedDB is async and high-capacity; **OPFS gives synchronous access handles inside
  dedicated Web Workers**. **[CONFIRMED]** (the sub-ms LocalStorage micro-benchmarks are UNVERIFIED
  figures but directionally standard).
- **VFS names - all real.** These are the VFS classes in **`rhashimoto/wa-sqlite`**:
  - **`IDBBatchAtomicVFS`** - maps pages to IndexedDB; **good fallback for Safari (incognito Safari has
    no OPFS)**; slower than the OPFS VFSes. **[CONFIRMED]** (the ">100 MB degradation / stack-depth"
    detail is a plausible but UNVERIFIED figure).
  - **`OPFSCoopSyncVFS`** - synchronous OPFS VFS that **supports multiple connections** via a
    lazily-closed access-handle pool; the recommended high-scale choice. **[CONFIRMED]**. "Not in
    Safari incognito" is consistent with OPFS being unavailable in Safari private mode. The "Chrome
    incognito 100 MB cap" is **[UNVERIFIED]** (specific quota figure).
  - **`AccessHandlePoolVFS`** - pre-opens a pool of access handles; **single wa-sqlite instance only
    (no multiple connections)**. **[CONFIRMED]**.
  - Fallback pattern (detect OPFS → OPFSCoopSyncVFS, else IDBBatchAtomicVFS) is exactly the
    community-recommended approach. [github.com/rhashimoto/wa-sqlite]
- **Tuning: `PRAGMA journal_mode=truncate` faster than wal on OPFS** - **[UNVERIFIED]** as a general
  claim (wa-sqlite guidance is VFS-dependent; some OPFS VFSes ship their own WAL variant). Treat as a
  benchmark-it-yourself hint, not a settled fact. `synchronous=normal` and a larger page cache are
  standard SQLite tuning.
- **HTTP Range Requests for large read-only DBs - [CONFIRMED].** `sql.js-httpvfs` (`phiresky`) and
  `sqlite-wasm-http` (`mmomtchev`) both implement an HTTP-Range VFS that fetches only the SQLite pages
  a query needs from a static CDN. **`requestChunkSize` defaults to 4096** to match the SQLite page
  size - **exactly as stated**. [github.com/phiresky/sql.js-httpvfs; github.com/mmomtchev/sqlite-wasm-http]

### Deployment / cross-origin isolation

- **`SharedArrayBuffer` requires cross-origin isolation - [CONFIRMED].** Multi-threaded WASM (and
  `SharedArrayBuffer`) require the document be cross-origin isolated via
  `Cross-Origin-Opener-Policy: same-origin` + `Cross-Origin-Embedder-Policy: require-corp` (a Spectre
  mitigation). Without them you fall back to single-threaded in-memory. [web platform standard]
- **`coi-serviceworker` on static hosts - [CONFIRMED].** `gzuidhof/coi-serviceworker` is a real service
  worker that injects COOP/COEP headers client-side so `SharedArrayBuffer` works on hosts you cannot
  set headers on (GitHub Pages, Cloudflare Pages). Constraints worth carrying forward: it must be a
  separate file served **from your own origin** (not a CDN), page must be HTTPS/localhost, and it
  **reloads once on first load** to install. [github.com/gzuidhof/coi-serviceworker]
- **Multi-tab: Web Locks API - [CONFIRMED as a real API].** The Web Locks API is a real browser API
  for coordinating access across tabs/workers; routing OPFS access through a shared worker and using
  Web Locks to hand off the connection when a tab is suspended is a sound pattern. The exact
  orchestration is a design choice, not a claim. [MDN Web Locks API]

### Three recommended patterns (accepted as reasonable)

1. **Real-time feed:** uFuzzy + filtered Jetstream - minimal memory and startup, best for trending/live
   short lists (<~10k).
2. **Scale-out archive:** SQLite FTS5 + OPFS in a Web Worker - offloads the heap to disk; scales to
   large historical archives.
3. **Hybrid:** MiniSearch + IndexedDB - incremental updates, BM25(+) match metadata, typo tolerance.
   (Gemini also lists "SearchFn" here as an alternative; drop it - use FlexSearch's own IndexedDB
   persistence instead.)

---

## Fact-check verdicts

| Claim | Verdict | Source | Note |
|---|---|---|---|
| Jetstream is an official atproto tool converting `subscribeRepos` → JSON over WebSocket | CONFIRMED | docs.bsky.app/blog/jetstream; bluesky-social GH org | Open-source, no auth |
| Jetstream connection-time filters: `wantedCollections` (NSID, prefix, max 100), `wantedDids` (max 10k) | CONFIRMED | docs.bsky.app/blog/jetstream | |
| Raw firehose ">200 GB/day" | CONFIRMED | jazco.dev/2024/09/24/jetstream | 232 GB/day peak (Brazil surge) |
| Filtered posts "60-150 KB/s" | PARTLY | docs.bsky.app/blog/jetstream | Overstated ~6-15x; real ~850 MB/day (~10 KB/s) compressed for all posts |
| `com.atproto.sync.getRepo` returns a CAR (root commit + MST nodes + records) | CONFIRMED | atproto.com/specs/repository | |
| Records are DRISL / DAG-CBOR (deterministic CBOR) | CONFIRMED | dasl.ing/drisl.html; atproto.com/specs/data-model | DRISL codec 0x71, aka dag-cbor |
| `atcute` exists with `@atcute/car` + `@atcute/cbor` | CONFIRMED | github.com/mary-ext/atcute; npm @atcute/cbor | mary-ext, TypeScript |
| MST key depth = floor(leadingZeros(SHA-256(key)) / 2) | CONFIRMED | atproto.com/specs/repository | 2-bit chunks, fanout 4 |
| Tap = official atproto repo-sync/backfill tool (backfill via getRepo → live) | CONFIRMED | atproto.com/blog/introducing-tap; FACTCHECK | `@atproto/tap` |
| Tap `rev`-based dedup / out-of-order discard | UNVERIFIED | - | Plausible (rev is real) but not confirmed as Tap's documented mechanism |
| Tap "Collection Signal Mode" (profile records as backfill markers) | UNVERIFIED | - | Named mode not found; likely Gemini-coined |
| FlexSearch: returns doc IDs, Web Worker offload | CONFIRMED | github.com/nextapps-de/flexsearch | |
| MiniSearch: radix tree, incremental add/remove, BM25 scoring | PARTLY | github.com/lucaong/minisearch | Real; scoring is BM25**+** (not plain BM25) |
| Orama: hybrid lexical+vector, <2 KB core, `@orama/plugin-data-persistence` | CONFIRMED | github.com/oramasearch/orama | |
| Orama plugin needs a Node stream polyfill under Vite (+500 KB) | UNVERIFIED | - | Env-specific, not confirmed |
| uFuzzy: indexless, regex over array, O(N) linear | CONFIRMED | github.com/leeoniya/uFuzzy | |
| uFuzzy bundle ~3 KB | PARTLY | github.com/leeoniya/uFuzzy | Repo says ~7.5 KB min |
| SQLite FTS5 (WASM) with OPFS/IndexedDB VFS, trigram tokenizer | CONFIRMED | sqlite.org/fts5.html | Trigram enables substring match |
| **"SearchFn"** - FlexSearch-compatible IndexedDB adapter | UNVERIFIED (likely invented) | - | No npm/repo found; Gemini packaging-drift |
| VFS `IDBBatchAtomicVFS` (IndexedDB pages, Safari-private fallback) | CONFIRMED | github.com/rhashimoto/wa-sqlite | Slower than OPFS VFSes |
| VFS `OPFSCoopSyncVFS` (sync OPFS, multi-connection, scales large) | CONFIRMED | github.com/rhashimoto/wa-sqlite | Not in Safari private mode |
| VFS `AccessHandlePoolVFS` (single instance, no multi-connection) | CONFIRMED | github.com/rhashimoto/wa-sqlite | |
| Chrome incognito 100 MB OPFS cap; IDB >100 MB degradation | UNVERIFIED | - | Specific quota/threshold figures |
| `PRAGMA journal_mode=truncate` faster than wal on OPFS | UNVERIFIED | - | VFS-dependent; benchmark it |
| `sql.js-httpvfs` / `sqlite-wasm-http` HTTP-Range VFS; `requestChunkSize` default 4096 = page size | CONFIRMED | github.com/phiresky/sql.js-httpvfs; mmomtchev/sqlite-wasm-http | |
| `SharedArrayBuffer` needs COOP `same-origin` + COEP `require-corp` (Spectre) | CONFIRMED | web platform standard | |
| `coi-serviceworker` injects COOP/COEP on static hosts (GH/CF Pages) | CONFIRMED | github.com/gzuidhof/coi-serviceworker | Own-origin file; reloads once |
| Web Locks API for multi-tab connection/lock transfer | CONFIRMED | MDN Web Locks API | Real API; orchestration is design |
| BM25: TF saturation + length norm; k1 ≈ 1.2-2.0, b ≈ 0.75; IDF = ln(1 + (N − n + 0.5)/(n + 0.5)) | CONFIRMED | Okapi/Lucene BM25 references | Standard formula and defaults |
| Bundle-size / memory / latency table figures (except noted) | UNVERIFIED | - | Gemini's own benchmarks; spot-checks mixed |

---

## What to take forward

- **Buildable blueprint.** The ingestion path (Jetstream JSON firehose with NSID/DID filters + CAR via
  `getRepo` parsed with atcute + Tap for backfill/live) and the storage path (wa-sqlite FTS5 over an
  OPFS VFS, or a Range-Request read-only DB on a CDN) are all real, first-party-or-well-maintained, and
  match the primary sources. This is a credible reference architecture for a no-backend atproto search
  SPA.
- **One name to drop: "SearchFn."** Treat as non-existent until proven otherwise; substitute
  FlexSearch's own IndexedDB persistence.
- **Numbers to re-derive before quoting:** the Jetstream filtered-bandwidth figure (use ~850 MB/day
  compressed, not 60-150 KB/s), the uFuzzy size (~7.5 KB), and any memory/quota/latency figures from
  the comparison table - those are Gemini benchmarks, not measured here.
- **atproto-fact hygiene:** cite the FACTCHECK as source of truth for MST (atproto's structure),
  non-archival relay (Sync v1.1), and Tap. Do not re-verify those.
