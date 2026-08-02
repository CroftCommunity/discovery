//! The enrolled-endpoint registry: `EndpointId -> Did` bindings.
//!
//! This is Phase 1's whole state: who is allowed to attach. It is deliberately
//! trivial (an in-memory map behind a mutex) because the interesting durability
//! and revocation questions move to Phase 2's *stateless* token path — the
//! registry stays the enrollment authority, not the per-connection oracle.
//!
//! Keyed by `EndpointId` because that is what the access check receives from
//! the relay; the `Did` is retained for audit and for re-issuing tokens.

use std::collections::HashMap;
use std::sync::Mutex;

use crate::did::Did;
use crate::endpoint_id::EndpointId;

#[derive(Default)]
pub struct Registry {
    bindings: Mutex<HashMap<EndpointId, Did>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            bindings: Mutex::new(HashMap::new()),
        }
    }

    /// Record an enrolled binding. Overwrites any prior binding for the same
    /// endpoint (re-enrollment is idempotent from the relay's point of view).
    pub fn bind(&self, endpoint: EndpointId, did: Did) {
        self.bindings.lock().unwrap().insert(endpoint, did);
    }

    /// Is this endpoint enrolled? The one question the access check asks.
    pub fn is_enrolled(&self, endpoint: &EndpointId) -> bool {
        self.bindings.lock().unwrap().contains_key(endpoint)
    }

    /// The DID bound to an endpoint, if any (for audit / token minting).
    pub fn did_for(&self, endpoint: &EndpointId) -> Option<Did> {
        self.bindings.lock().unwrap().get(endpoint).cloned()
    }

    pub fn len(&self) -> usize {
        self.bindings.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_then_one_binding() {
        let reg = Registry::new();
        assert_eq!(reg.len(), 0);
        assert!(reg.is_empty());

        let ep = EndpointId::from_bytes([9u8; 32]);
        reg.bind(ep, Did::parse("did:plc:x").unwrap());

        assert_eq!(reg.len(), 1);
        assert!(!reg.is_empty());
        assert!(reg.is_enrolled(&ep));
    }
}
