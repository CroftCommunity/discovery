//! THROWAWAY validation harness (TDD-exempt) for Part A of the Croft validation
//! campaign — the "fork + deterministic reconcile" claim, moved from argued to
//! *demonstrated on genuinely separate machines*.
//!
//! All correctness lives in (tested) `lineage-core`: `GroupState::apply`,
//! `conflict::detect`, `survivor::select_survivor`, `gov::detect_equivocation`.
//! This binary is glue: it builds a *fixed, deterministic shared world* (so
//! every box constructs the byte-identical genesis + signing keys from the same
//! compiled code, with nothing secret exchanged), applies one membership op,
//! and serializes the signed op-log as JSON. A second invocation re-loads N
//! such logs and runs reconcile/survivor *locally* — no superpeer, no orderer.
//!
//! The decentralization claim is proven by running `reconcile` independently on
//! two (or three) separate machines over the *same* exchanged logs and showing
//! the emitted verdict JSON is byte-identical: each peer computed the same
//! surviving state / same hard-stop from only the histories it holds.
//!
//! Usage:
//!   reconcile-harness world
//!   reconcile-harness apply <add|remove> <subject> <signer,signer,...>   # -> log JSON on stdout
//!   reconcile-harness reconcile <log1.json> <log2.json> [<log3.json> ...] # -> verdict JSON on stdout

use std::collections::{BTreeMap, BTreeSet};
use std::process::exit;

use lineage_core::conflict::{detect, ConflictReason, Resolution};
use lineage_core::dag::Lineage;
use lineage_core::gov::{
    detect_equivocation, sign_op, Directory, Genesis, GenesisRules, GroupState, OpKind, SignedOp,
};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::SigningIdentity;
use lineage_core::survivor::{select_survivor, BranchSummary, SurvivorRule};
use serde::Serialize;

fn did(s: &str) -> Did {
    Did::new(s)
}

/// The fixed shared world. MUST be identical on every box: same admins, same
/// founders, same thresholds, same key seeds -> same `GenesisId` and the same
/// reproducible Ed25519 signatures everywhere, with nothing exchanged.
struct World {
    genesis: Genesis,
    dir: Directory,
    ids: BTreeMap<String, SigningIdentity>,
}

fn world() -> World {
    // Four admins {alice, bob, carol, dave} so an *honest* partition can be
    // modelled with disjoint signer sets per branch (one side signs with
    // {alice,bob}, the other with {carol}/{dave}) — no shared signer, so a
    // genuine contradiction is NOT mislabelled as one admin equivocating.
    // Founders also include erin (a non-admin member who can be the contested
    // subject of an eject-vs-keep contradiction). Remove needs 2 sigs.
    let names = ["alice", "bob", "carol", "dave", "erin", "frank", "grace"];
    let mut ids = BTreeMap::new();
    let mut dir = Directory::new();
    for n in names {
        let id = SigningIdentity::from_seed(did(n), 1);
        dir.insert(id.verifying());
        ids.insert(n.to_string(), id);
    }

    let admins: BTreeSet<Did> =
        [did("alice"), did("bob"), did("carol"), did("dave")].into_iter().collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> =
        [did("alice"), did("bob"), did("carol"), did("dave"), did("erin")].into_iter().collect();
    let genesis = Genesis::new(rules, founders);

    World { genesis, dir, ids }
}

fn parse_kind(s: &str) -> OpKind {
    match s {
        "add" => OpKind::Add,
        "remove" => OpKind::Remove,
        other => {
            eprintln!("unknown op kind: {other} (want add|remove)");
            exit(2);
        }
    }
}

