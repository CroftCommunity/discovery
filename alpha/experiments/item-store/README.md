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
- **Phase 4 (E4 — statements + persistence): DONE.** Balance-forward statement
  chain (byte-day rent, hash-linked close, rollup/purge) + per-user SQLite
  persistence co-locating manifest/receipts/statements (`rusqlite`; `:memory:`
  mode in tests, file-backed in production).
- **Phase 5 (E5–E6 — audit + dial): DONE.** Random k-sample spot-check audit
  (`audit.rs`) with the closed-form detection math `1 - (1 - f)^k`, over a seeded
  deterministic RNG (`rng.rs`, mulberry32); the member-chosen assurance dial
  (`dial.rs`) priced at cost, linear in audit count, recorded as a signed ledger
  declaration; audit pricing added to `pricing.rs`.
- **Phase 6 (E7–E9 — seal + tombstone + grace): DONE — completes the E0–E9
  ledger core.** `seal.rs`: a signed seal declaration pinning a root, a write-path
  ceremony that destroys the credential so writes fail closed (loud typed error),
  and a rotation watch classifying root changes as customer-initiated or an alarm;
  the tombstone tier additionally destroys the unseal capability. `grace.rs`:
  co-signed grace events (fee waiver, deceased-member hold, throttle) as
  forward-only ledger entries that net to zero against the co-op grace account.
- **Test hardening (E86): E0–E9 mutation gate green.** The full-crate
  `cargo-mutants` run surfaced 11 survivors, all in `rng::next_f64`'s bit-mixing
  (uniformity/determinism properties cannot pin the exact algorithm); killed with
  a golden-vector parity test locking the mulberry32 sequence to the TS oracle
  (scoped re-run: 31/31 `rng` mutants caught). Trivial accessors + equivalent
  mutants excluded via `.cargo/mutants.toml`; zero real survivors across E0–E6.

## Develop

Standalone crate (no experiments-wide Rust workspace). From this directory:

```sh
cargo test                       # full suite
cargo test --test e0_identity    # the Phase 1 wiring test (E0)
cargo test --test e5_audit       # the Phase 5 audit wiring test (E5)
cargo test --test e6_dial        # the Phase 5 dial wiring test (E6)
cargo test --test e7_seal        # the Phase 6 seal wiring test (E7)
cargo test --test e8_tombstone   # the Phase 6 tombstone wiring test (E8)
cargo test --test e9_grace       # the Phase 6 grace wiring test (E9)
cargo clippy --all-targets -- -W clippy::pedantic -D warnings
cargo fmt --check
```
