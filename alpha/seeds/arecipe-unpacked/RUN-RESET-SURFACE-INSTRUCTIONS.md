# RUN-RESET-SURFACE — put `reset filters` back in sight

Small fix run for `CroftCommunity/arecipe`. One component, both pages inherit.

## Problem (owner-reported, screenshots 2026-07-15)

The toolbar-unification run placed the `reset filters` control INSIDE the
Filters ▾ popover (per its D7 spec — the spec was wrong, not the execution).
Result: with filters or a query active, the only visible signal is the badge
count on the closed chip, and clearing requires opening a disclosure the user
has no reason to open. Discoverability regression on Browse and Cookbook.

## Change (locked)

- Move the reset control OUT of the filters panel: render it as the compact
  text link (`reset-filters` testid preserved, `reset-filters-link` class
  preserved) inside the existing count block in the controls row, immediately
  before the honest count. Visible ONLY when the page reports an active
  filter/query (the existing `reflectReset(visible)` contract — no page API
  change).
- Remove it from the popover. House rule holds: no control appears twice.
- Row budget unchanged: the link lives in `rowControls`, so Browse stays at
  exactly 2 visible toolbar rows, Cookbook at 3 — existing assertions must
  keep passing unmodified.

## TDD (red first, per house convention)

1. RED — unit (`tests/unit/recipes/toolbar.spec.ts`): reset renders inside the
   count block, not inside the filters panel; hidden when `reflectReset(false)`;
   click fires `onReset`.
2. RED — e2e: update the deliberately-moved assertions from the toolbar run
   (`cookbook.spec` noted "reset lives inside Filters ▾") back to top-level
   visibility: with a facet active, `reset-filters` is visible WITHOUT opening
   `filters-dd`; clicking it clears facets, query, and badge. Enumerate every
   changed assertion in the run summary.
3. GREEN — move the element in `src/recipes/toolbar.ts`; CSS for the link's
   placement beside the count (no wrap → no third row at 390 px; extend
   `mobile-fit` overflow check if the count block needed style changes).
4. Full gate green; run summary with red → green order and the assertion list.

## Acceptance

1. With any filter or query active on Browse or Cookbook, `reset filters` is
   visible without opening Filters ▾; one tap restores the unfiltered feed and
   clears the search input and badge.
2. Inactive state shows no reset anywhere; popover no longer contains one.
3. Browse = 2 visible toolbar rows, Cookbook = 3, no horizontal overflow at
   ≤390 px; bundle-split guard and full gate green.
