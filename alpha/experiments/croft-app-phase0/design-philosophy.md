# Design Philosophy

A living document of the principles, decisions, and reasoning behind this
project. The goal is coherence: one set of principles governs every layer (core,
shell, design system) and every ecosystem ("pond"), so nothing drifts as the app
grows.

This is not a spec. It is the "why" that the specs and code should always be
checkable against. When a future decision feels uncertain, it should be
resolvable by appeal to something here. If it is not, that is a signal this
document needs to grow.

---

## 1. What we are building

A cross-platform, composable client for multiple social ecosystems, plus a
private group chat, presented through one crafted interface.

The ecosystems are deliberately kept as separate "ponds." They are not merged
into a single fused timeline or data model. The app gives each pond its own
honest surface while sharing one consistent visual identity and one consistent
set of interaction patterns across all of them.

Planned ponds, in rough order: own group chat, Bluesky, Mastodon (fediverse via
the Mastodon client API), Lemmy (forum). The architecture must make adding the
next pond a matter of presentation work, never structural change.

---

## 2. Core values

These are the things every decision serves. When values appear to conflict, the
tie-breakers in section 9 apply.

- **Consistency.** The app feels like one coherent thing across every platform
and every pond. A user learns it once.

- **Low maintenance.** The cost of keeping it alive and evolving it is modest.
Hard decisions are made once and reused, not re-made per platform or per pond.

- **Honest representation.** We never present a fiction that does not survive
outside the app. Different things are shown as different. We do not pretend two
ecosystems are one.

- **Crafted feel.** The interface earns the right to feel like itself, the way
the genuinely good apps do. No amateur smell.

- **Grow by use case.** We adopt patterns and add abstraction only at proven
need, never speculatively. Slim first, abstract when a real second case arrives.

- **Not extractive.** The app respects the user's attention and data. No
engagement-optimized ranking in the shell, no telemetry beyond crash and error,
no dark patterns. This is a stated invariant, not a configurable option.

---

## 3. The architectural spine

This is the single most important section. Almost everything else follows from
it.

### Functional core, imperative shell

The application logic lives in a pure core that performs no input or output. The
core is a function of the form: given the current state and an incoming intent,
return the new state plus a list of effects to be performed.

The core never touches the network, the clock, storage, or the screen. It
describes what it wants done as data ("fetch this," "store this") and hands those
descriptions to the shell. The shell performs them and feeds the results back in
as new intents.

Why this is the spine:

- It is the cheapest thing to test. A pure function needs no mocks. Behavior is
specified as plain given/when/then assertions.

- It is what makes one core span every platform. The core does no I/O, so it does
not care whether the host performing effects is a browser or a native shell.

- It is what makes protocol churn cheap. Network and parsing concerns are
contained at the edges, not threaded through the logic.

This is the same shape that Crux ships in production. We adopt the pattern, not
necessarily the framework (see section 8).

**The core describes effects; it never calls the port to perform them.** This is
the load-bearing consequence of "effects as data." The core emits an effect value
(for example, "fetch the feed from this cursor") and returns. Something on the
shell side later hands it a result intent. The core does not hold a port trait,
does not call it, and does not await it. This is precisely what lets the update
function stay a pure synchronous `(state, intent) -> (state, effects)`. An awaited
call cannot return from that signature, so a core that called the port would not
be this architecture. (The Phase 0 build spec makes this DECISION 1; see the
decisions log in section 13.)

### Three kinds of traffic cross the core boundary, and only three

- **Intents in.** Small serializable descriptions of what happened or what the
user wants. Not function calls into logic.

- **View models out.** Ready-to-render state. Never raw protocol records. The UI
paints a view model and nothing else.

- **Effect requests out, results back in.** The core asks for I/O as data; the
shell performs it and returns the outcome as a new intent. The "asks ... as data"
is literal: an effect is a value the core returns, not a method it invokes.

### The two guardrails

These two rules, held strictly, keep the architecture from eroding:

- **The core knows no platform.** If core code ever reaches for something
platform-specific, the architecture is leaking.

- **The design system knows no protocol.** If a protocol type ever leaks into a
presentation primitive, the architecture is leaking.

---

## 4. Honest seams (the pond principle)

We do not fuse ecosystems into a shared data model. Each pond keeps its own
native model end to end.

