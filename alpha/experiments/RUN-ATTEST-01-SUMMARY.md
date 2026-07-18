# RUN-ATTEST-01 summary — Attestation family: proving the model in code

`Own-lane run (like RUN-STELLIN-INFRA-01), executed 2026-07-18 against the instruction
file "RUN-ATTEST-01 — Attestation Family: Proving the Model in Code". All parts (0–6)
executed; nothing dropped from §7's drop order.`

## What landed

- **`attest-family/`** — new standalone experiment crate (group-principal-seam
  convention: pure workspace, own lockfile, no network deps). One attestation family
  on two axes (subject: persona|thing; consent: mutual | unilateral_notice |
  unilateral_private), with scope labels, supersede-never-revoke,
  per-viewer resolvability, corroboration structure, and provenance grades as shared
  machinery. 35 integration tests + 6 compile_fail doc-tests, all green; clippy clean
  on the new crate.
- **`attest-family/PRIMITIVES-ATTEST.md`** — the usable primitive language
  (DRAFT-PENDING living doc, alpha side): every §4 term with one sentence of
  definition and one sentence of what it is NOT; declared stand-ins; OC-1..OC-4
  surfaced, none decided.
- **`attest-family/FINDINGS.md`** — F-AT-1 (T-AT4.3 correlation residue), F-AT-2
  (T-AT4.4 issuer linkage seam, Modeled by design), F-AT-3 (hash-erasure reading of
  T-AT0.4), F-AT-4 (the no-suppression allowlist pin), F-AT-5 (covenant amendments
  must be causally chained).

**Status tags:** everything lands `Modeled` (fixture keypairs, in-memory fold,
loopback-free). T-AT6.4 reuses the substrate's R7 content-bound quorum machinery
unmodified and cites the reused portion at its existing grade (R7 count path
cross-package `Verified` per RUN-07; §7.6.1 competing-RuleChange hard-stop per
RUN-03); the covenant *modeling* on top is `Modeled`.

## Reuse (a condition of considered compatibility — honored)

- **Canonical encoding:** the §4.6 dag-cbor path via `serde_ipld_dagcbor` +
  `ipld-core` (the proven `public-roundtrip` crates), sorted `Ipld::Map`s with
  single-character keys so lexicographic and length-first canonical orders coincide.
  No canonicalization re-implemented.
- **R7 quorum machinery (T-AT6.4):** `local_storage_projection::DerivedFold` +
  `social_graph_core::{Ed25519Verifier, RegistryCredentialResolver, Session}` driven
  exactly as `croft-chat/tests/competing_quorums.rs` drives them; envelope-building
  test scaffolding copied from `croft-chat/tests/common/mod.rs` (scaffolding copied,
  machinery imported — nothing shadowed).
- **Crypto:** ed25519-dalek (social-graph-core's pin) + blake3 (the committed hash
  suite). Nothing new.
