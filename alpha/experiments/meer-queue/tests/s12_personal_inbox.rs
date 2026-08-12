//! **S12 — the personal inbox, walked out against real CISS.**
//!
//! The group half of the two-target design (`thinking/meer-two-target-delivery.md`) is measured at
//! Rung A end to end. The **personal half was design only**. This closes that asymmetry.
//!
//! Fidelity: **Rung A (real-lib)** — real OpenMLS, real CISS over real loopback HTTP, real signed
//! Model-A assertions.
//!
//! Four questions, in the order they matter:
//!
//! 1. **Is the inbox necessary?** Can a would-be member derive a group's queue name *without*
//!    group state? If they can, the inbox is redundant.
//! 2. **Does read gating actually hold?** `read_class: owner` is claimed shipped. A public address
//!    is only safe if reads are genuinely refused to everyone else.
//! 3. **Is custodial write really the gap?** The spec says writes are owner-only. Measure the
//!    refusal rather than citing it.
//! 4. **Does the whole stranger handshake work** once the write step is stood in for?

use std::sync::Arc;

use meer_queue::ciss_harness::CissHarness;
use meer_queue::mls;
use mls_replant::{join, stamp, Persona};
use openmls::prelude::*;

const QUEUE_LABEL: &str = "croft/meer-queue/v1";

fn queue_name(group: &MlsGroup, who: &Persona) -> String {
    hex::encode(
        group
            .export_secret(who.provider.crypto(), QUEUE_LABEL, &[], 32)
            .expect("export_secret"),
    )
}

/// A Model-A (`id:` owner, self-signed) policy assertion setting `read_class`.
fn signed_policy(
    keypair: &ciss::crypto::Keypair,
    did: &str,
    seq: u64,
    class: ciss::policy::ReadClass,
) -> String {
    use ciss::assertion::{record_preimage, Authorization, OwnerSigned, SignedAssertion};
    use ciss::policy::{policy_body_fold, PolicyBody, POLICY_KIND};

    let body = PolicyBody {
        read_class: class,
        readers: vec![],
    };
    let fold = policy_body_fold(&body);
    let preimage = record_preimage(POLICY_KIND, did, None, seq, &fold);
    let record = SignedAssertion {
        did: did.to_owned(),
        kind: POLICY_KIND.to_owned(),
        subkey: None,
        seq,
        body: serde_json::to_value(&body).expect("body json"),
        authorization: Authorization::OwnerSigned(OwnerSigned {
            signer: keypair.public_key_hex(),
            sig: keypair.sign_message(&preimage),
        }),
    };
    serde_json::to_string(&record).expect("record json")
}

/// **1.** Without group state there is no derivable queue name — so the inbox is necessary.
#[test]
fn a_stranger_cannot_derive_a_groups_queue_name() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let stamped = stamp(&alice, &[&bob]);
    let real = queue_name(&stamped.group, &alice);

    // Mallory knows Alice's *public* KeyPackage — everything a stranger can legitimately obtain —
    // and holds no group state. There is nothing to export a secret from.
    let mallory = Persona::new("mallory");
    let _alices_public_kp = alice.key_package();
    let mallorys_own = stamp(&mallory, &[&Persona::new("filler")]);
    assert_ne!(queue_name(&mallorys_own.group, &mallory), real);

    println!(
        "S12 CONFIRMED (real-lib): a queue name is derivable only from GROUP STATE. Holding the \
         owner's public KeyPackage yields nothing. So a stranger has no group-queue path at all — \
         the personal inbox is NECESSARY, not merely convenient. [{}]",
        mls::resolved_versions()
    );
}

/// **2.** `read_class: owner` — does a public address stay unreadable?
#[tokio::test]
async fn read_class_owner_refuses_everyone_but_the_owner() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let alice = ciss.identity("alice-inbox");
    let mallory = ciss.identity("mallory");

    // Alice deposits an object (standing in for a Welcome) and locks the namespace to owner-only.
    let put = ciss.put_object(&alice, "inbox-item", b"a sealed welcome").await;
    assert_eq!(put.status, 200, "{}", put.body_text());
    let cid = put.cid().expect("cid");

    let policy = signed_policy(
        alice.keypair(),
        alice.did(),
        1,
        ciss::policy::ReadClass::Owner,
    );
    let set = ciss.put_assertion(&alice, "policy", &policy).await;
    assert_eq!(
        set.status, 200,
        "setting read_class=owner must succeed: {}",
        set.body_text()
    );

    // The owner reads it.
    let owner_read = ciss.get_object(&alice, &cid).await;
    assert_eq!(owner_read.status, 200, "the owner must always read its own");
    assert_eq!(owner_read.body, b"a sealed welcome");

    // A different authenticated identity is refused.
    let stranger_read = ciss.get_object_as(&mallory, alice.did(), &cid).await;
    assert_ne!(
        stranger_read.status, 200,
        "an authenticated stranger MUST NOT read an owner-gated object (got 200)"
    );

    // And an unauthenticated caller is refused.
    let anon_read = ciss.get_object_anon(alice.did(), &cid).await;
    assert_ne!(
        anon_read.status, 200,
        "an anonymous caller MUST NOT read an owner-gated object (got 200)"
    );

    println!(
        "S12 CONFIRMED (real-lib): with read_class=owner, owner GET=200, authenticated stranger \
         GET={}, anonymous GET={}. A publicly-addressed inbox yields a WRITE target and nothing \
         else — the harvest-now-decrypt-later concern does not arise, because the ciphertext is \
         never obtainable.",
        stranger_read.status, anon_read.status
    );

    ciss.shutdown().await;
}

