//! E10 — RoQ-under-netem: synthetic CBR media over iroh QUIC *datagrams*, the C1 probe.
//!
//! The C1 unknown (`discovery/thinking/realtime-media-over-iroh.md`) is whether iroh's QUIC
//! congestion control fights a media bitrate estimator on the *datagram* path. We attack it with a
//! synthetic constant-bitrate source at Opus-shaped framing (no codec, no cpal — the headless rule),
//! carried over `conn.send_datagram` (the exact primitive `iroh-roq`/callme ship on), and measured
//! through the E6 `tc netem` rig.
//!
//! Spike-class: measures a decision, not product. Two roles:
//! - `roq-recv` accepts a connection and tallies loss / reorder / RFC3550 jitter / goodput / gap
//!   bursts (the packet-loss-concealment proxy).
//! - `roq-send` emits sequence + timestamp-stamped CBR datagrams at a target bitrate, and samples
//!   the *sender-side* CC signals every tick: selected-path RTT, `datagram_send_buffer_space`, and
//!   `send_datagram` errors (the buffer-full / would-block signal that reveals CC throttling).

use std::net::SocketAddr;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use bytes::Bytes;
use iroh::{EndpointAddr, RelayUrl};
use serde::Serialize;
use tracing::{info, warn};

use crate::node::{build_endpoint, build_endpoint_with_transport};

/// Per-datagram header (little-endian), padded to the target packet size.
const HDR: usize = 24;
const MARK_DATA: u64 = 0;
const MARK_END: u64 = 1;

fn encode(seq: u64, send_nanos: u64, mark: u64, size: usize) -> Bytes {
    let n = size.max(HDR);
    let mut buf = vec![0u8; n];
    buf[0..8].copy_from_slice(&seq.to_le_bytes());
    buf[8..16].copy_from_slice(&send_nanos.to_le_bytes());
    buf[16..24].copy_from_slice(&mark.to_le_bytes());
    Bytes::from(buf)
}

fn decode(b: &[u8]) -> Option<(u64, u64, u64)> {
    if b.len() < HDR {
        return None;
    }
    let seq = u64::from_le_bytes(b[0..8].try_into().ok()?);
    let send_nanos = u64::from_le_bytes(b[8..16].try_into().ok()?);
    let mark = u64::from_le_bytes(b[16..24].try_into().ok()?);
    Some((seq, send_nanos, mark))
}

#[derive(Serialize)]
pub struct RecvSummary {
    role: &'static str,
    received: u64,
    /// Highest seq seen minus lowest, plus one — the count the sender *intended* to land.
    span: u64,
    lost: u64,
    loss_pct: f64,
    reordered: u64,
    /// RFC3550 interarrival jitter, milliseconds (clock-skew-invariant).
    jitter_ms: f64,
    goodput_kbps: f64,
    /// Concealment proxy: number of sequence gaps (each = a PLC event) and the worst burst.
    gap_events: u64,
    max_consecutive_loss: u64,
    duration_s: f64,
    bytes_per_pkt: usize,
}

#[derive(Serialize)]
pub struct RttSample {
    t_ms: u64,
    rtt_ms: f64,
    send_buf_space: usize,
    is_relay: bool,
    /// The estimator's current target bitrate at this sample (== target_kbps when not adaptive).
    cur_kbps: u64,
}

#[derive(Serialize)]
pub struct SendSummary {
    role: &'static str,
    target_kbps: u64,
    frame_ms: u64,
    bytes_per_pkt: usize,
    duration_s: f64,
    /// Configured datagram send buffer (None = iroh default ~1 MiB).
    send_buf_bytes: Option<usize>,
    /// Delay-based AIMD estimator on/off + its final converged bitrate + the baseline RTT it locked.
    adaptive: bool,
    final_kbps: u64,
    base_rtt_ms: f64,
    sent: u64,
    /// `send_datagram` returned Err — the CC throttle / buffer-full signal. Bucketed by Display.
    send_errors: u64,
    err_buffer_full: u64,
    err_too_large: u64,
    err_other: u64,
    max_datagram_size: Option<usize>,
    achieved_kbps: f64,
    rtt_ms_min: f64,
    rtt_ms_max: f64,
    rtt_ms_mean: f64,
    samples: Vec<RttSample>,
}

