//! `item-store` — the cooperative metered-storage service, a Rust port of the
//! proven `item-storage-protocol` (the E0–E9 ledger) toward a network-accessible,
//! custom PDS-like store with metering built in.
//!
//! This crate ports the dependency-free TypeScript oracle
//! (`experiments/item-storage-protocol-standalone/`, which runs 81/81 across
//! E0–E11) module-by-module under TDD, then closes the `SEAM:`s (network
//! boundary, real blob backend, real CIDs). The build plan is
//! `discovery/alpha/plans/2026-07-31-1-plan-coop-metered-storage-service.md`.
//!
//! Phase 1 (this slice) ports **E0 — identity**: both parties exist as keys and
//! nothing more; recognition (signature verification) and counting (identifier
//! derivation) rest on the same public key.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod crypto;
pub mod identity;
