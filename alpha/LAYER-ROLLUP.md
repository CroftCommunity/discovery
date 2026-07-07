# Alpha + themes -> layer rollup (the layer-keyed maturity trace)

date: 2026-07-07
supersedes (as the live map): `BETA-ROLLUP.md` (theme-keyed, retained as historical provenance)

## Purpose

The auditable record of how `alpha/` content AND the beta *theme* synthesis (`beta/02`..`beta/08`) were
re-filed onto the beta **layer-cake**, keyed by layer rather than by theme. It replaces the theme-keyed
`BETA-ROLLUP.md` as the live source->landing map, because the themes are being discarded and a theme-keyed
map stops being accurate. `BETA-ROLLUP.md` stays frozen as the historical record of the theme era.

Discipline (per `plans/2026-07-07-refile-alpha-into-layers.md` and its Phase-0 ledger): the 2026-06-25
per-file audit was a *signal, not an authoritative set*; every disposition here was re-checked against the
actual source, and conclusions were enumerated by reading, not inherited. `alpha/` stays frozen; re-filing
copied or distilled INTO layers.

## The theme -> layer landing (each discarded theme's content, and where it now lives)

| theme (to discard) | lands in | status |
|---|---|---|
| 01 epistemic foundation | drystone-spec Part 1 | pre-existing (done before this re-file) |
| 02 enclosure and its inversion | history/ (material) + philosophy/ (intellectual) | history: `crofting-dry-stone-and-the-enclosure-inversion.md` (incl. CM-A1); philosophy: peer-standing set carries the argument |
| 03 the living ecosystem | cairn/ (open) + fenced/ (centered) | cairn: atproto-ecosystem/adjacent-systems/mls/willow/blacksky/social-lexicon; fenced: group-scale, operational-rates, platform-dominance-and-adoption, group-chat-failure-modes |
| 04 the protocol we proved | drystone-spec/ + impl/ | spec Part 2 (mature) + impl (delivery-layer/mls/drystone-design/experiments, transport + DDIA refs); CM-P1/P2 covered in spec |
| 05 identity you carry | drystone-spec/ + impl/ | spec §5 + impl; CM-A2 placed in spec open-threads (recovery lock/trust) |
| 06 safety without surveillance | drystone-spec/ + impl/ | spec §6/§7/§11 + impl; CM-P3 (geer rungs) covered in spec |
| 07 sustainability & stewardship | governance/ + philosophy/ | governance: foundation-cooperative-and-sustainability (incl. CM-A3), open-publication-and-ip-stewardship; philosophy: peer-standing (the WHY) |
| 08 croft the product | croft/ + socialization/ | croft: product-the-garden-of-ponds, social-graph-as-substrate; socialization: brand-and-voice, adoption-strategy, logo, assets |

## The layer-keyed source map (what landed in each layer, and how)

- **history/ (1):** `crofting-dry-stone-and-the-enclosure-inversion.md` distilled from theme 02 +
  `narrative/verticals/croft-the-name-and-the-commons.md` + the crofting/enclosure raw transcripts +
  `SOVEREIGN-COMMONS-DOSSIER.md` (CM-A1). Crofting-narrative colour claims carry `[UNVERIFIED]`.

- **philosophy/ (2):** peer-standing set (pre-existing) carries the enclosure/cooperative argument and
  theme 07's WHY; the batch-11 intake docs (lifeworld, commensurability, epistemics) carry the adjacent
  frames. `crystallized/principles.md` deep-foundation blocks and `narrative/lineage-of-a-design-imperative`
  are `covered` by spec Part 1 + peer-standing. CM-A4 parked (below).

- **cairn/ (3):** the open-field research (`public-social-protocols`, `atproto-private-data`,
  `atproto-sovereign-appview`, `germ-xchat-features`, `atproto-atmospheric-web`, `messaging-solutions-landscape`
  open half, `ECOSYSTEM.md`) is `covered` by the existing cairn docs (atproto-ecosystem, adjacent-systems,
  social-lexicon), verified by grep. No net-new cairn doc needed.

