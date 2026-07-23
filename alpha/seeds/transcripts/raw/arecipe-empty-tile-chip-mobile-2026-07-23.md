# Raw: arecipe — empty-tile chip (mobile vertical real estate) (2026-07-23)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) for the
dialogue; the RUN-EMPTY-TILE-CHIP instruction file below is preserved VERBATIM (it was pasted in full) —
PLAYBOOK §4.** Source: a pasted Claude chat session, 2026-07-23. UI render chrome stripped (thinking-summary
headers, "Code · HTML" / "Document · MD" markers, "Read and edited memory"). No secrets. Part of the
2026-07-23 batch (plan: `alpha/plans/2026-07-23-transcript-intake-socialization-arecipe-measurement.md`).

**Flags.** Design/UX — low fact-check burden. Measurement claims (photo tiles ~16:7, placeholder ~3:2) are
eyeballed off a production screenshot, `[UNVERIFIED]` but low-stakes. arecipe-repo work
(`CroftCommunity/arecipe`); the instruction file's execution lives there. A design-comparison HTML artifact
("Arecipe empty tile options") was produced in-session (not captured here).

---

## The dialogue

**User:** In tiles view on arecipe.app, recipes without pictures have the nice standing graphic and that's
great, but the tile is still image-size and it's not really useful screen-usage-wise. Can we shrink it
vertically to make the experience better but still keep the UI consistent feel and good UX?

**Assistant:** Yes — keep the media zone but make it a short band rather than deleting it. The tile still
reads as the same object (media area, then title block); it just stops pretending to be a photo. Concrete
shape: image tiles keep their aspect ratio; placeholder tiles get a fixed short height (72–96px, or
`aspect-ratio: 16 / 5` to stay proportional across breakpoints); scale the cutlery glyph down (36–44px) so
it sells "deliberate placeholder" not "image failed to load"; everything else identical (tint, radius, title
padding, type — consistency lives in shared tokens, not matching heights). The one thing that will bite: if
the grid uses uniform row heights, shortening the placeholder buys nothing because the row sizes to the
tallest sibling — set `align-items: start` on the grid container. A more aggressive variant: drop the band
and put the glyph in a small square chip to the left of the title (avatar style) — most compact, but it
changes the tile silhouette so scanning a mixed grid gets less rhythmic. The short band keeps vertical rhythm
and recovers most of the space — better for a browse grid.

**User:** Can you show me what you mean?

**Assistant:** Three phone mocks side by side, plus two live controls: a slider for band height and a
checkbox that demonstrates the grid caveat by breaking it on purpose. One thing I found measuring the
screenshot: the photo tiles crop to about 16:7, but the placeholder box is about 3:2 — so the empty tile is
not merely as tall as a photo tile, it is meaningfully taller than one. Matching the photo ratio alone
already recovers a third; the short band recovers most of the rest. (Colors/type eyeballed, not from tokens;
treat the glyph as a stand-in for the real SVG.) [Artifact: "Arecipe empty tile options" — HTML.]

**User:** I like the inline chip example, and understood that it's contextual — this is mostly a mobile
optimization for vertical screen real estate.

**Assistant:** Good pick, and the mobile framing resolves the objection. The uneven-silhouette cost only
bites in a multi-column grid where tiles sit in a visual rhythm; in a single-column feed you're scanning a
vertical list of titles anyway, so the chip just gets more titles per screen. Scope it by breakpoint, not by
tile state alone:
- **Single column (mobile):** chip. Vertical space is scarce.
- **Multi-column (tablet up):** keep a media zone, sized to the photo crop ratio or the short band.
  Horizontal space is the constraint; rows want to stay even; the empty tile no longer costs a screenful.
One media query around the empty-state rules, photo tile untouched at every width. Two details to pin: chip
size ≥ 44px so the tile row clears the touch-target floor (glyph stays `aria-hidden`, title carries the
accessible name); long titles need a line clamp (2–3 lines) or a `Gluehweingewuerz`-length name makes the
chip tile taller than the band it replaced.

**User:** Create a comprehensive Claude Code instruction file to execute this.

**Assistant:** Written with a D0 discovery gate up front (I don't know your tile render call sites, your
has-image predicate, or your existing breakpoint, and inventing a new breakpoint would be the easiest way to
quietly break the grid). Three things worth knowing: the photo-tile guard is a snapshot taken before any
edit (else "photo tiles unchanged" is untestable), sequenced first in Phase 1; the multi-column ratio
correction is carved out as a separate revertable rule flagged for review (it fixes the 3:2-vs-16:7 mismatch,
but you ruled on the chip, not that); Phase 4 makes the run falsifiable — if measured tiles-per-screen at
360px doesn't improve, stop and report rather than ship. [File below, verbatim.]

