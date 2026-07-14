//! `IrohGossipBus` — a [`Transport`] over iroh-gossip (feature `iroh-it`).
//!
//! Drop-in behind the same payload-blind port as `SharedDirBus`: the convergence
//! `Replicator` and the shell do not change when this adapter replaces the local
//! one. One bus = one gossip topic (bound at `connect`).
//!
//! Async↔sync bridge: the `Transport` trait is synchronous, iroh is async. Two
//! background tasks bridge the gap — an inbound task drains gossip `Received`
//! events into a sync channel that `drain()` reads, and an outbound task awaits
//! `broadcast` for frames `publish()` queues.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc as std_mpsc;
use std::sync::Arc;

use iroh::address_lookup::memory::MemoryLookup;
use iroh::endpoint::presets;
use iroh::protocol::Router;
use iroh::{Endpoint, EndpointAddr, RelayMode, TransportAddr};
use iroh_gossip::api::Event;
use iroh_gossip::net::Gossip;
use iroh_gossip::proto::TopicId;
use iroh_gossip::ALPN as GOSSIP_ALPN;
use n0_future::StreamExt;
use tokio::sync::mpsc as tokio_mpsc;

use crate::transport::{Frame, Topic, Transport};

// Wire framing for outbound gossip frames. iroh-gossip ids each message by
// `blake3(content)` and keeps a "received" cache (90 s), so re-broadcasting the
// *same* bytes is suppressed as already-seen — which starves a node that joins or
// returns *after* a frame's first broadcast (the X2 catch-up gap).
//
// The mechanism (sync-on-connect): each distinct frame is broadcast **once** in
// steady state (`TAG_LIVE`), reaching whoever is currently connected. When a *new*
// neighbor appears (`Event::NeighborUp`), the retained log is re-broadcast as
// `TAG_RESYNC` with a fresh per-broadcast nonce, so those messages have new ids and
// are delivered to the joiner instead of dedup-suppressed. Cost is O(log) per join
// event, not per tick.
const TAG_LIVE: u8 = 0x00;
const TAG_RESYNC: u8 = 0x01;
/// Length of the resync nonce that follows `TAG_RESYNC`.
const NONCE_LEN: usize = 8;

/// How a bus reaches its peers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelayChoice {
    /// n0's public relays + discovery — the cross-host / real-NAT path
    /// (`relay_mode = "n0"`). Needs outbound Internet reachability.
    N0,
    /// Direct-only over loopback: no relay, no Internet dependency
    /// (`relay_mode = "disabled"`/`"local"`). A same-host multi-process testbed —
    /// the mode to use where Internet UDP/relays are unavailable (a sandbox) or
    /// for fast, hermetic gossip tests.
    LocalDirect,
}

impl RelayChoice {
    /// Map a topology `relay_mode` string to a choice. `"disabled"`/`"local"`/
    /// `"loopback"` select the direct-only path; anything else (incl. `"n0"`)
    /// selects the n0 relays.
    #[must_use]
    pub fn from_relay_mode(mode: &str) -> Self {
        match mode {
            "disabled" | "local" | "loopback" => Self::LocalDirect,
            _ => Self::N0,
        }
    }
}

/// A command to the broadcast task.
enum Out {
    /// Broadcast a frame once (steady state). Re-sends of a frame already broadcast
    /// are skipped — a new neighbor gets it via [`Out::Resync`], not by re-flooding.
    Live(Vec<u8>),
    /// A new neighbor appeared — re-broadcast the retained log (with fresh ids) so it
    /// can catch up on frames first broadcast before it joined.
    Resync,
}

/// Gossip message counters, shared with the bridge tasks. Pure measurement
/// instrumentation (A4 / M1 fan-out) — it observes the wire, it does not change
/// what is broadcast. `live_sent` counts distinct steady-state broadcasts,
/// `resync_sent` counts per-frame connect-time re-broadcasts, `received` counts
/// inbound frames delivered to the fold.
#[derive(Clone, Default)]
struct Counters {
    live_sent: Arc<AtomicU64>,
    resync_sent: Arc<AtomicU64>,
    received: Arc<AtomicU64>,
}

