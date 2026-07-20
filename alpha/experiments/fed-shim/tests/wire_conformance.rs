//! fed-shim — wire conformance tests. Every assertion below anchors
//! the shim's byte output to a specimen file under `tests/specimens/`.
//! If a specimen changes, this test file changes with it, and the run
//! summary declares the change (`FED-SHIM.md §1 rule 4`).
//!
//! **Fidelity discipline.** Nothing in this test file is heuristic.
//! Every assertion is either against a specimen constant or against a
//! spec-defined structural invariant. If a claim can't be checked at
//! that grade, it belongs in `FED-SHIM.md §3` (firm non-goals), not
//! here.

use fed_shim::activities::*;
use fed_shim::actor::*;
use fed_shim::sig::*;

// -----------------------------------------------------------------------
// T-FS1 — actor identity + PEM shape
// -----------------------------------------------------------------------

#[test]
fn t_fs1_1_shimactor_deterministic() {
    let a = ShimActor::generate("alice-v1", "https://alice.example/users/alice", "alice");
    let b = ShimActor::generate("alice-v1", "https://alice.example/users/alice", "alice");
    assert_eq!(a.public_key_spki_der, b.public_key_spki_der);
    assert_eq!(a.public_key_pem, b.public_key_pem);
    assert_eq!(a.key_id, "https://alice.example/users/alice#main-key");
    assert_eq!(a.inbox_url, "https://alice.example/users/alice/inbox");
    assert_eq!(a.outbox_url, "https://alice.example/users/alice/outbox");
    assert_eq!(a.preferred_username, "alice");
}

#[test]
fn t_fs1_2_public_key_pem_is_spki() {
    let a = ShimActor::generate(
        "pem-shape-v1",
        "https://alice.example/users/alice",
        "alice",
    );
    // Specimen `mastodon-actor-doc-observed-shape.md`: PEM is SPKI
    // ("-----BEGIN PUBLIC KEY-----" delimiters, not RSA-specific).
    assert!(a.public_key_pem.starts_with("-----BEGIN PUBLIC KEY-----"));
    assert!(a.public_key_pem.trim_end().ends_with("-----END PUBLIC KEY-----"));
    // PEM has literal newlines inside the delimiters. The JSON-embedding
    // side (actor_document) is what escapes them — not the PEM value
    // itself.
    assert!(a.public_key_pem.contains('\n'));
}

// -----------------------------------------------------------------------
// T-FS2 — Follow activity JSON, specimen-anchored
// -----------------------------------------------------------------------

#[test]
fn t_fs2_1_follow_specimen_shape() {
    let a = ShimActor::generate("alice-v2", "https://alice.example/users/alice", "alice");
    let body = build_follow(
        &a,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/42",
    );
    // Specimen key-order: @context, id, type, actor, object.
    let s = std::str::from_utf8(&body).expect("valid utf-8");
    let expect_prefix = r#"{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#follows/42","type":"Follow","actor":"https://alice.example/users/alice","object":"https://bob.example/users/bob"}"#;
    assert_eq!(s, expect_prefix, "Follow byte-shape must match specimen");
    // No trailing newline, no BOM.
    assert!(!body.starts_with(&[0xef, 0xbb, 0xbf]));
    assert!(!body.ends_with(b"\n"));
}

#[test]
fn t_fs2_2_follow_no_extra_fields() {
    let a = ShimActor::generate("alice-v3", "https://alice.example/users/alice", "alice");
    let body = build_follow(
        &a,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/1",
    );
    let s = std::str::from_utf8(&body).unwrap();
    // Specimen calls out: to, cc, audience, published, bcc, bto MUST be omitted.
    for forbidden in ["\"to\":", "\"cc\":", "\"audience\":", "\"published\":", "\"bcc\":", "\"bto\":"] {
        assert!(!s.contains(forbidden), "Follow must not include {forbidden}");
    }
    // No LD-signature.
    assert!(!s.contains("\"signature\":"), "Follow must not carry LD signature");
}

// -----------------------------------------------------------------------
// T-FS3 — Undo Follow shape, specimen-anchored
// -----------------------------------------------------------------------

#[test]
fn t_fs3_1_undo_follow_specimen_shape() {
    let a = ShimActor::generate("alice-v4", "https://alice.example/users/alice", "alice");
    let body = build_undo_follow(
        &a,
        "https://alice.example/users/alice#follows/42",
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/42/undo",
    );
    let s = std::str::from_utf8(&body).unwrap();
    let expect = r#"{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#follows/42/undo","type":"Undo","actor":"https://alice.example/users/alice","object":{"id":"https://alice.example/users/alice#follows/42","type":"Follow","actor":"https://alice.example/users/alice","object":"https://bob.example/users/bob"}}"#;
    assert_eq!(s, expect, "Undo Follow byte-shape must match specimen");
}

// -----------------------------------------------------------------------
// T-FS4 — Delete(Actor) shape, specimen-anchored
// -----------------------------------------------------------------------

#[test]
fn t_fs4_1_delete_actor_specimen_shape() {
    let a = ShimActor::generate("alice-v5", "https://alice.example/users/alice", "alice");
    let body = build_delete_actor(&a, "https://alice.example/users/alice#delete");
    let s = std::str::from_utf8(&body).unwrap();
    let expect = r#"{"@context":"https://www.w3.org/ns/activitystreams","id":"https://alice.example/users/alice#delete","type":"Delete","actor":"https://alice.example/users/alice","object":"https://alice.example/users/alice","to":["https://www.w3.org/ns/activitystreams#Public"]}"#;
    assert_eq!(s, expect, "Delete(Actor) byte-shape must match specimen");
    // Actor == object (account-delete semantic).
    assert!(s.contains("\"actor\":\"https://alice.example/users/alice\""));
    assert!(s.contains("\"object\":\"https://alice.example/users/alice\""));
}

