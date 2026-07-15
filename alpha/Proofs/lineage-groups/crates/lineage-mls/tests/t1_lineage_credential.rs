//! T1 — lineage-proving credential rides on the MLS leaf (TEST-PLAN.md Tier 1).
//!
//! Real-library dependency #2 (proof-ledger) and COHESION #7: the TS model proves
//! INV-LINEAGE-NOT-LEAF *assuming* a leaf↔lineage mapping is available; this proves
//! openmls 0.8.1 can actually carry a *signed, unforgeable* lineage claim on the leaf,
//! readable and verifiable by a second member, so the presentation fold (E2.9) and
//! lineage-counted thresholds (E2.10) can rest on real library behaviour.
//!
//! Key-hierarchy decision (multi-device.md §10.1, 2026-06-16): logical binding —
//! the device key is INDEPENDENT; the lineage-root key signs a claim binding the
//! lineage id to that device. `lineage_sig` is what makes the claim unforgeable.
//! (The optional HD recovery seed is backup-only and does not enter this format.)
//!
//! These are mechanism-agnostic behaviour tests. GREEN spikes the mechanism both ways
//! per the decision: a real custom `Credential`/`CredentialType::Other` (probe the
//! openmls validation wall) first, then fall back to a structured signed
//! `BasicCredential` identity. The library-limitation finding is itself a deliverable
//! (records into a T1 findings doc).

use lineage_core::keys::SigningIdentity;
use lineage_core::Did;
use lineage_mls::{Device, LineageClaim};

fn did(s: &str) -> Did {
    Did::new(s)
}

/// A lineage root identity (the "parent") that signs each device's membership claim.
fn lineage_root(name: &str) -> SigningIdentity {
    SigningIdentity::from_seed(did(name), 0)
}

#[test]
fn t1_lineage_rides_on_leaf_and_verifies() {
    // A device under lineage "alice", with an independent device key, founds a group
    // carrying a lineage-proving credential.
    let root = lineage_root("alice-lineage");
    let claim = LineageClaim::sign(&root, &did("alice.phone"));
    let mut founder = Device::new_with_lineage(did("alice.phone"), claim.clone()).unwrap();
    founder.create_group().unwrap();

    // The founder's own leaf carries the claim, extractable from the group state...
    let idx = founder.leaf_index_of(&did("alice.phone")).unwrap().unwrap();
    let on_leaf = founder.lineage_claim_of(idx).expect("lineage claim must ride on the leaf");

    // ...and it verifies against the lineage root's public key (signed data alone).
    assert!(
        on_leaf.verify(&root.verifying()),
        "the leaf-carried lineage claim must verify against the lineage root key"
    );
    assert_eq!(on_leaf.lineage_id(), root.verifying().did(), "lineage id must be the root's");
}

#[test]
fn t1_other_member_reads_and_verifies_lineage() {
    // Two devices of the SAME lineage. The second member's leaf must be
    // readable+verifiable by the first — that is the prerequisite for the fold
    // (E2.9) and lineage-counted thresholds (E2.10): every client computes the
    // leaf→lineage mapping from signed data, not from an assertion.
    let root = lineage_root("alice-lineage");
    let phone_claim = LineageClaim::sign(&root, &did("alice.phone"));
    let laptop_claim = LineageClaim::sign(&root, &did("alice.laptop"));

    let mut phone = Device::new_with_lineage(did("alice.phone"), phone_claim).unwrap();
    let laptop = Device::new_with_lineage(did("alice.laptop"), laptop_claim.clone()).unwrap();
    phone.create_group().unwrap();

    let kp = laptop.key_package().unwrap();
    let (_commit, _welcome) = phone.add(&[kp]).unwrap();

    // The founder reads the OTHER device's leaf claim and verifies it.
    let laptop_idx = phone.leaf_index_of(&did("alice.laptop")).unwrap().unwrap();
    let read = phone
        .lineage_claim_of(laptop_idx)
        .expect("a member must be able to read another leaf's lineage claim");
    assert!(read.verify(&root.verifying()), "other member's claim must verify against the root");

    // Both leaves carry the SAME lineage id → they fold to one actor and count once.
    let phone_idx = phone.leaf_index_of(&did("alice.phone")).unwrap().unwrap();
    let mine = phone.lineage_claim_of(phone_idx).unwrap();
    assert_eq!(
        read.lineage_id(),
        mine.lineage_id(),
        "two devices of one lineage must share a lineage id (the fold/threshold key)"
    );
}

#[test]
fn t1_forged_lineage_claim_is_rejected() {
    // An outsider cannot forge membership in alice's lineage. A claim that names
    // alice's lineage id but is NOT signed by alice's root key must fail verification
    // — this is the security half of the dual primitive (provenance).
    let alice_root = lineage_root("alice-lineage");
    let mallory = lineage_root("mallory-lineage");

    // Forge: claim alice's lineage id for an attacker device, signed by mallory.
    let forged = LineageClaim::new(
        alice_root.verifying().did().clone(),
        did("mallory.device"),
        mallory.sign(b"whatever bytes mallory likes"),
    );
    assert!(
        !forged.verify(&alice_root.verifying()),
        "a lineage claim not signed by the lineage root must be rejected"
    );

    // The genuine claim for the same device id, signed by alice's root, verifies.
    let genuine = LineageClaim::sign(&alice_root, &did("alice.phone"));
    assert!(genuine.verify(&alice_root.verifying()));
}

