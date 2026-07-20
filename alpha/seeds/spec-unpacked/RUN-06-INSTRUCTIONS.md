# RUN-06 — Settle the consistency findings (rulings application)

`Branch: fresh off main (RUN-05 merged), e.g. run-06-findings-settlement. One markdown commit +
one code-touching commit (T3 only). Every task below is a ruled decision from the
CONSISTENCY-FINDINGS-2026-07 walk-through (2026-07-14) — apply, do not re-open.`

## Frozen-record rule (governs every task)

Never edit frozen records: RUN-0N summaries, past reviews-log entries, preserved historical blocks
(§7.6.11), historical changelog entries, or the bodies of bannered historical docs. Two sanctioned
exceptions only: (a) the one-line "applied" annotation on proposed-changes F4 in T2; (b) nothing
else. Where a ruled fix would touch a frozen record, skip it there and list the untouched site in
the summary.

## Tasks

### T1 — FND-1: repoint the origination-freshness citation §7.4.2 → §7.4 (living docs only)

The precondition ("caught up and corroborated-fresh to originate an add/remove/policy-change")
lives in §7.4 proper, not §7.4.2 (the MLS-recovery-hazards subsection). Fix in living docs:
- Part 2 §8.2(e): "(§7.4.2)" → "(§7.4)" in the clause "the freshness precondition on originating
  such an op (§7.4.2) is not yet exercised over live transport".
- EXPERIMENT-BACKLOG.md, the EXP-C1 entry's §8.2(e) note: same repoint.
Then grep the repo for remaining `§7.4.2` uses of this concept; frozen docs (proposed-changes,
past log entries) keep theirs as records — list every left-as-record site in the summary.

### T2 — FND-2: land the staged F4 wording in §11.11

Locate proposed-changes F4's staged §11.11 measurement-#1 wording (the "half-earned →
earned-in-shape (loopback), magnitude-open at hot-N = 500+" edit with the `fanout-single-run`
caveat) and apply it to Part 2 §11.11 exactly as staged. Sanctioned exception: change F4's
"Not yet applied to Part 2 — … pending review" line to "Applied (RUN-06)."

### T3 — FND-3: coordinated short-path repoint (the code-touching commit)

Repoint every short-form `alpha/SPEC-DIVERGENCE-REGISTER.md` (and any bare-path variant used as a
location claim) to `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`, doc and code together:
- the register's own SPEC-DELTA tag template (SPEC-DIVERGENCE-REGISTER.md, ~L19);
- the two live code tags: `croft-chat/.../src/iroh_bus.rs:360` and
  `croft-chat/.../tests/iroh_convergence.rs:66`;
- `REPO-README.md`; the two `croft-chat/plans/*` docs.
Comment/doc strings only, no behavior change. Verification: a repo-wide grep shows **zero**
remaining short-form uses outside frozen records; both suites + clippy green, zero new warnings.

### T4 — FND-4: conventions doc-method citation

In `conventions-and-decisions.md`, repoint all five `11-doc-method.md` citations to
`beta/impl/doc-writing-method.md` (relative form consistent with the file's other repo paths).
Ruling basis: that file carries the numbered "Rule N" convention Part 2 cites; the p10 seed doc is
its provenance ancestor, not the live reference.

### T5 — FND-7: revocation-authority reference

EXPERIMENT-BACKLOG.md §6d: repoint "the sibling `discovery/thinking/revocation-authority.md` (out
of this workspace)" to the in-repo `alpha/thinking/revocation-authority.md`, dropping the
out-of-workspace parenthetical. The fold-in superseded the external reference.

### T6 — FND-8: Part 1 typo (sanctioned one-word edit)

Part 1 §2.5 (~L529): "Part 2 Part 2 §7.6.1" → "Part 2 §7.6.1". This is the only Part 1 body edit
in the run.

### T7 — FND-9: two Active rows is the truth; fix the narrative

Ruling: `fanout-single-run` **stays Active** (single-run loopback magnitude is a proxy exactly as
hermetic-gossip is an environment limit; it retires on a repeated-run or hot-N measurement). The
register is unchanged. Fix every *living* doc asserting "hermetic-gossip is the only Active row" —
check MASTER-INDEX and any register/backlog framing lines; frozen summaries keep their statement
as a record. Add one line to the register's `fanout-single-run` row noting its retirement
condition (repeated-run or hot-N measurement) if not already stated.

### T8 — FND-10: shared vocabulary additions

In `conventions-and-decisions.md` A.11 (or its shared-term surface), add: **approval subject**,
**contradiction byte-head**, **horizon checkpoint**, **horizon-checkpoint manifest**,
**corroboration dials**, **quantified trust**. Each entry: one sentence derived from the Part 2
sentence that introduces the term (do not invent new phrasing) plus its home section (§7.2 R7;
§7.6.1; §7.6.9; §7.6.9/Appendix B; the corroboration-dials paragraph's section; §7.4/the
corroboration-dials paragraph). Match A.11's existing entry format exactly.

### T9 — FND-11: Part 1 back Map completeness

Add the missing Map line for Part 1's `## Upstream reference links (versioned)` back-matter
section. Whether the Map lists itself: match whatever Part 2's Map does; do not innovate.

### T10 — FND-5 and FND-6: ruled no-action

§7.6.11's preserved block stays verbatim (preserved means frozen); the Pass-2 changelog entry's
renamed fold-source stays as a provenance record. Record both rulings in the summary; touch
nothing.

### T11 — the double-status-tag polish

Part 2 §7.3.2, the F8 paragraph's closing sentence currently reads "`Design`, decided and now
test-run: …". Change the opening to "Decided by design and now test-run: …" so the sentence
carries exactly one status tag (the terminal `Modeled.`). Confirm the §7.2 R7 residuals paragraph
does not have the same double-tag shape (it should already end with a single `Modeled.`); fix
identically if it does.

## Bookkeeping

- Rule 15: update Part 2's back Map lines only where a section's description changed (§11.11 at
  minimum); `part-2-changelog.md` entry covering T1, T2, T11 (and T6 in Part 1's changelog if one
  exists, else note in the reviews-log entry).
- Reviews-and-experiments log: append `## 2026-07-14, RUN-06 findings settlement` listing each
  FND with its one-line ruling and disposition (applied / left-as-record / no-action).
- CONSISTENCY-FINDINGS-2026-07.md is a living doc: move each FND from "Needs an owner call" into a
  new "## Ruled and applied (RUN-06)" section carrying the ruling one-liner and what was done;
  keep the original finding text intact beneath it.
- MASTER-INDEX: bank RUN-06.

## Guardrails

- Verbatim-anchor rule: anchor not found exactly → skip, record, continue.
- Minimal diffs; no reflowing; no edits beyond the sites named.
- T3 is the only commit touching code; suites + clippy green, zero new warnings vs baseline.

## Output

`alpha/experiments/RUN-06-SUMMARY.md`: per-task status, before→after for every one-line change,
the T1 and T3 grep verification output, and the left-as-record site lists.
