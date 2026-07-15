//! Phase 1 — crypto/protocol feasibility experiments (the thesis gate).
//!
//! These tests verify, against the *real* openmls 0.8.1 library (not docs),
//! that the operations the whole reconnect model depends on actually exist and
//! compose. Each test maps to an experiment id from the plan (§4, Phase 1).
//!
//! Run: `cargo test -p lineage-mls`.

use lineage_core::Did;
use lineage_mls::{Device, Received};

fn did(s: &str) -> Did {
    Did::new(s)
}

/// E1.1 — a removed member cannot decrypt post-removal traffic (PCS holds).
/// Asserts the I4 forward-secrecy component.
#[test]
fn e1_1_removed_member_cannot_decrypt() {
    let mut alice = Device::new(did("alice")).unwrap();
    let mut bob = Device::new(did("bob")).unwrap();

    alice.create_group().unwrap();
    let bob_kp = bob.key_package().unwrap();
    let (_commit, welcome) = alice.add(&[bob_kp]).unwrap();
    bob.join_from_welcome(&welcome, None).unwrap();

    // Before removal: bob decrypts alice's message.
    let ct = alice.send(b"pre-removal: hello bob").unwrap();
    match bob.recv(&ct).unwrap() {
        Received::Application(p) => assert_eq!(p, b"pre-removal: hello bob"),
        other => panic!("expected application message, got {other:?}"),
    }

    // Alice removes bob; her epoch advances.
    let bob_idx = alice.leaf_index_of(&did("bob")).unwrap().expect("bob present");
    let epoch_before = alice.epoch().unwrap();
    alice.remove(&[bob_idx]).unwrap();
    assert!(alice.epoch().unwrap() > epoch_before, "epoch must advance on removal");

    // Post-removal traffic: bob is in a stale epoch and must NOT decrypt it.
    let ct2 = alice.send(b"post-removal: secret").unwrap();
    let result = bob.recv(&ct2);
    assert!(
        result.is_err(),
        "PCS violation: removed member decrypted post-removal traffic: {result:?}"
    );
}

/// E1.2 — external commit brings a B-member into A's epoch; both compute
/// identical group secrets afterward. Asserts the survivor re-key primitive.
#[test]
fn e1_2_external_commit_survivor() {
    // "Branch A" — the survivor epoch, founded by alice.
    let mut alice = Device::new(did("alice")).unwrap();
    alice.create_group().unwrap();

    // Carol comes from "branch B" with no prior welcome into A. She joins A's
    // surviving epoch via an external commit built from A's published info.
    let mut carol = Device::new(did("carol")).unwrap();

    let (group_info, ratchet_tree) = alice.publish_group_info().unwrap();
    let external_commit = carol
        .external_commit_join(&group_info, &ratchet_tree)
        .unwrap();

    // Alice (the existing member) processes carol's external commit.
    match alice.recv(&external_commit).unwrap() {
        Received::CommitMerged => {}
        other => panic!("expected commit, got {other:?}"),
    }

    // The survivor invariant: both sides now derive identical group secrets.
    assert_eq!(
        alice.epoch_proof().unwrap(),
        carol.epoch_proof().unwrap(),
        "survivor re-key failed: alice and carol disagree on the epoch secret"
    );

    // And they can actually talk in the shared epoch.
    let ct = carol.send(b"carol re-keyed into the survivor").unwrap();
    match alice.recv(&ct).unwrap() {
        Received::Application(p) => assert_eq!(p, b"carol re-keyed into the survivor"),
        other => panic!("expected application message, got {other:?}"),
    }
}