/// `apply` — build a fresh branch from the shared genesis, apply one signed op,
/// emit the resulting log (`Vec<SignedOp>`) as JSON. This is what a peer does
/// *while partitioned*: a local, validly-signed membership decision.
fn cmd_apply(kind: &str, subject: &str, signers_csv: &str) {
    let w = world();
    let kind = parse_kind(kind);
    let signer_names: Vec<&str> = signers_csv.split(',').filter(|s| !s.is_empty()).collect();
    let signers: Vec<&SigningIdentity> = signer_names
        .iter()
        .map(|n| {
            w.ids.get(*n).unwrap_or_else(|| {
                eprintln!("unknown signer: {n}");
                exit(2);
            })
        })
        .collect();

    let mut state = GroupState::new(w.genesis.clone());
    let op = sign_op(&state, kind, Some(did(subject)), &signers);
    if let Err(e) = state.apply(op, &w.dir) {
        eprintln!("apply rejected: {e}");
        exit(1);
    }
    // The op-log is the exchangeable history; state is reconstructed by replay.
    let json = serde_json::to_string_pretty(&state.log).expect("serialize log");
    println!("{json}");
}

// --- verdict types (serde, deterministic field order) ----------------------

#[derive(Serialize)]
struct OpView {
    seq: u64,
    kind: String,
    subject: Option<String>,
    /// Signers, sorted (BTreeMap key order) — the attribution a UI would show.
    signers: Vec<String>,
}

#[derive(Serialize)]
struct BranchView {
    label: String,
    head_hex: String,
    members: Vec<String>,
    ops: Vec<OpView>,
}

#[derive(Serialize)]
struct PairResult {
    left: String,
    right: String,
    resolution: String,
    conflicts: Vec<String>,
}

#[derive(Serialize)]
struct Verdict {
    branches: Vec<BranchView>,
    /// True when the branches do not share a single head — a fork is present.
    fork_detected: bool,
    distinct_heads: usize,
    pairwise: Vec<PairResult>,
    /// Aggregate over all pairs: did any pair hard-stop on a contradiction?
    contradiction: bool,
    /// The contested members, deduped + sorted. Both the removed-then-included
    /// op and the keeping branch are retained in `branches` (loser preserved).
    contested: Vec<String>,
    /// Deterministic survivor by the total-order rule, computed both in the
    /// given branch order and reversed; `survivor_order_independent` proves the
    /// choice does not depend on merge order (no orderer involved).
    survivor_head_hex: String,
    survivor_label: String,
    survivor_order_independent: bool,
    /// Equivocation (an admin double-signing the same seq) across branches —
    /// empty in an honest partition (different admins on different branches).
    equivocations: Vec<String>,
}

fn op_view(op: &SignedOp) -> OpView {
    OpView {
        seq: op.body.seq,
        kind: format!("{:?}", op.body.kind),
        subject: op.body.subject.as_ref().map(|d| d.0.clone()),
        signers: op.sigs.keys().map(|d| d.0.clone()).collect(),
    }
}

fn build_state(w: &World, log: Vec<SignedOp>) -> GroupState {
    let mut state = GroupState::new(w.genesis.clone());
    for op in log {
        if let Err(e) = state.apply(op, &w.dir) {
            eprintln!("replay rejected (tampered/incompatible log): {e}");
            exit(1);
        }
    }
    state
}

/// `state` — load ONE log and print the single branch's head + members. Used by
/// the A3 durable-queue test to show the end-state a peer reaches after syncing
/// a held log is identical whether it came via the broker or directly, and that
/// a tampered held log is rejected on replay (broker cannot alter state).
fn cmd_state(file: &str) {
    let w = world();
    let data = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("read {file}: {e}");
        exit(1);
    });
    let log: Vec<SignedOp> = serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("parse {file}: {e}");
        exit(1);
    });
    let state = build_state(&w, log); // exits(1) on tampered/incompatible replay
    #[derive(Serialize)]
    struct StateView {
        head_hex: String,
        members: Vec<String>,
        ops: Vec<OpView>,
    }
    let view = StateView {
        head_hex: GenesisId(state.head).to_hex(),
        members: state.members.iter().map(|d| d.0.clone()).collect(),
        ops: state.log.iter().map(op_view).collect(),
    };
    println!("{}", serde_json::to_string_pretty(&view).expect("serialize state"));
}

