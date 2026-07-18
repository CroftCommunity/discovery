//! B2 (RUN-18) — gap detection and repair (component; GROUPS.md A.2).
//!
//! A reader handed the newsletter stream minus one MIDDLE envelope detects the
//! gap **from the chain alone** (no oracle: no counts, no positions, no serving
//! node's word), names the missing identity, repairs it via interval backfill
//! from the landed DS/convergence store (P8), and ends **provably complete up
//! to the newest envelope held**.

use tier_proof::chain;
use tier_proof::envelope::Envelope;
use tier_proof::fold::Fold;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::roles::EnvelopeStore;
use tier_proof::source::{MemSource, RecordSource, SourceEvent};

const NEWSLETTER: &str = "scope:dispatch";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}
fn subscriber() -> Signer {
    Signer::from_seed([21u8; 32])
}

/// A newsletter with `n` chained issues. Returns the source, the genesis
/// identity, and the issue envelopes in order.
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
    src.put_record(
        &subscriber(),
        Record::SelfRegistration {
            scope: NEWSLETTER.to_string(),
        },
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
fn middle_gap_is_detected_and_named_from_the_chain_alone() {
    let (_src, genesis, issues) = chained_newsletter(5);
    let missing_issue = &issues[2]; // issue 3, a MIDDLE envelope
    let held: Vec<Envelope> = issues
        .iter()
        .filter(|e| e.identity_hex() != missing_issue.identity_hex())
        .cloned()
        .collect();

    let report = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    assert_eq!(
        report.missing,
        vec![missing_issue.identity_hex()],
        "the gap is a KNOWN omission: the missing identity is named"
    );
    assert!(
        !report.complete_up_to_newest(),
        "with a middle gap the stream is not complete"
    );
    assert_eq!(
        report.heads.len(),
        2,
        "the chain splits into two segments around the gap (no oracle needed \
         to see the shape)"
    );
    assert!(
        report.anchored,
        "the lower segment still reaches the genesis anchor"
    );
}

#[test]
fn repair_via_interval_backfill_ends_provably_complete_up_to_newest() {
    let (src, genesis, issues) = chained_newsletter(5);
    let state = Fold::run(&src.all()).expect("fold");

    // The landed DS/convergence store holds the full stream at its causal
    // positions (P8 machinery, reused — no new process).
    let mut store = EnvelopeStore::new();
    for env in &issues {
        let pos = state.position_of(&env.identity_hex()).expect("position");
        store.insert(env.clone(), pos);
    }

    // The reader holds everything except issue 3, detects, and names the gap.
    let missing_issue = &issues[2];
    let mut held: Vec<Envelope> = issues
        .iter()
        .filter(|e| e.identity_hex() != missing_issue.identity_hex())
        .cloned()
        .collect();
    let before = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    assert_eq!(before.missing, vec![missing_issue.identity_hex()]);

    // Repair: backfill the reader's proven window from the store. The
    // subscriber self-registered at position 1 with an open interval, so the
    // whole issue window is proven; the offered set contains the gap.
    let intervals = state.member_intervals(NEWSLETTER, &subscriber().did());
    let newest_pos = state
        .position_of(&issues.last().expect("issues").identity_hex())
        .expect("position");
    let offered = tier_proof::roles::offer_interval(&store, &intervals, (1, newest_pos + 1))
        .expect("backfill offer");
    assert!(
        offered
            .iter()
            .any(|(e, _)| e.identity_hex() == missing_issue.identity_hex()),
        "the backfill offer carries the missing envelope"
    );
    for (env, _) in offered {
        if !held.iter().any(|h| h.identity_hex() == env.identity_hex()) {
            held.push(env);
        }
    }

    // Provably complete up to the newest envelope held.
    let after = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);
    assert!(after.missing.is_empty(), "no known omissions remain");
    assert!(after.complete_up_to_newest());
    assert_eq!(
        after.newest_held(),
        Some(issues.last().expect("issues").identity_hex()).as_deref(),
        "the completeness claim reaches exactly the newest held envelope"
    );
}