/// Receiver: accept one connection and tally datagrams until END marker, connection close, or idle.
pub async fn run_recv(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    idle_ms: u64,
) -> Result<()> {
    let ep = build_endpoint(bind, &relay_url, quic_port, None, false).await?;
    ep.online().await;
    let addr = ep.addr();
    println!(
        "ROQ_RECV_ADDR={}",
        serde_json::to_string(&addr).context("serialize EndpointAddr")?
    );
    use std::io::Write;
    let _ = std::io::stdout().flush();
    info!(id = %addr.id, "roq receiver online");

    let incoming = ep.accept().await.context("no incoming connection")?;
    let conn = incoming.accept()?.await.context("handshake")?;
    info!("roq receiver: connection accepted");

    // TC-CC3: drain any concurrent BULK reliable stream the sender opens, so the media datagram flow
    // and a file transfer share one connection. Runs in the background; we only measure the datagrams.
    let bulk_conn = conn.clone();
    tokio::spawn(async move {
        while let Ok((mut send, mut recv)) = bulk_conn.accept_bi().await {
            // Drain to the void; ack with a byte so the sender's stream completes.
            let _ = recv.read_to_end(1024 * 1024 * 1024).await;
            let _ = send.write_all(b".").await;
            let _ = send.finish();
        }
    });

    let mut received: u64 = 0;
    let mut first_seq: Option<u64> = None;
    let mut high_seq: u64 = 0;
    let mut prev_seq: Option<u64> = None;
    let mut reordered: u64 = 0;
    let mut bytes_per_pkt = 0usize;
    // RFC3550 jitter accumulators.
    let mut jitter: f64 = 0.0;
    let mut prev_send_nanos: Option<u64> = None;
    let mut prev_recv_nanos: Option<u64> = None;
    let mut total_bytes: u64 = 0;
    // Gap bookkeeping for the PLC proxy: count of distinct gaps, worst consecutive run.
    // Seq is monotonic at the sender, so a jump in the running max marks lost packets.
    let mut gap_events: u64 = 0;
    let mut max_consecutive_loss: u64 = 0;

    let start = Instant::now();
    let idle = Duration::from_millis(idle_ms);

    loop {
        let read = conn.read_datagram();
        match tokio::time::timeout(idle, read).await {
            Err(_) => {
                info!("roq receiver: idle timeout, finalizing");
                break;
            }
            Ok(Err(e)) => {
                info!("roq receiver: connection closed ({e}), finalizing");
                break;
            }
            Ok(Ok(data)) => {
                let Some((seq, send_nanos, mark)) = decode(&data) else {
                    continue;
                };
                if mark == MARK_END {
                    info!("roq receiver: END marker, finalizing");
                    break;
                }
                let recv_nanos = start.elapsed().as_nanos() as u64;
                received += 1;
                bytes_per_pkt = data.len();
                total_bytes += data.len() as u64;
                if first_seq.is_none() {
                    first_seq = Some(seq);
                    high_seq = seq;
                }
                if seq > high_seq {
                    let jump = seq - high_seq;
                    if jump > 1 {
                        gap_events += 1;
                        max_consecutive_loss = max_consecutive_loss.max(jump - 1);
                    }
                    high_seq = seq;
                }
                if let Some(p) = prev_seq {
                    if seq < p {
                        reordered += 1;
                    }
                }
                prev_seq = Some(seq);
                // RFC3550 jitter: D = (recv_i - recv_{i-1}) - (send_i - send_{i-1}); J += (|D|-J)/16.
                if let (Some(ps), Some(pr)) = (prev_send_nanos, prev_recv_nanos) {
                    let d = (recv_nanos as i128 - pr as i128) - (send_nanos as i128 - ps as i128);
                    let d_ms = (d as f64).abs() / 1_000_000.0;
                    jitter += (d_ms - jitter) / 16.0;
                }
                prev_send_nanos = Some(send_nanos);
                prev_recv_nanos = Some(recv_nanos);
            }
        }
    }

    let dur = start.elapsed().as_secs_f64().max(1e-9);
    let span = first_seq.map(|f| high_seq.saturating_sub(f) + 1).unwrap_or(0);
    let lost = span.saturating_sub(received);
    let summary = RecvSummary {
        role: "roq-recv",
        received,
        span,
        lost,
        loss_pct: if span > 0 {
            lost as f64 / span as f64 * 100.0
        } else {
            0.0
        },
        reordered,
        jitter_ms: jitter,
        goodput_kbps: total_bytes as f64 * 8.0 / 1000.0 / dur,
        gap_events,
        max_consecutive_loss,
        duration_s: dur,
        bytes_per_pkt,
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}

/// Sender: dial the receiver and emit CBR datagrams at `target_kbps` for `duration_secs`.
#[allow(clippy::too_many_arguments)]
pub async fn run_send(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    recv_addr: EndpointAddr,
    target_kbps: u64,
    frame_ms: u64,
    duration_secs: u64,
    relay_only: bool,
    send_buf_bytes: Option<usize>,
    adaptive: bool,
    min_kbps: u64,
    rtt_budget_ms: f64,
    bulk_bytes: u64,
) -> Result<()> {
    // The CC lever: a small datagram send buffer makes QUIC drop (send_datagram errs) rather than
    // queue+pace under an over-cap source; the default ~1 MiB buffer absorbs seconds of media → bloat.
    let transport = send_buf_bytes.map(|n| {
        iroh::endpoint::QuicTransportConfig::builder()
            .datagram_send_buffer_size(n)
            .build()
    });
    let ep =
        build_endpoint_with_transport(bind, &relay_url, quic_port, None, relay_only, transport)
            .await?;
    ep.online().await;
    let conn = ep
        .connect(recv_addr, crate::node::ALPN)
        .await
        .context("connect to roq receiver")?;
    info!("roq sender: connected");

    // TC-CC3: optionally saturate a concurrent BULK reliable stream on the SAME connection, to test
    // whether QUIC's stream congestion control starves the unreliable media datagram flow.
    if bulk_bytes > 0 {
        let bconn = conn.clone();
        tokio::spawn(async move {
            if let Ok((mut send, _recv)) = bconn.open_bi().await {
                let chunk = vec![0u8; 64 * 1024];
                let mut left = bulk_bytes;
                while left > 0 {
                    let n = left.min(chunk.len() as u64) as usize;
                    if send.write_all(&chunk[..n]).await.is_err() {
                        break;
                    }
                    left -= n as u64;
                }
                let _ = send.finish();
            }
        });
    }

    let interval = Duration::from_millis(frame_ms.max(1));
    let max_dg = conn.max_datagram_size();
    // CBR framing: bytes/frame = bitrate/8 * frame_ms/1000. The estimator varies `cur_kbps`.
    let bytes_for = |kbps: u64| {
        (((kbps as f64 * 1000.0 / 8.0) * (frame_ms as f64 / 1000.0)).round() as usize).max(HDR)
    };
    let mut cur_kbps = target_kbps;
    let bytes_per_pkt = bytes_for(target_kbps);
    info!(bytes_per_pkt, ?max_dg, adaptive, "roq sender: cbr params");

    let start = Instant::now();
    let deadline = start + Duration::from_secs(duration_secs);
    let mut next = Instant::now();
    let mut seq: u64 = 0;
    let mut sent: u64 = 0;
    let mut sent_bytes: u64 = 0;
    let (mut e_buf, mut e_big, mut e_other) = (0u64, 0u64, 0u64);
    let mut samples: Vec<RttSample> = Vec::new();
    let mut last_sample = Instant::now();
    // Delay-based AIMD state: baseline RTT is the running minimum (the unloaded path); when the live
    // RTT climbs `rtt_budget_ms` above it the queue is filling → multiplicative-decrease the bitrate;
    // when RTT is back near baseline → additive-increase toward the target. (A GCC/BBR-lite stand-in
    // for the real media estimator C1 says MUST exist — proves the RTT signal is actionable.)
    let mut base_rtt = f64::INFINITY;

    while Instant::now() < deadline {
        let send_nanos = start.elapsed().as_nanos() as u64;
        let pkt_bytes = bytes_for(cur_kbps);
        let dg = encode(seq, send_nanos, MARK_DATA, pkt_bytes);
        match conn.send_datagram(dg) {
            Ok(()) => {
                sent += 1;
                sent_bytes += pkt_bytes as u64;
            }
            Err(e) => {
                let s = e.to_string().to_lowercase();
                if s.contains("buffer") || s.contains("blocked") {
                    e_buf += 1;
                } else if s.contains("large") || s.contains("size") {
                    e_big += 1;
                } else {
                    e_other += 1;
                }
            }
        }
        seq += 1;

        // Sample CC signals ~every 200ms: selected-path RTT + datagram send-buffer headroom, and
        // run one AIMD control step off the RTT.
        if last_sample.elapsed() >= Duration::from_millis(200) {
            last_sample = Instant::now();
            let paths = conn.paths();
            let sel = paths.iter().find(|p| p.is_selected());
            let (rtt_ms, is_relay) = match sel {
                Some(p) => (p.rtt().as_secs_f64() * 1000.0, p.is_relay()),
                None => (f64::NAN, false),
            };
            if adaptive && rtt_ms.is_finite() {
                if rtt_ms < base_rtt {
                    base_rtt = rtt_ms;
                }
                if rtt_ms > base_rtt + rtt_budget_ms {
                    // queue building → back off hard (×0.7), floored at min_kbps
                    cur_kbps = ((cur_kbps as f64 * 0.7) as u64).max(min_kbps);
                } else {
                    // path healthy → probe up additively (+8 kbps), capped at the configured target
                    cur_kbps = (cur_kbps + 8).min(target_kbps);
                }
            }
            samples.push(RttSample {
                t_ms: start.elapsed().as_millis() as u64,
                rtt_ms,
                send_buf_space: conn.datagram_send_buffer_space(),
                is_relay,
                cur_kbps,
            });
        }

        next += interval;
        let now = Instant::now();
        if next > now {
            tokio::time::sleep(next - now).await;
        } else {
            // Behind schedule (the source can't keep cadence) — skip the sleep, don't burst-catchup.
            next = now;
        }
    }

    // Tell the receiver we're done (best-effort; datagrams can be lost so send a few).
    for _ in 0..5 {
        let _ = conn.send_datagram(encode(seq, start.elapsed().as_nanos() as u64, MARK_END, HDR));
    }
    tokio::time::sleep(Duration::from_millis(300)).await;
    conn.close(0u32.into(), b"done");

    let dur = start.elapsed().as_secs_f64().max(1e-9);
    let rtts: Vec<f64> = samples.iter().map(|s| s.rtt_ms).filter(|v| v.is_finite()).collect();
    let (rmin, rmax, rmean) = if rtts.is_empty() {
        (f64::NAN, f64::NAN, f64::NAN)
    } else {
        let min = rtts.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = rtts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = rtts.iter().sum::<f64>() / rtts.len() as f64;
        (min, max, mean)
    };
    let send_errors = e_buf + e_big + e_other;
    if send_errors > 0 {
        warn!(send_errors, e_buf, e_big, e_other, "roq sender: datagram send errors (CC signal)");
    }
    let summary = SendSummary {
        role: "roq-send",
        target_kbps,
        frame_ms,
        bytes_per_pkt,
        duration_s: dur,
        send_buf_bytes,
        adaptive,
        final_kbps: cur_kbps,
        base_rtt_ms: if base_rtt.is_finite() { base_rtt } else { 0.0 },
        sent,
        send_errors,
        err_buffer_full: e_buf,
        err_too_large: e_big,
        err_other: e_other,
        max_datagram_size: max_dg,
        achieved_kbps: sent_bytes as f64 * 8.0 / 1000.0 / dur,
        rtt_ms_min: rmin,
        rtt_ms_max: rmax,
        rtt_ms_mean: rmean,
        samples,
    };
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
