# Consistency findings — RUN-05 full-pass (2026-07-14)

`Companion to RUN-05-SUMMARY.md. Three sections: what was fixed mechanically, what needs an owner
call, and what was checked and found clean (so silence is never ambiguous). The specs are the center
of gravity; FIX = mechanical, provable, meaning-preserving; FINDING = anything requiring a judgment
call or changing meaning.`

---

## Fixed mechanically (the full FIX list)

Each is provable against a RUN summary, a register, or the actual tree, and meaning-preserving.

| # | File · location | Before → after | Basis |
|---|---|---|---|
| M1 | `part-2-certifiable-design.md` §7.3.2 (F8 boundary ¶) | `` `Design`, decided; the fold's behavior carries no evidence tag until the two-competing-quorums experiment runs.`` → `` `Design`, decided and now test-run: the fold hard-stops with the order-independent `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED→GREEN, `two_competing_rulechange_quorums`, RUN-03). `Modeled.` `` | Phase 2.1a. RUN-03 Phase B ran the experiment; register row `competing-quorum-autoresolve` moved Active → Reconciled. |
| M2 | `part-2-certifiable-design.md` §7.2 (R7 residuals ¶, second residual) | `` …decided as such (see the §7.3.2 boundary note), and the fold's behavior there carries no evidence tag until the two-competing-quorums experiment runs.`` → `` …decided as such (see the §7.3.2 boundary note), and now test-run: the fold hard-stops with the order-independent `contradiction:{byte-head}` and the rule retains its pre-conflict value (RED→GREEN, `two_competing_rulechange_quorums`, RUN-03). `Modeled.` `` (first residual, the role-authorship gate, untouched) | Phase 2.1b. Same evidence as M1. |
| M3 | `part-2-changelog.md` (new pass entry) | Appended `## Two-competing-quorums fold behavior recorded as test-run (§7.3.2, §7.2 R7 residual)` (RUN-05), house style, hyphens only | Phase 1.4 — changelog needs an entry for every Part 2 edit. |
| M4 | `MASTER-INDEX.md` (reconciled/active summary, ~L115) | Moved `competing-quorum-autoresolve` out of Active into the RUN-03 Reconciled list; Active now = `hermetic-gossip` + `fanout-single-run` | Phase 2.5a. RUN-03 reconciled it; the doc still listed it active. |
| M5 | `MASTER-INDEX.md` (critical path item 1 + banked line) | Item 1 "build the competing-RuleChange predicate" → "✅ Closed (RUN-03 Phase B)"; banked line extended to RUN-03/04 | Phase 2.5a. |
| M6 | `MASTER-INDEX.md` (Track A A1 cell) | `competing-quorum … ⚠️ impl FALSIFIED … now a scoped build` → `competing-quorum ✅ design DECIDED (F8) + impl BUILT & test-run (RUN-03 Phase B; register Reconciled)` | Phase 2.5a. |
| M7 | `EXPERIMENT-BACKLOG.md` (recommended execution order) | Replaced the stale queue (led with already-done A4/M1 fan-out, automerge 0.7, "remaining fold open items") with the current queue: X3 automated harness; EXP-H1; EXP-C1; freshness/quiescence over live transport; MLS-welcome-over-iroh emission; BIP39; B1→A5; meer P2–P6; X1 parked. "Done" preamble updated through RUN-04. | Phase 2.5b. EXP-1/EXP-4 already marked done in-table. X3/meer rationale preserved. |
| M8 | `SPEC-ALIGNMENT-AND-ACTION-PLAN.md` (top banner) | Added the point-in-time banner: "decisions recorded in §7 remain the record; current roadmap: EXPERIMENT-BACKLOG.md; current evidence: SPEC-DIVERGENCE-REGISTER.md. Not maintained forward." | Phase 2.5c. No still-open item had its only home here (F1–F8 landed; §7 decisions recorded; §5 mirrors the registers), so no migration was needed. |
| M9 | `proposed-changes-2026-07-experiment-reconciliation.md` (top banner) | Added: "Historical record — all items landed (RUN-02, RUN-03). The authoritative text is Part 2." with the F4 fan-out follow-on noted as staged-not-landed | Phase 3.5. |
| M10 | `NEXT-RUN-INSTRUCTIONS.md` (banner) | Added a one-line superseded banner pointing at the current RUN-0N sequence / MASTER-INDEX / EXPERIMENT-BACKLOG | Phase 3.6. It is the RUN-01 brief; RUN-01..04 have run. |
| M11 | `RUN-SUMMARY-adjudication-language.md` (banner) | Added a one-line historical-record banner (landed; points at the current sequence) | Phase 3.6. |
| M12 | `RUN-SUMMARY-map-relocation.md` (banner) | Added a one-line historical-record banner (landed; points at the current sequence) | Phase 3.6. |
| M13 | `local_storage_projection/src/fold_derived.rs` (comment-only, separate commit) | Extended the `detect_competing_rulechange` guard comment with the queued F8 text; mirrored a shared-contract + RUN-03-audit note at the `detect_mutual_expulsion` guard | Phase 4. No behavior change (diff is comment-only); suites + clippy green (see RUN-05-SUMMARY). |

---

## Needs an owner call (FINDINGS)

Severity HIGH/MED/LOW. Quoted text and a proposed resolution for each. Nothing here was edited.

### HIGH
None. (Phase 0: the RUN-04 landing is intact — see Verified clean.)

### MED

