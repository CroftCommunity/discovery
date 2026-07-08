# Beta coverage-gap ledger (Phase 0, conclusion inventory)

date: 2026-07-08

status: **COMPLETE** (all 10 clusters in). Output of the Phase-0 conclusion-inventory fan-out (plan:
`2026-07-08-beta-comprehensive-pass.md`, backlog C8). Ten agents each re-read a group of raw dialogue
transcripts and tagged every load-bearing conclusion `BETA` (present in a beta layer doc) / `ALPHA-ONLY`
(in alpha synthesis, not beta) / `TRANSCRIPT-ONLY` (never pulled up). The `### Top gaps` per cluster are
the Phase-1 recovery work-list, aggregated by target layer below. The full per-conclusion tables live in
the fan-out returns (session journal); this ledger carries verdicts + the gap lists, which are the
actionable core.

Clusters (transcript groups): A atproto-core · B open-social/p2p · C cairn-primers · D
drystone-governance · E drystone-spec/publication · F delivery/storage/media · G philosophy/cybernetics ·
H governance/history/naming · I history/poems/product · J origins/economics/assets. (pr3–9 proof records
excluded — their conclusions live in impl/experiments and Proofs.)

## Headline findings

1. **The rollup's coverage gate PASS is narrow, not wrong — and in one place it is wrong.** For the
   *mature* layers (drystone-spec, impl, cairn tech-primers) the alpha→beta carry-forward is
   near-exhaustive; clusters C and E found almost nothing missing. But the gate verifies that
   *enumerated* conclusions were placed, not that the enumeration was exhaustive, and the outer/late
   layers show real, load-bearing gaps.

2. **`LAYER-ROLLUP.md:43–44` over-claims concretely.** It asserts `atproto-sovereign-appview`,
   `atproto-atmospheric-web`, the private-data research, and **all of `ECOSYSTEM.md`** are "covered by
   existing cairn docs, verified by grep." They are not: only the high-level *framings* landed; the
   concrete design conclusions and the entire open-social breadth of `ECOSYSTEM.md` (Solid, DSNP,
   Farcaster, Peat, Groundmist, did:webvh chain, aggregator license-map, iroh realtime/games) never
   reached beta. This is the single largest hole and it is `cairn/`-shaped.

3. **The gaps fall into recognizable KINDS** (not random omissions) — which tells us the rollup
   systematically dropped a *type* of content, matching the "summarized too heavily" instinct:
   - **Decision rationale, not the decision.** The settled choice landed; the *why* did not (IETF
     document-shape rationale; Noria naming rationale; center-free-not-CRDT framing; port DECISION 1–5).
   - **Named movements & prior-art bodies.** Platform Cooperativism (Scholz) + precedents; the
     proof-of-personhood survey; the crypto-wars lineage; the secondary MLS bibliography.
   - **Closest cousins / strongest prior art dropped.** Peat (the *exact* substrate bet), Groundmist,
     Marmot/White Noise, Keet/Holepunch.
   - **Concrete tactics behind a generic claim.** Onboarding antidotes + event-app wedge; service-proxy
     JWT recipe; iroh tiered-exposure product model; aggregator license-map; redb storage contract;
     encrypted-blob-vault; asymmetric federation.
   - **Plain-language analogies / illustrations.** Rural Electric Cooperative; Glushkov's "10 billion by
     hand"; the Princeps/BitTorrent oligarchy illustration.
   - **Present-day / global reinforcement of a thesis.** Right-to-Roam + the 1607→2020 Buccleuch loop;
     worldwide independent dry-stone durability; Featherstone/Thompson "moral economy."
   - **Load-bearing negative results & cautions.** iOS BLE-mesh caution; the four-property impossibility;
     the Rave-trap governance answer; the did:webvh "atproto can't resolve it" constraint.
   - **Whole strategic arguments.** Aggregation-Theory enshittification-shield; the cross-platform
     identity-provenance thesis; the Solid↔PDS two-pole positioning.

