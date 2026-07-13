//! Croft Relay & Placement Lab harness — entry point.
//!
//! Spike-class throwaway lab tooling (TDD-exempt, same class as `altdrive-spike-*` and the
//! reconcile/history harnesses): its job is to *measure* relay/meer behaviour, not to ship.
//! Subcommands are added in the spec §5 dependency order. E0 ships: `relay`, `responder`,
//! `generate`. All iroh APIs verified against pinned 1.0.0 source (`IROH-1.0.0-API-VERIFIED.md`).

mod e2;
mod generate;
mod meer;
mod node;
mod relay;
mod responder;
mod roq;
mod sync;

use std::net::{IpAddr, SocketAddr};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iroh::{EndpointAddr, RelayUrl};

use crate::generate::{GenParams, Mode};
use crate::relay::RelayPorts;

#[derive(Parser)]
#[command(about = "Croft relay & placement lab harness (E0+)")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Spawn a single iroh relay (self-signed TLS, AllowAll). Runs until Ctrl-C.
    Relay {
        /// IP to bind the relay sockets on (0.0.0.0 for all interfaces).
        #[arg(long, default_value = "0.0.0.0")]
        bind_ip: IpAddr,
        /// IP to advertise to clients (the relay's reachable address).
        #[arg(long)]
        advertise_ip: IpAddr,
        #[arg(long, default_value_t = 3340)]
        http_port: u16,
        #[arg(long, default_value_t = 3343)]
        https_port: u16,
        #[arg(long, default_value_t = 3478)]
        quic_port: u16,
        #[arg(long, default_value_t = 9090)]
        metrics_port: u16,
    },
    /// Run a responder homed on the relay; prints `RESPONDER_ADDR=<json>` and echoes bi-streams.
    Responder {
        #[arg(long, default_value = "0.0.0.0:2112")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        /// Relay QUIC port (omit to use websocket transport to the relay).
        #[arg(long)]
        quic_port: Option<u16>,
        /// Pin the endpoint secret key (64 hex chars = 32 bytes) so the endpoint id
        /// is stable across restarts — used by E7 to re-home the *same* peer on a
        /// different relay. Omit for a fresh random key.
        #[arg(long)]
        secret: Option<String>,
    },
    /// Open connections to a responder and classify the live path (relay vs direct).
    Generate {
        #[arg(long, default_value = "0.0.0.0:0")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Responder address as JSON (from its `RESPONDER_ADDR=` line), or `@/path/to/file`.
        #[arg(long)]
        responder_addr: String,
        #[arg(long, default_value = "matchmaking")]
        mode: String,
        #[arg(long, default_value_t = 10)]
        count: usize,
        #[arg(long, default_value_t = 16)]
        concurrency: usize,
        /// Bytes streamed per connection after the path settles (0 = ping only).
        #[arg(long, default_value_t = 0)]
        bytes: usize,
        /// Hold all connections open this long after establishment (idle-RSS sampling window).
        #[arg(long, default_value_t = 0)]
        hold_secs: u64,
        /// Max wait for the path to settle (hole-punch) before recording.
        #[arg(long, default_value_t = 3000)]
        settle_ms: u64,
    },
    /// E2: connect to a peer by bare id, resolving its relay through a controller-assigned record
    /// (in-memory MemoryLookup). Prints which relay the live path used. Endpoint is relay-only.
    E2Connect {
        #[arg(long, default_value = "0.0.0.0:0")]
        bind: SocketAddr,
        /// Target peer's endpoint id (hex/z32).
        #[arg(long)]
        peer_id: String,
        /// The relay the controller assigned this peer to (seeded into the lookup).
        #[arg(long)]
        assign_relay: String,
        /// All relays the endpoint knows (comma-separated URLs).
        #[arg(long, value_delimiter = ',')]
        relays: Vec<String>,
        #[arg(long)]
        quic_port: Option<u16>,
        #[arg(long, default_value_t = 15)]
        timeout_secs: u64,
    },
    /// E3: rendezvous hub holding the shared Automerge doc; accepts sync sessions. Runs until Ctrl-C.
    SyncHub {
        #[arg(long, default_value = "0.0.0.0:2112")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
    },
    /// Meer P0+P1: always-on Tier-0 blind message mirror (holds no payload key). Runs until Ctrl-C.
    Meer {
        #[arg(long, default_value = "0.0.0.0:2120")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Admission allowlist: EndpointId hex prefixes that may connect (empty = AllowAll).
        #[arg(long, value_delimiter = ',')]
        allow: Vec<String>,
    },
    /// Meer client: publish encrypted blobs / sync+verify / read stats / export / import.
    MeerMember {
        #[arg(long, default_value = "0.0.0.0:0")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Meer address as JSON (its `MEER_READY addr=` line), or `@/path/to/file`.
        #[arg(long)]
        meer_addr: String,
        /// One of: publish | sync | stats | export | import
        #[arg(long)]
        action: String,
        #[arg(long, default_value_t = 5)]
        count: usize,
        #[arg(long, default_value = "household-v1")]
        namespace: String,
        #[arg(long, default_value_t = 5)]
        expect: usize,
        /// File path for export/import.
        #[arg(long, default_value = "/tmp/meer-export.json")]
        file: String,
    },
    /// E10: RoQ receiver — accept a connection and tally datagram loss/jitter/goodput/gaps.
    RoqRecv {
        #[arg(long, default_value = "0.0.0.0:2112")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Finalize after this long with no datagram (sender close / link death).
        #[arg(long, default_value_t = 3000)]
        idle_ms: u64,
    },
    /// E10: RoQ sender — emit CBR datagrams at a target bitrate; sample sender-side CC signals.
    RoqSend {
        #[arg(long, default_value = "0.0.0.0:0")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Receiver address as JSON (its `ROQ_RECV_ADDR=` line), or `@/path/to/file`.
        #[arg(long)]
        recv_addr: String,
        /// Target media bitrate (kbps). Opus voice ≈ 32–64; music ≈ 128.
        #[arg(long, default_value_t = 64)]
        kbps: u64,
        /// Frame cadence (ms). Opus default packetization = 20.
        #[arg(long, default_value_t = 20)]
        frame_ms: u64,
        #[arg(long, default_value_t = 30)]
        duration_secs: u64,
        /// Force relay-only (clear IP transports) — the meer-relayed media case.
        #[arg(long, default_value_t = false)]
        relay_only: bool,
        /// Datagram send-buffer size (bytes). Omit for iroh default (~1 MiB). Small values make
        /// QUIC drop over-cap datagrams (send_datagram errs) instead of bufferbloating.
        #[arg(long)]
        send_buf_bytes: Option<usize>,
        /// Enable the delay-based AIMD bitrate estimator (back off when path-RTT climbs above baseline).
        #[arg(long, default_value_t = false)]
        adaptive: bool,
        /// Floor bitrate (kbps) the estimator won't drop below.
        #[arg(long, default_value_t = 8)]
        min_kbps: u64,
        /// RTT budget over baseline (ms) that triggers multiplicative-decrease.
        #[arg(long, default_value_t = 50.0)]
        rtt_budget_ms: f64,
        /// TC-CC3: also push this many bytes on a concurrent BULK reliable stream (0 = media only).
        #[arg(long, default_value_t = 0)]
        bulk_bytes: u64,
    },
    /// E3: one namespace member — contributes an entry, syncs with the hub, reports converged keys.
    SyncMember {
        #[arg(long, default_value = "0.0.0.0:0")]
        bind: SocketAddr,
        #[arg(long)]
        relay_url: String,
        #[arg(long)]
        quic_port: Option<u16>,
        /// Hub address as JSON (its `SYNC_HUB_ADDR=` line), or `@/path/to/file`.
        #[arg(long)]
        hub_addr: String,
        #[arg(long)]
        index: usize,
        #[arg(long)]
        population: usize,
        /// Force relay-only (the sharded-namespace condition). Default true.
        #[arg(long, default_value_t = true)]
        relay_only: bool,
    },
}

