# RUN-ATTEST-02 summary — Anchor personas: no default, hard splits, reality anchors

`Own-lane run, executed 2026-07-18 against the instruction file "RUN-ATTEST-02 —
Anchor Personas: No Default, Hard Splits, Reality Anchors", layered on the merged
RUN-ATTEST-01 attest-family crate (gate met in full: Parts 0–6 including the Part 6
issuer machinery were merged, so no pre-part was needed). All experiments EXP-PA1–6
executed; nothing dropped from §7's drop order.`

## The claim under test

**Accountability attaches per-persona; unity of the human stays private.** A member
holds several credentialed anchor personas; no public object ranks or distinguishes
a "primary" one; each carries a reality anchor (`vetted_holder`) while the graphs
stay hard-split. Privacy and meaningful provenance simultaneously, not traded off.

## What landed

- **`attest-family/` extended, not forked** — three new payload kinds
  (`vetting_fact`, `credential`, `credential_supersede`) + `vetted_holder` in the
  closed predicate vocabulary; a credential fold pass (standing REQUIRES a
  same-issuer vetting antecedent naming the same subject); `src/issuer.rs` (mint
  ceremony harness, seam-typed retained state, blinded per-epoch commitment
  lineage, OCSP-shaped status protocol, registry-free verifier, lineage audit);
  `src/anonymity.rs` (pool measurement + presentation composition). The T-AT4.3
  sweep and leaf-walkers are reused and extended in `tests/common/mod.rs`
  (persona-surface sweep + structural masking); nothing shadowed.
- **Fixtures before features (§3)** — `AnchorWorld` (H1..H5; P1a/P1b/P1c on H1,
  P2a/P2b on H2, one each on H3–H5), generated `coop_s()` (12 members, 15
  personas) and `coop_l()` (400 members, 441 personas) on the same code paths,
  `member_ref` derivation keeping holder names out of issuer state, single-use
  `MintEntropy`.
- **23 new integration tests + 1 compile_fail doc-test** across
  `t_pa1_no_default`, `t_pa2_unlinkability`, `t_pa3_friction`, `t_pa4_anonymity`,
  `t_pa5_dual`, `t_pa6_issuer`, `t_pa_substrate` — full suite now 58 integration
  + 7 doc-tests, green; crate clippy-clean.
- **Deliverable docs** — `FINDINGS-ANONYMITY-SETS.md` (measured tables + plain-
  language guidance, drift-guarded by `measure_anonymity_sets`); `FINDINGS.md`
  F-PA-1..3; `PRIMITIVES-ATTEST.md` anchor-persona vocabulary (anchor persona,
  reality anchor, credential, `vetted_holder` vs `sole_anchor(context)`,
  commitment, status check, seam boundary — each with an is/is-NOT pair).

**Status tags:** everything lands `Modeled` (fixture keypairs, in-memory fold, no
payment rail, no real vetting). T-PA3.2 and the T-PA6.2 covenant-lineage leg reuse
the substrate's R7 content-bound quorum machinery UNMODIFIED and cite the reused
portion at its existing grade (R7 count path cross-package `Verified` per RUN-07;
§7.6.1 hard-stop per RUN-03); the dial/covenant *modeling* on top is `Modeled`.
EXP-PA4 is measurement (Modeled fixtures), not pass/fail — but its harness is
red-first-proven (T-PA4.1).

## Red → green evidence (per §2, one full-suite RED batch before implementation)

- **RED** (2026-07-18, `--no-fail-fast`): all 21 then-written T-PA tests FAILED
  against `unimplemented!()` issuer/anonymity/fold stubs and staged violations;
  the pre-existing T-AT5.4 allowlist pin ALSO failed, by design, on the two new
  fold accessors pending review. Saved output sha256 `30a447d77c315c77…`
  (`attest2-red-run-full.txt`).
- **Staged-violation reds for negative invariants** (refutation-pin style):
  - T-PA1.1: a staged `pub minted_ordinal: u32` field on `Credential` made the
    all-files source scan fail; deleted at green.
  - T-PA3.2: the dial register staged UNGOVERNED (no RuleChange-threshold raise)
    — a lone owner could move the dial and the under-quorum assertion failed;
    sha256 `ee94870fbce15efd…` (`attest2-red-substrate-staged.txt`).
  - T-PA6.2 (lineage leg): the covenant establishment staged UNDER-QUORUM
    (approval antecedent omitted) — the reused machinery refused it whole, so no
    intact covenant lineage could exist; sha256 `723b80f895a69482…`
    (`attest2-red-substrate-staged-b.txt`).
  - `dial_inherits_contradiction_hard_stop` is **green at birth, stated
    honestly**: it pins behavior the reused substrate enforces regardless of
    threshold (concurrent same-register RuleChanges hard-stop at any quorum), so
    no staging short of breaking the reused machinery itself can red it — which
    is precisely what "inherited unchanged" means.
