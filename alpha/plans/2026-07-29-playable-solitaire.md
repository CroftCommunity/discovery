# Playable, verifiable solitaire in the drawer — the delivery slice

**Status:** EXECUTION IN PROGRESS (substrate-first). ✅ A (`pond-docformat` `808df73`), ✅ B
(`pond-outcome` `85df812`), ✅ C (binding `ff55c0d`) shipped + pushed. Next: **D (board UI, free-play
first)**, then **S (solver + winnable daily pack)** → turn on dailies; **E (design)** alongside.
Note: D's full *win-path* E2E is coupled to S (needs a known-winnable seed + winning-line fixture); D's
free-play mechanics E2E + a unit-tested win-screen do not need S.
**What this is:** the **delivery plan** that takes the shipped pieces (the drawer chrome, `solitaire-core`,
the cross-build determinism test) to a **playable, verifiable solitaire on `fun.croft.ing`**. It is a
cross-cutting slice spanning **master-plan P5/P6** (the `pond-docformat` / `pond-outcome` substrate —
still stubs) and **front-plan P2/P3/P4**. It **supersedes the high-level P2/P3/P4 stubs** in
`2026-07-28-games-drawer-solitaire-ui.md` and **pulls in master-plan P5/P6**; those phase entries
should point here. Grounded in the actual shipped code, not the earlier inference.

---

## Problem Statement

Shipped and green on `CroftCommunity/fun`:
- `solitaire-core` — the full Klondike draw-1 engine (`new_game`/`play_move`/`legal_moves`/`state_hash`,
  T1–T5, 14 tie-break tests, golden vectors), cross-build-verified native==wasm.
- the games-drawer chrome — accessible slide-out drawer, per-game static URLs, the game-module mount
  contract (`mount(container, services)` / `unmount()`), a placeholder game proving all three modes.
- `xbuild` — the cross-build determinism harness proving a **raw C-ABI + serde-JSON** path from Rust to
  wasm works under node, **without `wasm-bindgen`** (which is not installed).

Missing for a playable solitaire a stranger can open, play to a win, and get a self-verifying result:
1. **The board can't reach the browser** — `solitaire-core` has no browser binding, and its `GameState`
   doesn't even derive `Serialize` yet.
2. **No verifiable outcome** — `pond-outcome` (P8/master P6) and its envelope `pond-docformat`
   (P2/master P5) are empty stubs, so a win produces nothing checkable.
3. **No board UI** — the drawer mounts a placeholder; solitaire shows "coming soon".
4. **No visual identity** — the drawer is on a neutral baseline (front P2 not done).

Goal: deliver all four, so `/solitaire/` on `fun.croft.ing` is a real, offline, accessible game whose
win is an individually re-verifiable record.

---

## Reasoning

### Why the substrate (P5/P6) comes before the board UI

Front P4's definition of done includes "win → verifiable `pond-outcome` record." That record is a
`pond-outcome` value serialized through `pond-docformat`. Both are stubs. Building the board UI first
and bolting the record on later would mean shipping a "playable" solitaire whose central pond
property — the verifiable clean-clear — is faked or deferred. So `pond-docformat` (P5) → `pond-outcome`
(P6) are built first, as the substrate the binding and UI consume. They are also game-agnostic (match-3
and cribbage reuse them), so paying for them here is paying once.

### Why the binding is raw C-ABI + serde-JSON, not wasm-bindgen

