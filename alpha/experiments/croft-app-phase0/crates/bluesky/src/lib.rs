//! The Bluesky module: native types, the port trait the shell consumes, and a
//! fixture-backed fake. The real adapter (Jacquard) is M6.
//!
//! Note the asymmetry that makes the architecture hold: `types` carries no I/O
//! and no async, so the core can depend on this crate for the post shape alone
//! (DECISION 1, 2). The `port` trait is async and is consumed by the shell, not
//! the core.

pub mod pins;
pub mod types;

// The port is async (the shell implements it and does I/O). Only the shell
// depends on it. Gating it keeps the default build (what the core sees) to the
// pure data types.
pub mod port;

// Shared wire parsing for any protocol-speaking backend (fake or real).
#[cfg(feature = "serde")]
pub mod wire;

#[cfg(feature = "fake")]
pub mod fake;

// The real, network-backed adapter (M6).
#[cfg(feature = "adapter")]
pub mod adapter;
