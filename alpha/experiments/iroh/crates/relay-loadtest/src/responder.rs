//! Responder: a lab endpoint homed on the relay that accepts connections and echoes bi-streams.
//! It advertises its [`iroh::EndpointAddr`] as JSON so a generator on another host can dial it
//! (relayed via the Mac per the lab's sync convention).

use std::net::SocketAddr;

use anyhow::{Context, Result};
use iroh::{RelayUrl, SecretKey};
use tracing::{info, warn};

use crate::node::build_endpoint;

/// Largest single bi-stream payload the responder will echo (active-mode generator load).
const MAX_ECHO_BYTES: usize = 64 * 1024 * 1024;

/// Run the responder until cancelled. Prints `RESPONDER_ADDR=<json>` once online.
pub async fn run(
    bind: SocketAddr,
    relay_url: RelayUrl,
    quic_port: Option<u16>,
    secret: Option<SecretKey>,
) -> Result<()> {
    let ep = build_endpoint(bind, &relay_url, quic_port, secret, false).await?;
    // online() waits until the home-relay assignment is established, so addr() carries relay=.
    ep.online().await;
    let addr = ep.addr();
    let json = serde_json::to_string(&addr).context("serialize EndpointAddr")?;
    // Machine-parseable line the driver greps out of the logfile. Flush explicitly: stdout is
    // block-buffered when redirected to a file, and this long-lived process never exits to flush.
    println!("RESPONDER_ADDR={json}");
    use std::io::Write;
    let _ = std::io::stdout().flush();
    info!(id = %addr.id, "responder online");

    while let Some(incoming) = ep.accept().await {
        let conn = match incoming.accept() {
            Ok(accepting) => match accepting.await {
                Ok(conn) => conn,
                Err(e) => {
                    warn!("handshake failed: {e}");
                    continue;
                }
            },
            Err(e) => {
                warn!("accept failed: {e}");
                continue;
            }
        };
        tokio::spawn(async move {
            if let Err(e) = echo_conn(&conn).await {
                warn!("conn closed: {e}");
            }
        });
    }
    Ok(())
}

/// Echo every bi-stream until the peer closes the connection.
async fn echo_conn(conn: &iroh::endpoint::Connection) -> Result<()> {
    loop {
        let (mut send, mut recv) = match conn.accept_bi().await {
            Ok(s) => s,
            // Peer closed the connection — normal end of an idle/active session.
            Err(_) => return Ok(()),
        };
        let data = recv
            .read_to_end(MAX_ECHO_BYTES)
            .await
            .context("read bi-stream")?;
        send.write_all(&data).await.context("echo write")?;
        send.finish().context("finish echo")?;
    }
}
