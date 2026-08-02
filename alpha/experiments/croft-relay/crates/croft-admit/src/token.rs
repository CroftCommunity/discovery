//! Phase 2: signed per-endpoint capability tokens (JWT / EdDSA).
//!
//! This replaces "the relay asks a database on every connection" with "the
//! relay verifies a signature and holds no state". A token is a capability: it
//! says *this enrollment authority attests that endpoint `sub`, at tier `tier`,
//! may attach until `exp`*. Verification is three independent gates, all of
//! which must pass:
//!
//!   1. **Signature** by the enrollment key (jsonwebtoken/EdDSA over the JWS).
//!   2. **Not expired** — `now <= exp + leeway` (and `now >= iat - leeway`).
//!   3. **Bound id** — the token's `sub` equals the endpoint the relay
//!      *cryptographically authenticated* on this connection.
//!
//! Gate 3 is what makes a stolen token worthless: possessing the bytes does not
//! let you present them from a different endpoint, because the relay proved
//! which key it is talking to. A token without the enrollment key is inert;
//! the key without a token admits nothing.
//!
//! ## Determinism / clock injection
//!
//! We disable jsonwebtoken's built-in `exp` validation and check expiry
//! ourselves against a caller-supplied `now` (unix seconds). Two reasons: the
//! test matrix (expired / clock-skew bounds) must be deterministic, not
//! wall-clock-and-`sleep`; and the expiry decision is exactly the logic the
//! mutation-testing gate must not leave a survivor in, so it is explicit and
//! local rather than delegated to the library's `SystemTime::now()`. The HTTP
//! edge passes real `SystemTime` seconds; the core never reads the clock.
//!
//! ## Revocation
//!
//! There is none, by design (ADR-0003): tokens are short-lived and revocation
//! is "let it expire and refuse to re-issue". Replay of a still-valid token by
//! the legitimate endpoint is *fine* — tokens are capabilities, not nonces.

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::endpoint_id::EndpointId;
use crate::tier::Tier;

/// JWT claim set. `sub` carries the hex `EndpointId`; the rest is the minimal
/// capability envelope. Kept flat and boring so the wire form is auditable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Claims {
    /// Hex `EndpointId` this token authorizes.
    pub sub: String,
    /// Admission tier -> rate bucket (see `tier`).
    pub tier: Tier,
    /// Issuer (the enrollment service identifier).
    pub iss: String,
    /// Issued-at (unix seconds).
    pub iat: u64,
    /// Expiry (unix seconds).
    pub exp: u64,
}

/// A verified token: the claims plus the fact that gate 3 (id binding) passed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedClaims {
    pub endpoint: EndpointId,
    pub tier: Tier,
}

/// Every way verification can refuse. Each is a distinct deny so the deny path
/// is testable and mutation-visible; the relay treats all of them identically
/// (reject the attach).
#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    /// Not a well-formed JWS, or signature/algorithm check failed (covers wrong
    /// issuer key and tampering — jsonwebtoken does not distinguish them).
    SignatureOrMalformed,
    /// `iss` claim is not the issuer we trust.
    WrongIssuer,
    /// `now > exp + leeway`.
    Expired,
    /// `now < iat - leeway` (token from the future — clock skew beyond bound).
    NotYetValid,
    /// `sub` is not decodable as an `EndpointId`.
    BadSubject,
    /// `sub` != the connecting endpoint's authenticated id. The anti-replay
    /// gate: a valid token presented from the wrong endpoint.
    IdMismatch,
}

/// Mints tokens after enrollment. Holds the private signing key.
pub struct TokenIssuer {
    key: EncodingKey,
    iss: String,
    ttl_secs: u64,
}

impl TokenIssuer {
    /// `signing_key_der` is a PKCS#8 ed25519 private key (DER).
    pub fn new(signing_key_der: &[u8], iss: impl Into<String>, ttl_secs: u64) -> Self {
        TokenIssuer {
            key: EncodingKey::from_ed_der(signing_key_der),
            iss: iss.into(),
            ttl_secs,
        }
    }

    /// Mint a token for `endpoint` at `tier`, valid from `now` for `ttl_secs`.
    /// `now` is injected so issuance is deterministic in tests; the edge passes
    /// real unix seconds.
    pub fn mint(&self, endpoint: &EndpointId, tier: Tier, now: u64) -> String {
        let claims = Claims {
            sub: endpoint.to_hex(),
            tier,
            iss: self.iss.clone(),
            iat: now,
            exp: now + self.ttl_secs,
        };
        let header = Header::new(Algorithm::EdDSA);
        jsonwebtoken::encode(&header, &claims, &self.key)
            .expect("mint: encoding a well-formed claim set cannot fail")
    }
}

/// Verifies tokens at the relay admission point. Holds only the public key and
/// the trusted issuer string — no per-endpoint state.
pub struct TokenVerifier {
    validation: Validation,
    key: DecodingKey,
    expected_iss: String,
    leeway_secs: u64,
}

impl TokenVerifier {
    /// `verifying_key_der` is the raw 32-byte ed25519 public key.
    pub fn new(
        verifying_key_der: &[u8],
        expected_iss: impl Into<String>,
        leeway_secs: u64,
    ) -> Self {
        let mut validation = Validation::new(Algorithm::EdDSA);
        // We own expiry and issuer checks (deterministic + explicit deny
        // reasons); let jsonwebtoken own only signature + algorithm.
        validation.validate_exp = false;
        validation.required_spec_claims = Default::default();
        TokenVerifier {
            validation,
            key: DecodingKey::from_ed_der(verifying_key_der),
            expected_iss: expected_iss.into(),
            leeway_secs,
        }
    }

    /// Verify `token` for a connection whose relay-authenticated identity is
    /// `connecting`. All three gates must pass. `now` is injected unix seconds.
    pub fn verify(
        &self,
        token: &str,
        connecting: &EndpointId,
        now: u64,
    ) -> Result<VerifiedClaims, TokenError> {
        // Gate 1: signature + algorithm (and structural well-formedness).
        let data = jsonwebtoken::decode::<Claims>(token, &self.key, &self.validation)
            .map_err(|_| TokenError::SignatureOrMalformed)?;
        let claims = data.claims;

        // Issuer: we only honor tokens from the enrollment service we trust.
        if claims.iss != self.expected_iss {
            return Err(TokenError::WrongIssuer);
        }

        // Gate 2: temporal validity with symmetric leeway for clock skew.
        if now > claims.exp.saturating_add(self.leeway_secs) {
            return Err(TokenError::Expired);
        }
        if now + self.leeway_secs < claims.iat {
            return Err(TokenError::NotYetValid);
        }

        // Gate 3: the token's subject must be the endpoint the relay actually
        // authenticated on this connection. This is the anti-replay hinge.
        let sub = EndpointId::from_hex(&claims.sub).map_err(|_| TokenError::BadSubject)?;
        if &sub != connecting {
            return Err(TokenError::IdMismatch);
        }

        Ok(VerifiedClaims {
            endpoint: sub,
            tier: claims.tier,
        })
    }
}
