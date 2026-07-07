# Phase 0: the alpha -> layer disposition ledger

date: 2026-07-07
status: DRAFT for review (Phase 0 of `2026-07-07-refile-alpha-into-layers.md`). No files re-filed yet.

## What this is

The master disposition: every in-scope alpha content doc, re-keyed from its 2026-06-25 theme disposition
onto the beta layer-cake, with a treatment code and a status (already-covered vs to-file). It is built by
re-keying `2026-06-25-beta-coverage-per-file-audit.md` (which dispositioned all 165 files once, into themes)
through the theme->layer map, then reconciling against what each layer already holds. Conclusion-level
completeness items (the CM-A / CM-P carry-forwards) are tracked at the end; a `covered` mark is only valid
when a cited layer section is verified to carry the conclusion.

**Treatment codes:** `copy` (byte-faithful into the layer) · `distill` (synthesize a new/updated layer doc)
· `merge` (fold into an existing layer doc, keep the mature version) · `covered` (already in the layer;
verified pointer required) · `alpha-only` (process/index/raw; stays alpha) · `excluded` (do-not-carry).

**Theme -> layer re-key:** 01 -> spec Part 1 (done); 02 -> history(1)+philosophy(2); 03 -> cairn(3)+fenced(3');
04/05/06 -> spec(4)+impl(5); 07 -> governance(7)+philosophy(2); 08 -> croft(6)+socialization(8). The harm
half of 03/07 -> activism(9).

## The net-new picture (where the actual work is)

Most 04/05/06 content is already in **drystone-spec** and **impl** (the spec folded delivery-layer, mls,
drystone-design; the batches matured it), and most 03 open-field content is in **cairn**; those layers are
mostly `covered`/`merge` (verify-and-reconcile, little net-new). The real net-new filing is:

- **history (1)** and **croft (6)**: do not exist; must be created and filled (the biggest net-new).
- **fenced (3')**: only the two batch-11 docs exist; the centered-platform research (discord-dominance,
  discord-matrix, social-platform-cycle, group-chat-failure-modes, founder-motivations) is net-new.
- **governance (7)**: only the preventative-work doc exists; the foundation/co-op/IP-stewardship corpus is
  net-new.
- **socialization (8)**: has essay+pitches+logo; the narrative reservoir (long-form/short/messaging-and-
  quotes/brand-comms/adoption) and the wordmark assets are net-new/merge.
- **philosophy (2)**: has peer-standing + the 3 intake docs; the principles/lineage material is largely
  `covered` by spec Part 1, with CM-A4 net-new.

## Layer 1 — history/ (NEW, material history)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| narrative/verticals/croft-the-name-and-the-commons.md | etymology of croft; enclosure inversion; Clare; the commons-rebellion; four-axis alignment (material half) | distill | to-file (create layer) |
| seeds/raw croft-crofting-narrative / -research / croft-etymology-enclosure-tradition / croft-clare-enclosure-poems | crofting material history; enclosure-as-recurring; Clare poems (all [UNVERIFIED]) | covered-by-distill (seeds stay frozen; conclusions land via the doc above) | to-file |
| NAMING.md (Croft+Drystone etymology, Princeps Problem) | the name and what it means, materially | distill | to-file |
| CM-A1 (relationships as the newest enclosure) | "platforms rent our relationships back to us"; the third place as the enclosed commons | distill | to-file (completeness-critical) |

Boundary vs philosophy: history carries the *material/cultural* enclosure story (croft, common, dry-stone,
Clare); philosophy carries the *intellectual* argument (relational equality, non-domination). CM-A1 bridges
them and should land in history with a philosophy cross-reference.

## Layer 2 — philosophy/ (has: peer-standing set + lifeworld/commensurability/epistemics intake docs)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| crystallized/principles.md | capture-not-centralization; recurring-inversion; capabilities-not-rights; anti-fragile/pay-keepers/sunset (the deep-foundation blocks) | covered | verify vs spec Part 1 + peer-standing (folded to 01 §6 / 07 A2-A5); anti-fragile half -> governance |
| narrative/lineage-of-a-design-imperative.md | the 2,400-yr intellectual arc, quotes preserved whole | covered | verify vs spec Part 1 §2.1-2.6; if the philosophy layer wants the lineage as its own doc, distill |
| CM-A4 (linear/extractive vs cyclical/relational operating systems) | the two-value-systems frame; a third "why" beyond Ashby/Hayek/Scott and Ostrom | distill | to-file (completeness-critical, low-rank; keep-vs-retire call) |
| the 07 peer-standing/non-domination/cooperative argument | (the whole argument) | covered | already the layer's core (peer-standing-and-the-cooperative-form.md) |

## Layer 3 — cairn/ (open field; has: mls-and-mimi, willow-meadowcap, blacksky, adjacent-systems, atproto-ecosystem, social-lexicon)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| research/public-social-protocols.md | X/Bluesky/Threads/Mastodon/Pixelfed; three poles; dual-use DID | merge | reconcile into cairn atproto-ecosystem / adjacent-systems |
| research/atproto-private-data-architecture.md | real Private Data WG; defers E2EE/ZK; Croft on the harder ZK side | merge | reconcile (T7 tracked) |
| research/atproto-sovereign-appview-club.md | sovereign-AppView read-side; Twitter Circles | merge | reconcile |
| research/germ-xchat-features.md | privacy-free/convenience-effortful inversion; closest cousin | merge | reconcile into adjacent-systems |
| thinking/atproto-atmospheric-web.md | demand-side argument for Croft's crypto | covered/merge | reconcile |
| ECOSYSTEM.md field register (open half) | the live-field map | covered | ECOSYSTEM stays alpha (index); cairn carries the operative subset |
| research/messaging-solutions-landscape.md (open half) | the universal trade; field map; differentiators | merge (split) | open-field half -> cairn; centered half -> fenced |

## Layer 3' — fenced/ (centered platforms; has: group-scale-versus-e2ee, operational-rates-and-platform-economics)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| research/discord-dominance.md | zero-friction; the S-1; enshittification | distill | to-file (net-new; harm framing -> activism) |
| research/discord-matrix-groupchat.md | Matrix E2EE operational lessons (UTD invariant, mandatory-recovery onboarding, expectation-gap) | distill | to-file (net-new; was T16) |
| research/social-platform-cycle.md | the platform lifecycle; enshittification arc (historical + prescriptive) | distill (split) | descriptive -> fenced; the harm/prescriptive -> activism; historical -> history |
| research/group-chat-failure-modes.md + -plain.md | survivor-determinism; covert-ordering; genesis amendability; churn-fold Achilles heel | distill | to-file (net-new; was T5; also informs spec §11 / impl) |
| research/p2p-founder-motivations-adoption.md | the adoption chasm; only-Signal-crossed; institutional-mandate bridge | distill | to-file (net-new; was T11; adoption framing may go socialization) |
| research/messaging-solutions-landscape.md (centered half) | the centered-platform field map | merge (split) | -> fenced |

## Layer 4 — drystone-spec/ (mature; mostly covered/cited)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| crystallized/CROFT-PROTOCOL.md | the wire spec; substrate model (K7) | covered/cited | verify vs Part 2 §4/§5; the §2 spec-vs-code pre-image discrepancy is a known follow-on |
| crystallized/proof-ledger.md / test-narrative.md / conformance-suite.md / TEST-CORPUS.md | the proof results + the black-box conformance suite | covered/cited | backs the spec's green-real flags; TEST-CORPUS is alpha-only index |
| thinking/thesis-lineage-groups.md; merge-split-corpus.md | the thesis; two-tree; survivor reconnect; split/merge/conflict taxonomy | covered | verify vs Part 2 §7 / §11 |
| thinking/drystone-spec/ (section-2, section-x, skeleton) [was T1] | one-kind-of-peer; rights vs capabilities; append-only fold; frontier-closure | covered | verify supersession by Part 2 §5 / §7; if any clause is not yet in the spec, merge; T1 |
| the 05/06 mechanism conclusions (multi-device, plc-identity, revocation, freshness, membership-vs-access, geer, failed-op, group-privacy-lanes, meer, abuse-resistance, realtime-media) | (per-mechanism) | covered/merge | mostly folded via the spec + impl batches; verify each; CM-P1/P2/P3 + CM-A2 below |

## Layer 5 — impl/ (mature; has delivery-layer, mls, drystone-design, experiments, doc-method, refs)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| thinking/experiment-suite.md; crystallized proof/test corpus | the INV-* test spec; the experiment/validation corpus | covered/merge | reconcile into impl/experiments |
| thinking design notes (the 04/05/06 working detail behind the spec) | design-level detail (dials, ladders, taxonomies) | covered/merge | the spec cites conclusions; impl holds working-out (drystone-design already carries the p10 companions); verify no orphaned design detail |
| research/iroh-realtime-media-references.md; str0m-production-readiness.md | media-layer references; str0m ICE open | covered/cited | backs impl/spec media; T10 residual |

## Layer 6 — croft/ (NEW, product)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| thinking/app/design-philosophy.md | the garden thesis; honest seams; FCIS spine; craft rule | distill | to-file (create layer) |
| thinking/app/client-architecture-adr.md | one core + thin shells; Crux slim; option-C | distill | to-file |
| thinking/app/design-criteria.md | the quality bar; visual system | distill | to-file |
| thinking/app/ponds/* (build-order, games/apps lists, on-device-llm, fair-reveal, webxdc-security, build-shape) | ponds/pads model; inclusion pathways; resolver tier-zero; on-device-LLM detect-first; commit-reveal; webxdc security | distill | to-file (fold the model + leverage, cite specs) |
| thinking/app/build-specs/* | Phase-0 green-real 20/20 + the 5 DECISIONS; forward build plan | covered/cited | Phase-0 proof status cited; forward plan is process |
| thinking/social-graph-as-substrate.md [T26] | the social-graph-as-substrate reframe; group != member-set; sticky lifecycle; local-projection vs shared-anchor | distill | to-file (the core product reframe; already partly in spec §11 substrate, but the product surfacing is croft) |
| thinking/interaction-tiers.md | three products one send button; visible-cost privacy | distill/merge | croft (product) with a spec cross-ref |

## Layer 7 — governance/ (has: making-preventative-work-visible)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| thinking/foundation-and-ip-stewardship.md | three layers; AGPL+DCO; two-tier mark; entity phasing | distill | to-file (net-new) |
| thinking/cooperative-social-union-model.md | four-pillar Social Union; failure lineage; growth-apathetic moat | distill | to-file (net-new); CM-A3 below |
| thinking/governance-and-survivability.md | bankruptcy-remote steward; pre-funded archive runway | distill | to-file (net-new) |
| crystallized/principles.md (anti-fragile / pay-keepers / sunset half) | non-extraction is anti-fragile; pay-the-keepers; sunset | merge | into the governance sustainability doc |
| research/open-publication-and-ip-protection.md; socialization-and-publication-venues.md [K9] | CC-BY doc + Apache code; prior-art-first; IETF-then-arXiv; per-layer venue map | distill | to-file (IP/publication; venue map may split to socialization) |
| CM-A3 (non-mimicry moat) | the affirmative competitive case competitors structurally cannot ship | distill | to-file (completeness-critical) |

Decision-gated (carry the reasoning, not the citations): the MO Chapter 351 legal-review gate, the Noria
name, the capital-formation problem (T33). These stay surfaced, not resolved.

## Layer 8 — socialization/ (has: tilling-the-soil essay, sixty-second-pitch, coffee-shop, elevator, logo stub)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| narrative/long-form.md; short.md | the synthesis skeleton (drafting surface) | alpha-only | conclusions fold across layers; the drafts stay alpha |
| narrative/messaging-and-quotes.md [T4] | brand/voice/quotes reservoir | distill/merge | to-file (the brand reservoir; T4) |
| narrative/adoption-enablement.md; brand-comms-workbook.md | adoption enablement; brand-comms workbook | distill | to-file/merge (brand/adoption; T4/T11) |
| thinking/app/brand-and-voice-notes.md [T4] | brand/voice/tagline reservoir (app-side) | merge | into the brand reservoir |
| assets/ (croft-wordmark-*.png + README) | the draft wordmarks (license-gated) | copy | to-file (brand assets; join the logo stub) |
| narrative/verticals/croft-the-name (pitch/adoption half) | the name story for a general audience | merge | with the essay/pitches |

## Layer 9 — activism/ (has: the "platforms author the relational field" research set)

| alpha source | conclusions | treatment | status |
|---|---|---|---|
| research/discord-dominance.md (harm half) | moderator-labor-as-captured-value; enshittification | merge | the harm reading -> activism (the descriptive -> fenced) |
| research/social-platform-cycle.md (harm/prescriptive half) | enshittification as a lifecycle; the indictment | merge | -> activism |
| the uncompensated-community-labor + data-opacity indictment (T35) | (the harm case) | covered | activism already carries the set; reconcile |

## Conclusion-completeness carry-forwards (the must-verify list, the Phase-3 gate operates on these)

From the 2026-06-25 Gap list A, re-keyed to layers. Each must land in a layer (or be verified-present) before
any theme is deleted:

- **CM-A1** relationships-as-the-newest-enclosure -> history (bridges to philosophy). Rank: highest.
- **CM-A2** recovery = lock (buildable) vs trust (unsolved) -> drystone-spec (identity/recovery open item) or impl. Rank: high; sits next to the bannered recovery-anchor gate.
- **CM-A3** the non-mimicry moat -> governance. Rank: medium-high.
- **CM-A4** linear-vs-cyclical operating systems -> philosophy. Rank: low (keep-vs-retire).
- **CM-P1** rollup trust trilemma + accumulator/MMR end-state -> drystone-spec open items. Rank: low.
- **CM-P2** the honest-claim formulation ("we do not promise convergence") -> drystone-spec. Rank: low-medium.
- **CM-P3** geer visibility-dial 3-rung enumeration -> drystone-spec/impl. Rank: low.

Note: some of these may already be present after the batch-driven spec maturation (document-pass-1..10); the
Phase-3 audit greps each before deciding fold-vs-covered.

## What stays alpha (not re-filed)

- `alpha/seeds/` (162): raw provenance, frozen.
- `alpha/plans/` (9): process artifacts (this ledger and the plan included).
- `README.md`, `ANALYSIS.md`, `COHESION.md`, `ROADMAP.md`, `ROADMAP_TODO.md`, `TEST-PLAN.md`,
  `NAMING.md` (index/process/naming surfaces), the fact-check logs: alpha-only by design.
- `BETA-ROLLUP.md`: reworked in place in Phase 3 (theme-keyed -> layer-keyed), not re-filed.
- `SOVEREIGN-COMMONS-DOSSIER.md`: provenance/raw; its durable conclusions (CM-A1, CM-A4) are pulled into
  layers; retire-vs-keep is a later call.
- `beta/thinking/raw/`: to inventory before Phase 2 (likely a staging dir; fold or remove).

## Open items surfaced by this disposition (for the user)

1. **history vs philosophy split of the enclosure material.** Proposed: material/cultural (croft, Clare,
   dry-stone, the commons-rebellion) -> history; the intellectual argument -> philosophy; CM-A1 in history
   with a philosophy cross-ref. Confirm.

2. **fenced vs activism split of the platform research.** Proposed: the descriptive map (caps, dominance
   mechanics, failure modes) -> fenced; the harm reading (captured labor, enshittification-as-indictment) ->
   activism. discord-dominance and social-platform-cycle split across both. Confirm.

3. **adoption material home.** p2p-founder-motivations-adoption and adoption-enablement read as both fenced
   (why incumbents win) and socialization (the adoption-chasm thesis, T11). Proposed: the descriptive
   why-incumbents-win -> fenced; the adoption strategy -> socialization. Confirm.

4. **spec/impl reconciliation depth.** 04/05/06 are mostly `covered`; Phase 2 for those layers is a
   verify-and-merge pass (grep each conclusion, fold only orphans), not net-new authoring. Confirm that light
   touch is right rather than a full re-distill of the mature spec.
