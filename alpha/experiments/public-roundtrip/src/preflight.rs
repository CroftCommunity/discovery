//! Connectivity preflight. The brief mandates this as the critical first gate:
//! verify the PDS host, the PLC/DID resolver, and the Jetstream endpoint are
//! reachable before any feature code runs. If a required host is blocked, the
//! experiment is meaningless and we stop rather than stub.

use anyhow::Result;
use std::time::Instant;

pub struct Probe {
    pub label: String,
    pub target: String,
    pub reachable: bool,
    pub detail: String,
}

async fn http_probe(http: &reqwest::Client, label: &str, url: &str) -> Probe {
    let start = Instant::now();
    match http.get(url).send().await {
        Ok(resp) => Probe {
            label: label.to_string(),
            target: url.to_string(),
            reachable: true,
            detail: format!("HTTP {} in {:?}", resp.status().as_u16(), start.elapsed()),
        },
        Err(e) => Probe {
            label: label.to_string(),
            target: url.to_string(),
            reachable: false,
            detail: format!("error: {e}"),
        },
    }
}

/// Probe the three required hosts. `pds` and `jetstream_ws` are configurable.
pub async fn run(pds: &str, jetstream_ws: &str) -> Result<Vec<Probe>> {
    let http = reqwest::Client::builder()
        .user_agent("public-roundtrip-experiment/0.1")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let mut probes = Vec::new();
    probes.push(http_probe(&http, "PDS (_health)", &format!("{pds}/xrpc/_health")).await);
    probes.push(http_probe(&http, "PLC directory", "https://plc.directory/").await);

    // Jetstream is a WebSocket; probe by attempting the WS handshake, bounded by
    // a timeout so a silent server can't hang the gate.
    let start = Instant::now();
    let ws_probe = match tokio::time::timeout(
        std::time::Duration::from_secs(10),
        tokio_tungstenite::connect_async(jetstream_ws),
    )
    .await
    {
        Ok(Ok((_ws, _resp))) => Probe {
            label: "Jetstream (WS handshake)".into(),
            target: jetstream_ws.to_string(),
            reachable: true,
            detail: format!("WebSocket upgraded in {:?}", start.elapsed()),
        },
        Ok(Err(e)) => Probe {
            label: "Jetstream (WS handshake)".into(),
            target: jetstream_ws.to_string(),
            reachable: false,
            detail: format!("error: {e}"),
        },
        Err(_elapsed) => Probe {
            label: "Jetstream (WS handshake)".into(),
            target: jetstream_ws.to_string(),
            reachable: false,
            detail: "error: handshake timed out after 10s".into(),
        },
    };
    probes.push(ws_probe);
    Ok(probes)
}