The reasoning, which arrived from three independent directions:

- **It is more honest.** A merged model is a fiction that only exists inside our
app. The moment a user acts on it elsewhere, it breaks.

- **The protocols force it.** ActivityPub deliberately has no canonical data
shapes. There is nothing to normalize against even if we wanted to.

- **The market confirms it.** The fused-timeline approach has been built
(Openvibe) and the considered critique is that it solves the wrong problem.

What we share is the chrome, not the content. The same visual components, the
same spacing, the same type scale, the same motion render every pond, so the app
feels like one thing. But what flows into those components is each pond's own
native shape.

A direct consequence worth stating, because the build spec relies on it: the core
holds each pond's native type, not a normalized one. The only place a native type
becomes display-ready is the view-model projection. The projection is the seam
between "native shape" and "shared chrome," and it is pure, so it is fully tested.

**The seam is about experience shape, not protocol lineage.** Lemmy and Mastodon
both ride ActivityPub, but a forum and a microblog are different experiences and
stay separate ponds. Shared substrate underneath is an implementation detail
beneath the seam, never a reason to merge surfaces.

### Aggregation is allowed only where it is honest

Some cross-pond views are fine because they do not require pretending different
things are the same:

- A notification count or list across ponds is honest (each item stays labeled by
origin).

- A merged scrolling timeline is not honest (it implies an equivalence that does
not hold).

Aggregation is opt-in, per-capability, and only for things that survive being
counted or listed side by side.

### Pinning is a top-level shell primitive

A pin is a reference, not a copy and not a merge. It holds the address (the
native ID in its native pond) and a type hint, never the content. When rendered,
each pin asks its own pond's module to hydrate it with live data.

This means a pinned Bluesky thread and a pinned Lemmy thread can sit side by side
at the top level, each rendered natively by its own module, without any fusion. A
pin asserts only one thing: that this item is important. It is private local
workspace state, never a network write.

Every module participates in pinning by declaring two things: that its items are
pinnable (they have a stable ID), and that it can hydrate an item from that ID.
The shell owns the pin list and the pinned strip. Modules own "given this ID,
render this." A pin whose target no longer exists must degrade gracefully.

---

## 5. The module contract (how ponds plug in)

Each pond is a module behind a port (a narrow trait). The module owns its native
types and its adapter to the underlying library. The shell and core never know
which library is behind the port.

**Where the port trait lives.** The trait belongs to the module crate and is
consumed by the shell, not by the core. The core depends on a module only for its
native data types (the shapes that ride inside intents and sit in the model),
never for the port trait. This keeps the core free of the async, I/O-shaped
surface a port necessarily has, and is what preserves the pure synchronous update
function. (Phase 0 DECISION 1; section 13.)

Consequences we rely on:

- Adding a pond is writing one module against the existing contract. The shell
does not change.

- The underlying library is swappable. We choose libraries behind ports
specifically so the choice stays reversible.

- A module declares its capabilities (is it pinnable, does it emit notification
events, can it be searched). The shell lights up aggregated views only for
modules that honestly support them. A module that cannot support a unified view
simply does not appear in it, rather than faking it.

Modules also do not perform their own I/O in the finished design. They describe
requests; the host's effect handlers perform them. The shell holds the port and
calls it; the module supplies the trait and an implementation, and the core stays
out of it entirely. The core never holds, calls, or awaits the port. (Phase 0
takes one documented liberty about *where* the backend is wired: the shell
constructs the adapter directly in its own startup path and hands it to the
runtime loop, rather than threading it through a general effect-handler
registry. That registry is a later phase; the loop holding one concrete port is
enough now. This is a soft, provisional detail — the hard, never-relaxed rule is
that the core still never receives, holds, or calls the trait.)

---

## 6. Cross-platform strategy

### Consistency over native-citizen feel

We choose one crafted self-identity everywhere over feeling like a perfectly
native citizen of each OS. The good apps (Things, Linear, Obsidian, Zed) all made
this trade and nobody holds it against them, because the identity is obviously
crafted.

This is only correct if the consistent thing is genuinely good. The entire risk
of this choice moves to execution discipline (section 7).

**Division of responsibility:** the core owns behavior (what happens, what is
shown), the shell owns feel (how it moves, how it responds to touch). The app has
one identity everywhere.

