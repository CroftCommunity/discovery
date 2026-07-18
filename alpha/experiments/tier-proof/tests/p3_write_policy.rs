//! P3 — Write-policy axis + validate-before-relay (component; §A.8).
//!
//! A newsletter (open/**single**) and a forum (open/**open**) come from the same
//! machinery, differing only in the genesis write-policy field:
//!
//! - a non-author post into the newsletter is rejected at the relay gate and is
//!   absent from what is served;
//! - the newsletter author's post is served;
//! - the same author's post into the forum is served;
//! - the catalogue displays BOTH policy fields for each scope.
//!
//! VALIDATE-BEFORE-RELAY (§A.8): the relay re-emits only envelopes passing
//! signature + roster + write-policy; an invalid envelope injected into the
//! stream is dropped and never re-emitted.

use tier_proof::envelope::Envelope;
use tier_proof::fold::Fold;
use tier_proof::identity::Signer;
use tier_proof::records::{self, Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::relay::{self, RelayReject};
use tier_proof::source::{MemSource, RecordSource};

const NEWSLETTER: &str = "scope:dispatch";
const FORUM: &str = "scope:commons";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}
fn alice() -> Signer {
    Signer::from_seed([21u8; 32])
}

/// Both scopes founded by the same owner, Alice a self-registered member of each.
fn two_policy_source() -> MemSource {
    let (o, a) = (owner(), alice());
    let mut src = MemSource::new();
    src.put_record(
        &o,
        Record::Genesis(Genesis {
            scope: NEWSLETTER.to_string(),
            title: "The Dispatch".to_string(),
            write_policy: WritePolicy::Single,
            membership_policy: MembershipPolicy::Open,
            steward_set: vec![],
            threshold: 0,
        }),
    );
    src.put_record(
        &o,
        Record::Genesis(Genesis {
            scope: FORUM.to_string(),
            title: "The Commons".to_string(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Open,
            steward_set: vec![],
            threshold: 0,
        }),
    );
    src.put_record(
        &a,
        Record::SelfRegistration {
            scope: NEWSLETTER.to_string(),
        },
    );
    src.put_record(
        &a,
        Record::SelfRegistration {
            scope: FORUM.to_string(),
        },
    );
    src
}

fn msg(signer: &Signer, scope: &str, text: &str) -> Envelope {
    records::seal(
        signer,
        vec![],
        &Record::Message {
            scope: scope.to_string(),
            text: text.to_string(),
        },
    )
}

#[test]
fn catalogue_displays_both_policy_fields() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");
    let nl = state.catalogue.get(NEWSLETTER).expect("newsletter");
    assert_eq!(nl.write_policy, WritePolicy::Single);
    assert_eq!(nl.membership_policy, MembershipPolicy::Open);
    let fo = state.catalogue.get(FORUM).expect("forum");
    assert_eq!(fo.write_policy, WritePolicy::Open);
    assert_eq!(fo.membership_policy, MembershipPolicy::Open);
}

#[test]
fn non_author_post_into_newsletter_is_rejected_and_absent_from_serve() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");
    let intruder = msg(&alice(), NEWSLETTER, "sneaky broadcast");
    assert_eq!(
        relay::accepts(&state, &intruder),
        Err(RelayReject::WritePolicy),
        "a member who is not the single writer is rejected"
    );
    let served = relay::relay(&state, std::slice::from_ref(&intruder));
    assert!(served.is_empty(), "the rejected post is absent from serve");
}

#[test]
fn author_post_into_newsletter_is_served() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");
    // RUN-18 (B1): write-restricted scopes now demand chaining — the author's
    // first envelope anchors to the scope genesis (GROUPS.md A.2).
    let anchor = state.genesis_id(NEWSLETTER).expect("genesis id");
    let broadcast = records::seal(
        &owner(),
        vec![anchor],
        &Record::Message {
            scope: NEWSLETTER.to_string(),
            text: "this week".to_string(),
        },
    );
    assert!(relay::accepts(&state, &broadcast).is_ok());
    let served = relay::relay(&state, std::slice::from_ref(&broadcast));
    assert_eq!(served.len(), 1, "the owner's newsletter post is served");
}

#[test]
fn member_post_into_forum_is_served() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");
    let post = msg(&alice(), FORUM, "hello commons");
    assert!(
        relay::accepts(&state, &post).is_ok(),
        "any roster member may post to an open forum"
    );
    assert_eq!(relay::relay(&state, &[post]).len(), 1);
}

#[test]
fn non_member_post_into_forum_is_rejected() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");
    let outsider = Signer::from_seed([99u8; 32]);
    let post = msg(&outsider, FORUM, "gatecrash");
    assert_eq!(relay::accepts(&state, &post), Err(RelayReject::NotOnRoster));
}

#[test]
fn validate_before_relay_drops_invalid_and_never_reemits() {
    let state = Fold::run(&two_policy_source().all()).expect("fold");

    // A valid forum post and a signature-invalid envelope in the same stream.
    let good = msg(&alice(), FORUM, "legit");
    let mut forged = good.clone();
    forged.body.payload[0] ^= 0xff; // mutate the payload so the signature no longer covers it

    let out = relay::relay(&state, &[good.clone(), forged.clone()]);
    assert_eq!(out.len(), 1, "only the valid envelope is re-emitted");
    assert_eq!(out[0].identity(), good.identity());
    assert!(
        !out.iter().any(|e| e.identity() == forged.identity()),
        "the invalid envelope is never re-emitted"
    );
    assert_eq!(
        relay::accepts(&state, &forged),
        Err(RelayReject::BadSignature)
    );
}
