//! The relay-facing HTTP access-check service (Phase 1 wiring).
//!
//! iroh-relay's `access.http` mode POSTs to this service for every incoming
//! connection and grants attach only on `200` + body `true`. This module is
//! that endpoint: it reads the endpoint-id header, runs the pure `access::decide`
//! logic against the shared registry, and answers in the exact shape the relay
//! expects.
//!
//! ## Header-name correction (Phase-0 recon, reality wins)
//!
//! The plan and iroh's own doc-comment say the header is `X-Iroh-Endpoint-Id`.
//! The pinned `iroh-relay 1.0.0-rc.1` source actually sends the literal
//! `X-Iroh-NodeId` (`iroh-relay/src/main.rs`: `const X_IROH_ENDPOINT_ID: &str =
//! "X-Iroh-NodeId"`) — the NodeId->EndpointId rename did not reach the wire
//! literal. We key on the real bytes and accept the documented alias too, so a
//! future rename does not silently break admission. See ADR-0001.

use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};

use crate::access::decide;
use crate::registry::Registry;

/// The header iroh-relay 1.0.0-rc.1 actually sends (verified against source).
pub const IROH_ENDPOINT_ID_HEADER: &str = "X-Iroh-NodeId";
/// The header name the docs/plan use; accepted as an alias for forward safety.
pub const IROH_ENDPOINT_ID_HEADER_ALIAS: &str = "X-Iroh-Endpoint-Id";

/// Build the access-check router over a shared registry.
pub fn access_router(registry: Arc<Registry>) -> Router {
    Router::new()
        .route("/access", post(access_check))
        .with_state(registry)
}

/// Read the endpoint-id header and answer the relay's grant contract:
/// `200 OK` + body `true` to admit, `403 Forbidden` + body `false` otherwise.
async fn access_check(
    State(registry): State<Arc<Registry>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let header = headers
        .get(IROH_ENDPOINT_ID_HEADER)
        .or_else(|| headers.get(IROH_ENDPOINT_ID_HEADER_ALIAS))
        .and_then(|v| v.to_str().ok());

    if decide(&registry, header).is_allow() {
        (StatusCode::OK, "true")
    } else {
        // Deny-closed: any doubt is a 403. The body stays `false` so a relay
        // that only checks the status *and* one that checks the body agree.
        (StatusCode::FORBIDDEN, "false")
    }
}
