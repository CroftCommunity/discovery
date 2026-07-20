# RUN-COOK-FOLLOWS — cook follows + unified search/filter toolbar (Browse, Cookbook, Account)

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Execute top to bottom. [verify-in-run] items must be probed empirically and the
outcome recorded in the run summary before code depends on them. Anything that
contradicts this file's grounded claims is a FINDING (report it, don't silently
adapt).

## 0. Mission

Two coupled changes, one run:

**A. Cook follows.** The Browse handle-lookup stops being a feed-replacing
search and becomes a **look-up-then-follow** flow: looking up a cook shows a
PREVIEW of that cook's recipes (this-list-only — open a recipe, come back,
still their list); a **Follow** action from the preview adds them to your
cooks, so the default feed (after reset/back) includes them, merged. Signed
out, Follow persists to a durable device-local store (never the PDS). Signed
in, Follow writes a public per-cook record in the `app.arecipe` namespace —
these are "cook follows", arecipe's analog of `app.bsky.graph.follow`, in the
same spirit as the starter pack being curated from the arecipe Bluesky
account. The Account page's members listing gains the same add mechanism at
the top, an `added` badge, and per-row unfollow.

**B. Toolbar unification.** Browse and Cookbook currently stack ~4 rows of
mixed-idiom controls before content (worst on mobile). Restructure both to a
shared contract: content search is THE primary search on row 1; filters
collapse behind one disclosure with a count badge.

## 1. Standing conventions (non-negotiable)

1. **TDD, red first.** Every phase writes its failing tests BEFORE
   implementation; fixtures before the features that consume them; acceptance
   criteria (§6) are encoded as tests; the run summary evidences red → green
   order per phase. Implementation without a preceding failing test is a
   defect in the run.
2. **Repo gate** (`npm test` = lint + typecheck both tsconfigs + unit + build
   + e2e) green at the END OF EVERY PHASE.
3. **Style.** Vanilla strict TS, no framework, pure cores with injectable
   dependencies (model: `src/pages/browse-state.ts`, `src/social/reach.ts`),
   DOM wiring in pages/toolbar guarded by e2e, module comments explain why.
4. **Bundle split.** `browse.html` ships ZERO auth code (existing e2e guard).
   This constrains the design (D5) — do not weaken the guard to ship this run.
5. **Plan file** `plans/2026-07-XX-N-plan-cook-follows-and-toolbar.md` written
   before Phase 1 (problem, locked decisions, phases); Status line updated at
   completion, matching `plans/2026-07-13-1-plan-multi-meal-scheduling.md`.

## 2. Grounded context — verified in a PRE-SEARCH-RUN snapshot; re-verify in Phase 0

The analysis below was verified against a snapshot taken BEFORE the recipe-
text-search run merged. Structure is expected to hold; the toolbar has since
gained a `recipe-search` input. Phase 0 re-grounds against main.

- **Browse** (`src/pages/browse.ts`): `runFind(handle)` resolves → reads that
  author's whole repo → sets `current = { kind: 'search', entries, author }`,
  REPLACING the feed; the starter feed is `current.kind === 'starter'`. Last
  find survives navigation via sessionStorage `last-find`. This existing
  search-feed modality IS the preview mode — Part A largely re-frames it.
- **Starter pack** (`src/recipes/starter.ts`): hardcoded `STARTER_AUTHORS`
  + localStorage disabled-set prefs (`starter-pack-disabled`). The
  store-the-exception defensive posture is the pattern for the new store.
- **Cookbook membership** (`src/social/cookbook.ts` `resolveCookbook`):
  sources `you | starter | follow | follower`; bsky follows are read via
  `listRecords?collection=app.bsky.graph.follow` on YOUR OWN PDS
  (unauthenticated, CORS-open — verified 2026-07-08 per module comment; do
  not re-probe). Members render on Account via
  `src/social/cookbook-members-view.ts` (`mountMembersList`, source badges).
- **Reach prefs** (`src/social/reach.ts`): localStorage, ReachConfig
  `{starters, follows, followers}` — gains `added`.
- **Toolbar** (`src/recipes/toolbar.ts`): shared by both pages; testids
  `view-tiles`, `view-details`, `photos-only`, `recipes-status`,
  `reset-filters` (+ `recipe-search` post-search-run). Facet dropdowns use the
  `details.facet-dd` popover idiom; a count-bubble idiom exists in Account's
  taste prefs (`facet-count`).
