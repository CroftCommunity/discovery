//! Real iroh-blobs transport: a provider endpoint that serves a content-addressed
//! store over QUIC, and a fetch helper that pulls a blob by hash from a peer.
//!
//! This is genuine networking — two `iroh` endpoints, a `BlobsProtocol` mounted
//! on a `Router` under `iroh_blobs::ALPN`, and BLAKE3-verified streaming between
//! them. Only ciphertext ever touches this layer.

use anyhow::Context;
use iroh::endpoint::presets;
use iroh::protocol::Router;
use iroh::{Endpoint, EndpointAddr};
use iroh_blobs::store::mem::MemStore;
use iroh_blobs::{BlobsProtocol, Hash};

/// A provider node: an iroh endpoint serving an in-memory blob store.
///
/// `MemStore` is used for the experiment; `iroh_blobs::store::fs::FsStore` is the
/// persistent option for production (not needed here — no persistence across
/// restarts is in scope).
pub struct Provider {
    pub endpoint: Endpoint,
    pub store: MemStore,
    pub router: Router,
}

impl Provider {
    /// Bind a provider endpoint and mount the blobs protocol.
    pub async fn spawn() -> anyhow::Result<Self> {
        let endpoint = Endpoint::bind(presets::N0)
            .await
            .map_err(|e| anyhow::anyhow!("bind provider endpoint: {e}"))?;
        let store = MemStore::new();
        let blobs = BlobsProtocol::new(&store, None);
        let router = Router::builder(endpoint.clone())
            .accept(iroh_blobs::ALPN, blobs)
            .spawn();
        Ok(Self {
            endpoint,
            store,
            router,
        })
    }

    /// Add ciphertext bytes to the store, returning the BLAKE3 hash of those
    /// (cipher)bytes as computed by iroh-blobs.
    pub async fn add_ciphertext(&self, bytes: Vec<u8>) -> anyhow::Result<Hash> {
        let tag = self
            .store
            .add_bytes(bytes)
            .await
            .context("add ciphertext to store")?;
        Ok(tag.hash)
    }

    /// This provider's full address (node id + direct socket addresses). Handing
    /// the full address to a peer lets it dial us directly over loopback/LAN with
    /// no DNS/relay discovery round-trip.
    ///
    /// With `presets::N0`, direct addresses are discovered shortly after bind, so
    /// we poll briefly until at least one is available (or time out and return
    /// whatever we have — the caller reports what it got).
    pub async fn addr(&self) -> EndpointAddr {
        for _ in 0..50 {
            let addr = self.endpoint.addr();
            if !addr.addrs.is_empty() {
                return addr;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        self.endpoint.addr()
    }

    pub async fn shutdown(self) -> anyhow::Result<()> {
        self.router.shutdown().await.ok();
        self.endpoint.close().await;
        Ok(())
    }
}

/// A fetching node: its own endpoint, no store needed.
pub struct Fetcher {
    pub endpoint: Endpoint,
}

impl Fetcher {
    pub async fn spawn() -> anyhow::Result<Self> {
        let endpoint = Endpoint::bind(presets::N0)
            .await
            .map_err(|e| anyhow::anyhow!("bind fetcher endpoint: {e}"))?;
        Ok(Self { endpoint })
    }

    /// Fetch a blob by hash from `provider_addr` over real iroh QUIC.
    ///
    /// Returns the received bytes. iroh-blobs verifies BLAKE3 integrity during
    /// the streaming transfer, so a successful return means the bytes hash to the
    /// requested `hash`.
    pub async fn fetch(
        &self,
        provider_addr: EndpointAddr,
        hash: Hash,
    ) -> anyhow::Result<Vec<u8>> {
        let conn = self
            .endpoint
            .connect(provider_addr, iroh_blobs::ALPN)
            .await
            .map_err(|e| anyhow::anyhow!("connect to provider: {e}"))?;
        let bytes = iroh_blobs::get::request::get_blob(conn, hash)
            .bytes()
            .await
            .map_err(|e| anyhow::anyhow!("get_blob: {e}"))?;
        Ok(bytes.to_vec())
    }

    pub async fn shutdown(self) -> anyhow::Result<()> {
        self.endpoint.close().await;
        Ok(())
    }
}
