//! Shared endpoint + relay-map construction for the lab.
//!
//! All APIs here were verified against the pinned iroh 1.0.0 source — see
//! `relay-lab-runs/IROH-1.0.0-API-VERIFIED.md` for citations.

use std::net::SocketAddr;

use anyhow::{Context, Result};
use iroh::endpoint::presets;
use iroh::{Endpoint, RelayConfig, RelayMap, RelayMode, RelayUrl, SecretKey};
use iroh_relay::RelayQuicConfig;
use iroh_relay::tls::CaTlsConfig;

/// The ALPN every lab endpoint speaks. Arbitrary, but must match across responder + generator.
pub const ALPN: &[u8] = b"croft/relay-lab/0";

/// Build a single-relay [`RelayMap`] pointing at `relay_url`, optionally with a QUIC port.
pub fn relay_map(relay_url: &RelayUrl, quic_port: Option<u16>) -> RelayMap {
    let quic = quic_port.map(RelayQuicConfig::new);
    RelayConfig::new(relay_url.clone(), quic).into()
}

/// Build a lab endpoint homed on `relay_url`, bound to `bind`.
///
/// Uses `presets::Minimal` (crypto provider only — no n0 DNS/relays) plus an explicit custom relay,
/// and `CaTlsConfig::insecure_skip_verify()` so the endpoint accepts our self-signed lab relay cert.
/// ⚠️ insecure_skip_verify is LAB-ONLY (closed sandbox VPC) — never in product.
///
/// `relay_only` clears the IP transports (`clear_ip_transports`) so no direct path can ever form —
/// the genuine forced-passthrough configuration. (Verified: a relay-only *dial address* alone does
/// NOT force passthrough; iroh upgrades to direct via relay-coordinated hole-punch — see iroh's own
/// `endpoint_two_relay_only_becomes_direct` test. Only clearing the transport keeps traffic relayed.)
pub async fn build_endpoint(
    bind: SocketAddr,
    relay_url: &RelayUrl,
    quic_port: Option<u16>,
    secret: Option<SecretKey>,
    relay_only: bool,
) -> Result<Endpoint> {
    build_endpoint_with_transport(bind, relay_url, quic_port, secret, relay_only, None).await
}

/// As [`build_endpoint`], but with an optional custom QUIC transport config. E10 uses this to set
/// `datagram_send_buffer_size` — the lever that decides whether an over-cap datagram source
/// bufferbloats (large buffer: queue + pace) or drops cleanly (small buffer: `send_datagram` errs).
pub async fn build_endpoint_with_transport(
    bind: SocketAddr,
    relay_url: &RelayUrl,
    quic_port: Option<u16>,
    secret: Option<SecretKey>,
    relay_only: bool,
    transport: Option<iroh::endpoint::QuicTransportConfig>,
) -> Result<Endpoint> {
    let map = relay_map(relay_url, quic_port);
    let mut builder = Endpoint::builder(presets::Minimal)
        .relay_mode(RelayMode::Custom(map))
        .alpns(vec![ALPN.to_vec()])
        .ca_tls_config(CaTlsConfig::insecure_skip_verify());
    if let Some(sk) = secret {
        builder = builder.secret_key(sk);
    }
    if let Some(tc) = transport {
        builder = builder.transport_config(tc);
    }
    // bind_addr PUSHES an IP transport, so for relay-only we must NOT call it — we clear the
    // default IP transport instead, leaving only the relay path (no direct socket bound at all).
    let builder = if relay_only {
        builder.clear_ip_transports()
    } else {
        builder
            .bind_addr(bind)
            .with_context(|| format!("invalid bind addr {bind}"))?
    };
    let ep = builder.bind().await.context("endpoint bind failed")?;
    Ok(ep)
}
