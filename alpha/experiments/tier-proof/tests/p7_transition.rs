//! P7 — Tier transition as re-plant (component).
//!
//! An open-genesis scope gains a governed successor: a supersession record whose
//! lineage names the open genesis. Assertions:
//! - the catalogue presents ONE continuous identity, with the policy change at a
//!   causal position (not a new scope);
//! - pre-transition self-registrations remain historically valid and do NOT
//!   silently become gated-tier grants;
//! - the transition emits a plain-language DR-style banner artifact (presence
//!   asserted; wording is owner-editable).

use tier_proof::fold::{transition_banner, Fold, RequestStatus};
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{MemSource, RecordSource};

const SCOPE: &str = "scope:garden";

fn owner() -> Signer {
    Signer::from_seed([60u8; 32])
}
fn early() -> Signer {
    Signer::from_seed([61u8; 32])
}

/// An open scope, an early self-registration, then a governed supersession to a
/// gated tier authored by the owner and citing the open genesis in its lineage.
fn transitioned() -> (MemSource, String) {
    let mut src = MemSource::new();
    let genesis = src.put_record(
        &owner(),
        Record::Genesis(Genesis {
            scope: SCOPE.to_string(),
            title: "The Garden".to_string(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Open,
            steward_set: vec![],
            threshold: 0,
        }),
    );
    src.put_record(
        &early(),
        Record::SelfRegistration {
            scope: SCOPE.to_string(),
        },
    );
    // The supersession names the open genesis in its lineage (antecedents) and
    // its predecessor field, authored by the owner (governed).
    src.put_record_with_antecedents(
        &owner(),
        vec![genesis.clone()],
        Record::Supersession {
            scope: SCOPE.to_string(),
            predecessor: genesis.clone(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Gated,
        },
    );
    (src, genesis)
}

#[test]
fn catalogue_is_one_continuous_identity_with_a_policy_change() {
    let (src, genesis) = transitioned();
    let state = Fold::run(&src.all()).expect("fold");

    // Exactly one catalogue entry for the scope — one continuous identity.
    assert_eq!(state.catalogue.len(), 1, "no second scope is created");
    let entry = state.catalogue.get(SCOPE).expect("continuous scope");
    assert_eq!(
        entry.membership_policy,
        MembershipPolicy::Gated,
        "policy changed in place"
    );
    assert!(entry.superseded_by.is_some(), "records the successor");
    assert_eq!(
        entry.predecessor.as_deref(),
        Some(genesis.as_str()),
        "lineage names the open genesis"
    );
    assert!(
        entry.transition_at.is_some(),
        "the policy change has a causal position"
    );
}

#[test]
fn pre_transition_registration_stays_valid_and_is_not_a_gated_grant() {
    let (src, _g) = transitioned();
    let state = Fold::run(&src.all()).expect("fold");

    // The early member is still on the roster after the transition.
    assert!(
        state.roster_members(SCOPE).contains(&early().did()),
        "pre-transition member persists"
    );

    // Their membership is a self-registration interval, NOT a gated grant: no
    // request from them ever existed, so no grant could have admitted them.
    assert_eq!(
        state.request_status(SCOPE, &early().did()),
        RequestStatus::Unknown,
        "the historical self-registration did not become a gated grant",
    );
    // Exactly one open interval, dating from before the transition.
    let intervals = state.member_intervals(SCOPE, &early().did());
    assert_eq!(intervals.len(), 1);
    assert!(intervals[0].1.is_none(), "still an active member");
    let transition_at = state.catalogue.get(SCOPE).unwrap().transition_at.unwrap();
    assert!(
        intervals[0].0 < transition_at,
        "the interval predates the transition"
    );
}

#[test]
fn transition_emits_a_plain_language_banner() {
    let (src, _g) = transitioned();
    let state = Fold::run(&src.all()).expect("fold");
    let entry = state.catalogue.get(SCOPE).unwrap();

    let banner = transition_banner(entry);
    assert!(banner.is_some(), "a superseded scope has a banner artifact");
    let text = banner.unwrap();
    assert!(
        text.contains("The Garden"),
        "the banner names the scope in plain language"
    );
    // A non-superseded scope has no banner.
    let mut plain = MemSource::new();
    plain.put_record(
        &owner(),
        Record::Genesis(Genesis {
            scope: "scope:plain".into(),
            title: "Plain".into(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Open,
            steward_set: vec![],
            threshold: 0,
        }),
    );
    let ps = Fold::run(&plain.all()).expect("fold");
    assert!(transition_banner(ps.catalogue.get("scope:plain").unwrap()).is_none());
}
