//! **meer-queue** — Phase-0 spike for Drystone's blind store-and-forward node.
//!
//! Tests whether a mailbox that does no ordering, holds no group state, and holds no key is
//! sufficient to carry a real MLS conversation across an absence — against real OpenMLS, a
//! real CISS server, and a real iroh transport.
//!
//! - Spec:      `SPIKE-SPEC.md` (M1, M2, S1–S8)
//! - Plan:      `../../plans/2026-08-07-2-plan-meer-queue-spike.md`
//! - Discovery: `PHASE-0-FINDINGS.md`
//! - Bound by:  `beta/impl/delivery-layer/08-experiment-methodology.md` (the fidelity ladder)
//!
//! Stand-ins are tagged `SPEC-DELTA[...]` at their site and enumerated in
//! `../SPEC-DIVERGENCE-REGISTER.md`. Every tag has a row; every row has a tag.

pub mod admission;
pub mod ciss_harness;
pub mod groupinfo_policy;
pub mod meer;
pub mod mls;
pub mod node;
pub mod outer_seal;
pub mod queue;
pub mod relay;
pub mod transport;

/// Initialise stderr tracing for a probe or test run, honouring `RUST_LOG`.
///
/// Idempotent: repeated calls are ignored, so every test may call it without racing. Verdict
/// lines go to **stdout** and never through this subscriber, so a result can never be lost to
/// a log-level setting.
pub fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .try_init();
}
