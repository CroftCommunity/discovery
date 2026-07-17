//! Phase 9 (RUN-14 EXP-B): the §H hybrid **serve half** — a content-blind
//! sealed-record AppView that gates *offering*, never *reading*.
//!
//! This binary is the "server target" whose blindness is a compilation fact:
//! it is built WITHOUT the `client-seal` feature, so neither the seal/open code
//! nor the `chacha20poly1305` crate is in its dependency graph. There is no
//! configuration path that hands it a key — [`SealedState`] carries none.
//!
//! Run it to see the store stand up, serve opaque ciphertext to roster members,
//! and refuse everyone else with a flat 403. The full member/non-member matrix
//! (which needs signed service-auth tokens) is proven in the unit suite:
//!   cargo test --lib --features client-seal sealed

use std::sync::{Arc, Mutex};

use anyhow::Result;

use appview_validation::sealed::{
    add_to_roster, open_sealed_db, router, store_sealed, SealedState, GET_SEALED_RECORDS,
};
use appview_validation::serviceauth::MapResolver;
use appview_validation::server::http_get;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("install rustls ring crypto provider");

    println!("\n############ EXP-B — sealed offer-gating (§H hybrid serve half) ############\n");

    // A content-blind store. The bytes below are opaque to this process — it has
    // no seal/open code compiled in (no `client-seal` feature) and no key.
    let db = open_sealed_db("sealed.sqlite")?;
    add_to_roster(&db, "group:demo", "did:plc:member");
    store_sealed(&db, "group:demo", 0, b"nonce-000000", b"OPAQUE-CIPHERTEXT-BYTES-0");
    store_sealed(&db, "group:demo", 1, b"nonce-111111", b"OPAQUE-CIPHERTEXT-BYTES-1");
    println!("  stored 2 opaque records for group:demo; roster = [did:plc:member]");
    println!("  the server holds NO group key — SealedState has no key field, and");
    println!("  the seal/open AEAD is not compiled into this binary.");

    let state = SealedState {
        db: Arc::new(Mutex::new(db)),
        resolver: Arc::new(MapResolver::new()), // empty: this demo issues no tokens
        aud: "did:web:appview.stellin.test".to_string(),
        now: 1_800_000_000,
    };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    tokio::spawn(async move {
        axum::serve(listener, router(state)).await.unwrap();
    });

    // Anonymous request → flat 403 (no length/existence signal).
    let body = http_get(
        &addr.to_string(),
        &format!("/xrpc/{GET_SEALED_RECORDS}?group_id=group:demo&since=0"),
    )
    .await?;
    println!("\n  anonymous getSealedRecords → {}", body.trim());
    println!(
        "  (a verified roster member is offered the ciphertext; the offering-vs-reading"
    );
    println!("   distinction and the client-side decrypt are proven in the unit suite.)");

    println!("\n############ done ############");
    Ok(())
}
