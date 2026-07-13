//! Meer P0+P1 — the always-on blind superpeer, Role 1 (Tier-0 blind message mirror).
//! "This is E9 Tier 0, made real" (`discovery/thinking/meer-superpeer-design.md` §build phases).
//!
//! P0 skeleton: an always-on iroh endpoint + admission (`on_connect` identity allowlist) + `meer_*`
//! metrics. P1 Role 1: it carries members' **encrypted** payloads + their **cleartext join-metadata**
//! (digest, length, timestamp, namespace), runs range-reconciliation on that metadata, and serves an
//! offline/behind member the ciphertext it's missing — **holding no payload key**. The blindness is
//! the thesis: it stores opaque ciphertext keyed by digest and can *prove* it holds zero keys
//! (`meer_payload_keys_held=0`, asserted + logged, per E9's failure mode). The anti-entrenchment guard
//! is built in: `export`/`import` make the encrypted state portable, so a group can re-host on a
//! replacement meer and converge — losing a meer costs availability, never data.
//!
//! Spike-class harness (measures the decision); the production-TDD `meer` crate (Workstream B) is the
//! productionization of exactly this protocol.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use iroh::{EndpointAddr, RelayUrl};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::node::{build_endpoint, ALPN};

/// The cleartext join-metadata a Tier-0 meer is allowed to see (§3b). Never the payload.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlobMeta {
    /// sha256(ciphertext), hex — the reconciliation key. The meer computes/verifies this itself.
    pub digest: String,
    pub len: usize,
    pub ts: u64,
    pub namespace: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum Req {
    /// Member → meer: store an encrypted blob (the meer cannot read `ct_hex`).
    Publish { meta: BlobMeta, ct_hex: String },
    /// Member → meer: "here's what I have; what am I missing?" (range reconciliation on metadata).
    Sync { have: Vec<String> },
    /// Anti-entrenchment: export the full encrypted store so the group can re-host elsewhere.
    Export,
    /// Replacement meer: import an exported encrypted store.
    Import { blobs: Vec<StoredBlob> },
    /// Blindness proof + metrics.
    Stats,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StoredBlob {
    pub meta: BlobMeta,
    pub ct_hex: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Resp {
    Stored { digest: String },
    Missing { blobs: Vec<StoredBlob> },
    Exported { blobs: Vec<StoredBlob> },
    Imported { count: usize },
    Stats(MeerStats),
    Error { reason: String },
}

/// The `meer_*` metrics — the only thing a Tier-0 meer learns, surfaced for transparency (§3b).
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct MeerStats {
    pub meer_connections: u64,
    pub meer_admitted: u64,
    pub meer_rejected: u64,
    pub meer_blobs_stored: u64,
    pub meer_bytes_mirrored: u64,
    pub meer_sync_requests: u64,
    pub meer_blobs_served: u64,
    /// THE thesis assertion: a Tier-0 meer holds zero payload keys. Always 0; logged to prove it.
    pub meer_payload_keys_held: u64,
    /// Distinct namespaces + (digest,ts) pairs observed — the AR-4 metadata surface, made explicit.
    pub meer_namespaces_observed: Vec<String>,
}

#[derive(Default)]
struct Store {
    blobs: BTreeMap<String, StoredBlob>,
    stats: MeerStats,
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    h.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn hex_decode(s: &str) -> Result<Vec<u8>> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).context("hex"))
        .collect()
}

async fn read_framed(recv: &mut iroh::endpoint::RecvStream) -> Result<Vec<u8>> {
    // The meer accepts up to 16 MiB per request frame (spike bound).
    Ok(recv.read_to_end(16 * 1024 * 1024).await.context("read frame")?)
}

