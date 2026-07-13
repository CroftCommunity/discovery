# Standalone Experiment: Public AT Protocol Round-Trip

Publish a **custom-lexicon** record to a **real** Personal Data Server (PDS),
then consume it back off the **public Jetstream** network and serve it from a
minimal local AppView. This validates the public path of an AT Protocol app
end-to-end on live infrastructure, and surfaces the real-world friction the
stubbed experiments could not.

This crate is self-contained: no private-sync, encryption, MLS, Automerge, or
iroh. The only conceptual link to other experiments is the shared experimental
lexicon (`org.croftc.experiment.feed.post`) so results can be compared.

> ⚠️ **This experiment touches live, external, public infrastructure.**
> - Use a **dedicated throwaway** test account, never a personal/corporate identity.
> - Records published are **public**. Use innocuous test content only.
> - Run `cleanup` at the end to delete the test records.

## Status (full live run: 2026-06-13)

| Stage | What | Status |
|---|---|---|
| Gate | Toolchain + connectivity preflight | ✅ **PASSED** (live) |
| 1 | Define lexicon + validator | ✅ done |
| 2 | Login to real PDS | ✅ **app-password path** (OAuth infeasible headless) |
| 3 | `createRecord` to real PDS | ✅ 3 records published |
| 4 | Catch records back via Jetstream | ✅ **3/3, CID matched** |
| 5 | Index + serve via AppView | ✅ timeline served |
| 6 | `deleteRecord` cleanup | ✅ deleted; deletes propagate |

