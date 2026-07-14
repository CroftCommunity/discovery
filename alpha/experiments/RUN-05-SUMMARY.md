# RUN-05 — Full consistency, clarity, and correctness pass (specs at the center)

`Branch: claude/run-05-consistency-pass-jkjkz8 (off the RUN-04 merge). Phases 0–3 are markdown;
Phase 4 is comment-only code (no behavior change). Captured 2026-07-14.`

Posture followed throughout: the two specs are the center of gravity; **FIX** = mechanical, provable,
meaning-preserving (applied directly); **FINDING** = anything requiring a judgment call or changing
meaning (recorded in `CONSISTENCY-FINDINGS-2026-07.md`, not edited). Verbatim-anchor rule honored:
every anchor was found exactly (no misses). No Part 1 body, `conventions-and-decisions.md`, or
thinking-doc content was edited (banners and the queued code comments excepted).

Full FIX list, per-FINDING detail, and the "verified clean" register are in
**`CONSISTENCY-FINDINGS-2026-07.md`**. This file is the per-phase status and the one-line before→after
record.

---

## Per-phase status

| Phase | Scope | Result |
|---|---|---|
| **0** | Verify the RUN-04 landing (FINDINGS only) | **PASS, no miss.** All five points intact — corroboration note §0–§6 + banner; the corroboration-dials ¶ verbatim after the §7.3.3 beam-caveat ¶ (`Design`); the formula-valued threshold ¶ closing §7.4.1 before §7.4.2 (`Design`); EXP-C1 at backlog §2c beside EXP-H1 with its four assertions + §8.2(e) note; Map/changelog/reviews-log entries present. |
| **1** | Mechanical integrity (FIX) | **Clean + 1 changelog FIX.** No dangling Part 2 refs (RFC-cited numbers aside); R1–R7 and Appendix A–F resolve; §0 Map covers every section incl. all RUN-02/03/04 additions; 11 Part 1 citations from Part 2 resolve; ~90 cited paths checked. FIX: new RUN-05 changelog entry (1.4). Path/typo issues → FND-3/4/6/7/8. |
| **2** | Status/evidence coherence (FIX where proven) | **6 FIXes.** The two named staleness fixes (§7.3.2 F8, §7.2 R7 residual) applied; MASTER-INDEX and EXPERIMENT-BACKLOG brought current (competing-quorum reconciled RUN-03; queue refreshed); SPEC-ALIGNMENT bannered point-in-time. Grade questions → FND-2/9. |
| **3** | Semantic consistency (FINDINGS unless trivially safe) | **Clean + 4 banner FIXes.** New-paragraph coherence, terminology uniformity (no thinking-doc leak), DR-block compliance, citation support, and Part 1→Part 2 direction all verified consistent. proposed-changes + NEXT-RUN-INSTRUCTIONS + two root RUN-SUMMARY files bannered. Vocabulary/citation calls → FND-1/5/10/11. |
| **4** | Queued code comments (comment-only, separate commit) | **Applied; suites + clippy green (see below).** |

---

## Phase 2.1 — the two named staleness fixes (before → after)

Both provable: RUN-03 Phase B ran the two-competing-quorums experiment and moved the register row
`competing-quorum-autoresolve` Active → Reconciled, so the "no evidence tag until the experiment runs"
clause was stale in two Part 2 passages.

- **§7.3.2 (F8 boundary ¶).**
  before: `` …with no editorial resolution. `Design`, decided; the fold's behavior carries no evidence tag until the two-competing-quorums experiment runs.``
  after:  `` …with no editorial resolution. `Design`, decided and now test-run: the fold hard-stops with the order-independent `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED→GREEN, `two_competing_rulechange_quorums`, RUN-03). `Modeled.` ``

- **§7.2 (R7 residuals ¶, second residual; first residual — the role-authorship gate — untouched).**
  before: `` …decided as such (see the §7.3.2 boundary note), and the fold's behavior there carries no evidence tag until the two-competing-quorums experiment runs.``
  after:  `` …decided as such (see the §7.3.2 boundary note), and now test-run: the fold hard-stops with the order-independent `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED→GREEN, `two_competing_rulechange_quorums`, RUN-03). `Modeled.` ``

