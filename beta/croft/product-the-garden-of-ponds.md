# Croft the product: a composable garden of ponds and pads

date: 2026-07-07

status: product-shape synthesis. The client architecture is green-real (proven in code); the visual
system, the inclusion catalog, and the forward build plan are settled design intent unless flagged
otherwise. Decision-gated product calls are surfaced below, not resolved.

verification: the shared-core/thin-shell spine and its five binding decisions are green-real (Phase 0,
20/20 acceptance tests, DECISION 1 through DECISION 5). Everything downstream of that spine (the
inclusion pathways, the fair-reveal primitive, the on-device assistant tiers, the webxdc mitigations,
the visual system) is design and intent, and is marked as such where it matters.

---

## Overview

This doc holds the shape of Croft the product: what the thing is, how the client is built, and the bar
every piece of it must clear. Croft is a composable, user-respecting garden. One warm, consistent shell
hosts many units of two kinds, and the user assembles their own garden from whichever units earn a place
for them.

A **pond** is a connection to an existing social ecosystem (Bluesky, the fediverse via the Mastodon
client API, Lemmy). It brings external native data the user does not own, speaks that ecosystem's own
protocol, and keeps its native shape end to end.

A **pad** is a small self-contained app that runs in the shell (a game, a tool). It does not bring an
external ecosystem; it is an experience that runs locally or peer to peer, sandboxed and
permission-scoped.

Ponds are the bodies of water you connect to; pads are the lily pads that float in the garden. Both share
the shell, the design system, and one consistent set of interaction patterns, so the whole thing feels
like a single crafted object even as the set of units changes per user.

The garden grows from the social graph. That reframe (the social graph is the substrate; the durable
group is the bottom of the pyramid and chat is one tenant, peer to other activities hung off the group)
is the anchor for the whole product, and it lives in the sibling doc `social-graph-as-substrate.md`. This
doc does not reproduce it; it covers the product shape that rides on top: the garden thesis, the honest
seams, the functional-core spine, the client architecture, the quality bar, and the three inclusion
pathways.

## 1. The garden thesis: composability is the value, not a feature

Composability is itself the user-respecting value. It is a stance, not a feature.

The app does not decide how useful any given pond or pad is to a given user. The user decides, by shaping
their own garden. A pond that matters to one person is absent and unmissed for another, and the design
must let it be present-and-useful for the former and absent-without-feeling-incomplete for the latter.
"Useful or not to a given user" is a property of the design working as intended, not a failure of it.

The UI guarantees (consistency, warmth, low mental load, follow-or-ignore) are the constant soil quality.
The ponds and pads are what the user chooses to grow. The soil stays good no matter what is or is not
planted.

The garden test gates whether a unit should exist at all:

> Does it add optional leverage a user can adopt or ignore, without the surface feeling broken either way?

If yes, it is a candidate. It can be tried cheaply (one module behind a port) and kept only if it earns
its place. Abandoning a unit that fails to earn its place costs nothing structural, because it was always
just one module behind a port, which is exactly what makes speculative units safe to try. The garden test
gates existence; the design-criteria bar (section 5) gates behavior once a unit exists.

## 2. Honest seams

The garden does not fuse ecosystems into a shared data model. Each pond keeps its own native model end to
end. What is shared is the chrome, not the content: the same visual components, spacing, type scale, and
motion render every pond, but what flows into those components is each pond's own native shape.

The reasoning arrives from three independent directions:

It is more honest. A merged model is a fiction that only exists inside the app, and it breaks the moment a
user acts on that content elsewhere.

The protocols force it. ActivityPub deliberately has no canonical data shapes, so there is nothing to
normalize against even if fusion were wanted.

The market confirms it. The fused-timeline approach has been built and the considered critique is that it
solves the wrong problem.

The seam is about experience shape, not protocol lineage. Lemmy and the fediverse both ride ActivityPub,
but a forum and a microblog are different experiences and stay separate ponds. Shared substrate underneath
is an implementation detail beneath the seam, never a reason to merge surfaces.

Two consequences the rest of the architecture leans on:

Aggregation is allowed only where it is honest. A notification count or list across ponds is fine (each
item stays labeled by origin); a merged scrolling timeline is not (it implies an equivalence that does not
hold). Aggregation is opt-in, per-capability, and only for things that survive being counted or listed
side by side.

Follow-or-ignore is mandatory in both halves. Ignore is the default: a non-technical user never has to
know where a post physically lives or which protocol it rides. Follow is always available: a curious user
can always peel the surface back to the origin. Hiding the seams smooths them for the eye; it never welds
them shut. Every piece of content carries honest attribution, every borrowed technique or upstream project
is credited, and the path down to the source is real, never a dead end.

