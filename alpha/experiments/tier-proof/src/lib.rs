//! # tier-proof — RUN-17: the group model, demonstrated end to end.
//!
//! This crate BUILDS AND RUNS the RUN-16 model (the specification under test):
//! the open tier's one-signature self-registration, the gated tier's two-sided
//! membership facts with causal revocation, device-key delegation, the sealed
//! steward group, catalogue reconstructability, the write-policy axis, the
//! blinded roster, the delivery-roles rehearsal, and a measured scale rehearsal.
//!
//! Every part is TDD red-first: acceptance criteria land as failing tests before
//! implementation; predictions about live wire behaviour are constants in the
//! test files before the first live call.
//!
//! ## Grades (honesty contract)
//! - **component** — pure logic, no network (P1, P3, P7, P8, P9a).
//! - **live** — real bsky.social + Jetstream (P2, P5; **BLOCKED** here without
//!   credentials — the multi-party half runs behind the SAME resolver interface
//!   against locally generated keypairs, tagged `SPEC-DELTA[run17-… | stand-in]`).
//! - **loopback** — croft-group MLS harness over an in-proc transport (P6, P9b).
//!
//! The module map mirrors the RUN-16 model sections it proves; see
//! `alpha/experiments/RUN-17-SUMMARY.md` for the per-part claim mapping.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod canonical;
pub mod envelope;
pub mod identity;
pub mod records;

pub mod blind;
pub mod delegation;
pub mod fold;
pub mod relay;
pub mod roles;
pub mod scale;
pub mod source;
