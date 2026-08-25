# Croft — workspace orientation for agents (canonical, version-controlled)

## Identity (workspace architecture)

**Scope:** Thinking, synthesis, proofs, the Drystone spec, the backlog of record (`alpha/ROADMAP_TODO.md`), decision derivations.
**Not this repo:** product code (croft); contract text (connect); infrastructure (croft-stack).
**Provides:** spec, ADR origins, plans, registers. **Consumes:** experiment findings, transcripts.
Card + altitudes: `CroftC/.claude/ARCHITECTURE.md`.

This is the canonical agent-orientation doc. It lives in `discovery/` so it is
version-controlled; the top-level `CroftC/.claude/CLAUDE.md` imports it so Claude Code
auto-loads it when working anywhere under `CroftC/`.

`CroftC/` is not itself a git repo; it holds the work behind **Croft** — an open, sovereign,
peer-to-peer, local-first social/messaging platform meant to be run as a cooperative (non-extractive
infrastructure). The three original strands (thinking, proofs, experiments) now live in **one repo,
`discovery/`**: the standalone `Proofs/` and `experiments/` repos were folded into `discovery/alpha/`
(2026-07-15 and 2026-07-13) and are now frozen and archived (read-only). "Croft" is the name center of
gravity; the GitHub org is `CroftCommunity`. See `discovery/alpha/NAMING.md` for why.

## Maturity stages (alpha → beta → rc → publish)

As of 2026-06-24 each repo is organized as a **maturity lifecycle**: `alpha → beta → rc → publish`.
Each stage is a self-contained tree with its own linear git history; stages cohabit, and material
matures upward (alpha = first-pass, concurrently-discovered thinking → beta = resolved synthesis +
real validation → rc → publish). New material can still land in any stage, but the **current working
corpus is `alpha/`**. (Beta is built next from alpha using
`alpha/plans/2026-06-22-narrative-architecture-refactor-proposal.md` as the blueprint and
`alpha/COHESION.md` as the resolve-the-contradictions worklist.)

**Path convention for this file and `PLAYBOOK.md`:** any path that names *corpus* content (`seeds/…`,
`research/…`, `thinking/…`, `crystallized/…`, `narrative/…`, `plans/…`, `COHESION.md`,
`ECOSYSTEM.md`, `ROADMAP*.md`, `NAMING.md`, `ANALYSIS.md`, the dossier, the raw archive) is under the
**current stage dir** — i.e. `discovery/alpha/<path>`. Only **`AGENTS.md`** (this file),
**`PLAYBOOK.md`**, and a thin stage-pointer **`README.md`** live at the repo **root**: they describe
*how we work*, not the project, so they span stages. The folded-in proofs and experiments corpora sit
under `discovery/alpha/Proofs/` and `discovery/alpha/experiments/` and share discovery's staging
(everything imported is alpha-tier).

**Experiments now live *inside* discovery (2026-07-13).** The code-forward experiment corpus was
folded into **`discovery/alpha/experiments/`** so discovery and experimentation stay tight; the
standalone `experiments/` repo is **frozen** (superseded, read-only, no new work). Its own
`alpha/beta/rc/publish` staging collapses into discovery's — everything imported is alpha-tier and
sits under `alpha/experiments/`. The experiments↔spec bridge is documented in
`alpha/experiments/SPEC-ALIGNMENT-AND-ACTION-PLAN.md` (+ the `SPEC2-OVERLAY.md` and the proposed
spec-change diff under `beta/drystone-spec/`).

**Tier cleanliness (matured docs read clean).** A doc at a given stage carries **no references back to a
prior stage** — no sources footer, no provenance trace, no prior-stage file paths or seam-map pointers
inside it. The full "what was pulled, how it was treated, where it landed" map lives **only** in the prior
stage's rollup ledger (e.g. `alpha/BETA-ROLLUP.md`); unsettled threads wait in that stage's
`OPEN-THREADS.md` (with their connective tissue) until they land, then the content moves up clean and the
mapping moves into the rollup. The discipline tightens upward — `rc`/`publish` are clean of prior-tier
traces entirely. Full method in `MATURITY-ROLLUP.md` (§1, §3, §3a, §3b).

