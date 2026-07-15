# Jetstream-Fed AppView (Phase 3b, in-discipline)

Feeds the **exact** Phase 3a AppView from the **real AT Protocol Jetstream wire
format** instead of the local encrypted stack — proving the ingest swap promised
in 3a is additive: a new `RecordSource` impl, with **zero** changes to the
indexer or server.

```
cargo run     # 5-step lifecycle: feed -> ingest -> serve -> resume -> swap proof
cargo test    # lexicon/datetime validation tests
```

## Why this is the "in-discipline" 3b

The literal 3b ("publish to a live PDS via OAuth, connect to a live Jetstream
WebSocket") requires outbound network and credentials, and publishing to a
public network is an outward-facing action — contrary to the no-live-network
discipline every prior phase held deliberately. So this phase proves everything
that can be proven **without** the live network:

**Real here:**
- The Jetstream commit-event JSON shape (`did` / `time_us` / `kind` / `commit{rev,operation,collection,rkey,record,cid}`).
- `JetstreamSource: RecordSource` — kind filtering (skips `identity`/`account`), collection filtering, `operation`→`Action`, **create/update/delete**, and **cursor/resume** via `time_us`.
- **Genuine atproto CIDv1** (DAG-CBOR codec `0x71` + SHA-256), via `cid` + `serde_ipld_dagcbor` + `multihash-codetable` — this **closes the `cid` gap** the 3a README flagged (no more `b3-` stand-in; CIDs are real `bafyrei…`).
- `indexer.rs`, `server.rs`, `views.rs` carried **byte-identical** from 3a (asserted at runtime via `include_str!` comparison against `../local-appview/src/*`).

**Still stubbed (the only remaining work — needs your go-ahead + credentials):**
- The live socket/PDS. The feed here is a synthetic NDJSON log of our own
  records in the exact wire shape; pointing the `JetstreamSource` at a live
  Jetstream URL and publishing seed records to a real PDS via OAuth is the final
  flip. No outbound network or credentials are used in this crate.

## Lifecycle (all 5 steps PASS)

0. **Author + encode**: build `feed.post`/`feed.reaction` records, compute their **real CIDs**, and serialize a Jetstream NDJSON feed (2 creates + an `identity` event + a reaction create). Assert the CID is a real CIDv1/dag-cbor.
1. **Ingest**: `JetstreamSource` maps commits → `RecordEvent`, **skipping the identity event** and filtering to our collections; the (unchanged) indexer validates + stores; row count 3; cursor advances.
2. **Serve + hydrate**: the **same** axum server returns a hydrated `getTimeline` (post + reaction joined, distinct author/reactor DIDs) that validates against the read lexicon.
3. **Resume**: a later batch (an `update` to a post + a `delete` of the reaction) is processed by resuming **from the saved cursor** — only the new events are seen; the edit and removal are reflected, and `getPostThread` still validates.
4. **Swap proof**: `include_str!` comparison confirms `indexer.rs`/`server.rs`/`views.rs` are **byte-identical** to Phase 3a.

## `RecordEvent` ← Jetstream mapping (the gaps from 3a, now)

| Field | 3a (LocalStackSource) | 3b (JetstreamSource) |
|---|---|---|
| `cid` | `b3-<blake3>` stand-in | **real CIDv1/dag-cbor/sha256** ✓ closed |
| `action` | only `Create` | create/update/delete ✓ |
| `kind` filtering | n/a | identity/account skipped ✓ |
| cursor/resume | none (one-shot) | `time_us` cursor + `from_cursor` ✓ |
| `did` | synthesized from member key | from the commit event |
| time precision | millis | `time_us` → millis (still narrowed) |
| `rev` | absent | **carried on the wire, not yet indexed** |

Remaining true gaps for the *live* flip: a real WebSocket transport + reconnect,
real DIDs from identity events / OAuth, and using `rev` for repo-consistency
checks. None require indexer/server changes.

## Resolved versions

rustc 1.94.1 · rusqlite 0.32 (libsqlite3-sys 0.30.1, bundled) · axum 0.8.9 ·
tokio 1.52.3 · reqwest 0.13.4 · **cid 0.11.3** · **serde_ipld_dagcbor 0.6.4** ·
**multihash-codetable 0.2.2** · serde_json 1.0.150. Prior phases unchanged
(automerge 0.7.4 / openmls 0.8.1 / chacha20poly1305 0.10.1).

## Deviations & friction (honest)

- **No live network** (deliberate, see above). The synthetic feed is the
  legitimate stand-in; the wire *format* and all mapping logic are real, so the
  live `JetstreamSource` would reuse this `events()` body verbatim over socket
  frames instead of `Vec<String>` lines.
- **`rusqlite` pinned to 0.32 / `reqwest` `.query()` avoided** — same toolchain
  constraints as 3a (see that README); carried forward unchanged.
- **DAG-CBOR over `serde_json::Value`**: our records contain only
  strings/arrays/objects (no floats), so the JSON→DAG-CBOR round-trip is
  lossless and the CID is stable. A record with floats/bytes would need care
  (DAG-CBOR forbids non-canonical floats); not an issue for these lexicons.
- **`rev` is carried but not used.** Jetstream `rev` enables repo-consistency /
  ordering checks; the indexer orders by rkey/TID and doesn't need it yet. Left
  as explicit future surface.
- **The swap held with literally zero downstream edits** — the strongest
  possible confirmation of the 3a boundary, checked by the build itself.

## What the live flip needs (not built; needs go-ahead + credentials)

1. Publish seed records to a real PDS (atproto OAuth + `com.atproto.repo.createRecord`).
2. Replace the synthetic feed with a `tokio-tungstenite` WebSocket to a live
   Jetstream endpoint (with reconnect + cursor persistence). The `events()`
   mapping body is unchanged.
3. Resolve real DIDs (did:plc/did:web) from identity events.

All three are transport/auth/identity additions; `indexer.rs` / `server.rs` /
`views.rs` / the lexicons stay as they are.
