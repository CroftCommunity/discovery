//! Phase 2.5 — adversarial backfill provenance (deepens I8).
//!
//! A valid per-message signature is necessary but not sufficient. These probe
//! the two ways a well-signed branch could still be illegitimate: an author who
//! never held standing on the lineage, and a tampered (gapped/reordered)
//! message sequence. Run: `cargo test -p lineage-history --test backfill_adversarial`.

use std::collections::BTreeMap;

use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BackfillError, BranchHistory, HistoryStore, Message};

fn did(s: &str) -> Did {
    Did::new(s)
}
fn g(seed: &[u8]) -> GenesisId {
    GenesisId::from_bytes(seed)
}
fn verifier(dir: &BTreeMap<Did, VerifyingIdentity>) -> impl Fn(&Did, &[u8], &Sig) -> bool + '_ {
    move |d: &Did, msg: &[u8], sig: &Sig| dir.get(d).is_some_and(|v| v.verify(msg, sig))
}

/// World: branch `gb` shares a root with `mine`; alice held standing on it,
/// mallory never did (though her key is known/verifiable).
fn world() -> (
    SigningIdentity,
    SigningIdentity,
    BTreeMap<Did, VerifyingIdentity>,
    Lineage,
    GenesisId,
    GenesisId,
) {
    let alice = SigningIdentity::from_seed(did("alice"), 1);
    let mallory = SigningIdentity::from_seed(did("mallory"), 1);
    let mut dir = BTreeMap::new();
    dir.insert(did("alice"), alice.verifying());
    dir.insert(did("mallory"), mallory.verifying());

    let (gb, mine) = (g(b"branch"), g(b"mine"));
    let mut lineage = Lineage::new();
    lineage.add_root(gb, [did("alice")]);
    lineage.fork(gb, mine, [did("alice")]);
    (alice, mallory, dir, lineage, gb, mine)
}

/// A2.3a — a branch whose messages are validly signed by someone who never held
/// standing on the lineage is rejected (signature alone is not standing).
#[test]
fn a2_3a_unauthorized_author_rejected() {
    let (_alice, mallory, dir, lineage, gb, mine) = world();

    let mut donor = BranchHistory::new(gb);
    donor.append(&mallory, b"i was never here"); // valid signature, but no standing

    let mut store = HistoryStore::new();
    assert_eq!(
        store.backfill_import(&donor, mine, &lineage, verifier(&dir)),
        Err(BackfillError::UnauthorizedAuthor {
            author: did("mallory")
        })
    );
}

/// A2.3b — a tampered (gapped/reordered) sequence is rejected even when every
/// individual signature is valid.
#[test]
fn a2_3b_noncontiguous_sequence_rejected() {
    let (alice, _mallory, dir, lineage, gb, mine) = world();

    // A message validly signed for seq 5, placed at index 0 -> a gap.
    let mut gapped = BranchHistory::new(gb);
    let bytes = Message::signing_bytes(gb, 5, &did("alice"), b"from the future");
    gapped.push_raw(Message {
        author: did("alice"),
        seq: 5,
        branch: gb,
        payload: b"from the future".to_vec(),
        sig: alice.sign(&bytes),
    });
    let mut store = HistoryStore::new();
    assert_eq!(
        store.backfill_import(&gapped, mine, &lineage, verifier(&dir)),
        Err(BackfillError::NonContiguous { index: 0, seq: 5 })
    );

    // Two correctly-signed messages placed out of order (seq 1 then seq 0).
    let mut reordered = BranchHistory::new(gb);
    for seq in [1u64, 0] {
        let p = format!("msg{seq}");
        let b = Message::signing_bytes(gb, seq, &did("alice"), p.as_bytes());
        reordered.push_raw(Message {
            author: did("alice"),
            seq,
            branch: gb,
            payload: p.into_bytes(),
            sig: alice.sign(&b),
        });
    }
    let mut store2 = HistoryStore::new();
    assert!(matches!(
        store2.backfill_import(&reordered, mine, &lineage, verifier(&dir)),
        Err(BackfillError::NonContiguous { index: 0, seq: 1 })
    ));
}

/// A2.3c — a well-formed, entitled, authorized branch still imports cleanly
/// (the new checks don't reject legitimate history).
#[test]
fn a2_3c_legitimate_branch_still_imports() {
    let (alice, _mallory, dir, lineage, gb, mine) = world();

    let mut donor = BranchHistory::new(gb);
    donor.append(&alice, b"one");
    donor.append(&alice, b"two");

    let mut store = HistoryStore::new();
    assert!(store
        .backfill_import(&donor, mine, &lineage, verifier(&dir))
        .is_ok());
    assert_eq!(store.branch(gb).unwrap().len(), 2);
}