## The repos (discovery is the single active repo)

The three original strands now live in one repo. The standalone `Proofs/` and `experiments/` repos are
frozen and archived; their corpora were folded under `discovery/alpha/`.

```
discovery/    Thinking, synthesis, proofs, and experiments. The map of everything.   root: AGENTS.md · PLAYBOOK.md · README.md
                alpha/  seeds/ (raw source incl. transcripts/raw verbatim archive) · research/
                  (industry comparison) · thinking/ (our design) · crystallized/ (principles +
                  proof-ledger) · narrative/ · plans/ · ECOSYSTEM.md · COHESION.md · ROADMAP.md ·
                  ROADMAP_TODO.md · NAMING.md · ANALYSIS.md · the dossier
                  alpha/Proofs/       durable proofs (real openmls) — folded in 2026-07-15
                  alpha/experiments/  code-forward spikes — folded in 2026-07-13
                beta/   (resolved synthesis — eight themes + OPEN-THREADS staging ledger;
                  the resolved spec at beta/drystone-spec/, Part 1 + Part 2)
Proofs/       [FROZEN + archived 2026-07-15 — folded into discovery/alpha/Proofs/]
experiments/  [FROZEN + archived 2026-07-13 — folded into discovery/alpha/experiments/]
discovery/alpha/Proofs/        lineage-groups (real openmls) · lineage-group-model (TS) ·
                  encrypted-local-first-atproto
discovery/alpha/experiments/   appview-validation · public-roundtrip · android-p2p-app ·
                  encrypted-blob-share · croft-app-phase0 · croft-group · iroh ·
                  automerge-partial-reconstruction · local_storage_projection · mls-replant ·
                  replant-continuity · croft-chat  (+ MASTER-INDEX · EXPERIMENT-BACKLOG ·
                  SPEC-DIVERGENCE-REGISTER · SPEC-ALIGNMENT-AND-ACTION-PLAN · SPEC2-OVERLAY)
```

## Calling surfaces: connect · croft · relay (who owns what)

`discovery` is the thinking repo; the **product** of Croft calling is built across
three sibling repos under `CroftC/`. They are easy to confuse — keep the ownership
straight (two concurrent sessions nearly forked the contract 2026-08-16):

```
CroftCommunity/connect   CONTRACT OWNER + directory/status web + stopgap android
                         docs/contract.md = the canonical calling contract (lexicon,
                         croftcall:// deep link, cap model). web/ = the exchange page
                         (handle → DID → PDS → endpoint, callability, cap redeem;
                         Pages at connect.croft.ing). android/ = a stopgap receiver
                         (shipped v0.1.0 APK) — keep minimal. Has its own CLAUDE.md.

croft                    THE CLIENT (new). Shared Rust core + web/android/apple
                         shells. A declared CONSUMER of connect's contract — its
                         docs/CONTRACT.md points at connect as canonical. The real
                         calling client lives here.

relay = croft-stack      MEMBERSHIP / admission backbone (CISS accounting, budgets,
                         call-time). Not connect, not the client.
```

- **The two android apps are one app, converging:** `connect/android` (stopgap)
  folds into `croft/android` (the real client). Client-side cap consumption belongs
  in `croft/android`.
- **The contract is connect's to own;** breaks are deliberate + coordinated, never
  by drift. Consumers track it. Plan + milestones:
  `alpha/plans/2026-08-14-1-plan-connect-cap-issue-redeem.md`.

## Start here (in this order)

1. `discovery/alpha/README.md` — the corpus map (the root `README.md` covers only the stage layout).

2. `discovery/PLAYBOOK.md` (root) — **how we process incoming narrative, experiments, and proofs.**
   Follow it every time new material arrives (classify → place → verify verbatim → capture
   conversation + raw transcript → update ledger/cohesion/roadmap/manifest). Canonical process.

3. `discovery/alpha/COHESION.md` — where one document's loose end is closed (or duplicated) by
   another's proof. Read before concluding anything is "unproven."

4. `discovery/alpha/crystallized/` — `principles.md` (design + civic + product) and
   `proof-ledger.md` (every invariant/experiment with status + link to its proof).

5. `discovery/alpha/SOVEREIGN-COMMONS-DOSSIER.md` — the umbrella vision (pre-"Croft" naming).

