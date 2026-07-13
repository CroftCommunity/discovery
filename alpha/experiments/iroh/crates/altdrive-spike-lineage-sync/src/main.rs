//! Throwaway spike (T2g / MD-G2 / MD-G4 / MD-G5) — carry **lineage history
//! branches** over a per-group iroh-gossip topic; each receiver verifies them and
//! folds them by lineage into actors (multi-device fold), and honours signed
//! device revocations. This is the *transport* form of the green-real
//! `fold_by_lineage` (Proofs E2.9 / C4) and the revocation honesty in E2.11.
//!
//! Honesty boundary: the integrity check here is a sha-256 **hash chain** (each
//! message commits to the prior) plus a sha-256 **MAC** on the revoke marker
//! (keyed by the group/lineage genesis) — this proves a branch was not tampered
//! in transit, its ordering is contiguous, and a revoke is well-formed and
//! group-scoped. The Ed25519 **signature / standing / authority** check is the
//! green-real half in `Proofs/lineage-groups` (E2.7 / E2.11 / E2.12) and is not
//! re-implemented here. Carrying the real signed `lineage-history::Message`
//! end-to-end is the separate "faithful" follow-on.
//!
//! Topic model (MD-G4): the gossip topic is derived from the **group** id, so all
//! devices of all member lineages share one topic. Each `Branch` carries a
//! distinct `device_did` and the actor's `lineage_id`; receivers fold absorbed
//! branches by `lineage_id` — alice's two devices collapse to one actor, bob is a
//! second. (Contrast the earlier per-lineage-topic spike MD-G1/G2.)
//!
//! MD-G5: a `Revoke { target_device }` marker, MAC'd to the group genesis, is
//! broadcast by a member. After a node sees the revoke, the target device's
//! *subsequent* branches are REJECTed `(revoked)` and the device cannot re-enter
//! the accepted set; branches absorbed *before* the revoke are retained (history
//! is not clawed back — E2.11 honesty).
//!
//! Usage:
//!   altdrive-spike-lineage-sync <device-did> <lineage-id> <group-id> <self-addr-out> \
//!       [--revoke <target-device-did>] [bootstrap.json ...]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use iroh::{
    address_lookup::MemoryLookup, endpoint::presets, protocol::Router, Endpoint, EndpointAddr,
    EndpointId,
};
use iroh_gossip::{api::Event, net::Gossip, proto::TopicId, ALPN as GOSSIP_ALPN};
use n0_future::StreamExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const PORT: u16 = 2112;
const ROUNDS: usize = 18;
/// Round at which a `--revoke` node broadcasts its revoke marker (MD-G5). Early
/// enough that survivors have absorbed the target's pre-revoke branch first, and
/// late enough that several post-revoke broadcasts are rejected before exit.
const REVOKE_AT_ROUND: usize = 6;

/// One message in a device's branch. `hash` chains to the previous message.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Msg {
    seq: u64,
    payload: String,
    hash: [u8; 32],
}

/// A device's hash-chained history branch on a lineage, within a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Branch {
    group_id: String,
    lineage_id: String,
    device_did: String,
    msgs: Vec<Msg>,
}

/// A signed (MAC'd) device-revocation marker (MD-G5). Carries no content — only
/// the target device id and the issuer, bound to the group genesis by `mac`.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Revoke {
    group_id: String,
    by_device: String,
    target_device: String,
    mac: [u8; 32],
}

/// The wire envelope: a node broadcasts either a branch or a revoke.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum Wire {
    Branch(Branch),
    Revoke(Revoke),
}

/// Genesis hash for a lineage — the per-actor anchor every device of that lineage
/// chains its branch from.
fn lineage_genesis(lineage_id: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"croft-lineage-genesis:");
    h.update(lineage_id.as_bytes());
    h.finalize().into()
}

