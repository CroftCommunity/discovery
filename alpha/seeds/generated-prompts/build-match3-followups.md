# Handoff prompt — match-3 follow-ups: cascade animation, deadlock reshuffle, a variant mode, par tuning

> Paste this into a fresh Claude Code session started in `/Users/cpettet/git/chasemp/CroftC`.
> It is self-contained. Work in the standalone repo **`CroftCommunity/fun`**, checked out at
> `CroftC/fun` (git identity: **chasemp** / `chase@owasp.org` / remote `git@github-personal:CroftCommunity/fun`;
> run `gh auth switch --user chasemp` if using `gh`). Follow the phase-plan execution discipline:
> TDD-first, wiring test through the real entry point, **commit + push per stable point** (pushing `main`
> auto-deploys via `.github/workflows/deploy.yml` → GitHub Pages → `fun.croft.ing`), keep docs in sync.

## Your task

Plan and execute the four deferred match-3 follow-ups (from `fun/TODO/match3.md` → "Deferred"). Two are
**contained engineering** you can do directly; one is an **owner decision** you must surface before
building; one is **fuzzy tuning** with a versioning wrinkle. Do them in the order below (safest → owner-
gated). Each ships green and deployed on its own commit.

## Where things stand (all green + live on `CroftCommunity/fun`)

`/solitaire/` and `/match3/` are both playable, verifiable, accessible (axe both themes), and live.
match-3 is Candy-Crush-style target-score-in-moves with per-deal star targets. Read first:

- **`fun/docs/BUILDING-GAMES.md`** — the shelf standards every game meets (module contract, determinism
  core→wasm, verifiable outcome, tap-first input, tokens/identity, hints/settings, how-to guide, the gate).
- **`fun/plans/2026-07-30-match3-playable.md`** — the match-3 delivery plan (objective, phases, decisions).
- **`fun/TODO/match3.md`** — the running backlog (these four items are the "Deferred" section).

**The gate** (all must stay green): `npm run test` (typecheck · lint · unit(builds wasm first) · build)
+ `npm run e2e` (Playwright incl. axe) in `fun/`; and Rust: `cargo test --workspace`,
`cargo fmt --all --check`, `cargo clippy --workspace --all-targets`. Visual changes → rerun
`npm run guide:shots` (needs `npm run build:wasm && npm run build` first) and commit the shots.

**The match-3 code you build against:**
- `fun/crates/match3-core/` — the engine. Key public API (`src/engine.rs`, re-exported in `lib.rs`):
  `Game { board, colors, score, rng(priv) }`, `Game::new(board, seed, colors)`,
  `Game::play_move(from: Pos, to: Pos) -> MoveReport`, `Game::state_hash()`; and the **pure ops**
  `find_matches(&Board) -> Vec<Pos>`, `clear_cells(&mut Board, &[Pos]) -> ClearOutcome`,
  `apply_gravity(&mut Board)`, `refill(&mut Board, &mut DetRng, colors)`, `swap_legal`, `legal_swaps`,
  `has_legal_move`, `deal(seed,w,h,colors) -> Board`, `reference_score(seed,w,h,colors,budget) -> u64`.
  `MoveReport { legal, steps: Vec<StepReport{ cleared: Vec<Pos>, blocker_layers_removed, score_gained }>,
  score_gained }`. `Cell::{ Empty, Gem(u8), Blocker(u8) }` (blockers already modelled: `clear_cells`
  damages a blocker one layer per adjacent clear). `Pos = (row, col)`. `state_hash` folds board + colors +
  `rng.draws()` + score. Golden vectors: `crates/match3-core/tests/{golden_vectors,tie_breaks}.rs` +
  `crates/match3-core/vectors/*.json` (each has a locked `final_state_hash`), plus `tests/deal.rs`,
  `tests/reference.rs`. RULES doc: `crates/match3-core/RULES.md`.
- `fun/crates/match3-wasm/src/lib.rs` — the raw C-ABI binding (never panics). Holds `Session { seed,
  game, targets: [u64;3], swaps: Vec<[u8;4]>, assistance_used }`. Exports: `new_game`, `board_json`
  (a `BoardView` with `cells`, `score`, `movesLeft`, `moveBudget`, `targets`, `stars`, `won`),
  `legal_moves_json`, `play_swap`, `score`, `moves_left`, `current_hash`, `is_won`, `mark_assistance`,
  `outcome_json`. `targets_for(seed) = reference_score(seed,8,8,6,20).max(10) * [3/10, 6/10, 9/10]`.
  A `Match3: pond_outcome::Game` impl replays `(seed, swaps)` by dealing + applying swaps + grading.
  `WIDTH=HEIGHT=8, COLORS=6, MOVE_BUDGET=20`.
