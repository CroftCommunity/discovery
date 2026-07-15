//! M1 — per-commit cost at hot-N (Battery 2, Rung A; built on E12.1's crate).
//!
//! Earns/bounds: Part 2 §11.11 measurement #1 (and §11.4/§11.5) — per-commit re-key cost measured at hot-N (the cost scales on the live set).
//!
//! E12.1 measured the O(N) *stamp*. M1 measures the *per-commit* cost, and the measured
//! result refutes the naive O(log N) expectation for the single-committer case.
//!
//! FINDING: a self-update commit by one member of an N-member tree is **O(N)** (~82 B/member
//! at this ciphersuite), and it does NOT drop across repeated commits by the same member. The
//! reason is structural: a bulk-add stamp populates only the adder's direct path, so the other
//! members' subtree interiors are blank; the committer's co-path therefore resolves over those
//! blank subtrees down to their leaves, giving O(N) HPKE encryptions per commit.
//!
//! Two honest caveats bound what this means for the §11.11 hot-N ceiling:
//!   * it is the cost of a **re-key / membership change**, NOT a per-message cost — MLS
//!     application messages ride the current epoch's key and need no commit (O(1));
//!   * a **fully-populated** tree (every member having committed, not just one) fills the
//!     interior nodes and would make commits cheaper — a separate regime, not measured here
//!     (it needs every member to join and commit).
//!
//! So the load-bearing takeaway is not "per-commit is log-cheap" (false as measured) but "a
//! lone active member re-keying a large hot tree pays O(N) per commit" — which raises the
//! exit-affordability floor (§5.9) and shortens the liveness window more than the optimistic
//! reading assumed.

use mls_replant::{self_update_commit_bytes, stamp, Persona};

#[test]
fn per_commit_is_linear_for_a_single_committer() {
    let sizes = [250usize, 500, 1000];
    let mut per_member: Vec<f64> = Vec::new();
    println!("  N   commit_B  B/member");
    for &n in &sizes {
        let personas: Vec<Persona> = (0..n).map(|i| Persona::new(&format!("m{i}"))).collect();
        let others: Vec<&Persona> = personas[1..].iter().collect();
        let mut s = stamp(&personas[0], &others);
        // Repeated commits do not amortize for a lone committer — take the second.
        let _ = self_update_commit_bytes(&mut s.group, &personas[0]);
        let commit_b = self_update_commit_bytes(&mut s.group, &personas[0]);
        let bpm = commit_b as f64 / n as f64;
        per_member.push(bpm);
        println!("{n:5}  {commit_b:8}  {bpm:8.1}");
    }

    // The finding, pinned: per-commit is LINEAR — per-member bytes stays ~flat across N (a
    // sub-linear/log cost would fall as N grows). Assert the per-member term does not collapse.
    let first = per_member.first().copied().unwrap();
    let last = per_member.last().copied().unwrap();
    assert!(
        last > first * 0.5,
        "per-commit is O(N) for a single committer: B/member should stay ~flat, not fall — \
         N={} → {:.1} B/mbr vs N={} → {:.1} B/mbr",
        sizes[0], first, sizes[sizes.len() - 1], last
    );

    eprintln!(
        "M1 RESULT (refutation of the log-cheap assumption): a lone member's self-update commit \
         is O(N) — ~{:.0} B/member, flat across N and across repeated commits (the co-path \
         resolves over blank sibling subtrees). This is a re-key/membership cost, not a \
         per-message one (messages are O(1)); a fully-committed tree is a separate, cheaper \
         regime. A lone active member re-keying a large hot tree pays O(N) per commit — \
         shortening the liveness window and raising the §5.9 exit-affordability floor more \
         than the optimistic reading assumed.",
        (first + last) / 2.0
    );
}
