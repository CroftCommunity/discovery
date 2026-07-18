//! attest-family — RUN-ATTEST-01: the attestation family, proved in code.
//!
//! One attestation family, two axes:
//! - **subject type**: persona, or thing (business, product, work);
//! - **consent mode**: `mutual` (co-signed edge), `unilateral_notice`
//!   (subject notified, signed reply allowed, no countersign required),
//!   `unilateral_private` (note to self — vocabulary only in this run, OC-4).
//!
//! Shared machinery: scope labels, supersede-never-revoke, per-viewer
//! resolvability, corroboration structure, provenance grades. **No trust
//! score exists anywhere** — a tested impossibility (T-AT0.2, T-AT3.1), not a
//! non-goal. Queries return corroboration structure, viewer-relative; clients
//! do the weighting. The protocol computes provenance (consistent +
//! corroborated); it never computes utility (true/right) — that is left to
//! humans at the edges.
//!
//! Companion primitive language: `PRIMITIVES-ATTEST.md` (next to this crate).
//! Findings ledger: `FINDINGS.md`.

// Inherent `from_str(&str) -> Option<Self>` constructors on the closed enums
// deliberately shadow the trait-method name (they are total over the closed
// vocabulary and fallible by Option, not Err).
#![allow(clippy::should_implement_trait)]

pub mod canonical;
pub mod fixtures;
pub mod fold;
pub mod query;
pub mod types;
