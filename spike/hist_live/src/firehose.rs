//! Minimal `com.atproto.sync.subscribeRepos` consumer — enough to observe
//! commit-event `seq` values.  A tungstenite WebSocket over a manually
//! CONNECT-tunneled TCP+TLS stream, so the ambient HTTPS_PROXY / rustls CA
//! bundle is honored the same way the ureq path is.
//!
//! Frame format (atproto sync spec, event stream section): each binary
//! WebSocket message contains two consecutive dag-cbor items — a header
//! (`{op: 1|-1, t: "#commit"|...}`) and a body (`{seq, ...}` for #commit,
//! `{error, message}` for op=-1).
//!
//! We do NOT decode the `blocks` CAR field or the `ops` array for this
//! spike — the property under test is that `seq` is a delivery cursor
//! whose order is set by the server, not by any local fold.

use ipld_core::ipld::Ipld;
use rustls::pki_types::ServerName;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tungstenite::{client::client, Message};

#[derive(Debug, Clone)]
pub struct CommitEvent {
    pub seq: i64,
    pub repo: String,
    pub commit_cid: String,
    pub rev: String,
    /// Full dag-cbor bytes of the body (for the results doc's fixture).
    pub body_dag_cbor: Vec<u8>,
    /// A brief digest we can compare on: op paths + actions ("create",
    /// "update", "delete").
    pub ops: Vec<CommitOp>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitOp {
    pub action: String,
    pub path: String,
    pub cid: Option<String>,
}

pub struct FirehoseClient {
    ws: tungstenite::WebSocket<rustls::StreamOwned<rustls::ClientConnection, TcpStream>>,
}

#[derive(Debug, Default)]
pub struct ReadResult {
    pub commits: Vec<CommitEvent>,
    pub skipped_non_commit: usize,
    pub skipped_non_binary: usize,
}

impl FirehoseClient {
    /// Connect to `wss://<host>/xrpc/com.atproto.sync.subscribeRepos[?cursor=...]`.
    /// Uses the ambient `HTTPS_PROXY` if set (CONNECT-tunneled), and the
    /// rustls CA bundle from `SSL_CERT_FILE` (or `/root/.ccr/ca-bundle.crt`).
    pub fn connect(host: &str, cursor: Option<i64>) -> Result<Self, String> {
        let proxy = std::env::var("HTTPS_PROXY")
            .ok()
            .or_else(|| std::env::var("https_proxy").ok());
        let tcp = if let Some(p) = proxy {
            let p = p
                .strip_prefix("http://")
                .or_else(|| p.strip_prefix("https://"))
                .unwrap_or(&p)
                .to_string();
            connect_via_http_proxy(&p, host, 443)?
        } else {
            TcpStream::connect((host, 443))
                .map_err(|e| format!("tcp connect {}: {}", host, e))?
        };
        tcp.set_read_timeout(Some(Duration::from_secs(30)))
            .map_err(|e| format!("set read timeout: {}", e))?;

        // Rustls config with the environment CA bundle.
        let mut roots = rustls::RootCertStore::empty();
        let bundle_path = std::env::var("SSL_CERT_FILE")
            .unwrap_or_else(|_| "/root/.ccr/ca-bundle.crt".to_string());
        if let Ok(f) = std::fs::File::open(&bundle_path) {
            let mut r = std::io::BufReader::new(f);
            for cert in rustls_pemfile::certs(&mut r).flatten() {
                let _ = roots.add(cert);
            }
        }
        let _ = rustls::crypto::ring::default_provider().install_default();
        let tls_cfg = Arc::new(
            rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth(),
        );

        // Do the TLS handshake ourselves so tungstenite gets an
        // already-wrapped stream (its `client_tls` path lacks a proxy
        // hook).
        let server_name = ServerName::try_from(host.to_string())
            .map_err(|e| format!("server name: {}", e))?;
        let conn = rustls::ClientConnection::new(tls_cfg, server_name)
            .map_err(|e| format!("rustls client conn: {}", e))?;
        let tls_stream = rustls::StreamOwned::new(conn, tcp);

        // Build the WebSocket URL and let tungstenite do the HTTP upgrade
        // over our TLS stream.
        let path = match cursor {
            Some(c) => format!("/xrpc/com.atproto.sync.subscribeRepos?cursor={}", c),
            None => "/xrpc/com.atproto.sync.subscribeRepos".to_string(),
        };
        let url = format!("wss://{}{}", host, path);
        let req = tungstenite::http::Request::builder()
            .uri(&url)
            .header("Host", host)
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tungstenite::handshake::client::generate_key(),
            )
            .body(())
            .map_err(|e| format!("build request: {}", e))?;
        // tungstenite's `client()` accepts any Read+Write stream and drives
        // the HTTP upgrade over it.  rustls::StreamOwned implements both;
        // handshake happens lazily on first I/O.
        let (ws, _resp) = client(req, tls_stream)
            .map_err(|e| format!("ws upgrade: {}", e))?;

