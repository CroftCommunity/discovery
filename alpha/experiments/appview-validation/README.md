# Experiment: Minimal AT Protocol AppView, validated against the real public network

A **self-contained** Rust experiment that builds the simplest *complete* AT Protocol
AppView and validates it against the **real public atproto network** via Jetstream.
It depends on no prior experiment.

> **An AppView**, stated without outside context, is the indexing-and-serving half of
> an atproto app. It does three things: **ingest** records from a network event stream,
> **index** them into its own query-optimized store (a disposable projection — the
> authoritative copy lives in each user's PDS), and **serve** a read API (XRPC queries)
> returning hydrated views. This experiment builds all three, minimally, and points
> ingest at the live network.

The primary objective is **learning and validation, not feature breadth**. The headline
output is the field-by-field comparison of a *predicted* event shape against the real
Jetstream wire shape, plus the observed event rate and an honest friction log.

---

## Summary (goal · approach · effort · result)

*For reasoning about this experiment relative to other work — read this section alone for
the gist; everything below it is the supporting evidence.*

**Goal.** Learn how the AT Protocol (atproto) actually behaves by building the smallest
*complete, real* AppView and progressively stress it against the **live public network**,
treating every divergence between what we'd assume from docs and what reality returns as
the primary deliverable. Not a product; a learning instrument.

**Approach.** A single Rust crate, grown in phases, each an independently runnable binary
that reuses a shared core (a `RecordSource` ingest trait, a disposable SQLite index, an
XRPC-shaped server, atproto read/write helpers). Where a phase tests an assumption, we
**write the prediction down first, then run it against reality and report the gap**
(hypothesis-validation). Everything that *can* run against the real network *does* —
stubs only where a real run is impossible (and those are flagged).

**Effort.** ~1 day, one engineer-equivalent, ~2,400 lines across 7 binaries + shared
modules. Dependencies deliberately lean (tokio, tokio-tungstenite, rusqlite, axum,
reqwest, ciborium). Runs on stable Rust 1.94; no infrastructure beyond outbound network.

**Result.** A working ingest→index→serve pipeline proven against the live network, plus
the full write→firehose→index loop, the canonical backfill→live-tail bootstrap, crash
recovery, a live trending feed with identity hydration, and two hypothesis-validations
(labeler stream, full-firehose scale). The concrete learnings (below) repeatedly
contradicted reasonable assumptions — which is the point.

| # | Phase (binary) | Network? | Effort | Headline result |
|---|----------------|----------|--------|-----------------|
| 1 | read path (`appview-validation`) | live | M | ingest→index→serve works; `cid` is at `commit.cid` not top level; `identity`/`account` frames leak through collection filters |
| 2 | publish loop (`publish`) | live + creds | M | write→firehose→index loop closes; **custom NSIDs propagate with no pre-registration** |
| 3 | local proof (`local`) | none | M | typed comprehension, backfill, label-filtering, and batching (~120× row-by-row) proven deterministically |
| 4 | lifecycle (`bootstrap`) | live + creds | L | backfill→live-tail with **zero gap**; cursor-persistence crash recovery with **no loss** |
| 5 | trending feed (`feed`) | live | M | real trending from the like firehose, hydrated DID→handle + post text |
| 6 | labeler (`labeler`) | live | M | labeler stream is **binary DAG-CBOR**, signed labels, `seq` cursor — *not* JSON like Jetstream |
| 7 | scale (`firehose`) | live | M | full firehose ~357 ev/s; single node has **~64× headroom** → distribution premature |
| 8 | viewer-aware serving (`authserve`) | live (P-A3) + creds (P-A1/2) | M | **the AppView learns who its caller is** — atproto service-auth JWTs verified against real DID-doc keys (secp256k1/p256); `getProfileView` gates `openToWork` by verified recruiter identity; verified reads emit telemetry. Live P-A3 confirmed; P-A1/A2 blocked on creds. (RUN-14 EXP-A) |
| 9 | sealed offer-gating (`sealed`) | none | M | **§H hybrid serve half**: content-blind store offers ciphertext only to verified roster members (flat 403 otherwise, no length/existence leak); blindness is a **compilation boundary** (the AEAD crate is absent from the server's dep graph); roster gates offering, encryption alone gates reading. (RUN-14 EXP-B) |

**The through-line learning:** designing against atproto docs/tutorials diverges from
reality in small, expensive ways (field placement, event kinds, CBOR vs JSON, cursor
semantics), and several "you'll need X at scale" assumptions (distribution, custom-lexicon
gating) are **premature** — the network is more tractable and more permissive than assumed.
What's genuinely hard is the operational discipline (never lose your cursor; backfill+tail
handoff; CBOR + signatures for moderation), not raw throughput.

**What's NOT proven** (so this can be compared honestly): interactive OAuth/DPoP (the PWA
client-login leg — a browser hop this env lacks; named non-goal, RUN-14), label signature
*verification*, the raw repo firehose (CAR/MST), a genuinely distributed ingester, and
production-grade ranking/pagination/durability. **Now proven (RUN-14):** the AppView learns
its caller via service-auth JWTs (phase 8), and offers sealed records without holding the
key (phase 9). See *What this validates* at the bottom.

