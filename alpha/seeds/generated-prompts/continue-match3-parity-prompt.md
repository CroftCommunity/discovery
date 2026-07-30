# Handoff prompt — continue the match-3 → Candy-Crush parity program (specials next)

> Paste this into a fresh Claude Code session started in `/Users/cpettet/git/chasemp/CroftC`.
> Work in the standalone repo **`CroftCommunity/fun`**, checked out at `CroftC/fun` (git identity:
> **chasemp** / `chase@owasp.org` / remote `git@github-personal:CroftCommunity/fun`; `gh auth switch --user
> chasemp` if using `gh`). Phase-plan discipline: TDD-first through the real entry point, **commit + push
> per stable point** (pushing `main` auto-deploys via `.github/workflows/deploy.yml` → GitHub Pages →
> `fun.croft.ing`), keep docs in sync.

## Your task

Continue the owner-approved **match-3 → Candy-Crush-parity program**. The plan is written; execute it.

- **Source of truth (read first):** `fun/plans/2026-07-30-match3-parity-roadmap.md` — problem, locked
  decisions D3–D6, phased tracks A–D, reasoning, risks, DoD, and the par design.
- Backlog + decisions summary: `fun/TODO/match3.md`. Core rules: `fun/crates/match3-core/RULES.md`.
  Shelf standards: `fun/docs/BUILDING-GAMES.md`. Round-1 context: `fun/plans/2026-07-30-match3-followups.md`.

**Decisions are locked — do not re-litigate:** D3 jelly (done); D4 specials = **full parity** (striped,
wrapped, colour bomb, 2×2 fish, **and the combo matrix**); D5 par = a **deterministic player-ladder** baked
into an embedded table, 3★ = **strong-but-attainable** (not near-optimal); D6 in-browser AI hint/coach =
backlog. LLM subagents may be used **offline only** to calibrate par rungs, never on the verify path.

**Next up: Track B — specials (the long pole), phased one-at-a-time end-to-end.** Start with **B0**
(the special-gem model + shape detection), then B1 striped → B2 wrapped → B3 colour bomb → B4 fish
(seeded targeting) → B5 the combo matrix → B6 specials-aware solver + re-par. Each phase is its own green,
deployed slice with its own golden vectors and balance. See the roadmap's Track B for per-phase detail.

## Where things stand (all shipped, green, and live on `CroftCommunity/fun` as of 2026-07-30)

`/match3/` has **three objectives** over one 8×8 engine (plus solitaire). Latest commit `75ee5b8`.

**Done this program so far:**
- **Cascade animation, deadlock reshuffle** (round 1).
- **Track A — jelly**: a clear-the-jelly objective, playable/verifiable/winnable-daily.
- **Mobile hardening**: a `mobile-webkit` Playwright project (iOS Safari engine + touch) — the full suite
  passes on it; HTML5 drag tests are scoped to desktop (`test.skip` on `hasTouch`); phone tap targets
  bumped. **Run `npx playwright install webkit` once** or the mobile project can't launch.
