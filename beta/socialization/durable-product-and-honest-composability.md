# Durable product and honest composability: the model beneath the stance

`Status: socialization layer (Layer 8, how this reaches people). Register: product principle /
anti-extraction argument. Resolution: supplies the audience model and the stability discipline that
ground the composability stance carried in brand-and-voice.md and adoption-strategy.md. The
three-audiences model is treated as a settled design principle; the LTS-for-interfaces mechanism is a
settled ethos whose channel/cadence *commitment* (versus aspiration) is still open. Dialogue-sourced
lines carry verification flags.`

## Overview

The socialization layer already asserts that composability is a way of respecting the user — that a
surface you can shape to your own use is worth more than one shaped for you. `brand-and-voice.md`
carries that stance in its taglines (a rotating invitation that "demonstrates composability rather
than describing it") and its pillars ("you own none of this elsewhere; here you do"). What that stance
has been missing is the model underneath it: *why* shapeability respects the user, *who* the shaping
is actually for, and *what* keeps composability from curdling into the very churn it claims to reject.

This document supplies that model. It rests on two principles that travel together. First,
**shapeability is only valuable paired with stability** — a surface you can reshape freely, on a
foundation that never sits still, is not a gift; constant interface change is quietly extractive. The
discipline that pairs them is *LTS-for-interfaces*. Second, **settings serve three audiences, named by
their relationship to the system, not by depth** — and naming them correctly is what makes "shape it to
your use" a real offer rather than a slogan. Both principles land on the same non-extraction thesis
the layer is built around: a durable, stable, composable surface is precisely the thing a
growth-capital competitor is structurally disinclined to ship, because its business needs the churn.

## Charter: what this document covers

- **In scope:** the audience model beneath the composability stance (the three settings audiences), the
  stability discipline that makes shapeability honest (LTS-for-interfaces), and the tie from both to the
  non-extraction thesis.
- **Out of scope (and where it lives):** the brand voice and the tagline mechanics that *express* the
  composability stance (`brand-and-voice.md`); the go-to-market argument for reaching users at all
  (`adoption-strategy.md`); the substrate and protocol mechanics that make a shapeable surface
  buildable (the croft and drystone-spec layers).
- **Boundary call:** this is the "why the composability stance is user-respecting, and who it serves"
  register. It states the principle and carries its reasoning; it does not specify the per-platform
  settings UI or commit the LTS channel/cadence as a product promise. Those are downstream product work.

## Shapeability is only valuable paired with stability

The composability stance has an unstated dependency: it only respects the user if the ground beneath
the shaping holds still. A surface you can reshape endlessly, sitting on a foundation that itself
changes every release, does not accumulate into mastery — it resets. The originating design dialogue
put the failure mode plainly:

> Shapeability is only valuable paired with stability; constant UI change is quietly extractive.

> When interfaces change constantly, people never build a durable mental model, and "change it back"
> friction becomes an engagement lever.

`[dialogue-sourced 2026-06-15; verify wording before external use.]`

That is the load-bearing insight, and it is an anti-extraction argument, not a usability nicety. When
an interface never settles, the user can never finish learning it; the effort of re-learning, and the
friction of trying to put it back the way it was, is not an accident of shipping fast. It is an
engagement lever. Churn is a feature for a business whose revenue tracks time-on-surface.

The discipline that pairs shapeability with stability is *LTS-for-interfaces* — long-term support
applied to the surface people learn, not just to the code beneath it:

- **Channels, borrowed from software release trains:** alpha, beta, and stable. The stable channel is
  the default and the promise.
- **A long stable window — on the order of three years —** during which the *learned surface* (layout,
  names, where things live) is held still on purpose. Improvements ship *behind* that surface; the
  places a user's hands have memorized do not move.
- **Change is opt-in and paced.** New interface generations "train" on a slow cadence (roughly every
  six months) for the people who want them, and are not pushed onto everyone who does not.
- **Security is the named exception.** Changes required for safety ship regardless, and are
  over-communicated rather than slipped in.

The reason this principle must carry its own honest cost — the anti-rollup rule — is that the cost is
what turns it from a slogan into a commitment. Holding several interface generations live at once means
carrying real documentation and support weight. The dialogue named the consequence of dropping that
part:

> Honest cost: multiple live interface generations to support — name it and budget it, or the
> principle dies in year two.

`[dialogue-sourced 2026-06-15; verify wording before external use.]`

A stability promise whose cost is not budgeted is one that quietly lapses the first time shipping speed
is under pressure. So the principle travels with its price attached: to claim LTS-for-interfaces is to
claim the support budget that makes it real. Whether Croft commits to a specific channel and cadence
model, versus holding it as an aspiration, is the open part of this — the ethos is settled, the
concrete commitment is not.

## The three settings audiences

The three-audiences model is the audience layer the composability stance was asserting without. It
corrects a standard mistake: the basic/advanced settings split, which sorts controls by *depth* and in
doing so conflates two different things — "most people never touch this" and "this is simple." Depth
names collapse under their own imprecision, because a setting can be simple and heavily used, or complex
and rarely touched, and the basic/advanced axis cannot tell those apart.

The better cut is by the user's *relationship to the system*, not by how deep a control sits. There are
three audiences, and they are named by intent:

- **Never-touch.** People who change nothing. For them, the defaults *are* the product. Everything the
  composability stance promises is delivered, for this audience, by choosing defaults well — not by
  exposing a knob.
- **Tune-a-few.** People who will find and adjust one or two things that matter to them, and no more.
  They are served by a short, curated, genuinely findable list — not the full surface, and not a token
  gesture.
- **Full-surface.** People who want everything exposed. These are the early adopters and the fiercest
  defenders of the product, and the full, unfiltered surface is what earns their loyalty.

Naming by intent rather than depth is the whole point: it keeps the offer honest across all three.
Composability respects the never-touch user through defaults, the tune-a-few user through a curated
short list, and the full-surface user through the complete surface — three different deliveries of the
same respect. A design that only recognizes two audiences (the basic/advanced split) inevitably fails
one of them, usually by handing the tune-a-few user either too little or the full firehose.

`[three-audiences framing dialogue-sourced 2026-06-15; treated as a settled design principle, but
verify wording before external use.]`

## Why a growth-capital competitor cannot follow here

Both principles converge on the non-extraction thesis that organizes this layer, and the convergence is
the strategically interesting part. A surface that is durable (held stable for years), and composable
(shaped to the user across all three audiences), is not merely a nicer product than the incumbents' —
it is a product they are *structurally disinclined to build*.

The reason is mechanical, not moral. A business whose revenue tracks engagement needs the surface to
keep moving: change drives re-learning, re-learning drives time-on-surface, and "change it back"
friction is a lever it has every incentive to pull. Stability, on that model, is lost revenue. So the
growth-capital competitor cannot commit to a three-year stable window or an opt-in change cadence
without working against its own engine. This is the same asymmetry `adoption-strategy.md` leans on when
it argues that a non-extractive sustaining org is part of the product rather than a back-office detail:
the sustaining model determines what the product is even *allowed* to be. A durable, honestly
composable surface is a capability that follows from not needing the churn — which is to say, it is one
of the concrete things a member-aligned org can ship that a growth-capital one cannot copy without
changing what it is.

That is why this belongs in socialization and not only in the product layer. It is a differentiator you
can say out loud: not "our settings are nicer," but "we can afford to leave your surface alone, and they
can't."

## What this establishes (and does not)

Establishes the model beneath the composability stance the socialization layer already carries: that
shapeability is only user-respecting when paired with stability (the *LTS-for-interfaces* discipline —
channels, a multi-year stable window, opt-in paced change, the learned surface held still, security the
over-communicated exception, and the honest multi-generation support cost named alongside it); that
settings serve three audiences by relationship to the system (never-touch / tune-a-few / full-surface),
named by intent rather than depth so the offer stays honest for each; and that a durable, stable,
composable surface is something a growth-capital competitor is structurally disinclined to ship, which
ties both principles to the non-extraction thesis and makes them a differentiator, not just a
preference.

Does **not** re-argue the brand voice or the tagline mechanics that express composability (those live
in `brand-and-voice.md`), does **not** restate the adoption argument for reaching users
(`adoption-strategy.md`), and does **not** specify the per-platform settings UI or commit the
LTS channel and cadence as a product promise — the ethos is settled, but that commitment, and the
support budget it implies, remains open product work. The dialogue-sourced lines carry verification
flags and want a wording check before external use.
