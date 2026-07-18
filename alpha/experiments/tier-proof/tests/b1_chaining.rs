//! B1 (RUN-18) — chaining validation in write-restricted scopes (component;
//! GROUPS.md A.2 reception paragraph).
//!
//! In a `single`-writer scope every envelope MUST carry the author's previous
//! envelope as its FIRST antecedent (the chain-link convention); the first
//! envelope anchors to the scope genesis. Validate-before-relay (§A.8) is
//! extended with the chaining check: an unchained envelope injected at the
//! relay function is dropped unpropagated. Chaining is NOT required in
//! open-write scopes (the degeneration principle: the forum asks nothing the
//! substrate does not prove).
//!
//! Reuses the landed P3 scopes (newsletter open/single, forum open/open); no
//! new processes (brief §3).

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

/// The landed P3 fixture, plus the genesis identities the chain anchors to.
fn two_policy_source() -> (MemSource, String, String) {
    let (o, a) = (owner(), alice());
    let mut src = MemSource::new();
    let nl_genesis = src.put_record(
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
    let fo_genesis = src.put_record(
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
    (src, nl_genesis, fo_genesis)
}

fn msg(signer: &Signer, antecedents: Vec<String>, scope: &str, text: &str) -> Envelope {
    records::seal(
        signer,
        antecedents,
        &Record::Message {
            scope: scope.to_string(),
            text: text.to_string(),
        },
    )
}

#[test]
fn unchained_single_writer_envelope_fails_validation() {
    let (src, _nl, _fo) = two_policy_source();
    let state = Fold::run(&src.all()).expect("fold");
    // The single writer, but no chain link at all.
    let unchained = msg(&owner(), vec![], NEWSLETTER, "issue 1, unanchored");
    assert_eq!(
        relay::accepts(&state, &unchained),
        Err(RelayReject::Unchained),
        "a single-writer envelope without the author's chain link fails validation"
    );
}

#[test]
fn first_envelope_anchors_to_scope_genesis() {
    let (src, nl_genesis, _fo) = two_policy_source();
    let state = Fold::run(&src.all()).expect("fold");
    let first = msg(&owner(), vec![nl_genesis], NEWSLETTER, "issue 1");
    assert!(
        relay::accepts(&state, &first).is_ok(),
        "the author's first envelope, anchored to the scope genesis, validates"
    );
}

#[test]
fn fold_exposes_the_genesis_identity_and_tracks_the_chain_head() {
    let (mut src, nl_genesis, _fo) = two_policy_source();
    let i1 = src.put_record_with_antecedents(
        &owner(),
        vec![nl_genesis.clone()],
        Record::Message {
            scope: NEWSLETTER.to_string(),
            text: "issue 1".to_string(),
        },
    );
    let i2 = src.put_record_with_antecedents(
        &owner(),
        vec![i1.clone()],
        Record::Message {
            scope: NEWSLETTER.to_string(),
            text: "issue 2".to_string(),
        },
    );
    let state = Fold::run(&src.all()).expect("fold");
    assert_eq!(
        state.genesis_id(NEWSLETTER),
        Some(nl_genesis),
        "the fold exposes the genesis envelope identity the chain anchors to"
    );
    assert_eq!(
        state.chain_head(NEWSLETTER, &owner().did()),
        Some(i2),
        "the fold tracks the author's chain head in a write-restricted scope"
    );
}

#[test]
fn chained_stream_relays_and_an_unchained_envelope_is_dropped_unpropagated() {
    let (src, nl_genesis, _fo) = two_policy_source();
    let state = Fold::run(&src.all()).expect("fold");

    let i1 = msg(&owner(), vec![nl_genesis.clone()], NEWSLETTER, "issue 1");
    let i2 = msg(&owner(), vec![i1.identity_hex()], NEWSLETTER, "issue 2");
    // Skips i2: re-anchors to genesis although the head has moved — unchained.
    let skip = msg(&owner(), vec![nl_genesis], NEWSLETTER, "issue 3, skipping");

    let out = relay::relay(&state, &[i1.clone(), i2.clone(), skip.clone()]);
    let ids: Vec<String> = out.iter().map(Envelope::identity_hex).collect();
    assert_eq!(
        ids,
        vec![i1.identity_hex(), i2.identity_hex()],
        "the chained prefix relays; the unchained envelope is dropped"
    );
    assert!(
        !ids.contains(&skip.identity_hex()),
        "the unchained envelope is never re-emitted (validate-before-relay)"
    );
}

#[test]
fn chaining_is_not_required_in_open_write_scopes() {
    let (src, _nl, _fo) = two_policy_source();
    let state = Fold::run(&src.all()).expect("fold");
    // The forum (open/open): a roster member's unchained post is still valid —
    // the degeneration principle in code (no proof is demanded that the
    // substrate does not require for this policy pair).
    let post = msg(&alice(), vec![], FORUM, "hello commons");
    assert!(
        relay::accepts(&state, &post).is_ok(),
        "open-write scopes do not demand chaining"
    );
}
