# Phase 1 — Extract the kit to `CroftCommunity/croft-stack`

← [00-model-and-manifests.md](00-model-and-manifests.md) · [roadmap](README.md) · next →
[02-adopt-box-declaratively.md](02-adopt-box-declaratively.md)

**Status:** **DONE (2026-07-24, pushed `fcf49a7`).** · **Scope:** single phase, rename inline (no 1/1b
split) · **Depends-on:** Phase 0 gate (met) · **Gate-out:** MET with two documented toolchain
carve-outs (below).

## Outcome (2026-07-24)

`CroftC/croft-stack` cloned (remote `github-personal`), extracted verbatim (commit `6164a9a`, from
discovery kit `1004f58`), then renamed `appview-infra → croft-stack` (commit `fcf49a7`) — **44/44
symmetric string swap across 24 files, touching no logic**; the 3 discovery source-path references
(`alpha/experiments/appview-infra/`) were intentionally preserved. `LICENSE` (pre-existing) kept; the
stub README replaced by the kit's.

**`make check`: 12/13 sub-checks green** — no-secrets, stub (6), render (11), backup-audit (3), bootstrap
(6), docs (3), own-data (6, one mount-namespace skip), group-serving (7), runbook (8). The 2 red are
**toolchain-gated, not the rename:** `check-terraform` (terraform CLI absent; we use `tofu`, content
verified clean under `tofu fmt` → fold→2) and `check-local-drill` (litestream absent; the fire-drill is
deferred with backups paused → fold→3). Local dev needed bash 5 + bats + shellcheck + opentofu installed
(fold→1 dev-setup note).

**Decision (from review):** extraction is a **one-time bootstrap**. After it, `croft-stack` is the live
production repo and **diverges** from discovery; the discovery kit is frozen as the **as-built origin
snapshot** (it keeps the old `appview-infra` name as history — not swept). The rename therefore happens
**in `croft-stack`, after extraction** (the script wipes its target on every run, so a rename in the
target only survives because we do not re-extract).

---

## Problem

Production operations must not run from the discovery repo. The kit needs its own repo with its own
history, from which bootstrap/generation/deploys run. Two wrinkles the review found: the extraction
script emits a **tree, not a repo** (no git/commit/push, no history), and the kit still calls itself
**`appview-infra`** in 44 places — including the functional on-box path `/opt/appview-infra/` and the
extraction test assertions — so the extracted `croft-stack` repo would call itself the wrong name until
swept.

## Approach

Extract once to a local tree, verify it with the kit's own extraction test, seed the empty
`croft-stack` repo with it, then do the `appview-infra → croft-stack` rename sweep **in `croft-stack`**,
TDD-style, and prove `make check` green standalone.

## Preconditions

- **`bats` installed** — `make check` is entirely BATS tests (`bats not found` locally today). Install
  it first (`brew install bats-core`), or `make check` cannot run.
- Empty `CroftCommunity/croft-stack` exists (done).
- `gh auth switch --user chasemp`; remote host `github-personal`.

## Steps

1. **Extract to a local tree.** `scripts/extract-to-repo.sh /tmp/croft-stack-extract`. It tar-copies the
   kit to the target root (excludes `.git`/`__pycache__`/`*.pyc`/`drill/.work`/`*.tfstate`/`.terraform`),
   writes `PROVENANCE.md` (points back to discovery), and clears+rewrites the target.
2. **Verify the extraction** with the kit's own test: `bats tests/extraction.bats` — it extracts *and*
   runs `make check` on the extracted tree (so this is the real gate, not a manual `diff`; a raw diff is
   dirty because PROVENANCE.md is added and noise excluded).
3. **Seed the repo.** Clone the empty `croft-stack` to `CroftC/croft-stack` (nested, like the other
   pads); copy the extracted tree in; commit under `"Chase Pettet" <chase@owasp.org>`; push over
   `git@github-personal:CroftCommunity/croft-stack`. **Do not re-run the extractor against this repo
   afterward** — it would wipe the rename (extraction is one-time, per the decision above).
4. **Rename sweep `appview-infra → croft-stack`** in `croft-stack`, TDD-first: update
   `tests/extraction.bats` (+ any other bats asserting the name) to expect `croft-stack`, watch them
   fail, then sweep code+docs+PROVENANCE and the **functional `/opt/appview-infra/` → `/opt/croft-stack/`**
   deploy path (`deploy-receive.sh`, RUNBOOK), until green. The box has no `/opt/appview-infra/` yet
   (only the auth-helper spike's `/opt/auth-helper/`), so the path rename is forward-looking — no
   migration.
5. **Prove it.** `make check` green in a fresh clone; `grep -rn appview-infra` returns nothing load-bearing.

## Reasoning

- **One-time extract + rename-in-target** (owner's call) treats croft-stack as the live repo that
  diverges; discovery stays the frozen as-built record. Cleaner than sweeping the discovery corpus for a
  name only the production repo needs.
- **`make check` standalone is the real gate** — it proves croft-stack has no hidden dependency on the
  discovery tree, the whole point of the split. The kit self-tests this via `extraction.bats`.
- **TDD the rename** — the bats tests currently *assert* `appview-infra`; flipping them first makes the
  sweep verifiable rather than hopeful.

## Risks & cautions

- **`bats` missing** — hard blocker for the gate; install before starting.
- **The extractor wipes its target every run** — seed the repo and rename *after* extraction, and do not
  re-extract over the repo (extraction is one-time by decision).
- **Functional path rename** `/opt/appview-infra/ → /opt/croft-stack/` — forward-looking (box has no
  such path yet); ensure `deploy-receive.sh`, RUNBOOK, and any unit templates agree post-sweep.
- **Git identity** — `chasemp` / `github-personal` / `chase@owasp.org`; wrong identity (work) is the
  classic path mistake. Switch `gh` and confirm the remote host before pushing.
- **PROVENANCE.md** — the script hardcodes `appview-infra` in it; include it in the sweep (keep the
  pointer back to discovery, fix the name).

## Validation

`make check` green in a fresh clone of `CroftCommunity/croft-stack`; `bats tests/extraction.bats` green
(rewritten to assert `croft-stack`); `grep -rn appview-infra` finds nothing load-bearing; `git log` +
remote confirm `chasemp` / `github-personal`.

## References

- `alpha/experiments/appview-infra/kit/scripts/extract-to-repo.sh`, `tests/extraction.bats`,
  `docs/EXTRACTION.md` (subtree-split alternative), `docs/RUNBOOK.md`, `Makefile` (`check` = all BATS).
- Global git-identity-by-path rule (chasemp / `github-personal` / `chase@owasp.org`).
- Roadmap → Open decision 7 (`croft-stack`).
