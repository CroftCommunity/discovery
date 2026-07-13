//! Faithful end-to-end spike — carry the **real Ed25519-signed**
//! `lineage_history::Message` over live iroh-gossip and verify **signature AND
//! standing/authority** on receipt using the **real** `HistoryStore::backfill_import`
//! (Proofs E2.7/E2.12). This closes the honesty boundary every hash-chain transport
//! spike carried: those proved in-transit integrity + ordering but NOT *who wrote it*
//! — a valid hash chain claiming `device=alice` is forgeable. Here the bytes that are
//! signed-and-authority-checked in the Proofs model are the bytes on the wire, checked
//! on the wire by the same code.
//!
//! Every node broadcasts three canonical test vectors and evaluates the vectors it
//! receives from peers through the real backfill. The expected verdict on every node
//! (incl. the NAT'd Mac):
//!   - HONEST    (alice, a member, real signature)        -> ACCEPT
//!   - FORGED    (alice's branch, payload tampered)       -> REJECT BadSignature
//!   - NONMEMBER (mallory, VALID signature, no standing)  -> REJECT UnauthorizedAuthor
//! The third is the load-bearing case: the signature is real and verifies, yet the
//! author never held standing — the exact attack a hash chain cannot catch.
//!
//! Usage:
//!   altdrive-spike-faithful-sync <device> <group-id> <self-addr-out> [bootstrap.json ...]

use std::collections::{BTreeMap, HashMap};
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

use lineage_core::dag::Lineage;
use lineage_core::gov::{
    sign_op, Directory, Genesis, GenesisRules, GroupState as GovState, OpKind, SignedOp,
};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BranchHistory, HistoryStore, Message};

const PORT: u16 = 2112;
const ROUNDS: usize = 18;

/// One message on the wire — the real `Message`, field for field. `Sig` already
/// (de)serializes through hex; `Did`/`GenesisId` carry as string/array.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireMsg {
    author: String,
    seq: u64,
    branch: [u8; 32],
    payload: Vec<u8>,
    sig: Sig,
}

/// A branch of real signed messages, plus the vector label it demonstrates.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireBranch {
    vector: String,
    genesis: [u8; 32],
    msgs: Vec<WireMsg>,
}

/// A real threshold-signed governance op on the wire (Workstream C — retires the MD-G5 transport
/// MAC). Carries the genuine `gov::SignedOp` (a k-of-n Ed25519 bundle); the receiver verifies it with
/// the real `meets_threshold_by_lineage` against the group's replicated genesis rules + directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WireRevoke {
    vector: String,
    op: SignedOp,
    expect_accept: bool,
}

/// The tagged wire envelope: the faithful topic now carries both signed branches and signed
/// governance ops, each adjudicated by its own real verifier.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum WireItem {
    Branch(WireBranch),
    Revoke(WireRevoke),
}

fn gid(label: &str) -> GenesisId {
    GenesisId::from_bytes(label.as_bytes())
}

fn topic_for(group_id: &str) -> TopicId {
    TopicId::from_bytes(gid(&format!("croft-faithful-topic:{group_id}")).0)
}

/// The agreed group state every node holds (modelled exactly as the Proofs tests
/// build it; MLS would distribute it). alice + bob are members with standing;
/// mallory's branch claims descent from the root but mallory was never a member.
struct GroupState {
    lineage: Lineage,
    registry: HashMap<String, VerifyingIdentity>,
    root: GenesisId,
    g_alice: GenesisId,
    g_bob: GenesisId,
    g_forged: GenesisId,
    g_mallory: GenesisId,
    alice: SigningIdentity,
    mallory: SigningIdentity,
}

fn build_group(group_id: &str) -> GroupState {
    let root = gid(&format!("croft-faithful-root:{group_id}"));
    let g_alice = gid(&format!("branch:{group_id}:alice"));
    let g_bob = gid(&format!("branch:{group_id}:bob"));
    let g_forged = gid(&format!("branch:{group_id}:forged"));
    let g_mallory = gid(&format!("branch:{group_id}:mallory"));

    let alice_did = Did::new("alice");
    let bob_did = Did::new("bob");
    let mallory_did = Did::new("mallory");

    let alice = SigningIdentity::from_seed(alice_did.clone(), 1);
    let bob = SigningIdentity::from_seed(bob_did.clone(), 2);
    let mallory = SigningIdentity::from_seed(mallory_did.clone(), 3);

    // Every member's public key is known (distributed via the group) — including
    // mallory's: her signature WILL verify; she simply lacks standing.
    let mut registry = HashMap::new();
    registry.insert("alice".to_string(), alice.verifying());
    registry.insert("bob".to_string(), bob.verifying());
    registry.insert("mallory".to_string(), mallory.verifying());

    // The recorded lineage dag — the authority source (decided from recorded data,
    // never from a message's own assertion, I3).
    let mut lineage = Lineage::new();
    lineage.add_root(root, [alice_did.clone(), bob_did.clone()]);
    lineage.fork(root, g_alice, [alice_did.clone()]);
    lineage.fork(root, g_bob, [bob_did.clone()]);
    lineage.fork(root, g_forged, [alice_did.clone()]);
    // mallory's branch shares the root (so it is NOT a foreign-genesis rejection)
    // but records NO member — so standing() must deny it.
    lineage.fork(root, g_mallory, std::iter::empty::<Did>());

    GroupState {
        lineage,
        registry,
        root,
        g_alice,
        g_bob,
        g_forged,
        g_mallory,
        alice,
        mallory,
    }
}

