# discovery / beta / impl: Layer 5 (reference implementation, experiment-informed)

date: 2026-07-06

**What this layer is.** Layer 5 of the beta layer-cake: the reference-implementation and
experiment-informed design that sits between the protocol spec (Layer 4, `drystone-spec/`) and the
product (Layer 6, `croft/`). It is where design work that is grounded in real experiments and real
libraries lives while it matures, some of it destined to fold back up into the spec, some of it staying
as implementation guidance. Themes `04` / `05` / `06` (the protocol-we-proved, identity, safety) migrate
here as they settle.

## Contents

### `doc-writing-method.md`: the shared design-doc writing method

The single canonical writing method both corpora below follow (end-state rule, layer separation, epistemic
tags, the posture-summary-table practice, and the search-first / quote-discipline / no-orphaned-concepts
rules). It lives at the layer root, not inside a bundle, because it is shared. Where a corpus doc refers to
"doc 11" or "the method," it means this file. (It was formerly `delivery-layer/11-doc-method.md`; promoted
here 2026-07-06 as the single source of truth when the MLS bundle arrived with a newer copy.) A **§17
"Diagrams: author once, render to text and vector"** section was added 2026-07-07: author diagrams as
Mermaid or grid-clean ASCII and generate both a text/ASCII form (terminal-legible, archival, matching the
RFC discipline) and an SVG, rather than downscaling a vector into text.

### Reference notes (added 2026-07-07)

Grounded reference notes for the reference core, distilled from filed raw transcripts:

| doc | what it is |
|---|---|
| `transport-iroh-gossip-and-quic.md` | How the transport/delivery layer works, corroborating spec §6 (gossip overlay §6.10) and §11.9: control plane vs data plane (NeighborUp/Down = local routing-table membership; Received = the only data event; Lagged = backpressure); gossip has no verified delivery, so peer state comes from the QUIC transport layer (keep-alive heartbeats, not a gossip ping); the one-time TLS 1.3 handshake derives session keys and keep-alives reuse that verified trust; connection migration keeps the peer table stable. iroh core 1.0.0 (FACTCHECK SoT); iroh-gossip pre-1.0 specifics `[confirm]`. |
| `references-designing-data-intensive-applications.md` | Design-guidance notes on Kleppmann's DDIA: the three imperatives (reliability/scalability/maintainability), B-Trees vs LSM-Trees, non-uniform ACID isolation, hostile distributed systems, the Twitter fan-out and doctors-on-call examples, and the mapping onto Drystone's delivery-as-race, history-DAG / governance-log, and commit-liveness posture. Quotes [UNVERIFIED]. |

### `mls/`: the MLS-substrate understanding bundle

A self-contained study of MLS (RFC 9420 / RFC 9750) as the messaging substrate, and its Drystone-boundary
hazards. Grounded entirely in primary RFC text (every claim section-anchored, no secondary-source leans).

