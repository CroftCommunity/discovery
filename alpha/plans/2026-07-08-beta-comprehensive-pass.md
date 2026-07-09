# Beta comprehensive pass — coverage recovery, per-layer source-of-record, quote/resolution remediation

date: 2026-07-08

status: PLAN (not started; the quote/resolution tie-breaker and one exemplar are landed — see Phase 3).

## Problem Statement

Reading the beta layers surfaced three defects. They are distinct in kind and in severity, but they
share a root — the layer-cake re-file (2026-07-07) prioritized *placement* (every conclusion has one
home) over *completeness and craft* (every conclusion is actually present, and every doc reads at the
standard the beta template sets). The three:

1. **Craft drift (found, partly fixed).** Many source-grounded docs collapse source quotations into
   narrative prose — either woven into a sentence as a bare fragment or restated as silent paraphrase
   with only a citation appended — which contradicts `beta/README.md` §4 ("preserved whole as block
   quotes ... never paraphrased away") and `impl/doc-writing-method.md` §103 (two permitted forms:
   direct quote, or direct quote + labeled synthesis). Separately, some docs are the *compressed* rung
   of a resolution ladder (`doc-writing-method.md` §16: elevator / coffee-shop / library) whose full
   rung is neither labeled nor linked, so a reader hits "this is the compressed version" with no path
   to the full one. And in places the corpus reads as over-summarized: a doc is the summary of a source
   that is not itself present at full grain anywhere in-tier.

