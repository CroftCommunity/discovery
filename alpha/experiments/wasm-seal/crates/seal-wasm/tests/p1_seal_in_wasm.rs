//! P1 — MLS runs INSIDE a wasm32 module (the missing-test-evidence part).
//!
//! Every test in this file executes inside the wasm module under the
//! wasm-bindgen Node runner (`SPEC-DELTA[run19-node-runner]`): group create,
//! add member, seal/unseal roundtrip, application messages within an epoch,
//! commit + epoch roll, removed-member forward-blindness. Nothing here is
//! mocked — RED fails because the stack is absent; GREEN is real openmls
//! ciphertext produced and consumed in wasm.
//!
//! Prediction pins (written RED-first, reported CONFIRMED/DIVERGED in the
//! RUN-19 summary):
//! - PRED-CS: the available suite is the MTI
//!   `MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519` (pure-Rust provider —
//!   no platform crypto needed in wasm).
//! - PRED-RNG: entropy inside the module comes from getrandom's `js` backend
//!   (`crypto.getRandomValues`); observable as distinct groups/secrets from
//!   identical inputs. (The build-behavior half — compiling WITHOUT the `js`
//!   feature fails — is recorded in the summary, not testable from within a
//!   module that only exists because the feature is on.)
//! - PRED-API: the `group-seal` API surface is unchanged under wasm (no
//!   cfg-gated signatures) — the very fact this test source is identical to
//!   what a native consumer would write is the pin.

#![cfg(target_arch = "wasm32")]

use seal_wasm::seal::{self, Sealer};
use wasm_bindgen_test::wasm_bindgen_test;

/// PRED-CS: the sealing suite is the MTI X25519/AES-128-GCM/SHA-256/Ed25519.
#[wasm_bindgen_test]
fn pred_cs_ciphersuite_is_mti() {
    assert_eq!(
        seal::ciphersuite_name(),
        "MLS_128_DHKEMX25519_AES128GCM_SHA256_Ed25519"
    );
}

/// PRED-RNG: two genesis groups founded from identical inputs derive distinct
/// epoch secrets — real entropy is reaching the module (crypto.getRandomValues
/// via getrandom/js), not a deterministic or failing stub.
#[wasm_bindgen_test]
fn pred_rng_two_geneses_differ() {
    let a = Sealer::found("did:example:alice").expect("found a");
    let b = Sealer::found("did:example:alice").expect("found b");
    let sa = a.epoch_secret().expect("secret a");
    let sb = b.epoch_secret().expect("secret b");
    assert_eq!(sa.as_bytes().len(), 32);
    assert_ne!(sa.as_bytes(), sb.as_bytes());
}

/// Group create: a genesis group exists at epoch 0 with a 32-byte exported
/// secret — MLS key-schedule machinery is alive inside the module.
#[wasm_bindgen_test]
fn group_create_in_wasm() {
    let founder = Sealer::found("did:example:alice").expect("found");
    assert_eq!(founder.epoch().expect("epoch"), 0);
    assert_eq!(founder.epoch_secret().expect("secret").as_bytes().len(), 32);
}

/// Add member: a real Welcome (produced in wasm) admits a second member
/// (also in wasm) and both derive the identical epoch secret.
#[wasm_bindgen_test]
fn welcome_admits_and_secrets_agree() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let kp = bob.key_package().expect("kp");
    let (_commit, welcome) = alice.invite(&[kp]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");
    assert_eq!(alice.epoch().expect("e"), bob.epoch().expect("e"));
    // No Debug on EpochSecret (secret hygiene), so no assert_eq!.
    assert!(alice.epoch_secret().expect("s") == bob.epoch_secret().expect("s"));
}

