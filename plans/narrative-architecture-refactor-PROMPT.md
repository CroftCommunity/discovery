# Handoff prompt — "Narrative-architecture refactor" (propose-only pass)

> This file is a **ready-to-use prompt**. After `/clear`, paste it (or just say:
> *"Read `discovery/plans/narrative-architecture-refactor-PROMPT.md` and do exactly that."*).
> It is self-contained — assume the agent has none of the prior conversation's context.

---

You are doing a **design/narrative refactor pass** on the CroftC `discovery` corpus. This is **NOT
a code refactor** and **NOT a "make edits" task**. Your entire job is to **comb the whole corpus,
re-group every source by *type of thinking / intellectual lineage*, and produce a written PROPOSAL
+ a draft cross-reference index** — so that cohesive narratives can later be written from it. You
**propose**; the human decides and approves before anything is moved or rewritten.

## Why this exists (the gap you are filling)

The corpus already has **four index slices**, but each cuts a different way and **none groups by
type of thinking**:
- `COHESION.md` — by **seam** (a loose end ↔ the work that closes it).
- `seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md` — by **provenance** (what raw came in, fidelity, fact-check status).
- `ECOSYSTEM.md` — by **project relationship** (homage / build-on / partner / learn↔).
- `ROADMAP_TODO.md` — by **open backlog item**.

The **missing slice is by intellectual lineage / kind of thinking** (epistemology, history,
ecosystem comparison, civic/economics, protocol, identity, product, …). That slice is what makes it
possible to assemble **narratives**. Produce it.

## Orientation — read these FIRST (in order), then build a mental map

Working dir: `/Users/cpettet/git/chasemp/CroftC/discovery` (sibling repos `../Proofs`, `../experiments`).
1. `discovery/AGENTS.md` — canonical orientation. Note the named "bodies" already identified (the
   **design-imperative / "why" body**; the **app/client layer**; the protocol; the civic/dossier).
2. `discovery/PLAYBOOK.md` — the filing process + **provenance discipline** (§4). Internalize it; you
   must not violate it.
3. `discovery/README.md` — the repo map.
4. Skim the four existing indexes above so your new slice **complements** them, never forks/duplicates.

## The corpus terrain (comb ALL of it)

- `seeds/` — **raw, frozen** source. Especially `seeds/transcripts/raw/*` (≈a dozen+ Gemini design
  dialogues, **each with a companion `*-FACTCHECK.md`** carrying CONFIRMED/PARTLY/REFUTED/UNVERIFIABLE
  verdicts), plus `seeds/transcripts/design-dialogue-*`, `seeds/groupdynamics-unpacked/`,
  `seeds/generated-prompts/`, `seeds/*-unpacked/`.
- `research/` — industry comparison / analytical-lens deliverables.
- `thinking/` — our evolving design (incl. `thinking/app/` + `thinking/app/ponds/` = the app/client body).
- `crystallized/` — `principles.md` (design + civic + the "deeper foundation"), `proof-ledger.md`,
  `test-narrative.md`, `CROFT-PROTOCOL.md`, `conformance-suite.md`, `TEST-CORPUS.md`.
- `narrative/` — `long-form.md`, `short.md`, `verticals/`, `lineage-of-a-design-imperative.md`,
  `messaging-and-quotes.md` (the marketing/quotes reservoir).
- Standing docs: `SOVEREIGN-COMMONS-DOSSIER.md`, `NAMING.md`, `ANALYSIS.md`, `ROADMAP.md`, + the 4 indexes.
- Sibling repos hold proofs/experiments (`Proofs/`, `experiments/`) — reference for what's *proven*
  vs *thought*, but the narrative sources live in `discovery`.

**Scale tactic:** this is large. Use parallel read-only **Explore** subagents to inventory clusters
concurrently (e.g., one per top-level area), then synthesize. **Read the FACTCHECK files** — verification
status must travel with every source you cluster (a narrative built on REFUTED/UNVERIFIED material is a
liability).

## The human's starting hypothesis (validate, refine, expand — don't just accept)

They believe there is **more than one** cohesive narrative in here, and named three candidates:
1. **Epistemological / philosophical grounding** — *why starting a technical solution's shape from
   philosophy is beneficial.* (Likely spine: the **design-imperative body** —
   `narrative/lineage-of-a-design-imperative.md`, `thinking/local-first-as-design-imperative.md`,
   `crystallized/principles.md` "deeper foundation"; the Socrates→Mill→Peirce/Popper→Hayek→Ostrom→
   Ashby→Beer→Scott lineage; the crypto-wars liberty lineage; Bazelon/Carterfone rights; the
   corporation-vs-person / overjustification material.)
