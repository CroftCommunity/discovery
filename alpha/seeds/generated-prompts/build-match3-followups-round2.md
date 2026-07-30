# Handoff prompt — match-3 Candy-Crush parity program (execute the roadmap, start with jelly)

> Paste this into a fresh Claude Code session started in `/Users/cpettet/git/chasemp/CroftC`.
> Work in the standalone repo **`CroftCommunity/fun`**, checked out at `CroftC/fun` (git identity:
> **chasemp** / `chase@owasp.org` / remote `git@github-personal:CroftCommunity/fun`; `gh auth switch --user
> chasemp` if using `gh`). Phase-plan discipline: TDD-first through the real entry point, **commit + push
> per stable point** (pushing `main` auto-deploys via `.github/workflows/deploy.yml` → GitHub Pages →
> `fun.croft.ing`), keep docs in sync.

## Your task

Execute the **match-3 → Candy-Crush parity program**. The plan is already written and owner-approved:

- **Source of truth:** `fun/plans/2026-07-30-match3-parity-roadmap.md` (problem, locked decisions
  D3–D6, phased tracks A–D, reasoning, risks, DoD).
- Backlog + decision summary: `fun/TODO/match3.md`.
- Round-1 context (already shipped): `fun/plans/2026-07-30-match3-followups.md`.
- Shelf standards + core rules: `fun/docs/BUILDING-GAMES.md`, `fun/crates/match3-core/RULES.md`.

Decisions are **locked** — do not re-litigate them:
- **D3** third objective = **jelly**. **D4** specials = **full parity** (striped, wrapped, colour bomb,
  2×2 fish, **and the combo matrix**). **D5** par = a ladder of **deterministic** players baked into a
  committed table (LLM subagents only calibrate the rungs, never the verify path); retune now (no users).
  **D6** in-browser AI hint/coach = backlog.
- **Order:** jelly → specials (phased) → par ladder, with a cheap beam-based par table early.

**Start with Track A (jelly).** Build it as a full slice per the roadmap: A1 core (a per-cell jelly layer;
extend `state_hash` + `RULES.md`; jelly-aware clear; `deal_jelly` + `jelly_remaining`; re-lock vectors) →
A2 a `match3-solver`-style winnable-daily jelly solver + committed pack → A3 a mode-aware binding
(`Mode::Jelly`, `new_jelly_game`, a `Match3Jelly` outcome `kind` = `match3-jelly`, board_view jelly mask)
→ A4 UI (objective-toggle entry, jelly rendering, jelly-left HUD, verifiable clear result) + how-to + e2e
(incl. axe) + guide shots. It mirrors the shipped clear-the-blockers slice (3a–3d) closely. Consider the
roadmap's optional **Phase 0** (generalize the mode plumbing so objectives aren't copy-paste) first if the
duplication is already painful. After jelly, proceed down the roadmap; re-read it between tracks and stop
at Track D items for owner confirmation.

## Where things stand (all green + live on `CroftCommunity/fun`, as of 2026-07-30)

`/match3/` ships **two objectives** over one 8×8 engine, plus solitaire. Key APIs to build against:

- **Core (`crates/match3-core`)**: `Game::play_move` / `play_move_traced` (per-phase snapshots for the
  animation, same RNG → identical hash); `reshuffle_if_dead` (deterministic deadlock escape, folded into
  the hash); `deal` / `deal_blockers` + `blockers_remaining`; `reference_score` (greedy par, shipped) and
  `reference_score_beam` (stronger beam par, built, held); `pub mod blockers_mode`; `Cell::{Empty, Gem(u8),
  Blocker(u8)}` (`clear_cells` damages an adjacent blocker one layer per match); `state_hash` folds
  board + colors + `rng.draws()` + score (spec in `RULES.md`); golden vectors + tests under `tests/`.
- **Solver (`crates/match3-solver`)**: `find_clear` + `generate_pack`/`pack_to_doc` (kind
  `match3-blockers-pack` v1), committed pack `games/match3/blockers-pack.json` (365 seeds + fixture). **The
  reusable winnable-daily shape** — copy it for jelly and every win-objective mode.
- **Binding (`crates/match3-wasm`)**: mode-aware `Session` (`Mode` = `TargetScore` | `Blockers`, budget,
  blocker count, `Session::won()`); exports `new_game`, `new_blockers_game`, `board_json` (a `BoardView`
  with `mode`, a `blockers` mask, `blockersRemaining/Total`, score/targets/stars), `legal_moves_json`,
  `play_swap`, `play_swap_traced`, `moves_left`, `is_won`, `outcome_json`. Two `pond-outcome` impls:
  `Match3` (`match3`, score/stars) and `Match3Blockers` (`match3-blockers`, win = cleared, metric = swaps).
- **UI (`src/games/match3*.ts`)**: an objective toggle (Target score / Clear blockers), `?mode=blockers`
  deep-link, blocker tiles, blockers-left HUD, reduced-motion-safe per-phase cascade animation, mode-aware
  verifiable result; `verifyRecord` dispatches on the envelope `kind`; blockers seeds come from the pack
  (served at `/match3-blockers-pack.json` via `build.mjs`).

**The gate** (green each commit): in `fun/` — `npm run test` (typecheck · lint · unit[builds wasm] · build)
+ `npm run e2e` (Playwright incl. axe); Rust — `cargo test --workspace`, `cargo fmt --all --check`,
`cargo clippy --workspace --all-targets`. Visual change → `npm run build:wasm && npm run build &&
npm run guide:shots` and commit the shots.

## Guardrails (all phases — see the roadmap for the full list)

- **TDD-first** through the real entry point (crate API / wasm boundary / `/match3/` URL); the board UI
  **never** decides legality or win — always delegate to the core.
- **Determinism is the anchor:** every new state (jelly layers, special gems, fish, obstacles) folds into
  `state_hash` + `RULES.md`; any change to a locked golden-vector hash is re-locked **and explained**.
- **Verifiable outcomes:** any randomized effect (e.g. fish targeting) draws from the seeded `DetRng` so
  replay reproduces it. Par is computed **offline** by deterministic players and baked into a committed
  table — never a live model on the verify path.
- **Winnable dailies:** each objective needs a solver + committed pack; a specials-aware mode needs a
  specials-aware solver. Log any coverage cap — no silent truncation.
- **Never panic in the binding**; keep `pond-outcome` additive with distinct `kind`s; existing modes +
  solitaire stay green; commit + push per stable point; keep docs (`TODO/match3.md`, plan headers,
  `RULES.md`, `docs/BUILDING-GAMES.md`) in sync; regenerate guide shots on visual change.

## Definition of done (per slice, and the program)

Each slice ships green + deployed with its own tests/vectors and updated docs. The program is done when
jelly is a verifiable winnable-daily mode; the full specials system (striped, wrapped, colour bomb, fish,
and the combo matrix) is playable, deterministic, and verifiable with modes staying winnable-daily; par is
the deterministic player ladder baked into a committed table (calibrated offline, re-run after specials);
Track D items are each owner-confirmed before build; and the hint/coach remains on the backlog.
