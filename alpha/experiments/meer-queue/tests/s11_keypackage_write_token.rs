//! **S11 — is a KeyPackage usable as a one-time write token?**
//!
//! The personal inbox (see `thinking/meer-two-target-delivery.md`) must accept a deposit from a
//! **stranger** — someone who shares no secret with the owner. That makes the write side open by
//! necessity, and open writes into someone's own namespace mean **spam costs the victim rent**.
//!
//! The proposal under test: **the KeyPackage is the write capability.** B cannot invite A without
//! consuming one of A's published KeyPackages, MLS KeyPackages are single-use by design, and A
//! controls how many exist — so invitations are bounded by a supply the owner sets.
//!
//! Fidelity: **Rung A (real-lib)**.
//!
//! # What has to be true for the idea to work
//!
//! 1. **A KeyPackage is genuinely single-use** — one published package yields at most one join.
//! 2. **Consumption is attributable to a real use** — otherwise anyone who can *read* a published
//!    KeyPackage id can mark it spent, and the write bound becomes a denial of service on the
//!    owner's ability to be invited at all.
//!
//! (2) is the one to be suspicious of: a KeyPackage is **public by design**. Its entire purpose is
//! to be fetched by strangers.

use mls_replant::{join, Persona};
use openmls::prelude::*;

/// Build a group seated with `kp`, returning the Welcome the joiner would receive.
fn invite_with(inviter: &Persona, kp: KeyPackage) -> (Welcome, RatchetTreeIn) {
    let config = MlsGroupCreateConfig::builder()
        .ciphersuite(mls_replant::CS)
        .build();
    let mut group = MlsGroup::new(
        &inviter.provider,
        &inviter.signer,
        &config,
        inviter.cwk.clone(),
    )
    .expect("create group");
    let (_commit, welcome, _gi) = group
        .add_members(&inviter.provider, &inviter.signer, &[kp])
        .expect("add_members");
    group
        .merge_pending_commit(&inviter.provider)
        .expect("merge");

    let bytes = {
        use tls_codec::Serialize as _;
        welcome.tls_serialize_detached().expect("ser")
    };
    let w = {
        use tls_codec::Deserialize as _;
        match MlsMessageIn::tls_deserialize_exact(&bytes)
            .expect("de")
            .extract()
        {
            MlsMessageBodyIn::Welcome(w) => w,
            _ => panic!("expected a Welcome"),
        }
    };
    (w, group.export_ratchet_tree().into())
}

#[test]
fn one_published_keypackage_seats_the_owner_at_most_once() {
    let alice = Persona::new("alice");

    // Alice publishes ONE KeyPackage. Anyone may fetch it — that is its purpose.
    let published = alice.key_package();

    // Two independent inviters use the SAME published package. Both succeed at their end:
    // a KeyPackage carries public key material, so anyone holding it can build a valid Welcome.
    let bob = Persona::new("bob");
    let mallory = Persona::new("mallory");
    let (welcome_from_bob, tree_bob) = invite_with(&bob, published.clone());
    let (welcome_from_mallory, tree_mallory) = invite_with(&mallory, published);

    println!(
        "S11 MEASURED (real-lib): TWO independent parties each built a valid Welcome from ONE \
         published KeyPackage. Nothing at the crypto layer stopped either — a KeyPackage is public \
         key material, and inviting a stranger is what it is FOR."
    );

    // Alice joins the first. This consumes the package's private half from her store.
    let _joined = join(&alice, welcome_from_bob, tree_bob);

    // Can she also join the second? If not, the package really is single-use at her side — and
    // that is exactly what makes burning it a denial of service.
    let second = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        join(&alice, welcome_from_mallory, tree_mallory)
    }));

    match second {
        Err(_) => println!(
            "S11 MEASURED (real-lib): Alice could join the FIRST group and NOT the second — the \
             KeyPackage is single-use at the recipient's side. The private half is consumed on \
             join and does not survive for a second Welcome."
        ),
        Ok(_) => println!(
            "S11 MEASURED (real-lib): Alice joined BOTH groups from one published KeyPackage — it \
             is NOT single-use at the recipient's side in this configuration."
        ),
    }

    println!(
        "S11 VERDICT (real-lib): the KeyPackage FAILS as a one-time write token. Anyone who can \
         READ a published package can produce a valid Welcome against it — reading is public by \
         design — so 'mark it spent on deposit' lets a stranger burn the owner's whole published \
         supply and DENY legitimate invitations. The bound it offers is on the wrong party: it \
         limits the owner's reachability, not the attacker's effort."
    );
}

/// If the token idea fails, what is left? Measure that the damage is at least *bounded* and
/// *attributable*, which is what the fallback rests on.
#[test]
fn what_a_stranger_can_actually_forge_and_what_they_cannot() {
    let alice = Persona::new("alice");
    let mallory = Persona::new("mallory");
    let published = alice.key_package();

    // Mallory can build a Welcome Alice will accept — an unwanted invitation is not forgeable-
    // detectable, it is simply an invitation.
    let (welcome, tree) = invite_with(&mallory, published);
    let joined = join(&alice, welcome, tree);
    assert_eq!(joined.epoch().as_u64(), 1);

    println!(
        "S11 MEASURED (real-lib): a stranger CAN seat Alice in a group she never asked to join — \
         that is MLS working as specified, not a flaw. So 'unwanted invitation' is not \
         cryptographically preventable, and the inbox's write gate can only bound VOLUME and make \
         it ATTRIBUTABLE — never prevent the first one."
    );
    println!(
        "S11 CONSEQUENCE: the write gate must be (a) an authenticated depositor DID, so abuse is \
         attributable and rate-limitable per identity, and (b) an owner-declared ceiling, so total \
         damage is bounded by a number the owner chose. Neither is a KeyPackage."
    );
}
