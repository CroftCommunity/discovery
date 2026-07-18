# RUN-ATTEST-04 summary — Settlement riders (V4–V10): closing the walk

`Own-lane run, executed 2026-07-18 against the instruction file "RUN-ATTEST-04 —
Settlement Riders (V4–V10): Closing the Walk", layered on the merged
RUN-ATTEST-01/02/03 attest-family crate (gate met: RUN-ATTEST-03 merged, audited
2026-07-18 against snapshot 17discovery-main). Carries verdicts 4–10 of the
2026-07-18 owner-call walk into the crate and its docs. Nothing dropped from
§9's drop order — Parts A, B.1–B.4, C, and D all executed at full breadth.`

**The walk's ledger now reads TEN OF TEN: V1–V10 settled, and ZERO `pending`
OWNER-CALL tags remain anywhere in the attest lane** (grep audit below).

## The verdicts settled (owner decisions, confirmed in chat 2026-07-18; not relitigated)

- **V4** — graded resolvability default: counterparts of a standing co-signed
  edge resolve; strangers get cardinality only; silence IS the posture; OPEN is
  a deliberate, reversible policy supersede (workplace personas the motivating
  case). Reviews assert experience, not relationship — they grant no
  resolution; public-tier discoverability of published records is untouched.
- **V5 (PA OC-1)** — per-epoch signed tree heads over keyed commitments
  (CT/RFC-9162 shape) replace the per-credential receipt pile; confirmation =
  holder-stapled inclusion proofs (the verifier never contacts the co-op);
  revocation = per-epoch superseded set; value claim re-graded (consistency +
  holder self-verifiability + epoch-grain volume — never noncoercion or
  process truth).
- **V6 (PA OC-2)** — ledger posture inherited from V5; ceremony spacing is
  user guidance (inform, honor either choice); the era-anchoring move: issuer
  operational time is governance time — the co-op is literally a Drystone
  group and its governance lineage is the era spine.
- **Era-reissue** (owner refinement extending V5–V6) — holder-signed-only
  reissue under a new era; old-era credentials never expire; silence carries
  no penalty; membership era-graded, "meaningful but factual".
- **V7 (PA OC-3)** — `sole_anchor(context)` REJECTED, not deferred; uniqueness
  is group-local membership vetting (governance counts member handles;
  personas sign); portable proof-of-personhood refused ("eating the spider").
- **V8 (PA OC-4)** — fees have no protocol surface; fee attaches to vetting
  event + mint act; era-reissues free (structural pin T-A4.14); schedule in
  the co-op's governance lineage; generous bootstrap posture.
- **V9** — PRIMITIVES gains the generalized rule: *any monotonic, published,
  infrastructure-side ordering is a correlator; infrastructure publishes at
  era grain or not at all*, pointing at F-PA-3. FINDINGS untouched.
- **V10 (AT OC-1)** — PRIMITIVES stays in alpha beside the crate; the
  graduation trigger is a NAMED condition in its header (the attest lane's
  release pass: settlement landed + grades re-evaluated + lexicon drafts
  promoted/versioned). No thin beta summary is ever created.

## The removed machinery (§4.B.1 statement, verbatim requirement)

The per-credential public receipt pile (`EpochRecord` + its unordered
commitment-set fold) and the OCSP-shaped status check
(`StatusResponse`/`CredStanding`/`status_check`/`audit_lineage`) are REMOVED.
**Supersede-never-revoke governs Drystone-tier objects; test scaffolding and
issuer-internal structures are code, and code changes by commit — this removal
is not a lineage violation.** The superseded lexicon draft
(`ing.croft.attest.commitmentEpoch`) — which IS a published doc artifact —
stays visible, marked superseded by `treeHead`, never deleted.

## What landed

- **Part A (V4, red-first)** — `AttestState::resolvable`'s no-policy default is
  now the graded posture (standing-edge counterparts only; dissolved edges
  lapse). T-A4.1 (`fresh_persona_graded_by_default`, THE red — captured
  failing against the pre-change resolvable-to-all fold), T-A4.2
  (counterpart-and-only-counterpart, incl. dissolve lapse), T-A4.3 (OPEN as a
  supersede with lineage, reversible, workplace fixture end-to-end), T-A4.4
  (T-AT3.5's fuzz re-proven at the new stranger tier), T-A4.5 (remeasured
  sweeps, both postures). PRIMITIVES gains the definition + the CONTACT.md
  kinship sentence; the RUN-ATTEST-01 stand-in flag's V4 pointer lands in
  PRIMITIVES' stand-ins section (the frozen deviation note untouched).
