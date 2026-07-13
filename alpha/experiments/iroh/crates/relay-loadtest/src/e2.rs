//! E2 — DNS-driven placement (fast loop, in-memory injection).
//!
//! Demonstrates that a peer reaches another peer via the relay the *controller assigned*, not via
//! latency or an embedded address: we seed a [`MemoryLookup`] (the in-process stand-in for the
//! controller's published DNS/pkarr record) with `peer-id -> assigned-relay`, give the endpoint a
//! RelayMap containing *several* relays, then connect by bare endpoint id so iroh must resolve the
//! relay through the lookup. The endpoint is relay-only (`clear_ip_transports`) so the live path is
//! the assigned relay and we can read back which relay was actually used.
//!
//! Correct assignment -> connects via the assigned relay. Wrong assignment (peer is actually homed
//! elsewhere) -> no co-location -> connect fails: the E3 co-location thesis in miniature.

use std::net::SocketAddr;

use anyhow::{Context, Result};
use iroh::address_lookup::memory::MemoryLookup;
use iroh::endpoint::presets;
use iroh::{Endpoint, EndpointAddr, EndpointId, RelayConfig, RelayMap, RelayMode, RelayUrl, TransportAddr};
use iroh_relay::RelayQuicConfig;
use iroh_relay::tls::CaTlsConfig;
use serde::Serialize;
use tokio::time::{Duration, timeout};

use crate::node::ALPN;

#[derive(Debug, Serialize)]
pub struct E2Result {
    pub peer_id: String,
    pub assigned_relay: String,
    pub relays_known: usize,
    pub connected: bool,
    pub relay_used: Option<String>,
    pub is_relay_path: Option<bool>,
    pub error: Option<String>,
}

/// Connect to `peer_id` using only the controller-assigned relay (seeded into a MemoryLookup),
/// with `all_relays` in the RelayMap. Returns which relay the live path actually used.
pub async fn connect_via_placement(
    _bind: SocketAddr, // relay-only endpoint binds no direct socket
    peer_id: EndpointId,
    assigned_relay: RelayUrl,
    all_relays: Vec<RelayUrl>,
    quic_port: Option<u16>,
    connect_timeout_secs: u64,
) -> Result<E2Result> {
    let quic = quic_port.map(RelayQuicConfig::new);
    let map: RelayMap = all_relays
        .iter()
        .map(|u| RelayConfig::new(u.clone(), quic.clone()))
        .collect();

    // The controller's published record: peer_id is homed on assigned_relay.
    let lookup = MemoryLookup::new();
    lookup.add_endpoint_info(EndpointAddr::new(peer_id).with_relay_url(assigned_relay.clone()));

    let ep = Endpoint::builder(presets::Minimal)
        .relay_mode(RelayMode::Custom(map))
        .alpns(vec![ALPN.to_vec()])
        .ca_tls_config(CaTlsConfig::insecure_skip_verify())
        .address_lookup(lookup)
        .clear_ip_transports() // relay-only: the live path is the assigned relay, observably
        .bind()
        .await
        .context("bind")?;

    // Connect by BARE id — iroh must resolve the relay through the MemoryLookup.
    let mut result = E2Result {
        peer_id: peer_id.to_string(),
        assigned_relay: assigned_relay.to_string(),
        relays_known: all_relays.len(),
        connected: false,
        relay_used: None,
        is_relay_path: None,
        error: None,
    };

    let connect = ep.connect(EndpointAddr::new(peer_id), ALPN);
    match timeout(Duration::from_secs(connect_timeout_secs), connect).await {
        Ok(Ok(conn)) => {
            // Force a roundtrip so a path is selected.
            if let Ok((mut s, mut r)) = conn.open_bi().await {
                let _ = s.write_all(b"ping").await;
                let _ = s.finish();
                let _ = r.read_to_end(64).await;
            }
            result.connected = true;
            if let Some(p) = conn.paths().iter().find(|p| p.is_selected()) {
                result.is_relay_path = Some(p.is_relay());
                if let TransportAddr::Relay(u) = p.remote_addr() {
                    result.relay_used = Some(u.to_string());
                }
            }
        }
        Ok(Err(e)) => result.error = Some(format!("connect error: {e}")),
        Err(_) => result.error = Some(format!("timed out after {connect_timeout_secs}s")),
    }
    ep.close().await;
    Ok(result)
}