/// Serialize a real `BranchHistory` to the wire form.
fn to_wire(vector: &str, genesis: GenesisId, msgs: &[Message]) -> WireBranch {
    WireBranch {
        vector: vector.to_string(),
        genesis: genesis.0,
        msgs: msgs
            .iter()
            .map(|m| WireMsg {
                author: m.author.0.clone(),
                seq: m.seq,
                branch: m.branch.0,
                payload: m.payload.clone(),
                sig: m.sig.clone(),
            })
            .collect(),
    }
}

/// Reconstruct a real `BranchHistory` from the wire form (verbatim, via push_raw —
/// no re-signing, so a forged signature survives to be caught by backfill).
fn from_wire(b: &WireBranch) -> BranchHistory {
    let mut hist = BranchHistory::new(GenesisId(b.genesis));
    for m in &b.msgs {
        hist.push_raw(Message {
            author: Did::new(m.author.clone()),
            seq: m.seq,
            branch: GenesisId(m.branch),
            payload: m.payload.clone(),
            sig: m.sig.clone(),
        });
    }
    hist
}

/// The three canonical test vectors, built from the real signed history.
fn build_vectors(gs: &GroupState) -> Vec<WireBranch> {
    // HONEST: alice (a member) signs two messages on her branch.
    let mut honest = BranchHistory::new(gs.g_alice);
    honest.append(&gs.alice, b"alice: hello from a member");
    honest.append(&gs.alice, b"alice: second entry");

    // FORGED: a real-signed message whose payload is then tampered (sig goes stale).
    let mut forged_src = BranchHistory::new(gs.g_forged);
    let signed = forged_src.append(&gs.alice, b"original authorised payload").clone();
    let mut forged = BranchHistory::new(gs.g_forged);
    forged.push_raw(Message {
        author: signed.author.clone(),
        seq: signed.seq,
        branch: signed.branch,
        payload: b"TAMPERED IN TRANSIT".to_vec(), // sig no longer matches these bytes
        sig: signed.sig.clone(),
    });

    // NONMEMBER: mallory signs with her real key (valid signature) but holds no
    // standing in the lineage.
    let mut nonmember = BranchHistory::new(gs.g_mallory);
    nonmember.append(&gs.mallory, b"mallory: let me in");

    vec![
        to_wire("HONEST", gs.g_alice, honest.messages()),
        to_wire("FORGED", gs.g_forged, forged.messages()),
        to_wire("NONMEMBER", gs.g_mallory, nonmember.messages()),
    ]
}

