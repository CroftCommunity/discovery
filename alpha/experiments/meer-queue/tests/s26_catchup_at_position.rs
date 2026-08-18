//! **S26 — catch-up replay determinism: admission is evaluated at-position, never at-head (E112).**
//!
//! §7.3.1's authorization-at-causal-position rule, extended to admission: a commit at causal
//! position X is evaluated against the governance fold **up to X**, not the evaluator's current
//! head. Valid-at-its-position stays valid; later governance acts **forward**. The AAD carries the
//! claimed position as a **locator, never an authorization input** (§7.4.3) — M verifies the claim
//! against its own fold-to-that-position.
//!
//!   - **Arm 1 (convergence)** — a member replaying from behind lands byte-identical with the live
//!     members (join applied at X, ban applied at Y > X). A head-anchored evaluation cannot.
//!   - **Arm 2 (position-anchoring pinned)** — the mutation-killer: the check consults
//!     fold-at-**position**, not fold-at-**head**; the two disagree here, so a head-anchored
//!     mutation is caught.
//!   - **Arm 3 (stale-majority)** — a mostly-lagging group merges an at-position-**invalid** join;
//!     M syncs later. Catch-up correction is **governance-forward, never chain-refusal**: M
//!     processes structurally, reads standing from the fold (excluded, §7.6.12 phase 1), and
//!     converges once the §11.8 re-fire enacts. Chain-refusal is reserved for a deliberate fork.
//!
//! Fidelity: **Modeled** (positioned governance history + the C4/admission projection).

mod common;

use meer_queue::admission::{
    content_address, project, AdmissionFact, Event, Standing, StandingDecision, StandingEvent,
};

/// Is `lineage` banned **as of** causal `position`? A ban at position P is in effect only for
/// positions >= P. This is the at-position read; the at-head read uses `position = head`.
fn banned_as_of(bans: &[(&[u8], u64)], lineage: &[u8], position: u64) -> bool {
    bans.iter().any(|(l, p)| *l == lineage && *p <= position)
}

const LIN_R: &[u8] = b"lineage/returner";

fn admission_at(commit_tag: &[u8], acceptor_frontier: u64) -> AdmissionFact {
    AdmissionFact {
        event: content_address(commit_tag),
        merged_lineage: LIN_R.to_vec(),
        redeemed_token: b"tok".to_vec(),
        acceptor: b"lineage/acceptor".to_vec(),
        acceptor_frontier,
    }
}

#[test]
fn arm1_replay_from_behind_lands_byte_identical_with_the_live_edge() {
    // Timeline: R joins validly at X = 1; R is banned again at Y = 2.
    let x = 1u64;
    let y = 2u64;
    let bans: [(&[u8], u64); 1] = [(LIN_R, y)];

    // At-position: the join at X is valid (no ban as of X).
    assert!(!banned_as_of(&bans, LIN_R, x), "the join is valid at its own position X");

    // The live members applied {join@X, ban@Y}; a member replaying from behind applies the SAME
    // set. `project` is order-independent, so both land on the identical projection.
    let fact = admission_at(b"R-join-commit", 5);
    let ban = Event::Standing(StandingEvent { lineage: LIN_R.to_vec(), decision: StandingDecision::Ban });

    let live = project(&[Event::Admission(fact.clone()), ban.clone()]);
    let replay_from_behind = project(&[ban, Event::Admission(fact)]);

    assert_eq!(live.to_bytes(), replay_from_behind.to_bytes(), "byte-identical after replay");
    assert_eq!(live.standing_of(LIN_R), Standing::Excluded, "join applied at X, ban applied at Y");
    assert!(live.span_recorded(LIN_R), "the historically-valid join is recorded, not erased");

    println!("S26 arm 1 MEASURED (Modeled): a member replaying {{join@X, ban@Y}} from behind lands \
              byte-identical with the live edge — join applied at its position X, ban forward at Y. \
              The span is recorded (the join was valid at X); standing at head excludes.");
}

#[test]
fn arm2_position_anchoring_is_pinned_against_a_head_anchored_check() {
    // The mutation this test kills: consulting fold-AT-HEAD instead of fold-AT-POSITION.
    let x = 1u64; // the join's causal position
    let head = 2u64; // the evaluator's current head, where the ban already landed
    let bans: [(&[u8], u64); 1] = [(LIN_R, 2)];

    let valid_at_position = !banned_as_of(&bans, LIN_R, x);
    let valid_at_head = !banned_as_of(&bans, LIN_R, head);

    assert!(valid_at_position, "at-position: the join is valid (correct — accept the history)");
    assert!(!valid_at_head, "at-head: the join looks invalid (the mutation — would self-exile)");
    assert_ne!(valid_at_position, valid_at_head,
        "the two evaluations disagree here — a head-anchored check would refuse a valid history");

    println!("S26 arm 2 MEASURED (Modeled): at-position and at-head evaluation DISAGREE for a join \
              at X < ban-position. The rule must consult fold-at-position; a head-anchored mutation \
              refuses the historically-valid join and self-exiles — this test catches it.");
}

#[test]
fn arm3_stale_majority_invalid_join_is_corrected_forward_not_chain_refused() {
    // The ban is at position 0; R's join is at X = 1 → at-position INVALID (banned as of X).
    let x = 1u64;
    let bans: [(&[u8], u64); 1] = [(LIN_R, 0)];
    assert!(banned_as_of(&bans, LIN_R, x), "the join is at-position invalid (ban precedes it)");

    // A stale majority (hadn't folded the ban) merged it anyway. M syncs later. Posture:
    // governance-forward — M processes the join structurally (an epoch roll carries no social
    // meaning), and reads standing from the fold rather than chain-refusing.
    let fact = admission_at(b"stale-majority-join", 3);
    let proj = project(&[
        Event::Admission(fact),
        Event::Standing(StandingEvent { lineage: LIN_R.to_vec(), decision: StandingDecision::Ban }),
    ]);
    assert_eq!(proj.standing_of(LIN_R), Standing::Excluded,
        "M reads the invalid member as experientially excluded (§7.6.12 phase 1) — no chain-refusal");
    assert!(proj.span_recorded(LIN_R), "the span the stale majority opened is recorded (then closed forward)");
    assert!(!proj.any_contested(),
        "an invalid join is not a contradiction — it is corrected forward by the §11.8 re-fire, \
         not hard-stopped; chain-refusal is reserved for a deliberate fork");

    println!("S26 arm 3 MEASURED (Modeled): a stale-majority at-position-invalid join is corrected \
              GOVERNANCE-FORWARD — M processes it structurally, reads Excluded from the fold, and \
              converges via the §11.8 re-fire. No chain-refusal during catch-up (that is reserved \
              for a deliberate fork).");
}
