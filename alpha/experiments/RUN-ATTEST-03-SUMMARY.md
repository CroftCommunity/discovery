# RUN-ATTEST-03 summary — Settlement riders (V1–V3) + ATProto abilities matchup

`Own-lane run, executed 2026-07-18 against the instruction file "RUN-ATTEST-03 —
Settlement Riders (V1–V3) + ATProto Abilities Matchup", layered on the merged
RUN-ATTEST-01/02 attest-family crate (gate met: both audited merged against
snapshot 0718-discovery-main). Settles the first three owner-call verdicts of the
2026-07-18 walk and produces the ATProto integration brief. Nothing dropped from
§8's drop order — Parts A, B.1, B.2, and C all executed.`

## The verdicts settled (owner decisions, confirmed in chat 2026-07-18)

- **V1 (AT OC-2 → option B).** Vouches require a qualifying antecedent from a
  **closed class** — co-signed edge, transaction attestation, or ceremony fact —
  not an edge specifically. Rationale of record: shapes of one provenance
  mechanism; what varies is the kind of trust bound — bidirectional (edge) or
  unidirectional (transaction, ceremony) — and both are valid.
- **V2 (AT OC-3 ratified + tier clarification).** Persist-with-marker stands; the
  marker is kind-specific (`edge_superseded`); claw-back = the author removing
  the record from their own PDS on the public/ATProto tier, never proactive
  network pull-back; amend = whole-record replace; no wall-clock anywhere.
- **V3 (AT OC-4 → option B).** `unilateral_private` deferred from v1; ships as a
  private-substrate artifact (an MLS-group-of-one), never a fourth public
  consent mode.

**Explicitly untouched (the walk is paused at item 4):** PA-series OC-1..4, the
resolvable-to-all stand-in default, F-PA-3 graduation, and PRIMITIVES' home
(AT OC-1). Confirmed by grep at close: every `OWNER-CALL: OC-n pending` tag in
`t_pa1_no_default.rs` (PA OC-1/OC-2), `t_pa5_dual.rs` (PA OC-3),
`t_pa_substrate.rs` (PA OC-4), the `sole_anchor` references in `types.rs` and
PRIMITIVES' RUN-ATTEST-02 section are byte-untouched. Only the three AT-series
tags moved, per the §2 named conditional edit, to
`DECIDED (V-n, 2026-07-18, owner-confirmed in chat)`.

## What landed

- **Part A (crate riders, red-first)** — `AntecedentKind` closed enum
  (compile-boundary: no string escape hatch; 2 new compile_fail doc-tests) with
  the qualifying-antecedent standing rule (edge: author+subject are the folded
  edge's participants; transaction: payer=author, payee=subject; ceremony: fact
  names exactly {author, subject}); kind-derived grade set
  (`edge_backed`/`transaction_backed`/`ceremony_backed`, the kind set's exact
  image, in declaration order); the qualifying-kind list as a **governed
  register on the reused R7 machinery** (substrate rule_key 2
  `role_change_threshold` reinterpreted as the kind bitmask, declared stand-in;
  crate mirrors via `AntecedentRegister`, mirroring pattern of T-PA3.2's dial);
  marker renamed `edge_superseded` (kind-specific, unrepresentable for
  tx/ceremony — 2 more compile_fail doc-tests pin that those facts have no
  ended state); withdrawn-is-absent-not-tombstoned proven by masked-structural
  equality with a never-was world.
- **Part B.1** — `ATTEST-ATPROTO-MATCHUP.md` (alpha/experiments/, beside the
  other briefs): ten-row required-abilities inventory, every current-ATProto
  claim anchored to a primary source fetched in-session 2026-07-18
  (repository/lexicon/DID/sync/XRPC specs, PLC directory spec, the Spring 2026
  protocol roadmap, Private Data WG material — no import-markers needed, network
  was available). Carries the CID-circularity analysis (core-hash equality is
  the real join; CID citation is one-directional convenience), the
  delete-visibility bounding (no-residue at the authoritative layer and
  compliant views; NOT network amnesia), and the PLC-correlator analysis filed
  as **F-AT-6**.
- **Part B.2** — `attest-family/lexicons/`: DRAFT non-normative JSON for
  `ing.croft.attest.{edgeHalf,vouch,review,reviewReply,credential,commitmentEpoch}`
  — closed sets as lexicon `enum` (never `knownValues`), the V1 class in the
  vouch's antecedent citations, `unilateral_private` deliberately absent from
  the review consent enum, no numeric score field anywhere.
- **Part C (gated; NOT dropped)** — `src/atproto_map.rs`: pure, no-network
  lossless payload ↔ record-shape round-trip (T-A3.7) and the test-enforced
  `fields_without_lexicon_home` list (T-A3.8) — the two-tier boundary made
  mechanical (resolvability policy → drystone; ceremony session privates →
  drystone/private; seam-typed issuer state → issuer seam; plus
  withdraw/dissolve/supersede as record OPS, the stand-in facts as named
  residuals, envelope fields as fold/commit machinery, the detached epoch
  signature replaced by the repo commit signature).

**Status tags:** everything lands `Modeled`. T-A3.4 reuses the substrate's R7
content-bound quorum machinery unmodified and cites the reused portion at its
existing grade (R7 count path cross-package `Verified` per RUN-07; §7.6.1
hard-stop per RUN-03); the register *modeling* on top is `Modeled`.

## Red → green evidence (per the §2 carry-over, one RED batch before implementation)

Digests of saved outputs (scratchpad):

- **RED (Part A, 2026-07-18, `--no-fail-fast`)** — new tests written against the
  PRE-CHANGE fold (types extended, fold semantics untouched): T-A3.1
  `plumber_case_red` FAILED exactly as designed (transaction-backed edge-free
  vouch folded Pending — the captured failing state IS the verdict's
  motivation), T-A3.2 (ceremony-qualifying leg), T-A3.3, T-A3.5, and T-A3.4's
  governed-mirror assertions FAILED; the pre-existing T-AT5.4 allowlist pin
  ALSO failed on the four new register operations pending review — the F-AT-4
  flow working as designed. sha256 `a4f00c6c4e680259…`
  (`attest3-red-run-full.txt`).
- **RED (T-A3.6, refutation-pin style)** — the absence invariant was green at
  birth against the pre-change fold (corroboration already returned only
  Standing entries), so the violation was staged: a `withdrawn: bool` tombstone
  field on `CorroborationEntry`, withdrawn vouches included and flagged, a
  `"w"` tombstone key serialized. All three assertion families failed
  (masked-equality, trace scan, source scan); staging deleted at green. sha256
  `6f9b2d34706593b3…` (`attest3-red-tombstone-staged.txt`).
- **RED (Part C)** — T-A3.7/T-A3.8 against `unimplemented!()` stubs. sha256
  `f1afc25f7938f3f9…` (`attest3-red-map.txt`).
- **Green at birth, stated honestly** —
  `antecedent_register_inherits_contradiction_hard_stop` pins behavior the
  reused substrate enforces regardless of register (concurrent same-register
  quorum-met RuleChanges hard-stop, §7.6.1); no staging short of breaking the
  reused machinery could red it — which is precisely what "inherited
  unchanged" means (same class as RUN-ATTEST-02's
  `dial_inherits_contradiction_hard_stop`).
