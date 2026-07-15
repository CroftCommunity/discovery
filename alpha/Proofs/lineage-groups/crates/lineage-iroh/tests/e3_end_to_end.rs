//! Phase 3 experiments E3.1–E3.4 + a genesis→fork→recombine demo.
//!
//! These run the *real* Phase 1 MLS (and Phase 2 governance/DAG) end-to-end
//! over a [`BlindBroker`] transport — the in-process stand-in for iroh (see
//! PHASE_3_FINDINGS.md for why the real iroh dep is not vendored here). The
//! broker only ever handles opaque ciphertext + routing, so the blindness
//! claim (E3.4) is structural and directly testable.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::conflict::{detect, reconcile, ConflictReason, Escalator, Resolution};
use lineage_core::dag::Lineage;
use lineage_core::gov::{sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, SignedOp};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::SigningIdentity;
use lineage_iroh::{BlindBroker, GroupTopic, Node};

fn topic(seed: u64) -> GroupTopic {
    GroupTopic::from_seed(seed)
}

/// E3.1 — partition then reconnect converges over the transport, with the
/// broker carrying the missed commit while a peer was offline (I10 + I4).
#[test]
fn e3_1_partition_reconnect_converges() {
    let t = topic(1);
    let mut broker = BlindBroker::new();
    let mut alice = Node::new("alice", "alice", t).unwrap();
    let mut bob = Node::new("bob", "bob", t).unwrap();
    let mut carol = Node::new("carol", "carol", t).unwrap();

    // Genesis: alice founds, adds bob.
    alice.found_group().unwrap();
    alice
        .add_member(&mut broker, &"bob".into(), bob.key_package().unwrap(), &[])
        .unwrap();
    bob.pump(&mut broker).unwrap(); // bob joins via welcome

    // Both can talk.
    alice
        .broadcast(&mut broker, &["bob".into()], b"hello bob")
        .unwrap();
    assert_eq!(bob.pump(&mut broker).unwrap(), vec![b"hello bob".to_vec()]);
    assert_eq!(alice.epoch_proof().unwrap(), bob.epoch_proof().unwrap());

    // Partition: bob goes offline. Alice adds carol; the add-commit for bob is
    // held by the broker (it carries it), carol joins via her welcome.
    broker.set_offline(&"bob".into(), true);
    alice
        .add_member(
            &mut broker,
            &"carol".into(),
            carol.key_package().unwrap(),
            &["bob".into()],
        )
        .unwrap();
    carol.pump(&mut broker).unwrap();
    assert_eq!(alice.epoch_proof().unwrap(), carol.epoch_proof().unwrap());
    // Bob is now an epoch behind (still offline).
    assert_ne!(alice.epoch_proof().unwrap(), bob.epoch_proof().unwrap());

    // Reconnect: bob comes back, the broker delivers the carried commit, bob
    // catches up. Exactly one live epoch shared by all (I4), all converge (I10).
    broker.set_offline(&"bob".into(), false);
    bob.pump(&mut broker).unwrap();
    assert_eq!(alice.epoch_proof().unwrap(), bob.epoch_proof().unwrap());
    assert_eq!(alice.epoch_proof().unwrap(), carol.epoch_proof().unwrap());
    assert_eq!(alice.epoch().unwrap(), bob.epoch().unwrap());
    assert_eq!(alice.member_count().unwrap(), 3);
}

/// E3.4 — the broker is blind: it observed only ciphertext + routing metadata,
/// never the plaintext (and structurally never membership).
#[test]
fn e3_4_blind_broker() {
    let t = topic(4);
    let mut broker = BlindBroker::new();
    let mut alice = Node::new("alice", "alice", t).unwrap();
    let mut bob = Node::new("bob", "bob", t).unwrap();

    alice.found_group().unwrap();
    alice
        .add_member(&mut broker, &"bob".into(), bob.key_package().unwrap(), &[])
        .unwrap();
    bob.pump(&mut broker).unwrap();

    let secret = b"meet at the safehouse at noon";
    alice.broadcast(&mut broker, &["bob".into()], secret).unwrap();
    assert_eq!(bob.pump(&mut broker).unwrap(), vec![secret.to_vec()]);

    // The broker handled several envelopes...
    assert!(!broker.audit().is_empty());
    // ...but never saw the plaintext in any of them (nor in snapshots).
    assert!(
        broker.never_saw_plaintext(&[secret, b"alice", b"bob"]),
        "blind broker leaked plaintext/identity into observed bytes"
    );
}