- **Permutation harness:** none existed as a reusable unit; `fixtures::permutations`
  (Heap's) + `fixtures::seeded_shuffle` (xorshift, no wall-clock entropy) mirror the
  `convergence_property.rs` pattern. Candidate for extraction if a third crate needs it.

## Red → green evidence (per §2 TDD directive)

One full-suite RED batch was captured before implementation, per test group, then
implementation brought groups green. Digests of the saved outputs (scratchpad
`red-run-full.txt` / `green-run-full.txt`):

- RED (all groups, 2026-07-18): 31 of 35 integration tests FAILED (42 failure lines
  incl. per-test panics), sha256 `93ecf8c75d3ea913…`. The four floor tests, all seven
  Part 1 tests, and every Part 2–6 test failed against `unimplemented!` fold/query
  stubs or staged violations (below). The only pre-green passes: the trivial marker
  vocabulary pin, and the 4 compile_fail doc-tests.
- GREEN (final): 35 integration + 6 doc-tests, 0 failed, sha256 `e1fa9834f96aca9f…`.
  Per group: t_at0_floor 4/4 · t_at1_edge 8/8 · t_at2_vouch 4/4 ·
  t_at3_corroboration 5/5 · t_at4_resolvability 4/4 · t_at5_review 5/5 ·
  t_at6_predicate 3/3 · t_at6_covenant 2/2 · doc-tests 6/6.

**Refutation-pin reds for negative invariants** (a vacuously-green negative test is
no red, so the violation was staged, captured failing, then removed):

- T-AT0.1: canonical decode is fixture machinery, so it was green at birth; re-staged
  RED by stubbing `decode_envelope` (`unimplemented!`), then restored.
- T-AT0.2 / T-AT3.1: a staged `pub trust_score` field on `CorroborationStructure`
  made the source-scan + serialization-walk invariant fail; deleted at green.
- T-AT6.1: a staged free-form `note: &'static str` on `ProcessProvenance` (substrate-
  capable field) made the type-region scan fail; deleted at green.
- T-AT6.4: the covenant was first staged UNGOVERNED (no RuleChange-threshold raise) —
  a lone owner could weaken the register and both covenant tests failed; modeling it
  as a content-bound 2-quorum R7 rule turned them green. The red demonstrates exactly
  what the R7 modeling buys.

## Test map (per part)

| Part | Tests | Result |
|---|---|---|
| 0 floor | T-AT0.1 `canonical_roundtrip` (all 11 kinds byte-identical) · T-AT0.2 `no_score_field_exists` (source scan + numeric-leaf walk + compile_fail doc-tests) · T-AT0.3 `supersede_preserves_prior` · T-AT0.4 `ordering_ignores_wallclock` (hash-erased projection; see F-AT-3) | 4/4 green |
| 1 edge | T-AT1.1 half≠edge (pending never partial) · T-AT1.2 edge iff matching core hash · T-AT1.3 tampered core → two pendings forever, no error-verdict · T-AT1.4 side-local labels · T-AT1.5 supersede lineage · T-AT1.6 order-independent fold (all 120 permutations of 5 arrivals — superset of the spec's 3-object set) · T-AT1.7 ceremony grade (both-signed session → `in_person`; one-sided → `remote`; grade-is-metadata pinned by compile_fail doc-tests + source scan) · plus a marker-vocabulary pin | 8/8 green |
| 2 vouch | T-AT2.1 edge-antecedent required (`// OWNER-CALL: OC-2 pending`) · T-AT2.2 vouch supersede leaves edge fold identical · T-AT2.3 edge supersede → `antecedent_superseded` marker, never auto-withdraw (`// OWNER-CALL: OC-3 pending`) · T-AT2.4 transaction antecedent → `transaction_backed` (vouch + review) | 4/4 green |
| 3 corroboration | T-AT3.1 structure-not-scalar, canonical-hash order only · T-AT3.2 exact scope, no bleed · T-AT3.3 unresolvable attester ABSENT (no id bytes, no count) · T-AT3.4 viewer relativity, each view internally consistent · T-AT3.5 mutual count cardinality-only, leakage-fuzzed over 24 seeded cases | 5/5 green |
| 4 resolvability | T-AT4.1 holder cannot grant far end (opaque carries nothing) · T-AT4.2 policy change is supersede with lineage (+ rogue-author policy ignored) · T-AT4.3 correlation resistance property over 16 seeded cases (residue → F-AT-1) · T-AT4.4 issuer seam FINDINGS entry enforced by test | 4/4 green |
| 5 review | T-AT5.1 stands without countersign · T-AT5.2 deterministic notice fact (all arrival permutations) · T-AT5.3 signed reply is peer object (rogue reply does not attach) · T-AT5.4 no suppression path (public-op allowlist pin + behavioral; → F-AT-4) · T-AT5.5 freshness is presentation (stale marker; membership and order unchanged) | 5/5 green |
| 6 predicate + covenant | T-AT6.1 substrate unrepresentable (compile_fail doc-tests + closed-type scan) · T-AT6.2 predicate inseparable from issuer + process · T-AT6.3 refresh via supersede, staleness presentation only · T-AT6.4 covenant as R7 rule on REUSED machinery: (a) content-bound quorum (wrong-content approval refused), (b) under-quorum refused whole, (c) change + approval antecedents visible in exported lineage, + contradiction hard-stop inherited unchanged (both arrival orders, min-hash byte-head) | 5/5 green |

## OWNER-CALL tags emitted (decisions NOT made)

- **OC-1** — graduation target of `PRIMITIVES-ATTEST.md` (stays DRAFT-PENDING alpha).
- **OC-2** — vouch-without-edge; narrow option implemented, tagged at T-AT2.1.
- **OC-3** — vouches on superseded edges; persist-with-marker implemented, tagged at T-AT2.3.
- **OC-4** — `unilateral_private` in v1; vocabulary defined, zero tests by design.

## FIX vs FINDING classifications

- **FINDING (F-AT-1)** — correlation residue behind the identifier floor: shared
  counterpart, timing-shaped structure, ceremony geography. Expected; recorded.
- **FINDING (F-AT-2)** — the co-op issuer linkage seam; v1 posture = per-persona
  optional issuance + no-record covenant; deferred direction = BBS-style unlinkable
  presentations (§9). Modeled by design.
- **FINDING (F-AT-3)** — wall-clock claims change identities, never outcomes; T-AT0.4
  is stated (and passes) under hash erasure.
- **FINDING (F-AT-4)** — the no-suppression invariant's load-bearing form is the
  public-operation allowlist pin.
- **FINDING (F-AT-5)** — a covenant amendment must causally cite the register state it
  amends; an unchained quorum-met amendment is a concurrent competitor and correctly
  hard-stops. Surfaced by the first green attempt at T-AT6.4; test corrected to chain.
- **FIX (in-run)** — FINDINGS phrase wrapping broke T-AT4.4's exact-phrase check;
  rewrapped. No code behavior involved.

## Deviations from the instruction file (with reasons)

1. **T-AT1.6 breadth**: permutes all five arrivals (ceremony facts + halves +
   supersede; 120 orders) instead of only {half A, half B, supersede} — strictly a
   superset, cheap at this scale.
2. **Property-test style**: T-AT3.5 / T-AT4.3 use seeded-loop property tests (24/16
   cases, deterministic xorshift shuffles) in the `convergence_property.rs` style
   rather than `proptest` — no wall-clock/OS entropy is available to the harness by
   design (the harness itself obeys the no-wall-clock rule), and the case space is
   structured (worlds of keypairs), which proptest shrinks poorly. `proptest` was
   dropped from dev-deps as unused.
3. **Red staging for negative invariants**: T-AT0.1/0.2/6.1/6.4 reds were produced by
   staged violations (documented above) since a negative invariant over an empty
   crate is vacuously green — the refutation-pin style RUN-03 established.
4. **`mutual_connection_count` takes (a, b) with no viewer parameter**: the
   disclosure is cardinality-only and deliberately viewer-independent — identity of
   counterparts is not returned to anyone, so there is nothing for a viewer policy to
   filter. Recorded in the query docs.
5. **Stand-in default**: with no policy fact on record a persona is
   resolvable-to-all; declared in PRIMITIVES-ATTEST.md as a stand-in default, not a
   decided posture (adjacent to OC-4's axis but not one of the listed owner calls —
   flagged there rather than silently chosen).

## Definition of green (§5) — checklist

- All parts' tests green: **yes** (table above).
- T-AT0.* invariants green across the whole crate: **yes** (0.2's scan covers every
  src file; 0.3's byte-identity is re-asserted inside Parts 1/2/4/6 tests).
- `cargo test` clean in the pure workspace: **yes** (35 + 6 doc-tests; new crate is
  also clippy-clean; pre-existing substrate warnings untouched).
- Site gate: **run and green** (86 pages; the new docs live next to the crate, not in
  the published spec set, so the gate's scope is unchanged — run anyway per §2).
- `PRIMITIVES-ATTEST.md` present with every §4 term defined: **yes** (12 terms + 4
  machinery terms, each with an is/is-NOT pair).
- FINDINGS entries for T-AT4.3 residue and T-AT4.4 seam: **yes** (F-AT-1, F-AT-2).
