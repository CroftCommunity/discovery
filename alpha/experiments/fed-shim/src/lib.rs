//! fed-shim — the fediverse-wire conformance shim.
//!
//! Read `FED-SHIM.md` before extending this crate. The §0 governing
//! principle is load-bearing: this is a wire-conformance surface, not a
//! Mastodon replica. Every wire behavior modeled here is specimen-anchored
//! (`tests/specimens/`) or spec-cited; anything not specimen-anchored is
//! FIRM non-goal territory (§3).

pub mod actor;
pub mod activities;
pub mod inbox;
pub mod sig;
