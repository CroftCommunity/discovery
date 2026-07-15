//! THROWAWAY validation harness (TDD-exempt) for the **local-first history**
//! claim: per-branch signed message logs that reconcile by *voluntary consensual
//! backfill* — absorbing another party's branch as a *separate, navigable*
//! branch, never spliced into one canonical transcript. The same mechanism
//! serves BOTH a single user's multiple devices AND a group of distinct people,
//! all bound to a single shared ancestor (genesis). It is the alternative to the
//! rigidity of one-true-transcript designs: divergence stays livable.
//!
//! All correctness lives in (tested) `lineage-core` (`dag::Lineage`) and
//! `lineage-history` (`HistoryStore::backfill_import`, fold/unfold). This binary
//! is glue: build a fixed shared lineage, write a device/member's signed branch,
//! serialize it, and on another machine absorb donor branches with verification.
//!
//! Usage:
//!   history-harness write <name> <msg> [<msg> ...]        # -> branch JSON on stdout
//!   history-harness write-forged <name> <msg>             # branch w/ a tampered signature
//!   history-harness merge <myname> <donor.json> [<donor.json> ...]   # -> backfill report

use std::process::exit;

use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BranchHistory, HistoryStore, Message};
use serde::{Deserialize, Serialize};

/// The shared ancestor (root genesis) every device/member is bound to.
fn shared_root() -> GenesisId {
    GenesisId::from_bytes(b"croft-shared-lineage-root-v1")
}

/// Each party writes to its own branch, deterministically derived from its name
/// and forked off the shared root — so every machine computes the same ids.
fn branch_id(name: &str) -> GenesisId {
    GenesisId::from_bytes(format!("croft-branch:{name}").as_bytes())
}

fn did(name: &str) -> Did {
    Did::new(name)
}

/// The fixed roster — both a single user's devices and a group's members. Every
/// machine builds the identical lineage from this, so standing/sharing checks
/// agree everywhere. `mallory` is deliberately NOT here (an outsider with no
/// branch and no standing) to exercise rejection.
const ROSTER: &[&str] = &["dev1", "dev2", "dev3", "alice", "bob", "carol"];

fn signing_identity(name: &str) -> SigningIdentity {
    SigningIdentity::from_seed(did(name), 1)
}

/// The shared lineage: one root, one forked branch per roster member (each a
/// member of its own branch). Identical on every machine.
fn lineage() -> Lineage {
    let mut l = Lineage::new();
    let root = shared_root();
    l.add_root(root, ROSTER.iter().map(|n| did(n)));
    for n in ROSTER {
        l.fork(root, branch_id(n), [did(n)]);
    }
    l
}

fn verifier(name: &str) -> VerifyingIdentity {
    signing_identity(name).verifying()
}

// --- serializable message (Message itself isn't serde; wrap its public fields) -

#[derive(Serialize, Deserialize)]
struct MsgDto {
    author: Did,
    seq: u64,
    branch: GenesisId,
    payload: Vec<u8>,
    sig: Sig,
}

impl MsgDto {
    fn from_msg(m: &Message) -> Self {
        Self {
            author: m.author.clone(),
            seq: m.seq,
            branch: m.branch,
            payload: m.payload.clone(),
            sig: m.sig.clone(),
        }
    }
    fn into_msg(self) -> Message {
        Message {
            author: self.author,
            seq: self.seq,
            branch: self.branch,
            payload: self.payload,
            sig: self.sig,
        }
    }
}

fn serialize_branch(h: &BranchHistory) -> String {
    let dtos: Vec<MsgDto> = h.messages().iter().map(MsgDto::from_msg).collect();
    serde_json::to_string_pretty(&dtos).expect("serialize branch")
}

fn load_branch(file: &str) -> BranchHistory {
    let data = std::fs::read_to_string(file).unwrap_or_else(|e| {
        eprintln!("read {file}: {e}");
        exit(1);
    });
    let dtos: Vec<MsgDto> = serde_json::from_str(&data).unwrap_or_else(|e| {
        eprintln!("parse {file}: {e}");
        exit(1);
    });
    // Reconstruct the branch verbatim (push_raw keeps any tamper intact so the
    // verifier on the receiving side is what decides acceptance).
    let branch = dtos.first().map(|d| d.branch).unwrap_or_else(shared_root);
    let mut h = BranchHistory::new(branch);
    for d in dtos {
        h.push_raw(d.into_msg());
    }
    h
}