/// `reconcile` — load N exchanged logs, rebuild each branch locally, and run
/// detect/survivor with no third party. Emits a deterministic verdict.
fn cmd_reconcile(files: &[String]) {
    if files.len() < 2 {
        eprintln!("reconcile needs at least 2 log files");
        exit(2);
    }
    let w = world();

    let mut labels = Vec::new();
    let mut states = Vec::new();
    for f in files {
        let data = std::fs::read_to_string(f).unwrap_or_else(|e| {
            eprintln!("read {f}: {e}");
            exit(1);
        });
        let log: Vec<SignedOp> = serde_json::from_str(&data).unwrap_or_else(|e| {
            eprintln!("parse {f}: {e}");
            exit(1);
        });
        labels.push(f.clone());
        states.push(build_state(&w, log));
    }

    let branches: Vec<BranchView> = states
        .iter()
        .zip(&labels)
        .map(|(s, label)| BranchView {
            label: label.clone(),
            head_hex: GenesisId(s.head).to_hex(),
            members: s.members.iter().map(|d| d.0.clone()).collect(),
            ops: s.log.iter().map(op_view).collect(),
        })
        .collect();

    let distinct_heads: BTreeSet<[u8; 32]> = states.iter().map(|s| s.head).collect();
    let fork_detected = distinct_heads.len() > 1;

    // Pairwise reconcile — detect() is deterministic and order-independent.
    let mut pairwise = Vec::new();
    let mut contested = BTreeSet::new();
    let mut contradiction = false;
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            let res = detect(&states[i], &states[j]);
            let (resolution, conflicts) = match &res {
                Resolution::Heal => ("Heal".to_string(), Vec::new()),
                Resolution::HardStop(reasons) => {
                    contradiction = true;
                    let cs: Vec<String> = reasons
                        .iter()
                        .map(|r| match r {
                            ConflictReason::RemovedThenIncluded(d) => {
                                contested.insert(d.0.clone());
                                format!("RemovedThenIncluded({})", d.0)
                            }
                            ConflictReason::DissolvedThenContinued => {
                                "DissolvedThenContinued".to_string()
                            }
                        })
                        .collect();
                    ("HardStop".to_string(), cs)
                }
            };
            pairwise.push(PairResult {
                left: labels[i].clone(),
                right: labels[j].clone(),
                resolution,
                conflicts,
            });
        }
    }

    // Deterministic survivor by the total-order rule. Fold forward and reversed
    // to demonstrate order-independence (the survivor is a property of the set,
    // not the merge order).
    let summaries: Vec<(String, BranchSummary)> = states
        .iter()
        .zip(&labels)
        .map(|(s, label)| {
            (
                label.clone(),
                BranchSummary { genesis: GenesisId(s.head), member_count: s.members.len() },
            )
        })
        .collect();
    let rule = SurvivorRule::MemberCountThenGenesis;
    let fold = |order: &[(String, BranchSummary)]| -> GenesisId {
        let mut acc = order[0].1.genesis;
        let mut acc_sum = order[0].1;
        for (_, s) in &order[1..] {
            let winner = select_survivor(&acc_sum, s, rule);
            // Carry the winning summary forward.
            if winner == acc_sum.genesis {
                // acc stays
            } else {
                acc_sum = *s;
            }
            acc = winner;
        }
        acc
    };
    let survivor_fwd = fold(&summaries);
    let mut rev = summaries.clone();
    rev.reverse();
    let survivor_rev = fold(&rev);
    let survivor_order_independent = survivor_fwd == survivor_rev;
    let survivor_label = summaries
        .iter()
        .find(|(_, s)| s.genesis == survivor_fwd)
        .map(|(l, _)| l.clone())
        .unwrap_or_default();

    // Equivocation across branches (honest partition: empty).
    let mut equivocations = BTreeSet::new();
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            for a in &states[i].log {
                for b in &states[j].log {
                    for e in detect_equivocation(a, b, &w.dir) {
                        equivocations.insert(format!(
                            "{} double-signed seq {}",
                            e.culprit.0, e.seq
                        ));
                    }
                }
            }
        }
    }

    let verdict = Verdict {
        branches,
        fork_detected,
        distinct_heads: distinct_heads.len(),
        pairwise,
        contradiction,
        contested: contested.into_iter().collect(),
        survivor_head_hex: survivor_fwd.to_hex(),
        survivor_label,
        survivor_order_independent,
        equivocations: equivocations.into_iter().collect(),
    };
    println!("{}", serde_json::to_string_pretty(&verdict).expect("serialize verdict"));
}

