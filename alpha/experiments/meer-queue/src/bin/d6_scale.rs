//! **D6 probe** — how expensive is group construction at scale, and does the cost curve
//! look pathological?
//!
//! Disposition: `throwaway` — the real sweep is Phase 11 (S8).
//!
//! Not a decision gate: Open Question 3 pre-authorized the full S8 sweep with no time
//! ceiling. D6 exists to catch a pathological curve (e.g. quadratic wall-clock) *before*
//! Phase 11 spends hours discovering it.
//!
//! Also records the object sizes at these N, which gives S8 a sanity baseline — but note
//! these use `mls_replant::stamp`, i.e. `MlsGroupCreateConfig::default()`, i.e.
//! **`use_ratchet_tree_extension = false`**. These are the WITHOUT-tree numbers. The
//! with-tree case is what S8 must build its own construction for.

use std::time::Instant;

use mls_replant::{stamp, Persona};

fn main() {
    println!("=== D6: group construction cost (mls_replant::stamp, tree extension OFF) ===");
    println!(
        "{:>6}  {:>10}  {:>12}  {:>12}  {:>10}  {:>10}",
        "N", "stamp ms", "commit B", "welcome B", "B/mbr", "kp ms"
    );

    for n in [2usize, 10, 50, 200, 500] {
        // Persona creation is its own cost (keygen per member); time it separately so the
        // stamp figure is the group operation, not the key generation.
        let t0 = Instant::now();
        let others: Vec<Persona> = (1..n).map(|i| Persona::new(&format!("m{i}"))).collect();
        let planter = Persona::new("planter");
        let kp_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let refs: Vec<&Persona> = others.iter().collect();
        let t1 = Instant::now();
        let s = stamp(&planter, &refs);
        let stamp_ms = t1.elapsed().as_secs_f64() * 1000.0;

        let per_member = if s.member_count > 1 {
            s.welcome_bytes as f64 / (s.member_count - 1) as f64
        } else {
            0.0
        };
        println!(
            "{:>6}  {:>10.1}  {:>12}  {:>12}  {:>10.1}  {:>10.1}",
            s.member_count, stamp_ms, s.commit_bytes, s.welcome_bytes, per_member, kp_ms
        );
    }

    println!("\nRead the stamp-ms column for the shape: roughly linear in N is expected");
    println!("(the stamp is one add-all commit). A superlinear jump is the signal that");
    println!("S8's high-N rungs need a different construction strategy.");
}
