# Handoff prompt — build Phase D: the solitaire board UI (fun.croft.ing)

> Paste this into a fresh Claude Code session started in `/Users/cpettet/git/chasemp/CroftC`.
> It is self-contained: everything Phase D builds against is staged below. Follow the phase-plan
> execution discipline (TDD-first, wiring test through the entry point, commit + push per stable point,
> keep the plan doc in sync).

## Your task

Execute **Phase D — the solitaire board UI** of the delivery plan
`discovery/alpha/plans/2026-07-29-playable-solitaire.md` (read its "Phase D" section + the "UI/UX from
existing solitaire implementations" and "Gameplay decisions" subsections first — they are the spec).
Deliver a **playable, verifiable solitaire** at `fun.croft.ing/solitaire/`.

The game lives in the standalone repo **`CroftCommunity/fun`**, checked out at `CroftC/fun` (git
identity: **chasemp** / `chase@owasp.org` / remote `git@github-personal:CroftCommunity/fun`; run
`gh auth switch --user chasemp` if using `gh`). Commit **and push** per stable point (the owner has
been pushing each phase).

## What is already shipped (all green + pushed on `CroftCommunity/fun`)

- **`solitaire-core`** — full Klondike draw-1 engine (determinism-verified native==wasm).
- **`solitaire-wasm`** — the browser binding you drive (raw C-ABI + serde-JSON; no wasm-bindgen).
- **TS wrapper** `fun/src/games/solitaire-wasm.ts` — the typed API you call (below).
- **`pond-outcome` / `pond-docformat`** — the verifiable outcome record + envelope.
- **`solitaire-solver`** + **`fun/games/solitaire/daily-pack.json`** — 6 winnable seeds; `pack[0]` is
  your **win-path fixture** (replay it to a win).
- **The drawer chrome** — mounts a game via the contract below; `/solitaire/` currently shows
  "coming soon" (the registry marks it `status: "soon"`).
- Rust workspace: 52 tests green. Web: typecheck/lint/vitest(5)/Playwright(4, incl. axe) green.

## The contracts you build against (verified from the shipped code)

**1. Chrome game-module contract** — `fun/src/contract.ts`:
```ts
interface GameModule { mount(container: HTMLElement, services: GameServices): void; unmount(): void; }
interface GameServices { readonly mode: "drawer" | "fullscreen" | "standalone"; }
interface GameEntry { id; title; icon; status: "playable" | "soon"; load?: () => GameModule; }
```
Make solitaire playable = in `fun/src/registry.ts` flip its entry to `status: "playable"` with
`load: solitaireModule` (a `GameModule` factory you write in `fun/src/games/solitaire.ts`).

**2. The binding — `fun/src/games/solitaire-wasm.ts`** (already written, typed):
```ts
class Solitaire {
  static load(wasmUrl = "/solitaire.wasm"): Promise<Solitaire>
  newGame(seed: bigint): void
  board(): BoardView
  legalMoves(): SolMove[]
  currentHash(): string
  isWon(): boolean
  undo(): boolean            // true if a move was undone; marks assistance-used in the binding
  outcome(unfinished: "abandoned" | "stuck", declareAssistance: boolean): unknown  // pond-docformat envelope JSON
  play(move: SolMove): "applied" | "illegal" | "bad"
}
```
`BoardView = { foundations: [n,n,n,n]; stockCount: number; wasteTop?: CardView; wasteCount: number;
tableau: SlotView[][]; won: boolean }`; `SlotView = { faceUp: boolean; card?: CardView }` (**card is
absent for face-down cards — you cannot see hidden cards, by design**); `CardView = { suit: number;
rank: number }`.
`SolMove = "Draw" | "WasteToFoundation" | { WasteToTableau: { pile } } | { TableauToFoundation: { pile } }
| { TableauToTableau: { from; count; to } }`.
Encoding: **suit** 0=♣ 1=♦ 2=♥ 3=♠ (♣♠ black, ♦♥ red); **rank** 1..13 = A..K. `foundations[suit]` = top
rank on that suit's foundation (0 = empty). Tableau piles are bottom→top; the last slot is the exposed top.

**3. The daily pack / win fixture** — `fun/games/solitaire/daily-pack.json` is a `pond-docformat`
envelope `{ kind:"deal-pack", version:1, payload: PackEntry[] }`, `PackEntry = { seed: number; moves:
SolMove[] }`. **Replay `payload[0]`** (`newGame(BigInt(seed))`, then `play` each move) → `isWon()` is
true. Serve it as a static asset (add to `build.mjs`'s copy list, e.g. `/daily-pack.json`) so the daily
mode and the E2E can fetch it.

**4. Build / test / wasm toolchain:**
- Web: `npm run typecheck`, `npm run lint`, `npm run unit` (vitest/jsdom), `npm run e2e` (Playwright),
  `npm run build` (esbuild → `dist/`, copies `/solitaire.wasm`).
- **wasm:** `npm run build:wasm` (⇒ `tools/build-wasm.sh`, which builds with the **rustup** toolchain —
  Homebrew's rustc has no wasm std and shadows it on PATH, so `RUSTC` is set explicitly). `build.mjs`
  copies the artifact to `dist/solitaire.wasm`.
- **Playwright webServer must build the wasm first** — update `fun/playwright.config.ts` `webServer.command`
  to `npm run build:wasm && node build.mjs && node tools/serve.mjs` (today it skips `build:wasm`, so
  `/solitaire.wasm` would be missing for the E2E).
- Rust: `cargo test --manifest-path fun/Cargo.toml --workspace`; `cargo fmt --all --check`;
  `cargo clippy --workspace --all-targets`.

## Phase D deliverables (from the plan; gameplay decisions already made)

- **`fun/src/games/solitaire.ts`** — the `GameModule`. On `mount`: `await Solitaire.load()`, pick the
  seed (see modes), `newGame(seed)`, render the board and wire interactions. Render the standard
  Klondike layout: top-left **stock** (tap to draw; tap when empty to recycle) + **waste** (top
  playable); top-right **4 foundations**; below, **7 tableau columns** fanned downward (face-up cards
  overlap ~28%, rank+suit corner readable; face-down cards show a back).
- **Tap-to-move** (accessible foundation; drag is a later fast-follow): tap a source (waste top, or a
  face-up tableau card = the run from it to the top) → read `legalMoves()`, **glow the legal targets** →
  tap a target (foundation or tableau pile) → call the matching `play(move)` → re-render. Illegal tap →
  reject (board unchanged). **Double-tap a card → auto-send to foundation** if legal. Tap stock → draw.
  Keyboard-navigable; ARIA labels ("King of Spades, face up"); large touch targets.
- **Modes:** **daily deal by default** — seed = `pack[dayIndexUTC % pack.length]` from `daily-pack.json`
  (UTC rollover) — plus a **free-play/random** toggle (arbitrary seed; support `?seed=<n>` for
  deterministic testing).
- **Undo** control (calls `undo()`, which sets the binding's assistance flag). A **"Declare assistance
  used" setting, ON by default** — passed to `outcome(..., declareAssistance)`. An **"I'm stuck"** control
  → `outcome("stuck", ...)` → a `Stuck` record.
- **Verification-forward win screen:** on `isWon()`, lead with "Cleared clean ✓ — verifiable" (or
  "Cleared with assistance" / show `Stuck`), show the `outcome()` record + **moves-to-clear**, a one-tap
  **re-verify** (orchestrate via the binding: `newGame(seed)` → replay the record's moves →
  `currentHash()` === record `final_hash`), and a **share link** `/solitaire/?r=<base64url(record JSON)>`.
- **Share open path:** when `?r=` is present, decode + render the shared result and **re-verify it**
  before display (don't trust it).
- Flip solitaire to `status: "playable"` in `registry.ts`; update `fun/README.md`; breadcrumb
  `discovery/alpha/ROADMAP_TODO.md` E46 to "solitaire playable".

## Wiring tests (the gate)

`fun/tests/solitaire.spec.ts` (Playwright; the webServer builds the wasm):
- **Mechanics:** load `/solitaire/`, board renders (7 piles sized 1..7, hidden cards have no `card`);
  tapping the stock draws (wasteCount increases); selecting a card glows exactly the core's legal
  targets; performing one real legal move changes the board; an illegal tap leaves it unchanged; undo
  reverts. (Deal-agnostic assertions + one hook-informed real move.)
- **Win path (uses the fixture):** navigate to the `pack[0]` seed, replay `pack[0].moves` (via a
  `window.__solitaire` test hook exposing the `Solitaire` instance + a re-render, so the E2E drives the
  binding rather than 500 literal taps), assert the **win screen** appears with a **verifiable** record
  and a **share link**; open the share link in a fresh page and assert it **re-verifies**.
- Keep the existing 4 drawer E2E tests + 5 vitest green; add a vitest unit test for the win-screen
  render + verify-orchestration + share encode/decode (so those don't depend only on the E2E).

## Guardrails + open leans (confirm/adjust as you go)

- TDD-first; wiring test RED before, GREEN after; **commit + push after each stable point**; keep the
  delivery plan in sync (mark Phase D shipped in its header + status; flip front-plan P4 note).
- The board UI must **never decide legality** — always delegate to `legalMoves()` / `play()` status.
- Leans already recorded (override if you like): **daily rollover = UTC**; **share payload = the full
  self-verifying record** (base64url of the `pond-docformat` JSON); **undo IS in v1** with the declare-
  assistance setting (default ON); clean-clear = `Won && declared-no-assistance`.
- Design identity (Phase E) can land alongside or after; D can ship on the current neutral token
  baseline and be restyled. `frontend-design` skill + the plan's UI/UX section guide the look.
- Sandbox note: the Playwright E2E runs a **local** server + local chromium (works here); a *live*
  `fun.croft.ing` check may hit the known browser-egress limit — run the E2E against the local build.

## Definition of done

A stranger opens `fun.croft.ing/solitaire/` (or the local build), gets the **daily deal**, plays it via
taps with legal-move highlighting, and on a win sees a **verifiable clean-clear record + a share link**;
free-play + undo + "declare assistance" + "stuck" all work; `solitaire.spec.ts` (mechanics + win-path +
share round-trip) is green, existing tests still green, fmt/clippy/typecheck/lint clean; committed +
pushed; the delivery plan marks Phase D ✅ shipped.