---

## Run it

Nine independently runnable binaries (phases 8–9 added RUN-14: `authserve`, `sealed`;
run `cargo run --bin authserve` and `cargo run --bin sealed`, and the unit suites
`cargo test --lib serviceauth viewserve` and `cargo test --lib --features client-seal sealed`).
See the phase sections for what each proves:

```bash
cd experiments/appview-validation

# 1) read-path demo: live ingest -> index -> serve (no credentials needed)
cargo run --bin appview-validation

# 2) publish-loop: write -> network -> index (needs a test account)
BSKY_IDENTIFIER='you@example.com' BSKY_PASSWORD='app-password' cargo run --bin publish

# 3) local proof: typed comprehension, backfill, moderation/labels, scale (no network)
cargo run --bin local

# 4) lifecycle: backfill->live-tail no-gap + cursor crash recovery (needs a test account)
BSKY_IDENTIFIER='you@example.com' BSKY_PASSWORD='app-password' cargo run --bin bootstrap

# 5) live trending feed with identity/content hydration (no credentials)
cargo run --bin feed

# 6) subscribed-labeler hypothesis vs reality (no credentials)
cargo run --bin labeler

# 7) full-firehose scale: do you need a distributed ingester? (no credentials)
cargo run --bin firehose
```

The read-path demo requires outbound access to `wss://jetstream2.us-east.bsky.network`.
The run is bounded (≤200 post commits / 60s, then ≤300 like commits / 20s) and terminates
cleanly. It recreates `appview.sqlite` from scratch each time. The publish loop
additionally needs `https://bsky.social` reachable and credentials supplied **only via
env vars** (never hardcoded/committed; use a throwaway account and rotate after).

## Architecture (three components, clean ingest seam)

| Component | File | What it does |
|-----------|------|--------------|
| **Ingest** | `record_source.rs`, `jetstream.rs` | `RecordSource` trait yields normalized `RecordEvent`s; `JetstreamSource` is a real WebSocket consumer of the live public Jetstream, server-side filtered by collection NSID. |
| **Index** | `index.rs` | Disposable SQLite projection keyed by AT-URI; handles create (upsert) / update (replace) / delete (remove). |
| **Serve** | `server.rs` | axum HTTP server exposing two XRPC-shaped read queries, one of which is a genuine hydration (aggregation). |
| **Reality check** | `report.rs` | Diffs the predicted `RecordEvent` against the first real event, field by field. |

The ingest source sits behind a `RecordSource` trait on purpose: that seam is both good
architecture and a direct test of the hypothesis — *does a real network event populate
the same structure a local/stub source would?*

## Resolved versions (full pin in `Cargo.lock`)

```
rustc              1.94.1 (edition 2024)
tokio              1.52.3
tokio-tungstenite  0.29.0   (default-features off; connect, handshake, rustls-tls-native-roots)
tungstenite        0.29.0
rustls             0.23.40  (ring provider 0.17.14, installed at startup)
rusqlite           0.32.1   (bundled libsqlite3-sys 0.30.1)
axum               0.8.9
serde / serde_json 1.0.228 / 1.0.150
futures-util       0.3.32
anyhow             1.0.102
```

