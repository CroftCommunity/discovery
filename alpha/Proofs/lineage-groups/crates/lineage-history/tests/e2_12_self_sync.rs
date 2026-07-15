//! E2.12 — self-sync IS backfill. Two devices of one lineage with divergent
//! histories reconcile through the existing `backfill_import` path: no server,
//! no special-case code beyond shared-genesis, branches stay distinct and
//! navigable (no interleave). This is the data-model proof; the live-transport
//! form is T2g/T11. (The mechanism is also exercised by LOCAL_FIRST_HISTORY.)

use std::collections::BTreeMap;

use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BranchHistory, HistoryStore};

fn did(s: &str) -> Did {
    Did::new(s)
}

#[test]
fn e2_12_two_devices_of_one_lineage_self_sync_via_backfill() {
    // One lineage rooted at `root`; each device forks its own branch and is the
    // recorded member of that branch (so it has standing).
    let root = GenesisId::from_bytes(b"alice-lineage-root");
    let phone_branch = GenesisId::from_bytes(b"alice-phone-branch");
    let laptop_branch = GenesisId::from_bytes(b"alice-laptop-branch");

    let mut lineage = Lineage::new();
    lineage.add_root(root, [did("alice.phone"), did("alice.laptop")]);
    lineage.fork(root, phone_branch, [did("alice.phone")]);
    lineage.fork(root, laptop_branch, [did("alice.laptop")]);

    let phone_id = SigningIdentity::from_seed(did("alice.phone"), 1);
    let laptop_id = SigningIdentity::from_seed(did("alice.laptop"), 1);

    // Each device wrote independently while apart (divergent histories).
    let mut phone_hist = BranchHistory::new(phone_branch);
    phone_hist.append(&phone_id, b"from the phone: 1");
    phone_hist.append(&phone_id, b"from the phone: 2");

    let mut laptop_hist = BranchHistory::new(laptop_branch);
    laptop_hist.append(&laptop_id, b"from the laptop: 1");

    // Verify closure: look the author up and check the signature (no server).
    let mut vks: BTreeMap<Did, VerifyingIdentity> = BTreeMap::new();
    vks.insert(did("alice.phone"), phone_id.verifying());
    vks.insert(did("alice.laptop"), laptop_id.verifying());
    let verify = |author: &Did, bytes: &[u8], sig: &Sig| {
        vks.get(author).is_some_and(|v| v.verify(bytes, sig))
    };

    // The phone self-syncs by importing the laptop's branch — the SAME op as
    // catching up any forked branch (shared genesis → allowed).
    let mut phone_store = HistoryStore::new();
    *phone_store.branch_mut(phone_branch) = phone_hist;
    phone_store
        .backfill_import(&laptop_hist, phone_branch, &lineage, verify)
        .expect("same-lineage self-sync must succeed with no server");

    // Both branches present, distinct and navigable — never interleaved.
    assert_eq!(phone_store.branch_count(), 2, "two distinct device branches after self-sync");
    assert_eq!(phone_store.branch(phone_branch).unwrap().len(), 2);
    assert_eq!(phone_store.branch(laptop_branch).unwrap().len(), 1);

    // An UNRELATED lineage's branch is refused (the backfill privacy boundary).
    let mut foreign_lineage = Lineage::new();
    let foreign_root = GenesisId::from_bytes(b"someone-else-root");
    let foreign_branch = GenesisId::from_bytes(b"someone-else-branch");
    foreign_lineage.add_root(foreign_root, [did("mallory")]);
    foreign_lineage.fork(foreign_root, foreign_branch, [did("mallory")]);
    let mallory = SigningIdentity::from_seed(did("mallory"), 1);
    let mut foreign_hist = BranchHistory::new(foreign_branch);
    foreign_hist.append(&mallory, b"not your lineage");

    let result = phone_store.backfill_import(&foreign_hist, phone_branch, &lineage, verify);
    assert!(result.is_err(), "a branch from a lineage you don't share a root with is refused");
}
