# Build prompt: Wyrdle — the daily word game (fun.croft.ing, Tier-1)

Copy this into a fresh session to build the next shelf game. It is self-contained:
it carries the context the new session needs so it does not depend on this one.

---

## What you're building

**Wyrdle** — a daily word-guessing game (Wordle-family) for the `fun.croft.ing`
shelf. Name = **wyrm + word** (with a nod to Old-English *wyrd*, "fate"), matching
Croft's Old-English streak; **icon = 🐉 (dragon)**; identity leans into a playful
**wyrm/dragon** motif on the shelf's existing token architecture. It is a
**distinct, non-trademarked** game (do NOT call it Wordle or copy NYT assets/word
lists) — original name, original word list, our own look.

Chosen as the next build (owner, 2026-07-31) because it is the corpus's own pick
for the **purest expression of the verifiable-daily-share thesis** — single-player,
async, needs zero networking (`games-pond-authoritative-list.md` #7) — and it adds
a new genre to a shelf that already has card / match-3 / bubble.

## Context you need

- `fun.croft.ing` is a **two-tier game shelf** (discovery COHESION §62). **Tier 1**
  (this) = Croft-native, **build-fresh, determinism-first, verifiable**. Live
  Tier-1 games: **solitaire, match-3, bubble shooter**. Cribbage is gated (P2P).
  SuperTuxKart is a Tier-2 wrap **under owner review in parallel** — don't touch it.
- The **proven Tier-1 recipe** (follow it; `bubble` is the closest, freshest
  template): a Rust **`<game>-core`** crate (rules + `RULES.md` + golden vectors +
  a `state_hash`, native==wasm), a raw **C-ABI + serde-JSON `<game>-wasm`** binding
  (holds the game, never panics), a typed TS wrapper, a `GameModule` UI, a
  **verifiable `pond-outcome`** record (replay `(seed, moves)` → re-derive hash;
  `?r=` deflated self-verifying share), tap-first with the **core deciding
  legality**, WCAG-AA tokens in both themes, shared hints/settings, a How-to guide.
- **Standards are non-negotiable:** read `fun/docs/BUILDING-GAMES.md` and follow
  every section + its new-game checklist. **Game isolation:** everything Wyrdle
  owns lives in its own dirs — `crates/wyrdle-core`, `crates/wyrdle-wasm`,
  `src/games/wyrdle/` (module + wasm wrapper + how-to + assets); shared shelf infra
  (drawer chrome, settings, theme, how-to renderer, `pond-*`) is reused, never
  duplicated.
- **Process:** use the **`phase-plan` skill** (Pass 1+2+3) before building, and
  **TDD RED→GREEN** for every increment (see the global coding-agents rules +
  `fun/CLAUDE.md`). Commit per green phase.

## The design (Wyrdle specifics)

- **Objective:** guess the hidden N-letter word (default 5) within K guesses
  (default 6). Each guess returns a per-letter pattern: correct / present /
  absent. Win = exact word within K; else lose.
- **Determinism + verifiable outcome:** the answer for a given seed is fixed (from
  a **daily answer pack** — a curated list, indexed by UTC day, the same
  winnable-daily-pack machinery solitaire/match-3/bubble use). The move list is the
  **sequence of guesses**; `replay(seed, guesses)` re-derives each guess's pattern
  and the solved/failed result — **nothing trusted**. The classic **emoji-grid**
  (🟩🟨⬛) is the natural verification-forward result + the deflated `?r=` share
  (which re-verifies on open). This is the whole thesis in miniature: see a
  friend's grid, tap, play the same seed, your grid posts back.
- **Input:** tap-first — an on-screen keyboard (tap letters) that also mirrors
  physical typing; Enter to submit, Backspace to delete. The **core decides
  legality** (a guess must be a real word in the allowed list; the UI asks the core
  and rejects/【shakes】 non-words — an illegal guess changes nothing, with an E2E
  guardrail). Keyboard keys colour by best-known state.
- **Word lists (license-clean):** source a **permissively-licensed** English word
  list for (a) the allowed-guesses set and (b) a curated **answers** set — e.g. a
  public-domain / MIT word list (SCOWL, dwyl/english-words, or similar; confirm the
  licence and record it). Do NOT reuse NYT's Wordle answer list. Bake the answers
  into a daily pack; the allowed set can be embedded in the wasm.
- **Modes:** **daily** (UTC day → pack index) + **free-play** (`?seed=`) + shared
  (`?r=`). Hints on-by-default (reveal a letter? counts as assistance) + the shared
  settings; hints-off → "I'm stuck"/"give up" ends honestly.
- **Identity:** a playful **wyrm/dragon** theme on `tokens.css` (green/present/
  absent tiles must clear WCAG-AA in both themes; the 🟩🟨⬛ semantics need
  shape/label too, not colour alone — colour-blind-safe). Registry entry:
  `{ id: "wyrdle", title: "Wyrdle", icon: "🐉", status: "playable", load }`, own
  `/wyrdle/` URL. Add `TODO/wyrdle.md` + shelf-order/README updates.

## Rules / logistics

- **Git identity:** chasemp (`chase@owasp.org`, `github-personal`); repo
  `CroftCommunity/fun`. **Work in a git worktree off latest `origin/main`** — the
  match-3 session (and possibly others) actively commit to `fun`; do NOT build in a
  shared checkout. Push + PR (`gh auth switch --user chasemp` first — it can slip to
  the EMU account) only when the owner asks; the owner has been merging PRs to
  deploy (Pages workflow builds the wasm + publishes).
- **Mirror `bubble`** end-to-end — it is the most recent, cleanest Tier-1 example
  (hex board aside, the core→wasm→UI→outcome→daily-pack→how-to→e2e shape is
  identical). Its plan: `fun/plans/2026-07-30-bubble-shooter.md`.
- **Deliverable:** Wyrdle playable at `fun.croft.ing/wyrdle/` — daily + free-play +
  verifiable emoji-grid result + `?r=` share, hints/settings, How-to guide; full
  gate green (cargo `--workspace` + fmt + clippy; unit + e2e incl. axe both themes,
  360px, illegal-guess guardrail).

## One meta-note

The magic of this game is the **share-and-compare daily loop** — it is the
lightest, purest demonstration that a Croft game's outcome is a verifiable,
shareable record with no server. Keep that front and centre: the result screen and
the `?r=` share are the point, not an afterthought.