/// Seal/unseal roundtrip: ciphertext sealed by one member opens to the same
/// frame for the other; the sealed bytes are not the plaintext.
#[wasm_bindgen_test]
fn seal_unseal_roundtrip() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let kp = bob.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&[kp]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");

    let m = seal::msg("alice", "sealed in wasm, opened in wasm");
    let sealed = alice.seal(&m).expect("seal");
    assert!(!sealed
        .windows(m.text.len())
        .any(|w| w == m.text.as_bytes()));
    let opened = bob.open(&sealed).expect("open");
    assert_eq!(opened, m);
}

/// Application messages within an epoch: several messages, both directions,
/// no epoch movement.
#[wasm_bindgen_test]
fn app_messages_within_epoch_both_directions() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let kp = bob.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&[kp]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome");
    let epoch = alice.epoch().expect("e");

    for i in 0..3 {
        let ma = seal::msg("alice", &format!("a->b {i}"));
        assert_eq!(bob.open(&alice.seal(&ma).expect("seal")).expect("open"), ma);
        let mb = seal::msg("bob", &format!("b->a {i}"));
        assert_eq!(
            alice.open(&bob.seal(&mb).expect("seal")).expect("open"),
            mb
        );
    }
    assert_eq!(alice.epoch().expect("e"), epoch);
    assert_eq!(bob.epoch().expect("e"), epoch);
}

/// Commit + epoch roll: an add-commit fans out, every member applies it, all
/// advance to the same epoch and re-agree on the secret.
#[wasm_bindgen_test]
fn commit_rolls_epoch_for_all_members() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let kp_b = bob.key_package().expect("kp");
    let (_c, welcome_b) = alice.invite(&[kp_b]).expect("invite");
    bob.accept_welcome(&welcome_b).expect("welcome");
    let before = alice.epoch().expect("e");

    let mut carol = Sealer::enroll("did:example:carol").expect("enroll");
    let kp_c = carol.key_package().expect("kp");
    let (commit, welcome_c) = alice.invite(&[kp_c]).expect("invite");
    bob.apply_control(&commit).expect("bob applies commit");
    carol.accept_welcome(&welcome_c).expect("welcome");

    for s in [&alice, &bob, &carol] {
        assert_eq!(s.epoch().expect("e"), before + 1);
    }
    let sa = alice.epoch_secret().expect("s");
    assert!(sa == bob.epoch_secret().expect("s"));
    assert!(sa == carol.epoch_secret().expect("s"));
}

/// Removed-member forward-blindness: after the removal commit rolls the
/// epoch, the removed member cannot open post-roll ciphertext (forward
/// secrecy), while a remaining member still can.
#[wasm_bindgen_test]
fn removed_member_is_forward_blind() {
    let mut alice = Sealer::found("did:example:alice").expect("found");
    let mut bob = Sealer::enroll("did:example:bob").expect("enroll");
    let mut carol = Sealer::enroll("did:example:carol").expect("enroll");
    let kp_b = bob.key_package().expect("kp");
    let kp_c = carol.key_package().expect("kp");
    let (_c, welcome) = alice.invite(&[kp_b, kp_c]).expect("invite");
    bob.accept_welcome(&welcome).expect("welcome b");
    carol.accept_welcome(&welcome).expect("welcome c");

    // Carol can read in-epoch traffic before the removal.
    let pre = seal::msg("alice", "pre-removal");
    assert_eq!(
        carol.open(&alice.seal(&pre).expect("seal")).expect("open"),
        pre
    );

    let commit = alice.remove_member("did:example:carol").expect("remove");
    bob.apply_control(&commit).expect("bob applies");
    // Carol may process the commit that removes her (her group goes
    // inactive) or reject it — either way she must not read on.
    let _ = carol.apply_control(&commit);

    assert_eq!(alice.epoch().expect("e"), bob.epoch().expect("e"));
    let post = seal::msg("alice", "post-roll: carol must not read this");
    let sealed = alice.seal(&post).expect("seal");
    assert_eq!(bob.open(&sealed).expect("bob reads"), post);
    assert!(
        carol.open(&sealed).is_err(),
        "removed member decrypted post-roll traffic — forward secrecy broken"
    );
}