**FND-1 — `§7.4.2` citation for the origination freshness precondition (Phase 1.1/3.4).**
`part-2` §8.2(e) reads: "the freshness precondition on originating such an op **(§7.4.2)** is not yet
exercised over live transport." But §7.4.2 is titled *"Two MLS recovery hazards the corroboration
model dissolves"*; the precondition that a participant "**MUST** be (a) caught up and (b)
corroborated-fresh" to originate an add/remove/policy-change lives in §7.4 proper (the *"Membership
and governance acts require strict current plus corroboration"* paragraph), not §7.4.2. The same
`§7.4.2` citation for this concept is used consistently in `EXPERIMENT-BACKLOG.md` (EXP-C1), the
reviews-log, `proposed-changes` F1, and the RUN-05 instruction itself. *Proposed:* confirm the
intended target — either repoint the citation to §7.4 (the precondition's actual home) across the
doc set, or confirm §7.4.2 is the accepted shorthand. Left unedited because the citation is
doc-wide-consistent and correcting one site would desync it from the rest.

**FND-2 — §11.11 measurement #1 understates the fan-out (Phase 2.2 / proposed-changes F4).**
`part-2` §11.11 measurement #1 still reads: *"The **fan-out** half is still unearned but is now
runnable with no new infrastructure … The measurement moves from unearned to **half-earned**."* But
RUN-01 EXP-1 **measured** the fan-out over real iroh-gossip (loopback, `croft-chat/FANOUT-M1.md`:
per-node linear `2N+1`, O(N²) aggregate, heads converge). `proposed-changes` F4 stages the follow-on
edit (*half-earned → earned-in-shape (loopback), magnitude-open at hot-N = 500+*) but marks it "**Not
yet applied to Part 2** — this is the next §11.11 touch, pending review." *Proposed:* land F4's wording
(the loopback measurement + `fanout-single-run` magnitude caveat), or confirm the grade. A status move
on reviewed spec text → owner call, not a mechanical fix.

**FND-3 — stale `alpha/SPEC-DIVERGENCE-REGISTER.md` path, doc-and-code (Phase 1.3).**
The register does **not** live at `alpha/SPEC-DIVERGENCE-REGISTER.md`; it is at
`alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`. The short (broken) path appears in the register's own
SPEC-DELTA tag template (`SPEC-DIVERGENCE-REGISTER.md` L19), **two live code tags**
(`croft-chat/…/src/iroh_bus.rs:360`, `…/tests/iroh_convergence.rs:66`), `REPO-README.md`, and two
`croft-chat/plans/*` docs — while `NEXT-RUN-INSTRUCTIONS.md` and `SPEC-ALIGNMENT-AND-ACTION-PLAN.md`
use the correct full path. *Proposed:* a coordinated repoint of all short-form uses to
`alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`. Not fixed piecemeal: the register template matches the
greppable code tags, so the doc and the code SPEC-DELTA tags must move together (touches code, out of
this pass's markdown/comment scope).

**FND-4 — `11-doc-method.md` does not exist (Phase 1.3).**
`conventions-and-decisions.md` cites `11-doc-method.md` five times (its doc-method companion). No file
of that basename exists. Best candidates: `beta/impl/doc-writing-method.md` (the doc-method carrying the
numbered "Rule N" that Part 2 cites throughout — most likely) or `alpha/seeds/p10-p11-corpus/p10-drystone-doc-method.md`.
*Proposed:* repoint to `beta/impl/doc-writing-method.md`. Path-fixes to `conventions` are permitted, but
the two-candidate ambiguity makes it a judgment call, so it is flagged rather than guessed.

### LOW

**FND-5 — §7.6.11 carries pre-unification status tags and imperfect paths (Phase 2.2/1.3).**
§7.6.11's preservation banner reads `` `design; needs verification. … [confirm before publish]` `` — the
lowercase `design` and `[confirm before publish]` forms the p10 B.6 pass mapped to `Design` / `[confirm]`
(conventions A.9). The same subsection cites `impl/delivery-layer/12-replant-experiments.md` and
`impl/mls/mls-hardcases-and-posture.md` without the leading `../` (they resolve by basename to
`beta/impl/…`). It is a self-described "preserved" historical block. *Proposed:* normalize the tags to
A.9 and add the `../`, or confirm the block should stay verbatim as preserved.

**FND-6 — `12-side-histories-and-threading.md` renamed (Phase 1.3).**
`part-2-changelog.md` Pass-2 entry cites fold-source `12-side-histories-and-threading.md`; the file is now
`beta/impl/mls/side-histories-and-threading.md` (its sibling `07-history-modes.md`, cited in the same
sentence, still resolves with its `07-` prefix). Historical changelog entry. *Proposed:* leave as a
provenance record, or repoint to the current basename; note the asymmetry with `07-`.

**FND-7 — `discovery/thinking/revocation-authority.md` "out of this workspace" (Phase 1.3).**
`EXPERIMENT-BACKLOG.md` §6d says the revoke design "lives in the sibling
`discovery/thinking/revocation-authority.md` (out of this workspace)," but a file of that name exists
in-repo at `alpha/thinking/revocation-authority.md`. *Proposed:* confirm whether the in-repo copy
supersedes the external reference and repoint if so.

**FND-8 — doubled prefix in Part 1 §2.5 (Phase 1.6/1.7).**
Part 1 §2.5 (L529) reads "…**Part 2 Part 2 §7.6.1** enumerates both…" — a doubled "Part 2". Part 1 body
is no-edit in this pass. *Proposed:* the owner drops the duplicate "Part 2".

**FND-9 — the register has two Active rows, not one (Phase 2.3/2.5a).**
Several framings (and the RUN-05 brief) say "`hermetic-gossip` is the only Active divergence row," but the
`SPEC-DIVERGENCE-REGISTER.md` "Active divergences" table lists **two**: `hermetic-gossip`
(test-hermeticization) **and** `fanout-single-run` (proxy-measurement, "shape holds, magnitude
indicative"). This pass followed the register (evidence) and kept both active in `MASTER-INDEX.md`.
*Proposed:* confirm whether `fanout-single-run` stays Active or moves to "Already-declared caveats."

**FND-10 — RUN-02..04 spec terms not yet in the shared vocabulary (Phase 3.8).**
`conventions-and-decisions.md` A.11 codifies the human-adjudication vocabulary but does not carry the
newer Part 2 terms *approval subject*, *contradiction byte-head*, *horizon checkpoint*,
*horizon-checkpoint manifest*, *corroboration dials*, or *quantified trust*, several of which plausibly
belong in the shared vocabulary. *Proposed (not made):* add them to A.11 / the shared term surface so
experiment docs and tests inherit them by reference.

**FND-11 — Part 1 back Map omits a back-matter section (Phase 1.7).**
Part 1's `## 0. Map` lists every §-section but not the `## Upstream reference links (versioned)` section
(nor itself). Part 1 back-matter, no-edit here. *Proposed:* add the missing Map line, or confirm the Map
intentionally indexes only §-sections.

---

## Verified clean (checked, found consistent)

Stated explicitly so silence is never ambiguous.

**Phase 0 — RUN-04 landing intact (FINDINGS-only, no miss).**
1. `alpha/thinking/corroboration-and-quantified-trust.md` exists with its banner and sections §0–§6
   (epistemic floor; write side; read side / unreferenced tail; formula-valued thresholds; circular
   assertion awareness [exploratory, two seams]; the beam reframed; the contract experiment).
2. The corroboration-dials paragraph sits immediately after the §7.3.3 load-bearing-caveat paragraph
   ("…a sufficiently isolated node **cannot** establish final state on its own"), verbatim, tagged
   `Design`.
3. The formula-valued threshold paragraph closes §7.4.1 (immediately before the §7.4.2 heading),
   verbatim, tagged `Design`.
4. **EXP-C1** sits at backlog §2c beside **EXP-H1** (§2b), with its four RED-able assertions and the
   §8.2(e)-residual discharge note.
5. Map (§0 §7.3 + §7.4 lines), `part-2-changelog.md` (RUN-04 entry), and reviews-log (2026-07-14
   corroboration-dials entry) all present.

**Phase 1 — mechanical integrity.**
- 1.1 Part 2: every internal `§`, `§N.N(.N)`, `R1`–`R7`, and `Appendix A`–`F` reference resolves; the
  only "non-existent" section numbers are all RFC citations (RFC 9420/9750/8446). §7.2 defines exactly
  R1–R7. Every "Part 1 §…" citation from Part 2 (11 distinct: §2.0, §2.0.1, §2.1–§2.8, §3) resolves to a
  real Part 1 heading. The "Appendix G" mention is a self-aware retired label pointing at the existing
  `research-prompt-operational-rates.md`.
- 1.2 §0 back Map covers every substantive Part 2 section including all RUN-02/03/04 additions: §7.2 R7,
  the §7.3.2 two-competing-quorums note, the §7.3.3 corroboration dials, the §7.4.1 formula-valued k, the
  §7.6.2 continuity/decoupling passage, the §7.6.9 horizon example, and the Appendix B horizon-checkpoint
  manifest clause. §6.6 is covered by the "§6.5 through §6.7" range. §1 (Introduction) is intentionally
  not bulleted (it is the front matter housing the Map pointer). The §11-internal map (§11.1–§11.14) is
  complete.
- 1.3 ~90 cited paths checked; all specifically-named probe targets resolve (`competing_quorums.rs`,
  `rulechange_threshold_enforced.rs`, `fold_derived.rs`, `X3-CROSS-PACKAGE-SWEEP.md`,
  `croft-chat/FANOUT-M1.md`, the `scripts/*.sh`). The only broken-as-written paths are FND-3/4/6/7 and the
  §7.6.11 imperfections (FND-5).
- 1.4 `part-2-changelog.md` carries an entry for every Part 2 edit in the RUN-02/03/04 file lists (plus
  the new RUN-05 entry).
- 1.5 the reviews-and-experiments log carries the RUN-02 (2026-07-13 reconciliation landing), RUN-03
  (continuity + horizon + Phase B), and RUN-04 (corroboration dials) entries.
- 1.6 typos: the only genuine doubled-word is Part 1's "Part 2 Part 2" (FND-8); "Group Group Role" and
  the like are intentional term usage. DR-3 *hard-stop* spelling is clean across the spec (the only
  "hard stop" hits are the DR-3 rule and B.8 quoting the misspelling as forbidden). "byte-head" is
  hyphenated in all 18 occurrences (0 unhyphenated).
- 1.7 every Part 1 internal cross-reference resolves; the Part 1 map matches the §-structure (back-matter
  omission is FND-11).

**Phase 2 — status/evidence coherence.**
- 2.2 tag audit: F1/R7 is `Modeled` everywhere and never over-read as `Verified`; no passage claims
  relay/real-NAT evidence (§8.2(a)'s loopback caveat is intact); the fan-out claim is at loopback grade
  (it is understated, FND-2, not over-claimed). Pre-unification tags survive only in §7.6.11's preserved
  banner (FND-5).
- 2.3 every Reconciled-row "spec landing" pointer in the register resolves (`competing-quorum-autoresolve`
  → §7.3.2 / §7.6.1, RUN-03; `rulechange-quorum` → §7.2 R7, RUN-02; `automerge-0.6.1` → RUN-01 EXP-2). The
  two-active-rows question is FND-9.
- 2.4 the alignment doc's §7 decision annotations match reality (decision 2 executed by RUN-02; decision 5
  unchanged); the doc is now bannered point-in-time (M8).
- 2.5d the `alpha/experiments/README.md` `## Contents` tree matches the actual directory structure (all 12
  experiments present, purposes accurate).

**Phase 3 — semantic consistency.**
- 3.1 new-paragraph coherence: the corroboration-dials ¶ **sharpens** the beam-caveat ¶ (it names the
  governed residual and states "safe at every setting because the fail-closed rule is not a dial") and
  does **not** read as closing the beam (the caveat ¶ and the thinking note both keep the intrinsic limit
  open). The formula ¶ is consistent with §7.4.1's governed tolerance; the horizon ¶ with §7.3.3
  checkpoint semantics and the §7.4.3 stamp; the continuity ¶ with §7.6.2's arities and §5.10/§5.11. No
  tensions.
- 3.2 terminology uniformity: one term per concept (byte-head, horizon checkpoint, approval subject,
  corroboration dials, quantified trust used consistently). No thinking-doc vocabulary (`ForkDescriptor`,
  "Layer 1"/"Layer 2", projection-verb names) leaks into Part 2 — those appear only in the thinking notes;
  Part 2's "projection" uses are its own established terms (effective-roles projection, public-projection
  cache, point-in-time projection).
- 3.3 DR-block compliance: the RUN-02/03/04 inserts are continuity-framed and non-moral (the F8 ¶ is
  symmetric: "most often a misunderstanding or a legitimate grievance").
- 3.4 citation support: the new-passage Part 1 citation (§2.5 in the F8 boundary note and §7.6.1) is fully
  supported — Part 1 §2.5 is "why fork, not verdict" and states a verdict would "manufacture a consensus
  that was never available," exactly what the Part 2 passages invoke.
- 3.7 Part 1 → Part 2 direction: §2.5 (fork-not-verdict; mutual-expulsion / vacant-role residue) stays
  compatible with the refined §7.6 (the competing-quorum case is a "too many valid claims" contradiction,
  subsumed under §2.5's framing); §2.0.1 (timestamp-free ordering) stays compatible with §7.3.1. No Part 1
  assertion contradicts landed Part 2.
- 3.8 conventions A.11 terms are consistent with current spec usage; `open-threads.md` is consistent with
  the landed state — the bannered total-device-loss-recovery item's "Direction confirmed 2026-07-07" matches
  the alignment-doc decision-4 correction, and no Stage/experiment reference is a dead pointer. The
  shared-vocabulary addition is proposed as FND-10.

---

## Settlement (RUN-06, 2026-07-14)

Every RUN-05 FINDING above is now settled. Owner calls (FND-1/2/9 and the FND-5/10 scope) were taken
2026-07-14; the rest carried a single clear resolution. Dispositions:

| # | Sev | Disposition | What changed |
|---|-----|-------------|--------------|
| FND-1 | MED | **Owner call: accept §7.4.2 as shorthand.** No edit. | The GroupInfo-hazard §7.4.2 citations are correct as-is; the §8.2(e) originating-op freshness-precondition citation is confirmed as the accepted doc-wide shorthand for the §7.4 freshness + §7.4.2 recovery cluster. Closed intentional; future passes should not re-flag. |
| FND-2 | MED | **Applied: landed F4.** | §11.11 measurement #1 regraded *half-earned → earned in shape (both halves), magnitude-open at scale*, carrying the `fanout-single-run` caveat and the super-linear connect-time-resync flag (§6.8.1). `proposed-changes` F4 marked landed; changelog entry added. |
| FND-3 | MED | **Applied: coordinated repoint.** | All 7 short-form `alpha/SPEC-DIVERGENCE-REGISTER.md` uses → `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`: the register's own SPEC-DELTA template, the two live code tags (`iroh_bus.rs`, `iroh_convergence.rs` — comment-only), `REPO-README.md`, and three `croft-chat/plans/*` uses. Template and code tags moved together (no desync). |
| FND-4 | MED | **Applied: repoint to `doc-writing-method.md`.** | The 5 `11-doc-method.md` cites in `conventions-and-decisions.md` repointed to `doc-writing-method.md` (bare basename, matching the doc's companion-cite style; resolves to `beta/impl/doc-writing-method.md`, the canonical writing method per `RAW-ARTIFACTS-MANIFEST`). The `alpha/seeds/p10-*` raw-seed copy left verbatim (historical seed). |
| FND-5 | LOW | **Owner call: settle.** Applied. | §7.6.11 preserved banner normalized to A.9: `design → Design`, `[confirm before publish] → [confirm]`; the two cited paths gained the leading `../`. Block stays preserved; only tags/paths regularized. |
| FND-6 | LOW | **Applied: repoint basename.** | `part-2-changelog.md` Pass-2 entry `12-side-histories-and-threading.md` → `side-histories-and-threading.md`. Sibling `07-history-modes.md` left as-is — that file keeps its `07-` prefix (`beta/impl/delivery-layer/07-history-modes.md`), so the asymmetry is real and correct. |
| FND-7 | LOW | **Applied: in-repo copy is authoritative.** | The in-repo `alpha/thinking/revocation-authority.md` supersedes the "out of this workspace" reference. `EXPERIMENT-BACKLOG.md` §6d and `iroh/TEST-LOG.md` repointed; the "(out of this workspace)" phrasing dropped from the backlog. |
| FND-8 | LOW | **Verified already clean. No-op.** | No doubled "Part 2 Part 2" survives at Part 1 §2.5 (the site now reads a single "Part 2 §7.6.1"); resolved in prior history. Recorded so the register is not left with a phantom open item. |
| FND-9 | LOW | **Owner call: `fanout-single-run` stays Active.** No edit. | It is a live proxy-measurement gap (shape holds, magnitude indicative). The register and `MASTER-INDEX.md` already list both Active rows (`hermetic-gossip` + `fanout-single-run`); no live doc asserts a single active row, so nothing needed correcting. Consistent with landing F4 (FND-2). |
| FND-10 | LOW | **Owner call: settle.** Applied. | Conventions A.11 shared surface extended with the RUN-02..04 terms — *approval subject*, *contradiction byte-head*, *horizon checkpoint* / *horizon-checkpoint manifest*, *corroboration dials*, *quantified trust* — each anchored to its Part 2 definition. |
| FND-11 | LOW | **Applied: add Map line.** | Part 1's `## 0. Map` gains an entry for the `## Upstream reference links (versioned)` back-matter section (the Map already indexes non-§ back-matter, so the omission was a genuine gap, not an intentional §-only index). |

**Guardrail note.** FIX/FINDING discipline is inverted for this run by design: RUN-06 *is* the owner-call
pass, so the FINDINGS are settled rather than deferred. Edits touched three normally-protected surfaces —
Part 1 back-matter (FND-11, additive Map line only), `conventions-and-decisions.md` (FND-4 path-fix, FND-10
vocabulary addition), and the code SPEC-DELTA tags (FND-3, comment-only, verified). No Part 1 body prose,
no mechanism, and no code behavior changed; the `.rs` diff is comment-only by inspection.

**Addendum (RUN-07, 2026-07-14).** FND-1's ruling is refined 2026-07-14 from "accept §7.4.2 as
shorthand" to the explicit **range cite (§7.4–§7.4.2)** in the living docs (Part 2 §8.2(e) and the
one EXPERIMENT-BACKLOG.md quote of that clause); the two §7.4.2 cites in the §7.4.2-hazards table
rows are correct on their own terms and stay as-is. FND-5 and FND-6 dispositions are ratified post
hoc by the owner: mechanical normalization to current conventions is accepted on preserved /
provenance blocks as a narrow exception to the frozen-record rule. FND-8's original finding is
recorded as **mistaken** — the site was already clean, so the RUN-05 flag was a false positive, not
a fix.

---

## Traceability findings (RUN-08)

`Companion to RUN-08-SUMMARY.md Part 2. The spec ↔ experiment traceability pass ran against the
branch's post-Part-1 state. FIX = mechanical/provable/meaning-preserving (applied this run); FINDING
= judgment or meaning-adjacent (recorded here for a settlement pass, same as RUN-05 → RUN-06). Part 2
never moves a status tag; every tag that looks wrong is a finding, not an edit. IDs are FND-T*.`

**FIXes applied this run** (not findings, listed for completeness): backward-link Serves: headers on
the five load-bearing reports (2.2a); §-ref doc-comments on 21 spec-earning tests (2.2b, comment-only
commit); the `handcrafted-assertions` register row gained its retirement landing (2.2c); backlog §2d
(Vouch residual) and §6d/§6g rows carry spec/register pointers + retirement conditions (2.2d); the
`EVIDENCE-MAP.md` index (2.3). The Part 1B conditional edits (§10.5 footnote, F7) reconciled the
conformance ledger. No status tag was moved by Part 2.

### HIGH

None. Every Part 2 tag at or above `Modeled` resolves either to a named test + RUN (the reconciled
governance claims) or to a primary-source / substrate reference (the §4–§6/§10 band). No tag was
found unresolvable, and no cited test/path/RUN failed to resolve on grep.

### MED

- **FND-T1 — the substrate `Verified`/`Verified-RFC` band carries no test/RUN pointer and no
  standardized evidence parenthetical.** ~40 tags in §4–§6, §7.4.2, §7.6.3–§7.6.4, §8.1, §10.2–§10.4
  name *what was exercised* but point at an RFC/draft/spec section or a phrase like "against iroh
  1.0", "measured" — not a named experiment test + RUN. Example, §4.1: "`Verified` against real
  Ed25519 over live iroh: a forged message is rejected …". Meaning-adjacent: for a `Verified-RFC`
  claim the literature *is* the evidence (A.9), and for the substrate `Verified` rows the anchor is
  the conformance-core (cats 1–6, now 66/0) and the feasibility review, not an experiment RUN.
  **Proposed:** leave `Verified-RFC` rows as literature-anchored; add one standing note (not a
  per-sentence edit) that the substrate `Verified` band is anchored in the conformance-core + review;
  `EVIDENCE-MAP.md` §D carries this band explicitly. No auto-FIX (adding a RUN where the claim rests
  on an RFC would be an invented link).

- **FND-T2 — the §10.5 conformance footnote materially understated the emitted suite.** The 2026-07
  footnote said cats 7/8/9 and the revoke-authority vector were "specified but not yet emitted in the
  reference conformance-core", yet the folded conformance-core re-proves **66/0** (cat 7 real Rust,
  cats 8/9 TS-authoritative, cat-5b revoke-authority *mechanism*). Quoted (original): "conformance
  categories **7/8/9** … and the **revoke-authority-threshold** vector are **specified but not yet
  emitted**". **Reconciled by RUN-08 Part 1B** (the footnote now names 66/0 and the true residual, the
  over-the-wire authority distribution). Residual, recorded as a finding for ratification: the word
  "emitted" is doing two jobs — "vectors exist and re-prove" (true) versus "emitted from real
  over-the-wire MLS + real k-of-n" (the honesty boundary). **Proposed:** owner ratifies the RUN-08
  wording, or splits the two senses explicitly in a future pass.

- **FND-T4 — the standardized evidence parenthetical does not exist in Part 2.** 2.1d asks for
  `(evidence: <test or report>, RUN-NN[, grade])`; grep finds **0** occurrences in
  `part-2-certifiable-design.md`. The reconciled governance claims already carry inline test+RUN
  prose (e.g. §7.2 R7: "RED→GREEN: `rulechange_threshold_enforced.rs`; … X3 …"), just not in the
  standard shape. **Proposed:** adopt the parenthetical in the claims where all components already
  exist (§7.2 R7, §7.3.2 competing-quorums, §7.6.2 membership half, §8.2(e)); recorded rather than
  auto-applied this run to avoid churning tag-adjacent sentences the `EVIDENCE-MAP.md` columns already
  index in the standard form.

### LOW

- **FND-T3 — a few spec-earning test §-refs were mapped at the section level from the corpus.** The
  §-refs added in 2.2b are strongly supported for the governance/re-plant families (reviews-log,
  backlog, RUN summaries map them explicitly). Four are *inferred* rather than drawn from a prior
  explicit mapping: `convergence.rs` (P7 → §7.3), `iroh_convergence.rs` (P18 → §6.10/§7.3, loopback),
  `regress_free.rs` (V3′ → §7.3), `dedup.rs` (G3 → §6.6.4). **Proposed:** owner confirms or corrects
  the four; each is a section-level anchor, not a claim-changing edit.

- **FND-T5 — the off-ladder token `Reviewer-judgment` appears as a status-like tag in live Part 2
  text.** §10.4 (BLAKE3 length-extension): "`Verified` for the BLAKE3 length-extension property; the
  construction-level check is `Reviewer-judgment`". `Reviewer-judgment` is not on the A.9 ladder.
  **Proposed A.9 mapping:** `[confirm]` (rests on a judgment not yet independently verified) or
  `Synthesis`; **not auto-rewritten** — the normalization exception covers preserved blocks' tag
  *format*, not a live sentence's tag *meaning* (2.4a).

- **FND-T6 — former-tag vocabulary (`green-real`/`green-model`/`not_yet_emitted`/`PLACEHOLDER`) lives
  in alpha-tier and staging docs.** It appears in `conformance-suite.md`, the relay-lab-runs
  manifests, and `proposed-changes-…`/F7 — all pre-A.9-unification or staging vocabulary, acceptable
  there (A.9 records green-real→`Verified`, green-model→`Modeled` as the absorption). One instance
  introduced into the live §10.5 footnote by the Part 1B draft was removed this run. **Proposed:** if
  any of these tokens migrate into live Part 1/Part 2 sentences, map them to the ladder per A.9.

- **FND-T7 — bound-qualifier spelling drift: `real-NAT` vs `real NAT`.** Both spellings appear for
  the same bound (e.g. "real-NAT path remains X1" and "real NAT traversal"). The other qualifiers
  (loopback / substrate / cross-package / single-run) are single-spelling. **Proposed FIX (deferred
  to settlement to avoid a wide mechanical sweep this run):** canonicalize the compound *qualifier* to
  `real-NAT` (hyphenated) and leave the free-noun "real NAT" where it reads as prose; low risk, but a
  broad find-replace, so recorded rather than swept.

---

## Settlement (RUN-09, 2026-07-15)

Five RUN-08 traceability findings were confirmed for settlement by the owner 2026-07-15 (FND-T1,
FND-T4, FND-T5, FND-T6, FND-T7). The other two RUN-08 findings (FND-T2, FND-T3) are left as
recorded — FND-T2 was already reconciled by RUN-08 Part 1B with its residual meaning-ambiguity
noted for a future ratification, and FND-T3's four inferred section-level anchors await an owner
confirm-or-correct that this run's rulings did not cover. Dispositions:

| # | Sev | Disposition | What changed |
|---|-----|-------------|--------------|
| FND-T1 | MED | **Owner call: redefine the forward-link target per band. Applied.** | One A.9 evidence-linkage note now states the two forms: an *experiment-earned* tag (`Verified` on real crypto/transport, `Modeled`, `Measured`) resolves to a named test/report + RUN; a `Verified-RFC` / literature-anchored tag resolves to its primary-source anchor, which already *is* its evidence. **Substrate-band re-audit (≈40 tags, §4–§6, §7.4.2, §7.6.3–§7.6.4, §8.1, §10.2–§10.4):** under the right form the band closes as satisfied. The `Verified-RFC` and literature-anchored rows (§6 RFC 9420/9750 and iroh-1.0 citations, §7.4.2 recovery hazards, §7.6.3–§7.6.4 MSC/RFC and Part-2 self-cites, §10.2–§10.4 MLS-RFC + BLAKE3/Ed25519 primitives) resolve to their primary source. The *experiment-earned* substrate rows (§4 wire derivations, the §6 loopback-gossip rows, §6.6.4 dedup) already carry test+RUN pointers in `EVIDENCE-MAP.md` §B (conformance-core cats 1–6, `convergence.rs` / `iroh_convergence.rs` / `dedup.rs`). No experiment-earned tag in the band was found still lacking a pointer, so none stays a FINDING. |
| FND-T4 | MED | **Owner call: adopt going forward, no retrofit. Applied.** | One A.9 evidence-linkage note adopts the standardized `(evidence: <test or report>, RUN-NN[, grade])` parenthetical as the recommended form for evidence sentences written from RUN-09 onward. It is **not** back-fitted onto Part 2 sentences that already carry inline test+RUN prose; the `EVIDENCE-MAP.md` columns already index those in the standard form. |
| FND-T5 | LOW | **Applied: de-backtick.** | §10.4 (BLAKE3 length-extension) `` `Reviewer-judgment` `` → plain prose "the construction-level check rests on reviewer judgment". No eleventh rung is introduced; the off-ladder token no longer reads as a status tag. |
| FND-T6 | LOW | **Owner call: settle. Applied.** | One A.9 evidence-linkage note records that the legacy `green-real` / `green-model` / `not_yet_emitted` (and `PLACEHOLDER`) vocabulary is alpha-tier / staging only (B.6 records the absorption) and never appears in a live Part 1 / Part 2 sentence; a token that migrates in is mapped to its ladder rung. |
| FND-T7 | LOW | **Applied: canonicalize the qualifier.** | The compound *qualifier* `real-NAT` is hyphenated across the living docs. The only living-doc drift instance was `MASTER-INDEX.md` ("real NAT traversal" → "real-NAT traversal"); the spec set (`part-2-certifiable-design.md`, `part-2-changelog.md`, `EVIDENCE-MAP.md`) and the active tracking docs were already compliant. The free-noun "real NAT" (e.g. "behind a real NAT", "needs real NAT") is left as prose, and the frozen iroh-testbed transcripts and bannered historical records are out of the living-doc scope by design. |

**Guardrail note.** Part 1 is markdown-only and moves no status tag: the A.9 additions (FND-T1/T4/T6)
are evidence-linkage conventions, not new rungs; the §10.4 edit (FND-T5) removes backticks from an
off-ladder token without changing the sentence's meaning; the FND-T7 edit is a single hyphen. The
`EVIDENCE-MAP.md` §D band note and its open-FINDINGs list are updated to mark FND-T1/T4/T6 settled
(the map stays an index, not a status source). FND-T2/T3 stay open as recorded.

---

## RUN-10 findings (2026-07-15) — read-derived, from the three briefs and the site build

All five are read-and-report findings; **no spec, register, crate, or frozen record was edited this
run.** They are recorded for the owner. Prefix `FND-R10-`.

**FND-R10-1 (LOW, framing) — §5.10's "communal-namespace key ... rotates under churn" presupposes an
object Meadowcap's communal model does not have.** Part 2 §5.10: *"What is unworked is the key rotation
scheme, how the Group and its members jointly own the namespace and how the key rotates under churn"*
(and §5.2: *"key establishment and rotation under membership change"*). Live Meadowcap (Final, as of
21.11.2025): communal authority *"is derived from ownership of a given subspace key pair"*; the
namespace secret key is used only in an **owned** namespace. This is **not a contradiction** of any claim
Part 2 asserts — §5.10 correctly leaves the scheme "unworked" — but the framing presupposes a shared
whole-namespace key that a communal namespace does not have. Recommended (owner-decides, not this run):
restate the seam as (a) per-persona subspace-key rotation + (b) the read-scoped asset key
(`asset-keying.md`) + (c) a near-free identifier assignment. Source: `group-principal-seam.md` (RUN-10
Part 2). No edit made.

**FND-R10-2 (LOW, consistent — recorded so it is not re-litigated) — Meadowcap's only described
revocation workaround is structurally unavailable to a communal Group-principal.** Live Meadowcap: *"No
built-in revocation mechanism"*; the sole workaround (owner writes future-timestamped overwriting
entries) is **owned-namespace-specific**. Part 2 §5.10: the Group-principal is *communal by default*, and
*"the owned model is the apex Drystone rejects"*. Reconciliation (already consistent with §10.4's
"differ only on revocation immediacy"): revocation MUST come from the governance fold (R3/R4), never from
Meadowcap. Recorded so no future draft reaches for the overwrite trick. Source: `group-principal-seam.md`.
No edit made.

