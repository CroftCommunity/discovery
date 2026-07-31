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
//! Current span: **E0–E3** — identity (E0: keypairs, id derivation, sign/verify),
//! content-addressed items + a customer-signed Merkle manifest (E1–E2), and
//! transfer receipts + an append-only signed ledger with canonical serialization
//! (E3: two-mode postage metering, Bilateral | Unilateral). The E0–E3 mutation
//! gate is green (see `ROADMAP_TODO` E86). Phases 4–9 (statements, audit, seal,
//! the S3/atproto boundary, deploy) follow the build plan.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod canonical;
pub mod clock;
pub mod crypto;
pub mod identity;
pub mod item;
pub mod ledger;
pub mod manifest;
pub mod persist;
pub mod pricing;
pub mod receipts;
pub mod statements;
