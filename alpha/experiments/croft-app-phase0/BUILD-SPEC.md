# BUILD-SPEC: Phase 0

A build specification for an AI coding agent. It describes exactly what to build,
in what order, and the acceptance criteria for each step. It is paired with
`design-philosophy.md`, which holds the reasoning. This document tells you *what*
and *in what order*; the philosophy doc tells you *why*. When they appear to
conflict, stop and surface the conflict rather than guessing.

Phase 0 scope: a pure Rust core and a command-line shell that drives it, with a
test harness, proving the whole architecture against a real Bluesky feed. No GUI,
no design system, no second ecosystem, no pinning, no layout. Those are later
phases.

---

## 0. How to work

- **Version control.** Commit in small, logical units. Each milestone below ends
in a commit (or PR) with the stated definition of done met. Do not batch unrelated
changes. Commit messages state what changed and why.

- **Test-first.** For core behavior, write the test before or alongside the
implementation. A milestone is not done until its acceptance tests pass. The test
harness is part of Phase 0, not a follow-up.

- **Prescriptive vs permissive.** The architecture, the types, the boundaries,
and the tests in this document are decisions; implement them as specified. How you
write the body of a given function is yours, as long as it honors the constraints
in section 1. If a specified type or boundary seems wrong, surface it rather than
silently changing it.

- **Surface leaks.** If you find yourself wanting to put I/O in the core, or a
platform assumption in the core, or a protocol type where it does not belong,
stop. That is a sign the design is being violated. Raise it.

- **Do not fabricate fixtures.** The committed Bluesky fixtures (section 6) are
real recorded responses supplied with the repo. If they are absent, stop and ask;
do not hand-author timeline JSON from memory of the protocol shape. A fabricated
fixture defeats the entire point of testing against real shapes.

---

## 1. Binding constraints (honor these everywhere)

These are condensed from `design-philosophy.md`. Read that document in full before
starting. The non-negotiables:

- **Functional core, imperative shell.** The core performs no I/O. It is a
function of `(state, intent) -> (state, effects)`. It never touches network,
clock, storage, or screen. It describes effects as data; the shell performs them
and feeds results back as intents.

- **The core knows no platform.** No `std::net`, no HTTP client, no filesystem, no
clock, no async runtime *in the core crate*. If the core needs the current time or
a network result, it receives it as data in an intent. (Time-dependent values
enter as effects/intents, never read directly.)

- **The core does not call the port.** The core emits effect *data* and receives
results as intents. It never holds, calls, or awaits the port trait. This is what
keeps `update` a pure synchronous function. (See DECISION 1.)

- **Three kinds of boundary traffic only:** intents in, view models out, effect
requests out (with results returning as intents). Nothing else crosses.

- **View models are presentation-agnostic data.** They must render equally to a
terminal or a DOM. No view model may contain anything that assumes a specific
renderer. (The CLI is the proof of this: if the CLI cannot render a view model as
text, the view model is wrong.)

- **The design system knows no protocol.** (Not exercised in Phase 0, but do not
violate it in anticipation.)

- **No raw values in presentation, ever** (applies when UI arrives in Phase 1;
noted here so it is not forgotten). Phase 0 has no design tokens yet.

- **Make illegal states unrepresentable**, but do not manufacture impossible
combinations. Phase 0 uses a single load-lifecycle enum (section 3). The
cursor-bearing states carry their cursor in the variant itself, so a state that
logically must have a cursor cannot be built without one (see DECISION 4).

- **Ports at proven plurality.** The Bluesky access is behind a port because we
will have a second ecosystem later. Do not add other ports speculatively.

- **Not extractive.** No telemetry, no engagement ranking. The core presents
content in the order the source provides or by explicit user sort, never an
engagement-optimized reordering.

---

## 1a. Locked decisions (resolve the spec's open choices)

These five decisions close gaps that earlier drafts left open. They are binding.
Where a later section still reads as optional, these win.

**DECISION 1. The shell holds the port; the core never depends on the trait.**

The port trait lives in the `bluesky` crate and is consumed by the `cli` shell,
not by `core`. The core depends on `bluesky` only for the native post *data type*
it carries in intents and stores in the model. It does not depend on the port
trait or the adapter.