### Ingest-crate choice (and a deliberate deviation from the brief)

The brief suggested a dedicated Jetstream consumer crate (`atproto-jetstream`,
`rocketman`). **I deliberately did not use one.** I consume Jetstream directly via
`tokio-tungstenite` + `serde_json`. Rationale:

1. **The headline goal is a hypothesis-vs-reality field comparison.** A crate that
   deserializes events into *its own* event struct would pre-bake the very modeling
   decisions I am trying to test, masking the gap I want to measure. Parsing raw JSON
   keeps `RecordEvent` an honest prediction diffed against the real wire shape.
2. **Avoids third-party API churn** in a fast-moving ecosystem.
3. The WebSocket handshake was verified manually in preflight before committing to this.

This is logged as the kind of "pick the simplest option that keeps the real Jetstream
ingest genuinely real, and document it" choice the brief sanctions.

---

## Connectivity preflight (the gate, run before any feature code)

| Check | Result |
|-------|--------|
| DNS `jetstream2.us-east.bsky.network` | resolves (IPv6 `2604:2dc0:...`) |
| TLS connect :443 | OK (~15ms) |
| WS upgrade `/subscribe?wantedCollections=app.bsky.feed.post` (HTTP/1.1) | `HTTP/1.1 101 Switching Protocols`, live events streamed |
| crates.io registry | reachable |

**PASSED** — the experiment runs against reality, no stubbing.

---

## HEADLINE RESULT: hypothesis vs reality, field by field

`RecordEvent` (in `record_source.rs`) was written as a developer's prediction of a
Jetstream commit event *before* the reality check. Here is a representative real event
and how the predictions fared:

```json
{
  "did": "did:plc:qirul6ign6wrvkjluj3624kk",
  "time_us": 1781352361598278,
  "kind": "commit",
  "commit": {
    "rev": "3mo67pfkigg2k",
    "operation": "create",
    "collection": "app.bsky.feed.post",
    "rkey": "3mo67pfdnk22f",
    "cid": "bafyreia3ohoudaqsynj4mstdygp46bysjvxyb35p4snd5gdyuotqqsxohm",
    "record": { "$type": "app.bsky.feed.post", "createdAt": "...", "text": "...", "reply": {...} }
  }
}
```

| # | Prediction | Verdict | Reality |
|---|-----------|---------|---------|
| P1 | event carries action, did, collection, rkey, record, cid, cursor | **mostly HELD** | all present, but nested differently (see P2) |
| P2 | `cid` is at the event top level next to `did` | **WRONG** | `cid` lives at **`commit.cid`**, inside the commit object — *not* top level. The DID/time_us/kind are top level; everything else is under `commit`. |
| P3 | a cursor field `time_us` exists; units unknown | **HELD** | `time_us` present; value ÷ 1,000,000 → a plausible 2026 unix-seconds → **units are microseconds since epoch**. |
| P4 | every frame on a commit stream is `kind:"commit"` | **WRONG** | even on a **collection-filtered** stream, `kind:"identity"` and `kind:"account"` frames arrive (2 and 1 in one ~80s window). Server-side `wantedCollections` filtering does **not** suppress identity/account events. |
| P5 | a `delete` carries no record body and no cid | **HELD** (confirmed) | delete frames have `operation:"delete"`, no `record`, no `cid`. |
| P6 | `record.createdAt` is an ISO timestamp string | **HELD** | present and ISO-8601 — *but with real-world variety in precision* (some `...153Z` millis, some `...384050Z` micros). |

**Present-but-unmodeled real field:** `commit.rev` — the repo revision (a TID). My model
ignored it; a real AppView uses it for per-repo ordering and dedup.

This table is the concrete map of where designing-against-docs diverges from
designing-against-reality. The two that would have bitten a real build: **P2** (you'd
read `cid` from the wrong place and silently get nulls) and **P4** (your "commit"
handler must tolerate non-commit frames or it crashes / mislabels them).

---

