# RUN-SPEC-CCC — Part 1/2 currency + consistency/clarity/correctness pass

`Comprehensive run brief. Copy into a fresh session after a /clear. Scope decided 2026-07-20:
CORE-ONLY — bring the two spec documents current, correct, clear, and consistent. Reaching past
the core (graduating alpha lanes, or folding lane results into Part 1/2) is PARKED as ROADMAP A19
and is explicitly out of scope here. Primary-source re-verification (the CHANGELOG publication
checklist) is a SEPARATE later run and is out of scope here.`

## 0. What this is, and what it is not

Honest framing so the run is sized right: the original experiment→spec reconciliation (proposed-
changes F1–F8) **already landed** (RUN-02/03), and RUN-05 was a consistency pass with RUN-06
settling its findings. So this is **not** a large backlog of un-applied changes. It is:

1. a **currency** step — discover and apply whatever *small* Part-1/2-affecting delta has accrued
   since the last spec-touching run (the one known staged-not-landed item is F4's fan-out
   re-measurement; there may be a few earned tag-moves or drift points), and
2. a fresh **consistency / clarity / correctness (CCC)** pass over Part 1/2 given everything that
   has changed in the registers and lanes since RUN-05.

If Phase 0 finds the currency delta is essentially empty, say so plainly and let the run be a pure
CCC pass. Do not manufacture reconciliation work to look busy.

**In scope:** `beta/drystone-spec/` — `part-1-reasoning-underpinnings.md`,
`part-2-certifiable-design.md`, and their direct support docs (`conventions-and-decisions.md`,
`EVIDENCE-MAP.md`, `open-threads.md`, the two changelogs, `reference-index.md`,
`dag-cbor-and-content-addressing.md`, `proposed-changes-2026-07-experiment-reconciliation.md`).

**Out of scope (do not touch):** the design tree `beta/impl/drystone-design/`; the alpha lanes and
their registers except as *read-only evidence sources*; graduating any lane (A19); primary-source
re-verification; any crate/code (this is a docs-only run).

## 1. Standing discipline (binding)

- **Frozen-record + named-conditional-edit rule.** Part 1/2 are not rewritten at will. Every
  content change is either (a) a mechanical CCC fix that preserves meaning, or (b) a *named
  conditional edit* justified by a specific evidence pointer (a landed RUN result, an EVIDENCE-MAP
  row, a proposed-changes item). No unnamed content moves.
- **Grade honesty — no laundering.** Loopback/`Modeled` stays loopback/`Modeled`; `green-real`
  stays scoped to what ran; nothing becomes `Verified` without evidence that genuinely reaches that
  rung. Follow the A.9 evidence ladder and the grade vocabulary already in the spec
  (`Verified` / `green-real` / `Modeled` / `Design` / `Synthesis`).
- **The open items STAY open.** Part 2's own banner and `open-threads.md` name them: the
  load-bearing completeness beam, the `[gates-release]` byte-level encodings, the BLAKE3 §4
  re-proof, plus the standing firewall items (I9 trust-tier, X1 real-NAT, the recovery
  trust-predicate). This pass MUST NOT close, soften, or quietly resolve any of them. If the text
  is unclear *that* they are open, clarify toward more-open, never less.
- **Convention compliance.** Honor `conventions-and-decisions.md` — the DR language rules, the
  MUST/SHOULD/MAY (BCP 14) usage, the §0-map (Rule 15) obligation, and the changelog rule (Rule
  13): every touched section gets its §0-map line checked and a changelog entry.
- **FIX vs FINDING (the RUN-05 contract).** A **FIX** is mechanical, provable, meaning-preserving —
  apply it inline. A **FINDING** is anything requiring a judgment call or that would change a
  claim's meaning/grade — record it in the findings doc (Phase 3), do NOT edit. When in doubt,
  FINDING.
- **No completion claims without evidence.** Run the site gate and state its real output; do not
  assert "consistent" without having done the cross-reference/map/tag checks that prove it.

## 2. Phase 0 — discover the delta (hard gate, no edits)

