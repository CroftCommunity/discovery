# item-store

**Placeholder name** (the co-op + service are formally unnamed — ROADMAP_TODO A21;
the real name is a deliberate pass at Phase 9). `item-store` names the *noun* (items),
not the capability ("metered"), and ties to the `item-storage-protocol` it ports.

The cooperative **metered-storage service**: a network-accessible, custom PDS-like
store in Rust with metering built in (the E0–E9 ledger) and an S3-compatible interface,
destined for VPS deployment via croft-stack. It doubles as the substrate for the MLS
history-convergence server (one store, two consumers).

## What this is a port of

This crate ports the proven, dependency-free TypeScript oracle at
`../item-storage-protocol-standalone/` (which runs **81/81** assertions across E0–E11)
module-by-module under TDD, then closes the `SEAM:`s (network boundary, real blob
backend, real CIDs). The oracle's assertions are the port spec; `SEAM:` comments mark
every place a mock stands in for real infrastructure, so production gaps stay
enumerable by grep.

The two-layer design: a **dumb pluggable backend** (Layer 1: FS → Garage/SeaweedFS/R2,
behind a `BlobStore` trait) under a **boundary metering/provenance layer** (Layer 2: the
E0–E9 ledger — signed receipts for postage, a customer-signed manifest for rent,
balance-forward statements, audit, seal). The backend never meters.

## Build plan

`../../plans/2026-07-31-1-plan-coop-metered-storage-service.md` (phase-plan, three-pass +
Phase 0 discovery complete). Phases: 1 crypto/identity (E0) · 2 items+manifest (E1–E2) ·
3 receipts (E3) · 4 statements (E4) · 5 audit+dial (E5–E6) · 6 seal+grace (E7–E9) ·
7 S3 metered boundary · 8 atproto PDS blob surface · 9 croft-stack deploy · 10 convergence
(gated).

## Status

- **Phase 1 (E0 — identity): DONE.** Deterministic Ed25519 keypair derivation, stable
  id derivation (`id:` + SHA-256(pubkey)[..16]), sign/verify, peer pinning.
- **Phase 2 (E1–E2 — items + manifest): DONE.** Content-addressed `Item`/`ContentStore`
  (tamper-evident retrieval, dedup) + a customer-signed canonical Merkle manifest
  (order-independent root, tamper/omission detection).
- **Phase 3 (E3 — receipts + ledger): DONE.** Two-mode transfer receipts
  (`Bilateral` co-signed | `Unilateral` provider-signed) + an append-only,
  hash-linked, signed ledger + deterministic canonical serialization.
- **Test hardening (E86): E0–E3 mutation gate green** (`cargo-mutants`: 103 caught /
  0 missed; trivial accessors excluded via `.cargo/mutants.toml`).

## Develop

Standalone crate (no experiments-wide Rust workspace). From this directory:

```sh
cargo test                       # full suite
cargo test --test e0_identity    # the Phase 1 wiring test (E0)
cargo clippy --all-targets -- -W clippy::pedantic -D warnings
cargo fmt --check
```