**FND-R10-3 (LOW, informational) — the welcome-over-iroh emitter residual is already ~80% realized as
the brief's Option B, informally.** RUN-08 §1B produced two co-located artifacts —
`relay-lab-runs/C-mls-welcome-2026-07-15-run08/verdict.json` (over-the-wire welcome sourcing) beside
`conformance-suite-reprove.txt` (the 66/0 re-prove) — and called the emission "realized in the existing
formats without a vector-format change." So Option B (a thin adapter that hardens that side-by-side into
an asserting crate) is a low-marginal-cost formalization, not net-new work. Strengthens B as the fallback
to the recommended defer. Source: `EMITTER-INTEGRATION-BRIEF.md` (RUN-10 Part 3). No edit made.

**FND-R10-4 (LOW/MED, stale claim inside the frozen record) — `alpha/Proofs/FROZEN-NOTICE.md` undercounts
the emitted conformance categories.** FROZEN-NOTICE.md (line ~23): *"conformance-core that emits
categories **1–6**"*. But `crates/conformance/src/bin/emit_vectors.rs` `categories_present` lists cats
**1, 2, 3, 4, 5, 5b, 6, 7, 8, 9**, and Part 2 §10.5 confirms *"66/0 across categories 1–9"*. A
documentation lag **inside the frozen tree** (same theme as FND-T2, the superseded §10.5 "cats 7/8/9 not
yet emitted" footnote). **Not editable this run** — the frozen record is out of scope — so recorded here;
the owner may correct it at the next deliberate freeze-break (e.g. the `[gates-release]` pass). Verified
this run against both files. Source: `EMITTER-INTEGRATION-BRIEF.md`.

**FND-R10-5 (LOW, definitional) — two distinct layers share the label "Layer 2 / L2".** croft-group's
**L2 = MLS / encryption** (the `Frame`→ciphertext plan step) is different from the **parked "Layer-2
resolution-ACL"** design (fork-projection read-scope), which maps to croft-group **L3** (fork/merge +
reconvergence). No EVIDENCE-MAP row or Part 2 sentence is contradicted; the load-bearing correction is
that **croft-group L2 (MLS) does not depend on the parked resolution-ACL design**, so the L2a slice is
buildable now. Recorded, not auto-rewritten. Source: `CROFT-GROUP-L2-READINESS.md` (RUN-10 Part 4).

