//! lexicon-community — RUN-LEX-01: a clean-room, second-implementation spike
//! against the published CID-First Attestation Specification (badge.blue) and
//! the lexicon.community calendar/RSVP/location lexicons.
//!
//! The contribution surface, smallest first:
//!   1. an independent verifier built from the spec text alone, with an
//!      adversarial fixture corpus (`attest`, `cidfirst`, `didkey`);
//!   2. the freshness/status layer the spec omits — holder-stapled inclusion
//!      proofs, zero verifier callback (`staple`);
//!   3. lossless live-network consumption of real records (`consume`).
//!
//! Everything is single-author records (the thread's stated scope line). The
//! crate makes no network calls; live fetches are caller-supplied behind a flag.
//! Ambiguities hit during the clean-room build are logged in `AMBIGUITIES.md` —
//! that table IS the spec-feedback post.

pub mod attest;
pub mod cidfirst;
pub mod consume;
pub mod didkey;
pub mod ipld_json;
pub mod schema;
pub mod sign;
pub mod staple;
