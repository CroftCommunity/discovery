//! §2f — message continuity over **real transport** (§7.6.2 message half at loopback).
//!
//! Earns/bounds: Part 2 §7.6.2 message-continuity half — the four continuity claims,
//! previously harness-delivered (RUN-09, `e12_2_message_continuity.rs`), re-driven with the
//! B1 dataplane records carried over **real iroh-gossip at loopback** (`RelayChoice::LocalDirect`,
//! the transport `iroh_convergence.rs` already uses). Loopback grade — relay/real-NAT = X1;
//! the B1 record encoding stays `[gates-release]` (Appendix B / B1).
//!
//! Shape (RUN-11's §2f shaping, executed): two `IrohGossipBus` nodes at loopback; node A publishes
//! the B1 `Record`s (test-only serialization riding **inside** the existing gossip `Frame` payloads
//! — see `ser`/`de` below; NO B1 wire pinning) and node B drains-and-folds them into a `History`.
//! The four claims are re-asserted over transport delivery:
//!   (a) every pre-repoint entry is present after, exactly once;
//!   (b) in-flight entries land once, in causal order, on the post-repoint group;
//!   (c) both members converge byte-identically across arrival orders;
//!   (d) an injected duplicate or dropped frame is *detected*, not absorbed — the harness injects
//!       only the dup/drop fault into the fold stream; the transport itself delivers honestly.
//!
//! The B1 model reused is `replant-continuity`'s pure dataplane (`Record`/`History`) — the exact
//! RUN-09 shapes, now fed by the transport rather than the harness. The governance-generation stamp
//! stands in for the re-plant (pre-repoint stamp G0, in-flight/post-repoint stamp G1 > G0), so the
//! message-continuity half is exercised without re-driving the redb membership repoint (that is the
//! membership half, already `Verified`).
//!
//! Gated behind `iroh-it` so default `cargo test` never binds a socket. Run with:
//!   cargo test -p croft-chat --features iroh-it --test iroh_message_continuity -- --nocapture

#![cfg(feature = "iroh-it")]

use std::time::Duration;

use croft_chat::iroh_bus::{IrohGossipBus, RelayChoice};
use croft_chat::transport::{Frame, Topic, Transport};
use replant_continuity::dataplane::{Fold, History, Record};

// ---- test-only serialization (NOT the [gates-release] B1 wire encoding) --------------------------
//
// Rides inside the gossip `Frame(Vec<u8>)` payload only, so node B can reconstruct the `Record` it
// drained. The `[gates-release]` B1 record encoding (Appendix B / B1) is deliberately NOT pinned
// here; this layout exists solely to move a record across the loopback transport in this test.
fn ser(r: &Record) -> Vec<u8> {
    let mut b = Vec::with_capacity(73 + r.body.len());
    b.extend_from_slice(&r.author);
    b.extend_from_slice(&r.gen_stamp.to_be_bytes());
    match &r.antecedent {
        Some(a) => {
            b.push(1);
            b.extend_from_slice(a);
        }
        None => b.push(0),
    }
    b.extend_from_slice(&(r.body.len() as u64).to_be_bytes());
    b.extend_from_slice(&r.body);
    b
}

fn de(bytes: &[u8]) -> Option<Record> {
    let mut author = [0u8; 32];
    author.copy_from_slice(bytes.get(0..32)?);
    let gen_stamp = u64::from_be_bytes(bytes.get(32..40)?.try_into().ok()?);
    let (antecedent, mut off) = match bytes.get(40)? {
        1 => {
            let mut a = [0u8; 32];
            a.copy_from_slice(bytes.get(41..73)?);
            (Some(a), 73)
        }
        _ => (None, 41),
    };
    let blen = u64::from_be_bytes(bytes.get(off..off + 8)?.try_into().ok()?) as usize;
    off += 8;
    let body = bytes.get(off..off + blen)?.to_vec();
    Some(Record { author, gen_stamp, antecedent, body })
}

/// Author a record chained off `antecedent`, and return `(record, its id)`.
fn rec(author: [u8; 32], gen_stamp: u64, antecedent: Option<[u8; 32]>, body: &[u8]) -> Record {
    Record { author, gen_stamp, antecedent, body: body.to_vec() }
}

