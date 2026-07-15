# End-to-End Slice (Phase 6)

The whole model in **one program**, chaining every prior phase over real sockets:

```
encrypted MLS group (P1) + lexicon records (P2)
  -> public/private mirror with redaction (P4)
  -> publish public records to a real local PDS (P5)        [real HTTP createRecord]
  -> firehose over a real WebSocket -> AppView (P3a/P3b)     [real WS + axum + SQLite]
  -> hydrated, lexicon-valid public timeline
```

```
cargo run     # 6 checks: group -> identity map -> mirror-publish -> firehose/AppView -> non-leakage
cargo test
```

Goal (per the ask): *prove the model end to end, and surface the issues that need
addressing.* Fully offline — a local PDS on loopback, since the live network is
egress-blocked here (see `../local-pds-bridge/`).

## What runs (all 6 checks PASS)

1. **Encrypted private group**: MLS group (Alice+Bob, matching epoch key), an
   Automerge group doc with 3 posts (2 public, 1 private holding a secret) + 2
   reactions, synced via the real encrypt→decrypt path. The secret is confirmed
   absent from the ciphertext.
2. **Identity map**: each group author (MLS `did:key`) gets a PDS account
   (`did:plc`) via real `createSession`.
3. **Mirror-publish (the boundary)**: public-tagged records are published to the
   PDS via real `createRecord`; the private post is withheld; the public reaction
   on the private post is **redacted**; the public reaction on the public post is
   published with its `subject` **rewritten** from the private group URI to the
   published PDS URI/CID. **CID parity** holds across the boundary.
4. **Firehose → AppView**: the PDS firehose is consumed over a real WebSocket and
   served by the byte-identical AppView as a hydrated `getTimeline` — 2 public
   posts, the reaction joined onto its post (reactor = Bob's PDS DID), output
   valid against the read lexicon.
5. **Non-leakage + identity continuity (keystone)**: the secret, the private
   post's URI, and its rkey appear **nowhere** downstream (firehose frames or
   timeline); the private post is absent from the public feed; author/reactor DIDs
   are continuous through the chain.

## Issues surfaced (the deliverable, not the checkmarks)

Running the model end to end exposed five things the isolated phases hid:

1. **Identity is two-headed.** The group member identity (MLS-derived `did:key`)
   and the PDS account (`did:plc`) are *different identifiers for the same
   principal*. The bridge needs an explicit map, and **production needs a
   verifiable binding** (signed linkage) so an AppView/relay can trust that a
   published DID really is a given group member. Today nothing proves that link.
2. **AT-URIs change identity at the boundary.** A record is
   `at://<groupDid>/<coll>/<rkey>` privately and `at://<pdsDid>/<pdsRkey>` once
   published. Every `strongRef` (reaction `subject`) had to be **rewritten** to
   the published URI/CID, or it dangles. Any ref-bearing record type needs this.
3. **rkey is not stable across the boundary.** The PDS assigns a new rkey on
   publish, so a record's identity isn't preserved group→public. Clients must
   either track the mapping or pin rkeys at creation (publish with the group
   rkey). This has real consequences for dedup, edits, and idempotent re-publish.
4. **Concurrency is still unaddressed.** Single-committer membership only;
   concurrent MLS membership commits + CRDT fork resolution remain open (deferred
   since Phase 1) and would bite a real multi-device, multi-author group.
5. **Asymmetric transport realism.** The public side now runs over real sockets
   (HTTP + WebSocket); the private side's transport (iroh QUIC/blobs) is still
   stubbed. The next transport-focused phase should close that.

Two properties held up well and are worth banking:
- **The mirror is the single auditable choke point** for the public/private
  boundary — redaction, rewrite, and default-deny all live in one function.
- **The AppView never changed.** `indexer.rs`/`server.rs`/`views.rs` are the same
  bytes as Phases 3a/3b; the entire private→public→AppView path reuses them as-is.

## Resolved versions

rustc 1.94.1 · automerge 0.7.4 · openmls 0.8.1 · chacha20poly1305 0.10.1 ·
rusqlite 0.32 · axum 0.8.9 · tokio 1.52.3 · reqwest 0.13.4 · tokio-tungstenite
0.29.0 · cid 0.11.3 · serde_ipld_dagcbor 0.6.4 · serde_json 1.0.150.

## Where the model stands

The private→public→read-path is proven end to end on real sockets, with the
privacy boundary holding (non-leakage) and the data staying lexicon-valid
throughout. The open frontier is now well-defined: (a) a verifiable
group-identity↔DID binding, (b) stable cross-boundary record identity, (c)
concurrent membership/fork resolution, and (d) real private-side transport.
