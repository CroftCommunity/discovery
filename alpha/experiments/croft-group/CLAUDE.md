# Croft Group — Project Instructions

This workspace follows the standards and discipline defined in the global
coding-agents set. The relevant files (home-relative for portability —
`~/.claude/coding-agents` symlinks to the canonical checkout):

@~/.claude/coding-agents/CLAUDE.md
@~/.claude/coding-agents/agents.md
@~/.claude/coding-agents/tdd-guardian.md
@~/.claude/coding-agents/rust-enforcer.md

## Project Context

**croft-group** is the Croft group/messaging pond, brought onto the settled
Croft client architecture (`discovery/thinking/app/client-architecture-adr.md`,
option C: per-pond domain cores unified by a shared shell, with two callout
axes — platform `effects.rs` and implementation adapters behind a port).

Two crates:

- `crates/group-core` — the pure, WASM-clean domain core (`model` / `intent` /
  `effect` / `update` / `wire` / `project` / `view`). Mirrors the Bluesky-feed
  pond's `app-core` (`experiments/croft-app-phase0/`). Depends on nothing
  transport-related.
- `crates/croft-chat-cli` — the thin CLI shell. Holds the `Transport` port and
  its deterministic `InProcBus` fake; performs the core's effects; runs the
  pump (drain → `FrameReceived` → process). **DECISION 1:** the shell holds the
  port, the core never touches it.

Current scope: the messaging **happy-path** (join → send → receive → render
over the in-proc bus). MLS/identity, fork/merge + reconvergence-per-plane,
governance, the real-iroh adapter, and shared-shell composition are sequenced
as later phases (L1–L6), not built here.

## Discipline (project-specific reinforcement of the coding-agents standards)

1. **TDD is non-negotiable.** Every line of production code in any
   `crates/*/src/` directory must be written in response to a failing test.
   RED → VERIFY RED → GREEN → VERIFY GREEN → MUTATE → REFACTOR. See
   `tdd-guardian.md` for the full cycle and the mental mutation pass.

2. **The Rust enforcer applies.** See `rust-enforcer.md`. Key points:
   - `Result<T, E>` for fallible operations; `thiserror` for error types.
   - **No `unwrap()`/`expect()` outside `#[cfg(test)]`.**
   - Doc comments on every public item (`#![warn(missing_docs)]`).
   - `clippy::pedantic` clean; `cargo fmt --check` clean.
   - **`Zeroize` for any secret material.** None exists in the happy-path
     slice; it arrives with MLS/encryption (L2).

3. **No category of production code is exempt from TDD.** Type definitions,
   error enums, the wire format, the effect/intent enums — all driven by a
   failing test first. "This is just data" is the signal to stop and write
   the test.

4. **The core stays pure.** `group-core` does no I/O. It emits effect *data*
   (including diagnostics like `FrameDropped`); the shell performs the I/O.
   This keeps the core WASM-clean for a future web/desktop shell.

5. **Hostile input is survived, not trusted.** A malformed inbound frame is
   the corrupt/hostile-sender case — drop it observably (emit `FrameDropped`),
   never panic, never fatally propagate. This is distinct from fail-loud,
   which governs *our own* bugs.

6. **Wait for commit approval** before every commit. The user makes commits,
   not Claude.

## Build configuration is pragmatically exempt from TDD

Cargo manifests and `Cargo.lock` are build configuration — needed to even run
a failing test — and are not themselves TDD-gated. `lib.rs` re-export wiring is
module plumbing, not logic.

## When in doubt

- TDD violations: stop, write the failing test, watch it fail, then proceed.
- Architecture questions: consult the client-architecture ADR; if not covered,
  escalate before deciding.

The coding-agents standards (imported above) are authoritative. This file
extends them with project-specific reinforcement, not exceptions.