/// Spin up two loopback gossip nodes on `topic` (A binds first, B bootstraps from A's address —
/// the exact `iroh_convergence.rs` recipe), let the swarm form, and return the pair.
async fn two_nodes(topic: &Topic) -> (IrohGossipBus, IrohGossipBus) {
    // LocalDirect: hermetic loopback gossip (no relay / Internet), the same path
    // `iroh_convergence.rs` proves convergence over. Relay/real-NAT = X1.
    let bus_a = IrohGossipBus::connect(topic, vec![], RelayChoice::LocalDirect)
        .await
        .expect("bus a");
    let a_addr = bus_a.endpoint_addr();
    let bus_b = IrohGossipBus::connect(topic, vec![a_addr], RelayChoice::LocalDirect)
        .await
        .expect("bus b");
    tokio::time::sleep(Duration::from_secs(2)).await;
    (bus_a, bus_b)
}

/// Publish `records` from `a` repeatedly and drain `b`, collecting the distinct records `b` receives,
/// until `b` holds all `want` distinct ids or the round bound is hit. Returns the drained records in
/// arrival order (dups included — the transport may re-deliver, e.g. the resync re-broadcast).
async fn deliver_over_gossip(
    a: &mut IrohGossipBus,
    b: &mut IrohGossipBus,
    topic: &Topic,
    records: &[Record],
    want: usize,
) -> Vec<Record> {
    use std::collections::BTreeSet;
    let mut arrived: Vec<Record> = Vec::new();
    let mut seen: BTreeSet<[u8; 32]> = BTreeSet::new();
    for _ in 0..120 {
        for r in records {
            a.publish(topic, Frame(ser(r)));
        }
        for f in b.drain() {
            if let Some(r) = de(&f.0) {
                arrived.push(r.clone());
                seen.insert(r.id());
            }
        }
        if seen.len() >= want {
            return arrived;
        }
        tokio::time::sleep(Duration::from_millis(150)).await;
    }
    arrived
}

/// (a) pre-repoint exactly-once, (b) in-flight causal order, (c) cross-order byte-identical
/// convergence — all over real gossip delivery.
#[tokio::test]
async fn records_converge_over_transport_exactly_once_and_in_causal_order() {
    croft_chat::init_tracing();
    let topic = Topic("drystone/msg-continuity/converge".to_string());
    let author_a = [0xA1u8; 32];

    // A pre-repoint chain (stamp G0=1): root r0 <- r1 <- r2. Then the re-plant lifts the stamp,
    // and two in-flight entries (stamp G1=2) are authored in the repoint window: r3 <- r4, chained
    // off the pre-repoint tip r2.
    let r0 = rec(author_a, 1, None, b"pre-0");
    let r1 = rec(author_a, 1, Some(r0.id()), b"pre-1");
    let r2 = rec(author_a, 1, Some(r1.id()), b"pre-2");
    let r3 = rec(author_a, 2, Some(r2.id()), b"inflight-3");
    let r4 = rec(author_a, 2, Some(r3.id()), b"inflight-4");
    let all = [r0.clone(), r1.clone(), r2.clone(), r3.clone(), r4.clone()];

    // A's reference history (the authoring member's fold).
    let mut a_ref = History::new();
    assert_eq!(a_ref.deliver_all(all.clone()).applied, 5, "A folds its own 5 records");

    let (mut bus_a, mut bus_b) = two_nodes(&topic).await;
    let arrived = deliver_over_gossip(&mut bus_a, &mut bus_b, &topic, &all, all.len()).await;

    // B drains-and-folds the transport-delivered records into its own History.
    let mut b_hist = History::new();
    let report = b_hist.deliver_all(arrived.clone());

    // (a) every pre-repoint entry is present after, exactly once — the content-addressed set holds
    // each id once even though gossip re-delivered some frames (report.duplicates > 0 is expected
    // and is *detection*, not absorption).
    assert!(b_hist.contains(&r0.id()) && b_hist.contains(&r1.id()) && b_hist.contains(&r2.id()),
        "all three pre-repoint entries present after transport delivery");
    assert_eq!(b_hist.len(), 5, "exactly the five distinct records, none duplicated into the set");
    assert!(report.is_complete(), "no gap — the full causal chain arrived over gossip");

    // (b) in-flight entries land once, in causal order, on the post-repoint group.
    let order: Vec<[u8; 32]> = b_hist.ordered().iter().map(|r| r.id()).collect();
    let pos = |id: [u8; 32]| order.iter().position(|x| *x == id).expect("present");
    assert!(pos(r2.id()) < pos(r3.id()), "in-flight r3 follows the pre-repoint tip r2");
    assert!(pos(r3.id()) < pos(r4.id()), "in-flight entries in causal order (r3 before r4)");

    // (c) both members converge byte-identically, and across arrival orders: A's authored history
    // and B's transport-received history share a digest, and folding B's arrival stream reversed
    // yields the identical digest (arrival-order independence over the transport).
    assert_eq!(a_ref.digest(), b_hist.digest(), "both members converge byte-identically");
    let mut b_rev = History::new();
    let mut rev = arrived.clone();
    rev.reverse();
    b_rev.deliver_all(rev);
    assert_eq!(b_rev.digest(), b_hist.digest(), "convergence is arrival-order independent");

    eprintln!(
        "MSG-CONTINUITY over iroh-gossip (loopback): 5 B1 records (3 pre-repoint, 2 in-flight) \
         published by A, drained-and-folded by B; every entry present exactly once, in-flight in \
         causal order, both members byte-identical across arrival orders. (duplicates re-delivered \
         by gossip and detected: {})",
        report.duplicates
    );
}

