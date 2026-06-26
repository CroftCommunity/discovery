# Alpha left-behind / left-out audit (stricter eye, post-spec context)

date: 2026-06-26 · status: IN PROGRESS (resumable; one file at a time)

## Problem statement

The user was frustrated that we have been "too lax on identifying things that need to be included or
decided" — leaving real items as inline notes or as "optional/low/borderline/DEFERRED" limbo rather than
either **determining** them (folded / cited-by-design / excluded-with-reason) or **surfacing** them (an
OPEN-THREADS thread or a named decision). This audit walks **every** alpha file one at a time and forces
each to a binary: **DETERMINED** (a clear disposition) or **OPEN** (tracked as a thread/decision). Anything
that is neither — a left-behind conclusion, an un-decided deferral, a stray inline pointer — is the find.

## Approach

Baseline = the 2026-06-25 per-file coverage audit (`2026-06-25-beta-coverage-per-file-audit.md`), which
dispositioned 165 files and is trusted for FOLDED-verification. This pass adds the **stricter eye** and the
**current context** (the Drystone protocol spec built 2026-06-26; theme 01 retired → `drystone-spec`; 08
reframed on the social graph; the "surface every decision" standard). For each file: confirm the baseline
disposition still holds, then hunt specifically for (a) items the prior audit left in limbo (optional folds,
partials, DEFERRED, borderline) that the stricter standard says must be decided; (b) dispositions the
2026-06-26 work changed; (c) genuinely new files. **Decide or surface each — no limbo.**

## Disposition codes

**DETERMINED-FOLDED** · **DETERMINED-CITED** (alpha-only by design, reason) · **DETERMINED-EXCLUDED**
(reason) · **DETERMINED-PROCESS/RAW** · **OPEN→Tn** (surfaced as a thread) · **OPEN→decision** (a named
decision in README gates / BETA-ROLLUP deferred) · **FIND** (left-behind/left-out — needs action this pass).

---

## crystallized/ (7) — DONE

| file | disposition (stricter eye) | left-behind finding + action |
|---|---|---|
| conclusions.md | DETERMINED-FOLDED (04/01/03/06/07/08; M0–M4 → T12) | none. "M2 = the social graph you hold" corroborates the 08 reframe. |
| principles.md | DETERMINED-FOLDED (Tier-1 drained; Tier-2 reflected 04/06/08; Tier-3 ADR→08, three-audiences→T17, LTS→T18) | **FIND-1:** Tier-1 "user-need-first / Google+ lesson" was in limbo (prior audit "intentionally unfolded"). **Decided:** DETERMINED — subsumed by 08 §1 (composability as the user-respecting stance) + the razor (provenance-not-utility = not-data-extraction-first); recorded, not separately folded. User may override (fold a named line into 08/07). **FIND-2 (bookkeeping):** the deeper-foundation block's folds were the K-table's `01 §X` landings; 01 retired → drystone-spec this session, so those pointers are stale → fixed via a redirect note at the K-tables. |
| proof-ledger.md | DETERMINED-CITED (04 §3 results table + §5 flags derive from it) | none. |
| test-narrative.md | DETERMINED-CITED (the "what each proof means" layer behind 04 §3/§5) | none. |
| conformance-suite.md | DETERMINED-CITED (04 §3 + CROFT-PROTOCOL §12 + now `drystone-spec` Part 2 §9 conformance) | none. |
| TEST-CORPUS.md | DETERMINED-PROCESS/INDEX (cross-repo test catalog) | none. |
| CROFT-PROTOCOL.md | **disposition CHANGED this session:** was CITED→04; now DETERMINED-FOLDED → `drystone-spec` Part 2 (matured 2026-06-26) + still CITED by 04. `croft-*` tag-rename flagged in spec App-B. | none new (recorded in BETA-ROLLUP "01 review → Drystone spec" intake). |
