//! EXP-LEX-04 — stapled-status extension prototype.
//!
//! Acceptance criteria (red-first):
//!   * a valid era staple verifies with NO network access;
//!   * a superseded credential (staple against a head that revokes it) is rejected;
//!   * a forged inclusion proof is rejected;
//!   * the binding must name the credential + commitment it is presented with;
//!   * cross-era commitments for the same credential are unlinkable;
//!   * a benchmark table (staple bytes + verify time) vs a modeled callback.

mod common;

use std::time::Instant;

use lexicon_community::cidfirst::dag_cbor_of_json;
use lexicon_community::didkey::Curve;
use lexicon_community::sign::SignKey;
use lexicon_community::staple::{
    build_head, build_staple, commit, era_key, verify_staple, is_fresh, StapleError,
};

fn coop() -> SignKey {
    SignKey::from_seed(Curve::P256, b"RUN-LEX-01/coop-issuer")
}

fn creds(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| format!("bafyreicred{:0>50}", i))
        .collect()
}

const ERA1: [u8; 32] = [0x11; 32];
const ERA2: [u8; 32] = [0x22; 32];

#[test]
fn valid_staple_verifies_with_no_network() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let target = &cs[5];
    let staple = build_staple(&issuer, secret, &ERA1, target, &head, &commits).unwrap();

    // verify_staple takes (cred, pubkey, staple) — NO resolver, NO endpoint.
    verify_staple(target, &issuer.public(), &staple).expect("valid staple, zero callback");
}

#[test]
fn superseded_credential_is_rejected() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let target = cs[3].clone();
    // A newer head that REVOKES the target (it is in the superseded set).
    let (head2, commits2) = build_head(&issuer, secret, &ERA1, 2, &cs, std::slice::from_ref(&target));
    let staple = build_staple(&issuer, secret, &ERA1, &target, &head2, &commits2).unwrap();
    let err = verify_staple(&target, &issuer.public(), &staple).expect_err("superseded rejected");
    assert_eq!(err, StapleError::Superseded);
}

#[test]
fn forged_inclusion_is_rejected() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let mut staple = build_staple(&issuer, secret, &ERA1, &cs[2], &head, &commits).unwrap();
    // Flip a byte in the audit path.
    staple.path[0][0] ^= 0xff;
    let err = verify_staple(&cs[2], &issuer.public(), &staple).expect_err("forged path");
    assert_eq!(err, StapleError::ForgedInclusion);
}

#[test]
fn wrong_index_is_rejected() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let mut staple = build_staple(&issuer, secret, &ERA1, &cs[4], &head, &commits).unwrap();
    staple.index = (staple.index + 1) % staple.tree_size;
    let err = verify_staple(&cs[4], &issuer.public(), &staple).expect_err("wrong leaf index");
    assert_eq!(err, StapleError::ForgedInclusion);
}

#[test]
fn binding_must_name_the_presented_credential() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let staple = build_staple(&issuer, secret, &ERA1, &cs[1], &head, &commits).unwrap();
    // Present it as a DIFFERENT credential id.
    let err = verify_staple(&cs[7], &issuer.public(), &staple).expect_err("binding names cs[1]");
    assert_eq!(err, StapleError::BindingCredentialMismatch);
}

#[test]
fn forged_head_signature_is_rejected() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(8);
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let staple = build_staple(&issuer, secret, &ERA1, &cs[0], &head, &commits).unwrap();
    // A different issuer key must not validate the head.
    let impostor = SignKey::from_seed(Curve::P256, b"impostor");
    let err = verify_staple(&cs[0], &impostor.public(), &staple).expect_err("wrong issuer key");
    assert_eq!(err, StapleError::HeadSignature);
}

#[test]
fn freshness_is_verifier_policy_fail_closed() {
    let issuer = coop();
    let secret = b"coop-era-secret";
    let cs = creds(4);
    let (head1, commits1) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let staple_old = build_staple(&issuer, secret, &ERA1, &cs[0], &head1, &commits1).unwrap();
    // Structurally valid...
    verify_staple(&cs[0], &issuer.public(), &staple_old).unwrap();
    // ...but stale once epoch 2 is the floor.
    assert!(is_fresh(&staple_old, 1));
    assert!(!is_fresh(&staple_old, 2), "a frozen-era staple fails a newer freshness floor");
}

#[test]
fn cross_era_commitments_are_unlinkable() {
    let secret = b"coop-era-secret";
    let cred = "bafyreiwhoever";
    let k1 = era_key(secret, &ERA1);
    let k2 = era_key(secret, &ERA2);
    let c1 = commit(&k1, cred);
    let c2 = commit(&k2, cred);
    assert_ne!(c1, c2, "same credential, different eras → disjoint commitments (T-A4.15)");
}

#[test]
fn benchmark_staple_vs_callback() {
    let issuer = coop();
    let pk = issuer.public();
    let secret = b"coop-era-secret";
    let cs = creds(1024); // a realistic era.
    let (head, commits) = build_head(&issuer, secret, &ERA1, 1, &cs, &[]);
    let target = &cs[512];
    let staple = build_staple(&issuer, secret, &ERA1, target, &head, &commits).unwrap();

    // Wire size of the staple (holder → verifier).
    let head_bytes = dag_cbor_of_json(&staple.head.record).unwrap().len();
    let binding_bytes = dag_cbor_of_json(&staple.binding.record).unwrap().len();
    let path_bytes = staple.path.len() * 32;
    let sig_bytes = staple.head.sig.len() + staple.binding.sig.len();
    let total = head_bytes + binding_bytes + path_bytes + sig_bytes + 32 /*commitment*/ + 16;

    // Verify time, averaged.
    let iters = 2000;
    let t0 = Instant::now();
    for _ in 0..iters {
        verify_staple(target, &pk, &staple).unwrap();
    }
    let per = t0.elapsed() / iters;

    let profile = if cfg!(debug_assertions) { "dev/debug (unoptimized — a release build is ~1-2 orders faster)" } else { "release" };
    let table = format!(
        "# EXP-LEX-04 benchmark — staple vs status-by-callback\n\n\
         `Measured local (Modeled grade, loopback), {profile} build. Era size = {} credentials; audit path = {} nodes.`\n\n\
         | metric | holder-stapled inclusion proof | status-by-callback (OCSP-shaped) |\n\
         |---|---|---|\n\
         | verifier→issuer network | **none** | 1 round trip per check |\n\
         | privacy | issuer learns nothing | issuer learns (verifier, credential) every check |\n\
         | offline verify | **yes** | no |\n\
         | wire bytes (holder→verifier) | **{} B** | ~0 (request) + status response |\n\
         | verify time (measured) | **{:?}** | dominated by network RTT (tens of ms) |\n\
         | trust base | 2 CID-first sigs + Merkle path | issuer liveness + TLS + honest-answer |\n\n\
         Head record {} B · binding {} B · path {} B ({} nodes) · sigs {} B.\n\
         The staple is a strict improvement on the axis the spec leaves open: freshness\n\
         WITHOUT the (verifier, subject) capture leak OCSP taught the web to avoid.\n",
        cs.len(), staple.path.len(), total, per,
        head_bytes, binding_bytes, path_bytes, staple.path.len(), sig_bytes,
    );
    std::fs::write(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("BENCH-STAPLE.md"),
        &table,
    )
    .unwrap();
    eprintln!("{table}");
    assert!(total < 4096, "a staple stays small: {total} B");
}
