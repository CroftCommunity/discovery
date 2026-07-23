# RUN-RECIPE-META-STRIP

Add serves and difficulty alongside the existing prep time, and present all three as a
three-row strip hanging off the bottom of the recipe image, in the manner of the Wikibooks
recipe infobox.

Repo: `CroftCommunity/arecipe`. Vanilla strict TypeScript, esbuild, no framework,
page-per-destination.

---

## 0. Standing directives

**TDD, red first.** Acceptance criteria as failing tests before implementation, fixtures
before features, red-to-green order evidenced in the run summary.

**Do not extend a lexicon you do not own.** See section 2. This is the gate on the whole
run and it is resolved by discovery, not by assumption.

**Degrade to nothing.** Every field here is optional at the source. Absent means the row
is not rendered. There are no placeholder rows, no "unknown", no em dash fillers.

---

## 1. Why three rows, and what they are

The Wikibooks Cookbook presents recipe metadata as an infobox under the image, and
`{{Recipe summary}}` is where servings, time and difficulty live. The corpus we are
importing in RUN-WIKIBOOKS-CORPUS carries these fields on most recipes, so the shape is
not invented: it is the shape the incoming data already has.

Three rows, in this order:

```
┌──────────────────────────┐
│                          │
│        recipe image      │
│                          │
├──────────────────────────┤
│ Serves      4            │
│ Time        30 minutes   │
│ Difficulty  ●●●○○ Average│
└──────────────────────────┘
```

Order rationale, and it is a real reason rather than a preference: serves is the field
that changes what you buy, time is the field that changes whether you cook it tonight,
difficulty is the field you check last and least. Most-consequential first.

---

## 2. Discovery gate (D0) — do this before writing any other code

Recipes are consumed from `exchange.recipe.recipe`, which is owned by recipe.exchange and
not by arecipe. arecipe's own records live under `app.arecipe.*`.

Read `LEXICONS.md` and the live record shapes and **report, before changing anything**:

1. Does `exchange.recipe.recipe` already define servings, yield, or difficulty? Under what
   names and types?
2. How is the existing prep time represented today: structured minutes, or free text?
   Quote the actual field definition.
3. What does the recipe page currently render for time, and where does it sit relative to
   the image?

Write findings to `runs/<runid>/D0-discovery.md`. Then branch:

- **Path A — the fields exist upstream.** Read and render them. No lexicon change. This is
  the happy path and the rest of the run is pure UI.

- **Path B — they do not exist.** Do not add them to `exchange.recipe.recipe`. Options,
  for the owner:
  - **B1** Propose the fields upstream to recipe.exchange and wait. Correct, slow, blocks
    this run.
  - **B2** An `app.arecipe.recipeMeta` sidecar record holding a strongRef to the recipe
    plus the three fields. Works for any recipe including other people's, costs a second
    record and a join on read.
  - **B3** Open-world optional fields written only onto records arecipe itself authors,
    including the Wikibooks import. Cheapest, but invisible on recipes authored elsewhere.

  **Recommendation: B3 now, B1 in parallel.** The immediate consumer is the Wikibooks
  corpus, which arecipe authors and therefore controls, and open-world fields on a record
  you author are within the protocol's grain. Raise B1 as a separate conversation with
  recipe.exchange so the field names converge rather than fork.

  **Owner decision O1:** confirm the path. Do not proceed past D0 on Path B without it.

---

## 3. Data model (D1)

Whatever the storage answer, the render layer takes one normalized view model:

```ts
type RecipeMeta = {
  serves?:     { display: string; hint?: { min: number; max?: number } };
  time?:       { display: string; hintMinutes?: number };
  difficulty?: { value: 1|2|3|4|5; label: string };
};
```

**`display` is authoritative for rendering. `hint` is only ever for sorting and
filtering.** This matters because the source values are free text: the `{{Recipe summary}}`
documentation itself shows `servings = 1-2`, `yield = 4 burgers`, `time = 30 minutes`.
Typing serves as a number loses "1-2" and, worse, quietly rewrites it as 1.

Difficulty labels follow the Cookbook policy's five-point wording:

| value | label |
|---|---|
| 1 | Very easy |
| 2 | Easy |
| 3 | Average |
| 4 | Hard |
| 5 | Very hard |

Out-of-range or non-numeric difficulty → field omitted entirely. Never clamp; a clamped
5 that was really garbage is worse than a missing row.

Yield versus servings: if only `yield` is present, render it in the serves row with the
yield text as the value ("4 burgers"). If both, serves wins and yield is dropped from the
strip. One row, one job.

