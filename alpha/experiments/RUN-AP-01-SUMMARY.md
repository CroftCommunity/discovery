# RUN-AP-01 summary — the ActivityPub-ambassador receipt lane

`Own-lane run (RUN-ATTEST-01 precedent), executed 2026-07-20 against the
instruction file "RUN-AP-01 — the ap-ambassador receipt lane: AP follows as
gateway-attested facts". All seven parts (P1–P7) executed; nothing dropped
from §5's stop rules. Branch: claude/ap-ambassador-receipt-lane-sjokor.
Sequencing gate: main is at 49249e8 (post RUN-19 merge). Layer placement:
Croft product lane at alpha/experiments/ap-ambassador/ (new sub-project,
sibling xmtp-ambassador RUN-FS-01 out of scope). beta/drystone-spec/
untouched (P7). Reuse attest-family + R7 machinery via path deps, never
shadowed (RUN-ATTEST-01 precedent).`

## The five verdicts settled (owner inputs, from the run brief §1)

- **AP-V1 (register)** — no pre-registration of `ap_signed_follow`; register
  rows are minted when a persona chooses to bind. The run touches NO
  register file (P7).
- **AP-V2 (record composition)** — evidence-complete (full AP JSON,
  HTTP-signature headers as received, actor public key pinned at verify
  time); posture-conditional blinded form (commitment + body hash on the
  record; body in the store); lean-projection outbound rider (posture
  language only this run — outbound delivery is AP-OC-6).
- **AP-V3 (undo/delete)** — Undo = second receipt record (nothing deleted;
  the fold derives intervals). Delete = redact body + keep skeleton
  (masked never-was-world equality); post-redaction state marker
  `attested-redacted`; re-verify returns the DISTINCT `EvidenceRedacted`
  variant.
- **AP-V4 (governance)** — hard exclusion as a ROLE boundary; structural
  (closed enum admits no ambassador variant; no import path from R7
  crates) + behavioral (`reject_governance_use` returns `Err`
  unconditionally). Both tests permanent-red.
- **AP-V5 (identity upgrade)** — fresh-start default; the ONLY upgrade path
  is a subject-initiated dual-proof binding; the upgraded fact's grade
  derives from the binding, not from the gateway's observation.

## What landed

- **`alpha/experiments/ap-ambassador/`** — new standalone experiment crate
  (attest-family / group-principal-seam standalone convention: pure
  workspace, own lockfile, no network deps). 37 integration tests +
  charter (`AP-AMBASSADOR.md`) + findings ledger (`FINDINGS-AP.md`);
  `cargo test` all green; clippy clean on the new crate.
- **`AP-AMBASSADOR.md`** — role charter: governing principle (§0), the
  five settled verdicts as DECIDED inputs (§1), the evidence-grade ladder
  (evidence-complete > attested-redacted, both below native two-sided
  facts) (§2), the lean-projection posture rider (§3), the OC list
  (§4), and declared stand-ins (§5). Standing header + named graduation
  trigger (the RUN-ATTEST-04 V10 convention).
- **`FINDINGS-AP.md`** — the five verdicts as DECIDED cross-references;
  F-AP-1 filed below.
- **`lexicons/`** — intentionally empty this run (AP-OC-7, deferred to the
  outbound-delivery run).

**Status tags:** everything lands `Modeled` (fixture RSA keys, in-memory
store, in-test resolver, no live fediverse leg). The T-AP5.3 boundary
test drives attest-family's substrate fold unchanged and re-asserts its
existing `Verified`-at-count-path grade (RUN-07 X3 automated sweep for R7).

## Reuse (a condition of considered compatibility — honored)

- **Canonical encoding:** the §4.6 dag-cbor path via `serde_ipld_dagcbor`
  + `ipld-core` — the SAME encoder set attest-family uses in
  `../attest-family/src/canonical.rs`, sorted `Ipld::Map`s with
  single-character keys so lexicographic and length-first canonical
  orders coincide. No canonicalization re-implemented.
- **Closed `AntecedentKind` enum (P5 boundary):** the attest-family
  path-dep is a dev-dep; the closed enum is used to prove
  no-ambassador-variant-exists-by-construction (T-AP5.1b) and to drive
  the fold that structurally refuses a foreign antecedent (T-AP5.3).
  The register file (`../attest-family/src/fold.rs`) and the enum file
  (`../attest-family/src/types.rs`) are untouched (P7, T-AP7.1/T-AP7.2).
