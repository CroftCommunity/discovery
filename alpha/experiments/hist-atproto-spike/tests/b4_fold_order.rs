//! B4 — fold-from-repo-order rejection (RUN-HIST-01).
//!
//! The ordering rider (GROUPS.md v2 A.7; HIST-ATPROTO-MATCHUP.md §1) as a
//! test, serving history-durability.md §K (convergence ordering is the
//! predecessor chain; delivery artifacts never order): frames delivered in
//! commit/firehose order that differs from chain order MUST fold to the same
//! state as chain-order delivery. The structural half — a fold that consumed
//! repo order as ordering is impossible to compile — lives in
//! `src/delivery.rs` (private cursor, no accessor, no Ord; two compile_fail
//! doc-tests pin it). RED-able: the staged red is a deliberately
//! order-sensitive fold (appends in arrival order), whose state digest
//! depends on the delivery permutation.

use hist_atproto_spike::delivery::Deliverer;
use hist_atproto_spike::envelope::fixture_chain;
use hist_atproto_spike::fold::fold;

#[test]
fn delivery_permutations_fold_to_identical_state() {
    // Two subspaces' chains, interleaved several ways.
    let a = fixture_chain("b4-a", 8);
    let b = fixture_chain("b4-b", 6);
    let mut envs: Vec<_> = a.iter().chain(b.iter()).map(|(e, _)| e.clone()).collect();

    // Order 1: chain order (a then b, each genesis-first).
    let mut d1 = Deliverer::new();
    let chain_order = fold(envs.iter().cloned().map(|e| d1.deliver(e)).collect::<Vec<_>>());

    // Order 2: "commit order" — interleaved tail-first (a plausible
    // repo/firehose arrival), i.e. NOT chain order.
    envs.reverse();
    let mut d2 = Deliverer::new();
    let commit_order = fold(envs.iter().cloned().map(|e| d2.deliver(e)).collect::<Vec<_>>());

    // Order 3: a deterministic shuffle (stride pick — no RNG, no wall-clock).
    let mut strided = Vec::new();
    let n = envs.len();
    let mut i = 0;
    for _ in 0..n {
        i = (i + 5) % n; // 5 is coprime with 14 = 8 + 6
        strided.push(envs[i].clone());
    }
    let mut d3 = Deliverer::new();
    let stride_order = fold(strided.into_iter().map(|e| d3.deliver(e)).collect::<Vec<_>>());

    assert_eq!(
        chain_order.digest(),
        commit_order.digest(),
        "MUST-NOT violated: folding depended on repo/firehose delivery order"
    );
    assert_eq!(chain_order.digest(), stride_order.digest());
    assert_eq!(chain_order, commit_order);
    assert_eq!(chain_order, stride_order);
}

#[test]
fn folded_chains_are_in_predecessor_order_not_arrival_order() {
    let chain = fixture_chain("b4-order", 7);
    let subspace = chain[0].0.subspace;

    // Deliver strictly in reverse (worst-case arrival).
    let mut d = Deliverer::new();
    let state = fold(
        chain
            .iter()
            .rev()
            .map(|(e, _)| d.deliver(e.clone()))
            .collect::<Vec<_>>(),
    );

    let folded = state.chains().get(&subspace).expect("subspace folded");
    assert_eq!(
        folded.iter().map(|e| e.counter).collect::<Vec<_>>(),
        (0..7).collect::<Vec<_>>(),
        "the folded chain is in predecessor-chain order regardless of arrival"
    );
    for w in folded.windows(2) {
        assert_eq!(
            w[1].predecessor, w[0].entry_digest,
            "each link is the in-payload predecessor chain, never a cursor"
        );
    }
    assert_eq!(state.pending_count(), 0, "a complete set fully chains");
}

#[test]
fn missing_link_stays_pending_never_guessed_from_delivery_order() {
    // Withhold one mid-chain entry: successors must sit in pending, not be
    // spliced into the chain because they "arrived next".
    let chain = fixture_chain("b4-pending", 5);
    let subspace = chain[0].0.subspace;
    let mut d = Deliverer::new();
    let delivered: Vec<_> = chain
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != 2)
        .map(|(_, (e, _))| d.deliver(e.clone()))
        .collect();
    let state = fold(delivered);
    let folded = state.chains().get(&subspace).expect("prefix folded");
    assert_eq!(
        folded.iter().map(|e| e.counter).collect::<Vec<_>>(),
        vec![0, 1],
        "the chain stops at the break; delivery adjacency never bridges it"
    );
    assert_eq!(state.pending_count(), 2, "the successors wait as pending");
}
