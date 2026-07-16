# Visual identity and the progressive-depth website

date: 2026-07-10 · updated 2026-07-16 (the depth model generalized from three tiers to five; the
one-liner candidate table added, under review)

status / register: working design-system chapter. The tectonic palette, the typographic pairing, the
drystone visual language, and the progressive-depth information architecture are settled design direction;
the exact hex values are a working palette (design intent from a design board, not a locked brand standard).
This is the *how-it-looks* and *how-it's-structured* layer, sibling to `brand-and-voice.md` (how it sounds);
it does not re-derive the motto or voice. The five-tier gradient model below is settled as a *structure*;
the tier-1 one-liner itself is **under review** (see the candidates section) and nothing downstream of it
updates until the owner selects.

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

## The gradients: five tiers of one truth

The progressive-depth model below began as three depths (elevator → one-pager → library). Generalized, it
is a five-tier gradient, shallowest to deepest: **one-liner → elevator → over tea → classroom → library**.
The original model's load-bearing constraint — *each telling must tell the whole story you need* — is
preserved and generalized as the invariant that governs every tier:

> **The invariant.** The tiers differ in **order** (what comes first), **energy** (how it is said), and
> **altitude** (how much mechanism is visible) — **never in truth.** A shallower tier may narrow the truth
> arbitrarily; it may never bend it. Nothing said at any tier is retracted at a deeper one; the deeper
> tiers only add.

Each tier has a character: the question it answers, the perspective it speaks from, its energy, its depth,
and the specific ways it fails.

### 1. The one-liner — the inscription

- **Question answered:** "Should I turn my head?"
- **Perspective:** the stranger in motion, zero granted seconds. They did not ask; they owe nothing.
- **Energy:** still, carved. One breath. A sentence you could cut into a lintel.
- **Depth:** zero mechanism, pure identity. It names what kind of thing this is, not how it works.
- **The special burden:** this is the only tier that travels *without its author* — it will be repeated
  secondhand by people who do not understand it, on podcasts, in group chats, in a skeptic's mouth. So it
  must survive repetition by a non-understander: it may narrow the truth arbitrarily, never bend it.
