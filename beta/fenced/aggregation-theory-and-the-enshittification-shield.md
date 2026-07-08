# Aggregation Theory and the enshittification shield: the economics of Croft's open-protocol bet

`Status: fenced layer. Register: market-economics analysis — the structural economic logic of the centered
field, distinct from the descriptive map (which measures the field) and from the normative harm case (which
indicts it). Resolution: library — the spine of Croft's market thesis. External economic concepts are cited
by name (Ben Thompson / Aggregation Theory; Cory Doctorow / enshittification); the one verbatim quote and the
dialogue-coined framings carry inline verification flags and need a refresh pass against primary sources
before external use.`

## Overview

The centered commercial field's dominance is not only a set of onboarding and network-effect mechanics; it
rests on an economic logic that can be named precisely, and that logic is what an open protocol changes. This
document holds that logic. It applies **Aggregation Theory** (the economics of demand aggregation) and
**enshittification** (the lifecycle of value extraction) to the single question that motivates Croft's
open-protocol bet: what happens to a platform's incentive to extract when the switching cost that anchors its
users falls toward zero. The load-bearing conclusion is that a zero-switching-cost open protocol is
*structurally enshittification-resistant* in a way a centered platform cannot be — extraction triggers exodus
rather than lock-in, so the extractive move stops paying. That conclusion is the fenced-layer counterpart to
the activism harm case: the harm layer argues that extraction is *wrong*; this layer argues why, on an open
protocol, it also stops *working*.

## Charter: what this document covers

- **In scope:** the economic logic of the centered field — Aggregation Theory as the demand-side account of
  how the incumbents win, enshittification as the lifecycle of extraction, and the switching-cost hinge that
  couples the two; and the structural inversion an open protocol performs on that hinge.
