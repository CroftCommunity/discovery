//! B6 (RUN-18) — auditable reach (component, vs the landed harness; the
//! subscriber-count claim of PUBLICATIONS.md §4/§5, executable).
//!
//! On the landed P2 open-tier machinery:
//! - an INDEPENDENT second fold over the same records re-derives EXACTLY the
//!   roster count the DS serves;
//! - an unsubscribe (authenticated deletion of the self-registration) moves
//!   both counts identically;
//! - a count the DS merely asserts, with no folded records behind it, is
//!   detectable as unsupported by the auditor.
//!
//! Grade: component against the landed harness — RUN-17's P2 live legs were
//! BLOCKED and wrote no live records, so there are none to reuse (noted per
//! the brief; the live upgrade rides the P2 `ATP_TEST_*` path).

use tier_proof::audit::{self, CountVerdict};
use tier_proof::fold::Fold;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{MemSource, RecordSource};

const NEWSLETTER: &str = "scope:dispatch";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}

/// The open-enrollment newsletter with three subscribers; returns the source
/// and one subscriber's registration id (for the unsubscribe).
fn enrolled_newsletter() -> (MemSource, Signer, String) {
    let o = owner();
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
    let mut bob_reg = String::new();
    let mut bob = Signer::from_seed([31u8; 32]);
    for seed in [30u8, 31, 32] {
        let s = Signer::from_seed([seed; 32]);
        let reg = src.put_record(
            &s,
            Record::SelfRegistration {
                scope: NEWSLETTER.to_string(),
            },
        );
        if seed == 31 {
            bob_reg = reg;
            bob = s;
        }
    }
    (src, bob, bob_reg)
}

#[test]
fn an_independent_second_fold_rederives_exactly_the_count_the_ds_serves() {
    let (src, _bob, _reg) = enrolled_newsletter();

    // The DS's own fold — what it serves as the subscriber count.
    let ds_state = Fold::run(&src.all()).expect("ds fold");
    let ds_count = ds_state.roster_members(NEWSLETTER).len();
    assert_eq!(ds_count, 3);

    // The auditor's INDEPENDENT fold over the same records.
    let derived = audit::derive_roster_count(&src.all(), NEWSLETTER);
    assert_eq!(
        derived, ds_count,
        "the second fold re-derives exactly the served count"
    );
    assert_eq!(
        audit::audit_count(&src.all(), NEWSLETTER, ds_count),
        CountVerdict::Supported,
        "a served count backed by folded records audits as supported"
    );
}

#[test]
fn an_unsubscribe_moves_both_counts_identically() {
    let (mut src, bob, bob_reg) = enrolled_newsletter();

    // The unsubscribe: an authenticated deletion of bob's own registration.
    src.delete(&bob, &bob_reg);

    let ds_count = Fold::run(&src.all())
        .expect("ds fold")
        .roster_members(NEWSLETTER)
        .len();
    let derived = audit::derive_roster_count(&src.all(), NEWSLETTER);
    assert_eq!(ds_count, 2, "the DS count drops by one");
    assert_eq!(derived, 2, "the audited count drops by the same one");
    assert_eq!(
        audit::audit_count(&src.all(), NEWSLETTER, ds_count),
        CountVerdict::Supported
    );
}

#[test]
fn a_merely_asserted_count_is_detectable_as_unsupported() {
    let (src, _bob, _reg) = enrolled_newsletter();

    // The DS asserts a reach of 5000 with no folded records behind it.
    assert_eq!(
        audit::audit_count(&src.all(), NEWSLETTER, 5000),
        CountVerdict::Unsupported {
            asserted: 5000,
            derived: 3
        },
        "an asserted count with no records behind it fails the audit, and the \
         auditor names the number the records actually support"
    );
}
