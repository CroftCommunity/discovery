# RUN-SHOPPING-LIST — shopping lists from scheduled meal plans (per-recipe + combined)

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Independent of RUN-RECIPE-IMPORT (separate branch; both may touch
docs/LEXICONS.md — whichever merges second rebases and re-runs the gate).
[verify-in-run] items are probed and recorded in the run summary before code
depends on them. Contradictions with this file's grounding are FINDINGS.

## 0. Mission

From the Meals page (and the public published-plan calendar view), generate a
shopping list for a chosen range of scheduled meals, in two views:

- **By recipe** — one section per scheduled recipe (×N when it repeats in the
  range), one ingredient per line, verbatim. Lines the aggregator could NOT
  roll up carry a flag here (owner ruling: the non-aggregate view is where
  "this didn't get rolled up / couldn't be determined" shows), so the mental
  delta at the store is only the flagged stragglers.
- **Combined** — ingredients grouped across recipes: same ingredient in three
  recipes becomes one line with a summed quantity where units are compatible,
  and an honest "as listed" section for everything the parser declines.

Both views are copyable and downloadable as one file. Aggregation is a small
DETERMINISTIC parser — no ML, no dependency, conservative by design: being
honest about misses beats being confident and wrong.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation; fixtures before
   the features that consume them; run summary evidences red → green per
   phase. The parser is fixture-table-driven from day one.
2. **Gate green** (`npm test`) at every phase boundary; browse bundle-split
   guard untouched (this run adds nothing to browse).
3. **Style**: strict vanilla TS; pure cores with injectable deps; DOM wiring
   guarded by e2e; module comments explain why; degrade-not-blank posture for
   anything network.
4. **Plan file** `plans/2026-07-XX-N-plan-shopping-list.md` before coding;
   Status updated at completion, house format.

## 2. Grounded context (verified 2026-07-16 snapshot; Phase 0 re-grounds)

- Meal plans: `app.arecipe.mealPlan` records — `weeks[] → days[7] → slot`
  where a slot holds `meals[]` and each meal references a recipe as a
  `com.atproto.repo.strongRef`; the in-memory meal items carry
  `{ uri, cid, name }` (name denormalized; INGREDIENTS ARE NOT — the list
  builder must resolve each unique uri to its recipe record). `startDate` is
  OPTIONAL ("v1 renders relative week labels and ignores this"), and weeks
  carry a `repeat` count — so a plan may be dated or undated, and the range
  selector must handle both. Legacy single-`recipe` slots are migrated on
  read.
- Meals page (`src/pages/meals.ts`, ~1300 lines): signed-in planner + a
  public cold view (`?did=`) that resolves the owner and renders the
  calendar; per-plan controls; the ICS calendar publish flow is the
  export-precedent. [Phase 0: locate the existing week-expansion helper
  (repeat stamping), the per-plan range realities, and the existing
  single-recipe record fetch used by the recipe page — reuse it for
  resolution rather than writing a new fetch path.]
- Recipes: `ingredients: string[]` free text; observed shapes are simple
  ("2 cups flour", "1 tbsp olive oil", "cucumber") — low variation is the
  premise this design leans on, and the flag system is the honesty valve.
- Export idiom: inline panel + copy + file download (browse export ↑ and the
  ICS flow are the patterns to match).

## 3. Locked design decisions

- **D1 Entry + range.** A "Shopping list" action on the Meals page per plan,
  present in BOTH the signed-in planner and the public `?did=` calendar view
  (the module is auth-free). Range selection: dated plans get from/to date
  pickers defaulting to the plan's full expanded range; undated plans get a
  week-based selector (week 1..N, honoring `repeat` expansion) defaulting to
  the whole plan. "All scheduled" is the default in both modes.
- **D2 Resolution.** Collect the unique recipe strongRefs in range; resolve
  each via the existing single-recipe fetch path (session-cached; injectable
  for tests). A recipe that fails to resolve appears in BOTH views by its
  denormalized name with an "ingredients unavailable" flag — never silently
  dropped, never blanking the rest.
- **D3 Parser (pure, conservative).** For each ingredient line produce
  `{ qty?, unit?, name, raw }`:
  quantities — integers, decimals, ASCII fractions (`1/2`), unicode vulgar
  fractions (½ ¼ ¾ ⅓ ⅔ ⅛ …), mixed forms (`1 ½`, `1 1/2`), and ranges
  (`1-2`) kept AS ranges;
  units — a small canonical table with synonyms and plural forms only
  (tsp/teaspoon, tbsp/tablespoon, cup, oz/ounce, lb/pound, g/gram, kg,
  ml, l/liter/litre, pinch); an unrecognized middle token is part of the
  NAME, not a unit;
  names — lowercase, trim, collapse whitespace, simple plural fold
  (trailing s/es with a small exception list). NO descriptor stripping in
  v1: "ground cinnamon" and "cinnamon" do NOT merge — they flag instead.
  (Descriptor folding is the first Deferred item; conservative beats wrong.)
  A line yielding no confident parse is UNPARSED and carries its raw text.
