//! The dual-proof identity-upgrade binding (AP-V5 / P6).
//!
//! Fresh-start default: an AP actor acquiring an atproto identity opens a
//! new interval with NO linkage. The ONLY upgrade path is a
//! subject-initiated dual-proof binding: a record signed with the DID repo
//! key over `{DID, AP actor id, AP-side origin proof, antecedent = H(old
//! receipt)}`, minted by the subject, carrying the old gateway fact as
//! antecedent.
//!
//! - A binding missing either proof leg is rejected.
//! - A binding whose signer is the gateway (not the subject) is rejected.
//! - The AP-side origin proof is fixture-level this run (a signed AP
//!   activity from the actor's key). The live rel="me"/actor-document leg
//!   is gated with the live leg (brief §6).

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::records::ReceiptRecord;
use crate::types::*;

/// A DID (persona identity, atproto side). Byte-opaque this run —
/// declared stand-in; the atproto DID-doc resolve is out of scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Did(pub String);

/// The AP-side origin proof: a signed AP activity produced by the AP
/// actor's key, proving the AP actor consents to the binding. Fixture-level
/// this run — the proof is verified with the same RSA verify path P1 uses,
/// but the fetching of the actor's public key is fixtured, not live.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApOriginProof {
    /// The keyId that signed this proof (the AP actor's `#main-key`).
    pub key_id: KeyId,
    /// The raw AP activity JSON that was signed (must name both the DID
    /// and the receipt id being bound — else it is not a proof about THIS
    /// binding).
    pub activity_body: Vec<u8>,
    /// The base64-encoded RSA-SHA256 signature over the activity body's
    /// canonical hash.
    pub signature_b64: String,
    /// The SPKI DER bytes of the AP actor's public key at proof-verify
    /// time (byte-faithful, like the receipt evidence).
    pub actor_key_spki_der: Vec<u8>,
}

/// The binding record — the subject-authored fact that upgrades an AP
/// identity into a Croft persona. Signed by the DID repo key (ed25519).
#[derive(Debug, Clone)]
pub struct BindingRecord {
    pub did: Did,
    pub ap_actor: ActorId,
    pub ap_origin_proof: ApOriginProof,
    /// The old gateway receipt this binding names as antecedent. The
    /// upgraded fact carries this in its lineage — it does NOT link
    /// unilaterally; a caller MUST supply the antecedent id.
    pub antecedent_receipt: ReceiptId,
    /// The DID repo public key at signature time (ed25519 SPKI-32 bytes).
    pub did_repo_pubkey: [u8; 32],
    /// The DID repo signature over the canonical binding bytes (ed25519,
    /// 64 bytes).
    pub did_repo_signature: [u8; 64],
}

/// The upgraded persona fact — the outcome of a valid binding. Carries the
/// binding's own id as its own identity, inherits grade "binding-attested"
/// (NOT the gateway's grade), and holds continuity across the identity
/// boundary via `antecedent_receipt`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpgradedPersonaFact {
    pub did: Did,
    pub ap_actor: ActorId,
    pub antecedent_receipt: ReceiptId,
    pub grade: BindingGrade,
}

/// The grade an upgraded persona fact carries — deliberately its OWN grade,
/// not the gateway's. AP-V5: "the upgraded fact inherits grade from the
/// binding, not from the gateway's observation."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingGrade {
    /// Fixture-level AP-side origin proof (this run). Live upgrade rides
    /// the gated live leg (brief §6).
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
    /// The binding is missing the DID repo signature or the signature
    /// does not verify against the stated did_repo_pubkey.
    MissingDidSignature,
    /// The AP-side origin proof is missing, wrong-typed, or does not
    /// verify against the stated actor_key_spki_der.
    MissingApOriginProof,
    /// The binding's stated signer is the ambassador (gateway) itself,
    /// not the subject. Bindings are subject-initiated — a
    /// gateway-authored binding is rejected.
    GatewayAuthoredBinding,
    /// The AP-side origin proof does not name the DID or the receipt id
    /// that this binding is trying to bind — the proof is for a
    /// different binding.
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

/// Canonical bytes to sign: dag-cbor of `{did, ap_actor, ap_origin, antecedent}`.
/// The did_repo_pubkey and did_repo_signature are NOT included in the
/// signing bytes (they are the signature envelope, not the payload).
pub fn binding_signing_bytes(
    did: &Did,
    ap_actor: &ActorId,
    ap_origin_proof: &ApOriginProof,
    antecedent_receipt: &ReceiptId,
) -> Vec<u8> {
    use std::collections::BTreeMap;
    use ipld_core::ipld::Ipld;
    let mut m: BTreeMap<String, Ipld> = BTreeMap::new();
    m.insert("a".to_string(), Ipld::String(ap_actor.0.clone()));
    m.insert("d".to_string(), Ipld::String(did.0.clone()));
    m.insert("n".to_string(), Ipld::Bytes(antecedent_receipt.0.to_vec()));
    // Sub-map for the AP origin proof (all four fields, so the signature
    // covers exactly what will later be verified against the AP actor's
    // key).
    let mut po: BTreeMap<String, Ipld> = BTreeMap::new();
    po.insert("b".to_string(), Ipld::Bytes(ap_origin_proof.activity_body.clone()));
    po.insert("i".to_string(), Ipld::String(ap_origin_proof.key_id.0.clone()));
    po.insert("k".to_string(), Ipld::Bytes(ap_origin_proof.actor_key_spki_der.clone()));
    po.insert("s".to_string(), Ipld::String(ap_origin_proof.signature_b64.clone()));
    m.insert("p".to_string(), Ipld::Map(po));
    serde_ipld_dagcbor::to_vec(&Ipld::Map(m)).expect("dag-cbor encode of pure map cannot fail")
}