/// `reform` — the trap-door / clean-exit backstop. An ejected member re-forms a
/// group consisting of the shared ancestor MINUS the removers, and the lineage
/// DAG proves the re-formation legibly descends from the original (no false
/// ancestry, no history erasure). Deterministic, so every box agrees.
fn cmd_reform(ejected: &str, removers_csv: &str, followers_csv: &str) {
    let w = world();
    let g0 = w.genesis.id;
    let parse = |csv: &str| -> Vec<Did> {
        csv.split(',').filter(|s| !s.is_empty()).map(did).collect()
    };
    let removers = parse(removers_csv);
    let followers = parse(followers_csv);

    // Re-formed membership: the ejected member + followers, minus the removers.
    let mut members: BTreeSet<Did> = followers.iter().cloned().collect();
    members.insert(did(ejected));
    for r in &removers {
        members.remove(r);
    }

    // Fresh genesis derived deterministically from the re-formation inputs.
    let mut seed = b"reform-v1:".to_vec();
    seed.extend_from_slice(ejected.as_bytes());
    for m in &members {
        seed.push(0);
        seed.extend_from_slice(m.0.as_bytes());
    }
    let g1 = GenesisId::from_bytes(&seed);

    // Lineage: original root (with its founders) + the re-formed fork.
    let mut lin = Lineage::new();
    lin.add_root(g0, w.genesis.founders.iter().cloned());
    lin.fork(g0, g1, members.iter().cloned());

    #[derive(Serialize)]
    struct ReformVerdict {
        original_genesis_hex: String,
        reformed_genesis_hex: String,
        reformed_members: Vec<String>,
        removers: Vec<String>,
        shares_lineage_with_original: bool,
        removers_excluded_from_membership: bool,
        ejected_has_standing_on_original: bool,
        ejected_has_standing_on_reformed: bool,
        removers_retain_lineage_standing: bool,
    }
    let removers_excluded = removers.iter().all(|r| !members.contains(r));
    let removers_standing = removers.iter().all(|r| lin.standing(r, g1));
    let verdict = ReformVerdict {
        original_genesis_hex: g0.to_hex(),
        reformed_genesis_hex: g1.to_hex(),
        reformed_members: members.iter().map(|d| d.0.clone()).collect(),
        removers: removers.iter().map(|d| d.0.clone()).collect(),
        shares_lineage_with_original: lin.shares_lineage(g1, g0),
        removers_excluded_from_membership: removers_excluded,
        ejected_has_standing_on_original: lin.standing(&did(ejected), g0),
        ejected_has_standing_on_reformed: lin.standing(&did(ejected), g1),
        removers_retain_lineage_standing: removers_standing,
    };
    println!("{}", serde_json::to_string_pretty(&verdict).expect("serialize reform verdict"));
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["world"] => {
            let w = world();
            println!("genesis_id_hex: {}", w.genesis.id.to_hex());
            let members: Vec<String> = w.genesis.founders.iter().map(|d| d.0.clone()).collect();
            println!("founders: {members:?}");
        }
        ["apply", kind, subject, signers] => cmd_apply(kind, subject, signers),
        ["state", file] => cmd_state(file),
        ["reform", ejected, removers, followers] => cmd_reform(ejected, removers, followers),
        ["reconcile", files @ ..] if files.len() >= 2 => {
            cmd_reconcile(&files.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  reconcile-harness world");
            eprintln!("  reconcile-harness apply <add|remove> <subject> <signer,signer,...>");
            eprintln!("  reconcile-harness state <log.json>");
            eprintln!("  reconcile-harness reconcile <log1.json> <log2.json> [<log3.json> ...]");
            eprintln!("  reconcile-harness reform <ejected> <removers-csv> <followers-csv>");
            exit(2);
        }
    }
}
