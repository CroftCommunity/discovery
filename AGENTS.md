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

- Provenance: complete — all transcripts/PRs filed verbatim (see RAW-ARTIFACTS-MANIFEST.md).
