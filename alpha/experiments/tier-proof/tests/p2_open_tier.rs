//! P2 — Open tier, the one-signature tier (LIVE; live leg BLOCKED here).
//!
//! The live path (genesis + self-registration written to a real PDS, ingested
//! from real Jetstream) is proven reachable by RUN-14; it needs `ATP_TEST_*`
//! credentials this environment does not carry, so the live leg reports
//! **BLOCKED** (guardrail 4: BLOCKED beats pretended) and the multi-party half
//! runs against locally generated keypairs behind the SAME record-source
//! interface a live Jetstream source implements —
//! `SPEC-DELTA[run17-live-source | declared-stand-in]`.
//!
//! Prediction-first (guardrail 3) — constants BEFORE any live call, reported
//! CONFIRMED/DIVERGED/BLOCKED in RUN-17-SUMMARY.md:
//!
//! - P2-1: a self-registration `create` commit propagates as a Jetstream
//!   `commit` frame with `operation:"create"` and the record under
//!   `commit.record`.
//! - P2-2: a `delete` of that record propagates as a `commit` frame with
//!   `operation:"delete"` carrying the `rkey` but no record body.
//! - P2-3: the repo commit signature chains to the author's DID-doc key, so
//!   "own signing proves self-registration" needs no second party.

use tier_proof::fold::Fold;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{LiveLeg, MemSource, RecordSource, SourceEvent};

const OPEN_SCOPE: &str = "scope:town-square";

#[test]
fn live_leg_is_blocked_not_pretended() {
    // The live probe inspects env for credentials and NEVER fabricates a result.
    match tier_proof::source::live_open_tier_leg() {
        LiveLeg::Blocked { reason } => {
            assert!(
                reason.contains("ATP_TEST"),
                "blocked reason names the missing creds"
            );
        }
        LiveLeg::Ran { .. } => { /* credentials present: the live leg ran */ }
    }
}

/// Build the open-tier fixture: an open/open genesis and a self-registration,
/// each an envelope whose payload is the canonical record, authored by its own
/// signer (the one-signature tier: the registrant signs their own membership).
fn open_tier_source() -> (MemSource, String) {
    use tier_proof::identity::Signer;
    let steward = Signer::from_seed([10u8; 32]);
    let alice = Signer::from_seed([11u8; 32]);

    let mut src = MemSource::new();
    src.put_record(
        &steward,
        Record::Genesis(Genesis {
            scope: OPEN_SCOPE.to_string(),
            title: "Town Square".to_string(),
            write_policy: WritePolicy::Open,
            membership_policy: MembershipPolicy::Open,
            steward_set: vec![],
            threshold: 0,
        }),
    );
    let reg_id = src.put_record(
        &alice,
        Record::SelfRegistration {
            scope: OPEN_SCOPE.to_string(),
        },
    );
    (src, reg_id)
}

#[test]
fn self_registration_folds_into_catalogue_and_roster() {
    let (src, _reg) = open_tier_source();
    let state = Fold::run(&src.all()).expect("fold");

    let cat = state.catalogue.get(OPEN_SCOPE).expect("scope in catalogue");
    assert_eq!(cat.write_policy, WritePolicy::Open);
    assert_eq!(cat.membership_policy, MembershipPolicy::Open);

    let members = state.roster_members(OPEN_SCOPE);
    let alice = tier_proof::identity::Signer::from_seed([11u8; 32]).did();
    assert!(
        members.contains(&alice),
        "self-registered author is a member"
    );
}

#[test]
fn own_signature_proves_self_registration_without_a_second_party() {
    // P2-3 at component grade: the registration envelope verifies against the
    // author's own key alone — no steward, no second signature.
    let (src, reg_id) = open_tier_source();
    let env = src
        .all()
        .into_iter()
        .find_map(|e| match e {
            SourceEvent::Put(env) if env.identity_hex() == reg_id => Some(env),
            _ => None,
        })
        .expect("registration envelope present");
    assert!(
        env.verify().is_ok(),
        "registration verifies on the author's signature alone"
    );
}

#[test]
fn deleting_the_registration_is_a_leave() {
    let (mut src, reg_id) = open_tier_source();
    let alice = tier_proof::identity::Signer::from_seed([11u8; 32]);
    src.delete(&alice, &reg_id); // author deletes their own registration

    let state = Fold::run(&src.all()).expect("fold");
    let members = state.roster_members(OPEN_SCOPE);
    assert!(
        !members.contains(&alice.did()),
        "a deleted registration leaves the roster (leave via the firehose)"
    );
}

#[test]
fn catalogue_and_roster_are_reconstructable_from_backfill_plus_tail() {
    let (src, _reg) = open_tier_source();

    // The "live" fold, with a working index.
    let indexed = Fold::run(&src.all()).expect("indexed fold");

    // Drop the entire index; re-fold from backfill + tail split at an arbitrary
    // causal point. The rebuilt state must be byte-identical.
    let (backfill, tail) = src.split_backfill_tail();
    let mut replay = backfill;
    replay.extend(tail);
    let rebuilt = Fold::run(&replay).expect("rebuilt fold");

    assert_eq!(
        indexed.canonical_digest(),
        rebuilt.canonical_digest(),
        "catalogue + roster rebuilt from backfill+tail are byte-identical"
    );
}
