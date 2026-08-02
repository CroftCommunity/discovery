//! The local live leg: a real `iroh-relay` server gated by our admission.
//!
//! Everything above this test was app-side or used a synthetic `ClientRequest`.
//! Here a real `iroh_relay::server::Server` runs on localhost with our
//! `TokenAccess` as its `AccessControl`, and real `iroh_relay::client::Client`s
//! connect through it. This proves three things end to end, over the wire:
//!
//!   1. A client presenting a valid croft-admit token is **admitted**; one with
//!      a bogus or expired token is **denied at the handshake**.
//!   2. Endpoint A's datagram **reaches endpoint B through our relay** — real
//!      forwarded traffic, gated by our token check.
//!   3. A byte-accounting of a minimal relayed contact exchange, as a datapoint
//!      for the Phase-3 coordination-bucket calibration (see the caveat in the
//!      byte-accounting test and ADR-0004 — this is a localhost relayed-payload
//!      figure, not the full holepunch-disco total, which still needs two
//!      NAT'd iroh endpoints).
//!
//! The server/client wiring mirrors iroh-relay's own `tests/runtime_auth.rs`.
//! (This crate always enables iroh-relay's `server` feature, so no cfg gate.)

use std::net::Ipv4Addr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use iroh_base::{RelayUrl, SecretKey};
use iroh_dns::dns::DnsResolver;
use iroh_relay::client::{Client, ClientBuilder, ConnectError};
use iroh_relay::protos::relay::{ClientToRelayMsg, Datagrams, RelayToClientMsg};
use iroh_relay::server::{RelayConfig, Server, ServerConfig};
use iroh_relay::tls::{default_provider, CaTlsConfig};
use n0_future::{SinkExt, StreamExt};
use ring::signature::{Ed25519KeyPair, KeyPair};

use croft_admit::endpoint_id::EndpointId as AdmitId;
use croft_admit::tier::Tier;
use croft_admit::token::{TokenIssuer, TokenVerifier};
use croft_relay_embed::TokenAccess;

const ISS: &str = "https://admit.croft.ing";
const TTL: u64 = 900;
const LEEWAY: u64 = 30;

struct Enrollment {
    issuer: TokenIssuer,
    public_raw: Vec<u8>,
}
fn enrollment() -> Enrollment {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let kp = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    Enrollment {
        issuer: TokenIssuer::new(pkcs8.as_ref(), ISS, TTL),
        public_raw: kp.public_key().as_ref().to_vec(),
    }
}
impl Enrollment {
    fn verifier(&self) -> TokenVerifier {
        TokenVerifier::new(&self.public_raw, ISS, LEEWAY)
    }
    /// A token for `sk`'s endpoint at `tier`, valid now.
    fn token(&self, sk: &SecretKey, tier: Tier) -> String {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.issuer
            .mint(&AdmitId::from_bytes(*sk.public().as_bytes()), tier, now)
    }
}

fn secret(seed: u8) -> SecretKey {
    SecretKey::from_bytes(&[seed; 32])
}

/// Spawn a relay on `127.0.0.1:0` gated by our `TokenAccess`, return its URL.
async fn spawn_gated_relay(verifier: TokenVerifier) -> (Server, RelayUrl) {
    let access = Arc::new(TokenAccess::new(verifier));
    let mut relay = RelayConfig::new((Ipv4Addr::LOCALHOST, 0));
    relay.access = access;
    let mut config = ServerConfig::default();
    config.relay = Some(relay);
    let server = Server::spawn(config).await.expect("relay spawns");
    let url = format!("http://{}", server.http_addr().expect("http addr"))
        .parse()
        .expect("relay url");
    (server, url)
}

async fn connect(url: &RelayUrl, sk: &SecretKey, token: &str) -> Result<Client, ConnectError> {
    let tls = CaTlsConfig::default()
        .client_config(default_provider())
        .expect("client tls config");
    ClientBuilder::new(url.clone(), sk.clone(), DnsResolver::new())
        .tls_client_config(tls)
        .auth_token(token)
        .connect()
        .await
}

/// Send a relay ping and wait for its pong — a liveness check that the
/// admitted connection actually carries traffic.
async fn ping(client: &mut Client, data: [u8; 8]) {
    client
        .send(ClientToRelayMsg::Ping(data))
        .await
        .expect("send ping");
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match client.next().await.expect("stream open").expect("msg") {
                RelayToClientMsg::Pong(echo) => {
                    assert_eq!(echo, data, "pong payload mismatch");
                    break;
                }
                RelayToClientMsg::Ping(p) => {
                    client
                        .send(ClientToRelayMsg::Pong(p))
                        .await
                        .expect("send pong");
                }
                _ => {}
            }
        }
    })
    .await
    .expect("ping round trip timed out");
}

#[tokio::test]
async fn valid_token_admits_and_bogus_or_expired_is_denied() {
    let enr = enrollment();
    let (_server, url) = spawn_gated_relay(enr.verifier()).await;

    // A valid, endpoint-bound token is admitted and carries traffic.
    let a = secret(1);
    let good = enr.token(&a, Tier::Broker);
    let mut client = connect(&url, &a, &good)
        .await
        .expect("valid token admitted");
    ping(&mut client, [7u8; 8]).await;

    // A structurally bogus token is denied at the handshake.
    let bogus = connect(&url, &secret(2), "not-a-real-token").await;
    assert!(
        matches!(bogus, Err(ConnectError::Handshake { .. })),
        "bogus token must be denied at handshake, got {bogus:?}"
    );

    // A token minted for endpoint A but presented from endpoint C is denied
    // (the anti-replay hinge, now over the real relay handshake).
    let c = secret(3);
    let stolen = enr.token(&a, Tier::Broker);
    let replayed = connect(&url, &c, &stolen).await;
    assert!(
        matches!(replayed, Err(ConnectError::Handshake { .. })),
        "token replayed from another endpoint must be denied, got {replayed:?}"
    );
}