---

## RUN-EMPTY-TILE-CHIP (verbatim)

Repo: `CroftCommunity/arecipe`
Surface: recipe tiles on Browse and Cookbook (and any other tile call sites found in D0)
Owner ruling date: 2026-07-23

### Goal

Recipes without a picture currently render the placeholder cutlery graphic inside a media box that is
**taller than the photo tiles it sits next to**. Measured off a production Android screenshot at 2026-07-23:
photo tiles crop to roughly 16:7, the placeholder box is roughly 3:2. The empty state therefore costs more
vertical space than a real photo does, which is backwards.

Fix it by replacing the media band with an **inline chip** on single-column layouts: the existing glyph in a
small rounded square to the left of the title, no media zone at all.

This is a mobile vertical-real-estate optimization. It is not a redesign of the tile.

### Scope

In scope: a layout-aware variant decision for pictureless recipe tiles; chip rendering and chip CSS at the
single-column breakpoint; correcting the multi-column empty media box to match the photo aspect ratio (see
"Owner review"); tests, measurement, and a11y verification for the above.

Out of scope, do not touch: photo tiles at any width (markup, ratio, crop, credit line stay exactly as they
are); the placeholder SVG artwork itself (reuse, do not redraw); any new color, radius, shadow, or type token
(everything comes from tokens that already exist); the Meals page recipe palette, the recipe detail page, and
the meta strip work in RUN-RECIPE-META-STRIP; any new dependency (CSS framework, icon package, or a test
runner not already in `package.json`).

### Decisions already made, do not relitigate

1. Pictureless tiles use the **inline chip** at single-column widths. The short-band treatment was considered
   and rejected for mobile.
2. The chip uses the **same glyph, same tint, same corner radius family, same title type** as today. Family
   identity lives in shared tokens, not matching heights.
3. Multi-column widths **keep a media zone**. The chip is not applied there.
4. No new animation. Respect `prefers-reduced-motion` if you touch anything that already animates.

### Owner review required, do not ship silently

The multi-column empty media box is currently ~3:2 while photos are ~16:7. Correcting it to the photo ratio
is a genuine bug fix but was **not** explicitly ruled by the owner. Implement it as a **separate, single,
independently revertable CSS rule** and call it out at the top of the run summary with a before/after height
in px at 1024px viewport width. If the owner rejects it, deleting that one rule must fully restore current
behavior without affecting the chip work.

### Phase D0: discovery gate (STOP before writing any code)

Report findings and wait. Do not begin Phase 1 until D0 is answered in the run summary. Find and report, with
file paths and line references:
- **D0.1** Where a recipe tile is rendered. No framework, no router — expect a shared helper plus per-page
  call sites. Enumerate every call site (the cook-preview list view added by add-cook is a likely third).
- **D0.2** How "has a picture" is determined. Report the exact predicate; note whether it distinguishes "no
  image recorded" from "image recorded but failed to load" (the chip must cover the first; the second needs
  an explicit decision — D0.7).
- **D0.3** The tile CSS: which stylesheet, which selectors set the media aspect ratio, which tokens for the
  placeholder tint and glyph stroke.
- **D0.4** The existing breakpoint where the tile grid goes one→multi column. Report the exact media query.
  **Reuse it. Do not invent a new breakpoint value.**
- **D0.5** The shared inline-SVG helper introduced by the reset-icon work. The chip must go through it, not a
  second copy of the glyph markup.
- **D0.6** The test setup: runner, whether a DOM environment (jsdom) is available, whether Playwright is
  already a dev dependency. Determines which Phase-1 tests are automated vs documented manual checks. Do not
  add a runner to make tests convenient.
- **D0.7** Whether the tile link's accessible name comes from the title text, an `aria-label`, or the image
  `alt`. The chip removes an image element, so if the accessible name depends on `alt` anywhere, that is a
  regression risk to fix in this run.

If D0 reveals tiles are rendered by string concatenation in more than one place with drifted markup, **say so
and stop** — consolidating them is a larger change than this run authorizes.

### Phase 1: tests first (mandatory, red before green)

TDD is a standing requirement. Write these as failing tests, capture the red output, then implement. The run
summary must show red-then-green evidence.
- **1.1 Variant decision (pure, always automatable).** Extract `tileMediaVariant({ hasImage, columns })` →
  `'photo' | 'chip' | 'band'`. Cases: has image + single → `'photo'`; has image + multi → `'photo'`; no image
  + single → `'chip'`; no image + multi → `'band'`. No DOM dependency.