- **Write path**: `src/recipes/write.ts` (createRecord/deleteRecord patterns),
  auth via `src/auth/boot.ts` — auth-bearing pages only.
- **Lexicons**: registry `docs/LEXICONS.md`; fixture lexicons under
  `tests/fixtures/lexicons/` (e.g. `app.arecipe.mealPlan.json`).
- **E2E**: hermetic routed fixtures (`tests/e2e/browse.spec.ts` +
  `tests/fixtures/atproto/listRecords-browse-mixed.json`, 4 recipes);
  `mobile-fit.spec.ts` exists for viewport checks; `cookbook.spec.ts`,
  `account.spec.ts` are the other regression guards this run touches.

## 3. Locked design decisions

- **D1 Lookup = preview.** Selecting a cook (typeahead or submit) shows THAT
  cook's recipes only — preserve the existing `kind:'search'` feed behavior
  including open-recipe-and-return and `last-find` restore. The preview's
  status area gains a **Follow / Following** control (testid `follow-cook`)
  and the cook's handle. Reset / navigating back to the default feed leaves
  the preview.
- **D2 Follow = merge into the pile.** The default Browse feed becomes
  starter-pack cooks + followed cooks, merged (loaded via the existing
  `loadAuthorsFeed` multi-author path). After following, the previewed cook
  appears in the default feed on the next reset/return. Unfollow is available
  on Account (and toggling Follow off in a preview).
- **D3 The record: one per cook, public.** New collection
  `app.arecipe.cookFollow` (confirm final NSID against `docs/LEXICONS.md`
  conventions in Phase 0; keep the flat `app.arecipe.*` style of
  `app.arecipe.mealPlan`). Record value mirrors `app.bsky.graph.follow`:
  `{ subject: <did>, createdAt: <datetime> }`. Add = createRecord, unfollow =
  deleteRecord (rkey lookup by subject), list = `listRecords` on your own PDS
  — the identical read path the cookbook resolver already uses for bsky
  follows. Lexicon JSON added to `tests/fixtures/lexicons/` and registered in
  `docs/LEXICONS.md` BEFORE the write path is implemented (fixtures before
  features).
- **D4 Signed-out tier.** `src/social/cook-follows-local.ts`: durable
  localStorage store of `{ did, handle }[]` with the repo's defensive posture
  (read failure → empty, write failure → silent degrade). Never touches a PDS.
- **D5 Local store = the universal read model.** Browse (zero-auth bundle)
  ONLY reads the local store. Signed-in pages (Account, Cookbook) sync PDS
  cookFollow records → local store on load (mirror down). This keeps the
  bundle-split guard intact and makes signed-out behavior fall out for free.
- **D6 Sign-in publish is offered, never silent.** When a session exists and
  the local store holds follows with no matching PDS record, Account shows a
  one-confirm offer listing the handles ("Publish N saved cooks as follows?").
  Local-to-public is a visibility change the user must see. Declining keeps
  them local (offer remains available, not nagging — a dismissible section).
- **D7 Toolbar contract, both pages.**
  Row 1: full-width `recipe-search` input; Browse adds compact `+ Cook`
  (testid `add-cook`, opens an inline panel — the export-panel idiom — housing
  the existing actor typeahead) and the export `↑`; Cookbook adds nothing
  (New Recipe stays in the page header).
  Row 2 (Cookbook only): the `Mine | Liked | All` segmented control.
  Row 3: `Tiles | Details` + ONE `Filters ▾` disclosure (`details.facet-dd`
  idiom, testid `filters-dd`) with a `facet-count` badge showing the number of
  active filters; the honest count (`N of M shown`) right-aligned OUTSIDE the
  disclosure. Popover contents: Photos only, Meal, Cuisine, diet-preference
  link (Browse only), reset. No control appears in two places.
- **D8 Members view.** Account listing gains the shared add panel at top,
  `added` source badge (extend `SOURCE_LABEL`/`SOURCE_ORDER`; priority after
  `starter`, before `follow`), and per-row unfollow for `added` members
  (deleteRecord signed-in + local removal; local-only rows removable too).
- **D9 Deferred** (list verbatim in the plan file): follows-of-follows reach
  source; per-author facet/filter in the merged feed; relocating export;
  starter-pack UI changes.

## 4. Phases

### Phase 0 — ground against main
Read the current `toolbar.ts`, `browse.ts`, `cookbook.ts`, `account.ts`,
`docs/LEXICONS.md` on main (post-search-run). Record in the run summary: the
current toolbar row structure and testids, the search input's placement, and
any drift from §2. Drift = FINDING + adapt the phase details, not the locked
decisions.