**The headline finding is confirmed:** records authored and published locally via
app-password auth to a real PDS came back to us off the public Jetstream network
with matching AT-URI **and** CID, and were then served from our own minimal
AppView. See [Live run results](#live-run-results-2026-06-13) below.

## Toolchain & resolved versions (this run)

- `rustc` / `cargo` **1.94.1** (≥ 1.80 required ✅)
- Resolved crate versions (pinned in `Cargo.lock`, committed):
  - `reqwest` 0.13.1 · `tokio` 1.52.3 · `tokio-tungstenite` 0.29.0 · `rustls` 0.23.40
  - `rusqlite` 0.32.1 (`libsqlite3-sys` 0.30.1, bundled) · `axum` 0.8.9
  - `serde_json` 1.0.150 · `chrono` 0.4.45 · `clap` 4.6.1

Notes on version friction (the atproto Rust ecosystem and its TLS deps move fast):
- `reqwest` 0.13 **renamed/feature-gated** TLS and query: needed `rustls` (not
  `rustls-tls`) and an explicit `query` feature for `.query(...)`.
- `rusqlite`/`libsqlite3-sys` **latest (0.40/0.38) failed to compile** — its build
  script uses the unstable `cfg_select` feature. Pinned `rusqlite` to 0.32.
- `tokio-tungstenite`'s `rustls-tls-webpki-roots` produced `UnknownIssuer` on the
  Jetstream TLS handshake; switching to `rustls-tls-native-roots` (system CA store)
  fixed it. `reqwest` also uses native certs (`rustls-native-certs`) — both for
  consistency and to drop the `webpki-roots` crate flagged by the license gate.

## Connectivity preflight (this run, live)

```
✓ PDS (_health)              HTTP 200            https://bsky.social/xrpc/_health
✓ PLC directory              HTTP 200            https://plc.directory/
✓ Jetstream (WS handshake)   WebSocket upgraded  wss://jetstream2.us-east.bsky.network/subscribe
Gate PASSED.
```

## Architecture choice: direct XRPC, not `atrium-oauth`

The brief suggests `atrium`/`atrium-oauth`. This crate instead talks to the
stable `com.atproto.*` XRPC endpoints directly over `reqwest`, and consumes
Jetstream over a raw `tokio-tungstenite` WebSocket. Rationale (a finding in itself):

1. **Headless OAuth is infeasible here.** The atproto OAuth flow requires a
   publicly reachable client-metadata URL *and* a redirect/callback the
   authorization server can reach, plus an interactive browser approval. This
   ephemeral container exposes no public callback and has no browser, so
   `atrium-oauth`'s primary value cannot be exercised. The brief explicitly
   blesses the **app-password fallback** (`com.atproto.server.createSession`) as
   a legitimate, reportable result — that is the auth path implemented here.
2. **Raw capture is better for the comparison goal.** Reading the *unmodeled*
   Jetstream JSON gives the truest event shape to compare against the assumed
   `RecordEvent` struct from the local AppView (3a).
3. **No API-churn risk.** Direct XRPC removes dependence on the fast-moving
   higher-level client surface, so this artifact stays runnable.

## Usage

```bash
# 0. Gate (no creds needed)
cargo run -- preflight

# Smoke-test the Jetstream parser against live traffic (no creds needed)
cargo run -- consume --collection app.bsky.feed.post --seconds 6

# 1. Configure a throwaway account
cp .env.example .env && $EDITOR .env     # set ATPROTO_HANDLE + ATPROTO_APP_PASSWORD

# 2. Verify auth + DID/PDS resolution
cargo run -- whoami

# 3+4. THE HEADLINE: publish N records, catch them back off Jetstream, index them
cargo run -- roundtrip --count 3

# 5. Serve the indexed posts from the minimal AppView
cargo run -- serve --db roundtrip.sqlite
curl 'http://127.0.0.1:8088/xrpc/org.croftc.experiment.feed.getTimeline'

# 6. Clean up the public test records
cargo run -- cleanup --yes
```

## Stages

1. **Lexicon** — `lexicons/org.croftc.experiment.feed.post.json` (`type: record`,
   `key: tid`, `text` + `createdAt`) and a read query
   `org.croftc.experiment.feed.getTimeline` (`type: query`). A targeted Rust
   validator (`src/lexicon.rs`) enforces the record constraints. A PDS stores
   arbitrary records by collection NSID, so publishing a *custom* lexicon does
   **not** require Bluesky to know it.
2. **Auth** — app-password `createSession`, then resolve `handle → DID → PDS`
   (via the public resolver + PLC `#atproto_pds` service) and use the repo's
   actual PDS endpoint.
3. **Publish** — `createRecord` (rkey assigned by the PDS); capture AT-URI + CID.
4. **Consume** — filtered Jetstream subscription (`wantedCollections` + our DID);
   match returned AT-URI/CID against what `createRecord` returned; measure
   observed propagation latency.
5. **Index & serve** — SQLite index (validated on ingest) + one axum XRPC query.
6. **Cleanup** — `deleteRecord`; optionally observe delete-commit propagation.

## Live run results (2026-06-13)

Test account: `@ngvalidation2112.bsky.social` (`did:plc:xyfhcaweaeyew3zrgk6jaln7`),
a dedicated throwaway. All published content was innocuous and **deleted** at the end.

**Auth path used:** app-password / `com.atproto.server.createSession`.
Full OAuth was *not* attempted — infeasible in a headless container (no public
callback URL, no browser). This is the brief-anticipated fallback, and the
honest headline on auth: for a real product, OAuth is the part that still has to
be solved for end users; the simple app-password path is what makes a headless/
server-side integration tractable today.

**Identity/PDS resolution (a real friction point):** the account authenticates
against the **entryway** `bsky.social`, but its repo lives on a *different* PDS,
`stropharia.us-west.host.bsky.network`. `createSession` returns a DID document;
the `#atproto_pds` service endpoint there is the host writes must go to — using
the entryway host for `createRecord` would be wrong. The code resolves this from
the DID doc, and an independent handle→DID→PLC lookup cross-checked it.

**Published → returned (the proof):**

| AT-URI (rkey) | CID | Returned on Jetstream | Latency |
|---|---|---|---|
| `…/3mo6acxpkay2k` | `bafyreiel33ppof…fryuha` | ✅ CID match | 1216 ms |
| `…/3mo6acxsl5u24` | `bafyreibpatlbg2…kpw3zq` | ✅ CID match | 749 ms |
| `…/3mo6acxvusq2y` | `bafyreif66jm6k5…2eja3q` | ✅ CID match | 638 ms |

**Observed propagation latency** (createRecord returns → seen on Jetstream,
wall-clock incl. our own RTT): **min 638 ms, avg 867 ms, max 1216 ms.** The first
record is consistently slowest (connection warm-up); steady-state is sub-second.

**Raw Jetstream commit event shape** — confirmed live (create):

```json
{
  "did": "did:plc:xyfhcaweaeyew3zrgk6jaln7",
  "time_us": 1781353018143169,
  "kind": "commit",
  "commit": {
    "rev": "3mo6acxpo5y2k",
    "operation": "create",
    "collection": "org.croftc.experiment.feed.post",
    "rkey": "3mo6acxpkay2k",
    "cid": "bafyreiel33ppof…fryuha",
    "record": { "$type": "org.croftc.experiment.feed.post", "text": "…", "createdAt": "…" }
  }
}
```

What an AppView ingest layer needs: `did` + `commit.collection` + `commit.rkey`
(→ reconstruct the AT-URI), `commit.cid`, `commit.record`, `commit.operation`,
and `time_us` (cursor/ordering). **Delete events omit `cid` and `record`**
(observed during cleanup: three `delete` commits, `cid=-`) — hence those are
`Option` in `src/jetstream.rs`. No surprising/absent fields versus expectation;
the unmodeled raw JSON matched the struct cleanly.

**Served timeline JSON** (`GET /xrpc/org.croftc.experiment.feed.getTimeline`),
posts originated as local `createRecord`s, traveled the public network, served
from our own index — reverse-chronological #3, #2, #1:

```json
{"posts":[
  {"uri":"at://…/3mo6acxvusq2y","cid":"bafyreif66jm6k5…","text":"…record #3","createdAt":"2026-06-13T12:16:58.010+00:00", …},
  {"uri":"at://…/3mo6acxsl5u24","cid":"bafyreibpatlbg2…","text":"…record #2", …},
  {"uri":"at://…/3mo6acxpkay2k","cid":"bafyreiel33ppof…","text":"…record #1", …}
]}
```

**Cleanup:** all 3 records deleted via `deleteRecord`; **deletes propagate** as
Jetstream `delete` commit events (no `cid`/`record`). Useful for AppView GC.

**Custom-lexicon acceptance:** the PDS accepted `org.croftc.experiment.feed.post`
without knowing the lexicon — confirmed. A PDS stores arbitrary records by NSID.

### Friction log (live)

- **OAuth vs app-password** — the single biggest real-world gap; see above.
- **Entryway ≠ PDS** — must read the PDS endpoint from the DID doc, not assume
  the login host. Easy to get wrong; would bite a naive build.
- **App password vs account password** — `createSession` accepted the account
  password directly here (throwaway account, no 2FA). Production accounts with
  2FA require an app password; the code/README assume app-password by convention.
- **TLS root sources** — `reqwest` (rustls) and `tokio-tungstenite` needed
  *different* root configs in one binary; `tokio-tungstenite`'s webpki feature
  gave `UnknownIssuer` on the Jetstream handshake (fixed with native roots).
- **Dependency licensing** — the rustls TLS stack pulls CA-list crates
  (`webpki-root-certs`, CDLA-Permissive-2.0) and a UEFI-target transitive
  (`r-efi`, tri-licensed incl. LGPL) that the org's Cycode license gate flags.
- **Propagation feel** — sub-second steady-state. For the eventual public/private
  UX blend, the public path feels "near-live" but not instant; ~0.6–1.2s is the
  budget to design around versus an instant local/private write.

## Extensions (follow-up work)

### #1 — Real OAuth, driven to the consent wall ✅ (live)

`cargo run -- oauth --handle <handle>` runs the genuine atproto OAuth flow as
far as a headless machine can, against live `bsky.social`:

- Discovery: handle → DID → PDS → `oauth-protected-resource` → authorization
  server → `oauth-authorization-server` (PAR endpoint).
- PKCE (S256), an ephemeral **P-256 DPoP** key, and a **pushed authorization
  request (PAR)** — including the mandatory **DPoP-nonce handshake** (first PAR
  returns `use_dpop_nonce`; we retry with the server's nonce and it's accepted).
- Emits the authorization URL and starts a loopback `127.0.0.1` callback listener.

**Finding:** the earlier "OAuth is infeasible locally" is too pessimistic. *Every
back-channel step is fully automatable and works against the real server* — DPoP
proofs are accepted, PAR returns a `request_uri`. The **only** un-automatable
step in this environment is the human clicking "approve" in a browser, after
which the loopback catches `?code=...` and a DPoP-bound token exchange completes.
That is a precise, measured boundary, not a hand-wave. Six back-channel round
trips precede consent.

### #2 — Multi-account / cross-PDS round-trip ✅ (live, true cross-PDS)

`cargo run -- multi-roundtrip` authenticates N accounts (numbered env vars
`ATPROTO_HANDLE_N` / `ATPROTO_APP_PASSWORD_N`, or the single default), resolving
each to its *own* PDS, then publishes one record per account, filters Jetstream
on **all** their DIDs at once, attributes each returned event to its account/PDS,
reports per-PDS latency, and cleans up every account.

**Confirmed live with two accounts on two different PDS hosts:**
- `did:plc:xyfh…` on `stropharia.us-west.host.bsky.network`
- `did:plc:zqus…` on `jellybaby.us-east.host.bsky.network`
- `distinct PDS hosts: 2 (cross-PDS round-trip)` — a **single multi-DID Jetstream
  subscription caught both** records (CID-matched, attributed to the correct PDS).
  Per-PDS latency: us-west 1853ms, us-east 1666ms.

**Finding (prediction confirmed):** the public *consume* path is PDS-agnostic —
Jetstream is a global relay, so one subscription aggregates commits across PDSes
and needs no per-PDS handling. Only the *write* path is PDS-specific (each DID
resolves to its own host). Latencies were comparable across the two hosts.
(Also exercised earlier at N=1 and degenerate N=2 / same account.)

**Expected cross-PDS result (our thinking, to confirm live):** Jetstream is a
*relay/firehose* aggregating commits across all PDSes, so a single multi-DID
subscription should return both accounts' records regardless of which PDS each
lives on — the consume side is PDS-agnostic by design. What we'd expect to
actually differ: (a) each account resolves to a *different*
`*.host.bsky.network` endpoint for its **writes**, and (b) minor per-PDS latency
variation. If confirmed, the public *consume* path needs no per-PDS handling —
only the write path is PDS-specific.

### #3 — Latency distribution at volume ✅ (live)

`cargo run -- latency --count 25` publishes N records, stamping each with a
**monotonic** send instant *after* `createRecord` returns (so it measures pure
commit→Jetstream propagation, not the publish HTTP call), catches them back, then
cleans up and reports the distribution.

**Result (25 samples, live):** `min=91  p50=132  p90=182  p95=189  p99=251
max=251  mean=144` ms; cold first record 115ms. **Finding:** steady-state public
propagation is **~130ms median, p99 ~250ms** — genuinely near-live, much tighter
than single samples suggested. A rare ~11.8s outlier seen once in #2 confirms the
value of distributions over point measurements: design for a ~250ms p99 but
tolerate occasional multi-second stragglers.

### #4 — Real-ish indexer: backfill + resume + update/delete ✅ (live)

`cargo run -- index [--backfill-repo <handle|did>] [--seconds N] [--db ...]`
turns the toy AppView into a durable indexer:

- **Backfill** — pulls existing records straight from the repo (`listRecords`)
  so the index is complete on first run, not just from when it started.
- **Resume cursor** — persists Jetstream's `time_us` to a `meta` table after
  every event; on restart it subscribes with `&cursor=`, so the gap is replayed
  and nothing is missed. Runs until `Ctrl-C` (or `--seconds` for tests).
- **Mutations** — handles `create`/`update` (re-index) and `delete` (remove from
  the index), not just creates.

**Verified live:** backfilled 3 records into a fresh DB; restarted, resumed from
the saved cursor, caught live `create`s (`+`) and `delete`s (`-`), ended with the
index correctly drained to 0 rows and the cursor advanced for the next run.

### #5 — Richer lexicon: replies + threaded query ✅ (live)

The record lexicon gains an optional `reply` (a `replyRef` of two `strongRef`s —
thread `root` and immediate `parent`, each `{uri, cid}`), and a new
`org.croftc.experiment.feed.getThread` query reconstructs a thread. The AppView
stores `root_uri`/`parent_uri` on ingest and serves `getThread?uri=<root>`.

`cargo run -- thread` publishes a 3-post thread (root → reply → reply-to-reply),
indexes it, and prints the assembled thread. **Verified live** — the thread came
back correctly nested:

```
thread root — richer-lexicon experiment
    first reply to the root
        a reply to the reply (depth 2)
```

**Finding:** custom schemas stretch comfortably to reference-linked content — the
PDS stored our `reply` strong-refs without complaint, and rebuilding a thread is
just `root_uri` grouping + `parent_uri` walking. No need for Bluesky's own types.

### #6 — Cross-experiment bridge (source-agnostic boundary) ✅ (live, public side)

`cargo run -- compare` runs the public path (createRecord → Jetstream) and an
**in-process local-path baseline**, normalizing *both* through one boundary
(`src/bridge.rs`): a `NormalizedEvent` that an AppView ingest consumes, with
`NormalizedEvent::from_jetstream` as the public mapping and a `LocalPath` trait
as the seam where experiment 3a's real private path plugs in.

**Live results:**
- **Boundary works:** both sources normalize to the *identical* key set
  (`cid, collection, did, operation, record, rkey, source, uri`) — one ingest
  path handles both. This is the architectural payoff.
- **What the public mapping had to do** (the hypothesis-vs-reality finding):
  *build* the `uri` (Jetstream supplies none), lift `collection/rkey/cid/record`
  out of the nested `commit` object, and carry `operation` (create/update/delete)
  that the assumed flat `RecordEvent` never modeled.
- **Latency:** local ~0.01ms vs public ~1–4s end-to-end (includes `createRecord`;
  cf. #3's pure-propagation ~130ms). High-variance samples (1.2s, 4.4s) recur —
  design for occasional multi-second stragglers.

**To finish the true diff:** implement `bridge::LocalPath` against experiment
3a's private path and pass it to `compare` in place of `InProcessLocalPath`. The
seam is ready; only 3a's code is external to this repo.

## Further validations (trust, integrity, behavior)

### V1 — Does the PDS enforce our custom lexicon? ❌ No (security boundary) ✅ (live)

`cargo run -- probe-validation` publishes deliberately malformed records and an
entirely unknown lexicon, bypassing our own validator, and reads back what the
PDS does.

**Result (live):** the PDS **accepted every case** — `text` as a number, all
required fields missing, and a totally unknown `org.croftc.experiment.feed.bogus`
collection — each with `validationStatus: "unknown"`. Our own validator rejected
the malformed ones.

**Finding (trust boundary):** the PDS does **not** validate records against
lexicons it doesn't know; it stores whatever you send and marks it `unknown`. So
**schema validation is the AppView/client's responsibility, not the PDS's** — a
consumer must never assume firehose/PDS records conform to a custom lexicon, and
must validate on ingest (which our AppView does).

### V2 — Is handle↔DID bidirectionally verified? ✅ (live)

`cargo run -- verify-identity --handle <handle>` checks both directions:
forward (handle → DID via `resolveHandle`) and reverse (the DID document's
`alsoKnownAs` must itself claim `at://<handle>`).

**Result (live, both accounts):** both verified — e.g. `ngvalidation2112.bsky.social`
→ `did:plc:xyfh…`, and that DID's `alsoKnownAs` is exactly
`["at://ngvalidation2112.bsky.social"]`. Same for the second account.

**Finding (anti-impersonation):** a handle is only trustworthy when *both*
directions agree. Forward resolution alone is spoofable (anyone can point a DNS
record or `/.well-known` at a DID); the DID must reciprocally claim the handle.
Any consumer that displays handles must do this bidirectional check.

### V3 — Is the CID a real content hash? ✅ (live)

`cargo run -- cid-check` recomputes a record's CID from scratch — canonical
DAG-CBOR (`serde_ipld_dagcbor`) → sha2-256 → multihash → CIDv1 → base32
(`src/cidv1.rs`) — and compares to the server's.

**Result (live):**
- recomputed CID == server's `createRecord` CID (e.g. `bafyreiddqq6fi6…`) ✓
- recomputed from the **server-stored** value also matches ✓ (server didn't
  alter content)
- tampering with one field produces a **completely different** CID ✓

**Finding (integrity):** the CID is a genuine sha2-256 of the canonical DAG-CBOR
encoding — independently verifiable and **tamper-evident**. A consumer can
confirm a record's `uri`/`cid` pair really corresponds to the bytes it received;
any mutation changes the CID. (Bonus: matching bit-for-bit confirms DAG-CBOR's
length-first canonical key ordering — the encoder handled it correctly.)

### V4 — Update round-trip + cross-account thread ✅ (live)

`cargo run -- update-roundtrip` creates a record then `putRecord`s the same rkey
with edited text. **Result:** a new CID, and the firehose emitted
`operation=create` then `operation=update` (update CID matched the `putRecord`
response); the in-memory index re-indexed to the edited text. The
create/update/delete trifecta is now fully exercised live.

`cargo run -- cross-thread` (two accounts) has account 1 author a root and
account 2 reply to it with a strong-ref, then backfills both repos and assembles
the thread. **Result:** a **2-author thread across two DIDs on two PDSes**
(`stropharia.us-west` root → `jellybaby.us-east` reply), nested depth 0→1.

**Finding:** mutation propagates correctly (`operation` drives index-vs-reindex),
and references/threading work **across repos and PDSes** — a strong-ref doesn't
care that root and reply live on different servers, because resolution is by
AT-URI/DID, not by host. Multi-author social graphs are a client-side assembly
over independent repos.

### V5 (capstone) — Cryptographic chain of custody ✅ (live)

`cargo run -- verify-repo` exports the whole repo and verifies it end to end
(`src/repo_verify.rs`):

1. **CAR export** (`com.atproto.sync.getRepo`) parsed into blocks — validates
   data portability (you can pull your entire signed repo).
2. **Block integrity:** every block hashes to its CID (`sha256(bytes) == CID`) —
   live: `6/6 blocks verify`.
3. **Signed commit** decoded (DID, `rev`, MST root); DID matches the account.
4. **Commit signature** verified against the account's `#atproto` signing key
   from its DID document — live: a **secp256k1** key verified the commit.
5. **MST inclusion:** walking the Merkle Search Tree from the signed root, our
   anchored record's CID is among the committed value CIDs — live: `present: YES`.

**Finding (the whole trust model in one line):** a record is *provably* part of a
repo whose signed root chains to a verified identity. The chain —
**DID-doc key → signed commit → MST root → record CID → record bytes (V3)** —
is fully checkable by any consumer with no trust in the PDS or relay. This fuses
V2 (identity) and V3 (content) into end-to-end "who-said-what, provably." It also
means **tamper or omission is detectable at every layer**: alter a record and its
CID changes (V3) and MST membership breaks; forge a commit and the signature
fails; spoof identity and the DID-doc key won't match.

### V6 — Quick wins (media, ordering, identity history, durability) ✅ (live)

- **Blob round-trip** (`blob-check`): `uploadBlob` returned a raw-codec CID
  (`bafkrei…`), the blob ref survived embedding in a record (an *extra* field the
  PDS accepted — re-confirming V1), and `getBlob` returned **byte-identical**
  content. The media path works and is content-addressed like records.
- **TID/rkey ordering** (`tid-check`): five rkeys published in sequence decoded to
  **strictly monotonic** microsecond timestamps within seconds of now, and their
  **lexical sort equals chronological order** — rkeys are sortable TIDs, so
  "order by rkey" is a free, correct chronological sort.
- **PLC audit log** (`plc-audit`): the DID's signed operation history is
  time-ordered and the latest op's `#atproto` key **matches the current DID-doc
  key** — identity is anchored in a signed, auditable history, not a mutable
  pointer.
- **Jetstream replay window** (`replay-check`): reconnecting from a cursor 5s
  *before* an event **replayed that event** — resume is genuine replay, not just
  live tail, so the indexer's restart guarantee (V4-#4) holds against real gaps.

### V7 — Moderation labels ✅ (live) — see [MODERATION.md](MODERATION.md)

A full sub-experiment (`cargo run -- moderation`): self-labeling round-trip,
labeler discovery, `queryLabels`, and **label-signature verification**. Live
results: self-labels survive intact inside the signed record; the Bluesky
labeler is a signed identity with a dedicated `#atproto_label` secp256k1 key;
**10/10 sampled label signatures verified** against it; and `subscribeLabels`
returns 404 (pull-only, no public firehose). Diagnostic arc worth noting: label
verification first failed on cid-bearing labels — the root cause was that a
label's `cid` is a lexicon **string**, not a CBOR CID link like in the repo MST.

**Finding:** moderation is **cryptographically sound but semantically opt-in** —
labels are separate signed assertions by independent labeler identities; you
choose which to trust, verify against the labeler's (time-valid) key, and enforce
policy yourself. Full goal/attempts/results/conclusions in **MODERATION.md**.

## Debrief — complete picture & thinking

**One-line result:** the public AT Protocol path is solid and *near-live*. A
custom-lexicon record authored locally, published via a real PDS, returns off the
public Jetstream network with matching AT-URI/CID in **~130ms median (p99 ~250ms)**
and is servable from our own AppView — with deletes and replies handled too.

**What we now know with confidence (measured, not assumed):**
- **Identity indirection is real:** you log in at an entryway (`bsky.social`) but
  must follow the DID document to the *actual* PDS (`*.host.bsky.network`) for
  writes. Resolved live in every command; easy to get wrong.
- **Custom lexicons just work:** the PDS stored `org.croftc.experiment.*`
  records — including reply strong-refs — with no pre-registration. Threading is
  just `root_uri` grouping + `parent_uri` walking.
- **Propagation is near-live but variable:** ~130ms median, but multi-second
  outliers (1.2s, 4.4s, one ~11.8s) recur. Design for a ~250ms p99 and tolerate
  occasional stragglers.
- **The event-shape boundary is the integration crux:** Jetstream gives no ready
  `uri` (build it), nests fields under `commit`, and adds `operation`/`time_us`.
  `src/bridge.rs` normalizes this; a public and a local source converge to one
  shape, so a single ingest path serves both.
- **A durable indexer is straightforward:** persist `time_us` as a resume cursor,
  handle create/update/delete, backfill via `listRecords`. Verified live.

**The genuinely hard part is auth — and it's now pinned down:** the *entire*
OAuth back-channel (discovery, PKCE, DPoP, PAR + nonce handshake) is automatable
and accepted by the live server; only the human consent click can't be
automated. The product work is **OAuth UX**, not OAuth plumbing.

**Cross-PDS is now confirmed live (#2):** two accounts on two different PDS
hosts (`stropharia.us-west`, `jellybaby.us-east`) both returned through a single
multi-DID Jetstream subscription — the consume path is PDS-agnostic; only writes
are PDS-specific, as predicted.

**One seam left open (environment, not design):** #6 needs experiment 3a's
private-path code to implement `bridge::LocalPath` — the seam and normalized
boundary are ready and tested on the public side.

**For the public/private UX blend:** private/direct writes are effectively
instant (~µs in-process); public writes are ~130ms-to-seconds via the relay.
That gap is the core UX fact — instant for direct/private content, "near-live,
occasionally delayed" for anything routed through the public firehose.

## Security notes

- The app password is read only from the environment / `.env` (git-ignored) and
  is **never logged** (error paths echo only the server's response body).
- The AppView binds to `127.0.0.1` only, and returns generic error messages to
  clients (detail is logged server-side) so internal paths aren't leaked.
- The PDS endpoint read from the (externally-controlled) DID document is
  validated to be `https` with a host before any authenticated request is sent
  to it — guards against SSRF via a malicious/compromised DID doc.
- `cleanup` requires an explicit `--yes`.