/// A snapshot of a bus's gossip message counts.
#[derive(Clone, Copy, Debug, Default)]
pub struct BusStats {
    /// Distinct frames broadcast once in steady state (`TAG_LIVE`).
    pub live_sent: u64,
    /// Per-frame connect-time re-broadcasts (`TAG_RESYNC`), summed over resync events.
    pub resync_sent: u64,
    /// Inbound frames received off the swarm and handed to the fold.
    pub received: u64,
}

/// A gossip-backed transport bound to a single topic.
pub struct IrohGossipBus {
    endpoint: Endpoint,
    _router: Router,
    inbound_rx: std_mpsc::Receiver<Vec<u8>>,
    outbound_tx: tokio_mpsc::UnboundedSender<Out>,
    counters: Counters,
    _tasks: Vec<tokio::task::JoinHandle<()>>,
}

impl IrohGossipBus {
    /// Bind an endpoint, join `topic`'s gossip swarm (bootstrapping from
    /// `bootstrap` peer addresses), and start the bridge tasks.
    ///
    /// # Errors
    /// Propagates endpoint-bind / gossip-subscribe failures.
    pub async fn connect(
        topic: &Topic,
        bootstrap: Vec<EndpointAddr>,
        relay: RelayChoice,
    ) -> anyhow::Result<Self> {
        // Make bootstrap peers resolvable via an in-memory address lookup so the
        // swarm can dial them (1.0.0 API: MemoryLookup + builder.address_lookup).
        let boot_ids: Vec<_> = bootstrap.iter().map(|a| a.id).collect();
        let mut builder = match relay {
            // n0's public relays + discovery — the cross-host / real-NAT path.
            RelayChoice::N0 => Endpoint::builder(presets::N0),
            // Direct-only over loopback: no relay, no Internet dependency. The
            // `Minimal` preset sets just the crypto provider; the default IPv4
            // bind (0.0.0.0:0) enumerates local addrs including loopback.
            RelayChoice::LocalDirect => {
                Endpoint::builder(presets::Minimal).relay_mode(RelayMode::Disabled)
            }
        };
        if !bootstrap.is_empty() {
            builder = builder.address_lookup(MemoryLookup::from_endpoint_info(bootstrap.clone()));
        }
        let endpoint = builder.bind().await?;
        match relay {
            // Wait for a relay home (this also completes reflexive-addr discovery).
            RelayChoice::N0 => endpoint.online().await,
            // There is no relay home under `Disabled`, so `online()` would block
            // forever (it watches home-relay status). Wait instead until a local
            // direct address is known, so the address we publish is dialable.
            RelayChoice::LocalDirect => wait_for_direct_addrs(&endpoint).await,
        }
        tracing::info!(node_id = %endpoint.id(), ?relay, "iroh endpoint bound");

        let gossip = Gossip::builder().spawn(endpoint.clone());
        let router = Router::builder(endpoint.clone())
            .accept(GOSSIP_ALPN, gossip.clone())
            .spawn();

        let topic_id = TopicId::from_bytes(topic_id_bytes(&topic.0));
        tracing::info!(topic = %topic.0, peers = boot_ids.len(), "gossip subscribing");
        let subscription = gossip.subscribe(topic_id, boot_ids).await?;
        let (sender, mut receiver) = subscription.split();

        let (inbound_tx, inbound_rx) = std_mpsc::channel::<Vec<u8>>();
        let (outbound_tx, mut outbound_rx) = tokio_mpsc::unbounded_channel::<Out>();
        let counters = Counters::default();

        // Inbound: gossip events → sync channel. A `NeighborUp` asks the broadcast
        // task to resync so the newly-connected peer catches up (sync-on-connect).
        let resync_tx = outbound_tx.clone();
        let recv_counter = counters.received.clone();
        let inbound_task = tokio::spawn(async move {
            let mut received: u64 = 0;
            while let Some(event) = receiver.next().await {
                match event {
                    Ok(Event::Received(msg)) => {
                        let Some(frame) = strip_frame(&msg.content) else {
                            tracing::warn!(len = msg.content.len(), "gossip frame malformed; dropping");
                            continue;
                        };
                        received += 1;
                        recv_counter.fetch_add(1, Ordering::Relaxed);
                        tracing::debug!(received, len = frame.len(), "gossip received");
                        if inbound_tx.send(frame).is_err() {
                            break; // bus dropped
                        }
                    }
                    Ok(Event::NeighborUp(id)) => {
                        tracing::info!(peer = %id, "NeighborUp — resync");
                        let _ = resync_tx.send(Out::Resync);
                    }
                    Ok(Event::NeighborDown(id)) => tracing::warn!(peer = %id, "NeighborDown"),
                    Ok(_) => {}
                    Err(e) => {
                        tracing::warn!("gossip receiver error: {e}");
                        break;
                    }
                }
            }
        });

        // Outbound: broadcast each distinct frame ONCE (`TAG_LIVE`) in steady state;
        // on `Resync`, re-broadcast the retained log as `TAG_RESYNC` with fresh
        // nonces (distinct ids) so a joiner receives frames it missed. No per-tick
        // re-flood: the retained set dedups the runtime's periodic re-publish.
        let live_counter = counters.live_sent.clone();
        let resync_counter = counters.resync_sent.clone();
        let outbound_task = tokio::spawn(async move {
            let mut retained: std::collections::HashSet<Vec<u8>> = std::collections::HashSet::new();
            let mut nonce: u64 = 0;
            while let Some(cmd) = outbound_rx.recv().await {
                match cmd {
                    Out::Live(bytes) => {
                        // Already broadcast once → skip; new neighbors get it via Resync.
                        if !retained.insert(bytes.clone()) {
                            continue;
                        }
                        let mut framed = Vec::with_capacity(1 + bytes.len());
                        framed.push(TAG_LIVE);
                        framed.extend_from_slice(&bytes);
                        if let Err(e) = sender.broadcast(framed.into()).await {
                            tracing::warn!("gossip broadcast (live) error: {e}");
                        } else {
                            live_counter.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    Out::Resync => {
                        tracing::debug!(frames = retained.len(), "gossip resync");
                        for bytes in &retained {
                            nonce += 1;
                            let mut framed = Vec::with_capacity(1 + NONCE_LEN + bytes.len());
                            framed.push(TAG_RESYNC);
                            framed.extend_from_slice(&nonce.to_be_bytes());
                            framed.extend_from_slice(bytes);
                            if let Err(e) = sender.broadcast(framed.into()).await {
                                tracing::warn!("gossip broadcast (resync) error: {e}");
                            } else {
                                resync_counter.fetch_add(1, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        });

        Ok(Self {
            endpoint,
            _router: router,
            inbound_rx,
            outbound_tx,
            counters,
            _tasks: vec![inbound_task, outbound_task],
        })
    }

    /// This endpoint's address, for out-of-band exchange with peers.
    #[must_use]
    pub fn endpoint_addr(&self) -> EndpointAddr {
        self.endpoint.addr()
    }

    /// A snapshot of this bus's gossip message counts (A4 / M1 fan-out measurement).
    #[must_use]
    pub fn stats(&self) -> BusStats {
        BusStats {
            live_sent: self.counters.live_sent.load(Ordering::Relaxed),
            resync_sent: self.counters.resync_sent.load(Ordering::Relaxed),
            received: self.counters.received.load(Ordering::Relaxed),
        }
    }
}

impl Transport for IrohGossipBus {
    fn subscribe(&mut self, _topic: &Topic) {
        // Subscription is established at `connect` (one topic per bus); the port's
        // per-call topic is ignored here.
    }

    fn publish(&mut self, _topic: &Topic, frame: Frame) {
        if self.outbound_tx.send(Out::Live(frame.0)).is_err() {
            tracing::warn!("iroh outbound channel closed; frame dropped");
        }
    }

    fn drain(&mut self) -> Vec<Frame> {
        let mut frames = Vec::new();
        while let Ok(bytes) = self.inbound_rx.try_recv() {
            frames.push(Frame(bytes));
        }
        frames
    }
}

/// Strip the wire tag (and the resync nonce) from an inbound gossip frame,
/// returning the inner frame bytes. `None` if the frame is too short or the tag
/// is unknown.
fn strip_frame(content: &[u8]) -> Option<Vec<u8>> {
    match content.first().copied()? {
        TAG_LIVE => Some(content.get(1..)?.to_vec()),
        TAG_RESYNC => Some(content.get(1 + NONCE_LEN..)?.to_vec()),
        _ => None,
    }
}

/// Poll until the endpoint reports at least one IP (direct) address, so the
/// `EndpointAddr` we hand to a peer is dialable without a relay. Bounded (~5 s) so
/// a misconfigured bind fails observably instead of hanging like `online()` would.
async fn wait_for_direct_addrs(endpoint: &Endpoint) {
    for _ in 0..100 {
        if endpoint
            .addr()
            .addrs
            .iter()
            .any(|a| matches!(a, TransportAddr::Ip(_)))
        {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
    tracing::warn!("no direct addresses after ~5s; publishing a relay-less address anyway");
}

/// Derive a stable 32-byte `TopicId` from a topic string (four FNV-1a streams).
/// Deterministic, so peers using the same `Topic` string join the same swarm.
fn topic_id_bytes(topic: &str) -> [u8; 32] {
    const SEEDS: [u64; 4] = [
        0xcbf2_9ce4_8422_2325,
        0x9e37_79b9_7f4a_7c15,
        0xff51_afd7_ed55_8ccd,
        0xc4ce_b9fe_1a85_ec53,
    ];
    let mut out = [0u8; 32];
    for (i, seed) in SEEDS.iter().enumerate() {
        let mut h = *seed;
        for b in topic.as_bytes() {
            h ^= u64::from(*b);
            h = h.wrapping_mul(0x0000_0100_0000_01b3);
        }
        out[i * 8..i * 8 + 8].copy_from_slice(&h.to_be_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topic_id_is_deterministic_and_distinct() {
        assert_eq!(topic_id_bytes("a"), topic_id_bytes("a"));
        assert_ne!(topic_id_bytes("a"), topic_id_bytes("b"));
    }

    // Network integration test: two in-process endpoints exchange a frame over
    // gossip. Gated so default `cargo test` (no feature) never binds a socket.
    #[tokio::test]
    async fn two_endpoints_exchange_a_frame() {
        crate::init_tracing();
        let topic = Topic("croft-chat/iroh-it".to_string());

        // SPEC-DELTA[hermetic-gossip | test-hermeticization]: loopback gossip only,
        // not the relay path (see the register). Register: alpha/experiments/SPEC-DIVERGENCE-REGISTER.md
        // Direct-only over loopback: hermetic (no relay / Internet dependency).
        let mut a = IrohGossipBus::connect(&topic, vec![], RelayChoice::LocalDirect)
            .await
            .expect("bus a");
        let a_addr = a.endpoint_addr();
        let mut b = IrohGossipBus::connect(&topic, vec![a_addr], RelayChoice::LocalDirect)
            .await
            .expect("bus b");

        // Give the swarm a moment to form, then A broadcasts.
        for _ in 0..50 {
            a.publish(&topic, Frame(b"hello over gossip".to_vec()));
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            let got = b.drain();
            if got.iter().any(|f| f.0 == b"hello over gossip") {
                return; // delivered
            }
        }
        panic!("frame not delivered over gossip within the timeout");
    }
}
