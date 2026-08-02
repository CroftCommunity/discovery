//! croft-relay-embed — the iroh-relay embedding adapter (Phase 2, Option A).
//!
//! ADR-0001 chose *embed*: depend on `iroh-relay` as a library and supply
//! admission through its public `AccessControl` trait, rather than forking. This
//! crate is that adapter, and the only place a real `iroh` dependency lives. All
//! the admission *logic* stays in the relay-agnostic `croft-admit`; here we do
//! three small, honest things:
//!
//!   1. Pull the token off the connection (`ClientRequest::auth_token()` — the
//!      relay already parses `Authorization: Bearer` / `?token=` for us).
//!   2. Pull the **cryptographically authenticated** endpoint id off the
//!      connection (`ClientRequest::endpoint_id()` — proven by the relay
//!      handshake before this hook runs) and hand both to
//!      `croft_admit::TokenVerifier`.
//!   3. Map the verdict to `Access::Allow` / `Access::Deny`.
//!
//! ## The one thing the trait cannot carry (ADR-0004)
//!
//! `iroh_relay::server::Access` is `Allow` / `Deny { reason }` — `Allow` has no
//! rate-limit field. So the tier the token proved cannot be attached to the
//! connection through this seam in 1.0.x. We still *compute* the tier's bucket
//! here (`EmbedDecision::Admit { tier, bucket }`) so the enforcement point is
//! ready and tested; applying it waits on the upstream
//! `Access::Allow { rate_limit: Option<ClientRateLimit> }` change this project
//! proposes (plan Phase 5). Until then a wrapping layer around the admitted
//! stream is the fallback.

use std::time::{SystemTime, UNIX_EPOCH};

use iroh_relay::server::{Access, AccessControl, ClientRequest};

use croft_admit::endpoint_id::EndpointId;
use croft_admit::registry::Registry;
use croft_admit::tier::{bucket_for, RateBucket, Tier};
use croft_admit::token::{TokenError, TokenVerifier};

/// The richer verdict the adapter computes before flattening to `Access`.
/// Carries the tier + bucket so the (currently un-plumbable) rate decision is
/// tested and ready; see the module note.
#[derive(Debug, PartialEq, Eq)]
pub enum EmbedDecision {
    Admit {
        tier: Tier,
        bucket: RateBucket,
    },
    /// No token presented on the connection.
    DenyNoToken,
    /// Token present but verification failed.
    DenyToken(TokenError),
}

impl EmbedDecision {
    pub fn is_admit(&self) -> bool {
        matches!(self, EmbedDecision::Admit { .. })
    }

    /// Flatten to the relay's coarser grant. The tier/bucket are dropped here
    /// only because `Access::Allow` cannot carry them yet (ADR-0004).
    pub fn to_access(&self) -> Access {
        match self {
            EmbedDecision::Admit { .. } => Access::Allow,
            EmbedDecision::DenyNoToken => Access::Deny {
                reason: Some("no admission token".to_string()),
            },
            EmbedDecision::DenyToken(_) => Access::Deny {
                reason: Some("invalid admission token".to_string()),
            },
        }
    }
}

/// Convert the relay's authenticated endpoint id into croft-admit's form.
/// Infallible: both are the same 32 ed25519 public-key bytes.
fn to_admit_id(req: &ClientRequest) -> EndpointId {
    EndpointId::from_bytes(*req.endpoint_id().as_bytes())
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX epoch")
        .as_secs()
}

/// Phase 2 embed: admit on a valid signed token bound to the connecting id.
pub struct TokenAccess {
    verifier: TokenVerifier,
}

// `AccessControl` requires `Debug`; keep it opaque so we never risk logging key
// material held inside the verifier.
impl std::fmt::Debug for TokenAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("TokenAccess")
    }
}

impl TokenAccess {
    pub fn new(verifier: TokenVerifier) -> Self {
        TokenAccess { verifier }
    }

    /// The pure decision, with an injected clock so it is testable without the
    /// wall clock. `on_connect` calls this with the real time.
    pub fn decide(&self, req: &ClientRequest, now: u64) -> EmbedDecision {
        let Some(token) = req.auth_token() else {
            return EmbedDecision::DenyNoToken;
        };
        let connecting = to_admit_id(req);
        match self.verifier.verify(&token, &connecting, now) {
            Ok(v) => EmbedDecision::Admit {
                tier: v.tier,
                bucket: bucket_for(v.tier),
            },
            Err(e) => EmbedDecision::DenyToken(e),
        }
    }
}

impl AccessControl for TokenAccess {
    async fn on_connect(&self, request: &ClientRequest) -> Access {
        self.decide(request, unix_now()).to_access()
    }
}

/// Phase 1 embed: admit if the connecting endpoint is in the registry. The
/// in-process equivalent of the HTTP hook, for a deployment that would rather
/// hold the registry in the relay than call out to `croft-admit`'s `/access`.
pub struct RegistryAccess {
    registry: std::sync::Arc<Registry>,
}

impl std::fmt::Debug for RegistryAccess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RegistryAccess")
    }
}

impl RegistryAccess {
    pub fn new(registry: std::sync::Arc<Registry>) -> Self {
        RegistryAccess { registry }
    }
}

impl AccessControl for RegistryAccess {
    async fn on_connect(&self, request: &ClientRequest) -> Access {
        if self.registry.is_enrolled(&to_admit_id(request)) {
            Access::Allow
        } else {
            Access::Deny {
                reason: Some("endpoint not enrolled".to_string()),
            }
        }
    }
}
