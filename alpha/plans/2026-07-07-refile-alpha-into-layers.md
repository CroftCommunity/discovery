# Plan: re-file alpha into the beta layer-cake, discard the theme files

date: 2026-07-07
status: PLAN (decisions resolved 2026-07-07: mixed treatment with a conclusion-level completeness
discipline; delete themes after the coverage gate; one commit per layer. Not started.)

## Problem statement

The beta synthesis was first organized as eight narrative **themes** (`beta/02`..`beta/08`, plus the
retired `01` that became the Drystone spec Part 1). That was the wrong filing strategy: a theme cuts
*across* maturity layers, so a single conclusion ends up split between a theme narrative and a layer doc,
or duplicated, and the "one home per claim" invariant the layer-cake is built on cannot hold. The project
has since moved to the **layer-cake** as the canonical structure (`beta/LAYERS.md`: history, philosophy,
cairn, fenced, drystone-spec, impl, croft, governance, socialization, activism). We now want to **re-file
the alpha corpus directly into layers** and **discard the theme files**; the prior theme mappings are no
longer meaningful.

Three facts make this a reconciliation-and-completion job, not a blank migration:

1. **The layers are already partly populated.** philosophy, cairn, fenced, drystone-spec, impl, governance,
   and socialization all hold content, some migrated from themes, some filed directly from batches. Re-filing
   alpha must MERGE into what is there without duplicating it, and must keep the more-mature version when an
   alpha doc overlaps content already in a layer.

2. **The alpha corpus was already dispositioned once, into themes.** `plans/2026-06-25-beta-coverage-per-file-audit.md`
   dispositioned all 165 alpha files exactly once (folded-to-theme vs alpha-only-by-design). That audit is the
   starting inventory; the job is to **re-key it from theme to layer**, not to re-triage from scratch.

3. **The alpha->beta map is theme-keyed.** `BETA-ROLLUP.md` records the source->treatment->landing trace
   organized by theme (01..08). When themes are discarded, that ledger must be rebuilt as **layer-keyed**, or
   it stops being an accurate map.

Provenance constraint (PLAYBOOK §4, non-negotiable): **alpha stays frozen.** Re-filing means COPYING or
DISTILLING alpha content INTO beta layers; alpha files are never moved, edited, or deleted. `alpha/seeds/`
(162 files: transcripts, corpus freezes, generated prompts) is the raw provenance floor and is NOT re-filed.

## Scope: what moves, what does not

**In scope (the alpha content corpus to re-file, ~92 docs):**

- `alpha/thinking/` (61, incl. `app/` and `drystone-spec/` subtrees): the first-pass design thinking.
- `alpha/research/` (16): the landscape/field research.
- `alpha/crystallized/` (7): conclusions, principles, CROFT-PROTOCOL, proof-ledger, conformance-suite,
  TEST-CORPUS, test-narrative.
- `alpha/narrative/` (8, incl. `verticals/`): long-form, short, pitches, brand-comms, croft-the-name.
- `alpha/assets/` (3): the two croft wordmark PNGs + README.

**Out of scope (stays alpha, frozen):**

- `alpha/seeds/` (162): raw provenance (transcripts, corpus freezes, generated prompts). Never re-filed.
- `alpha/plans/` (9): process/reflection artifacts (this plan included). Stay alpha by design.
- The four ledgers (`BETA-ROLLUP.md`, `COHESION.md`, `ROADMAP_TODO.md`, `ECOSYSTEM.md`): process/index
  surfaces. `BETA-ROLLUP` is reworked in place (Phase 3); the others stay.

**To inventory before Phase 2:** `beta/thinking/raw/` (an existing beta dir of unknown content) and the
current contents of every populated layer (so re-filing merges rather than duplicates).

## Target layers, and the theme->layer correspondence (the coarse map)

The theme->layer correspondence is mostly known and is the scaffold for Phase 0:

```
theme (to discard)                      lands in layer(s)
01 epistemic foundation        -> ALREADY drystone-spec Part 1 (done)
02 enclosure and its inversion -> history (1, NEW) + philosophy (2)
03 the living ecosystem        -> cairn (3) + fenced (3')
04 the protocol we proved      -> drystone-spec (4) + impl (5)
05 identity you carry          -> drystone-spec (4) + impl (5)
06 safety without surveillance -> drystone-spec (4) + impl (5)
07 sustainability & stewardship-> governance (7) + philosophy (2)
08 croft the product           -> croft (6, NEW) + socialization (8)
```

Two target layers do not exist yet and must be created: **history (Layer 1)** and **croft (Layer 6)**.

Coarse alpha-subtree -> layer intuition (Phase 0 refines to per-file):