- **1.2 Rendered structure.** If D0.6 finds a DOM env, assert against the rendered node; else produce a
  serializable view-model and assert on that, covering DOM writing in Phase 3's visual check. Chip variant
  emits no media-band element; exactly one glyph via the D0.5 helper; glyph `aria-hidden="true"` contributing
  nothing to the accessible name; the tile link's accessible name equals the full title (including a clamped
  long title); photo variant markup byte-identical to current output (snapshot before changing anything).
- **1.3 Title clamp.** A pathologically long title (`Mulled Wine Spice (Gluehweingewuerz)` and one ~twice
  that) still renders its complete text in the DOM. The clamp is visual only (`-webkit-line-clamp`); text
  stays present for assistive tech.
- **1.4 Regression guard.** A fixture list mixing photo and pictureless recipes renders the same number of
  tiles in the same order as before.

### Phase 2: implementation

- Chip square: minimum **44×44 CSS px** (the touch-target floor set by the reset-icon work). Chip is inside
  the tile link, so the whole row is tappable, but the chip itself must not drop the row below the floor.
- Chip glyph: scale down proportionally, ~26px inside a 46px square. Must read as deliberate, not a cropped
  or failed image.
- Chip background: the existing placeholder tint token. Chip radius: a smaller sibling of the tile radius,
  derived from the existing radius token, not a new magic number.
- Do not introduce `--yolk` into any new load-bearing role (it fails non-text contrast at ~2:1 on `--tile`).
- Title in the chip row: clamp to **3 lines**. Keep current font family, weight, size.
- Row layout: flex, vertically centered, gap derived from existing tile padding scale.
- Gate the chip behind the D0.4 media query. Above that breakpoint the empty tile keeps a media zone.
- The multi-column ratio correction goes in its own clearly commented rule.
- While in the grid CSS: verify whether the tile grid uses uniform row heights; if empty and photo tiles ever
  differ in height at multi-column widths, `align-items: start` on the grid container is required or the
  shorter tile stretches back out. Report what you found.

### Phase 3: layout and accessibility verification

At viewport widths **360, 390, 768, 1024, 1280**, with a fixture feed alternating photo and pictureless
recipes, verify and capture evidence: chip appears at and only at single-column widths; no horizontal
overflow at 360px with the longest title; visible keyboard focus ring unchanged and not clipped by the new
flex row; accessible name of each tile link is the recipe title (checked in the a11y tree, not inferred);
glyph-stroke contrast against the chip tint unchanged (state the ratio); no layout shift on load (confirm CLS
not made worse). Automate with Playwright if available (D0.6); else manual with screenshots.

### Phase 4: required measurement table

Fill with real numbers at **360px width**, not estimates: height of one pictureless tile; height of one photo
tile; tiles fully visible in an 800px-tall viewport (alternating feed); scroll distance to reach the 10th
recipe. Also report the multi-column empty tile height at 1024px, before/after, for the owner-review item. If
the "after" numbers don't show a clear tiles-per-screen improvement, stop and report rather than shipping.

### Acceptance criteria

D0 reported and gate passed before implementation; every Phase-1 test written first, observed failing, now
passing (red captured); photo tiles provably unchanged at every width (pre-change snapshot); chip at
single-column only, using the pre-existing breakpoint; chip square ≥ 44×44px; glyph `aria-hidden`, from the
shared helper, accessible name is the title; long titles clamp visually at 3 lines with full text in the DOM;
no new dependency/token/breakpoint; measurement table filled; multi-column ratio correction isolated in one
revertable rule and flagged at the top; `prefers-reduced-motion` respected, no new animation.

### Rollback

Two independent pieces: (1) delete the chip CSS block and make `tileMediaVariant` never return `'chip'` →
tiles return to today's behavior at every width; (2) delete the single commented multi-column ratio rule
(independent of 1). State in the summary that both reverts were verified, or why not.

### Parallel safety and rebase notes

RUN-RECIPE-META-STRIP touches the recipe detail page image area (different surface; if it also edits the
shared stylesheet, expect a conflict — do not resolve by reformatting the other run's rules). RUN-BUNDLE-
PRECACHE touches `scripts/build.mjs` and `assets/` — this run should touch neither (stop and explain if you
find yourself editing the build script). RUN-RESET-SURFACE owns the shared inline-SVG helper — consume it, do
not restructure its signature (propose changes in the summary instead of making them unilaterally). Rebase
onto `main` before opening the PR and re-run the full suite.

### Run summary requirements

Lead with the owner-review item. Then: D0 findings with file paths; red-then-green test evidence; the filled
measurement table; screenshots at the five widths, before/after; the `align-items` finding; anything you were
told not to relitigate that you nonetheless think is wrong, stated plainly at the end rather than acted on.