- **Out of scope (and where it lives):** the *measured* switching-cost and lock-in mechanics — captured social
  graph, non-portable identity, un-owned data, and the field data points (for example the migration that
  stayed on the incumbent against the maintainer's own preference) — are the descriptive map in
  `platform-dominance-and-adoption.md`; this document cites that map rather than reproducing it. The go-to-
  market response (what a challenger does about the adoption chasm) is `../socialization/`. The harm reading
  (extraction as indictment, captured community labor) is `../activism/`.
- **Boundary call:** this is the "why the economics invert under an open protocol" register — an analysis, not
  a measurement. `platform-dominance-and-adoption.md` maps *that* switching costs hold users; this document
  argues *what follows* economically when a protocol drops them to near zero. The two are complements: the map
  supplies the mechanic, this doc supplies the consequence.

## Aggregation Theory: the demand-side account

Aggregation Theory, coined by **Ben Thompson** on Stratechery (2015), is the economic explanation for how the
centered platforms come to dominate. Its core move is a shift in what winning controls. Before the internet,
the winners of an industry controlled **supply** — the oil, the printing presses, the broadcast towers — and
distribution was scarce and expensive. Online, once distribution, marginal, and transaction costs fall toward
zero, control of supply stops being the lever. The winners instead control **demand**, by owning the best
direct user experience, and suppliers are then forced to commoditize themselves onto the aggregator's terms.

> Zero distribution costs. Zero marginal costs. Zero transactions. This is what the Internet enables, and it
> is completely transforming not just technology companies but companies in every single industry.
>
> — Ben Thompson, *Aggregation Theory* (Stratechery, 2015)

`[dialogue-sourced quote; confirm exact wording against Thompson's primary Stratechery text before external
use — attribute to Aggregation Theory as a body of work, not to a single dated essay.]`

The flywheel Thompson describes is self-reinforcing: superior user experience draws a mass of users, and the
mass of users forces suppliers to modularize and compete on the aggregator's platform, which improves the
experience further. The follow-up ("Defining Aggregators," 2017) grades aggregators by how much of the supply
cost they still bear — a supply-acquiring level (a service that pays for its supply), a transaction-cost level
(a marketplace that does not own supply but carries onboarding friction), and a zero-supply-cost level (a
platform whose suppliers self-optimize onto it at near-infinite margin). The endgame Thompson names is
monopoly, and he names only two counters to it: a genuinely new interface paradigm, or **decentralized
protocols that drop switching costs to zero** so users can move their demand freely. The second counter is the
one Croft's bet rides on. `[dialogue-sourced framing of the levels and counters; confirm against Thompson's
primary essays before external use.]`

## The switching-cost hinge

Aggregation Theory explains how demand is captured; **enshittification** — Cory Doctorow's account of the
platform lifecycle — explains what the platform then does with the captured demand: it is first good to its
users, then shifts value toward its business customers, then claws value back for itself, then declines. The
hinge that couples the two theories, and the reason the extractive phase is survivable for the platform, is
**high switching cost**. Extraction only pays while users cannot leave. The mechanics that raise that cost on
a centered platform — the captured social graph, the non-portable identity, the data that is not the user's to
take with them — are measured in the sibling map `platform-dominance-and-adoption.md`, which also records the
capital-structure driver (growth capital needs an exit, an exit needs an extraction story) that makes the
extractive turn near-inevitable for a venture-funded centered platform. This document does not restate that
map; it takes the switching cost as the given input and reasons about what changes when the input goes to
zero.

## The enshittification shield: what an open protocol inverts

On an open protocol, identity, history, and the social graph are portable by construction: a user who is not
served well can move to another provider and carry their followers and records with them. When switching cost
falls toward zero, the enshittification hinge breaks. The extractive move that pays on a centered platform —
degrade the user's experience to capture more value, confident the captured graph will hold them — instead
triggers a fast exodus, because nothing holds the user once leaving is cheap. The incentive inverts: the
rational move for an open-protocol operator is to *keep* serving users well, because the moment it stops, the
users are gone. One dialogue framed this as the operator being *trapped by the math* — the extractive lever
that funds the centered lifecycle simply does not exist when there is no hostage to extract against.
`[dialogue-coined framing; confirm before external use.]`

This is the *enshittification shield*: not a policy, a licence, or a promise of good behavior, but a
structural property of a zero-switching-cost protocol. A centered platform can promise not to enshittify and
later break the promise, because the capital-structure pressure toward extraction is installed by its
structure and its lock-in makes breaking the promise survivable. An open protocol cannot enshittify
profitably in the first place, because the precondition for profitable extraction — the user who cannot leave
— has been removed. The resistance is structural, not aspirational, which is precisely why it can be carried
as a load-bearing claim rather than a hope.

## Where the value bet moves

If users cannot be captured, the economic question becomes why capital would fund an open protocol at all. The
answer reframes the bet from capturing users to owning the infrastructure layer of a new standard — what one
dialogue called becoming *the Red Hat of social infra*: the protocol itself is free and open, but operating
the heavy pieces (indexing, moderation tooling, the read-and-write infrastructure) is complex enough that a
best-in-class operator can be the enterprise-grade provider of them. `[dialogue-coined framing; confirm before
external use.]` Two adjacent theses ride alongside it: a tollbooth or on-ramp position (the flagship
application is the convenient front door, monetized on convenience — domains, premium storage, a cut of
tips — rather than on lock-in), and Aggregation Theory turned to the operator's advantage (owning the largest
unified directory of demand so that developers plug into that audience by default). The through-line is that
the value bet has moved *off* the captured user and *onto* the infrastructure and the on-ramp, which is the
only place value can accrue once switching cost is gone.

## The load-bearing conclusion for Croft

A zero-switching-cost open protocol is structurally enshittification-resistant in a way a centered platform
cannot be. This is the spine of Croft's market thesis. The reason it must travel with the conclusion — the
anti-rollup point — is that the conclusion is worthless without its mechanism: it is not that open protocols
are morally better and therefore will not extract, it is that high switching cost is the *precondition* for
profitable extraction, and an open protocol removes that precondition, so the extractive incentive inverts
from "degrade and hold" to "serve or lose them." Drop the mechanism and the claim decays into a values
slogan that a skeptic can dismiss; keep the mechanism and the claim is an economic argument that stands on the
same footing as Aggregation Theory itself. This is the fenced-layer counterpart to the activism harm case:
where activism argues that platform extraction is a harm to be resisted, this layer argues that on an open
protocol the same extraction is an economic dead end — the harm case says extraction is *wrong*, the market
case says it stops *working*.

## What this establishes (and does not)

Establishes the economic spine of Croft's open-protocol bet: that Aggregation Theory explains how the centered
platforms capture demand; that enshittification explains what they do with it; that high switching cost is the
hinge coupling the two and the precondition for profitable extraction; that an open protocol drops that
switching cost toward zero and thereby inverts the extractive incentive (extraction triggers exodus, not
lock-in), which is the *enshittification shield*; and that the value bet consequently moves off the captured
user and onto the infrastructure and on-ramp layers.

Does **not** measure the switching-cost mechanics or reproduce the field data points behind them — those are
the descriptive map in `platform-dominance-and-adoption.md`, which this document cites and complements rather
than duplicates. Does **not** argue the harm case (that extraction is an indictment and community labor a
captured value) — that is `../activism/`. Does **not** prescribe the go-to-market response to the adoption
chasm — that is `../socialization/`. Does **not** certify the one verbatim Thompson quote or the dialogue-
coined framings (*trapped by the math*, *the Red Hat of social infra*); those carry inline flags and need a
refresh pass against primary sources before external use.
