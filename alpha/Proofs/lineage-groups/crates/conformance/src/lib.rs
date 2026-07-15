//! `conformance` — Croft conformance vectors + runner (Workstream D).
//!
//! Two halves:
//! * [`model`] — the language-neutral JSON vector schemas (hex-encoded bytes).
//! * [`runner`] — re-proves each vector against the *real* `lineage-core` /
//!   `lineage-history` public API and reports per-vector PASS/FAIL.
//!
//! The vector *values* are never hand-written. The `emit-vectors` binary derives
//! every value by running the real implementation; the `run-vectors` binary
//! feeds each recorded input back through that same API and diffs the output.
//! Re-proving the freshly emitted vectors is the green check.
//!
//! Cardinal rule honored here: no cryptographic constant is typed by hand — the
//! emitter computes signatures, hashes, and keys from the real code, and the
//! runner recomputes them rather than trusting the recorded value.

#![warn(missing_docs)]

pub mod model;
pub mod runner;

pub use runner::CheckOutcome;