`xbuild` (master P2) already proved raw C-ABI exports + a static buffer + a node loader work to
`wasm32-unknown-unknown` with the rustup toolchain, and `wasm-bindgen`/`wasm-pack` are **not installed**
(installing them is a `cargo install` + toolchain surface we don't need). The binding follows the
`xbuild` pattern: the wasm module **holds the game state** (a single game per tab — solitaire is
single-player), and exposes small integer-argument exports plus JSON-string returns via a `ptr`+`len`
static buffer. The UI never re-implements rules; it renders the board JSON and calls typed move exports.

### Why the wasm holds state (not stateless round-tripping)

A stateless binding would serialize the whole `GameState` to JS and back on every move — verbose
marshalling and easy to desync. Holding the single game in a wasm-side `static` (single-threaded wasm,
one game per tab) means the UI calls `new_game(seed)`, then `legal_moves_json()` / typed `play_*` /
`board_json()` — the state lives in one place, the source of truth. The move list is tracked wasm-side
too, so the outcome record is produced from the authoritative sequence.

### Why typed move exports (not a serialized Move)

`Move` has struct variants (`TableauToTableau { from, count, to }`). Marshalling that across C-ABI is
awkward. Instead expose one export per move type with integer args
(`play_draw()`, `play_waste_to_foundation()`, `play_waste_to_tableau(pile)`,
`play_tableau_to_foundation(pile)`, `play_tableau_to_tableau(from, count, to)`), each returning a status
code (0 = applied, 1 = illegal, 2 = bad-arg). The UI picks the export from the tapped source/target;
`legal_moves_json()` tells it what's legal so it highlights correctly. This keeps the ABI trivial and
the legality decision entirely in the core.

### Why tap-to-move first (drag is the fast-follow, already decided)

Confirmed with the owner in the front-plan: tap source → tap target is the accessible foundation
(mouse/touch/keyboard identical, legal-move highlighting natural); drag-and-drop is a later enhancement
(front-plan Phase 7). Nothing here precludes it.

### UI/UX from existing solitaire implementations (informs Phases D + E)

Drawn from canonical Klondike layouts and modern web implementations (Microsoft Solitaire,
solitaired.com, GNOME Aisleriot) — the conventions Phase D's board and Phase E's identity target:

- **Layout (the standard grid):** top-left **stock** (face-down; tap to draw) + **waste** (face-up,
  top playable); top-**right** four **foundations** (one per suit, build A→K); below, **7 tableau
  columns** fanned downward — face-up cards overlap ~28% (rank+suit corner readable), face-down cards
  tighter. Felt-table motif (our own palette on the token layer, Phase E).
- **Interactions:** **tap source → tap target** (our accessible foundation; legal targets glow, illegal
  snaps back); tap stock to draw, tap empty stock to **recycle** (↻); **double-tap → auto-send to
  foundation** as a loved convenience; **drag-and-drop is the fast-follow** (front-plan P7) with tap
  always present. Auto-flip exposed cards; a win **cascade** animation.
- **Controls:** Undo + Hint (both count as **assistance**), New game (**daily** / free-play toggle),
  and — our signature — **verify + share** on win. Move counter = our **moves-to-clear** metric.
- **Modern convention = our design, validated:** the Wordle-ification of solitaire (Microsoft/
  solitaired) is exactly **daily challenge + streak/clean-clear stats + share results** — which is our
  daily deal + clean-clear count + share link. Good confirmation the pond direction matches where the
  genre went.
- **Accessibility + responsive:** large touch targets, keyboard select (arrow + Enter, aligns with the
  tap model), ARIA card labels ("King of Spades, face up"), high-contrast; the 7-column grid scales,
  fan tightens on narrow screens, stock/waste/foundations stay reachable. (Sources folded into the
  Review Log.)

### Alternatives considered and rejected

- **wasm-bindgen binding** — rejected: not installed, unneeded; raw C-ABI + serde-JSON is proven.
- **Stateless binding (round-trip GameState)** — rejected: marshalling churn + desync risk.
- **Board UI before the outcome substrate** — rejected: ships a solitaire without its verifiable-clean-
  clear, the pond's whole point.
- **A bespoke per-game save format** — rejected: `pond-docformat` is the shared, versioned envelope
  (P2 discipline) all games use; solitaire is its first consumer.

---

## Verified Assumptions

Confirmed firsthand (2026-07-29):

- **`solitaire-core` public API** (`crates/solitaire-core/src/lib.rs`): `GameState` (fields
  `foundations: [u8;4]`, `stock: Vec<Card>`, `waste: Vec<Card>`, `tableau: [Vec<TableauCard>;7]`,
  `draws: u64`), `GameState::new_game(seed)`, `play_move(Move) -> Result<(), MoveError>`,
  `legal_moves() -> Vec<Move>`, `is_won()`, `state_hash(&GameState) -> String`; `Move` (Serialize +
  Deserialize), `MoveError` (`Illegal` / `BadPile`). `TableauCard { card, face_up }`, `Card { suit,
  rank }` with `index()`/`color()`.
- **`GameState`/`Card`/`TableauCard` do NOT derive `Serialize`** (`derive(Clone[,Copy],PartialEq,Eq,
  Debug)` only) — the binding (Phase C) must add `Serialize` (behind a view or directly) to emit board
  JSON. `Move` already derives Serialize.
- **`pond-docformat` and `pond-outcome` are 10-line stub crates** (module doc only, no API).
- **The chrome contract exists** (`src/contract.ts`): `GameModule { mount(container, services);
  unmount() }`, `GameServices { mode }`, `GameEntry { id, title, icon, status, load? }`; the registry
  marks solitaire `status: "soon"` today. Making it playable = provide a `load` that returns a
  `GameModule`, flip status to `"playable"`.
- **The raw-C-ABI wasm path works** (`crates/xbuild`): static buffer + `ptr`/`len`, built with
  `RUSTC="$(rustup which --toolchain stable rustc)" "$(rustup which --toolchain stable cargo)" build
  --target wasm32-unknown-unknown`, loaded via `WebAssembly.instantiate` in node. Browser loads the
  same way (`fetch` + `WebAssembly.instantiateStreaming`).
- **esbuild build** emits per-game static pages; `/solitaire/` already exists (shows "coming soon").
- **Determinism discipline:** the RNG uses fixed-width `u32` sampling (the cross-build fix); any new
  state added to `GameState` must be folded into `state_hash` or it breaks the anchor.

Unverified — resolve in-phase (do not assume):

- Exact **board-JSON shape** the UI consumes (Phase C pins it against the real `GameState`).
- How the wasm build is **wired into esbuild** for the browser (fetch the `.wasm` asset; where
  `build.mjs` places it) — Phase C decides and records.
- Whether serving `.wasm` needs a MIME/type tweak in `tools/serve.mjs` (it already maps `.wasm`).

---

## Documentation Impact

- **`fun/crates/pond-docformat/src/lib.rs`** + fixtures — real implementation. Phase A.
- **`fun/crates/pond-outcome/src/lib.rs`** + tests — real implementation. Phase B.
- **`fun/crates/solitaire-core/src/…`** — add `Serialize` derives (+ possibly a `board_view`). Phase C.
- **`fun/crates/solitaire-wasm/src/lib.rs`** — the raw-C-ABI binding (currently a stub). Phase C.
- **`fun/src/games/solitaire.ts`** (NEW) + **`fun/src/registry.ts`** (flip to `playable` + `load`) +
  **`fun/build.mjs`** (wasm asset wiring). Phase D.
- **`fun/src/tokens.css`** / **`fun/styles.css`** + **`fun/docs/DESIGN.md`** (NEW) — the identity. Phase E.
- **`fun/README.md`** — update the crate table (substrate built; solitaire playable). Phase D/E.
- **`fun/crates/xbuild`** — optionally add solitaire replay-vector cross-check reusing the binding.
  Phase C (follow-on).
- **`discovery/alpha/plans/2026-07-28-games-drawer-solitaire-ui.md`** — annotate Phases 2/3/4 to point
  here (sequential discovery-repo edit). Phase A (first phase).
- **`discovery/alpha/plans/2026-07-27-games-pond-fun-crofting.md`** — Outcome Summary rows for P5/P6 as
  they land; **discovery-repo edits done sequentially** (never inside a parallel set). Phases A/B.
- **`fun/crates/solitaire-solver/**`** (NEW) + **`fun/games/solitaire/daily-pack.json`** (NEW) — the
  build-time solver + generated winnable-daily pack. Phase S.
- **`fun/src/settings.ts`** (or chrome settings) — the "Declare assistance used" toggle. Phase D.
- **`discovery/alpha/ROADMAP_TODO.md` E46** — breadcrumb when solitaire goes playable; also note the
  solitaire solver now in scope (resolves the earlier deferred "solitaire par/solver" question). Phase D.

---

## Concurrency Map

```
Sequential spine:  Phase A (pond-docformat) → Phase B (pond-outcome) → Phase C (binding) → Phase D (board UI)
Parallel tracks (all merge before D):
  • Phase E (design system)          ∥ A–C  — CSS/theme + chrome.ts theme toggle
  • Phase S (solver + daily pack)    ∥ A–C  — new crate + data file, feeds D's daily mode
```
Phase D depends on C + B + **S** (daily pack) + E (tokens). Free-play works without S; the default
daily mode needs the pack.

**Parallel {Phase E ∥ {A→B→C}}:**
- **Disjoint write-sets (verified per file):** Phase E owns `fun/src/tokens.css`, `fun/src/theme.ts`,
  `fun/styles.css`, `fun/docs/DESIGN.md`, **and `fun/src/chrome.ts`** (the theme-toggle control lives in
  the chrome header — a real chrome edit, so E owns it in this window). Phases A/B write `fun/crates/**`
  only. Phase C writes `fun/crates/solitaire-wasm/**`, `fun/crates/solitaire-core/src/{board,card}.rs`
  (the `Serialize` derives), `fun/src/games/solitaire-wasm.ts`, and `fun/build.mjs`. **The one file that
  could be contested is `chrome.ts` — and only E touches it** (A/B touch no `fun/src`; C touches
  `src/games/*` + `build.mjs`, never `chrome.ts`/`main.ts`), so the sets are genuinely disjoint and the
  parallelism is safe. If, mid-flight, C turns out to need a `chrome.ts` change, pull it sequential.
- **Shared-state contract (invariants):** if run as concurrent agents, both in worktrees off the `fun`
  feature branch; neither invokes `git checkout`/`stash`/`rebase` in the parent worktree; neither edits
  the **discovery repo** (all plan-doc/roadmap edits are sequential, done by the orchestrator); Phase E
  touches no `crates/**`, the Rust phases touch no `styles.css`/`tokens.css`/`DESIGN.md`; each builds to
  its own `target/` / `node_modules` is shared read-only (no `npm install` in E — deps already present).
- **Phase S ∥ A–C:** S adds a **new member crate** (`crates/solitaire-solver`) + a data file
  (`games/solitaire/daily-pack.json`) — disjoint from A/B (`pond-*`), C (`solitaire-wasm` + core
  derives), and E (styling). Its one shared touch is the workspace `Cargo.toml` `members` list; add the
  `solitaire-solver` member **once, up front** (the master-plan stub-freeze pattern) so S runs parallel
  without a `Cargo.toml` race. S reads `solitaire-core` (done), writes only its own crate + the pack.
- **Re-entry verification:** parent HEAD == pre-dispatch SHA; `git -C discovery status` clean;
  `crates/**` untouched by E, `styles.css`/`tokens.css` untouched by A/B/C/S, `Cargo.toml` `members`
  frozen since dispatch; `npm test` + `cargo test --workspace` green on the merged branch; no orphan
  `cargo`/`node` processes.

Everything else sequential: B needs A (envelope); C needs solitaire-core (done) + B (outcome export);
D needs C (binding) + B (record) + **S (daily pack)** + the chrome (done). **E and S are independent
tracks** parallelizable with A–C. Default: run **sequential A→B→C→D** with **E and S folded in before
D**, unless the owner wants E/S parallelized as concurrent agents.

---

## Phases

### Phase A — `pond-docformat` (master P5): the versioned document envelope — ✅ SHIPPED (`fun` `808df73`)

**Goal:** one versioned, forward/unknown-field-tolerant serialization envelope for all durable pond
documents (saves, codes, outcome records), with a per-version fixture. Fail-loud on unreadable
newer-major.
**Changes:**
- [ ] `pond-docformat`: a generic `Envelope<T>` (or `{ kind: String, version: u32, payload: serde_json::Value }`
  plus typed read/write) that tags each document with `kind`+`version`; on read, an **unknown minor**
  field is preserved/ignored per a documented policy (not silently dropped without record), and an
  **unknown major** version is a **loud typed error** (`thiserror`), never a silent fallback.
- [ ] `read<T: DeserializeOwned>(bytes) -> Result<Doc<T>, DocError>` + `write<T: Serialize>(kind, version, &T) -> Vec<u8>`.
- [ ] `fixtures/` — one committed fixture per (kind, version); the P10 compatibility-matrix seed.
**Call chain:** (consumed later) `pond-outcome`/saves → `pond_docformat::{read,write}` → typed round-trip.
**Wiring test:** `test_roundtrip_and_version_policy` — write a v1 doc, read it back typed; load a
committed older fixture under current code (asserts the documented unknown-field behavior); assert a
synthetic newer-**major** fixture returns the loud typed error. RED before the crate, GREEN after.
**Depends on:** nothing (leaf substrate).
**Read-set:** — . **Write-set:** `crates/pond-docformat/**`, `fun/crates/pond-docformat/fixtures/**`.
**Shared-state contract:** no shared mutable state beyond the file write-set.
**Risks:** over-engineering the schema — keep it a thin envelope; the **policy** (unknown-field +
version-skew handling, fail-loud) is the deliverable. Silent fallback on skew is forbidden.
**Done when:** 1) **Behavioral:** any game can persist/reload a typed document through one versioned
envelope; an old fixture loads, an unreadable newer-major fails loudly. 2) **Verification:**
`cargo test -p pond-docformat` green incl. the wiring test.
**Validation:** **Moderate** — wiring test + author two versions of one fixture and prove forward-load.