- **GREEN (final)**: 67 integration + 11 doc-tests, 0 failed; crate
  clippy-clean (all remaining clippy output is pre-existing substrate-dep
  warnings, untouched). sha256 `8b1bef4da5c0cc06…` (`attest3-green-final.txt`).
  Part-A checkpoint green: sha256 `bc5333080739ab8b…`
  (`attest3-green-run-full.txt`).

## Test map

| Part | Tests | Result |
|---|---|---|
| A.1 V1 | T-A3.1 `plumber_case_red` (tx-backed edge-free vouch: pending pre-change → standing `transaction_backed`, end-to-end through corroboration + withdraw path) · T-A3.2 `antecedent_class_is_closed` (3-kind vocabulary exact; unit-variant source region; near-miss fold cases all pending: bare, wrong-payer, wrong-payee, wrong-pair; ceremony kind proven qualifying) · T-A3.3 `grade_derives_from_kind` (multi-antecedent vouch carries the SET in declaration order; kind set ↔ grade set exact; no-consumption scan re-asserted; serialization `"j"` list; numeric walk clean) · T-A3.4 `antecedent_register_governed` (under-quorum refused whole; quorum-met narrow applies with lineage + approval antecedent visible; refused change traceless; mirror: mask 7 stands the tx vouch, mask 1 folds it pending while the edge vouch stands) + `antecedent_register_inherits_contradiction_hard_stop` (both orders, byte-head min; register keeps established class) | 7/7 green |
| A.2 V2 | T-A3.5 `only_edges_supersede` (tx/ceremony vouches: zero markers under adversarial dissolves naming their ids; edge vouch marked — contrast; single guarded attach site pinned; compile_fail doc-tests pin no-supersede-field on TransactionFact/CeremonyFact) · T-A3.6 `withdrawn_is_absent_not_tombstoned` (masked-structural equality withdrawn-world ↔ never-was-world; no id trace, no tombstone vocabulary; query-module source scan; superseded review absent as entry; lineage retains bytes + Withdrawn status — the V2 boundary as assertions) | 2/2 green |
| A.3 V3 | No test by design — zero tests remains the deliberate statement; PRIMITIVES + `ConsentMode` docs carry the decided sentence | — |
| C | T-A3.7 `lexicon_roundtrip_lossless` (per mapped kind, optional fields both ways; `$type` = draft lexicon id; deterministic; record-layer numeric walk date-only; commitmentEpoch content round-trip with unordered set intact) · T-A3.8 `fields_without_lexicon_home_documented` (three named surfaces with tiers; mechanical exactness over all 14 payload kinds: mapped iff in the B.2 six-lexicon scope; every unmapped kind named) | 2/2 green |

Prior 58 integration tests: untouched except the named rename (T-AT2.1
`vouch_cites_edge_antecedent` → `vouch_requires_qualifying_antecedent`) and the
mechanical consequences of the named marker rename (T-AT2.3's expected marker
value; the t_at1 marker-vocabulary pin).

