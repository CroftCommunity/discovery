//! `seal-wasm` — RUN-19 P1: the MLS seal stack inside a wasm32 module.
//!
//! The claim under test: the croft-group L2a seal stack (`group-seal` →
//! `lineage-mls` → openmls 0.8.1 + `openmls_rust_crypto`, the pure-Rust
//! provider) not only *compiles* to `wasm32-unknown-unknown` (which upstream
//! CI builds) but *runs* real group traffic there (which upstream CI does not
//! test). The tests in `tests/p1_seal_in_wasm.rs` execute group create, add,
//! seal/unseal, epoch roll, and removed-member forward-blindness INSIDE the
//! wasm module under the wasm-bindgen Node runner
//! (`SPEC-DELTA[run19-node-runner]`).
//!
//! GREEN: [`seal`] re-exports the real croft-group stack (no
//! re-implementation); the RED state proved the harness fails without it.

#![warn(missing_docs)]

pub mod seal;

#[cfg(target_arch = "wasm32")]
pub mod js;

#[cfg(target_arch = "wasm32")]
pub mod js_persist;
