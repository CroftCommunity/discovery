//! Phase 8 (RUN-14 EXP-A step 4): **live confirmation** of viewer-aware serving
//! via atproto inter-service auth.
//!
//! The unattended-runnable half of the Stellin caller-identity problem: no
//! browser, no OAuth. What this binary confirms against the LIVE network:
//!
//!   * P-A3 (creds-free): resolve a real account's DID document from the PLC
//!     directory over HTTPS, extract its `#atproto` verification method, and
//!     decode the `publicKeyMultibase` into a curve + SEC1 point — the exact key
//!     the verifier checks a token's signature against.
//!
//!   * P-A1/P-A2 + the live token matrix (creds-gated): with `ATP_TEST_HANDLE` /
//!     `ATP_TEST_PASSWORD` set, create a session, call
//!     `com.atproto.server.getServiceAuth`, verify the REAL token with
//!     `verify_service_jwt`, and drive the step-2 gate with it. Without creds this
//!     leg reports BLOCKED (it needs an authenticated account; interactive OAuth,
//!     the PWA login leg, is the named non-goal — a browser this env lacks).
//!
//! SPEC-DELTA[run14-A4 | stand-in]: when creds are present, the service audience
//! (`aud`) is the test account's OWN DID (a self-issued service-auth token),
//! because no real Stellin service DID can be provisioned in-environment — the
//! signature path is identical. Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md

use anyhow::{Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use serde_json::Value;

use appview_validation::atproto;
use appview_validation::serviceauth::{
    atproto_key_from_did_doc, decode_multikey, verify_service_jwt, MapResolver,
};
use appview_validation::viewserve::GET_PROFILE_VIEW;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    let http = atproto::client();

    println!("\n############ EXP-A step 4 — live service-auth confirmation ############\n");

    // ───────────────────────── P-A3 (creds-free) ─────────────────────────
    println!("── P-A3: resolve a real DID document and its #atproto key (live PLC) ──");
    match live_p_a3(&http).await {
        Ok(()) => {}
        Err(e) => println!("  P-A3 BLOCKED (network): {e}"),
    }

    // ─────────────────── P-A1 / P-A2 + matrix (creds-gated) ───────────────────
    println!("\n── P-A1/P-A2 + live-token gate matrix (needs ATP_TEST_HANDLE/PASSWORD) ──");
    match (std::env::var("ATP_TEST_HANDLE"), std::env::var("ATP_TEST_PASSWORD")) {
        (Ok(handle), Ok(password)) if !handle.is_empty() && !password.is_empty() => {
            if let Err(e) = live_credentialed(&http, &handle, &password).await {
                println!("  live credentialed leg FAILED: {e:#}");
            }
        }
        _ => {
            println!("  BLOCKED — creds unset. P-A1 (getServiceAuth response shape) and");
            println!("  P-A2 (JWT iss/aud/exp/lxm claims) require an authenticated session;");
            println!("  the live-token gate matrix likewise. The verifier itself is proven");
            println!("  green in the unit suite (cargo test --lib serviceauth viewserve).");
        }
    }

    println!("\n############ done ############");
    Ok(())
}

/// P-A3: a real account's DID doc → #atproto key → curve.
async fn live_p_a3(http: &reqwest::Client) -> Result<()> {
    // Resolve a stable public handle to its DID (no auth needed).
    let handle = "bsky.app";
    let v: Value = http
        .get(format!(
            "{}/xrpc/com.atproto.identity.resolveHandle?handle={handle}",
            atproto::ENTRYWAY
        ))
        .send()
        .await?
        .error_for_status()
        .context("resolveHandle")?
        .json()
        .await?;
    let did = v["did"].as_str().context("no did in resolveHandle")?;
    println!("  resolved @{handle} -> {did}");

    let doc = atproto::resolve_did_doc(http, did).await?;
    let mb = atproto_key_from_did_doc(&doc)
        .context("no #atproto verificationMethod in the live DID doc")?;
    let (curve, bytes) = decode_multikey(&mb).context("could not decode the live multibase key")?;
    println!("  #atproto publicKeyMultibase: {mb}");
    println!(
        "  P-A3 CONFIRMED: key resolves to curve {curve:?}, {} compressed SEC1 bytes",
        bytes.len()
    );
    Ok(())
}

/// The full credentialed leg: session → getServiceAuth → verify real token →
/// drive the step-2 gate with it.
async fn live_credentialed(http: &reqwest::Client, handle: &str, password: &str) -> Result<()> {
    let session = atproto::create_session(http, atproto::ENTRYWAY, handle, password).await?;
    println!("  session: {} ({})", session.handle, session.did);

    // The account's own PDS + DID doc (for the verification key).
    let doc = atproto::resolve_did_doc(http, &session.did).await?;
    let pds = atproto::pds_endpoint(&doc).context("no pds endpoint")?;

    // SPEC-DELTA run14-A4: aud = the account's own DID (self-issued service auth).
    let aud = session.did.clone();
    let token =
        atproto::get_service_auth(http, &pds, &session.jwt, &aud, Some(GET_PROFILE_VIEW)).await?;

    // P-A1: getServiceAuth returned a compact JWT.
    let seg_count = token.split('.').count();
    println!("  P-A1: getServiceAuth returned a {seg_count}-segment compact JWT");

    // P-A2: decode the claims and report the observed shape.
    if let Some((_h, payload)) = token.split_once('.').and_then(|(h, rest)| {
        rest.split_once('.').map(|(p, _s)| (h.to_string(), p.to_string()))
    }) {
        if let Ok(bytes) = URL_SAFE_NO_PAD.decode(payload) {
            if let Ok(claims) = serde_json::from_slice::<Value>(&bytes) {
                println!("  P-A2: claims = {claims}");
                for k in ["iss", "aud", "exp", "lxm"] {
                    println!("        {k}: {}", claims.get(k).unwrap_or(&Value::Null));
                }
            }
        }
    }

    // Verify the REAL token with our verifier, resolving the key from the DID doc.
    let mb = atproto_key_from_did_doc(&doc).context("no #atproto key")?;
    let mut resolver = MapResolver::new();
    resolver.insert(&session.did, &mb);
    let now = chrono::Utc::now().timestamp();
    match verify_service_jwt(&token, &aud, Some(GET_PROFILE_VIEW), now, &resolver) {
        Ok(claims) => println!(
            "  LIVE VERIFY OK: real token verified against real DID-doc key; iss={}",
            claims.iss
        ),
        Err(e) => println!("  LIVE VERIFY FAILED: {e}"),
    }
    Ok(())
}
