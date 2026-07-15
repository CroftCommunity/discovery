# discovery — Croft thinking & synthesis (staged)

**Published Drystone spec site:** https://croftcommunity.github.io/discovery/ — Part 1, Part 2, and the
companions with every cross-reference as a followable link. Built by `site/build.py`; the same build is a
broken-reference gate on every push and PR (see `site/README.md`).

This repo is organized as a **maturity lifecycle**: `alpha → beta → rc → publish`. Each stage is a
self-contained tree with its own linear git history; the stages cohabit, and material matures upward.

- **`alpha/`** — the current working corpus: first-pass, concurrently-discovered thinking, research,
  narrative drafts, and the standing indexes (`COHESION.md`, `ECOSYSTEM.md`, `ROADMAP_TODO.md`,
  `ROADMAP.md`, `NAMING.md`, the raw-transcript archive under `seeds/`). New material currently lands
  here. Once `beta/` exists, treat `alpha/` as the frozen substrate (paths inside it are relative to
  it and true as of the freeze).
- **`beta/`** — *(not yet created)* the resolved synthesis: contradictions collapsed, organized by
  narrative thread, structured for synthesis + real validation rather than initial thinking. Built
  from `alpha/` using `alpha/plans/2026-06-22-narrative-architecture-refactor-proposal.md` as the
  blueprint and `alpha/COHESION.md` as the resolve-worklist. Beta docs reference beta docs, with a
  "Sources (alpha)" pointer down.
- **`rc/`, `publish/`** — later maturity stages.

**Cross-stage docs at the repo root** (they describe *how we work*, not the project, so they span
stages):

- **`AGENTS.md`** — agent orientation; the canonical doc the harness auto-loads. Start here.
- **`PLAYBOOK.md`** — the intake / filing process.

Sibling repos `Proofs/` and `experiments/` use the same `alpha → beta → rc → publish` staging,
aligned stage-for-stage. Git identity: chasemp (`chase@owasp.org`, `github-personal`). Don't
commit/push unless asked.