---

## Settlement (RUN-11, 2026-07-16)

The RUN-11 Part 1 riders confirm the RUN-10 findings (FND-R10-1/4/5) plus two carried RUN-08
traceability findings (FND-T2, FND-T3) and FND-T4, and record the emitter decision and the Map audit
catches. Owner rulings were taken 2026-07-15. Dispositions:

| # | Ruling | Disposition | What changed |
|---|--------|-------------|--------------|
| FND-T2 | **Ratified. No spec edit.** | The RUN-08 §10.5 wording stands as reconciled; the footnote already names the over-the-wire authority distribution as the residual. The two senses of "emitted" (vectors exist and re-prove, versus emitted from real over-the-wire MLS + real k-of-n) **may be split explicitly whenever that footnote is next touched, not before.** Recorded here; no Part 2 edit. |
| FND-T3 | **Confirmed. Doc-comments unchanged.** | The four inferred test §-mappings stand as owner-confirmed: `convergence.rs` → §7.3; `iroh_convergence.rs` → §6.10/§7.3 (loopback); `regress_free.rs` → §7.3; `dedup.rs` → §6.6.4. The 2.2b doc-comments already carry these anchors; nothing edited. |
| FND-T4 | **Applied, narrowly. One claim FINDING-stopped.** | The standard `(evidence: …, RUN-NN[, grade])` parenthetical reshaped the existing inline prose in three of the four candidate claims — §7.2 R7 (`rulechange_threshold_enforced.rs` + `rulechange_quorum_via_api.rs` + X3, RUN-07, `Verified`), §7.3.2 competing-quorums (`two_competing_rulechange_quorums`, RUN-03, `Modeled`), §8.2(e) (R7 count tests + X3, RUN-07, `Verified`) — with no information added or dropped. The fourth, **§7.6.2 membership half, is a FINDING**: its RUN-NN component is missing (the membership half was imported as already-`Verified` from the standalone experiments corpus and carries no discovery-RUN stamp; EVIDENCE-MAP row 52 lists the test `e12_7_*` but no RUN), so per FND-T4's own rule ("a FINDING if any component turns out missing") it was **not** reshaped. `part-2-changelog.md` entry added. |
| FND-R10-1 | **Applied.** | §5.10's "how does the communal-namespace key rotate under churn" framing is reframed per the seam brief: a communal namespace has no shared whole-namespace secret to rotate; the question decomposes into per-subspace write authority (§4.5) and the fold-gated asset key (§5.11), leaving a near-free identifier assignment and the primary-versus-secondary choice. `Design` tag; the seam brief is cited (`beta/impl/drystone-design/group-principal-seam.md`). `part-2-changelog.md` entry added. |
| FND-R10-4 | **Bannered.** | One correction banner added atop `alpha/Proofs/FROZEN-NOTICE.md` (body untouched): the "emits categories 1–6" line understates the folded core, which emits and re-proves categories 1–9 (66/0, RUN-08). The frozen body below the banner is the original; the record itself is not edited (the deliberate correction waits for the next freeze-break). |
| FND-R10-5 | **Applied.** | In the living docs the parked design is referred to as the "resolution-ACL (croft-group L3)"; croft-group L2 (MLS) does not depend on it. The L2-readiness brief and the backlog §3 rows already carry the correct label (the model); a one-line disambiguation note was added at the collision's origin, `alpha/thinking/the-shape-of-disagreement.md` §4 (whose own internal "Layer 2" names the resolution ACL), mapping it to croft-group L3 and citing the readiness brief as the model. No other living doc conflates the two. |
| Emitter decision | **Recorded.** | `EMITTER-INTEGRATION-BRIEF.md` annotated: "Decided: Option C — defer to the `[gates-release]` pass (owner, 2026-07-15); Option B remains the fallback." The §10.5 residual line stays as-is; emitter integration is now formally deferred by decision and stays on the parked list. |
| Map fixes (8) | **8a no-op (already clean); 8b applied.** | (a) The flagged duplicate §7.6 back-Map line is **not present** in the current tree — a single §7.6 entry stands at the `## 0. Map`, so nothing was removed (recorded so the audit is not left with a phantom, same as RUN-05 FND-8). (b) The surviving copy read "message-continuity half open"; it now reads "message-continuity half `Modeled` at loopback grade (RUN-09)", matching the §7.6.2 body and the EVIDENCE-MAP row. The site build (gate + anchor audit) stays clean after the edit. `part-2-changelog.md` entry added. |