- **Part B.1 (V5/V6)** — `src/issuer.rs` reworked: era-keyed commitments
  (`key_e = KDF(coop_secret, era_anchor_e)`, flagged in-code as the modeled,
  owner-revisitable choice), Merkle tree over canonical-order leaves, ONE
  signed `TreeHead` per epoch {root, leaf count, superseded-set root, era
  anchor}, cadence = epoch roll OR N facts (N mirrored from the reused R7
  machinery — T-A4.9 in `t_pa_substrate.rs`, with the contradiction hard-stop
  re-asserted, and the quorum-met governance fact's hash used as the era
  anchor: era-anchoring as behavior). T-A4.6 (staged-violation red: a receipt
  pile on the public surface — also broke the audit), T-A4.7 (permuted mint
  orders → byte-identical head; successor of T-PA1.4), T-A4.8 (dictionary
  resistance incl. the published superseded set; positive control via the
  holder binding).
- **Part B.2 (stapling)** — `verify_staple(credential, issuer, head, staple)`
  is pure; `HolderBinding` (issuer-signed credential↔commitment pairing,
  holder-held) makes the staple verifier-checkable without the era key;
  `holder_proof`/`holder_staple` are the holder channel (commitment-keyed —
  no subject parameter exists). T-A4.10 (superseded cannot staple fresh;
  freshness fail-closed restated), T-A4.11 (verifier never contacts issuer:
  machinery-absence scan + signature pins + drop-the-state behavioral proof),
  T-A4.12 (holder verifies own inclusion; tampered head/proof/swap fail),
  T-A4.13 (audit over heads: five tamper classes caught, totals-only report).
- **Part B.3 (era-reissue)** — `reissue()`: holder-signed request only
  (NotHolderSigned otherwise), chains the ORIGINAL vetting + the request as
  antecedents, supersedes the old credential, fresh commitment under the new
  era key, NO seam write and NO dial check. `Credential` gains the required
  `era` anchor; new `ReissueRequest` payload kind; `CredentialView` exposes
  era facts only (`era`, `holder_requested`). T-A4.14 (chains-vetting +
  free-at-the-dial-cap — one test, two verdicts), T-A4.15 (cross-era
  unlinkability: "who reissued" masked-identical across worlds; era byte
  populations and one holder's cross-era staples disjoint), T-A4.16 (the
  guard with teeth: field-set pin + crate-wide standing-vocabulary scan +
  serialization pin; red staged as `current_member: bool`, deleted at green).
- **Part B.4 (docs)** — PRIMITIVES: tree head / staple / era-reissue /
  commitment(V5) definitions with is/is-NOT pairs, the value-claim sentence,
  the co-op-as-Drystone-group sentence, the V6 user-guidance sentence, the
  silence sentence, the meaningful-but-factual guard. Matchup Row 7 revised
  to the stapling model, dated as a V5 revision (prior text in git history).
  Lexicons: `ing.croft.attest.treeHead` DRAFT added; `commitmentEpoch` marked
  superseded; `credential` gains required `era`. `atproto_map` round-trip
  extended: `head_to_record`/`head_from_record` (T-A3.7 family), with
  superseded set round-tripping non-trivially.
- **Part C (V7)** — PRIMITIVES `sole_anchor(context)` converted to a recorded
  REJECTION carrying the full rationale (rejected pole findable, the
  full-issuance-facts treatment); the quorum sentence lands beside the
  register vocabulary AND the substrate-test header; T-PA5.3's carve-out
  phrasing removed — the unaskable pin stands unqualified, its needle set now
  pinning the rejection text.
- **Part D (V8/V9/V10)** — the fee-posture paragraph, the graduated ordering
  rule (F-PA-3 pointer; FINDINGS untouched), the V10 header trigger, the owed
  cross-encoder sentence (landed in PRIMITIVES, cross-referenced from matchup
  row 2), the MASTER-INDEX row (walk complete, zero pending OCs).