fn parse_relay_url(s: &str) -> Result<RelayUrl> {
    s.parse().with_context(|| format!("invalid relay url {s:?}"))
}

fn load_responder_addr(s: &str) -> Result<EndpointAddr> {
    let json = if let Some(path) = s.strip_prefix('@') {
        std::fs::read_to_string(path).with_context(|| format!("reading {path}"))?
    } else {
        s.to_string()
    };
    serde_json::from_str(json.trim()).context("parsing responder EndpointAddr JSON")
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    match Cli::parse().cmd {
        Cmd::Relay {
            bind_ip,
            advertise_ip,
            http_port,
            https_port,
            quic_port,
            metrics_port,
        } => {
            let ports = RelayPorts {
                http: http_port,
                https: https_port,
                quic: quic_port,
                metrics: metrics_port,
            };
            let (server, url) = relay::spawn(bind_ip, advertise_ip, ports).await?;
            println!(
                "RELAY_READY url={url} https={:?} quic={:?}",
                server.https_addr(),
                server.quic_addr()
            );
            use std::io::Write;
            let _ = std::io::stdout().flush();
            tokio::signal::ctrl_c().await.context("waiting for ctrl-c")?;
            drop(server);
        }
        Cmd::Responder {
            bind,
            relay_url,
            quic_port,
            secret,
        } => {
            let url = parse_relay_url(&relay_url)?;
            let sk = match secret {
                Some(hex) => {
                    let bytes = (0..hex.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
                        .collect::<std::result::Result<Vec<u8>, _>>()
                        .context("parse --secret hex")?;
                    let arr: [u8; 32] = bytes
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("--secret must be 64 hex chars (32 bytes)"))?;
                    Some(iroh::SecretKey::from_bytes(&arr))
                }
                None => None,
            };
            responder::run(bind, url, quic_port, sk).await?;
        }
        Cmd::Generate {
            bind,
            relay_url,
            quic_port,
            responder_addr,
            mode,
            count,
            concurrency,
            bytes,
            hold_secs,
            settle_ms,
        } => {
            let url = parse_relay_url(&relay_url)?;
            let advertised = load_responder_addr(&responder_addr)?;
            let mode: Mode = mode.parse()?;
            // Each unit is an independent endpoint (own node key = one relay client connection);
            // passthrough clears IP transports per endpoint so traffic stays on the relay.
            let summary = generate::run(GenParams {
                mode,
                count,
                concurrency,
                bind,
                relay_url: url,
                quic_port,
                advertised,
                bytes,
                hold_secs,
                settle_ms,
            })
            .await?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Cmd::E2Connect {
            bind,
            peer_id,
            assign_relay,
            relays,
            quic_port,
            timeout_secs,
        } => {
            let pid: iroh::EndpointId = peer_id.parse().context("parsing peer id")?;
            let assign = parse_relay_url(&assign_relay)?;
            let all: Vec<RelayUrl> = relays
                .iter()
                .map(|s| parse_relay_url(s))
                .collect::<Result<_>>()?;
            let res =
                e2::connect_via_placement(bind, pid, assign, all, quic_port, timeout_secs).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
        Cmd::SyncHub {
            bind,
            relay_url,
            quic_port,
        } => {
            let url = parse_relay_url(&relay_url)?;
            sync::run_hub(bind, url, quic_port).await?;
        }
        Cmd::Meer {
            bind,
            relay_url,
            quic_port,
            allow,
        } => {
            let url = parse_relay_url(&relay_url)?;
            meer::run_meer(bind, url, quic_port, allow).await?;
        }
        Cmd::MeerMember {
            bind,
            relay_url,
            quic_port,
            meer_addr,
            action,
            count,
            namespace,
            expect,
            file,
        } => {
            let url = parse_relay_url(&relay_url)?;
            let addr = load_responder_addr(&meer_addr)?; // same EndpointAddr JSON shape
            let act = match action.as_str() {
                "publish" => meer::MemberAction::Publish { count, namespace },
                "sync" => meer::MemberAction::SyncAndVerify { expect },
                "stats" => meer::MemberAction::Stats,
                "export" => meer::MemberAction::Export { out: file },
                "import" => meer::MemberAction::Import { from: file },
                other => anyhow::bail!("unknown --action {other}"),
            };
            // base_ts: a fixed lab timestamp (Date/now unavailable in some contexts; deterministic here).
            meer::run_member(bind, url, quic_port, addr, act, 1_750_000_000).await?;
        }
        Cmd::RoqRecv {
            bind,
            relay_url,
            quic_port,
            idle_ms,
        } => {
            let url = parse_relay_url(&relay_url)?;
            roq::run_recv(bind, url, quic_port, idle_ms).await?;
        }
        Cmd::RoqSend {
            bind,
            relay_url,
            quic_port,
            recv_addr,
            kbps,
            frame_ms,
            duration_secs,
            relay_only,
            send_buf_bytes,
            adaptive,
            min_kbps,
            rtt_budget_ms,
            bulk_bytes,
        } => {
            let url = parse_relay_url(&relay_url)?;
            let addr = load_responder_addr(&recv_addr)?; // same EndpointAddr JSON shape
            roq::run_send(
                bind, url, quic_port, addr, kbps, frame_ms, duration_secs, relay_only,
                send_buf_bytes, adaptive, min_kbps, rtt_budget_ms, bulk_bytes,
            )
            .await?;
        }
        Cmd::SyncMember {
            bind,
            relay_url,
            quic_port,
            hub_addr,
            index,
            population,
            relay_only,
        } => {
            let url = parse_relay_url(&relay_url)?;
            let hub = load_responder_addr(&hub_addr)?; // same EndpointAddr JSON shape
            let res =
                sync::run_member(bind, url, quic_port, hub, index, population, relay_only).await?;
            println!("{}", serde_json::to_string_pretty(&res)?);
        }
    }
    Ok(())
}