### Phase 1 — pure core: cook-follows stores
RED `tests/unit/social/cook-follows-local.spec.ts`: add/list/remove/has;
duplicate add is idempotent; corrupt storage reads as empty; write failure
degrades silently; `{did, handle}` shape preserved.
RED `tests/unit/social/cook-follows-pds.spec.ts` (fetch-fake pattern from
`tests/unit/recipes/read.spec.ts`): list parses `listRecords` pages of
cookFollow records; follow issues createRecord with `{subject, createdAt}`;
unfollow resolves rkey by subject then deleteRecord; mirror-down writes the
local store from PDS state (and does not erase local-only rows pending D6).
GREEN: implement `cook-follows-local.ts` + `cook-follows-pds.ts`; lexicon
fixture + `docs/LEXICONS.md` row land first.

### Phase 2 — membership integration
RED: `resolveCookbook` unit coverage for the `added` source (enabled/disabled
via ReachConfig; degrade-not-blank on failure; source-priority order you →
starter → added → follow → follower); members-view badge + unfollow row.
GREEN: wire resolver, reach prefs, members view, Account add-panel mount, and
the D6 publish offer.

### Phase 3 — Browse preview + follow + merged default feed
RED: unit tests for the default-feed composition (starter + local follows,
deduped by DID); preview state exposes the followed/not-followed flag; follow
from preview updates the store; e2e (Phase 5) cases drafted now.
GREEN: `+ Cook` panel, preview status Follow control, merged default feed via
`loadAuthorsFeed`, `last-find` behavior preserved.

### Phase 4 — toolbar restructure (both pages)
RED: update/extend toolbar unit + e2e expectations for D7 DELIBERATELY —
this run intends behavior change, so failing-then-updated specs must be
enumerated in the run summary (which assertions changed and why). New testids
per D7; existing testids preserved wherever the control's semantics are
unchanged.
GREEN: restructure `renderToolbar` + both pages' wiring + CSS.

### Phase 5 — e2e + mobile
Hermetic routed fixtures throughout. New/extended specs (RED first):
- lookup shows preview only (fixture cook's recipes, not starter's); open a
  recipe, go back, preview persists; Follow → reset → default feed includes
  the cook's recipes (route a second author fixture — extend fixtures FIRST).
- signed-out follow survives reload (local store); no PDS write occurs
  (assert no createRecord request was routed).
- Account: add at top of listing, `added` badge, unfollow removes from list
  and default feed; D6 offer appears when local-only follows exist with a
  session.
- Filters ▾: badge count reflects active filters; popover contains exactly
  D7's contents; reset clears and badge hides; honest count outside.
- `mobile-fit.spec.ts` extension: at a ≤390 px viewport, Browse shows at most
  TWO control rows before the first recipe card and no horizontal overflow;
  Cookbook at most three.
- bundle-split guard still green.

### Phase 6 — docs + closeout
`docs/LEXICONS.md` (done in Phase 1, re-check), plan file Status, run summary:
red → green evidence per phase, Phase 0 drift findings, the deliberately-
changed e2e assertions list, and any [verify-in-run] outcomes.

## 5. [verify-in-run] ledger
1. Final NSID for the cook-follow collection per LEXICONS.md conventions.
2. That `listRecords` for a novel `app.arecipe.*` collection behaves on the
   live PDS as it does for mealPlan (expected yes; confirm in a live-tagged
   test or record as assumption if live tests are out of scope this run).
3. Where the search-run left the toolbar rows (Phase 0) and whether the
   Filters ▾ disclosure needs the exclusive-accordion `name` attribute used by
   `taste-never` to avoid popover stacking.

## 6. Acceptance criteria (each maps to a named test)
1. Looking up a cook previews their list only; recipe round-trip keeps it.
2. Follow merges the cook into the default feed after reset/back.
3. Signed out: follows are durable locally, zero PDS writes.
4. Signed in: follow = one public `app.arecipe` cookFollow record per cook;
   unfollow deletes it; Account lists `added` members with the same add UI on
   top.
5. Local follows are offered — not silently pushed — for publish on sign-in.
6. Browse ≤2 control rows before content on mobile, Cookbook ≤3; one Filters
   disclosure with an accurate count badge; honest counts preserved.
7. Browse bundle still ships zero auth code; full gate green every phase.