**Status tags:** everything lands `Modeled`. T-A4.9 reuses the substrate's R7
content-bound quorum machinery unmodified at its existing grade (R7 count path
cross-package `Verified` per RUN-07; §7.6.1 hard-stop per RUN-03); the cadence
register modeling on top is `Modeled`.

## T-A4.5 — the remeasured sweeps, stated explicitly (§8 requirement)

Both re-run postures, 16 seeded cases each, T-AT4.3's shared-leaf property and
the fold-side T-PA2.1 leg asserted throughout: **every case is equal or
stronger — no weaker case exists, so no FINDING is filed.** Opted-open (all
participants take the V4 OPEN posture) reproduces the pre-V4 disclosure level
and the original property holds unchanged; zero-policy (the new default)
exposes strictly less — no resolved far ends and no attester-linked entries
reach a stranger, asserted directly (the counterpart id may not appear on the
stranger surface at all).

## Red → green evidence

Digests of saved outputs (scratchpad):

- **RED (Part A)** — the five T-A4.x tests written against the PRE-CHANGE
  fold: all five failed, T-A4.1 exactly as designed (the fresh persona
  resolved to a stranger under the old default — the captured red IS the
  stand-in being retired). sha256 `e8e7254c744c8691…`
  (`attest4-red-part-a.txt`). Part-A green checkpoint: sha256
  `23aa02c217e7a3e0…` (`attest4-green-part-a.txt`).
- **RED (Part B, natural)** — mid-rework run: the pre-existing T-PA3.1
  allowlist pin failed on the twelve new issuer/verifier operations pending
  review — the F-AT-4 flow working as designed (reviewed + allowlisted with
  notes, below). sha256 `a765965630f3885c…` (`attest4-red-part-b-first.txt`).
- **RED (Part B, staged violations)** — (a) T-A4.6: a per-credential receipt
  pile staged onto `lineage_bytes` — the leaf-population prong failed AND the
  head audit refused to decode (the publication shape is load-bearing);
  (b) T-A4.16: a `current_member: bool` staged onto `CredentialView` +
  serialized — the field-set, vocabulary, and key-set prongs all failed.
  Staging deleted at green. sha256 `9ac8c6a88c199528…`
  (`attest4-red-staged-violations.txt`).
