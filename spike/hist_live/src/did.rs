//! DID doc utilities — enough to (a) find the PDS `serviceEndpoint` and (b)
//! extract the signing multikey for repo commit signature verification.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DidDoc {
    #[serde(default)]
    pub service: Vec<Service>,
    #[serde(default)]
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Service {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub controller: String,
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: String,
}

impl DidDoc {
    pub fn pds_endpoint(&self) -> Option<&str> {
        self.service
            .iter()
            .find(|s| s.type_ == "AtprotoPersonalDataServer" || s.id.ends_with("#atproto_pds"))
            .map(|s| s.service_endpoint.as_str())
    }

    /// The atproto signing method (`#atproto`).
    pub fn atproto_signing_method(&self) -> Option<&VerificationMethod> {
        self.verification_method
            .iter()
            .find(|v| v.id.ends_with("#atproto"))
    }
}

/// Parse a `did:key`-style multibase string (`zQ3s...` for secp256k1,
/// `z6Mk...` for Ed25519, `zDna...` for P-256) into (curve, raw bytes).
///
/// Multibase base58btc = 'z' prefix.  The decoded bytes carry a multicodec
/// varint prefix; secp256k1-pub = 0xe7 0x01, ed25519-pub = 0xed 0x01,
/// p256-pub = 0x80 0x24.
pub fn parse_multikey(mk: &str) -> Result<(KeyCurve, Vec<u8>), String> {
    let s = mk.strip_prefix('z').ok_or("expected 'z' multibase prefix")?;
    let decoded = bs58::decode(s)
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_vec()
        .map_err(|e| format!("base58 decode: {}", e))?;
    if decoded.len() < 3 {
        return Err("multikey too short".into());
    }
    let (curve, key_bytes) = match decoded[0..2] {
        [0xe7, 0x01] => (KeyCurve::Secp256k1, decoded[2..].to_vec()),
        [0xed, 0x01] => (KeyCurve::Ed25519, decoded[2..].to_vec()),
        [0x80, 0x24] => (KeyCurve::P256, decoded[2..].to_vec()),
        _ => {
            return Err(format!(
                "unknown multicodec prefix {:02x}{:02x}",
                decoded[0], decoded[1]
            ))
        }
    };
    Ok((curve, key_bytes))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCurve {
    Secp256k1,
    Ed25519,
    P256,
}

