//! E0 load generator: create N independent endpoints (each its own node key = one relay *client*
//! connection — the unit the relay accounts and the memory wall is measured against), connect each
//! to a responder, and classify the live (selected) path as relay or direct, with RTT.
//!
//! - `matchmaking`: dial the responder's full advertised address (relay + direct candidates) and
//!   allow hole-punching; record which path each connection settles on.
//! - `passthrough`: build each endpoint with IP transports cleared so the connection is forced to
//!   stay on the relay — the worst-case relay wall.
//!
//! After establishing all endpoints the generator holds them open (`--hold-secs`) so the
//! orchestrator can sample the relay's RSS/CPU with N client connections live.

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use iroh::endpoint::Connection;
use iroh::{Endpoint, EndpointAddr, RelayUrl};
use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;
use tokio::time::{Instant, sleep};

use crate::node::{ALPN, build_endpoint};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Matchmaking,
    Passthrough,
}

impl Mode {
    /// Passthrough forces relay-only by clearing the endpoint's IP transports.
    fn relay_only(self) -> bool {
        matches!(self, Mode::Passthrough)
    }
}

impl std::str::FromStr for Mode {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "matchmaking" => Ok(Mode::Matchmaking),
            "passthrough" => Ok(Mode::Passthrough),
            other => anyhow::bail!("unknown mode {other:?} (matchmaking|passthrough)"),
        }
    }
}

/// Parameters for one generation run.
pub struct GenParams {
    pub mode: Mode,
    /// Number of independent endpoints to create (= relay client connections).
    pub count: usize,
    pub concurrency: usize,
    pub bind: SocketAddr,
    pub relay_url: RelayUrl,
    pub quic_port: Option<u16>,
    pub advertised: EndpointAddr,
    /// Bytes streamed per connection after the path settles (0 = ping only / idle).
    pub bytes: usize,
    /// How long to hold all endpoints open after establishment (idle-wall sampling window).
    pub hold_secs: u64,
    /// Max time to wait for the path to settle (matchmaking hole-punch) before recording.
    pub settle_ms: u64,
}