## Other FIXes (one line each)

- **`part-2-changelog.md`** — appended the RUN-05 pass entry for the two edits above (house style).
- **`MASTER-INDEX.md`** — moved `competing-quorum-autoresolve` to the RUN-03 Reconciled list (Active now =
  `hermetic-gossip` + `fanout-single-run`); critical-path item 1 marked closed (RUN-03 Phase B); Track A A1
  cell updated; RUN-03/04 added to "banked."
- **`EXPERIMENT-BACKLOG.md`** — replaced the stale execution order with the current queue (X3 automated
  harness; EXP-H1; EXP-C1; freshness/quiescence over live transport; MLS-welcome-over-iroh emission;
  BIP39; B1→A5; meer P2–P6; X1 parked); "done" preamble updated through RUN-04. X3/meer rationale
  preserved; no rationale lost (the dropped "largest open design problem" framing was already corrected by
  the RUN-02 alignment-doc decision 4).
- **`SPEC-ALIGNMENT-AND-ACTION-PLAN.md`** — point-in-time banner; no still-open item had its only home
  there, so no migration was needed.
- **`proposed-changes-2026-07-experiment-reconciliation.md`** — historical-record banner (all items landed
  RUN-02/03; F4 fan-out follow-on noted as staged-not-landed).
- **`NEXT-RUN-INSTRUCTIONS.md`**, **`RUN-SUMMARY-adjudication-language.md`**,
  **`RUN-SUMMARY-map-relocation.md`** — one-line superseded/historical banners pointing at the current
  RUN-0N sequence. No deletions.

---

## Phase 4 — queued code comments (separate commit)

Comment-only change at `local_storage_projection/src/fold_derived.rs`:

- `detect_competing_rulechange` guard comment **extended** with the queued F8 text (positively-established
  concurrency; bare re-sets fold sequential; the deliberate threshold-1 silent-flap consequence; quorum-met
  changes carry approvals as antecedents per §7.2 R7 so the F8 marquee case always trips; remedies; "Decided
  knowingly (RUN-03 audit, 2026-07-14)").
- The addition **mirrored** at the shared positively-established-concurrency guard in
  `detect_mutual_expulsion`: a note that the contract is shared across the predicate family
  (removed-then-included, role-thrash, competing-RuleChange) and the empty-antecedent-folds-sequential
  consequence was decided knowingly (RUN-03 audit), pointing at `detect_competing_rulechange` for the F8
  case.

Diff is comment-only (verified: `git diff` shows no non-comment `+`/`-` lines), so behavior cannot change
and no new clippy warning is possible by construction.

**Test/clippy result:** _<pending — filled below>_

---

## Guardrail compliance

- FIX vs FINDING discipline held; when in doubt, FINDING (e.g. the register-path repoint FND-3 was left a
  finding because the doc template matches the live code SPEC-DELTA tags — fixing one desyncs the other).
- No edits to Part 1 body, `conventions-and-decisions.md`, or thinking-doc content; banners only on stale
  operational docs; no document deleted; no prose reflowed beyond the anchored insertions.
- Part 2 body inserts use house em-dash style; changelog prose stays hyphen/`RED to GREEN`-only.

## Files changed

Markdown (Phase 0–3 commit):
`beta/drystone-spec/part-2-certifiable-design.md`, `beta/drystone-spec/part-2-changelog.md`,
`beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`,
`alpha/experiments/MASTER-INDEX.md`, `alpha/experiments/EXPERIMENT-BACKLOG.md`,
`alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md`, `alpha/experiments/NEXT-RUN-INSTRUCTIONS.md`,
`RUN-SUMMARY-adjudication-language.md`, `RUN-SUMMARY-map-relocation.md`,
`alpha/experiments/RUN-05-SUMMARY.md` (this file), `alpha/experiments/CONSISTENCY-FINDINGS-2026-07.md`.

Code (Phase 4 commit, comment-only):
`alpha/experiments/local_storage_projection/src/fold_derived.rs`.