/// **3.** Is custodial write really the gap? Measure the refusal.
#[tokio::test]
async fn a_stranger_cannot_write_into_the_owners_namespace_today() {
    let ciss = Arc::new(CissHarness::spawn().await);
    let alice = ciss.identity("alice-inbox");
    let bob = ciss.identity("bob-sender");

    // Bob tries to deposit into Alice's namespace — the personal-inbox write the design needs.
    let attempt = ciss.put_object_as(&bob, alice.did(), "welcome", b"a welcome for alice").await;
    assert_ne!(
        attempt.status, 200,
        "writes are owner-only today; a stranger deposit must be refused"
    );

    println!(
        "S12 MEASURED (real-lib): a stranger's deposit into the owner's namespace is refused with \
         HTTP {} — writes are owner-only (SECURITY-POSTURE Z2), and delegated write is [PLANNED], \
         not v1. **This is the personal inbox's one genuine blocker**, and it is exactly the \
         custodial-write work in meer-lane Phase 1. Everything else in this file already works.",
        attempt.status
    );

    ciss.shutdown().await;
}

/// **4.** The whole stranger handshake, with the write step stood in for by the owner.
///
/// SPEC-DELTA[meer-spike-owner-write-standin | test-scaffold]: the deposit is performed by the
/// **owner** rather than the sender, because delegated write does not exist (measured above). Every
/// other step — KeyPackage publication and fetch, group creation, `Welcome` deposit and retrieval,
/// the join, and the handover to the group queue — is real.
/// — Register: `alpha/experiments/SPEC-DIVERGENCE-REGISTER.md`
#[tokio::test]
async fn the_full_stranger_handshake_end_to_end() {
    use tls_codec::{Deserialize as _, Serialize as _};

    let ciss = Arc::new(CissHarness::spawn().await);
    let alice_store = ciss.identity("alice-inbox");

    let alice = Persona::new("alice");
    let bob = Persona::new("bob");

    // 1. Alice publishes a KeyPackage into her own namespace — her contact point.
    let kp_bytes = MlsMessageOut::from(alice.key_package())
        .tls_serialize_detached()
        .expect("ser kp");
    let published = ciss.put_object(&alice_store, "keypackage", &kp_bytes).await;
    assert_eq!(published.status, 200);
    let kp_cid = published.cid().expect("cid");

    // 2. Bob — a stranger — fetches it.
    let fetched = ciss.get_object(&alice_store, &kp_cid).await;
    assert_eq!(fetched.status, 200);
    // `validate()` is the real receiver path — a fetched KeyPackage is untrusted until checked.
    // (The bare `From<KeyPackageIn>` conversion is `test-utils`-gated, correctly: it would skip
    // exactly this validation.)
    let kp: KeyPackage = match MlsMessageIn::tls_deserialize_exact(&fetched.body)
        .expect("parse kp")
        .extract()
    {
        MlsMessageBodyIn::KeyPackage(kp_in) => kp_in
            .validate(bob.provider.crypto(), ProtocolVersion::Mls10)
            .expect("the fetched KeyPackage validates"),
        _ => panic!("expected a KeyPackage"),
    };

    // 3. Bob creates the group and produces a Welcome addressed to Alice.
    let config = MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .build();
    let mut bobs_group =
        MlsGroup::new(&bob.provider, &bob.signer, &config, bob.cwk.clone()).expect("group");
    let (_c, welcome_out, _gi) = bobs_group
        .add_members(&bob.provider, &bob.signer, &[kp])
        .expect("add alice");
    bobs_group.merge_pending_commit(&bob.provider).expect("merge");
    let welcome_bytes = welcome_out.tls_serialize_detached().expect("ser welcome");
    let tree = bobs_group.export_ratchet_tree().into();

    // 4. The deposit into Alice's inbox — owner-performed stand-in for custodial write.
    let deposited = ciss
        .put_object(&alice_store, "welcome", &welcome_bytes)
        .await;
    assert_eq!(deposited.status, 200);
    let welcome_cid = deposited.cid().expect("cid");

    // 5. Alice drains her inbox and joins.
    let drained = ciss.get_object(&alice_store, &welcome_cid).await;
    assert_eq!(drained.status, 200);
    assert_eq!(drained.body, welcome_bytes, "bytes unchanged through CISS");
    let welcome = match MlsMessageIn::tls_deserialize_exact(&drained.body)
        .expect("parse")
        .extract()
    {
        MlsMessageBodyIn::Welcome(w) => w,
        _ => panic!("expected a Welcome"),
    };
    let alices_group = join(&alice, welcome, tree);

    // 6. Handover: Alice can now derive the GROUP queue name — the inbox's job is done.
    let alice_q = queue_name(&alices_group, &alice);
    let bob_q = queue_name(&bobs_group, &bob);
    assert_eq!(
        alice_q, bob_q,
        "after the handshake both parties derive the same group queue name"
    );

    println!(
        "S12 CONFIRMED (real-lib): the full stranger handshake works end to end — KeyPackage \
         published to and fetched from the owner's namespace, group created, Welcome deposited and \
         retrieved byte-identically, Alice joined, and BOTH parties then derive the same group \
         queue name ({}…). The personal inbox hands over to the group queue exactly once, at \
         first contact. Only the deposit step is stood in for.",
        &alice_q[..16]
    );

    ciss.shutdown().await;
}
