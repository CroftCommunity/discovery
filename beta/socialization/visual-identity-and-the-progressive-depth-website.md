# Visual identity and the progressive-depth website

date: 2026-07-10

status / register: working design-system chapter. The tectonic palette, the typographic pairing, the
drystone visual language, and the progressive-depth information architecture are settled design direction;
the exact hex values are a working palette (design intent from a design board, not a locked brand standard).
This is the *how-it-looks* and *how-it's-structured* layer, sibling to `brand-and-voice.md` (how it sounds);
it does not re-derive the motto or voice.

## Overview

This chapter holds the visual design system and the website's information architecture: the palette, the
type, the drystone visual language, the craft philosophy that keeps the surface from reading as either
corporate-bland or amateurish, and the progressive-depth structure that lets a newcomer take the whole
story in three sentences or dig to bedrock. It is deliberately scoped away from two neighbours: the motto,
taglines, and voice live in `brand-and-voice.md` (referenced, not repeated — the "grow your own ___" funnel
and the two-speeds voice are settled there), and the product shape, the plot surface, and the logo marks
live elsewhere (the croft layer for the plot; the logo docs in this layer for the marks). This is the look
and the page structure only.

The governing intent, stated by the user, frames every choice: the site should be *easy but not clunky,
unassuming, genuinely welcoming, and easy to praise* — and above all it must communicate that this is **not
seeking a profit motive or value transfer**, but joining forces. The design answer is craft: make it feel
like a well-made physical object.

## The Shaker discipline (the craft stance)

The mental model is Shaker furniture — utilitarian, elegant, unadorned, highly polished. Shakers did not
build to be "mindful"; they built to be *useful*, and the beauty followed from the utility. That stance is
the shield against the two failure modes the design must avoid:

- **Corporate blandness** — the templated, could-be-any-startup look that says nothing.
- **Amateurish open-source** — the "feels amateurish" look that undermines trust, even though clunky
  open-source is sometimes the very best. The antidote is not decoration; it is *precision*: strict
  alignment, a rigid grid, generous negative space, and typography doing ~90% of the visual work. The site
  looks professional because it is precise, and it looks kind because it does not treat the visitor as a
  product. Precision is how a non-commercial project earns respect without hype.

Concrete rules that fall out of this: generous whitespace and wide section spacing (crowding reads as
amateur; room reads as confidence); no gamification, engagement hooks, pop-ups, or tracking notices (the
site should not track anyone, and should invite attention rather than demand it); clean page updates over
flashy animation.

## The tectonic palette (warmth without the co-opted earth tones)

The requirement was warmth — but *not* the standard beige-and-sage "earth tone" scheme, which has been
co-opted by lifestyle blogs and DTC ceramic brands and now reads faux-homey. The answer is a **tectonic**
palette: the warmth of a stone wall that has sat in the sun, not the softness of a wellness app. Lean into
rock colours and patterns — homey, not faux-homey.

Three roles: a granite/basalt **base** (structure), a ruddy-iron **fire** (the warmth driver — oxidised
iron / burnt terracotta / rust-lichen, not aggressive tech-orange), and a moss **life** accent (the spark
between the stones — highlights, active states, "growth", used sparingly).

Working palette (from the design board; hex are design intent, not a locked standard):

| Token | Hex | Role |
|---|---|---|
| Deep Schist | `#2F3539` | dark charcoal/granite — background/structure |
| Light Granite | `#A2A9AB` | muted grey — surfaces, subtle text |
| Ruddy Orange | `#B75C34` | terracotta/rust — accents, headings, primary actions |
| Dark Moss | `#3D6546` | earthy green — highlights, growth, active states |
| Iron Ore Black | `#1C1E20` | deep black — primary text (gentler than pure black) |
| Oatmeal Canvas | `#E9E1D6` | warm cream — background variance (not blinding white) |

## Typography and the drystone visual language