### Phase B — `pond-outcome` (master P6): the verifiable outcome record — ✅ SHIPPED (`fun` `85df812`)

**Goal:** a self-checking outcome record: given `(kind, seed, move list)`, replay via the game core and
emit `{ kind, seed, result, final_hash, move_count }` that anyone re-verifies by replaying. Local only.
**Changes:**
- [ ] `pond-outcome`: `Record { kind, seed, moves, move_count, final_hash, result, assistance }` where
  `result: Outcome` is `Won | Stuck | Abandoned` and `assistance: Option<bool>` is the **self-declared**
  flag (`Some(true/false)` when the "Declare assistance used" setting is on, `None` when off). `attest(...)`
  replays through the core and records `final_hash = state_hash` + `result` (`Won` verified by `is_won`
  after replay). `verify(&Record) -> bool` re-replays and re-hashes — **never** trusts a stored field —
  and, for `result: Won`, also asserts `is_won` holds after replay. On mismatch, `verify` makes the
  expected-vs-actual hash available (typed result or log), so a tamper/regression is diagnosable, not a
  bare `false`. **Assistance is NOT verifiable** (it can't be derived from the winning move list — it is
  an honesty declaration); `verify` proves the *deal was cleared legitimately*, and `assistance`/`Stuck`
  are declared metadata. `clean_clear(&Record) = result == Won && assistance == Some(false)`.
