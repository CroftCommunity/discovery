# BUILD-SPEC: Phases 1 and 2

A build specification for an AI coding agent, continuing from Phase 0. It is
paired with `design-philosophy.md` (the why) and follows `BUILD-SPEC.md` (Phase
0, the functional core and CLI shell, now landed). This document tells you what
and in what order for Phases 1 and 2; the philosophy doc tells you why. When they
appear to conflict, stop and surface the conflict rather than guessing.

These two phases are teed up together so the execution produces real, usable
assets (a working web UI and a desktop app) and teaches us where the design and
the architecture meet reality. Phase 2's details may be re-anchored to what Phase
1 actually teaches, exactly as Phase 1 was re-anchored to Phase 0's real outcome.

---

## 0. State carried forward from Phase 0

Phase 0 landed essentially to spec. What is true now, and what carries forward:

- The core is pure `(state, intent) -> (state, effects)`: no I/O, no async, no
clock, WASM-clean. 20 acceptance tests pass (behavior A1-D2, projection P1-P7).
This is the proven spine these phases build on.

- DECISION 1 held: the async port lives in the `bluesky` crate and is consumed by
the shell, never the core. The core depends on `bluesky` for native data types
only.

- View models render to text through the CLI for every state. This proves the
view models are presentation-agnostic, which Phase 1 depends on completely:
Phase 1 renders the same view models to the DOM.

- Fixtures are real recorded `getTimeline` responses, not fabricated.

- The Phase 0 real adapter reads the public AppView's `getAuthorFeed`
(unauthenticated, same shape as `getTimeline`) behind the port. Authentication is
deliberately deferred (see section 0b).

### 0a. Correction carried forward: the curl workaround

Phase 0's real adapter shells out to system `curl` for its HTTPS GET. This was a
reaction to a license-review flag on a transitive TLS crate
(`webpki-roots`, CDLA-Permissive-2.0). That license is permissive and fine for
this use case. The flag was a review prompt, not a prohibition, and the right
response was to confirm fitness for use, not to reshape the networking approach
around it.

Carried-forward guidance:

- Do not let a license-review flag silently reshape architecture. If a license
needs handling, handle it openly and deliberately. Do not quietly work around a
flag.

- Use a normal HTTP/TLS approach where it is the right tool. On web (Phase 1) this
is the browser's `fetch`, so there is no Rust TLS crate involved at all and
nothing to flag. On desktop (Phase 2) use a normal Rust HTTP client or Tauri's
HTTP, whichever reads cleanest; do not contort around the license.

- Low-priority cleanup (not a milestone, do it when convenient): replace the
Phase 0 CLI adapter's `curl` shell-out with a normal HTTP client, so the codebase
does not carry a workaround whose original reason did not hold. Until then, the
CLI keeps working as-is; this is tidying, not a blocker.

### 0b. Authentication is deferred and worked around

Authentication is out of scope for Phases 1 and 2. It is a cross-cutting concern
(per-pond connections, token custody, OAuth flows) that will be designed properly
in a later phase, not half-built to unblock UI work.

The workaround is free and already in place: read public, unauthenticated feeds
via the public AppView's `getAuthorFeed`, which returns the same shape as the
authed `getTimeline`. The entire web UI and desktop app can be built and
exercised against real public Bluesky data with no login flow. Because the port
hides the choice, swapping public-read for authed-read later changes only the
adapter, not the UI or the shell. Nothing built in these phases is redone when
auth arrives.

### 0c. One core hardening item (from Phase 0 review)

The cursor invariant (BUILD-SPEC §3a) is proven informally, by enumeration, and
depends on discipline. While working in the core during Phase 1, add a debug
assertion in `update` that checks the invariant on every transition (a
cursor-mandatory state is never constructed without a cursor; the C2 cursor
discard is intentional and the new cursor comes from the intent). This converts a
discipline-dependent invariant into a mechanically-caught one, so a future state
addition that violates it fails a test loudly. Small task, fold it into M1.1.

---

# PHASE 1: the Leptos web UI (the craft phase)

