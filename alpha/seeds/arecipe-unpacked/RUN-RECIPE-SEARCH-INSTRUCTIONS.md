# RUN-RECIPE-SEARCH — full-text recipe search on Browse and Cookbook

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Execute top to bottom. Where this file says [verify-in-run], probe empirically
and record the outcome in the run summary before depending on the claim.

## 0. Mission

Add ranked, typo-tolerant, content-aware text search to the Browse and Cookbook
pages, powered by MiniSearch (https://github.com/lucaong/minisearch). A query
like `feta` must surface recipes whose **ingredients** contain feta even when
the title does not; `chicken lemon` must mean recipes matching **both** terms;
`brocolli` must still find broccoli. The meals-palette name filter
(`src/recipes/meal-plan-palette.ts`) is explicitly out of scope — it stays as
`includes()` filtering.

Why MiniSearch: the corpus is a bounded in-memory `CachedRecipe[]` (hundreds,
maybe low thousands — feeds are whole author repos via `listRecords`, already
fetched and IndexedDB-cached before render). MiniSearch gives BM25 ranking,
per-field boosting, prefix and fuzzy matching in a ~7 KB gzipped
zero-dependency package with incremental add/remove. Heavier engines (SQLite
FTS5 WASM, Orama) were evaluated and rejected for this scale; do not substitute
one.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Every phase writes its failing tests BEFORE any
   implementation. Acceptance criteria in §6 are encoded as tests; fixtures
   are created/extended before the features that consume them. The run summary
   must show the red-to-green order per phase (which specs failed first, which
   commit turned them green). Implementation without a preceding failing test
   is a defect in the run itself.

2. **Repo gate.** `npm test` = lint + typecheck (app AND tests tsconfigs) +
   unit + build + e2e. The gate must be green at the end of every phase, not
   just at the end of the run.

3. **Style.** Vanilla strict TypeScript, no framework, esbuild bundles,
   page-per-destination. New logic follows the repo's pure-core pattern
   (`src/pages/browse-state.ts` is the model: no DOM, injectable dependencies,
   unit-tested in isolation; DOM wiring lives in the page/toolbar and is
   guarded by e2e). Module comments explain the why, matching existing files.

4. **Bundle-split constraint.** `browse.html` ships ZERO auth code, enforced by
   an existing e2e test. The new search module must not import anything from
   `src/auth/`. MiniSearch itself is dependency-free; keep it that way.

5. **Plan file.** Before Phase 1, add
   `plans/2026-07-XX-N-plan-recipe-text-search.md` (date/sequence per repo
   convention) recording the problem statement, the locked decisions in §3,
   and the phase list. Update its Status line when the run completes, matching
   the format of `plans/2026-07-13-1-plan-multi-meal-scheduling.md`.

## 2. Grounded context (verified against the repo 2026-07-15)

- Recipe records: `exchange.recipe.recipe`, open-world. Required fields
  (validated in `src/recipes/read.ts`): `name`, `text`, `ingredients[]`,
  `instructions[]`, `createdAt`, `updatedAt`. Optional/extension fields read
  defensively elsewhere: `recipeCuisine`, `recipeCategory`, `suitableForDiet`,
  `dishKey`, `versionLabel`, `funFacts[]` (see `src/recipes/model.ts` and
  `src/pages/browse-state.ts` for the defensive-accessor pattern to copy).

- Browse pipeline (`src/pages/browse.ts` `computeShown()`): entries → hidden
  removed (`exclusions`) → `matchesFilter` (photos + facets + diet) →
  `matchesTaste` → `collapseVersions` → `windowPage` (50/page). Cookbook
  (`src/pages/cookbook.ts` `renderFeedView`) runs the same primitives over
  `activeEntries()` (source = all | mine | liked, feed swappable in place via
  `update()`).

- Shared toolbar: `src/recipes/toolbar.ts` (`renderToolbar`), used by both
  pages, byte-identical testids: `view-tiles`, `view-details`, `photos-only`,
  `recipes-status`, `reset-filters`. Status uses the honest-count pattern
  `"N of M shown"` when any filter is active.

- E2E convention: hermetic routed fixtures
  (`tests/e2e/browse.spec.ts` + `tests/fixtures/atproto/listRecords-browse-mixed.json`).
  The mixed fixture holds 4 recipes: Greek Salad (ingredients incl. feta),
  American Pancakes, Italian Minestrone (no image), Greek Vegan Lunch Bowl
  (ingredients incl. lemon).

- Unit tests: Vitest under `tests/unit/…` mirroring `src/…`; happy-dom +
  fake-indexeddb available where needed.

## 3. Locked design decisions

Do not re-open these; they were decided against the deployment analysis.

- **D1 Engine.** `minisearch` (latest release; record the exact version and
  its min+gzip size in the run summary). Runtime dependency, imported as ESM
  default export. [verify-in-run: esbuild bundles it cleanly and bundled types
  satisfy strict TS; if the bundle grows by >15 KB gzipped, flag it.]

- **D2 Indexed fields and boosts.** Per recipe, extracted defensively (missing
  or mistyped → empty string, never throw): `name` (boost 4),
  `ingredients` joined with newlines (boost 3), `text` (boost 2),
  `instructions` joined (boost 1), plus `versionLabel`, fun-fact texts, and
  the normalized `cuisine`/`category` from `recipeFacets` folded into one
  auxiliary field (boost 1) so "thai" works as free text.

- **D3 Query semantics.** `combineWith: 'AND'`, `prefix: true`, `fuzzy: 0.2`.
  AND encodes ingredient-search intent (every term must match); prefix keeps
  as-you-type responsive; fuzzy 0.2 ≈ one edit per five characters.

- **D4 Ranking vs feed order.** Empty/whitespace query = identity: input order
  preserved exactly, zero MiniSearch involvement. Non-empty query = the result
  is BOTH a filter and an ordering: only matches survive, in descending score
  order, replacing feed order.

- **D5 Pipeline position.** The query stage runs AFTER exclusions/facets/diet/
  taste and BEFORE `collapseVersions`, so a match on any version's content
  surfaces its dish's representative card. Pagination stays last; the page
  offset resets to 0 on every query change (same as the other filters).

- **D6 Index lifecycle.** Rebuild whole index when the candidate entry set
  changes (new find, starter feed load, cookbook `update()`, cookbook source
  switch, liked-feed load). Memoize on entries-array identity; no incremental
  bookkeeping, no persistence (records already live in IndexedDB; rebuild is
  milliseconds at this scale).

- **D7 UI.** One search input in the shared toolbar, both pages, testid
  `recipe-search`, placeholder `search recipes…`, debounced ~150 ms,
  `type="search"` (native clear affordance). Query is transient page state:
  NOT persisted to localStorage/sessionStorage; navigating away drops it.
  The reset control clears it and its visibility condition includes it; an
  active query renders the `"N of M shown"` status.

- **D8 Deliberately deferred** (list verbatim under Deferred in the plan
  file): match highlighting from MiniSearch match metadata; `autoSuggest`
  autocomplete; swapping the meals-palette filter onto this module; Web Worker
  or persisted index (revisit only if the corpus reaches tens of thousands).

## 4. Phases

### Phase 1 — pure core `src/recipes/search.ts`

RED: create `tests/unit/recipes/search.spec.ts` covering at minimum:

- extraction: an open-world record missing `ingredients`/`funFacts`/etc.
  indexes without throwing; mistyped fields (e.g. `ingredients: 42`) read as
  empty; `funFacts` legacy singular string shape (see `funFactsOf`) is
  included.

- ranking: for the same single term present in one recipe's name and another
  recipe's instructions only, the name match scores first.

- content reach: a term present ONLY in ingredients returns that recipe.

- AND: `chicken lemon` returns only recipes matching both terms.

- fuzzy: a one-edit typo (`brocolli`) matches `broccoli`; prefix: `pancak`
  matches Pancakes.

- identity: empty and whitespace-only queries return the input entries in
  unchanged order.

- lifecycle: `createRecipeSearch` called with a new entries array reflects
  the new set (old entries gone, new ones searchable).

Confirm all red. GREEN: implement
`createRecipeSearch(entries: readonly CachedRecipe[])` returning
`{ query(q: string): CachedRecipe[] }` (or ranked URIs mapped back — pick one,
document it), keyed by `uri`, options per D2/D3. No DOM, no page imports.

### Phase 2 — pipeline integration (Browse + Cookbook)

RED: unit tests for the composition seams:

- query stage composes with `collapseVersions`: given two records sharing a
  `dishKey` where only the non-primary version's ingredients match, the
  representative (primary) card is what survives collapse.

- query composes with facets: facet filter first, then query, both applied.

- memoization: same entries array reference does not rebuild the index
  (assert via an injectable factory/spy).

GREEN: wire the stage into `computeShown()` in `browse.ts` and the equivalent
filter path in `cookbook.ts`'s `renderFeedView` (covering `update()` and
source switches per D6). Keep the stage itself pure and shared — do not fork
logic per page.

