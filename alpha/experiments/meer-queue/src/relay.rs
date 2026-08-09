//! **Copied** from `../iroh/crates/mls-welcome-over-iroh/src/relay.rs` (unmodified except as
//! noted). Copied rather than depended on because that crate path-depends on
//! `Proofs/lineage-groups/lineage-mls`, an MLS wrapper this spike deliberately does not want —
//! the meer touches no MLS types at all.
//!
//! **Change from the original:** `RelayPorts::default()` used fixed lab ports
//! (3340/3343/3478/9090). Fixed ports collide when tests run concurrently, so this copy adds
//! `RelayPorts::ephemeral()`, which reserves four free ports from the OS. The original
//! `Default` is left intact for reference.
//!
//! E0 single relay: spawn an `iroh-relay` server with a self-signed cert and `AllowAll` admission,
//! on explicit (reachable) ports, and advertise its URL.
//!
//! Verified against iroh-relay 1.0.0 source (see `IROH-1.0.0-API-VERIFIED.md`).

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use anyhow::{Context, Result};
use iroh::RelayUrl;
use iroh_relay::server::testing::self_signed_tls_certs_and_config;
use iroh_relay::server::{
    AllowAll, CertConfig, QuicConfig, RelayConfig as RelayServerConfig, Server, ServerConfig,
    TlsConfig,
};

/// Ports the relay binds. Same on every shard so cross-shard runs compare.
#[derive(Debug, Clone, Copy)]
pub struct RelayPorts {
    pub http: u16,
    pub https: u16,
    pub quic: u16,
    /// OpenMetrics/Prometheus scrape endpoint (spec §3 relay counters incl. send_packets_dropped).
    pub metrics: u16,
}

impl RelayPorts {
    /// Four free ports, reserved from the OS by binding and immediately releasing.
    ///
    /// Racy in principle (another process could take a port between release and re-bind);
    /// in practice fine for a loopback test, and far safer than fixed ports when several
    /// tests run at once. Added by the meer-queue spike — see the module header.
    pub fn ephemeral() -> std::io::Result<Self> {
        fn free() -> std::io::Result<u16> {
            let l = std::net::TcpListener::bind("127.0.0.1:0")?;
            Ok(l.local_addr()?.port())
        }
        Ok(Self { http: free()?, https: free()?, quic: free()?, metrics: free()? })
    }
}

impl Default for RelayPorts {
    fn default() -> Self {
        // Lab defaults, all > UDP 2112 and inside the "all-from-self" SG range.
        Self {
            http: 3340,
            https: 3343,
            quic: 3478,
            metrics: 9090,
        }
    }
}

/// Spawn the relay on `bind_ip` with `ports`, advertising `advertise_ip` to clients.
///
/// Returns the running [`Server`] (drop = shutdown) and the advertised [`RelayUrl`].
pub async fn spawn(
    bind_ip: IpAddr,
    advertise_ip: IpAddr,
    ports: RelayPorts,
) -> Result<(Server, RelayUrl)> {
    let (_certs, server_config) = self_signed_tls_certs_and_config();
    let tls = TlsConfig::new(
        SocketAddr::new(bind_ip, ports.https),
        CertConfig::Manual { server_config },
    );

    let mut relay = RelayServerConfig::new(SocketAddr::new(bind_ip, ports.http));
    relay.tls = Some(tls);
    relay.key_cache_capacity = Some(1024);
    relay.access = Arc::new(AllowAll);

    let mut config = ServerConfig::default();
    config.relay = Some(relay);
    config.quic = Some(QuicConfig::new(SocketAddr::new(bind_ip, ports.quic)));
    // Expose the relay's OpenMetrics endpoint so the driver can scrape §3 counters.
    config.metrics_addr = Some(SocketAddr::new(bind_ip, ports.metrics));

    let server = Server::spawn(config).await.context("relay spawn failed")?;

    let url: RelayUrl = format!("https://{advertise_ip}:{}", ports.https)
        .parse()
        .context("building relay url")?;
    Ok((server, url))
}
