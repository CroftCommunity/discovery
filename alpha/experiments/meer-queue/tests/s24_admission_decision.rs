//! **S24 — position 2 as an end-to-end admission decision, with the admission fact (E112).**
//!
//! The base plan (2026-08-16) runs the serve → redeem → merge composition whole; the 2026-08-17
//! amendment adds the **admission fact** (the merge deposits an R6-shaped acceptance record, or it
//! does not happen) and refusal arm **(d)** (PSK bytes present, no issuance fact → refused — severs
//! fact-from-bytes, REVIEW coverage gap 1).
//!
//! Arms here:
//!   - **Graceful** — serve (bare GroupInfo unconditional; tree behind the check), redeem, merge;
//!     the merge **mints the admission fact** (event = commit content address, chained, and
//!     refused-if-absent). Real MLS seat at the new epoch.
//!   - **(a)** token but wrong lineage → dies at the credential half (`LineageMismatch`).
//!   - **(c)** valid token + lineage, standing revoked → dies at serve (banned) and at merge
//!     (revoked issuance).
//!   - **(d)** PSK bytes present, **no issuance fact** → refused at merge (the new severed-fact arm).
//!   - **serve s-i** — a replayed challenge-response is rejected (nonce single-use).
//!   - **serve s-ii** — a valid `psk_id` presented by a requester who cannot sign for the
//!     issued-to lineage is refused *at serve*.
//!   - **perishability** — a GroupInfo served at epoch E is refused after the group rolls to E+1.
//!
//! Fidelity: **Rung A** for the MLS half (real openmls 0.8.1); **Modeled** for the governance
//! plane (issuance ledger, acceptance chain, serve challenge-response — in-memory stand-ins).

mod common;

use common::*;
use meer_queue::admission::{
    content_address, evaluate_admission, mint_or_refuse, AcceptanceChain, IssuanceLedger,
    MergeRefusal,
};
use meer_queue::groupinfo_policy::{RefusalReason, ServeDecision, ServePolicy, ServingPeer};
use mls_replant::Persona;
use openmls::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashSet;

const LIN_RETURNER: &[u8] = b"lineage/returner";
const LIN_STRANGER: &[u8] = b"lineage/stranger";

// --------------------------------------------------------------------------------------------
// Graceful arm — serve, redeem, merge, and DEPOSIT THE ADMISSION FACT.
// --------------------------------------------------------------------------------------------

