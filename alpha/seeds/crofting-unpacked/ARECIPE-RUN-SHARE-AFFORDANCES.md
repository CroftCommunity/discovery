# ARECIPE RUN: share affordances (recipe + cookbook)

Target repo: `CroftCommunity/arecipe` (main). Execute on a fresh branch
`run-share-affordances` off `main`. Write `RUN-SHARE-SUMMARY.md`. Do not
merge; leave for review.

## Why

Both share URLs already exist; neither has a one-tap affordance:

- `recipe.html?u=<at-uri>[&by=<handle>]` is a real, shareable document
  (stated at the top of `src/pages/recipe.ts`).
- `cookbook.html?did=<did>` is a shareable, public cold-view of any account's
  recipe feed (stated in `src/pages/cookbook.ts`).

This run adds the buttons, nothing else. No new pages, no new lexicon
records, no new network calls, no new origins, no CSP change, no new
dependencies.

## Conventions that govern this run

- Read `docs/PRACTICES.md` and `CLAUDE.md` first and follow them where they
  differ from anything here.
- TDD-first, standing directive: acceptance criteria below become failing
  tests BEFORE implementation; fixtures before features; the summary shows
  red-to-green order with the actual outputs.
- The hermetic gate (`npm test`: lint + typecheck + unit + build + e2e) must
  pass at the end. No `@live` tests in this run.
- Mirror the existing clipboard affordance pattern in `src/recipes/view.ts`
  (the copy-to-clipboard button: `data-copy` payload read by a click handler,
  `navigator.clipboard.writeText`, transient confirmation) rather than
  inventing a second idiom. Mirror existing e2e patterns (see
  `tests/e2e/cookbook.spec.ts`, `tests/e2e/interactions.spec.ts`) for
  clipboard assertions and `data-testid` naming.
- Keep the recipe entry bundle light: do NOT add any static import that pulls
  `@atproto/api` into `recipe.html` (the file's own NOTE explains the split-
  chunk discipline; respect it).

## Phase 0: read before writing

Read `src/pages/recipe.ts`, `src/pages/cookbook.ts`, `src/recipes/view.ts`,
and one passing e2e spec for each page. Record in the summary: where the
canonical recipe URL parameters come from on the recipe page, and where the
viewed cookbook's DID is known on the cookbook page (both the signed-in own
view and the `?did=` cold view).

## Phase 1 (RED): tests first

1. Unit (Vitest): a new pure module `src/share/urls.ts` with
   `buildRecipeShareUrl(origin, atUri, handle?)` and
   `buildCookbookShareUrl(origin, did)`. Write the unit tests FIRST against
   the intended signatures: canonical param encoding (at-uri and DID must be
   URL-encoded exactly once), `by` included only when a handle is present,
   origin passed in rather than read from globals (pure function). Run: red.

2. E2E (Playwright, hermetic): 
   - `tests/e2e/recipe-share.spec.ts`: the recipe page shows a share button
     (`data-testid="share-recipe"`); activating it puts the canonical
     `recipe.html?u=...` URL for the currently viewed recipe on the clipboard
     and shows a transient confirmation. 
   - `tests/e2e/cookbook-share.spec.ts`: the cookbook page shows a share
     button (`data-testid="share-cookbook"`) on the `?did=` cold view, and it
     copies the canonical `cookbook.html?did=...` URL for the cookbook being
     viewed; assert the same button exists on the signed-in own-cookbook view
     only if the hermetic harness already fakes a session (follow whatever
     the existing hermetic cookbook spec does; if own-view requires `@live`,
     scope the own-view assertion out and say so in the summary).
   Run: red. Record output.

## Phase 2 (GREEN): implementation

- Implement `src/share/urls.ts` to the unit tests.
- Recipe page: mount the share button near the recipe title, wired to the
  canonical URL built from the page's own `u`/`by` params (normalized, not
  echoed raw). Behavior: if `navigator.share` exists, call it with the recipe
  title and URL; otherwise fall back to the clipboard pattern. In BOTH cases
  the clipboard fallback path is what the hermetic e2e asserts (Playwright
  environments don't expose the native share sheet; feature-detect so the
  test exercises the fallback deterministically — follow the existing
  pattern if one exists, otherwise gate on `typeof navigator.share`).
- Cookbook page: same affordance, labeled for the cookbook, using the DID of
  whichever cookbook is being viewed.
- Accessibility: real `<button>`, `aria-label`, visible focus, confirmation
  text announced via the same mechanism the existing copy button uses.
- No CSP or build.mjs changes should be needed; if one turns out to be, STOP
  and record why instead of making it.

## Phase 3: gate and summary

`npm test` fully green. RUN-SHARE-SUMMARY.md records: Phase 0 findings, the
red outputs, the green outputs, files touched, the feature-detect approach
for `navigator.share`, and anything scoped out (with the reason).

## Deliberately out of scope (do not do)

- Share previews / OG meta work (tracked separately in docs/PREVIEWS.md).
- Any change to the `?did=` or `?u=` URL formats themselves.
- Share buttons on dish.html, meals.html, or plan pages (future runs).