- [ ] A "clean-clear count" accumulator (additive count, never a ratio — the discipline).
- [ ] Depends on a game core to replay: take a generic replay closure, or feature-gate a `solitaire`
  integration. Simplest: `pond-outcome` is generic over a `Replay` trait; `solitaire-core` (or the
  binding) implements it. Decide in-phase; record the choice.
**Call chain:** game end → `pond_outcome::attest(kind, seed, moves)` → replay via core → record;
`verify(record)` → re-replay → hash match/mismatch.
**Wiring test:** `test_outcome_reverifies_and_tamper_detected` — attest a real solitaire win (seed +
a scripted winning move list), `verify` passes; mutate one move / the `final_hash` and assert `verify`
fails. Runs through `solitaire-core`, not a mock. RED before, GREEN after.
**Depends on:** Phase A (envelope), `solitaire-core` (done).
**Read-set:** `crates/solitaire-core/**`, `crates/pond-docformat/**`.
**Write-set:** `crates/pond-outcome/**`.
**Shared-state contract:** no shared mutable state beyond the file write-set. **No network** (explicit).
**Risks:** conflating record with leaderboard (out of scope, gated); trusting client arithmetic —
`verify` must re-run the core. A scripted *winning* solitaire move list is non-trivial to hand-author;
mitigate by capturing one via a short greedy/solver run in a `#[ignore]` helper and committing it as a
fixture (like the golden-vector recorder), or by asserting on a *partial* attested record if a full win
is impractical — record which.
**Done when:** 1) **Behavioral:** finishing (or advancing) a game yields a record any party re-verifies
by replay; tampering is detected. 2) **Verification:** `cargo test -p pond-outcome` green incl. wiring.
**Validation:** **Moderate** — wiring test + manually tamper a serialized record and confirm rejection.

### Phase C — `solitaire-wasm`: the browser binding (front P3, raw C-ABI + serde-JSON) — ✅ SHIPPED (`fun` `ff55c0d`)

