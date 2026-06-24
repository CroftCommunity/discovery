# Narrative-architecture refactor — proposal (propose-only)

date: 2026-06-22
status: **PROPOSAL — nothing moved, renamed, or rewritten.** This doc + the DRAFT index in §4 are
the only new artifacts. Everything else was read-only.
author: design/narrative refactor pass (per `plans/narrative-architecture-refactor-PROMPT.md`)

---

## Problem statement

The `discovery` corpus is large (~120 markdown files across `seeds/`, `research/`, `thinking/`,
`crystallized/`, `narrative/`, plus standing docs) and it already carries **four index slices** — but
each cuts a different way and **none groups by *kind of thinking*:**

```
  COHESION.md ............ by SEAM          (a loose end ↔ the proof/doc that closes it)
  RAW-ARTIFACTS-MANIFEST . by PROVENANCE    (what raw came in, fidelity, fact-check status)
  ECOSYSTEM.md ........... by RELATIONSHIP  (homage / build-on / partner / learn↔)
  ROADMAP_TODO.md ........ by OPEN ITEM     (the single provenance-indexed backlog)
```

A narrative is assembled from a **lineage of thinking** — "this is the epistemology thread, that is the
cooperative-economics thread" — and that is exactly the slice the corpus does not have. As a concrete
symptom: `narrative/verticals/` has been a **promised-but-empty directory** since it was created; its
README lists six planned verticals but says "awaiting more odds-and-ends fragments before drafting."
The fragments exist. What is missing is the connective layer that says, for any given source, *which
lineage of thinking it belongs to and which narrative it feeds* — so a drafter can pull the right spine
in the right order without re-combing 120 files.

## Approach

1. Comb the whole corpus (six parallel read-only Explore passes, one per cluster) and tag every source
   with: its candidate thinking-type group(s), whether it is **spine / supporting / provenance-only**,
   and its **verification status** (pulled from the companion `*-FACTCHECK.md` where one exists).
2. Synthesize a **taxonomy of thinking-type groups** (§1) and, per group, a **source inventory** (§2).
3. Confirm which **cohesive narratives** the corpus actually supports — validating, refining, and
   expanding the human's three-candidate hypothesis (§3).
4. Design and partially **draft a new standing index** — the *source → lineage → narrative* map — that
   *links to*, never forks, the four existing slices, and that feeds the empty `narrative/verticals/`
   (§4).
5. Recommend structuring changes **only** where they help, defaulting to an overlay index over physical
   moves; flag the one tempting move that would *violate* PLAYBOOK §4 (§5).
6. Surface the decisions that are the human's to make (§6).

## Reasoning