- **Green at birth, stated honestly** — T-A4.9's quorum legs pin behavior the
  reused substrate enforces regardless of register (same class as
  RUN-ATTEST-02's dial tests); its issuer-side leg (Nth fact publishes the
  era'd head) is new behavior proven against the new module. T-A4.11's
  scan/signature prongs are absence pins over the reworked source — their red
  is the deleted status-check machinery itself (present pre-rework by
  construction).
- **GREEN (final)** — 91 tests passing across 22 targets (integration +
  compile_fail doc-tests), 0 failed; crate clippy-clean (remaining clippy
  output is pre-existing substrate-dep warnings, untouched). sha256
  `bedec3ad770f07be…` (`attest4-green-final.txt`).

## Test map

| Part | Tests | Result |
|---|---|---|
| A (V4) | T-A4.1 `fresh_persona_graded_by_default` (red captured) · T-A4.2 `counterpart_resolves` · T-A4.3 `open_is_a_posture_supersede` · T-A4.4 `cardinality_still_leakless` · T-A4.5 `sweeps_remeasured` | 5/5 green |
| B.1 (V5/V6) | T-A4.6 `tree_head_is_the_only_publication` (staged red) · T-A4.7 `leaves_canonical_by_commitment` · T-A4.8 `keyed_commitments_resist_dictionary` · T-A4.9 `head_cadence_is_governed` (substrate; era-anchoring as behavior; hard-stop re-asserted) | 4/4 green |
| B.2 (stapling) | T-A4.10 `superseded_cannot_staple_fresh` · T-A4.11 `verifier_never_contacts_issuer` · T-A4.12 `holder_verifies_own_inclusion` · T-A4.13 `audit_over_heads` | 4/4 green |
| B.3 (era-reissue) | T-A4.14 `reissue_chains_vetting_no_new_antecedent` · T-A4.15 `cross_era_commitments_unlinkable` · T-A4.16 `no_standing_computation` (staged red) · `no_wall_clock_in_issuer_pipeline` (the T-AT0.4 scan extended crate-wide) | 4/4 green |
| C (V7) | T-PA5.3 re-pinned unqualified (rejection needles) | green |
| D | doc/tag settlement — no new tests by design | — |

**Prior-suite churn, confined and named (§7):**

- *Resolvability-default-dependent tests, updated to graded* — each change is
  an explicit attester/reviewer OPEN posture (the V4 machinery itself),
  commented `V4 churn (RUN-ATTEST-04, named)` in place: `t_at5_review.rs`
  fixture + `freshness_is_presentation`; `t_pa5_dual.rs` `fix()`;
  `t_a3_riders.rs` `plumber_case_red`, `grade_derives_from_kind`,
  `withdrawn_is_absent_not_tombstoned` (review half).
- *Receipt-pile/status-check tests, replaced old → new*: T-PA1.4
  `commitment_fold_is_unordered_per_epoch` → T-A4.7; T-PA2.3
  `status_check_no_cross_leak` → T-A4.11; T-PA6.2 (commitment-audit leg) →
  T-A4.13; T-PA6.3 `status_check_protocol` → T-A4.10/T-A4.11/T-A4.12; T-PA6.4
  reworked in place to the staple flow; T-PA2.1's swept surface → published
  envelopes + holder bindings (floor = issuer key + era anchor); T-PA2.2's
  refusal `SaltReused` → `NonceReused` (the salts left with the pile);
  T-PA1.3 reworked to the head surface (leaf count, keyed commitments).
- *sole_anchor carve-out phrasing*: T-PA5.3 needles + `types.rs` doc.
- T-AT0.\* floor invariants re-green untouched, plus the new crate-wide
  wall-clock scan over the reworked issuer module.

## F-AT-4 allowlist extensions (reviewed, with notes — §8 requirement)

The natural RED's T-PA3.1 pin failure on the new public operations was the pin
doing its job. Reviewed and allowlisted with notes in `t_pa3_friction.rs`:

- `set_cadence` / `cadence` — mirror the R7-governed head cadence exactly as
  `set_dial` mirrors the anchor dial; standing input by quorum only.
- `era` / `genesis_era` / `roll_era` — era-fact plumbing; rolling can only
  OPEN a new era; nothing is removed, hidden, or demoted; old-era heads stay
  published.
- `holder_proof` / `holder_staple` — the HOLDER channel, keyed by the
  commitment only the holder and issuer know; no subject parameter, no
  verifier query shape (T-A4.11).
- `reissue` — holder-request-gated; touches neither seam nor dial (T-A4.14);
  standing is judged by the fold's antecedent rule, not by the issuer.
- `verify_tree_head` / `verify_inclusion` / `verify_staple` /
  `verifier_accepts` / `audit_heads` — pure functions over bytes; no state
  parameter exists, so no operation can suppress and no issuer contact exists
  to leak (verifier, subject).

Removed from the allowlist with their machinery: `status_check`,
`verify_status_response`, `audit_lineage`.

## Tag-audit grep (§8 requirement; saved as `attest4-tag-audit.txt`, sha256 `78884310aced44cf…`)

- `grep -rni "OC-[0-9][^)]* pending|STILL OPEN" src tests PRIMITIVES-ATTEST.md
  FINDINGS.md` → **(none)** — zero pending OWNER-CALL tags in the lane.
- The six named moves, and ONLY those, made: PA OC-1 → `DECIDED (V5, …)` at
  T-PA1.3; PA OC-2 → `DECIDED (V6, …)` at T-A4.7 (moved with T-PA1.4's
  successor); PA OC-3 → `DECIDED (V7, …)` at T-PA5.3; PA OC-4 →
  `DECIDED (V8, …)` at T-PA3.2; AT OC-1 → `DECIDED (V10, …)` in PRIMITIVES'
  header + owner-calls list; the resolvable-to-all stand-in flag's V4 pointer
  in PRIMITIVES' stand-ins section (the frozen RUN-ATTEST-01 summary is
  untouched). The RUN-ATTEST-03 V1–V3 tags are byte-untouched.

