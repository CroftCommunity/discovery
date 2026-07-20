# RUN-RECIPE-IMPORT — import a recipe from a link (Alchemy)

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Independent of RUN-SHOPPING-LIST (separate branch; both may touch
docs/LEXICONS.md — whichever merges second rebases and re-runs the gate).
[verify-in-run] items are probed and recorded in the run summary before code
depends on them. Contradictions with this file's grounding are FINDINGS.

## 0. Mission

On the Alchemy page (`mine.html`), a cook pastes a recipe URL and gets a
prefilled LOCAL DRAFT — name, ingredients, instructions, and facets where
present — which opens in the editor for review. Nothing publishes without the
user's normal editor flow.

Why this is feasible: recipe sites overwhelmingly embed schema.org/Recipe
structured data as JSON-LD (`recipeIngredient`, `recipeInstructions`) because
Google's recipe rich results require it — and arecipe's own lexicon already
uses schema.org's field names, so the mapping is nearly identity. The genuine
obstacle is CORS plus the no-backend stance: recipe sites don't send CORS
headers, a static PWA cannot fetch them cross-origin, and
docs/GITHUB-CORS-PROBE.md records that a proxy origin would conflict with
docs/SECURITY.md. So acquisition is LAYERED: try the direct fetch; when it
fails (the common case, and the mobile reality), fall back to a paste flow.
That friction is the honest price of serverless — the UI says so plainly
rather than pretending.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation; the JSON-LD
   fixture corpus lands BEFORE the extractor; run summary evidences
   red → green per phase.
2. **Gate green** at every phase boundary; browse bundle-split guard
   untouched (this run touches mine/editor only; the parser modules are
   auth-free pure).
3. **Style**: strict vanilla TS; pure cores with injectable deps; DOM wiring
   guarded by e2e; zero new runtime dependencies; module comments explain
   why.
4. **Plan file** `plans/2026-07-XX-N-plan-recipe-import.md` before coding;
   Status updated at completion.
5. **Safety**: fetched/pasted HTML is parsed ONLY via `DOMParser` (inert
   document) — never assigned into live DOM; every extracted string is
   entity-decoded, tag-stripped, and clamped to the recipe lexicon's field
   maxima.

## 2. Grounded context (verified 2026-07-16 snapshot; Phase 0 re-grounds)

- Alchemy = `mine.html` / `src/pages/mine.ts` (nav label "Alchemy", testid
  `tab-mine`): authoring entry + the local drafts list; drafting needs no
  session.
- Drafts: `src/recipes/drafts-local.ts` — `Draft { id, fields: EditorFields,
  savedAt, status }`, `createDraftStore().save(fields, id?, status?)`,
  IndexedDB-backed, legacy-tolerant reads. [Phase 0: the exact EditorFields
  shape, and how the editor opens an existing draft (URL param vs store
  handoff) — the import lands via `save(...)` then that same open path.]
- Recipe lexicon: open-world; required name/text/ingredients/instructions;
  optional recipeCuisine/recipeCategory/suitableForDiet — schema.org-named
  already. Lexicon changes register in docs/LEXICONS.md
  (fixture lexicons under tests/fixtures/lexicons/).
- Structured data in the wild (researched 2026-07-16): Google's Recipe
  rich-results documentation defines the properties (`recipeIngredient`;
  `recipeInstructions` as text, HowToStep[], or HowToSection[]); JSON-LD is
  Google's recommended format and has been since 2017; real pages also show
  the LEGACY `ingredients` key and instructions-as-one-string. The extractor
  must tolerate all of these.
- No-backend stance: docs/PHILOSOPHY.md ("no backend, no user database");
  GITHUB-CORS-PROBE.md rules out proxy origins. No third-party fetch/proxy
  services — that would move user browsing data through an untrusted origin.

## 3. Locked design decisions

