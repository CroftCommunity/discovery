# RUN-09 — Documentation hygiene: the spec ↔ experiment traceability pass

`Branch: fresh off main, e.g. run-09-traceability. Gate: RUN-08 must be merged (it changes the
§10.5 conformance annotations this pass would otherwise immediately re-flag). Markdown throughout,
plus comment-only edits in test/experiment files (one separate commit). This run NEVER moves a
status tag — it fixes and builds the links around tags; any tag that looks wrong is a FINDING.`

## What "correct annotation" means here

Every claim and its evidence must be navigable in both directions:
- **Forward (spec → evidence):** every Part 2 status tag above `Design` resolves to its evidence —
  named test(s), report file, RUN number — with the claim's environment bounds stated (loopback /
  substrate / cross-package / single-run / real-NAT-pending).
- **Backward (experiment → spec):** every experiment crate, report, spec-earning test, SPEC-DELTA
  tag, register row, and backlog item names the spec section and claim it serves.
The FIX/FINDING discipline from RUN-05 governs: mechanical and provable → FIX; judgment or
meaning-adjacent → FINDING. The frozen-record rule holds, with its ratified narrow exception
(mechanical normalization of tags/paths to current conventions).

## Phase 0 — inventory (no edits)

Read `conventions-and-decisions.md` A.9 (the evidence ladder — the canonical tag vocabulary for
this whole run) and A.11 (shared terms). Then build, in the summary:
1. every evidence tag in Part 2 (and any in Part 1), with section and the sentence's evidence
   parenthetical if present;
2. every SPEC-DELTA tag in code; every register row; every backlog item with a status; every
   experiment report (`*-SWEEP.md`, `FANOUT-M1.md`, `X3-*.md`, spike READMEs) and spec-earning
   test file.

## Phase 1 — forward links (spec → evidence)

1.1 Every tag ≥ `Modeled` carries an evidence pointer (test name(s) and/or report path, RUN
    number). Where the pointer is missing but the evidence is unambiguous in exactly one RUN
    summary or report: add it (FIX). Ambiguous or contested: FINDING.
1.2 Every cited test exists under the cited name (grep), every cited report path resolves, every
    cited RUN number matches the summary that did the work. Fix renames/paths (FIX).
1.3 Environment bounds: wherever the register or a report bounds a claim (loopback-only,
    single-run magnitude, substrate-only, cross-package-for-the-count), the spec sentence carrying
    the tag states the bound. Missing bound whose wording is already fixed elsewhere in the doc
    set: FIX by reusing that wording verbatim. Missing bound needing fresh wording: FINDING with
    proposed sentence.
1.4 Pointer format: standardize the evidence parenthetical to one shape —
    `(evidence: <test or report>, RUN-NN[, grade])` — applying mechanically only where all
    components already exist in the sentence; FINDING where standardizing would add or drop
    information.

## Phase 2 — backward links (experiment → spec)

2.1 **Report headers.** Every experiment report and spike README opens with a standard block
    (add where missing, FIX):
    `Serves: Part 2 §X.Y (<claim, one clause>) — earns/bounds: <tag or bound> — register: <row(s)
    or none> — landed: RUN-NN.`
    Populate only from verified Phase 0/1 data; anything uncertain → FINDING.
2.2 **Spec-earning tests.** Every test file whose green earns or bounds a spec claim carries a
    doc-comment naming the section and claim (the `competing_quorums.rs` header is the model).
    Add where missing (comment-only commit); the mapping comes from Phase 1's verified links.
2.3 **SPEC-DELTA and register.** Every SPEC-DELTA code tag resolves to a live register row; every
    register row's spec pointer and evidence pointer resolve; every retired row names its
    retirement run. Fix dead pointers (FIX); substance → FINDING.
2.4 **Backlog.** Every open item names the spec section or register row it would move and the
    retirement condition; every done item names its landing run. Add missing pointers where
    unambiguous (FIX).

## Phase 3 — the living traceability artifact

Create `beta/drystone-spec/EVIDENCE-MAP.md` (living doc):
- One row per tagged claim: section | claim (short clause) | tag | bounds | evidence (tests,
  reports, RUN) | register rows | gates (`[gates-release]` items, X-items) it waits on.
- Populated ONLY from links verified in Phases 1–2; a claim with unresolved links appears with its
  FINDING id, not an invented pointer.
- Header documents the regeneration recipe (the Phase 0 scan) and the rule that this map never
  *sources* a status — the spec sentence is authoritative, the map is an index.
- Add one pointer line to Part 2 (the §8.2 preamble or the Map header — wherever house style
  allows a single sentence) naming EVIDENCE-MAP.md as the claims-to-evidence index. Add the map to
  MASTER-INDEX and the Rule 15/changelog bookkeeping.

## Phase 4 — annotation vocabulary conformance

4.1 A.9 is the only tag ladder. Inventory every tag-like token in the doc set; off-ladder tokens
    (legacy or improvised) get a FINDING each with the proposed A.9 mapping — do not auto-rewrite
    body text (the ratified normalization exception covers preserved blocks' tag *format*, not
    live sentences' tag *meaning*).
4.2 One spelling per bound qualifier (loopback / substrate / cross-package / single-run /
    real-NAT): FIX pure spelling drift; FINDING where two spellings might mean two things.

## Guardrails

- No status tag moves, upgrades, or downgrades anywhere, under any provocation: FINDING instead.
- Comment-only code edits (Phase 2.2), separate commit; both suites + clippy green, zero new
  warnings.
- Frozen records untouched except the ratified normalization exception; verbatim anchors; minimal
  diffs; when in doubt, FINDING.

## Output

1. `alpha/experiments/RUN-09-SUMMARY.md`: the Phase 0 inventory counts, per-phase FIX list,
   and the link-resolution statistics (N claims, N fully linked, N FINDINGs).
2. CONSISTENCY-FINDINGS-2026-07.md: new `## Traceability findings (RUN-09)` section, same
   HIGH/MED/LOW + proposed-resolution format as before — these get the same walk-through and a
   settlement run.
3. `beta/drystone-spec/EVIDENCE-MAP.md` as above.