2. **Coverage gaps (the user's stronger concern; unverified).** Load-bearing conclusions and needed
   context that live in the alpha corpus — and especially in the raw transcripts — appear never to have
   been surfaced into any layer doc. `../LAYER-ROLLUP.md`'s conclusion-coverage gate reads **PASS**, but
   that gate verifies that *enumerated theme conclusions* are placed; it does not prove the enumeration
   was **exhaustive**. The concern is precisely about the enumeration's completeness, which the existing
   gate cannot answer.

3. **No per-layer source-of-record (the user's request).** Each layer cites thinkers, books, papers,
   RFCs, and sites inline, but only `activism/` has a consolidated reference index. The basis of each
   layer should exist as a complete, checkable set — one reference file per layer. For `cairn/`, that
   "reference file" is a **projects directory** (the ecosystem register), not a bibliography, matching
   its role (`LAYERS.md`: "cairn catalogues the stones; drystone builds the wall").

## Approach

Four phases. Phase 0 is discovery-only and gates the rest: you cannot remediate, index, or recover what
you have not inventoried.

### Phase 0 — Inventory (discovery, no doc edits)

- **Citation inventory.** Extract every inline attribution (author, work, year, RFC/DOI, URL) from every
  layer doc, grouped per layer. Mechanical (grep + read). Output: a raw per-layer source list, the seed
  for the Phase 2 reference files.
- **Conclusion inventory (the load-bearing part).** Re-read the **raw transcripts** (not the alpha
  synthesis, which is what the existing gate trusted) and enumerate load-bearing conclusions and
  needed context. Tag each: **(a)** present in a beta layer doc, **(b)** present only in alpha synthesis,
  **(c)** present only in a transcript / never pulled up. The (b) and (c) sets are the coverage-gap
  ledger. Output: `plans/2026-07-08-beta-coverage-gap-ledger.md`.
- This phase is the natural fan-out: one reader per transcript (or per layer), each returning a structured
  conclusion list, synthesized into the ledger. Same-model subagents (see the global subagent-model rule).

### Phase 1 — Coverage recovery

For each (b)/(c) gap judged load-bearing, choose the target layer + register per `LAYERS.md` register
discipline, and draft the surfacing into the correct layer. Then **correct `../LAYER-ROLLUP.md`'s
coverage gate honestly** — it currently over-claims PASS; it should record what the exhaustive re-read
found and re-derive the verdict.

### Phase 2 — Per-layer source-of-record

One `reference-index.md` per layer, modeled on `activism/reference-index.md`: sources grouped by type
(papers / books / RFCs & specs / reports / sites), each with its epistemic-status tag, a
PRIMARY/PRIMARY-VENUE/SECONDARY/NOT-YET-PULLED marker, and a resolvable locator (DOI, URL, RFC §).
Cross-check each citation against the inline claim it grounds — this catches the "cited but only
paraphrased" cases that Phase 3 also touches, so Phases 2 and 3 are done together per doc in the second
sub-pass. **`cairn/` is the exception:** it gets a **projects register** (formalize the existing
per-project docs into an indexed `projects/` set or a projects table — one row per ecosystem project with
status, what-we-credit, and what-bubbles-into-the-spec), not a bibliography.

### Phase 3 — Quote & resolution remediation sweep

Apply the `LAYERS.md` "Quote discipline and resolution labels" tie-breaker across all source-grounded
docs. The shape is already proven on the exemplar (`philosophy/commensurability-and-the-two-ledgers.md`):
elevate genuine quotations to standing block quotes each keeping its per-quote `[UNVERIFIED]` flag;
demote coined term-fragments from quote marks to italic terms of art; add a `Resolution:` label and
rung links to every ladder doc; and name every "compressed version" companion by a resolvable pointer,
not a prose title. **Honesty constraint:** AI-surfaced material is never upgraded to a clean verbatim
quote — if no verbatim exists, the source's position is presented as labeled synthesis.

### Phase 4 — Current-state decisions register (paired with reasoning)

A beta-level `DECISIONS.md`: a scannable current-state register of every decision — both **settled**
(pulled from `CLOSED-THREADS.md` and the layer docs) and **open gates** (the README "Standing decisions"
and `ROADMAP_TODO.md` §A) — where each row states the decision at pitch resolution and **links down to
its full reasoning at library resolution** in the layer doc that grounds it. It is a *pitch-resolution
index over the reasoning*, never a replacement for it (the `LAYERS.md` "reasoning travels with the
decision" rule). This is the user's "current-state doc with decisions," built so it cannot repeat the
rollup's mistake: a decision may not enter the register unless its library-resolution reasoning exists and
is linked, and that reasoning matures forward with the decision. Depends on Phase 1 (recovery restores the
missing reasoning the register must point at) and reinforces Phase 3 (the resolution-ladder discipline).

## Sequencing and Reasoning

- **Phase 0 first** because everything downstream is a function of the two inventories. The conclusion
  inventory in particular must re-derive conclusions from transcripts rather than trust the theme-
  conclusion list, since the completeness of that list is exactly what is in doubt.
- **Coverage recovery (1) before reference files (2)** because newly surfaced content brings new
  citations; indexing first would index a moving target.
- **Reference files (2) and the quote sweep (3) share a per-doc cross-check** (does each citation
  correspond to a correctly-rendered quote or a labeled synthesis?), so they run together doc-by-doc.
- The per-layer source-of-record is also the natural home for the verification flags now scattered
  inline, and it front-loads the rc/publish hardening (flags resolve as facts harden). Modeling on
  `activism/reference-index.md` avoids inventing a format. `cairn`-as-projects-directory matches its
  stated role in `LAYERS.md`.
- The existing coverage gate's PASS is not wrong, it is **narrow**: it answers a different question
  (are the enumerated conclusions placed?) than the one being asked (was the enumeration complete?).
  The plan does not discard it; it widens it.

## Scope / non-goals

- **Alpha corpus content stays frozen** (PLAYBOOK §4). Only additive artifacts (this plan, the gap
  ledger, the corrected rollup gate) touch alpha.
- **FACTCHECK facts are not re-verified** (atproto / iroh / iOS — cite the SoT).
- **The user's standing decision gates are surfaced, not resolved** (README "Standing decisions").

## Open scoping questions (the user's calls — surface, don't resolve)

- **Coverage-recovery depth:** exhaustive re-read of every raw transcript, or targeted (only transcripts
  flagged as under-harvested by the Phase-0 first pass)? Exhaustive is the honest answer to the stated
  concern; targeted is cheaper. Recommendation: exhaustive, fanned out.
- **Reference-file granularity:** one file per layer (recommended, matches `activism/`), or one
  beta-wide bibliography with per-layer sections?
- **cairn register form:** an indexed `cairn/projects/` directory (one doc per project, plus an index),
  or a single projects-table `reference-index.md`? The existing per-project docs favor the former.

## Progress

- 2026-07-08 — Tie-breaker codified in `beta/LAYERS.md` ("Quote discipline and resolution labels").
- 2026-07-08 — Phase 3 exemplar landed: `philosophy/commensurability-and-the-two-ledgers.md` (four
  quotations elevated to standing block quotes; Polanyi/Jacobs term-fragments demoted to italics;
  resolution label added). Awaiting user reaction before the full sweep.
- 2026-07-08 — **Phase 0 COMPLETE.** Citation inventory (raw): `2026-07-08-beta-citation-inventory-raw.md`.
  Conclusion inventory (10-cluster transcript fan-out): `2026-07-08-beta-coverage-gap-ledger.md` — the
  rollup gate was NOT exhaustive; mature layers carried cleanly, outer/late layers + all of `ECOSYSTEM.md`
  have load-bearing gaps. `LAYER-ROLLUP.md` gate widened honestly (line 43–44 correction). The Phase-1
  recovery work-list is organized by target layer in the ledger. Next: Phase 1 (recovery) — user's
  depth/sequencing call.
- 2026-07-08 — **Phase 1 started (biggest hole first): ECOSYSTEM.md → cairn, staged in OPEN-THREADS.**
  Eight recovery threads staged in `beta/OPEN-THREADS.md` (T41–T47 cairn cohort + T48 governance), each a
  proposed cairn/governance doc with target layer + gates + alpha provenance — content not yet promoted
  into settled layer docs (per tier discipline). Covers the un-carried `ECOSYSTEM.md` §1–§6 (cairn) and §8
  (governance); §9 already carried, §7 partial.
- 2026-07-08 — **Phase 4 added on user steer:** "current-state doc with decisions, but the reasoning must
  mature with it or we can't bring folks along." Codified as the `LAYERS.md` "reasoning travels with the
  decision (anti-rollup rule)" and the `DECISIONS.md` deliverable (backlog C11).
- 2026-07-08 — **Phase 1 promotion round done (ECOSYSTEM.md → cairn/governance).** All eight staged threads
  (T41–T48) promoted into real layer docs — 7 new cairn docs + 1 governance doc — written to beta maturity
  (reasoning carried whole per the anti-rollup rule; verification flags inline; user-decision gates
  surfaced, not resolved). Verified clean of prior-tier references; indexed in the cairn/governance
  READMEs; promotion map recorded in `OPEN-THREADS.md`. Awaiting user review before the threads move to
  `CLOSED-THREADS.md`. Residual: reconcile the T47 Keyhive pointer in `willow-meadowcap.md`.
- 2026-07-08 — **Phase 1 recovery round 2 done (the outer/late layers).** 12 new docs written to beta
  maturity across philosophy (cybernetic-failure, proof-of-personhood, peer-rights-razor-lineage), history
  (enclosure-inversion present+global), croft (presence-ritual + composed ponds), socialization
  (adoption-tactics, durable-product/composability), fenced (app-store/Rave-trap, aggregation-theory
  shield), and impl (redb contract, iOS/BLE, four-property tension). All verified clean of prior-tier refs;
  quotes standing alone; FACTCHECK-refuted material dropped (peer-rights); FACTCHECK SoT used for iOS facts.
  Indexed in all six layer READMEs.
- 2026-07-08 — **Consolidation principle codified on user steer:** "alpha had duplicate thinking from
  context-setting on each dive; beta pulls it together WHOLE but cleaned up and clear." Added to `LAYERS.md`
  as "Consolidate duplicate thinking into one whole (the beta synthesis job)" — consolidate don't lift;
  whole not lossy; a single alpha pointer is a starting point not the boundary. **Follow-up (backlog C12):**
  a consolidation-completeness sweep over the recovered docs, since the recovery agents were pointed at the
  ledger-identified homes (one pointer each) rather than the full prior-tier scatter.
- 2026-07-08 — **C12 (consolidation-completeness sweep) DONE.** 10-agent fan-out over the 20 recovery docs;
  16 gained fold-ins, 4 already complete. Caught a thesis-correctness fix (fenced/app-store-survivability
  had presented the raw central-control playbook the alpha synthesis *rejects*), added Jo Freeman as the
  Princeps Problem's scholarly root, the CRDT/Automerge leg of substrate-prior-art, the full FREEK cost
  curve, and more. All clean; one-home respected (cross-ref not import); 494+/148-.
- 2026-07-08 — **Phase 2 (per-layer source-of-record) DONE (backlog C9).** All 10 layers now carry a
  `reference-index.md` (cairn's is a ~95-entry projects register). Modeled on `activism/reference-index.md`;
  every source grouped by type with epistemic tag + PRIMARY/SECONDARY marker + resolvable locator + the doc
  it grounds, cross-checked against use. The indexing surfaced two defects → tracked as **C13** (the
  AGPL-vs-Apache reference-code license conflict between the two governance docs) and **C14** (the
  drystone-spec Part 1/Part 2 Project-Hydra `[confirm]` status inconsistency).
- 2026-07-09 — **C13 DECIDED (AGPL-3.0-or-later + DCO reference impl) and C14 reconciled (spec Part1/Part2
  Hydra flags).** Both propagated across the docs; A14's reference-code half superseded by C13.
- 2026-07-09 — **Phase 4 DONE (C11): `beta/DECISIONS.md`** — 12 settled decisions + 11 open gates, each a
  pitch-resolution row linking down to its beta reasoning home (anti-rollup rule); reasoning-gaps flagged
  (A6 vote-accumulation, S3/S4 privacy properties, the A8 app-body IP call, A5 republish-UX). Indexed in
  `beta/README.md` alongside LAYERS.md and the per-layer reference-indexes.
- 2026-07-09 — **Remaining pass work:** Phase 3 (quote/resolution sweep, C10 — exemplar done; sweep the
  older pre-recovery docs); the deferred spec-rationale additions + governance mottos/PDS-revenue; and the
  move of promoted threads T41–T48 into `CLOSED-THREADS.md` after review.