**Goal:** the UI can drive `solitaire-core` in the browser: start a game, read the full board as JSON,
enumerate legal moves, apply typed moves, detect a win, and export a verifiable outcome record — all
computed by the Rust core over wasm, no `wasm-bindgen`.
**Changes:**
- [ ] `solitaire-core`: add `#[derive(Serialize)]` to `GameState`/`Card`/`TableauCard` (or a
  `board_view` projection) so the board emits JSON. Fold nothing new into `state_hash` (serialize is
  additive, not state). Re-run the golden vectors (hashes unchanged — serialize doesn't alter state).
- [ ] `solitaire-wasm` (cdylib, raw C-ABI, `xbuild` pattern): a wasm-side `static` holding the current
  `GameState` **and** the applied move list. Exports: `new_game(seed_lo, seed_hi)`;
  `board_json() -> ptr` + `board_len() -> u32`; `legal_moves_json() -> ptr`/`len`; typed
  `play_draw()`, `play_waste_to_foundation()`, `play_waste_to_tableau(pile)`,
  `play_tableau_to_foundation(pile)`, `play_tableau_to_tableau(from, count, to)` → status code
  (0 applied / 1 illegal / 2 bad-arg); `is_won() -> u32`; `outcome_json() -> ptr`/`len` (the full
  `Record` via `pond-outcome` over the tracked move list).
- [ ] **Gameplay state in the binding:** an **undo stack** (snapshot the `GameState` before each
  `play_*`; `undo()` pops and sets an `assistance_used` flag); `declare_assistance(on: u32)` wiring the
  setting (controls whether `outcome_json` includes `assistance`); `mark_stuck()` → `result = Stuck`;
  `verify_json(ptr, len) -> u32` (re-verify a shared record, for the share-link open path). Seed
  handling stays in JS (daily = date-derived index into the Phase S pack; free = arbitrary); the binding
  just takes a `u64` seed via `new_game(seed_lo, seed_hi)`.
- [ ] A TS wrapper `fun/src/games/solitaire-wasm.ts` that loads the `.wasm`, decodes the `ptr`/`len`
  strings, and presents a typed API (`newGame`, `board()`, `legalMoves()`, `play(move)`, `undo()`,
  `isWon()`, `markStuck()`, `declareAssistance(bool)`, `outcome()`, `verify(record)`). `build.mjs`
  builds + places the `.wasm` (documented; served with the existing `.wasm` MIME).
- [ ] (Follow-on) add a solitaire case to `xbuild`/`check.mjs` for an active native==wasm cross-check
  through the binding.
**Call chain:** UI → `solitaireWasm.newGame(seed)` → wasm holds `GameState` → `board()` JSON → UI
renders; UI tap → `play_*` → wasm mutates → `board()` re-read.
**Wiring test:** `binding.spec.ts` / a node harness — load the wasm, `new_game(0)`, assert `board_json`
matches the deal (spot-check pile sizes + a known card), replay the golden-vector draw-cycle via
`play_draw()` ×28 and assert `state`-derived hash equals the **locked native golden hash** (determinism
through the boundary), and assert an illegal typed move returns status 1 with the board unchanged. RED
before the binding, GREEN after.
**Depends on:** Phase B (`pond-outcome` for `outcome_json`), `solitaire-core` (done), `xbuild` pattern.
**Read-set:** `crates/solitaire-core/**`, `crates/pond-outcome/**`, `crates/xbuild/**` (pattern).
**Write-set:** `crates/solitaire-wasm/**`, `crates/solitaire-core/src/{board,card}.rs` (derives),
`fun/src/games/solitaire-wasm.ts`, `fun/build.mjs` (wasm asset).
**Shared-state contract:** wasm build via the rustup toolchain (RUSTC recipe); no ports; the wasm
`static` game state is per-instance in the browser. Adding derives to solitaire-core must keep the
golden hashes byte-identical (verify).
**Risks:** the board JSON drifting from the core model → pin it with the binding wiring test asserting
through the boundary. `static mut` for the held game needs the same `// SAFETY` discipline as `xbuild`
(single-threaded wasm). serde on `wasm32-unknown-unknown` is already proven (serde_json compiles).
**The binding must never panic:** a Rust panic in wasm **aborts the whole module** (the game becomes
unresponsive). Every fallible path maps to a status code / empty-JSON return, not `unwrap`/`panic!`:
`play_*` returns 0/1/2; a serialization failure (should be impossible for the board) returns an empty
buffer the wrapper treats as an error, not a trap. No `unwrap`/`expect` on the hot path.
**Done when:** 1) **Behavioral:** TS starts a solitaire game, reads the board, lists legal moves,
applies typed moves (illegal rejected), detects a win, and exports a re-verifiable outcome — reproducing
the golden hashes through the wasm boundary. 2) **Verification:** `binding.spec.ts` green + solitaire
golden hash matches through the binding.
**Validation:** **Broad** — wiring test + cross-check the same wasm the browser loads + a manual
`newGame → play → board` from a node REPL.

### Phase D — solitaire board UI (front P4): tap-to-move, win → verifiable record

**Goal:** `/solitaire/` is **playable**: a real board over the binding, tap-source → tap-target with
legal-move highlighting from the core, draw-1 stock cycling, win → a verifiable `pond-outcome` record
shown. Implements the chrome game-module contract; mounts in all three modes.
**Changes:**
- [ ] `fun/src/games/solitaire.ts` — a `GameModule` (`mount`/`unmount`) that: loads the binding,
  `newGame(seed)`, renders the board JSON (7 tableau piles with face-up/down cards, 4 foundations, stock
  pile, waste top), and drives tap-to-move: tap a card/stack → `legalMoves()` → glow legal targets →
  tap target → the matching `play_*` → re-render. Draw by tapping the stock; recycle when empty.
- [ ] **Modes:** **daily deal by default** (date-derived index into the Phase S winnable-seed pack, so
  everyone gets the same winnable deal that day) + a **free-play / random** toggle (arbitrary seed). A
  visible "today's deal" vs "free play" control.