## Deviations from the instruction file (with reasons)

1. **`HolderBinding` is a modeled design element beyond the instruction's
   letter**: with commitments keyed (`HMAC(key_e, credential_id)`, T-A4.8), a
   verifier cannot recompute the credential↔leaf pairing, so the staple
   carries an issuer-signed binding {credential hash ↔ commitment}. This is
   what makes `verify(head, proof, credential)` a pure total function while
   preserving dictionary resistance; the binding is holder-held, never
   published.
2. **Reviews are gated by the same `resolvable()` as vouches** (RUN-ATTEST-01
   machinery, unchanged): V4's "reviews assert experience... public-tier
   discoverability untouched" is carried as (a) a review grants no default
   resolution (only standing edges do) and (b) the default is a Drystone-tier
   traversal dial, never a publication gate — record copies on the public
   tier stay discoverable. Review fixtures whose viewers are strangers now
   opt OPEN explicitly (named churn).
3. **Cross-era supersession stays out of superseded sets**: a closed era's
   tree is frozen, so a reissued old-era credential's supersession is visible
   in its own public supersede lineage (and in fresh-head absence), not in
   any set. Same-era supersessions enter the live set (T-A4.10). This is the
   freshness model working as designed: old heads are era-stamped truth.
4. **`Credential` gains a required `era` field** (canonical encoding + lexicon
   + map change): T-A4.16 requires the FOLD to expose issued-under as an era
   fact, and the fold sees only envelopes — so the era anchor must be signed
   payload content. Consequence: the T-PA2.1 population floor is now
   {issuer key, era anchor} — the anchor is a deliberate population-wide era
   fact, exactly like the issuer key.
5. **T-PA2.1's surface excludes Merkle proof paths**: interior proof nodes
   are shared by value-adjacent leaves of ANY holders (adjacency is
   keyed-commitment order, holder-independent), so they'd fail a
   pairwise-vs-floor assertion without correlating holders. Documented in the
   test header; bindings (per-credential unique) are swept instead, and
   T-A4.15 separately pins staple disjointness across eras.
6. **The cadence register reuses rule_key 0 in a distinct issuer-operations
   group** (declared stand-in, PRIMITIVES §3): all four substrate rule keys
   already carry declared reinterpretations in the anchor-dial group, and the
   era-spine framing (the co-op's own governance group) makes the separate
   GroupId the honest model, not a dodge.
7. **`MintRefusal::SaltReused` → `NonceReused`**: commitment salts no longer
   exist (keyed commitments need none); the single-use-entropy discipline
   T-PA2.2 pins now lands on the mint nonces, same refusal semantics.
8. **T-A4.9's hard-stop leg is a re-assertion, not only a citation**: the
   instruction says "hard-stop inherited"; the competing-cadence pair is run
   for real so inheritance is behavior.

## Definition of green (§7) — checklist

- Parts A–D tests green red-first; T-A4.1's and T-A4.6's reds captured
  (T-A4.1 against the pre-change default, T-A4.6 staged); staged reds
  deleted at green: **yes** (digests above).
- The six named tag moves made and no others; grep audit in this summary:
  **yes**.
- Prior suite green with churn confined to the three named classes, each
  change named: **yes** (table above).
- T-AT0.\* floor invariants re-green including the wall-clock scan over the
  new issuer module: **yes** (`no_wall_clock_in_issuer_pipeline`, crate-wide).
- PRIMITIVES carries every §3–§6 addition with is/is-NOT pairs and the V10
  header: **yes** (doc-pin tests T-PA5.3/T-AT4.4/T-PA2.4 green).
- Matchup Row 7 revised and dated: **yes** (V5 revision, 2026-07-18).
- Lexicon treeHead draft present with round-trip: **yes** (JSON-validated;
  `head_to_record`/`head_from_record` in T-A3.7).
- Pure workspace clean; clippy clean on touched code: **yes** (remaining
  warnings are pre-existing substrate deps, untouched).
- Site gate: **run and green** (86 pages; resolver suite 29/29 OK — the new
  material lives beside the crate and in alpha/experiments/, outside the
  published spec set; run anyway per the carry-over).