### One execution model

- The UI is one web app (web technology, because web is a first-class day-one
target, not a port).

- The core compiles to WebAssembly and runs inside the webview on every platform,
including desktop and mobile. This is what makes "desktop is a representation of
the web version" literally true rather than aspirational.

- The only thing that differs per platform is the set of effect handlers (who
performs the I/O). On web, the browser. On desktop and mobile, the native shell
with richer powers (filesystem, keychain, native notifications, routed network).

- A command-line shell is a legitimate fourth kind of shell. It drives the same
core through the same boundary, rendering view models as text and performing
effects with a plain native client. It is the cheapest test harness and dogfooding
surface, and it doubles as a purity check: if the CLI cannot render a view model
as text, the view model has a renderer assumption it should not have.

### Platform order

Web first, then desktop (macOS, as a wrapped representation of the web app), then
Android, then iOS and Linux. iOS reduces to "Android plus Apple signing and
push." Linux reduces to "macOS minus the webview rough edges." Neither requires
architectural change; they are new effect-handler sets and packaging.

(Phase 0 builds the core plus the CLI shell first, before any GUI, precisely
because it proves the spine with no UI-polish surface. The web UI is Phase 1.)

---

## 7. Craft as discipline (avoiding the amateur smell)

The crafted feel is not a talent problem, it is a discipline problem. The amateur
smell comes from a specific, enumerable list of failures:

- Motion and timing that ignore what the user is doing (the loudest tell).

- Inconsistent spacing and rhythm (values placed by eye).

- Typography not taken seriously (default line-heights, weak hierarchy).

- Optical and alignment sloppiness.

- Neglected states (empty, loading, error, edge cases).

- Webview tells (text selection where it should not be, wrong cursors, tap
highlights, browser-y scroll, webpage-style focus rings).

### Design tokens are the mechanism

A design token is a named design decision. Spacing, type scale, color, radius,
and motion values are each defined once, by name, in one place. Nothing in the
UI ever writes a raw value (no hardcoded pixel, hex, or duration). Everything
reaches for a token.

This makes the whole UI sit on one invisible grid and share one motion feel,
crafted once and correct everywhere. It is also why our architecture helps craft
rather than hurting it: a single token source is easier to get right and keep
right than re-achieving quality across several native UIs.

### The one rule

**Nothing ships placed by eye.** Every space, size, color, radius, and duration
comes from a token. The moment a value is hardcoded because it "looks about
right," that is the first drop of the amateur smell.

### Motion is shared intent, per-platform execution

The parameters of motion (what animates, durations, easing) are design-system
tokens, shared and crafted once. The frame-by-frame execution is the shell's job
per platform. This gives consistent, deliberately-tuned motion identity
everywhere without letting quality drift between platforms.

---

## 8. Test-first as a first-class citizen

Testing is not a phase, it is how we get the low-maintenance, evolve-
independently property. The architecture was, unnamed, a testability architecture
all along.

The layers, by what is actually testable:

- **Core behavior.** Full test-first. Given state, when intent, then new state
and effects. No mocks, deterministic, fast. This is the bulk of the logic and the
regression net. A feature is not done until its given/when/then exists.

- **View-model projection.** Also pure, tested the same way. Keep logic out of
components and in the projection, so components are too dumb to need testing. Much
of what looks like "UI behavior" is actually projection and tests this way.

- **Interaction wiring.** A thin layer of "this control emits that intent" tests.
Assert the wiring, not the behavior (already tested in the core). Most
interaction bugs are wiring bugs.

- **Visual quality.** The aesthetic judgment itself stays human, deliberately.
But everything around it that can regress silently is pinned by two mechanical
tools: token-contract tests (fail the build if any component resolves to a raw
value instead of a token) and per-state visual snapshots (fail on unexpected
render change, human approves the diff).

### The state-coverage habit

The highest-value UI testing habit: every state of every component gets a
required snapshot. The amateur smell lives in neglected states. If the suite
requires a snapshot per state, an undesigned state is a failing test. This makes
"every state was considered" a mechanical guarantee, not a hope.