- [ ] **Undo + assistance:** an undo control (calls `undo()`, which sets the binding's assistance flag);
  a **"Declare assistance used" setting, ON by default** (`declareAssistance(true)`), surfaced in the
  chrome/settings. When on, the outcome carries `assistance`; when off, it's omitted.
- [ ] **Stuck:** an "I'm stuck" control → `markStuck()` → `result = Stuck` outcome (with the same
  share/record surface as a win).
- [ ] **Verification-forward win screen:** lead with "Cleared clean ✓ — verifiable" (or "Cleared with
  assistance" / "Stuck") + the `outcome()` record + a one-tap **re-verify** (`verify`) + **moves-to-clear**.
- [ ] **Share-results link:** encode the record (base64url of the `pond-docformat` JSON) into a
  `/solitaire/?r=<record>` URL; opening it renders the shared result and **re-verifies it** via
  `verify(record)` before displaying (so a shared claim is checked, not trusted). The pre-leaderboard
  social hook.
- [ ] `registry.ts`: flip solitaire to `status: "playable"` with `load: solitaireModule`.
**Call chain:** `/solitaire/` → chrome mounts `solitaireModule` → `newGame(seed)` → render → tap →
`play_*` → re-render → win → `outcome()` shown + verified.
**Wiring test:** `solitaire.spec.ts` (Playwright E2E) — load `/solitaire/`, assert the dealt board
renders (7 piles, correct counts); play a **scripted winning deal** (a fixed seed with a known winning
move sequence, from the Phase B fixture) via taps; assert the win state **and** a verifiable outcome
record appears and re-verifies. Name the edges: an illegal tap is rejected (board unchanged); tapping a
card glows exactly the legal targets; stock cycling works; **the share link round-trips** (win → copy
`?r=` → open in a fresh page → it re-verifies and renders); **assistance declaration** (with the setting
on, using undo marks the record assisted; with it off, the record omits assistance). Repeat in-drawer
and full-screen. RED before the UI, GREEN after.
**Depends on:** Phase C (binding), Phase B (`pond-outcome`), **Phase S (winnable-daily pack — for the
default daily mode; free-play works without it)**, the chrome (done), Phase E (tokens, for a non-neutral
board — or ship on the baseline and restyle when E merges).
**Read-set:** `fun/src/games/solitaire-wasm.ts`, `fun/src/contract.ts`, `fun/src/tokens.css`.
**Write-set:** `fun/src/games/solitaire.ts`, `fun/src/registry.ts`, `fun/README.md`.
**Shared-state contract:** front-end only; no shared mutable state beyond the file write-set.
**Risks:** the UI drifting into deciding legality — it must delegate to the core (`legalMoves`/`play_*`
status); the illegal-tap assertion guards it. Rendering a full tableau on small screens — keep
layout-driven; responsive polish is front-plan Phase 5. Authoring a *scripted winning deal* — reuse the
Phase B fixture (the captured winning sequence), don't hand-solve in the test.
**Done when:** 1) **Behavioral:** a stranger opens `/solitaire/`, plays a Klondike draw-1 deal to a win
via taps with legal-move highlighting, and gets a re-verifiable clean-clear record. 2) **Verification:**
`solitaire.spec.ts` green (scripted win + illegal-tap + all three modes) + Rust/unit still green.
**Validation:** **Broad** — wiring E2E + a manual play session (mouse + touch) + confirm the emitted
record re-verifies.

### Phase E — design system / the playful identity (front P2)  *(parallel with A–C)*

**Goal:** `fun.croft.ing`'s own playful identity on croft-pwa's token architecture — palette, type,
card/felt motifs, light/dark — applied to the chrome and the solitaire board.
**Changes:**
- [ ] `fun/src/tokens.css` + a `theme.ts` (follow-system + manual toggle, the resolved default): a
  distinct games palette + card/felt motifs, structured like croft-pwa's tokens; `docs/DESIGN.md`.
- [ ] Apply to the drawer chrome + solitaire board (card faces, suits, foundations). Guided by the
  `frontend-design` skill; contrast validated (the `dataviz` discipline for any stat tiles / clean-clear
  count).
**Call chain:** chrome + solitaire module consume CSS custom properties; `theme.ts` switches at the root.
**Wiring test:** `theme.spec.ts` — chrome + a rendered board carry the tokens; toggling light/dark
updates computed styles; **axe contrast passes in BOTH themes** (the edges, not one happy-path theme).
**Depends on:** the chrome (done); merges before Phase D's visual polish.
**Read-set:** croft-pwa `tokens.css`/`theme.ts`/`brand.html` (reference), the chrome.
**Write-set:** `fun/src/tokens.css`, `fun/src/theme.ts`, `fun/styles.css`, `fun/docs/DESIGN.md`.
**Shared-state contract:** touches only CSS/theme + `DESIGN.md`; **does not edit `chrome.ts`/`main.ts`
logic** (keeps it disjoint from the Rust track and from Phase C's TS wrapper). No `npm install`.
**Risks:** dark-mode contrast failures (the common miss) — caught by the both-themes axe assertion;
over-designing before the board exists — tokens + chrome first, board specifics land with/after D.
**Done when:** 1) **Behavioral:** the drawer and board wear a distinct, accessible identity in light and
dark. 2) **Verification:** `theme.spec.ts` + axe contrast green in both themes.
**Validation:** **Moderate** — wiring test + axe both themes + a manual look review against `DESIGN.md`.

### Phase S — Klondike solver + winnable-daily-seed pack (build-time)