/// E1.3 — reinit / fresh-genesis produces a clean third epoch; the two parents
/// are not its ancestors in MLS (the "mint a third" path).
///
/// FINDING (recorded in PHASE_1_FINDINGS.md): openmls 0.8.1 has no high-level
/// reinit-with-continuity API, so "fresh genesis" is new-group + re-add. The
/// test asserts what MLS *can* guarantee — the third group's secret is
/// unrelated to either parent and both parents remain independent.
#[test]
fn e1_3_fresh_genesis_third_epoch() {
    // Two independent parent branches.
    let mut alice = Device::new(did("alice")).unwrap();
    alice.create_group().unwrap();
    let parent_a_secret = alice.epoch_proof().unwrap();

    let mut dave = Device::new(did("dave")).unwrap();
    dave.create_group().unwrap();
    let parent_b_secret = dave.epoch_proof().unwrap();
    assert_ne!(parent_a_secret, parent_b_secret, "parents must be distinct groups");

    // Mint a fresh genesis (the "sixteenth-great-grandparent") and bring both
    // members in clean. Frank is the founder of the third group.
    let mut frank = Device::new(did("frank")).unwrap();
    let mut alice2 = Device::new(did("alice")).unwrap();
    let mut dave2 = Device::new(did("dave")).unwrap();
    let kps = vec![alice2.key_package().unwrap(), dave2.key_package().unwrap()];
    let (_commit, welcome) = frank.fresh_genesis(&kps).unwrap();
    alice2.join_from_welcome(&welcome, None).unwrap();
    dave2.join_from_welcome(&welcome, None).unwrap();

    let third_secret = frank.epoch_proof().unwrap();

    // The third epoch is unrelated to either parent...
    assert_ne!(third_secret, parent_a_secret, "third epoch collided with parent A");
    assert_ne!(third_secret, parent_b_secret, "third epoch collided with parent B");
    // ...and everyone in it converges.
    assert_eq!(third_secret, alice2.epoch_proof().unwrap());
    assert_eq!(third_secret, dave2.epoch_proof().unwrap());
    assert_eq!(frank.member_count().unwrap(), 3);
}

/// E1.4 — revocation under no-co-present-peers: a remove commit produced while
/// a peer is offline still rekeys that peer correctly when applied later, and
/// the removed member stays out. Asserts the "broker carries revocation" claim.
#[test]
fn e1_4_queued_revocation_applies_later() {
    let mut alice = Device::new(did("alice")).unwrap();
    let mut bob = Device::new(did("bob")).unwrap();
    let mut carol = Device::new(did("carol")).unwrap();

    // Genesis with all three.
    alice.create_group().unwrap();
    let (commit_add, welcome) = alice
        .add(&[bob.key_package().unwrap(), carol.key_package().unwrap()])
        .unwrap();
    let _ = commit_add;
    bob.join_from_welcome(&welcome, None).unwrap();
    carol.join_from_welcome(&welcome, None).unwrap();
    alice.assert_member_count(3);

    // Alice removes bob, but carol is OFFLINE. The commit is queued (held by a
    // broker), not delivered. Alice merges her own pending commit immediately.
    let bob_idx = alice.leaf_index_of(&did("bob")).unwrap().expect("bob present");
    let queued_commit = alice.stage_remove(&[bob_idx]).unwrap();
    alice.merge_own_pending().unwrap();

    // Later: carol comes back online and applies the queued commit.
    match carol.recv(&queued_commit).unwrap() {
        Received::CommitMerged => {}
        other => panic!("expected commit, got {other:?}"),
    }

    // Carol rekeyed correctly: she and alice share the post-removal epoch.
    assert_eq!(
        alice.epoch_proof().unwrap(),
        carol.epoch_proof().unwrap(),
        "queued revocation did not rekey the offline peer to the survivor epoch"
    );

    // Bob is gone and cannot follow.
    let ct = alice.send(b"after bob removed").unwrap();
    assert!(carol.recv(&ct).is_ok(), "carol should decrypt post-removal traffic");
    let ct2 = alice.send(b"still without bob").unwrap();
    assert!(bob.recv(&ct2).is_err(), "removed bob must not decrypt");
}

// Small ergonomic assertion used above.
trait MemberCountAssert {
    fn assert_member_count(&self, n: usize);
}
impl MemberCountAssert for Device {
    fn assert_member_count(&self, n: usize) {
        assert_eq!(self.member_count().unwrap(), n, "unexpected member count");
    }
}
