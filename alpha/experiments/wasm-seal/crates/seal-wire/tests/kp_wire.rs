//! P2 red-first: a key package must survive the wire boundary and still
//! admit a member (the artifact the cross-build goldens exchange).

use group_seal::Sealer;
use seal_wire::{key_package_from_bytes, key_package_to_bytes};

#[test]
fn key_package_roundtrips_and_admits() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");

    let kp = bob.key_package().expect("kp");
    let bytes = key_package_to_bytes(&kp).expect("to bytes");
    let kp_back = key_package_from_bytes(&bytes).expect("from bytes");

    let (_commit, welcome) = alice.invite(&[kp_back]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");
    assert!(alice.epoch_secret().expect("s") == bob.epoch_secret().expect("s"));
}

#[test]
fn garbage_bytes_are_rejected() {
    assert!(key_package_from_bytes(b"not an mls message").is_err());
}
