//! atproto **inter-service auth** ("service auth") JWT verification — the
//! mechanism by which a real AppView learns *who its caller is* without a browser
//! or OAuth. RUN-14 EXP-A, the one unproven capability behind every Stellin
//! differentiator: the AppView has never known its caller.
//!
//! The flow (client side, proven live in `bin/authserve.rs`): the client
//! authenticates to its OWN PDS, calls `com.atproto.server.getServiceAuth` with
//! `aud` = the target service DID (and optionally `lxm` = the lexicon method it
//! intends to call), and presents the returned compact JWT as a bearer token.
//! The service (this module) verifies the signature against the issuer's
//! `#atproto` verification method, resolved from the issuer's DID document.
//!
//! ─────────────────────────────────────────────────────────────────────────────
//!  PREDICTIONS (write them down before the first live call — house style)
//! ─────────────────────────────────────────────────────────────────────────────
//!   P-A1: `getServiceAuth` returns `{ "token": <compact JWT> }`.
//!   P-A2: JWT claims include `iss` (user DID), `aud` (requested DID), `exp`
//!         (short, ~a minute out), and `lxm` when a lexicon method was requested.
//!   P-A3: the signature verifies against the `#atproto` verification method in
//!         the issuer's DID document (secp256k1 **or** p256), resolved via the
//!         PLC directory over HTTPS.
//! Reported CONFIRMED/DIVERGED in `bin/authserve.rs` and RUN-14-SUMMARY.md.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde::Deserialize;

/// The signing curve carried by an atproto `#atproto` verification method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Curve {
    /// secp256k1 (`ES256K`) — most `did:plc` accounts.
    Secp256k1,
    /// NIST p256 (`ES256`) — some accounts.
    P256,
}

/// Distinct rejection reasons. Distinctness is a step-1 acceptance criterion: a
/// caller (and the gated route) must be able to tell *why* a token was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    /// Not three base64url segments, or a segment/JSON that will not decode.
    Malformed(String),
    /// Signature did not verify against the issuer's resolved key.
    BadSignature,
    /// `exp` is at or before `now`.
    Expired,
    /// `aud` is not the audience this service expected.
    WrongAudience,
    /// `lxm` is absent or does not match the lexicon method this route requires.
    WrongLxm,
    /// The issuer DID could not be resolved to an `#atproto` key.
    UnresolvableIssuer,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::Malformed(w) => write!(f, "malformed token: {w}"),
            VerifyError::BadSignature => write!(f, "bad signature"),
            VerifyError::Expired => write!(f, "token expired"),
            VerifyError::WrongAudience => write!(f, "wrong audience"),
            VerifyError::WrongLxm => write!(f, "wrong lxm for this route"),
            VerifyError::UnresolvableIssuer => write!(f, "unresolvable issuer DID"),
        }
    }
}
impl std::error::Error for VerifyError {}

/// The verified claims we care about.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceAuthClaims {
    /// The caller's DID (`iss`) — the identity the AppView now *knows*.
    pub iss: String,
    /// The audience DID this token was minted for (`aud`).
    pub aud: String,
    /// Expiry, unix seconds.
    pub exp: i64,
    /// The lexicon method the token is scoped to, if any (`lxm`).
    pub lxm: Option<String>,
}

/// Resolves an issuer DID to its `#atproto` public key, as `publicKeyMultibase`.
///
/// Synchronous on purpose: the live path (`bin/authserve.rs`) does the async
/// PLC fetch first and hands a resolved `(did -> multibase)` map in, so
/// verification stays a pure, unit-testable function with a stub resolver.
pub trait KeyResolver {
    /// The `publicKeyMultibase` string for the issuer's `#atproto` method, or
    /// `None` if the DID / method cannot be resolved.
    fn atproto_key(&self, did: &str) -> Option<String>;
}

#[derive(Deserialize)]
struct JwtHeader {
    alg: String,
}

#[derive(Deserialize)]
struct JwtPayload {
    iss: String,
    aud: String,
    exp: i64,
    #[serde(default)]
    lxm: Option<String>,
}

/// Decode a `publicKeyMultibase` (`z…` base58btc multicodec) into its curve and
/// the raw compressed SEC1 point bytes.
pub fn decode_multikey(multibase: &str) -> Option<(Curve, Vec<u8>)> {
    let b58 = multibase.strip_prefix('z')?; // multibase 'z' == base58btc
    let bytes = bs58::decode(b58).into_vec().ok()?;
    // multicodec varint prefixes: secp256k1-pub = 0xe7 0x01, p256-pub = 0x80 0x24.
    if let Some(rest) = bytes.strip_prefix(&[0xe7, 0x01]) {
        Some((Curve::Secp256k1, rest.to_vec()))
    } else if let Some(rest) = bytes.strip_prefix(&[0x80, 0x24]) {
        Some((Curve::P256, rest.to_vec()))
    } else {
        None
    }
}

