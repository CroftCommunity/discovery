//! Workstream C part 2 — MLS key-distribution over the wire.
//!
//! The faithful messaging crate models the verifying-key registry / group keys as *agreed state*
//! ("MLS would distribute it"). This spike makes that real: a founder creates a REAL openmls group
//! (via `lineage-mls`), and the **openmls Welcome** — the message that carries the encrypted group
//! secrets to a new member — travels a **real iroh connection** (homed on a real relay) to the joiner,
//! who runs `join_from_welcome` and derives the **same MLS exporter secret** (`epoch_proof`) as the
//! founder. That equality, computed only from a wire-delivered Welcome, is the proof the group key was
//! distributed over the transport, not assumed.
//!
//! Single process, real iroh + real relay (loopback), deterministic. Spike-class.
//! NOTE: the joiner's KeyPackage (public pre-key material) is handed to the founder in-process; the
//! security-relevant *key distribution* is the Welcome, which is what crosses the wire.

mod node;
mod relay;

use std::net::{IpAddr, Ipv4Addr};

use anyhow::{Context, Result};
use lineage_core::ids::Did;
use lineage_core::keys::SigningIdentity;
use lineage_mls::{Device, LineageClaim};
use serde::Serialize;

use crate::node::{build_endpoint, ALPN};
use crate::relay::{spawn as spawn_relay, RelayPorts};

#[derive(Serialize)]
struct Verdict {
    welcome_bytes_over_wire: usize,
    founder_epoch: u64,
    joiner_epoch: u64,
    founder_member_count: usize,
    joiner_member_count: usize,
    epoch_secret_match: bool,
    joiner_secret_is_nonzero: bool,
    /// The lineage-folded membership/standing the joiner derives FROM the wire-distributed MLS group.
    founder_fold: std::collections::BTreeMap<String, usize>,
    joiner_fold: std::collections::BTreeMap<String, usize>,
    fold_matches: bool,
    verdict: &'static str,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let loopback = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let ports = RelayPorts { http: 3340, https: 3343, quic: 3478, metrics: 9099 };
    let (_relay, relay_url) = spawn_relay(loopback, loopback, ports)
        .await
        .context("spawn loopback relay")?;

    // Two endpoints homed on the relay.
    let founder_ep =
        build_endpoint("127.0.0.1:0".parse()?, &relay_url, Some(3478), None, false).await?;
    let joiner_ep =
        build_endpoint("127.0.0.1:0".parse()?, &relay_url, Some(3478), None, false).await?;
    founder_ep.online().await;
    joiner_ep.online().await;
    let joiner_addr = joiner_ep.addr();

    // --- Build a REAL openmls group whose leaves carry T1 lineage claims, so membership/standing is
    //     derivable from the group state itself (not a modeled registry). alice has TWO devices in one
    //     lineage (they must fold to ONE actor, E2.9/E2.10); bob is a second lineage. ---
    let root_a = SigningIdentity::from_seed(Did::new("lineage-a"), 10);
    let root_b = SigningIdentity::from_seed(Did::new("lineage-b"), 11);
    let claim = |root: &SigningIdentity, dev: &str| LineageClaim::sign(root, &Did::new(dev));

    let mut founder = Device::new_with_lineage(
        Did::new("alice.laptop"),
        claim(&root_a, "alice.laptop"),
    )?;
    let mut joiner = Device::new_with_lineage(
        Did::new("alice.phone"),
        claim(&root_a, "alice.phone"),
    )?;
    let bob = Device::new_with_lineage(Did::new("bob.phone"), claim(&root_b, "bob.phone"))?;

    founder.create_group()?;
    let joiner_kp = joiner.key_package()?; // public pre-keys (handed in-process)
    let bob_kp = bob.key_package()?;
    let (_commit, welcome_bytes) = founder.add(&[joiner_kp, bob_kp])?; // founder now at the new epoch
    let founder_secret = founder.epoch_proof()?;

    // --- Carry the Welcome over a REAL iroh connection (founder -> joiner). ---
    let joiner_accept = tokio::spawn(async move {
        let incoming = joiner_ep.accept().await.context("joiner: no incoming")?;
        let conn = incoming.accept()?.await.context("joiner: handshake")?;
        let (_send, mut recv) = conn.accept_bi().await.context("joiner: accept_bi")?;
        let bytes = recv
            .read_to_end(8 * 1024 * 1024)
            .await
            .context("joiner: read welcome")?;
        Ok::<Vec<u8>, anyhow::Error>(bytes)
    });

    let conn = founder_ep
        .connect(joiner_addr, ALPN)
        .await
        .context("founder: connect to joiner")?;
    let (mut send, _recv) = conn.open_bi().await.context("founder: open_bi")?;
    send.write_all(&welcome_bytes)
        .await
        .context("founder: send welcome")?;
    send.finish().context("founder: finish")?;

    let wire_welcome = joiner_accept.await.context("join task")??;
    anyhow::ensure!(wire_welcome == welcome_bytes, "welcome corrupted on the wire");

    // --- Joiner installs the wire-delivered Welcome and derives the group secret. ---
    joiner.join_from_welcome(&wire_welcome, None)?;
    let joiner_secret = joiner.epoch_proof()?;

    // --- The binding: the joiner derives the lineage-folded MEMBERSHIP/STANDING from the MLS group it
    //     got over the wire (fold_by_lineage reads the leaf credentials). alice's two devices fold to
    //     one actor; bob is a second. This registry is now MLS-distributed, not modeled agreed state. ---
    let fold_sizes = |d: &Device| -> std::collections::BTreeMap<String, usize> {
        d.fold_by_lineage()
            .unwrap_or_default()
            .into_iter()
            .map(|(k, leaves)| (k, leaves.len()))
            .collect()
    };
    let founder_fold = fold_sizes(&founder);
    let joiner_fold = fold_sizes(&joiner);
    let fold_matches = founder_fold == joiner_fold;
    // alice's lineage must fold its two devices to one actor (2 leaves under one lineage key).
    let alice_two_devices_one_actor = joiner_fold.values().any(|&n| n == 2);
    let two_actors = joiner_fold.len() == 2;

    let secret_match = founder_secret == joiner_secret;
    let nonzero = joiner_secret.iter().any(|&b| b != 0);
    let pass = secret_match && nonzero && fold_matches && alice_two_devices_one_actor && two_actors;
    let v = Verdict {
        welcome_bytes_over_wire: wire_welcome.len(),
        founder_epoch: founder.epoch()?,
        joiner_epoch: joiner.epoch()?,
        founder_member_count: founder.member_count()?,
        joiner_member_count: joiner.member_count()?,
        epoch_secret_match: secret_match,
        joiner_secret_is_nonzero: nonzero,
        founder_fold,
        joiner_fold,
        fold_matches,
        verdict: if pass {
            "PASS — joiner derived BOTH the MLS group secret AND the lineage-folded standing from a wire-delivered Welcome"
        } else {
            "FAIL"
        },
    };
    println!("{}", serde_json::to_string_pretty(&v)?);
    conn.close(0u32.into(), b"done");
    anyhow::ensure!(pass, "MLS key-dist + standing-binding over the wire failed");
    eprintln!("MLS-WELCOME-OVER-IROH PASS");
    Ok(())
}
