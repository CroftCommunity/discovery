//! `did:key` resolution — the inline attestation's `key` field.
//!
//! A `did:key:z…` is multibase-base58btc (`z`) over a multicodec-prefixed
//! compressed public key. The spec blesses three ECDSA curves; the multicodec
//! prefix selects which:
//!
//! | curve | multicodec | unsigned-varint prefix | point len |
//! |---|---|---|---|
//! | secp256k1 | `0xe7`   | `e7 01` | 33 |
//! | P-256     | `0x1200` | `80 24` | 33 |
//! | P-384     | `0x1201` | `81 24` | 49 |
//!
//! Resolution is offline: the key material is IN the did:key. (AMBIGUITIES.md
//! A-1 records the open question the spec leaves here — nothing binds this
//! embedded key to the `issuer` DID's verification methods, so an inline
//! attestation proves "some key signed", not "the issuer signed", until a DID
//! document check is added. This module resolves the key; the binding gap is a
//! verifier-policy concern surfaced in `verify.rs`.)

use ecdsa::signature::Verifier;

use crate::ipld_json::ConvError;

/// A resolved ECDSA public key over one of the three blessed curves.
#[derive(Debug, Clone)]
pub enum PubKey {
    K256(k256::ecdsa::VerifyingKey),
    P256(p256::ecdsa::VerifyingKey),
    P384(p384::ecdsa::VerifyingKey),
}

/// Which curve a key/signature belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Curve {
    K256,
    P256,
    P384,
}

fn read_uvarint(b: &[u8]) -> Option<(u64, usize)> {
    let (mut x, mut s, mut i) = (0u64, 0u32, 0usize);
    while i < b.len() {
        let c = b[i];
        x |= ((c & 0x7f) as u64) << s;
        i += 1;
        if c & 0x80 == 0 {
            return Some((x, i));
        }
        s += 7;
        if s >= 64 {
            return None;
        }
    }
    None
}

impl PubKey {
    pub fn curve(&self) -> Curve {
        match self {
            PubKey::K256(_) => Curve::K256,
            PubKey::P256(_) => Curve::P256,
            PubKey::P384(_) => Curve::P384,
        }
    }

    /// Parse a `did:key:z…` string.
    pub fn parse_did_key(did: &str) -> Result<Self, ConvError> {
        let mb = did
            .strip_prefix("did:key:")
            .ok_or_else(|| ConvError("did:key: wrong prefix".into()))?;
        let z = mb
            .strip_prefix('z')
            .ok_or_else(|| ConvError("did:key: expected base58btc 'z'".into()))?;
        let raw = bs58::decode(z)
            .into_vec()
            .map_err(|e| ConvError(format!("did:key base58: {e}")))?;
        let (codec, n) = read_uvarint(&raw).ok_or_else(|| ConvError("did:key: bad multicodec".into()))?;
        let point = &raw[n..];
        match codec {
            0xe7 => Ok(PubKey::K256(
                k256::ecdsa::VerifyingKey::from_sec1_bytes(point)
                    .map_err(|e| ConvError(format!("k256 key: {e}")))?,
            )),
            0x1200 => Ok(PubKey::P256(
                p256::ecdsa::VerifyingKey::from_sec1_bytes(point)
                    .map_err(|e| ConvError(format!("p256 key: {e}")))?,
            )),
            0x1201 => Ok(PubKey::P384(
                p384::ecdsa::VerifyingKey::from_sec1_bytes(point)
                    .map_err(|e| ConvError(format!("p384 key: {e}")))?,
            )),
            other => Err(ConvError(format!("did:key: unblessed multicodec {other:#x}"))),
        }
    }

    /// Render this key back to a `did:key:z…` string (used to build fixtures).
    pub fn to_did_key(&self) -> String {
        let (codec_varint, point): (&[u8], Vec<u8>) = match self {
            PubKey::K256(k) => (&[0xe7, 0x01], k.to_encoded_point(true).as_bytes().to_vec()),
            PubKey::P256(k) => (&[0x80, 0x24], k.to_encoded_point(true).as_bytes().to_vec()),
            PubKey::P384(k) => (&[0x81, 0x24], k.to_encoded_point(true).as_bytes().to_vec()),
        };
        let raw: Vec<u8> = [codec_varint, &point].concat();
        format!("did:key:z{}", bs58::encode(raw).into_string())
    }

    /// Verify a raw (r‖s, fixed-length, big-endian) signature over `msg`.
    /// Per the spec, `msg` is the 36-byte binary CID; the per-curve default
    /// digest applies (SHA-256 for K-256/P-256, SHA-384 for P-384 — AMBIGUITIES
    /// A-5). Returns Ok(()) on a valid signature.
    pub fn verify_raw(&self, msg: &[u8], sig_bytes: &[u8]) -> Result<(), ConvError> {
        match self {
            PubKey::K256(vk) => {
                let sig = k256::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("k256 sig: {e}")))?;
                vk.verify(msg, &sig).map_err(|e| ConvError(format!("k256 verify: {e}")))
            }
            PubKey::P256(vk) => {
                let sig = p256::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("p256 sig: {e}")))?;
                vk.verify(msg, &sig).map_err(|e| ConvError(format!("p256 verify: {e}")))
            }
            PubKey::P384(vk) => {
                let sig = p384::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("p384 sig: {e}")))?;
                vk.verify(msg, &sig).map_err(|e| ConvError(format!("p384 verify: {e}")))
            }
        }
    }

    /// Whether a raw signature is in low-S canonical form (the spec mandates it).
    /// A high-S signature is malleable; a strict verifier rejects it.
    pub fn is_low_s(&self, sig_bytes: &[u8]) -> Result<bool, ConvError> {
        Ok(match self {
            PubKey::K256(_) => {
                let s = k256::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("k256 sig: {e}")))?;
                s.normalize_s().is_none()
            }
            PubKey::P256(_) => {
                let s = p256::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("p256 sig: {e}")))?;
                s.normalize_s().is_none()
            }
            PubKey::P384(_) => {
                let s = p384::ecdsa::Signature::from_slice(sig_bytes)
                    .map_err(|e| ConvError(format!("p384 sig: {e}")))?;
                s.normalize_s().is_none()
            }
        })
    }
}
