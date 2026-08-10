//! Deposit and drain over a real iroh connection.
//!
//! SPEC-DELTA[meer-spike-drain-auth | stand-in]: a drain is scoped by the caller's iroh
//! `EndpointId`, taken off the authenticated QUIC connection. The spec target is **CISS
//! account identity**; this exercises the same shape but not multi-device-per-account auth.
//! Note that MLS identity is deliberately *not* used and never will be — presenting group
//! credentials to a blind store would tell it which groups you are in, which is the metadata
//! the blindness exists to prevent.
//! — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`
//!
//! **The wire format carries no recipient field on a drain.** The server derives the queue
//! from `connection.remote_id()`. There is nothing for a caller to claim and therefore nothing
//! to validate — a scope that cannot be misstated rather than one that is checked.
//!
//! **Sealed payloads travel as raw length-prefixed bytes**, never re-encoded. M2 is the claim
//! that byte-identical forwarding holds, and a transport that base64'd or JSON-wrapped the
//! payload would put an encode/decode step inside the path under test. The framing wraps the
//! bytes; it never touches them.

use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use anyhow::{Context, Result};
use iroh::endpoint::Connection;
use iroh::{Endpoint, EndpointAddr, RelayUrl};
use tokio::sync::Mutex;

use crate::ciss_harness::CissHarness;
use crate::meer::{Digest, KeyInventory, Meer, RecipientId};
use crate::node::{build_endpoint, ALPN};
use crate::relay::{spawn as spawn_relay, RelayPorts};

/// Opcodes. Deliberately tiny: the meer's whole protocol is deposit, drain, ack.
const OP_DEPOSIT: u8 = 0x01;
const OP_DRAIN: u8 = 0x02;
const OP_ACK: u8 = 0x03;

/// A meer listening on an iroh endpoint, homed on a loopback relay it also runs.
pub struct MeerServer {
    endpoint: Endpoint,
    meer: Arc<Mutex<Meer>>,
    relay_url: RelayUrl,
    /// Dropping the relay shuts it down, so it is held for the server's lifetime.
    _relay: iroh_relay::server::Server,
    accept: Option<tokio::task::JoinHandle<()>>,
}

impl MeerServer {
    /// Stand up a loopback relay, bind an endpoint on it, and serve the meer protocol.
    ///
    /// # Errors
    /// If the relay or endpoint cannot be built.
    pub async fn spawn(ciss: Arc<CissHarness>) -> Result<Self> {
        let loopback = IpAddr::V4(Ipv4Addr::LOCALHOST);
        let ports = RelayPorts::ephemeral().context("reserve relay ports")?;
        let (relay, relay_url) = spawn_relay(loopback, loopback, ports).await?;
        let endpoint = build_endpoint(
            "127.0.0.1:0".parse().expect("loopback bind addr"),
            &relay_url,
            Some(ports.quic),
            None,
            false,
        )
        .await?;

        let meer = Arc::new(Mutex::new(Meer::new(ciss)));
        let served = Arc::clone(&meer);
        let ep = endpoint.clone();
        let accept = tokio::spawn(async move {
            while let Some(incoming) = ep.accept().await {
                let meer = Arc::clone(&served);
                tokio::spawn(async move {
                    match incoming.await {
                        Ok(conn) => {
                            if let Err(e) = serve(conn, meer).await {
                                tracing::debug!(error = %e, "connection ended");
                            }
                        }
                        Err(e) => tracing::debug!(error = %e, "handshake failed"),
                    }
                });
            }
        });

        Ok(Self {
            endpoint,
            meer,
            relay_url,
            _relay: relay,
            accept: Some(accept),
        })
    }

    /// The address a client dials — id **plus** relay URL.
    ///
    /// Not a bare `EndpointId`: `presets::Minimal` configures no DNS discovery, so a bare id
    /// has nothing to resolve against (Phase 0, D5).
    #[must_use]
    pub fn addr(&self) -> EndpointAddr {
        self.endpoint.addr()
    }

    /// The relay this meer is homed on; clients home on the same one.
    #[must_use]
    pub fn relay_url(&self) -> &RelayUrl {
        &self.relay_url
    }

    /// What key material this meer holds. M1 asserts on it.
    pub async fn key_inventory(&self) -> KeyInventory {
        self.meer.lock().await.key_inventory()
    }

    /// Stop accepting and close the endpoint.
    pub async fn shutdown(mut self) {
        if let Some(accept) = self.accept.take() {
            accept.abort();
        }
        self.endpoint.close().await;
    }
}

