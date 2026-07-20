# Vendored reference — the CID-First Attestation Specification

`Source of record: https://badge.blue/ (the "CID-First Attestation Specification"),
with the Rust reference crate atproto-attestation
(https://tangled.org/ngerakines.me/atproto-crates, https://crates.io/crates/atproto-attestation)
and the lexicon-community org Discussion #8 ("Attestation Lexicon"). Author: Nick
Gerakines. Fetched in-session 2026-07-20. This file is a faithful reproduction of
the spec's normative content for clean-room auditability — it is NOT our work and
carries no Croft/Drystone claims. The clean-room verifier in src/ is built from
THIS TEXT, not from the reference crate's source.`

Everything below is the spec as fetched; our reading, disambiguations, and the
feedback we would carry back to the thread live in `AMBIGUITIES.md`, not here.

## 1. CID computation (seven steps, deterministic)

Given a record (JSON object), a metadata object, and a repository DID:

1. **Strip signatures** — remove any existing `signatures` array from the record.
2. **Prepare metadata** — remove `cid` and `signature` fields from the metadata
   if present.
3. **Add repository** — insert `repository` field into the metadata, set to the
   repository DID.
4. **Insert `$sig`** — place the prepared metadata into the record as the `$sig`
   field.
5. **Serialize** — convert the entire object to DAG-CBOR (deterministic CBOR).
6. **Hash** — compute SHA-256 over the DAG-CBOR bytes.
7. **Wrap as CIDv1** — CIDv1, codec DAG-CBOR (0x71), multihash SHA-256 (0x12),
   32-byte digest, base32lower (`bafy…`) string form.

Property: identical `(record, metadata, repository)` always yields the same CID;
different repository DIDs yield different CIDs for the same record content.

## 2. The `$sig` metadata object

A transient object inserted during CID computation, NOT persisted in the final
record.

| field | requirement | notes |
|---|---|---|
| `$type` | required | attestation type identifier (e.g. `com.example.signature`) |
| `repository` | auto | repository DID; added in step 3 |
| `key` | inline | did:key public-key reference (inline attestations) |
| custom | optional | `issuer`, `issuedAt`, `purpose`, … |

`cid` and `signature` are stripped from the metadata before it becomes `$sig`.

## 3. Inline attestation entry

```json
{
  "$type": "com.example.inlineSignature",
  "key": "did:key:zQ3sh...",
  "issuer": "did:plc:issuer123",
  "issuedAt": "2024-01-01T00:00:00.000Z",
  "cid": "bafyrei...",
  "signature": { "$bytes": "base64-normalized-signature" }
}
```

Required: `$type`, `key`, `cid`, `signature.$bytes`. The entry is appended to the
record's `signatures` array. Creation: compute CID → sign CID bytes (ECDSA) →
normalize to low-S → base64 → assemble entry → append.

## 4. Remote attestation

**Proof record** (in the attestor's repo):

```json
{ "$type": "com.example.attestation", "issuer": "did:plc:issuer123", "purpose": "verification", "cid": "bafyrei..." }
```

**strongRef entry** (appended to the source record's `signatures`):

```json
{ "$type": "com.atproto.repo.strongRef", "uri": "at://did:plc:attestor/com.example.attestation/3abc…", "cid": "bafyrei..." }
```

Creation: compute attestation CID over the source record → build proof record
(metadata + computed `cid`) → compute the proof record's own DAG-CBOR CID → build
the strongRef with the proof's AT-URI and CID → append to source `signatures`.

## 5. Repository binding (replay prevention)

The repository DID is injected into `$sig` before CID generation, so copying a
record between repos changes `repository` → changes `$sig` → changes DAG-CBOR →
changes the SHA-256 → changes the CID, invalidating every signature.
"Verification must use the repository DID where the record is actually stored."

## 6. Signature suite

ECDSA over P-256 (secp256r1), P-384 (secp384r1), K-256 (secp256k1). All
signatures normalized to low-S: for `(r, s)` with `s > n/2`, replace with
`(r, n − s)`. `$bytes` uses standard base64 (padded; decoders accept unpadded).

## 7. Verification

**Inline:** extract entry → rebuild `$sig` (strip `cid`/`signature`, add
`repository`) → recompute CID → resolve public key from `key` → base64-decode
signature → verify ECDSA against the recomputed CID bytes.

**Remote:** extract strongRef → fetch proof record via AT-URI → check the proof's
DAG-CBOR CID matches the strongRef `cid` → rebuild `$sig` from the proof record
(strip `cid`, add `repository`) → recompute the attestation CID over the source
record → check it matches the `cid` stored in the proof record.

## 8. Multiple attestations

A record may carry any number of entries; inline and remote may be mixed. The
`signatures` array is always stripped before CID computation, so the signing
payload is stable regardless of how many entries are present.

## 9. Lexicon-community Discussion #8 (grounding)

The thread proposes the same mechanism under the field name `sigs` (not
`signatures`): DAG-CBOR of the record excluding `sigs`, add `$sig`, sign. Open
questions raised in-thread and not settled there: (a) verification-method
resolution ("is there an algorithm that's considered the default? … how do I
compute it?"); (b) a bare `did` field of unclear referent (rename/document
requested); (c) cross-repo writes are NOT enabled in atproto v1, so an
attestation lives as a record in the issuer's OWN repo. Thread active; most recent
activity 2025-02-10; owner @ngerakines engaged.
