//! Reading the `ing.croft.iroh.endpoint` record from a DID's PDS repo.
//!
//! The lexicon record (rkey `self`) binds a DID to an iroh `EndpointId` and a
//! home relay URL. Enrollment fetches it to prove the enrolling party actually
//! controls that DID's repo and has published the endpoint we are being asked
//! to admit.
//!
//! The trait is synchronous on purpose: the core admission logic stays
//! deterministic and dependency-light (no async runtime, no HTTP client in the
//! test path). The production adapter is a thin async wrapper that performs the
//! real `com.atproto.repo.getRecord` XRPC call and hands the parsed record (or
//! a timeout error) to this same logic. Deny-closed on any error is enforced by
//! `enroll`, not here — this module only *reports* what the PDS said.

use crate::endpoint_id::EndpointId;

/// The decoded `ing.croft.iroh.endpoint` / rkey `self` record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EndpointRecord {
    /// The endpoint the DID's owner published as theirs.
    pub endpoint_id: EndpointId,
    /// Home relay URL the record advertises (e.g. `https://relay.croft.ing`).
    /// Carried through for completeness; admission does not gate on it in v1.
    pub home_relay: String,
}

/// Why a PDS lookup did not yield a usable record.
#[derive(Debug, PartialEq, Eq)]
pub enum PdsError {
    /// DID has no `ing.croft.iroh.endpoint`/`self` record.
    NotFound,
    /// Network/PDS timeout or transport failure. Treated as deny by `enroll`.
    Timeout,
    /// Record present but not decodable to the lexicon shape.
    Malformed,
}

/// Fetch a DID's published endpoint record. Implemented by the real PDS client
/// in production and by an in-memory fixture in tests.
pub trait PdsResolver {
    fn fetch_endpoint_record(&self, did: &crate::did::Did) -> Result<EndpointRecord, PdsError>;
}
