# match3-p1 — the candy-crush match-3 P1 determinism foundation

A code-forward spike implementing **Phase 1 (P1)** of the match-3 (candy-crush-style) build guide, built to
the per-pond build discipline in `beta/croft/build-order-and-ponds-roadmap.md` ("Per-pond build discipline")
and the P1 run brief in
`alpha/seeds/transcripts/raw/croft-games-pond-roadmap-browser-p2p-phased-build-2026-07-22.md`.

P1 is **not** "write the game." It is the **determinism foundation**: a headless, deterministic core whose
outcomes are **verifiable by replay against a state hash**. The rules and the golden vectors come first; the
engine is grown red-first against them.

## What's here

- **`RULES.md`** — the rules document + the fully-specified tie-break tables (T1–T4). The first deliverable.
- **`vectors/`** — the golden-vector corpus (hand-authored inputs + hand-computed step-0 expectations;
  recorded final state-hash anchors). Schema in `vectors/README.md`.
- **`crates/match3-core/`** — the deterministic engine:
  - `board.rs` — cell/board model + char-grid parse/print.
  - `rng.rs` — seeded ChaCha20 refill stream (the `alpha/Proofs/lineage-groups` determinism primitive).
  - `engine.rs` — `find_matches` (T1), `clear_cells` (T2), `apply_gravity` (T3), `refill` (T4),
    `swap_legal`, and the `Game` cascade loop.
  - `hash.rs` — the canonical `state_hash`.
  - `tests/tie_breaks.rs` — RNG-free unit tests, one per tie-break rule (the red-first driver).
  - `tests/golden_vectors.rs` — corpus replay: step-0 expectations, replay determinism, locked-hash
    regression.

## P1 decisions (as decided by the owner)

| Decision | Choice |
|---|---|
| Language | **Rust → wasm** (free native+wasm cross-build determinism test; matches existing Cargo workspaces) |
| Specials in v1 | **None** — plain match-3 |
| Representative blocker | **One layered blocker** tile (`Blocker(layers)`) |

Deliberately deferred as owner balance decisions (surfaced, not resolved): cascade score multipliers, par
bands, the special-tile set. Scoring is flat in P1 (`+10`/gem, `+20`/blocker layer) so no balance call is
smuggled in.

## Run it

```sh
cargo test          # 16 tie-break unit tests + 3 golden-vector tests
cargo fmt --check
cargo clippy --all-targets
```

To re-record the locked hashes after an intended rules change:

```sh
cargo test --test golden_vectors print_final_hashes -- --ignored --nocapture
```

## Not in P1 (the not-yet set)

Special tiles; cascade multipliers / par bands; level generation (P4); saves + share codes and their
compatibility-matrix sustainment (P10); the P2 version-and-unknown-field document policy; anything
network / iroh / resolver. The natural next steps are the **native+wasm cross-build determinism test**
(the reason Rust was chosen — build `match3-core` to wasm and assert the corpus hashes match native) and a
throwaway feel-spike, both named in the run brief.
