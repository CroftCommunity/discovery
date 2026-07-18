//! B5 (RUN-18) — retraction, the three-way distinction (component; the
//! tamper-evident-history delta of PUBLICATIONS.md §4, executable).
//!
//! Publish a short chained stream, then RETRACT a middle issue (delete the
//! content record). Assertions:
//! - the issue's EXISTENCE stays provable from the chain (its identity is
//!   referenced by its successor) even though its content is gone;
//! - the reader's detector classifies three absent-content cases distinctly
//!   and correctly: **never-existed** (no chain references it), **retracted**
//!   (referenced; deletion verifiable at source), **withheld-from-me**
//!   (referenced; no source offers it; deletion cannot be shown);
//! - a vanilla current-state check over the same repo shows the retracted
//!   issue as simply absent — indistinguishable from never-existed — the
//!   contrast that motivates the delta, asserted, not narrated.
//!
//! LIVE-OPTIONAL (guardrail 4): with `ATP_TEST_*` credentials the deletion
//! runs against the real PDS and upgrades this part to live grade; absent
//! them the deletion is the landed harness's authenticated delete event —
//! `SPEC-DELTA[run18-retraction-local | stand-in]` — and the live leg reports
//! BLOCKED, never pretended.

use tier_proof::chain::{self, Existence};
use tier_proof::envelope::Envelope;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::source::{LiveLeg, MemSource, RecordSource, SourceEvent};

const NEWSLETTER: &str = "scope:dispatch";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}

fn chained_newsletter(n: usize) -> (MemSource, String, Vec<Envelope>) {
    let o = owner();
    let mut src = MemSource::new();
    let genesis = src.put_record(
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
    let mut issues = Vec::new();
    let mut prev = genesis.clone();
    for i in 1..=n {
        let id = src.put_record_with_antecedents(
            &o,
            vec![prev.clone()],
            Record::Message {
                scope: NEWSLETTER.to_string(),
                text: format!("issue {i}"),
            },
        );
        prev = id.clone();
        let env = src
            .all()
            .into_iter()
            .find_map(|e| match e {
                SourceEvent::Put(env) if env.identity_hex() == id => Some(env),
                _ => None,
            })
            .expect("issue envelope present");
        issues.push(env);
    }
    (src, genesis, issues)
}

#[test]
fn live_retraction_leg_is_blocked_not_pretended() {
    match tier_proof::source::live_retraction_leg() {
        LiveLeg::Blocked { reason } => {
            assert!(
                reason.contains("ATP_TEST"),
                "blocked reason names the missing creds"
            );
        }
        LiveLeg::Ran { .. } => { /* credentials present: the live deletion ran */ }
    }
}

#[test]
fn retracted_issue_existence_stays_provable_and_the_three_cases_classify_distinctly() {
    let (mut src, genesis, issues) = chained_newsletter(5);

    // RETRACT the middle issue 2: the author deletes the content record.
    // SPEC-DELTA[run18-retraction-local | stand-in]: the landed harness's
    // authenticated delete event stands in for a live PDS record deletion
    // (no ATP_TEST_* credentials in this environment; guardrail 4).
    let retracted = issues[1].identity_hex();
    src.delete(&owner(), &retracted);

    // The reader's held set after backfill: issues 1, 3, 5. Issue 2 is
    // retracted (gone at source); issue 4 is withheld by every source the
    // reader reached, with no deletion shown.
    let held = vec![issues[0].clone(), issues[2].clone(), issues[4].clone()];

    // EXISTENCE stays provable from the chain: both absent identities are
    // referenced by their successors and named as known omissions.
    let report = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    let mut expected = vec![retracted.clone(), issues[3].identity_hex()];
    expected.sort();
    assert_eq!(
        report.missing, expected,
        "content gone, existence provable: the successors' antecedents name both"
    );

    // The three-way classification, distinct and correct.
    let source_view = src.all();
    assert_eq!(
        chain::classify_existence(NEWSLETTER, &owner().did(), &retracted, &held, &source_view),
        Existence::Retracted,
        "referenced + deletion verifiable at source = retracted"
    );
    assert_eq!(
        chain::classify_existence(
            NEWSLETTER,
            &owner().did(),
            &issues[3].identity_hex(),
            &held,
            &source_view
        ),
        Existence::WithheldFromMe,
        "referenced + not offered + deletion not shown = withheld-from-me"
    );
    let never = "deadbeef".repeat(8);
    assert_eq!(
        chain::classify_existence(NEWSLETTER, &owner().did(), &never, &held, &source_view),
        Existence::NeverExisted,
        "an identity no chain references never existed"
    );
}

#[test]
fn vanilla_current_state_cannot_distinguish_retracted_from_never_existed() {
    let (mut src, _genesis, issues) = chained_newsletter(5);
    let retracted = issues[1].identity_hex();
    src.delete(&owner(), &retracted); // SPEC-DELTA[run18-retraction-local | stand-in]

    let events = src.all();
    let never = "deadbeef".repeat(8);

    // The vanilla check sees only the current state: the retracted issue is
    // simply absent, exactly like an identity that never existed.
    assert!(
        !chain::vanilla_present(&events, &retracted),
        "vanilla current state: the retracted issue is absent"
    );
    assert!(
        !chain::vanilla_present(&events, &never),
        "vanilla current state: a never-existed identity is absent"
    );
    // ...the same absence. The chained reader distinguishes them (see the
    // classification test) — the delta, asserted.
    assert!(
        chain::vanilla_present(&events, &issues[2].identity_hex()),
        "an unretracted issue is present in the current state"
    );
}
