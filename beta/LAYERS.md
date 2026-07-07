# discovery / beta: the layer model

date: 2026-07-06

**What this is.** The canonical description of the beta layer-cake: the nine layers, what each holds, the
two ways the stack is traversed, and the register distinctions that keep adjacent layers from duplicating
each other. `README.md` carries the index table; this file carries the reasoning. When the model and a
layer `README` disagree, this file governs.

## The nine layers

```
Layer  Dir              Holds                                                       Register / role
──────────────────────────────────────────────────────────────────────────────────────────────────
1      history/         crofting, dry-stone stacking, cairns, the space itself      MATERIAL history
2      philosophy/      the principles + thinkers; the pure peer-standing argument  INTELLECTUAL history
3      cairn/           the field of existing bolstering tech we build on:          THE FIELD (what exists)
                        iroh, MLS, Willow/Meadowcap, CBOR-DAG, atproto/AT, AP,
                        CRDT, QUIC, p2p; products Roomy, Blacksky, p2panda,
                        SimpleX, Matrix. Some bubbles into the spec; some is
                        tracked for network-effect and homage.
4      drystone-spec/   the protocol (principles → certifiable spec)                the WHAT (protocol)
5      impl/            reference core: pragmatic, maintainable, cross-platform     the WHAT (built)
6      croft/           product + brand; Croft as one "flavor" on the neutral core  the WHAT (product)
7      governance/      foundation + cooperative: legal / financial actualization   the manifestation
8      socialization/   brand, voice, adoption: getting the message out             presentation
9      activism/        why not the status quo: current-state harm, uncompensated   the present "why"
                        community labor, data opacity
```

## Why this order

The stack is ordered **why-first, then field, then build, then edges**. The grounding is not built last even
though, in wall-clock terms, evidence and framing often get written late; it is the *ground everything
stands on*, so it sits at the base:

- **Layers 1–2 are the "why-in-principle."** History (why the form *resonates*: material and cultural
  precedent, the croft, the commons, the drystone wall) and philosophy (why the form is *right*: peer
  standing, non-domination, the cooperative conclusion).
- **Layer 3 is the field.** `cairn/` is the survey of the existing bolstering tech, the building blocks and
  products that already exist. It sits between the principles and the spec because **the spec had to survey
  the field first**, to know whether the ecosystem held the parts to assemble a safe amount of novelty
  practically (designing iroh, MLS, CBOR-DAG, or Willow from scratch would have been too heavy a blocker).
  The spec is built on the principles *and* on the surveyed field.
- **Layers 4–7 are the build.** Protocol (4) → reference implementation (5) → product/brand (6) →
  manifestation as a running institution (7). Define the protocol from the principles and the field,
  implement a maintainable core, wrap it as a product, then ground it in the world as a foundation-sponsored
  cooperative.
- **Layers 8–9 are the outward edges.** Socialization carries the message outward; activism is the
  present-tense "why not the status quo", the current-state indictment that motivates a newcomer.

**cairn is the inverse of activism.** Both survey the field; they differ in valence. Activism (Layer 9) is
the case *against* the incumbents, the extractive tech we refuse. Cairn (Layer 3) is the catalogue of what
we build *on*, the enabling tech we credit and reuse. Same activity, opposite sign. The name is deliberate:
a cairn is dry-stacked waymarker stones raised by many hands to mark a path for those who come after, which
is exactly what an index of this space is, and it shares the dry-stone construction family with `drystone`
itself (cairn catalogues the stones; drystone builds the wall).

## The "why" is three layers, split by tense

The single most useful distinction in the model. The "why" is not one bucket, it is three, separated by
tense:

- **history (Layer 1): why it resonates**, the *past made it precedented*. Material and cultural: the
  croft and the surviving common, dry-stone construction, cairns.
- **philosophy (Layer 2): why it is right**, the *principles make it right*. The intellectual lineage
  (Anderson on relational equality, Pettit on non-domination, List & Pettit on group agency, Ostrom on the
  commons, Beer on viability) and the pure peer-standing → cooperative-form argument.
- **activism (Layer 9): why not the status quo**, the *present makes it urgent*. The sourced,
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
can constitute peer standing. That argument lives in **philosophy (Layer 2)**. **Governance (Layer 7)** is
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

- **Justification order (bottom-up, = the numbering):** history → philosophy → cairn → spec → impl → croft →
  governance → socialization → activism. "What rests on what." The why and the field ground the spec; the
  spec grounds the build; the build grounds the manifestation and the outward edges.
- **Build order (where you actually start):** you *start at the spec* (Layer 4), "define and build the
  protocol everything else rests on," principles-first and field-informed, then implementation → product →
  governance. The why-layers (1, 2) and the field survey (3) and the outward-edge layers (8, 9) are written
  alongside and often finalized late, but they are logically prior or outward. Keeping these two orders
  distinct is what prevents the "the numbering doesn't match how I'd start" confusion: you build from 4, but
  the stack is grounded from 1.

## Register discipline (so adjacent layers don't duplicate)

Each layer states a shared concept in its **own register**, and no two compete to be the source of truth
for the same claim:

- The **peer-standing argument** appears as *principle* in philosophy, as *empirical harm* in activism, as
  *human-facing presentation* in socialization, and as *manifestation* in governance. One claim, four
  registers, one primary home each.
- The **spec** states persona as a *principle* (Part 1) and as a *mechanism* (Part 2); the vocabulary of
  record (`persona-definition.md`, Appendix D) is a *lattice* over the mechanism. Different altitudes, single
  source per claim.

## Current state (2026-07-07)

Seeded: `philosophy/` (peer-standing set + prior-art), `cairn/` (atproto-ecosystem + the social-lexicon
research brief; more ecosystem material to migrate), `governance/` (reserved; README only),
`socialization/` (essay + pitch), `activism/` (research set), `drystone-spec/` (populated),
`impl/` (delivery-layer + mls bundles + the shared doc-writing-method). Not yet created: `history/`
(theme 02 is its seed), `croft/` (theme 08). The `02`–`08` theme docs at beta root remain the reading
spine until their content migrates into layers.

**cairn migration backlog** (ecosystem material currently living elsewhere that belongs here): the MLS
scaling survey, the Willow/Meadowcap analysis, the Blacksky research (in raw transcripts), the ecosystem
landscape in the local-authority notes, and the Roomy/p2panda/SimpleX/Matrix tracking. Modular Politics
stays in `philosophy/prior-art/` (it is an academic governance *frame*, not a shippable building block).
