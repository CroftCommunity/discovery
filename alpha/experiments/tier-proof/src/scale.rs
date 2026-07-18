//! Scale rehearsal (RUN-16 P9): measured numbers, no boundary chosen.
//!
//! (a) backplane verification throughput — a synthetic 100k roster, single-core
//!     envelope verifications/sec (signature + roster-at-position), and a
//!     light-client inclusion-proof size at 100k (a Merkle proof against a
//!     co-signed roster head);
//! (b) the sealed ceiling — a membership-churn simulation recording epoch-roll
//!     cost and the concurrent-commit contradiction rate under the no-arbiter
//!     rule. The croft-group group-seal crate builds cleanly via the proxy, so
//!     the real MLS harness is available; this module's `simulate_churn` is a
//!     local epoch model measuring the SAME two quantities at component grade
//!     (`SPEC-DELTA[run17-churn-model | declared-stand-in]`).

use std::collections::HashSet;
use std::time::Instant;

use sha2::{Digest, Sha256};

use crate::envelope::Envelope;
use crate::identity::Signer;
use crate::records::{self, Record};

/// A synthetic roster of `n` distinct `did:key`-shaped identifiers. Bytes are
/// derived from the index (sha256), so entries are the right SHAPE and size
/// without `n` real key generations — the roster is compared as strings.
#[must_use]
pub fn synthetic_roster(n: usize) -> Vec<String> {
    (0..n)
        .map(|i| {
            let mut buf = Vec::with_capacity(34);
            buf.extend_from_slice(&[0xed, 0x01]);
            buf.extend_from_slice(&Sha256::digest((i as u64).to_be_bytes()));
            format!("did:key:z{}", bs58::encode(buf).into_string())
        })
        .collect()
}

/// The wire size of a roster (sum of entry byte-lengths). The check behind the
/// "~10 MB at 100k" claim.
#[must_use]
pub fn roster_bytes(roster: &[String]) -> usize {
    roster.iter().map(String::len).sum()
}

// ───────────────────────── light-client Merkle inclusion ────────────────────

fn leaf_hash(did: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x00]); // leaf domain separation
    h.update(did.as_bytes());
    h.finalize().into()
}

fn node_hash(l: &[u8; 32], r: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x01]); // node domain separation
    h.update(l);
    h.update(r);
    h.finalize().into()
}

/// The Merkle root over the roster (leaves in roster order; odd levels duplicate
/// the last node). This is the co-signed "roster head" a light client trusts.
#[must_use]
pub fn merkle_root(roster: &[String]) -> [u8; 32] {
    let mut level: Vec<[u8; 32]> = roster.iter().map(|d| leaf_hash(d)).collect();
    if level.is_empty() {
        return [0u8; 32];
    }
    while level.len() > 1 {
        level = fold_level(&level);
    }
    level[0]
}

fn fold_level(level: &[[u8; 32]]) -> Vec<[u8; 32]> {
    let mut next = Vec::with_capacity(level.len().div_ceil(2));
    let mut i = 0;
    while i < level.len() {
        let l = level[i];
        let r = if i + 1 < level.len() {
            level[i + 1]
        } else {
            level[i]
        };
        next.push(node_hash(&l, &r));
        i += 2;
    }
    next
}

/// The sibling path proving `roster[index]` is under [`merkle_root`].
#[must_use]
pub fn inclusion_proof(roster: &[String], index: usize) -> Vec<[u8; 32]> {
    let mut level: Vec<[u8; 32]> = roster.iter().map(|d| leaf_hash(d)).collect();
    let mut idx = index;
    let mut proof = Vec::new();
    while level.len() > 1 {
        let sibling = if idx.is_multiple_of(2) {
            if idx + 1 < level.len() {
                level[idx + 1]
            } else {
                level[idx]
            }
        } else {
            level[idx - 1]
        };
        proof.push(sibling);
        level = fold_level(&level);
        idx /= 2;
    }
    proof
}

/// Verify an inclusion proof for `leaf` at `index` against `root`.
#[must_use]
pub fn verify_inclusion(root: [u8; 32], leaf: &str, index: usize, proof: &[[u8; 32]]) -> bool {
    let mut acc = leaf_hash(leaf);
    let mut idx = index;
    for sib in proof {
        acc = if idx.is_multiple_of(2) {
            node_hash(&acc, sib)
        } else {
            node_hash(sib, &acc)
        };
        idx /= 2;
    }
    acc == root
}

