//! B6 — omission detection (RUN-HIST-01).
//!
//! Serves rbsr-construction req. 5 (omission resistance at the content-blind
//! store seam) and history-durability.md §I's admission rule via
//! HIST-ATPROTO-MATCHUP.md row 6: a responder serving pages with one entry
//! withheld is detected by the member via predecessor linkage regardless of
//! responder honesty. Admission REUSES the proven A2.3 machinery — standing
//! plus contiguity, `lineage-history::backfill_import` (Proofs Phase 2.5) —
//! never a reimplementation. RED-able: a signature-valid but non-contiguous
//! span MUST be rejected; the staged red is the signature-deep check A2.3
//! proved insufficient, which accepts all three illegitimate spans below.

use hist_atproto_spike::envelope::fixture_chain;
use hist_atproto_spike::omission::{admit_span, registry_verify, AdmitError};
use lineage_core::dag::Lineage;
use lineage_core::ids::{lineage_genesis, Did};
use lineage_core::keys::SigningIdentity;
use lineage_history::{BackfillError, BranchHistory, Message};

/// Fixture: the group lineage, its standing author, a well-signed stranger,
/// and the author's envelope chain as signed span messages.
struct Fixture {
    lineage: Lineage,
    genesis: lineage_core::ids::GenesisId,
    author: SigningIdentity,
    stranger: SigningIdentity,
}

fn fixture() -> Fixture {
    let genesis = lineage_genesis("hist-spike-b6-group");
    let author = SigningIdentity::from_seed(Did::new("did:fixture:scribe-author"), 61);
    let stranger = SigningIdentity::from_seed(Did::new("did:fixture:stranger"), 62);
    let mut lineage = Lineage::new();
    // The stranger is deliberately NOT among the branch's ever-members.
    lineage.add_root(genesis, [author.did().clone()]);
    Fixture {
        lineage,
        genesis,
        author,
        stranger,
    }
}

/// A donor branch carrying the chain's envelopes as signed message payloads,
/// optionally withholding one index (keeping original seqs — the honest-
/// looking omission).
fn donor_span(
    f: &Fixture,
    signer: &SigningIdentity,
    label: &str,
    n: u64,
    withhold: Option<usize>,
) -> BranchHistory {
    // First build the fully-signed branch…
    let mut full = BranchHistory::new(f.genesis);
    for (env, _) in fixture_chain(label, n) {
        full.append(signer, &env.canonical_bytes());
    }
    match withhold {
        None => full,
        Some(k) => {
            // …then re-serve it with entry k withheld, seqs untouched: every
            // message still verifies individually; the SPAN is illegitimate.
            let mut served = BranchHistory::new(f.genesis);
            for (i, m) in full.messages().iter().enumerate() {
                if i != k {
                    served.push_raw(m.clone());
                }
            }
            served
        }
    }
}

#[test]
fn honest_span_is_admitted_and_yields_the_chain() {
    let f = fixture();
    let donor = donor_span(&f, &f.author, "b6-honest", 8, None);
    let verifiers = [f.author.verifying(), f.stranger.verifying()];
    let envs = admit_span(&donor, f.genesis, &f.lineage, registry_verify(&verifiers))
        .expect("legitimate history admits");
    assert_eq!(envs.len(), 8);
    assert_eq!(envs.iter().map(|e| e.counter).collect::<Vec<_>>(), (0..8).collect::<Vec<_>>());
}

#[test]
fn withheld_entry_with_valid_signatures_is_rejected_noncontiguous() {
    let f = fixture();
    // Entry 4 withheld; every remaining message individually well-signed.
    let donor = donor_span(&f, &f.author, "b6-withheld", 8, Some(4));
    let verifiers = [f.author.verifying()];
    match admit_span(&donor, f.genesis, &f.lineage, registry_verify(&verifiers)) {
        Err(AdmitError::Backfill(BackfillError::NonContiguous { index, seq })) => {
            assert_eq!((index, seq), (4, 5), "the omission is located, not vague");
        }
        Ok(envs) => panic!(
            "A2.3 regression: a signature-valid but non-contiguous span was \
             ADMITTED ({} entries) — admission must be standing plus \
             contiguity, not signature-deep",
            envs.len()
        ),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}

#[test]
fn compliant_transport_omission_is_named_by_bounding_digests() {
    // The responder hides the omission from the transport layer: it re-signs
    // a fresh, contiguous-seq span whose PAYLOADS skip envelope 4. (It can,
    // here, because the fixture "responder" holds the author key — the worst
    // case: a compromised scribe.) Transport checks all pass; the §G
    // predecessor chain names the missing span anyway.
    let f = fixture();
    let chain = fixture_chain("b6-payload-gap", 8);
    let mut donor = BranchHistory::new(f.genesis);
    for (i, (env, _)) in chain.iter().enumerate() {
        if i != 4 {
            donor.append(&f.author, &env.canonical_bytes());
        }
    }
    let verifiers = [f.author.verifying()];
    match admit_span(&donor, f.genesis, &f.lineage, registry_verify(&verifiers)) {
        Err(AdmitError::EnvelopeGap {
            subspace,
            after_digest,
            before_digest,
        }) => {
            assert_eq!(subspace, chain[0].0.subspace);
            assert_eq!(
                (after_digest, before_digest),
                (chain[3].0.entry_digest, chain[5].0.entry_digest),
                "the omission is named by its §I bounding digests, \
                 responder honesty irrelevant"
            );
        }
        Ok(envs) => panic!(
            "omission undetected: a span with a predecessor-chain break was \
             admitted whole ({} entries)",
            envs.len()
        ),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}

#[test]
fn strangers_wellsigned_history_is_rejected_unauthorized() {
    let f = fixture();
    // Perfectly-signed span by an identity with no standing on the lineage.
    let donor = donor_span(&f, &f.stranger, "b6-stranger", 5, None);
    let verifiers = [f.author.verifying(), f.stranger.verifying()];
    match admit_span(&donor, f.genesis, &f.lineage, registry_verify(&verifiers)) {
        Err(AdmitError::Backfill(BackfillError::UnauthorizedAuthor { author })) => {
            assert_eq!(&author, f.stranger.did());
        }
        Ok(envs) => panic!(
            "A2.3 regression: a stranger's perfectly-signed \"history\" was \
             ADMITTED ({} entries) — standing is required, not just a valid \
             signature",
            envs.len()
        ),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}

#[test]
fn tampered_ordering_fails_signature_binding() {
    // Renumbering seqs to fake contiguity breaks the signatures (seq is
    // inside the signed bytes) — the A2.3 second illegitimate branch.
    let f = fixture();
    let full = donor_span(&f, &f.author, "b6-renumber", 6, None);
    let mut renumbered = BranchHistory::new(f.genesis);
    for (i, m) in full.messages().iter().enumerate().filter(|(i, _)| *i != 2) {
        let new_seq = if i < 2 { i as u64 } else { i as u64 - 1 };
        renumbered.push_raw(Message {
            seq: new_seq,
            ..m.clone()
        });
    }
    let verifiers = [f.author.verifying()];
    match admit_span(&renumbered, f.genesis, &f.lineage, registry_verify(&verifiers)) {
        Err(AdmitError::Backfill(BackfillError::BadSignature { seq, .. })) => {
            assert_eq!(seq, 2, "the first renumbered message fails its binding");
        }
        Ok(_) => panic!("tampered ordering admitted"),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}
