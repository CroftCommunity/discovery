//! Enrollment: prove DID-control of an endpoint, then bind it.
//!
//! The check is one-directional and cheap: to enroll `endpoint` under `did`,
//! the DID's PDS repo must already publish `endpoint` in its
//! `ing.croft.iroh.endpoint`/`self` record. That proves the enrolling party
//! controls the DID's repo (only the DID owner can write that record) *and*
//! that they intend this endpoint. We do not phone the endpoint; the relay's
//! cryptographic attach handles proof-of-key-possession later.
//!
//! Deny-closed is the invariant: every non-affirmative PDS outcome
//! (`NotFound`, `Timeout`, `Malformed`) and every id mismatch returns an error
//! and writes nothing. Only an exact endpoint match binds. See ADR-0002.

use crate::did::Did;
use crate::endpoint_id::EndpointId;
use crate::pds::{PdsError, PdsResolver};
use crate::registry::Registry;

#[derive(Debug, PartialEq, Eq)]
pub enum EnrollError {
    /// The PDS record exists but names a different endpoint than the one being
    /// enrolled. Hard deny — never bind on a mismatch.
    EndpointMismatch,
    /// The PDS could not affirm the binding (not found / timeout / malformed).
    /// Deny-closed: absence of a "yes" is a "no".
    PdsUnavailable(PdsError),
}

/// Verify DID-control of `endpoint` via the PDS, and on success record the
/// binding in `registry`. Returns `Ok(())` only when the PDS record for `did`
/// names exactly `endpoint`.
pub fn verify_and_bind(
    registry: &Registry,
    resolver: &dyn PdsResolver,
    did: &Did,
    endpoint: EndpointId,
) -> Result<(), EnrollError> {
    let record = resolver
        .fetch_endpoint_record(did)
        .map_err(EnrollError::PdsUnavailable)?;

    if record.endpoint_id != endpoint {
        return Err(EnrollError::EndpointMismatch);
    }

    registry.bind(endpoint, did.clone());
    Ok(())
}
