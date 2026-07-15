# Source-Agnostic Local AppView (Phase 3a)

An **AppView** — the ingest/index/serve component of an atproto app — built over
the encrypted local-first sync stack from the prior phases. The whole point of
this phase is a **source-agnostic ingest boundary**: the indexer and server
consume records only through a `RecordSource` trait and never touch Automerge,
MLS, encryption, or iroh. We feed it from our own decrypted group document; in
Phase 3b a Jetstream consumer populates the *same* event struct — a swap, not a
rewrite.

```
cargo run     # 5-step lifecycle: setup -> ingest -> serve -> hydrate -> rebuild
cargo test    # lexicon/datetime/comparator unit tests
```

No live PDS, OAuth, DID resolution, Jetstream, CAR decoding, or relay. iroh
transport stays stubbed (content-addressing real), as in every prior phase.

## The three components, with a hard seam at ingest

- **`source.rs`** — `RecordSource` trait + `RecordEvent` (the seam) + `LocalStackSource` (behind it).
- **`indexer.rs`** — SQLite (`rusqlite`, bundled) projection; validates each record against its lexicon on ingest; hydration queries. Zero references to the private stack.
- **`server.rs`** — axum XRPC server at `/xrpc/<nsid>`.
- **`groupdoc.rs`** — group-document layout (behind the seam; see deviation below).
- **`views.rs`** — hydrated `PostView`/`ReactionView` matching the read lexicons.

### The ingest boundary

```rust
pub enum Action { Create, Update, Delete }

pub struct RecordEvent {
    pub action: Action,
    pub did: String,          // author DID (AT-URI authority / repo owner)
    pub collection: String,   // NSID
    pub rkey: String,
    pub cid: Option<String>,  // "b3-…" BLAKE3 stand-in here
    pub record: Value,        // record body JSON (Null for deletes)
    pub observed_at: i64,     // millis (≈ Jetstream time_us)
}

pub trait RecordSource { fn events(&mut self) -> Vec<RecordEvent>; }
```

`RecordEvent` is modeled on a Jetstream **commit** event so the trait has two
interchangeable implementations.

### How closely `RecordEvent` matches a real Jetstream event

A Jetstream commit carries `did`, `time_us`, `kind:"commit"`, and a `commit`
object with `rev`, `operation`, `collection`, `rkey`, `record`, `cid`. We mirror
the essential fields. **Fields a real Jetstream event has that the local source
does not — exactly the surface 3b must fill:**

| Field | Local source | Real Jetstream | 3b work |
|---|---|---|---|
| `cid` | `b3-<blake3>` stand-in | real CIDv1 over DAG-CBOR | add a CID encoder |
| `rev` | absent (no MST/commit log) | repo revision id | carry from commit |
| cursor / sequence | none (one-shot replay) | `time_us` replay cursor | persist + resume |
| `kind` | always a record commit | also `identity` / `account` | handle/ignore non-commits |
| time precision | milliseconds | microseconds | widen field |
| `action` coverage | only `Create` emitted | create/update/delete | emit all (indexer already handles them) |

## Lifecycle (all 5 steps PASS)

1. **Stack setup** (reused): MLS group, two members derive a matching epoch key, an encrypted group document synced with 2 posts (Alice) + 1 reaction (Bob).
2. **Ingest**: `LocalStackSource` emits `RecordEvent`s; the indexer validates each against its lexicon and stores it; a deliberately malformed record (missing `createdAt`) is **rejected**; row count = 3.
3. **Serve + query**: a real axum server is started on an ephemeral port and queried over HTTP with `reqwest` — `getTimeline` and `getPostThread?uri=…`.
4. **Hydration (keystone)**: the timeline genuinely **joins** posts with their reactions — the reacted-to post carries `reactions: [{emoji:"🔥", reactor:<Bob DID>}]`, with `author:<Alice DID>` distinct from the reactor — and both responses **validate against the read lexicons' `output` schemas**.
5. **Rebuild**: the SQLite table is wiped and replayed from the source, yielding **identical** query results — proving the index is a disposable projection.

