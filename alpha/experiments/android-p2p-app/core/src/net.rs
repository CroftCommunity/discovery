//! The iroh transport: an ephemeral peer-to-peer channel for exchanging Automerge
//! snapshots between members of a group.
//!
//! ## Patterns borrowed from Delta Chat's production iroh integration
//!
//! * **Lazily started.** We do not bind an [`Endpoint`] at app start. The endpoint
//!   is only created when the user actually opens/creates a group ([`P2pNode::bind`]),
//!   so the p2p machinery (sockets, relay connection, holepunching) is not spun up
//!   while it is not needed — better for battery and correctness on mobile.
//! * **Bootstrap by NodeAddr + TopicId, IP excluded.** A group is joined from an
//!   invite that carries the inviter's [`EndpointAddr`] *relay url + public key* plus
//!   a random 32-byte topic id (see [`crate::protocol`]). The direct IP is deliberately
//!   left out of the invite so it is not persisted by whoever carries the invite;
//!   holepunching discovers the direct path at connect time.
//!
//! For this *basic* experiment a group channel exchanges full document snapshots over
//! a single bidirectional QUIC stream. Gossip-style epidemic broadcast (iroh-gossip)
//! is the natural scale-up for larger groups — see the README for why it is not used
//! here (a transitive `sha2`/`ed25519-dalek` version conflict with `automerge`).

use std::future::Future;
use std::net::SocketAddr;
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use iroh::endpoint::presets;
use iroh::{Endpoint, EndpointAddr, EndpointId, RelayMode, SecretKey, TransportAddr};

/// Application-layer protocol identifier negotiated on every connection. Both peers
/// must present the same ALPN or the connection is refused.
pub const ALPN: &[u8] = b"croftc-exp/p2p-group-automerge/0";

/// Upper bound on a single document snapshot we will read from a peer (64 MiB).
/// A real app would stream/chunk; this is a basic experiment.
const MAX_SNAPSHOT: usize = 64 * 1024 * 1024;

/// Deadline applied to each network wait so a slow/non-responsive peer cannot block
/// the (synchronous, FFI-driven) command path indefinitely.
const IO_TIMEOUT: Duration = Duration::from_secs(15);

/// Await `fut` with [`IO_TIMEOUT`], turning a timeout into a labelled error.
async fn with_timeout<F: Future>(label: &str, fut: F) -> Result<F::Output> {
    tokio::time::timeout(IO_TIMEOUT, fut)
        .await
        .map_err(|_| anyhow!("{label} timed out after {IO_TIMEOUT:?}"))
}

/// An iroh endpoint bound for a single group session.
pub struct P2pNode {
    endpoint: Endpoint,
}

impl P2pNode {
    /// Lazily bind an endpoint. Pass an existing [`SecretKey`] to keep a stable
    /// identity across runs, or `None` to generate a fresh one.
    ///
    /// `relay` selects the connectivity model:
    /// * `true`  — n0 defaults (relay + DNS discovery): how the real app reaches
    ///   peers across NATs using only the relay url + public key from an invite.
    /// * `false` — relay/discovery disabled: used by the hermetic host integration
    ///   test, which connects two endpoints over loopback via explicit direct addrs
    ///   and therefore needs no external network.
    pub async fn bind(secret: Option<SecretKey>, relay: bool) -> Result<Self> {
        let secret = secret.unwrap_or_else(SecretKey::generate);
        let endpoint = if relay {
            Endpoint::builder(presets::N0)
                .secret_key(secret)
                .alpns(vec![ALPN.to_vec()])
                .relay_mode(RelayMode::Default)
                .bind()
                .await
                .map_err(|e| anyhow::anyhow!("bind endpoint (relay): {e}"))?
        } else {
            let loopback: SocketAddr = (std::net::Ipv4Addr::LOCALHOST, 0).into();
            Endpoint::builder(presets::Minimal)
                .secret_key(secret)
                .alpns(vec![ALPN.to_vec()])
                .relay_mode(RelayMode::Disabled)
                .bind_addr(loopback)
                .map_err(|e| anyhow::anyhow!("bind addr: {e}"))?
                .bind()
                .await
                .map_err(|e| anyhow::anyhow!("bind endpoint (direct): {e}"))?
        };
        Ok(Self { endpoint })
    }