#[test]
fn e2_9_and_c4_devices_of_one_lineage_fold_to_one_actor() {
    // E2.9 (member-list fold) + corpus C4 (add-vs-add of the same person on two
    // device keys must not double-count). Two devices of lineage "alice" plus one
    // device of lineage "bob" are all in the group; folding by lineage yields
    // exactly two actors, with alice's two leaves under one.
    let alice = lineage_root("alice-lineage");
    let bob = lineage_root("bob-lineage");

    let mut phone = Device::new_with_lineage(
        did("alice.phone"),
        LineageClaim::sign(&alice, &did("alice.phone")),
    )
    .unwrap();
    let laptop = Device::new_with_lineage(
        did("alice.laptop"),
        LineageClaim::sign(&alice, &did("alice.laptop")),
    )
    .unwrap();
    let bob_dev = Device::new_with_lineage(
        did("bob.phone"),
        LineageClaim::sign(&bob, &did("bob.phone")),
    )
    .unwrap();
    phone.create_group().unwrap();
    phone.add(&[laptop.key_package().unwrap()]).unwrap();
    phone.add(&[bob_dev.key_package().unwrap()]).unwrap();

    let folded = phone.fold_by_lineage().expect("group exists");
    assert_eq!(folded.len(), 2, "three leaves of two lineages fold to two actors");
    assert_eq!(
        folded.get("lineage:alice-lineage").map(|v| v.len()),
        Some(2),
        "alice's two devices fold under one actor (no double-count)"
    );
    assert_eq!(folded.get("lineage:bob-lineage").map(|v| v.len()), Some(1));
}

#[test]
fn e2_11_revoking_one_device_leaves_the_lineage_other_devices_intact() {
    // Remove one device leaf under a lineage; the epoch rotates and the lineage's
    // OTHER device stays a member (device revocation as a normal membership op).
    let alice = lineage_root("alice-lineage");
    let mut phone = Device::new_with_lineage(
        did("alice.phone"),
        LineageClaim::sign(&alice, &did("alice.phone")),
    )
    .unwrap();
    let laptop = Device::new_with_lineage(
        did("alice.laptop"),
        LineageClaim::sign(&alice, &did("alice.laptop")),
    )
    .unwrap();
    phone.create_group().unwrap();
    phone.add(&[laptop.key_package().unwrap()]).unwrap();
    let epoch_before = phone.epoch().unwrap();

    // Revoke the laptop device by its leaf index.
    let laptop_idx = phone.leaf_index_of(&did("alice.laptop")).unwrap().unwrap();
    phone.remove(&[laptop_idx]).unwrap();

    assert!(phone.epoch().unwrap() > epoch_before, "revocation rotates the epoch");
    assert!(
        phone.leaf_index_of(&did("alice.laptop")).unwrap().is_none(),
        "the revoked device is gone from the group"
    );
    assert!(
        phone.leaf_index_of(&did("alice.phone")).unwrap().is_some(),
        "the lineage's other device is unaffected by revoking one device"
    );
}

/// Spike-both / "document the wall" (TEST-PLAN T1 decision). Probe whether
/// openmls 0.8.1 lets a *real custom credential type* (`CredentialType::Other`)
/// found a group, versus only `BasicCredential`. The library doc says it
/// "currently only supports the BasicCredential" — this records what actually
/// happens, which is the deliverable, not a pass/fail of our design.
#[test]
fn t1_probe_custom_credential_type_wall() {
    use openmls::prelude::*;
    use openmls_basic_credential::SignatureKeyPair;
    use openmls_rust_crypto::OpenMlsRustCrypto;

    let ciphersuite = lineage_mls::CIPHERSUITE;
    let provider = OpenMlsRustCrypto::default();
    let signer = SignatureKeyPair::new(ciphersuite.signature_algorithm()).unwrap();

    // A custom credential type (not Basic, not X509) carrying arbitrary bytes.
    let custom = Credential::new(CredentialType::Other(0xCA11), b"lineage-claim-bytes".to_vec());
    let cwk = CredentialWithKey {
        credential: custom,
        signature_key: signer.public().into(),
    };

    let result = MlsGroup::builder()
        .ciphersuite(ciphersuite)
        .build(&provider, &signer, cwk);

    // Record the wall, do not assert a design outcome. Whichever way it lands is
    // the finding (see T1_LINEAGE_CREDENTIAL_FINDINGS.md).
    match &result {
        Ok(_) => println!(
            "WALL-PROBE: openmls 0.8.1 ACCEPTED CredentialType::Other(0xCA11) to found a group \
             — a real custom credential type is usable; structured-BasicCredential is a choice, \
             not a forced fallback."
        ),
        Err(e) => println!(
            "WALL-PROBE: openmls 0.8.1 REJECTED CredentialType::Other(0xCA11): {e:?} \
             — the wall is real; the structured-BasicCredential identity is the correct path."
        ),
    }
}