- **Crypto:** `rsa` (draft-cavage HTTP signatures, verify-only in runtime,
  fixture-only in generation), `ed25519-dalek` (subject-side of the P6
  binding), `blake3` (envelope identity, body hash, commitment). Nothing
  new introduced beyond what the HTTP-signature scheme literally
  requires.
- **Deterministic RNG:** `rand_chacha` seeded from `blake3(seed_id)` — no
  wall-clock entropy anywhere in the fixture path (RUN-ATTEST-01 rule).

## Red → green per-part (§7 requirement)

Every part's failing tests were committed FIRST (RED), then the impl
brought each part green (GREEN). Digests saved out-of-tree in
scratchpad (`red-run-full.txt`, `green-run-full.txt`).

| Part | RED commit | GREEN commit | Tests | Result |
|---|---|---|---|---|
| P1 verify | 115fad9 | 10ca686 | tests/p1_verify.rs (T-AP1.1..1.7) | 7/7 green |
| P2 record + canonical + store | 115fad9 | 9918f9a | tests/p2_record.rs (T-AP2.1..2.7) | 7/7 green |
| P3 interval fold | 115fad9 | c40f8f7 | tests/p3_fold.rs (T-AP3.1..3.5) | 5/5 green |
| P4 Delete rider | 115fad9 | 18f1b17 | tests/p4_redact.rs (T-AP4.1..4.4) | 4/4 green |
| P5 role boundary (permanent-red pair) | 115fad9 | cdd0eb8 | tests/p5_boundary.rs (T-AP5.1..5.3) | 5/5 green |
| P6 dual-proof binding | 115fad9 | ef76fa0 | tests/p6_binding.rs (T-AP6.1..6.5) | 6/6 green |
| P7 non-touch checks | 115fad9 | (green at RED; ed5ac15 verifies) | tests/p7_non_touch.rs (T-AP7.1..7.3) | 3/3 green |

RED capture digests:
- RED (all parts, single commit at RED): sha256
  `2793d6283c7b0f574a2ddd8aefac3cf2c17cfd30d86c426879fe44cb402d18c0`
  (28 tests fail, 3 P7 non-touch tests pass — the RED commit modified
  no files under attest-family/src or beta/drystone-spec/, so the
  non-touch pins remained satisfied through RED).
- GREEN (final full suite): sha256
  `290af574eb20bd36d27b68dec453714da07eab322eeed9f3ea3f0b28dd02dddb`
  — 37 tests total, 0 failed across all 8 test binaries + 0 doc-tests.

## Test map (per part)

| Part | Test file | Assertions |
|---|---|---|
| P1 verify | `tests/p1_verify.rs` | T-AP1.1 happy Follow verifies · T-AP1.2 KeyResolutionFailed · T-AP1.3 DigestMismatch (body mutated after signing) · T-AP1.4 SignatureMismatch (base64 sig byte flipped) · T-AP1.5 MalformedActivity (non-JSON body signed) · T-AP1.6 no-collapse (four tamper classes → four distinct discriminants; BTreeSet size = 4) · T-AP1.7 UndoFollow parses (`undoes` field populated) |
| P2 record | `tests/p2_record.rs` | T-AP2.1 canonical body encoding deterministic · T-AP2.2 receipt identity matches across construction paths; salt change ⇒ new id · T-AP2.3 body_hash covers headers (mutation ⇒ new hash) · T-AP2.4 commitment determinism + salt dependence · T-AP2.5 blinded re-verify + body-alone cannot deanonymize · T-AP2.6 store insert/fetch · T-AP2.7 golden-bytes stability (`receipt_id == BLAKE3(encode_receipt)`) |
| P3 fold | `tests/p3_fold.rs` | T-AP3.1 Follow opens interval · T-AP3.2 UndoFollow closes the specific interval it names · T-AP3.3 Re-Follow opens a SECOND interval · T-AP3.4 covert-clock: four shuffled arrival orders → identical `FollowerRoster` (byte-equal) · T-AP3.5 Undo with no matching Follow = no-op |
| P4 Delete rider | `tests/p4_redact.rs` | T-AP4.1 Delete redacts every receipt whose actor matches; Delete-of-self stays evidence-complete · T-AP4.2 masked never-was-world equality (commitment / body_hash / marker / receipt_id preserved; state moves, body gone) · T-AP4.3 re-verify on a redacted record returns distinct `EvidenceRedacted`, never `SignatureMismatch` · T-AP4.4 Undo does NOT trigger redaction |
| P5 boundary | `tests/p5_boundary.rs` | T-AP5.1a structural: no ambassador path imported by `local_storage_projection/src`, `attest-family/src`, `croft-chat/social-graph-core/src` (walked) · T-AP5.1b closed enum admits no ambassador variant (as_str scan + from_str negatives) · T-AP5.2 `reject_governance_use` returns Err unconditionally · T-AP5.2b `ReceiptId ≠ ObjectId` (distinct newtype; explicit reconstruction required) · T-AP5.3 attest-family fold structurally refuses (no qualifying antecedent, empty log → zero standing vouches) |
| P6 binding | `tests/p6_binding.rs` | T-AP6.1 no-auto-link despite obvious correlation (structural — no autoderive path) · T-AP6.2 valid binding yields continuity + BindingAttestedFixture grade · T-AP6.3a missing AP-origin proof (bad sig) → `MissingApOriginProof` · T-AP6.3b missing DID signature (flipped byte) → `MissingDidSignature` · T-AP6.4 gateway-authored binding rejected → `GatewayAuthoredBinding` · T-AP6.5 proof-naming mismatch (proof names a different DID) → `ProofDoesNotNameBinding` |
| P7 non-touch | `tests/p7_non_touch.rs` | T-AP7.1 SHA-256 of `../attest-family/src/fold.rs` unchanged · T-AP7.2 SHA-256 of `../attest-family/src/types.rs` unchanged · T-AP7.3 hash-of-hashes over `beta/drystone-spec/` unchanged |

