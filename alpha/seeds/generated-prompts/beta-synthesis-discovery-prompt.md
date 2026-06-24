# Prompt: Kick off the `beta` synthesis for `discovery`

Copy this into a fresh session to begin populating `discovery/beta/`. It is **comprehensive but still
a pointer** — the canonical rules live in `discovery/AGENTS.md`, `discovery/PLAYBOOK.md`, the refactor
plan, and `discovery/alpha/COHESION.md`. Follow those; don't re-derive them here.

---

You're in the CroftC workspace (`/Users/cpettet/git/chasemp/CroftC`). I want to start the **beta
synthesis for the `discovery` repo**. This is a maturation pass, not new intake. **Don't move, edit,
delete, or rewrite anything in `alpha/` — it is the frozen provenance floor. Don't commit or push until
I approve; show me the plan first (PLAYBOOK §3b).** Git identity: chasemp (`chase@owasp.org`,
`github-personal`).

## Where things stand (the staging model)

Each repo is staged `alpha → beta → rc → publish` (see `discovery/AGENTS.md` → "Maturity stages").
`discovery/`'s entire first-pass corpus is **frozen under `discovery/alpha/`**. Cross-stage process
docs live at the `discovery/` root: `AGENTS.md` (orientation, auto-loaded), `PLAYBOOK.md` (intake
process), `README.md` (stage map). `Proofs/` and `experiments/` are staged the same way, aligned
stage-for-stage.

**ORIENT FIRST (read before doing anything — these are the source of truth):**
- `discovery/AGENTS.md` — the staging model + the path convention (corpus paths are under `alpha/`).
- `discovery/PLAYBOOK.md` — process + the commit discipline (§3b).
- `discovery/alpha/plans/2026-06-22-narrative-architecture-refactor-proposal.md` — **the blueprint**:
  seven lineage groups (G1–G7) → eight candidate narratives (N1–N8), reading-order spines, and a
  §3a "do-not-build-on-sand" verification-flag list. Treat it as a strong draft to **re-validate**,
  not gospel (it predates this whole session's intakes — see Step 1).
- `discovery/alpha/COHESION.md` — **the contradiction worklist**: ~36 seam entries, several flagged
  DRIFT / "declared-open-here, walked-out-there." This is the list of things beta must resolve.
- `discovery/alpha/README.md` — the corpus map.

## The goal of beta

Build `discovery/beta/`: the **resolved, cohesive, themed synthesis** of the alpha corpus. Specifically:
1. **Pull together split threads.** Where one transcript or analysis left a piece unfinished and
   another file completed it, write the single combined account. Stop making the reader re-comb ~120
   files to assemble one idea.
2. **Collapse contradictions.** Where alpha declares something "unsolvable / open" in one place and
   walks it out in another, beta states the resolved conclusion once (using COHESION as the worklist).
3. **Harvest conclusions documented-but-missed** — decisions/findings that landed in a transcript or a
   dialogue but never got pulled up into the synthesis layer. Surface them as you comb.
4. Structure for **synthesis + real validation**, not first-pass thinking.

## Tier discipline (the core invariant — do not break it)

- **Each tier is cohesive within itself: beta docs reference only beta docs** for their forward
  narrative.
- **Provenance flows down.** Every beta doc ends with a `Sources (alpha)` list pointing at the alpha
  files it synthesizes. Optionally add a single `→ graduated to beta/<x>` pointer on the *major* alpha
  spine docs (that is the only edit allowed in alpha, and only if cheap; otherwise leave alpha
  untouched and keep the linkage one-directional from beta down).
- **Alpha is frozen.** Never move/edit/delete alpha content (PLAYBOOK §4). Raw stays frozen forever.
  Keeping vs. eventually dropping the alpha tier is my call later; default is keep-for-provenance.

## Step 1 — re-validate the blueprint against the CURRENT frozen alpha

The refactor plan is dated 2026-06-22 and **predates this session's intakes.** Re-comb `alpha/` and
refresh the G1–G7 / N1–N8 taxonomy against what's actually there now. Known additions to fold in (find
these and anything else that landed):
- `alpha/narrative/verticals/croft-the-name-and-the-commons.md` — a drafted vertical (the plan said
  verticals were empty); the etymology / commons-inversion / global-enclosure thread (G3a/G1/G2).
- `alpha/seeds/transcripts/raw/croft-clare-enclosure-poems-2026-06-23.md` — public-domain Clare poems
  (companion source to the above).
