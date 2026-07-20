//! P3 — the interval fold.
//!
//! Acceptance (RUN-AP-01 §2 P3):
//! - T-AP3.1 Follow opens an interval; is_currently_following=true.
//! - T-AP3.2 Undo Follow closes the interval it names; is_currently_following=false.
//! - T-AP3.3 Re-Follow opens a SECOND interval; both are present in
//!   causal-position order.
//! - T-AP3.4 Covert-clock: shuffled delivery order → identical roster.
//! - T-AP3.5 Undo without matching open Follow = no-op.

use ap_ambassador::fold::*;
use ap_ambassador::records::*;
use ap_ambassador::types::*;

fn mk_receipt(
    kind: ActivityKind,
    actor: &str,
    object: &str,
    activity_id: &str,
    undoes: Option<ReceiptId>,
    salt_byte: u8,
) -> ReceiptRecord {
    // A body-hash + commitment that differ per receipt so ReceiptIds differ.
    let body_hash = [salt_byte; 32];
    let salt = Salt([salt_byte.wrapping_add(1); 32]);
    let commitment = commitment_helper(&salt, &body_hash);
    ReceiptRecord {
        kind,
        actor: ActorId::new(actor),
        object: object.to_string(),
        activity_id: activity_id.to_string(),
        undoes,
        state: ReceiptState::EvidenceComplete,
        commitment,
        body_hash,
        attestation_marker: ReceiptRecord::GATEWAY_MARKER.into(),
    }
}

fn commitment_helper(salt: &Salt, body_hash: &[u8; 32]) -> [u8; 32] {
    ap_ambassador::records::commitment(salt, body_hash)
}

// T-AP3.1

#[test]
fn t_ap3_1_follow_opens_interval() {
    let f = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "1",
        None,
        1,
    );
    let roster = fold_roster([&f]);
    let a = ActorId::new("https://alice.example/actor");
    assert!(roster.is_currently_following(&a, "https://example.social/bob"));
    assert_eq!(roster.open_count(&a, "https://example.social/bob"), 1);
    let iv = roster.intervals(&a, "https://example.social/bob");
    assert_eq!(iv.len(), 1);
    assert!(iv[0].close.is_none());
}

// T-AP3.2

#[test]
fn t_ap3_2_undo_follow_closes_interval() {
    let f = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "1",
        None,
        2,
    );
    let f_id = f.receipt_id();
    let u = mk_receipt(
        ActivityKind::UndoFollow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "2",
        Some(f_id),
        3,
    );
    let roster = fold_roster([&f, &u]);
    let a = ActorId::new("https://alice.example/actor");
    assert!(!roster.is_currently_following(&a, "https://example.social/bob"));
    let iv = roster.intervals(&a, "https://example.social/bob");
    assert_eq!(iv.len(), 1);
    assert_eq!(iv[0].open, f_id);
    assert!(iv[0].close.is_some());
}

// T-AP3.3

#[test]
fn t_ap3_3_re_follow_opens_second_interval() {
    let f1 = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "1",
        None,
        4,
    );
    let f1_id = f1.receipt_id();
    let u = mk_receipt(
        ActivityKind::UndoFollow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "2",
        Some(f1_id),
        5,
    );
    let f2 = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "3",
        None,
        6,
    );
    let roster = fold_roster([&f1, &u, &f2]);
    let a = ActorId::new("https://alice.example/actor");
    assert!(roster.is_currently_following(&a, "https://example.social/bob"));
    let iv = roster.intervals(&a, "https://example.social/bob");
    assert_eq!(iv.len(), 2, "two distinct intervals must exist");
    // Exactly one open, one closed.
    assert_eq!(iv.iter().filter(|i| i.close.is_none()).count(), 1);
    assert_eq!(iv.iter().filter(|i| i.close.is_some()).count(), 1);
}

// T-AP3.4 — the covert-clock red: shuffled arrival = identical roster.

#[test]
fn t_ap3_4_covert_clock_order_independent() {
    let f1 = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "1",
        None,
        10,
    );
    let f1_id = f1.receipt_id();
    let u1 = mk_receipt(
        ActivityKind::UndoFollow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "2",
        Some(f1_id),
        11,
    );
    let f2 = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "3",
        None,
        12,
    );
    let f3 = mk_receipt(
        ActivityKind::Follow,
        "https://alice.example/actor",
        "https://example.social/carol",
        "4",
        None,
        13,
    );

    let arrivals = vec![
        vec![&f1, &u1, &f2, &f3],
        vec![&f3, &f2, &u1, &f1],
        vec![&u1, &f2, &f3, &f1],
        vec![&f2, &f1, &f3, &u1],
    ];
    let base = fold_roster(arrivals[0].iter().copied());
    for (i, order) in arrivals.iter().enumerate().skip(1) {
        let got = fold_roster(order.iter().copied());
        assert_eq!(got, base, "arrival order {i} must produce identical roster");
    }
}

// T-AP3.5 — Undo pointing at an unknown Follow is a no-op.

#[test]
fn t_ap3_5_undo_no_match_is_noop() {
    let phantom_id = ReceiptId([0xaa; 32]);
    let u = mk_receipt(
        ActivityKind::UndoFollow,
        "https://alice.example/actor",
        "https://example.social/bob",
        "99",
        Some(phantom_id),
        20,
    );
    let roster = fold_roster([&u]);
    let a = ActorId::new("https://alice.example/actor");
    // No pair recorded because no Follow ever opened.
    assert!(roster.intervals(&a, "https://example.social/bob").is_empty());
    assert!(!roster.is_currently_following(&a, "https://example.social/bob"));
}