/// E3.3 — total-device-loss recovery: a new device for the same DID joins the
/// live epoch via a broker-held snapshot + external commit, and can decrypt
/// forward only (never traffic from epochs it was not part of).
#[test]
fn e3_3_device_loss_recovery() {
    let t = topic(3);
    let mut broker = BlindBroker::new();
    let mut alice = Node::new("alice", "alice", t).unwrap();
    let mut bob = Node::new("bob", "bob", t).unwrap();

    alice.found_group().unwrap();
    alice
        .add_member(&mut broker, &"bob".into(), bob.key_package().unwrap(), &[])
        .unwrap();
    bob.pump(&mut broker).unwrap();

    // Pre-loss traffic that the recovered device must NOT be able to read.
    let pre = alice.send_raw(b"pre-recovery secret").unwrap();

    // Bob's device is lost. Alice publishes the current epoch snapshot.
    alice.publish_snapshot(&mut broker).unwrap();

    // A fresh device for the same DID recovers via external commit.
    let mut bob2 = Node::new("bob2", "bob", t).unwrap();
    bob2.join_via_broker(&mut broker, &["alice".into()]).unwrap();
    alice.pump(&mut broker).unwrap(); // alice admits bob2's external commit

    // Recovered device reached the live epoch...
    assert_eq!(alice.epoch_proof().unwrap(), bob2.epoch_proof().unwrap());
    // ...but cannot read pre-recovery traffic (forward-only).
    assert!(
        bob2.try_recv(&pre).is_err(),
        "recovered device must not decrypt pre-recovery epochs"
    );
    // ...and can read traffic from here forward.
    alice
        .broadcast(&mut broker, &["bob2".into()], b"welcome back")
        .unwrap();
    assert_eq!(bob2.pump(&mut broker).unwrap(), vec![b"welcome back".to_vec()]);
}

