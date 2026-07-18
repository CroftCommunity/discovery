//! P6 (blinded roster half) — the blind (component).
//!
//! A members-only salt is steward state; roster entries are published as
//! `hash(DID ‖ salt)` commitments. Assertions:
//! - an outsider with the full PUBLIC chain cannot link a known DID to the
//!   roster (they lack the salt);
//! - a member holding their individual attestation (their DID + the salt) proves
//!   their OWN membership;
//! - salt rotation republishes commitments and OLD commitments stop matching;
//! - removal + rotation demonstrates forward-blindness: a removed member can no
//!   longer link anyone, and their own commitment is gone.
//!
//! The sealed steward GROUP half (real MLS deliberation → public verdict, over
//! the croft-group crates) is the isolated `steward-seal/` sub-crate.

use tier_proof::blind::{self, BlindedRoster};

fn dids() -> Vec<String> {
    vec![
        "did:key:zAlice".to_string(),
        "did:key:zBob".to_string(),
        "did:key:zCarol".to_string(),
    ]
}

#[test]
fn outsider_with_public_chain_cannot_link_a_known_did() {
    let salt = b"members-only-secret-salt-v1";
    let roster = BlindedRoster::publish(&dids(), salt);

    // The published commitments are all an outsider sees.
    let published = roster.commitments();
    assert_eq!(published.len(), 3);

    // An outsider KNOWS "did:key:zBob" but not the salt. They cannot recompute
    // the commitment, so they cannot find Bob among the commitments.
    assert!(
        !blind::links_with_salt(published, "did:key:zBob", b"a-guessed-salt"),
        "without the real salt an outsider cannot link a known DID",
    );
    // Even brute-forcing DIDs is futile without the salt — a different salt on
    // the real DID still does not match.
    assert!(!blind::links_with_salt(
        published,
        "did:key:zAlice",
        b"wrong-salt-2"
    ));
}

#[test]
fn member_proves_their_own_membership() {
    let salt = b"members-only-secret-salt-v1";
    let roster = BlindedRoster::publish(&dids(), salt);
    // Bob holds his DID and the members-only salt (his individual attestation).
    assert!(
        blind::links_with_salt(roster.commitments(), "did:key:zBob", salt),
        "a member with the salt proves their own membership",
    );
    // A non-member with the salt still cannot forge membership.
    assert!(!blind::links_with_salt(
        roster.commitments(),
        "did:key:zMallory",
        salt
    ));
}

#[test]
fn salt_rotation_invalidates_old_commitments() {
    let old_salt = b"members-only-secret-salt-v1";
    let new_salt = b"members-only-secret-salt-v2";
    let roster_v1 = BlindedRoster::publish(&dids(), old_salt);
    let roster_v2 = roster_v1.rotate(&dids(), new_salt);

    // The republished commitments differ entirely.
    assert!(
        roster_v1
            .commitments()
            .iter()
            .all(|c| !roster_v2.commitments().contains(c)),
        "rotation republishes: no old commitment survives",
    );
    // A member using the OLD salt no longer matches the NEW roster.
    assert!(!blind::links_with_salt(
        roster_v2.commitments(),
        "did:key:zBob",
        old_salt
    ));
    assert!(blind::links_with_salt(
        roster_v2.commitments(),
        "did:key:zBob",
        new_salt
    ));
}

#[test]
fn removal_and_rotation_is_forward_blind() {
    let old_salt = b"members-only-secret-salt-v1";
    let new_salt = b"members-only-secret-salt-v2";
    let roster_v1 = BlindedRoster::publish(&dids(), old_salt);

    // Carol is removed; the steward rotates the salt and republishes for the
    // remaining members only.
    let remaining = vec!["did:key:zAlice".to_string(), "did:key:zBob".to_string()];
    let roster_v2 = roster_v1.rotate(&remaining, new_salt);

    // Carol's commitment is gone from the new roster.
    assert_eq!(roster_v2.commitments().len(), 2);
    assert!(!blind::links_with_salt(
        roster_v2.commitments(),
        "did:key:zCarol",
        new_salt
    ));

    // Forward-blindness: Carol, holding only the OLD salt, cannot link ANY
    // current commitment to ANY DID — the future roster is opaque to her.
    for did in ["did:key:zAlice", "did:key:zBob", "did:key:zCarol"] {
        assert!(
            !blind::links_with_salt(roster_v2.commitments(), did, old_salt),
            "a removed member cannot read the post-removal roster",
        );
    }
}