#[tokio::test]
async fn endpoint_a_reaches_endpoint_b_through_our_relay() {
    let enr = enrollment();
    let (_server, url) = spawn_gated_relay(enr.verifier()).await;

    let a = secret(10);
    let b = secret(11);
    let mut ca = connect(&url, &a, &enr.token(&a, Tier::Broker))
        .await
        .expect("A admitted");
    let mut cb = connect(&url, &b, &enr.token(&b, Tier::Broker))
        .await
        .expect("B admitted");

    // A sends a datagram addressed to B's endpoint id; the relay forwards it.
    let payload = Bytes::from_static(b"hello-from-a");
    ca.send(ClientToRelayMsg::Datagrams {
        dst_endpoint_id: b.public(),
        datagrams: Datagrams {
            ecn: None,
            segment_size: None,
            contents: payload.clone(),
        },
    })
    .await
    .expect("A sends to B");

    // B receives it, tagged with A's endpoint id as the origin.
    let got = tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            match cb.next().await.expect("B stream open").expect("B msg") {
                RelayToClientMsg::Datagrams {
                    remote_endpoint_id,
                    datagrams,
                } => break (remote_endpoint_id, datagrams.contents),
                _ => continue,
            }
        }
    })
    .await
    .expect("B receives A's datagram within timeout");

    assert_eq!(got.0, a.public(), "origin endpoint id");
    assert_eq!(got.1, payload, "relayed payload is intact");
}

#[tokio::test]
async fn coordination_bytes_datapoint_for_calibration() {
    // A minimal "make contact" exchange through the relay: A -> B a small
    // datagram, B -> A a small ack. We account the app-payload bytes each
    // endpoint pushes *into* the relay, which is what `[limits.client.rx]`
    // (the coordination bucket) meters.
    //
    // CAVEAT (ADR-0004 SPEC-DELTA stays open): this is a localhost, relay-client
    // figure for a single contact round-trip. It is NOT the full holepunch
    // disco total (that needs two iroh magicsock endpoints on separate NATs).
    // It bounds the exchange from below and confirms the sizing rationale; it
    // does not replace the placeholder constant.
    let enr = enrollment();
    let (_server, url) = spawn_gated_relay(enr.verifier()).await;

    let a = secret(20);
    let b = secret(21);
    let mut ca = connect(&url, &a, &enr.token(&a, Tier::Broker))
        .await
        .expect("A");
    let mut cb = connect(&url, &b, &enr.token(&b, Tier::Broker))
        .await
        .expect("B");

    let a_to_b = Bytes::from_static(b"syn"); // 3 bytes: a minimal contact probe
    let b_to_a = Bytes::from_static(b"ack"); // 3 bytes: its acknowledgement
    let mut a_pushed = 0usize;
    let mut b_pushed = 0usize;

    ca.send(ClientToRelayMsg::Datagrams {
        dst_endpoint_id: b.public(),
        datagrams: Datagrams {
            ecn: None,
            segment_size: None,
            contents: a_to_b.clone(),
        },
    })
    .await
    .unwrap();
    a_pushed += a_to_b.len();

    // B waits for it, then acks.
    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let RelayToClientMsg::Datagrams { .. } = cb.next().await.unwrap().unwrap() {
                break;
            }
        }
    })
    .await
    .unwrap();
    cb.send(ClientToRelayMsg::Datagrams {
        dst_endpoint_id: a.public(),
        datagrams: Datagrams {
            ecn: None,
            segment_size: None,
            contents: b_to_a.clone(),
        },
    })
    .await
    .unwrap();
    b_pushed += b_to_a.len();

    tokio::time::timeout(Duration::from_secs(5), async {
        loop {
            if let RelayToClientMsg::Datagrams { .. } = ca.next().await.unwrap().unwrap() {
                break;
            }
        }
    })
    .await
    .unwrap();

    let per_endpoint_max = a_pushed.max(b_pushed);
    // The coordination bucket must clear this round-trip with headroom, and it
    // does: 4 KiB/s is >~1000x a few-byte contact exchange, while sustained
    // media (>=24 kB/s) would exhaust it. This validates the sizing direction.
    let coord_bps = croft_admit::tier::bucket_for(Tier::Coordination)
        .bytes_per_second
        .unwrap();
    assert!(
        (per_endpoint_max as u64) < coord_bps,
        "a single contact round-trip ({per_endpoint_max} B/endpoint) must fit well inside the coordination bucket ({coord_bps} B/s)"
    );

    // Emit the datapoint so the run captures it (visible with `--nocapture`).
    println!(
        "CALIBRATION-DATAPOINT relayed_contact_roundtrip a_to_relay={a_pushed}B b_to_relay={b_pushed}B coord_bucket={coord_bps}B/s (localhost relay-client; not the holepunch total)"
    );
}