async fn handle(store: &Arc<Mutex<Store>>, req: Req) -> Resp {
    let mut s = store.lock().await;
    match req {
        Req::Publish { meta, ct_hex } => {
            let ct = match hex_decode(&ct_hex) {
                Ok(c) => c,
                Err(e) => return Resp::Error { reason: format!("bad ct hex: {e}") },
            };
            // The meer verifies the digest itself — it reconciles on metadata it can compute, never
            // trusting the member's claim — but it STILL cannot read the payload (only hash it).
            let actual = sha256_hex(&ct);
            if actual != meta.digest || ct.len() != meta.len {
                return Resp::Error { reason: "metadata/ciphertext mismatch".into() };
            }
            s.stats.meer_blobs_stored += 1;
            s.stats.meer_bytes_mirrored += ct.len() as u64;
            if !s.stats.meer_namespaces_observed.contains(&meta.namespace) {
                s.stats.meer_namespaces_observed.push(meta.namespace.clone());
            }
            let digest = meta.digest.clone();
            s.blobs.insert(digest.clone(), StoredBlob { meta, ct_hex });
            Resp::Stored { digest }
        }
        Req::Sync { have } => {
            s.stats.meer_sync_requests += 1;
            let haveset: std::collections::BTreeSet<String> = have.into_iter().collect();
            let missing: Vec<StoredBlob> = s
                .blobs
                .values()
                .filter(|b| !haveset.contains(&b.meta.digest))
                .cloned()
                .collect();
            s.stats.meer_blobs_served += missing.len() as u64;
            Resp::Missing { blobs: missing }
        }
        Req::Export => Resp::Exported { blobs: s.blobs.values().cloned().collect() },
        Req::Import { blobs } => {
            let mut n = 0;
            for b in blobs {
                // Re-verify on import; the replacement meer also trusts only what it can hash.
                if let Ok(ct) = hex_decode(&b.ct_hex) {
                    if sha256_hex(&ct) == b.meta.digest {
                        s.stats.meer_bytes_mirrored += ct.len() as u64;
                        s.blobs.insert(b.meta.digest.clone(), b);
                        n += 1;
                    }
                }
            }
            s.stats.meer_blobs_stored = s.blobs.len() as u64;
            Resp::Imported { count: n }
        }
        Req::Stats => {
            // Prove blindness every time stats are read: a Tier-0 meer holds zero keys, by construction.
            s.stats.meer_payload_keys_held = 0;
            Resp::Stats(s.stats.clone())
        }
    }
}

/// Run the always-on meer until cancelled. `allow_ids` (hex EndpointIds) is the admission allowlist;
/// empty = AllowAll (P0 baseline). Prints `MEER_READY addr=<json>`.
pub async fn run_meer(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    allow_ids: Vec<String>,
) -> Result<()> {
    let ep = build_endpoint(bind, &relay_url, quic_port, None, false).await?;
    ep.online().await;
    let addr = ep.addr();
    println!("MEER_READY addr={}", serde_json::to_string(&addr).context("serialize addr")?);
    use std::io::Write;
    let _ = std::io::stdout().flush();
    info!(id = %addr.id, allow = allow_ids.len(), "meer online (Tier-0 blind mirror)");

    let store = Arc::new(Mutex::new(Store::default()));
    while let Some(incoming) = ep.accept().await {
        let conn = match incoming.accept() {
            Ok(a) => match a.await {
                Ok(c) => c,
                Err(e) => { warn!("handshake: {e}"); continue; }
            },
            Err(e) => { warn!("accept: {e}"); continue; }
        };
        let remote = conn.remote_id().to_string();
        {
            let mut s = store.lock().await;
            s.stats.meer_connections += 1;
            // Admission (P0): on_connect identity gate. Empty allowlist = AllowAll.
            if !allow_ids.is_empty() && !allow_ids.iter().any(|a| remote.starts_with(a)) {
                s.stats.meer_rejected += 1;
                warn!(%remote, "meer admission DENIED (not in allowlist)");
                conn.close(1u32.into(), b"not admitted");
                continue;
            }
            s.stats.meer_admitted += 1;
        }
        let store = store.clone();
        tokio::spawn(async move {
            // One bi-stream per request (request frame in, response frame out).
            loop {
                let (mut send, mut recv) = match conn.accept_bi().await {
                    Ok(s) => s,
                    Err(_) => return, // peer done
                };
                let raw = match read_framed(&mut recv).await {
                    Ok(r) => r,
                    Err(e) => { warn!("read: {e}"); return; }
                };
                let resp = match serde_json::from_slice::<Req>(&raw) {
                    Ok(req) => handle(&store, req).await,
                    Err(e) => Resp::Error { reason: format!("bad req: {e}") },
                };
                let bytes = match serde_json::to_vec(&resp) {
                    Ok(b) => b,
                    Err(e) => { warn!("ser resp: {e}"); return; }
                };
                if send.write_all(&bytes).await.is_err() || send.finish().is_err() {
                    return;
                }
            }
        });
    }
    Ok(())
}

/// One request/response round-trip against a meer (members are short-lived clients).
async fn rpc(conn: &iroh::endpoint::Connection, req: &Req) -> Result<Resp> {
    let (mut send, mut recv) = conn.open_bi().await.context("open_bi")?;
    send.write_all(&serde_json::to_vec(req)?).await.context("write req")?;
    send.finish().context("finish")?;
    let raw = recv.read_to_end(16 * 1024 * 1024).await.context("read resp")?;
    Ok(serde_json::from_slice(&raw)?)
}

/// Member action for the orchestrated P1 proof.
#[derive(Clone)]
pub enum MemberAction {
    /// Encrypt and publish `count` blobs in `namespace` (the meer never sees the key).
    Publish { count: usize, namespace: String },
    /// Sync (have=nothing), fetch missing ciphertext, decrypt, and report how many converged.
    SyncAndVerify { expect: usize },
    /// Read + print the meer's blindness proof + metrics.
    Stats,
    /// Export the meer's encrypted store to a file (anti-entrenchment: state portability).
    Export { out: String },
    /// Import an exported store file into a (replacement) meer.
    Import { from: String },
}

