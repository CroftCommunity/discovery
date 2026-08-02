//! atproto DID — the identity that *owns* an endpoint.
//!
//! Croft's identity lives in atproto. Enrollment binds a DID to an
//! `EndpointId` only after proving the DID's PDS repo names that endpoint (see
//! `enroll`). We keep the DID as an opaque, validated string; the admission
//! authority never parses did:plc / did:web internals — that is the PDS/PLC's
//! job, cited not re-verified (AGENTS.md fact-check rule).

/// An atproto decentralized identifier, e.g. `did:plc:abc123...`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Did(String);

#[derive(Debug, PartialEq, Eq)]
pub enum DidError {
    /// Missing the `did:` scheme prefix or otherwise empty of a method.
    Malformed,
}

impl Did {
    /// Accept a syntactically plausible DID. We require the `did:<method>:`
    /// shape and a non-empty method-specific id; anything else is rejected so a
    /// junk value can never reach the registry as a "binding".
    pub fn parse(s: &str) -> Result<Self, DidError> {
        let mut parts = s.splitn(3, ':');
        match (parts.next(), parts.next(), parts.next()) {
            (Some("did"), Some(method), Some(id)) if !method.is_empty() && !id.is_empty() => {
                Ok(Did(s.to_string()))
            }
            _ => Err(DidError::Malformed),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_plausible_did_and_preserves_it() {
        let d = Did::parse("did:plc:abc123").unwrap();
        assert_eq!(d.as_str(), "did:plc:abc123");
        assert_eq!(
            Did::parse("did:web:example.com").unwrap().as_str(),
            "did:web:example.com"
        );
    }

    #[test]
    fn rejects_junk_and_empty_components() {
        for bad in [
            "",         // empty
            "plc:abc",  // no did: scheme
            "did:plc:", // empty method-specific id
            "did::abc", // empty method
            "did:plc",  // no third component at all
        ] {
            assert_eq!(
                Did::parse(bad),
                Err(DidError::Malformed),
                "{bad:?} must be rejected"
            );
        }
    }
}
