# Handoff prompt — match-3 round-2 follow-ups: more objectives (jelly / ingredients), specials, and adopting the stronger par

> Paste this into a fresh Claude Code session started in `/Users/cpettet/git/chasemp/CroftC`.
> It is self-contained. Work in the standalone repo **`CroftCommunity/fun`**, checked out at
> `CroftC/fun` (git identity: **chasemp** / `chase@owasp.org` / remote `git@github-personal:CroftCommunity/fun`;
> run `gh auth switch --user chasemp` if using `gh`). Follow phase-plan execution discipline: TDD-first,
> wiring the test through the real entry point, **commit + push per stable point** (pushing `main`
> auto-deploys via `.github/workflows/deploy.yml` → GitHub Pages → `fun.croft.ing`), keep docs in sync.

## Your task

Continue the match-3 backlog from `fun/TODO/match3.md` → "Deferred", now that round 1 shipped
(cascade animation, deadlock reshuffle, the clear-the-blockers objective, and a held stronger reference).
Three items remain, all **owner-gated or data-gated** — surface the decision first, do not invent:

1. **More variant objectives — jelly / ingredients** (owner decision, then build one slice).
2. **Specials — striped / wrapped / colour-bomb** (owner decision; a large, separate core track).
3. **Adopt the stronger reference / retune the par** (data-gated; a rules-version bump when adopted).

Read first: `fun/plans/2026-07-30-match3-followups.md` (round-1 plan + decisions D1/D2),
`fun/TODO/match3.md`, `fun/docs/BUILDING-GAMES.md`, `fun/crates/match3-core/RULES.md`.

## Where things stand (all green + live on `CroftCommunity/fun`)

`/match3/` ships **two objectives** over one 8×8 engine, plus solitaire. Everything below is real,
tested, and deployed as of 2026-07-30:

- **Core (`crates/match3-core`)** — the deterministic engine. New since round 1:
  - `Game::play_move_traced(from, to) -> (MoveReport, Vec<Board>)` — the same resolution as `play_move`
    (byte-identical final `state_hash`) but emitting a board snapshot per cascade phase (the animation source).
  - `reshuffle_if_dead(&mut Board, &mut DetRng) -> bool` — after a settle with no legal swap, permutes gems
    deterministically (blockers fixed) into a live, match-free board; called at the end of `play_move`, so
    replay reproduces it. A live board is untouched (0 draws).
  - `deal_blockers(seed, w, h, colors, blockers) -> Board` + `blockers_remaining(&Board) -> u32` — the
    clear-the-blockers deal + win metric (win = 0 blockers).
  - `reference_score_beam(seed, w, h, colors, budget, beam_width) -> u64` — a less-myopic beam par that
    provably ≥ `reference_score`. **Built but NOT wired into targets** (see item 3).
  - `pub mod blockers_mode` — shared config (WIDTH/HEIGHT/COLORS/BLOCKERS/MOVE_BUDGET = 8/8/6/6/30).
  - `Cell::{Empty, Gem(u8), Blocker(u8)}`; `clear_cells` damages an adjacent blocker one layer per match.
    `state_hash` folds board + colors + `rng.draws()` + score (spec in `RULES.md`). Golden vectors in
    `crates/match3-core/vectors/*.json` (locked `final_state_hash`); tests in `tests/{golden_vectors,
    tie_breaks,deal,reference,traced,reshuffle,blockers}.rs`.
- **Solver (`crates/match3-solver`)** — `find_clear(seed, node_budget)` (budgeted blocker-damage-first DFS)
  + `generate_pack` / `pack_to_doc` (kind `match3-blockers-pack` v1). The committed winnable-daily pack is
  `games/match3/blockers-pack.json` (365 seeds + a fixture line). Mirrors `solitaire-solver` exactly:
  fast tests replay the committed pack; `#[ignore]` tests regenerate it byte-identically. **This is the
  reusable winnable-daily shape** any new win-objective mode should copy.
- **Binding (`crates/match3-wasm`)** — mode-aware. A `Session` holds a `Mode` (`TargetScore` | `Blockers`),
  its budget, and blocker count; `Session::won()` centralizes the win check. Exports: `new_game`
  (target-score), `new_blockers_game`, `board_json` (a `BoardView` with `mode`, a `blockers` mask,
  `blockersRemaining`/`blockersTotal`, plus the score/targets/stars), `legal_moves_json`, `play_swap`,
  `play_swap_traced` (per-phase snapshot JSON for the animation), `moves_left`, `is_won`, `outcome_json`.
  Two `pond-outcome` impls: `Match3` (kind `match3`, VERSION 1, score/stars) and `Match3Blockers`
  (kind `match3-blockers`, VERSION 1, win = cleared, metric = swap count).
- **UI (`src/games/match3.ts` + `match3-wasm.ts` + `match3-outcome.ts` + `match3-howto.ts`)** — an
  objective toggle (Target score / Clear blockers), `?mode=blockers` deep-link, blocker tiles, a
  blockers-left HUD, per-phase cascade animation (reduced-motion-safe), and a mode-aware verifiable result
  screen. `verifyRecord` dispatches on the envelope `kind`. The blockers daily/free seeds come from the
  pack (served at `/match3-blockers-pack.json` via `build.mjs`).

**The gate** (must stay green each commit): in `fun/` — `npm run test` (typecheck · lint · unit[builds
wasm] · build) + `npm run e2e` (Playwright incl. axe); Rust — `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets`. Visual change → rerun
`npm run build:wasm && npm run build && npm run guide:shots` and commit the shots.

