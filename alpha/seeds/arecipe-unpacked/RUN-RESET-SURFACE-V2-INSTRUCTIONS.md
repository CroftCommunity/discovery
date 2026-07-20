# RUN-RESET-SURFACE v2 — surface `reset` + one shared reset icon (Browse, Cookbook, Meals)

Fix-and-polish run for `CroftCommunity/arecipe`. Supersedes RUN-RESET-SURFACE
v1 (unexecuted) — same visibility fix, now shipping WITH the shared reset icon
so the control is touched once, not twice. One run, one branch.

## 0. Problem + mission

The toolbar-unification run placed `reset filters` INSIDE the Filters ▾
popover (per its spec — the spec was wrong, not the execution). With filters
active, the only visible cue is the badge on a closed disclosure; clearing
takes two taps behind a control the user has no reason to open
(owner-reported, screenshots 2026-07-15).

Mission: (a) move reset back into sight on Browse and Cookbook; (b) replace
the three text reset controls — toolbar reset (both pages) and the Meals plan
resets — with ONE shared reset icon button: counterclockwise arrow-in-a-circle,
rust-colored, consistent everywhere something can be reset.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Failing tests before implementation; the run summary
   evidences red → green order. Deliberately-changed assertions (there are
   known ones — the toolbar run moved reset e2e expectations into the popover)
   are enumerated in the summary with reasons.
2. **Gate green** (`npm test`: lint + typecheck both tsconfigs + unit + build
   + e2e) at completion; browse bundle-split guard untouched.
3. **Style**: strict vanilla TS, shared pure helpers, DOM wiring guarded by
   e2e, module comments explain why. No raw hex outside `styles.css` tokens.
4. **Plan file** `plans/2026-07-XX-N-plan-reset-surface.md` before coding;
   Status updated at completion, house format.

## 2. Grounded context (verified in the post-cook-follows snapshot, 2026-07-15)

- `src/recipes/toolbar.ts`: `resetBtn` (`testid reset-filters`, class
  `reset-filters-link`) is appended to `filtersPanel` (inside the popover),
  hidden unless the page calls `reflectReset(true)`; honest count sits in
  `.browse-count` inside `rowControls`. Browse asserts exactly 2 visible
  `.toolbar-row`s, Cookbook counts 3 (`browse.spec.ts:463`,
  `cookbook.spec.ts:203`) — these must keep passing unmodified.
