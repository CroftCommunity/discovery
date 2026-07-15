//! AR-5 — MLS ratchet-tree + rekey scaling under the per-device-as-member model.
//!
//! multi-device.md flagged that making each device a distinct MLS member could
//! blow up the tree (e.g. 50 people × 3 devices = 150 leaves). This measures the
//! real cost on openmls 0.8.1: the per-add commit size and the rekey (remove)
//! commit size as the leaf count grows. MLS uses a left-balanced binary tree, so
//! a single commit's path is ~O(log N) — BUT our groups set
//! `use_ratchet_tree_extension(true)` (so newcomers join without an out-of-band
//! tree), which embeds the FULL tree in the commit/group-info, making commit size
//! ~O(N). This test measures that and pins the affordability ceiling + the
//! mitigation. Run: `cargo test -p lineage-mls --test ar5_tree_scaling -- --nocapture`.
//!
//! FINDING (2026-06-16): with the ratchet-tree extension ON, per-add and rekey
//! commits grow ~linearly: ~1.4 KB @ 8 leaves → ~11 KB @ 128 leaves (~1.8× per
//! doubling). Affordable for realistic interactive/quiet groups (hundreds of
//! leaves → tens of KB). For very large groups, disabling the extension (ship the
//! tree out-of-band / via the broker snapshot) restores ~O(log N) commits. So
//! per-device-as-member is affordable at human group scale; the broadcast tier
//! (1000s) must not use the embedded-tree mode.

use lineage_mls::{Device, LineageClaim};
use lineage_core::keys::SigningIdentity;
use lineage_core::Did;

fn did(s: &str) -> Did {
    Did::new(s)
}

#[test]
fn ar5_tree_commit_size_under_per_device_model() {
    let root = SigningIdentity::from_seed(did("founder-lineage"), 0);
    let mut founder = Device::new_with_lineage(
        did("founder.dev"),
        LineageClaim::sign(&root, &did("founder.dev")),
    )
    .unwrap();
    founder.create_group().unwrap();

    // Milestones: report the commit size for the add that crosses each leaf count.
    let milestones = [8usize, 32, 64, 128];
    let target = *milestones.last().unwrap();

    let mut samples: Vec<(usize, usize)> = Vec::new(); // (leaf_count, add_commit_bytes)
    let mut last_index = founder.leaf_index_of(&did("founder.dev")).unwrap().unwrap();

    for n in 1..=target {
        // A fresh member with its own lineage (worst case: every leaf distinct).
        let lin = SigningIdentity::from_seed(did(&format!("L{n}")), 0);
        let dev_did = did(&format!("L{n}.dev"));
        let member =
            Device::new_with_lineage(dev_did.clone(), LineageClaim::sign(&lin, &dev_did)).unwrap();
        let kp = member.key_package().unwrap();
        let (commit, _welcome) = founder.add(&[kp]).unwrap();
        last_index = founder.leaf_index_of(&dev_did).unwrap().unwrap();
        let leaves = founder.member_count().unwrap();
        if milestones.contains(&leaves) {
            samples.push((leaves, commit.len()));
        }
    }

    // Rekey cost: remove one leaf at full size and measure the commit.
    let (rekey_commit, _w) = founder.remove(&[last_index]).unwrap();

    println!("AR-5 tree scaling (per-device-as-member, openmls 0.8.1):");
    for (leaves, bytes) in &samples {
        println!("  leaves={leaves:>4}  add_commit_bytes={bytes}");
    }
    println!(
        "  rekey(remove) commit at {} leaves = {} bytes",
        founder.member_count().unwrap() + 1,
        rekey_commit.len()
    );

    // The group built to `target`+founder (= target+1) leaves, then removed one.
    assert_eq!(
        founder.member_count().unwrap(),
        target,
        "built {target}+founder leaves, removed one → {target} remain"
    );
    // Affordability ceiling: even with the full tree embedded, a ~150-leaf group's
    // commit stays in the tens of KB — fine for an interactive/quiet group.
    let large = samples.iter().find(|(l, _)| *l == 128).map(|(_, b)| *b).unwrap();
    assert!(
        large < 32 * 1024,
        "embedded-tree commit at 128 leaves ({large}B) must stay affordable (<32 KiB) for \
         human-scale groups; the broadcast tier (1000s) must disable the tree extension"
    );
    // Growth IS roughly linear (the embedded tree), which is the documented finding —
    // assert it is observably super-logarithmic so the finding can't silently regress.
    let small = samples.iter().find(|(l, _)| *l == 8).map(|(_, b)| *b).unwrap();
    assert!(large > small * 4, "embedded-tree growth is ~linear (the finding), not log");
}
