//! Phase 1 access-check: the relay's HTTP-hook contract.
//!
//! iroh-relay's `access.http` mode POSTs to a configured URL for every incoming
//! connection, passing the authenticated endpoint id in `X-Iroh-Endpoint-Id`
//! (hex). It grants attach only on HTTP 200 with body `true`. This module is
//! that endpoint: parse the header, ask the registry, answer.
//!
//! Every failure path denies. A missing header, a malformed header, an
//! unenrolled endpoint — all `deny`. There is no code path that admits on
//! uncertainty. That is the deny-closed rule the plan (Phase 1) and ADR-0002
//! require, and it is why the decision is a pure function with an exhaustive
//! match rather than a chain of early `true` returns.

use crate::endpoint_id::EndpointId;
use crate::registry::Registry;

/// The admission verdict. `allow` maps to HTTP 200 + `true`; anything else to a
/// deny response the relay reads as "reject this attach".
#[derive(Debug, PartialEq, Eq)]
pub enum AccessDecision {
    Allow,
    /// Header absent or not valid hex / wrong length.
    DenyMalformedId,
    /// Well-formed id, but not enrolled.
    DenyNotEnrolled,
}

impl AccessDecision {
    pub fn is_allow(&self) -> bool {
        matches!(self, AccessDecision::Allow)
    }
}

/// Decide admission for the value of an `X-Iroh-Endpoint-Id` header.
///
/// `header` is `None` when the header is absent. Parsing and enrollment are the
/// only two gates; both must pass to `Allow`.
pub fn decide(registry: &Registry, header: Option<&str>) -> AccessDecision {
    let Some(raw) = header else {
        return AccessDecision::DenyMalformedId;
    };
    let endpoint = match EndpointId::from_hex(raw) {
        Ok(id) => id,
        Err(_) => return AccessDecision::DenyMalformedId,
    };
    if registry.is_enrolled(&endpoint) {
        AccessDecision::Allow
    } else {
        AccessDecision::DenyNotEnrolled
    }
}