## Read lexicons

`org.croftc.experiment.feed.getTimeline` and `...getPostThread`, written as real
atproto `query` lexicons: `type: query` with a `params` `parameters` block and a
JSON `output.schema`, with `#postView`/`#reactionView` defs referenced via local
`ref`s. The server returns JSON matching those output schemas (asserted in Step 4
via a `validate_output` that walks the query lexicon).

## Resolved versions

rustc 1.94.1 (≥ 1.80) · automerge **0.7.4** · openmls 0.8.1 / rust_crypto 0.5.1 ·
chacha20poly1305 0.10.1 · blake3 1.8.5 · serde_json 1.0.150 · **rusqlite 0.32**
(libsqlite3-sys 0.30.1, bundled) · **axum 0.8.x** · **tokio 1.x** · **reqwest 0.13.x** ·
iroh 0.98.2 / iroh-blobs 0.102.0 (resolvable, not linked).

## Deviations & friction (honest)

- **`rusqlite` pinned to 0.32, not latest 0.40.** rusqlite 0.40 pulls
  `libsqlite3-sys 0.38.1`, whose build script uses the still-unstable
  `cfg_select` macro and fails to compile on stable rustc 1.94.1. 0.32 uses
  `libsqlite3-sys 0.30.1`; the API we use is identical. (A real future toolchain
  bump or a `cfg_select`-free sys release would lift this.)
- **`reqwest` `.query()` avoided.** With `default-features = false` (no system
  TLS — we only hit `http://localhost`), `RequestBuilder::query` was not
  available, so query strings are built directly with a tiny percent-encoder.
  Server-side decoding is axum's standard `Query` extractor.
- **Group doc is a multi-repo (`did → collection → rkey`), extending the prior
  phase's single `collection → rkey` layout.** An AppView inherently indexes many
  authors, and atproto's unit of authorship is the per-DID repo; our shared CRDT
  document co-mingles authors, so a per-record author dimension is needed to
  synthesize correct AT-URIs and to distinguish post-author from reactor. This is
  the main place the AppView model rubbed against our stack: the private group doc
  is one CRDT object, whereas atproto thinks in per-author repos. Prefixing the
  DID resolves it cleanly and is arguably *more* atproto-faithful for an AppView.
- **AT-URI authority is a synthesized `did:key:z<hex>`** from the member's MLS
  signature public key (the same bytes as the 4-tuple subspace). A real DID
  (did:plc / did:web) arrives in 3b with identity/OAuth.
- **Hydration felt natural**, not forced: a single `subject`-keyed lookup joins
  reactions onto posts. This is good early signal that the record lexicons support
  the queries we want — the reason to do 3a before touching a network.
- **The source-agnostic boundary held cleanly.** `indexer.rs` and `server.rs`
  import only `RecordEvent`, the lexicon validator, and view structs — no
  Automerge / MLS / crypto / iroh. Verified by inspection; the only stack-aware
  code is `LocalStackSource` + `groupdoc`, both explicitly behind the trait.

## What Phase 3b needs (the swap, not a rewrite)

1. Publish a few records to a **real PDS** (atproto OAuth + identity, then
   `com.atproto.repo.createRecord` with a genuine CIDv1 over DAG-CBOR) so they
   exist on the network.
2. Add a **`JetstreamSource: RecordSource`** (e.g. via `rocketman` or an
   `atproto-jetstream` consumer) that connects to a Jetstream instance, filters
   our collection NSIDs, and maps each commit into the existing `RecordEvent`,
   filling the gap fields above (`cid`, `rev`, cursor, `kind`, microsecond time,
   update/delete operations).
3. **No change to `indexer.rs` or `server.rs`.** They already consume
   `RecordEvent`, validate on ingest, and serve hydrated views; that is the whole
   payoff of the boundary.