This habit has a text-mode ancestor in Phase 0: the CLI renderer must render every
view state (loading, empty, feed, both footers, full error, inline error). Same
discipline, no pixels yet. The per-state snapshot rule in Phase 1 is the same idea
with images.

### Sequencing

The test and snapshot harnesses are part of the first walking skeleton's
definition of done, never retrofitted. Retrofitting tests onto untested code is
the thing that never actually happens.

---

## 9. Decision rules (tie-breakers and recurring questions)

These resolve the questions that come up repeatedly.

### Where does this code belong: core or shell?

- Can it cause a wrong result? It belongs in the core (correctness, behavior,
platform-independent, tested).

- Is it about how a specific platform's input or output physically behaves? It
belongs in the shell (efficiency, input characteristics, may vary per platform,
cannot cause a bug).

Example: the guard against firing a duplicate load while one is in flight is
correctness, so it lives in the core. Throttling the raw scroll-event firehose is
input-tuning, so it lives in the shell and is never load-bearing for correctness.

### How do we model state?

Make illegal states unrepresentable, but do not manufacture combinations that
cannot occur. Use an enum (one axis, sequential states) until a genuinely
independent second axis appears. Only then split that one axis into a separate
field. Do not reach for orthogonal fields when the combinations are mostly
nonsense.

A concrete application of this, from Phase 0: data a state cannot function without
(like the cursor a load-more or its error-recovery needs) goes *inside the
variant* that needs it, not in a separate optional field that could be `None` when
the variant demands it. "An error-while-appended with nothing to retry from"
should be unbuildable, not merely avoided. (Phase 0 DECISION 4. The invariant is
traced and proven in the build spec, section 3a.)

### When do we abstract or add a port?

At proven plurality, never before. Two real implementations justify a port. One
hypothetical future one does not. Ubiquitous, unlikely-to-be-swapped dependencies
(async runtime, HTTP client) are normal dependencies, not ports.

The Bluesky port is justified now even though Phase 0 has one real adapter, because
the *fake* is the second implementation and the second ecosystem is a near-term
certainty, not a hypothetical. The port earns its place by carrying the fake and
the real adapter from day one.

### When do we adopt a dependency vs build it?

Build the slim version ourselves when adopting would mean inheriting churn or
coupling we do not want (e.g. a pre-1.0 framework whose pattern we can mirror in a
few hundred lines). Adopt when the library is mature, fits behind a port, and
saves real work. Always keep protocol libraries behind ports so the choice stays
reversible.

### Consistency vs native feel, when re-litigated

The itch to add a platform-specific affordance because the app feels slightly
non-native is the consistency resolution working as intended, not failing. A
strong consistent identity is the feature. Re-open this only if the consistent
identity turns out genuinely poor, not merely non-native.

---

## 10. The stack (current decisions, with reversibility noted)

- **Shell:** Tauri. Owns the window and native effects on every platform. Chosen
for maturity, the proven Android path, and the notification-plugin ecosystem.

- **UI:** Leptos (Rust, compiled to WASM, renders to the DOM). Runs in the same
memory space as the WASM core, so there is no serialization boundary between UI
and core. The usual "thin component ecosystem" objection does not apply because we
build our own design system regardless.

- **Core:** pure Rust, compiled to WASM, run inside the webview everywhere. Also
compiles natively to back the CLI shell.

- **Bluesky module:** Jacquard (lean, ergonomic) or ATrium (conservative, broader
features today), behind a port. Reversible. Verify Jacquard's moderation coverage
before committing.

- **Mastodon module:** megalodon (multi-server: Mastodon, Pleroma, Firefish,
GoToSocial, Pixelfed), targeting the Mastodon client API (not ActivityPub C2S,
which is implemented by essentially nobody). Behind a port.

- **Lemmy module:** lemmy-client-rs (official, WASM-aware, manages version skew).
Behind a port. Lemmy is explicitly designed to support alternative frontends, so
this fits the grain.

- **Chat:** our own Rust library (a given).

- **Server side:** a given. Note the carried requirement: background push
fundamentally requires the server to send to FCM/APNs, because a closed client
cannot receive push on its own.

The renderer-spectrum note: we rejected GPU-native (Zed/GPUI) because web-first
needs a real web target. We rejected Dioxus-owns-everything because it would put
our least-mature, still-0.x dependency on the load-bearing mobile-push path,
where the entire plugin ecosystem assumes Tauri.

