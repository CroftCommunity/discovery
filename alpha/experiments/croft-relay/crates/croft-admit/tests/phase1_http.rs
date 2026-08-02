//! Phase 1 over HTTP: the access-check contract as the relay actually calls it.
//!
//! Drives the axum router with `oneshot` (no socket bound), asserting the exact
//! grant shape iroh-relay's `access.http` mode requires: `200` + `true` to
//! admit, deny otherwise — and that it reads the real `X-Iroh-NodeId` header.

mod common;
use common::*;

use std::sync::Arc;

use axum::body::{to_bytes, Body};
use axum::http::{Request, StatusCode};
use croft_admit::http_api::{
    access_router, IROH_ENDPOINT_ID_HEADER, IROH_ENDPOINT_ID_HEADER_ALIAS,
};
use croft_admit::registry::Registry;
use tower::ServiceExt; // oneshot

async fn call(
    registry: Arc<Registry>,
    header_name: Option<&str>,
    header_val: Option<&str>,
) -> (StatusCode, String) {
    let mut builder = Request::builder().method("POST").uri("/access");
    if let (Some(n), Some(v)) = (header_name, header_val) {
        builder = builder.header(n, v);
    }
    let resp = access_router(registry)
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = resp.status();
    let bytes = to_bytes(resp.into_body(), 64).await.unwrap();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

#[tokio::test]
async fn enrolled_endpoint_gets_200_true_on_real_header() {
    let reg = Arc::new(Registry::new());
    let ep = endpoint_from_seed(1);
    reg.bind(ep, did("did:plc:alice"));

    let (status, body) = call(reg, Some(IROH_ENDPOINT_ID_HEADER), Some(&ep.to_hex())).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "true");
}

#[tokio::test]
async fn documented_alias_header_also_works() {
    let reg = Arc::new(Registry::new());
    let ep = endpoint_from_seed(1);
    reg.bind(ep, did("did:plc:alice"));

    let (status, body) = call(reg, Some(IROH_ENDPOINT_ID_HEADER_ALIAS), Some(&ep.to_hex())).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "true");
}

#[tokio::test]
async fn unenrolled_endpoint_gets_403_false() {
    let reg = Arc::new(Registry::new());
    let ep = endpoint_from_seed(1); // never enrolled

    let (status, body) = call(reg, Some(IROH_ENDPOINT_ID_HEADER), Some(&ep.to_hex())).await;

    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, "false");
}

#[tokio::test]
async fn missing_header_denies_closed() {
    let reg = Arc::new(Registry::new());
    let (status, body) = call(reg, None, None).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, "false");
}

#[tokio::test]
async fn malformed_header_denies_closed() {
    let reg = Arc::new(Registry::new());
    let (status, body) = call(reg, Some(IROH_ENDPOINT_ID_HEADER), Some("not-hex")).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, "false");
}