- **GREEN** (final): 58 integration + 7 doc-tests, 0 failed, crate clippy-clean;
  sha256 `fb4821679e51aa66…` (`attest2-green-run-full.txt`).

## Test map (per experiment)

| Experiment | Tests | Result |
|---|---|---|
| EXP-PA1 no-default | T-PA1.1 `no_rank_representable` (all-src-files scan, T-AT0.2 banned set + ordinal/primary/rank/sequence; serialization numeric walk) · T-PA1.2 `sibling_indistinguishability` (masked structural identity + 4-selector battery over 16 seeded 13-candidate worlds, canonical presentation order) · T-PA1.3 `issuer_lineage_carries_commitments_not_identities` (no persona/credential ids raw or derived; registry-free verification; OC-1 tag) · T-PA1.4 `commitment_fold_is_unordered_per_epoch` (byte-identical lineage across mint orders; canonical sort; OC-2 tag) | 4/4 green |
| EXP-PA2 unlinkability | T-PA2.1 `sibling_credentials_unlinkable` (12 seeded cases; sibling pairwise surface intersection ⊆ population floor = issuer key only) · T-PA2.2 `independent_derivation_enforced` (compile_fail on `MintEntropy` move + deterministic `SaltReused` refusal, state unchanged) · T-PA2.3 `status_check_no_cross_leak` (exact field set {g,h,s}; zero numerics; disjoint from lineage) · T-PA2.4 FINDINGS doc-pin (F-PA-1) | 4/4 green |
| EXP-PA3 sybil friction | T-PA3.1 `no_credential_without_vetting_antecedent` (mint path, bare, wrong-subject, wrong-author all pending; new-module public-op allowlist pin) · T-PA3.2 `anchor_count_is_a_governed_dial` + `dial_inherits_contradiction_hard_stop` (in `t_pa_substrate.rs`; reused R7; dial mirrored into `IssuerState` and enforced at the seam; OC-4 tag) · T-PA3.3 `member_anchor_count_not_public` (two-world masked-structural equality [3,2,1,1,1] vs [2,2,2,1,1]; totals-only lineage numerics) | 5/5 green |
| EXP-PA4 anonymity sets | T-PA4.1 `harness_correct_on_known_fixture` (hand-computed 6-persona pools + bundle members, red-first) · `measure_anonymity_sets` (M-PA4.2/3, tables below, doc drift-guard) · T-PA4.4 `presentation_is_subset_capable` (subset presents alone; zero trace/count of unpresented credentials) | 3/3 green |
| EXP-PA5 dual attachment | T-PA5.1 `record_stays_with_persona` (superseded vouch, review + signed reply dispute, superseded credential — all carried, bytes unchanged) · T-PA5.2 `siblings_unaffected` (P1b/P1c full sweeps byte-identical before/after; P1a's changed) · T-PA5.3 `anchor_is_not_uniqueness` (vocabulary pin; Holder/MemberRef/SeamBoundary token-absent from public-object modules; seam exposes no public fn; same-holder vs split-holder worlds masked-identical; OC-3 tag) | 3/3 green |
| EXP-PA6 issuer | T-PA6.1 `issuer_state_is_assertions_plus_process_only` (region scan: no String/PersonaId/free bytes; seam named exactly once; runtime export sweep) · T-PA6.2 `covenant_audit_without_unmasking` (honest passes with totals-only report; dropped-commitment→TotalMismatch, 31-byte→MalformedCommitment, unsigned tamper→BadSignature, gap→NonContiguousEpochs) + `covenant_rule_lineage_intact_without_unmasking` (substrate leg: chained establishment + approvals exported; no subject persona bytes) · T-PA6.3 `status_check_protocol` (deterministic signed answers, unknown handled, forged standing rejected, fail-closed-is-app-policy) · T-PA6.4 `supersede_reaches_verifier_without_registry` (end-to-end; old bytes intact; siblings untouched) | 5/5 green |

## The T-PA1.2 adversary-class statement (reproduced verbatim from the test)

> ADVERSARY CLASS (stated honestly, reproduced verbatim in the run summary):
> this test models a PUBLIC-DATA-ONLY adversary. It sees every byte a
> third-party viewer can sweep — the candidates' published credentials and
> vetting facts and the issuer's epoch lineage — but has no issuer insider
> state (salts, seam ledger, mint order), no network/traffic metadata, and no
> behavioral/stylometric features. Within that class the test proves (a)
> structural indistinguishability: after masking identifier values, every
> candidate's published bundle is byte-identical; and (b) a battery of
> concrete deterministic selectors over the raw public bytes picks H1's
> first-minted persona no better than chance across seeded worlds. It does
> not — and cannot — quantify over every deterministic function; the
> structural half is what makes the battery representative.

## Measured tables (M-PA4.2 / M-PA4.3 — measurement, Modeled fixtures)

Anonymity-set size per (issuer, predicate):

| predicate | COOP-S (15 personas) | COOP-L (441 personas) |
|---|---|---|
| `vetted_holder` | 15 | 441 |
| `over_18` | 14 | 419 |
| `phone_verified` | 10 | 265 |
| `payment_verified` | 7 | 155 |

Shrink from presentation-side bundle composition:

| bundle shown together | COOP-S | COOP-L |
|---|---|---|
| `over_18` alone | 14 | 419 |
| + `phone_verified` | 9 | 245 |
| + `payment_verified` | 5 | 71 |

Guidance (full text in `FINDINGS-ANONYMITY-SETS.md`): coarse predicates by
default; small co-ops should tell members their cover is thin (the full bundle in
COOP-S leaves a pool of five); federation across issuers is the eventual widener,
noted as direction only.

## OWNER-CALL tags emitted (decisions NOT made; RUN-ATTEST-02 §8 numbering)

- **OC-1** — issuer public-lineage content; blinded commitments implemented
  (narrowest), tagged at T-PA1.3.
- **OC-2** — sibling-batching mitigation; unordered per-epoch folds implemented,
  tagged at T-PA1.4; epoch-membership residue recorded (F-PA-1).
- **OC-3** — whether `sole_anchor(context)` ever ships; vocabulary only, tagged
  at T-PA5.3, NOT built.
- **OC-4** — fee semantics (flat vs vetting-tier); pure policy over the T-PA3.2
  dial, tagged there.

## FIX vs FINDING classifications

- **FINDING (F-PA-1)** — residual sibling correlators outside the model: shared
  counterpart personas (cites RUN-ATTEST-01 F-AT-1), behavioral/stylometric
  linkage, network-layer metadata, and the new epoch-membership quantization.
  Out of protocol scope; mitigation locations named (client hygiene, transport,
  epoch sizing/OC-2). Modeled by design.
- **FINDING (F-PA-2)** — publication-unlinkability ≠ presentation-unlinkability:
  the same credential shown twice is trivially linkable; BBS-style unlinkable
  presentations are the deferred §9 layer. Stated so the two claims are never
  conflated.
- **FINDING (F-PA-3)** — mint lamports must be epoch-coarse: a per-mint counter
  would republish mint order through the persona-published envelope, resurrecting
  the batching correlator T-PA1.4 kills. Design practice carried forward.
- **FIX (in-run)** — PRIMITIVES phrase wrapping broke T-PA5.3's exact-phrase
  check (`one human may hold several` split across a line break); rewrapped. No
  code behavior involved — the same failure class as RUN-ATTEST-01's T-AT4.4 fix.
- **Reviewed allowlist extension (F-AT-4 flow, working as designed)** — the RED
  run's T-AT5.4 failure on the new `credentials`/`credential` fold accessors was
  the pin doing its job; both were reviewed (read-only view accessors, no
  suppression capability) and added to the allowlist with a review note.

## Deviations from the instruction file (with reasons)

1. **T-PA1.2's "no deterministic function" is tested as structural masking + a
   concrete selector battery**, not a quantification over all functions (which no
   test can do); the adversary-class statement above bounds the claim honestly.
2. **T-PA3.2 and the T-PA6.2 covenant-lineage leg live in one file**
   (`t_pa_substrate.rs`) because both need the substrate dev-deps; the
   commitment-audit leg of T-PA6.2 stays in `t_pa6_issuer.rs`. Split documented
   in both files.
3. **The status check answers a third state, `unknown`,** for hashes outside the
   issuer's assertion lineage (OCSP shape) — deterministic and signed, never a
   fabricated current/superseded verdict.
4. **Credentials coexist with RUN-ATTEST-01's `Predicate` kind** rather than
   replacing it: predicate semantics (no vetting antecedent) are frozen record;
   the credential is the stricter anchor-persona unit layered beside it.
5. **`dial_inherits_contradiction_hard_stop` green at birth** — see the red
   evidence section; recorded rather than staged into a fake red.

## Definition of green (§5) — checklist

- All T-* green red-first: **yes** (test map + red digests above).
- M-* produced with T-PA4.1 proving the harness: **yes** (harness red-first on
  hand-computed fixture; measured values reproduced the hand-derived
  congruence counts exactly).
- `FINDINGS-ANONYMITY-SETS.md` + T-PA2.4 FINDINGS entry present: **yes**
  (drift-guarded by tests).
- `PRIMITIVES-ATTEST.md` updated (anchor persona, reality anchor,
  `vetted_holder` vs `sole_anchor(context)`, commitment, status check — each
  with an is/is-not pair): **yes** (plus credential and seam boundary).
- Pure workspace clean: **yes** (no new dependencies; `cargo test` 58 + 7 green;
  new code clippy-clean; pre-existing substrate warnings untouched).
- Site gate: **run and green** (docs live next to the crate, outside the
  published spec set; run anyway per the §2 carry-over).
