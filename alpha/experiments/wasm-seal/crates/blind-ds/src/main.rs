//! `blind-ds` — the content-blind Delivery Service over real QUIC
//! (WebTransport / HTTP-3 CONNECT), the GROUPS.md A.7 DS role extended with
//! the EXP-B offer-gating pattern.
//!
//! What it holds: per group, an offer **roster** (member ids) and a list of
//! `(seq, opaque bytes)`. What it never holds: keys, plaintext, message
//! order (seq is a delivery cursor, never an order claim), or membership
//! authority (the roster is TOLD to it; the authority lives in the group's
//! fold — roster-admin caller auth is the RUN-14 EXP-A service-auth seam,
//! a named non-goal here). Every refusal is the one flat [`ds_proto::REFUSED`]
//! — non-member, unknown group, and unknown anything are indistinguishable.
//!
//! Announces `{"port": …, "cert_hash": …}` as one stdout JSON line; the
//! certificate is `Identity::self_signed` — exactly 14 days, the browser's
//! `serverCertificateHashes` cap (PRED-WT3).

use std::collections::{HashMap, HashSet};
use std::net::{Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use ds_proto::{read_frame, read_json, write_frame, write_json, Request, Response, REFUSED};
use wtransport::endpoint::IncomingSession;
use wtransport::{Endpoint, Identity, ServerConfig};

/// One group's DS-side state: who may be offered, and the opaque blobs.
#[derive(Default)]
struct GroupState {
    roster: HashSet<String>,
    blobs: Vec<(u64, Vec<u8>)>,
}

type Store = Arc<Mutex<HashMap<String, GroupState>>>;

#[tokio::main]
async fn main() {
    let mut port: u16 = 4433;
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--port") {
        port = args
            .get(i + 1)
            .and_then(|p| p.parse().ok())
            .unwrap_or(4433);
    }

    // IPv4 explicitly: this environment has no IPv6 (os error 97).
    let identity = Identity::self_signed(["localhost", "127.0.0.1"]).expect("identity");
    let cert_hash = hex::encode(identity.certificate_chain().as_slice()[0].hash().as_ref());

    let config = ServerConfig::builder()
        .with_bind_address(SocketAddr::from((Ipv4Addr::LOCALHOST, port)))
        .with_identity(identity)
        .build();
    let server = Endpoint::server(config).expect("endpoint");
    let bound = server.local_addr().expect("local addr").port();

    // The announce line the client/test reads.
    println!("{}", serde_json::json!({ "port": bound, "cert_hash": cert_hash }));
    use std::io::Write as _;
    std::io::stdout().flush().ok();

    let store: Store = Arc::new(Mutex::new(HashMap::new()));
    loop {
        let incoming = server.accept().await;
        tokio::spawn(handle_session(incoming, Arc::clone(&store)));
    }
}

async fn handle_session(incoming: IncomingSession, store: Store) {
    let Ok(request) = incoming.await else { return };
    let Ok(conn) = request.accept().await else { return };
    // One bidirectional stream per request (PRED-WT2).
    loop {
        let Ok(stream) = conn.accept_bi().await else { return };
        tokio::spawn(handle_request(stream, Arc::clone(&store)));
    }
}

async fn handle_request(
    (mut send, mut recv): (wtransport::SendStream, wtransport::RecvStream),
    store: Store,
) {
    let Ok(request) = read_json::<_, Request>(&mut recv).await else {
        return;
    };

    // The single flat refusal (asserted byte-identical across causes).
    let refuse = Response {
        ok: false,
        error: Some(REFUSED.to_string()),
        seqs: Vec::new(),
    };

    match request {
        Request::RosterAdd { group, member } => {
            store
                .lock()
                .expect("lock")
                .entry(group)
                .or_default()
                .roster
                .insert(member);
            let _ = write_json(&mut send, &Response { ok: true, error: None, seqs: Vec::new() }).await;
        }
        Request::RosterRemove { group, member } => {
            if let Some(g) = store.lock().expect("lock").get_mut(&group) {
                g.roster.remove(&member);
            }
            let _ = write_json(&mut send, &Response { ok: true, error: None, seqs: Vec::new() }).await;
        }
        Request::Put { group, seq, member } => {
            let Ok(blob) = read_frame(&mut recv).await else { return };
            let allowed = {
                let mut store = store.lock().expect("lock");
                let allowed = store
                    .get(&group)
                    .is_some_and(|g| g.roster.contains(&member));
                if allowed {
                    store
                        .get_mut(&group)
                        .expect("present")
                        .blobs
                        .push((seq, blob));
                }
                allowed
            };
            if allowed {
                let _ = write_json(&mut send, &Response { ok: true, error: None, seqs: Vec::new() }).await;
            } else {
                let _ = write_json(&mut send, &refuse).await;
            }
        }
        Request::Fetch {
            group,
            from_seq,
            member,
        } => {
            let offered: Option<Vec<(u64, Vec<u8>)>> = {
                let store = store.lock().expect("lock");
                store.get(&group).and_then(|g| {
                    // Offer-gate: only a roster member is offered anything;
                    // an unknown group falls through to the SAME refusal.
                    g.roster.contains(&member).then(|| {
                        g.blobs
                            .iter()
                            .filter(|(s, _)| *s >= from_seq)
                            .cloned()
                            .collect()
                    })
                })
            };
            match offered {
                Some(blobs) => {
                    let header = Response {
                        ok: true,
                        error: None,
                        seqs: blobs.iter().map(|(s, _)| *s).collect(),
                    };
                    if write_json(&mut send, &header).await.is_err() {
                        return;
                    }
                    for (_, blob) in &blobs {
                        if write_frame(&mut send, blob).await.is_err() {
                            return;
                        }
                    }
                }
                None => {
                    let _ = write_json(&mut send, &refuse).await;
                }
            }
        }
    }
    let _ = send.finish().await;
}