- `fun/src/games/match3.ts` — the `GameModule` (board UI, tap + drag to swap, HUD, win cascade, result
  screen). `match3-wasm.ts` (typed wrapper), `match3-outcome.ts` (verify/share, `Verifier` re-derives
  score+stars), `match3-howto.ts` (guide), `share.ts` (deflate+base64url + `dayIndexUTC`, shared).
- `fun/crates/pond-outcome/` — `Record { …, result: Outcome, assistance, score?, stars? }`,
  `Outcome::{Won,Stuck,Abandoned,Lost}`, `Replayed::new/scored`, `attest`, `verify` (re-replays, never
  trusts stored fields), `to_doc`/`from_doc` via `pond-docformat`.

---

## Item 1 — Full step-by-step cascade animation (contained; the biggest "feel" win)

**Why it was deferred:** `Game::play_move` resolves the whole cascade **atomically** (swap → loop:
find_matches → clear_cells → apply_gravity → refill until stable). `board_json` returns only the settled
board, and `MoveReport.steps` carries each step's *cleared positions* but **no intermediate board
snapshots**. The UI therefore can't render the clear→fall→refill sequence that makes match-3 feel good.

**Approach (keep it verifiable + determinism-safe):**
- Add an **additive** traced move to `match3-core`: `Game::play_move_traced(&mut self, from, to) ->
  (MoveReport, Vec<Board>)` that does *exactly* what `play_move` does — same RNG stream, so the final
  board and `state_hash` are byte-identical — but also pushes a `board.clone()` after each phase
  (after-swap, after-each-clear, after-each-gravity, after-each-refill). Do **not** change `play_move`
  or the pure ops, so the golden vectors are untouched. Prove `play_move_traced` and `play_move` end in
  the same state (a test: same seed + swap → equal final board + equal `state_hash`).
- Binding: a `play_swap_traced` export (or extend `play_swap` to stash the last trace) that returns the
  snapshot sequence as JSON. The committed state is the last snapshot (same as `play_swap`).
- UI (`match3.ts`): animate the sequence — render each snapshot with a short delay, CSS-transition gems
  falling and fade the cleared cells. **Reduced-motion → skip straight to settled** (as the win cascade
  does). Keep taps/drag disabled during the ~few-hundred-ms animation, then re-enable.
- Tests: (unit/rust) traced == atomic final state; (e2e) a swap animates then the board matches the
  core's settled board; the score still ends correct. Don't assert exact frames.

**Done when:** a swap visibly clears → falls → refills (cascades included), reduced-motion skips it, the
committed hash/score are unchanged, gate green, deployed.

## Item 2 — Reshuffle on a mid-run deadlock (contained; determinism-anchor care)

**Why it was deferred:** to stay verifiable the reshuffle must happen inside the **core's move
resolution** (so `Match3::replay`, which just deals + applies swaps, reshuffles identically) — which can
change the locked golden-vector `final_state_hash`.

**Approach:**
- In `Game::play_move`, after the cascade settles, if `has_legal_move(&self.board)` is false, **reshuffle
  deterministically** using `self.rng` (e.g. a Fisher-Yates permutation of the gem cells consuming
  `rng.index` draws) until the board has a legal move and no free matches; bound the attempts. This folds
  into `state_hash` (board + draws change) and is reproduced on replay because replay calls `play_move`.
- **Golden-vector anchor:** check every committed vector — does any end in a no-legal-move board? If so,
  its `final_state_hash` changes. Re-lock those hashes with the recorder pattern and **document in the
  commit exactly which vectors changed and why** (deadlock reshuffle). The other expectations
  (`move_legal`, `step0_cleared`, `step0_score`) must be unchanged — assert that. Update `RULES.md`.
- The 8×8/6 deal already guarantees an opening legal move, so this only affects post-move deadlocks
  (rare). The UI needs little/no change (surface a "board reshuffled" status if you like).
- Tests: a constructed deadlock-ending board → `play_move` leaves a board with a legal move,
  deterministically; and an `outcome` for such a game still `verify`s.