2. **History of decentralization** — federated + peer-to-peer. (Likely spine: the P2P-origin dialogue,
   `research/` messaging-landscape, the crypto-wars→P2P "four-property impossibility," the protocol
   landscape (SSB/Briar/Matrix/Cwtch/Quiet/SimpleX/Keet/Wesh/Peat), Steem→Hive/DSNP/Solid/atproto
   history, the Ma-Bell→Carterfone telecom arc.)
3. **The ecosystem & how options compare.** (Likely spine: `ECOSYSTEM.md`, `research/` comparisons,
   the sovereign-AppView club, Solid/DSNP/Frequency, atproto atmospheric-web, iroh ecosystem.)

**Look hard for additional threads** — strong candidates the corpus may support: a **civic /
cooperative-economics** narrative (cooperative-social-union, Ostrom, coops/PBC/DAO, surveillance
capitalism, the durable-maintenance-vs-extractive-attention frame, sustainability↔cooperative
mechanism); a **product / app-layer** narrative (`thinking/app/`, ponds/pads); an
**identity & provenance** narrative (did:plc / did:webvh / cross-platform); a **protocol / crypto-proof**
narrative (lineage-groups MLS proofs, CROFT-PROTOCOL). Decide which are real, cohesive threads vs
sub-threads, and say why.

## Deliverables (write these; do not touch anything else)

Write a single proposal doc at **`discovery/plans/2026-06-22-narrative-architecture-refactor-proposal.md`**
(create `plans/` if needed). It must contain, with **Problem Statement / Approach / Reasoning** up top
(per the repo's plan-doc rule):

1. **Proposed taxonomy** — the set of *thinking-type / lineage* groups, each with a one-line definition
   and the boundary that separates it from its neighbors. Note overlaps explicitly.
2. **Per-group source inventory** — for each group, the sources that belong to it (path + one-line role
   + verification status from its FACTCHECK), marked **spine** (load-bearing) vs **supporting** vs
   **provenance-only/raw**. A source may appear in >1 group — note the cross-reference.
3. **Candidate narratives** — for each cohesive narrative you confirm, name it, state its thesis in one
   sentence, list its spine sources in reading order, and note what's missing / unproven / decision-gated.
4. **The recombed cross-reference index (design + first draft)** — propose a new standing index
   (suggested: `discovery/SOURCE-LINEAGE-MAP.md` or `narrative/SOURCE-MAP.md`) that maps every source →
   its thinking-type group(s) → the narrative(s) it feeds, with bidirectional cross-refs. **Draft it**
   (it can be partial), and state how it relates to the four existing indexes (it should *link to*, not
   duplicate, COHESION/MANIFEST/ECOSYSTEM/ROADMAP_TODO).
5. **Restructuring recommendations** — if (and only if) physical moves/renames/new-subdirs would help,
   propose them **with rationale, trade-offs, and a migration sketch** — but as a recommendation for
   human approval, executed later, never now. Default to an **overlay index** over physical moves where
   possible (cheaper, reversible, provenance-safe).
6. **Open questions & decisions for the human** — anything genuinely their call (which narratives to
   build first, where the index lives, any contested grouping). Surface, don't resolve.

## Hard constraints

- **PROPOSE ONLY.** Do not move, rename, delete, or rewrite any existing file. Do not edit COHESION /
  ECOSYSTEM / MANIFEST / principles / etc. The only file you create is the proposal doc (and optionally
  the *draft* index, clearly labeled DRAFT/PROPOSED). Everything else is read-only.
- **Raw seeds are frozen** (PLAYBOOK §4). Never propose relocating raw transcripts; regrouping raw is an
  **index/overlay** concern, not a physical move.
- **Verification status travels with every source.** Pull each transcript's FACTCHECK verdict; flag
  dialogue-sourced / UNVERIFIED / REFUTED material so narratives aren't built on sand. (Note: the project
  source-of-truth for atproto/iroh/iOS facts is `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`
  + its addenda — cite, don't re-verify.)
- **Complement, don't fork, the existing indexes.** If your new index would duplicate COHESION/MANIFEST/
  ECOSYSTEM/ROADMAP_TODO, link to them instead.
- **Don't resolve the human's decisions** (license, recovery-anchor, cooperative mechanism, which name,
  etc.) — list them.
- **No commits, no pushes** — this repo set is reviewed first; leave the proposal uncommitted for review.

## Definition of done

A single, well-structured proposal doc (+ optional DRAFT index) that a human can read in one sitting and
either approve, redline, or redirect — such that the *next* session can execute the regrouping/index and
then start drafting the confirmed narratives. Be concrete (real paths, real source names), opinionated
(recommend, don't just enumerate), and honest about overlaps, gaps, and verification status.
