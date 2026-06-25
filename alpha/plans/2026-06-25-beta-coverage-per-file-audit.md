# Per-file alpha→beta coverage audit (close the coverage list to zero)

date: 2026-06-25

## Problem statement

The alpha→beta synthesis was checked once by a **surface-grouped content sweep** (the 2026-06-25 audit
that produced K1–K12 and T11–T17). A grouped skim is exactly how coverage gaps slipped through before (the
ledger was caught overstating `crystallized/principles.md` Tier-1 coverage). This audit is the **per-file**
completeness pass: walk *every* file in `discovery/alpha/` and give it an explicit, evidence-backed
disposition against `discovery/beta/`, so nothing settled was dropped, nothing unsettled was lost, and every
alpha file is accounted for exactly once.

## Approach

Fanned out one read-only agent per subtree (crystallized · thinking-top · thinking/app+drystone-spec ·
research · narrative+dossier · top-level/process+plans · seeds), all on the session's primary model (Opus),
each adversarially verifying every FOLDED claim by opening the named `beta/NN §X` rather than trusting
`BETA-ROLLUP`'s tables. The primary then independently re-verified the highest-value new
CONCLUSION-MISSING finds with direct `grep` against `beta/0*.md`. 165 files dispositioned.

## Reasoning

The 2026-06-25 content sweep already drained the obvious settled-but-unfolded bucket (K1–K12) and staged
the unsettled bucket (T1–T17). A per-file pass cannot re-find those; its value is the **long tail** — the
single dropped conclusion inside an otherwise-folded file, the partial fold where a headline landed but a
load-bearing nuance was flattened, and the file that nobody claimed at all. Those are exactly the finds
below.

## Verdict (summary)

- **165 files dispositioned, each exactly once.** No file is unaccounted for.
- **Every FOLDED spine independently verified** at its named `beta/NN §X`. The `principles.md` Tier-1
  "drained" claim **holds adversarially** (every Tier-1 principle + the deeper-foundation block walked and
  found in 01/03/06/07/08). The K9 venue-map landed **intact** (not flattened) at 07 §C4. The K12 invariant
  landed at **08 §10** (note: a couple of internal references say "§6" — a citation nit, the content is at §10).
- **4 genuinely-absent settled conclusions** found (CONCLUSION-MISSING — fold candidates, **awaiting your
  approval**; not auto-folded). **3 partial folds** found (headline/discipline present, a sub-nuance
  flattened — lower priority, arguably alpha-detail).
- **3 new unsettled threads** found and **staged** as **T18 / T19 / T20** in `beta/OPEN-THREADS.md`
  (additive, safe); plus a provenance augmentation to **T4**.
- **Tier-cleanliness re-confirmed** (no beta theme doc edited; grep clean).
- **No decision gate resolved.**

---

## Master matrix (one row per alpha file · disposition · evidence · action)

Disposition codes: **FOLDED** · **CITED** (cited-not-folded, alpha-only by design) · **PROVENANCE/RAW** ·
**PROCESS/INDEX** · **STAGED** (`Tn`) · **EXCLUDED** (do-not-carry) · **CM** (conclusion-missing) ·
**TM** (thread-missing) · **DEFERRED**. Action `—` = no action (correctly dispositioned, stays alpha or
verified folded).

### crystallized/ (7)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| crystallized/principles.md | FOLDED | Tier-1 drain verified: capture-not-centralization→01 §6.3; recurring-inversion→01 §6.2; capabilities-not-rights/planes→01 §6.1; anti-fragile/pay-keepers/sunset→07 A2–A5; crowding-out→07 A2b; free-tier→08 Charter; per-tier matrix/transparent-offline/no-operator→03 §5; deeper-foundation block→01 §1/§2.5/§3/§4/§5/§6 | — (Tier-2 CITED backs 04; Tier-3 LTS→**TM/T18**) |
| crystallized/conclusions.md | FOLDED | conclusions live across 04/01/03; M0–M4 + M3 consumer-pull → **STAGED T12** | — |
| crystallized/proof-ledger.md | CITED | 04 §3 results table + §5 flags derive from it (I1–I10, cross-machine, 66/0, checkpoint) | — |
| crystallized/test-narrative.md | CITED | the "what each proof means" layer behind 04 §3/§5; AR-5 broadcast-tier rule confirmed in 04 §5 | — |
| crystallized/conformance-suite.md | CITED | 04 §3 cites 66/0 + "black-box suite a second impl must pass"; CROFT-PROTOCOL §12 contract | — |
| crystallized/TEST-CORPUS.md | PROCESS/INDEX | cross-repo test catalog ("the catalog, not the status tracker") | — |
| crystallized/CROFT-PROTOCOL.md | CITED | Drystone wire spec; 04 §2/§5 synthesizes structure + carries proof flags; K7 substrate-model verified 04 §2 | — |