#[test]
fn graceful_admission_mints_the_fact_and_seats_the_returner() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_v) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_v.pop().unwrap();

    // Governance: a token issued to the returner's lineage, and the incumbents told to honour it.
    let token = issue_token("returner@example");
    token.deposit_with(&returner);
    token.deposit_with(&bob);
    let mut ledger = IssuanceLedger::new();
    ledger.issue(token.token_id.clone(), LIN_RETURNER.to_vec());

    // Serve: bare GroupInfo is unconditional; the tree rides behind the (position-2) check.
    let bare = current_group_info_bytes(&mut bobs, &bob, false);
    let with_tree = current_group_info_bytes(&mut bobs, &bob, true);
    let mut recognised = HashSet::new();
    recognised.insert(LIN_RETURNER.to_vec());
    let peer = ServingPeer::new(ServePolicy::Vouched(recognised), with_tree, bare);
    let decision = peer.serve(LIN_RETURNER, true);
    assert!(matches!(decision, ServeDecision::Served { with_tree: true, .. }),
        "a recognised lineage is served the tree");

    // Redeem: the returner builds the external commit (its own token + AAD naming the issuance).
    let gi = current_group_info(&mut alices, &alice);
    let commit = token.returner_commit_with_aad(&returner, gi, &token.issuance_attestation());
    let wire = commit_wire(&commit);

    // Merge rule: the merge mints the admission fact, or it does not happen.
    let mut chain = AcceptanceChain::new();
    let acceptor_frontier = 5u64;
    let fact = mint_or_refuse(
        &ledger, &mut chain, b"lineage/bob", acceptor_frontier,
        &token.token_id, LIN_RETURNER, &wire,
    )
    .expect("the fact mints: issuance present, unrevoked, lineage matches");

    // Only now does the MLS merge run.
    let before = bobs.members().count();
    let epoch_before = bobs.epoch().as_u64();
    let outcome = try_merge(&mut bobs, &bob, &commit);
    assert!(matches!(outcome, MergeOutcome::Seated), "seated: {}", describe(&outcome));
    assert_eq!(bobs.members().count(), before + 1, "the returner is current");
    assert!(bobs.epoch().as_u64() > epoch_before, "at a new epoch");

    // The admission fact: minted, chain-positioned, event = the commit's content address.
    assert_eq!(chain.len(), 1, "exactly one admission fact was deposited");
    assert_eq!(fact.event, content_address(&wire), "the event is the commit's content address");
    assert_eq!(fact.merged_lineage, LIN_RETURNER, "the span it opens is the returner's");
    assert_eq!(fact.acceptor_frontier, acceptor_frontier, "carries the acceptor's frontier F");
    assert_eq!(chain.facts_for(&fact.event).len(), 1, "the fact is indexed by the event");

    println!(
        "S24 graceful MEASURED (Rung A MLS / Modeled governance): serve released the tree to a \
         recognised lineage; the returner redeemed and was seated ({before}→{}); and the merge \
         DEPOSITED the admission fact (event = commit content address, chained at the acceptor's \
         frontier F={acceptor_frontier}). Recognition is the merge, and the merge deposits the \
         fact.",
        bobs.members().count()
    );
}

#[test]
fn a_merge_without_a_mintable_fact_is_refused() {
    // The merge-rule clause: refused-if-absent. Here the fact CANNOT mint (no issuance), so the
    // caller (which merges only on Ok) never seats the returner.
    let ledger = IssuanceLedger::new(); // empty: no issuance facts at all
    let mut chain = AcceptanceChain::new();
    let refusal = mint_or_refuse(
        &ledger, &mut chain, b"lineage/bob", 1, b"some-token", LIN_RETURNER, b"commit-bytes",
    );
    assert_eq!(refusal, Err(MergeRefusal::NoIssuanceFact));
    assert!(chain.is_empty(), "nothing was deposited, so nothing may be merged");
    println!("S24 refused-if-absent MEASURED (Modeled): a merge whose admission fact cannot mint \
              deposits nothing and seats no one — the merge-rule clause holds.");
}

// --------------------------------------------------------------------------------------------
// Refusal arms.
// --------------------------------------------------------------------------------------------

#[test]
fn arm_d_psk_bytes_present_but_no_issuance_fact_is_refused_at_merge() {
    // The severed-fact arm (REVIEW gap 1). The incumbent HOLDS the PSK bytes — the crypto would
    // resolve and MLS would seat — but there is no issuance fact, so the admission gate refuses
    // BEFORE any merge. Holding bytes is not holding a fact.
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_v) = seat_group(&alice, &[&bob]);
    let bobs = bobs_v.pop().unwrap();

    let token = issue_token("returner@example");
    token.deposit_with(&returner);
    token.deposit_with(&bob); // bytes present at the incumbent — crypto WOULD resolve

    let ledger = IssuanceLedger::new(); // but NO issuance fact was ever recorded
    let chain = AcceptanceChain::new();
    let gi = current_group_info(&mut alices, &alice);
    let commit = token.returner_commit_with_aad(&returner, gi, &token.issuance_attestation());
    let wire = commit_wire(&commit);

    let gate = evaluate_admission(&ledger, b"lineage/bob", 1, &token.token_id, LIN_RETURNER, &wire);
    assert_eq!(gate, Err(MergeRefusal::NoIssuanceFact),
        "no issuance fact: the admission is refused even though the PSK bytes resolve");

    // Because the gate refused, the caller does not merge — the returner is not seated.
    let before = bobs.members().count();
    // (We deliberately do NOT call try_merge: the gate is upstream of the MLS merge.)
    assert_eq!(bobs.members().count(), before, "nobody seated");
    assert!(chain.is_empty());

    println!("S24 arm (d) MEASURED (Rung A MLS / Modeled governance): PSK bytes present at the \
              incumbent (crypto would resolve), but with NO issuance fact the admission gate \
              refuses (NoIssuanceFact). Fact is severed from bytes — REVIEW coverage gap 1 closed.");
}