- Meals (`src/pages/meals.ts`): TWO text reset buttons, class
  `meals-reset-btn` — the header `renderResetControl` (`testid reset-plan`)
  and a per-plan variant (~line 1023) — BOTH confirm-gated ("Reset the
  plan? " → Confirm). The header row also hosts the calendar chip with a
  manual **Resync** action. `Reset on publish` is a settings checkbox, NOT in
  scope.
- Palette (`styles.css` `:root`): `--tile #f4f7f5`, `--yolk #e8a013`,
  `--yolk-deep #b87d0a`, `--rust #b4552d`, soft-tint precedent
  `--enamel-soft: #175e5414`.
- Inline-SVG precedent: `src/build-stamp.ts` builds its GitHub mark via
  `createElementNS`. `assets/icons/` holds app-icon PNGs — not for UI glyphs.
- `tests/e2e/mobile-fit.spec.ts` enforces ≥44px tap targets for a named list
  of browse controls + bottom nav.

Phase 0: re-ground this section against main (RUN-FOLLOW-THROUGH may have
landed); drift = FINDING, adapt details not decisions.

## 3. Locked design decisions

- **D1 The glyph.** Counterclockwise arrow-in-a-circle (reset/revert
  semantics). Clockwise stays RESERVED for refresh-type actions — the Meals
  header's calendar **Resync** shares a row with reset, so the direction
  distinction is load-bearing. Never render a clockwise variant for reset.
- **D2 One helper.** New `src/icons.ts` (zero-dep): `resetIcon()` returns an
  inline SVG (createElementNS, `stroke: currentColor`, `fill: none`,
  `aria-hidden="true"`, viewBox'd so CSS sizes it), plus
  `resetIconButton(label: string)` returning a `<button type="button">`
  containing the icon with `aria-label` and `title` set to `label`. All three
  sites consume the helper; no site draws its own arrow.
- **D3 Color, contrast-guarded.** Icon color `var(--rust)` — measured ~4.5:1
  against `--tile`, passing WCAG non-text 3:1 with margin (`--yolk` FAILS at
  ~2:1 and must not be used for icon strokes; `--yolk-deep` is borderline
  ~3.2:1). Pressed/hover state: soft rust tint background — new token
  `--rust-soft: #b4552d14` in `styles.css`, mirroring `--enamel-soft`'s alpha
  pattern. A PERMANENT unit guard: a contrast spec that reads `styles.css`,
  parses `--rust` and `--tile`, computes the WCAG ratio (pure math util in the
  test), and asserts ≥ 3:1 — so a future palette tweak can't silently make
  reset illegible.
- **D4 Toolbar reset (Browse + Cookbook).** The reset control moves OUT of
  the filters panel into the `.browse-count` block in `rowControls`,
  immediately before the honest count. It becomes
  `resetIconButton('reset filters')`, keeping `testid reset-filters` and the
  existing `reflectReset(visible)` contract (no page API change). Appears only
  when a filter/query is active — its contextual appearance is the
  discoverability mitigation for going icon-only. The popover loses its copy
  (house rule: no control twice). Row budgets unchanged: Browse 2, Cookbook 3.
- **D5 Meals resets.** Both `meals-reset-btn` instances become
  `resetIconButton('Reset plan')` (header keeps `testid reset-plan`; per-plan
  variant keeps its current testid — verify in Phase 0). The confirm flow is
  UNTOUCHED: tapping the icon still swaps to "Reset the plan? " + Confirm.
  Destructive weight lives in the confirm step, not the glyph.
- **D6 Tap targets + a11y.** Rendered hit area ≥44px at mobile widths for all
  three sites (CSS padding, not a bigger glyph — visual glyph ~16-18px).
  Extend mobile-fit's tap-target list with `reset-filters` (active state) and
  `reset-plan`. `aria-label`/`title` mandatory via the helper.
- **D7 Fallback, deferred not built.** If icon-only tests badly in use, the
  contingency is a responsive label (icon + `reset` text ≥480px, icon-only
  below). Record under Deferred in the plan file; do NOT build it this run.

## 4. Phases

### Phase 0 — re-ground
Confirm §2 against main; record drift as FINDINGS (esp. the per-plan meals
reset testid and any toolbar changes from intervening runs).

### Phase 1 — icon helper + tokens (pure)
RED `tests/unit/icons.spec.ts`: `resetIcon()` yields an `svg` with
`aria-hidden`, currentColor stroke, no fill; `resetIconButton('x')` yields a
button with aria-label + title `x` containing exactly that svg;
counterclockwise geometry pinned (assert the path/marker data equals the
committed constant — a change to the glyph must be a deliberate test edit).
RED contrast guard per D3 (parse styles.css, ratio ≥3:1; also assert
`--rust-soft` exists once added).
GREEN: `src/icons.ts`, `--rust-soft` token, shared `.reset-icon-btn` styles.

### Phase 2 — toolbar relocation (Browse + Cookbook)
RED unit (`toolbar.spec.ts`): reset renders inside `.browse-count` before the
status, NOT inside the filters panel; hidden via `reflectReset(false)`; click
fires `onReset`; it is the helper's button (aria-label present).
RED e2e: revert the toolbar-run assertions — with a facet active,
`reset-filters` visible WITHOUT opening `filters-dd`; one tap clears facets,
query, input, and badge; inactive state shows no reset anywhere; row-count
assertions pass unmodified.
GREEN: move + restyle.

### Phase 3 — meals resets
RED (meals unit/e2e per existing coverage style): both reset sites render the
icon button with labels; confirm flow byte-identical in behavior (tap →
note + Confirm/Cancel; Confirm still resets; Cancel restores); Resync chip
unaffected.
GREEN: swap both buttons to the helper.

### Phase 4 — mobile + closeout
Extend mobile-fit tap-target list (D6); full gate; plan-file Status; run
summary with red → green order, the enumerated assertion changes, and the
contrast numbers as measured by the new guard.

## 5. Acceptance criteria

1. With any filter or query active on Browse or Cookbook, a rust
   counterclockwise reset icon is visible beside the honest count without
   opening Filters ▾; one tap restores the unfiltered feed; inactive state
   shows no reset; the popover contains none.
2. Both Meals resets use the same icon; the confirm gate is behaviorally
   unchanged; Resync remains visually distinct (no clockwise reset anywhere).
3. All reset glyphs stroke `var(--rust)`; the committed contrast guard proves
   ≥3:1 against `--tile` and fails the build if the palette drifts below it.
4. Tap targets ≥44px at mobile widths for all three sites; aria-label/title
   present everywhere via the single helper.
5. Browse 2 / Cookbook 3 visible toolbar rows, no horizontal overflow ≤390px,
   bundle-split guard green, full gate green.