/// Serve one connection. **The recipient is the connection**, never a field in the request.
async fn serve(conn: Connection, meer: Arc<Mutex<Meer>>) -> Result<()> {
    let caller = RecipientId::new(conn.remote_id().to_string());
    loop {
        let Ok((mut send, mut recv)) = conn.accept_bi().await else {
            return Ok(());
        };
        let op = read_u8(&mut recv).await?;
        match op {
            OP_DEPOSIT => {
                let recipients: Vec<RecipientId> = read_strings(&mut recv)
                    .await?
                    .into_iter()
                    .map(RecipientId::new)
                    .collect();
                let payload = read_bytes(&mut recv).await?;
                let digest = {
                    let mut m = meer.lock().await;
                    m.publish(&payload, &recipients).await?
                };
                write_bytes(&mut send, digest.as_str().as_bytes()).await?;
            }
            OP_DRAIN => {
                // NOTE: no recipient is read from the wire. `caller` is the authenticated id.
                let have: Vec<Digest> = read_strings(&mut recv).await?.into_iter().map(Digest::new).collect();
                let blobs = {
                    let m = meer.lock().await;
                    m.drain(&caller, &have).await?
                };
                write_u32(&mut send, u32::try_from(blobs.len()).unwrap_or(u32::MAX)).await?;
                for blob in blobs {
                    write_bytes(&mut send, &blob).await?;
                }
            }
            OP_ACK => {
                let acked: Vec<Digest> = read_strings(&mut recv).await?.into_iter().map(Digest::new).collect();
                meer.lock().await.ack(&caller, &acked);
                write_u32(&mut send, 0).await?;
            }
            other => anyhow::bail!("unknown opcode {other}"),
        }
        send.finish().ok();
    }
}

/// A client of the meer protocol.
///
/// Also accepts inbound connections, so a peer can hand it bytes **directly** rather than via
/// the meer. That is the "carried live" path S3 contrasts with a drain — and it is what makes
/// reachability a discriminating observation: a client that is up answers a dial, and one that
/// has been torn down does not. (M1 needed that distinction and did not have it until a
/// surviving mutant showed the difference was invisible.)
pub struct MeerClient {
    endpoint: Endpoint,
    received: Arc<Mutex<Vec<Vec<u8>>>>,
    accept: Option<tokio::task::JoinHandle<()>>,
}

impl MeerClient {
    /// Bind a client endpoint homed on `relay_url`.
    ///
    /// # Errors
    /// If the endpoint cannot be built.
    pub async fn connect(relay_url: &RelayUrl) -> Result<Self> {
        Self::connect_with_key(relay_url, None).await
    }

    /// Bind a client endpoint with an explicit secret key.
    ///
    /// The key **is** the identity: rebinding with the same secret returns the same
    /// `EndpointId`, and therefore the same queue. That is what lets M1 model a genuine
    /// absence — Bob's endpoint is torn down and later rebound, not merely marked away.
    ///
    /// # Errors
    /// If the endpoint cannot be built.
    pub async fn connect_with_key(
        relay_url: &RelayUrl,
        secret: Option<iroh::SecretKey>,
    ) -> Result<Self> {
        let endpoint = build_endpoint(
            "127.0.0.1:0".parse().expect("loopback bind addr"),
            relay_url,
            None,
            secret,
            false,
        )
        .await?;

        let received = Arc::new(Mutex::new(Vec::new()));
        let sink = Arc::clone(&received);
        let ep = endpoint.clone();
        let accept = tokio::spawn(async move {
            while let Some(incoming) = ep.accept().await {
                let sink = Arc::clone(&sink);
                tokio::spawn(async move {
                    let Ok(conn) = incoming.await else { return };
                    while let Ok((mut send, mut recv)) = conn.accept_bi().await {
                        match read_bytes(&mut recv).await {
                            Ok(bytes) if !bytes.is_empty() => sink.lock().await.push(bytes),
                            _ => {}
                        }
                        let _ = write_u32(&mut send, 0).await;
                        send.finish().ok();
                    }
                });
            }
        });

        Ok(Self { endpoint, received, accept: Some(accept) })
    }

    /// Hand `bytes` straight to `peer`, bypassing the meer entirely — the live-carriage path.
    ///
    /// # Errors
    /// If the peer is unreachable.
    pub async fn live_deliver(&self, peer: EndpointAddr, bytes: &[u8]) -> Result<()> {
        let conn = self.endpoint.connect(peer, ALPN).await?;
        let (mut send, mut recv) = conn.open_bi().await?;
        write_bytes(&mut send, bytes).await?;
        send.finish()?;
        let _ = read_u32(&mut recv).await?;
        conn.close(0u32.into(), b"done");
        Ok(())
    }

    /// Everything handed to this client directly (not drained from the meer).
    pub async fn received(&self) -> Vec<Vec<u8>> {
        self.received.lock().await.clone()
    }

    /// This client's dialable address.
    #[must_use]
    pub fn addr(&self) -> EndpointAddr {
        self.endpoint.addr()
    }

    /// Can `peer` be reached right now? Discriminating: a live client answers, a torn-down one
    /// does not, because [`Self::connect_with_key`] runs an accept loop for as long as it lives.
    ///
    /// # Errors
    /// If the peer cannot be reached.
    pub async fn probe(&self, peer: EndpointAddr) -> Result<()> {
        let conn = self.endpoint.connect(peer, ALPN).await?;
        let (mut send, mut recv) = conn.open_bi().await?;
        write_bytes(&mut send, &[]).await?;
        send.finish()?;
        let _ = read_u32(&mut recv).await?;
        conn.close(0u32.into(), b"probe");
        Ok(())
    }