- `alpha/thinking/foundation-and-ip-stewardship.md` + the `alpha/NAMING.md` **"Noria" foundation-name
  CANDIDATE** section — a major new G2 body (code/brand/coop three-layer IP stewardship, AGPL+DCO lock,
  fiscal-sponsor phasing). **NOT-LEGAL-ADVICE; Noria is a candidate pending legal clearance, not
  decided.**
- `alpha/thinking/membership-vs-access-the-public-door.md` — the stake-vs-access decoupling (G6/G7 +
  the ten-second-door / E11).
- `alpha/research/discord-dominance.md` "Update 2026-06-22" — IPO + moderator-labor-as-captured-value
  (G2/G3b counter-illustration).
- `alpha/COHESION.md` now runs to §36; `alpha/ROADMAP_TODO.md` to E29.

Deliver a short **"what changed since the plan"** delta and the **validated theme list** for beta.

## Step 2 — propose the beta structure, then WAIT for approval

Propose the `discovery/beta/` layout: a `beta/README.md` (the beta map) + one synthesis doc per
validated theme, each in reading order with (a) the one-line thesis, (b) what contradictions/seams it
resolves, (c) its alpha sources. Surface the open structural calls the plan flagged (still mine to
decide): N2 history vs N3 ecosystem as one narrative or two; does N8 "safety-under-blindness" graduate
to its own theme or fold into N5; how to split N1 epistemic upstream of N2; which themes are
spine-complete vs decision-gated. **Stop and get my approval before drafting.**

## Step 3 — draft theme-by-theme (after approval)

Per theme: write the single resolved account; resolve its COHESION seams + DRIFT flags; harvest the
"documented-but-missed" conclusions; carry verification status on every claim; end with `Sources
(alpha)`. Draft least-gated first (N1 epistemic foundation and N5 the proven protocol are the most
spine-complete; N4 cooperative and N7 app are decision-gated).

## Hard guardrails — verification travels with every claim (from the plan's §3a)

Do **not** carry these into beta:
- The crofting **"ancient free clan" myth** (use `croft-crofting-research.md` as truth; the narrative
  file is color only).
- The **REFUTED** crypto-wars fabrications (Zimmermann "Stalin" quote, the Meyer letter, "Voskop") and
  Hard Fork 23 "$5M" (it's ≈ $6.3M / 23.6M STEEM); **did:key** atproto-resolvable (REFUTED — plc+web).
- The **MO §351 statute/fee specifics** and any cooperative/IP/legal/financial dollar figures —
  **NOT-LEGAL-ADVICE**; carry the *reasoning*, not the citations; mark `[UNVERIFIED]`.
- Discord financials (private company; third-party estimates) — mark `[UNVERIFIED]`.
- **atproto / iroh / iOS facts:** cite the FACTCHECK source-of-truth
  (`alpha/seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`) — iroh `1.0.0`; no
  native AT-Proto E2EE; range-set reconciliation not MST. Don't re-verify.
- Confirm Peirce / Popper / Ostrom exact wording against primary editions before any external publish.
- Reconcile the brand DRIFT (`alpha/thinking/app/brand-and-voice-notes.md` vs `alpha/NAMING.md`) before
  any brand chapter.

**Surface the standing decisions, don't resolve them:** MPL-2.0 license, total-device-loss
recovery-anchor, the cooperative legal-review gate (MO Ch.351), the CroftC Phase-0 IP/ownership call,
and the still-open **load-bearing-few principles (genome vs strategy)** question. Beta docs for gated
themes carry a "decision-gated" banner.

## Scope of THIS effort (and what's explicitly NOT)

- **In scope: `discovery/beta/` only.**
- **Not now (flag as the next two efforts):**
  - **experiments-beta** — most spikes are still genuinely alpha-maturity; they stay in
    `experiments/alpha/` and keep getting built up there. Don't graduate them yet.
  - **Proofs-beta** — needs its own dedicated pass: strip the odd point-in-time / specific-conversation
    / analogy references in the proofs that don't make sense out of context, and build a comprehensive
    view of **how the proofs relate to the discovery themes and the experiment content**. The proofs
    are strong; the connective framing is what needs cleanup. Treat as a separate effort.

## Definition of done for the first pass

A `discovery/beta/` with a `README.md` (the beta map), the validated theme list, and the first one or
two least-gated themes drafted as resolved synthesis docs with `Sources (alpha)` footers — produced
without touching `alpha/`, with verification status on every claim, and with my decisions surfaced not
resolved. Show me the Step-1 delta and the Step-2 structure proposal and wait for approval before
drafting.
