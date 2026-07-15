# Local PDS + Live Bridge (Phase 5)

The live flip — on loopback. A real local atproto **PDS** (speaking the XRPC wire
protocol), a real publishing **client**, a real **WebSocket firehose**, and the
byte-identical **AppView** from Phases 3a/3b, wired end to end over actual TCP
sockets.

```
cargo run     # PDS up -> publish -> firehose -> AppView serves hydrated timeline
cargo test
```

## Why a local PDS

The intended live flip targets a real PDS + live Jetstream. From this environment
that is impossible: the egress proxy allowlists hosts, and `bsky.social`,
`plc.directory`, the Jetstream hosts — and the npm/Docker registries needed to
run the official Bluesky PDS — all return:

```
Host not in allowlist: <host>. Add this host to your network egress settings to allow access.
```

So instead of faking it, this phase builds a **minimal real PDS in Rust** and
runs the entire live data path on loopback. Everything is real except the PDS is
our own minimal implementation rather than bsky's; pointing the client and the
firehose consumer at a live host (once allowlisted + given real creds) is the
only remaining change — `base` and the WS URL.

## What runs, for real, over real sockets

1. **`pds.rs` — a minimal atproto PDS** (axum):
   - `POST /xrpc/com.atproto.server.createSession` — identifier + password → `{did, handle, accessJwt}` (auto-provisions accounts on this throwaway server; honors `ATP_IDENTIFIER`/`ATP_APP_PASSWORD` if set).
   - `POST /xrpc/com.atproto.repo.createRecord` — Bearer auth, owner-only (`repo` must match the session DID), **validates the record against its lexicon**, assigns a TID rkey + a **real CIDv1** (DAG-CBOR/SHA-256), stores it, and appends a commit to the firehose.
   - `GET /jetstream/subscribe` — a **WebSocket firehose** that backfills the commit log (like `subscribeRepos` cursor=0) then holds open for live events, emitting Jetstream-shape JSON.
2. **`publisher.rs` — the client** doing `createSession` + `createRecord` over real HTTP (`reqwest`). This is the exact code path a live PDS run uses; only `base` changes.
3. **`tokio-tungstenite`** consumes the firehose over a **real WebSocket**; frames are mapped by the **byte-identical `JetstreamSource`** from Phase 3b.
4. The **byte-identical `indexer.rs` / `server.rs` / `views.rs`** from Phase 3a index and serve a hydrated `getTimeline`.

## Lifecycle (all 4 steps PASS)

1. Local PDS up on an ephemeral loopback port.
2. Two real sessions (Alice, Bob); Alice publishes 2 posts, Bob publishes a reaction to Alice's post — all via real HTTP `createRecord`. **CID parity**: the PDS-assigned CID equals a locally recomputed real CIDv1, and is a valid atproto CID.
3. A real WebSocket delivers the 3 commit frames; `JetstreamSource` maps them to 3 `RecordEvent`s.
4. The AppView indexes them and serves a hydrated timeline — Bob's 🔥 reaction joined onto Alice's post, distinct author/reactor DIDs — and the output validates against the read lexicon.

## What this proves vs. doesn't

**Proves (real, over sockets):** the full live data path — app-password session
handshake, authenticated owner-only `createRecord` with server-side lexicon
validation, server-assigned rkey + real CIDs with client/server CID parity, a
WebSocket firehose with backfill, and the unchanged AppView consuming it. The
swap surface from 3b (`cid`, operations, cursor, transport) is now exercised over
real I/O, not a synthetic feed.

**Does not prove:** interop with bsky's *actual* PDS — real did:plc registration
and resolution, repo MST/CAR construction and commit signing, OAuth (vs. our
opaque session token), and the real Jetstream service (CBOR `subscribeRepos` →
JSON translation). Those need the live network.

## Resolved versions

rustc 1.94.1 · axum 0.8.9 · tokio 1.52.3 · reqwest 0.13.4 · tokio-tungstenite
0.29.0 · rusqlite 0.32 · cid 0.11.3 · serde_ipld_dagcbor 0.6.4 · serde_json 1.0.150.

## Deviations & friction (honest)

- **Opaque session token, not a signed JWT.** A real PDS issues a signed
  `accessJwt`; we issue an opaque session id since we control both ends. The
  handshake shape (`createSession` → bearer → `createRecord`) is identical.
- **Firehose is Jetstream-shape JSON, not CBOR `subscribeRepos`.** Real PDSes
  emit signed CBOR repo-commit frames; Jetstream is the public JSON translation.
  We emit the JSON shape directly (so the 3b `JetstreamSource` is reused
  verbatim); a live run would point at a real Jetstream instance, which emits
  exactly this shape.
- **No real DIDs / MST / CAR / signing.** DIDs are `did:plc:<hash(handle)>`;
  records are stored as JSON, not in a signed Merkle Search Tree. This phase is
  about the *wire mechanics and data path*, not repo cryptography.
- **Backfill via idle-timeout read.** The consumer reads firehose frames until a
  short idle gap, which works against both our bounded backlog and a real
  infinite firehose. No "done" sentinel is used (real firehoses don't send one).
- **Two `axum` servers + a WS client in one `tokio` runtime** on loopback — the
  realistic shape (PDS and AppView are separate services).

## How to flip this to a live PDS

1. Allowlist `bsky.social` (or your PDS host), `plc.directory`, and a Jetstream
   host in the environment's network egress settings.
2. Provide real creds as env vars (`ATP_IDENTIFIER`, `ATP_APP_PASSWORD`) — do not
   paste them into chat.
3. Point `publisher`'s `base` at the live PDS and the consumer's WS URL at the
   live Jetstream. (OAuth would replace app-password auth for production; the
   `createRecord`/firehose/AppView path is unchanged.)