## 3. The functional-core / imperative-shell spine

This is the single most important structural decision, and almost everything else follows from it.

The application logic lives in a pure core that performs no input or output. The core is a function of the
form: given the current state and an incoming intent, return the new state plus a list of effects to be
performed. The core never touches the network, the clock, storage, or the screen. It describes what it
wants done as data ("fetch this," "store this") and hands those descriptions to the shell. The shell
performs them and feeds the results back in as new intents.

The load-bearing consequence is that the core describes effects; it never calls a port to perform them.
The core emits an effect value and returns; something on the shell side later hands it a result intent.
The core does not hold a port trait, does not call it, and does not await it. This is precisely what lets
the update function stay a pure synchronous `(state, intent) -> (state, effects)`. An awaited call cannot
return from that signature, so a core that called the port would not be this architecture (this is binding
DECISION 1, green-real in Phase 0).

Exactly three kinds of traffic cross the core boundary, and only three:

Intents in: small serializable descriptions of what happened or what the user wants, never function calls
into logic.

View models out: ready-to-render state, never raw protocol records. The UI paints a view model and
nothing else.

Effect requests out, results back in: the core asks for I/O as data; the shell performs it and returns the
outcome as a new intent.

Two guardrails, held strictly, keep the architecture from eroding:

The core knows no platform. If core code ever reaches for something platform-specific, the architecture is
leaking.

The design system knows no protocol. If a protocol type ever leaks into a presentation primitive, the
architecture is leaking.

This is the shape Crux ships in production. Croft adopts the pattern slim, not necessarily the framework,
because the pattern can be mirrored in a few hundred lines and adopting a pre-1.0 framework whole would
inherit churn behind a port anyway. Honest seams is made structural here too: the core holds each pond's
native type (DECISION 2), and the only place a native type becomes display-ready is the pure view-model
projection, which is fully tested.

## 4. One shared core plus thin per-platform shells (the client architecture)

The client architecture is a single shared, pure functional core consumed by thin per-platform shells,
with two orthogonal callout axes. This is the accepted Croft client architecture, and new and prior client
work adapts to it. It is green-real: Phase 0 demonstrated the shape running across CLI, web, and desktop
surfaces with 20/20 acceptance tests passing.

```
SHARED (platform-agnostic, no I/O, no async, no clock; WASM-clean)
  core     pure (state, intent) -> (state, effects) + projection (model -> view model)
  shell    cross-platform composition logic (layout, slots, pinning)
  design   design system (tokens, primitives)
  <port>   the I/O contract as a trait, held by the shell, never called by the core (DECISION 1)

PER-PLATFORM SHELLS (thin; the only place platform code lives)
  cli      effects.rs + runtime loop + text render
  web      effects.rs + runtime + components
  desktop  effects.rs + native host
  ...      future surfaces add only their own effects.rs + render target
```

The two callout axes:

Platform axis: each platform shell provides its own `effects.rs`, the handler that performs the core's
effect-requests (emitted as data, never as function calls) and feeds results back in as new intents. Same
core, different effect-performers.

Implementation axis: swappable adapters behind a port, orthogonal to platform. A pond port carries a
fixture-backed fake and a real adapter; the chat Transport port carries an in-process fake and a later
real peer-to-peer adapter. The core and shells are blind to which adapter is wired.

Maintenance minimization is the stated driver. Business logic lives once in the core; a new platform is a
new thin shell (one `effects.rs` plus a render target), not a re-implementation; and behavior cannot drift
between surfaces because they share the core verbatim. A command-line shell is a legitimate fourth kind of
shell and doubles as a purity check: if the CLI cannot render a view model as text, the view model has a
renderer assumption it should not have.

**Decomposition: per-pond cores unified by the shared shell (option C).** Not one god-core (which would
couple a feed read-model with a group engine), and not two disconnected cores (which would re-fatten the
shells). Each pond is symmetric: add a domain core plus its port, and the existing shell composes both
view models. This is honest seams made structural (ponds kept native, not fused). A carried caveat: the
chat core is richer than the feed read-model (bidirectional and real-time, with more complex state and
governance planes), so the pattern transfers cleanly but the core content is substantially more complex.