**Done when:** a would-be deadlock reshuffles instead of ending the round, replay/verify still holds,
golden vectors re-locked-and-explained, gate green, deployed.

## Item 3 — A variant objective (OWNER DECISION FIRST — do not invent)

**Why it was deferred:** variant objectives and specials are new modes + balance calls the master plan
reserved for the owner. **Surface the decision, get a choice, then build** (mirror how the target-score
objective was decided — present concrete options grounded in what the core supports, recommend one, ask).

Options to put to the owner (with the real cost of each):
- **Clear-the-blockers** *(recommended first — closest to the pond's verifiable-win model)*: deal gems +
  `Blocker` cells; win = all blockers cleared; metric = **moves/swaps-to-clear** (fewer better), exactly
  like solitaire's clean-clear. The engine already models blockers and `clear_cells` damages them.
  **New work:** a blocker-placing deal + a guarantee each daily board is actually clearable — i.e. a
  **solver / winnable-daily pack** (the same problem solitaire solved; reuse that shape). Verifiable win
  via `pond-outcome` (`Won` when no blockers remain).
- **Jelly** (clear a jelly overlay under gems) — needs a new per-cell jelly layer in the core (new state
  + hashing), plus solvability. Bigger core change.
- **Ingredients** (drop ingredient pieces to the bottom row) — new falling-piece mechanic. Biggest.
- **Specials** (striped 4-match / wrapped L-or-T / colour-bomb 5-match + activation cascades) — a large
  core extension (match-shape detection + special gems + activation) and heavy balance. A separate track.

**Do:** write the decision up, ask via the question tool, and only after a choice, plan+build that mode
as its own slice per `BUILDING-GAMES.md` (core rule + vectors → binding → UI mode toggle → how-to → tests
→ deploy). If clear-the-blockers is chosen, expect a match-3 winnable-daily solver + pack like solitaire's.

## Item 4 — Stronger reference / fraction tuning (fuzzy; a versioning wrinkle)

**Why it was deferred:** the per-deal par is a **greedy best-swap** playout (`reference_score`), which is
myopic (ignores cascades a lookahead would set up); the 30/60/90% fractions are a first guess.

**Approach + the wrinkle to respect:**
- Options: a shallow **lookahead/beam** `reference_score` variant in `match3-core` (compare score
  distributions across many seeds), and/or retune the fractions against **real play data** (scores from
  shared records) rather than guessing.
- **Versioning wrinkle:** `verify` re-derives targets via `targets_for(seed)`. Changing the reference or
  fractions changes targets for **every** seed, so a record made under the old par could re-verify to a
  different stars/`Won`. With no real users this is fine to change in place; **if records exist in the
  wild, bump `Match3::VERSION`** and keep the old par for old-version records (read the version from the
  `pond-docformat` envelope). Treat a par change as a rules-version bump. Document the choice.
- Tests: reference variant deterministic + monotonic; targets strictly increasing; if versioned, an
  old-version record still verifies under the old par.

**Done when:** the par is measurably fairer (or the fractions retuned with a recorded rationale),
verification versioning is handled, gate green, deployed.

---

## Guardrails (all items)

- **TDD-first**, wiring test through the real entry point (crate API / wasm boundary / `/match3/` URL),
  RED before GREEN. The board UI must **never decide legality** — always delegate to the core.
- **Determinism is the anchor:** any new state folded into a game must go into `state_hash`; any change
  that alters a locked golden-vector hash must be re-locked **and explained** in the commit.
- **Never panic in the binding** — status codes / empty-JSON, no `unwrap` on the hot path.
- Keep `pond-outcome` changes **additive** (solitaire must stay green).
- **Commit + push per stable point** (auto-deploys); keep `TODO/match3.md`, the plan header, and
  `docs/BUILDING-GAMES.md` in sync; regenerate guide shots on any visual change.
- Sandbox note: the Playwright e2e runs locally against the built site (works here); a *live*
  `fun.croft.ing` check may hit the known browser-egress limit — verify with the local build + `curl`.

## Definition of done (the whole batch)

Items 1, 2, and (if the owner greenlights one) 3 are shipped: cascades animate (reduced-motion-safe),
deadlocks reshuffle verifiably, and — pending the owner's pick — a second match-3 mode is playable; par
tuning (item 4) is either done with a recorded rationale or explicitly re-deferred with the versioning
note captured. Every commit leaves Rust + `npm run test` + `npm run e2e` green and is deployed; docs and
`TODO/match3.md` reflect reality.
