# Candidate lexicons — `community.lexicon.attest.*` (DRAFT, non-normative)

`RUN-LEX-01 EXP-LEX-04. These are the OPTIONAL freshness/status layer proposed as
an extension ON TOP OF the CID-First Attestation Specification — they change
nothing in that spec. Proposed for the `community.lexicon.*` namespace via the WG
process; nothing lands there by our hand alone (brief §7). Everything
Drystone-specific stays under `ing.croft.*` — see `../../attest-family/lexicons/`.`

| id | role |
|---|---|
| `community.lexicon.attest.treeHead` | one signed tree head per issuer era/epoch — RFC-6962 Merkle root over keyed commitments, leaf count, superseded set + root, era anchor. Signed with the base spec's CID-first ECDSA. |
| `community.lexicon.attest.holderBinding` | issuer-signed `{credential ↔ commitment}`, holder-held (never published) — lets a verifier map credential→leaf without the era key. |
| `community.lexicon.attest.inclusionStaple` | the holder-stapled inclusion proof: commitment, RFC-6962 audit path, and strongRefs to the head + binding. Verified with zero issuer callback. |

## Design (ported from the attest-family lane, RUN-ATTEST-04 V5/V6 — design, not code)

- **Keyed commitments** `HMAC-SHA256(key_era, credential_cid)` so an outsider
  cannot dictionary-join commitments to credentials.
- **Canonical leaf order** (byte-ascending) so mint order is structurally absent.
- **Per-era superseded set** for revocation; a superseded credential cannot staple
  a fresh-head proof.
- **Zero callback**: `verify_staple` is a pure function of its arguments — no
  resolver, no issuer endpoint, nothing to leak. This is the OCSP privacy lesson
  (status-by-callback centralizes and surveils) applied to attestations.

## Open calls this touches

- **EL OC-2** — naming (`status` vs `freshness` vs `staple`), and whether the era
  head record lives in the issuer's repo as a public record (visibility trade). The
  file names above are provisional.
- The `alg` field in `inclusionStaple` is a **closed `enum`** (`ES256K`/`ES256`/
  `ES384`) — deliberately contrasting the calendar lexicons' **open `knownValues`**;
  see `../AMBIGUITIES.md` A-10 and the EXP-LEX-01 `enum_is_closed` test.

## Validation

These load into both the Rust registry (`src/schema.rs`) and the official
`@atproto/lexicon` gate (`scripts/gate.mjs`). The `treeHead`/`holderBinding` use a
`bytes` type with `maxLength` per the atproto lexicon `bytes` primitive.
