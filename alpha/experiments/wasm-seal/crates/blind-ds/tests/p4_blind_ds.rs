//! P4 — real QUIC through the content-blind DS (red-first).
//!
//! The server is spawned as a separate PROCESS (its own binary — the role
//! discipline), the client is the native WebTransport client speaking the
//! browser-identical protocol. All traffic is real MLS ciphertext from the
//! P1 stack.
//!
//! Prediction pins (PRED-WT, written RED-first):
//! - PRED-WT1: the session is HTTP/3-CONNECT-established over QUIC on
//!   localhost, and the client connects ONLY when the pinned SHA-256 cert
//!   hash matches (`serverCertificateHashes` parity); a wrong hash is
//!   refused at the TLS layer, before any request.
//! - PRED-WT2: one bidirectional stream per request; `u32-LE || JSON`
//!   header + `u32-LE || bytes` payload frames both ways.
//! - PRED-WT3: `Identity::self_signed` mints an exactly-14-day certificate
//!   (wtransport 0.7.1 `validity_days(14)`) — the same ≤2-week cap the
//!   browser imposes on `serverCertificateHashes` certs, so the dev-trust
//!   path is browser-parity by construction.

use std::process::{Child, Command, Stdio};
use std::io::{BufRead as _, BufReader};

use ds_client::DsClient;
use group_seal::Sealer;

/// The spawned DS process + its self-announced port and cert hash.
struct DsProcess {
    child: Child,
    url: String,
    cert_hash: String,
}

impl DsProcess {
    fn spawn() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_blind-ds"))
            .arg("--port")
            .arg("0")
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("spawn blind-ds");
        let stdout = child.stdout.take().expect("stdout");
        let line = BufReader::new(stdout)
            .lines()
            .next()
            .expect("announce line")
            .expect("announce io");
        let v: serde_json::Value = serde_json::from_str(&line).expect("announce json");
        let port = v["port"].as_u64().expect("port");
        let cert_hash = v["cert_hash"].as_str().expect("cert_hash").to_string();
        Self {
            child,
            url: format!("https://127.0.0.1:{port}"),
            cert_hash,
        }
    }
}

impl Drop for DsProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

/// A founded two-member group producing real sealed frames for the wire.
fn sealed_pair() -> (Sealer, Sealer) {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let kp = bob.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&[kp]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");
    (alice, bob)
}

fn msg(sender: &str, text: &str) -> group_core::ChatMessage {
    group_core::ChatMessage {
        sender: sender.to_string(),
        text: text.to_string(),
    }
}

/// PRED-WT1: right hash connects; wrong hash is refused before any request.
#[tokio::test]
async fn connect_requires_pinned_cert_hash() {
    let ds = DsProcess::spawn();
    assert!(DsClient::connect(&ds.url, &ds.cert_hash).await.is_ok());

    let mut wrong = hex::decode(&ds.cert_hash).expect("hex");
    wrong[0] ^= 0xff;
    assert!(
        DsClient::connect(&ds.url, &hex::encode(wrong)).await.is_err(),
        "a non-pinned certificate must be refused (serverCertificateHashes parity)"
    );
}

/// The offer-gated store: a member's sealed frame rides QUIC up and back
/// byte-identically, and only a member is offered it.
#[tokio::test]
async fn member_offered_nonmember_refused_over_quic() {
    let ds = DsProcess::spawn();
    let client = DsClient::connect(&ds.url, &ds.cert_hash).await.expect("connect");

    client.roster_add("g1", "did:example:alice").await.expect("roster");
    client.roster_add("g1", "did:example:bob").await.expect("roster");

    let (mut alice, mut bob) = sealed_pair();
    let m = msg("alice", "sealed, stored blind, fetched over QUIC");
    let sealed = alice.seal(&m).expect("seal");

    client
        .put("g1", 1, "did:example:alice", &sealed)
        .await
        .expect("member put");

    // Offered to the member, byte-identical, and it still unseals.
    let got = client.fetch("g1", 0, "did:example:bob").await.expect("member fetch");
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].1, sealed, "the DS must return the ciphertext untouched");
    assert_eq!(bob.open(&got[0].1).expect("open").text, m.text);

    // Refused to the outsider — put and fetch both.
    assert!(client
        .put("g1", 2, "did:example:mallory", &sealed)
        .await
        .is_err());
    assert!(client.fetch("g1", 0, "did:example:mallory").await.is_err());
}

