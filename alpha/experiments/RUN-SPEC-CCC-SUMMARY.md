# RUN-SPEC-CCC — Part 1/2 currency + consistency/clarity/correctness pass

`Run summary, 2026-07-20. Branch run-spec-ccc-2026-07-20, off main. Docs-only (no code, no TDD-code
cycle; the no-unverified-claims rule holds). Scope decided 2026-07-20: CORE-ONLY — bring the two spec
documents (Part 1, Part 2) and their direct support docs current, correct, clear, consistent. Lane
graduation (ROADMAP A19) and primary-source re-verification (the CHANGELOG publication checklist) were
out of scope. Companion findings: CONSISTENCY-FINDINGS-2026-07.md, "## RUN-SPEC-CCC (2026-07-20)" section.`

## Headline

The currency delta was **essentially empty**, so this was a near-pure CCC pass. The last core-touching
run was **RUN-12** (fully recorded in the Part 2 changelog); everything since leaves
`beta/drystone-spec/part-*` untouched. The pass surfaced **two mechanical FIXes** and **one LOW FINDING**.
The marquee FIX is a genuine defect the two prior consistency passes (RUN-05/06/07) miscleared: a
line-split doubled cross-reference in Part 1 §2.5 that renders as "Part 2 Part 2 §7.6.1" on the published
site, invisible to single-line grep. It was confirmed by rendering Part 1 to HTML before the fix.

## Phase 0 — the delta ledger (hard gate, no edits)

Read-only survey of everything that could make Part 1/2 non-current:

| Source | Finding |
|---|---|
| `proposed-changes-2026-07-experiment-reconciliation.md` | F1–F8 all landed (RUN-02/RUN-03). F4's fan-out follow-on — the brief's anticipated "marquee" currency edit — **already landed** (RUN-06) and was then **sharpened by RUN-09** (K=5 replication, `fanout-single-run` retired). Nothing to apply. The doc's *header banner*, however, was stale (said F4 "remains staged, not landed") — see CCC-2. Its RUN-14/15/16/18/19 addenda target the design tree (`social-mapping.md`, `GROUPS.md`), out of scope. |
| `EVIDENCE-MAP.md` | Current through RUN-12; FND-T1/T2/T3/T4 settled or recorded; defers to the Part 2 sentence as source of truth. No earned tag found ahead of its spec sentence except the arguable §11.11 #1 (FND-CCC-1). |
| `CONSISTENCY-FINDINGS-2026-07.md` / `SPEC-ALIGNMENT-AND-ACTION-PLAN.md` | No open item lands in Part 1/2 requiring an edit. Alignment doc is correctly bannered point-in-time. |
| Changelogs + post-RUN-12 summaries | Part 2 changelog's last spec-touching entry is RUN-12. RUN-13 = site-tooling only. RUN-14…RUN-19 and every lane run (RUN-AP-01, RUN-ATTEST-01…04, RUN-HIST-01, RUN-LEX-01, RUN-FED-SHIM-01) declare `beta/drystone-spec/part-*` untouched, staging any spec-facing implications in their own registers / the proposed-changes doc. Confirmed; no exception. |

**Gate result:** delta empty. Proceeded to a pure CCC pass. (Greps sanity-checked against known-present
terms per the SIGPIPE-false-zero caution; no "not found" was trusted blind.)

## Phase 1 — currency (named conditional edits only)

No Part 1/2 body currency edit was owed (the anticipated F4 §11.11 edit is long since landed). One
support-doc currency FIX applied:

- **CCC-2** — `proposed-changes-2026-07-experiment-reconciliation.md` top banner: corrected the stale
  "F4 remains staged, not landed / §11.11 still reads *half-earned*" to record that F4's follow-on landed
  (RUN-06, *half-earned → earned in shape*) and was sharpened (RUN-09, *magnitude replicated at loopback,
  open at hardware scale*; `fanout-single-run` retired). Aligns the banner to its own F4 section, the Part 2
  changelog, the RUN-06 FND-2 disposition, and the live §11.11 text. No grade moved.

## Phase 2 — CCC (FIX vs FINDING)

### FIX applied

