//! `lineage-core` — pure logic for the lineage-groups validation.
//!
//! Phase 0 delivers the deterministic substrate every later phase relies on:
//! a logical clock, a seeded RNG, and the shared identity/provenance types.
//!
//! Phase 2 (the Phase 1 gate is GO) adds the data model: a signed governance
//! op log with immutable genesis-rule evaluation ([`gov`]), the lineage DAG
//! with standing queries ([`dag`]), deterministic survivor selection
//! ([`survivor`]), the conflict detector with human escalation ([`conflict`]),
//! and Ed25519 governance signing ([`keys`]).

pub mod clock;
pub mod conflict;
pub mod dag;
pub mod gov;
pub mod ids;
pub mod keys;
pub mod merkle;
pub mod rng;
pub mod survivor;

pub use clock::{Lamport, LamportClock};
pub use conflict::{
    detect, quorum_override, reconcile, ConflictReason, Decision, Escalator, OverrideOutcome,
    Resolution,
};
pub use dag::Lineage;
pub use gov::{
    detect_equivocation, sign_op, Directory, Equivocation, Genesis, GenesisRules, GroupState,
    OpBody, OpKind, RejectReason, SignedOp,
};
pub use ids::{group_genesis, group_topic, lineage_genesis, DeviceId, Did, GenesisId};
pub use keys::{Sig, SigningIdentity, VerifyingIdentity};
pub use rng::DetRng;
pub use survivor::{select_survivor, BranchSummary, SurvivorRule};
