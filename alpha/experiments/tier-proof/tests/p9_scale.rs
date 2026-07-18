//! P9 — Scale rehearsal, measured (component; numbers are the deliverable).
//!
//! Two measurements; this test asserts their INVARIANTS (so the numbers are
//! trustworthy), and `bin/measure` emits the actual figures into MEASUREMENTS.md.
//! No boundary number is chosen here — that is an owner call (§4 non-goal).
//!
//! (a) backplane verification throughput: a synthetic 100k roster (its ~10 MB
//!     claim is checked, not assumed), single-core envelope verifications/sec
//!     (signature + roster-at-position), and a light-client inclusion-proof size
//!     at 100k (log-N against a co-signed roster head);
//! (b) the sealed ceiling: a membership-churn simulation at increasing N,
//!     recording epoch-roll cost and the concurrent-commit contradiction rate
//!     under the no-arbiter rule. The croft-group loopback harness backs this
//!     when it builds; otherwise a local epoch model stands in
//!     (`SPEC-DELTA[run17-churn-model | declared-stand-in]`).

use std::collections::HashSet;
use tier_proof::scale;

#[test]
fn synthetic_100k_roster_is_about_ten_megabytes() {
    let roster = scale::synthetic_roster(100_000);
    let distinct: HashSet<&String> = roster.iter().collect();
    assert_eq!(distinct.len(), 100_000, "100k distinct roster entries");
    let bytes = scale::roster_bytes(&roster);
    // The ~10 MB claim: checked, with generous bounds (a did:key line is ~57 B).
    assert!(bytes > 5_000_000, "roster is multi-megabyte (got {bytes} B)");
    assert!(bytes < 20_000_000, "roster is ~10 MB, not absurd (got {bytes} B)");
}

#[test]
fn light_client_inclusion_proof_is_log_n() {
    let roster = scale::synthetic_roster(100_000);
    let root = scale::merkle_root(&roster);
    let index = 42_000;
    let proof = scale::inclusion_proof(&roster, index);

    // log2(100_000) ≈ 16.6 → a 17-level proof.
    assert!(proof.len() <= 18, "proof is log-N ({} hashes)", proof.len());
    assert!(scale::verify_inclusion(root, &roster[index], index, &proof), "the proof verifies");

    // A wrong leaf does not verify against the same root/position.
    assert!(!scale::verify_inclusion(root, "did:key:zNotOnTheRoster", index, &proof));

    // The proof is small: 32 B/level plus the co-signed head.
    let size = scale::proof_size_bytes(&proof);
    assert!(size < 1024, "inclusion proof under 1 KB (got {size} B)");
}

#[test]
fn verification_throughput_is_positive_and_measured() {
    let roster = scale::synthetic_roster(1_000);
    let roster_set: HashSet<String> = roster.iter().cloned().collect();
    let report = scale::measure_verify_throughput(2_000, &roster_set);
    assert_eq!(report.verified, 2_000, "every sampled envelope verified");
    assert!(report.per_second > 0.0, "a positive throughput was measured");
}

#[test]
fn churn_simulation_reports_epoch_roll_and_contradiction_rate() {
    // (b) at component grade over the local epoch model.
    for n in [8usize, 64, 256] {
        let curve = scale::simulate_churn(n, 20);
        assert_eq!(curve.n, n);
        assert!(curve.epoch_roll_nanos > 0, "epoch-roll cost recorded at N={n}");
        // Under the no-arbiter rule, concurrent commits to the same epoch
        // contradict; the rate is in [0, 1].
        assert!((0.0..=1.0).contains(&curve.contradiction_rate));
        // Concurrency was actually exercised (some contradictions at N>1).
        assert!(curve.concurrent_commits > 0);
    }
}