    /// Tear this endpoint down.
    pub async fn close(mut self) {
        if let Some(a) = self.accept.take() {
            a.abort();
        }
        self.endpoint.close().await;
    }

    /// The id a depositor names to send this client mail — and the id the meer will scope this
    /// client's own drains by.
    #[must_use]
    pub fn recipient_id(&self) -> RecipientId {
        RecipientId::new(self.endpoint.id().to_string())
    }

    /// Deposit `sealed` for `recipients`. Returns the content address the meer stored it at.
    ///
    /// # Errors
    /// If the connection fails or the meer refuses the deposit.
    pub async fn deposit(
        &self,
        server: EndpointAddr,
        sealed: &[u8],
        recipients: &[RecipientId],
    ) -> Result<Digest> {
        let conn = self.endpoint.connect(server, ALPN).await?;
        let (mut send, mut recv) = conn.open_bi().await?;
        write_u8(&mut send, OP_DEPOSIT).await?;
        let names: Vec<String> = recipients.iter().map(ToString::to_string).collect();
        write_strings(&mut send, &names).await?;
        write_bytes(&mut send, sealed).await?;
        send.finish()?;
        let digest = read_bytes(&mut recv).await?;
        conn.close(0u32.into(), b"done");
        Ok(Digest::new(String::from_utf8(digest)?))
    }

    /// Drain this client's own queue, declaring what it already holds.
    ///
    /// # Errors
    /// If the connection fails.
    pub async fn drain(&self, server: EndpointAddr, have: &[Digest]) -> Result<Vec<Vec<u8>>> {
        let conn = self.endpoint.connect(server, ALPN).await?;
        let (mut send, mut recv) = conn.open_bi().await?;
        write_u8(&mut send, OP_DRAIN).await?;
        let names: Vec<String> = have.iter().map(ToString::to_string).collect();
        write_strings(&mut send, &names).await?;
        send.finish()?;
        let count = read_u32(&mut recv).await?;
        let mut out = Vec::with_capacity(count as usize);
        for _ in 0..count {
            out.push(read_bytes(&mut recv).await?);
        }
        conn.close(0u32.into(), b"done");
        Ok(out)
    }

    /// Acknowledge delivery so the meer prunes.
    ///
    /// # Errors
    /// If the connection fails.
    pub async fn ack(&self, server: EndpointAddr, acked: &[Digest]) -> Result<()> {
        let conn = self.endpoint.connect(server, ALPN).await?;
        let (mut send, mut recv) = conn.open_bi().await?;
        write_u8(&mut send, OP_ACK).await?;
        let names: Vec<String> = acked.iter().map(ToString::to_string).collect();
        write_strings(&mut send, &names).await?;
        send.finish()?;
        let _ = read_u32(&mut recv).await?;
        conn.close(0u32.into(), b"done");
        Ok(())
    }
}

// ---- framing. Length-prefixed; the payload is never transformed. ----

async fn write_u8(send: &mut iroh::endpoint::SendStream, v: u8) -> Result<()> {
    send.write_all(&[v]).await?;
    Ok(())
}

async fn read_u8(recv: &mut iroh::endpoint::RecvStream) -> Result<u8> {
    let mut b = [0u8; 1];
    recv.read_exact(&mut b).await?;
    Ok(b[0])
}

async fn write_u32(send: &mut iroh::endpoint::SendStream, v: u32) -> Result<()> {
    send.write_all(&v.to_be_bytes()).await?;
    Ok(())
}

async fn read_u32(recv: &mut iroh::endpoint::RecvStream) -> Result<u32> {
    let mut b = [0u8; 4];
    recv.read_exact(&mut b).await?;
    Ok(u32::from_be_bytes(b))
}

async fn write_bytes(send: &mut iroh::endpoint::SendStream, bytes: &[u8]) -> Result<()> {
    write_u32(send, u32::try_from(bytes.len()).context("payload too large for u32")?).await?;
    send.write_all(bytes).await?;
    Ok(())
}

async fn read_bytes(recv: &mut iroh::endpoint::RecvStream) -> Result<Vec<u8>> {
    let len = read_u32(recv).await? as usize;
    let mut buf = vec![0u8; len];
    recv.read_exact(&mut buf).await?;
    Ok(buf)
}

async fn write_strings(send: &mut iroh::endpoint::SendStream, items: &[String]) -> Result<()> {
    write_u32(send, u32::try_from(items.len()).context("too many items")?).await?;
    for item in items {
        write_bytes(send, item.as_bytes()).await?;
    }
    Ok(())
}

async fn read_strings(recv: &mut iroh::endpoint::RecvStream) -> Result<Vec<String>> {
    let count = read_u32(recv).await?;
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(String::from_utf8(read_bytes(recv).await?)?);
    }
    Ok(out)
}