/// Genesis hash for a group — the shared anchor the topic and the revoke MAC are
/// derived from.
fn group_genesis(group_id: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"croft-group-genesis:");
    h.update(group_id.as_bytes());
    h.finalize().into()
}

fn topic_for(group_id: &str) -> TopicId {
    // MD-G4: the gossip topic is derived from the group (not the lineage), so all
    // member lineages share it.
    let mut h = Sha256::new();
    h.update(b"croft-group-topic:");
    h.update(group_id.as_bytes());
    TopicId::from_bytes(h.finalize().into())
}

fn chain_hash(prev: &[u8; 32], seq: u64, payload: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(prev);
    h.update(seq.to_le_bytes());
    h.update(payload.as_bytes());
    h.finalize().into()
}

/// MAC for a revoke marker: commits the target device to the group genesis. The
/// structural stand-in for the green-real Ed25519 authority check (E2.11).
fn revoke_mac(group_id: &str, target_device: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"croft-revoke-mac:");
    h.update(group_genesis(group_id));
    h.update(target_device.as_bytes());
    h.finalize().into()
}

/// Build a device's branch of `n` messages, chained from the lineage genesis.
fn build_branch(group_id: &str, lineage_id: &str, device_did: &str, n: u64) -> Branch {
    let mut msgs = Vec::new();
    let mut prev = lineage_genesis(lineage_id);
    for seq in 0..n {
        let payload = format!("{device_did} entry {seq}");
        let hash = chain_hash(&prev, seq, &payload);
        msgs.push(Msg { seq, payload, hash });
        prev = hash;
    }
    Branch {
        group_id: group_id.to_string(),
        lineage_id: lineage_id.to_string(),
        device_did: device_did.to_string(),
        msgs,
    }
}

/// Verify a received branch: shared group + contiguous seqs + intact hash chain
/// (the structural half of backfill; Ed25519/standing is green-real in Proofs).
fn verify_branch(b: &Branch, my_group: &str) -> std::result::Result<(), String> {
    if b.group_id != my_group {
        return Err(format!("foreign group {}", b.group_id));
    }
    let mut prev = lineage_genesis(&b.lineage_id);
    for (i, m) in b.msgs.iter().enumerate() {
        if m.seq != i as u64 {
            return Err(format!("non-contiguous at index {i} (seq {})", m.seq));
        }
        let expect = chain_hash(&prev, m.seq, &m.payload);
        if expect != m.hash {
            return Err(format!("broken hash chain at seq {} (tampered)", m.seq));
        }
        prev = m.hash;
    }
    Ok(())
}