## Cluster completion tracker

| Cluster | Status | Verdict (one line) |
|---|---|---|
| A atproto-core | in | Rollup OVER-CLAIMED — ~8 concrete design gaps (sovereign-AppView, private-data-WG, Aggregation Theory, iOS BLE, blob-vault). |
| B open-social/p2p | in | **Largest hole:** all of ECOSYSTEM.md un-carried (Solid/DSNP/Farcaster/Peat/Groundmist/did:webvh/aggregators/iroh media+games). |
| C cairn-primers | in | Essentially exhaustive; 1 minor gap (secondary MLS bibliography). |
| D drystone-governance | in | Spec-mechanics fully carried; gaps in prior-art body, adversarial-critique, field corrections. |
| E drystone-spec/publication | in | Near-exhaustive; 2 narrow gaps (IETF doc-shape rationale; Willow-interop mandate). |
| F delivery/storage/media | in | Mechanics carried; gaps in Rave-trap governance answer, redb contract, valuation-edge, media proof. |
| G philosophy/cybernetics | in | 4 load-bearing ALPHA-ONLY misses (cybernetic-failure, Platform Coop, onboarding, REA analogy). |
| H governance/history/naming | in | Core carried; gaps in fiscal-sponsor reasoning, naming rationale, present-day enclosure arc. |
| I history/poems/product | in | Product spine carried; gaps in service-proxy recipe, tiered-exposure model, Presence pond + no-streak ethic. |
| J origins/economics/assets | in | Gaps: cross-platform identity thesis, Peat, PDS-compliance revenue, four-property impossibility, Bazelon. |

---

## Phase-1 recovery work-list, by target layer (most-critical first)

Each item: **name** — coverage — source transcript — one-line why it is load-bearing. Cross-layer
duplicates (e.g. Peat appears for both cairn and impl) are listed once under their best home.

