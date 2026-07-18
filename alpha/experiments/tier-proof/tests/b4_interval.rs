//! B4 (RUN-18) — chaining and interval backfill compose (component;
//! GROUPS.md A.2 + A.7).
//!
//! A subscriber enrolled after position J repairs gaps within [J, now) only.
//! The chain crossing J is visible AS STRUCTURE — the antecedent reference
//! exists and the detector names it — without the pre-J envelope ever being
//! offered: detection may see history's shape; OFFERING respects the interval
//! rule (the landed P8 offering-side refusal).
//!
//! Integration over B1–B3 + the landed P8 store; introduces no new machinery
//! by design (recorded as such in RUN-18-SUMMARY.md if born green).

use tier_proof::chain;
use tier_proof::envelope::Envelope;
use tier_proof::fold::Fold;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::roles::{self, EnvelopeStore, OfferReject};
use tier_proof::source::{MemSource, RecordSource, SourceEvent};

const NEWSLETTER: &str = "scope:dispatch";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}
fn late_subscriber() -> Signer {
    Signer::from_seed([22u8; 32])
}

/// Timeline: genesis · issues 1–3 · the late subscriber enrolls (position J) ·
/// issues 4–6. Returns (source, genesis id, issues in order, J).
fn newsletter_with_late_enrollment() -> (MemSource, String, Vec<Envelope>, u64) {
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
    let put_issue = |src: &mut MemSource, prev: &mut String, i: usize| {
        let id = src.put_record_with_antecedents(
            &o,
            vec![prev.clone()],
            Record::Message {
                scope: NEWSLETTER.to_string(),
                text: format!("issue {i}"),
            },
        );
        *prev = id.clone();
        src.all()
            .into_iter()
            .find_map(|e| match e {
                SourceEvent::Put(env) if env.identity_hex() == id => Some(env),
                _ => None,
            })
            .expect("issue envelope present")
    };
    for i in 1..=3 {
        issues.push(put_issue(&mut src, &mut prev, i));
    }
    let enrollment = src.put_record(
        &late_subscriber(),
        Record::SelfRegistration {
            scope: NEWSLETTER.to_string(),
        },
    );
    for i in 4..=6 {
        issues.push(put_issue(&mut src, &mut prev, i));
    }
    let state = Fold::run(&src.all()).expect("fold");
    let j = state.position_of(&enrollment).expect("enrollment position");
    (src, genesis, issues, j)
}

#[test]
fn late_subscriber_repairs_within_interval_only_and_sees_the_crossing_as_structure() {
    let (src, genesis, issues, j) = newsletter_with_late_enrollment();
    let state = Fold::run(&src.all()).expect("fold");
    let intervals = state.member_intervals(NEWSLETTER, &late_subscriber().did());
    assert_eq!(intervals, vec![(j, None)], "enrolled at J, open-ended");

    // The landed store holds the whole stream at causal positions.
    let mut store = EnvelopeStore::new();
    for env in &issues {
        let pos = state.position_of(&env.identity_hex()).expect("position");
        store.insert(env.clone(), pos);
    }
    let now = state
        .position_of(&issues.last().expect("issues").identity_hex())
        .expect("position")
        + 1;

    // Backfill [J, now): exactly the post-enrollment issues are offered.
    let offered = roles::offer_interval(&store, &intervals, (j, now)).expect("offer");
    let offered_ids: Vec<String> = offered.iter().map(|(e, _)| e.identity_hex()).collect();
    let expected_ids: Vec<String> = issues[3..].iter().map(Envelope::identity_hex).collect();
    assert_eq!(offered_ids, expected_ids, "the interval window and no more");

    // A window reaching before J is refused outright — the pre-J envelope is
    // never offered (offering-side refusal, P8).
    assert_eq!(
        roles::offer_interval(&store, &intervals, (0, now)),
        Err(OfferReject::NotProven),
        "offering respects the interval rule"
    );

    // Detection over the held (post-J) stream: the chain crossing J is visible
    // as structure — issue 4's antecedent names issue 3 — without issue 3
    // being held or offered.
    let held: Vec<Envelope> = offered.into_iter().map(|(e, _)| e).collect();
    let report = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    assert_eq!(
        report.missing,
        vec![issues[2].identity_hex()],
        "the crossing reference exists and is named: history's shape is seen"
    );
    assert!(!report.anchored, "the held stream does not reach genesis");
    assert_eq!(
        report.heads,
        vec![issues.last().expect("issues").identity_hex()],
        "one held segment, headed by the newest issue"
    );
}

#[test]
fn a_gap_within_the_interval_is_repairable_while_the_crossing_stays_structural() {
    let (src, genesis, issues, j) = newsletter_with_late_enrollment();
    let state = Fold::run(&src.all()).expect("fold");
    let intervals = state.member_intervals(NEWSLETTER, &late_subscriber().did());

    let mut store = EnvelopeStore::new();
    for env in &issues {
        let pos = state.position_of(&env.identity_hex()).expect("position");
        store.insert(env.clone(), pos);
    }
    let now = state
        .position_of(&issues.last().expect("issues").identity_hex())
        .expect("position")
        + 1;

    // The reader lost issue 5 (inside the interval): two known omissions —
    // the in-window gap and the crossing.
    let held: Vec<Envelope> = vec![issues[3].clone(), issues[5].clone()];
    let before = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    let mut expected_missing = vec![issues[2].identity_hex(), issues[4].identity_hex()];
    expected_missing.sort();
    assert_eq!(before.missing, expected_missing);

    // Repair from the proven window: the in-window gap closes; the crossing
    // remains structural (and is the ONLY remaining omission).
    let offered = roles::offer_interval(&store, &intervals, (j, now)).expect("offer");
    let mut repaired = held;
    for (env, _) in offered {
        if !repaired.iter().any(|h| h.identity_hex() == env.identity_hex()) {
            repaired.push(env);
        }
    }
    let after = chain::detect(&genesis, NEWSLETTER, &owner().did(), &repaired);
    assert_eq!(
        after.missing,
        vec![issues[2].identity_hex()],
        "repairs happen within [J, now) only; the crossing is not repairable \
         and stays visible as structure"
    );
    assert_eq!(after.heads.len(), 1, "the held segment is whole again");
}
