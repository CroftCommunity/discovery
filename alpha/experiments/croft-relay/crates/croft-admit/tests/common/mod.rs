//! Shared test scaffolding: ed25519 keypairs and an in-memory PDS fixture.
//!
//! This is the seed of the integration harness the plan (Phase 0) wants every
//! later phase to extend rather than re-rig. It is app-side only; the live-relay
//! legs attach here once the relay can be stood up.
//!
//! Each `tests/*.rs` is compiled as its own crate, so helpers used by only some
//! files read as dead code in the others — allow it for this shared module.
#![allow(dead_code)]

use std::collections::HashMap;

use croft_admit::did::Did;
use croft_admit::endpoint_id::EndpointId;
use croft_admit::pds::{EndpointRecord, PdsError, PdsResolver};

use ring::signature::{Ed25519KeyPair, KeyPair};

/// An ed25519 keypair in the forms jsonwebtoken wants: PKCS#8 DER for the
/// signer, raw 32-byte public key for the verifier.
pub struct KeyMaterial {
    pub pkcs8_der: Vec<u8>,
    pub public_raw: Vec<u8>,
}

pub fn generate_keypair() -> KeyMaterial {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).expect("keygen");
    let kp = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).expect("parse pkcs8");
    KeyMaterial {
        pkcs8_der: pkcs8.as_ref().to_vec(),
        public_raw: kp.public_key().as_ref().to_vec(),
    }
}

/// A deterministic `EndpointId` from a single seed byte — enough to make ids
/// distinct and stable across a test without pulling in randomness.
pub fn endpoint_from_seed(seed: u8) -> EndpointId {
    EndpointId::from_bytes([seed; 32])
}

pub fn did(s: &str) -> Did {
    Did::parse(s).expect("valid test did")
}

/// In-memory PDS: maps DID -> the record it publishes, or a forced error to
/// exercise deny-closed paths (timeout / not-found).
#[derive(Default)]
pub struct MockPds {
    records: HashMap<String, EndpointRecord>,
    errors: HashMap<String, PdsError>,
}

impl MockPds {
    pub fn new() -> Self {
        Self::default()
    }

    /// DID publishes `endpoint` in its `ing.croft.iroh.endpoint`/`self` record.
    pub fn publish(&mut self, did: &Did, endpoint: EndpointId) -> &mut Self {
        self.records.insert(
            did.as_str().to_string(),
            EndpointRecord {
                endpoint_id: endpoint,
                home_relay: "https://relay.croft.ing".to_string(),
            },
        );
        self
    }

    /// Force a PDS error for a DID (e.g. `Timeout`) to test deny-closed.
    pub fn fail(&mut self, did: &Did, err: PdsError) -> &mut Self {
        self.errors.insert(did.as_str().to_string(), err);
        self
    }
}

impl PdsResolver for MockPds {
    fn fetch_endpoint_record(&self, did: &Did) -> Result<EndpointRecord, PdsError> {
        if let Some(err) = self.errors.get(did.as_str()) {
            // PdsError isn't Clone; re-materialize the variant.
            return Err(match err {
                PdsError::NotFound => PdsError::NotFound,
                PdsError::Timeout => PdsError::Timeout,
                PdsError::Malformed => PdsError::Malformed,
            });
        }
        self.records
            .get(did.as_str())
            .cloned()
            .ok_or(PdsError::NotFound)
    }
}