### cairn/ (the heaviest — includes the whole un-carried ECOSYSTEM.md)
- **did:webvh↔did:plc identity chain + the "atproto can't resolve did:webvh" negative result** — ALPHA-ONLY (B14/J8/J9) — Croft's credible-exit key-provenance design *and* a hard external constraint; beta carries did:webvh only as an unexplained gate.
- **Peat / peat-mesh (Defense Unicorns): Rust+iroh+Automerge+MLS, proven denied/degraded** — ALPHA-ONLY (B16/J10) — the single strongest prior art for Croft's *exact* substrate bet; cairn credits weaker neighbors but omits the closest.
- **Groundmist (Ink & Switch): local-first × atproto, DID-as-root-authority for private offline sync** — ALPHA-ONLY (B15) — closest local-first-private atproto relative to Croft's model.
- **Marmot + White Noise (Nostr+MLS): shipping metadata-hidden MLS group messenger** — TRANSCRIPT-ONLY (B17) — a live cousin to Croft's core thesis; adjacent-systems currently dismisses Nostr as "public-by-default."
- **Solid↔Bluesky-PDS two-pole positioning (WebID/Solid-OIDC/DPoP vs public indexed pipeline)** — ALPHA-ONLY (B18) — the explicit statement that Croft sits *between* the poles.
- **DSNP: social-graph-as-utility, no-token, delegation-without-master-keys, unbundle** — ALPHA-ONLY (B19) — shared goals Croft holds while rejecting the chain.
- **Aggregator license-map + no-per-activity-fee finding** — TRANSCRIPT-ONLY (A21/B20) — feeds the aggregator-pond strategy; only Bridgy Fed survived into beta.
- **Sovereign-AppView private-blocking / local shadow-ban; asymmetric federation ("gated castle"); encrypted-blob-vault** — ALPHA-ONLY (A14/A15/A16) — the concrete demonstrations that a self-run AppView inverts atproto public-by-default (Croft's private-overlay posture).
- **ATProto Private-Data WG engineering debates (trusted-PDS vs ZK; cheap self-host; key-revocation)** — ALPHA-ONLY (A12) — the design-tension map Croft's own private-data model must navigate.
- **Relay non-archival post-Sync-v1.1 (~$34/mo, Pi-capable)** — ALPHA-ONLY (A3) — sharp correction to "relays store everything."
- **Bridgy Fed / A New Social "credible exit"** — ALPHA-ONLY (A21) — Croft's nearest AP↔AT bridge cousin.
- **iroh realtime media (callme/iroh-roq/WebRTC-over-iroh 9/10) & iroh games roster** — ALPHA-ONLY (B24/B25/F19) — the calls-pond and games-pond building blocks (games = the cold-start hook).
- **Keet/Holepunch, Gun/OrbitDB competitive wedge; Blossom content-addressed media; CRDT alternatives (Loro/Y-CRDT/Diamond); iroh app-vitality roster incl. Holochain-on-iroh** — ALPHA/TRANSCRIPT-ONLY (F17/B27/B28/B30) — ecosystem breadth reinforcing the substrate bet.
- **Secondary MLS bibliography (Signal PGS, Rösler "More is Less", Blessing/Anderson)** — TRANSCRIPT-ONLY (C15) — low-severity corroboration for `mls-and-mimi.md`.
- **atproto service-proxying + custom-lexicon appview + service-auth JWT recipe** — TRANSCRIPT-ONLY (I16) — operational spine of Croft's owned pond (also relevant to croft/).

### governance/
- **Fiscal-sponsor analysis: SPI/Debian as working proof; Aspiration interim; SFC vs OSC** — ALPHA-ONLY (H9) — empirical proof the 3-layer model exists + the concrete Phase-1 vehicle beta only gestures at.
- **Platform Cooperativism as a named movement (Scholz 2014) + Stocksy/Drivers-Coop/Resonate** — ALPHA-ONLY (G11) — beta argues the co-op form *necessary* and even flags this literature unsourced, yet omits the movement name and existence-proofs.
- **PDS-hosting enterprise-compliance revenue model (SEC 17a-4/FINRA, WORM, $3.5B fines)** — ALPHA/TRANSCRIPT (J11) — the only concrete demand-validated sustainability path; carry as tracked input, not as Croft's answer.
- **Founding mottos: "the means determine the end"; "every revolution has a maintenance phase = the coop"** — TRANSCRIPT/ALPHA-ONLY (H10) — the thesis for why the non-extractive structure is itself non-negotiable.
- **INTERNAL CONFLICT to reconcile:** `foundation-cooperative-and-sustainability.md:42` says ref-impl license = AGPL-3.0-or-later; `open-publication-and-ip-stewardship.md:31` says Apache-2.0. (D-note) — beta-internal, must be reconciled.

### socialization/
- **Cultural-inertia onboarding antidotes (guest pass, jargon-hiding, starter-pack graph injection, Frames) + event-app Trojan-horse wedge** — ALPHA-ONLY (G13) — the concrete substance `adoption-strategy.md` (self-flagged open) is missing.
- **Rural Electric Cooperative analogy** — ALPHA-ONLY (G12) — the primary plain-language device for explaining the co-op.
- **The moat "from not having things" + maintenance-bounded-by-entropy framing** — PARTIAL/TRANSCRIPT-ONLY (I20/I21) — the most quotable articulation of why non-extraction is a moat.
- **LTS-for-interfaces / "constant UI change is quietly extractive" + settings-audiences model** — ALPHA-ONLY, note-only (F29/F30) — beta OPEN-THREADS:819 admits socialization/08 asserts the composability stance "without the audience model beneath it."
- **Logo meadow-in-letters brief** — ALPHA-ONLY (I25) — a concrete brand-asset concept unrecorded (assets doc has the drystone-stacking mark, a different concept).

### history/
- **Right to Roam (Nick Hayes, "the fence creates the crime") + 1607→2020 Buccleuch loop + "Scotland won twice"** — ALPHA-ONLY (H21) — the most vivid present-day closure of the enclosure-inversion arc, the layer's whole thesis.
- **Global dry-stone durability independently discovered (Incan/Japanese/Zimbabwean/Mediterranean)** — TRANSCRIPT-ONLY (H15) — the cross-civilizational engineering backbone of the Drystone metaphor.
- **Featherstone / E.P. Thompson "moral economy" + Captain Swing + Raymond Williams** — TRANSCRIPT-ONLY (I5) — the scholarly spine under the enclosure poems.
- **Bernera Riot 1874 (first crofter legal win)** — ALPHA-ONLY (I3) — completes the resistance sequence (beta has only Braes/Glendale).
- **Crofting-name-is-thesis explicit 3-feature mapping** — ALPHA-ONLY (I24) — the bridge welding history/ to philosophy/; in neither today.

### philosophy/
- **"Cybernetic failure" definition + Glushkov's "10 billion by hand"** — ALPHA-ONLY (G10) — the crispest form of the whole variety argument ("failure is architectural, not moral; better leadership never fixes it"); the spec's structural claim currently rests on the reader inferring it.
- **Proof-of-personhood prior-art survey + principal/personhood/DID/OAuth layering** — TRANSCRIPT-ONLY (D28) — a whole register grounding the governance weight model and persona vocabulary; beta carries only the flat "out of scope" line.
- **Ma Bell / Bazelon "privately beneficial without being publicly detrimental" (Hush-A-Phone/Carterfone)** — ALPHA-ONLY (J13) — the precise legal ancestor of the "no right to remove others' rights" razor; OPEN-THREADS T28 flags it as needing a home.
- **Crypto-wars lineage (Zimmermann/Bernstein/Barlow/Diffie–Hellman)** — TRANSCRIPT-ONLY (J14) — the civic "why" origin; MUST drop the FACTCHECK-REFUTED quotes if surfaced.
- **The candidate set of ~5 load-bearing non-negotiables** — ALPHA/deferred (H11) — the transcript calls consolidating them "the real founding act"; scattered, never assembled.
- **Princeps Problem named framing + BitTorrent/blockchain/Augustus illustration** — ALPHA/TRANSCRIPT-ONLY (H13) — the legible statement of the P2P failure the whole design guards against (principle carried in part-1 §2.7; the illustration is not).
- **Cross-platform identity-provenance thesis (OOB attestation is the only linkage; hub-and-spoke, did:plc-as-transparency-log)** — ALPHA-ONLY (J8/B22) — Croft's entire cross-platform identity design; beta records only the derived open decisions. (Could also live in cairn/ or drystone-spec.)

### croft/
- **Presence & Ritual pond ("the project's heart") + question-of-the-day with no streak ("the whole ethic in miniature")** — ALPHA-ONLY (I19) — a whole product pond + a load-bearing no-Skinner-box voice principle, only PRD-stub pointers in beta.
- **Game-outcome-as-custom-lexicon design (ephemeral over iroh, only settled outcome durable, attestation is the hard part)** — TRANSCRIPT-ONLY (I18) — the concrete shape of the first composed pond.
- **iroh tiered-exposure product model (public bridge → browser-peer → native; relay = complete broker)** — ALPHA-ONLY (I17) — the product-facing answer to "how does the web version work."
- **Composition-vs-valuation two-edge-types (directional weighted inter-group trust, no shared keys)** — PARTIAL/ALPHA-ONLY (F15) — completes the graph-of-groups trust model (composition is carried; valuation is not).
- **Membership-vs-access decoupling deepening** — BETA-partial (J2) — named as a boundary in OPEN-THREADS; the guest-door-without-governance-weight resolution wants surfacing.

### drystone-spec/
- **IETF document-shape rationale (RFC 6762/IKEv2 precedent; "philosophy must cash out into a mechanic")** — TRANSCRIPT-ONLY (E24) — why Drystone is a Part-1-reasoning + Part-2-mechanics + Alternatives-appendix doc; executed but the rationale is unrecorded.
- **Willow-launch interop pre-emption mandate ("where does the spec force two impls to agree")** — TRANSCRIPT-ONLY (E-gap2) — a spec obligation not recorded as such.
- **Four-property impossibility (moderation+multi-device+PFS+offline-mesh) licensing the MLS Delivery Service/superpeer** — ALPHA-ONLY (J12/F) — the mathematical argument that makes the superpeer honest; asserted-but-unlicensed in beta.
- **Matrix one-way-latch correction as durable grounding for P-Durable-Enablement** — TRANSCRIPT/CLOSED-THREADS-ONLY (D10) — survives only as a do-not-reintroduce note; the reframed §2.4 principle carries the name without its Matrix-derived grounding.

### impl/
- **redb storage-engine contract (MVCC single-writer, constant-time savepoints for late-event rollback, auth_/idx_ table families, blobs-in-iroh-blobs, fold-as-sole-writer)** — ALPHA-ONLY (F13/D19) — the actual local-storage implementation contract; only tracked as open-thread T25. Include the no-Jepsen/linearizability caveat (D19).
- **In-the-wild iroh media proof (callme/iroh-roq "proven Opus floor")** — ALPHA-ONLY (F19) — spec §6.12 asserts media-over-iroh works but cites no wild reference.
- **iOS wake-hook taxonomy (SLC/BGAppRefresh/BGProcessing/silent-push+NSE) + the BLE-scavenger caution** — ALPHA-ONLY (A19/A20) — the concrete mobile-background execution model + the negative result that forces the meer/superpeer.
- **Superpeer / home-node personal-cloud topology + PDS convergence** — BETA-partial (J16) — origin reasoning for the load-bearing always-on anchor; tracked as centralization risk but the topology reasoning isn't in a body doc.

### fenced/
- **The Rave-trap governance answer ("app stores care about your governance, not your transport"; decouple control/media plane, no public discovery by default, hardcoded kill-switch, edge moderation)** — ALPHA-ONLY (F20) — the app-store-survivability + abuse-posture argument; media spec §6.12 covers transport only.
- **Aggregation Theory + enshittification-shield (open protocol → ~0 switching cost → exodus disciplines the platform)** — ALPHA-ONLY (A22) — the spine of Croft's market thesis.
- **Delta Chat / RFC 9788 chatmail metadata correction** — ALPHA-ONLY (D21) — the updated posture (leak reduced to relational-metadata residue, no Sealed Sender) has no home; fenced omits Delta entirely.
- **Delta Chat grant-funded no-VC sustainability + activist-endorsement model** — ALPHA-ONLY (B23) — the funding-model data point (also governance-relevant).

---

## Per-cluster top-gaps (detailed record)

The numbered gap lists as returned by each cluster agent are preserved verbatim in the fan-out returns.
The aggregate above is the deduplicated, layer-organized synthesis of those lists. Cluster C's full
conclusion table (the format exemplar) and every cluster's `### Top gaps` are the source; the by-layer
work-list is what Phase 1 executes against.

### Note on the ECOSYSTEM.md hole (cluster B meta-finding)
The four 2026-06-22 transcripts (opensocial, iroh, solid, groundmist) were routed by
`ROADMAP_TODO.md` E26 to be "registered as rows in `ECOSYSTEM.md`" — which is frozen in alpha and never
carried into beta. The 2026-07-07 MLS/Willow transcript, by contrast, directly sourced `cairn/` and is
fully covered. So the coverage cliff is temporal: the July material was folded into the layer-cake; the
June ecosystem survey was parked in `ECOSYSTEM.md` and the rollup mistakenly counted it as covered. The
`cairn/` reference-work (backlog C9, cairn-as-projects-register) and this recovery pass should treat
`ECOSYSTEM.md` as the primary un-harvested source.
