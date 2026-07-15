# Local-First, Lexicon-Respecting Mini-App (Phase 2)

A minimal "microblog + reactions" group feed built on the encrypted local-first
sync core from `../encrypted-sync-slice/`. It answers one question:

> Does a local-first app whose data respects AT Protocol **lexicon** schemas
> compose cleanly with the encrypted CRDT stack, such that the same records
> could later be published to a PDS **without reshaping the data**?

No live PDS, OAuth, DID resolution, relay, AppView, or firehose is touched. Only
the **data-model interop** is proven. iroh transport stays stubbed (real BLAKE3
content addressing), exactly as in Phase 1.

```
cargo run     # 6-step lifecycle + interop assertion + pass/fail summary
cargo test    # datetime + lexicon validation + 4-tuple comparator unit tests
```

## What's reused vs. new

Reused unchanged from the Phase 1 slice (copied, not modified in place):
`store.rs` (BLAKE3 content addressing; QUIC stubbed), `address.rs` (Willow 4-tuple),
`mls.rs` (openmls exporter → epoch key), `crypto.rs` (ChaCha20-Poly1305),
`doc.rs` (automerge 0.7 plumbing).

New in this phase:
- `lexicons/` — two real atproto Lexicon JSON files.
- `src/lexicon.rs` — a targeted validator that reads the schema JSON.
- `src/record.rs` — record types, `$type` handling, TID rkeys, datetime, CID.
- `src/repo.rs` — atproto-style repo model (`collection NSID → rkey → record`) in Automerge.

## Lexicon spec rules followed

Verified against the current atproto Lexicon spec:
- Each file is JSON with `"lexicon": 1`, an `"id"` (the NSID), and `"defs": { "main": ... }`.
- `main` is `"type": "record"` with `"key": "tid"` and a `"record"` object schema (`required` + `properties`).
- Field names are **lowerCamelCase** ASCII (`createdAt`, not `created_at`).
- Datetime fields are `"type": "string"`, `"format": "datetime"`.
- Record objects carry `$type` set to the **bare NSID** (no `#main` suffix); `main` types are referenced without a fragment.
- Object schemas are treated as **closed** (only declared properties + `$type`).
- The reaction `subject` is a `ref` to a `#strongRef` def (uri + cid), the local analogue of `com.atproto.repo.strongRef`.

Experimental status is signaled by the `org.croftc.experiment.*` namespace and `EXPERIMENTAL` in each schema description.

## Lifecycle (all 6 steps PASS)

1. Group setup — Alice/Bob derive a matching epoch-N content key (reused MLS path).
2. Alice posts — `feed.post` built, **validated**, written under `collection/rkey`, snapshot encrypted + stored by 4-tuple.
3. Bob syncs — fetch → decrypt → load → read → **re-validate** the received record; prints the JSON with `$type`.
4. Bob reacts — `feed.reaction` referencing Alice's post via strongRef, validated, applied as an incremental change, encrypted + stored; Alice applies and renders the feed.
5. Epoch rotation — add Carol; key rotates; Carol bootstraps from a fresh snapshot under the **new** key and **re-validates every record** across the epoch boundary.
6. **Interop assertion (keystone)** — the post, as Carol reconstructed it (authored → validated → encrypted → CRDT-synced → epoch-rotated → decrypted), is serialized to the exact `record` payload of a `com.atproto.repo.createRecord` body and asserted to validate. It does. No PDS is called.

## The outcome that matters most

A record authored locally, validated against a real atproto lexicon, encrypted
under an MLS-exporter epoch key, synced as an Automerge CRDT entry, and
reconstructed by another member **across an epoch rotation**, comes out the
other side as **valid atproto lexicon JSON ready for `createRecord` unchanged**.
The local-first/atproto data-model interop — the foundation of the public/private
split — holds on the real stack. The remaining public-side work is transport and
auth, not data reshaping.

## Resolved versions

rustc 1.94.1 (≥ 1.80) · automerge **0.7.4** · openmls 0.8.1 / rust_crypto 0.5.1 /
traits 0.5.0 / basic_credential 0.5.0 · chacha20poly1305 0.10.1 · blake3 1.8.5 ·
serde_json 1.0.150 · iroh 0.98.2 / iroh-blobs 0.102.0 (resolvable, **not linked**).

## Deviations & real-world friction (honest)

- **No lexicon→Rust codegen crate used.** With only two record types, hand-written
  structs + a targeted validator are less machinery than wiring a generator, and
  no codegen crate resolved cleanly against the current spec on this toolchain.
  The validator is *schema-driven* (it reads the lexicon JSON), so the schemas
  remain the source of truth — but it is not a general-purpose Lexicon engine
  (it covers the constructs our two schemas use: `record`, `object`, `string`
  with `maxLength`/`maxGraphemes`/`format`, `array`, `ref`).
- **`maxGraphemes` is approximated** by Unicode scalar count (`chars().count()`).
  True grapheme-cluster counting needs `unicode-segmentation`; for the ASCII +
  single-codepoint-emoji text used here the two agree. A multi-codepoint emoji
  (e.g. a family ZWJ sequence) would be over-counted — flagged, not fixed.
- **CID is a stand-in.** `strongRef.cid` is a BLAKE3 digest (`b3-…`), not a real
  CIDv1 multihash over DAG-CBOR. The *shape* (uri + cid) matches atproto; a real
  deployment would compute the CID over the canonical encoding.
- **strongRef felt slightly forced, but natural enough.** Modeling `subject` as an
  inline `#strongRef` def rather than referencing `com.atproto.repo.strongRef`
  keeps the experiment self-contained; the field shape is identical, so swapping
  to the canonical external ref later is a one-line lexicon change.
- **Records stored as JSON-string scalars** in Automerge (keyed by rkey), not as
  native nested Automerge objects. This is deliberate: the record travels as the
  opaque, lexicon-valid blob that `createRecord` wants. Native sub-object modeling
  would enable field-level CRDT merge *within* a record but is unnecessary for the
  interop question (and would complicate the createRecord round-trip).
- **`$type` vs. lexicon `properties`.** `$type` is required on the record object
  but is not itself a declared property in the lexicon; the validator special-cases
  it (required + equals the NSID) while keeping objects otherwise closed. This
  matches atproto's actual treatment but is an easy place to get a naive validator
  wrong.
- **Two complementary addressing systems coexist cleanly.** The Willow 4-tuple
  addresses *encrypted sync payloads*; the atproto `collection/rkey` model
  organizes *records inside* the decrypted document. They did not conflict — the
  4-tuple `path` (`/repo/<collection>`) even lines up with the repo layout.

## What the next phase needs (not built here)

To actually publish to a PDS: OAuth/identity (atproto OAuth + DID resolution for
the authoring account), the real `com.atproto.repo.createRecord` call (with a
genuine CIDv1 over DAG-CBOR for strongRefs), and the public/private split logic
that decides which group records are mirrored to the public namespace. All of
that is **transport + auth + a CID encoder** layered onto data that is already
lexicon-valid — no change to the record model proven here.