fn cmd_write(name: &str, msgs: &[String], forge: bool) {
    let id = signing_identity(name);
    let mut h = BranchHistory::new(branch_id(name));
    for m in msgs {
        h.append(&id, m.as_bytes());
    }
    if forge {
        // Flip one byte of the last message's signature to simulate tampering in
        // transit; the receiver's verifier must then reject this branch.
        let mut dtos: Vec<MsgDto> = h.messages().iter().map(MsgDto::from_msg).collect();
        if let Some(last) = dtos.last_mut() {
            let mut bytes = last.sig.0;
            bytes[0] ^= 0x01;
            last.sig = Sig(bytes);
        }
        println!("{}", serde_json::to_string_pretty(&dtos).expect("ser"));
        return;
    }
    println!("{}", serialize_branch(&h));
}

fn cmd_merge(myname: &str, donor_files: &[String]) {
    let lineage = lineage();
    let my_branch = branch_id(myname);
    let mut store = HistoryStore::new();
    // My own branch exists locally first (it is mine; donors are absorbed beside it).
    let _ = store.branch_mut(my_branch);

    let verify = |author: &Did, bytes: &[u8], sig: &Sig| -> bool {
        // Every roster member's public key is known; verify against the claimed author.
        ROSTER
            .iter()
            .map(|n| verifier(n))
            .any(|v| v.did() == author && v.verify(bytes, sig))
    };

    println!("== local-first backfill report for '{myname}' (my branch {}) ==", my_branch);
    for f in donor_files {
        let donor = load_branch(f);
        match store.backfill_import(&donor, my_branch, &lineage, &verify) {
            Ok(()) => println!(
                "  ABSORBED {} ({} msgs) from {} -> kept as a separate navigable branch",
                short(donor.branch),
                donor.messages().len(),
                f
            ),
            Err(e) => println!("  REJECTED {} from {} -> {}", short(donor.branch), f, e),
        }
    }

    // Navigable view: each branch is separate (no interleave). Show per-branch
    // counts and the total — proving history is navigable, not spliced.
    println!("-- navigable branches held by '{myname}' --");
    let mut total = 0usize;
    for n in ROSTER {
        if let Some(b) = store.branch(branch_id(n)) {
            total += b.messages().len();
            println!(
                "  branch {} ({}): {} msgs{}",
                short(branch_id(n)),
                n,
                b.messages().len(),
                if b.messages().is_empty() { " (mine, empty)" } else { "" }
            );
        }
    }
    println!("  branch_count={} total_msgs={} (separate branches, never interleaved)", store.branch_count(), total);

    // Demonstrate fold (I9): hiding one absorbed branch removes it from the daily
    // view without deleting it — local-first, no ambient pressure.
    if let Some(first_donor) = donor_files.first() {
        let donor = load_branch(first_donor);
        let b = store.branch_mut(donor.branch);
        let before = b.visible().len();
        b.fold();
        let after_folded = b.visible().len();
        b.unfold();
        let after_unfold = b.visible().len();
        println!(
            "-- fold demo on branch {}: visible {} -> folded {} -> unfolded {} (lossless, inert)",
            short(donor.branch),
            before,
            after_folded,
            after_unfold
        );
    }
}

fn short(g: GenesisId) -> String {
    g.to_hex()[..12].to_string()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["write", name, msgs @ ..] if !msgs.is_empty() => {
            cmd_write(name, &msgs.iter().map(|s| s.to_string()).collect::<Vec<_>>(), false)
        }
        ["write-forged", name, msg] => cmd_write(name, &[msg.to_string()], true),
        ["merge", myname, donors @ ..] if !donors.is_empty() => {
            cmd_merge(myname, &donors.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  history-harness write <name> <msg> [<msg> ...]");
            eprintln!("  history-harness write-forged <name> <msg>");
            eprintln!("  history-harness merge <myname> <donor.json> [<donor.json> ...]");
            exit(2);
        }
    }
}
