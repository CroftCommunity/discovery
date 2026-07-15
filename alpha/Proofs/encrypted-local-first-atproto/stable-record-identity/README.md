# Stable Cross-Boundary Record Identity (Phase 9)

Resolves the Phase 6 issue where publishing reassigned rkeys, so a record's
identity changed group→public and every strongRef needed a lookup-table rewrite.

```
cargo run     # pin rkey -> pure URI mapping -> idempotent -> conflict -> edit
```

## The fix: pin the rkey at creation

The committer mints the rkey **once** (a TID at authoring time, in the group) and
publishes with that exact rkey via `createRecord`'s `rkey` field. Then:

- **The public URI is a pure authority rewrite of the group URI** — swap the DID,
  keep `collection` + `rkey`: `at://<groupDid>/c/rkey` → `at://<pdsDid>/c/rkey`.
  No lookup table. Combined with the Phase 8 binding, the authority swap is
  *verifiable*, so strongRef rewriting becomes a deterministic pure function.
- **Idempotent re-publish**: `createRecord` with the same (rkey, content) returns
  the same uri/cid with `idempotent: true` — no duplicate.
- **Conflict protection**: `createRecord` with the same rkey but *different*
  content returns `409` ("use putRecord") — a create never clobbers.
- **Edits via `putRecord`**: same rkey, new content → **same URI, new CID**. The
  record's identity is stable across edits while its content hash changes.

## Lifecycle (all 5 steps PASS)

1. PDS honors the caller's rkey — identity pinned at creation.
2. Public URI == `public_uri(group_uri)` computed purely by swapping the DID — match, no lookup table.
3. Idempotent re-create (same rkey + content) — no duplicate, same CID.
4. Conflicting create (same rkey, new content) → `409`.
5. `putRecord` edit — URI stable, CID changed, readback shows new content.

## Issues surfaced / resolved

- **RESOLVED:** the Phase 6 lookup table for strongRef rewriting is gone — the
  mapping is pure once rkeys are pinned, and verifiable via the Phase 8 binding.
- The committer must mint the rkey once and reuse it on publish; two independent
  publishes of "the same" record must agree on the rkey or they create two records
  (rkeys are not content-addressed).
- Edits require `putRecord` (upsert); `createRecord` is create-only and `409`s on
  conflict — clients must choose the right verb. Real PDSes also enforce
  per-collection rkey rules (`key: tid` vs literal).
- rkeys are namespaced by DID (the repo), so no global collision; within one repo
  the TID clock must stay monotonic.

## Resolved versions

rustc 1.94.1 · axum 0.8.9 · tokio 1.52.3 · reqwest 0.13.4 · cid 0.11.3 · serde_json 1.0.150.