- **fenced/ (3'):** batch-11 (`group-scale-versus-e2ee`, `operational-rates-and-platform-economics`) plus the
  re-file net-new `platform-dominance-and-adoption.md` (discord-dominance, social-platform-cycle descriptive,
  messaging-landscape centered half, founder-motivations why-incumbents-win) and `group-chat-failure-modes.md`
  (group-chat-failure-modes(-plain), discord-matrix Matrix E2EE lessons). Harm reading routed to activism;
  adoption strategy to socialization.

- **drystone-spec/ (4):** Part 1+2 (mature) `covered` for theme 01/04/05/06 mechanism conclusions
  (`CROFT-PROTOCOL`, `proof-ledger`, `test-narrative`, `conformance-suite`, `thesis-lineage-groups`,
  `merge-split-corpus`, `thinking/drystone-spec/` §2/§X, and the 05/06 mechanism notes), verified by the
  2026-06-25 audit + the batch maturations. Net-new this re-file: `dag-cbor-and-content-addressing.md`
  (batch), and CM-A2 in `open-threads.md`. CM-P1/P2/P3 verified present.

- **impl/ (5):** delivery-layer, mls, drystone-design (p10 companions), experiments, doc-method (+§17),
  transport-iroh-gossip-and-quic, references-DDIA. The 04/05/06 design detail is `covered`/`merge` (the spec
  cites conclusions; impl holds the working-out). No net-new re-file doc beyond the batch additions.

- **croft/ (6):** `product-the-garden-of-ponds.md` + `social-graph-as-substrate.md` distilled from theme 08
  + `thinking/app/*` + `thinking/social-graph-as-substrate.md`. Decision-gated product items surfaced.

- **governance/ (7):** `foundation-cooperative-and-sustainability.md` (theme 07 + foundation/co-op/
  survivability + principles sustainability half; CM-A3) + `open-publication-and-ip-stewardship.md`
  (open-publication + venue map) + `making-preventative-work-visible.md` (batch). Decision gates surfaced.

- **socialization/ (8):** essay + pitches + logo (pre-existing) + `brand-and-voice.md` (messaging-and-quotes
  + brand-comms-workbook + app/brand-and-voice-notes; T4) + `adoption-strategy.md` (founder-motivations
  strategy half + adoption-enablement; T11) + `assets/` (the two wordmark PNGs, copied). `narrative/long-form`
  and `short` stay alpha-only (drafting skeletons).

- **activism/ (9):** relational-field set (pre-existing) + `platform-extraction-and-captured-labor.md`
  (the extraction/captured-labor harm strand, T35, from discord-dominance + social-platform-cycle harm
  halves).

## Conclusion-coverage gate (the Phase-4 deletion gate)

Every carry-forward conclusion is placed or verified-present:

| item | verdict | where |
|---|---|---|
| CM-A1 relationships-as-newest-enclosure | PLACED | history/crofting-dry-stone-and-the-enclosure-inversion §6 |
| CM-A2 recovery lock-vs-trust | PLACED | drystone-spec/open-threads.md §2 (recovery-anchor sharpening) |
| CM-A3 non-mimicry moat | PLACED | governance/foundation-cooperative-and-sustainability |
| CM-A4 linear/cyclical operating systems | PARKED | keep-vs-retire; the adjacent value-systems frame is touched by philosophy/commensurability-and-the-two-ledgers; the explicit manifesto framing stays alpha-only (SOVEREIGN-COMMONS-DOSSIER), a later keep-vs-retire call, low rank |
| CM-P1 rollup trilemma / MMR | COVERED | drystone-spec/part-2 + conventions + open-threads |
| CM-P2 no-convergence honest claim | COVERED | drystone-spec/part-2 + CHANGELOG (social-legibility invariant) |
| CM-P3 geer 3-rung enumeration | COVERED | drystone-spec/part-2 + impl |

Every discarded theme's conclusions are captured across layers (table above): the net-new layer docs
(history, croft, fenced, governance, socialization) were distilled FROM the theme files plus the alpha
sources, so the theme conclusions are in the layers by construction; the mature layers (spec, impl, cairn,
philosophy, activism) were always the theme targets and are verified-covered.

**Gate verdict: PASS, with one parked item (CM-A4, keep-vs-retire, non-blocking).** No unique theme
conclusion is lost if `beta/02`..`beta/08` are discarded. Phase 4 (discard) is therefore safe, pending the
user's go-ahead and the CM-A4 keep-vs-retire call.

## What stays alpha (not re-filed), for the record

`seeds/` (162, frozen provenance); `plans/` (process); the index/process surfaces (`README`, `ANALYSIS`,
`COHESION`, `ROADMAP(_TODO)`, `TEST-PLAN`, `NAMING`, the fact-check logs); `BETA-ROLLUP.md` (superseded,
historical); `SOVEREIGN-COMMONS-DOSSIER.md` (provenance/raw; durable conclusions CM-A1 pulled up, CM-A4
parked); `narrative/long-form.md` + `short.md` (drafting skeletons). `beta/thinking/raw/` holds two raw
review-note text files (`01_beta_review.txt`, `open threads review Jun 26...txt`), process/raw artifacts
misfiled under beta, candidates to relocate to alpha provenance in a later cleanup; they do not gate the
theme discard.