**The awareness-vs-interactivity line keeps option C clean.** Cross-pond awareness (read-only surfacing of
one pond's content inside another's view) is expected now and cheap: it is composition in the shell, where
a message carries an `at://` reference, the shell resolves it via the other pond's port to a renderable
card, and nothing flows back. Cross-pond interactivity (acting in pond A from pond B) needs a broker to
translate idioms; it sits between cores, never inside a pond core, and is deferred by default per honest
seams until a concrete cross-pond action is committed to.

## 5. The quality bar and the visual-system discipline

The crafted feel is not a talent problem, it is a discipline problem. The amateur smell comes from a
specific, enumerable list of failures: motion and timing that ignore what the user is doing; inconsistent
spacing placed by eye; typography not taken seriously; optical and alignment sloppiness; neglected states
(empty, loading, error, edge cases); and webview tells (stray text selection, wrong cursors, browser-y
scroll, webpage-style focus rings).

The mechanism against all of that is design tokens. A design token is a named design decision; spacing,
type scale, color, radius, and motion values are each defined once, by name, in one place. The one rule:

> Nothing ships placed by eye. Every space, size, color, radius, and duration comes from a token.

The moment a value is hardcoded because it "looks about right," that is the first drop of the amateur
smell. Two mechanical tools pin what can regress silently: token-contract tests (fail the build if any
component resolves to a raw value) and per-state snapshots (fail on unexpected render change, a human
approves the diff). The highest-value habit is that every state of every component gets a required
snapshot, so an undesigned state is a failing test rather than a hope.

Every pond and pad is checked against a shared criteria set adapted from mature super-app design guidance,
with the host-as-gatekeeper mechanics stripped out. The shared bar is five principles, and it is a real
checklist, not a word list — each criterion is here because a unit got it wrong the hard way, so the next
unit inherits the lesson.

*Friendliness* (minimize interference with the goal): each view has one clear key point, nothing on it
competes with the user's actual decision, and the surface is calm by default — lead with the essential and
reveal detail progressively. Not extractive: no tracking, no engagement-optimized reordering, no dark
patterns; content appears in source order or the user's explicit sort.

*Clarity* (navigation, feedback, recoverable exceptions): the user always knows where they are, where they
can go, and how to return. Two rules are load-bearing. First, the **depth-of-three exit-trap**: past two
levels of interior depth, provide a persistent way to move between sections and get home, because the
documented super-app failure case is that depth-of-three traps users; the shell guarantees the escape hatch
even when a unit owns its interior. Second, a **feedback taxonomy** matched to the moment — a lightweight,
auto-dismissing confirmation for success and minor status (never for errors); a persistent, clearly
communicated treatment for errors, which must be seen and understood, never flashed and gone; and a result
view for the completion of a process, with the next step if any. Loading is local and in-place rather than a
modal that hides the view and manufactures anxiety, with **one loading animation per view, never more**, and
a cancel plus progress when it runs long. Every error state is designed, states its cause, and offers a
recovery — no dead ends.

*Convenience* (respect the input device and the hand): prefer choices, recognition, and existing data over
making the user type. Touch targets are generous — the documented physical floor is roughly **7-9mm**, treated
as a floor and larger for primary actions — so mis-taps do not happen. Destructive actions are hard to
trigger by accident and easy to undo.

*Consistency* (the shared vocabulary): controls and interaction patterns are consistent across every view and
every unit, achieved by the unit opting into the shared design system rather than by the host stamping an
immovable chrome on top; recognition over recall, the same icon and label meaning everywhere.

*Accessibility* (first-class, not an afterthought): the world's largest app treats elderly-friendly and
accessible design as named, mandatory, separate concerns, and so does a product meant to serve everyone from
grandparents to young children. Respect system font-size and scaling so layouts hold when type is enlarged;
color never carries meaning alone (every color signal is paired with an icon, shape, position, or label,
which matters acutely for a green-and-warm palette where red-green deficiency bites); large hotspots, clear
labels, strong contrast; and save-and-resume wherever a task has any length. On top of the shared bar:

Pond-extra criteria: honest seams, attribution and traceability, capability declaration, broker as little
as possible, graceful degradation.

Pad-extra criteria: sandbox and isolation, user-authorized permission scopes, vetted by default, instant
start, save and resume, trust mechanics where outcomes matter, and social-graph discovery rather than a
store.

