# RUN-TIMERS-SEASONALITY — a timer page, and seasonality that only ever helps

Self-contained instruction file for Claude Code, repo `CroftCommunity/arecipe`.
Two independent features in one run because both are small, neither needs a
model, and neither touches the other's files. They may be split into two
branches if that is more convenient; the phases are already separated.

`[verify-in-run]` items are probed in Phase 0 and recorded in the run summary
BEFORE code depends on them. Contradictions with stated context are FINDINGS:
record them, do not silently adapt.

**TDD is mandatory.** Every acceptance criterion is encoded as a failing test
before the implementation that satisfies it exists. The run summary records red
output then green output per phase. A phase that went green with no recorded red
is a failed phase and gets redone.

---

# FEATURE A — Timers

## A0. Mission

A standalone timer page holding several concurrent named timers, reachable from
navigation and from the reference page, and surfaced compactly inside focus
mode. Timers survive leaving the recipe, because that is the actual kitchen
need: you start the rice, go look at the meal plan, come back.

## A1. The decision the whole feature rests on

**Store the absolute end timestamp, never the remaining seconds.**

A countdown that decrements a stored `remaining` value drifts under timer
throttling and dies outright when the tab is backgrounded or the device sleeps.
An absolute `endsAt` is recomputed against `Date.now()` on every render and on
every wake, so a timer that "ran" while the phone was asleep is simply already
expired when you look at it. That is the correct behavior and it falls out of
the data model rather than being defended by code.

Any implementation that persists a remaining-duration counter is a failed
feature.

## A2. Locked design decisions

**A-D1 — Device-scoped, never synced.** Timers are ephemeral device state. They
are persisted locally (IndexedDB, following the `drafts-local.ts` shape
`[verify-in-run]`) so they survive navigation and reload. **They are never
written to the PDS.** A timer is not a record.

**A-D2 — Data shape.**

```ts
interface Timer {
  id: string;
  label: string;        // user text, may be empty
  endsAt: number;       // epoch ms
  durationMs: number;   // for restart, never for countdown math
  createdAt: number;
}
```

`durationMs` exists only so a finished timer can be restarted. Nothing computes
a countdown from it.

**A-D3 — Pure core.** `src/timers/timer-state.ts` holds pure functions:
`remainingMs(timer, now)`, `isExpired(timer, now)`, `addTimer`, `removeTimer`,
`restartTimer(timer, now)`. `now` is always injected. No ambient clock, no
`setInterval` inside the pure module. The page owns the tick; the module owns
the arithmetic.

**A-D4 — Firing, honestly.** When a timer expires while the page is visible:
visual state change plus an audible cue. If the user has explicitly enabled
notifications for timers, also fire a `Notification`.

Permission hygiene: **never request notification permission on page load.** Only
request it in direct response to the user turning on "notify me", and treat
denial as a permanent, silent no.

Audio hygiene: browsers block audio that starts without a user gesture, so prime
the audio path on the first "start timer" tap and reuse it.

**A-D5 — State the limit rather than faking it.** With no backend there is no
reliable background alarm. A service worker plus `setTimeout` is not a
dependable scheduler and must not be presented as one. When the page is
backgrounded, the timer is still *correct* on return because of A1, but the
alert may be late. The settings copy says this plainly in one sentence. Do not
build an unreliable background scheduler and do not imply one exists.

**A-D6 — Focus-mode surface.** A compact strip in `.focus-top`
`[data-testid="timer-strip"]` listing running timers as label plus remaining.
Tapping opens the timer page. It renders nothing when no timer is running. It
must not take vertical space from the current step when idle, and it must never
steal focus or scroll position.

**A-D7 — Entry points.** Navigation, and a link from the reference page.
`[verify-in-run]` whether "reference" means the help/user-guide page, the
recipe-page reference section, or both; if ambiguous, wire both, since each is
a single anchor.

**A-D8 — Duration chips (OPTIONAL, last phase, droppable).** Detect explicit
durations in instruction text ("simmer for 20 minutes") and render a tap-to-
start chip on that step. Conservative parse only: an explicit number plus an
explicit unit. No inference, no ranges resolved to a guess, no "until golden".
If the parser is not comfortably above 95% precision on the fixture set, **drop
this phase entirely** rather than shipping chips that start wrong timers.

## A3. Phases

**Phase A0 — Re-ground.** Record: the local-store helper signature, the nav
structure and how a new page is registered in `scripts/build.mjs` (allowlist
copy, not a glob), the focus view builder location from RUN-COOK-FOCUS, and the
reference page's identity per A-D7.

**Phase A1 (RED).** Unit tests on `timer-state.ts`:

1. `remainingMs` with `now` before `endsAt` returns the difference.
2. `remainingMs` with `now` past `endsAt` returns 0, never negative.
3. `isExpired` is false at exactly `endsAt - 1`, true at exactly `endsAt`.
4. A timer created at T and read at T + 8h (simulating device sleep) is expired,
   with no accumulated drift.
5. `restartTimer` sets `endsAt` to `now + durationMs` and leaves `durationMs`
   unchanged.
6. `addTimer` and `removeTimer` do not mutate their input array.
7. Multiple concurrent timers each compute independently.

Persistence tests:

8. Round-trip write then read preserves `endsAt` exactly.
9. A store containing an expired timer reads back as expired, not as a fresh
   timer.

E2E:

10. Starting a timer on the timer page and navigating away and back shows it
    still running with the correct remaining time.
11. `[data-testid="timer-strip"]` appears in focus mode when a timer runs and is
    absent when none run.
12. Notification permission is NOT requested on load; it is requested only after
    the notify toggle is turned on.