Phase 1 compiles the proven core to WASM for the first time, builds the design
system, and renders the same view models the CLI rendered to text now to the DOM,
in a browser, against real public Bluesky data. This is the phase where craft
becomes active and load-bearing. The architecture is mostly proven; the risk here
is execution quality, not structure.

## 1.1 Binding constraints for Phase 1

All Phase 0 constraints still hold (pure core, the two guardrails, honest seams,
the decisions log). Additionally:

- **The core stays pure and unchanged in behavior.** Phase 1 adds a renderer and
an effect handler around the core; it does not change `update`'s logic. The only
core edit is the debug assertion in 0c.

- **The UI renders view models only.** The Leptos components paint `FeedView` and
its `PostCard`s exactly as the CLI did, just to the DOM. No protocol types in the
UI. No logic in components that belongs in the projection (if a component needs to
decide something, that decision belongs in `project.rs`, tested).

- **Nothing placed by eye.** Every space, size, color, radius, and duration comes
from a design token. This is mechanically enforced (see M2 and the snapshot
gate), not left to vigilance.

- **The design system knows no protocol.** The `design` crate produces Leptos
primitives that consume only tokens. It does not import `bluesky` or any protocol
type. It renders whatever view-model data it is handed.

- **Browser fetch for effects, no Rust TLS.** The web effect handler performs
`FetchFeed` via the browser's `fetch` API. The browser does TLS. No Rust TLS
crate, nothing to license-flag (see 0a).

- **No auth.** Read public feeds only (see 0b).

- **WASM-clean for real.** The core compiled WASM-clean in principle in Phase 0;
Phase 1 compiles it to WASM in fact. If any existing dependency turns out to
preclude WASM, surface it rather than working around it silently.

## 1.2 What gets built (crates and structure)

Additions to the existing workspace. Do not disturb the Phase 0 crates beyond the
0c assertion.

```
experiments/crates/
  core/                       # unchanged except the 0c debug assertion
  bluesky/                    # unchanged
  cli/                        # unchanged (curl cleanup is optional, 0a)

  design/                     # NEW: the design system
    Cargo.toml
    src/
      lib.rs
      tokens.rs               # the single source of spacing, type, color,
                              #   radius, motion. Named constants only.
      primitives.rs           # Leptos components consuming only tokens:
                              #   Card, Avatar, Text, Button, and the view-state
                              #   chrome (loading, empty, error, footers)

  web/                        # NEW: the web shell (Leptos app + browser effects)
    Cargo.toml
    index.html
    src/
      main.rs                 # mount Leptos, wire the runtime loop
      runtime.rs              # drives core: intent in, effects out, results in
                              #   (the DOM analogue of the CLI's runtime loop)
      effects.rs              # performs FetchFeed via browser fetch
      app.rs                  # the single-column feed UI, built from `design`
    style/
      reset.css               # webview-tell elimination lives here
```

Dependency arrows: `web` depends on `core`, `bluesky` (data types, and the port
plus a browser-fetch adapter), and `design`. `design` depends on Leptos and
nothing else (no core, no bluesky). The same discipline as Phase 0: the platform
crate (`web`) may know everyone; nobody knows it; `design` knows no protocol.

Note on the runtime loop: it is the same shape as the CLI's. Hold the core
`Model` in a Leptos signal, apply intents via `update`, update the signal with
the new model (UI re-renders), perform each returned effect, turn results into
follow-up intents, loop. The CLI already proved this loop; Phase 1 changes only
where intents come from (DOM events) and where view models go (the DOM).

## 1.3 The design system (the heart of this phase)

Before writing any component, read the frontend-design skill at
`/mnt/skills/public/frontend-design/SKILL.md` and follow its process (brainstorm a
token system, critique it against templated defaults, then build). The design must
embody the project's values: warm, consistent, low mental load, not extractive,
crafted enough to "feel like itself" (the philosophy's stated bar: the good apps
earn the right to feel like themselves).

