# discovery / beta / croft: Layer 6 (the product, Croft as one flavor)

date: 2026-07-07

**What this layer is.** The product layer: **Croft**, the first application built on the neutral Drystone
protocol. It holds the product thinking, the app shape, and the design philosophy, everything that is
specific to Croft-the-product rather than to Drystone-the-protocol. Croft is deliberately *one flavor* on
the neutral core: the protocol (Layer 4) and the reference implementation (Layer 5) name no ecosystem;
Croft is the first ecosystem on top, and the layer is kept separate so the neutral stack is never
conflated with one product's choices.

## Scope

In scope: the product shape and its design philosophy, the social-graph-as-substrate reframe (the social
graph is the substrate; chat is one tenant, peer to other activities hung off a durable group), the
garden-of-ponds-and-pads model, the one-core-plus-thin-shells client architecture, the quality bar and
craft rules, and the inclusion pathways for activities (build-fresh / wrap / port).

Boundary calls:

- **vs `drystone-spec/` (Layer 4) and `impl/` (Layer 5).** The spec and the reference core are the
  *neutral* stack, no ecosystem named. Croft is a product built on them. A mechanism belongs to the spec;
  the *product surfacing* of that mechanism (how Croft presents it) belongs here. The social-graph-as-
  substrate is core Drystone in the spec; its *product* expression (the group's-face UX, ponds/pads) is
  croft.

- **vs `socialization/` (Layer 8).** Croft is the product's shape and design; socialization is how the
  product is voiced and pitched for adoption. The garden model lives here; the essay and the elevator pitch
  live there.

- **vs `governance/` (Layer 7).** Governance is the foundation-and-cooperative that *operates* the neutral
  stack and the Croft flavor; croft is the product itself.

## Contents

| doc | what it is |
|---|---|
| `product-the-garden-of-ponds.md` | The product shape: the garden thesis (a composable garden of ponds and pads on one core, thin shells), the honest seams, the functional-core / imperative-shell spine, one-core-plus-thin-shells client architecture, the quality/craft bar, and the three inclusion pathways for activities (build-fresh / wrap / port). |
| `social-graph-as-substrate.md` | The reframe that anchors the product: the social graph is the substrate and chat is one tenant; the durable group (group identity != member set; implicit/sticky/pruned lifecycle); the local-projection vs shared-anchor seam; and the load-bearing-but-invisible group's-face UX (the hardest product problem). |
| `presence-ritual-and-composed-ponds.md` | Four product conclusions the taxonomy left as stubs: the Presence & Ritual pond ("the project's heart" — thinking-of-you ping, guestbook, and the **question-of-the-day with no streak**, "the whole ethic in miniature"); game-outcome-as-custom-lexicon (ephemeral over iroh, only the settled outcome durable; **attestation is the open hard part**); the iroh tiered-exposure product model (public bridge → browser-peer → native; relay as complete broker); and the **valuation edge** (directional weighted inter-group trust with no shared keys). Dialogue-sourced calls flagged. *(Phase-1 recovery.)* |

## Provenance & status

Seeded 2026-07-07 as part of the alpha->layers re-file (plan:
`../../alpha/plans/2026-07-07-refile-alpha-into-layers.md`). Distilled by reading the actual sources (the
theme `08-croft-the-product.md` synthesis being discarded, the `thinking/app/` corpus, and
`thinking/social-graph-as-substrate.md`), verified from source rather than inherited from a prior audit.
**Decision-gated:** the Croft product shape carries open product decisions (the group's-face UX iteration,
reconciling the sticky-group lifecycle with membership-vs-access) tracked in `../OPEN-THREADS.md`; those
stay surfaced, not resolved. See `../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.

## What this layer establishes (and does not)

Establishes the Croft product shape and its design philosophy as one flavor on the neutral stack, so the
product's choices are tracked separately from the protocol and the reference core. Does **not** define the
protocol or the reference core (those are `drystone-spec/` and `impl/`), voice or pitch the product (that is
`socialization/`), or build the operating institution (that is `governance/`).