The visual system is expressed as roles, not a flat list of colors: a warm surface (cream), a
high-contrast ink for text and edges, a restrained brand (deep green kept deliberately distinct from any
success-green), and a small fixed set of semantic accents tied to what a moment means (attention,
pending) that does not grow as ponds multiply. Status semantics (success, warning, error) are held apart
from the attention accents. Color never carries meaning alone: every color signal is paired with a
non-color signal (icon, shape, position, label), which is both an accessibility requirement and a
robustness one. A pond shows its own true brand color only in the space it owns (honest attribution), so
the shell's meaning-color and a pond's identity-color never fight because they do different jobs in
different places. The specific hex values and shade ramps live in the design system's tokens; this states
the roles and intent they must satisfy.

## 6. The three inclusion pathways for activities

Three inclusion pathways determine how much effort a pad (an activity or game) costs, not how fun it is.
Which one applies depends on what the source already speaks.

**Build-fresh.** Write it in the Rust core. Correct where the rules are simpler than an integration would
be, where a clean license is wanted, and where the outcome-record loop should be native rather than bolted
on. This covers nearly every abstract classic and the drawing game. Build-fresh also sidesteps trademark
name traps (generic names, own boards): the mechanics are free, the names and boards are not.

**Wrap.** Implement a webxdc-compatible API surface (`sendUpdate`, `setUpdateListener`,
`joinRealtimeChannel`) backed by the iroh transport, and an entire open catalog becomes wrappable for the
cost of one compatibility layer rather than one effort per game. The catch is that each wrapped app's
license and assets are inherited and quality varies, so the surfaced set is still hand-picked.

**Port.** For non-webxdc games that speak raw WebRTC, polyfill the browser WebRTC API onto a direct iroh
QUIC stream and mock the signaling handshake. Because direct point-to-point channels are available (not
broadcast-only), the port host is a better host than webxdc's broadcast layer. This is the path for the
real-time arena games and the netcode-framework games.

The decision rule: webxdc-native game then wrap, raw-WebRTC game then port, everything else build-fresh.
License verification on wrapped and ported code is flagged for bundle time; the load-bearing netcode
dependencies in particular need a live license check against their repos before bundling.

**The deep-link resolver is tier-zero.** It is the single most strategically important component because
it is simultaneously the core navigation UX and the entire acquisition model (there is no public discovery
by design). Its URL grammar composes pond identity and capability, then app id and version, then instance
and entry context, shareable at three depths. One catalog manifest feeds it. Cold-install deep-linking is
not privately achievable (the seamless-deferred mechanisms that would enable it are gone or require
fingerprinting, which conflicts with the no-tracking stance), so cold arrival resolves to a claim-code
"one-more-tap" flow, framed as a feature rather than a failure. This is design and intent.

**Fair-reveal is the leverage primitive.** One reusable commit-reveal module powers governance-grade
voting, dice fairness, and hidden-info games at once. It is built once and early because it unlocks a
whole column of activities at a stroke, and its bridge value is that local real-time voting is a fun group
toy that is also cooperative-governance infrastructure. Sequencing note (design intent): prove one pond
fully end to end before building three, because the integration surface (resolver, transport, context
boundaries, and shell cohering into something a non-technical person finds simple) is where local-first
projects actually die.

## 7. The on-device assistant: detect-first, accelerate-never-gate

A natural-language assistant ("any travel games?" resolving to ranked internal deep-links plus a one-line
orientation) is an attractive front-end onto the same resolver section 6 already builds: intent text in,
real catalog links out, one resolver and one manifest consumed by two front-ends (links others send, and
language the user types). But it rides on an on-device model, and there is no single on-device model that
can be assumed present. Coverage is a patchwork: strong on capable Apple devices with token-level guided
generation, workable but steeply device-gated on Android flagships, real but desktop-only and flag-gated
in Chrome, and effectively absent in Firefox and mobile web (bundled-WASM-or-nothing there).

That coverage reality forces the invariant, and the invariant is exactly the rule that keeps the
navigation spine honest:

Detect availability first, on every platform. When the model is absent, the assistant simply is not
offered. No dead ends, no degraded-but-broken state.

Strictly optional and gracefully absent. It is an accelerant, trivially disabled, never load-bearing.

Accelerate but never gate. Every path the assistant offers must also be reachable without it, so the
menus, verbs, pins, and deep-links must be a complete navigation system on their own. The assistant only
ranks intent onto links that already exist and adds no navigation plumbing, and its output is constrained
hard to real catalog links (the model is trusted only to rank intent, never to emit a link that does not
resolve).

Held this way, uneven model coverage stops being a problem and becomes a bonus on devices that have it. The
deep-link grammar is the spine; the assistant is one optional language adapter onto it, never a second,
privileged way in. This is verified feasibility (vendor-documented as of mid-2026) plus design intent for
the tiered engine handling; specific model sizes and device lists move monthly and are re-checked at build
time.

