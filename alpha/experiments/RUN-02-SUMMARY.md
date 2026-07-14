# RUN-02 summary — 2026-07 experiment reconciliation landed into the spec set

`Run: RUN-02, 2026-07-13. Scope: markdown surgery only (no code, no tests, no register-row
deletions). Branch: claude/run-02-spec-reconciliation-0fqfxw.`

Landed the reviewed diff set `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md`
(items F1 through F7, plus the new F8) into Part 2 and the bookkeeping docs, encoding the owner's
calls. No anchor was missed; every anchor text in the instructions was found verbatim. One location
discrepancy is noted below (the Map).

## Per-task status

| Task | Status | Notes |
|---|---|---|
| **T1** — amend F1 in the staged-diff doc | **done** | Replaced the R7 diff block with the amended block; added the residuals block with the verbatim lead-in; appended the "Called 2026-07-13" line under **The call for you.**; changed the F1 header and summary-table row from `needs-call` to `called`. |
| **T2** — apply F1 to Part 2 | **done** | R7 bullet inserted after R6 (R1–R6 bullet style); residuals paragraph inserted after the "R3 and R6…" closing paragraph; §8.2(e) diff applied; §7.3 cross-reference sentence added (see T2c placement). |
| **T3** — record the two-competing-quorums decision (F8) | **done** | §7.3.2 boundary paragraph extended (T3a); §7.6.1 contradiction enumeration extended (T3b); F8 entry + summary-table row added to the staged-diff doc (T3c). |
| **T4** — apply F2–F6 | **done** | F2 (§7.6.2), F3 (§6.8.1), F4 (§11.11 #1), F5 (§8.2(a)) applied verbatim. F6 verified as a deliberate non-change: §7.6.3's ReInit-stranding `[confirm]` stands, no Part 2 edit. |
| **T5** — apply F7 (footnote) | **done** | Footnote added immediately beneath the §10.5 realization-ledger table; F7 marked `called` (header, summary row, and call block answered). See T5 placement. |
| **T6** — Rule 15 Map + changelogs | **done** | Front `## 0. Map` entries updated for §6, §7.1/§7.2, §7.3, §7.6, §8, §10, §11; `part-2-changelog.md` pass entry appended in house style. See Map-location note. |
| **T7** — reviews-and-experiments log entry | **done** | New section `## 2026-07-13, Real-substrate spikes, reconciliation landing` appended, mirroring the v2 entry: what ran, the three reconciled deltas, the one active (`hermetic-gossip`), and the spec effects. |
| **T8** — registers + alignment doc | **done** | (a) SPEC-DIVERGENCE-REGISTER: verified rows correct; added "Spec landing: §7.2 R7 (RUN-02)." to the rulechange-quorum reconciled row's evidence cell. (b) SPEC-ALIGNMENT §7: all five open decisions annotated in place with a bold **Decided (RUN-02, …)** lead. (c) EXPERIMENT-BACKLOG: two-competing-quorums item carries the decided behavior; croft-group L2–L5 note carries the reuse decision and compatibility condition. |

## Placement judgments made

- **Map location (Rule 15).** The instructions state the `## 0. Map` "lives after Appendix F." Part 2
  has exactly one `## 0. Map`, and it lives at the **front** (line 7); there is no back map after
  Appendix F (the only other map-titled block is the §11-scoped `### 0. Map` at level 3, inside §11).
  The front `## 0. Map` is the Rule-15 per-section index, so it is the one updated. This is a location
  discrepancy in the instructions, not an anchor miss: the map exists and was updated.

- **T2c (§7.3 cross-reference).** Placed the sentence "Threshold enforcement for policy changes,
  including the approval subject and the prior-rule-governs semantics, is specified at §7.2 R7." as the
  last sentence of the §7.3.1 **authorization-precondition** paragraph, immediately before its closing
  `Design.` tag. That paragraph is where the k-of-n quorum / standing-at-causal-position semantics live,
  so the cross-reference sits with the local prose it qualifies.

- **T3a (§7.3.2 boundary).** Added the two-competing-quorums text as a **new paragraph** immediately
  after the "Boundary with the §7.6 hard-stop" paragraph (before the running-example line), rather than
  inlined into that paragraph. It opens "One case is decided on the escalation side of this line…",
  which reads as a follow-on to the just-stated line; a new paragraph keeps the boundary paragraph's
  topical bound and matches house style.

- **T3b (§7.6.1 note).** A natural insertion point existed: the "**Contradiction: too many valid
  claims.**" bullet already enumerates contradiction shapes (mutual removal; removed-then-included
  merge). Added the R7 quorum-collision as a **third example** in that same bullet, cross-referenced to
  §7.3.2. No separate note in §7.6 body was needed.

- **T4 / F4 (§11.11 measurement #1).** Measurement #1 is a single flowing paragraph whose tail is the
  public-regime *Extension* sentence; the staged diff's context was abbreviated (ended at "versus
  live-N.") and elided that Extension. Placed the *Partial (2026-07)* note as a **new 4-space-indented
  continuation sub-paragraph at the end of measurement #1** (after the Extension), matching the
  sub-paragraph style already used inside item 3 of the same list. This keeps the status note attached
  to measurement #1 without splitting the main sentence from its Extension.

- **T5 (F7 footnote).** The sole "realization ledger" table in §10 is the §10.5
  dependency-versus-realization ledger. Placed the conformance-vector footnote **immediately beneath
  that table**, between the table and its "single sentence for this section" summary. **No ledger row
  unambiguously covers the conformance vectors** (the rows are substrate components: group key
  agreement, transport, signature, hash, capability, data model), so no row was additionally annotated,
  per the T5 conditional.

## Guardrails honored

- Markdown only. No Part 1 edit, no Rust, no test, no file under `alpha/experiments/*/src`, no
  register-row deletion.
- House style matched: Part 2 inserts use the section's em-dash / `→` / `↔` usage and R1–R6 bullet
  shape and status-tag placement; the changelog and reviews-log additions keep those files' em-dash-free
  prose convention.
- Minimal diffs: no untouched paragraphs reflowed.

## Files changed

1. `beta/drystone-spec/proposed-changes-2026-07-experiment-reconciliation.md` — F1 amended (R7 block,
   residuals, call answered, header/table → `called`); F7 answered and → `called`; F8 added (entry +
   summary row); landing plan updated.
2. `beta/drystone-spec/part-2-certifiable-design.md` — §7.2 R7 + residuals; §8.2(a) and §8.2(e); §7.3.1
   cross-reference; §7.3.2 two-competing-quorums paragraph; §7.6.1 contradiction example; §7.6.2 re-plant
   corroboration; §6.8.1 RBSR annotation; §11.11 #1 partial note; §10.5 conformance footnote; front
   `## 0. Map` entries (§6, §7.1/§7.2, §7.3, §7.6, §8, §10, §11).
3. `beta/drystone-spec/part-2-changelog.md` — "Pass: 2026-07 real-substrate experiment reconciliation".
4. `beta/impl/experiments/drystone-reviews-and-experiments-log.md` — 2026-07-13 reconciliation-landing
   section.
5. `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md` — rulechange-quorum evidence cell (spec landing).
6. `alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md` — §7 open decisions 1–5 annotated.
7. `alpha/experiments/EXPERIMENT-BACKLOG.md` — two-competing-quorums item; croft-group L2–L5 reuse note.
8. `alpha/experiments/RUN-02-SUMMARY.md` — this file.
