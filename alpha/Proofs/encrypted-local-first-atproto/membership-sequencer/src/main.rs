//! Phase 10: the membership sequencer (delivery service).
//!
//! Phase 7 proved MLS membership must be totally ordered and used an ad-hoc
//! tiebreak. This builds the explicit **delivery-service sequencer** that
//! provides that order: concurrent add-commits are linearized (accept-first-per-
//! epoch); losers catch up by applying the accepted commit and re-submit. We
//! drain a queue of THREE concurrent proposals to show forks never occur and
//! every proposal eventually commits (liveness, no starvation).

#[allow(dead_code)]
mod mls;
mod seq;

use mls::Member;
use openmls::prelude::MlsGroup;
use seq::{Sequencer, Submit};

fn section(t: &str) {
    println!("\n=== {t} ===");
}
fn pass(r: &mut Vec<(&'static str, bool)>, name: &'static str, ok: bool) {
    println!("  -> {}: {}", name, if ok { "PASS" } else { "FAIL" });
    r.push((name, ok));
}

/// Apply the winning commit: the committer merges its pending; every other
/// current member applies the commit; the new member joins via the Welcome.
fn resolve(
    members: &[Member],
    groups: &mut [Option<MlsGroup>],
    current: &mut Vec<usize>,
    winner_committer: usize,
    winner_new: usize,
    commit: &[u8],
    welcome: &[u8],
) {
    mls::merge_own(groups[winner_committer].as_mut().unwrap(), &members[winner_committer]);
    for idx in current.clone() {
        if idx != winner_committer {
            mls::apply_commit(groups[idx].as_mut().unwrap(), &members[idx], commit);
        }
    }
    let g = mls::join_from_welcome(&members[winner_new], welcome);
    groups[winner_new] = Some(g);
    current.push(winner_new);
}

/// Drive a set of pending `(committer, new_member)` adds through the sequencer
/// until all are committed. Returns the number of sequencer rounds used.
fn drain(
    members: &[Member],
    groups: &mut Vec<Option<MlsGroup>>,
    current: &mut Vec<usize>,
    sequencer: &mut Sequencer,
    mut pending: Vec<(usize, usize)>,
) -> usize {
    let mut rounds = 0;
    while !pending.is_empty() {
        rounds += 1;
        let round_epoch = sequencer.epoch;

        // Every pending committer stages an add against the current epoch.
        let mut staged: Vec<(usize, usize, Vec<u8>, Vec<u8>)> = Vec::new();
        for &(committer, new_member) in pending.iter() {
            let (commit, welcome) = mls::stage_add(groups[committer].as_mut().unwrap(), &members[committer], &members[new_member]);
            staged.push((committer, new_member, commit, welcome));
        }

        // Submit all to the sequencer; it accepts the first, rejects the rest.
        let mut winner = None;
        for (i, (committer, new_member, commit, _w)) in staged.iter().enumerate() {
            match sequencer.submit(round_epoch, commit.clone()) {
                Submit::Accepted => {
                    println!("  round {rounds}: ACCEPT {} adds {} at epoch {round_epoch}", members[*committer].name, members[*new_member].name);
                    winner = Some(i);
                }
                Submit::Rejected { current_epoch } => {
                    println!("  round {rounds}: reject {} adds {} (epoch advanced to {current_epoch}; will re-submit)", members[*committer].name, members[*new_member].name);
                }
            }
        }
        let wi = winner.expect("sequencer accepted exactly one");

        // Losers abort their staged commit; they will re-stage next round.
        for (i, (committer, _n, _c, _w)) in staged.iter().enumerate() {
            if i != wi {
                mls::clear_own(groups[*committer].as_mut().unwrap(), &members[*committer]);
            }
        }
        let (wp, wt, wc, ww) = staged[wi].clone();
        resolve(members, groups, current, wp, wt, &wc, &ww);
        pending.retain(|&(p, t)| !(p == wp && t == wt));
    }
    rounds
}

fn main() {
    let mut results: Vec<(&'static str, bool)> = Vec::new();

    let members: Vec<Member> = ["Alice", "Bob", "Carol", "Dave", "Erin", "Frank"]
        .iter().map(|n| Member::new(n)).collect();
    let mut groups: Vec<Option<MlsGroup>> = (0..members.len()).map(|_| None).collect();
    let mut current: Vec<usize> = Vec::new();

    // ------------------------------------------------------------------
    section("STEP 0: Alice founds the group; sequencer tracks the epoch order");
    // ------------------------------------------------------------------
    let ag = mls::create_group(&members[0]);
    let start_epoch = ag.epoch().as_u64();
    groups[0] = Some(ag);
    current.push(0);
    let mut sequencer = Sequencer::at(start_epoch);
    println!("founded at epoch {start_epoch}; members = Alice");

    // ------------------------------------------------------------------
    section("STEP 1: Sequential adds (Bob, then Carol) via the sequencer");
    // ------------------------------------------------------------------
    drain(&members, &mut groups, &mut current, &mut sequencer, vec![(0, 1)]);
    drain(&members, &mut groups, &mut current, &mut sequencer, vec![(0, 2)]);
    println!("members now: Alice, Bob, Carol at epoch {}", sequencer.epoch);

    // ------------------------------------------------------------------
    section("STEP 2: THREE CONCURRENT adds at one epoch (Alice+Dave, Bob+Erin, Carol+Frank)");
    // ------------------------------------------------------------------
    // All three committers act against the same epoch — a 3-way fork attempt.
    let rounds = drain(&members, &mut groups, &mut current, &mut sequencer, vec![(0, 3), (1, 4), (2, 5)]);
    println!("drained 3 concurrent proposals in {rounds} sequencer rounds");
    pass(&mut results, "1. 3 concurrent proposals all committed (liveness, no starvation)", rounds == 3 && current.len() == 6);

    // ------------------------------------------------------------------
    section("STEP 3: All six members converged — no fork");
    // ------------------------------------------------------------------
    let epochs: Vec<u64> = (0..6).map(|i| groups[i].as_ref().unwrap().epoch().as_u64()).collect();
    let ref_members = mls::member_identities(groups[0].as_ref().unwrap());
    let membership_agree = (0..6).all(|i| mls::member_identities(groups[i].as_ref().unwrap()) == ref_members) && ref_members.len() == 6;
    let ref_key = members[0].content_key(groups[0].as_ref().unwrap());
    let keys_agree = (0..6).all(|i| members[i].content_key(groups[i].as_ref().unwrap()) == ref_key);
    println!("epochs = {epochs:?}");
    println!("all 6 agree on membership? {membership_agree}");
    println!("all 6 derive the same epoch key ({}…)? {keys_agree}", hex::encode(&ref_key[..6]));
    let all_same_epoch = epochs.iter().all(|&e| e == epochs[0]);
    pass(&mut results, "2. No fork: identical epoch, membership, and key across all members", all_same_epoch && membership_agree && keys_agree);

    // The sequencer's accepted log is the canonical total order.
    println!("sequencer accepted {} commits, totally ordered; final epoch = {}", sequencer.accepted.len(), sequencer.epoch);
    pass(&mut results, "3. Sequencer holds the canonical total order of commits", sequencer.accepted.len() as u64 == sequencer.epoch - start_epoch);

    // ------------------------------------------------------------------
    section("SUMMARY");
    // ------------------------------------------------------------------
    let mut all = true;
    for (name, ok) in &results {
        println!("[{}] {}", if *ok { "PASS" } else { "FAIL" }, name);
        all &= *ok;
    }
    println!("\nOVERALL: {}", if all { "PASS — sequencer linearizes concurrent membership; group never forks" } else { "FAIL" });

    section("ISSUES SURFACED");
    println!("  1. The sequencer is a REQUIRED role and a single point of ordering: it must be");
    println!("     available, and if malicious it can censor or reorder MEMBERSHIP (not read content,");
    println!("     which stays E2E-encrypted). Candidates: the superpeer, or a designated/rotating");
    println!("     member. Trust + availability of this role is now a first-class design concern.");
    println!("  2. Wasted work under contention: every losing committer stages a commit that is");
    println!("     discarded and must be re-staged (fresh key package each time). N concurrent");
    println!("     proposals take N rounds; throughput is one membership change per epoch.");
    println!("  3. Fairness/ordering policy lives in the sequencer (here: first-submitted wins).");
    println!("     A real DS needs an explicit, auditable ordering rule and likely backpressure.");
    println!("  4. Content sync stays unordered/P2P (CRDT) — only membership needs this sequencer,");
    println!("     so the local-first property is preserved for the data path.");

    section("VERSION REPORT");
    println!("rustc {} | openmls {}", env!("SLICE_RUSTC_VERSION"), env!("SLICE_VER_OPENMLS"));

    if !all {
        std::process::exit(1);
    }
}