- **D4 Aggregation.** Key = normalized name. Within a key: sum quantities
  whose units share a family (volume with volume via the table's ratios,
  weight with weight; counts with counts); ranges sum end-to-end and stay
  ranges; quantities from a recipe scheduled ×N multiply by N; NEVER convert
  across volume/weight/count families — incompatible families render as
  separate quantities under the one heading ("flour — 2 cups + 100 g").
  Bare unquantified lines ("cucumber") aggregate as a count of occurrences
  rendered "×N", never as an invented quantity. UNPARSED lines go to the
  Combined view's "as listed" section attributed to their recipes, and their
  originals are flagged in the By-recipe view (D3 of the mission).
- **D5 Output.** Inline panel (export-panel idiom) with two tabs —
  "By recipe" and "Combined" — plus Copy (active tab) and Download. The
  downloaded file is one markdown document, Combined first, then By recipe,
  headed by the plan name and range. Filename
  `shopping-<plan-slug>-<range>.md`. No persistence, no PDS writes, no
  checkbox state (Deferred).
- **D6 Code placement.** Pure core `src/recipes/shopping-list.ts`: parse,
  normalize, aggregate, and render-to-markdown — every behavior above
  unit-tested with zero DOM. Page wiring in meals.ts only.
- **D7 Deferred (verbatim in plan file):** descriptor folding
  ("ground/chopped/fresh X" → X with provenance), pantry exclusions, aisle
  grouping, servings scaling, persistent/checkable list state, publishing a
  list as a record.

## 4. Phases

### Phase 0 — ground against main
Locate: week expansion/repeat stamping helper; dated vs undated plan handling
in the calendar view; the single-recipe record fetch to reuse; the download
mechanics of the ICS/export flows; current meals.ts toolbar/header seams for
the action. Record findings; drift from §2 = FINDING.

### Phase 1 — parser core (fixtures first)
Create `tests/fixtures/shopping/ingredient-lines.json`: a table of real-shaped
lines and expected parses — cover every D3 form, the browse-mixed fixture's
ingredients, unicode fractions, ranges, unit synonyms/plurals, name-only
lines, and deliberately unparseable strings. RED
`tests/unit/recipes/shopping-list.spec.ts` driving the table; GREEN the
parser. The table is the contract — extending the grammar later means
extending the table first.

### Phase 2 — aggregation + flags
RED: same-name compatible-unit sums; ×N multiplication; range arithmetic;
cross-family separation under one heading; bare-line ×N; unparsed →
"as listed" with recipe attribution; per-recipe flag mapping (each raw line
knows whether it rolled up). Property-style sanity: aggregating one recipe's
list is identity modulo normalization. GREEN.

### Phase 3 — range + resolution
RED: dated-range filtering over expanded weeks (repeat honored); undated
week-index selection; unique-ref collection; resolution with an injected
fetcher incl. the unavailable-recipe path. GREEN.

### Phase 4 — panel + export
RED: panel renders both tabs from injected results; Copy copies the active
tab; Download produces the D5 document; action present in planner AND public
view; unavailable-recipe flags visible; markdown renderer covered at unit
level. GREEN; testids for action, tabs, copy, download.

### Phase 5 — e2e + closeout
Hermetic routed fixtures: a plan (dated) + recipes → open list → both views
correct incl. a deliberate roll-up (same ingredient across two recipes), a
flagged line, and a ×2 repeat; an undated plan exercising week selection; an
unresolvable ref showing the unavailable flag; public `?did=` view has the
action. Mobile-fit: panel in viewport, tap targets. Plan Status; run summary
with red → green per phase and the fixture-table row count.

## 5. Acceptance criteria (each maps to a named test)

1. A date range (or week selection on undated plans) yields both views with
   every scheduled recipe represented, ×N respected.
2. Same-ingredient lines across recipes combine with correct sums in
   compatible units; incompatible families list separately under one heading;
   nothing converts across families.
3. Every line the aggregator declined appears in "as listed" with recipe
   attribution AND is flagged in the By-recipe view.
4. Unresolvable recipes degrade to a named, flagged entry — never dropped,
   never blanking.
5. Copy and Download produce the documented formats; the action works signed
   out on the public plan view.
6. The parser fixture table covers all D3 forms and passes; gate green at
   every phase; browse bundle untouched.