**Goal:** a **build-time** solver that classifies a seed's deal as winnable, used to generate a
**dated winnable-daily-seed pack** the runtime indexes by date. The runtime never runs the solver;
this mirrors the master-plan level-pack discipline (byte-identical regeneration from a master seed,
P10). Also provides the bounded "is this state hopeless?" check that can confirm a player's `Stuck`.
**Changes:**
- [ ] `crates/solitaire-solver` — a depth-first / best-first Klondike draw-1 solver over `solitaire-core`
  with transposition pruning and a **per-seed time/node budget** (unwinnability can be expensive to
  prove; a seed that exceeds the budget is classified `unknown` and excluded from the pack, not
  guessed). `classify(seed) -> Winnable | Unwinnable | Unknown`. Optionally `winning_line(seed) ->
  Option<Vec<Move>>` (reused to capture the Phase B/D test fixture — resolves open question #2).
- [ ] A build-time generator: run `classify` over a master-seed-derived seed stream, collect the first
  N winnable seeds into `games/solitaire/daily-pack.json` (a versioned `pond-docformat` doc), dated
  by index. Byte-identically regenerable on a clean machine (P10 drill).
- [ ] A runtime **daily selector** (JS/binding): `dailySeed(date) = pack[dayIndex(date) % pack.len]`.
- [ ] (Optional) a bounded `is_hopeless(state)` used to confirm a player-declared `Stuck`.
**Call chain:** build step → `solitaire-solver::classify` over the seed stream → `daily-pack.json`;
runtime → `dailySeed(today)` → `new_game(seed)`.
**Wiring test:** `test_pack_is_all_winnable_and_regenerates` — every seed in a freshly generated pack
`classify`es `Winnable` (and, spot-checked, `winning_line` replays to `is_won`); regenerating the pack
from the master seed is **byte-identical** (the P10 drill in miniature). Plus a solver unit test on a
few known-winnable and a known-unwinnable deal.
**Depends on:** `solitaire-core` (done). Independent of A/B/C (can run parallel with them).
**Read-set:** `crates/solitaire-core/**`. **Write-set:** `crates/solitaire-solver/**`,
`fun/games/solitaire/daily-pack.json`.
**Shared-state contract:** CPU-bound, no network, no shared state beyond output files. (If run parallel
with other Rust phases, it only *adds* a new member crate + a data file — disjoint write-set; the
workspace `members` edit is done once, up front, like the master-plan stub freeze.)
**Risks:** proving unwinnability is the expensive tail — **the budget + `Unknown`-exclusion keeps
generation bounded** and never ships a guessed-winnable daily. Solver determinism doesn't affect the
game hash (it only *selects* seeds), but the pack must regenerate byte-identically or the P10 drill
breaks. This is the heaviest phase — time-box it; if the solver underperforms, ship free-play first and
turn on dailies when the pack lands.
**Done when:** 1) **Behavioral:** a dated winnable-seed pack exists; `dailySeed(today)` yields a deal
the solver proved winnable; the pack regenerates byte-identically. 2) **Verification:**
`cargo test -p solitaire-solver` green incl. the pack-all-winnable + byte-identical-regeneration test.
**Validation:** **Broad** — wiring test + regenerate the pack on a clean checkout and diff (byte-
identical) + spot-check a daily deal is winnable by replaying its `winning_line`.

---

## Open Questions

- **[RESOLVED 2026-07-29]** `pond-outcome` replay mechanism → a small **`Replay` trait** implemented by
  `solitaire-core`, so `pond-outcome` stays game-agnostic (owner: go-with-lean).
- **[RESOLVED 2026-07-29]** Scripted winning deal → **capture via the Phase S solver's `winning_line`**
  and commit as a fixture (owner: agreed; the solver is now in scope, so this is free).
- **[RESOLVED 2026-07-29]** Board-JSON for face-down cards → **`{ faceUp: false }` with rank/suit
  omitted** (the UI cannot see hidden cards). Exact field names pinned in Phase C against the real model.
- **[RESOLVED 2026-07-29]** Undo → **undo/hints ARE in v1**, with a **"Declare assistance used" setting
  (ON by default)**; assistance is self-declared (not replay-derivable); clean-clear = `Won &&
  assistance == Some(false)`. (Supersedes the earlier no-undo lean.)
- **[RECOMMENDED: PHASE-GATED (Phase S)]** Solver **budget + daily-pack size** — the per-seed node/time
  budget (unwinnability is the expensive tail) and how many dated seeds the pack holds (a year? rolling?).
  *Rationale: needed to run generation, not to start the solver. Lean: generous per-seed budget,
  `Unknown` excluded; pack sized to ≥ 1 year of dailies, regenerable.*
- **[RECOMMENDED: PHASE-GATED (Phase D)]** **Daily rollover** — what timezone/boundary defines "today's
  deal" (UTC midnight is simplest and shared globally). *Rationale: affects `dailySeed(date)`. Lean: UTC.*
- **[RECOMMENDED: PHASE-GATED (Phase D)]** **Share-link payload** — encode the full record (bigger URL,
  self-verifying offline) vs just `(seed, result, moves)` and re-derive. *Rationale: lean = full record,
  base64url of the `pond-docformat` JSON, so the recipient verifies with no server.*
- **[RECOMMENDED: ADVISORY]** Run Phase E and Phase S in parallel with A–C (concurrent agents), or fold
  them in sequentially? *Rationale: parallel saves wall-clock; sequential is simpler. Owner call.*

---

## Review Log

- **2026-07-29 — Pass 1+2 (combined).** Authored as the delivery plan for a playable, verifiable
  solitaire, grounded in the shipped code (verified `solitaire-core` API, the chrome contract, the
  `xbuild` raw-C-ABI pattern, the `pond-docformat`/`pond-outcome` stubs, `GameState` lacking
  `Serialize`). Spans master P5/P6 + front P2/P3/P4; supersedes the front-plan's P2/P3/P4 stubs (to be
  annotated) and pulls the master substrate.
  - **Pass 2 gap analysis folded in:** (1) sequenced the outcome **substrate (A→B) before the board UI
    (D)** so the verifiable clean-clear isn't faked/deferred. (2) Chose **raw C-ABI + serde-JSON with
    wasm-holding-state + typed move exports** over wasm-bindgen (proven by `xbuild`, no new toolchain).
    (3) Flagged that `solitaire-core` needs `Serialize` derives (Phase C) and that adding them must keep
    the golden hashes byte-identical. (4) Made the **scripted-winning-deal** problem explicit with a
    capture-as-fixture mitigation (a full Klondike win is hard to hand-author). (5) Kept all
    discovery-repo/plan edits **sequential** (never in the E∥A–C parallel window) — the same isolation
    lesson from the master plan's Pass 3. (6) Every phase has a wiring test through the real entry point
    (the wasm boundary for C, the `/solitaire/` URL for D).
  - **Honesty holds:** board JSON shape, the replay-trait shape, and the winning-deal source are OPEN
    (flagged), not assumed. `pond-outcome.verify` must re-replay, never trust a stored field.
  - **Pending:** Pass 3 quality gates (fresh context) + annotate the front-plan Phases 2/3/4 and the
    master-plan P5/P6 rows to point here.

