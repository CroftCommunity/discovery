//! B7 — checkpoint prune guard (RUN-HIST-01).
//!
//! Serves history-durability.md §L via HIST-ATPROTO-MATCHUP.md row 7:
//! deleting index records below a checkpoint is permitted only when the
//! checkpoint marker record is present and verifiable; prune-without-
//! checkpoint is rejected. **§L's checkpoint construction remains OPEN — the
//! guard tests the GATE, not the construction** (the commitment here is the
//! declared placeholder shape, src/checkpoint.rs). RED-able: the staged red
//! prunes unconditionally, checkpoint or no checkpoint.

use hist_atproto_spike::checkpoint::{
    fixture_commitment, CheckpointRecord, EnvelopeIndex, PruneError,
};
use hist_atproto_spike::envelope::fixture_chain;

fn loaded_index(label: &str, n: u64) -> (EnvelopeIndex, Vec<hist_atproto_spike::envelope::Envelope>) {
    let chain: Vec<_> = fixture_chain(label, n).into_iter().map(|(e, _)| e).collect();
    let mut idx = EnvelopeIndex::new();
    for e in &chain {
        idx.insert(e.clone());
    }
    (idx, chain)
}

fn checkpoint_for(chain: &[hist_atproto_spike::envelope::Envelope], below: u64) -> CheckpointRecord {
    let subspace = chain[0].subspace;
    let prefix_head = chain[below as usize - 1].entry_digest;
    CheckpointRecord {
        subspace,
        below_counter: below,
        prefix_head,
        commitment: fixture_commitment(&subspace, below, &prefix_head),
    }
}

#[test]
fn prune_without_checkpoint_is_rejected() {
    let (mut idx, chain) = loaded_index("b7-nocp", 10);
    let subspace = chain[0].subspace;
    match idx.prune_below(&subspace, 6, None) {
        Err(PruneError::NoCheckpoint) => {
            assert_eq!(idx.len(), 10, "nothing was deleted");
        }
        Ok(pruned) => panic!(
            "unsafe prune: {pruned} entries deleted below counter 6 with NO \
             checkpoint marker — §L forbids pruning below an anchor without \
             a corroborated checkpoint"
        ),
        Err(other) => panic!("wrong rejection shape: {other:?}"),
    }
}

#[test]
fn prune_with_verifiable_checkpoint_is_permitted_and_bounded() {
    let (mut idx, chain) = loaded_index("b7-ok", 10);
    let subspace = chain[0].subspace;
    let cp = checkpoint_for(&chain, 6);
    let pruned = idx
        .prune_below(&subspace, 6, Some(&cp))
        .expect("verifiable checkpoint gates the prune open");
    assert_eq!(pruned, 6, "exactly the covered prefix is pruned");
    assert_eq!(idx.len(), 4);
    assert!(idx.get(&subspace, 5).is_none(), "below the boundary: gone");
    assert!(idx.get(&subspace, 6).is_some(), "at the boundary: retained");
}

#[test]
fn checkpoint_must_cover_the_requested_prune() {
    let (mut idx, chain) = loaded_index("b7-cover", 10);
    let subspace = chain[0].subspace;

    // Checkpoint at 4 cannot authorize a prune below 6.
    let cp4 = checkpoint_for(&chain, 4);
    match idx.prune_below(&subspace, 6, Some(&cp4)) {
        Err(PruneError::BoundaryMismatch) => assert_eq!(idx.len(), 10),
        other => panic!("under-covering checkpoint accepted: {other:?}"),
    }

    // A checkpoint for another subspace authorizes nothing here.
    let other_chain: Vec<_> = fixture_chain("b7-other", 8).into_iter().map(|(e, _)| e).collect();
    let cp_other = checkpoint_for(&other_chain, 6);
    match idx.prune_below(&subspace, 6, Some(&cp_other)) {
        Err(PruneError::WrongSubspace) => assert_eq!(idx.len(), 10),
        other => panic!("cross-subspace checkpoint accepted: {other:?}"),
    }
}

#[test]
fn unverifiable_marker_gates_closed() {
    let (mut idx, chain) = loaded_index("b7-bad", 10);
    let subspace = chain[0].subspace;

    // Tampered commitment bytes.
    let mut cp = checkpoint_for(&chain, 6);
    cp.commitment[0] ^= 1;
    match idx.prune_below(&subspace, 6, Some(&cp)) {
        Err(PruneError::BadCommitment) => assert_eq!(idx.len(), 10),
        other => panic!("tampered commitment accepted: {other:?}"),
    }

    // Marker whose prefix_head does not match the store's boundary entry.
    let mut cp = checkpoint_for(&chain, 6);
    cp.prefix_head = chain[2].entry_digest; // wrong anchor
    cp.commitment = fixture_commitment(&cp.subspace, cp.below_counter, &cp.prefix_head);
    match idx.prune_below(&subspace, 6, Some(&cp)) {
        Err(PruneError::BoundaryMismatch) => assert_eq!(idx.len(), 10),
        other => panic!("wrong-anchor checkpoint accepted: {other:?}"),
    }
}