The four existing slices are operational (what's unresolved, what came from where, who we build on,
what's still open). They are not reading paths. A reader who wants "the philosophical case for starting
from local-first" cannot get there from any of them — they would have to already know that
`narrative/lineage-of-a-design-imperative.md`, `thinking/local-first-as-design-imperative.md`, and the
"deeper foundation" of `crystallized/principles.md` are one continuous argument, and that
`seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md` is its raw origin.

The "kind of thinking" cut is the one that turns a pile of true things into a story, because a story is
a single lineage of thinking walked in order. Building it as an **overlay index** (not a reorg) keeps
raw seeds frozen (PLAYBOOK §4), keeps provenance attached to every source, and is cheap and reversible.
And it is the missing input the planned verticals were "waiting for fragments" to start — so this slice
is not a fifth competing index, it is the **bridge from the corpus to `narrative/`.**

A hard discipline runs through the whole proposal: **verification status travels with every source.**
Several spine candidates are Gemini-dialogue-sourced and carry REFUTED or UNVERIFIABLE material (§3a).
A narrative built on a fabricated quote or a wrong statute is a liability, so each narrative below
states what it must *not* carry forward.

---

## 1. Proposed taxonomy — seven lineages of thinking

These are *kinds of thinking*, not topics. A topic (say "atproto") shows up in several; a lineage is a
single intellectual move the corpus makes repeatedly.

```
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ G1  EPISTEMIC FOUNDATION         philosophy/epistemology → architecture       │
 │       "why no center can hold truth; why local-first is the generative premise"│
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G2  CIVIC, COOPERATIVE & ECONOMIC SUSTAINABILITY                              │
 │       "why we refuse extraction, and the structure that lets us survive it"    │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G3  DECENTRALIZATION IN TIME & IN THE FIELD   (two lineages, one kind)         │
 │       (a) diachronic = the history;  (b) synchronic = the present ecosystem    │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G4  THE LINEAGE-GROUPS PROTOCOL & ITS PROOFS   (the crypto-proof spine)        │
 │       "what we built and what we actually proved"                              │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G5  IDENTITY & PROVENANCE         "keys are not identity; what you carry"       │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G6  SAFETY, MODERATION & GOVERNANCE UNDER BLINDNESS                            │
 │       "how a system that cannot read content stays safe and governable"        │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │ G7  THE PRODUCT / APP LAYER       "Croft the client — the garden of ponds"      │
 └─────────────────────────────────────────────────────────────────────────────┘
```

**Definitions and boundaries.**

- **G1 — Epistemic foundation.** Thinking that derives the system's *shape* from a claim about
  knowledge itself (a distributed system sees assertions and agreement, never the world; therefore
  compute provenance, never utility; therefore local-first is the unit). Boundary vs G2: G1 is the
  *epistemology* ("truth is local and corroborated"); G2 is the *political economy* ("extraction is the
  enemy, the cooperative is the maintenance plan"). They meet at "plurality is a survival condition,"
  but G1 derives it from Ashby/Hayek/Scott and G2 from Ostrom/the rug-pull cycle.

- **G2 — Civic, cooperative & economic sustainability.** Why non-extraction is the point (not a missing
  feature), and the legal/economic mechanism that makes it durable (cooperative, progressive
  decentralization, pay-the-keepers, anti-rug-pull structure). Boundary vs G3: G2 is *our normative
  stance and our survival mechanism*; G3 is *the comparative record* (what happened to others, what the
  field looks like).

- **G3 — Decentralization in time & in the field.** Comparative/landscape thinking. It has two
  lineages that are the same intellectual move on different time-axes: **(a) diachronic** — the history
  (enclosure→commons, crofting, the crypto-wars liberty arc, BitTorrent→IPFS→iroh, SSB→Willow,
  Steem→Hive, the VC social-platform rug-pull cycle); **(b) synchronic** — the present ecosystem and how
  the options compare (atproto / Solid / DSNP / Nostr / Farcaster / Matrix / Discord / Germ, the
  sovereign-AppView club, the messaging-solutions field). Keeping them as one *group* with two
  *narratives* is a deliberate call — see §3 N2/N3 and the open question in §6.

- **G4 — The lineage-groups protocol & its proofs.** The crypto-proof spine: the thesis (group as a
  navigable lineage; two/three-tree binding; survivor-epoch re-key), the wire spec, and the
  proof/test/conformance apparatus. Boundary vs G6: G4 is the *cryptographic/structural* invariants
  (what verifies); G6 is the *adversarial/social* problem of safety when the broker is blind (what a
  hostile member or an abuse-hub does). They share `freshness-signal` and `revocation-authority` (noted
  as overlaps).

- **G5 — Identity & provenance.** DID-lineage thinking: keys≠identity, the DID-method choice for the
  MLS root, cross-platform linkage (did:webvh↔did:plc, `alsoKnownAs`), multi-device as
  distinct-members-under-one-lineage. A sub-lineage of G4 by dependency, but a distinct *kind of
  thinking* (it reasons about identity continuity and cross-network attestation, not group crypto), and
  the human named it as a likely thread — so it stands as its own group.

- **G6 — Safety, moderation & governance under blindness.** The genuinely distinct problem: a
  content-blind system still has to resist abuse hubs, gate stale authority, respond to faithful-path
  failures, and moderate by consent without becoming a surveillance plane. This *emerged* from the
  combing (it was not in the human's three candidates) and is cohesive enough to be its own thread.

- **G7 — The product / app layer.** "Croft the client": one functional core + thin per-platform shells
  (FCIS), the garden of ponds + pads, interaction tiers, design criteria, brand/voice. The newest body
  (2026-06-22); postdates the `narrative/verticals/` README, which is why no planned vertical covers it.

**Explicit overlaps (a source can sit in two groups):**

```
  freshness-signal, revocation-authority ........... G4 ∩ G6
  cooperative-social-union-model, governance-and-survivability  G2 ∩ G1 (durability ← epistemic humility)
  cross-platform-identity-provenance, plc-identity-resilience .. G5 ∩ G4
  interaction-tiers, social-layer .................. G7 ∩ G4 (product surface over the protocol)
  crofting-research / crypto-wars dialogue ......... G3(a) ∩ G2 (history that is also our normative root)
  atproto-private-data-architecture ................ G3(b) ∩ G5 ("different not weaker" is both)
```

---

## 2. Per-group source inventory

Tags: **[S]** spine (load-bearing) · **[U]** supporting · **[R]** provenance-only/raw. Verification:
pulled from the companion `*-FACTCHECK.md` where one exists; "design-synthesis" = original reasoning,
no external fact claims to verify; "cite SoT" = defer to the atproto/iroh source-of-truth FACTCHECK.

### G1 — Epistemic foundation
| Source | Role | Tag | Verification |
|---|---|---|---|
| `narrative/lineage-of-a-design-imperative.md` | The 2,400-yr arc (Socrates→Mill→Peirce/Popper→Hayek→Ostrom→Ashby→Beer→Scott) | **[S]** | quotes verified in-dialogue; appendix flags Peirce/Popper/Ostrom **confirm-before-publishing** |
| `thinking/local-first-as-design-imperative.md` | Arc → architecture (local-first as the generative premise) | **[S]** | design-synthesis |
| `crystallized/principles.md` ("The deeper foundation") | The razor: compute provenance, never utility; no right to remove rights | **[S]** | design-synthesis; Bazelon/Carterfone legal ancestor CONFIRMED |
| `seeds/transcripts/raw/croft-architecture-design-dialogue-2026-06-20.md` | Raw origin; the Kleppmann letter; governance-as-substrate | **[R]** | cleaned-paste; cited refs verified |
| `SOVEREIGN-COMMONS-DOSSIER.md` (§3) | Umbrella why (commons vs tragedy, linear vs cyclical) | **[U]** | **high provenance debt** on Gemini-era stats/dates |

### G2 — Civic, cooperative & economic sustainability
| Source | Role | Tag | Verification |
|---|---|---|---|
| `research/social-platform-cycle.md` | The VC rug-pull cycle; extraction imperative → enshittification | **[S]** | research deliverable |
| `crystallized/principles.md` (Tier 1) | Non-extraction, credit-union, pay-the-keepers, progressive decentralization | **[S]** | design-synthesis |
| `thinking/cooperative-social-union-model.md` | MO Ch.351 LCA mechanism (the sustainability answer) | **[S]** | **legal-review gate**; statute specifics partly wrong (see §3a) |
| `thinking/governance-and-survivability.md` | Anti-rug-pull: bankruptcy-remote steward + pre-funded archive | **[U]** | design-synthesis |
| `seeds/transcripts/raw/cooperative-social-union-governance-dialogue-2026-06-22.md` | Raw of the cooperative model | **[R]** | FACTCHECK: framework real; **REFUTED** §351.1015→.1036, fee figures; carry reasoning not citations |
| `seeds/transcripts/raw/crypto-wars-to-p2p-pds-economics-dialogue-2026-06-22.md` | PDS-hosting economics + enterprise-compliance demand | **[R]** | FACTCHECK: skeleton CONFIRMED; **REFUTED** fabricated quotes (Zimmermann "Stalin", Meyer letter, "Voskop") |
| `research/p2p-founder-motivations-adoption.md` | Why P2P founders built; only Signal crossed the chasm | **[U]** | research deliverable |
| `research/open-publication-and-ip-protection.md`, `research/socialization-and-publication-venues.md` | IP/defensive-publication & venue strategy | **[R]** | operational, not narrative spine |
| `seeds/transcripts/raw/croft-crofting-research.md` | Commons/enclosure deep root (shared with G3a) | **[S]** | scholarly; CONFIRMED — see G3a |

### G3 — Decentralization in time (a) & in the field (b)
| Source | Role | Tag | Verification |
|---|---|---|---|
| `seeds/transcripts/raw/croft-crofting-research.md` | (a) Enclosure→commons, crofting history, 1886 Act | **[S]** | scholarly, CONFIRMED; corrects the romantic myth |
| `seeds/transcripts/raw/croft-crofting-narrative.md` | (a) The quotable telling (color, not spine) | **[R]** | tertiary sources; "ancient free clan" arc is **MYTH** (use research file as truth) |
| `seeds/transcripts/raw/p2p-architecture-origin-dialogue.md` | (a) BitTorrent→IPFS→iroh; SSB→Willow; stack discovery | **[R]** | iroh CONFIRMED; Willow flagged in-construction (sound) |
| `seeds/transcripts/raw/groundmist-hive-identity-chain-iroh-games-dialogue-2026-06-22.md` | (a) Steem→Hive takeover (anti-capture cautionary) | **[R]** | Hard Fork 23 ≈ $6.3M (not $5M); **REFUTED** did:key atproto-resolvable |
| `ECOSYSTEM.md` | (b) The relational register (transport/crypto/CRDT/identity/social/apps/trust) | **[S/INDEX]** | iroh `1.0.0` standing correction; dialogue-sourced rows flagged |
| `research/messaging-solutions-landscape.md` | (b) Signal/SSB/Matrix/Briar/Delta/Session comparison | **[S]** | systematically verified (Delta iroh = ephemeral-only; SSB decline) |
| `research/discord-dominance.md`, `research/discord-matrix-groupchat.md` | (b) The UX bar; privacy-free vs convenience-hard split | **[S]** | research deliverables |
| `research/public-social-protocols.md` | (b) X/Bluesky/Threads/Mastodon/Pixelfed; DID dual-use | **[S]** | research deliverable |
| `research/atproto-private-data-architecture.md`, `research/atproto-sovereign-appview-club.md` | (b) "different not weaker"; what owning the AppView unlocks | **[S/U]** | research; cite atproto SoT |
| `research/germ-xchat-features.md`, `research/group-chat-failure-modes(-plain).md` | (b) Germ vs X Chat; field failure modes | **[U]** | research deliverables |
| `seeds/transcripts/raw/{croft-atproto-sovereign-appview-open-social, croft-atproto-pds-germ-privatedata, solid-pds-webid-scalingtrust-dsnp, opensocial-nostr-farcaster-aggregators, iroh-quic-localfirst-ecosystem, atproto-architecture-appview-relay-explainer}-*.md` | (b) Raw ecosystem dialogues | **[R]** | each has FACTCHECK; mostly CONFIRMED; cite atproto/iroh SoT; minor PARTLY/REFUTED per file |

### G4 — The lineage-groups protocol & its proofs
| Source | Role | Tag | Verification |
|---|---|---|---|
| `thinking/thesis-lineage-groups.md` | The protocol thesis (group as navigable lineage; two-tree binding) | **[S]** | design-synthesis; proof-backed |
| `crystallized/CROFT-PROTOCOL.md` | The normative wire spec (interop anchor) | **[S]** | proof status inline (green-real/green-model/design) |
| `crystallized/proof-ledger.md` | Every I/E/V/S claim + status (Phase 1 gate = GO) | **[S]** | green-real on openmls 0.8.1 |
| `crystallized/test-narrative.md` | Why each test, what it tells us, what's still open | **[S]** | reasoning over proofs |
| `crystallized/conformance-suite.md`, `crystallized/TEST-CORPUS.md` | What a conformant impl must pass; the catalog | **[S/U]** | 66 vectors pass/0 fail |
| `thinking/merge-split-corpus.md` | Three-tree taxonomy (S1–S4/M1–M6/C1–C10) | **[U]** | feeds conformance |
| `thinking/{design-notes-addendum, experiment-suite, model-holds-up-summary}.md` | Roll-ups; sim spec; honest scorecard | **[U/R]** | design-synthesis; model-holds-up = external critique |
| `seeds/groupdynamics-unpacked/{THESIS,SOCIAL_LAYER,MULTI_DEVICE}.md` | Frozen seed of the thesis/social/multi-device | **[R]** | frozen verbatim seed of the thinking docs |
| `seeds/transcripts/design-dialogue-2026-06-13-to-14.md` | The richest single seed — where the design reasoning happened | **[R]** | preserved-verbatim; landscape systematically verified |
| `crystallized/conclusions.md`, `ANALYSIS.md`, `TEST-PLAN.md`, `ROUND-2026-06-17-*.md` | Synthesis / corpus-map / proof-sequencing / session summary | **[U/INDEX]** | green-real status |

### G5 — Identity & provenance
| Source | Role | Tag | Verification |
|---|---|---|---|
| `thinking/plc-identity-resilience.md` | DID-method choice for the MLS root; PLC read-replica | **[S]** | design + dependency caveats |
| `thinking/cross-platform-identity-provenance.md` | Hub-and-spoke attestation; `alsoKnownAs`; no cross-platform authority key | **[S]** | design-synthesis |
| `thinking/multi-device.md` | Device = distinct MLS member under one DID lineage | **[S]** | design-synthesis |
| `seeds/transcripts/raw/croft-identity-provenance-dialogue-2026-06-20.md` | Raw; did:webvh↔did:plc bridge verification | **[R]** | artifact gap (corrected text summarized, not pasted) |
| `seeds/transcripts/raw/croft-atproto-pds-germ-privatedata-dialogue-2026-06-22.md` | Germ Anchor-Key-in-profile pattern (shared w/ G3b) | **[R]** | FACTCHECK strong; cite atproto SoT |
| `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md` | **Project source-of-truth** for atproto/iroh/iOS facts | **[INDEX]** | the cite-don't-re-verify anchor (iroh 1.0.0; no native AT-Proto E2EE) |

### G6 — Safety, moderation & governance under blindness
| Source | Role | Tag | Verification |
|---|---|---|---|
| `thinking/abuse-resistance-and-the-rave-trap.md` | Stay off the abuse-hub path without a surveillance plane (Signal's shape) | **[S]** | design-synthesis |
| `thinking/geer-gating-peer.md` | Consented, disclosed, revocable content-gating role (blind-by-default is the thesis) | **[S]** | design-synthesis |
| `thinking/failed-op-response.md` | Detection fixed, response is a dial (LOUD/SILENT/BLACKHOLE) | **[U]** | design-synthesis |
| `thinking/freshness-signal.md` | No-false-current; freshness gates authority (shared w/ G4) | **[S]** | DECIDED 2026-06-17; tested E2.16 |
| `thinking/revocation-authority.md` | Threshold membership authority (shared w/ G4) | **[S]** | DECIDED 2026-06-17 |
| `thinking/meer-superpeer-design.md` | Always-on blind participant; anti-entrenchment | **[U]** | green-real (meer running) |
| `seeds/transcripts/raw/iroh-realtime-media-rave-trap-scaled-usecases-dialogue-2026-06-22.md` | Raw Rave-trap case | **[R]** | overlaps abuse-resistance doc; cite existing intakes |

### G7 — The product / app layer
| Source | Role | Tag | Verification |
|---|---|---|---|
| `thinking/app/README.md` | Entry point to the app body; decision log pointer | **[S/INDEX]** | — |
| `thinking/app/design-philosophy.md` | FCIS, honest seams, the garden thesis, decision rules | **[S]** | design-synthesis |
| `thinking/app/client-architecture-adr.md` | ADR: one core + thin shells (proven in Phase 0) | **[S]** | green-real (PR #10 Phase 0) |
| `thinking/app/design-criteria.md` | Quality bar for ponds/pads | **[S]** | design-synthesis |
| `thinking/interaction-tiers.md` | Three products, one send button | **[S]** | distilled from germ-xchat dialogue |
| `thinking/social-layer.md` | Graph-you-hold; layer separation; 5 safety invariants (shared G4) | **[S]** | design-synthesis |
| `thinking/app/build-specs/{BUILD-SPEC, BUILD-SPEC-PHASE-1-2}.md` | Phase 0 (built) + Phases 1–2 (spec'd) | **[S/U]** | Phase 0 green-real |
| `thinking/app/ponds/*` (8 files) | The garden catalog (games, utilities, fair-reveal, build-order, on-device-LLM, webxdc security) | **[U]** | in-session license/API verification |
| `thinking/app/brand-and-voice-notes.md` | "Croft / Grow your own" (draft) | **[U]** | **DRIFT vs `NAMING.md`** — flagged ROADMAP_TODO/COHESION §24 |
| `seeds/transcripts/raw/{croft-app-design-dialogue, croft-app-ponds-games-dialogue, croft-app-portdecision-review}-*.md` | Raw of the app body | **[R]** | cleaned-paste; decisions live in the working docs |
| `seeds/{multiecosystemapp-unpacked, apps-unpacked}/*`, `seeds/generated-prompts/*` | **Frozen seeds** = byte-identical source of the `thinking/app/` working copies | **[R]** | preserved-verbatim (see §5 duplication note — do NOT delete) |

---

## 3. Candidate narratives

The human's hypothesis (three narratives) holds and **expands to seven confirmed threads plus one
candidate.** Each is named with a one-line thesis, its spine in reading order, and what is
missing / unproven / decision-gated.

**N1 — "Why start from philosophy" (epistemic grounding).** *Confirmed.*
Thesis: a distributed system can only ever establish provenance, never truth — so the honest design
computes provenance and leaves judgment at the edges, and that single epistemic claim, independently
rediscovered across 2,400 years, is what makes local-first the generative premise.
Spine: `narrative/lineage-of-a-design-imperative.md` → `thinking/local-first-as-design-imperative.md`
→ `crystallized/principles.md` "The deeper foundation" (raw: `croft-architecture-design-dialogue-2026-06-20.md`).
Missing/gated: appendix quote-verification (Peirce, Popper, Ostrom) must be confirmed against primary
editions before any external publication.

**N2 — "The history of decentralization" (diachronic, G3a + G2 root).** *Confirmed.*
Thesis: every era's commons gets enclosed (Highland crofts → crypto-wars → VC social platforms), and
the antidote keeps rhyming — secure standing, real exit, no central arbiter.
Spine: `research/social-platform-cycle.md` → `croft-crofting-research.md` (deep root) →
`crypto-wars-to-p2p-pds-economics-dialogue` (liberty lineage) → `p2p-architecture-origin-dialogue.md`
(BitTorrent→iroh; SSB→Willow) → `research/messaging-solutions-landscape.md` +
`research/p2p-founder-motivations-adoption.md` → `groundmist-hive-...` (Steem→Hive cautionary).
Missing/gated: **do not carry** the crofting "ancient free clan" myth (use the research file, the
narrative file is color only), the fabricated crypto-wars quotes, or Hard Fork 23 = "$5M."

**N3 — "The ecosystem & how the options compare" (synchronic, G3b).** *Confirmed.*
Thesis: against the live field (atproto / Solid / DSNP / Nostr / Farcaster / Matrix / Discord / Germ),
Croft's bet — public social on shared DID identity + a private MLS layer the host is blind to — is
"different, not weaker," and the field's own direction (atproto deferring E2EE) sharpens that.
Spine: `ECOSYSTEM.md` → `research/{messaging-solutions-landscape, discord-dominance,
discord-matrix-groupchat, public-social-protocols}.md` → `research/atproto-private-data-architecture.md`
+ `research/atproto-sovereign-appview-club.md` (supporting raw: the six ecosystem dialogues in G3b).
Missing/gated: dialogue-sourced ECOSYSTEM rows stay flagged; **cite the atproto/iroh source-of-truth
FACTCHECK** rather than re-deriving (iroh `1.0.0`; iroh-docs = range-set reconciliation not MST; no
native AT-Proto E2EE).

**N4 — "Sustainability & the cooperative" (G2).** *Confirmed — a real additional thread, the one the
human suspected.* Thesis: refusing extraction is anti-fragile, not just ethical — but it only survives
if a concrete legal/economic mechanism (a member-owned cooperative, progressive decentralization,
pay-the-keepers) carries it, because the all-volunteer model is the single biggest long-term risk.
Spine: `research/social-platform-cycle.md` → `crystallized/principles.md` Tier 1 →
`thinking/cooperative-social-union-model.md` → `thinking/governance-and-survivability.md` (raw:
`cooperative-social-union-governance-dialogue`, crypto-wars PDS-economics body).
Missing/gated: **legal-review gate** (ROADMAP_TODO D5/E25) — the MO Ch.351 specifics are partly wrong
(§351.1015→.1036; fee figures) and are an attorney's call; carry the *reasoning*, not the citations.
Also gated by the **MPL-license decision** (ROADMAP_TODO A-section) and the **CroftC IP/ownership**
question for the app body.

**N5 — "The protocol we proved" (G4).** *Confirmed.*
Thesis: model a group as a navigable lineage of conversations, not an eternal room; bind an MLS ratchet
+ a governance DAG + a history CRDT; under partition, fork cleanly and escalate to a human rather than
auto-adjudicate — and this is green-real on openmls 0.8.1, not a sketch.
Spine: `thinking/thesis-lineage-groups.md` → `thinking/merge-split-corpus.md` →
`crystallized/CROFT-PROTOCOL.md` → `crystallized/proof-ledger.md` + `crystallized/test-narrative.md` →
`crystallized/conformance-suite.md` (raw origin: `design-dialogue-2026-06-13-to-14.md`).
Missing/gated: the referee/survivor-selection problem and the unbounded device-churn-record sleeper
risk (per `model-holds-up-summary.md` / `conclusions.md`).

**N6 — "Identity you carry across platforms" (G5).** *Confirmed.*
Thesis: keys are not identity — a person is a DID lineage, each device a distinct member — and
cross-platform continuity is attestation (hub-and-spoke `alsoKnownAs`), never a single cross-network
authority key that cannot safely exist.
Spine: `thinking/plc-identity-resilience.md` → `thinking/cross-platform-identity-provenance.md` →
`thinking/multi-device.md` (raw: `croft-identity-provenance-dialogue-2026-06-20.md`; cite atproto SoT).
Missing/gated: total-device-loss recovery (the backup-vs-delegation fork — the headline open protocol
problem); the recovery-anchor decision (ROADMAP_TODO A-section) is the human's.

**N7 — "Croft the product" (G7).** *Confirmed.*
Thesis: surface the proven substrate as a composable garden — native "ponds" (Bluesky/Mastodon/Lemmy,
honest seams) and small "pads" — on one functional core with thin per-platform shells, so behavior can
not drift across platforms and generosity carries no marginal cost.
Spine: `thinking/app/README.md` → `design-philosophy.md` → `client-architecture-adr.md` →
`design-criteria.md` → `thinking/interaction-tiers.md` → `build-specs/BUILD-SPEC.md` → `ponds/*`
(raw: the three app dialogues).
Missing/gated: brand DRIFT vs `NAMING.md`; the deep-link resolver (tier-zero UX unlock) and the
sustainability↔cooperative-mechanism tie-in (shared with N4); CroftC Phase-0 IP/ownership.

**N8 (candidate) — "Safety without surveillance" (G6).** *Confirmed thread; graduate-or-fold is the
human's call.* Thesis: a content-blind system stays safe by structure, not inspection — scale/peer
levers and consented, revocable roles (geer), with stale authority gated by a freshness signal — so
moderation never becomes the surveillance plane the design refuses to be.
Spine: `thinking/abuse-resistance-and-the-rave-trap.md` → `thinking/geer-gating-peer.md` →
`thinking/failed-op-response.md` → `thinking/freshness-signal.md` → `thinking/revocation-authority.md`.
Decision (§6): graduate to its own vertical, or fold into N5 as its adversarial chapter.

### 3a. Verification flags that gate narratives (do-not-build-on-sand)

```
  N2  crofting "ancient free clan" arc ........ MYTH (use croft-crofting-research.md; narrative = color)
  N2  Zimmermann "Stalin" quote, Meyer letter, "Voskop" ... REFUTED fabrications — drop the quotes, keep skeleton
  N2  Hard Fork 23 "$5M" ...................... ≈ $6.3M / 23.6M STEEM
  N3  iroh version / iroh-docs structure / AT-Proto E2EE ... cite atproto SoT FACTCHECK (1.0.0; range-set not MST; no native E2EE)
  N3  did:key atproto-resolvable ............. REFUTED (plc + web only)
  N4  MO §351.1015, CA-41 fee, name-res fee ... REFUTED specifics — legal review; carry reasoning not citations
  N1  Peirce / Popper / Ostrom exact wording .. confirm vs primary edition before publishing
  N4/N1 SOVEREIGN-COMMONS-DOSSIER Gemini-era stats/dates ... high provenance debt — re-verify before external use
  N7  brand-and-voice-notes vs NAMING.md ...... DRIFT — reconcile before brand narrative
```

---

## 4. The recombed cross-reference index — design + first draft (DRAFT / PROPOSED)

**What it is.** A new standing index that adds the missing slice — *by lineage of thinking* — and
maps every source to the narrative(s) it feeds. It is the **bridge from the corpus to
`narrative/verticals/`** (the empty, promised dir), not a fifth competing operational index.

**What it is NOT (how it relates to the four existing slices).** It **links to**, never duplicates:

```
  this index answers ........ "what KIND of thinking is this, and which narrative does it feed?"
  → for SEAM status ......... links to COHESION.md §N
  → for PROVENANCE/fidelity . links to RAW-ARTIFACTS-MANIFEST.md
  → for RELATIONSHIP ........ links to ECOSYSTEM.md §N
  → for OPEN ITEMS .......... links to ROADMAP_TODO.md
```

A row never restates a fact owned by another index; it carries a thinking-type group, a
narrative-feeds list, a spine/supporting/raw tag, a verification flag, and pointers.

**Suggested location.** `discovery/SOURCE-LINEAGE-MAP.md` (top-level, peer to the other four standing
indexes) — *recommended*; alternative `narrative/SOURCE-MAP.md` (closer to where it is consumed). This
is an open decision (§6).

**Column spec (proposed):**

```
| source (path) | lineage group(s) | tag | feeds narrative(s) | verification | see-also |
```

### 4a. DRAFT — source → lineage → narrative (spine sources; partial, extend later)

| source | group(s) | tag | feeds | verification | see-also |
|---|---|---|---|---|---|
| narrative/lineage-of-a-design-imperative.md | G1 | S | N1 | confirm Peirce/Popper/Ostrom pre-publish | — |
| thinking/local-first-as-design-imperative.md | G1 | S | N1 | design-synthesis | principles.md |
| crystallized/principles.md | G1,G2 | S | N1,N4 | design-synthesis | COHESION §22,§25 |
| research/social-platform-cycle.md | G2,G3a | S | N2,N4 | research | — |
| thinking/cooperative-social-union-model.md | G2 | S | N4 | legal-review gate | ROADMAP_TODO D5/E25; COHESION §33 |
| thinking/governance-and-survivability.md | G2 | U | N4 | design-synthesis | — |
| seeds/.../croft-crofting-research.md | G3a,G2 | S | N2 | scholarly CONFIRMED | NAMING.md |
| seeds/.../croft-crofting-narrative.md | G3a | R | N2 (color) | **MYTH arc — color only** | MANIFEST; COHESION §16 |
| seeds/.../crypto-wars-...-economics-dialogue.md | G3a,G2 | R | N2,N4 | **REFUTED quotes** | crypto-wars FACTCHECK; COHESION §25 |
| ECOSYSTEM.md | G3b | S/INDEX | N3 | iroh 1.0.0; rows flagged | — |
| research/messaging-solutions-landscape.md | G3b | S | N2,N3,N5(sec) | verified | — |
| research/discord-dominance.md | G3b | S | N3 | research | — |
| research/public-social-protocols.md | G3b,G5 | S | N3,N6 | research | — |
| research/atproto-private-data-architecture.md | G3b,G5 | S | N3,N6 | cite atproto SoT | COHESION §26 |
| thinking/thesis-lineage-groups.md | G4 | S | N5 | proof-backed | proof-ledger |
| crystallized/CROFT-PROTOCOL.md | G4 | S | N5 | proof status inline | conformance-suite |
| crystallized/proof-ledger.md | G4 | S | N5 | green-real (gate GO) | TEST-CORPUS; COHESION §1 |
| crystallized/test-narrative.md | G4 | S | N5 | reasoning-over-proofs | — |
| seeds/transcripts/design-dialogue-2026-06-13-to-14.md | G4 | R | N5 | preserved-verbatim | MANIFEST |
| thinking/plc-identity-resilience.md | G5,G4 | S | N6 | design + caveats | — |
| thinking/cross-platform-identity-provenance.md | G5 | S | N6 | design-synthesis | COHESION §21 |
| thinking/multi-device.md | G5 | S | N5,N6 | design-synthesis | — |
| seeds/.../atproto-atmospheric-web-iroh-mobile-FACTCHECK.md | G5,G3b | INDEX | N3,N6 | **source-of-truth — cite** | AGENTS.md |
| thinking/abuse-resistance-and-the-rave-trap.md | G6 | S | N8 | design-synthesis | — |
| thinking/geer-gating-peer.md | G6 | S | N8 | design-synthesis | — |
| thinking/freshness-signal.md | G6,G4 | S | N5,N8 | DECIDED; E2.16 | — |
| thinking/revocation-authority.md | G6,G4 | S | N5,N8 | DECIDED | — |
| thinking/app/design-philosophy.md | G7 | S | N7 | design-synthesis | thinking/app/README |
| thinking/app/client-architecture-adr.md | G7,G4 | S | N7 | green-real (Phase 0) | COHESION §23 |
| thinking/interaction-tiers.md | G7,G4 | S | N7 | distilled | principles Tier 3 |
| thinking/social-layer.md | G7,G4 | S | N7,N5 | design-synthesis | — |
| thinking/app/brand-and-voice-notes.md | G7 | U | N7 | **DRIFT vs NAMING.md** | COHESION §24 |

### 4b. DRAFT — reverse view: narrative → spine in reading order

```
  N1  lineage-of-a-design-imperative → local-first-as-design-imperative → principles("deeper foundation")
  N2  social-platform-cycle → crofting-research → crypto-wars-economics → p2p-origin → messaging-landscape
        + founder-motivations → groundmist-hive(Steem→Hive)
  N3  ECOSYSTEM → {messaging-landscape, discord-dominance, discord-matrix, public-social-protocols}
        → atproto-private-data-architecture + atproto-sovereign-appview-club
  N4  social-platform-cycle → principles(Tier 1) → cooperative-social-union-model → governance-and-survivability
  N5  thesis-lineage-groups → merge-split-corpus → CROFT-PROTOCOL → proof-ledger + test-narrative → conformance-suite
  N6  plc-identity-resilience → cross-platform-identity-provenance → multi-device
  N7  app/README → design-philosophy → client-architecture-adr → design-criteria → interaction-tiers
        → build-specs/BUILD-SPEC → ponds/*
  N8  abuse-resistance-and-the-rave-trap → geer-gating-peer → failed-op-response → freshness-signal → revocation-authority
```

### 4c. How the lineage map relates to the six planned verticals

The map shows the planned `narrative/verticals/` set (created before the newest bodies) has **gaps**:

```
  planned vertical                 ← lineage map says
  ─────────────────────────────────────────────────────────────────────────
  1 lineage-group-protocol         = N5 (G4)                       covered
  2 multi-device-and-identity      = N6 (G5)                       covered
  3 social-graph-you-hold          = part of N7 (G7) + social-layer covered
  4 the-cooperative                = N4 (G2)                       covered
  5 the-civic-why                  = N2 (G3a) + upstream N1 (G1)   N1 epistemic root MISSING upstream
  6 substrate-and-economics        = part of N3 (G3b) + N7         partial
  ── not in the planned set ──────────────────────────────────────────────
    N1 epistemic foundation (standalone)   ........... MISSING (currently folded under "civic-why")
    N3 ecosystem comparison (standalone)    .......... MISSING (only as backdrop)
    N7 the app/product body                 .......... MISSING (postdates the verticals README)
    N8 safety-under-blindness               .......... MISSING (candidate)
```

That gap analysis is the concrete payoff: the lineage map tells the next session *which verticals to
add and what spine each pulls*, instead of "awaiting more fragments."

---

## 5. Restructuring recommendations

**Default: overlay index over physical moves.** Adopt `SOURCE-LINEAGE-MAP.md` as the one new standing
artifact and leave the file tree as-is. It is cheap, reversible, provenance-safe, and it solves the
stated problem (the missing slice + the stalled verticals) without touching a single existing file.

**Recommended, low-risk follow-ons (human-approved, executed later — not now):**

1. **Draft the four missing verticals** the map surfaces (N1 epistemic, N3 ecosystem, N7 app, and
   N8-if-graduated), and split "the-civic-why" so N1 (epistemic) sits upstream of N2 (history). Update
   `narrative/verticals/README.md` to the expanded set. *Trade-off:* more verticals = more to maintain;
   mitigated because each maps to an existing spine, so drafting is assembly, not invention.

2. **Add a one-line back-pointer** from each of the four existing indexes to the lineage map (e.g.
   COHESION/ECOSYSTEM/ROADMAP_TODO/MANIFEST each gain "by-thinking-type: see SOURCE-LINEAGE-MAP.md"), so
   the five slices are mutually discoverable. *Trade-off:* five tiny edits; pure upside.

3. **Reconcile the brand DRIFT** (`thinking/app/brand-and-voice-notes.md` vs `NAMING.md`) before N7's
   brand chapter is written (already tracked: COHESION §24).

**Explicitly NOT recommended — a tempting move that would violate PLAYBOOK §4.** The app-body combing
found that `seeds/multiecosystemapp-unpacked/*` and `seeds/apps-unpacked/*` are byte-identical to their
`thinking/app/` working copies, and one Explore pass suggested deleting the seeds to remove "13 files of
duplication." **Do not.** That duplication is the intended frozen-seed↔working-copy discipline
(PLAYBOOK §3b/§4); the seeds are the provenance anchor and their removal needs explicit user
authorization after a byte-identical re-verification (PLAYBOOK §2b.3). The lineage map handles this the
right way — by *labeling* the seed rows **[R] frozen** and pointing the working-copy rows at them — an
index concern, not a physical move.

**No raw relocation, ever.** Regrouping raw transcripts by thinking-type is purely an overlay in the
map; raw stays where it is, frozen.

---

## 6. Open questions & decisions for the human

1. **Where does the index live?** `discovery/SOURCE-LINEAGE-MAP.md` (top-level peer to the four
   indexes — *my recommendation*) or `narrative/SOURCE-MAP.md` (closer to consumption)?

2. **Stop at this proposal, or draft the full index now?** §4 is a real-but-partial draft (spine
   sources). Do you want the next pass to (a) extend it to *all* ~120 sources, and/or (b) draft the four
   missing verticals — or hold at the proposal for review first? *(This was one of your two pre-launch
   knobs; left open per "propose, don't resolve.")*

3. **Are N2 (history) and N3 (ecosystem comparison) two narratives or one?** They are one *kind of
   thinking* (G3) on two time-axes. I lean two narratives (a historical arc reads differently from a
   present-day comparison), but they could be two acts of one "decentralization" narrative.

4. **Does N8 (safety-under-blindness) graduate** to its own vertical, or fold into N5 as its
   adversarial chapter? It is cohesive enough to stand alone; folding keeps the protocol story in one
   place.

5. **Reshape the six planned verticals** to the lineage map's set (adding epistemic / ecosystem / app /
   safety, splitting civic-why)? See §4c.

6. **Which narrative to draft first?** N1 and N5 are the most spine-complete and least
   decision-gated; N4 and N7 are gated (legal review / IP / brand). My suggestion: N1 then N5.

7. **The standing decision gates** (yours, already in ROADMAP_TODO — flagged here only because they
   block specific narratives, not for resolution): MPL-2.0 license, recovery-anchor choice, the
   cooperative legal-review (MO Ch.351), and the CroftC Phase-0 IP/ownership call. N4 and N7 should
   carry "decision-gated" banners until these land.

---

## Definition-of-done check

This is a single proposal doc a human can read in one sitting, with a real (partial) DRAFT index, that
can be approved / redlined / redirected so the next session can (a) site and extend the index and (b)
start drafting the confirmed narratives — without re-combing the corpus. Nothing was moved, renamed, or
edited; raw stays frozen; verification status travels with every source; the four existing indexes are
linked, not forked; the human's decisions are surfaced, not resolved.
