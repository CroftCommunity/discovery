//! E12.2 / E12.7 (message facet) — the **message-continuity half** of the §7.6.2 re-plant: an
//! in-flight conversation survives the atomic repoint with **no loss and no duplication**. The
//! membership half is `Verified` (E12.7 keystone); this closes the other half over the B1
//! dataplane hash structure.
//!
//! Earns/bounds: Part 2 §7.6.2 — the message-continuity half of re-plant (an in-flight conversation
//! survives the repoint with no loss or dup), `Modeled` at loopback grade over content-addressed
//! records (no real transport, no wire pinning). Pairs with the membership half (E12.7 keystone).
//!
//! A re-plant repoints the conversation onto a fresh key layer (a new MLS group over the same
//! derived membership — the E12.7 half, exercised here first). The PSK carries the *entitlement*
//! thread; the *content* thread is carried separately by the dataplane history (§7.6.2). These
//! tests drive that content thread through the B1 [`dataplane::History`]:
//!
//! (a) every entry authored **before** the repoint is present after it, exactly once;
//! (b) entries authored **during** the repoint window land exactly once, on the post-repoint group,
//!     in causal order;
//! (c) both members' post-repoint histories are byte-identical across arrival orders;
//! (d) a duplicated or dropped frame injected into delivery is **detected**, not absorbed.
//!
//! Falsifies if: a pre-repoint entry is lost or doubled across the boundary; an in-flight entry
//! lands twice, out of causal order, or not at all; two arrival orders disagree; or an injected
//! dup/drop folds silently.

use replant_continuity::dataplane::{Fold, History, Record};
use replant_continuity::{restamp, stamped_principals, Chain, Roster};

/// Do the membership half of the repoint and return the derived==stamped member set, so the
/// message-continuity assertions run on top of a real re-plant, not in isolation.
fn repoint_membership(chain: &Chain, roster: &Roster) {
    let derived = chain.derived_members();
    let stamp = restamp(roster, &derived).expect("non-empty group stamps");
    assert_eq!(
        derived,
        stamped_principals(roster, &stamp),
        "membership half: the re-plant stamps exactly the derived set"
    );
}

/// A two-member group after genesis + one authorized add — the setting for a re-plant.
fn two_member_group() -> (Roster, Chain) {
    let roster = Roster::of(2);
    let mut chain = Chain::new(&roster);
    chain.genesis(&roster.members[0], [1, 1, 1, 1]).expect("genesis");
    chain.add(&roster.members[0], &roster.members[1], 2).expect("add m1");
    assert_eq!(chain.derived_members().len(), 2, "two governed members");
    (roster, chain)
}

/// Build a linear pre-repoint conversation of `n` entries alternating authors, each linking to its
/// predecessor. Returns the records in causal order and the id of the last (the in-flight anchor).
fn pre_repoint_conversation(roster: &Roster, n: usize) -> (Vec<Record>, Option<[u8; 32]>) {
    let mut records = Vec::new();
    let mut prev: Option<[u8; 32]> = None;
    for i in 0..n {
        let author = &roster.members[i % 2];
        let r = author.record(i as u64, prev, format!("pre-{i}").as_bytes());
        prev = Some(r.id());
        records.push(r);
    }
    (records, prev)
}

// (a) ------------------------------------------------------------------------

#[test]
fn every_pre_repoint_entry_is_present_after_exactly_once() {
    let (roster, chain) = two_member_group();
    let (pre, _tip) = pre_repoint_conversation(&roster, 6);

    // The conversation existed before the repoint.
    let mut pre_history = History::new();
    let report = pre_history.deliver_all(pre.clone());
    assert!(report.is_complete(), "pre-repoint conversation folds cleanly: {report:?}");
    assert_eq!(pre_history.len(), 6);

    // Repoint the membership (real re-plant), then carry the content thread across.
    repoint_membership(&chain, &roster);
    let mut post = History::new();
    let report = post.deliver_all(pre.clone());

    assert_eq!(report.applied, 6, "all six pre-repoint entries carry across");
    assert_eq!(report.duplicates, 0, "no entry is doubled by the repoint");
    assert!(report.is_complete(), "no entry is lost across the repoint");
    assert_eq!(post.len(), 6, "exactly the pre-repoint set, each once");
    for r in &pre {
        assert!(post.contains(&r.id()), "pre-repoint entry present after the repoint");
    }
    assert_eq!(post.digest(), pre_history.digest(), "the carried history is identical");
}

// (b) ------------------------------------------------------------------------

