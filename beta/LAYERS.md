# discovery / beta: the layer model

date: 2026-07-06

**What this is.** The canonical description of the beta layer-cake: the layers, what each holds, the
two ways the stack is traversed, and the register distinctions that keep adjacent layers from duplicating
each other. `README.md` carries the index table; this file carries the reasoning. When the model and a
layer `README` disagree, this file governs.

## The layers

The field (tier 3) has **two halves**: `cairn/` (the open field we build among) and `fenced/` (the fenced
field we are an alternative to). They are the same activity, surveying the landscape, on opposite sides of
the fence, so they share a tier rather than each taking their own integer.

```
Layer  Dir              Holds                                                       Register / role
──────────────────────────────────────────────────────────────────────────────────────────────────
1      history/         crofting, dry-stone stacking, cairns, the space itself      MATERIAL history
2      philosophy/      the principles + thinkers; the pure peer-standing argument  INTELLECTUAL history
3      cairn/           the OPEN field, existing bolstering tech we build among:     THE FIELD (open)
                        iroh, MLS, Willow/Meadowcap, CBOR-DAG, atproto/AT, AP,
                        CRDT, QUIC, p2p; products Roomy, Blacksky, p2panda,
                        SimpleX, Matrix. Some bubbles into the spec; some is
                        tracked for network-effect and homage.
3′     fenced/          the FENCED field, the centered commercial platforms:        THE FIELD (fenced)
                        Telegram, Discord, WhatsApp, Signal, Slack, Teams,
                        Reddit, X, iMessage, Messenger, LINE, WeChat. Roster/
                        call/broadcast ceilings, E2EE stance by surface+layer,
                        per-group rates, economics. A descriptive map, no
                        argument; feeds the spec (§11.9.1, §11.13) and activism.
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
- **Layer 3 is the field, in two halves.** `cairn/` is the survey of the existing *open* bolstering tech,
  the building blocks and products we build among. `fenced/` is the survey of the *fenced* field, the
  centered commercial platforms we are an alternative to. The field sits between the principles and the
  spec because **the spec had to survey the field first**, to know whether the ecosystem held the parts to
  assemble a safe amount of novelty practically (designing iroh, MLS, CBOR-DAG, or Willow from scratch would
  have been too heavy a blocker), and to know what the centered incumbents can and cannot do (the fenced
  map's E2EE-vs-scale tradeoff and per-group rates ground the spec's scaling posture at §11.9.1 and §11.13).
  The spec is built on the principles *and* on both halves of the surveyed field.
- **Layers 4–7 are the build.** Protocol (4) → reference implementation (5) → product/brand (6) →
  manifestation as a running institution (7). Define the protocol from the principles and the field,
  implement a maintainable core, wrap it as a product, then ground it in the world as a foundation-sponsored
  cooperative.
- **Layers 8–9 are the outward edges.** Socialization carries the message outward; activism is the
  present-tense "why not the status quo", the current-state indictment that motivates a newcomer.

**The field-and-response triad: cairn, fenced, activism.** Three layers touch "the field," each in its own
register, and none competes to be the source of truth for another's claim:

- **cairn (Layer 3, the open field):** the catalogue of what we build *among*, the enabling composable tech
  we credit and reuse. The name is deliberate: a cairn is dry-stacked waymarker stones raised by many hands
  to mark a path for those who come after, which is exactly what an index of this space is, and it shares
  the dry-stone construction family with `drystone` itself (cairn catalogues the stones; drystone builds
  the wall).

- **fenced (Layer 3′, the fenced field):** the descriptive map of the centered commercial platforms, their
  extent and shape (how large rosters/calls/broadcasts can grow, what each can and cannot do, how
  communities behave inside them, how they are monetized). It makes no argument; it draws the map. The
  name is exact: these platforms were never an open commons that got *enclosed*, they were built *fenced*
  from the start, so the accurate word is the state (fenced), not the act (enclosing). That leaves
  "enclosure" free for its true historical meaning in `philosophy/` and `activism/`.

- **activism (Layer 9, harm and response):** what the fenced map *means* in harm terms and anti-society
  terms, and what we do about it (up to and including education). It reads its harm case off the fenced map.

So cairn and fenced are the two halves of the field survey (open and fenced, same activity on opposite
sides of the fence), and activism is the normative reading that sits atop the fenced half. cairn is the
inverse of activism in valence (credit vs indictment); fenced is the neutral map both of them are drawn
against.

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

- **Justification order (bottom-up, = the numbering):** history → philosophy → (cairn + fenced) → spec →
  impl → croft → governance → socialization → activism. "What rests on what." The why and the field (both
  halves) ground the spec; the spec grounds the build; the build grounds the manifestation and the outward
  edges. Activism, at the top, reads its harm case off the fenced half of the field.
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

## Quote discipline and resolution labels (cross-layer, this file governs)

Two conventions cut across every layer and are stated once here because two existing rules
(`README.md` §4 "preserved whole as block quotes" and `impl/doc-writing-method.md` §103 "quote the
shortest span") pull against each other on length, and per-doc improvisation had let quotes collapse
into narrative prose. Where those two disagree, **this section is the tie-breaker.**

- **Quotes stand alone; synthesis is labeled; terms of art are not quotes.** A source's words appear
  as a **set-off block quote**, attributed to a precise locator, interleaved with the narrative
  (the worked model is `philosophy/lifeworld-and-the-system.md`) — never folded into a sentence as a
  bare fragment and never restated in our own words with only a citation appended (silent paraphrase
  is a defect, per `doc-writing-method.md` §103). A **coined term of art** (Polanyi's *disembedding*,
  Scott's *metis*, Jacobs's *organized complexity*) is rendered in *italics as a named concept*, not
  in quotation marks, because a single term in quote marks masquerades as a quotation it is not. When
  the block quote and shortest-span rules collide, the block quote wins: preserve the whole load-
  bearing passage.
- **Technical design docs follow RFC-style citation, not the block-quote form.** The block-quote rule
  above governs **narrative-prose** docs (history, philosophy, activism, socialization, governance, the
  cairn/impl survey prose), where a source's words would otherwise collapse into the narrative. The
  **`drystone-spec/` Parts 1 and 2** (and technical design docs like them) instead follow **RFC-style
  conventions**: a short normative or spec clause is cited **inline in quotation marks with a precise
  locator (RFC §, MSC, CVE) and a verification flag** (`Verified-RFC` / `[confirm]` / `Verified`). That
  form is **compliant** — it meets the rule's substantive intent (attribution preserved, verification
  labeled, the source's claim not masquerading as ours) and reads the way a spec is meant to read. Do not
  convert the spec's inline-clause citations to set-off block quotes; a design doc reads differently than
  straight prose. The anti-paraphrase core still holds everywhere: no doc silently restates a source's
  words as its own.
- **AI-surfaced quotations never lose their flag.** Much of the corpus was surfaced in AI discussion
  and is not verified against a primary edition. Elevating such a passage to a standing block quote
  does **not** upgrade its epistemic status: every such quote keeps its per-quote
  `[UNVERIFIED, confirm against primary edition before publish]` flag (README §4). Never manufacture a
  clean-looking verbatim quote from material you cannot source; if no verbatim exists, present the
  source's position as explicitly-labeled synthesis rather than as a quotation.
- **Label the resolution and link the next rung.** A doc that is one grain of a `doc-writing-method.md`
  §16 ladder (elevator pitch / coffee shop / library) states its own resolution and points to the rung
  above and below it, the way `drystone-spec/part-1-reasoning-underpinnings.md` opens with
  `Resolution: library ... for shorter tellings see the coffee-shop overview, then the elevator pitch.`
  A doc that calls itself "the compressed version" MUST name its companion by a resolvable pointer, not
  by prose title alone, so a reader is never left asking where the full version went.
- **Reasoning travels with the decision (the anti-rollup rule).** A settled decision recorded at a terse
  resolution (a current-state register, a decisions list, a charter line) MUST link to its full reasoning
  at library resolution, and that reasoning MUST mature forward *with* the decision — never be left behind.
  Recording the decision without its grounding is precisely the failure the 2026-07-08 coverage audit found
  (`../alpha/plans/2026-07-08-beta-coverage-gap-ledger.md`): the alpha→beta rollup systematically kept the
  decision and dropped the rationale, prior-art body, and illustration behind it, which reads as "over-
  summarized" and, worse, cannot bring a newcomer along — a decision you cannot re-derive is one a reader
  must take on faith. So a decision register is a *pitch-resolution index over the reasoning*, not a
  replacement for it; every row points down to where the argument is carried whole.

## Consolidate duplicate thinking into one whole (the beta synthesis job)

A defining reason alpha matures into beta: alpha accumulated **duplicate thinking**. Each time a dive
entered a particular area it re-set the context from scratch, so the same idea was re-derived, re-framed,
and partly re-stated across several docs and transcripts. Beta's job is to **pull all of those scattered
instances together into one WHOLE treatment — cleaned up and clear** — not to transcribe any single
fragment.

- **Consolidate, don't lift.** Recovering or maturing a claim means gathering *every* place the thinking
  appears in the prior tier, reconciling them, and writing the union once, at full grain, in its one home
  (the one-home-per-claim invariant). The best-stated version wins the framing; the others contribute any
  detail it lacked; the repeated context-setting is dropped.
- **Whole, not lossy.** "Cleaned up" means the *duplication* and the scaffolding each dive needed to
  orient itself are removed — never that reasoning, prior art, or nuance is dropped. Completeness of the
  argument is preserved (see "reasoning travels with the decision"); only the redundancy is cut.
- **A single alpha pointer is a starting point, not the boundary.** When maturing an area, search the whole
  prior tier for the other instances before writing; a doc that consolidates only the first source it was
  handed is half-done and will read as thinner than the corpus actually supports.

## Current state (2026-07-07)

Seeded: `philosophy/` (peer-standing set + prior-art), `cairn/` (atproto-ecosystem + the social-lexicon
research brief; more ecosystem material to migrate), `fenced/` (seeded 2026-07-07 batch eleven: the
centered-platform capability map + the per-group operational rates and platform economics),
`governance/` (reserved; README only), `socialization/` (essay + pitch), `activism/` (research set),
`drystone-spec/` (populated), `impl/` (delivery-layer + mls bundles + the shared doc-writing-method).
Seeded 2026-07-07 as part of the alpha->layers re-file: `history/` (Layer 1, the material
crofting/dry-stone/enclosure-inversion record) and `croft/` (Layer 6, the product: the garden of ponds/pads
and the social-graph-as-substrate reframe). Both net-new layers now exist. The `02`–`08` theme docs were
re-filed into layers and **discarded 2026-07-07**; the layer-cake is now the sole structure. The
source-to-layer trace is `../alpha/LAYER-ROLLUP.md`.

**`fenced/` created (2026-07-07, batch eleven).** The tenth layer and the fenced half of the field (tier
3′), the descriptive counterpart to `cairn/` and the substrate `activism/` reads its harm case from. First
population is two survey docs distilled from the batch-11 raw transcript: `group-scale-versus-e2ee.md` (the
14-platform capability map and the two forces) and `operational-rates-and-platform-economics.md` (the three
per-group rates plus the Telegram economics). Both feed the Drystone spec's §11 (large-group scaling)
(§11.9.1 encryption posture, §11.13 empirical basis).

**Multi-layer intake (2026-07-07).** A batch of web-session material was distilled across layers (raw
transcripts frozen in `../alpha/seeds/transcripts/raw/`): **philosophy** gained three docs, the Lifeworld/
System and public-sphere argument (Habermas + Arendt/Postman/Debord/Morozov), the commensurability / two-
ledgers / legibility-and-metis grounding (Polanyi, Sandel, Georgescu-Roegen, Taleb, Putnam, Scott, Jacobs;
"social value is a local-authority concern," "why a ledger cannot be the social backplane"), and an
epistemics note (Rosenhan, the replication crisis, provenance-first verification); **governance** gained its
first content doc (the improvement paradox and making preventative work visible); **socialization** gained a
logo brand-asset stub (drystone-stacking, brief only); **impl** gained transport (iroh-gossip/QUIC) and
Kleppmann-DDIA reference notes plus a diagrams section in the doc-writing method; **drystone-spec** gained a
dag-cbor / content-addressing companion primer; and **fenced** gained a Facebook group-size data point. The
arecipe project material from the same batch was skipped (filed under `CroftC/arecipe`).

**cairn migration: done (2026-07-07).** The former backlog is now filed as four survey docs distilled from
the raw transcripts: `mls-and-mimi.md` (incl. the scaling survey), `willow-meadowcap.md`,
`blacksky-and-atproto-community.md`, and `adjacent-systems.md` (Roomy/p2panda/SimpleX/Briar/Cwtch/Matrix/
Session/Nostr, the ecosystem landscape). Modular Politics stays in `philosophy/prior-art/` (an academic
governance *frame*, not a shippable building block).
