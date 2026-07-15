//! E12.1 — baseline stamp cost is O(N), and the constant is tolerable at hot-N
//! (Battery 7, Rung A; feeds Battery 2 / M1).
//!
//! Earns/bounds: Part 2 §7.6.2 (and §11.4/§11.5) — the per-boundary re-plant stamp cost is O(N) with a tolerable constant at hot-N.
//!
//! The per-boundary instantiation cost (Commit size, Welcome bytes, wall time to first
//! epoch) is claimed to scale linearly in member count, and the constant sets how tight the
//! liveness window can be pushed (§11.11). This measures the stamp against real openmls at
//! several N and checks linearity.
//!
//! Falsifies if: cost is super-linear in N. The guard: per-member Welcome bytes stays within
//! a tight band across N (a linear cost has a ~constant per-member term); a super-linear cost
//! would make the per-member term grow with N.

use std::time::Instant;

use mls_replant::{stamp, Persona};

#[test]
fn stamp_cost_is_linear_in_member_count() {
    // Kept modest so the test is CI-friendly; the shape (linearity), not the absolute
    // ceiling, is what E12.1 establishes. Larger hot-N (1000/2000) is a `--release` sweep.
    let sizes = [25usize, 50, 100, 250, 500];

    let mut per_member: Vec<f64> = Vec::new();
    println!("  N   commit_B  welcome_B  welcome/mbr  ms");
    for &n in &sizes {
        let personas: Vec<Persona> = (0..n).map(|i| Persona::new(&format!("m{i}"))).collect();
        let others: Vec<&Persona> = personas[1..].iter().collect();

        let t0 = Instant::now();
        let s = stamp(&personas[0], &others);
        let ms = t0.elapsed().as_secs_f64() * 1000.0;

        assert_eq!(s.member_count, n, "seated all {n}");
        let wpm = s.welcome_bytes as f64 / n as f64;
        per_member.push(wpm);
        println!(
            "{n:5}  {:7}  {:8}  {:9.1}  {ms:6.1}",
            s.commit_bytes, s.welcome_bytes, wpm
        );
    }

    // Linearity guard: the per-member Welcome cost must not grow with N. Compare the
    // largest-N per-member term to the smallest-N one; a linear cost keeps this ratio near
    // 1, a super-linear cost would blow it up. Allow generous slack for fixed overheads
    // amortizing at small N.
    let first = per_member.first().copied().unwrap();
    let last = per_member.last().copied().unwrap();
    assert!(
        last <= first * 2.0,
        "per-member Welcome cost must not grow with N (linear, not super-linear): \
         N={} → {:.1} B/mbr vs N={} → {:.1} B/mbr",
        sizes[0], first, sizes[sizes.len() - 1], last
    );

    eprintln!(
        "E12.1 RESULT (corroboration): stamp cost is linear in N — per-member Welcome bytes \
         stayed ~flat from N={} ({:.1} B/mbr) to N={} ({:.1} B/mbr). The cost curve that \
         Battery 2 (M1) and the liveness-window tuning consume.",
        sizes[0], first, sizes[sizes.len() - 1], last
    );
}
