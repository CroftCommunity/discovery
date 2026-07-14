# RUN-04 — Corroboration dials, the stamp beacon, and the beam contract (summary)

`Markdown-only design pass, 2026-07-14. Branch: claude/run-04-corroboration-dials-3f9xyk (fresh off
main at merge b576ad1). No code, no tests. Sequencing gate satisfied: RUN-03 is merged
(reconciliation-horizon.md present; EXP-H1 present in EXPERIMENT-BACKLOG.md §2b).`

## Sequencing gate

**PASS.** `alpha/thinking/reconciliation-horizon.md` exists and the **EXP-H1** backlog entry is present
(EXPERIMENT-BACKLOG.md §2b, "horizon-manifest determinism"). Both references the run cross-links resolve.
No improvisation required.

## Per-task status

| Task | Status | Notes |
|---|---|---|
| **T1** — new design note `corroboration-and-quantified-trust.md` | ✅ done | Created, voice-consistent with `reconciliation-horizon.md` / `the-shape-of-disagreement.md`. Banner (captured 2026-07-14, naming the two Part 2 Design paragraphs + EXP-C1 as the concrete landings, self-echo flagged exploratory) plus all seven sections (0 epistemic floor, 1 write side already built, 2 read side / unreferenced tail, 3 formula-valued thresholds, 4 circular assertion awareness [exploratory, two seams], 5 the beam reframed, 6 the contract experiment). |
| **T2** — Part 2 corroboration-dials paragraph (§7.3.3) | ✅ done | Inserted verbatim as a normal paragraph immediately after the load-bearing-caveat paragraph (anchor found exactly). Rendered non-blockquote; bold lead-in preserved. |
| **T3** — Part 2 formula-valued threshold paragraph (§7.4.1) | ✅ done | Inserted verbatim as the final paragraph of §7.4.1, immediately before the `#### 7.4.2` heading (anchor found exactly), after the existing §7.4.1 running-example line. |
| **T4** — backlog EXP-C1 | ✅ done | Added as new subsection **§2c** in `EXPERIMENT-BACKLOG.md`, immediately after §2b (EXP-H1) — beside EXP-H1. Four RED-able assertions verbatim; shares-boundary-machinery-with-EXP-H1 note and the §8.2(e)-residual discharge note included. |
| **T5** — Map (Rule 15) + changelog | ✅ done | §0 Map §7.3 line updated (§7.3.3 now names the corroboration dials) and §7.4 line updated (§7.4.1 now also admits the formula-valued freshness threshold). `part-2-changelog.md` entry appended in house style covering T2 and T3. |
| **T6** — reviews-and-experiments log | ✅ done | `## 2026-07-14, Corroboration dials and the beam contract (design pass)` appended to `beta/impl/experiments/drystone-reviews-and-experiments-log.md`, covering the quantified-trust framing, the stamp-as-beacon reading, the two Part 2 Design paragraphs, the self-echo exploration (two seams), and EXP-C1. |

## Placement judgments

- **Canonical file locations.** The authoritative Part 2, changelog, conventions, and reviews-log live in
  `beta/`, not `alpha/seeds/` (confirmed against RUN-03-SUMMARY.md's edited-file list). The task named these
  files by bare name (`conventions-and-decisions.md`, `part-2-changelog.md`); resolved to
  `beta/drystone-spec/part-2-certifiable-design.md`, `beta/drystone-spec/part-2-changelog.md`, and
  `beta/impl/experiments/drystone-reviews-and-experiments-log.md`. No `conventions-and-decisions.md` edit was
  needed or made (the language guardrail cites its DR block; it was read, not edited).
- **T2 section index.** The beam paragraph (the load-bearing caveat) sits in **§7.3.3**, so the Map update
  went to the §7.3 map bullet (which enumerates §7.3.3) rather than a standalone §7.3 line, matching the
  task's "§7.3 (or wherever the beam paragraph's section indexes)" instruction.
- **T3 "final paragraph of §7.4.1."** §7.4.1 currently ends with a `*Running example:*` italic line before
  the §7.4.2 heading. The new paragraph was placed after that running-example line and immediately before
  the §7.4.2 heading, honoring both "final paragraph of §7.4.1" and "immediately before the §7.4.2 heading."
- **T4 "beside EXP-H1."** EXP-H1 is backlog §2b. EXP-C1 was added as a sibling subsection §2c directly after
  it (not folded into §2b), keeping each experiment its own indexed entry, consistent with the §2a/§2b split.
- **Changelog dash style.** The `part-2-changelog.md` prose uses hyphens, not em-dashes (established house
  convention, re-confirmed in the RUN-03 entry). The new changelog section was written em-dash-free and
  double-hyphen-free (verified); the Part 2 inserts themselves use house em-dash style as required.

## Anchor misses

**None.** All three verbatim anchors were found exactly:

- T2 anchor: "The load-bearing caveat, stated rather than claimed away: a sufficiently isolated node
  **cannot** establish final state on its own" — found (part-2-certifiable-design.md §7.3.3).
- T3 anchor: `#### 7.4.1. The false-positive tolerance is a governed utility judgment, not a constant`, with
  the `#### 7.4.2` heading as the placement boundary — both found.
- T5 anchors: §0 Map bullets for §7.3 and §7.4 — found.

## Guardrail compliance

- Markdown only. No code, no tests, no Part 1 edits, no `conventions-and-decisions.md` edits.
- Minimal diffs: only the anchored insertion points and the two Map lines were touched; no untouched
  paragraph was reflowed.
- T2 rendered as a normal Part 2 paragraph (blockquote markers stripped, bold lead-in kept).
- DR language (conventions A.11): the two inserts are continuity-framed and non-moral; MUST/MAY casing and
  `Design` status-tag placement preserved.

## Full file list (this run)

Created:
- `alpha/thinking/corroboration-and-quantified-trust.md` (T1)
- `alpha/experiments/RUN-04-SUMMARY.md` (this file)

Modified:
- `beta/drystone-spec/part-2-certifiable-design.md` (T2 §7.3.3 paragraph; T3 §7.4.1 paragraph; T5 §0 Map §7.3 and §7.4 lines)
- `alpha/experiments/EXPERIMENT-BACKLOG.md` (T4 §2c EXP-C1)
- `beta/drystone-spec/part-2-changelog.md` (T5 changelog entry)
- `beta/impl/experiments/drystone-reviews-and-experiments-log.md` (T6 log entry)