- `research/` (landscape) -> **cairn** (open/composable field) and **fenced** (centered platforms:
  discord-dominance, discord-matrix-groupchat, group-chat-failure-modes, messaging-solutions-landscape,
  social-platform-cycle map onto the fenced map; public-social-protocols, atproto-* map onto cairn).
- `thinking/drystone-spec/` and the spec-shaped `thinking/*` design notes -> **drystone-spec** (fold into
  Part 1/2 where they are conclusions) or **impl** (where they are working design). Most are already
  reflected in the spec/impl; Phase 0 marks each as already-covered vs still-to-fold.
- `thinking/app/` (build-specs, platforms, ponds, prds, design-philosophy) -> **croft** (Layer 6, product).
- `crystallized/principles` -> **philosophy** / spec Part 1; `CROFT-PROTOCOL`, `proof-ledger`,
  `conformance-suite`, `TEST-CORPUS`, `test-narrative` -> **drystone-spec** + **impl** (cited/backing).
- `narrative/` (long-form, short, pitches, brand-comms, croft-the-name) -> **socialization**; the
  croft-the-name etymology also seeds **history**.
- `assets/` (wordmarks) -> **socialization** (brand assets).
- The peer-standing / non-domination / cooperative material -> **philosophy** (already largely there);
  the manifestation (foundation/co-op) -> **governance**; the harm case -> **activism** (already there).

## Approach (phased)

### Phase 0 — the layer disposition ledger (the spine)

Produce one master table, keyed at the level of **each doc's load-bearing conclusions, not just the file**:
every in-scope alpha doc -> {its load-bearing conclusions, target layer per conclusion, treatment,
status-vs-current-layer}. Seed it from the 2026-06-25 per-file audit (re-key theme->layer) rather than
re-triaging. Conclusion-level keying is what makes the completeness guarantee real: it is how a `covered`
disposition is checked (below) and how the Phase-3 gate audits conclusion coverage rather than mere file
coverage. Treatment codes:

- `copy` (file byte-faithful into the layer; for docs already layer-shaped and single-homed),
- `distill` (synthesize into a new/existing layer doc; for raw or multi-topic docs),
- `merge` (fold into an existing layer doc, keeping the more-mature version),
- `covered` (already represented in the layer from theme-migration or a batch) **-- verified, not assumed:
  a `covered` mark MUST cite the specific layer doc + section that captures the alpha doc's conclusions, and
  a check must confirm each load-bearing conclusion actually appears there. If any conclusion is missing, the
  disposition flips to `merge` or `distill`. This is the mechanism that makes the mixed (per-file) treatment
  miss nothing that a distill-everything pass would have caught, at far less cost. It reprises the old
  rollup's CONCLUSION-MISSING audit.**,
- `alpha-only` (process/index/raw; stays alpha by design),
- `excluded` (do-not-carry-forward; record why, once).

Deliverable: this table, as the new layer-keyed map (Phase 3 turns it into the rebuilt rollup). Nothing is
filed in Phase 0; it is pure disposition + reconciliation against current layer contents.

### Phase 1 — stand up the two missing layers

Create `beta/history/` (Layer 1) and `beta/croft/` (Layer 6), each with a README in the established
layer-README shape (what-this-layer-is, scope, boundary calls vs neighbors, contents placeholder,
provenance, "what this establishes and does not"). These are the targets Phase 2 fills.

### Phase 2 — layer-by-layer re-filing (the bulk of the work)

Work one layer at a time (recommended order: history, croft, then complete philosophy, cairn, fenced,
socialization, governance; drystone-spec and impl are largely done, so mostly `covered`/`merge`). For each
layer: file/distill the alpha docs the Phase-0 table assigns to it, MERGE with existing content (no
duplication), and hold beta tier discipline (no prior-tier refs in beta docs; verification flags on carried
claims; AI-sourced quotes tagged; em-dash- and drift-clean). One commit per layer keeps it reviewable and
resumable. Distillations that need synthesis run as opus subagents grounded strictly in the named alpha
sources (the established pattern).

### Phase 3 — rebuild the alpha->beta map as layer-keyed

Rework `BETA-ROLLUP.md` (or supersede it with a `LAYER-ROLLUP.md`) from theme-keyed to layer-keyed: one
section per layer, each listing the alpha sources that landed there and their treatment. Run the coverage
audit at **conclusion granularity**: every in-scope alpha doc dispositioned exactly once; every load-bearing
conclusion either filed into a layer or verified-present (for `covered`) with a cited target; and every
"settled conclusion not yet folded" and "CONCLUSION-MISSING" item from the old rollup either folded into a
layer or explicitly parked in OPEN-THREADS. Auditing conclusions (not just files) is the gate for Phase 4,
and is what lets the mixed treatment discard the themes safely.

### Phase 4 — discard the theme files (only after the coverage gate)