- **Failure modes:** the **lie of compression** (bending the truth to fit the breath); the **empty
  vessel** (a line so general it is true of anything — and therefore says nothing); **jargon** (true but
  dead — the stranger's head never turns).
- **Test battery:** the **secondhand test** (repeat it through a non-understander; does it still say
  something true?), the **hostile reading** (read it as a skeptic trying to make it overclaim; does it
  survive?), the **library check** (walk it down to Part 1/Part 2 and the evidence map; is it still
  exactly true?). The candidates under this battery are in the one-liner section below.

### 2. The elevator — the witness

- **Question answered:** "What is this?"
- **Perspective:** the witness. It states what is; it does not argue, sell, or explain how.
- **Energy:** calm, plain, declarative. 2–3 sentences of whole-truth fact.
- **Depth:** what it is and what changes for you — no mechanism, no hype.
- **Failure modes:** hype (a witness does not sell); mechanism creep (the "how" belongs two tiers down);
  the partial truth that needs a correction later (the invariant forbids it).
- **Existing residents:** `drystone-elevator-pitch.md` (the protocol telling, plain-spoken + technical)
  and `sixty-second-pitch.md` (the relational-field reframe). The one-liner register that
  `sixty-second-pitch.md` carries under `## The one-liner` is hereby **promoted to tier 1** — it is a
  one-liner, not an elevator pitch, and it competes under the tier-1 test battery. Cross-reference it
  there; do not duplicate it here.

### 3. Over tea — the friend

- **Question answered:** "What would this be like *for me*?"
- **Perspective:** the friend across the table. The subject is the listener's own social life — their
  group chat, their club, their family thread — never the protocol in the abstract.
- **Energy:** warm, unhurried, dialogic. It waits for the "huh, so what happens when…" and answers it.
- **Depth:** one sustained metaphor per concept, with the seam marked — the point where the metaphor ends
  and the mechanism differs is said out loud, so the metaphor never quietly overclaims.
- **Failure modes:** metaphor drift (the metaphor starts making claims the mechanism doesn't back);
  unmarked seams; condescension (the friend explains *with* you, not *at* you).
- **Existing residents:** `coffee-shop-telling.md`, and the site's one-pagers (the Soil, below).

### 4. The classroom — the guide

- **Question answered:** "How does it work — and why *must* it work that way?"
- **Perspective:** the guide. Patient and cumulative: each chapter needs only what came before.
- **Energy:** learner order — **need before mechanism**. The story breaks first; the mechanism arrives as
  the only honest repair. Nothing is introduced before the learner has felt why it has to exist.
- **Depth:** full mechanism truth at library precision, told in narrative order rather than normative
  order — and every claim ends in something you can run (a named test, an evidence-map row, a command).
  The open problems are taught by name, never papered over.
- **What it is not:** a different *axis* from the library, not a shallower copy of it — a **path**, not a
  reference. The classroom and the library hold the same truth; one walks you, one waits for you.
- **Failure modes:** lecture drift (mechanism before need); hand-waving at the hard parts (the classroom
  shows the ladder, including the rungs not yet climbed); claims that end in prose instead of something
  runnable.
- **Existing residents:** the classroom arc and chapter scaffolds under `alpha/classroom/` (bodies
  drafted in conversation, not by runs).

### 5. The library — the reviewer

- **Question answered:** "Is it true?"
- **Perspective:** the reviewer. The reader drives; the text withstands.
- **Energy:** normative order, the evidence ladder, primary sources. No narrative sequencing — sections
  stand where the design puts them, and every status tag is earned or says it isn't.
- **Depth:** everything, with its provenance.
- **Failure modes:** narrative leaking into normative order; an unearned tag (the A.9 ladder and the
  evidence map exist so this fails loudly).
- **Existing residents:** this tier already exists — Part 1 and Part 2, the published spec site, and
  `beta/drystone-spec/EVIDENCE-MAP.md`.

### The loop principle

The tagline and the classroom's closing refrain are designed as **siblings**: the shallowest and the
deepest tiers touch. The classroom's refrain — *"And underneath, nothing changed: it is still two people
keeping their own memory of what was said, signing it, and pointing at each other's words."*
(`alpha/classroom/00-arc.md`) — is the ten-chapter, fully-mechanized form of the same sentence the
one-liner carves for the stranger. Walk the gradient down and the last thing the classroom says is the
first thing the site said. The one-liner is the sentence the classroom **earns**.

## The one-liner: candidates under test

`FOR REVIEW — owner selects; the elevator doc's one-liner register (sixty-second-pitch.md,
promoted to tier 1 above) updates only after selection. This run declares no winner.`

The tier-1 test battery, applied to every candidate:

1. **The secondhand test.** Repeat the line through someone who does not understand the project — a
   podcast host, a friend-of-a-friend, a commenter paraphrasing from memory. Does what survives still say
   something true?
2. **The hostile reading.** Read it as a skeptic trying to make it overclaim ("oh, so it promises X?").
   Does the line survive without needing a defender in the room?
3. **The library check.** Walk the line down the gradient to Part 1/Part 2 and the evidence map. Is it
   still exactly true at full precision — narrowed, never bent?

| candidate | source truth it compresses | secondhand survival | hostile reading | library check |
|---|---|---|---|---|
| "It proves what was said. Never who was right." | the razor — the substrate computes provenance, never utility; fork-not-verdict (§7.6: the harshest act is a fork, never a verdict) | strong: the two-beat shape survives paraphrase ("it proves what was said, not who was right"); weakness — "it" needs an antecedent the stranger doesn't have | invites "so it settles nothing?" — the exact objection the library answers (deterministic tiebreak or hard-stop to human adjudication); it cannot be stretched into a promise it doesn't make | true at full precision: §7.6 hard-stop, R7 content-bound approval; no bend |
| "Groups no one owns." | center-freedom — no node holds canonical or privileged authority over shared state (Part 1; peer symmetry) | strong: five words, hard to garble | risks "unowned = ungoverned/anarchy"; the library answers (governance is real, k-of-n, §7.2) but the line alone doesn't; empty-vessel risk — many decentralization projects could claim it | true: no owner exists at the substrate and groups govern themselves; narrowed (says nothing of memory or provenance), not bent |
| "Everyone keeps their own memory. The group still agrees." | canonical local state + convergence — byte-identical folds across arrival orders (§7.3; the convergence/dedup tests); the refrain's sibling | strong: two plain sentences that repeat cleanly | "agrees — even when they disagree?" — honest answer: it agrees on *what was said* (the record), never on who was right; a hostile reader can stretch "agrees" past the record, so the narrowing must hold the line | true narrowly and exactly: convergence is about the record; genuine disagreement is a recorded fact that hard-stops (§7.6). Tightest coupling to the classroom refrain (the loop principle) |
| "Memory without a master." | center-freedom, mood-forward — no privileged node over shared memory | very strong: three words, alliterative, sticky | "master" carries loaded framing some readers will bristle at; invites "so nobody maintains it?" (answer: everyone does); highest empty-vessel risk — mood over mechanism, claimable by any P2P store | true: no canonical node (Part 1); but it names no group, no governance, no provenance — the narrowest candidate |
| "Disagreement is a fact, not a failure." | fork-not-verdict — contradiction is a first-class recorded fact; hard-stop over auto-merge (§7.6) | strong: aphorism shape holds through repetition | "so it never resolves anything?" — the library answers (party-neutral tiebreaks resolve; social disputes escalate to humans, §7.6.1); out of context it reads as a poster platitude — empty-vessel risk without the product noun | true: §7.6 treats two honest quorums as a fact to show, never a race to win |

Assessment cells are this run's reads under the battery, recorded to make the review concrete — they are
inputs to the owner's selection, not a ranking. The refrain-coupled candidate (row 3) is the loop
principle's natural sibling; that is a design observation, not a verdict.

## The progressive-depth website (the gradient applied to the page structure)

The information architecture is the direct answer to the user's concern that there is a lot to communicate
but no one owes you a long diatribe. Every major concept is offered at progressive depths, and the reader
chooses how far to dig (this is the same follow-or-ignore discipline the voice practices in
`brand-and-voice.md`, applied to page structure). The homepage carries three of the five tiers on-page —
the elevator, the one-pager (the over-tea tier in written form), and the library door — with the one-liner
above them as the tagline and the classroom as a named path beside them:

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

(In the five-tier gradient above: the Surface is tier 2, the Soil is tier 3 in written form, the Bedrock
is tier 5. Tier 1 — the one-liner — sits above the Surface as the tagline in the hero; tier 4 — the
classroom — is a named path beside the Bedrock, "walk me through it" next to "let me verify it".)

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
with working hex values; the Lora + Inter pairing; the drystone visual language), the five-tier gradient
model with its invariant and per-tier characters (one-liner → elevator → over tea → classroom → library;
the tiers differ in order, energy, and altitude — never in truth), the loop principle coupling the tagline
to the classroom refrain, and the website's progressive-depth information architecture (the depth model on
the page, the homepage blueprint, and the calm onboarding register). The tier-1 one-liner itself is **not**
established — the candidate table above is under review and the owner selects. It does **not** own the
motto, taglines, or voice (those are `brand-and-voice.md`, and the "grow your own ___" funnel is settled
there), the logo marks (those are the logo docs in this layer), or the product shape and the personal-plot
surface (that is the croft layer). It does not lock the palette to final brand-standard hex values.