### thinking/ (top level, 27)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| thinking/abuse-resistance-and-the-rave-trap.md | FOLDED | 06 §1/§2/§10/§11 (rave-trap, Signal-vs-Telegram, fork blast-radius) | — |
| thinking/atproto-atmospheric-web.md | FOLDED | 03 §6 (demand-side argument for Croft's crypto) | — |
| thinking/cooperative-social-union-model.md | FOLDED | 07 A6–A7 + B6 (four-pillar Social Union; failure lineage) | — (growth-apathetic moat → **CM-3**) |
| thinking/cross-platform-identity-provenance.md | FOLDED | 05 §5/§6/§8 (hub-and-spoke attestation; convergence hedge); per-platform doc → STAGED T6 | — |
| thinking/design-notes-addendum.md | FOLDED | K1→04 §3/§5; K4→06 §7; K6→01 §6.1 all verified | — (trilemma → **CM-1 partial**; legibility-guarantee → **CM-2 partial**) |
| thinking/experiment-suite.md | PROCESS/INDEX | test spec (INV-* assertions); proofs it specs landed in 04 via proof-ledger | — (CITED secondary) |
| thinking/failed-op-response.md | FOLDED | 06 §4 (LOUD/SILENT/BLACKHOLE dial; immune signal; k-corroboration) | — |
| thinking/foundation-and-ip-stewardship.md | FOLDED | 07 B1–B5 (three layers; AGPL+DCO; two-tier mark; entity phasing) | — |
| thinking/freshness-signal.md | FOLDED | 06 §5 (reasoned socially; MEMBERSHIP-FRESH; fresh-but-wrong boundary) | — |
| thinking/geer-gating-peer.md | FOLDED | 06 §3 (consented gating; labels-not-enforce; compellability) | — (3-rung enumeration → **CM-4 partial**) |
| thinking/governance-and-survivability.md | FOLDED | 07 A8 (bankruptcy-remote steward + pre-funded archive runway) | — (archive mechanics CITED) |
| thinking/group-privacy-lanes-design-note.md | FOLDED | 06 §10 (three-lane routing; `public:true` on MLS is a contradiction) | — |
| thinking/interaction-tiers.md | FOLDED | 08 §5 ("three products, one send button"; visible-cost privacy) | — |
| thinking/ios-opportunistic-p2p.md | STAGED (T14) | four-property impossibility folded 03 §1/§9; iOS-background-limitation → T14 | — |
| thinking/local-first-as-design-imperative.md | FOLDED | K6→01 §6.1, K7→04 substrate-model; premise/theorem/rights-floor → 01 §3/§5/§6 | — (recovery two-tiers → **CM-5**; search substrate → **TM/T19**) |
| thinking/meer-superpeer-design.md | FOLDED | K8→06 §1 (Tier 0/1/2/no-mirror dial, NOT flattened to binary); §7 anti-entrenchment | — |
| thinking/membership-vs-access-the-public-door.md | FOLDED | 06 §8 + 08 §8 (membership≠access; public door; Sybil softening) | — |
| thinking/merge-split-corpus.md | FOLDED | 04 §3 (split/merge/conflict taxonomy → conformance) | — (conflict-reason gaps C4/C7/C8/C9/C10 → **TM/T20**) |
| thinking/model-holds-up-summary.md | PROCESS/INDEX | research-pass scorecard; soften-no-referee→03 §1/04; sleeper-risk→K1 | — (CITED secondary) |
| thinking/multi-device.md | FOLDED | 05 §1/§2/§7 (keys≠identity; thresholds count lineages; §10.1 recovery decision) | — |
| thinking/open-considerations.md | FOLDED + STAGED | §1/§6→08; §2 recovery→05/04; §4→T2; §5/§9→T3; §7 dedup→T13; §8→07 | — (§3 Automerge-audit = borderline-ROADMAP per OPEN-THREADS) |
| thinking/open-edges.md | PROCESS/INDEX | triage surface; settled items landed (MEMBERSHIP-FRESH/ADMIN-FLOOR), unsettled seed T-threads | — |
| thinking/plc-identity-resilience.md | FOLDED | 05 §3/§4/§8 (DID-method scorecard; validating PLC read-replica; archive resizing) | — (replica internals CITED) |
| thinking/realtime-media-over-iroh.md | FOLDED | 04 §4 (media rides substrate blind; str0m/RoQ/MoQ); residual → STAGED T10/T13 | — |
| thinking/revocation-authority.md | FOLDED | 06 §6 (threshold dial; ADMIN FLOOR; never-irrevocable ladder; capture≠brick) | — |
| thinking/social-layer.md | FOLDED | 06 §9 (S1–S5; S5 Twitter-Circles structural-not-runtime); S3 unsolved surfaced | — (Google+ lesson intentionally unfolded) |
| thinking/thesis-lineage-groups.md | FOLDED | 04 §1/§2/§3/§5 (thesis; two-tree; survivor reconnect; I1–I10; honesty boundaries) | — |

### thinking/app/ (15)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| thinking/app/README.md | PROCESS/INDEX | the "untangle" index; open-risks surface in 08 banner + §6/§9 | — |
| thinking/app/design-philosophy.md | FOLDED | 08 §1 (garden thesis), §2 (honest seams), §3 (FCIS spine), §6 (craft rule) | — (§13 games data-layer → STAGED T15) |
| thinking/app/client-architecture-adr.md | FOLDED | 08 §3 (one core + thin shells; Crux slim), §4 (option-C; awareness/interactivity) | — |
| thinking/app/design-criteria.md | FOLDED | 08 §6 (quality bar; WeChat skeleton stripped; visual system) | — (palette role detail abstracted, not load-bearing) |
| thinking/app/brand-and-voice-notes.md | STAGED (T4) | brand/voice/tagline reservoir; 08 banner says reconcile DRIFT before any brand chapter | **add to T4 provenance** (app-side half) |
| thinking/app/build-specs/BUILD-SPEC.md | CITED | 08 §3 carries Phase-0 green-real 20/20 + the 5 DECISIONS proof status | — |
| thinking/app/build-specs/BUILD-SPEC-PHASE-1-2.md | PROCESS/INDEX | Phases 1–2 build-intent (Leptos/Tauri/tokens); forward build plan | — |
| thinking/app/ponds/apps-pond-utility-list.md | CITED | utilities catalog; 08 §7 folds pathways/leverage not the catalog | — |
| thinking/app/ponds/build-order.md | FOLDED | 08 §7 (six-phase; resolver tier-zero; fair-reveal leverage) + §9 (E11 resolver) | — |
| thinking/app/ponds/build-shape-pass.md | CITED | license table + schema detail; "local voting = governance infra" bridge → 08 §7 | — |
| thinking/app/ponds/fair-reveal-primitive-spec.md | CITED | commit-reveal spec; 08 §7 folds the primitive's existence/leverage, not the crypto | — |
| thinking/app/ponds/games-pond-authoritative-list.md | FOLDED | 08 §7 (three inclusion pathways: build-fresh/wrap/port) | — |
| thinking/app/ponds/on-device-llm-feasibility.md | FOLDED (K12) | 08 §10 (detect-first, accelerate-never-gate, every path reachable without it) | — |
| thinking/app/ponds/p2p-games-pond-launch-set.md | CITED | the earlier games hunt; superseded by authoritative-list (which IS folded) | — |
| thinking/app/ponds/webxdc-security-and-competitive-games.md | FOLDED | 08 §6 (Cure53/CSP; disable-WebRTC; hard context boundary; per-match pseudonym) | — |

### thinking/drystone-spec/ (3)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| thinking/drystone-spec/README.md | STAGED (T1) | DRAFT index; lists open items E30/A11/A12/A13; not folded into CROFT-PROTOCOL | — |
| thinking/drystone-spec/section-2-peers-rights-capabilities.md | STAGED (T1) | one-kind-of-peer; rights vs capabilities; meer-as-PeerSet — NOT asserted in beta | — |
| thinking/drystone-spec/section-x-governance-conflicts.md | STAGED (T1) | append-only fold; timestamp-free order; frontier-closure §X.8.5 open; Matrix-CVE contrast | — |

### research/ (16)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| research/README.md | PROCESS/INDEX | research-corpus index, companion to ECOSYSTEM | — |
| research/messaging-solutions-landscape.md | FOLDED | 03 §1 (universal trade, verbatim) + §2 (field map) + §5 (differentiators) | — |
| research/public-social-protocols.md | FOLDED | 03 §3 (X/Bluesky/Threads/Mastodon/Pixelfed; three poles) + §4 (dual-use DID) | — |
| research/germ-xchat-features.md | FOLDED | 03 §5 (privacy-free/convenience-effortful inversion, verbatim) + §6 (closest cousin) | — |
| research/atproto-private-data-architecture.md | FOLDED | 03 §6 (real Private Data WG; defers true E2EE/ZK; Croft on harder ZK side) | — (forward-tracking → STAGED T7) |
| research/atproto-sovereign-appview-club.md | FOLDED | 03 §8 (sovereign-AppView read-side; Twitter Circles→S5) | — |
| research/discord-dominance.md | FOLDED | 03 §7 (zero-friction, S-1, enshittification) + 07 B6 (moderator-labor-as-captured-value) | — |
| research/social-platform-cycle.md | FOLDED | historical→02 §6; prescriptive→07 A1/A2/A8/B6 (both halves verified) | — |
| research/iroh-realtime-media-references.md | CITED | media-layer reference backing 04 §4; iroh facts cite FACTCHECK | — (open residual → STAGED T10) |
| research/str0m-production-readiness.md | CITED | gates challenge C2 in realtime-media; backs 04 media leg | — (str0m ICE `[OPEN]` → STAGED T10) |
| research/open-publication-and-ip-protection.md | FOLDED (K9) | 07 Pillar C (CC-BY 4.0 doc + Apache-2.0 code; prior-art-first; IETF I-D then arXiv) | — |
| research/socialization-and-publication-venues.md | FOLDED (K9) | 07 §C4 per-layer venue map **intact** (transport/crypto/DID/social/data/flagship) | — (academic-venue table abstracted, not load-bearing) |
| research/group-chat-failure-modes.md | STAGED (T5) | survivor-determinism, covert-ordering, genesis amendability, churn-fold Achilles heel | — |
| research/group-chat-failure-modes-plain.md | STAGED (T5) | plain-English twin (T5 provenance names it) | — |
| research/p2p-founder-motivations-adoption.md | STAGED (T11) | adoption-chasm; only-Signal-crossed; institutional-mandate fourth bridge | — |
| research/discord-matrix-groupchat.md | STAGED (T16) | Matrix E2EE operational lessons (UTD invariant, mandatory-recovery onboarding, expectation-gap) | — |

### narrative/ + verticals/ + dossier (7)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| narrative/lineage-of-a-design-imperative.md | FOLDED | 01 §2.1–2.6 (2,400-yr arc, quotes preserved whole + flags); razor/premise → 01 §1/§3/§5 | — (toolkit + reinforcements → DEFERRED, tracked in BETA-ROLLUP 01) |
| narrative/verticals/croft-the-name-and-the-commons.md | FOLDED | 02 §1/§2/§3/§4/§5/§7 (etymology; inversion; Clare; commons-rebellion; four-axis alignment) | — |
| narrative/messaging-and-quotes.md | STAGED (T4) | brand/voice reservoir; K11 (Gneezy & Rustichini) crossed → 07 A2b; rest STAGED | — |
| narrative/long-form.md | PROCESS/INDEX | synthesis skeleton; conclusions folded across beta; adoption-curve risk → STAGED T11 | — |
| narrative/short.md | PROCESS/INDEX | 3-page cut of long-form; no unique settled conclusion | — |
| narrative/verticals/README.md | PROCESS/INDEX | verticals directory index (only #7 drafted+folded; #1–6 unwritten stubs) | — |
| SOVEREIGN-COMMONS-DOSSIER.md | PROVENANCE/RAW | pre-Croft umbrella; durable content distributed → K2/K3/K5/K10 + 07 verified | — (renting-relationships → **CM-narrative-1**; linear-vs-cyclical → **CM-narrative-2**; adoption → STAGED T11) |

### top-level index / process / plans (16)

| path | disposition | evidence / beta-location | action |
|---|---|---|---|
| README.md | PROCESS/INDEX | repo map + conventions | — |
| ANALYSIS.md | PROCESS/INDEX | 2026-06-15 corpus map; through-lines landed in 04 | — |
| COHESION.md | PROCESS/INDEX | seam-tracker (37 §); resolved seams map onto K1–K12 / T1–T17 | — |
| ECOSYSTEM.md | FOLDED | field register → 03 §2/§3/§6/§7/§9 verified | — (§8 cooperative register → CITED; BETA-ROLLUP §8-routing nit, see notes) |
| NAMING.md | FOLDED | Croft+Drystone→02 §7 + §3 (Princeps Problem); Noria→07 B5; reservoir sweeps EXCLUDED | — |
| ROADMAP.md | PROCESS/INDEX | 2026-06-15 milestones; M0–M4 → STAGED T12; item-15 sustainability→07 | — |
| ROADMAP_TODO.md | PROCESS/INDEX | provenance-indexed backlog; items map to gates/T-threads; no settled-closed item beta lacks | — |
| TEST-PLAN.md | CITED | proof-sequencing artifact backing 04's flags (peer to proof-ledger) | — |
| ROUND-2026-06-17-media-meer-conformance.md | FOLDED | 04 §4 (E10 characterized; E12 green-real; E11 characterized; meer P0+P1 blind) | — (residual → STAGED T10) |
| BETA-ROLLUP.md | PROCESS/INDEX | the alpha→beta trace ledger (sole home of source→landing map) | **edited** (record CMs + close coverage view) |
| assets/README.md | PROVENANCE/RAW | brand-asset manifest (draft wordmarks, license-gated) | — (brand chapter → STAGED T4) |
| plans/2026-06-22-narrative-architecture-refactor-proposal.md | PROCESS/INDEX | the N1–N8 precursor to the eight beta themes | — |
| plans/narrative-architecture-refactor-PROMPT.md | PROVENANCE/RAW | handoff prompt that spawned the proposal | — |
| plans/2026-06-24-beta-factcheck-report.md | PROCESS/INDEX | pass-1 fact-check (0 blockers, 6 MAJOR, 16 MINOR) | — |
| plans/2026-06-24-beta-factcheck-corrections-log.md | PROCESS/INDEX | correction-accounting log; confirms alpha stayed frozen | — |
| plans/2026-06-24-beta-factcheck-pass2-report.md | PROCESS/INDEX | pass-2 re-audit (GO; all 7 resolved 2026-06-25) | — |

### seeds/ (74)

**transcripts (top, 2) + raw index/manifest:**

| path | disposition | evidence | action |
|---|---|---|---|
| seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md | PROCESS/INDEX | provenance-status inventory | — |
| seeds/transcripts/design-dialogue-2026-06-13-to-14.md | PROVENANCE/RAW | "richest single seed"; settled content lifted → thinking/ → 04/05/06 | — |
| seeds/transcripts/raw/README.md | PROCESS/INDEX | provenance archive index | — |

**FACTCHECK source-of-truth files (11) — all CITED-NOT-FOLDED (beta cites, does not re-verify):**

| path | evidence |
|---|---|
| raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md | **primary atproto/iroh SoT** (iroh 1.0.0; range-set recon not MSTs); cited by 03/05/08 |
| raw/atproto-architecture-appview-relay-explainer-2026-06-22-FACTCHECK.md | backs 03; defers to primary SoT |
| raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22-FACTCHECK.md | backs 03 §6 / T7 (real Private Data WG) |
| raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22-FACTCHECK.md | backs 03 §8 (sovereign appview; Circles→S5) |
| raw/cooperative-social-union-governance-dialogue-2026-06-22-FACTCHECK.md | MO §351 SoT (NOT-LEGAL-ADVICE); backs 07; $/statute = do-not-carry |
| raw/croft-drystone-protocol-naming-dialogue-2026-06-22-FACTCHECK.md | backs 02 §4; "Skartsia and Tomi" REFUTED |
| raw/crypto-wars-to-p2p-pds-economics-FACTCHECK.md | backs 01 §5 (Hush-A-Phone CONFIRMED; Zimmermann/Meyer/Voskop REFUTED) |
| raw/groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22-FACTCHECK.md | backs 02 §6; Steem HF23=$6.3M not $5M (do-not-carry) |
| raw/iroh-quic-localfirst-ecosystem-dialogue-2026-06-22-FACTCHECK.md | iroh SoT; defers to primary |
| raw/opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22-FACTCHECK.md | backs 03 (Neynar→Farcaster Jan 2026 CONFIRMED) |
| raw/solid-pds-webid-scalingtrust-dsnp-dialogue-2026-06-22-FACTCHECK.md | backs 03 (Solid/WebID/DSNP) |

**raw dialogues + PR seeds (33) — all PROVENANCE/RAW (frozen; settled content already lifted to the synthesis layer):**

`raw/atproto-architecture-appview-relay-explainer-2026-06-22.md`,
`raw/atproto-atmospheric-web-iroh-mobile-dialogue.md`,
`raw/cooperative-social-union-governance-dialogue-2026-06-22.md`,
`raw/croft-app-design-dialogue-2026-06-20-to-22.md`,
`raw/croft-app-ponds-games-dialogue-2026-06-20-to-22.md`,
`raw/croft-app-portdecision-review-2026-06-21.md`,
`raw/croft-architecture-design-dialogue-2026-06-20.md` (01 footer),
`raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md`,
`raw/croft-atproto-sovereign-appview-open-social-dialogue-2026-06-22.md`,
`raw/croft-clare-enclosure-poems-2026-06-23.md` (→02 §3/§5),
`raw/croft-crofting-narrative.md` (→02 §4/§5, all `[UNVERIFIED]`),
`raw/croft-crofting-research.md` (→02 §3/§4/§7),
`raw/croft-discord-money-ipo-onboarding-dialogue-2026-06-22.md`,
`raw/croft-drystone-protocol-naming-dialogue-2026-06-22.md`,
`raw/croft-etymology-enclosure-tradition-dialogue-2026-06-23.md` (→02 §1/§2/§5),
`raw/croft-foundation-coop-ip-naming-dialogue-2026-06-23.md` (→07),
`raw/croft-identity-provenance-dialogue-2026-06-20.md` (→05 §5 quotes),
`raw/crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` (→01 §5; EXCLUDED set lives here),
`raw/drystone-peers-rights-governance-matrix-dialogue-2026-06-24.md` (**T1 provenance**),
`raw/germ-xchat-design-dialogue.md` (→03 §5/§6),
`raw/groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` (mostly excluded),
`raw/iroh-quic-localfirst-ecosystem-dialogue-2026-06-22.md`,
`raw/iroh-realtime-media-rave-trap-scaled-usecases-dialogue-2026-06-22.md` (→04/06),
`raw/opensocial-nostr-farcaster-aggregators-dialogue-2026-06-22.md`,
`raw/p2p-architecture-origin-dialogue.md` (maintenance-phase fragment→07),
`raw/solid-pds-webid-scalingtrust-dsnp-dialogue-2026-06-22.md`,
`raw/pr3-encrypted-local-first.md`, `raw/pr4-public-roundtrip.md`,
`raw/pr5-encrypted-blob-share.md` (couples T13), `raw/pr6-appview-validation.md`,
`raw/pr7-android-p2p.md`, `raw/pr8-lineage-groups.md`, `raw/pr9-lineage-group-model.md`.

**generated-prompts (9) — README is PROCESS/INDEX; the other 8 are PROVENANCE/RAW (frozen handoff prompts, disposition #3):**

`generated-prompts/README.md` (INDEX); `achilles-heel-research-prompt.md`,
`beta-coverage-per-file-audit-prompt.md` (this audit's own driver — meta),
`beta-factcheck-pass2-prompt.md`, `beta-factcheck-prompt.md`,
`beta-synthesis-discovery-prompt.md`, `file-transcripts-prompt.md`,
`games-pond-research-prompt.md`, `structural-tests-visibility-regimes-prompt.md`.

**unpacked seed duplicates (18) — all PROVENANCE/RAW (frozen; byte-identical or abridged ancestors of canonical twins; canonical is equal-or-richer, so no unique content lives only in a seed):**

apps-unpacked (8, byte-identical to `thinking/app/ponds/*`): `apps-pond-utility-list.md`,
`build-order.md`, `build-shape-pass.md`, `fair-reveal-primitive-spec.md`,
`games-pond-authoritative-list.md`, `on-device-llm-feasibility.md`,
`p2p-games-pond-launch-set.md`, `webxdc-security-and-competitive-games.md`.
multiecosystemapp-unpacked (6): `design-philosophy.md`, `design-criteria.md`, `BUILD-SPEC.md`,
`BUILD-SPEC-PHASE-1-2.md` (byte-identical to `thinking/app/*`); `brand-and-voice-notes.md`
(canonical has 7 extra lines); `games-pond-research-prompt.md` (dup of the generated-prompt).
groupdynamics-unpacked (4, 2026-06-13 origins; canonical twins equal-or-richer):
`THESIS.md`→thinking/thesis-lineage-groups→04; `MULTI_DEVICE.md`→thinking/multi-device→05;
`SOCIAL_LAYER.md`→thinking/social-layer (canonical added S5)→06 §9;
`messaging-solutions-landscape.md`→research/messaging-solutions-landscape→03.

---

## Gap list A — CONCLUSION-MISSING (settled; belongs in a theme; **fold candidates awaiting your approval**)

These are presented for your decision. **No beta theme doc was edited.** If you approve a fold, it is done
per-theme as clean beta narrative (no prior-tier links; verification flags only where the source has them),
exactly as K1–K12 were, and a `BETA-ROLLUP` trace row is added.

### Genuinely absent (primary independently grep-confirmed ABSENT from beta) — strong candidates

**CM-A1 — "Connection itself is the newest enclosure — platforms rent our relationships back to us."**
- *Conclusion:* the present-day enclosure target is human relationship / the third place, not land or IP —
  "make even communication and connection a product sold to you at a premium." A sharp civic framing that
  bridges the historical enclosure arc (02 §1–5) to *why a social/messaging product specifically* is the
  answer.
- *Lives in:* `SOVEREIGN-COMMONS-DOSSIER.md` §2 / §12 ("renting our relationships back to us").
- *Target:* **02 §6** (which today stops at "collectively created value, privately captured" and never names
  the relationship-as-the-enclosed-commons turn).
- *Verification:* grep of `beta/0*.md` for renting / relationship-back / third place / connection-itself →
  **nothing.** Genuinely absent. **Rank: highest.**

**CM-A2 — Recovery is two separable tiers: the lock (buildable now) vs the trust (unsolved-in-general).**
- *Conclusion:* recovery splits into Tier-1 **the lock** (mechanism — Shamir/threshold shares, sealed,
  release predicate, optional timelock — *buildable now*, predicate should be threshold across independent
  trust domains, never a single gate) and Tier-2 **the trust** (who/when release is legitimate — the
  genuinely-unsolved social problem; TLS cert-chain-vs-issuer analogy). Converts beta's "recovery is the
  open problem" into the more precise "the mechanism is shippable; only the trust predicate is undecided."
- *Lives in:* `thinking/local-first-as-design-imperative.md` (the delegate-primitive section).
- *Target:* **05 §7** (or 04 carried-open).
- *Verification:* grep of 05/04 for the lock/trust split, "buildable now," "social problem" → **nothing.**
  Genuinely absent. **Rank: high** (it materially sharpens the corpus's top open decision — and sits next to
  the bannered recovery-anchor gate without resolving it).

**CM-A3 — The "non-mimicry moat": non-extraction unlocks a feature class extractive competitors structurally cannot ship.**
- *Conclusion:* the affirmative competitive case (vs 07's defensive anti-rug-pull case) — protocol-level
  ad-blocking, open-book financials, community-voted treasury, non-algorithmic feeds, maintenance-first
  budgeting, user-controlled data TTL, digital-legacy trusts, right-to-repair forkability — are downstream
  consequences of the cooperative structure that competitors *cannot* mimic without unwinding their model.
  Individual items may be roadmap-grade; the **unifying claim** is the settled conclusion.
- *Lives in:* `thinking/cooperative-social-union-model.md` (growth-apathetic / non-mimicry features).
- *Target:* **07** (a short subsection under Pillar A) or **08**.
- *Verification:* grep of 07/08 for non-mimicry / growth-apathetic / structurally-cannot-ship / transparency-
  as-a-feature / right-to-repair / protocol-level-ad → **nothing.** Genuinely absent. **Rank: medium-high.**

**CM-A4 — The linear/extractive vs cyclical/relational "operating systems" frame.**
- *Conclusion:* the deepest civic "why" — two opposed value systems (time, nature, waste, value-of-a-person,
  cognition), with the thesis choosing the cyclical/relational one. A *third* framing of the "why" that
  neither 01 (Ashby/Hayek/Scott) nor 07 (Ostrom/rug-pull) carries.
- *Lives in:* `SOVEREIGN-COMMONS-DOSSIER.md` §3.1 / §12.
- *Target:* **01** or **07**.
- *Verification:* grep → **nothing.** Absent, but reads more as manifesto than load-bearing conclusion —
  **rank: low**; best treated as a keep-vs-retire decision when the dossier's retire-vs-keep call is made.

### Partial folds (headline/discipline already in beta; only a sub-nuance/framing flattened — lower priority, arguably alpha-detail)

**CM-P1 — Roll-up trust *trilemma* + the accumulator/MMR end-state.** `design-notes-addendum.md` §2 frames
three ways to trust a checkpoint (authority-signed = the referee trap, consciously avoid; threshold-signed;
accumulator/MMR = trusted-party-free, heaviest, the direction to build toward). **Beta 04 §3 already lands
the answer and the discipline** ("single-authority/broker checkpoint is rejected," "the broker is no finality
authority," threshold-signed quorum). What is absent is only the explicit *three-way framing* and the
*accumulator/MMR as a named future direction*. Target if folded: a sentence in **04 §5** open-items. Low.

**CM-P2 — The positive "guarantee actually made" sentence.** `design-notes-addendum.md` §6/§8: the guarantee
is *not* "the group converges to one membership" but "every party can always see a true, attributable
history, and always has a clean exit"; the named failure mode is "claiming convergence while delivering
legibility." **Beta 04 already carries the "social-legibility invariant" as a first-class invariant** (lines
17/32/98). What is flattened is the explicit *honest-claim formulation* ("we do not promise convergence").
Target if folded: **04 §1 or §5**. Low–medium.

**CM-P3 — Geer visibility-dial 3-rung enumeration.** `geer-gating-peer.md`: rung-1 report-gated (geer holds
no key — *the only rung that shrinks compellability*), rung-2 classifier-gated (client-side scanning
territory), rung-3 full-key Tier-2. **Beta 06 already says "scoped to the least-invasive rung" and "other
rungs remain compellable"** (lines 137/301). What is absent is only the explicit rung *enumeration*. Target
if folded: **06 §3**. Low (the load-bearing point is present; this is detail).

### Not a gap (recorded for ledger accuracy)

- **ECOSYSTEM.md §8 cooperative prior-art register** (Ostrom/Stocksy/Mondragon/Packers/credit-union/Drivers/
  Resonate/Social.coop/PCC/SPI/SFC/Aspiration). 07 carries the *operative* subset it needs (failure lineage
  in B6; mark-holding model in B1/B5); the broader homage register is correctly **CITED-NOT-FOLDED** (the
  register stays alpha by design). The only defect is a `BETA-ROLLUP` routing pointer: the 03 table tags
  `ECOSYSTEM.md → synthesized → 03`, but the cooperative §8 material is cited by **07's** reasoning, not 03's
  body. Noted in BETA-ROLLUP's coverage view; not a coverage gap.

---

## Gap list B — THREAD-MISSING (unsettled; **staged** as T18–T20, additive)

These were staged directly in `beta/OPEN-THREADS.md` (a process artifact; safe additive action).

**T18 — LTS-for-interfaces / shapeability-paired-with-stability.** `principles.md` Tier-3 carries a
settled-as-stance principle absent from beta: "shapeability is only valuable paired with stability; constant
UI change is quietly extractive," with a concrete LTS-for-interfaces mechanism (alpha/beta/stable channels,
~3yr stable window, opt-in change trained on a ~6mo cadence, security the over-communicated exception). The
product-layer twin of the non-extraction thesis. **Distinct from T17** (which scopes the three-audiences
settings model + composable-interface ramp). Target: 08. Gate: decide LTS channel/cadence as a commitment
vs aspiration; name the documentation/support cost.

**T19 — Blind-peer encrypted-search / coverage-attestation substrate.** `local-first-as-design-imperative.md`
lands a substantial unbuilt design: blind peers expose the hash-tree skeleton (not payload); search is a
bounded subtree scan where the hash tree is the shard map and the gather is cryptographically attestable
(coverage as a checkable set-cover over hashes); the two offload "animals" (HA-search-member vs
crown-jewel search-mediator); encrypted-search leakage profiles ("you pick a leakage profile, not avoid
one"). Author flags content-predicate search-coverage attestation as a genuinely-new seam wanting its own
threat model. Target: 04 (substrate capability) or a search/discovery theme. Gate: write the threat model;
the honest-plaintext-evaluation half is the hard piece.

**T20 — Conflict-reason corpus gaps (C4/C7/C8/C9/C10).** `merge-split-corpus.md` §4 enumerates five real,
unmodeled/partial reconcile-semantics gaps not in any thread: C4 add-vs-add of the same person on different
device keys across a partition (fold by lineage, not double-count); C7 dissolve-vs-continue (undefined); C8
diamond-recombine over a multi-parent DAG (topology proven, conflict-detection untested); C9 equivocation
hardening (partial); C10 ban-evasion re-add via a new device leaf (must not silently re-confer standing).
Target: 04 (widens "what was proved" toward the full conflict space). **Overlaps T5** (scale/churn) but is a
distinct surface (reconcile semantics); confirm-and-fold-where-subsumed noted in the thread.

**T4 provenance augmentation (not a new thread):** added `thinking/app/brand-and-voice-notes.md` and
`thinking/app/assets`/`assets/README.md` brand drafts to T4's alpha provenance — the app-side half of the
brand DRIFT, so T4 doesn't lose it.

**Borderline (left in their existing homes, no action):** the "embedded-trust" corollary (reads settled but
co-travels with the unverified 16-project survey → kept in T11); the Automerge-over-application audit and the
core-crypto GPL-3.0-vs-openmls/mls-rs license decision (already named in OPEN-THREADS' closing note as
"borderline engineering, likely ROADMAP not a beta thread").

---

## Coverage view — closed

Every alpha source listed in `BETA-ROLLUP`'s prior "coverage view" is now closed to either **folded → beta §**
or **alpha-only by design (reason)**, and the new CONCLUSION-MISSING finds are recorded there. See the updated
`alpha/BETA-ROLLUP.md` → "Coverage view." The remaining open work is your fold-approval decision on Gap list A.

## Tier-cleanliness — re-confirmed

No beta theme doc (`beta/0*.md`) was edited by this audit. The required grep returns nothing (see run log).
Edits were confined to process artifacts: `beta/OPEN-THREADS.md` (which may point into alpha by design) and
`alpha/BETA-ROLLUP.md` (the prior-level ledger).

## Decision gates — untouched

No gate was resolved (MPL license; recovery anchor; cooperative legal review; Noria name; CroftC Phase-0 IP;
genome-vs-strategy; capability-mechanism Track A/B; key-custody default). CM-A2 (recovery two-tiers) sits
adjacent to the recovery-anchor gate but only sharpens the *mechanism-vs-trust* distinction; it does not pick
the anchor.

## Definition of done — checklist

- [x] Every file under `discovery/alpha/` (165) appears exactly once in the master matrix with disposition + evidence.
- [x] All FOLDED claims independently verified against the named beta section (not trusted from the ledger).
- [x] New threads staged (T18–T20) + T4 provenance augmented.
- [x] New conclusions listed for approval (Gap list A) — beta themes **not** silently edited.
- [x] `BETA-ROLLUP` coverage view closed to zero-or-explicitly-alpha-only.
- [x] Tier-cleanliness re-confirmed.
- [x] No gate resolved; nothing committed/pushed.