/// E3.2 — a contradictory membership decision under partition hard-stops and
/// escalates over the transport; the signed op travels as an opaque payload
/// through the blind broker, and there is no silent re-admit (I6).
#[test]
fn e3_2_conflict_reconnect_hard_stops() {
    let t = topic(2);
    let mut broker = BlindBroker::new();

    let alice = SigningIdentity::from_seed(Did::new("alice"), 1);
    let bob = SigningIdentity::from_seed(Did::new("bob"), 1);
    let mut dir = Directory::new();
    dir.insert(alice.verifying());
    dir.insert(bob.verifying());

    let admins: BTreeSet<Did> = [Did::new("alice"), Did::new("bob")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> = [Did::new("alice"), Did::new("bob"), Did::new("carol")]
        .into_iter()
        .collect();
    let genesis = Genesis::new(rules, founders);

    // Left partition boots carol; the signed op is shipped as an OPAQUE payload
    // through the broker (it learns nothing of its contents).
    let mut left = GroupState::new(genesis.clone());
    let op = sign_op(&left, OpKind::Remove, Some(Did::new("carol")), &[&alice, &bob]);
    let op_bytes = serde_json::to_vec(&op).unwrap();
    broker.relay(lineage_iroh::Envelope {
        to: "right".into(),
        topic: t,
        kind: lineage_iroh::EnvKind::Handshake,
        ciphertext: op_bytes,
    });
    left.apply(op, &dir).unwrap();

    // Right partition keeps carol (its own view).
    let right = GroupState::new(genesis.clone());

    // Reconnect: right receives left's op off the broker and reconstructs
    // left's resulting view, then reconciles.
    let delivered = broker.drain_topic(&"right".into(), t);
    assert_eq!(delivered.len(), 1);
    let mut left_view = GroupState::new(genesis.clone());
    let received: SignedOp = serde_json::from_slice(&delivered[0].ciphertext).unwrap();
    left_view.apply(received, &dir).unwrap();

    struct Rec(Vec<ConflictReason>);
    impl Escalator for Rec {
        fn on_conflict(&mut self, r: &ConflictReason) {
            self.0.push(r.clone());
        }
    }
    let mut rec = Rec(Vec::new());
    let resolution = reconcile(&left_view, &right, &mut rec);

    assert_eq!(detect(&left_view, &right), resolution);
    assert_eq!(
        resolution,
        Resolution::HardStop(vec![ConflictReason::RemovedThenIncluded(Did::new("carol"))])
    );
    assert_eq!(rec.0.len(), 1, "human escalation must fire exactly once");
    // No silent re-admit: right never applied the remove; carol stays.
    assert!(right.members.contains(&Did::new("carol")));
}

/// Gate demo — genesis → fork → recombine across nodes over the transport, with
/// the lineage DAG recording provenance and MLS minting a fresh third epoch.
#[test]
fn e3_genesis_fork_recombine_demo() {
    let mut lineage = Lineage::new();

    // --- genesis: alice founds, adds bob (topic/branch A) -------------------
    let ta = topic(100);
    let mut broker_a = BlindBroker::new();
    let mut alice = Node::new("alice", "alice", ta).unwrap();
    let mut bob = Node::new("bob", "bob", ta).unwrap();
    alice.found_group().unwrap();
    alice
        .add_member(&mut broker_a, &"bob".into(), bob.key_package().unwrap(), &[])
        .unwrap();
    bob.pump(&mut broker_a).unwrap();
    let ga = GenesisId::from_bytes(b"branch-A");
    lineage.add_root(ga, [Did::new("alice"), Did::new("bob")]);
    let secret_a = alice.epoch_proof().unwrap();

    // --- fork: a splinter mints a fresh genesis (branch B) ------------------
    let tb = topic(200);
    let mut broker_b = BlindBroker::new();
    let mut frank = Node::new("frank", "frank", tb).unwrap();
    let mut carol = Node::new("carol", "carol", tb).unwrap();
    frank.found_group().unwrap();
    frank
        .add_member(&mut broker_b, &"carol".into(), carol.key_package().unwrap(), &[])
        .unwrap();
    carol.pump(&mut broker_b).unwrap();
    let gb = GenesisId::from_bytes(b"branch-B");
    lineage.fork(ga, gb, [Did::new("frank"), Did::new("carol")]);
    let secret_b = frank.epoch_proof().unwrap();
    assert_ne!(secret_a, secret_b, "fork is a distinct epoch");

    // --- recombine: mint a fresh third genesis inheriting both as ancestry --
    let tc = topic(300);
    let mut broker_c = BlindBroker::new();
    let mut frank3 = Node::new("frank", "frank", tc).unwrap();
    let mut alice3 = Node::new("alice", "alice", tc).unwrap();
    frank3.found_group().unwrap();
    frank3
        .add_member(&mut broker_c, &"alice".into(), alice3.key_package().unwrap(), &[])
        .unwrap();
    alice3.pump(&mut broker_c).unwrap();
    let gc = GenesisId::from_bytes(b"branch-C");
    lineage.recombine(ga, gb, gc, [Did::new("frank"), Did::new("alice")]);
    let secret_c = frank3.epoch_proof().unwrap();

    // The third epoch is unrelated to both parents (forward keys never merge)...
    assert_ne!(secret_c, secret_a);
    assert_ne!(secret_c, secret_b);
    assert_eq!(frank3.epoch_proof().unwrap(), alice3.epoch_proof().unwrap());

    // ...but provenance is preserved: the recombine shares lineage with both
    // parents, and alice — a party to branch A — has standing on the recombine.
    assert!(lineage.shares_lineage(gc, ga));
    assert!(lineage.shares_lineage(gc, gb));
    assert!(lineage.standing(&Did::new("alice"), gc));
    assert!(lineage.standing(&Did::new("bob"), gc)); // bob was on branch A's lineage
}