/// Verify a revoke marker: shared group + intact MAC.
fn verify_revoke(r: &Revoke, my_group: &str) -> std::result::Result<(), String> {
    if r.group_id != my_group {
        return Err(format!("foreign group {}", r.group_id));
    }
    if revoke_mac(&r.group_id, &r.target_device) != r.mac {
        return Err("bad revoke MAC".to_string());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let raw: Vec<String> = std::env::args().skip(1).collect();
    // Split out the optional `--revoke <target>` flag, keep the rest positional.
    let mut args: Vec<String> = Vec::new();
    let mut revoke_target: Option<String> = None;
    let mut it = raw.into_iter();
    while let Some(a) = it.next() {
        if a == "--revoke" {
            revoke_target = it.next();
        } else {
            args.push(a);
        }
    }
    if args.len() < 4 {
        eprintln!("usage: altdrive-spike-lineage-sync <device-did> <lineage-id> <group-id> <self-addr-out> [--revoke <target-device>] [bootstrap.json ...]");
        std::process::exit(2);
    }
    let device = args[0].clone();
    let lineage_id = args[1].clone();
    let group_id = args[2].clone();
    let self_out = &args[3];
    let bootstrap_files = &args[4..];

    let topic = topic_for(&group_id);

    let mut boot_addrs: Vec<EndpointAddr> = Vec::new();
    for f in bootstrap_files {
        let s = std::fs::read_to_string(f).with_context(|| format!("read {f}"))?;
        boot_addrs.push(serde_json::from_str(&s).with_context(|| format!("parse {f}"))?);
    }
    let boot_ids: Vec<EndpointId> = boot_addrs.iter().map(|a| a.id).collect();

    let builder = Endpoint::builder(presets::N0)
        .clear_ip_transports()
        .bind_addr(format!("0.0.0.0:{PORT}"))
        .context("bind UDP/v4 2112")?;
    let builder = if boot_addrs.is_empty() {
        builder
    } else {
        builder.address_lookup(MemoryLookup::from_endpoint_info(boot_addrs.clone()))
    };
    let endpoint = builder.bind().await.map_err(|e| anyhow!("endpoint bind: {e}"))?;
    endpoint.online().await;

    let my_addr = endpoint.addr();
    std::fs::write(self_out, serde_json::to_string(&my_addr)?)
        .with_context(|| format!("write {self_out}"))?;
    println!(
        "[{device}] group={group_id} lineage={lineage_id} id={} addr->{self_out}{}",
        endpoint.id(),
        revoke_target
            .as_ref()
            .map(|t| format!(" (REVOKER of {t})"))
            .unwrap_or_default()
    );

    let gossip = Gossip::builder().spawn(endpoint.clone());
    let _router = Router::builder(endpoint.clone()).accept(GOSSIP_ALPN, gossip.clone()).spawn();
    let topic_sub = gossip.subscribe(topic, boot_ids).await?;
    let (sender, mut receiver) = topic_sub.split();

    // Fold-by-lineage: actor lineage_id -> { device_did -> msg count }. Seeded with
    // our own branch so every node counts itself as an actor (MD-G4).
    let folded: Arc<Mutex<BTreeMap<String, BTreeMap<String, usize>>>> =
        Arc::new(Mutex::new(BTreeMap::new()));
    folded
        .lock()
        .expect("lock")
        .entry(lineage_id.clone())
        .or_default()
        .insert(device.clone(), 3);
    // Devices we have seen a valid revoke for (MD-G5).
    let revoked: Arc<Mutex<BTreeSet<String>>> = Arc::new(Mutex::new(BTreeSet::new()));
    let rejects: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let (fold_r, rev_r, rej_r, my_grp, dname) = (
        folded.clone(),
        revoked.clone(),
        rejects.clone(),
        group_id.clone(),
        device.clone(),
    );

    tokio::spawn(async move {
        while let Some(ev) = receiver.next().await {
            match ev {
                Ok(Event::Received(msg)) => match serde_json::from_slice::<Wire>(&msg.content) {
                    Ok(Wire::Branch(b)) if b.device_did == dname => { /* our own echo */ }
                    Ok(Wire::Branch(b)) => {
                        if rev_r.lock().expect("lock").contains(&b.device_did) {
                            rej_r
                                .lock()
                                .expect("lock")
                                .push(format!("{}: revoked", b.device_did));
                            println!("[{dname}] REJECT branch device={} — (revoked)", b.device_did);
                            continue;
                        }
                        match verify_branch(&b, &my_grp) {
                            Ok(()) => {
                                fold_r
                                    .lock()
                                    .expect("lock")
                                    .entry(b.lineage_id.clone())
                                    .or_default()
                                    .insert(b.device_did.clone(), b.msgs.len());
                                println!(
                                    "[{dname}] ABSORB device={} lineage={} msgs={} (folded)",
                                    b.device_did,
                                    b.lineage_id,
                                    b.msgs.len()
                                );
                            }
                            Err(why) => {
                                rej_r.lock().expect("lock").push(format!("{}: {why}", b.device_did));
                                println!("[{dname}] REJECT branch device={} — {why}", b.device_did);
                            }
                        }
                    }
                    Ok(Wire::Revoke(r)) => match verify_revoke(&r, &my_grp) {
                        Ok(()) => {
                            rev_r.lock().expect("lock").insert(r.target_device.clone());
                            if r.target_device == dname {
                                println!(
                                    "[{dname}] received REVOKE of SELF by={} — cannot re-enter accepted set",
                                    r.by_device
                                );
                            } else {
                                println!(
                                    "[{dname}] REVOKE target={} by={} (pre-revoke branches retained)",
                                    r.target_device, r.by_device
                                );
                            }
                        }
                        Err(why) => {
                            println!("[{dname}] REJECT revoke target={} — {why}", r.target_device);
                        }
                    },
                    Err(e) => println!("[{dname}] undecodable payload: {e}"),
                },
                Ok(Event::NeighborUp(id)) => println!("[{dname}] NeighborUp {}", id.fmt_short()),
                Ok(Event::NeighborDown(id)) => println!("[{dname}] NeighborDown {}", id.fmt_short()),
                Ok(_) => {}
                Err(e) => {
                    println!("[{dname}] recv error: {e}");
                    break;
                }
            }
        }
    });

    // Our honest branch. In MD-G4 (no revoke flag) we also broadcast a deliberately
    // TAMPERED branch (stale hash) to prove the chain check still rejects it — it
    // never enters the fold.
    let good = Wire::Branch(build_branch(&group_id, &lineage_id, &device, 3));
    let good_bytes = serde_json::to_vec(&good)?;

    let tampered_bytes = if revoke_target.is_none() {
        let mut t = build_branch(&group_id, &lineage_id, &format!("{device}.TAMPER"), 3);
        if let Some(m) = t.msgs.get_mut(1) {
            m.payload = "SECRETLY ALTERED".to_string(); // hash now stale
        }
        Some(serde_json::to_vec(&Wire::Branch(t))?)
    } else {
        None
    };

    let revoke_bytes = revoke_target.as_ref().map(|target| {
        let r = Revoke {
            group_id: group_id.clone(),
            by_device: device.clone(),
            target_device: target.clone(),
            mac: revoke_mac(&group_id, target),
        };
        serde_json::to_vec(&Wire::Revoke(r)).expect("encode revoke")
    });

    for i in 0..ROUNDS {
        tokio::time::sleep(Duration::from_secs(2)).await;
        if let Err(e) = sender.broadcast(good_bytes.clone().into()).await {
            println!("[{device}] broadcast round {i} error: {e}");
        }
        if let Some(t) = &tampered_bytes {
            if i % 5 == 2 {
                let _ = sender.broadcast(t.clone().into()).await;
            }
        }
        if let (Some(rb), Some(target)) = (&revoke_bytes, &revoke_target) {
            if i >= REVOKE_AT_ROUND {
                // Rebroadcast every round after the threshold: a single gossip
                // broadcast can be lost (branches survive only because they repeat),
                // and a revocation marker is persistent, not one-shot. Idempotent on
                // receivers. The issuer also self-applies so it stops absorbing the
                // target's later branches and reports the revocation.
                if i == REVOKE_AT_ROUND {
                    revoked.lock().expect("lock").insert(target.clone());
                    println!("[{device}] BROADCAST revoke target={target} at round {i} (self-applied)");
                }
                let _ = sender.broadcast(rb.clone().into()).await;
            }
        }
    }
    tokio::time::sleep(Duration::from_secs(2)).await;

    let fold = folded.lock().expect("lock").clone();
    let rej = rejects.lock().expect("lock").clone();
    let rev = revoked.lock().expect("lock").clone();
    let actor_view: BTreeMap<String, Vec<String>> = fold
        .iter()
        .map(|(lin, devs)| (lin.clone(), devs.keys().cloned().collect()))
        .collect();
    println!(
        "[{device}] SUMMARY folded_actors={} actors={:?} revoked={:?} rejected={}",
        actor_view.len(),
        actor_view,
        rev,
        rej.len()
    );
    Ok(())
}