/// Sign a binding record with the subject's DID repo key. This is the
/// only sanctioned mint path — a gateway cannot substitute here, because
/// the gateway does not hold the DID repo private key (T-AP6.4 pins
/// gateway-authored bindings rejected).
pub fn sign_binding(
    did: Did,
    ap_actor: ActorId,
    ap_origin_proof: ApOriginProof,
    antecedent_receipt: ReceiptId,
    did_repo_key: &SigningKey,
) -> BindingRecord {
    let bytes = binding_signing_bytes(&did, &ap_actor, &ap_origin_proof, &antecedent_receipt);
    let sig: Signature = did_repo_key.sign(&bytes);
    BindingRecord {
        did,
        ap_actor,
        ap_origin_proof,
        antecedent_receipt,
        did_repo_pubkey: did_repo_key.verifying_key().to_bytes(),
        did_repo_signature: sig.to_bytes(),
    }
}

/// Verify a binding record. Returns an `UpgradedPersonaFact` on success.
///
/// - DID repo signature must verify (else `MissingDidSignature`).
/// - AP-side origin proof must verify against its stated actor key (else
///   `MissingApOriginProof`).
/// - The proof activity body must name both the DID and the antecedent
///   receipt id (else `ProofDoesNotNameBinding`).
/// - The `did_repo_pubkey` must not equal the caller-supplied gateway key
///   (else `GatewayAuthoredBinding`) — proves the mint was subject-side.
pub fn verify_binding(
    binding: &BindingRecord,
    antecedent: &ReceiptRecord,
    gateway_pubkey: &[u8; 32],
) -> Result<UpgradedPersonaFact, BindingError> {
    // 1) Gateway didn't sign this.
    if &binding.did_repo_pubkey == gateway_pubkey {
        return Err(BindingError::GatewayAuthoredBinding);
    }

    // 2) DID repo signature verifies.
    let vk = VerifyingKey::from_bytes(&binding.did_repo_pubkey)
        .map_err(|_| BindingError::MissingDidSignature)?;
    let sig = Signature::from_bytes(&binding.did_repo_signature);
    let bytes = binding_signing_bytes(
        &binding.did,
        &binding.ap_actor,
        &binding.ap_origin_proof,
        &binding.antecedent_receipt,
    );
    vk.verify(&bytes, &sig)
        .map_err(|_| BindingError::MissingDidSignature)?;

    // 3) AP-side origin proof verifies against the AP actor key.
    if !verify_ap_origin_proof(&binding.ap_origin_proof) {
        return Err(BindingError::MissingApOriginProof);
    }

    // 4) The proof activity body names both the DID and the antecedent receipt.
    if !proof_names_binding(&binding.ap_origin_proof, &binding.did, &binding.antecedent_receipt) {
        return Err(BindingError::ProofDoesNotNameBinding);
    }

    // 5) The antecedent receipt id passed in equals what the binding claims.
    // (Structural — the caller supplies the antecedent to cross-check with
    // the binding's stated id; a mismatch means the caller has confused
    // records.)
    if antecedent.receipt_id() != binding.antecedent_receipt {
        return Err(BindingError::ProofDoesNotNameBinding);
    }

    Ok(UpgradedPersonaFact {
        did: binding.did.clone(),
        ap_actor: binding.ap_actor.clone(),
        antecedent_receipt: binding.antecedent_receipt,
        grade: BindingGrade::BindingAttestedFixture,
    })
}

fn verify_ap_origin_proof(proof: &ApOriginProof) -> bool {
    use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
    use rsa::pkcs1v15::{Signature as RsaSignature, VerifyingKey as RsaVerifyingKey};
    use rsa::pkcs8::DecodePublicKey;
    use rsa::signature::Verifier as _;
    use rsa::RsaPublicKey;
    use sha2::Sha256;
    let Ok(sig_bytes) = B64.decode(proof.signature_b64.as_bytes()) else {
        return false;
    };
    let Ok(pub_key) = RsaPublicKey::from_public_key_der(&proof.actor_key_spki_der) else {
        return false;
    };
    let verifying_key = RsaVerifyingKey::<Sha256>::new(pub_key);
    let Ok(signature) = RsaSignature::try_from(sig_bytes.as_slice()) else {
        return false;
    };
    verifying_key.verify(&proof.activity_body, &signature).is_ok()
}

fn proof_names_binding(proof: &ApOriginProof, did: &Did, antecedent: &ReceiptId) -> bool {
    // Simple substring check on the raw proof body. Fixture-level; a real
    // implementation would parse the JSON. Both the DID URL and the
    // receipt-id hex must appear in the signed body — the binding is
    // proof-of-consent for THIS DID over THIS receipt, no other.
    let Ok(text) = std::str::from_utf8(&proof.activity_body) else {
        return false;
    };
    let did_ok = text.contains(&did.0);
    let ant_ok = text.contains(&format!("{}", antecedent));
    did_ok && ant_ok
}