/// (d) an injected duplicate or dropped frame is *detected*, not absorbed — over real gossip
/// delivery, with the harness injecting only the fault into the fold stream.
#[tokio::test]
async fn injected_dup_and_drop_are_detected_over_transport() {
    croft_chat::init_tracing();
    let topic = Topic("drystone/msg-continuity/faults".to_string());
    let author_a = [0xB2u8; 32];

    let r0 = rec(author_a, 1, None, b"f-0");
    let r1 = rec(author_a, 1, Some(r0.id()), b"f-1");
    let r2 = rec(author_a, 1, Some(r1.id()), b"f-2");
    let all = [r0.clone(), r1.clone(), r2.clone()];

    let (mut bus_a, mut bus_b) = two_nodes(&topic).await;
    let arrived = deliver_over_gossip(&mut bus_a, &mut bus_b, &topic, &all, all.len()).await;
    // Distinct records B honestly received, in causal author order (for deterministic injection).
    let mut distinct: Vec<Record> = Vec::new();
    for want in &all {
        if let Some(r) = arrived.iter().find(|r| r.id() == want.id()) {
            distinct.push(r.clone());
        }
    }
    assert_eq!(distinct.len(), 3, "all three records arrived over gossip before fault injection");

    // Duplicate: fold the honest set, then re-fold one received record. The re-fold is DETECTED as a
    // duplicate and the history digest is unchanged — the dup is not absorbed as a second entry.
    let mut h = History::new();
    assert_eq!(h.deliver_all(distinct.clone()).applied, 3, "honest set folds");
    let before = h.digest();
    assert_eq!(h.fold(distinct[1].clone()), Fold::Duplicate, "re-delivered frame detected as a duplicate");
    assert_eq!(h.digest(), before, "the duplicate changed nothing — detected, not absorbed");

    // Drop: the harness drops the middle record r1 from the fold stream (a dropped frame). r2, whose
    // antecedent is r1, cannot fold — the gap is REPORTED, not silently absorbed.
    let dropped = distinct[1].clone();
    let with_drop: Vec<Record> = distinct.iter().filter(|r| r.id() != dropped.id()).cloned().collect();
    let mut hd = History::new();
    let report = hd.deliver_all(with_drop);
    assert!(!report.is_complete(), "the dropped frame leaves a detectable gap");
    assert!(report.unresolved_gaps.contains(&dropped.id()), "the exact dropped antecedent is named as the gap");

    eprintln!(
        "MSG-CONTINUITY fault injection over iroh-gossip (loopback): a re-folded frame was detected \
         as a duplicate (digest unchanged), and a dropped antecedent left a named, unresolved gap — \
         both detected, neither absorbed."
    );
}