---

## 11. Known risks and watch points

- **Android background push** is the one genuine load-bearing risk. It works
through community 0.x Tauri plugins plus a Kotlin FCM service, and requires
server-side push origination. Isolate it behind a notification port so the plugin
is swappable. Verify the chosen plugin is current before relying on it.

- **The beautiful-shell-to-finished-feel gap** is where projects like Spacedrive
stalled. The crafted shell is the part that demos well and finishes hard. Treat
polish (motion, states, webview tells) as the actual work, not the finish.

- **State neglect** is the most likely place craft is lost. The per-state
snapshot requirement is the defense.

- **Pre-1.0 dependency churn** (Crux if adopted, the push plugins, the atproto
crates). Ports and our own thin plumbing are the insulation.

- **The Leptos-in-Tauri combination** is slightly less trodden than the
mainstream paths. Expect occasional assembly over tutorial-following. The failure
mode is debugging, not a dead end.

---

## 12. Why the whole set is coherent

Every decision serves the same small handful of values, and the architectural
choice (pure core, effects as data, honest seams) turns out to be simultaneously
the cheapest to maintain, the easiest to test, the most portable across platforms,
and the most honest about the protocols.

When one structural decision is independently the right answer to maintenance and
testing and portability and honesty, that is not a coincidence. It is a sign the
underlying model fits the problem.

The places that remain open are all either reversible (which atproto crate, which
push plugin) or deferred (iOS, Linux), and they are behind ports specifically so
they stay reversible.

---

## 13. Decisions log

Concrete, binding decisions live here once made, with a one-line reason and a
pointer to where they bite. The philosophy above is the "why"; this is the "what
we actually settled." Phase numbers refer to the build specs.

**Phase 0 - DECISION 1. The shell holds the port; the core never depends on the
trait.** The port trait lives in the module crate and is consumed by the shell.
The core depends on a module's native data types only. Reason: the pure
synchronous `update` signature cannot hold if the core awaits a port; effect-as-
data and effect-as-call are different architectures and we are the former.
Bites: crate dependency graph, core purity, WASM-cleanliness of the core.

**Phase 0 - DECISION 2. The core stores the native post type directly.** The model
holds the module's native post shape, not a core-defined or normalized one.
Reason: honest seams; the projection is the only native-to-display conversion
point. Bites: the `core -> module` data-type dependency edge, the projection.

**Phase 0 - DECISION 3. Phase 0 timestamps are absolute, formatted from post
data.** Relative time is deferred because it would require the clock to enter the
projection as a parameter, and there is no need yet. Reason: the core does not read
the clock. Bites: the projection's timestamp field.

**Phase 0 - DECISION 4. Cursor-bearing states carry the cursor in the variant.**
States that cannot function without a cursor hold it non-optionally in the variant,
so the illegal "recovery state with nothing to retry from" cannot be built.
Reason: make illegal states unrepresentable. Bites: the `FeedStatus` enum shape and
the retry paths. The invariant is proven in build spec section 3a. Note the
enforcement is *structural*, not disciplinary: the cursor lives inside the
variant, so the type system rejects a cursorless `LoadingMore`/`ErrorWhileAppended`
at compile time. No runtime assertion or state-machine validator is needed, and
the section 3a enumeration is a guide for anyone *adding* states, not a check the
code relies on.

**Phase 0 - DECISION 5. `reason` is a string for now; the CLI error path drives
the fake.** No structured error type yet (grow by use case), and the shell injects
failures by configuring the fake's error mode rather than inventing its own.
Reason: slim first; single source of failure behavior. Bites: intent/status fields,
the CLI command set. Assumption: the fake's error mode is enough to exercise every
core error-handling path in Phase 0, because the core collapses all failures to a
single `FeedLoadFailed { reason }` intent regardless of cause. Watch point: when
the real adapter introduces failure *kinds* the fake cannot inject (timeout,
rate-limit, a specific protocol error), either extend the fake's error mode or add
a second injection path — and if a structured error type arrives, the acceptance
tests should name which core error path each case drives.

---

*This document grows. When a decision is made that future-us would want the
reasoning for, it belongs here. New binding decisions go in section 13 with a
reason and a "bites" pointer.*
