//! E3 — namespace-sharded fan-out sync (the core thesis).
//!
//! A real eventual-sync workload over iroh, forced relay-only: members of a namespace are all
//! co-located on one relay shard and converge a shared Automerge document by meeting a local peer
//! (the rendezvous hub) — no point-to-point arrangement between members. The claim under test:
//! within a correctly-sharded namespace, every member converges and the relay's dropped-packet
//! count stays ~0 (no co-location miss).
//!
//! `sync-hub`    — holds the shared doc, accepts sync sessions, merges everyone (the rendezvous).
//! `sync-member` — contributes one entry, runs an Automerge sync session with the hub, reports its
//!                 converged key count. Run twice (contribute pass, then pull pass) for full convergence.
//!
//! Automerge sync protocol verified against automerge 0.10.0 source (sync.rs / autocommit.rs).

use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{Context, Result};
use automerge::sync::{Message, State, SyncDoc};
use automerge::transaction::Transactable;
use automerge::{AutoCommit, ReadDoc, ROOT};
use iroh::endpoint::Connection;
use iroh::RelayUrl;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::node::build_endpoint;

/// One Automerge anti-entropy session over a single iroh bi-stream. Both sides run this same loop:
/// generate→send (empty frame = no message), read peer frame, receive; terminate when both sides
/// send an empty frame in the same round. QUIC bi-streams are full-duplex so write-then-read by both
/// sides does not deadlock.
async fn sync_session(doc: &Mutex<AutoCommit>, conn: &Connection, initiator: bool) -> Result<()> {
    let (mut send, mut recv) = if initiator {
        conn.open_bi().await.context("open_bi")?
    } else {
        conn.accept_bi().await.context("accept_bi")?
    };
    let mut state = State::new();
    loop {
        let out = {
            let mut d = doc.lock().await;
            let msg = d.sync().generate_sync_message(&mut state).map(|m| m.encode());
            msg
        };
        let bytes = out.unwrap_or_default();
        write_frame(&mut send, &bytes).await?;
        let peer = read_frame(&mut recv).await?;
        if !peer.is_empty() {
            let msg = Message::decode(&peer).context("decode sync msg")?;
            let mut d = doc.lock().await;
            d.sync()
                .receive_sync_message(&mut state, msg)
                .context("receive sync msg")?;
        }
        if bytes.is_empty() && peer.is_empty() {
            break;
        }
    }
    let _ = send.finish();
    Ok(())
}

async fn write_frame(send: &mut iroh::endpoint::SendStream, payload: &[u8]) -> Result<()> {
    let len = (payload.len() as u32).to_be_bytes();
    send.write_all(&len).await.context("write len")?;
    if !payload.is_empty() {
        send.write_all(payload).await.context("write payload")?;
    }
    Ok(())
}

async fn read_frame(recv: &mut iroh::endpoint::RecvStream) -> Result<Vec<u8>> {
    let mut len = [0u8; 4];
    recv.read_exact(&mut len).await.context("read len")?;
    let n = u32::from_be_bytes(len) as usize;
    if n == 0 {
        return Ok(Vec::new());
    }
    let mut buf = vec![0u8; n];
    recv.read_exact(&mut buf).await.context("read payload")?;
    Ok(buf)
}

#[derive(Debug, Serialize)]
pub struct MemberResult {
    pub index: usize,
    pub converged_keys: usize,
    pub expected: usize,
    pub converged: bool,
    pub heads: usize,
}

/// Run the rendezvous hub: hold the shared doc, accept sync sessions forever, merge everyone.
/// Prints `SYNC_HUB_ADDR=<json>` once online and `SYNC_HUB_KEYS=<n>` on each session completion.
pub async fn run_hub(bind: SocketAddr, relay_url: RelayUrl, quic_port: Option<u16>) -> Result<()> {
    let ep = build_endpoint(bind, &relay_url, quic_port, None, false).await?;
    ep.online().await;
    let addr = ep.addr();
    println!("SYNC_HUB_ADDR={}", serde_json::to_string(&addr)?);
    use std::io::Write;
    let _ = std::io::stdout().flush();

    let doc = Arc::new(Mutex::new(AutoCommit::new()));
    while let Some(incoming) = ep.accept().await {
        let doc = doc.clone();
        tokio::spawn(async move {
            let conn = match incoming.accept() {
                Ok(a) => match a.await {
                    Ok(c) => c,
                    Err(_) => return,
                },
                Err(_) => return,
            };
            // Loop sync sessions per connection — a member re-syncs (a fresh bi-stream each time)
            // via its anti-entropy loop until it holds the whole namespace. accept_bi erroring means
            // the member closed the connection (converged or gave up).
            loop {
                match sync_session(&doc, &conn, false).await {
                    Ok(()) => {
                        let keys = {
                            let d = doc.lock().await;
                            d.keys(ROOT).count()
                        };
                        println!("SYNC_HUB_KEYS={keys}");
                        let _ = std::io::stdout().flush();
                    }
                    Err(_) => break,
                }
            }
        });
    }
    Ok(())
}

/// Run one member: contribute entry `p{index}=index`, sync once with the hub, report converged keys.
/// `relay_only` forces the connection onto the relay (the sharded-namespace condition).
pub async fn run_member(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    hub: iroh::EndpointAddr,
    index: usize,
    population: usize,
    relay_only: bool,
) -> Result<MemberResult> {
    let ep = build_endpoint(bind, &relay_url, quic_port, None, relay_only).await?;
    let doc = Mutex::new(AutoCommit::new());
    {
        let mut d = doc.lock().await;
        d.put(ROOT, format!("p{index}"), index as i64)
            .context("put own entry")?;
    }
    let conn = ep.connect(hub, crate::node::ALPN).await.context("connect hub")?;
    // Anti-entropy loop: re-sync with the hub until we hold the whole namespace (every member's
    // entry) or we run out of attempts. A single session converges us to the hub's *current* state;
    // because members contribute concurrently, the hub fills over time, so we retry until complete.
    let max_attempts = (population + 8).max(20);
    let mut keys = 0;
    for _ in 0..max_attempts {
        // Tolerate a transient session error (e.g. a racing concurrent peer) and retry.
        if let Err(e) = sync_session(&doc, &conn, true).await {
            tracing::warn!("member {index} sync attempt: {e:#}");
        }
        keys = {
            let d = doc.lock().await;
            d.keys(ROOT).count()
        };
        if keys >= population {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
    let heads = {
        let mut d = doc.lock().await;
        d.get_heads().len()
    };
    ep.close().await;
    Ok(MemberResult {
        index,
        converged_keys: keys,
        expected: population,
        converged: keys >= population,
        heads,
    })
}