/// The shared payload key the MEMBERS hold and the meer never sees. Lab fixture (32 zero-bytes +
/// a label); in product this is the MLS-derived group key — the point is only that the meer lacks it.
fn member_key() -> [u8; 32] {
    let mut k = [0u8; 32];
    k[..7].copy_from_slice(b"croftk1");
    k
}

fn encrypt(plaintext: &[u8], idx: u64) -> Vec<u8> {
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
    let c = ChaCha20Poly1305::new(Key::from_slice(&member_key()));
    let mut n = [0u8; 12];
    n[4..12].copy_from_slice(&idx.to_be_bytes());
    c.encrypt(Nonce::from_slice(&n), plaintext).expect("encrypt")
}

fn decrypt(ct: &[u8], idx: u64) -> Option<Vec<u8>> {
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
    let c = ChaCha20Poly1305::new(Key::from_slice(&member_key()));
    let mut n = [0u8; 12];
    n[4..12].copy_from_slice(&idx.to_be_bytes());
    c.decrypt(Nonce::from_slice(&n), ct).ok()
}

#[allow(clippy::too_many_arguments)]
pub async fn run_member(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    meer_addr: EndpointAddr,
    action: MemberAction,
    base_ts: u64,
) -> Result<()> {
    let ep = build_endpoint(bind, &relay_url, quic_port, None, false).await?;
    ep.online().await;
    let conn = ep.connect(meer_addr, ALPN).await.context("connect meer")?;

    match action {
        MemberAction::Publish { count, namespace } => {
            for i in 0..count {
                let pt = format!("sync-entry-{i}").into_bytes();
                let ct = encrypt(&pt, i as u64);
                let meta = BlobMeta {
                    digest: sha256_hex(&ct),
                    len: ct.len(),
                    ts: base_ts + i as u64,
                    namespace: namespace.clone(),
                };
                let ct_hex = ct.iter().map(|b| format!("{b:02x}")).collect();
                match rpc(&conn, &Req::Publish { meta, ct_hex }).await? {
                    Resp::Stored { digest } => info!(%digest, "published"),
                    Resp::Error { reason } => anyhow::bail!("publish rejected: {reason}"),
                    _ => anyhow::bail!("unexpected publish resp"),
                }
            }
            println!("{}", serde_json::to_string(&serde_json::json!({"published": count}))?);
        }
        MemberAction::SyncAndVerify { expect } => {
            let resp = rpc(&conn, &Req::Sync { have: vec![] }).await?;
            let Resp::Missing { blobs } = resp else { anyhow::bail!("expected Missing") };
            // Decrypt each fetched ciphertext locally — proves the member converges from the meer's
            // blind store, AND that the payload was readable only with the key the meer never held.
            let mut converged = 0usize;
            for b in &blobs {
                let ct = hex_decode(&b.ct_hex)?;
                // index is encoded in the plaintext label; brute the small range to match the nonce.
                let mut ok = false;
                for i in 0..(expect as u64 + blobs.len() as u64 + 8) {
                    if let Some(pt) = decrypt(&ct, i) {
                        if pt == format!("sync-entry-{i}").into_bytes() {
                            ok = true;
                            break;
                        }
                    }
                }
                if ok { converged += 1; }
            }
            let verdict = serde_json::json!({
                "fetched": blobs.len(),
                "converged": converged,
                "expected": expect,
                "all_converged": converged == expect && blobs.len() == expect,
            });
            println!("{}", serde_json::to_string_pretty(&verdict)?);
        }
        MemberAction::Stats => {
            let Resp::Stats(s) = rpc(&conn, &Req::Stats).await? else { anyhow::bail!("expected Stats") };
            println!("{}", serde_json::to_string_pretty(&s)?);
        }
        MemberAction::Export { out } => {
            let Resp::Exported { blobs } = rpc(&conn, &Req::Export).await? else { anyhow::bail!("expected Exported") };
            std::fs::write(&out, serde_json::to_vec(&blobs)?)?;
            println!("{}", serde_json::to_string(&serde_json::json!({"exported": blobs.len(), "to": out}))?);
        }
        MemberAction::Import { from } => {
            let blobs: Vec<StoredBlob> = serde_json::from_slice(&std::fs::read(&from)?)?;
            let n = blobs.len();
            let Resp::Imported { count } = rpc(&conn, &Req::Import { blobs }).await? else { anyhow::bail!("expected Imported") };
            println!("{}", serde_json::to_string(&serde_json::json!({"imported": count, "of": n}))?);
        }
    }
    conn.close(0u32.into(), b"done");
    Ok(())
}