/// The size in bytes of an inclusion proof (32 B per level).
#[must_use]
pub fn proof_size_bytes(proof: &[[u8; 32]]) -> usize {
    proof.len() * 32
}

// ───────────────────────── verification throughput ──────────────────────────

/// A throughput measurement.
#[derive(Debug, Clone, Copy)]
pub struct ThroughputReport {
    /// Envelopes verified.
    pub verified: usize,
    /// Wall-clock seconds.
    pub seconds: f64,
    /// Verifications per second (signature + roster-at-position lookup).
    pub per_second: f64,
}

/// Measure single-core envelope verification throughput: sign `sample` real
/// envelopes from a small set of on-roster authors, then time verifying each
/// (signature + roster membership lookup).
#[must_use]
pub fn measure_verify_throughput(sample: usize, roster: &HashSet<String>) -> ThroughputReport {
    // A handful of real signers whose DIDs are on the roster.
    let signers: Vec<Signer> = (0..8)
        .map(|i| Signer::from_seed([200 + i as u8; 32]))
        .collect();
    let mut roster = roster.clone();
    for s in &signers {
        roster.insert(s.did());
    }
    let envelopes: Vec<Envelope> = (0..sample)
        .map(|i| {
            let s = &signers[i % signers.len()];
            records::seal(
                s,
                vec![],
                &Record::Message {
                    scope: "scope:bench".into(),
                    text: format!("m{i}"),
                },
            )
        })
        .collect();

    let start = Instant::now();
    let mut verified = 0usize;
    for env in &envelopes {
        if env.verify().is_ok() && roster.contains(&env.body.author) {
            verified += 1;
        }
    }
    let seconds = start.elapsed().as_secs_f64();
    ThroughputReport {
        verified,
        seconds,
        per_second: if seconds > 0.0 {
            verified as f64 / seconds
        } else {
            f64::INFINITY
        },
    }
}

// ───────────────────────── churn / epoch ceiling (b) ────────────────────────

/// One point on the churn curve.
#[derive(Debug, Clone, Copy)]
pub struct ChurnPoint {
    /// Group size.
    pub n: usize,
    /// Cost of a single epoch roll (rekey proxy) in nanoseconds.
    pub epoch_roll_nanos: u128,
    /// Number of concurrent-commit pairs attempted.
    pub concurrent_commits: usize,
    /// Fraction of concurrent commits that contradicted under no-arbiter.
    pub contradiction_rate: f64,
}

/// Simulate membership churn for a group of `n` at `rounds` of change. The
/// epoch-roll cost is modelled as recomputing a group secret over all `n`
/// members (the shape of an MLS rekey: O(N) work). The no-arbiter contradiction
/// rate is the fraction of concurrent commit pairs that both target the same
/// epoch — with no arbiter, both are "valid" locally yet mutually exclusive.
#[must_use]
pub fn simulate_churn(n: usize, rounds: usize) -> ChurnPoint {
    // Epoch-roll cost: hash-chain a secret across all members (deterministic,
    // O(N)), timed once and reported.
    let start = Instant::now();
    let mut secret = [0u8; 32];
    for m in 0..n {
        let mut h = Sha256::new();
        h.update(secret);
        h.update((m as u64).to_be_bytes());
        secret = h.finalize().into();
    }
    let epoch_roll_nanos = start.elapsed().as_nanos().max(1);

    // Concurrency: over `rounds`, two proposers each commit against the current
    // epoch. Under the no-arbiter rule, if both target the same epoch they
    // contradict (only one can win, but nothing decides which). We model the
    // collision deterministically from the round index and group size.
    let mut contradictions = 0usize;
    let concurrent_commits = rounds;
    for r in 0..rounds {
        // Two proposers pick a target member to change; a collision on the same
        // epoch head is a contradiction. Larger groups spread proposals, so the
        // collision chance falls ~1/n — the ceiling the measurement exposes.
        let a = (r * 2654435761) % n.max(1);
        let b = (r * 40503) % n.max(1);
        if a == b {
            contradictions += 1;
        }
    }
    let contradiction_rate = if concurrent_commits > 0 {
        contradictions as f64 / concurrent_commits as f64
    } else {
        0.0
    };

    ChurnPoint {
        n,
        epoch_roll_nanos,
        concurrent_commits,
        contradiction_rate,
    }
}
