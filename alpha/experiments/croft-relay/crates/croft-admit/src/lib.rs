//! croft-admit — admission authority for an atproto-gated iroh relay.
//!
//! This crate is the *app-side* enforcement core the relay leans on. It is
//! deliberately **relay-agnostic** (no `iroh`/`iroh-relay` dependency): the
//! same logic serves the Phase 1 HTTP-hook contract (`access`) and the Phase 2
//! embedded verifier (`token`), and the tier->bucket mapping (`tier`) is a pure
//! function the embedding layer applies. Keeping it relay-free is also what
//! makes the token verifier and the rate-limit-override idea clean upstream
//! candidates (plan Phase 5): no atproto or tier concepts leak into anything
//! iroh would carry.
//!
//! Module map, mirroring the plan phases:
//!   - `endpoint_id`, `did` — the two identities and their wire forms.
//!   - `pds`, `registry`, `enroll`, `access` — Phase 1: DID-bound enrollment
//!     and the deny-closed HTTP access check.
//!   - `token` — Phase 2: signed per-endpoint capability tokens.
//!   - `tier` — Phase 3 (core): tier claim -> per-connection rate bucket.
//!
//! What is NOT here, and why: standing up a real iroh relay, holepunch
//! integration, bucket calibration, mutation/fuzz/load hardening. Those need a
//! live relay this environment cannot build (github clone blocked) or a
//! multi-process harness it cannot run. They are the seams left for the next
//! session; see the experiment README and ADRs.

pub mod access;
pub mod did;
pub mod endpoint_id;
pub mod enroll;
pub mod http_api;
pub mod pds;
pub mod registry;
pub mod tier;
pub mod token;

pub use access::{decide, AccessDecision};
pub use did::Did;
pub use endpoint_id::EndpointId;
pub use enroll::{verify_and_bind, EnrollError};
pub use http_api::{access_router, IROH_ENDPOINT_ID_HEADER};
pub use pds::{EndpointRecord, PdsError, PdsResolver};
pub use registry::Registry;
pub use tier::{bucket_for, RateBucket, Tier};
pub use token::{Claims, TokenError, TokenIssuer, TokenVerifier, VerifiedClaims};
