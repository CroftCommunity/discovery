//! ap-ambassador — RUN-AP-01: the AP-ambassador receipt lane.
//!
//! The ambassador ingests inbound ActivityPub activities (Follow / Undo /
//! Delete), verifies them at the HTTP-signature layer, mints an
//! evidence-complete receipt over the canonical dag-cbor envelope path (reused
//! path-dep style from `attest-family`), and derives a follower-interval
//! roster by folding Follow/Undo pairs.
//!
//! **Governing principle:** the ambassador respects the customs of the
//! protocol federated with. Delivery-plane role (A.7 sense) — no ordering
//! authority, no membership authority, no governance conductivity.
//!
//! See `AP-AMBASSADOR.md` for the charter and the five settled verdicts
//! (AP-V1..V5). See `FINDINGS-AP.md` for the run's findings.

pub mod binding;
pub mod boundary;
pub mod canonical;
pub mod fixtures;
pub mod fold;
pub mod records;
pub mod redact;
pub mod store;
pub mod types;
pub mod verify;