        Ok(FirehoseClient { ws })
    }

    /// Read binary messages until either `deadline` elapses or `count`
    /// commit events have been captured.  Non-#commit events (identity,
    /// account, etc.) are counted as skipped.
    pub fn read_commits(
        &mut self,
        count: usize,
        deadline: Instant,
    ) -> Result<ReadResult, String> {
        let mut out = Vec::new();
        let mut skipped_non_commit = 0usize;
        let mut skipped_non_binary = 0usize;
        while out.len() < count {
            if Instant::now() >= deadline {
                break;
            }
            match self.ws.read() {
                Ok(Message::Binary(bytes)) => match parse_commit_event(&bytes) {
                    Some(ev) => out.push(ev),
                    None => skipped_non_commit += 1,
                },
                Ok(Message::Ping(p)) => {
                    let _ = self.ws.send(Message::Pong(p));
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => {
                    skipped_non_binary += 1;
                }
                Err(tungstenite::Error::Io(e))
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut =>
                {
                    if Instant::now() >= deadline {
                        break;
                    }
                }
                Err(e) => return Err(format!("ws read: {}", e)),
            }
        }
        Ok(ReadResult {
            commits: out,
            skipped_non_commit,
            skipped_non_binary,
        })
    }

    pub fn close(mut self) {
        let _ = self.ws.close(None);
    }

    /// Debug helper: return the next raw binary frame's bytes (or None on
    /// non-binary / timeout).
    pub fn raw_read(&mut self) -> Result<Option<Vec<u8>>, String> {
        match self.ws.read() {
            Ok(Message::Binary(b)) => Ok(Some(b)),
            Ok(_) => Ok(None),
            Err(tungstenite::Error::Io(e))
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut =>
            {
                Ok(None)
            }
            Err(e) => Err(format!("ws read: {}", e)),
        }
    }
}

/// Open a TCP connection to `host:port` via an HTTP CONNECT proxy at
/// `proxy_authority` (e.g. `127.0.0.1:44727`).
pub fn connect_via_http_proxy(
    proxy_authority: &str,
    host: &str,
    port: u16,
) -> Result<TcpStream, String> {
    let mut tcp = TcpStream::connect(proxy_authority)
        .map_err(|e| format!("tcp connect proxy {}: {}", proxy_authority, e))?;
    let req = format!(
        "CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\nProxy-Connection: Keep-Alive\r\n\r\n",
    );
    tcp.write_all(req.as_bytes())
        .map_err(|e| format!("write CONNECT: {}", e))?;
    // Read status line up to \r\n\r\n.
    let mut buf = Vec::new();
    let mut b = [0u8; 1];
    loop {
        tcp.read_exact(&mut b)
            .map_err(|e| format!("read CONNECT response: {}", e))?;
        buf.push(b[0]);
        if buf.ends_with(b"\r\n\r\n") {
            break;
        }
        if buf.len() > 8192 {
            return Err("CONNECT response too large".into());
        }
    }
    let resp = String::from_utf8_lossy(&buf);
    let first_line = resp.lines().next().unwrap_or("");
    if !first_line.contains(" 200 ") {
        return Err(format!("proxy refused CONNECT: {}", first_line.trim()));
    }
    Ok(tcp)
}

/// Parse an atproto sync event into a `CommitEvent`, returning None for
/// non-#commit events or malformed bodies.
pub fn parse_commit_event(bytes: &[u8]) -> Option<CommitEvent> {
    // Two consecutive dag-cbor items: header, then body.  `from_reader`
    // rejects trailing data, so use a manual CBOR-item byte-length probe
    // to split the frame into header and body slices.
    let header_len = cbor_item_len(bytes)?;
    let header: Ipld = serde_ipld_dagcbor::from_slice(&bytes[..header_len]).ok()?;
    let header_map = if let Ipld::Map(m) = header { m } else { return None };
    let t = if let Some(Ipld::String(s)) = header_map.get("t") { s.clone() } else { return None };
    if t != "#commit" {
        return None;
    }
    let body_bytes = bytes[header_len..].to_vec();
    let body: Ipld = serde_ipld_dagcbor::from_slice(&body_bytes).ok()?;
    let body_map = if let Ipld::Map(m) = body { m } else { return None };

    let seq = match body_map.get("seq") {
        Some(Ipld::Integer(i)) => *i as i64,
        _ => return None,
    };
    let repo = match body_map.get("repo") {
        Some(Ipld::String(s)) => s.clone(),
        _ => return None,
    };
    let commit_cid = match body_map.get("commit") {
        Some(Ipld::Link(c)) => c.to_string(),
        _ => return None,
    };
    let rev = match body_map.get("rev") {
        Some(Ipld::String(s)) => s.clone(),
        _ => return None,
    };
    let mut ops = Vec::new();
    if let Some(Ipld::List(list)) = body_map.get("ops") {
        for op in list {
            if let Ipld::Map(m) = op {
                let action = if let Some(Ipld::String(s)) = m.get("action") { s.clone() } else { continue };
                let path = if let Some(Ipld::String(s)) = m.get("path") { s.clone() } else { continue };
                let cid = m.get("cid").and_then(|v| if let Ipld::Link(c) = v { Some(c.to_string()) } else { None });
                ops.push(CommitOp { action, path, cid });
            }
        }
    }
    Some(CommitEvent {
        seq,
        repo,
        commit_cid,
        rev,
        body_dag_cbor: body_bytes,
        ops,
    })
}

/// Return the byte length of the first CBOR item at `bytes[0]`, or `None`
/// if the encoding is malformed.  Supports the DAG-CBOR subset (definite
/// lengths only; tags followed by exactly one nested item).
pub fn cbor_item_len(bytes: &[u8]) -> Option<usize> {
    if bytes.is_empty() {
        return None;
    }
    let head = bytes[0];
    let major = head >> 5;
    let additional = head & 0x1f;
    let (arg, arg_bytes) = decode_arg(additional, &bytes[1..])?;
    let head_len = 1 + arg_bytes;
    match major {
        0 | 1 => Some(head_len), // integer
        2 | 3 => Some(head_len + arg as usize), // byte/text string
        4 => {
            // array of `arg` items
            let mut cur = head_len;
            for _ in 0..arg {
                let l = cbor_item_len(bytes.get(cur..)?)?;
                cur += l;
            }
            Some(cur)
        }
        5 => {
            // map of `arg` key-value pairs
            let mut cur = head_len;
            for _ in 0..arg {
                let kl = cbor_item_len(bytes.get(cur..)?)?;
                cur += kl;
                let vl = cbor_item_len(bytes.get(cur..)?)?;
                cur += vl;
            }
            Some(cur)
        }
        6 => {
            // tag + one nested item
            let nested = cbor_item_len(bytes.get(head_len..)?)?;
            Some(head_len + nested)
        }
        7 => {
            // simple / float — additional encodes the payload length already
            Some(head_len)
        }
        _ => None,
    }
}

/// Decode a CBOR `additional info` field into (arg value, bytes-consumed-after-head).
fn decode_arg(additional: u8, rest: &[u8]) -> Option<(u64, usize)> {
    match additional {
        n if n < 24 => Some((n as u64, 0)),
        24 => rest.first().map(|b| (*b as u64, 1)),
        25 => rest.get(..2).map(|b| (u16::from_be_bytes([b[0], b[1]]) as u64, 2)),
        26 => rest.get(..4).map(|b| {
            (
                u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as u64,
                4,
            )
        }),
        27 => rest.get(..8).map(|b| {
            (
                u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]),
                8,
            )
        }),
        _ => None, // 28..30 reserved, 31 indefinite (not in dag-cbor)
    }
}
