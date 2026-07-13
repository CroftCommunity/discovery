//! Throwaway Phase 0 spike (B-gossip) — iroh-gossip mesh, ONE node per host.
//!
//! Each node joins a shared `TopicId`, bootstrapping from zero or more peer
//! `EndpointAddr`s (passed as JSON files, relayed between hosts). It broadcasts a
//! tagged message every 2 s and records which *senders* it received from. This
//! lets us test:
//!   * transitive delivery — node-1 and node-3 bootstrap only via node-2, yet
//!     each receives the other's broadcasts (epidemic broadcast across the mesh);
//!   * drop-a-node resilience — kill the relaying node and see if delivery holds.
//!
//! Bootstrap mirrors the blob spike's NAT fix: the peer's full `EndpointAddr`
//! (relay URL + addrs) is seeded into an in-memory address lookup, then its
//! `EndpointId` is passed to `subscribe`.
//!
//! Usage:
//!   altdrive-spike-gossip <name> <topic-hex64> <self-addr-out> [<bootstrap-addr.json> ...]

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use iroh::{
    address_lookup::MemoryLookup, endpoint::presets, protocol::Router, Endpoint, EndpointAddr,
    EndpointId,
};
use iroh_gossip::{api::Event, net::Gossip, proto::TopicId, ALPN as GOSSIP_ALPN};
use n0_future::StreamExt;

/// Gossip binds UDP 2112 — the only port the sandbox Security Group opens
/// among the boxes (gossip runs standalone here, so it doesn't clash with the
/// blob spike which also uses 2112).
const PORT: u16 = 2112;
/// Broadcast rounds (×2 s) — long enough for the mesh to form and converge.
const ROUNDS: usize = 18;

fn parse_topic(s: &str) -> Result<TopicId> {
    if s.len() != 64 {
        return Err(anyhow!("topic must be 64 hex chars (32 bytes), got {}", s.len()));
    }
    let mut arr = [0u8; 32];
    for (i, b) in arr.iter_mut().enumerate() {
        *b = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).context("bad hex in topic")?;
    }
    Ok(TopicId::from_bytes(arr))
}

/// Extract the sender name from a "hello-from-<name>-<round>" message.
fn sender_of(content: &str) -> Option<String> {
    content.strip_prefix("hello-from-").map(|rest| {
        rest.rsplit_once('-').map(|(name, _)| name.to_string()).unwrap_or_else(|| rest.to_string())
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: altdrive-spike-gossip <name> <topic-hex64> <self-addr-out> [<bootstrap.json> ...]");
        std::process::exit(2);
    }
    let name = args[0].clone();
    let topic = parse_topic(&args[1])?;
    let self_out = &args[2];
    let bootstrap_files = &args[3..];

    // Load bootstrap peer addresses (full EndpointAddr w/ relay) from JSON files.
    let mut boot_addrs: Vec<EndpointAddr> = Vec::new();
    for f in bootstrap_files {
        let s = std::fs::read_to_string(f).with_context(|| format!("read {f}"))?;
        boot_addrs.push(serde_json::from_str(&s).with_context(|| format!("parse {f}"))?);
    }
    let boot_ids: Vec<EndpointId> = boot_addrs.iter().map(|a| a.id).collect();

    // Endpoint, seeded with bootstrap addresses so id→relay resolves immediately.
    let builder = Endpoint::builder(presets::N0)
        .clear_ip_transports()
        .bind_addr(format!("0.0.0.0:{PORT}"))
        .context("bind UDP/v4 2113")?;
    let builder = if boot_addrs.is_empty() {
        builder
    } else {
        builder.address_lookup(MemoryLookup::from_endpoint_info(boot_addrs.clone()))
    };
    let endpoint = builder.bind().await.map_err(|e| anyhow!("endpoint bind: {e}"))?;
    endpoint.online().await;

    // Publish our dialable address so other hosts can bootstrap from us.
    let my_addr = endpoint.addr();
    std::fs::write(self_out, serde_json::to_string(&my_addr)?)
        .with_context(|| format!("write {self_out}"))?;
    println!("[{name}] id={} addr->{self_out} bootstrap={:?}", endpoint.id(), boot_ids);

    let gossip = Gossip::builder().spawn(endpoint.clone());
    let _router = Router::builder(endpoint.clone()).accept(GOSSIP_ALPN, gossip.clone()).spawn();

    let topic_sub = gossip.subscribe(topic, boot_ids).await?;
    let (sender, mut receiver) = topic_sub.split();

    // Receiver task — record the distinct senders we hear from.
    let seen: Arc<Mutex<BTreeSet<String>>> = Arc::new(Mutex::new(BTreeSet::new()));
    let neighbors: Arc<Mutex<BTreeSet<String>>> = Arc::new(Mutex::new(BTreeSet::new()));
    let (seen_r, nb_r, rname) = (seen.clone(), neighbors.clone(), name.clone());
    tokio::spawn(async move {
        while let Some(ev) = receiver.next().await {
            match ev {
                Ok(Event::NeighborUp(id)) => {
                    nb_r.lock().expect("lock").insert(id.fmt_short().to_string());
                    println!("[{rname}] NeighborUp {}", id.fmt_short());
                }
                Ok(Event::NeighborDown(id)) => {
                    println!("[{rname}] NeighborDown {}", id.fmt_short());
                }
                Ok(Event::Received(msg)) => {
                    let content = String::from_utf8_lossy(&msg.content).to_string();
                    if let Some(s) = sender_of(&content) {
                        seen_r.lock().expect("lock").insert(s);
                    }
                    println!("[{rname}] RECV '{content}' via {}", msg.delivered_from.fmt_short());
                }
                Ok(Event::Lagged) => println!("[{rname}] lagged"),
                Err(e) => {
                    println!("[{rname}] recv error: {e}");
                    break;
                }
            }
        }
    });

    // Broadcast loop.
    for i in 0..ROUNDS {
        tokio::time::sleep(Duration::from_secs(2)).await;
        let m = format!("hello-from-{name}-{i}");
        if let Err(e) = sender.broadcast(m.into_bytes().into()).await {
            println!("[{name}] broadcast round {i} error: {e}");
        }
    }
    tokio::time::sleep(Duration::from_secs(2)).await;

    let seen_final = seen.lock().expect("lock").clone();
    let nb_final = neighbors.lock().expect("lock").clone();
    println!("[{name}] SUMMARY senders_received={:?} neighbors_seen={:?}", seen_final, nb_final);
    Ok(())
}