- **D1 Entry + layered acquisition.** An "Import from link" affordance on
  Alchemy opening an inline panel (house panel idiom): URL field + Import.
  Flow: attempt `fetch(url, { mode: 'cors' })` with a short timeout —
  `no-cors` is useless (opaque responses) and must not be used. On success →
  parse ladder. On failure (CORS/network), the SAME panel expands to the
  paste fallback: a textarea accepting either the page source or the visible
  recipe text, with brief copy explaining why ("this site doesn't allow
  direct reading from the browser — paste the page or the recipe text
  instead"). The URL stays attached as provenance either way. Error taxonomy
  surfaced honestly: could-not-fetch vs fetched-but-no-recipe-found vs
  parsed-partially.
- **D2 Parse ladder.** (1) JSON-LD: every
  `script[type="application/ld+json"]` in the inert document; find the
  Recipe object through top-level objects, arrays, and `@graph`, tolerating
  `@type` as string or array. (2) Pasted-text heuristic (D4) when no JSON-LD
  Recipe exists. Microdata/itemprop extraction is DEFERRED, not built —
  JSON-LD plus paste covers the practical field; note it in the plan file.
- **D3 Field mapping.** `name` → name; `recipeIngredient` (or legacy
  `ingredients`) → ingredients[]; `recipeInstructions`: string → split on
  numbered/newline boundaries; HowToStep[] → each step's text;
  HowToSection[] → flatten, each section name as its own line prefixed
  `— <Section>`; `description` → text (clamped); `recipeCuisine` /
  `recipeCategory` → the matching facet fields when present (string or
  first-of-array, normalized to the app's facet casing). Images are NOT
  imported in v1 (blob-upload complexity, and photographs are where
  third-party rights bite hardest) — Deferred with that reason recorded.
- **D4 Text heuristic (paste of visible text).** Ingredients block = the
  longest run of ≥3 consecutive lines that start with a quantity, fraction,
  or bullet; instructions = numbered lines, or paragraphs following a
  heading token (instructions / method / directions / steps,
  case-insensitive). Confidence gating: an empty bucket imports PARTIALLY
  with the missing side flagged in the panel and the draft left blank there
  — never fabricate content.
- **D5 Destination + provenance.** Result lands as
  `draftStore.save(fields, undefined, 'draft')` and the editor opens on it.
  Add optional `sourceUrl` (string, uri format) to EditorFields AND the
  recipe lexicon (fixture + docs/LEXICONS.md row; [verify-in-run: the
  repo's lexicon-evolution convention — additive optional field, readers
  tolerate absence]). The editor shows it as a small provenance line.
  Near publish, an imported draft (sourceUrl present) shows ONE gentle
  etiquette line encouraging instructions in your own words — copy goes in
  the run summary for the owner's review before it's considered final.
- **D6 Scope honesty.** The importer imports "just the instructions and
  ingredients" (owner's words) plus name/facets/description. It does not
  rate, dedupe against existing recipes, or bulk-import. Deferred list:
  microdata, image import, Web Share Target registration (share a page from
  the phone's browser straight into the import panel — natural fast-follow),
  bulk import.

## 4. Phases

### Phase 0 — ground against main
Locate: EditorFields shape; the editor's draft-open path; the panel idiom to
copy; lexicon-evolution precedent (how mealsPerDay was added is the model);
where mine.ts mounts actions. Drift = FINDING.

### Phase 1 — JSON-LD extractor (fixtures first)
Create `tests/fixtures/import/`: handcrafted pages covering — plain Recipe
object; Recipe inside `@graph`; `@type` as array; instructions as one
string; HowToStep[]; HowToSection[] with names; legacy `ingredients` key;
HTML entities and embedded tags inside strings; multiple ld+json scripts
where only one is a Recipe; a page with NO recipe. RED
`tests/unit/import/jsonld.spec.ts` driving every fixture, including
clamping to lexicon maxima and tag-stripping; GREEN
`src/import/recipe-jsonld.ts` (pure, DOMParser-injectable).

### Phase 2 — text heuristic
RED `tests/unit/import/text-heuristic.spec.ts`: fixture pastes — clean
quantity-led ingredient runs; bulleted runs; numbered instructions; heading-
token splits; a paste with ingredients only (partial import, flagged); prose
with neither (no-recipe-found). GREEN `src/import/recipe-text.ts`.

### Phase 3 — acquisition + panel
RED: unit specs with an injected fetch — success path routes to the ladder;
CORS/timeout failure expands the paste flow; error taxonomy strings; URL
retained as sourceUrl in every path. Panel DOM specs per house style.
GREEN: `src/import/acquire.ts` + mine.ts panel wiring (testids for url
input, import, paste area, status).

### Phase 4 — draft handoff + lexicon
RED: mapping into EditorFields (all D3 rules); save-then-open flow with a
fake store; sourceUrl surfaces in the editor; the etiquette line renders
only for imported drafts at the publish step; lexicon fixture + LEXICONS.md
row land BEFORE the field is written. GREEN.

### Phase 5 — e2e + closeout
Hermetic: serve a fixture recipe page SAME-ORIGIN via Playwright routing so
the direct-fetch path is exercisable → URL import lands a draft and opens
the editor prefilled; a routed CORS-failing URL expands the paste flow →
pasting fixture source imports; pasting plain recipe text imports via the
heuristic; a no-recipe paste shows the honest error; partial import shows
the flagged missing side. Mobile-fit: panel + textarea usable at ≤390px.
Plan Status; run summary: red → green per phase, fixture inventory,
[verify-in-run] outcomes, and the D5 etiquette copy for owner review.

## 5. Acceptance criteria (each maps to a named test)

1. A URL to a JSON-LD recipe page (same-origin fixture) imports name,
   ingredients, instructions, and present facets into a draft that opens in
   the editor — nothing published.
2. A CORS-blocked URL degrades to the paste flow with honest copy; pasted
   page source imports identically; pasted plain text imports via the
   heuristic; provenance (sourceUrl) is attached in every path.
3. Every fixture shape in Phase 1's corpus parses; no-recipe and partial
   cases surface honestly and never fabricate.
4. Fetched/pasted HTML never touches the live DOM; all strings are
   sanitized and clamped (unit-proven).
5. `sourceUrl` is an additive optional lexicon field, registered and
   fixture-backed; imported drafts show the provenance line and the single
   etiquette line at publish.
6. Zero new runtime dependencies; browse bundle untouched; gate green at
   every phase.