- **Par (#6)**: target-score star tiers are now a **player ladder** — 1★ = a random-legal player (floor),
  2★ = greedy, 3★ = beam-8 (strong-but-attainable) — baked offline into a committed table and embedded in
  the binding; 3★ is no longer trivial. Free-play seeds fall back to live greedy tiers.

**Key APIs / files to build against:**
- **`crates/match3-core`** (the deterministic engine):
  `Game::play_move` / `play_move_traced` (per-phase snapshots, same RNG → identical hash);
  `reshuffle_if_dead`; deals `deal` / `deal_blockers` / `deal_jelly`; win metrics `blockers_remaining` /
  `jelly_remaining`; reference players `reference_score` (greedy) / `reference_score_beam` / `random_score`;
  mode configs `blockers_mode` / `jelly_mode` / `target_score_mode`.
  `Cell::{Empty, Gem(u8), Blocker(u8)}`; `Board` also carries a per-cell **jelly** overlay grid
  (`jelly()`, `jelly_at`, `set_jelly`, `from_rows_with_jelly`). `clear_cells → ClearOutcome
  {gems_cleared, blocker_layers_removed, jelly_layers_removed}`; `StepReport` carries the same counts.
  **`state_hash`** = SHA-256 over board + colors + draws + score, with each cell a tag byte
  (`0x00` empty / `0x01,c` gem / `0x02,l` blocker) **plus a jelly section appended only when present**
  (spec in `RULES.md`). Golden vectors: `crates/match3-core/vectors/*.json` + `tests/`.
- **`crates/match3-solver`** (build-time, offline): a shared budgeted DFS `search` + `find_clear` /
  `find_dejelly`; `generate_pack` / `generate_jelly_pack`; the par ladder `par_tiers` + `generate_par_pack`.
  Committed data: `games/match3/{blockers-pack,jelly-pack,par-pack}.json`. Fast tests replay the committed
  packs; `#[ignore]` generators/regen-drills re-run the solver. **This is the reusable winnable-daily +
  par shape — copy it, don't invent a second.**
- **`crates/match3-wasm`** (binding): `Mode::{TargetScore, Blockers, Jelly}` + `Session::won()`; exports
  `new_game` / `new_blockers_game` / `new_jelly_game`, `target_daily_seed`, `board_json` (a `BoardView`
  with `mode`, `cells`, a `blockers` mask, a `jelly` grid, `blockersRemaining/Total`, `jellyRemaining/Total`,
  `targets`/`stars`, `movesLeft`/`moveBudget`, `won`), `legal_moves_json`, `play_swap`,
  `play_swap_traced`, `moves_left`, `is_won`, `current_hash`, `outcome_json`, `mark_assistance`. The par
  table is `include_bytes!`-embedded and looked up in `targets_for`. Three `pond-outcome` impls:
  `Match3` / `Match3Blockers` / `Match3Jelly` (kinds `match3` / `match3-blockers` / `match3-jelly`).
- **UI (`src/games/match3*.ts`)**: an objective toggle (Target score / Clear blockers / Clear jelly),
  `?mode=blockers` / `?mode=jelly` deep-links, blocker tiles + jelly backing, reduced-motion-safe per-phase
  cascade animation, a mode-aware verifiable result; `verifyRecord` dispatches on the envelope `kind`;
  daily seeds come from the packs / `target_daily_seed`.

**The gate (green each commit):** in `fun/` — `npm run test` (typecheck · lint · unit[builds wasm] ·
build) + `npm run e2e` (Playwright, **chromium + mobile-webkit**, incl. axe — needs webkit installed);
Rust — `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets`.
Visual change → `npm run build:wasm && npm run build && npm run guide:shots` and commit the shots.

## Specials (Track B) — the hard parts to get right

- **Determinism / the fingerprint.** Add special gems as a new `Cell` variant (a new tag byte, e.g.
  `0x03,…`) or a parallel layer, folded into `state_hash` **only when present** so gem-only boards hash
  identically and pre-specials golden vectors need no re-lock (the jelly pattern). Re-lock (and explain)
  any vector that legitimately changes.
- **Shape detection is new.** `find_matches` today finds only line runs; specials need to classify a match
  as line-3 / line-4 / L-or-T / line-5 / 2×2, with **deterministic creation-placement** tie-break rules in
  `RULES.md`.
- **Activation can chain.** A special clears (matched or swapped) with a line/area/colour effect that can
  trigger further matches/gravity/refill → recursive, ordered, deterministic resolution.
- **Randomized effects use the seeded RNG.** Fish targeting (B4) must draw from the game `DetRng` (folded
  into `draws`), so replay/verify reproduces it. No unseeded randomness anywhere.
- **Winnability + par shift.** Specials make boards easier — a specials-aware solver keeps modes
  winnable-daily (B6), and the par ladder must be re-run so 3★ stays honest (C3, feeds the par table).
- **Combos (B5)** are the big balance phase — land the individual specials first.

## After specials (remaining backlog)

- **C2** — offline LLM-subagent calibration study of the par rungs (weak/medium/strong ↔ human skill),
  study-only, never shipped. **C3** — regenerate the par table once specials land.
- **Track D** (owner-confirm each before building): ingredients / order-mixed / timed objectives; more
  obstacle families (licorice, spreading chocolate, meringue, locks, timed bombs).
- **Follow-ups:** solitaire card tap-target sizing (mobile); the in-browser AI hint/coach (D6, advisory).

## Guardrails (all phases)

TDD-first through the real entry point; the board UI **never** decides legality/win (delegate to the core);
determinism is the anchor (new state → `state_hash` + `RULES.md` + re-locked/explained vectors); randomized
effects draw from the seeded RNG; never panic in the binding; `pond-outcome` additive with distinct
`kind`s; reuse the winnable-daily solver+pack + par shape; existing modes + solitaire + **both e2e
projects** stay green; commit + push per stable point (auto-deploys); keep `TODO/match3.md`, the plan
headers, `RULES.md`, `docs/BUILDING-GAMES.md` in sync; regenerate guide shots on any visual change.

## Optional first: playtest what's live

Jelly and the new ladder par are live at `fun.croft.ing` (and locally via `cd fun && npm run serve`). If
the owner wants to feel the difficulty before more building, playtest first, then start Track B / B0.

## Definition of done (the program)

The full specials system (striped, wrapped, colour bomb, fish, and the combo matrix) is playable,
deterministic, and verifiable, with modes staying winnable-daily and the par re-run to stay honest; Track D
items are each owner-confirmed before build; the hint/coach stays backlogged. Every commit leaves Rust +
`npm run test` + `npm run e2e` (both projects) green and is deployed; docs reflect reality.
