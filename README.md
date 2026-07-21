# discovery — Croft thinking & synthesis (staged)

**Published Drystone spec site:** https://croftcommunity.github.io/discovery/ — Part 1, Part 2, and the
companions with every cross-reference as a followable link. Built by `site/build.py`; the same build is a
broken-reference gate on every push and PR (see `site/README.md`).

[![pages](https://github.com/CroftCommunity/discovery/actions/workflows/pages.yml/badge.svg)](https://github.com/CroftCommunity/discovery/actions/workflows/pages.yml)

This repo is organized as a **maturity lifecycle**: `alpha → beta → rc → publish`. Each stage is a
self-contained tree with its own linear git history; the stages cohabit, and material matures upward.

- **`alpha/`** — the first-pass corpus: concurrently-discovered thinking, research, narrative drafts,
  the standing indexes (`COHESION.md`, `ECOSYSTEM.md`, `ROADMAP_TODO.md`, `ROADMAP.md`, `NAMING.md`,
  the raw-transcript archive under `seeds/`), and the folded-in code-forward experiments
  (`alpha/experiments/`) and durable proofs (`alpha/Proofs/`). New intake and new experiment work land
  here; `alpha/` is the frozen provenance floor for everything matured above it (paths inside it are
  relative to it and true as of the freeze).
- **`beta/`** — the resolved synthesis: contradictions collapsed, organized by narrative thread,
  structured for synthesis + real validation rather than initial thinking. Built from `alpha/` (see
  `beta/README.md`); beta docs reference beta docs, with a "Sources (alpha)" pointer down. The
  resolved spec lives here (`beta/drystone-spec/`, Part 1 + Part 2).
- **`rc/`, `publish/`** — later maturity stages (not yet created).

**Cross-stage docs at the repo root** (they describe *how we work*, not the project, so they span
stages):

- **`AGENTS.md`** — agent orientation; the canonical doc the harness auto-loads. Start here.
- **`PLAYBOOK.md`** — the intake / filing process.

The **experiments** and **Proofs** corpora were folded into discovery (2026-07-13 and 2026-07-15) and
now live under `alpha/experiments/` and `alpha/Proofs/`, so discovery is the single authoritative
home; the standalone `experiments/` and `Proofs/` repos are frozen and archived (read-only). Git
identity: chasemp (`chase@owasp.org`, `github-personal`). Don't commit/push unless asked.