- **Type pairing.** A sturdy literary serif for headings (Lora, in the working board; Merriweather or
  Newsreader are register-mates) paired with a clean, highly legible sans for body and UI (Inter). It
  should read like a well-loved textbook or a classic essay collection — this pairing is a large part of
  how the surface beats the "amateur" look.
- **Drystone patterns without looking 1997.** Abstract, crisp, geometric — masonry blueprints, not
  photorealistic stone. Three devices: an interlocking drystone vector line pattern in place of thin CSS
  borders between sections; geological strata (heavy solid blocks of granite-grey and ruddy-orange stacked
  with zero margin, reading as supporting layers of earth); and a fine stipple/grain over flat CSS shapes so
  they feel like chiselled slate — tactile, but crisp and modern.
- **Line-art graphics, not icons.** Clean elegant line art — a drystone wall, a spade, a plot boundary —
  rather than cartoonish illustration or generic tech icons.

## The progressive-depth website (elevator → one-pager → library)

The information architecture is the direct answer to the user's concern that there is a lot to communicate
but no one owes you a long diatribe. Every major concept is offered at three depths, and the reader chooses
how far to dig (this is the same follow-or-ignore discipline the voice practices in `brand-and-voice.md`,
applied to page structure):

1. **The Surface — the elevator pitch.** A calm, 2–3 sentence typeset summary per concept. Not marketing
   hype: a plain statement of fact. The load-bearing constraint: *each elevator pitch should tell the whole
   story you need* — if it cannot, boil it down harder. The pitch is complete on its own; the deeper layers
   are optional, never required to make sense of it.
2. **The Soil — the one-pager.** A subtle, kind invitation to expand, worded in the world rather than in UI
   ("Understand the landscape" / "Why we build this way", never "Read More"), opening a single beautifully
   formatted page of philosophy.
3. **The Bedrock — the library.** The deep ideology, the protocol specifications, the historical context —
   kept off the homepage so the front door stays light and welcoming, while proving the project has real
   substantive weight for anyone who wants it.

Homepage blueprint (the calm, scannable flow — like walking up to a well-kept cottage):

```
+-------------------------------------------------------------+
|  [Logo]  Croft                              [Library] [Join] |
+-------------------------------------------------------------+
|                 GROW YOUR OWN.                              |
|   A quiet, personal space on the modern web — a plot of     |
|   your own, built to last. No profit motive.                |
|   [Explore the Protocol]        [See a Croft in Action]     |
+-------------------------------------------------------------+
|  The Plot            The Wall                The Valley      |
|  It belongs to you.  Built on Drystone —     Joined by       |
|  You tend it.        no mortar, alignment.   cooperation.    |
|  (each: a quiet "Understand the landscape" → the one-pager)  |
+-------------------------------------------------------------+
|  "A collaborative effort to reclaim a kinder, more human    |
|   web. You are entirely welcome here."                       |
+-------------------------------------------------------------+
```

Onboarding follows the same calm register — like filling a clean ledger, not a spinning wizard: the
identity step (the Bluesky handle as "the mailbox at the edge of your property"), the plot-location step
(two unapologetically clear choices: hosted-for-you, or bring-your-own-server / the Drystone blueprints),
then one text box — "write your first log." Calls to action read as an open gate, not high-pressure sign-up:
"Start your plot", "Read the drystone blueprints", "Join the valley".

## What this establishes (and does not)

This establishes the visual design system (the Shaker craft stance; the tectonic granite/ruddy/moss palette
with working hex values; the Lora + Inter pairing; the drystone visual language) and the website's
progressive-depth information architecture (the elevator-pitch / one-pager / library depth model, the
homepage blueprint, and the calm onboarding register). It does **not** own the motto, taglines, or voice
(those are `brand-and-voice.md`, and the "grow your own ___" funnel is settled there), the logo marks (those
are the logo docs in this layer), or the product shape and the personal-plot surface (that is the croft
layer). It does not lock the palette to final brand-standard hex values.
