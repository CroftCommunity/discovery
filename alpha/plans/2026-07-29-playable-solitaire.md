# Playable, verifiable solitaire in the drawer — the delivery slice

**Status:** Pass 1+2 (combined). Pass 3 pending. Planning only — no code written yet.
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
- **`discovery/alpha/ROADMAP_TODO.md` E46** — breadcrumb when solitaire goes playable. Phase D.

---

## Concurrency Map

```
Sequential spine:  Phase A (pond-docformat) → Phase B (pond-outcome) → Phase C (binding) → Phase D (board UI)
Parallel:          Phase E (design system) runs alongside A–C; it must MERGE before Phase D's visual polish.
```

**Parallel {Phase E ∥ {A→B→C}}:**
- **Disjoint write-sets:** Phase E writes only `fun/src/tokens.css`, `fun/styles.css`,
  `fun/docs/DESIGN.md`, and theme wiring in `fun/src/` chrome files it already owns; Phases A/B/C write
  `fun/crates/**` (+ Phase C touches `fun/src/games/…` binding wrapper, not the chrome styles). No file
  overlap **provided** Phase E does not edit `chrome.ts`/`main.ts` logic (styling via CSS + a `theme.ts`
  only). If E needs a chrome DOM change, it serializes before/after C — flagged.
- **Shared-state contract (invariants):** if run as concurrent agents, both in worktrees off the `fun`
  feature branch; neither invokes `git checkout`/`stash`/`rebase` in the parent worktree; neither edits
  the **discovery repo** (all plan-doc/roadmap edits are sequential, done by the orchestrator); Phase E
  touches no `crates/**`, the Rust phases touch no `styles.css`/`tokens.css`/`DESIGN.md`; each builds to
  its own `target/` / `node_modules` is shared read-only (no `npm install` in E — deps already present).
- **Re-entry verification:** parent HEAD == pre-dispatch SHA; `git -C discovery status` clean;
  `crates/**` untouched by E and `styles.css`/`tokens.css` untouched by A/B/C; `npm test` + `cargo test
  --workspace` green on the merged branch; no orphan `cargo`/`node` processes.

Everything else sequential: B needs A (envelope); C needs solitaire-core (done) + B (outcome export);
D needs C (binding) + B (record) + the chrome (done). Design (E) is the only independent track. Default
is to run **sequential A→B→C→D→E** unless the owner wants E parallelized.

---

## Phases

### Phase A — `pond-docformat` (master P5): the versioned document envelope

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

### Phase B — `pond-outcome` (master P6): the verifiable outcome record

**Goal:** a self-checking outcome record: given `(kind, seed, move list)`, replay via the game core and
emit `{ kind, seed, result, final_hash, move_count }` that anyone re-verifies by replaying. Local only.
**Changes:**
- [ ] `pond-outcome`: `attest(kind, seed, moves) -> Record` (replays through the core, records
  `final_hash` = `state_hash`), and `verify(&Record) -> bool` (re-replays and re-hashes — **never**
  trusts a stored field). Serialized via `pond-docformat` (Phase A).
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

### Phase C — `solitaire-wasm`: the browser binding (front P3, raw C-ABI + serde-JSON)

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
  (0 applied / 1 illegal / 2 bad-arg); `is_won() -> u32`; `outcome_json() -> ptr`/`len` (via
  `pond-outcome` over the tracked move list).
- [ ] A TS wrapper `fun/src/games/solitaire-wasm.ts` that loads the `.wasm`, decodes the `ptr`/`len`
  strings, and presents a typed API (`newGame`, `board()`, `legalMoves()`, `play(move)`, `isWon()`,
  `outcome()`). `build.mjs` builds + places the `.wasm` (documented; served with the existing `.wasm`
  MIME).
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
- [ ] Win: on `isWon()`, show a celebration + the `outcome()` record and a "verify" affordance that
  re-checks it (calls `verify`). Clean-clear = no undo used (undo is a later nicety; P4 can ship without
  undo, so every clear is clean — record that simplification).
- [ ] `registry.ts`: flip solitaire to `status: "playable"` with `load: solitaireModule`.
- [ ] Seed choice: a `new game` control seeds from a UI source (e.g. a counter or date-derived; **not**
  `Math.random` in the hashed path — the seed is fine to be arbitrary, but record how it's chosen).
**Call chain:** `/solitaire/` → chrome mounts `solitaireModule` → `newGame(seed)` → render → tap →
`play_*` → re-render → win → `outcome()` shown + verified.
**Wiring test:** `solitaire.spec.ts` (Playwright E2E) — load `/solitaire/`, assert the dealt board
renders (7 piles, correct counts); play a **scripted winning deal** (a fixed seed with a known winning
move sequence, from the Phase B fixture) via taps; assert the win state **and** a verifiable outcome
record appears and re-verifies. Name the edges: an illegal tap is rejected (board unchanged); tapping a
card glows exactly the legal targets; stock cycling works. Repeat in-drawer and full-screen. RED before
the UI, GREEN after.
**Depends on:** Phase C (binding), Phase B (`pond-outcome`), the chrome (done), Phase E (tokens, for a
non-neutral board — or ship on the baseline and restyle when E merges).
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

---

## Open Questions

- **[RECOMMENDED: PHASE-GATED (Phase B)]** How does `pond-outcome` obtain a game to replay — a generic
  `Replay` trait implemented by `solitaire-core`, or a feature-gated dependency? *Rationale: shapes the
  crate's dependency direction; not needed to start Phase A.* Agent lean: a small `Replay` trait so
  `pond-outcome` stays game-agnostic.
- **[RECOMMENDED: PHASE-GATED (Phase B/D)]** Source of the **scripted winning deal** used by the
  outcome wiring test + the board E2E — capture one via an `#[ignore]` greedy/solver helper and commit
  it as a fixture, or assert on a partial (non-win) attested record if a full win is impractical to
  script. *Rationale: a hand-authored winning Klondike line is hard; the capture approach mirrors the
  golden-vector recorder.*
- **[RECOMMENDED: PHASE-GATED (Phase C)]** Exact **board-JSON shape** (field names, how face-down cards
  are represented to the UI — hidden rank or a `null`?). *Rationale: pin against the real `GameState`
  in Phase C; the UI (D) targets it.* Agent lean: expose face-down cards as `{ faceUp: false }` with the
  rank/suit **omitted** (the UI must not see hidden cards, matching the game).
- **[RECOMMENDED: ADVISORY]** Undo in P4 — recommend **no undo in the first playable** (so every clear
  is trivially "clean"), adding undo + the assistance flag later. *Rationale: keeps the clean-clear
  definition simple for v1; decide at Phase D.*
- **[RECOMMENDED: ADVISORY]** Run Phase E in parallel with A–C, or sequentially after D? *Rationale:
  parallel saves wall-clock but the board (D) restyles once E lands; sequential is simpler. Owner call.*

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