/// The governance group used for the revoke-authority vectors. Deterministic (seeds), so the sender
/// builds the SignedOp and every receiver rebuilds the identical rules + directory to verify it.
/// admins = {alice, bob} (each its own lineage); Remove threshold = 2; founders include carol (the
/// revoke target). Mirrors how the green-real `gov` proofs (E2.1/E2.10) build their state.
fn build_gov(group_id: &str) -> (GovState, Directory, BTreeMap<Did, Did>) {
    let alice = SigningIdentity::from_seed(Did::new("alice"), 1);
    let bob = SigningIdentity::from_seed(Did::new("bob"), 2);
    let _ = group_id;

    let admins: std::collections::BTreeSet<Did> =
        [Did::new("alice"), Did::new("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2u32);
    let rules = GenesisRules { admins, thresholds };
    let founders: std::collections::BTreeSet<Did> =
        [Did::new("alice"), Did::new("bob"), Did::new("carol")]
            .into_iter()
            .collect();
    let genesis = Genesis::new(rules, founders);
    let state = GovState::new(genesis);

    let mut dir = Directory::new();
    dir.insert(alice.verifying());
    dir.insert(bob.verifying());

    // Each admin is its own lineage (single-device); the by-lineage count then == distinct admins.
    let mut lineage_of = BTreeMap::new();
    lineage_of.insert(Did::new("alice"), Did::new("alice"));
    lineage_of.insert(Did::new("bob"), Did::new("bob"));
    (state, dir, lineage_of)
}

/// Two revoke-authority vectors carried as REAL k-of-n Ed25519 bundles (not a MAC):
/// AUTHORIZED (alice+bob = 2 lineages ≥ threshold 2 → accept) and UNDERTHRESHOLD (alice only → reject).
fn build_revoke_vectors(group_id: &str) -> Vec<WireRevoke> {
    let alice = SigningIdentity::from_seed(Did::new("alice"), 1);
    let bob = SigningIdentity::from_seed(Did::new("bob"), 2);
    let (state, _dir, _lin) = build_gov(group_id);
    let target = Some(Did::new("carol"));

    let authorized = sign_op(&state, OpKind::Remove, target.clone(), &[&alice, &bob]);
    let under = sign_op(&state, OpKind::Remove, target, &[&alice]);
    vec![
        WireRevoke { vector: "REVOKE-AUTHORIZED".into(), op: authorized, expect_accept: true },
        WireRevoke { vector: "REVOKE-UNDERTHRESHOLD".into(), op: under, expect_accept: false },
    ]
}

/// Verify a revoke op exactly as a member would on receipt: real k-of-n Ed25519, counted by lineage,
/// against the group's replicated genesis rules + directory. This is `meets_threshold_by_lineage`,
/// green-real in `gov` — now exercised over the live wire.
fn verify_revoke(group_id: &str, op: &SignedOp) -> bool {
    let (state, dir, lineage_of) = build_gov(group_id);
    state.meets_threshold_by_lineage(op, &dir, &lineage_of)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 3 {
        eprintln!("usage: altdrive-spike-faithful-sync <device> <group-id> <self-addr-out> [bootstrap.json ...]");
        std::process::exit(2);
    }
    let device = args[0].clone();
    let group_id = args[1].clone();
    let self_out = &args[2];
    let bootstrap_files = &args[3..];

    let topic = topic_for(&group_id);
    let gs = build_group(&group_id);
    // Two item kinds on the faithful topic: signed branches (signature+standing) and real
    // threshold-signed governance ops (k-of-n authority) — both adjudicated by their real verifier.
    let mut items: Vec<WireItem> = build_vectors(&gs).into_iter().map(WireItem::Branch).collect();
    items.extend(build_revoke_vectors(&group_id).into_iter().map(WireItem::Revoke));
    let vector_bytes: Vec<Vec<u8>> = items
        .iter()
        .map(|v| serde_json::to_vec(v).expect("encode item"))
        .collect();

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
    println!("[{device}] group={group_id} id={} addr->{self_out}", endpoint.id());

    let gossip = Gossip::builder().spawn(endpoint.clone());
    let _router = Router::builder(endpoint.clone()).accept(GOSSIP_ALPN, gossip.clone()).spawn();
    let topic_sub = gossip.subscribe(topic, boot_ids).await?;
    let (sender, mut receiver) = topic_sub.split();

    // Per-vector verdict (genesis -> "ACCEPT" | "REJECT <reason>"), first verdict wins.
    let verdicts: Arc<Mutex<BTreeMap<String, String>>> = Arc::new(Mutex::new(BTreeMap::new()));
    let v_r = verdicts.clone();
    let dname = device.clone();

    tokio::spawn(async move {
        // Each receiver holds the agreed group state + its own branch (bob's).
        let gs = build_group(&group_id);
        let registry = gs.registry.clone();
        let verify = move |did: &Did, bytes: &[u8], sig: &Sig| {
            registry
                .get(&did.0)
                .map(|vi| vi.verify(bytes, sig))
                .unwrap_or(false)
        };
        let mut store = HistoryStore::new();
        // Register our own branch (bob) so backfill compares against a held branch.
        let _ = store.branch_mut(gs.g_bob);

        while let Some(ev) = receiver.next().await {
            match ev {
                Ok(Event::Received(msg)) => match serde_json::from_slice::<WireItem>(&msg.content) {
                    Ok(WireItem::Branch(wb)) => {
                        // Skip vectors already adjudicated (dedupe repeated broadcasts).
                        if v_r.lock().expect("lock").contains_key(&wb.vector) {
                            continue;
                        }
                        let donor = from_wire(&wb);
                        let verdict = match store.backfill_import(&donor, gs.g_bob, &gs.lineage, &verify)
                        {
                            Ok(()) => "ACCEPT".to_string(),
                            Err(e) => format!("REJECT {e}"),
                        };
                        println!(
                            "[{dname}] vector={} genesis={} -> {verdict}",
                            wb.vector,
                            GenesisId(wb.genesis)
                        );
                        v_r.lock().expect("lock").insert(wb.vector.clone(), verdict);
                    }
                    Ok(WireItem::Revoke(wr)) => {
                        if v_r.lock().expect("lock").contains_key(&wr.vector) {
                            continue;
                        }
                        // Real k-of-n Ed25519, counted by lineage — the MD-G5 MAC, retired.
                        let met = verify_revoke(&group_id, &wr.op);
                        let verdict = if met { "ACCEPT" } else { "REJECT under-threshold" };
                        let ok = met == wr.expect_accept;
                        println!(
                            "[{dname}] vector={} (revoke-authority) -> {verdict} [{}]",
                            wr.vector,
                            if ok { "as-expected" } else { "UNEXPECTED" }
                        );
                        v_r.lock()
                            .expect("lock")
                            .insert(wr.vector.clone(), verdict.to_string());
                    }
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

    for _ in 0..ROUNDS {
        tokio::time::sleep(Duration::from_secs(2)).await;
        for vb in &vector_bytes {
            let _ = sender.broadcast(vb.clone().into()).await;
        }
    }
    tokio::time::sleep(Duration::from_secs(2)).await;

    let v = verdicts.lock().expect("lock").clone();
    println!(
        "[{device}] SUMMARY root={} verdicts={:?}",
        gs.root, v
    );
    Ok(())
}
