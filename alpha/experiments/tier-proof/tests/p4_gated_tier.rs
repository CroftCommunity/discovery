//! P4 — Gated tier: two-sided facts and causal revocation (§A.3).
//!
//! The heart of the run. This part is genuinely multi-party. Without live
//! credentials it runs against locally generated keypairs behind the same
//! interfaces the live path would use — `SPEC-DELTA[run17-multiparty |
//! declared-stand-in]`. The threshold-1 case is the single-steward case; the
//! threshold-2 case is the multi-steward co-sign, also against local keypairs.
//!
//! Proven here:
//! - a steward GRANT (citing the member's REQUEST hash among its antecedents)
//!   admits the member FROM the grant's causal position;
//! - multi-steward co-sign reaches a threshold before admission;
//! - a REVOCATION is a causal cut: a message from the revoked member with
//!   antecedents BEFORE the cut still verifies; AFTER the cut it is rejected;
//! - silence is not a verdict: an unanswered request stays `pending` forever;
//! - leave-vs-revoke asymmetry: the member leaves with no steward act;
//! - the fold emits the member's interval set (grant→cut);
//! - archive habit: a second folder rebuilds the identical roster from the
//!   archive alone, signatures re-verified (verifiable, not trusted).

use tier_proof::envelope::Envelope;
use tier_proof::fold::{AdmitReject, Fold, RequestStatus};
use tier_proof::identity::Signer;
use tier_proof::records::{self, Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{MemSource, RecordSource};

const GATED: &str = "scope:steward-hall";

fn s1() -> Signer {
    Signer::from_seed([30u8; 32])
}
fn s2() -> Signer {
    Signer::from_seed([31u8; 32])
}
fn bob() -> Signer {
    Signer::from_seed([32u8; 32])
}

fn gated_genesis(stewards: &[&Signer], threshold: u32) -> Genesis {
    Genesis {
        scope: GATED.to_string(),
        title: "Steward Hall".to_string(),
        write_policy: WritePolicy::Open,
        membership_policy: MembershipPolicy::Gated,
        steward_set: stewards.iter().map(|s| s.did()).collect(),
        threshold,
    }
}

#[test]
fn grant_admits_member_from_the_grants_causal_position() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    // The grant cites the request hash among its antecedents (R7 shape).
    let grant = src.put_record_with_antecedents(
        &s1(),
        vec![req.clone()],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );

    let state = Fold::run(&src.all()).expect("fold");
    assert!(
        state.roster_members(GATED).contains(&bob().did()),
        "granted member is on the roster"
    );

    // Membership starts AT the grant's causal position, not the request's.
    let grant_pos = state.position_of(&grant).expect("grant position");
    let intervals = state.member_intervals(GATED, &bob().did());
    assert_eq!(
        intervals,
        vec![(grant_pos, None)],
        "interval opens at the grant position"
    );
}

#[test]
fn a_non_steward_grant_does_not_admit() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    // bob "grants" himself — not a steward.
    src.put_record_with_antecedents(
        &bob(),
        vec![req],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    let state = Fold::run(&src.all()).expect("fold");
    assert!(
        !state.roster_members(GATED).contains(&bob().did()),
        "a self-grant does not admit"
    );
}