// -----------------------------------------------------------------------
// T-FS5 — Actor document JSON-LD, specimen-anchored
// -----------------------------------------------------------------------

#[test]
fn t_fs5_1_actor_document_specimen_shape() {
    let a = ShimActor::generate("alice-v6", "https://alice.example/users/alice", "alice");
    let doc = actor_document(&a);
    let s = std::str::from_utf8(&doc).unwrap();
    // Key order per specimen: @context, id, type, preferredUsername, inbox, outbox, publicKey.
    // Rather than pin the FULL bytes (public key body varies with the
    // deterministic seed), assert structural invariants + key order.
    assert!(s.starts_with(r#"{"@context":["https://www.w3.org/ns/activitystreams","https://w3id.org/security/v1"],"id":"https://alice.example/users/alice","type":"Person","preferredUsername":"alice","inbox":"https://alice.example/users/alice/inbox","outbox":"https://alice.example/users/alice/outbox","publicKey":{"id":"https://alice.example/users/alice#main-key","owner":"https://alice.example/users/alice","publicKeyPem":"-----BEGIN PUBLIC KEY-----\n"#));
    assert!(s.ends_with(r#"-----END PUBLIC KEY-----\n"}}"#));
    // No trailing newline.
    assert!(!doc.ends_with(b"\n"));
    // No forbidden Mastodon-extra fields the shim doesn't model (§1 non-emit list).
    for forbidden in ["\"following\":", "\"followers\":", "\"featured\":", "\"endpoints\":", "\"icon\":", "\"image\":", "\"attachment\":", "\"tag\":", "\"alsoKnownAs\":"] {
        assert!(!s.contains(forbidden), "actor doc must not include {forbidden}");
    }
}

// -----------------------------------------------------------------------
// T-FS6 — HTTP Signature header shape, specimen-anchored
// -----------------------------------------------------------------------

#[test]
fn t_fs6_1_signature_header_shape() {
    let a = ShimActor::generate("alice-v7", "https://alice.example/users/alice", "alice");
    let body = build_follow(
        &a,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/9",
    );
    let post = build_inbox_post(
        &a,
        "https://bob.example/users/bob/inbox",
        "Mon, 20 Jul 2026 12:00:00 GMT",
        body,
    );
    // Specimen key-order (Mastodon emit): keyId, algorithm, headers, signature.
    let sig_hdr = post
        .headers
        .iter()
        .find(|(k, _)| k == "signature")
        .expect("signature header present")
        .1
        .clone();
    let key_pos = sig_hdr.find("keyId=").expect("keyId present");
    let alg_pos = sig_hdr.find("algorithm=").expect("algorithm present");
    let hdrs_pos = sig_hdr.find("headers=").expect("headers present");
    let sig_pos = sig_hdr.find("signature=").expect("signature present");
    assert!(key_pos < alg_pos && alg_pos < hdrs_pos && hdrs_pos < sig_pos, "Signature header key-order must match specimen");

    // Covered headers exactly: (request-target) host date digest.
    assert!(sig_hdr.contains(r#"headers="(request-target) host date digest""#));
    // Algorithm.
    assert!(sig_hdr.contains(r#"algorithm="rsa-sha256""#));
    // keyId ends in #main-key (specimen actor-doc row).
    assert!(sig_hdr.contains(r#"keyId="https://alice.example/users/alice#main-key""#));
}

#[test]
fn t_fs6_2_digest_matches_body_sha256() {
    use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
    use sha2::{Digest, Sha256};
    let a = ShimActor::generate("alice-v8", "https://alice.example/users/alice", "alice");
    let body = build_follow(
        &a,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/9",
    );
    let post = build_inbox_post(
        &a,
        "https://bob.example/users/bob/inbox",
        "Mon, 20 Jul 2026 12:00:00 GMT",
        body.clone(),
    );
    let digest = post
        .headers
        .iter()
        .find(|(k, _)| k == "digest")
        .expect("digest header present")
        .1
        .clone();
    assert!(digest.starts_with("SHA-256="));
    let b64 = digest.strip_prefix("SHA-256=").unwrap();
    let bytes = B64.decode(b64.as_bytes()).unwrap();
    let expect = Sha256::digest(&post.body);
    assert_eq!(bytes.as_slice(), expect.as_slice());
}

#[test]
fn t_fs6_3_deterministic_signed_post() {
    let a = ShimActor::generate("alice-v9", "https://alice.example/users/alice", "alice");
    let body = build_follow(
        &a,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/9",
    );
    let p1 = build_inbox_post(
        &a,
        "https://bob.example/users/bob/inbox",
        "Mon, 20 Jul 2026 12:00:00 GMT",
        body.clone(),
    );
    let p2 = build_inbox_post(
        &a,
        "https://bob.example/users/bob/inbox",
        "Mon, 20 Jul 2026 12:00:00 GMT",
        body,
    );
    // §1 row 9: deterministic given (actor, date, body).
    assert_eq!(p1.method, p2.method);
    assert_eq!(p1.path, p2.path);
    assert_eq!(p1.host, p2.host);
    assert_eq!(p1.headers, p2.headers);
    assert_eq!(p1.body, p2.body);
}
