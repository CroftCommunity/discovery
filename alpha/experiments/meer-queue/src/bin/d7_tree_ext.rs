//! **D7 probe** (added during Phase 0 — a gap Pass 3 identified but created no probe for).
//!
//! Question: is S8's construction path actually available? Pass 3 established that
//! `mls_replant::stamp` cannot serve S8 — it hardcodes `MlsGroupCreateConfig::default()`
//! (so `use_ratchet_tree_extension = false`) and discards the `GroupInfo` from
//! `add_members`. Phase 11 therefore needs its own config-parameterized construction.
//! This probe confirms that construction compiles and produces the two objects S8 is
//! actually about — and takes a first directional reading of the O(N) growth that has
//! **no prior measurement anywhere in this corpus**.
//!
//! Disposition: `promote` — becomes the config-parameterized builder in `src/mls.rs`
//! (Phase 11). The real sweep, with the full object table and the crossover, is S8.

use mls_replant::{Persona, CS};
use openmls::prelude::*;
use tls_codec::Serialize as _;

/// Build a group of `n` members with the ratchet-tree extension on or off, and return
/// `(commit_bytes, welcome_bytes, group_info_bytes)`.
fn build(n: usize, tree_ext: bool) -> (usize, usize, Option<usize>) {
    let planter = Persona::new("planter");
    let others: Vec<Persona> = (1..n).map(|i| Persona::new(&format!("m{i}"))).collect();
    let kps: Vec<KeyPackage> = others.iter().map(Persona::key_package).collect();

    let config = MlsGroupCreateConfig::builder()
        .use_ratchet_tree_extension(tree_ext)
        .ciphersuite(CS)
        .build();

    let mut group = MlsGroup::new(&planter.provider, &planter.signer, &config, planter.cwk.clone())
        .expect("create group");

    let (commit, welcome, group_info) = group
        .add_members(&planter.provider, &planter.signer, &kps)
        .expect("add_members");
    group.merge_pending_commit(&planter.provider).expect("merge");

    let commit_b = commit.tls_serialize_detached().expect("ser commit").len();
    let welcome_b = welcome.tls_serialize_detached().expect("ser welcome").len();
    let gi_b = group_info.map(|gi| gi.tls_serialize_detached().expect("ser gi").len());
    (commit_b, welcome_b, gi_b)
}

fn main() {
    const CAP: usize = 2 * 1024 * 1024;
    println!("=== D7: is S8's construction available, and what does the tree extension cost? ===");
    println!("(2 MiB cap = {CAP} bytes)\n");
    println!(
        "{:>6}  {:>4}  {:>12}  {:>12}  {:>14}  {:>12}",
        "N", "ext", "commit B", "welcome B", "group_info B", "welcome B/mbr"
    );

    for n in [2usize, 10, 50, 200] {
        for ext in [false, true] {
            let (c, w, gi) = build(n, ext);
            let per = if n > 1 { w as f64 / (n - 1) as f64 } else { 0.0 };
            println!(
                "{:>6}  {:>4}  {:>12}  {:>12}  {:>14}  {:>12.1}",
                n,
                if ext { "ON" } else { "off" },
                c,
                w,
                gi.map_or("None".to_string(), |b| b.to_string()),
                per
            );
        }
    }

    println!("\nNOTE: `group_info` is returned by add_members only when the config asks for it;");
    println!("`mls_replant::stamp` discards it entirely, which is why S8 needs this path.");
    println!("These are FIRST measurements of the tree-ON case — no prior exists in the corpus.");
    println!("The full object table and the 2 MiB crossover are Phase 11 (S8), not this probe.");
}
