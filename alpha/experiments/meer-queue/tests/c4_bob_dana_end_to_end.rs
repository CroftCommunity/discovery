//! **C4 — the Bob/Dana stale-admission end-to-end (E112).**
//!
//! The review's Bob/Dana story run whole, with the **admission fact** as the detection trigger.
//! A returner with her *own* valid token + lineage key (both legs genuine — only standing is
//! stale) is served and merged by a stale group; the merge mints the admission fact (a span opens);
//! sync arrives; the projection reads standing OVER spans (excluded); the §11.8 re-fire closes the
//! span forward (re-key excludes). No hard-stop fires — the routine case stays routine.
//!
//!   - **Arm 1 (same-branch, whole group stale).** Real MLS seat + admission fact + counted
//!     exposure window + real re-key exclusion + order-independent projection = Excluded, no
//!     CONTESTED.
//!   - **Arm 1a (arrival-order permutation).** admission-fact-then-ban and ban-then-admission-fact
//!     project **byte-identically** (span recorded, subject excluded, no contested). The arm that
//!     pins the comparator placement; its mutation target is an impl that lets the admission fact
//!     compete on the membership slot.
//!   - **Arm 2 (diverged branch).** The admission fact's event — not queue-name divergence — is
//!     what names the fork.
//!   - **Arm 3 (genuine-contradiction control).** A readmission *quorum* racing the ban →
//!     CONTESTED; an admission fact racing the same ban → never. Both sides, one arm.
//!
//! Fidelity: **Rung A** for the MLS half (real openmls 0.8.1 — the seat, the exposure reads, the
//! re-key exclusion); **Modeled** for the governance projection (the admission-fact / standing
//! model in `src/admission.rs`).

mod common;

use common::*;
use meer_queue::admission::{
    content_address, mint_or_refuse, project, AcceptanceChain, Event, IssuanceLedger, Standing,
    StandingDecision, StandingEvent,
};
use meer_queue::mls::{open, seal};
use mls_replant::Persona;
use openmls::prelude::*;

const LIN_RETURNER: &[u8] = b"lineage/returner";

fn leaf_of(group: &MlsGroup, who: &Persona) -> LeafNodeIndex {
    let want = who.cwk.credential.serialized_content().to_vec();
    group
        .members()
        .find(|m| m.credential.serialized_content() == want)
        .expect("member present")
        .index
}

/// **Arm 1 — same-branch, whole group stale: seat, count exposure, re-fire, exclude.**
#[test]
fn arm1_stale_admission_is_chain_visible_counted_and_repaired_without_hard_stop() {
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_v) = seat_group(&alice, &[&bob]);
    let mut bobs = bobs_v.pop().unwrap();

    // Governance: the returner's OWN token, issued to her lineage. Both legs genuine.
    let token = issue_token("returner@example");
    token.deposit_with(&returner);
    token.deposit_with(&alice);
    token.deposit_with(&bob);
    let mut ledger = IssuanceLedger::new();
    ledger.issue(token.token_id.clone(), LIN_RETURNER.to_vec());

    // The whole group is stale (nobody has folded the ban yet). Liberal posture: they serve+merge.
    let gi = current_group_info(&mut alices, &alice);
    let (commit, mut returners) = token.returner_join(&returner, gi, &token.issuance_attestation());
    let wire = commit_wire(&commit);

    // The merge mints the admission fact — the collision is CHAIN-VISIBLE from this point, before
    // any read failure.
    let mut chain = AcceptanceChain::new();
    let fact = mint_or_refuse(&ledger, &mut chain, b"lineage/bob", 5, &token.token_id, LIN_RETURNER, &wire)
        .expect("stale incumbents can mint: they have not folded the ban");
    assert_eq!(fact.event, content_address(&wire), "the admission event is chain-visible");

    // Both stale incumbents merge (whole group stale).
    assert!(matches!(try_merge(&mut alices, &alice, &commit), MergeOutcome::Seated));
    assert!(matches!(try_merge(&mut bobs, &bob, &commit), MergeOutcome::Seated));

    // Exposure window: the returner reads what is sealed during her span. Count it (the S25
    // propagation number, from the repair side).
    let mut exposure = 0u32;
    for i in 0..3 {
        let ct = seal(&mut alices, &alice, format!("during-span-{i}").as_bytes()).expect("seal");
        if open(&mut returners, &returner, &ct).is_ok() {
            exposure += 1;
        }
        // bob folds the same message so the group stays converged.
        let _ = open(&mut bobs, &bob, &ct);
    }
    assert_eq!(exposure, 3, "the returner could read every message sealed during her open span");

    // Sync arrives (the ban folds). The projection reads standing OVER the span → Excluded, span
    // recorded, and NO hard-stop (the admission fact is not a standing decision).
    let events = vec![
        Event::Admission(fact.clone()),
        Event::Standing(StandingEvent { lineage: LIN_RETURNER.to_vec(), decision: StandingDecision::Ban }),
    ];
    let proj = project(&events);
    assert_eq!(proj.standing_of(LIN_RETURNER), Standing::Excluded, "standing read over the span");
    assert!(proj.span_recorded(LIN_RETURNER), "the window was real, the record says so");
    assert!(!proj.any_contested(), "no hard-stop: the routine case stayed routine");

    // §11.8 re-fire: the corrective removal closes the span forward. Real MLS re-key excludes.
    let idx = leaf_of(&alices, &returner);
    let (rm, _, _) = alices
        .remove_members(&alice.provider, &alice.signer, &[idx])
        .expect("remove");
    alices.merge_pending_commit(&alice.provider).expect("alice merges removal");
    assert!(matches!(try_merge(&mut bobs, &bob, &rm), MergeOutcome::Seated));

    // Post-rekey: the returner can no longer read the new epoch (AEAD-level exclusion).
    let after = seal(&mut alices, &alice, b"after the re-fire").expect("seal");
    assert!(open(&mut returners, &returner, &after).is_err(),
        "the corrective removal re-keyed her out: exposure is everything in the closed span, no more");

    println!(
        "C4 arm 1 MEASURED (Rung A MLS / Modeled projection): a stale whole-group admission was \
         CHAIN-VISIBLE at merge (admission fact event), the exposure window was COUNTED ({exposure} \
         messages), the projection read standing over the span → Excluded with NO hard-stop, and \
         the §11.8 re-fire re-keyed the returner out (AEAD). The window was real, the record says \
         so, nothing was retroactively unmade."
    );
}