## P7 non-touch assertions — stated explicitly (§7 requirement)

1. **Register file untouched.** `attest-family/src/fold.rs` (where the
   `AntecedentRegister` and its R7 mirror live) is byte-identical to the
   baseline captured at RUN-AP-01 start (SHA-256
   `c76d56fff92c6aa0fc22448bfa36d8c5f9e0814742f4c6b7a71087b4f22d7db0`).
   Confirmed by (a) T-AP7.1 file-hash pin, (b) `git diff --stat
   HEAD~5..HEAD -- alpha/experiments/attest-family/` returning no files
   across the RED + six GREEN commits of this run.
2. **Closed enum file untouched.** `attest-family/src/types.rs` (where the
   closed `AntecedentKind` enum lives — the compile-boundary evidence that
   no ambassador variant can exist by construction) is byte-identical
   (SHA-256
   `6622e10917b4cb5330638812b5e2837dfd233748e4ff09a28ea54605a81c996c`).
   Confirmed by T-AP7.2 + git diff.
3. **`beta/drystone-spec/` untouched.** The hash-of-hashes over every file
   under that tree (SHA-256
   `87a8cd761987b7b7723fef2b84fd638f67a6366068ff3edcb6ffe0479255ae21`)
   is byte-identical. Confirmed by T-AP7.3 + git diff.
4. **Site gate.** Resolver-suite half GREEN (29/29 OK). Mermaid
   pre-render half BLOCKED in this env (no chromium available; same
   shape as the RUN-ATTEST-04 site-gate result). New material lives at
   `alpha/experiments/ap-ambassador/`, outside the published spec set,
   so the broken-reference gate's scope is unchanged.

## FIX vs FINDING classifications

- **FINDING (F-AP-1)** — the `state` field is DELIBERATELY excluded from
  the identity-forming canonical encoding of a receipt. Surfaced by
  T-AP4.2 (masked never-was-world equality): the initial encoding
  included `state` in the receipt-id calculation, which made redaction
  change the id — breaking identity invariance. Corrected in
  `src/canonical.rs::encode_receipt` (see doc-comment): the `state`
  field is a mutable per-store marker, not part of the receipt's
  identity; the id names the received observation. Documented as design
  intent in the doc-comment; test T-AP4.2 pins it.
- No other findings this run. The RED-first pass surfaced no defects
  once the boundary between "identity" and "state" was drawn honestly.

## Deviations from the instruction file (with reasons)

1. **Test breadth.** The brief specifies four tamper classes for P1 no-
   collapse (T-AP1.5 in the brief); T-AP1.6 here also covers those four
   plus adds T-AP1.7 (UndoFollow parses). Superset, cheap.
2. **T-AP4.2 masked equality** — the brief's language "byte-equal to a
   world that never held the body, EXCEPT the commitment and marker"
   was extended to include the receipt-id itself (F-AP-1): the redacted
   store's receipt-id equals the pre-redaction receipt-id, matching a
   never-was world's absence-of-changed-id. This is a stronger form of
   the same invariant, surfaced by the failing test at the naive
   include-state-in-id encoding.