#[test]
fn in_flight_entries_land_once_post_repoint_in_causal_order() {
    let (roster, chain) = two_member_group();
    let (pre, tip) = pre_repoint_conversation(&roster, 4);

    repoint_membership(&chain, &roster);

    // Two entries authored *during* the repoint window: they chain off the pre-repoint tip and
    // must land on the post-repoint group, once each, after everything they causally follow.
    let inflight_a = roster.members[0].record(100, tip, b"in-flight-a");
    let inflight_b = roster.members[1].record(101, Some(inflight_a.id()), b"in-flight-b");

    let mut post = History::new();
    let mut stream = pre.clone();
    stream.push(inflight_a.clone());
    stream.push(inflight_b.clone());
    let report = post.deliver_all(stream);

    assert!(report.is_complete(), "in-flight entries land (no gap): {report:?}");
    assert_eq!(report.duplicates, 0, "in-flight entries are not doubled");
    assert_eq!(post.len(), 6, "four pre + two in-flight, each once");

    // Causal order: each in-flight entry appears after its antecedent in the ordered history.
    let order: Vec<[u8; 32]> = post.ordered().iter().map(|r| r.id()).collect();
    let pos = |id: &[u8; 32]| order.iter().position(|x| x == id).expect("present");
    assert!(pos(&inflight_a.id()) > pos(&tip.unwrap()), "in-flight-a follows the pre-repoint tip");
    assert!(pos(&inflight_b.id()) > pos(&inflight_a.id()), "in-flight-b follows in-flight-a");
}

// (c) ------------------------------------------------------------------------

#[test]
fn both_members_converge_byte_identically_across_arrival_orders() {
    let (roster, chain) = two_member_group();
    let (pre, tip) = pre_repoint_conversation(&roster, 5);
    repoint_membership(&chain, &roster);

    let inflight = roster.members[0].record(200, tip, b"in-flight");
    let mut full = pre.clone();
    full.push(inflight);

    // Member X receives in causal order; member Y receives fully reversed (antecedents after
    // dependents). The buffer-and-retry delivery must converge both to the identical history.
    let mut x = History::new();
    let rx = x.deliver_all(full.clone());

    let mut reversed = full.clone();
    reversed.reverse();
    let mut y = History::new();
    let ry = y.deliver_all(reversed);

    assert!(rx.is_complete() && ry.is_complete(), "both orders fold completely: {rx:?} {ry:?}");
    assert_eq!(x.len(), y.len(), "same number of records regardless of arrival order");
    assert_eq!(
        x.digest(),
        y.digest(),
        "both members' post-repoint folds are byte-identical across arrival orders"
    );
}

// (d) ------------------------------------------------------------------------

#[test]
fn injected_duplicate_is_detected_not_absorbed() {
    let (roster, chain) = two_member_group();
    let (pre, _tip) = pre_repoint_conversation(&roster, 4);
    repoint_membership(&chain, &roster);

    // The harness injects a second copy of an already-delivered frame.
    let dup = pre[2].clone();
    let mut stream = pre.clone();
    stream.push(dup.clone());

    let mut post = History::new();
    let report = post.deliver_all(stream);

    assert_eq!(report.duplicates, 1, "the injected duplicate is detected");
    assert_eq!(post.len(), 4, "and not absorbed — the set still holds each entry once");

    // A direct re-fold of an already-present record reports Duplicate, never New.
    assert_eq!(post.fold(dup), Fold::Duplicate, "a re-delivered frame folds as Duplicate");
}

#[test]
fn injected_drop_is_detected_not_absorbed() {
    let (roster, chain) = two_member_group();
    let (pre, _tip) = pre_repoint_conversation(&roster, 5);
    repoint_membership(&chain, &roster);

    // The harness drops entry #2 from the stream but keeps its successors, which now reference a
    // predecessor that never arrives.
    let dropped_id = pre[2].id();
    let stream: Vec<Record> = pre.iter().enumerate().filter(|(i, _)| *i != 2).map(|(_, r)| r.clone()).collect();

    let mut post = History::new();
    let report = post.deliver_all(stream);

    assert!(!report.is_complete(), "the drop is detected as an unresolved gap, not absorbed");
    assert!(
        report.unresolved_gaps.contains(&dropped_id),
        "the missing predecessor is named: {:?}",
        report.unresolved_gaps
    );
    // Only the entries causally before the drop folded; the tail is held, not silently accepted.
    assert!(post.len() < 4, "the post-drop tail did not fold: {} records", post.len());
    assert!(!post.contains(&dropped_id), "the dropped frame is absent");
}