`tokens.rs` defines named constants, the single source of truth, in these
categories: spacing (a strict scale, e.g. multiples of 4), a type scale (named
sizes with line-heights bound to them), color (semantic names like surface,
text-primary, accent, never raw hex at call sites), radius, and motion (named
durations and easing curves). The font-size and warmth concerns the project cares
about live here as first-class, deliberate choices.

`primitives.rs` defines the Leptos components that consume only tokens: the post
card, avatar (with an explicit no-avatar rendering, never a broken image), text,
button, and the chrome for every view state. Components read tokens; they never
write raw values.

Motion is a shared token, executed by the shell. Define motion parameters
(durations, easing) as tokens now even if Phase 1's motion is minimal, so the
identity is consistent from the start. Respect reduced-motion preferences.

## 1.4 Webview-tell elimination (do not skip)

The philosophy names webview tells as a top source of the amateur smell. `reset.css`
and component styles must, at minimum: disable text selection where it should not
be (UI chrome, buttons), remove mobile tap-highlight, set correct cursors, replace
webpage-style focus rings with deliberate app-style focus states (while keeping
visible keyboard focus for accessibility), and tame browser scroll behavior so it
feels like an app, not a web page. This is required for Phase 1 to be considered
done, not a later polish pass.

## 1.5 The per-state snapshot harness (the definition-of-done gate)

Stand up a visual snapshot harness as part of Phase 1, not after. The required
snapshot set is the same enumeration the CLI already renders, now as DOM/visual
snapshots:

- Feed: loading from cold; loaded with posts and more-available footer; loaded
with posts and end-reached footer; loading-more (posts plus footer spinner);
empty; error from cold (full error view with retry); error-while-appended (posts
plus inline footer error with retry).

- Card: standard post; missing avatar (designed placeholder); very long text;
long author name and handle (clean truncation, timestamp not pushed off).

The rule: every state has a required snapshot, so an undesigned state is a failing
test. You cannot mark Phase 1 done while any snapshot is missing. This forces the
unhappy states to be designed, which is where craft is won or lost.

Additionally, a token-contract check: a test (or lint) that fails if a primitive
resolves to a raw value (a hardcoded pixel, hex, or duration) instead of a token.
This mechanically enforces "nothing placed by eye."

## 1.6 Phase 1 milestones

**M1.1 - WASM core + the 0c assertion.** Compile the core to `wasm32` for the
first time. Add the cursor-invariant debug assertion to `update`. DoD: the core
compiles to WASM; all 20 Phase 0 tests still pass; the new assertion is present
and a deliberately-wrong transition would trip it (add one test that exercises it
under debug).

**M1.2 - Design tokens.** Following the frontend-design skill, define `tokens.rs`:
spacing, type scale, color, radius, motion, as named constants. DoD: a complete,
self-consistent token set exists; a short written rationale (in a comment or the
README) ties the choices to the project's values rather than to templated
defaults.

**M1.3 - Primitives.** Build the `design` primitives consuming only tokens,
including every view-state chrome. DoD: primitives render in isolation; the
token-contract check passes (no raw values); `design` imports no protocol type.

**M1.4 - The feed UI + runtime loop.** Build `web/app.rs` (single-column feed from
primitives) and `web/runtime.rs` (the DOM runtime loop). DoD: the app renders a
`FeedView` from the core; intents flow from DOM events; the loop matches the CLI's
shape.

**M1.5 - Browser-fetch effect handler.** Implement `web/effects.rs` performing
`FetchFeed` via browser fetch against the public AppView (no auth, no Rust TLS).
DoD: the running web app loads a real public feed; scrolling to the end loads
more; an induced failure renders the error states.

**M1.6 - Webview-tell pass + snapshot harness.** Apply `reset.css` and the
webview-tell fixes; stand up the per-state snapshot harness with the full required
set; wire the token-contract check. DoD: all required snapshots exist and pass;
the token-contract check passes; the webview-tell checklist (1.4) is satisfied;
reduced motion and visible keyboard focus both work.

Phase 1 is complete when M1.1-M1.6 are done. At that point the same proven core
runs in a browser, rendering real public Bluesky data through a crafted design
system, with every state designed and snapshotted, free of the obvious webview
tells. This is the first real, shareable asset.