13. The reference page and nav both link to the timer page.

**Phase A2 (GREEN).** Implement in order: `timer-state.ts`, persistence, the
timers page, the focus strip, the entry points.

**Phase A3 (OPTIONAL).** Duration chips, with the 95% precision gate. If
dropped, record that it was dropped and why.

## A4. Acceptance criteria

1. No stored countdown exists; all remaining time derives from `endsAt` (1 to 4,
   plus a grep in the summary for any persisted remaining-duration field).
2. Timers survive navigation, reload, and device sleep (8, 9, 10).
3. Several timers run at once, independently (7).
4. The focus strip appears only when relevant (11).
5. Notification permission is only ever requested on explicit opt-in (12).
6. The settings copy states the background-alert limitation in one sentence.

## A5. Out of scope for Feature A

Syncing timers anywhere. Background scheduling. Timer presets library. Voice
control. Holding the wake lock from the timer page (focus mode already owns it).

---

# FEATURE B — Seasonality

## B0. Mission

Surface what is good right now. Nothing else.

## B1. The rule that defines the feature

**Seasonality is only ever a boost. It is never a drag.**

No out-of-season warnings. No "this is a poor month for asparagus". No planner
nudge saying a chosen recipe is wrong. The feature adds signal to things that
are in season and is otherwise completely silent. A user who never opens
settings should experience it as an occasional pleasant highlight, never as the
app second-guessing a decision they already made.

It is turn-off-able in settings, and off means byte-identical behavior to today.

## B2. Locked design decisions

**B-D1 — Static data, versioned in repo.** `src/seasonality/produce.ts` (or a
JSON asset `[verify-in-run]` for consistency with existing static data). Shape
is produce-keyed with per-region month sets, because one item has different
seasons in different places:

```ts
interface Produce {
  id: string;              // stable slug
  display: string;
  aliases: string[];       // explicit, curated
  seasons: Record<RegionId, number[]>;  // months 1-12
}
```

**B-D2 — Region is explicit, never inferred.** Do **not** geolocate. Do not
guess from locale. The region is a settings value with a documented default that
is visibly labelled so it is obviously changeable. `[verify-in-run]` the default
region is an owner decision; record which was chosen and why in the summary. If
the owner has not decided by Phase 0, use a single clearly-labelled default and
flag it rather than blocking.

**B-D3 — Matching is explicit, never fuzzy.** A recipe ingredient matches a
produce item only through the curated `aliases` list, reusing the existing
conservative ingredient normalization. No stemming beyond what already ships, no
substring matching, no embeddings. A wrong "in season" badge is cheap but it is
still noise, and noise is what kills a boost-only feature.

**B-D4 — Boost, never filter.** `seasonBoost(recipe, month, region)` returns a
non-negative score contribution. It is added to the existing ranking score. It
**cannot remove a result and cannot reorder a result below where it would
otherwise sit.** An explicit user-chosen "in season" filter chip is allowed,
because that is the user asking; automatic filtering is not.

**B-D5 — Surfaces.** An "in season" badge on recipe cards where at least one
core ingredient matches. An optional "in season now" strip on Browse. Nothing on
the meal planner: the planner is where a drag would hurt most and there is no
boost-shaped thing to add there.

**B-D6 — Settings.** One toggle, default on, persisted locally. Off means the
badge, the strip, and the boost all disappear and ranking is identical to the
pre-feature baseline.

## B3. Phases

**Phase B0 — Re-ground.** Record: the existing ranking/scoring path on Browse
and Cookbook, the existing settings surface and its persistence, the ingredient
normalization helper, and how static data assets are currently shipped.

**Phase B1 (RED).** Unit tests:

1. `isInSeason(produceId, month, region)` is true inside the season set, false
   outside, for two different regions with different sets.
2. Unknown produce id returns false rather than throwing.
3. `seasonBoost` returns 0 for a recipe with no matching ingredient.
4. `seasonBoost` returns a positive value for a matching one.
5. Alias matching hits a curated alias and **misses** a near-miss string that is
   not in the alias list (this test exists to pin B-D3; a fuzzy implementation
   fails it).
6. Applying boosts to a result list never removes an entry and never lowers an
   entry's position relative to the unboosted list.
7. With the setting off, the ranking output is deep-equal to the unboosted
   ranking.

E2E:

8. The badge appears on a card whose ingredient is in season for the configured
   region and month.
9. Toggling the setting off removes the badge and the strip.
10. No copy anywhere in the app states or implies that anything is out of
    season. Assert by grepping the rendered DOM of Browse, Cookbook, recipe and
    meals pages for the phrase set recorded in Phase 0.

**Phase B2 (GREEN).** Implement: data, matcher, boost, badge, strip, setting.

## B4. Acceptance criteria

1. Boost only; ranking positions never worsen (6).
2. Off is indistinguishable from the current build (7, 9).
3. Matching is curated-alias only (5).
4. Region is explicit, defaulted and labelled; no geolocation call exists
   (grep in the summary).
5. No negative seasonality copy exists anywhere (10).

## B5. Out of scope for Feature B

Out-of-season anything. Planner nudges or warnings. Geolocation. Automatic
region detection. Fuzzy or embedding-based produce matching. Per-recipe
seasonality authoring by users. Local-farm or CSA data.

---

# Gate

`npm test` fully green. `RUN-TIMERS-SEASONALITY-SUMMARY.md` records Phase 0
findings for both features, red then green output per phase, files touched, the
chosen default region and rationale, whether the optional duration-chip phase
shipped or was dropped, and the two greps required by A4.1 and B4.4.