## Observed event rate (real volume data point)

| Stream (server-side filtered) | Sample | Rate |
|-------------------------------|--------|------|
| `app.bsky.feed.post` | 200 commits in **~4.9s** | **~40 commits/s** |
| `app.bsky.feed.like` | 300 commits in **~1.6s** | **~185 commits/s** |

Both bounds (200 / 300 commits) were hit almost immediately — the windows never came
close to expiring. **`app.bsky.feed.like` alone runs ~4–5× the post rate.** Implication
for a real AppView: even a *single* high-volume collection is a firehose torrent; the
public-facing ingest path must assume buffering / backpressure and cannot block on
synchronous per-event indexing at scale. (These numbers are a snapshot; rate varies with
time of day and is not the full network — only the filtered collection.)

---

## Per-step results (last run)

| Step | Result |
|------|--------|
| 1. versions + connectivity | reported; handshake `HTTP 101` confirmed at runtime |
| 2. live ingest (posts) | 200 commits; first raw event dumped beside normalized model; shape report produced |
| 3. index | 193 post rows (200 commits = 193 creates + 7 deletes); deletes wired but removed 0 rows **because the deleted posts predated our window and weren't in the index** — an honest, expected result of a bounded live capture |
| 3b. likes capture | 295 like rows from 300 commits |
| 4–5. serve + query | axum on `127.0.0.1:<port>`; real HTTP GET to `getRecentPosts` returns hydrated JSON (author, text, createdAt, replyTo) |
| 6. hydration proof | `getLikeCountsBySubject` `GROUP BY subject_uri` — returned a subject with `likeCount: 2`, i.e. two distinct like records joined into one view row (not an echo of stored data) even within a 1.6s window |
| 7. disposable projection | index recreated from scratch each run; rebuild = re-run ingest |

Aggregate: 503 frames → 500 commits (488 creates, 12 deletes) + 3 non-commit
(`identity`×2, `account`×1), **0 malformed**.

---

---

## Publish phase — closing the write → network → index loop (`--bin publish`)

The first phase validated only the *read* path. This phase authenticates to a real PDS,
publishes records, and confirms each one comes **back through our own Jetstream indexer** —
proving the full `write → PDS → firehose → Jetstream → AppView index` loop against the
live public network. Credentials are read **only** from `BSKY_IDENTIFIER` /
`BSKY_PASSWORD` env vars (never hardcoded, printed, written to disk, or committed; the
access JWT lives in memory only).

Steps and last-run result (against a throwaway test account):

| Step | XRPC call | Result |
|------|-----------|--------|
| P1 | `com.atproto.server.createSession` | authenticated; got `did:plc:…` + accessJWT (not printed) |
| P2 | `com.atproto.repo.createRecord` (`app.bsky.feed.post`) | published `at://…/app.bsky.feed.post/<rkey>` with a unique marker |
| P3 | watch Jetstream (DID+collection filtered, **cursor-replayed** from ~10s pre-publish) | **LOOP CLOSED** — our own post observed on the live firehose and indexed, early-stopped instantly |
| P4 (stretch) | `createRecord` with **custom NSID** `org.owasp.validation.note` | published, then **also observed flowing on Jetstream** |
| P5 | `com.atproto.repo.deleteRecord` ×2 | cleaned both records off the test account |

### New findings from the publish phase

- **The full write→read loop works against reality.** A record created via `createRecord`
  reliably appears on Jetstream within a couple of seconds and is indexable by the exact
  same pipeline used for the read-path demo — the `RecordSource` seam paid off (the
  publish watcher reuses `JetstreamSource` unchanged, just with DID + cursor filters and
  an early-stop callback).
