# AMBIGUITIES — the clean-room feedback table

`RUN-LEX-01 EXP-LEX-03. Every entry is a point where building a verifier from the
CID-First Attestation Specification (badge.blue) + lexicon-community Discussion #8
ALONE underdetermined behavior, or where the spec text and the reference
implementation (crates.io atproto-attestation 0.14.5) disagree. Each is grounded:
"observed" facts come from running the reference CLI and from real PDS records,
captured 2026-07-20. This table IS the spec-feedback post — phrased as questions,
respectfully, second-implementer to spec author.`

Posture: nothing here is a "bug" claim. A second implementation is the cheapest
way to find the sentences a spec still needs, and we found ten. Two (A-1, A-8) are
security-relevant and worth a sentence each in the spec; the rest are
precision/interop.

| # | Where | The ambiguity (as a question) | What we observed / chose | Suggested spec sentence |
|---|---|---|---|---|
| **A-1** | Inline verify, key resolution | Nothing binds the inline `key` (a `did:key`) to the claimed `issuer` DID. Does a verifier check the key against the issuer's DID-document verification methods? | Without the check, an attacker recomputes the public CID and **re-signs it with their own key**, swapping `key`+`signature`; a naive verifier accepts it. We demonstrate this (`foreign_key_swap_lax_accepts_but_strict_rejects`) and add an opt-in `require_issuer_binding` that resolves the issuer DID doc. | "A verifier MUST confirm the `key` is a verification method authorized by the `issuer` DID document; otherwise an inline attestation authenticates a key, not an issuer." |
| **A-2a** | `$sig` contents | Is the `key` field part of the signed `$sig` metadata (and thus of the CID), or not? The spec lists `key` as a `$sig` field. | The **reference impl computes the CID with no `key` in `$sig`**. To interoperate we EXCLUDE `key` from the signed bytes. (Corollary of A-1: `key` being outside the signature is exactly why the binding check matters.) | "The `key` field is transported in the entry but is NOT part of `$sig` and is not covered by the signature." |
| **A-2b** | Inline output | The spec's inline entry requires `key`, but the reference CLI **omits `key` entirely** from its output. How does a verifier resolve the key? | Observed: `atproto-attestation-sign inline` emits `{$type, purpose, cid, signature}` — no `key`. We had to derive it from the signing key out-of-band. | "Signers MUST populate `key` with the signing key's `did:key`; a verifier cannot proceed without it." |
| **A-3** | Signature §6 | Low-S is mandated for creation. Must a **verifier reject** a high-S (malleable) signature? | We enforce it (`require_low_s`, default on) and reject high-S (`high_s_signature_rejected`). RustCrypto's verify would otherwise accept both. | "Verifiers MUST reject signatures whose S is not in low-S canonical form." |
| **A-4** | "Sign CID bytes" | *Which* bytes of the CID are signed — the 36-byte binary CIDv1, the 32-byte multihash digest, or the base32 `bafy…` string? | **Observed (interop probe): the reference signs the 36-byte binary CIDv1** (`0x01 0x71 0x12 0x20 ‖ digest`). Verified by reproducing its signature. We adopt this. | "The signature covers the 36-byte binary CIDv1, not the base32 string nor the bare digest." |
| **A-5** | ECDSA digest | ECDSA signs a *hash* of the message. Since the CID already embeds SHA-256, is the outer signature over `SHA-256(cid_bytes)` (a second hash), and which digest for P-384? | Observed: reference uses the curve default — **SHA-256 for P-256/K-256** (so the CID is hashed again). By symmetry P-384 uses SHA-384. | "The ECDSA signature is computed over the curve's default digest of the 36-byte CID: SHA-256 for P-256/K-256, SHA-384 for P-384." |
| **A-6** | No-scalar / validation | Does Lexicon validation stop a record smuggling an out-of-schema field (e.g. a numeric score)? | **Observed: `@atproto/lexicon` IGNORES unknown fields** — our `rsvp_unknown_field` (with `smuggledScore`) VALIDATES under the official tooling (our strict Rust validator rejects it). | "Lexicon validation alone does not bar unknown fields; a no-scalar/no-smuggle guarantee must be enforced by the AppView, not assumed from schema validity." |
| **A-7** | Field naming | Discussion #8 uses `sigs`; the badge.blue spec uses `signatures`. Which is normative? | Divergence between the two published sources. We follow `signatures` (the formal spec + reference impl). | "The array is named `signatures` (superseding the thread's earlier `sigs`)." |
| **A-8** | Remote pattern | The remote proof record carries **no explicit signature** — its integrity rests entirely on the attestor's repo commit signature. Is that intended? Inline carries ECDSA; remote does not. | Observed: `atproto-attestation-sign remote` emits a proof record with `{$type, issuer, purpose, cid}` and no signature field. Trust = "whoever controls the attestor repo." | "A remote attestation is authenticated by the attestor's repository commit signature, NOT by an in-record signature; verifiers gain the attestor's assertion, not a detached proof." |
| **A-9** | strongRef resolution | A strongRef pins `{uri, cid}`. When the target is later edited, resolving the URI yields a different current CID. How should consumers treat the drift? | **Observed in real data**: an RSVP's `subject` strongRef pins a CID that no longer matches the target event's current CID (the event was edited). | "Resolving a strongRef by URI returns the CURRENT record; compare its CID to the pinned `cid` to detect drift — a mismatch is expected after edits, not corruption." |
| **A-10** | Discussion #8 `did` field | The bare `did` field's referent is unclear (the thread's own open complaint — is it the signed object's did, the signer's, the issuer's?). | Not resolvable from text. We use explicit-referent names throughout (`issuer`, `key`) and closed `enum`s over open `knownValues` for closed sets. | "Rename `did` to name its referent (`issuerDid` / `signerKey`); prefer `enum` over `knownValues` for closed vocabularies." |

## The two that matter most

- **A-1 + A-2** together are one security story: the inline pattern, as the
  reference implements it, proves *a key signed this CID in this repo*. It does not
  prove *the issuer signed it* until the verifier resolves the issuer DID document
  and checks the key is authorized. Anyone can re-sign a public CID. The fix is one
  verifier step, and we ship it (strict posture, default on).

- **A-8**: inline and remote are NOT interchangeable in their trust base. Inline is
  a detached cryptographic proof; remote is "the attestor's repo asserts this,"
  authenticated only by the repo commit. Both are useful; conflating them would let
  a reader over-trust a remote attestation.

## Alignment we bring (not ambiguities — shared instincts)

Sign content not location (CID over DAG-CBOR); repository binding for replay;
inline-with-content over hash-of-content; `enum` (closed) over `knownValues`
(open) for closed sets. Everything we build rides single-author records — the
thread's stated scope line (no cross-repo writes in atproto v1).

## Grounding

- badge.blue "CID-First Attestation Specification" (§1–§11); Discussion #8; the
  reference crate `atproto-attestation` 0.14.5 (`-sign`/`-verify` CLIs).
- Interop probe (A-2, A-4, A-5): reference-signed records verified by our clean-room
  verifier — `fixtures/golden/interop_*`.
- Real records (A-9): `fixtures/recorded/*` from pds.cauda.cloud &
  gomphus.host.bsky.network, 2026-07-20.
- Official-tooling finding (A-6): `scripts/gate.mjs` under `@atproto/lexicon`.