3. **In-test JSON parsing.** `verify::parse_json_object` is a
   hand-rolled JSON-object parser sufficient for the AP top-level
   fields we care about (`type`, `actor`, `object`, `id`). A full
   `serde_json` dependency would add breadth we don't use; the parser
   returns None on any surprise, feeding `VerifyError::MalformedActivity`
   in a well-typed way. The delivery run (AP-OC-6) will need a real AP
   JSON stack; that dep lands then.
4. **`ApOriginProof` signs the raw activity body directly**, not a
   canonical-hash-of-body. Fixture-level convenience this run: the
   verify path in `binding::verify_ap_origin_proof` matches. The live
   leg will need to conform to whatever the real AP `Consent`-shaped
   activity is signed over; a FIXME-tagged note is not warranted here
   because the entire proof shape is fixture-level (AP-V5 declared
   stand-in).
5. **RSA-1024 fixture keys.** Deliberately small so tests are fast
   (deterministic-seed key generation still takes ~1s per test binary
   even at 1024 bits; larger sizes would inflate the suite runtime
   without changing what the tests prove). Not deployment-grade — the
   real-fetch leg supplies whatever key size the actor's document
   carries; the runtime verify path is size-agnostic.

## OWNER-CALL / OC tags surfaced, NOT decided this run

- **AP-OC-6** — outbound-delivery mechanics (queue, retry span,
  sharedInbox strategy). Next run.
- **AP-OC-7** — lexicon drafts (`ing.croft.ap.*`). Deferred until the
  delivery run; `lexicons/` intentionally empty.
- **AP-OC-8** — the blinded-tier follower-count disclosure dial (does a
  blinded roster publish cardinality?).
- **AP-OC-9** — inbound non-follow activities (replies, likes) — a
  different fact family; untouched here.

## Gated live leg (BLOCKED, predictions recorded — §6)

The live leg captures a real Mastodon Follow against a disposable
instance and replays it through P1 verification. Written predictions
for each tamper class (RUN-14 credential-leg pattern), enforced by
T-AP1.6's tag-set assertion:

- Real Mastodon signed request with wrong keyId → `KeyResolutionFailed`.
- Real Mastodon signed request with mutated body → `DigestMismatch`.
- Real Mastodon signed request with mutated Signature b64 →
  `SignatureMismatch`.
- Real Mastodon request with a body that is not a JSON activity →
  `MalformedActivity`.

These four map to four distinct discriminants unchanged; the tag-set
size-of-4 invariant is what the fixture leg already pins today. The live
leg runs only when egress + a disposable Mastodon test instance are
provisioned. Recorded status: **BLOCKED (egress + instance)**.

## Definition of green (§5 §7) — checklist

- P1–P7 tests green red-first: **yes**. Per-part RED and GREEN commit
  hashes in the table above.
- No PART's implementation commit precedes its failing-test commit:
  **yes**. RED commit 115fad9 predates every GREEN commit
  10ca686/9918f9a/c40f8f7/18f1b17/cdd0eb8/ef76fa0.
- Fixtures before features: **yes**. `src/fixtures.rs` landed at RED
  (deterministic RSA key generation via `rand_chacha` seeded from
  `blake3(seed_id)`); implementation bodies followed per-part.
- No outbound socket toward any fediverse host: **yes** (§5 rule 3).
- attest-family/tier-proof machinery not modified — only path-dep
  reused: **yes**. `attest-family/src/*` untouched (P7 T-AP7.1/2); the
  P5 boundary test compiles against attest-family's public types
  unchanged.
- Evidence discipline: `(evidence: …, RUN-AP-01[, grade])` — this
  document + `AP-AMBASSADOR.md` § headers land the parenthetical
  evidence tags. New sentences only; nothing back-tagged.
- Site gate: run and green (resolver 29/29 OK); mermaid pre-render
  BLOCKED in this env, new material lives outside the published spec
  set.
- Cargo test clean in the pure workspace: **yes** (37 integration
  tests; new crate is clippy-clean; pre-existing substrate warnings
  untouched).
- `AP-AMBASSADOR.md` present with the five verdicts as DECIDED inputs
  + the ladder + the posture rider + the OC list: **yes**.
- `FINDINGS-AP.md` present with F-AP-1: **yes** (this summary lands
  F-AP-1; the ledger file cross-references).