### Pass 3: Quality Gates — 2026-07-29
**TDD ordering:** every phase is test-first with a wiring test through the real entry point (the crate
API for A/B, the wasm boundary for C, the `/solitaire/` URL for D, both themes for E). No changes.
**Specificity / mutation resistance:** phases name accept-and-reject edges (version minor-vs-major,
tamper, illegal-move-status + board-unchanged, legal-glow-exactly, both-theme contrast). Held.
**Observability / robustness:** added — (C) **the binding must never panic** (a wasm panic aborts the
module); all fallible paths map to status codes / empty-JSON, no `unwrap` on the hot path. (B) `verify`
surfaces expected-vs-actual hash on mismatch, not a bare `false`.
**Debugging readiness:** commit-per-phase + wiring tests as checkpoints; the C boundary hash and B
verify are the instrumented failures.
**Validation calibration:** Broad for C (wasm boundary) and D (browser E2E); Moderate for A/B/E. Held.
No Phase 0 — the unknowns (board schema, replay mechanism, winning-deal source) are phase-gated with
leans and pinned in-phase against the real code, not deferred to a discovery pass.
**Concurrency honesty — the material fix:** the E∥A–C write-set claim was hand-wavy about `chrome.ts`.
Corrected to a per-file check: **E owns `chrome.ts`** (the theme toggle is a real chrome edit) and it is
the *only* parallel phase that touches it (A/B write `crates/**`; C writes `src/games/*` + `build.mjs`,
never `chrome.ts`) — so the sets are genuinely disjoint. Re-entry checks map to the invariants (parent
HEAD unchanged, `git -C discovery` clean, `crates/**` vs styling untouched cross-track).
**Coherence:** solves the stated problem; scope matches; supersession of the front-plan P2/P3/P4 stubs
and the pull of master P5/P6 are now cross-linked in those docs (below). Fixed the `chrome.ts`
ownership wording.
**Documentation impact:** every listed doc has an owning phase; the front-plan/master-plan pointers are
added now (Pass 3) rather than deferred, so the three plans read coherently for the next reader.
**Confirmed ready:** yes. Open-question walk-through pending the owner's call on #1–#3 (all have leans;
none BLOCKING — the plan can start Phase A immediately).

### Gameplay decisions (owner, 2026-07-29) — folded into the phases
Post-Pass-3 product decisions that sharpen the game and add scope. Code-shaping questions #1–#3 resolved
with the agent leans (Replay trait; capture winning-deal as a fixture; face-down cards omit rank/suit).

1. **Both modes; daily deal by default.** A date-derived **daily deal** everyone shares (the follow-chain
   comparison unit) is the default; a **free-play / random** mode sits alongside.
2. **Winnable-filtered dailies ⟹ a Klondike solver, as a BUILD-TIME tool.** The solver runs offline over
   many seeds to generate a **winnable-daily-seed pack** (dated, byte-identically regenerable — the
   master-plan level-pack / P10 discipline); the runtime just indexes the pack by date and never runs
   the solver. New **Phase S**. **Stalemate/stuck is a first-class outcome** (`Stuck`), noted in the
   record — player-declared at runtime, optionally confirmed by a bounded no-progress check.
3. **Assistance is self-declared, default-honest.** Undo/hints exist. Assistance (undo/hint use) **cannot
   be derived from the move list** — the recorded winning sequence is clean by construction — so it is a
   self-declared meta-fact. A **"Declare assistance used" setting, ON by default:** on → the record
   carries the assistance flag; off → assistance is omitted (not claimed clean, just unstated).
   **Clean-clear = `Won && assistance == Some(false)`.**
4. **Verification-forward win screen + a share-results link** — the win screen leads with "Cleared clean
   ✓ verifiable" + one-tap re-verify, and a **share link** encodes the result (seed + outcome record) so
   a recipient can open and re-verify it (the pre-leaderboard social hook; ties to per-game URLs).
5. **Compare metric = moves-to-clear + clean/assisted binary** (count, not ratio).

Scope impact: **new Phase S (solver + winnable-daily pack)**; Phase B's `Record` gains a `result`
enum (`Won | Stuck | Abandoned`) + `assistance: Option<bool>`; Phase C gains an undo state-stack +
assistance tracking + daily/free seed handling + a share-encode/decode; Phase D gains the mode toggle,
undo + the declare-assistance setting, the verify-forward win screen, and the share link.

### Execution reorder + UI/UX research — 2026-07-29
- **Reorder (owner):** do **Phase S before Phase D** so the winnable-daily pack + the **winning-line
  fixture** exist, and D's full **win-path E2E comes online with the board** (rather than free-play
  first). Sequence now: A✅ → B✅ → C✅ → **S → D** (E alongside).
- **UI/UX research (owner asked to look at other implementations):** folded canonical Klondike +
  modern-web conventions into Reasoning → "UI/UX from existing solitaire implementations". Key
  confirmation: modern solitaire's daily-challenge + streak-stats + share-results pattern *is* our
  daily-deal + clean-clear + share-link direction. Sources: Smart Interface Design Patterns
  (drag-and-drop UX), LogRocket & Pencil&Paper (drag/drop patterns), solitaire-play.com (Klondike),
  and reference impls (kimgarpvall/HectorVilas on GitHub).