**Guardrail note.** Part 1 moves no experiment status tag. The Part 2 edits are: the §5.10 reframe
(a framing change ratified by FND-R10-1, `Design`), three FND-T4 evidence-form reshapes (form-only,
no tag moved, no information added or dropped), and one back-Map wording line mirroring an
already-recorded grade. The FROZEN-NOTICE change is a banner only (frozen body untouched, the
ratified normalization exception does not even apply because the body is not touched). FND-T2/T3 are
recorded rulings with no edit.

**Follow-on (RUN-11, 2026-07-16): the §7.6.2 membership-half FND-T4 FINDING is resolved.** The
finding was that the membership half carried no discovery-RUN stamp, so the `(evidence: …, RUN-NN,
grade)` reshape could not be applied without inventing a pointer. Rather than invent one, the E12.7
keystone tests (`e12_7_1_stamp_tracks_derivation`, `e12_7_2_removal_propagates`,
`e12_7_3_unauthorized_no_drift`) were **re-proven in-environment, 3/3 green on real openmls 0.8.1
(RUN-11 re-proof)** — the same move RUN-08 used to re-prove the conformance suite 66/0. The §7.6.2
membership-half sentence now carries the standard parenthetical `(evidence: e12_7_1/2/3_*.rs, RUN-11
re-proof, `Verified`)`; EVIDENCE-MAP row and `part-2-changelog.md` updated. No status tag moved (the
membership half was and stays `Verified`); the reshape adds only the now-earned pointer.

