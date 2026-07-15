//! M1 — the per-commit **cost band**: sparse (lone committer) vs populated tree (Battery 2,
//! Rung A). This is the honest close on the M1 refutation started in `m1_per_commit_cost`.
//!
//! Earns/bounds: Part 2 §11.11 measurement #1 (and §11.4/§11.5) — the per-commit cost band: sparse committer vs populated tree.
//!
//! `m1_per_commit_cost` measured the *sparse* regime and refuted "commits are log-cheap": a lone
//! committer on a bulk-add-stamped tree pays **O(N)** per commit, because the stamp populates
//! only the adder's direct path, leaving every other member's subtree interior blank — the
//! committer's co-path resolves down to the blank leaves.
//!
//! This test brackets the OTHER end of the band. When every member has committed once (a full
//! round-robin), the tree interior is fully populated, and a subsequent self-update only
//! re-keys the path from the committer's leaf to the root — **O(log N)**. So the true statement
//! is a band, not a point:
//!
//!   * **sparse** (fresh stamp, lone active committer): O(N) per commit — the pessimistic floor;
//!   * **populated** (steady-state, everyone commits): O(log N) per commit — the optimistic ceiling.
//!
//! What decides which regime a hot group is in is *how many distinct members commit*: a group
//! with one active re-keyer stays sparse and pays O(N); a group where activity is spread across
//! members populates the tree and settles toward O(log N). Both are true; the §11.11 liveness
//! window must be tuned for the sparse floor, not the populated ceiling.
//!
//! Falsifies if: the populated-tree per-commit cost does NOT fall relative to the sparse cost as
//! N grows (i.e. populating the interior buys nothing), which would collapse the band to a
//! single O(N) point and contradict the MLS tree-KEM path-update claim.

use mls_replant::{apply_commit, commit, join, self_update_commit_bytes, stamp, MlsGroup, Persona};

#[test]
fn per_commit_cost_band_sparse_is_on_and_populated_falls() {
    let sizes = [8usize, 16, 32];
    println!("  N   sparse_B  spB/mbr   pop_B  popB/mbr");

    let mut sparse_pm: Vec<f64> = Vec::new();
    let mut pop_pm: Vec<f64> = Vec::new();

    for &n in &sizes {
        // ---- sparse regime: lone committer on a fresh stamp (the O(N) floor) ----
        let personas: Vec<Persona> = (0..n).map(|i| Persona::new(&format!("m{i}"))).collect();
        let others: Vec<&Persona> = personas[1..].iter().collect();
        let mut s = stamp(&personas[0], &others);
        let _ = self_update_commit_bytes(&mut s.group, &personas[0]); // warm
        let sparse_b = self_update_commit_bytes(&mut s.group, &personas[0]);

        // ---- populated regime: every member joins and commits once (round-robin) ----
        // Re-stamp a fresh group so the sparse warm-up above doesn't contaminate the measurement.
        let personas2: Vec<Persona> = (0..n).map(|i| Persona::new(&format!("p{i}"))).collect();
        let others2: Vec<&Persona> = personas2[1..].iter().collect();
        let s2 = stamp(&personas2[0], &others2);
        // Planter keeps s2.group; every other member joins from the Welcome + ratchet tree.
        let mut groups: Vec<MlsGroup> = Vec::with_capacity(n);
        groups.push(s2.group);
        let welcome = s2.welcome.expect("welcome for a populated stamp");
        for p in &personas2[1..] {
            groups.push(join(p, welcome.clone(), s2.ratchet_tree.clone()));
        }
        // Round-robin: each member commits once; broadcast every commit to the others.
        for i in 0..n {
            let (_b, msg) = commit(&mut groups[i], &personas2[i]);
            for (j, p) in personas2.iter().enumerate() {
                if j != i {
                    apply_commit(&mut groups[j], p, &msg);
                }
            }
        }
        // Now the tree interior is populated. Measure a fresh commit by member 0.
        let (pop_b, _msg) = commit(&mut groups[0], &personas2[0]);

        let sp_bpm = sparse_b as f64 / n as f64;
        let pop_bpm = pop_b as f64 / n as f64;
        sparse_pm.push(sp_bpm);
        pop_pm.push(pop_bpm);
        println!("{n:5}  {sparse_b:8}  {sp_bpm:7.1}  {pop_b:6}  {pop_bpm:8.1}");
    }

    // Claim 1 (re-confirms m1_per_commit_cost): sparse per-member cost stays ~flat (O(N)).
    let sp_first = *sparse_pm.first().unwrap();
    let sp_last = *sparse_pm.last().unwrap();
    assert!(
        sp_last > sp_first * 0.5,
        "sparse per-commit is O(N): B/member should stay ~flat, not fall — N={} → {:.1} vs N={} → {:.1}",
        sizes[0], sp_first, sizes[sizes.len() - 1], sp_last
    );

    // Claim 2 (the band): populated per-member cost FALLS as N grows (O(log N) beats O(N)).
    let pop_first = *pop_pm.first().unwrap();
    let pop_last = *pop_pm.last().unwrap();
    assert!(
        pop_last < pop_first * 0.75,
        "populated per-commit is sub-linear: B/member should fall as N grows — N={} → {:.1} vs N={} → {:.1}",
        sizes[0], pop_first, sizes[sizes.len() - 1], pop_last
    );

    // Claim 3 (the gap): at the largest N, populated is materially cheaper than sparse.
    let sparse_big = *sparse_pm.last().unwrap();
    let pop_big = *pop_pm.last().unwrap();
    assert!(
        pop_big < sparse_big,
        "at N={} a populated-tree commit must be cheaper per member than a sparse one — \
         populated {:.1} vs sparse {:.1}",
        sizes[sizes.len() - 1], pop_big, sparse_big
    );

    eprintln!(
        "M1 COST BAND (honest close): per-commit cost is a BAND, not a point. Sparse (lone \
         committer on a fresh stamp) is O(N) — flat per-member across N. Populated (every member \
         committed once) is O(log N) — per-member cost falls as N grows. Which regime a hot group \
         sits in is decided by how many distinct members commit: concentrate re-keying on one \
         member and you pay the O(N) floor; spread it and you settle toward the O(log N) ceiling. \
         Tune the §11.11 liveness window for the floor."
    );
}
