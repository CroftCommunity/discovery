# RUN-LEX-01 summary — engaging the Lexicon Community attestation work

`Own-lane run, executed 2026-07-20 against the instruction file "ENGAGE-LEX"
(§6 experiment package RUN-LEX-01), landed at
alpha/experiments/lexicon-community/. Grounding fetched in-session: the badge.blue
CID-First Attestation Specification, the reference crate atproto-attestation
0.14.5, lexicon-community Discussion #8, the vendored community calendar/RSVP/
location lexicons, and real records from two public PDSes. Standing red-first
directive honored (evidence below). Drop order under pressure (§6): 05 was
downgraded to a worked example per its own instruction; nothing else dropped.`

**All five EXP land. 33 acceptance tests green red-first; the official
`@atproto/lexicon` gate green; the one-command demo green; a ten-row grounded
ambiguity table (the feedback post) with two security-relevant entries.**

## What landed, by experiment

- **EXP-LEX-01 — fixture corpus + schema harness (8 tests + official gate).**
  Vendored 6 real community lexicons (calendar event/rsvp, 4 location) + 3 candidate
  `community.lexicon.attest.*` DRAFTs. Golden records validate; adversarial records
  fail for the stated reason (missing-required, wrong-type/Incident-1 class,
  strongRef-missing-cid, closed-enum); `knownValues` proven OPEN, `enum` proven
  CLOSED. The **validator of record** is `scripts/gate.mjs` under official
  `@atproto/lexicon` — green, and it surfaced **A-6**: official tooling IGNORES
  unknown fields, so a smuggled scalar validates (matters for the no-scalar
  discipline).

- **EXP-LEX-02 — live-network consumption (5 tests).** Real records captured from
  pds.cauda.cloud and gomphus.host.bsky.network (2026-07-20): 2 calendar events,
  an RSVP, and the RSVP's cross-repo strongRef target. Proven: recomputed CID ==
  authoritative PDS CID; JSON→DAG-CBOR **byte-identical** to the authoritative CAR
  block; IPLD-JSON round-trip stable; a real cross-repo strongRef resolves — and
  its **drift** (pinned cid ≠ current cid after the target was edited) is detected
  and reported honestly (A-9). Live fetch behind `LEXCOMM_LIVE=1`; the crate makes
  no network calls.

- **EXP-LEX-03 — clean-room verifier (11 tests).** Inline + remote patterns, repo
  binding, CID addressing, did:key resolution over all three blessed curves, built
  from `SPEC-BADGE-BLUE.md` alone. **Interop: our verifier accepts records signed
  by the reference implementation**, inline AND remote — the strongest evidence a
  second implementation can bring. Adversarial set all fail for the stated reason:
  mutated payload → CidMismatch, cross-repo replay → CidMismatch / ProofBindingMismatch,
  tampered strongRef → ProofCidMismatch, high-S → HighS, **foreign-key swap →
  lax-ACCEPTS (the A-1 vulnerability) / strict-REJECTS (the fix)**. The interop
  probe empirically resolved the spec's underspecified signing input (A-4/A-5).
  One-command demo: an organizer-signed attendance attestation over a **real public
  RSVP**, verified (`cargo run --example demo_attendance`).

- **EXP-LEX-04 — stapled-status extension (9 tests + benchmark).** The lane's
  CT/RFC-9162 design ported onto the spec's shape, on the spec's SHA-256 hash so it
  composes: the issuer signs the head record's CID with the SAME CID-first ECDSA.
  Green: valid staple verifies with **zero network**; superseded rejected; forged
  inclusion / wrong index rejected; binding must name the presented credential;
  forged head signature rejected; freshness is fail-closed verifier policy;
  cross-era commitments unlinkable. Candidate lexicons drafted
  (`community.lexicon.attest.{treeHead,holderBinding,inclusionStaple}`). Benchmark
  (`BENCH-STAPLE.md`): 870 B staple, offline verify, vs a callback that costs a RTT
  AND leaks (verifier, credential) to the issuer every check.

