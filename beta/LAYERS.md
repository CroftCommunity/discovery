# discovery / beta: the layer model

date: 2026-07-06

**What this is.** The canonical description of the beta layer-cake: the eight layers, what each holds, the
two ways the stack is traversed, and the register distinctions that keep adjacent layers from duplicating
each other. `README.md` carries the index table; this file carries the reasoning. When the model and a
layer `README` disagree, this file governs.

## The eight layers

```
Layer  Dir              Holds                                                       Register / role
──────────────────────────────────────────────────────────────────────────────────────────────────
1      history/         crofting, dry-stone stacking, cairns, the space itself      MATERIAL history
2      philosophy/      the principles + thinkers; the pure peer-standing argument  INTELLECTUAL history
3      drystone-spec/   the protocol (principles → certifiable spec)                the WHAT (protocol)
4      impl/            reference core: pragmatic, maintainable, cross-platform     the WHAT (built)
5      croft/           product + brand; Croft as one "flavor" on the neutral core  the WHAT (product)
6      governance/      foundation + cooperative: legal / financial actualization   the manifestation
7      socialization/   brand, voice, adoption: getting the message out             presentation
8      activism/        why not the status quo: current-state harm, uncompensated   the present "why"
                        community labor, data opacity
```

## Why this order

The stack is ordered **why-first**. The "why" is not built last even though, in wall-clock terms, the
evidence and framing often get written late, it is the *ground everything stands on*, so it sits at the
base:

- **Layers 1–2 are the "why-in-principle."** History (why the form *resonates*: it has material and
  cultural precedent, the croft, the commons, the drystone wall) and philosophy (why the form is *right*:
  peer standing, non-domination, the cooperative conclusion). The spec is built *on* these principles, so
  they precede it.
- **Layers 3–6 are the build.** Protocol (3) → reference implementation (4) → product/brand (5) →
  manifestation as a running institution (6). This is the order you actually construct things in: define
  the protocol from the principles, implement a maintainable core, wrap it as a product, then ground it in
  the world as a foundation-sponsored cooperative.
- **Layers 7–8 are the outward edges.** Socialization carries the message outward; activism is the
  present-tense "why not the status quo", the current-state indictment that motivates a newcomer.

## The "why" is three layers, split by tense

The single most useful distinction in the model. The "why" is not one bucket, it is three, separated by
tense:

- **history (Layer 1): why it resonates**, the *past made it precedented*. Material and cultural: the
  croft and the surviving common, dry-stone construction, cairns.
- **philosophy (Layer 2): why it is right**, the *principles make it right*. The intellectual lineage
  (Anderson on relational equality, Pettit on non-domination, List & Pettit on group agency, Ostrom on the
  commons, Beer on viability) and the pure peer-standing → cooperative-form argument.
- **activism (Layer 8): why not the status quo**, the *present makes it urgent*. The sourced,
  current-state harm case against incumbent platforms.

History and philosophy cross-link heavily but are **two narratives, deliberately kept separate**: "this
has cultural precedent" and "this is principled" are different arguments and read differently.

## Two histories, not one

`history/` is the *material* history (the thing and the space: crofting, dry-stone, cairns).
`philosophy/` is the *intellectual* history (the ideas: principles and thinkers). They will cross-reference
constantly, and that is expected, but they are not merged, because collapsing "the story of the croft" into
"the story of the principles" would flatten two distinct lineages into one and lose the thing each does
well.

## Philosophy argues; governance manifests

The peer-standing → cooperative-form argument is the **philosophical** claim that only a cooperative form
can constitute peer standing. That argument lives in **philosophy (Layer 2)**. **Governance (Layer 6)** is
its *manifestation*: the foundation that sponsors development and the cooperative that operates the thing,
the legal and financial actualization. Philosophy justifies the form; governance builds it. This is why the
peer-standing docs were moved out of governance into philosophy on 2026-07-06: governance is "how does this
ground in the world," not "why is this right."

Two entities live in governance, not one:
- a **foundation** sponsoring development of the *neutral* stack (protocol, implementation, Croft product);
- a **cooperative** as the operating body, the edge-free ownership form the argument requires.

That is also what "Croft is a flavor" means: the foundation stewards the neutral protocol and reference
implementation; Croft is one product built on top; the cooperative operates it.

## Two traversals

The layer *numbers* are a stack of concerns, not a schedule. Two different orders run over the same stack:

- **Justification order (bottom-up, = the numbering):** history → philosophy → spec → impl → croft →
  governance → socialization → activism. "What rests on what." The why grounds the spec; the spec grounds
  the build; the build grounds the manifestation and the outward edges.
- **Build order (where you actually start):** you *start at the spec* (Layer 3), "define and build the
  protocol everything else rests on," principles-first, then implementation → product → governance. The
  why-layers (1, 2, 8) are written alongside and often finalized late, but they are logically prior. Keeping
  these two orders distinct is what prevents the "the numbering doesn't match how I'd start" confusion: you
  build from 3, but the stack is grounded from 1.

## Register discipline (so adjacent layers don't duplicate)

Each layer states a shared concept in its **own register**, and no two compete to be the source of truth
for the same claim:

- The **peer-standing argument** appears as *principle* in philosophy, as *empirical harm* in activism, as
  *human-facing presentation* in socialization, and as *manifestation* in governance. One claim, four
  registers, one primary home each.
- The **spec** states persona as a *principle* (Part 1) and as a *mechanism* (Part 2); the vocabulary of
  record (`persona-definition.md`, Appendix D) is a *lattice* over the mechanism. Different altitudes, single
  source per claim.

## Current state (2026-07-06)

Seeded: `philosophy/` (peer-standing set), `governance/` (reserved; README only), `socialization/`
(essay + pitch), `activism/` (research set), `drystone-spec/` (populated). Not yet created: `history/`
(theme 02 is its seed), `impl/` (themes 04/05/06), `croft/` (theme 08). The `02`–`08` theme docs at beta
root remain the reading spine until their content migrates into layers.
