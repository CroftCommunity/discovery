# Croft — workspace orientation for agents (canonical, version-controlled)

This is the canonical agent-orientation doc. It lives in `discovery/` so it is
version-controlled; the top-level `CroftC/.claude/CLAUDE.md` imports it so Claude Code
auto-loads it when working anywhere under `CroftC/`.

`CroftC/` is not itself a git repo; it holds three sibling repos that together hold the
thinking, proofs, and experiments behind **Croft** — an open, sovereign, peer-to-peer,
local-first social/messaging platform meant to be run as a cooperative (non-extractive
infrastructure). "Croft" is the name center of gravity; the GitHub org is `CroftCommunity`.
See `discovery/NAMING.md` for why.

## The three repos

```
discovery/    Thinking & synthesis. The map of everything.
                seeds/ (raw source incl. transcripts/raw verbatim archive) · research/
                (industry comparison) · thinking/ (our design) · crystallized/ (principles +
                proof-ledger) · narrative/ · ECOSYSTEM.md · COHESION.md · ROADMAP.md ·
                NAMING.md · ANALYSIS.md · PLAYBOOK.md · AGENTS.md (this file) · the dossier
Proofs/       Durable proofs — verify an invariant that becomes a design principle.
                lineage-groups (real openmls) · lineage-group-model (TS) ·
                encrypted-local-first-atproto
experiments/  Code-forward spikes — "does this work / what's actually true?"
                appview-validation · public-roundtrip · android-p2p-app · encrypted-blob-share
```

## Start here (in this order)

1. `discovery/README.md` — the repo map.

2. `discovery/PLAYBOOK.md` — **how we process incoming narrative, experiments, and proofs.**
   Follow it every time new material arrives (classify → place → verify verbatim → capture
   conversation + raw transcript → update ledger/cohesion/roadmap/manifest). Canonical process.

3. `discovery/COHESION.md` — where one document's loose end is closed (or duplicated) by
   another's proof. Read before concluding anything is "unproven."

4. `discovery/crystallized/` — `principles.md` (design + civic + product) and
   `proof-ledger.md` (every invariant/experiment with status + link to its proof).

5. `discovery/SOVEREIGN-COMMONS-DOSSIER.md` — the umbrella vision (pre-"Croft" naming).

## Reference indexes, filing & the backlog (where things go)

File new material the same way every time — `PLAYBOOK.md` is the canonical process (classify →
preserve raw verbatim → distill → update connective tissue). The standing indexes to reach for:

- **`ROADMAP_TODO.md`** — the single **provenance-indexed backlog** of open items (origin
  `file:line` + a durable section-header key). Add new open items *here* rather than starting a
  parallel list; `ROADMAP.md` carries the reasoning, this aggregates it. "Roadmap possibles" live
  here.

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

- **Git identity:** chasemp account on all three repos —
  `git@github-personal:CroftCommunity/<repo>.git`, committer `Chase Pettet <chase@owasp.org>`.
  Reading croftc PRs uses the `cpettet_croftc` gh account (`gh auth switch`).

- **Don't commit / push / open PRs** on these repos unless explicitly asked — material is
  reviewed first. When asked to commit, see PLAYBOOK §3b.

- **Provenance is non-negotiable:** keep raw artifacts verbatim; redact only secrets; mark
  volatile facts `[UNVERIFIED]`; distinguish modeled-vs-real for proofs; don't over-claim.

- **Don't resolve the user's decisions** (license gates, recovery-anchor choice, etc.) —
  surface them.

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

- Provenance: complete — transcripts/PRs filed (see RAW-ARTIFACTS-MANIFEST.md); the app dialogue
  is a content-faithful **cleaned-paste** (no pristine export existed), labeled per PLAYBOOK §4.
