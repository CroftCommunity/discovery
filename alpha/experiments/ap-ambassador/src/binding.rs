//! The dual-proof identity-upgrade binding (AP-V5 / P6).
//!
//! Fresh-start default: an AP actor acquiring an atproto identity opens a
//! new interval with NO linkage. The ONLY upgrade path is a subject-initiated
//! dual-proof binding: a record signed with the DID repo key over
//! `{DID, AP actor id, AP-side origin proof, antecedent = H(old receipt)}`.

use ed25519_dalek::SigningKey;

use crate::records::ReceiptRecord;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Did(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApOriginProof {
    pub key_id: KeyId,
    pub activity_body: Vec<u8>,
    pub signature_b64: String,
    pub actor_key_spki_der: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BindingRecord {
    pub did: Did,
    pub ap_actor: ActorId,
    pub ap_origin_proof: ApOriginProof,
    pub antecedent_receipt: ReceiptId,
    pub did_repo_pubkey: [u8; 32],
    pub did_repo_signature: [u8; 64],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradedPersonaFact {
    pub did: Did,
    pub ap_actor: ActorId,
    pub antecedent_receipt: ReceiptId,
    pub grade: BindingGrade,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingGrade {
    BindingAttestedFixture,
}

impl BindingGrade {
    pub fn as_str(&self) -> &'static str {
        match self {
            BindingGrade::BindingAttestedFixture => "binding-attested-fixture",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingError {
    MissingDidSignature,
    MissingApOriginProof,
    GatewayAuthoredBinding,
    ProofDoesNotNameBinding,
}

impl std::fmt::Display for BindingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BindingError::MissingDidSignature => write!(f, "binding: DID repo signature missing or invalid"),
            BindingError::MissingApOriginProof => write!(f, "binding: AP-side origin proof missing or invalid"),
            BindingError::GatewayAuthoredBinding => write!(f, "binding: gateway-authored bindings are rejected — bindings are subject-initiated"),
            BindingError::ProofDoesNotNameBinding => write!(f, "binding: AP-side origin proof does not name this binding"),
        }
    }
}
impl std::error::Error for BindingError {}

pub fn binding_signing_bytes(
    _did: &Did,
    _ap_actor: &ActorId,
    _ap_origin_proof: &ApOriginProof,
    _antecedent_receipt: &ReceiptId,
) -> Vec<u8> {
    unimplemented!("P6 GREEN: canonical dag-cbor over binding fields")
}

pub fn sign_binding(
    _did: Did,
    _ap_actor: ActorId,
    _ap_origin_proof: ApOriginProof,
    _antecedent_receipt: ReceiptId,
    _did_repo_key: &SigningKey,
) -> BindingRecord {
    unimplemented!("P6 GREEN: subject signs the binding envelope")
}

pub fn verify_binding(
    _binding: &BindingRecord,
    _antecedent: &ReceiptRecord,
    _gateway_pubkey: &[u8; 32],
) -> Result<UpgradedPersonaFact, BindingError> {
    unimplemented!("P6 GREEN: verify DID repo sig + AP origin proof + gateway-not-signer + proof names binding")
}