## Reference indexes, filing & the backlog (where things go)

File new material the same way every time — `PLAYBOOK.md` is the canonical process (classify →
preserve raw verbatim → distill → update connective tissue). The standing indexes to reach for:

- **`ROADMAP_TODO.md`** — the single **provenance-indexed backlog** of open items (origin
  `file:line` + a durable section-header key). Add new open items *here* rather than starting a
  parallel list; `ROADMAP.md` carries the reasoning, this aggregates it. "Roadmap possibles" live
  here.

- **`ROADMAP-BOARD.md`** (repo root) — how the backlog is worked on the GitHub Projects kanban
  (`CroftCommunity/Croft Roadmap`): the product lanes (the `beta/LAYERS.md` layer cake plus a
  cross-cutting `Decisions & Gates` lane), the `Backlog → Ready → Parked → WIP → Done` columns, and the
  convention that the board is a *curated* working surface drawn from `ROADMAP_TODO.md` — not a second
  backlog. Graduate items onto the board; do not re-list them.

- **Workspace tracking scheme** — `CroftC/.claude/TRACKING.md` routes the whole workspace:
  this file's backlog remains the backlog of record; per-repo `TODO.md` files are ops-only
  behind a scope header; plan files follow the dated naming + Status + reasoning convention;
  plan-scoped IDs (M/O/plan-local D) are qualified with their plan when cited outside it.

- **Workspace decision registry** — `CroftC/.claude/DECISIONS.md`: tagged, greppable index of
  design decisions and prior art across all repos (grep it before choosing a library or building
  a capability). Anti-rollup: rows point into the reasoning homes — this repo's registers
  (`beta/DECISIONS.md`, `alpha/ROADMAP_TODO.md` §A, `crystallized/principles.md`) stay
  authoritative; the workspace file only routes.

- **`ECOSYSTEM.md`** — the relational register of related projects/tools (homage / build-on /
  partner / rebroadcast / learn↔). Add or update a row whenever new material names an
  org/project/tool. (§5b atmospheric-web apps; §5c app-layer tooling/clients.)

- **`seeds/transcripts/RAW-ARTIFACTS-MANIFEST.md`** + `seeds/transcripts/raw/README.md` — what raw
  came in and its preservation status (**verbatim** / **cleaned-paste** / condensed / distilled).

- **`COHESION.md`** — the seam-tracker (a loose end ↔ the proof/spike/doc that closes it). Check
  before declaring anything "unproven."

- **Fact-check sources of truth — cite, do NOT independently re-verify:** atproto / iroh / iOS-P2P
  facts are settled in `seeds/transcripts/raw/atproto-atmospheric-web-iroh-mobile-FACTCHECK.md`
  (~75 claims, verdicts + URLs). Align to its corrections, notably: **iroh is `1.0.0`** (companion
  crates pre-1.0 — iroh-docs/gossip `0.100`, iroh-blobs `0.102`; `NodeId`→`EndpointId`; relays were
  formerly "DERP"); iroh-docs uses **range-based set reconciliation + LWW**, not Merkle Search Trees
  (an AT-Proto structure Gemini conflated); there is **no native AT-Proto E2EE / "AT Messaging"
  working group** (REFUTED) — real AT-Proto E2EE is third-party (Germ MLS, XMTP↔Bluesky bridge),
  the gap Croft's lineage-groups MLS proof answers.

## Working rules

- **Git identity:** chasemp account — `git@github-personal:CroftCommunity/<repo>.git`, committer
  `Chase Pettet <chase@owasp.org>`. `discovery` is the active repo; the folded `Proofs` and
  `experiments` repos are frozen and archived. Reading croftc PRs uses the `cpettet_croftc` gh account
  (`gh auth switch`).

- **Don't commit / push / open PRs** on these repos unless explicitly asked — material is
  reviewed first. When asked to commit, see PLAYBOOK §3b.

- **Concurrent sessions:** multi-turn work happens in a worktree (`worktrees/discovery/<slug>`,
  branch `claude/<slug>`), never in the shared checkout; contested surfaces (`ROADMAP_TODO.md`,
  the ledgers, this file) are claimed first. Protocol + reasons:
  `CroftC/.claude/COORDINATION.md`.