---

## Item 1 — More variant objectives: jelly and/or ingredients (OWNER DECISION FIRST)

**Why deferred:** new modes + balance calls reserved for the owner (same as clear-the-blockers was).
Clear-the-blockers is now the **template**: mode-aware binding + a solver/pack for winnability + a UI
mode entry. Surface the choice, get a pick, then build one as its own slice.

Options to put to the owner (with the real cost of each), building on what exists:
- **Jelly** — a jelly overlay under gems; clear all jelly to win. **New core state**: a per-cell jelly
  layer (a new `Cell` facet or a parallel jelly grid), which means **new `state_hash` bytes** (extend the
  `RULES.md` hash spec + re-lock golden vectors) and a jelly-aware `clear_cells` (a match over a jellied
  cell removes a jelly layer). Then a jelly deal + a `match3-solver`-style winnable pack (reuse the
  `find_clear` shape, win = no jelly). Medium-large core change.
- **Ingredients** — ingredient pieces that must fall to the bottom row. **New falling-piece mechanic**:
  ingredient cells that gravity moves but matches don't clear, exiting at the bottom. Biggest core change
  (new movement rule + new win check + solver). A separate, larger slice.

**Do:** write the decision up, ask via the question tool (present concrete options grounded in the engine,
recommend one, ask), and only after a pick build that mode as a full slice per `BUILDING-GAMES.md`
(core state + rules + re-locked vectors → solver + committed pack → binding mode (`Mode` variant +
`Match3<Mode>` outcome kind) → UI toggle entry → how-to → tests → deploy). Keep the target-score and
clear-the-blockers modes untouched.

## Item 2 — Specials: striped / wrapped / colour-bomb (OWNER DECISION; large, separate track)

**Why deferred:** a large core extension — match-shape detection (4-in-a-row → striped, L/T → wrapped,
5-in-a-row → colour-bomb), special-gem cells, and activation cascades (a striped clears a line; a
colour-bomb clears a colour; combining two specials) — plus heavy balance. This is its **own track**, not
a quick slice.

**Do:** surface it as a scoped decision (which specials, in which order, and against which objective they
apply). If greenlit, plan it as a multi-phase effort of its own (shape detection + special-gem model +
activation rules each with golden vectors, then binding/UI), and expect to re-lock hashes and extend the
`state_hash` spec. Do not fold it into another item.

## Item 3 — Adopt the stronger reference / retune the par (DATA-GATED — a rules-version bump)

**Where it stands (decision D2, 2026-07-30):** `reference_score_beam` is **built and validated**
(deterministic, monotone, ≥ greedy, strictly better on some seeds) but `targets_for` in `match3-wasm`
still uses the greedy `reference_score`, so **no shared result is re-graded**. The fraction retune is
"best driven by real play data" that did not exist at round 1.

**Approach + the wrinkle to respect:**
- If/when real play data (shared records) exists, decide the par: switch `targets_for` to
  `reference_score_beam` (and/or retune the 30/60/90% fractions against observed scores).
- **Versioning:** `verify` re-derives targets from the seed, so any par change re-grades every past record.
  Treat it as a **`Match3::VERSION` bump to 2**: keep the greedy par for version-1 records (read the version
  from the `pond-docformat` envelope in `Match3::replay`), use the new par for version-2 records. Do not
  change par silently. Procedure is recorded in `RULES.md` ("Per-deal par … & versioning") and the round-1
  plan's D2.
- **Runtime cost:** the beam is heavier than greedy and `targets_for` runs in `new_game`/`verify` (the
  browser hot path). Before adopting, confirm the beam is fast enough in wasm, or precompute/cheapen it.
- Tests: an old-version record still verifies under the old par; the new par's targets are strictly
  increasing and deterministic.

**Done when:** either the par is adopted with the version bump handled and a recorded rationale, or it is
explicitly re-deferred again (with the reason — still no real data) and the versioning note kept current.

---

## Guardrails (all items)

- **TDD-first**, wiring the test through the real entry point (crate API / wasm boundary / `/match3/` URL),
  RED before GREEN. The board UI **never** decides legality or win — always delegate to the core.
- **Determinism is the anchor:** any new state folded into a game must go into `state_hash`; any change that
  alters a locked golden-vector hash must be re-locked **and explained** in the commit.
- **Never panic in the binding** — status codes / empty JSON, no `unwrap` on the hot path.
- Keep `pond-outcome` changes **additive** and each mode's outcome a distinct `kind` (solitaire, `match3`,
  and `match3-blockers` must all stay green).
- Reuse the **winnable-daily solver + pack** shape (`match3-solver` / `solitaire-solver`) for any new
  win-objective mode — do not invent a second pattern.
- **Commit + push per stable point** (auto-deploys); keep `TODO/match3.md`, the plan headers, `RULES.md`,
  and `docs/BUILDING-GAMES.md` in sync; regenerate guide shots on any visual change.
- Sandbox note: Playwright e2e runs locally against the built site (works here); a *live* `fun.croft.ing`
  check may hit the browser-egress limit — verify with the local build + `curl`.

## Definition of done (the whole batch)

Each greenlit objective (item 1) is a playable, verifiable mode shipped as its own slice; specials
(item 2) are either planned as a scoped track or explicitly deferred with the decision captured; the par
(item 3) is adopted with the `Match3::VERSION` bump handled or re-deferred with the versioning note current.
Every commit leaves Rust + `npm run test` + `npm run e2e` green and is deployed; docs and `TODO/match3.md`
reflect reality.