Delete `beta/02`..`beta/08` ONLY once Phase 3 confirms every unique theme conclusion is captured in a layer.
Then update: `beta/README.md` (remove the theme reading-spine table and the theme-vs-layer framing; make the
layer-cake the sole canonical structure), `LAYERS.md` (drop the "02-08 remain the reading spine" notes),
`OPEN-THREADS.md` (repoint every thread whose promotion target was a theme, e.g. T4/T11->socialization,
T26->croft, T33->governance/philosophy, to its layer), and any doc that cross-references a theme number.

### Phase 5 — coherence sweep + commits

Final drift/em-dash/tier-discipline sweep across all touched docs; update the manifest; confirm no dangling
theme references remain anywhere; carry out (or explicitly re-defer) the two standing end-of-run sweeps
(peer->persona reconciliation in current bodies; em-dash tidy of the pre-existing docs).

## Reasoning (why this shape)

- **Layer-first gives one home per claim; theme-first did not.** A theme is a reading path across layers, so
  it necessarily co-locates a principle (philosophy), a mechanism (spec), and a pitch (socialization) that
  each belong in different layers. That is the exact duplication/splitting the move to layers exists to fix.
  Re-keying to layers restores single-home-per-claim.

- **Re-key from a signal, do not blindly inherit.** The 2026-06-25 audit classified all 165 files once, so
  it is a valuable *starting signal* that saves re-triaging from scratch. But it is not a presumed-complete
  or authoritative set: it is a prior to test, not the answer. Its own CM list proves it missed conclusions,
  so every disposition (especially the "file nothing new" verdicts: `covered`, `alpha-only`, `excluded`) is
  re-checked against the actual alpha file in Phase 2, and conclusions are enumerated by reading the file,
  not by trusting the matrix row. This is the same do-not-trust-a-closed-loop-authority discipline as the
  epistemics layer: the matrix accelerates, the primary decides.

- **Reconcile, do not blank-migrate.** Because the layers are already partly populated (from theme migration
  and batches), a naive "copy alpha into empty layers" would duplicate and would clobber more-mature layer
  content. The `covered`/`merge` treatments and the reconciliation-against-current-contents step are what
  keep the layers coherent.

- **Provenance stays with alpha; the map is layer-keyed.** Alpha frozen + a layer-keyed rollup preserves the
  auditable source->landing trace the corpus depends on, while letting beta read clean at its maturity level.

- **Discard last, behind a coverage gate.** The theme files are the current *record* of some conclusions
  until those conclusions are confirmed present in a layer. Deleting them before the Phase-3 coverage audit
  would risk losing a unique synthesis conclusion. Discarding after the gate is safe.

- **One commit per layer.** The corpus is large; per-layer commits keep the work reviewable, resumable
  across sessions, and easy to revert if a layer's re-filing goes wrong.

## Decisions (resolved 2026-07-07 unless marked open)

1. **Treatment default: mixed per-file, with a conclusion-level completeness discipline. RESOLVED.** The
   worry that motivated considering "distill everything" was completeness (not missing anything). That is
   handled not by the treatment code but by keying Phase 0 at conclusion granularity and by making `covered`
   a verified disposition (cited target + confirmed-present, else it flips to merge/distill), with the
   Phase-3 gate auditing conclusion coverage. This gives the no-miss guarantee of a distill-everything pass
   without its waste and divergence risk. Mixed treatment adopted.

2. **`alpha/research/` and `alpha/plans/` fate.** Recommend: `research/` re-files into cairn/fenced (mostly
   distilled, since it is landscape prose); `plans/` stays alpha (process artifacts). Confirm.

3. **Create history (1) and croft (6) now, in Phase 1.** Confirm, and confirm the seed correspondence
   (02->history, 08 + thinking/app->croft).

4. **Discarded theme files: delete. RESOLVED.** `beta/02`..`beta/08` are deleted after the Phase-3 coverage
   gate; their provenance is alpha + the rebuilt layer-keyed rollup, so nothing is lost.

5. **Sequencing: one commit per layer. RESOLVED.** Reviewable, resumable across sessions, and each layer's
   re-file is independently revertable.

6. **`beta/thinking/raw/` disposition.** Unknown dir; I will inventory it in Phase 0 and propose fold-or-remove
   then. Flagging so it is not a surprise.

7. **Overlap reconciliation policy.** When an alpha doc overlaps content already in a layer, keep the
   more-mature layer version and record the alpha source as `covered`/`merge` in the rollup, rather than
   re-importing the less-mature alpha prose. Confirm.

## What this plan is not

It does not itself re-file anything (Phase 0 onward is execution, gated on the decisions above). It does not
touch `alpha/` corpus content (frozen). It does not delete any theme file before the Phase-3 coverage gate.