**D1 tests:** the free-text cases from the template documentation round-trip unchanged;
"1-2" produces a hint of `{min:1,max:2}` and a display of "1-2"; "4 burgers" produces no
numeric hint; "30 minutes" produces `hintMinutes: 30`; "about an hour" produces a display
and no hint; difficulty 0, 6, "hard", and "" each omit the field.

---

## 4. Render (D2)

A single pure function, `renderMetaStrip(meta: RecipeMeta): HTMLElement | null`, returning
`null` when all three fields are absent. Callers must handle null by leaving the image
alone: an image with no strip keeps its normal corner treatment, and there is no empty
container.

### Markup

A description list, because that is what it is:

```html
<dl class="meta-strip">
  <div class="meta-row"><dt>Serves</dt><dd>4</dd></div>
  <div class="meta-row"><dt>Time</dt><dd>30 minutes</dd></div>
  <div class="meta-row">
    <dt>Difficulty</dt>
    <dd><span class="dots" aria-hidden="true">…</span><span>Average</span></dd>
  </div>
</dl>
```

Dots are decoration and are `aria-hidden`. The text label is the accessible value. A
screen reader hears "Difficulty, Average", not five list items.

### Visual

Follow the existing tokens; do not introduce a palette.

- The strip is attached to the image, not floating near it: no gap, the image's bottom
  corners square off where the strip meets it, and the strip carries the image's outer
  radius on its own bottom corners. It reads as one object.
- Background `--tile`. Rows separated by a hairline, not by whitespace, so the strip stays
  compact on a phone.
- Labels are the utility register: smaller, letter-spaced, lower emphasis. Values carry
  the weight. The label column is a fixed measure so the three values align on a common
  left edge; a ragged value column is what makes this pattern look homemade.
- **Difficulty dots use `--rust` (#b4552d, about 4.5:1 on `--tile`). They must not use
  `--yolk`,** which was already ruled to fail non-text contrast at roughly 2:1. Empty dots
  are outlined in the same stroke at reduced opacity, never filled with a lighter tint that
  drops below threshold.
- Height budget: the whole strip must not exceed 25% of the image height at a 390px
  viewport width. If it does, the type scale is wrong. Test this.

### Restraint

One accent, one place. The dots are the only expressive element in this component;
everything else is quiet. No icons on the label rows, no colour coding of difficulty
(green-to-red difficulty scales are both a cliché and an accessibility problem), no
animation on load.

**D2 tests:** all eight presence combinations of the three fields render correctly, with
the all-absent case returning null; row order is stable regardless of which fields are
present; the `dl` structure is correct and the dots carry `aria-hidden`; the accessible
name of the difficulty row contains the label text; a snapshot test on the generated
markup for each combination.

---

## 5. Placement and surfaces (D3)

- **Recipe page:** the strip goes directly under the recipe image. This is the only place
  it renders in this run.
- **Browse and Cookbook cards:** out of scope. Adding it there is a density decision that
  deserves its own look at the card layout. Note it as a follow-on.
- **Focus mode:** the strip renders, with the time row and serves row kept and difficulty
  suppressed. Rationale: difficulty is a pre-cook decision field and Focus mode is a
  during-cook surface. **Owner decision O2** if you disagree; the code takes a flag either
  way and the flag is tested in both positions.
- **Recipes with no image:** the strip renders standalone, at the top of the recipe body,
  with all four corners rounded. It must not look like an orphaned fragment. Test this
  case explicitly; a meaningful share of the Wikibooks corpus has no image, since only 802
  of roughly 3,600 recipes carry one.

---

## 6. Search, sort and filter (D4, small)

The `hint` fields exist so that a later run can sort by time and filter by difficulty
without re-parsing. In this run:

- Add the hints to the MiniSearch document shape so they are present in the index, but do
  **not** add filter UI. The unified search and filter chrome is its own in-flight piece
  of work and this run must not fork it.
- Add nothing to the reset behaviour. The shared reset icon ruling stands untouched.

**D4 tests:** indexed documents carry the hints; the existing search behaviour is
unchanged, proven by the existing suite still passing without modification.

---

## 7. Acceptance

1. D0 discovery report exists and O1 is answered before any lexicon or record change.
2. Every deliverable has a failing test in its history.
3. All eight presence combinations render, screenshotted at 390px and 1280px, with the
   screenshots in the run summary.
4. Contrast is measured, not asserted: report the computed ratio for the dots on `--tile`.
5. The no-image case and the Focus mode case are both shown.
6. No change to `exchange.recipe.recipe` unless the owner explicitly chose that path.
7. The run summary quotes the D0 findings verbatim, including the current prep-time field
   definition, so the owner can see what was actually there rather than what was assumed.

## 8. Out of scope

Card surfaces. Filter and sort UI. Nutrition and energy. Yield as its own row. Editing
these fields in the recipe editor, which is a follow-on once the storage path from O1 is
settled.