**Follow-on (RUN-11, 2026-07-16): the §2e green-real scope-wall stop is partly resolved.** Part 3
FINDING-stopped the row's `Design → green-real` aspiration because it exceeded the run's `Design`-grade
scope wall. As a follow-on, the **subspace-derivation half** was moved to `green-real` by *reusing* the
`Verified` lineage fold (`lineage-mls::Device::fold_by_lineage`, RUN-08 `fold_matches`) on real openmls
0.8.1 leaves — `group-principal-seam/tests/subspace_fold_green_real.rs` folds a persona's several device
leaves to one subspace identity, deterministically. No firewall or gate was crossed: the `SubspaceId`
**byte encoding** stays `[gates-release]` (Appendix B / E.1) and the revocation-authority **trust tier**
stays **I9** — both are deliberate parked gates, not defects, and remain open. EVIDENCE-MAP carries a
new `green-real` row for the fold; the Design-model bindings (`seam.rs`) are unchanged.

---

## Settlement (RUN-12, 2026-07-16)

The RUN-12 Part 1 ruling settles how evidence that entered the corpus from **outside** the numbered-run
system is stamped, and applies that ruling to the one place a re-proof stood in for it. Owner ruling
taken 2026-07-16. Dispositions:

| # | Ruling | Disposition | What changed |
|---|--------|-------------|--------------|
| Import-provenance ruling (A.9) | **Applied.** | A one-sentence rider on FND-T4's evidence-form note: for evidence imported from outside the numbered-run system, the standard parenthetical's `RUN-NN` slot instead carries **import provenance** — `imported: <corpus> @ <commit>` — a verifiable pointer to the exact tree where the evidence lives and passes, and a retroactive RUN number is never invented. `conventions-and-decisions.md` A.9 edited; no status tag moves. |
| §7.6.2 membership half (the fourth FND-T4 retrofit) | **Reshaped; RUN-11 FINDING closed.** | The membership half was imported already-`Verified` from the standalone experiments corpus (`replant-continuity`'s `e12_7_*` tests, commit `d52ed6f`) and carries no discovery-RUN stamp. RUN-11 resolved the FND-T4 gap by **re-proving in-environment** and stamping `RUN-11 re-proof`; the RUN-12 ruling supersedes that with the cleaner import-provenance form. The §7.6.2 membership-half parenthetical now reads `(evidence: the e12_7_* tests, imported: replant-continuity @ d52ed6f, `Verified`)` — wording adapted to the sentence, no information added or dropped. No status tag moved (the membership half was and stays `Verified`). `part-2-changelog.md` entry added. |
| EVIDENCE-MAP row 52 | **Applied.** | Row 52's evidence cell carries the same import provenance in place of the `RUN-11 re-proof` note: `e12_7_1/2/3_*.rs` (E12.7; **imported: replant-continuity @ `d52ed6f`**), closing the FND-T4 §7.6.2 gap. |
| Site companion allowlist | **Applied.** | The 7 companion/exploratory unresolved references that passed as a silent soft baseline are now an explicit allowlist in `site/build.py` (`COMPANION_ALLOWLIST`, each entry: doc-id, ref, one-line reason). The gate now fails the build if the actual companion-unresolved set differs from the allowlist in **either** direction — a new unlisted unresolved ref (a broken link), or a listed entry that no longer fires (a stale allowlist) — so companion drift becomes catchable, not just Part 1 / Part 2 drift. |

**Guardrail note.** Part 1 moves no experiment status tag. The Part 2 edit is the §7.6.2 membership-half
evidence-form reshape (form-only, no tag moved, no information added or dropped). The A.9 edit is one
rider sentence on an existing evidence-linkage note. The `site/build.py` change tightens the gate; the
7 allowlisted refs are unchanged companion cross-references into unpublished docs (COHESION, ROADMAP,
doc-writing-method, social-layer) and one external MLS section. The site build (gate + anchor audit)
stays clean after the edits.

**The RUN-11 §7.6.2 membership-half FINDING is now closed.** Its RUN-11 follow-on resolution (re-prove
and stamp `RUN-11 re-proof`) is retired in favor of import provenance, which is a stronger record: it
names the exact tree where the evidence lives and passes rather than asserting an in-environment re-run.
No re-invention of a RUN number remains anywhere in the §7.6.2 evidence.