Reasoning: the core's contract is `fn update(model, intent) -> (Model,
Vec<Effect>)`, a pure synchronous function. An `async` port trait that the core
called and awaited could not return from that signature. Effect-as-data and
effect-as-awaited-call are different architectures, and the whole test story holds
only under effect-as-data. The philosophy doc already chose this in sections 3 and
5; this decision makes it explicit and removes the "choose one" latitude in the
old section 2.

**DECISION 2. The core stores the native Bluesky post type directly.**

`Model` holds `Vec<bluesky::types::Post>` (the native shape), not a core-defined
post type and not opaque bytes. This is consistent with honest seams: the core
carries the native shape end to end, and the projection (`project.rs`) is the only
place that converts native into display-ready view fields. The dependency edge
`core -> bluesky` is therefore a data-type edge, never a trait edge.

**DECISION 3. Timestamps in Phase 0 are absolute, formatted from post data.**

The projection formats an absolute timestamp string from a value the post already
carries (e.g. its `createdAt`). Relative time ("2h ago") is deferred to a later
phase, because it would require the current time to enter the projection as a
parameter supplied by the shell, and Phase 0 has no need for it. The core does not
read the clock. Do not import `chrono::Utc::now()` or any wall-clock source into
the core crate.

**DECISION 4. Cursor-bearing states carry the cursor in the variant.**

`FeedStatus` variants that require a cursor to function correctly carry it in the
variant rather than relying on a separate `Option` field on the model that might
be `None`. This makes "an `ErrorWhileAppended` with no cursor to retry from"
unrepresentable rather than merely avoided by convention. See section 3 for the
exact shape. The invariant this guarantees is traced and proven in section 3a.

**DECISION 5. `reason` is a string in Phase 0; the CLI error path drives the
fake.**

The failure `reason` carried by `FeedLoadFailed`, `ErrorCold`, and
`ErrorWhileAppended` is a plain `String` in Phase 0. A structured error enum is a
later concern and is not justified yet (grow by use case). The CLI's "force an
error path" command works by configuring the fake into its error mode (section 6),
not by inventing a separate failure-injection mechanism in the shell.

---

## 2. Repository layout (Phase 0)

A Cargo workspace. Only the crates needed for Phase 0 are created now.

```text
(repo root)
  Cargo.toml                  # workspace
  README.md                   # points at design-philosophy.md and this spec
  design-philosophy.md        # the reasoning (companion doc)
  BUILD-SPEC.md               # this file

  crates/
    core/                     # pure logic, no I/O; the heart
      Cargo.toml
      src/
        lib.rs                # re-exports; module wiring
        model.rs              # Model, FeedStatus
        intent.rs             # Intent
        effect.rs             # Effect
        view.rs               # view models (FeedView, PostCard, footer markers)
        update.rs             # the update function
        project.rs            # Model -> view model (pure)
      tests/
        feed_behavior.rs      # spec groups A-D as tests
        projection.rs         # projection specs P1-P7 as tests

    bluesky/                  # the first ecosystem module; owns the port trait
      Cargo.toml
      src/
        lib.rs
        port.rs               # the BlueskyPort trait + request/response data
        types.rs              # native Bluesky shapes (NOT normalized)
        fake.rs               # fixture-backed fake implementing the port
        adapter.rs            # real implementation (Jacquard) - last step
      tests/
        adapter.rs            # adapter tested against recorded fixtures
      fixtures/
        timeline_page_1.json  # recorded real responses (committed, supplied)
        timeline_page_2.json
        timeline_empty.json

    cli/                      # the imperative shell for Phase 0
      Cargo.toml
      src/
        main.rs               # arg parsing, the runtime loop
        runtime.rs            # drives core: intent in, effects out, results in
        effects.rs            # performs effects (holds the port, native HTTP)
        render.rs             # renders view models to the terminal