/// Verify a service-auth JWT end to end.
///
/// Check order gives each failure a distinct variant (each fixture breaks exactly
/// one thing): parse → resolve issuer → signature → exp → aud → lxm.
pub fn verify_service_jwt<R: KeyResolver>(
    token: &str,
    expected_aud: &str,
    required_lxm: Option<&str>,
    now_unix: i64,
    resolver: &R,
) -> Result<ServiceAuthClaims, VerifyError> {
    // STEP-1 RED STUB — implemented in the green commit.
    let _ = (token, expected_aud, required_lxm, now_unix, resolver);
    let _ = (URL_SAFE_NO_PAD, decode_multikey);
    unimplemented!("verify_service_jwt — implemented in the green commit")
}

// ─────────────────────────────────────────────────────────────────────────────
//  Test support: in-test key generation + JWT minting (fixtures before features).
//  This is scaffolding, not the mechanism under test; the mechanism is the
//  verifier above. Deterministic keys (fixed scalars) — no RNG in this env.
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;
    use serde_json::json;

    /// A resolver backed by an in-memory map, for tests.
    pub struct StubResolver {
        pub keys: std::collections::HashMap<String, String>,
    }
    impl KeyResolver for StubResolver {
        fn atproto_key(&self, did: &str) -> Option<String> {
            self.keys.get(did).cloned()
        }
    }

    fn b64(bytes: &[u8]) -> String {
        URL_SAFE_NO_PAD.encode(bytes)
    }

    /// Encode a compressed SEC1 point as an atproto `publicKeyMultibase`.
    fn multikey(curve: Curve, compressed: &[u8]) -> String {
        let mut buf = match curve {
            Curve::Secp256k1 => vec![0xe7, 0x01],
            Curve::P256 => vec![0x80, 0x24],
        };
        buf.extend_from_slice(compressed);
        format!("z{}", bs58::encode(buf).into_string())
    }

    /// A test identity: a curve, its multibase pubkey, and a signer over the
    /// signing input (returns the base64url r‖s signature).
    pub struct TestIdent {
        pub did: String,
        pub curve: Curve,
        pub multibase: String,
        sign: Box<dyn Fn(&[u8]) -> String>,
    }

    /// secp256k1 identity from a fixed 32-byte scalar.
    pub fn secp256k1_ident(did: &str, scalar: [u8; 32]) -> TestIdent {
        use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
        let sk = SigningKey::from_slice(&scalar).expect("valid secp256k1 scalar");
        let vk: VerifyingKey = *sk.verifying_key();
        let compressed = vk.to_encoded_point(true).as_bytes().to_vec();
        let multibase = multikey(Curve::Secp256k1, &compressed);
        let signer = move |input: &[u8]| {
            let sig: Signature = sk.sign(input);
            let sig = sig.normalize_s().unwrap_or(sig); // atproto low-S
            b64(&sig.to_bytes())
        };
        TestIdent {
            did: did.to_string(),
            curve: Curve::Secp256k1,
            multibase,
            sign: Box::new(signer),
        }
    }

    /// p256 identity from a fixed 32-byte scalar.
    pub fn p256_ident(did: &str, scalar: [u8; 32]) -> TestIdent {
        use p256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
        let sk = SigningKey::from_slice(&scalar).expect("valid p256 scalar");
        let vk: VerifyingKey = *sk.verifying_key();
        let compressed = vk.to_encoded_point(true).as_bytes().to_vec();
        let multibase = multikey(Curve::P256, &compressed);
        let signer = move |input: &[u8]| {
            let sig: Signature = sk.sign(input);
            let sig = sig.normalize_s().unwrap_or(sig);
            b64(&sig.to_bytes())
        };
        TestIdent {
            did: did.to_string(),
            curve: Curve::P256,
            multibase,
            sign: Box::new(signer),
        }
    }

    impl TestIdent {
        /// Mint a well-formed service-auth JWT with these claims.
        pub fn mint(&self, aud: &str, exp: i64, lxm: Option<&str>) -> String {
            let alg = match self.curve {
                Curve::Secp256k1 => "ES256K",
                Curve::P256 => "ES256",
            };
            let header = b64(json!({ "typ": "JWT", "alg": alg }).to_string().as_bytes());
            let mut claims = json!({ "iss": self.did, "aud": aud, "exp": exp });
            if let Some(l) = lxm {
                claims["lxm"] = json!(l);
            }
            let payload = b64(claims.to_string().as_bytes());
            let signing_input = format!("{header}.{payload}");
            let sig = (self.sign)(signing_input.as_bytes());
            format!("{signing_input}.{sig}")
        }

        /// Mint a token, then corrupt the signature (flip its last byte) — a
        /// well-formed token whose signature must fail to verify.
        pub fn mint_badsig(&self, aud: &str, exp: i64, lxm: Option<&str>) -> String {
            let good = self.mint(aud, exp, lxm);
            let (rest, sig) = good.rsplit_once('.').unwrap();
            let mut raw = URL_SAFE_NO_PAD.decode(sig).unwrap();
            let n = raw.len() - 1;
            raw[n] ^= 0x01;
            format!("{rest}.{}", b64(&raw))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;
    use std::collections::HashMap;

    const AUD: &str = "did:web:appview.stellin.test";
    const LXM: &str = "app.stellin.getProfileView";
    const NOW: i64 = 1_800_000_000; // fixed clock

    fn resolver_for(idents: &[&TestIdent]) -> StubResolver {
        let mut keys = HashMap::new();
        for id in idents {
            keys.insert(id.did.clone(), id.multibase.clone());
        }
        StubResolver { keys }
    }

    // P-A3: a well-formed secp256k1 token verifies against the resolved key.
    #[test]
    fn accepts_valid_secp256k1() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint(AUD, NOW + 60, Some(LXM));
        let claims = verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r).expect("valid token");
        assert_eq!(claims.iss, "did:plc:alice");
        assert_eq!(claims.aud, AUD);
        assert_eq!(claims.lxm.as_deref(), Some(LXM));
    }

    // p256 accounts (ES256) verify on the other curve.
    #[test]
    fn accepts_valid_p256() {
        let carol = p256_ident("did:plc:carol", [9u8; 32]);
        let r = resolver_for(&[&carol]);
        let tok = carol.mint(AUD, NOW + 60, None);
        let claims = verify_service_jwt(&tok, AUD, None, NOW, &r).expect("valid p256 token");
        assert_eq!(claims.iss, "did:plc:carol");
    }

    // (a) bad signature → BadSignature.
    #[test]
    fn rejects_bad_signature() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint_badsig(AUD, NOW + 60, Some(LXM));
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r),
            Err(VerifyError::BadSignature)
        );
    }

    // (b) expired → Expired.
    #[test]
    fn rejects_expired() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint(AUD, NOW - 1, Some(LXM));
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r),
            Err(VerifyError::Expired)
        );
    }

    // (c) wrong aud → WrongAudience.
    #[test]
    fn rejects_wrong_aud() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint("did:web:someone.else", NOW + 60, Some(LXM));
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r),
            Err(VerifyError::WrongAudience)
        );
    }

    // (d) wrong lxm for the route → WrongLxm.
    #[test]
    fn rejects_wrong_lxm() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint(AUD, NOW + 60, Some("app.stellin.somethingElse"));
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r),
            Err(VerifyError::WrongLxm)
        );
    }

    // (e) unresolvable issuer → UnresolvableIssuer.
    #[test]
    fn rejects_unresolvable_issuer() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let empty = StubResolver { keys: HashMap::new() };
        let tok = alice.mint(AUD, NOW + 60, Some(LXM));
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &empty),
            Err(VerifyError::UnresolvableIssuer)
        );
    }

    // A token with no lxm claim is fine on a route that requires none…
    #[test]
    fn no_lxm_required_accepts_no_lxm() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint(AUD, NOW + 60, None);
        assert!(verify_service_jwt(&tok, AUD, None, NOW, &r).is_ok());
    }

    // …but a route that requires an lxm rejects a token that carries none.
    #[test]
    fn lxm_required_rejects_missing_lxm() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        let tok = alice.mint(AUD, NOW + 60, None);
        assert_eq!(
            verify_service_jwt(&tok, AUD, Some(LXM), NOW, &r),
            Err(VerifyError::WrongLxm)
        );
    }

    // Garbage input is Malformed, never a panic.
    #[test]
    fn rejects_malformed() {
        let alice = secp256k1_ident("did:plc:alice", [7u8; 32]);
        let r = resolver_for(&[&alice]);
        for junk in ["", "a.b", "not-a-jwt", "a.b.c.d"] {
            match verify_service_jwt(junk, AUD, Some(LXM), NOW, &r) {
                Err(VerifyError::Malformed(_)) => {}
                other => panic!("expected Malformed for {junk:?}, got {other:?}"),
            }
        }
    }
}