/// EXP-B refusal parity: non-member on a real group and anyone on a
/// nonexistent group get byte-identical flat refusals — no existence leak.
#[tokio::test]
async fn refusals_are_flat_and_identical() {
    let ds = DsProcess::spawn();
    let client = DsClient::connect(&ds.url, &ds.cert_hash).await.expect("connect");
    client.roster_add("g1", "did:example:alice").await.expect("roster");

    let e1 = client
        .fetch("g1", 0, "did:example:mallory")
        .await
        .expect_err("non-member must be refused");
    let e2 = client
        .fetch("no-such-group", 0, "did:example:mallory")
        .await
        .expect_err("unknown group must be refused");
    assert_eq!(
        e1.to_string(),
        e2.to_string(),
        "refusals must not distinguish existence from membership"
    );
}

/// Offering vs reading, across the wire: roster removal stops future
/// offering; it cannot reach into what was already fetched.
#[tokio::test]
async fn roster_removal_stops_offering_not_reading() {
    let ds = DsProcess::spawn();
    let client = DsClient::connect(&ds.url, &ds.cert_hash).await.expect("connect");
    client.roster_add("g1", "did:example:alice").await.expect("roster");
    client.roster_add("g1", "did:example:bob").await.expect("roster");

    let (mut alice, mut bob) = sealed_pair();
    let sealed = alice.seal(&msg("alice", "before removal")).expect("seal");
    client.put("g1", 1, "did:example:alice", &sealed).await.expect("put");

    let fetched = client.fetch("g1", 0, "did:example:bob").await.expect("fetch");
    client.roster_remove("g1", "did:example:bob").await.expect("remove");

    // Offering stops…
    assert!(client.fetch("g1", 0, "did:example:bob").await.is_err());
    // …but the already-fetched ciphertext + a held key still read.
    assert_eq!(bob.open(&fetched[0].1).expect("open").text, "before removal");
}

/// The blindness guard at the source level (mirrors the RUN-14 EXP-C scan):
/// the server's own source must contain no seal/unseal vocabulary at all.
/// (The dependency-graph half is `cargo tree -p blind-ds -e normal` —
/// recorded by `make p4-blindness`.)
#[test]
fn blindness_source_scan_guard() {
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let forbidden = ["openmls", "group_seal", "group-seal", "Sealer", "unseal", "decrypt"];
    for entry in std::fs::read_dir(&src_dir).expect("src dir") {
        let path = entry.expect("entry").path();
        let text = std::fs::read_to_string(&path).expect("read");
        for token in forbidden {
            assert!(
                !text.contains(token),
                "{} contains forbidden token {token:?} — the DS must be content-blind",
                path.display()
            );
        }
    }
}

/// Sanity magnitude (NOT a benchmark): median/p90 of a put+fetch roundtrip
/// over real localhost QUIC, printed for the summary.
#[tokio::test]
async fn latency_sanity_magnitude() {
    let ds = DsProcess::spawn();
    let client = DsClient::connect(&ds.url, &ds.cert_hash).await.expect("connect");
    client.roster_add("g1", "did:example:alice").await.expect("roster");

    let (mut alice, _bob) = sealed_pair();
    let sealed = alice.seal(&msg("alice", "latency probe")).expect("seal");

    let mut samples = Vec::with_capacity(50);
    for i in 0..50u64 {
        let t0 = std::time::Instant::now();
        client.put("g1", i, "did:example:alice", &sealed).await.expect("put");
        let got = client.fetch("g1", i, "did:example:alice").await.expect("fetch");
        assert!(!got.is_empty());
        samples.push(t0.elapsed());
    }
    samples.sort();
    println!(
        "P4 LATENCY put+fetch roundtrip over localhost QUIC (n=50, {}B blob): median {:?}, p90 {:?}",
        sealed.len(),
        samples[25],
        samples[45]
    );
}