- **EXP-LEX-05 — lens seam (worked example).** Polite Goshawk (Lenses) is at
  WG-formation with no concrete format (repo 404 on 2026-07-20), so downgraded per
  instruction to `LENS-SEAM-WORKED-EXAMPLE.md`: the Croft envelope-projection →
  `community.lexicon.calendar.event` field map, ready to become a real lens (with
  round-trip tests over EXP-LEX-02's fixtures) the day a format lands.

## Red → green evidence

The standing directive: acceptance criteria as FAILING tests before the
implementation is trusted. This run grounded the CID machinery against real
network truth first (to de-risk), then evidenced red-first with the lane's
staged-violation technique: each EXP's core decision function
(`schema::validate_record`, `consume::cid_matches`/`reserializes_identically`,
`attest::verify_inline`, `staple::verify_staple`) reverted to a permissive stub,
the suite captured RED, then restored to GREEN.

- **RED** — 18 failures across all four suites (EXP-LEX-01: 5, -02: 3, -03: 5,
  -04: 5); positive/interop cases that don't exercise the stubbed reason stayed
  green, exactly as designed. `cargo test --no-fail-fast`. sha256
  `08d7dc56c87772d3…` (`scratchpad/lex-red.txt`).
- **GREEN** — 33 passed, 0 failed across `exp01_schema` (8), `exp02_consume` (5),
  `exp03_verify` (11), `exp04_staple` (9). sha256 `e7598662b630dc59…`
  (`scratchpad/lex-green.txt`).
- Stubs deleted at green (grep `RED-STUB` → none). Clippy clean on all targets.

## Test map

| EXP | tests | result |
|---|---|---|
| 01 | golden validate · knownValues-open · enum-closed · missing-required · wrong-type · unknown-field · strongRef-missing-cid · schemas-load | 8/8 green |
| 02 | cid-matches-authoritative · byte-identical-reserialize · round-trip-stable · strongRef-drift · live-flag | 5/5 green |
| 03 | interop-inline · interop-inline-wrong-repo · interop-remote · self-inline-strict · self-remote · mutated-payload · foreign-key-swap · high-S · proof-cid-tamper · remote-replay · demo-over-real-rsvp | 11/11 green |
| 04 | valid-no-network · superseded · forged-inclusion · wrong-index · binding-names-cred · forged-head-sig · freshness · cross-era-unlinkable · benchmark | 9/9 green |
| official gate | golden(2) · knownValues-open · adversarial-rejected(4) · A-6 note | GREEN |

## Grades (stated)

- **Verified-against-reference**: EXP-LEX-03 interop (our verifier ↔ the reference
  impl's real output) and EXP-LEX-02 (our CID path ↔ real PDS CIDs) — checked
  against external ground truth, not just self-consistency.
- **Modeled (loopback)**: EXP-LEX-04 staple machinery — the crypto is real
  (SHA-256/HMAC/ECDSA, RFC-6962), the era/issuer/co-op context is modeled, no
  deployment. Benchmark numbers are a dev/debug build (a release build is ~1-2
  orders faster; stated in `BENCH-STAPLE.md`).
- **Documented**: EXP-LEX-05 worked example; the AMBIGUITIES table (grounded in
  fetched sources + the interop probe).

## Deviations & honesty

1. **Green-first, red-evidenced-after**: the CID machinery was built and grounded
   against real records before the red pass, because grounding against network
   truth was the highest-value de-risk. Red-first is evidenced by the
   staged-violation revert (digests above), the lane's own accepted technique. Not
   hidden.
2. **`key` excluded from the signed `$sig`** (A-2): required for interop with the
   reference impl; documented as a load-bearing decision, and it sharpens A-1.
3. **Nothing field-deployed**; staple grades Modeled. Anything published on ATProto
   inherits F-AT-6 correlators — the demo README carries the disclosure sentence.
4. **Candidate lexicons** are proposed for `community.lexicon.*` via the WG process;
   nothing lands there by our hand alone. `ing.croft.*` stays the Drystone home.

## Open calls carried (not resolved here — surfaced for the owner)

- **EL OC-1** — first post targets Discussion #8 revival (it is ACTIVE, owner
  @ngerakines engaged, last activity 2025-02-10) vs a fresh thread citing it.
  Reading favors joining #8. Owner call.
- **EL OC-2** — staple naming (status/freshness/staple) and head-record visibility.
- **EL OC-3** — demo attests our own test event vs a real third-party RSVP; this
  run demonstrates over a real public RSVP for the CID/verify proof, but a
  consent-clean own-test-event on a real PDS is the posture for anything published.
- **EL OC-4** — a TypeScript companion verifier (their ecosystem's language) as a
  follow-up run.
- **EL OC-5** — who speaks (personal vs Croft org persona); layer-scoped comms rule.

## Definition of show-and-tell ready (§6) — checklist

- EXP-LEX-01 + 03 green with a non-empty ambiguity table: **yes** (10 rows).
- One-command attendance-attestation demo: **yes** (`cargo run --example demo_attendance`).
- Fixtures publishable under MIT the same day: **yes** (see `../attest-family`
  license posture; fixtures are our own records + reference-impl outputs +
  vendored MIT community lexicons — license gate is the owner's, surfaced not
  resolved).