- **CCC-1** — `part-1-reasoning-underpinnings.md` §2.5 (fork-not-verdict ¶): removed a duplicate "Part 2"
  token. Source had "Part 2" ending one wrapped line and "Part 2 §7.6.1" beginning the next; markdown
  paragraph flow joined them into a visible "Part 2 Part 2 §7.6.1" on the site. **Rendered Part 1 to HTML
  to confirm** (reader saw "too few; Part 2 Part 2 §7.6.1 enumerates both"), applied the fix, re-rendered
  clean. Overturns RUN-05 FND-8's RUN-06 "no-op" and RUN-07 "false positive" dispositions, which trusted a
  single-line grep that cannot see a newline-straddling duplication. Defect entered document-pass-6
  (`d136868`, 2026-07-06). A markdown-aware doubled-word sweep over Part 1/2 + support docs found no other
  line-split duplication ("Group Group Role" in Part 2 is intentional term usage). Part-1-changelog entry
  added; no §0-map change.

### FINDING recorded (owner call, not edited)

- **FND-CCC-1 (LOW)** — §11.11 measurement #1: Part 2 tags it `Load-bearing, unearned` (the claim, at
  representative-hardware hot-N ≥ 500), EVIDENCE-MAP row 70 tags it `Measured` (the loopback evidence that
  exists). Reconcilable as evidence-rung vs claim-status and the map's bounds column scopes it, but a strict
  map↔spec equality check flags it. Proposed: confirm the intended two-angle layering (and optionally note
  it on row 70) or reconcile the tokens. Likely intended (set by RUN-08/RUN-09); flagged per the
  record-arguable-grades rule.

### Verified clean (full list in the findings doc)

Cross-reference integrity (site gate, 0 hard-gated), grade/evidence coherence (all load-bearing tags match
their map rows except FND-CCC-1; all tokens on-ladder; no off-ladder live tag), terminology/DR compliance
(`real-NAT` / `byte-head` / `hard-stop` canonical, no doubled prefixes remain), §0-map ↔ structure (1191
headings anchored, no desync; §7.6 / §6.8 map entries current), Part 1 → Part 2 direction (§2.5 supports
the §7.3.2 / §7.6.1 citations; no Part 1 assertion contradicts landed Part 2).

## Site gate output (recorded)

`site/build.py` run with the mermaid **renderer** stubbed (no-op SVG; the pinned mermaid-cli needs a
headless browser this environment blocks — the broken-ref/anchor resolver runs first regardless, per the
brief). Post-edit:

```
documents built            : 87  (8 spec, 2 gradients, 11 classroom, 66 exploratory)
headings anchored          : 1191
§-references found          : 3195
  resolved -> links         : 3102
  external (RFC/BCP) literal : 86
  skipped in code spans      : 84
  unresolved                 : 7  (hard-gated 0, companion 7)
repo-path citation links   : 20
mermaid diagrams rendered  : 2 (stubbed; parse gate not run)
gate OK (check-only; no site written)
```

Stages that ran: broken-ref gate (0 hard-gated), anchor-alignment (no desync), companion-allowlist gate
(matches exactly, no drift). Stage not run: mermaid parse/render (renderer unavailable in-env; 2 blocks
stubbed). The pure-stdlib resolver unit tests pass 28/29 (the one error is only that a single test imports
the absent `markdown` module).

## Definition-of-green confirmation

Phase 0 delta ledger produced (empty, recorded). Phase 1 currency: no Part 1/2 body edit owed; one
support-doc banner FIX. Phase 2 CCC complete with FIX-vs-FINDING discipline (2 FIXes, 1 FINDING). Findings
doc written (appended to CONSISTENCY-FINDINGS-2026-07.md). Part-1 changelog updated; §0-maps reconciled
(no change needed). Site-gate output recorded above.

**No open item was closed and no grade was laundered.** The load-bearing beam, the `[gates-release]`
byte-level encodings, the BLAKE3 §4 re-proof, and the firewall items (I9 trust-tier, X1 real-NAT, the
recovery trust-predicate) are untouched and remain open. Scope stayed inside `beta/drystone-spec/` Part 1/2
+ support docs; A19 (lane graduation) and primary-source re-verification remain untouched.