    /// This endpoint's public key / id.
    pub fn id(&self) -> EndpointId {
        self.endpoint.id()
    }

    /// This endpoint's full address (id + relay url + discovered IP addrs). The
    /// invite builder ([`crate::protocol::Invite::new`]) drops the IP addrs.
    pub fn addr(&self) -> EndpointAddr {
        self.endpoint.addr()
    }

    /// Wait until the endpoint is online (has a relay home / discovered addresses).
    /// Only meaningful when bound with `relay = true`.
    pub async fn online(&self) {
        self.endpoint.online().await;
    }

    /// The locally bound UDP socket addresses. Used by the host test to build a
    /// directly-dialable [`EndpointAddr`] over loopback.
    pub fn bound_sockets(&self) -> Vec<SocketAddr> {
        self.endpoint.bound_sockets()
    }

    /// An [`EndpointAddr`] for **direct** dialing (relay disabled): id + IP addrs.
    /// Used by the hermetic test, never put into an invite.
    pub fn direct_addr(&self) -> EndpointAddr {
        EndpointAddr::from_parts(
            self.endpoint.id(),
            self.bound_sockets().into_iter().map(TransportAddr::Ip),
        )
    }

    /// Connector role of the snapshot exchange: open a bi stream, push our snapshot,
    /// then read the peer's snapshot back. Returns the peer's snapshot bytes for the
    /// caller to merge.
    pub async fn connect_and_exchange(
        &self,
        peer: EndpointAddr,
        local_snapshot: Vec<u8>,
    ) -> Result<Vec<u8>> {
        let conn = with_timeout("connect", self.endpoint.connect(peer, ALPN))
            .await?
            .map_err(|e| anyhow!("connect: {e}"))?;
        let (mut send, mut recv) = with_timeout("open_bi", conn.open_bi())
            .await?
            .map_err(|e| anyhow!("open_bi: {e}"))?;

        with_timeout("write", send.write_all(&local_snapshot))
            .await?
            .context("write local snapshot")?;
        send.finish().context("finish send stream")?;

        let peer_snapshot = with_timeout("read", recv.read_to_end(MAX_SNAPSHOT))
            .await?
            .map_err(|e| anyhow!("read peer snapshot: {e}"))?;

        conn.close(0u32.into(), b"done");
        Ok(peer_snapshot)
    }

    /// Acceptor role of the snapshot exchange: accept one incoming connection, read
    /// the peer's snapshot, then send ours back. Returns the peer's snapshot bytes.
    pub async fn accept_and_exchange(&self, local_snapshot: Vec<u8>) -> Result<Vec<u8>> {
        // Waiting for an incoming connection is legitimately open-ended (this is a
        // listener), so it is not bounded; every step *after* a peer arrives is.
        let incoming = self
            .endpoint
            .accept()
            .await
            .context("endpoint closed before a connection arrived")?;
        // `Incoming` is `IntoFuture` (not `Future`), so await it inside an async block.
        let conn = with_timeout("accept", async { incoming.await })
            .await?
            .map_err(|e| anyhow!("accept: {e}"))?;
        let (mut send, mut recv) = with_timeout("accept_bi", conn.accept_bi())
            .await?
            .map_err(|e| anyhow!("accept_bi: {e}"))?;

        let peer_snapshot = with_timeout("read", recv.read_to_end(MAX_SNAPSHOT))
            .await?
            .map_err(|e| anyhow!("read peer snapshot: {e}"))?;

        with_timeout("write", send.write_all(&local_snapshot))
            .await?
            .context("write local snapshot")?;
        send.finish().context("finish send stream")?;
        let _ = with_timeout("closed", conn.closed()).await;
        Ok(peer_snapshot)
    }

    /// Gracefully close the endpoint, flushing any queued close frames. The mobile
    /// "tear down when the group view closes" lesson from Delta Chat maps here.
    pub async fn close(&self) {
        self.endpoint.close().await;
    }
}
