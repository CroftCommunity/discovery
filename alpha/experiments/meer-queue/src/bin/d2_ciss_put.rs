//! **D2 probe** — what does an `App` need to construct, and what auth does a real
//! `PUT /{did}/objects/{key}` require over loopback HTTP?
//!
//! Disposition: `promote` — becomes `src/ciss_harness.rs` in Phase 1.
//!
//! Success criteria (plan Phase 0 D2): a recorded 2xx with the returned content
//! address, and the exact header set that produced it.
//!
//! Also probes, because Phase 1's wiring test depends on them:
//!   - the `MAX_OBJECT_BYTES` boundary at BOTH edges (exactly 2 MiB accepted,
//!     2 MiB + 1 refused) — the Pass 3 mutation-resistance addition;
//!   - `GET /{did}/du`, which S2's dedup measurement needs;
//!   - dedup: the same bytes PUT twice, and one blob file on disk.

use ciss::server::{App, Blobs, Db, Limits};

/// CISS's own cap (`CISS/src/blobstore.rs:25`), restated so the probe can sit on it.
const MAX_OBJECT_BYTES: usize = 2 * 1024 * 1024;

#[tokio::main]
async fn main() {
    let dir = tempfile::tempdir().expect("tempdir");
    let blob_root = dir.path().to_path_buf();
    let app = App::with_limits(
        "meer-spike-provider",
        Blobs::Fs(blob_root.clone()),
        Db::Memory,
        Limits {
            store_ceiling: 1024 * 1024 * 1024,
            did_cap: None,
        },
    )
    .expect("build app");

    let router = app.router();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind");
    let addr = listener.local_addr().expect("addr");
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let server = tokio::spawn(async move {
        axum::serve(listener, router)
            .with_graceful_shutdown(async move {
                let _ = rx.await;
            })
            .await
            .expect("serve");
    });

    // The caller identity: an `id:` session, the same shape CISS's own harness uses
    // (tests/common/mod.rs:181-183, :313-320).
    let keypair = ciss::crypto::derive_keypair("meer-spike", "alice");
    let did = ciss::identity::derive_id(&keypair.verifying_key());
    let challenge = format!("ciss-session/v1/{did}");
    let pubkey_hex = keypair.public_key_hex();
    let session_sig = keypair.sign_message(&challenge);

    let client = reqwest::Client::new();
    let base = format!("http://{addr}");

    let put = |key: String, bytes: Vec<u8>| {
        let (c, b, p, s) = (
            client.clone(),
            base.clone(),
            pubkey_hex.clone(),
            session_sig.clone(),
        );
        let did = did.clone();
        async move {
            let resp = c
                .put(format!("{b}/{did}/objects/{key}"))
                .header("x-croft-pubkey", p)
                .header("x-croft-session", s)
                .body(bytes)
                .send()
                .await
                .expect("send");
            (resp.status().as_u16(), resp.text().await.unwrap_or_default())
        }
    };

    println!("=== D2: CISS PUT/GET over real loopback HTTP ===");
    println!("addr        = {addr}");
    println!("did         = {did}");
    println!("headers     = x-croft-pubkey, x-croft-session (challenge: {challenge})");

    // 1. An ordinary small object.
    let (status, body) = put("hello.txt".to_string(), b"meer spike payload".to_vec()).await;
    println!("\n[1] small PUT      -> {status} {body}");

    // 2. GET it back by content address, and confirm bytes round-trip.
    let cid = body
        .split('"')
        .find(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit()))
        .map(str::to_string);
    match &cid {
        Some(cid) => {
            let resp = client
                .get(format!("{base}/{did}/objects/{cid}"))
                .header("x-croft-pubkey", &pubkey_hex)
                .header("x-croft-session", &session_sig)
                .send()
                .await
                .expect("send");
            let st = resp.status().as_u16();
            let got = resp.bytes().await.expect("bytes").to_vec();
            println!(
                "[2] GET {cid} -> {st}, {} bytes, round-trips = {}",
                got.len(),
                got == b"meer spike payload"
            );
        }
        None => println!("[2] GET skipped — no cid parsed from body"),
    }

    // 3. The cap, at BOTH edges (Pass 3).
    let (st_at, _) = put("at-cap".to_string(), vec![0u8; MAX_OBJECT_BYTES]).await;
    let (st_over, body_over) = put("over-cap".to_string(), vec![0u8; MAX_OBJECT_BYTES + 1]).await;
    println!("[3] PUT exactly 2 MiB   -> {st_at}");
    println!("    PUT 2 MiB + 1       -> {st_over} {}", body_over.trim());

    // 4. Dedup: identical bytes twice.
    let (_, b1) = put("dup-a".to_string(), b"identical".to_vec()).await;
    let (_, b2) = put("dup-b".to_string(), b"identical".to_vec()).await;
    println!("[4] same bytes twice -> same cid = {}", b1 == b2);

    // 5. du — the accounting S2 needs.
    let du = client
        .get(format!("{base}/{did}/du"))
        .header("x-croft-pubkey", &pubkey_hex)
        .header("x-croft-session", &session_sig)
        .send()
        .await
        .expect("send");
    println!(
        "[5] GET /du -> {} {}",
        du.status().as_u16(),
        du.text().await.unwrap_or_default().trim()
    );

    // 6. What actually landed on disk (S2's second, independent source).
    let mut files = vec![];
    for entry in walk(&blob_root) {
        files.push(entry);
    }
    println!("[6] blob files on disk = {}", files.len());
    for f in files.iter().take(10) {
        println!("      {}", f);
    }

    let _ = tx.send(());
    let _ = server.await;
    println!("\nD2 done; server shut down cleanly.");
}

/// Every regular file under `root`, as a path string relative to it.
fn walk(root: &std::path::Path) -> Vec<String> {
    let mut out = vec![];
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else {
            continue;
        };
        for e in rd.flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if let Ok(rel) = p.strip_prefix(root) {
                out.push(rel.display().to_string());
            }
        }
    }
    out.sort();
    out
}
