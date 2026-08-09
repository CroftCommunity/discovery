//! **D5 probe** — do the copied `relay.rs` / `node.rs` build and run without `lineage-mls`,
//! and does a loopback relay actually carry a connection in this environment?
//!
//! Disposition: `promote` — becomes Phase 4's transport.
//!
//! Success criteria (plan Phase 0 D5): a loopback relay spawns and two endpoints connect
//! over the spike's own ALPN. The plan's risk note says: if relay spawn fails here, fall
//! back to direct addressing and record it as a divergence — never a silent drop to an
//! in-memory channel, which would stand in for the transport itself.

#[path = "../node.rs"]
mod node;
#[path = "../relay.rs"]
mod relay;

use std::net::{IpAddr, Ipv4Addr};

use node::{build_endpoint, ALPN};
use relay::{spawn as spawn_relay, RelayPorts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let loopback = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let ports = RelayPorts::ephemeral()?;
    println!("=== D5: real iroh over a loopback relay ===");
    println!("relay ports (ephemeral): {ports:?}");

    let (_relay, relay_url) = spawn_relay(loopback, loopback, ports).await?;
    println!("relay up: {relay_url}");

    // Two endpoints homed on that relay.
    let server_ep = build_endpoint("127.0.0.1:0".parse()?, &relay_url, Some(ports.quic), None, false).await?;
    let client_ep = build_endpoint("127.0.0.1:0".parse()?, &relay_url, Some(ports.quic), None, false).await?;
    // Dial by the endpoint's full `addr()` (id + relay URL), not a bare EndpointId:
    // `presets::Minimal` configures no DNS discovery, so a bare id has nothing to resolve
    // against. This is how the ancestor does it (mls-welcome-over-iroh/src/main.rs:65,103).
    let server_addr = server_ep.addr();
    let server_id = server_ep.id();
    println!("server endpoint id: {server_id}");
    println!("client endpoint id: {}", client_ep.id());

    // A minimal deposit/drain shape: client sends bytes, server echoes their length back.
    // This is the ALPN round trip Phase 4 needs, nothing more.
    let server = tokio::spawn(async move {
        let incoming = server_ep.accept().await.expect("accept");
        let conn = incoming.await.expect("connecting");
        let remote = conn.remote_id();
        let (mut send, mut recv) = conn.accept_bi().await.expect("accept_bi");
        let payload = recv.read_to_end(64 * 1024).await.expect("read");
        send.write_all(&(payload.len() as u32).to_be_bytes()).await.expect("write");
        send.finish().expect("finish");
        // Hold the connection open long enough for the client to read the reply.
        conn.closed().await;
        (remote, payload)
    });

    let conn = client_ep.connect(server_addr, ALPN).await?;
    let (mut send, mut recv) = conn.open_bi().await?;
    let sealed = b"pretend this is a real MLS PrivateMessage";
    send.write_all(sealed).await?;
    send.finish()?;
    let reply = recv.read_to_end(64).await?;
    let echoed = u32::from_be_bytes(reply[..4].try_into().unwrap());
    conn.close(0u32.into(), b"done");

    let (remote_id, got) = server.await?;
    println!("bytes sent      = {}", sealed.len());
    println!("server received = {} (identical: {})", got.len(), got == sealed);
    println!("server echoed   = {echoed}");
    println!("server saw peer = {remote_id}");
    println!(
        "drain-scope check: server can identify the caller by EndpointId = {}",
        remote_id == client_ep.id()
    );
    println!("\nD5 OK: real iroh connection over a real loopback relay, spike ALPN, no lineage-mls.");
    Ok(())
}