## F-AT-6 (reproduced from FINDINGS.md)

> **F-AT-6 — Residual correlators, ATProto edition (RUN-ATTEST-03 Part B;
> FINDING, recorded not solved).** Sibling of F-AT-1 and F-PA-1. When personas
> realize as ATProto accounts, the identity layer itself introduces correlators
> the fixture-keypair model never had — all in infrastructure the design does
> not control: **PLC operation-log timing** (`did:plc` creation and rotation
> operations are publicly logged with server timestamps, permanently — sibling
> personas created the same day cluster in the log); **PDS hosting choice**
> (the PLC log carries the full history of PDS locations; siblings homed on the
> same small PDS share an infrastructure fingerprint); **enumerability** (the
> set of all identifiers is enumerable — the joins run at population scale
> without crawling attestation repos). The T-PA2.1 floor survives untouched;
> what this entry records is that the floor sits on an identity layer that
> leaks metadata sideways. Mitigations (named, not solved): distinct PDS hosts
> per sibling persona; staggered creation and rotation (the F-PA-3 practice
> family: any monotonic, published, infrastructure-side ordering is a
> correlator); issuer-side epoch-coarse commitment folds so mint traces cannot
> be joined against PLC timestamps more precisely than an epoch. Grade:
> Modeled by design.

## F-AT-4 allowlist extensions (reviewed, with notes)

The RED run's T-AT5.4 failure on the four new fold-module operations was the pin
doing its job. Reviewed and allowlisted with a note in the test:
`fold_with_register` (mirrors the R7-governed qualifying-kind register into a
fold; changes STANDING/qualification only — never removes, hides, or demotes a
stored object; lineage and views remain; moves only by content-bound quorum,
reachable by no subject's unilateral act) and `full`/`from_mask`/`allows` (pure
constructors/readers on the mirror type). FINDINGS F-AT-4 gained the one-line
pointer.

## Deviations from the instruction file (with reasons)

1. **The legacy single `grade` field survives beside the new `grades` set** on
   `VouchView`/`CorroborationEntry`. §6 requires the 58 prior tests untouched,
   and T-AT2.4 pins the legacy semantics (tx antecedent → `transaction_backed`,
   else base-edge grade); the kind-derived set is therefore additive
   (serialized as the `"j"` string list beside `"g"`), with the legacy field
   documented as edge-inherited presentation metadata.
2. **Register establishment is itself modeled as a quorum act**: the substrate
   genesis default for rule_key 2 is 1 — read as the pre-V1 edge-only posture —
   and T-A3.4's fixture raises it to 7 (the full class) under the content-bound
   gate before exercising narrowing. Widening later is the same shape.
3. **The qualifying transaction is binding, not merely cited**: payer must be
   the vouch author and payee the vouch subject (T-A3.2's near-miss cases).
   T-AT2.4's *grade* path (any cited transaction → `transaction_backed`
   metadata) is unchanged — grade is metadata; standing is the new rule.
4. **T-A3.8's list is broader than the three named surfaces** — mechanical
   exactness (every `None` kind named) forced honest rows for the stand-in
   facts (transaction/thing/vetting — no lexicon in the B.2 six-kind scope; the
   credential's `vetting` ref is a named residual), the record-op kinds
   (withdraw/dissolve/supersede — V2 operations, not record kinds), the
   envelope fields, and the detached epoch signature.
5. **`atproto_map` uses declared stand-in leaf encodings**
   (`did:croft-fixture:<hex>` personas; hex-cid strongRefs) — fixture keypairs
   are RUN-ATTEST-01 §3's declared stand-in for DIDs, and the spike is a shape
   mapping, not a wire format.
6. **No import-markers were needed in the brief**: network was available in the
   session, so every row carries a fetched primary-source anchor (the
   import-provenance ruling's fallback went unused).

## Definition of green (§6) — checklist

- Part A tests green red-first, T-A3.1's red captured against the pre-change
  fold: **yes** (digests above).
- OC-2/OC-3/OC-4 tags resolved per §2 and ONLY those: **yes** (PA-series +
  OC-1 confirmed untouched; grep audit above).
- PRIMITIVES updated (V1 antecedent class + new **antecedent kind** term, V2
  marker + absence rule, V3 deferral) with is/is-NOT pairs maintained: **yes**
  (doc-pin tests T-PA5.3/T-AT4.4 still green).
- ATTEST-ATPROTO-MATCHUP.md complete: ten rows anchored (none import-marked —
  all fetched), CID-circularity analysis, delete-visibility bounding, F-AT-6
  filed: **yes**.
- Lexicon drafts present: **yes** (six + README, JSON-validated).
- Part C green (not dropped): **yes**.
- Full suite green, prior tests untouched except the named rename: **yes**
  (67 + 11; the marker-rename consequences documented above).
- Crate clippy-clean: **yes** (substrate-dep warnings pre-existing, untouched).
- Site gate: **run and green** (86 pages; resolver suite OK; the new docs live
  next to the crate and in alpha/experiments/, outside the published spec set —
  run anyway per the §2 carry-over).