```

The dependency arrows:

- `cli` depends on `core` and `bluesky` (the latter for both the port trait and
the native types). `cli` is the only crate that holds and calls the port.

- `core` depends on the `bluesky` crate for its native *data types only* (the post
shape). It does **not** depend on the port trait or the adapter (DECISION 1, 2).

- `bluesky` depends on nothing in `core`.

- Nobody depends on `cli`.

The port trait lives in `bluesky` and is consumed by `cli`. This is a decision,
not an option (DECISION 1). A core that held an async port could not keep the pure
synchronous `update` signature in section 3.

---

## 3. The core types (implement as specified)

These encode our decisions. Names may be adjusted for Rust idiom; semantics may
not.

### Model and status

`Model` holds: the loaded posts as `Vec<bluesky::types::Post>` (the native type,
per DECISION 2; no protocol *parsing* happens in the core, it only stores and
projects what the shell already parsed) and a `FeedStatus`.

Note the cursor is **not** a free-floating `Option` field on the model. It lives
inside the `FeedStatus` variants that need it, so its presence is provable where
correctness depends on it (DECISION 4).

`FeedStatus` is a six-variant enum (one axis: the load lifecycle). Do not model
this as orthogonal booleans; see philosophy section 9.

- `Idle` - never opened. No posts, no cursor.

- `LoadingCold` - first load in flight, no posts yet, no cursor.

- `Loaded { cursor: Option<Cursor> }` - posts present in the model, nothing in
flight. The cursor `Option` here is the honest place where "more available"
(`Some`) vs "true end" (`None`) is expressed. This is the *only* `Option<Cursor>`
in the status; it is meaningful, not an illegal-state hole.

- `LoadingMore { cursor: Cursor }` - posts present, a next-page load in flight.
Carries the cursor it is loading from (always present; you cannot be loading more
without knowing from where).

- `ErrorCold { reason: String }` - first load failed, no posts. Carries a reason.

- `ErrorWhileAppended { reason: String, cursor: Cursor }` - a load-more failed but
posts are still present. Carries a reason **and** the preserved cursor, so retry
can always resume. The cursor is non-optional here by construction (DECISION 4): a
load-more failure necessarily had a cursor it was loading from, so the recovery
state keeps it.

This shape makes the two retry paths total: `ErrorCold` retries from `None`,
`ErrorWhileAppended` retries from a cursor the type guarantees is present.

### Intent

What happened or what the user wants. Phase 0 set:

- `OpenFeed`

- `FeedReachedEnd`

- `FeedPageLoaded { posts, next_cursor }` - a successful fetch result coming back
in. `posts` is `Vec<bluesky::types::Post>`; `next_cursor` is `Option<Cursor>`.

- `FeedLoadFailed { reason: String }` - a failed fetch result coming back in
(DECISION 5).

- `RetryRequested`

### Effect

What the core asks the shell to do. Phase 0 set:

- `FetchFeed { cursor: Option<Cursor> }`

The core emits this as data. The core never performs it and never calls the port
to fulfill it (DECISION 1).

### update

Signature (shape, not literal): `fn update(model: Model, intent: Intent) ->
(Model, Vec<Effect>)`. Pure. No async, no I/O, no time reads. Deterministic. This
signature is load-bearing for DECISION 1: it is the reason the port cannot live in
the core.

---

## 3a. The cursor invariant (proven, not asserted)

DECISION 4 claims: no transition can construct a cursor-bearing state without a
cursor in hand. This is the proof, by enumerating every entry into the two
cursor-mandatory states. Preserve this property if you ever add states or
transitions.

The two cursor-mandatory states are `LoadingMore { cursor }` and
`ErrorWhileAppended { reason, cursor }`. Every entry into them:

- Into `LoadingMore`, via C1: from `Loaded { cursor: Some(c) }`. The guard
requires `Some(c)`, so `c` is in hand. `Loaded { cursor: None }` is handled by C3
(do nothing), so you cannot enter `LoadingMore` cursorless from the loaded state.

- Into `LoadingMore`, via D2: from `ErrorWhileAppended { cursor: c }`. The source
variant carries `c` structurally.

- Into `ErrorWhileAppended`, via C5: from `LoadingMore { cursor: c }`. The source
variant already carries `c`, carried straight through. No other path reaches
`ErrorWhileAppended`.

The cursor-free states (`Idle`, `LoadingCold`, `ErrorCold`) need no cursor: cold
loads and their retry start from `None`. The one optional cursor (`Loaded`) is the
only place `None` is representable, and there it is a meaningful value (true end),
not a hole.

**The one subtle point:** in C2, the cursor `LoadingMore` was loading *from* is
spent and dropped; the new `Loaded` cursor comes from the intent's `next_cursor`.
This is correct (a consumed cursor is spent), but it is the only place in the
machine where a cursor is discarded rather than carried. If pagination ever
misbehaves, look here first.

---

## 4. Behavior specification (acceptance tests)

Implement these as tests in `core/tests/feed_behavior.rs`. Each is given/when/then
against `update`. These are acceptance criteria: the milestone is not done until
all pass.

### Group A: opening

- **A1.** Given `Idle`, no posts. When `OpenFeed`. Then status `LoadingCold`, and
exactly one effect `FetchFeed { cursor: None }`.

- **A2.** Given `Loaded` with posts. When `OpenFeed`. Then status unchanged, posts
unchanged, no effects. (No wasteful re-fetch.)

- **A3.** Given `LoadingCold`. When `OpenFeed`. Then status unchanged, no effects.
(No duplicate in-flight load.)

### Group B: first page arriving

- **B1.** Given `LoadingCold`, no posts. When `FeedPageLoaded { posts: [some],
next_cursor: Some }`. Then status `Loaded { cursor: Some(..) }`, posts equal that
page in order, no effects.

- **B2.** Given `LoadingCold`, no posts. When `FeedPageLoaded { posts: [],
next_cursor: None }`. Then status `Loaded { cursor: None }`, posts empty, no
effects. (Empty is a valid loaded state, not an error.)

- **B3.** Given `LoadingCold`, no posts. When `FeedLoadFailed { reason }`. Then
status `ErrorCold { reason }`, posts empty, no effects. (No auto-retry.)

### Group C: load-more

- **C1.** Given `Loaded { cursor: Some(c) }`, posts present. When `FeedReachedEnd`.
Then status `LoadingMore { cursor: c }`, exactly one effect `FetchFeed { cursor:
Some(c) }`.

- **C2.** Given `LoadingMore`, posts present. When `FeedPageLoaded { posts: [more],
next_cursor: Some }`. Then status `Loaded { cursor: Some(..) }`, new posts appended
*after* existing in order, no effects. (Append, never replace. The cursor it was
loading from is spent and dropped; the new cursor is the intent's; see 3a.)

- **C3.** Given `Loaded { cursor: None }`, posts present. When `FeedReachedEnd`.
Then status unchanged, no effects. (At the true end; do not fetch.)

- **C4.** Given `LoadingMore`. When `FeedReachedEnd`. Then status unchanged, no
effects. (No stacked requests from rapid scroll. This guard is correctness and
lives in the core.)

- **C5.** Given `LoadingMore { cursor: c }`, posts present. When `FeedLoadFailed {
reason }`. Then status `ErrorWhileAppended { reason, cursor: c }`, existing posts
untouched, the cursor preserved in the variant, no effects. (A failed load-more
must not destroy existing posts, and must keep the cursor for retry.)

### Group D: retry

- **D1.** Given `ErrorCold`, no posts. When `RetryRequested`. Then status
`LoadingCold`, one effect `FetchFeed { cursor: None }`.

- **D2.** Given `ErrorWhileAppended { reason, cursor: c }`, posts present. When
`RetryRequested`. Then status `LoadingMore { cursor: c }`, one effect `FetchFeed {
cursor: Some(c) }`, existing posts untouched. (The cursor comes from the variant,
so this path cannot fail for lack of a cursor.)

---

## 5. Projection specification (acceptance tests)

Implement in `core/tests/projection.rs`. The projection `project(&Model) ->
FeedView` is pure. View models must be renderable as plain text (the CLI proves
this).

- **P1.** `Loaded { cursor: None }`, three posts -> `FeedView` with three
`PostCard`s in order, footer marker `EndReached`.

- **P2.** `Loaded { cursor: Some(..) }`, posts -> footer marker `MoreAvailable`.

- **P3.** A post with author, text, timestamp, and possibly-missing avatar -> a
`PostCard` with display-ready fields: author display name, handle, body text, a
display timestamp string, and an avatar value that is either a URL or an explicit
`NoAvatar` marker (never a null for the renderer to handle). No raw protocol type
appears in the `PostCard`. The timestamp is an **absolute** formatted string
derived from the post's own carried timestamp (DECISION 3); the projection does
not read the clock.

- **P4.** `LoadingCold`, no posts -> a `Loading` view (distinct from empty).

- **P5.** `Loaded { cursor: .. }`, zero posts -> an `Empty` view with its message.

- **P6.** `ErrorCold { reason }` -> a full `Error` view carrying the reason and a
retry affordance marker.

- **P7.** `ErrorWhileAppended { reason, cursor }`, posts present -> the normal feed
of posts plus a footer `InlineError` marker with a retry affordance (never the full
error view).

The clock-source decision for P3 is closed: absolute timestamp from post data,
relative time deferred (DECISION 3). Leave a one-line comment in `project.rs`
pointing at DECISION 3 so the next reader knows it was a choice, not an oversight.

---

## 6. The Bluesky port and the fake (build before the real adapter)

### The port

`port.rs` (in the `bluesky` crate, per DECISION 1) defines the trait the **shell**
depends on. One capability for Phase 0: fetch a page of the timeline given an
optional cursor, returning a page of native posts plus an optional next cursor, or
an error. The trait method is async (it is implemented by the shell side, which
does I/O). The core never sees this trait; the *shell runtime* calls it and turns
results into `FeedPageLoaded` / `FeedLoadFailed` intents.

`types.rs` holds the native Bluesky post shape. Keep it native. Do not normalize it
into a cross-ecosystem type. (Honest seams: there is no shared post model.) This is
the type the core imports for its model (DECISION 2).

### The fixtures (supplied, not fabricated)

The fixtures in `bluesky/fixtures/` are **real recorded** `getTimeline` responses,
committed to the repo and supplied with it. Do not invent their contents
(section 0). If they are missing, stop and ask for them rather than authoring
plausible-looking JSON, because the entire reason they exist is to test parsing
against shapes the protocol actually produces, not shapes we imagine. Record-and-
refresh procedure (if you ever need to regenerate them) is an out-of-band task
owned by a human, not something the agent does mid-build.

### The fake (this is what Phase 0 tests run against)

`fake.rs` implements the port from the committed fixtures. It returns
`timeline_page_1.json` for a `None` cursor, `timeline_page_2.json` for the cursor
that page 1 returns, and can be configured to return an error or
`timeline_empty.json` for testing the unhappy paths. The fake is deterministic and
needs no network. Its error mode is what the CLI's error-path command drives
(DECISION 5).

### Why fake first

Phase 0's correctness must not depend on the network or on which atproto crate we
ultimately choose. The fake lets every behavior and projection test be fast and
deterministic. The real adapter is the *last* step of Phase 0 and is swappable.

---

## 7. The CLI shell

The CLI is a real imperative shell, not a toy. It implements the same three
responsibilities every shell will: feed intents in, render view models out,
perform effects. It is also the only crate that holds and calls the port
(DECISION 1).

- `runtime.rs` holds the core `Model`, applies intents via `update`, and for each
returned `Effect` performs it (via `effects.rs`) and turns the result into a
follow-up intent, looping until no effects remain. This loop is the architecture in
miniature; keep it clean and obvious. The translation from a port result to a
`FeedPageLoaded` or `FeedLoadFailed` intent happens here, not in the core.

- `effects.rs` holds the `BlueskyPort` implementation and performs `FetchFeed` by
calling it. In Phase 0 the CLI can be run in two modes: against the fake (default,
offline, deterministic) and against the real adapter (once built). Performing
effects is the shell's job; this is where the native HTTP client lives, never in
the core.

- `render.rs` renders a `FeedView` to the terminal as text. It renders every view
variant: loading, empty, the feed with cards, the more-available and end-reached
footers, the full error, and the inline error. Rendering all states is required
(it is the text-mode equivalent of the per-state snapshot rule).

CLI commands for Phase 0 (shape, not literal): open the feed and print it; trigger
load-more and print the appended result; force an error path **by putting the fake
into error mode** (DECISION 5) to exercise the error rendering; a flag to choose
fake vs real backend. Keep argument parsing minimal and obvious.

---

## 8. Milestones (each ends in a commit/PR with its DoD)

**M1 - Workspace and core types.**
Create the workspace, the `core` crate, and the types in section 3 (Model,
FeedStatus with cursor-in-variant per DECISION 4, Intent, Effect, empty `update`
stub). Add the `core -> bluesky` data-type dependency (DECISION 2) with a stub
native post type if the bluesky crate is not yet fleshed out. DoD: workspace
builds, types compile, `update` exists as a stub, and `core` has no dependency on
the port trait or adapter.

**M2 - Core behavior, test-first.**
Implement the spec-group A-D tests in `feed_behavior.rs`, then implement `update`
until all pass. DoD: all of A1-D2 pass; `update` is pure (no I/O, no async, no
clock); the cursor invariant in section 3a holds.

**M3 - Projection, test-first.**
Implement the P1-P7 tests, then `project`. DoD: all projection tests pass; view
models contain no protocol types and no renderer assumptions; the timestamp is an
absolute string from post data and `project.rs` carries the DECISION 3 comment.

**M4 - Bluesky port and fake.**
Define the port trait (in `bluesky`, per DECISION 1) and native types; place the
supplied committed fixtures; implement the fake. DoD: the fake serves the fixtures
deterministically, including empty and error modes; no network access in any test;
fixtures are the supplied real recordings, not fabricated.

**M5 - CLI shell against the fake.**
Build the runtime loop, effects-via-port (the shell holding the port), and the
renderer for all view states. Wire it to the fake. DoD: `cli` opens a feed from
fixtures and prints it; load-more appends and prints; the error-path command puts
the fake in error mode and renders the inline and full error views; runs fully
offline.

**M6 - Real adapter (last).**
Implement `adapter.rs` against the real Bluesky access (lean Jacquard; confirm its
moderation/labeling coverage is not needed for Phase 0 before committing). Add a
CLI flag to use it. Add a parsing test exercising the wire parser against the
committed fixtures (not the live network). DoD: the CLI can fetch a real timeline
behind the same port; swapping fake/real changes nothing in `core`; adapter parsing
is tested against fixtures.

*Moderation/labeling checkpoint (blocks committing a Jacquard adapter):* before
adopting Jacquard specifically, confirm Phase 0 needs no moderation/labeling
coverage (it does not — Phase 0 only reads and displays a feed, applies no
labels, and enforces no mutes/blocks). This must be re-checked the moment a phase
adds any moderation-affected surface. *Implementation note:* the committed M6
adapter is a thin `ureq` client over the public AppView feed endpoint rather than
Jacquard. It is reversible behind the port (philosophy §9), satisfies the DoD, and
defers both the Jacquard adoption and its moderation question until a phase
actually needs them.

Phase 0 is complete when M1-M6 are done. At that point the entire spine is proven:
pure tested core, effects requested not performed, a module behind a swappable
port held by the shell, view models that render as text, all driven through one
clean loop, against both fixtures and a live feed.

---

## 9. Explicitly out of scope for Phase 0

Do not build these now; they are later phases. Building them now is scope creep.

- Any GUI, Leptos, Tauri, or design system / tokens.

- WASM compilation target (the core must *remain* WASM-compatible, i.e. no
core-crate dependency that precludes WASM, but we do not build the WASM target in
Phase 0). Note this is one more reason DECISION 1 holds: an async port trait in the
core would put an async surface on the crate we are keeping WASM-clean.

- A second ecosystem (Mastodon, Lemmy).

- A structured error type (DECISION 5 keeps `reason` a string for now).

- Relative timestamps (DECISION 3 defers them).

- Pinning, layout/slots, notifications, auth flows beyond the minimum to read a
feed.

- Motion, snapshots, or visual concerns (no UI yet).

Note the one standing constraint among these: keep the `core` crate
WASM-compatible from the start (no incompatible dependencies), even though we do
not compile to WASM yet. This costs nothing now and avoids a painful retrofit.

---

## 10. Definition of done for Phase 0 (checklist)

- [ ] Workspace builds; crate dependency arrows match section 2 (core depends on
bluesky data types only, never the port trait or adapter).

- [ ] `core` has no I/O, no async, no clock, no platform dependency, does not hold
or call the port, and remains WASM-compatible.

- [ ] `FeedStatus` carries cursors in the variants that need them; an
`ErrorWhileAppended` with no cursor is unrepresentable (DECISION 4); the section 3a
invariant holds.

- [ ] All behavior tests (A1-D2) pass.

- [ ] All projection tests (P1-P7) pass.

- [ ] View models contain no protocol types and no renderer assumptions; timestamp
is absolute from post data (DECISION 3).

- [ ] The Bluesky port exists in the `bluesky` crate and is held by the `cli`
shell; the fake serves committed (supplied, real) fixtures deterministically,
including empty and error modes; core/projection tests use no network.

- [ ] The CLI drives the core through one clean intent/effect loop and renders
every view state as text; the error path drives the fake's error mode.

- [ ] The real adapter works behind the same port; swapping fake/real changes
nothing in `core`; adapter parsing is fixture-tested.

- [ ] Each milestone landed as its own commit/PR with its DoD met.

- [ ] `README.md` points to `design-philosophy.md` and this spec.

---

*Phase 1 (web UI: Leptos, design tokens, per-state snapshots, webview-tell
discipline) gets its own build spec once Phase 0 is landed and the spine is
proven.*
