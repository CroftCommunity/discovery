//! End-to-end round-trip: fed-shim produces a signed Follow; the
//! ap-ambassador verify path (unchanged, dev-dep) accepts it.
//!
//! This test proves the shim's SEND direction interoperates with an
//! INDEPENDENT implementation of the same signature scheme
//! (ap-ambassador's verify). The two implementations of the Mastodon
//! draft-cavage HTTP-signature scheme sharing zero code except the
//! serde-cbor / rsa / sha2 / base64 crate dependencies gives the
//! wire-conformance claim its teeth (`FED-SHIM.md §5 Kinship`).
//!
//! **NOT a live-Mastodon test.** Mastodon in the wild might carry
//! additional headers, different Date-header behaviors, or subtle
//! encoding quirks the specimens didn't capture. This test's claim is
//! that shim ⇄ ap-ambassador round-trip: the shim's byte output is
//! accepted by an ap-ambassador-shape receiver. Real-Mastodon behavior
//! is the attended live leg (§4 of the charter).

use ap_ambassador::types::{ActivityKind, ActorId, KeyId, SignedRequest};
use ap_ambassador::verify::{verify_ap_http_signature, KeyResolver};
use fed_shim::activities::build_follow;
use fed_shim::actor::ShimActor;
use fed_shim::sig::build_inbox_post;

/// An ap-ambassador KeyResolver backed by a fed-shim actor.
struct ShimResolver {
    key_id: String,
    spki_der: Vec<u8>,
}

impl KeyResolver for ShimResolver {
    fn resolve(&self, key_id: &KeyId) -> Option<Vec<u8>> {
        if key_id.0 == self.key_id {
            Some(self.spki_der.clone())
        } else {
            None
        }
    }
}

fn shim_to_signed_request(post: fed_shim::sig::SignedInboxPost) -> SignedRequest {
    SignedRequest {
        method: post.method,
        path: post.path,
        headers: post.headers,
        body: post.body,
    }
}

#[test]
fn t_fs_rt_1_shim_follow_verifies_through_ambassador() {
    let alice = ShimActor::generate(
        "shim-rt-alice-v1",
        "https://alice.example/users/alice",
        "alice",
    );
    let body = build_follow(
        &alice,
        "https://bob.example/users/bob",
        "https://alice.example/users/alice#follows/1",
    );
    let post = build_inbox_post(
        &alice,
        "https://bob.example/users/bob/inbox",
        "Mon, 20 Jul 2026 12:00:00 GMT",
        body,
    );
    let resolver = ShimResolver {
        key_id: alice.key_id.clone(),
        spki_der: alice.public_key_spki_der.clone(),
    };
    let req = shim_to_signed_request(post);
    let verified = verify_ap_http_signature(&req, &resolver).expect("shim → ambassador roundtrip");
    assert_eq!(verified.activity.kind, ActivityKind::Follow);
    assert_eq!(verified.activity.actor, ActorId::new(&alice.actor_url));
    assert_eq!(verified.activity.object, "https://bob.example/users/bob");
    assert_eq!(verified.actor_key_spki_der, alice.public_key_spki_der);
    assert_eq!(verified.actor_key_id.0, alice.key_id);
}