/// **Arm 1a — arrival-order permutation projects byte-identically (pins the comparator placement).**
#[test]
fn arm1a_admission_fact_and_ban_are_order_independent_and_never_contest() {
    let fact = meer_queue::admission::AdmissionFact {
        event: content_address(b"some-commit"),
        merged_lineage: LIN_RETURNER.to_vec(),
        redeemed_token: b"tok".to_vec(),
        acceptor: b"lineage/bob".to_vec(),
        acceptor_frontier: 5,
    };
    let ban = StandingEvent { lineage: LIN_RETURNER.to_vec(), decision: StandingDecision::Ban };

    let fact_then_ban = project(&[Event::Admission(fact.clone()), Event::Standing(ban.clone())]);
    let ban_then_fact = project(&[Event::Standing(ban), Event::Admission(fact)]);

    assert_eq!(fact_then_ban.to_bytes(), ban_then_fact.to_bytes(),
        "both ingest orders project byte-identically");
    assert_eq!(fact_then_ban.standing_of(LIN_RETURNER), Standing::Excluded);
    assert!(fact_then_ban.span_recorded(LIN_RETURNER), "the span is recorded in both orders");
    assert!(!fact_then_ban.any_contested(),
        "an admission fact racing a ban is enactment vs decision — never a contradiction pair");

    println!(
        "C4 arm 1a MEASURED (Modeled): admission-fact-then-ban and ban-then-admission-fact project \
         BYTE-IDENTICALLY — span recorded, subject Excluded, never CONTESTED. The admission fact \
         does not compete on the standing slot. (Mutation target: an impl that treats the fact as a \
         standing decision would either contest or become order-dependent here.)"
    );
}