- **Provenance is non-negotiable:** keep raw artifacts verbatim; redact only secrets; mark
  volatile facts `[UNVERIFIED]`; distinguish modeled-vs-real for proofs; don't over-claim.

- **A prior measurement carries its configuration — check it before reusing the number.** Citing a
  measured figure instead of re-deriving it is right (that is what the FACTCHECK files are for), but
  a number is only evidence about the configuration it was taken under. Worked example, 2026-08-08:
  the corpus's Welcome-size figure (~152–155 B/member, E12.1) was cited as bounding the `Welcome`
  vs 2 MiB risk. It does not — `mls-replant` builds every group with
  `MlsGroupCreateConfig::default()`, whose `use_ratchet_tree_extension` is `false`, so **every
  Welcome measurement in the corpus was the without-tree case: the safe case, not the risk case.**
  The O(N) object the question was actually about had never been measured here at all (first data:
  `experiments/meer-queue/PHASE-0-FINDINGS.md` D7, where the extension roughly doubles per-member
  cost). Before reusing a measurement, name the configuration it was taken under and confirm it is
  the one you are reasoning about.

- **Before inventing a seam, grep the substrate for one.** Same session: a spike planned its own
  `Clock` trait for deterministic time, when `CISS/src/clock.rs` already had a public, tested
  `SimClock` built for exactly that ("no wall-clock reads", day granularity). A parallel seam beside
  an existing one is a divergence nobody declared. The find was forced by treating an "obvious"
  stand-in as a decision requiring a probe rather than an acknowledgement — which is the general
  rule: **a stand-in the agent finds convenient is the kind that should need a decision.**

- **Don't resolve the user's decisions** (license gates, recovery-anchor choice, etc.) —
  surface them.

- **No emojis in documents.** Markdown docs, plans, READMEs, session logs, status boards — use
  plain-text status tokens (e.g. `DONE` / `LIVE` / `PENDING` / `PARTIAL` / `n/a`), not emoji.
  ASCII box-drawing and arrows (`┌ │ └ → ▼`) are fine — those are diagrams, not emoji.
  (Tolerated exception: the `ROADMAP_TODO.md` backlog's existing status markers.)

## Headline state (keep fresh; mirrors PLAYBOOK §7)

- Lineage-groups Phase 1 crypto gate is **GO** on real openmls 0.8.1 (survivor-epoch re-key
  with post-compromise security).

- Biggest open problem: multi-device + total-device-loss recovery (backup-vs-delegation fork).

- **New body (2026-06-22): the app / client layer** ("Croft" the product) — a composable
  garden of **ponds** (Bluesky/Mastodon/Lemmy, native) + **pads** (small apps), with the **Croft
  Group** pond = lineage-groups surfaced on iroh. Design at `thinking/app/`; dialogue at
  `seeds/transcripts/raw/croft-app-design-dialogue-2026-06-20-to-22.md`. Phase 0 built externally
  (**CroftC PR #10**) — import deferred (the IP/ownership call is the user's; ROADMAP §13). Top
  open risk: infra-sustainability ↔ the cooperative *mechanism* (open-considerations §8).

- **The "why" foundation (2026-06-20): the design-imperative body** — the deepest grounding yet. A
  cross-field, cross-millennium lineage (Socrates→Mill→Peirce/Popper→Hayek→Ostrom→Ashby→Beer→Scott;
  `narrative/lineage-of-a-design-imperative.md`) and the protocol-substrate architecture it implies
  (`thinking/local-first-as-design-imperative.md`), distilled to `crystallized/principles.md` ("The
  deeper foundation"). The razor: **compute provenance, never utility**; **local-first state is the
  generative premise** (architecture = epistemology); **no right to remove the rights of others**.
  Distinct from the *app/client*-layer philosophy in `thinking/app/`. Top open frontier: the
  centerless-meets-center seam (ROADMAP_TODO D8).

- Provenance: complete — transcripts/PRs filed (see RAW-ARTIFACTS-MANIFEST.md); the app dialogue
  is a content-faithful **cleaned-paste** (no pristine export existed), labeled per PLAYBOOK §4.