## 1.7 Out of scope for Phase 1

Tauri/desktop (Phase 2), the composable shell and slots (Phase 2), pinning (Phase
2), any second pond, authentication (deferred, 0b), notifications, layout
persistence. Phase 1 is one column, one pond, public read, crafted.

---

# PHASE 2: desktop wrap + the composable shell

Phase 2 proves "desktop is the wrapped web app" and builds the frame the whole
garden hangs in: the slot system, the serializable layout document, and pinning.
The web UI from Phase 1 becomes a desktop app with no second UI codebase, and then
gains the composability that is the project's organizing thesis.

Re-anchor note: review what Phase 1 actually taught before treating 2.x details as
fixed, the same way Phase 1 was re-anchored to Phase 0.

## 2.1 Binding constraints for Phase 2

All prior constraints hold. Additionally:

- **Desktop is the Phase 1 web bundle wrapped, not a new UI.** Tauri hosts the
same Leptos app. If you find yourself writing desktop-specific UI, stop: only the
effect handlers and native integration differ, not the interface.

- **Desktop effects use a normal HTTP approach.** A normal Rust HTTP client or
Tauri's HTTP, platform TLS, no license contortion (0a). The core and the loop are
unchanged; only who performs `FetchFeed` differs from the web shell.

- **The shell owns layout and pinning; modules do not.** The slot system, the
layout document, and the pin list are shell-level concerns (philosophy §1a, §4a).
A module only declares whether its items are pinnable and how to hydrate one from
an ID.

- **Pins are references, not copies.** A pin holds an address (native ID plus a
type hint) and hydrates through its pond's module. A pin whose target is gone
degrades gracefully. (Phase 2 has one pond, so pinning is proven against Bluesky;
the contract must not assume only one pond.)

- **No browser storage in artifacts** does not apply here (this is a real app),
but the layout document must be a plain serializable value the shell persists, not
scattered ad-hoc state.

## 2.2 What gets built

```
experiments/crates/
  ... (Phase 0 and 1 crates) ...

  desktop/                    # NEW: the Tauri shell for macOS
    (Tauri project structure)
    src/
      effects.rs              # performs FetchFeed via a normal HTTP client
      main.rs                 # hosts the Phase 1 web bundle in a Tauri window

  shell/                      # NEW: the composable frame (may live in `web`
                              #   initially if that is cleaner; decide and note)
    src/
      lib.rs
      slots.rs                # named regions the UI exposes; a contribution
                              #   registry (a module declares it can fill a slot)
      layout.rs               # the serializable layout document: which panels,
                              #   where, sizes, visibility (the Obsidian-workspace
                              #   model)
      pinning.rs              # the pin list (addresses + type hints) and the
                              #   pinned-strip; is-pinnable + hydrate-by-id hooks
```

Note: the slot/layout/pin system is shell-level and platform-agnostic; it should
work in the web shell too, not only desktop. Build it so it is shared, with the
desktop crate simply being another host of the same shell. Where exactly the
`shell` code lives (its own crate vs inside `web`) is your call; pick the cleaner
arrangement and document it.

## 2.3 The composable shell concepts

- **Slots.** The UI exposes named regions (e.g. a column area, a sidebar, the
top-level pinned strip). A module contributes a panel into a region by
declaration, not by the shell knowing the module. This is the VS-Code-style
contribution model, kept minimal.

- **Layout document.** A single serializable value describing which panels exist,
their arrangement, sizes, and visibility. The user edits it by arranging the UI;
the shell persists and restores it. This is the Obsidian-workspaces model. Pins
live in the same persisted document.

- **Pinning.** The top-level pinned strip is part of the frame, above the panels.
A pin references a native item by address and hydrates through its module. With
one pond in Phase 2, prove it against Bluesky (pin a post or a thread to the top),
but keep the hooks (is-pinnable, hydrate-by-id) general so a second pond later
gets pinning for free.

## 2.4 Phase 2 milestones