/// **Arm 2 — the admission fact, not queue-name divergence, names the fork.**
#[test]
fn arm2_diverged_branch_is_named_by_the_admission_fact() {
    // Stale Bob merges (mints the fact); synced Carol has folded the ban and refuses (her ledger
    // revoked the issuance), so she never mints. The branches differ by a CHAIN-READABLE fact.
    let alice = Persona::new("alice");
    let bob = Persona::new("bob");
    let returner = Persona::new("returner");
    let (mut alices, mut bobs_v) = seat_group(&alice, &[&bob]);
    let mut _bobs = bobs_v.pop().unwrap();

    let token = issue_token("returner@example");
    token.deposit_with(&returner);
    token.deposit_with(&bob);

    // Bob is stale: issuance present, unrevoked.
    let mut bob_ledger = IssuanceLedger::new();
    bob_ledger.issue(token.token_id.clone(), LIN_RETURNER.to_vec());
    // Carol is synced: she folded the ban → her issuance is revoked.
    let mut carol_ledger = IssuanceLedger::new();
    carol_ledger.issue(token.token_id.clone(), LIN_RETURNER.to_vec());
    carol_ledger.revoke(&token.token_id);

    let gi = current_group_info(&mut alices, &alice);
    let commit = token.returner_commit_with_aad(&returner, gi, &token.issuance_attestation());
    let wire = commit_wire(&commit);

    let mut bob_chain = AcceptanceChain::new();
    let bob_result = mint_or_refuse(&bob_ledger, &mut bob_chain, b"lineage/bob", 5, &token.token_id, LIN_RETURNER, &wire);
    let mut carol_chain = AcceptanceChain::new();
    let carol_result = mint_or_refuse(&carol_ledger, &mut carol_chain, b"lineage/carol", 9, &token.token_id, LIN_RETURNER, &wire);

    assert!(bob_result.is_ok(), "stale Bob mints and merges — his branch has the fact");
    assert!(carol_result.is_err(), "synced Carol refuses — her branch has no such fact");

    // The fork is named by the admission event, readable from Bob's chain by anyone who folds it —
    // not by a queue name only members who exchange traffic across the fork would ever observe.
    let event = content_address(&wire);
    assert_eq!(bob_chain.facts_for(&event).len(), 1, "the fork is named by the chain fact");
    assert!(carol_chain.is_empty(), "the other branch is chain-distinguishable by the fact's absence");

    println!(
        "C4 arm 2 MEASURED (Rung A commit / Modeled governance): the two branches differ by a \
         CHAIN-READABLE admission fact (event {}), not by a queue name only cross-fork traffic \
         would reveal (S18's silent case). The admission fact is what names the fork.",
        hex::encode(&event[..8])
    );
}

/// **Arm 3 — the boundary the typing must hold, from both sides.**
#[test]
fn arm3_quorum_contests_the_ban_but_an_admission_fact_never_does() {
    let lin = LIN_RETURNER.to_vec();

    // An admission FACT racing the ban: enactment vs decision → Excluded, never contested.
    let fact = meer_queue::admission::AdmissionFact {
        event: content_address(b"commit"),
        merged_lineage: lin.clone(),
        redeemed_token: b"tok".to_vec(),
        acceptor: b"lineage/bob".to_vec(),
        acceptor_frontier: 5,
    };
    let fact_vs_ban = project(&[
        Event::Admission(fact),
        Event::Standing(StandingEvent { lineage: lin.clone(), decision: StandingDecision::Ban }),
    ]);
    assert_eq!(fact_vs_ban.standing_of(&lin), Standing::Excluded);
    assert!(!fact_vs_ban.any_contested(), "an admission fact never contests the standing slot");

    // A readmission QUORUM racing the ban: two decisions on the slot → CONTESTED, order-independent.
    let quorum_vs_ban = project(&[
        Event::Standing(StandingEvent { lineage: lin.clone(), decision: StandingDecision::ReadmitQuorum }),
        Event::Standing(StandingEvent { lineage: lin.clone(), decision: StandingDecision::Ban }),
    ]);
    assert_eq!(quorum_vs_ban.standing_of(&lin), Standing::Contested);
    assert!(quorum_vs_ban.any_contested(), "two rival standing decisions hard-stop");
    // And it is order-independent (G1 pattern).
    let ban_vs_quorum = project(&[
        Event::Standing(StandingEvent { lineage: lin.clone(), decision: StandingDecision::Ban }),
        Event::Standing(StandingEvent { lineage: lin.clone(), decision: StandingDecision::ReadmitQuorum }),
    ]);
    assert_eq!(quorum_vs_ban.to_bytes(), ban_vs_quorum.to_bytes(), "contradiction is order-independent");

    println!(
        "C4 arm 3 MEASURED (Modeled): a readmission QUORUM racing the ban is two decisions on the \
         standing slot → CONTESTED (order-independent hard-stop), while an admission FACT racing \
         the same ban is enactment vs decision → Excluded, never contested. The routine/genuine \
         line is pinned from both sides. (CONTESTED's own pinning stays croft-chat's, per E108.)"
    );
}
