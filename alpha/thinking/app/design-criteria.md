# Design Criteria: Ponds and Pads

The standard that every pond and every pad is checked against. The
`design-philosophy.md` doc holds the why (the values, the architecture, the
garden thesis); this doc holds the bar each composable unit must clear to belong
in the garden. It is a checklist and a rationale, not a visual spec or a
committed palette.

Structure: shared criteria first (everything must meet these), then the
pond-specific and pad-specific additions, then the visual system (palette,
semantic color, typography intent). The four-principle skeleton is adapted from
the most mature super-app design guidance (WeChat's), reframed onto this
project's values and stripped of its host-as-gatekeeper mechanisms.

---

## 0. The two kinds of composable unit

- **A pond** is a connection to an existing social ecosystem. It brings external,
native data the user does not own (Bluesky, Mastodon, Lemmy), speaks that
ecosystem's own protocol, and keeps its native shape (honest seams). A pond's
extra criteria are about honesty, attribution, and brokering as little as
possible.

- **A pad** is a small, self-contained app that runs inside the shell (a game, a
tool, a webxdc-style guest). It does not bring an external social ecosystem; it
is an experience that runs locally or peer-to-peer. A pad's extra criteria are
about sandboxing, permission scopes, vetting, and trust.

Metaphor that holds: ponds are bodies of water you connect to; pads are the lily
pads that float in your garden. Ponds are ecosystems; pads are apps.

Both share the design system, the shell, and all of the shared criteria below.
They differ only in the unit-specific sections.

---

## 1. Shared criteria (every pond and pad must meet these)

Organized under four principles. These are the bar; failing one is a reason to
fix or to not surface the unit.

### 1.1 Friendliness (minimize interference with the user's goal)

- Each view has one clear key point. The user understands what this screen is for
within a moment of arriving.

- Nothing on a view distracts from the user's actual decision or action. No
elements unrelated to the goal compete for attention.

- The surface is calm by default. Lead with the essential; reveal detail
progressively as interest deepens (this is follow-or-ignore as a layout rule:
the first thing shown is enough, more is available below or deeper).

- Not extractive. No tracking, no engagement-optimized reordering, no dark
patterns. Content appears in the order the source provides or by the user's
explicit sort, never an attention-farming reorder.

### 1.2 Clarity (navigation, feedback, recoverable exceptions)

- The user always knows where they are, where they can go, and how to return.

- Depth discipline: keep the interior shallow. Past two levels of depth, provide
a persistent way to move between sections and to get home, so the user never
feels lost or accidentally exits. (Borrowed from the documented super-app failure
case where depth-of-three traps users.)

- Guaranteed exit. The user can always get back out. The shell guarantees the
escape hatch even when a unit owns its interior. (The host standardizes the
connective tissue, the way back, not necessarily the whole interior.)

- Timely feedback. Minimize waiting; when waiting is unavoidable, show it.

  - Prefer local, in-place loading feedback over modal loading that covers the
  view (modal loading hides what is happening and causes anxiety).

  - One loading animation per view, never more.

  - If loading is long, offer a cancel and show progress.

- Feedback taxonomy (use the right one for the moment):

  - Lightweight auto-dismissing confirmation for success and minor status. Never
  for errors.

  - Persistent, clearly-communicated treatment for errors. An error must be seen
  and understood, never flashed and gone.

  - A result view for the completion of a process, with the next step if any.

- Exceptions are controllable and always provide a way out. Every error state is
designed (not an afterthought), states its cause, and offers a recovery. The user
is never stuck on a dead end. (This is the per-state-design discipline; it is also
follow-or-ignore's promise that no path is a dead end.)

### 1.3 Convenience (respect the input device and the hand)

- Reduce input. Prefer choices, recognition, and existing data over making the
user type. Offer history and selection over free text where possible.

- Generous touch targets. Clickable areas are large enough for a finger, never so
small or dense that mis-taps happen. (The documented physical bar is roughly
7-9mm; treat that as the floor, larger for primary actions.)

- Avoid mistakes by construction. Destructive actions are hard to trigger
accidentally and easy to undo.

### 1.4 Consistency (the shared vocabulary)

- Consistent controls and interaction patterns across every view and every unit.
The user learns the app once and that knowledge transfers everywhere.

- The unit renders through the shared design system (tokens, primitives, the
shared chrome). Consistency is achieved by the unit opting into the shared
vocabulary, not by the host stamping an immovable overlay on top. (We take the
consistency super-apps achieve without the mandatory host-controlled chrome that
produces it for them.)

- Recognition over recall: instantly recognizable icons and labels, the same
meaning everywhere.

### 1.5 Accessibility (first-class, not an afterthought)

The world's largest app treats elderly-friendly and accessibility design as
named, mandatory, separate concerns. Given the goal of serving everyone from
grandparents to young children, so do we.

- Respect system font-size and scaling. Text remains legible and layouts hold
when the user has enlarged type.

- Color never carries meaning alone. Every color signal is paired with a
non-color signal (icon, shape, position, label). This is both an accessibility
requirement (color-vision deficiency is common, and a green-and-warm palette is
exactly where red-green deficiency bites) and a robustness one.

- Large hotspots, clear labels, strong contrast for text and edges.

- Save-and-resume where a task has any length, so it can be set down and picked
up later.

---

## 2. Pond-specific criteria

A pond meets all shared criteria, plus:

- **Honest seams.** The pond keeps its ecosystem's native data shape. No fusing
into a normalized cross-pond model. The only place native becomes display-ready
is the view-model projection.

- **Attribution and traceability (follow-or-ignore at the data layer).** Content
is honestly attributed to its origin. A curious user can always follow a piece of
content down to its real source in its real ecosystem. The pond may show its own
true brand color in its own header as attribution (this is where a pond's real
identity appears, honestly, in the space it owns), distinct from the shell's
semantic accents.

- **Capability declaration.** The pond declares what it honestly supports
(pinnable items, notification events, search, aggregation). The shell lights up
shared/aggregated views only for ponds that genuinely support them; a pond never
fakes a capability it lacks.

- **Broker as little as possible.** Direct fetches for the bulk of public content;
route through our own infrastructure only where it adds genuine functional
leverage (custom state, private overlays). The expensive thing (content) stays on
the direct path; only the cheap, private thing involves our server.

- **Graceful degradation.** When the ecosystem cannot supply something (a missing
thread parent, a deleted item, an unreachable source), the pond degrades visibly
and gracefully rather than breaking or pretending.

---

## 3. Pad-specific criteria

A pad meets all shared criteria, plus:

- **Sandbox and isolation.** A pad runs contained. It cannot crash the host, and a
pad failure is caught and shown as a recoverable error, not a host crash. A pad
cannot reach beyond what it has been granted.

- **Permission scopes, user-authorized.** A pad declares the capabilities it needs
(its scopes). The user authorizes them; the host enforces them. This is the
honest version of the mini-app permission model: it constrains the guest, it does
not surveil the user. A pad gets only what it was granted, and the user can see
what that is.

- **Vetted by default.** Pads are curated, not an open store. Vetting covers code
quality, onboarding smoothness, and the non-extractive guarantee (no tracking, no
phoning home, instant start, works first time for a non-technical user). The
ecosystem is inconsistent in maturity; the curated set is how the quality and
values bar is kept.

- **Instant start, no heavy onboarding.** A pad launches immediately. No
multi-hundred-megabyte downloads, no fiddly multi-step setup before first use.
(The anti-pattern is the impressive-but-heavy pad that demos well and onboards
terribly; that is the wrong thing to surface, especially as a first impression.)

- **Save and resume.** A pad that holds any state lets the user leave and come
back without losing it.

- **Trust mechanics where outcomes matter.** If a pad produces a shared or
published result (a game outcome on a leaderboard), it carries an appropriate
trust mechanism (for example, mutual attestation by the participants) rather than
accepting unilateral self-report where that would be gameable. The strength of
the mechanism scales with the stakes; low-stakes social fun may accept less.

- **Social-graph discovery, not a catalog.** A pad is discovered the way
super-apps actually drive discovery: through the social graph (a friend shares or
challenges), not through browsing a store. This is native to the federated-social
model and is the proven, higher-converting discovery path.

---

## 4. The visual system

This section states intent and structure, not final committed values. The token
discipline from the philosophy doc applies: nothing is placed by eye; everything
resolves to a named token.

### 4.1 Palette roles

A disciplined palette is a small set of roles, each expanded into shades, not a
flat list of colors. The roles:

- **Surface (cream).** Backgrounds, cards, the calm field. Warm, like
unbleached paper.

- **Ink (near-black).** Text, outlines, crisp edges and eye-distinction. The
high-contrast keyline that makes type and shapes legible.

- **Brand (deep green).** Identity, used with restraint. Deliberately deep and
saturated, so it never reads as the same role as a success-green.

- **Semantic accents (a small fixed set, by meaning).** Accent is tied to what a
moment *means*, not to which pond it belongs to. This stays a small finite set
that does not grow as ponds multiply, and it means the same thing everywhere,
reinforcing consistency.

  - **Attention / notification (orange-terracotta).** "Something wants you." The
  higher-urgency draw-the-eye color.

  - **Pending / in-progress (warm yellow).** "Something is happening, hold on."
  The gentler, lower-urgency activity color.

- **Status semantics (derived, held apart).** Success, warning, error. These are
a different job from the attention accents and must be kept visually distinct:

  - Success green must be distinct from brand green (so "this succeeded" never
  blurs into "this is the brand").

  - Warning must be distinct from pending-yellow, or folded into the
  yellow-orange range deliberately rather than added as a clashing third warm
  tone.

  - Error red, clearly its own thing, never auto-dismissed (see 1.2).

### 4.2 Pond identity is not an accent

Ponds are distinguished by light-weight identity signals, not by owning a system
accent color: a pond glyph or logo in the panel header, the pond's name, and the
pond's own true brand color appearing only in the space the pond owns (honest
attribution). The shell's semantic accents and the ponds' own brand colors never
fight, because they do different jobs in different places: the shell speaks
meaning-color everywhere; a pond shows its true colors in its own territory.

### 4.3 Color never carries meaning alone

Restated here because it is load-bearing: every semantic color is paired with a
non-color signal. A notification is orange-terracotta and has a shape/count/
position; pending is yellow and has a spinner/label. (Accessibility, and a
green-plus-warm palette makes red-green deficiency especially relevant.)

### 4.4 Typography intent

Type is a system (a named scale with bound line-heights), not per-view decisions.
Font size and warmth are first-class, deliberate choices (the project cares about
readable, comfortable type). Respect system scaling (4.1.5 / 1.5). The bar is
warm and legible over clever.

---

## 5. How to use this document

- When proposing a new pond or pad, check it against the shared criteria (section
1) and its unit-specific criteria (section 2 or 3). A unit that cannot meet the
shared criteria does not belong in the garden, however good it is in isolation.

- The garden test (philosophy 1a) still gates whether a unit should exist at all
(does it add optional leverage a user can adopt or ignore). This document gates
how it must behave and look once it does.

- The visual system (section 4) is the standard the design system implements. The
specific hex values, shade ramps, and type scale are defined in the design
system's tokens, not here; this states the roles and intent they must satisfy.

---

*This document grows with the design system. When a criterion is learned (often
the hard way, from a unit that got something wrong), it belongs here so the next
unit inherits the lesson.*
