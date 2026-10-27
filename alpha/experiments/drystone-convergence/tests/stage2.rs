// Stage 2: gap-detection simulation.
//
// Tests the seeded deterministic multi-node simulation with controlled
// delivery. Exercises two gap cases from the brief:
//
//   Referenced-gap: a node holds G whose predecessor F is absent. The fold
//   must detect the gap (GapError) rather than fold onward as if complete.
//
//   Unreferenced-tail gap: a node's set looks complete (no absent predecessors)
//   but is missing a new-head fact F that nothing points to. Reference-based
//   detection cannot catch this. Documented as a known limit, not a pass.
//
//   Convergence after fill: delivering the missing fact makes the gapped node
//   converge to an identical fingerprint as a node that always had the full set.

use drystone_convergence::fold::GapError;
use drystone_convergence::simulation::Node;
use drystone_convergence::types::{Fact, FactPayload};

/// Referenced-gap detection (MUST PASS per brief).
///
/// Setup:
///   F1 = Add(m),      preds={}
///   F2 = Remove(m),   preds={F1}       ← causally after F1; changes member slot
///   G  = GrantRole(m, r), preds={F2}   ← causally after F2
///
/// Full node: has F1, F2, G.
/// Gapped node: has F1, G (missing F2 — a predecessor of G).
///
/// The gapped node must detect the gap because G references F2 which is absent.
/// After delivering F2, it must converge to the full node's fingerprint.
#[test]
fn stage2_referenced_gap_detected() {
    let f1 = Fact::new("n0".into(), 1, vec![],
                 FactPayload::AddMember("alice".into()));
    let f2 = Fact::new("n0".into(), 2, vec![f1.id],
                 FactPayload::RemoveMember("alice".into()));
    let g  = Fact::new("n0".into(), 3, vec![f2.id],
                 FactPayload::GrantRole("alice".into(), "admin".into()));

    // Full node: holds F1, F2, G.
    let mut full_node = Node::new("full");
    full_node.deliver(f1.clone());
    full_node.deliver(f2.clone());
    full_node.deliver(g.clone());

    let full_state = full_node.fold().expect("Full node should fold without gaps");

    // Gapped node: holds F1 and G but NOT F2.
    let mut gapped_node = Node::new("gapped");
    gapped_node.deliver(f1.clone());
    gapped_node.deliver(g.clone());
    // Missing F2, which is a predecessor of G.

    let gap_result = gapped_node.fold();
    assert!(
        matches!(gap_result, Err(GapError { .. })),
        "Gapped node must detect the missing predecessor (R3), not fold as if complete"
    );
    if let Err(ref e) = gap_result {
        assert!(e.missing.contains(&f2.id),
            "GapError must name F2 as the missing fact");
    }

    // Convergence after fill: deliver F2 to the gapped node.
    gapped_node.deliver(f2.clone());
    let filled_state = gapped_node.fold().expect("Node should fold cleanly after fill");
    assert_eq!(
        full_state.fingerprint(),
        filled_state.fingerprint(),
        "After fill, gapped node must converge to identical fingerprint as full node"
    );
}

/// Convergence simulation with multiple nodes and controlled delivery order.
///
/// Three nodes receive facts in different orders; after pairwise reconciliation
/// all must converge to the same fingerprint.
#[test]
fn stage2_convergence_after_fill_multiple_nodes() {
    // Build a non-trivial causal chain.
    let f1 = Fact::new("n0".into(), 1, vec![],
                 FactPayload::AddMember("alice".into()));
    let f2 = Fact::new("n1".into(), 1, vec![],
                 FactPayload::AddMember("bob".into()));
    let f3 = Fact::new("n0".into(), 2, vec![f1.id],
                 FactPayload::GrantRole("alice".into(), "admin".into()));
    let f4 = Fact::new("n1".into(), 2, vec![f2.id, f3.id],
                 FactPayload::SetThreshold("admin".into(), 2, 3));

    // Node A: gets all facts immediately.
    let mut node_a = Node::new("A");
    for f in [&f1, &f2, &f3, &f4] { node_a.deliver(f.clone()); }

    // Node B: starts with only f1 and f2 (missing f3 and f4).
    let mut node_b = Node::new("B");
    node_b.deliver(f1.clone());
    node_b.deliver(f2.clone());

    // Node C: starts with f4 but not its predecessors — has a referenced gap.
    let mut node_c = Node::new("C");
    node_c.deliver(f4.clone());

    assert!(node_b.fold().is_ok(), "Node B has no gaps (f3, f4 simply absent)");
    assert!(node_c.fold().is_err(), "Node C must detect gap: f4 references f2, f3 which are absent");

    // Reconcile B ← A, C ← A.
    node_b.reconcile_from(&node_a);
    node_c.reconcile_from(&node_a);

    let fp_a = node_a.fold().unwrap().fingerprint();
    let fp_b = node_b.fold().unwrap().fingerprint();
    let fp_c = node_c.fold().unwrap().fingerprint();
    assert_eq!(fp_a, fp_b, "Node B must converge to A after reconciliation");
    assert_eq!(fp_a, fp_c, "Node C must converge to A after fill");
}

/// Unreferenced-tail gap: reference-based detection cannot catch a missing
/// new-head fact (nothing points to it). This is the documented limit.
///
/// Node holds a complete-looking set: all predecessors of its facts are present.
/// A newer fact F_new exists in the network but nothing the node holds points to it.
/// The fold succeeds — no gap is detected — but the result is stale.
///
/// This is the expected behaviour per the brief:
///   "Assert that reference-based detection alone does NOT catch this."
/// Record it in RESULTS.md as the case requiring completeness-ahead corroboration.
#[test]
fn stage2_unreferenced_tail_gap_not_detectable() {
    let f1 = Fact::new("n0".into(), 1, vec![],
                 FactPayload::AddMember("alice".into()));
    let f2 = Fact::new("n0".into(), 2, vec![f1.id],
                 FactPayload::GrantRole("alice".into(), "admin".into()));

    // F_new: a causally-later fact that changes a slot. Nothing the node holds points to it.
    let f_new = Fact::new("n0".into(), 3, vec![f2.id],
                    FactPayload::RemoveMember("alice".into()));

    // Node missing f_new — an unreferenced tail gap.
    let mut node = Node::new("stale");
    node.deliver(f1.clone());
    node.deliver(f2.clone());
    // f_new not delivered; nothing the node holds references it.

    // Reference-based detection cannot catch this: no predecessor is absent.
    let stale_result = node.fold();
    assert!(
        stale_result.is_ok(),
        "DOCUMENTED LIMIT: reference-based gap detection cannot catch an \
         unreferenced tail. The node folds successfully but produces a stale result."
    );

    // The stale state shows alice as a member (the remove is missing).
    let stale_state = stale_result.unwrap();
    assert!(stale_state.members.contains("alice"),
        "Stale node sees alice as member (the remove is hidden in the tail gap)");

    // Full node with f_new sees alice as NOT a member.
    let mut full_node = Node::new("full");
    full_node.deliver(f1.clone());
    full_node.deliver(f2.clone());
    full_node.deliver(f_new.clone());
    let full_state = full_node.fold().unwrap();
    assert!(!full_state.members.contains("alice"),
        "Full node sees alice removed");

    // The two states differ — the tail gap is silent.
    assert_ne!(
        stale_state.fingerprint(),
        full_state.fingerprint(),
        "DOCUMENTED LIMIT: the stale and full states differ, but the node cannot \
         detect this discrepancy through predecessor references alone. \
         This requires completeness-ahead corroboration or a dataplane checkpoint."
    );
}