#[test]
fn multi_steward_cosign_reaches_threshold() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1(), &s2()], 2)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );

    // First steward grants — threshold 2 not yet reached.
    src.put_record_with_antecedents(
        &s1(),
        vec![req.clone()],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    let after_one = Fold::run(&src.all()).expect("fold");
    assert!(
        !after_one.roster_members(GATED).contains(&bob().did()),
        "one of two co-signs: not yet a member"
    );
    assert_eq!(
        after_one.request_status(GATED, &req),
        RequestStatus::Pending
    );

    // Second distinct steward co-signs — threshold reached.
    src.put_record_with_antecedents(
        &s2(),
        vec![req.clone()],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    let after_two = Fold::run(&src.all()).expect("fold");
    assert!(
        after_two.roster_members(GATED).contains(&bob().did()),
        "two co-signs: admitted"
    );
    assert_eq!(
        after_two.request_status(GATED, &req),
        RequestStatus::Granted
    );
}

fn msg_with(signer: &Signer, antecedents: Vec<String>, text: &str) -> Envelope {
    records::seal(
        signer,
        antecedents,
        &Record::Message {
            scope: GATED.to_string(),
            text: text.to_string(),
        },
    )
}

#[test]
fn revocation_is_a_causal_cut() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    let grant = src.put_record_with_antecedents(
        &s1(),
        vec![req],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    // The steward revokes bob at a causal cut.
    let cut = src.put_record(
        &s1(),
        Record::Revocation {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );

    let state = Fold::run(&src.all()).expect("fold");

    // A message whose antecedents are BEFORE the cut still verifies.
    let before = msg_with(&bob(), vec![grant.clone()], "said while a member");
    assert!(
        state.admit_message(&before).is_ok(),
        "a message before the cut is admitted"
    );

    // A message that causally depends on the revocation (or later) is rejected.
    let after = msg_with(&bob(), vec![cut.clone()], "said after the cut");
    assert_eq!(
        state.admit_message(&after),
        Err(AdmitReject::OutsideMembership),
        "a message after the cut is rejected"
    );

    // The interval set is exactly grant→cut.
    let g = state.position_of(&grant).unwrap();
    let c = state.position_of(&cut).unwrap();
    assert_eq!(
        state.member_intervals(GATED, &bob().did()),
        vec![(g, Some(c))]
    );
}

#[test]
fn silence_is_not_a_verdict() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    // A first request that IS granted, then a SECOND request nobody answers.
    let first = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    src.put_record_with_antecedents(
        &s1(),
        vec![first],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    let second = src.put_record(
        &Signer::from_seed([40u8; 32]),
        Record::Request {
            scope: GATED.to_string(),
        },
    );

    // Fold at several later points: the unanswered request never flips to a
    // verdict. There is no timeout code path to trigger.
    for _ in 0..3 {
        src.put_record(
            &bob(),
            Record::Message {
                scope: GATED.to_string(),
                text: "time passes".into(),
            },
        );
        let state = Fold::run(&src.all()).expect("fold");
        assert_eq!(
            state.request_status(GATED, &second),
            RequestStatus::Pending,
            "unanswered request stays pending"
        );
    }
}

#[test]
fn leave_excludes_without_a_steward_act() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    src.put_record_with_antecedents(
        &s1(),
        vec![req],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    // bob leaves on his own signature — no steward record follows.
    src.put_record(
        &bob(),
        Record::Leave {
            scope: GATED.to_string(),
        },
    );

    let state = Fold::run(&src.all()).expect("fold");
    assert!(
        !state.roster_members(GATED).contains(&bob().did()),
        "member leaves unilaterally"
    );
    // And this is NOT a revocation — no causal cut was recorded by a steward.
    assert_eq!(
        state.cut_position(GATED, &bob().did()),
        None,
        "leave is not a revocation"
    );
}

#[test]
fn archive_rebuild_reproduces_roster_with_signatures_reverified() {
    let mut src = MemSource::new();
    src.put_record(&s1(), Record::Genesis(gated_genesis(&[&s1()], 1)));
    let req = src.put_record(
        &bob(),
        Record::Request {
            scope: GATED.to_string(),
        },
    );
    src.put_record_with_antecedents(
        &s1(),
        vec![req],
        Record::Grant {
            scope: GATED.to_string(),
            subject: bob().did(),
        },
    );
    let events = src.all();
    let live = Fold::run(&events).expect("live fold");

    // Write the folded ops to a state table (archive), then a SECOND folder
    // rebuilds from the archive alone — signatures re-verified on rebuild.
    let archive = tier_proof::fold::archive(&events);
    let rebuilt = tier_proof::fold::rebuild_from_archive(&archive).expect("rebuild");
    assert_eq!(
        live.canonical_digest(),
        rebuilt.canonical_digest(),
        "archive rebuild reproduces the roster"
    );
    assert_eq!(
        rebuilt.dropped_count(),
        0,
        "all archived signatures re-verified"
    );

    // Tamper one archived envelope: the rebuild drops it (verifiable, not trusted).
    let tampered = tier_proof::fold::tamper_first_put_for_test(&archive);
    let rebuilt_bad = tier_proof::fold::rebuild_from_archive(&tampered).expect("rebuild bad");
    assert!(
        rebuilt_bad.dropped_count() >= 1,
        "a tampered archive entry is dropped, not trusted"
    );
}
