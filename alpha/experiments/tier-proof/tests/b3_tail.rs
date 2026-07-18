//! B3 (RUN-18) — the tail, honestly (component; GROUPS.md A.2 + A.8).
//!
//! With the newest envelope withheld, the DETECTOR'S OWN CLAIM is under test:
//! it MUST report "complete as of <newest held>" and MUST NOT claim full
//! currency — the withheld tail is undetectable until anything newer arrives
//! by any path (the completeness-ahead doctrine). Then the tail arrives via
//! the landed P8 swarm-path machinery (a second delivery path converged by
//! `H(envelope)`), is detected and folded, and the claim advances: the
//! multimodal closure, executable.

use tier_proof::chain;
use tier_proof::envelope::Envelope;
use tier_proof::identity::Signer;
use tier_proof::records::{Genesis, MembershipPolicy, Record, WritePolicy};
use tier_proof::roles::{self, EnvelopeStore};
use tier_proof::source::{MemSource, RecordSource, SourceEvent};

const NEWSLETTER: &str = "scope:dispatch";

fn owner() -> Signer {
    Signer::from_seed([20u8; 32])
}
fn subscriber() -> Signer {
    Signer::from_seed([21u8; 32])
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
fn withheld_tail_claim_is_complete_as_of_newest_held_and_never_currency() {
    let (_src, genesis, issues) = chained_newsletter(5);
    // The newest issue is withheld: the reader holds a perfect prefix.
    let held: Vec<Envelope> = issues[..4].to_vec();
    let report = chain::detect(&genesis, NEWSLETTER, &owner().did(), &held);

    // The withheld tail is invisible: the held prefix looks complete...
    assert!(
        report.complete_up_to_newest(),
        "a withheld TAIL is undetectable from the chain alone"
    );
    let newest_held = issues[3].identity_hex();
    assert_eq!(report.newest_held(), Some(newest_held.as_str()));

    // ...so the claim MUST be scoped to the newest held envelope, exactly,
    // and MUST NOT assert currency in any wording.
    let claim = report.claim();
    assert_eq!(
        claim,
        format!("complete as of {newest_held}"),
        "the claim is 'complete as of <newest held>' and nothing stronger"
    );
    assert!(
        !claim.to_lowercase().contains("current") && !claim.to_lowercase().contains("up to date"),
        "full currency is never claimed"
    );
}

#[test]
fn tail_arriving_via_the_swarm_path_is_detected_folded_and_the_claim_advances() {
    let (_src, genesis, issues) = chained_newsletter(5);

    // DS path delivered the prefix; the withheld tail exists only on the
    // swarm path (the landed P8 second-delivery-path machinery).
    let mut ds = EnvelopeStore::new();
    for (i, env) in issues[..4].iter().enumerate() {
        ds.insert(env.clone(), i as u64);
    }
    let mut swarm = EnvelopeStore::new();
    swarm.insert(issues[4].clone(), 4);

    let before = chain::detect(
        &genesis,
        NEWSLETTER,
        &owner().did(),
        &ds.entries().into_iter().map(|(e, _)| e).collect::<Vec<_>>(),
    );
    assert_eq!(
        before.claim(),
        format!("complete as of {}", issues[3].identity_hex())
    );

    // The second path converges by H(envelope) (P8): the tail arrives, is
    // detected as newer (it chains onto the previously-newest held), folds in,
    // and the claim advances — silent-to-detected, then closed.
    let converged = roles::converge(&[&ds, &swarm]);
    assert_eq!(converged.len(), 5, "the tail folds in, deduped by identity");
    let after = chain::detect(&genesis, NEWSLETTER, &owner().did(), &converged);
    assert!(after.complete_up_to_newest());
    assert_eq!(
        after.claim(),
        format!("complete as of {}", issues[4].identity_hex()),
        "the claim advances to the arrived tail"
    );
}