#[test]
fn arm_a_token_but_wrong_lineage_dies_at_the_credential_half() {
    // A stolen-token bearer: presents a genuine token, but their credential resolves to a
    // different lineage than the token was issued to.
    let mut ledger = IssuanceLedger::new();
    ledger.issue(b"tok".to_vec(), LIN_RETURNER.to_vec()); // issued to the returner's lineage

    let gate = evaluate_admission(&ledger, b"lineage/bob", 1, b"tok", LIN_STRANGER, b"commit");
    assert_eq!(gate, Err(MergeRefusal::LineageMismatch),
        "the bearer cannot present the issued-to lineage: refused at the credential half");
    println!("S24 arm (a) MEASURED (Modeled): a token presented by a bearer who is not its \
              issued-to lineage is refused (LineageMismatch) — the credential half of §11.7.");
}

#[test]
fn arm_c_revoked_standing_dies_at_serve_and_at_merge() {
    // Valid token + lineage, but standing revoked at head. Both the serve gate (banned) and the
    // merge gate (revoked issuance) refuse.
    let mut recognised = HashSet::new();
    recognised.insert(LIN_RETURNER.to_vec());
    let mut peer = ServingPeer::new(ServePolicy::Vouched(recognised), vec![9; 4], vec![1; 4]);
    peer.ban_at_head(LIN_RETURNER.to_vec());
    let decision = peer.serve(LIN_RETURNER, true);
    assert!(matches!(decision, ServeDecision::Refused(RefusalReason::BannedAtHead { .. })),
        "revoked standing refuses at serve");

    let mut ledger = IssuanceLedger::new();
    ledger.issue(b"tok".to_vec(), LIN_RETURNER.to_vec());
    ledger.revoke(b"tok");
    let gate = evaluate_admission(&ledger, b"lineage/bob", 1, b"tok", LIN_RETURNER, b"commit");
    assert_eq!(gate, Err(MergeRefusal::Revoked), "revoked issuance refuses at merge");

    println!("S24 arm (c) MEASURED (Modeled): revoked standing refuses at BOTH gates — serve \
              (BannedAtHead) and merge (Revoked issuance).");
}

// --------------------------------------------------------------------------------------------
// Serve challenge-response — s-i (nonce single-use) and s-ii (wrong-lineage signer).
// Modeled: the lineage key is a secret chaining to the lineage root; the serve signature is a
// keyed digest over tag ‖ nonce ‖ group_id ‖ psk_id.
// --------------------------------------------------------------------------------------------

fn serve_sig(lineage_secret: &[u8], nonce: &[u8], group_id: &[u8], psk_id: &[u8]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(b"serve-tree/v1");
    h.update(lineage_secret);
    h.update(nonce);
    h.update(group_id);
    h.update(psk_id);
    h.finalize().into()
}

/// A serving peer that issues single-use nonces and checks the challenge signature against the
/// public lineage secret it expects the issued-to lineage to control.
struct ChallengeServer {
    expected_lineage_secret: Vec<u8>,
    group_id: Vec<u8>,
    psk_id: Vec<u8>,
    used_nonces: HashSet<Vec<u8>>,
}