/// Immutable per-unit context shared across the spawned connect tasks.
struct Ctx {
    mode: Mode,
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    advertised: EndpointAddr,
    bytes: usize,
    settle_ms: u64,
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct PathStat {
    pub count: usize,
    pub rtt_ms_min: f64,
    pub rtt_ms_p50: f64,
    pub rtt_ms_mean: f64,
    pub rtt_ms_max: f64,
}

impl PathStat {
    fn from_samples(mut s: Vec<f64>) -> Self {
        if s.is_empty() {
            return Self::default();
        }
        s.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = s.len();
        let sum: f64 = s.iter().sum();
        Self {
            count: n,
            rtt_ms_min: s[0],
            rtt_ms_p50: s[n / 2],
            rtt_ms_mean: sum / n as f64,
            rtt_ms_max: s[n - 1],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GenSummary {
    pub mode: &'static str,
    pub endpoints: usize,
    pub established: usize,
    pub failed: usize,
    pub live_relay: usize,
    pub live_direct: usize,
    pub live_unknown: usize,
    pub relay: PathStat,
    pub direct: PathStat,
    pub bytes_per_conn: usize,
    pub total_bytes_streamed: u64,
    pub establish_ms: u128,
    pub hold_secs: u64,
}

struct ConnResult {
    ok: bool,
    is_relay: Option<bool>,
    rtt_ms: Option<f64>,
    streamed: u64,
}

/// Build the dial address for the chosen mode from the responder's advertised address.
fn dial_addr(mode: Mode, advertised: &EndpointAddr, relay_url: &RelayUrl) -> EndpointAddr {
    match mode {
        Mode::Matchmaking => advertised.clone(),
        Mode::Passthrough => EndpointAddr::new(advertised.id).with_relay_url(relay_url.clone()),
    }
}

/// Poll the connection's paths until it settles on a direct path or `settle` elapses.
async fn settle_path(conn: &Connection, settle: Duration) -> (Option<bool>, Option<f64>) {
    let deadline = Instant::now() + settle;
    let mut last = (None, None);
    loop {
        if let Some(p) = conn.paths().iter().find(|p| p.is_selected()) {
            last = (Some(p.is_relay()), Some(p.rtt().as_secs_f64() * 1000.0));
            if p.is_ip() {
                break;
            }
        }
        if Instant::now() >= deadline {
            break;
        }
        sleep(Duration::from_millis(50)).await;
    }
    last
}

/// Drive one endpoint: build it, connect, roundtrip to trigger pathing, settle, optional bulk stream.
async fn run_one(ctx: &Ctx) -> Result<(ConnResult, Endpoint, Connection)> {
    let ep = build_endpoint(
        ctx.bind,
        &ctx.relay_url,
        ctx.quic_port,
        None,
        ctx.mode.relay_only(),
    )
    .await?;
    let dial = dial_addr(ctx.mode, &ctx.advertised, &ctx.relay_url);
    let conn = ep.connect(dial, ALPN).await.context("connect")?;

    roundtrip(&conn, 8).await.context("ping roundtrip")?;
    let (is_relay, rtt_ms) = settle_path(&conn, Duration::from_millis(ctx.settle_ms)).await;

    let mut streamed = 0u64;
    if ctx.bytes > 0 {
        roundtrip(&conn, ctx.bytes).await.context("bulk roundtrip")?;
        streamed = ctx.bytes as u64;
    }

    Ok((
        ConnResult {
            ok: true,
            is_relay,
            rtt_ms,
            streamed,
        },
        ep,
        conn,
    ))
}

/// Open a bi-stream, send `n` bytes, and read the responder's echo back.
async fn roundtrip(conn: &Connection, n: usize) -> Result<()> {
    let (mut send, mut recv) = conn.open_bi().await.context("open_bi")?;
    let payload = vec![0u8; n];
    send.write_all(&payload).await.context("write")?;
    send.finish().context("finish")?;
    let echoed = recv.read_to_end(n + 16).await.context("read echo")?;
    anyhow::ensure!(echoed.len() == n, "echo len {} != {}", echoed.len(), n);
    Ok(())
}

/// Run the full generation and return the summary.
pub async fn run(params: GenParams) -> Result<GenSummary> {
    let ctx = Arc::new(Ctx {
        mode: params.mode,
        bind: params.bind,
        relay_url: params.relay_url,
        quic_port: params.quic_port,
        advertised: params.advertised,
        bytes: params.bytes,
        settle_ms: params.settle_ms,
    });
    let sem = Arc::new(Semaphore::new(params.concurrency.max(1)));
    let mut set: JoinSet<Result<(ConnResult, Endpoint, Connection)>> = JoinSet::new();

    let start = Instant::now();
    for _ in 0..params.count {
        let permit = sem.clone().acquire_owned().await.expect("semaphore");
        let ctx = ctx.clone();
        set.spawn(async move {
            let r = run_one(&ctx).await;
            drop(permit);
            r
        });
    }

    let mut results: Vec<ConnResult> = Vec::with_capacity(params.count);
    // Hold the live endpoints + connections open so the relay's RSS is sampleable during the window.
    let mut held: Vec<(Endpoint, Connection)> = Vec::with_capacity(params.count);
    while let Some(joined) = set.join_next().await {
        match joined {
            Ok(Ok((res, ep, conn))) => {
                results.push(res);
                held.push((ep, conn));
            }
            Ok(Err(e)) => {
                tracing::warn!("endpoint failed: {e:#}");
                results.push(ConnResult {
                    ok: false,
                    is_relay: None,
                    rtt_ms: None,
                    streamed: 0,
                });
            }
            Err(e) => {
                tracing::warn!("task panicked: {e}");
                results.push(ConnResult {
                    ok: false,
                    is_relay: None,
                    rtt_ms: None,
                    streamed: 0,
                });
            }
        }
    }
    let establish_ms = start.elapsed().as_millis();

    let established = results.iter().filter(|r| r.ok).count();
    let failed = results.len() - established;
    let mut relay_rtts = Vec::new();
    let mut direct_rtts = Vec::new();
    let (mut live_relay, mut live_direct, mut live_unknown) = (0, 0, 0);
    let mut total_bytes = 0u64;
    for r in &results {
        total_bytes += r.streamed;
        match r.is_relay {
            Some(true) => {
                live_relay += 1;
                if let Some(rtt) = r.rtt_ms {
                    relay_rtts.push(rtt);
                }
            }
            Some(false) => {
                live_direct += 1;
                if let Some(rtt) = r.rtt_ms {
                    direct_rtts.push(rtt);
                }
            }
            None => {
                if r.ok {
                    live_unknown += 1;
                }
            }
        }
    }

    let summary = GenSummary {
        mode: match params.mode {
            Mode::Matchmaking => "matchmaking",
            Mode::Passthrough => "passthrough",
        },
        endpoints: params.count,
        established,
        failed,
        live_relay,
        live_direct,
        live_unknown,
        relay: PathStat::from_samples(relay_rtts),
        direct: PathStat::from_samples(direct_rtts),
        bytes_per_conn: params.bytes,
        total_bytes_streamed: total_bytes,
        establish_ms,
        hold_secs: params.hold_secs,
    };

    // Emit an upfront marker so a sampler can confirm establishment during the hold window
    // (the full JSON summary is only printed by main after the hold completes).
    eprintln!(
        "ESTABLISHED={established} relay={live_relay} direct={live_direct} failed={failed} establish_ms={establish_ms}"
    );

    if params.hold_secs > 0 {
        tracing::info!("holding {} endpoints for {}s", held.len(), params.hold_secs);
        sleep(Duration::from_secs(params.hold_secs)).await;
    }

    // Graceful shutdown so the relay sees clean disconnects.
    for (ep, _conn) in held.drain(..) {
        ep.close().await;
    }
    Ok(summary)
}