## 8. The pad security bar

Pads run third-party code, so the sandbox is the only thing standing between a malicious bundled dependency
and the user. The governing lesson, from the Cure53 audit of the webxdc ecosystem, is blunt: **CSP alone
does not contain a webview.** WebRTC connections, DNS-prefetch, and several CSP-bypass paths were all found
to be able to exfiltrate data from a webview that CSP believed it had isolated. The webxdc project's fixes
were correspondingly blunt (on Chromium, exhaust the per-process WebRTC connection cap so no new one can
open; on WebKit, delete `RTCPeerConnection` from the namespace entirely; block DNS in the renderer).

Because Croft wraps its own webview host rather than inheriting that project's, it inherits the problem but
not the fixes. Three product-layer commitments follow (design intent, action-flagged per platform):

Disable WebRTC in the webview, per platform. This is pure upside and the one genuinely-owned action item:
the transport is iroh QUIC, not browser WebRTC, so a pad never needs the webview's WebRTC at all, and
disabling it closes the exfiltration hole at zero functional cost. DNS-prefetch and speculative connections
in Chromium-family webviews need an equivalent check at the embedding layer.

Make honest seams a hard context boundary, not just a UX one. The pad surface runs third-party code; the
social panels run first-party code against the pond protocols. Keeping those as separate webview contexts
(separate origins, separate capabilities) means a compromised pad cannot reach the social session's tokens
or identity keys. The seam that is a UX boundary is also a security boundary.

Hand pads an ephemeral per-match pseudonym by default. Identity comes from the cryptographic identity
layer, not a self-reported field (which is stronger than the spoofable in-app identity the audited
ecosystem exposed). But a pad that can read a stable identifier can correlate a player across sessions, so
the default is a per-match pseudonym, with durable identity exposed only on explicit user action.

## Open decisions (surfaced, not resolved)

These are product calls that remain open; they are tracked in `../OPEN-THREADS.md` and stay surfaced rather
than closed here.

The group's-face UX. The one surface a group genuinely needs is its home or face, and the hardest UX
problem in the whole product is keeping that face from becoming a settings page for a graph node. Entry
points are plural but convergent (a person, an activity, a chat, or the face all route to the same group).
The iteration of this surface is open. The substrate reasoning behind it lives in
`social-graph-as-substrate.md`.

Reconciling the sticky-group lifecycle with membership-vs-access. Whether and how a group becomes durable
by an affirmative "sticky" act, and how that lifecycle lines up with the separate axes of who governs a
unit (membership) versus who may enter or post in a given pad (access), is not settled at the product
layer. The two axes are deliberately decoupled, but their product surfacing is an open call.

## What this establishes (and does not)

Establishes the product shape of Croft as a composable garden of ponds and pads riding on one proven
spine. Composability is a stance, not a feature (the garden test gates existence, the design-criteria bar
gates behavior). Honest seams keep every pond native, with follow-or-ignore mandatory in both halves. The
functional-core / imperative-shell spine (a pure synchronous `(state, intent) -> (state, effects)` core
emitting effects as data, with the core knowing no platform and the design system knowing no protocol) is
green-real: one shared core plus thin per-platform shells, two callout axes, and per-pond cores unified by
the shared shell (option C), all demonstrated in Phase 0 at 20/20 with five binding decisions proven. The
quality bar is token-enforced ("nothing ships placed by eye"), with a role-based visual system and
mechanical state coverage. Activities enter by three inclusion pathways (build-fresh, wrap, port) over a
tier-zero deep-link resolver and a build-once fair-reveal primitive. The on-device assistant is strictly
optional (detect-first, accelerate-never-gate, every path reachable without it). The pad security bar
turns iroh-as-transport into a free WebRTC-containment win, makes honest seams a hard context boundary, and
defaults pads to a per-match pseudonym.

Does not establish (and points elsewhere): the social-graph-as-substrate reframe that anchors the product
(the durable group, group identity versus member set, the group lifecycle), which is the sibling doc
`social-graph-as-substrate.md`; the neutral protocol and reference core the product consumes; the voice and
pitch used to socialize the product; or the institution that operates it. Two product calls remain open and
surfaced rather than resolved: the group's-face UX iteration, and reconciling the sticky-group lifecycle
with the membership-versus-access split. Everything downstream of the green-real spine (the inclusion
pathways and resolver, the fair-reveal primitive, the on-device assistant tiers, the webxdc mitigations,
and the visual system) is settled design and intent, not yet proven in code.