**M2.1 - Desktop wrap.** Stand up the Tauri project on macOS hosting the Phase 1
web bundle, with `desktop/effects.rs` performing `FetchFeed` via a normal HTTP
client (platform TLS, no license contortion). DoD: the desktop app launches and is
visibly the same UI as the web app, loading a real public feed; "desktop is the
wrapped web app" is demonstrably true (same bundle, native window, native effect
handler).

**M2.2 - Slots + contribution registry.** Build the slot system: named regions and
a registry a module contributes panels into. Refit the existing single feed as a
panel contributed into a column slot. DoD: the feed renders as a contributed panel
in a slot; the shell does not hard-code the feed; adding a second panel type would
require no shell change.

**M2.3 - Layout document.** Implement the serializable layout document and
persistence: which panels, arrangement, sizes, visibility. The user can arrange
panels and the arrangement survives restart. DoD: a user-arranged layout persists
and restores; the layout is one serializable value, not scattered state.

**M2.4 - Pinning.** Build the pin list and the top-level pinned strip; add the
is-pinnable and hydrate-by-id hooks to the Bluesky module; pin a Bluesky item to
the top and have it hydrate live; handle the gone-target degraded state. DoD: a
user can pin a Bluesky post/thread to the top-level strip; it hydrates through the
module; pins persist in the layout document; a pin to a missing item degrades
gracefully.

**M2.5 - Snapshot coverage for the new surfaces.** Extend the per-state snapshot
harness to the new shell surfaces: a panel in a slot, the pinned strip (empty,
with pins, with a degraded pin), arranged layouts. DoD: required snapshots exist
for the new surfaces; the token-contract check still passes across all new
components.

Phase 2 is complete when M2.1-M2.5 are done. At that point the same crafted UI is
a real desktop app, the composable frame exists (slots, persisted layout,
top-level pinning), and the architecture's "garden" thesis is demonstrably real
against one pond, ready for a second pond to slot in (Phase 3) with no structural
change.

## 2.5 Out of scope for Phase 2

Android and other platforms (Phase 3), a second pond (Phase 3), authentication
(deferred), notifications and aggregation (later), the iroh/Croft chat pond
(later), games (later). Phase 2 is desktop + the composable frame, one pond,
public read.

---

## Definition of done (both phases, checklist)

Phase 1:

- [ ] Core compiles to WASM; all 20 Phase 0 tests still pass; the 0c invariant
assertion is present and tested.

- [ ] `design` crate exists with a complete token set and primitives that consume
only tokens; imports no protocol type.

- [ ] The web app renders real public Bluesky feeds; load-more works; error states
render.

- [ ] Effects use browser fetch; no Rust TLS crate; no license contortion.

- [ ] Every required view state has a passing snapshot; the token-contract check
passes; webview tells addressed; reduced motion and keyboard focus work.

- [ ] No auth (public read only); core behavior unchanged.

Phase 2:

- [ ] Desktop app launches on macOS as the wrapped Phase 1 bundle; same UI; loads
a real public feed via a normal HTTP approach (no license contortion).

- [ ] Slot system and contribution registry exist; the feed is a contributed
panel; adding a panel type needs no shell change.

- [ ] The serializable layout document persists and restores user arrangement;
pins live in it.

- [ ] Top-level pinning works against Bluesky via is-pinnable + hydrate-by-id;
gone-target degrades gracefully; the hooks are general (not one-pond-specific).

- [ ] Snapshots cover the new shell surfaces; token-contract check still passes.

- [ ] Each milestone landed as its own commit/PR with its DoD met.

---

## Carried-forward notes (low priority, not milestones)

- Replace the Phase 0 CLI adapter's `curl` shell-out with a normal HTTP client
when convenient (0a). Tidying, not a blocker.

- Authentication and the per-pond-connection model are owned by a later phase
(0b). Phase 3 (a second pond) is the natural place that work comes due.

---

*Phase 3 (Android + the second pond, Mastodon via megalodon, where authentication
and the per-pond-connection model come due) gets its own build spec once Phases 1
and 2 are landed and the web UI, desktop wrap, and composable frame are proven.*