- **Custom-lexicon records flow on Jetstream with no pre-registration.** This directly
  answers the question the read-only phase deferred: publishing a record under a brand-new
  custom NSID (`org.owasp.validation.note`) and subscribing `wantedCollections` to that
  NSID returned the record. **The firehose/Jetstream layer is collection-agnostic** — it
  relays any collection a repo commits; lexicon "registration" is a convention for
  *consumers/AppViews*, not a gate the network enforces. (What you don't get for free is
  anyone else's AppView *understanding* the record — only propagation.)
- **The replay cursor is the right tool for catch-after-publish.** Connecting *after*
  `createRecord` would race the event; replaying Jetstream from `published_us − 10s`
  guarantees capture without missing the live tail. Both records were caught in ~0s.
- **A DID-filtered subscription is effectively idle**, so without the early-stop callback
  (added to the `RecordSource` trait this phase) the watcher would block the full window
  after already seeing its target.

---

## Local proof — the four remaining gaps, proven locally (`--bin local`)

The network phases left four things unproven. This binary proves each **locally and
deterministically** (no network, no credentials), reusing the same `Index` and the same
`RecordSource` seam — a `FixtureSource` is swapped in for Jetstream, demonstrating the
pipeline is genuinely source-agnostic.

```bash
cargo run --bin local
```

| # | Gap | What's demonstrated | Last-run result |
|---|-----|---------------------|-----------------|
| **L1** | custom-lexicon **comprehension** | deserialize stored records into a typed `ValidationNote` and **reject** non-conforming ones (vs raw storage accepting everything) | 5 stored → **2 typed-valid, 3 rejected** (missing field; wrong type `"NaN"`; `$type` mismatch) |
| **L2** | **PDS backfill** | a bounded *historical* batch fed through the same pipeline (not live tail) | **25 historical posts** backfilled via `FixtureSource` |
| **L3** | **moderation / labels** | a `labels` side-table joined at read time; the served view filters labeled rows | view **25 → 23** with moderation (2 hidden by `spam` / `!hide`) |
| **L4** | **scale** | batched-transaction indexing vs naive row-by-row autocommit | **200 rows/s** row-by-row vs **~24,000 rows/s** batched ≈ **~120× faster** |

### What each result actually teaches

- **L1 — comprehension ≠ propagation.** The publish phase proved a custom NSID *flows*;
  here a consumer that knows the schema turns opaque JSON into a typed view and rejects
  malformed records. `$type` checks + serde required-field/type validation are the whole
  mechanism. *Raw storage accepted all 5; comprehension is the layer that enforces the
  contract.*
- **L2 — backfill is just a different `RecordSource`.** Swapping Jetstream for a bounded
  historical source needs zero pipeline changes — the payoff of the trait seam. The real
  implementation is `com.atproto.repo.listRecords` paginated by cursor (or a CAR export);
  the *shape* (bounded replay into the same index) is what's proven here.
- **L3 — labels are read-time join, not re-index.** Moderation lives in a side table; the
  moderated XRPC view is a `NOT IN (SELECT … FROM labels …)`. Re-labeling changes the view
  without touching indexed records — matching how atproto separates labelers from AppViews.
- **L4 — batching is mandatory at network scale.** Row-by-row autocommit fsyncs per insert
  (~200 rows/s here); one transaction does ~24k rows/s — **~120×**. The live `like` stream
  alone ran ~185 ev/s earlier, so a real ingester must batch writes and absorb bursts
  (buffer/backpressure) rather than commit per event. This is the concrete form of the
  "single collection is a torrent" finding.

### Honest limits of the local proof

These prove the **mechanisms** locally, not their production forms: backfill uses a
fixture, not a live `listRecords` crawl with real pagination/rate-limits; labels come from
a synthetic labeler, not a subscribed atproto labeler service; and the scale number is
single-process SQLite on this machine, not a distributed ingester. They establish the
designs are sound and show where the real work (network pagination, labeler subscription,
write sharding/queues) would go.

---

## AppView lifecycle, against the live network (`--bin bootstrap`)

The single most important property of a firehose consumer is that it **never loses its
place**. This binary proves the two halves of that against the live network, using the
test account (publishes then `deleteRecord`-cleans its own records).

### #1 — Backfill → live-tail with no gap

The canonical AppView bootstrap: crawl a repo's history, then keep up with the firehose
*without missing anything that arrived during the crawl*.

1. Take a checkpoint cursor **before** backfill.
2. Backfill the repo via `com.atproto.repo.listRecords` (paginated — page size 2 forced
   multiple pages).
3. Publish more records **after** backfill (the gap-risk events).
4. Live-tail Jetstream **from the pre-backfill checkpoint**.

Last-run result: backfilled **3** history records over **3 pages**; live-tail (replaying
from the checkpoint) saw all **5**; the **2** post-backfill posts were caught — **gap = 0**;
overlap (3 records in both paths) was **deduped by upsert**; final unique posts = **5**.
→ *Backfill alone would have missed the 2 late arrivals; resuming the live tail from a
pre-backfill checkpoint closes the gap, and idempotent upsert makes the overlap harmless.*

### #2 — Cursor persistence + crash recovery

1. Consume, **checkpointing the cursor to disk** (`cursor_state` table) after each event,
   until processing post **A**, then "crash" (stop).
2. Publish post **B** during the downtime.
3. Restart **from the persisted cursor**.

Last-run result: **B (published during downtime) was captured after resume** — no loss. A
is re-delivered (at-least-once), and the upsert index makes replay idempotent.
→ *This is why a real AppView persists its cursor: a crash must cost reprocessing, never
data loss.*

**Bonus learning:** writes are accepted by the **entryway** (`bsky.social`), but repo
**reads** (`listRecords`/`getRecord`) must hit the account's **real PDS**, resolved from
its DID document — last run: `stropharia.us-west.host.bsky.network`. The entryway vs PDS
split is invisible until you read a repo directly.

## Live trending feed with hydration (`--bin feed`)

What an AppView does *beyond* storage: turn opaque `at://did/...` references into a view a
human can read. No credentials — pure public read path.

1. **#3 Trending:** ingest `app.bsky.feed.post` + `app.bsky.feed.like` together for ~25s,
   then rank the most-liked subjects observed (real feed-generator logic over the live like
   firehose).
2. **#4 Hydration:** for each trending post, resolve author **DID → handle + displayName**
   (PLC directory + `app.bsky.actor.profile`) and fetch the **post text**
   (`com.atproto.repo.getRecord`).

Last-run result: **4,996 likes + 958 posts in 25s** (~238 commits/s combined); top subject
had **16 likes**; all 8 trending posts hydrated to real authors + readable text, e.g.:

```
#1  ♥ 16   @dennycarter.bsky.social (Denny Carter)
     "Listened to an NPR story this morning about mass child starvation in Senegal…"
     at://did:plc:ahcborosqrfchi3iiyhoixch/app.bsky.feed.post/3mo6avzxnas22
```

→ *This also fixes the earlier "likes rarely reference posts we indexed" gap: trending
subjects are hydrated **on demand by fetching**, not by hoping they're in our window.*
Image-only posts hydrate with empty text (handled gracefully, not crashed).

---

## Hypothesis-validation: subscribed labeler service (`--bin labeler`)

Two of the "still unproven" items were worth a *hypothesis-validating* pass rather than a
full build — they teach the most precisely where reality diverges from the assumption.

The hypothesis (a developer spoiled by Jetstream's clean JSON):
- **H1** label events are JSON over WebSocket *text* frames;
- **H2** a label ≈ `{ src, uri, val, cts }`;
- **H3** the cursor is time-based (`time_us`);
- **H4** one object per message.

We connect to Bluesky's **real** moderation labeler (endpoint resolved from its DID
document's `#atproto_labeler` service) and compare. Last-run reality:

| H | Verdict | Reality |
|---|---------|---------|
| H1 | **WRONG** | **Binary** WS frames carrying **DAG-CBOR** — the `com.atproto.sync` framing: *two concatenated CBOR objects*, a `{op,t}` header (`t="#labels"`) + body. **Jetstream is a value-add JSON proxy over exactly this**; labelers expose the raw CBOR stream. |
| H2 | **PARTIAL** | `src/uri/val/cts` exist, but a real label also carries `ver` (versioning), a **64-byte cryptographic `sig`** (every label signed), and an optional **`neg`** (negation/retraction). |
| H3 | **WRONG** | cursor is **`seq`**, a monotonic sequence integer — not a timestamp. |
| H4 | **WRONG** | every message is header+body framed. |

Observed label taxonomy in one window (60 labels): `nudity`×35, `porn`×11, `corpse`×5,
`self-harm`×5, `gore`, `impersonation`, `sexual`, `spam`. → *The big lesson: the firehose
family speaks CBOR; the friendly JSON we built everything else on is a convenience layer,
and consuming labels (or the raw repo firehose) means DAG-CBOR + signature verification.*

## Hypothesis-validation: distributed ingester at full-firehose scale (`--bin firehose`)

The hypothesis: **HS1** the full unfiltered firehose is fast (~2,000 ev/s); **HS2** a
single SQLite node can't keep up; **HS3** therefore you need a distributed/sharded
ingester. We measured: this machine's batched-insert ceiling, then ran the **full
unfiltered Jetstream** through a realistic ingester (bounded queue + batching consumer =
real backpressure) for 15s.

Last-run reality:

| HS | Verdict | Evidence |
|----|---------|----------|
| HS1 | **WRONG (lower)** | full firehose was **~357 events/s, ~0.2 MB/s** — *far* below the assumed 2,000. (One public Jetstream instance, this window; varies by time of day.) |
| HS2 | **REFUTED** | one node kept up trivially: **lag 0**, max queue depth **23 / 2048**, **zero** backpressure stalls. |
| HS3 | **PREMATURE** | headroom = capacity / rate ≈ **23,000 / 357 ≈ 64×**. A single batched node has ~64× headroom; distribution is justified by HA/redundancy, multi-collection and hydration fan-out — **not** raw insert throughput, until ~64× growth. |

**Honest secondary finding:** at 357 ev/s the consumer was "write-busy" ~74% of wall time
— not because it's saturated, but because the batch rarely fills, so it commits ≈ per
event (the per-transaction fsync tax). *Time-based* batching (flush every N ms) would
collapse that; it's the real reason naive low-volume ingesters feel slow.

## Honest friction & surprise log

- **rustls crypto provider panic (blocking).** `rustls 0.23` panics at first TLS use
  ("Could not automatically determine the process-level CryptoProvider") when feature
  flags don't unambiguously select one — which happens with `tokio-tungstenite`
  `default-features = false`. Fix: add `rustls` with the `ring` feature and call
  `rustls::crypto::ring::default_provider().install_default()` once at startup. Invisible
  when designing against docs; a hard stop in practice.
- **Producer/consumer queue-depth gauge underflowed.** Tracking depth with an unsigned
  counter (`fetch_add` on enqueue, `fetch_sub` on dequeue) panicked with "attempt to add
  with overflow": the consumer can dequeue and `fetch_sub` *before* the producer's
  post-send `fetch_add` runs (the item lands in the channel before the gauge is bumped),
  underflowing `usize` to a huge value. Fix: a signed `AtomicI64` where the transient
  negative is harmless. A textbook lost-update race, invisible until volume makes it likely.
- **reqwest `rustls-tls` (webpki bundled roots) rejected `bsky.social`** with
  `invalid peer certificate: UnknownIssuer`, even though the OS trust store (curl) accepts
  it. Fix: switch the feature to `rustls-tls-native-roots` so reqwest uses the system CA
  store — consistent with the WS client. A reminder that "rustls" alone doesn't imply a
  trust anchor source.
- **`libsqlite3-sys 0.38` won't build on stable 1.94.1.** Latest `rusqlite 0.40` →
  `libsqlite3-sys 0.38`, whose build script uses the unstable `cfg_select!` macro
  (E0658). Pinned `rusqlite = 0.32.1` (→ `libsqlite3-sys 0.30.1`) to build on stable.
  A real "the ecosystem moves faster than the stable toolchain" tax.
- **`wantedCollections` does not filter event *kinds*.** Surprise that directly refuted
  P4: identity/account frames still arrive on a collection-filtered subscription. A naive
  `match kind { "commit" => ... }` with an `unwrap`/`unreachable` elsewhere would crash.
- **`cid` placement (P2).** The most likely silent bug for a real builder — `cid` is
  nested in `commit`, not top level.
- **`createdAt` precision varies** (millis vs micros) — real data is messier than a
  single assumed format; parse leniently.
- **Deletes reference records you don't have.** With a bounded capture, delete frames
  almost always target older records absent from your index, so `DELETE` removes 0 rows.
  Correct behavior, but a reminder that without backfill your view is a recent-window
  slice, not a complete mirror.
- **WebSocket framing in `curl`.** During preflight, raw `curl -N` output interleaved
  binary WS frame headers with the JSON payloads — a reminder that the WS frame layer is
  separate from the JSON; the tungstenite client handles deframing and the payloads are
  clean.
- **No malformed frames** in the windows observed — real public data was well-formed for
  these two common collections (the lenient parser was never exercised on bad input, but
  the path exists).

---

## What this validates — and what's still unproven

**Validated (read path):** a minimal **ingest → index → serve** pipeline consumes real
records off the live public atproto network via Jetstream, indexes them into a disposable
SQLite projection, and serves a genuinely hydrated view over an XRPC-shaped HTTP query —
and the real event shape, rate, and behavior are now mapped against what a developer would
assume.

**Validated (write path, `--bin publish`):** authenticating to a real PDS and publishing a
record (`createSession` + `createRecord`) is proven, and the published record **round-trips
through the live firehose into our own indexer**. **Custom-lexicon records are confirmed to
flow on Jetstream** with no pre-registration. Records are cleaned up with `deleteRecord`.

**Validated locally (`--bin local`):** **custom-lexicon comprehension** (typed views +
rejection of non-conforming records), **moderation/labels** (read-time filtering), and
**scale** (batched indexing ~120× row-by-row) — see *Local proof* for caveats.

**Validated against the live network (`--bin bootstrap`):** the AppView **bootstrap
lifecycle** — real `listRecords` **backfill** with cursor pagination handed off to a
**live tail from a pre-backfill checkpoint with zero gap**, and **cursor persistence +
crash recovery** with no data loss across a simulated crash (at-least-once + idempotent
upsert). Plus DID-document **PDS resolution** (entryway-write vs real-PDS-read split).

**Validated against the live network (`--bin feed`):** a **real-time trending feed** from
the live like firehose, with **identity + content hydration** (DID → handle/displayName via
PLC + profile; post text via `getRecord`) — opaque `at://` references rendered human-readable.

**Hypothesis-validated against reality (`--bin labeler`, `--bin firehose`):** the
**subscribed labeler** stream (real Bluesky moderation labeler — binary DAG-CBOR, signed
labels, `seq` cursor) and the **full-firehose scale** question (single node has ~64×
headroom; distribution is premature for throughput). See the two hypothesis sections above.

**Still unproven (honest remaining gaps):**

- **OAuth.** All authenticated calls use app/account-password auth (`createSession`), not
  the atproto OAuth flow (auth-server discovery, DPoP, token refresh).
- **Label signature *verification* and a labeler-backed moderation pipeline.** We *read*
  signed labels but don't verify the `sig` against the labeler's `#atproto_label` key, nor
  feed them into the serving filter end-to-end.
- **Raw repo firehose (`com.atproto.sync.subscribeRepos`)** — CAR/CBOR/MST decoding (the
  labeler proved we *can* read this framing; the repo firehose is the heavier cousin).
- **A genuinely distributed ingester** — single-node headroom is measured; HA/redundancy,
  sharding, and behavior at ~64× growth are not built.
- **Feed ranking** beyond like-count/chronological, pagination beyond a simple limit, auth
  on read XRPC endpoints, multi-user, and persistence/durability guarantees.

## Scope discipline (what was intentionally NOT built)

No **OAuth** (the publish binary uses app/account-password `createSession`, not the OAuth
flow), no DID resolution beyond what events already carry, no raw firehose/CAR/CBOR
decoding, no relay, no moderation/labels, no ranking beyond chronological, no pagination
beyond `limit`, no auth on the read XRPC endpoints, no multi-user, no persistence
guarantees, no frontend beyond JSON. Single-file disposable SQLite. Bounded captures so
runs terminate cleanly. _(The publish loop, `--bin publish`, deliberately steps beyond the
original read-only scope to validate the write→network→index loop end-to-end.)_