| doc | what it is |
|---|---|
| `mls-overview-and-terms.md` | The vocabulary + architecture reference. Pins client / member / group / epoch / leaf(LeafNode) / device / credential and the message verbs, each anchored to a specific RFC 9420/9750 section; the ratchet-tree invariant, the two-service (AS/DS) architecture, and an Alice-and-Bob narrative. Resolves the leaf-vs-client-vs-device conflation (leaf = the key a member holds; client = software on a device; device may host many clients; a user may have many devices; each client has one leaf). |
| `mls-hardcases-and-posture.md` | The MLS-to-Drystone design and posture doc: nine numbered hazard sections (linear chain, fork-as-escalation, under-determination, rights-vs-roles staleness, external-join, replay/nonce, FS-and-durability, DMLS, ReInit non-atomicity), the §10 concept-alignment map (direct / partial / Drystone-only / underused-MLS), and the §11 posture summary table. Twelve open items. |
| `side-histories-and-threading.md` | Candidate note: the three-tier threading model (subid / inherited side history / subgroup branch) with the entitlement-divergence selector rule; cross-references `mls-hardcases-and-posture.md §9` for tier-3 cost. |
| `mls-session-summary.md` | The MLS-session history and reasoning record (the "how the thinking went" register, per the method's rule that history lives in the log, not the design docs). |

### `drystone-design/`: the p10 deep-design companions (folded into the spec)

The ten p10 design companions the consolidated spec draws from: `asset-keying`, `history-durability`,
`fold-semantics`, `governance-finality`, `liveness-freshness`, `authority-and-complement`,
`fact-and-chain-representation`, `scaling-and-ordering`, `cast-beat-map`, `social-mapping`. These carry the
deeper design detail behind Part 2's sections; the spec cites the conclusions, these hold the working-out.
Filed 2026-07-07 (document-pass-8, the p10/p11 swap).

### `experiments/`: the convergence/fold experiment corpus

`drystone-experiments-consolidated`, `drystone-convergence-experiment-brief-v3`,
`drystone-reviews-and-experiments-log`, `drystone-fold-coverage-audit`. The validation corpus behind the
spec's `green-real` / `green-model` claims and the completeness-beam discharge path.

### `delivery-layer/`: the messaging/delivery-layer design corpus (00–10, 12)

The design of Drystone's messaging and delivery layer: the layer on top of the two settled substrate
choices, MLS (RFC 9420 / RFC 9750) for group key agreement and message protection, and iroh (core 1.0,
shipped 2026-06-15) for transport, discovery, and the gossip overlay. It is the follow-on from the
messaging-layer research prompt filed at `../../alpha/seeds/generated-prompts/`. A self-contained,
self-numbered set; start at `delivery-layer/00-session-summary.md`.

| doc | what it is |
|---|---|
| `00-session-summary.md` | Entry point: what the design session set out to do, the grounded substrate facts, the findings. |
| `01-delivery-architecture.md` | The architectural design (status: design, for folding into Part 2; Realizes P-Local-Truth / P-Knowable-Truth / P-Peer-Equality / P-Durable-Enablement). |
| `02-references.md` | References for the design (incl. the CALM grounding). |
| `03-pitch-outcomes.md`, `04-pitch-technical.md` | The pitch in outcome and technical registers. |
| `05-experiments.md`, `10-experiments-round2.md`, `12-replant-experiments.md` | Experiment plans: the delivery-mode experiments, round two, and the tree re-plant / atomic-swap experiments. |
| `06-deltachat-analysis.md` | Delta Chat comparison analysis. |
| `07-history-modes.md` | History/durability modes. |
| `08-experiment-methodology.md` | The fidelity-ladder methodology the experiments are tagged against (Rung A real-stack vs Rung B model). |
| `09-provenance.md` | Provenance ledger for the corpus. |

(The corpus's former `11-doc-method.md` is now the shared `../doc-writing-method.md`; internal "doc 11" references resolve there.)

## Key design results (from the corpus, grounded this round)

- **The atomic-swap re-plant.** At a boundary N, the group's authoritative membership is read from the
  governance chain (not replayed from MLS state); a fresh MLS group is stamped over exactly that set and
  the old tree is cut down. No replay, deterministic boundary, membership read from the governance
  authority. The MLS tree is demoted to a disposable key-distribution artifact: nothing downstream
  (dataplane hash tree, current- and history-governance hash trees) reads the tree's shape, so tree-byte
  nondeterminism across independent planters is a dedup, not a fork.
- **Grounded against primaries:** MLS group creation is unilateral and needs a KeyPackage per member
  (O(N) instantiation at the boundary); KeyPackages are single-use with a last-resort package as the
  offline escape hatch; a fresh stamp rotates every member's leaf key (a group-wide key refresh, the
  favorable PCS answer). The center-free constraint is KeyPackage availability at the boundary.
- **CALM grounding:** the CALM theorem (consistent + coordination-free iff monotonic) was verified
  against the primary sources (Hellerstein & Alvaro, *Keeping CALM*, arXiv:1901.01930 / CACM 2020; formal
  proof lineage Ameloot, Neven & Van den Bussche 2013). Monotonicity is about information growth, not
  time; consensus is the coordination you pay for when a problem is non-monotonic, not a picture of
  monotonicity. Used to frame the local-authority / center-free design.

## Provenance & status

- **Assembled from conversation** (multi-session), delivered 2026-07-06 via `seven-grounding.zip`. Filed
  byte-verbatim (13/13 `diff -q` confirmed), em-dash-clean as delivered. Raw transcript at
  `../../alpha/seeds/transcripts/raw/drystone-delivery-layer-design-2026-07-06.md`. See
  `../../alpha/seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`.
- **Design maturity, not spec.** `01` is explicitly "for folding into Part 2" once it holds; it is not
  yet normative. The experiments are tagged on the doc-08 fidelity ladder (Rung A on the real mls-rs /
  iroh stack; Rung B for Drystone's own hash structures that are not built yet).
- **Open residue flagged in the corpus (not by me this session):** pin the iroh subcrate versions
  (iroh-gossip's manifest still pins an iroh rc, an integration-residue item); whether mls-rs exposes
  ReInit as first-class emitting the resumption PSK vs fresh-create-plus-manual-PSK; and the KeyPackage
  availability/cost that tunes the boundary N.

## What this layer establishes (and does not)

Establishes a grounded, experiment-informed design for the delivery layer and the atomic-swap re-plant
model, ready to be exercised on the validated stack. Does **not** yet fold into the spec (that happens
when the experiments hold), does not build Drystone's own governance/dataplane hash structures (Rung B),
and does not pick the optimal boundary N (it measures the per-boundary cost that would tune it).