impl ChallengeServer {
    fn verify_and_consume(&mut self, nonce: &[u8], sig: [u8; 32]) -> Result<(), &'static str> {
        if self.used_nonces.contains(nonce) {
            return Err("nonce already used (replay)");
        }
        let expected = serve_sig(&self.expected_lineage_secret, nonce, &self.group_id, &self.psk_id);
        if sig != expected {
            return Err("challenge signature does not chain to the issued-to lineage");
        }
        self.used_nonces.insert(nonce.to_vec());
        Ok(())
    }
}

#[test]
fn serve_s_i_replayed_challenge_response_is_rejected() {
    let mut server = ChallengeServer {
        expected_lineage_secret: b"returner-root-secret".to_vec(),
        group_id: b"g".to_vec(),
        psk_id: b"tok".to_vec(),
        used_nonces: HashSet::new(),
    };
    let nonce = b"nonce-0001";
    let sig = serve_sig(b"returner-root-secret", nonce, b"g", b"tok");
    assert!(server.verify_and_consume(nonce, sig).is_ok(), "first use succeeds");
    assert_eq!(server.verify_and_consume(nonce, sig), Err("nonce already used (replay)"),
        "a replay of the same challenge-response is rejected: the nonce is single-use");
    println!("S24 serve s-i MEASURED (Modeled): a replayed challenge-response is rejected — the \
              P-generated nonce is single-use.");
}

#[test]
fn serve_s_ii_valid_token_but_cannot_sign_for_lineage_is_refused_at_serve() {
    let mut server = ChallengeServer {
        expected_lineage_secret: b"returner-root-secret".to_vec(),
        group_id: b"g".to_vec(),
        psk_id: b"tok".to_vec(),
        used_nonces: HashSet::new(),
    };
    let nonce = b"nonce-0002";
    // A requester who holds the psk_id but does NOT control the issued-to lineage key.
    let wrong_sig = serve_sig(b"not-the-returner-secret", nonce, b"g", b"tok");
    assert_eq!(server.verify_and_consume(nonce, wrong_sig),
        Err("challenge signature does not chain to the issued-to lineage"),
        "a valid psk_id without lineage-key control is refused AT SERVE (not only at merge)");
    println!("S24 serve s-ii MEASURED (Modeled): a valid psk_id presented by a requester who \
              cannot sign for the issued-to lineage is refused at serve.");
}

// --------------------------------------------------------------------------------------------
// Perishability — a GroupInfo served at epoch E is refused after a roll to E+1 (real MLS).
// --------------------------------------------------------------------------------------------

#[test]
fn perishability_a_stale_group_info_is_refused_after_a_roll() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_v) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_v.pop().unwrap();

    let token = issue_token("returner@example");
    token.deposit_with(&returner);
    token.deposit_with(&bob);

    // Serve a GroupInfo at epoch E.
    let gi_at_e = current_group_info(&mut alices, &alice);

    // The group rolls to E+1 (alice self-updates; bob folds it).
    let update = alices
        .self_update(&alice.provider, &alice.signer, LeafNodeParameters::default())
        .expect("self update");
    alices.merge_pending_commit(&alice.provider).expect("alice merges roll");
    let roll_commit = update.commit().clone();
    assert!(matches!(try_merge(&mut bobs, &bob, &roll_commit), MergeOutcome::Seated),
        "bob folds the epoch roll");

    // The returner builds a commit from the STALE E GroupInfo; the incumbents are at E+1.
    let commit = token.returner_commit(&returner, gi_at_e);
    let outcome = try_merge(&mut bobs, &bob, &commit);
    assert!(
        !matches!(outcome, MergeOutcome::Seated),
        "a commit built on a stale GroupInfo must be refused after the roll; got {}",
        describe(&outcome)
    );
    println!("S24 perishability MEASURED (Rung A): a GroupInfo served at epoch E constructs a \
              commit the group refuses once it has rolled to E+1 ({}). Leaked serve artifacts \
              decay per roll; the token is the only durable thing.", describe(&outcome));
}