Build a **delta ledger** (a scratch doc or the run-summary's opening section) of everything that
could make Part 1/2 non-current, by reading — not editing:

- `proposed-changes-2026-07-experiment-reconciliation.md` — confirm F1–F8 landed; capture any item
  still marked staged/not-landed (F4's fan-out re-measurement is the known one) and the exact §
  it targets.
- `EVIDENCE-MAP.md` — any earned status-tag that the spec text does not yet reflect (a row whose
  grade is ahead of the sentence it maps to).
- `CONSISTENCY-FINDINGS-2026-07.md` and `SPEC-ALIGNMENT-AND-ACTION-PLAN.md` — any FND/action item
  still open that lands in Part 1/2.
- `part-1-changelog.md` / `part-2-changelog.md` — identify the last spec-touching run; then scan
  the `RUN-*-SUMMARY.md` files *after* that point for any that claim a Part 1/2 edit or a tag-move
  the spec should carry. Most recent lanes (ATTEST/HIST/AP/LEX/CONTACT/GROUPS) stage in their own
  registers and design docs and do **not** touch the core — confirm that, and list any exception.

**Gate:** the delta ledger is the Phase 1 work-list. If it is empty or trivial, record that and
proceed to Phase 2 as a pure CCC pass. Use clean direct greps and sanity-check the tool against a
known-present term before trusting any "not found" (SIGPIPE under the command proxy has produced
false zeroes here).

## 3. Phase 1 — currency (apply the delta; named conditional edits only)

For each ledger item, apply the smallest faithful edit to Part 1/2 at its honest grade, with a
changelog entry and a one-line justification citing the evidence pointer. The F4 follow-on
(§11.11 fan-out: *half-earned* → *earned in shape (loopback), magnitude-open at hot-N ≈ 500+*) is
the expected marquee edit — apply it exactly as the proposed-changes doc stages it, no further.
Anything that would move a grade *up* without ladder-satisfying evidence is a FINDING, not a FIX.

Each touched section: update its §0-map line if meaning changed; add the changelog entry (house
style — no em-dashes in changelog prose per the existing convention).

## 4. Phase 2 — consistency / clarity / correctness (the RUN-05 method)

Over Part 1 and Part 2, FIX-vs-FINDING throughout:

- **Cross-reference integrity.** Every §N / §N.N / R-rule / Appendix reference resolves to an
  existing target; every Part-1↔Part-2 citation resolves. Fix numbering drift (FIX). A citation
  whose *target no longer supports the citing sentence* is a FINDING.
- **§0-map ↔ structure.** The back/front map matches the actual section tree, including any
  Phase-1 edits. FIX.
- **EVIDENCE-MAP ↔ spec coherence.** Every load-bearing claim's grade tag matches its EVIDENCE-MAP
  row and the registers; flag any tag whose correct value is arguable as a FINDING.
- **Terminology / DR compliance.** One term per concept; the DR language rules honored; no lane
  vocabulary leaking into the normative core. Pure synonym drift → FIX toward the first-introduced
  form; meaning-bearing divergence → FINDING.
- **Part 1 → Part 2 direction.** Where Part 1 characterizes a mechanism Part 2 has refined
  (policy/quorum, contradiction/escalation, recovery, freshness), confirm Part 1 asserts nothing
  Part 2 now contradicts. Tension → FINDING with both passages quoted; do not edit Part 1's body to
  resolve a substantive tension unilaterally.
- **Clarity.** Only where a sentence is genuinely ambiguous or a claim's grade is unclear — tighten
  toward precision and toward more-open on the open items. Never restyle prose for taste; never
  reflow untouched paragraphs.
- **Typos / grammar.** Meaning-preserving only; list each in the summary.

## 5. Phase 3 — findings, changelog, gate, summary

- `CONSISTENCY-FINDINGS-<date>.md` (new, or append to the existing one) with three sections:
  **Fixed mechanically** (the full FIX list), **Needs an owner call** (each FINDING with
  severity HIGH/MED/LOW, the quoted text, and a proposed resolution), **Verified clean** (each
  check that passed, stated, so silence is never ambiguous).
- Changelog entries for every Phase-1/Phase-2 content edit.
- Run the site gate (`site/build.py`: broken-ref + anchor + mermaid) and record its real output;
  green is part of done. (If the mermaid renderer's headless browser is unavailable in-env, the
  broken-ref/anchor resolver still runs first — record which stages ran.)
- `RUN-SPEC-CCC-SUMMARY.md`: the Phase-0 delta ledger, per-phase FIX list (file, section,
  before→after for one-liners), the findings hand-off, the site-gate output, and an explicit
  confirmation that no open item was closed and no grade was laundered.

## 6. Execution model

Fresh branch off `main`; docs-only (no code, so no TDD-code cycle, but the no-unverified-claims
rule holds). Land via the house flow (PR into `main`), matching prior runs. Do NOT push or merge
until the owner asks, unless the session's standing instruction says otherwise.

## 7. Definition of green

Phase 0 delta ledger produced; Phase 1 currency edits applied as named conditional edits at honest
grades (or "delta empty" recorded); Phase 2 CCC complete with FIX-vs-FINDING discipline; findings
doc written; changelogs updated; §0-maps reconciled; site-gate output recorded; summary confirms
no open item closed, no grade laundered, and scope stayed inside `beta/drystone-spec/` Part 1/2 +
support docs. A19 (lane graduation) and primary-source re-verification remain untouched.