### Phase 3 — toolbar input

RED: extend the toolbar/page unit coverage (and prepare the e2e in Phase 4)
for: input present with testid `recipe-search`; `onQueryChange` fired
debounced; reset clears the input and the query; reset visibility includes an
active query; page offset resets on query change.

GREEN: add the input + `onQueryChange` callback to `renderToolbar` and wire
both pages. Preserve all existing testids and class hooks byte-identical —
`tests/e2e/browse.spec.ts` and `cookbook.spec.ts` are the regression guards.

### Phase 4 — e2e (hermetic, routed mixed fixture)

RED first, using `listRecords-browse-mixed.json` routing from
`browse.spec.ts` (new file `tests/e2e/search.spec.ts` or extend browse.spec —
follow whichever the existing specs' granularity suggests):

- type `feta` → exactly Greek Salad shown, status `1 of 4 shown`.
- type `tomato` → Greek Salad + Minestrone (ingredient-level, order by score).
- type `pancaks` (typo) → American Pancakes shown (fuzzy).
- clear via reset → 4 recipes, original starter status string restored.
- query + facet compose: cuisine `greek` + query `lemon` → only the Lunch
  Bowl.
- Cookbook cold view (`?did=`) exercises the same input (route the fixture per
  `cookbook.spec.ts` conventions).

If the fixture needs an additional record to make a case unambiguous, extend
the fixture FIRST (fixtures before features) and note it; keep existing counts
in untouched specs green.

### Phase 5 — docs + closeout

- Plan file Status updated (gate results, spec counts).
- `docs/` touched only if an existing doc enumerates page features; no new doc.
- Run summary must include: red-to-green evidence per phase, minisearch
  version + measured bundle-size delta, the [verify-in-run] outcomes from D1,
  and anything that deviated from this file with the reason.

## 5. Acceptance criteria (each maps to a named test above)

1. Ingredient-only terms find recipes on both Browse and Cookbook.
2. Multi-term queries are AND semantics.
3. One-edit typos and prefixes match.
4. Name matches outrank body matches.
5. Ranked order applies only while a query is active; empty query preserves
   feed order byte-identically.
6. Version-collapsed dishes surface when any version matches.
7. Reset clears the query; status shows honest counts; pagination resets.
8. Meals palette behavior unchanged; browse bundle still ships zero auth code;
   full `npm test` gate green.
