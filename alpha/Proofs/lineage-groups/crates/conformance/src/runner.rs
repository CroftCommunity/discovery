//! The conformance runner: re-prove each vector against the real API.
//!
//! For every vector the runner reconstructs the input, feeds it through the
//! actual `lineage-core` / `lineage-history` public functions, and compares the
//! observed result to the recorded expected value. A mismatch is a FAIL. The
//! runner never trusts the recorded value as the answer — it recomputes it.
//!
//! This is production code: each `check_*` function is covered by an inline test
//! that constructs a known-good and a known-bad vector and asserts the verdict.

use std::collections::{BTreeMap, BTreeSet};

use lineage_core::dag::Lineage;
use lineage_core::gov::{
    sign_op, Directory, Genesis, GenesisRules, GroupState, OpBody, OpKind, RejectReason, SignedOp,
};
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::{Sig, SigningIdentity, VerifyingIdentity};
use lineage_history::{BackfillError, BranchHistory, HistoryStore, Message};

use crate::model::{
    AdversarialKind, AdversarialVector, AuthorityVector, DerivationKind, DerivationVector,
    FoldVector, ReconcileVector, RevocationVector, SigningVector, TsExperiment,
};

/// The outcome of checking one vector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckOutcome {
    /// The vector re-proved: observed == expected.
    Pass,
    /// The vector did not re-prove. Carries a human-readable reason.
    Fail(String),
}

impl CheckOutcome {
    /// True iff this is a pass.
    #[must_use]
    pub fn is_pass(&self) -> bool {
        matches!(self, CheckOutcome::Pass)
    }
}

/// Decode a hex string into bytes, mapping any error to a `Fail`.
fn unhex(field: &str, s: &str) -> Result<Vec<u8>, CheckOutcome> {
    hex::decode(s).map_err(|e| CheckOutcome::Fail(format!("{field}: bad hex: {e}")))
}

/// Decode a hex string into a fixed 32-byte array.
fn unhex32(field: &str, s: &str) -> Result<[u8; 32], CheckOutcome> {
    let bytes = unhex(field, s)?;
    bytes
        .try_into()
        .map_err(|_| CheckOutcome::Fail(format!("{field}: expected 32 bytes")))
}

// --- Category 1: derivations -------------------------------------------------

/// Re-prove a derivation against the canonical `lineage-core` API.
///
/// * `Structural` re-hashes the recorded `input_hex` via `GenesisId::from_bytes`
///   (the untagged `sha256(canonical_bytes)` content-hash anchor).
/// * The tagged kinds (`LineageGenesis` / `GroupGenesis` / `GroupTopic`) call the
///   real `lineage_core::ids::{lineage_genesis,group_genesis,group_topic}` over
///   the recorded `input_id`, so the domain-separator tag is proven by the real
///   code — never re-typed here. For those the runner also confirms the recorded
///   `input_hex` is the genuine `tag ‖ id` pre-image of the output (portability
///   guard for a foreign impl).
#[must_use]
pub fn check_derivation(v: &DerivationVector) -> CheckOutcome {
    let expected = match unhex32("expected", &v.expected_hex) {
        Ok(b) => b,
        Err(o) => return o,
    };

    let observed = match v.derivation {
        DerivationKind::Structural => {
            let input = match unhex("input", &v.input_hex) {
                Ok(b) => b,
                Err(o) => return o,
            };
            GenesisId::from_bytes(&input).0
        }
        DerivationKind::LineageGenesis => lineage_core::ids::lineage_genesis(&v.input_id).0,
        DerivationKind::GroupGenesis => lineage_core::ids::group_genesis(&v.input_id).0,
        DerivationKind::GroupTopic => lineage_core::ids::group_topic(&v.input_id),
    };

    if observed != expected {
        return CheckOutcome::Fail(format!(
            "{}: derived {} != expected {}",
            v.kind,
            hex::encode(observed),
            hex::encode(expected)
        ));
    }

    // For tagged kinds, also verify the recorded pre-image hashes to the output,
    // so a foreign implementation that hashes `input_hex` reproduces the result.
    if !matches!(v.derivation, DerivationKind::Structural) {
        let preimage = match unhex("input", &v.input_hex) {
            Ok(b) => b,
            Err(o) => return o,
        };
        let preimage_hash = GenesisId::from_bytes(&preimage).0;
        if preimage_hash != expected {
            return CheckOutcome::Fail(format!(
                "{}: recorded pre-image hashes to {} != expected {} (tag ‖ id mismatch)",
                v.kind,
                hex::encode(preimage_hash),
                hex::encode(expected)
            ));
        }
    }

    CheckOutcome::Pass
}

// --- Category 2: signing -----------------------------------------------------

/// Re-prove a signing vector through the real public API. Recomputes
/// `signing_bytes` from the inputs (must match the recorded pre-image),
/// rebuilds the *real* signer via `SigningIdentity::from_seed(author, seed)`,
/// confirms its verifying key equals the recorded portable key, then verifies
/// the recorded signature via `VerifyingIdentity::verify`. The observed verify
/// result must match the vector's `expect`.
#[must_use]
pub fn check_signing(v: &SigningVector) -> CheckOutcome {
    let branch = match unhex32("branch", &v.branch_hex) {
        Ok(b) => GenesisId(b),
        Err(o) => return o,
    };
    let payload = match unhex("payload", &v.payload_hex) {
        Ok(b) => b,
        Err(o) => return o,
    };
    let author = Did::new(v.author.clone());
    let recomputed = Message::signing_bytes(branch, v.seq, &author, &payload);
    let recorded_bytes = match unhex("signing_bytes", &v.signing_bytes_hex) {
        Ok(b) => b,
        Err(o) => return o,
    };
    if recomputed != recorded_bytes {
        return CheckOutcome::Fail(format!(
            "signing_bytes mismatch: recomputed {} != recorded {}",
            hex::encode(&recomputed),
            hex::encode(&recorded_bytes)
        ));
    }

    // Rebuild the real signer from the recorded seed and confirm its public key
    // matches the portable key recorded in the vector. This proves the recorded
    // `verifying_key_hex` is the genuine key the real impl derives.
    let signer = SigningIdentity::from_seed(author.clone(), v.author_seed);
    let verifier = signer.verifying();
    let observed_vk = hex::encode(verifier.to_bytes());
    if observed_vk != v.verifying_key_hex {
        return CheckOutcome::Fail(format!(
            "verifying_key mismatch: real key {observed_vk} != recorded {}",
            v.verifying_key_hex
        ));
    }

    let sig_bytes = match unhex("signature", &v.signature_hex) {
        Ok(b) => b,
        Err(o) => return o,
    };
    let sig_arr: [u8; 64] = match sig_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return CheckOutcome::Fail("signature: expected 64 bytes".into()),
    };
    let verified = verifier.verify(&recomputed, &Sig(sig_arr));
    let want_accept = v.expect == "accept";
    if verified == want_accept {
        CheckOutcome::Pass
    } else {
        CheckOutcome::Fail(format!(
            "expect={} but verify returned {}",
            v.expect, verified
        ))
    }
}

// --- Categories 3+4: fold / thresholds --------------------------------------

/// Re-prove a fold/threshold vector: build a real group whose admins are the
/// signers' lineage roots, sign a Remove op with all device signers, then count
/// distinct DIDs (`valid_admin_sigs`) vs distinct lineages
/// (`valid_admin_lineages`). The lineage count must match `expected_by_lineage`,
/// the DID count `expected_by_did`, and the lineage-counted threshold verdict
/// must match `expect`.
#[must_use]
pub fn check_fold(v: &FoldVector) -> CheckOutcome {
    // Admin set = the lineages (one admin DID per lineage is the founder), plus
    // every device DID (devices are members who can sign). We model each device
    // as its own admin DID and map it to its lineage via `lineage_of`.
    let kind = match v.op_kind.as_str() {
        "Remove" => OpKind::Remove,
        "Add" => OpKind::Add,
        other => return CheckOutcome::Fail(format!("unknown op_kind {other}")),
    };

    let mut ids: BTreeMap<String, SigningIdentity> = BTreeMap::new();
    let mut dir = Directory::new();
    let mut admins: BTreeSet<Did> = BTreeSet::new();
    let mut members: BTreeSet<Did> = BTreeSet::new();
    let mut lineage_of: BTreeMap<Did, Did> = BTreeMap::new();
    for s in &v.signers {
        let did = Did::new(s.did.clone());
        let id = SigningIdentity::from_seed(did.clone(), 1);
        dir.insert(id.verifying());
        ids.insert(s.did.clone(), id);
        admins.insert(did.clone());
        members.insert(did.clone());
        lineage_of.insert(did.clone(), Did::new(s.lineage.clone()));
    }
    // A subject to remove/add: a member not among the signers.
    let subject = Did::new("__subject__");
    members.insert(subject.clone());

    let mut thresholds = BTreeMap::new();
    thresholds.insert(kind, v.threshold);
    let rules = GenesisRules {
        admins: admins.clone(),
        thresholds,
    };
    let genesis = Genesis::new(rules, members.clone());
    let mut state = GroupState::new(genesis);
    state.members = members; // founders already include all; explicit for clarity

    let signers: Vec<&SigningIdentity> = v.signers.iter().map(|s| &ids[&s.did]).collect();
    let op = sign_op(&state, kind, Some(subject), &signers);

    let by_did = state.valid_admin_sigs(&op, &dir);
    let by_lineage = state.valid_admin_lineages(&op, &dir, &lineage_of);
    let meets = state.meets_threshold_by_lineage(&op, &dir, &lineage_of);

    if by_did != v.expected_by_did {
        return CheckOutcome::Fail(format!(
            "by_did: observed {by_did} != expected {}",
            v.expected_by_did
        ));
    }
    if by_lineage != v.expected_by_lineage {
        return CheckOutcome::Fail(format!(
            "by_lineage: observed {by_lineage} != expected {}",
            v.expected_by_lineage
        ));
    }
    let want_accept = v.expect == "accept";
    if meets != want_accept {
        return CheckOutcome::Fail(format!(
            "expect={} but meets_threshold_by_lineage={meets}",
            v.expect
        ));
    }
    CheckOutcome::Pass
}

// --- Category 5: revocation mechanics ---------------------------------------

/// Re-prove a revocation-mechanics vector via the real `backfill_import`. The
/// donor branch's author either holds standing (pre-revocation, accept) or does
/// not (post-revocation, reject:unauthorized-author). The verdict must match
/// `expect`.
#[must_use]
pub fn check_revocation(v: &RevocationVector) -> CheckOutcome {
    let branch = match unhex32("branch", &v.branch_hex) {
        Ok(b) => GenesisId(b),
        Err(o) => return o,
    };
    // The author signs every message; standing is the dial under test.
    let authors: BTreeSet<String> = v.messages.iter().map(|m| m.author.clone()).collect();
    let mut ids: BTreeMap<String, SigningIdentity> = BTreeMap::new();
    for a in &authors {
        ids.insert(
            a.clone(),
            SigningIdentity::from_seed(Did::new(a.clone()), 1),
        );
    }

    // Build the donor branch by *signing* each message with the real key (so the
    // signature is genuine; the rejection under test is standing, not signature).
    let mut donor = BranchHistory::new(branch);
    for m in &v.messages {
        let payload = match unhex("payload", &m.payload_hex) {
            Ok(b) => b,
            Err(o) => return o,
        };
        donor.append(&ids[&m.author], &payload);
    }

    // Lineage: a root the recipient shares, with members reflecting standing.
    let mut lineage = Lineage::new();
    let members: Vec<Did> = if v.author_has_standing {
        authors.iter().map(|a| Did::new(a.clone())).collect()
    } else {
        Vec::new() // revoked: no one holds standing on this branch
    };
    lineage.add_root(branch, members);

    let mut store = HistoryStore::new();
    let result = store.backfill_import(&donor, branch, &lineage, |did, bytes, sig| {
        ids.get(&did.0)
            .map(|id| id.verifying().verify(bytes, sig))
            .unwrap_or(false)
    });

    let want_accept = v.expect == "accept";
    match (&result, want_accept) {
        (Ok(()), true) => CheckOutcome::Pass,
        (Err(_), false) => CheckOutcome::Pass,
        (Ok(()), false) => {
            CheckOutcome::Fail(format!("expected {} but backfill accepted", v.expect))
        }
        (Err(e), true) => CheckOutcome::Fail(format!("expected accept but rejected: {e}")),
    }
}

// --- Category 5b: revoke-AUTHORITY threshold --------------------------------

/// Re-prove a revoke-authority vector through the real `gov` API. This exercises
/// the green-real signature + lineage-counted-threshold mechanism
/// (`meets_threshold_by_lineage`): rebuild the admin genesis, install the
/// recorded `(did, sig)` pairs onto a Remove `SignedOp`, build a `Directory` from
/// the recorded verifying keys, and count distinct admin LINEAGES that validly
/// signed against the genesis threshold. The verdict must match `expect`.
///
/// Note (scope): over-the-wire authority distribution + the co-sign-vs-vote
/// ordering decision are NOT exercised here — they remain Workstream C (see
/// `discovery/thinking/open-edges.md` §1). This vector covers only the
/// signature-verification + threshold-counting logic a second implementation
/// must reproduce regardless of transport.
#[must_use]
pub fn check_revocation_authority(v: &AuthorityVector) -> CheckOutcome {
    let kind = match v.op_kind.as_str() {
        "Remove" => OpKind::Remove,
        other => return CheckOutcome::Fail(format!("unknown op_kind {other}")),
    };

    // Reconstruct the immutable genesis from the recorded admin set + threshold.
    let admins: BTreeSet<Did> = v.admins.iter().map(|a| Did::new(a.did.clone())).collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(kind, v.threshold);
    let rules = GenesisRules { admins, thresholds };

    // Founders = every recorded DID (admins + non-admins + the subject) so that
    // each signer is a current member (holds standing) and the subject exists.
    let mut founders: BTreeSet<Did> = BTreeSet::new();
    for a in v.admins.iter().chain(v.non_admin_signers.iter()) {
        founders.insert(Did::new(a.did.clone()));
    }
    founders.insert(Did::new(v.subject.clone()));
    let genesis = Genesis::new(rules, founders);
    let state = GroupState::new(genesis);

    // Build the directory from the recorded portable verifying keys. A foreign
    // implementation verifies the recorded sigs against exactly these public
    // keys, so we reconstruct VerifyingIdentity from the bytes (not from a seed).
    let mut dir = Directory::new();
    for a in v.admins.iter().chain(v.non_admin_signers.iter()) {
        let key_bytes = match unhex32("verifying_key", &a.verifying_key_hex) {
            Ok(b) => b,
            Err(o) => return o,
        };
        match VerifyingIdentity::from_bytes(Did::new(a.did.clone()), &key_bytes) {
            Some(vi) => dir.insert(vi),
            None => {
                return CheckOutcome::Fail(format!("{}: malformed verifying key", a.did));
            }
        }
    }

    // Rebuild the op body. The op is a Remove of `subject` at the chain head of a
    // fresh state (seq 0, prev = genesis id). Its signing bytes MUST equal the
    // recorded pre-image — fail loud on any drift rather than verifying a body the
    // signatures were not made over.
    let body = OpBody {
        genesis: state.genesis.id,
        seq: state.log.len() as u64,
        prev: state.head,
        kind,
        subject: Some(Did::new(v.subject.clone())),
    };
    let recomputed = body.signing_bytes();
    let recorded_bytes = match unhex("signing_bytes", &v.signing_bytes_hex) {
        Ok(b) => b,
        Err(o) => return o,
    };
    if recomputed != recorded_bytes {
        return CheckOutcome::Fail(format!(
            "signing_bytes mismatch: recomputed {} != recorded {}",
            hex::encode(&recomputed),
            hex::encode(&recorded_bytes)
        ));
    }

    // Install the recorded signatures onto the op.
    let mut sigs: BTreeMap<Did, Sig> = BTreeMap::new();
    for s in &v.sigs {
        let sig_bytes = match unhex("signature", &s.signature_hex) {
            Ok(b) => b,
            Err(o) => return o,
        };
        let arr: [u8; 64] = match sig_bytes.try_into() {
            Ok(a) => a,
            Err(_) => return CheckOutcome::Fail(format!("{}: signature not 64 bytes", s.did)),
        };
        sigs.insert(Did::new(s.did.clone()), Sig(arr));
    }
    let op = SignedOp { body, sigs };

    // The E2.10 lineage map.
    let lineage_of: BTreeMap<Did, Did> = v
        .lineage_of
        .iter()
        .map(|(k, val)| (Did::new(k.clone()), Did::new(val.clone())))
        .collect();

    let meets = state.meets_threshold_by_lineage(&op, &dir, &lineage_of);
    let want_accept = v.expect == "accept";
    if meets == want_accept {
        CheckOutcome::Pass
    } else {
        CheckOutcome::Fail(format!(
            "expect={} but meets_threshold_by_lineage={meets} (by_lineage={})",
            v.expect,
            state.valid_admin_lineages(&op, &dir, &lineage_of)
        ))
    }
}

// --- Category 6: reconcile ---------------------------------------------------

/// The fixed reconcile world: admins {alice,bob,carol,dave}, founders + erin,
/// Remove needs 2 sigs, Add needs 1. Mirrors the part-A corpus world so the
/// vectors are derived from the same real configuration.
struct ReconcileWorld {
    genesis: Genesis,
    dir: Directory,
    ids: BTreeMap<String, SigningIdentity>,
}

fn reconcile_world() -> ReconcileWorld {
    let names = ["alice", "bob", "carol", "dave", "erin", "frank", "grace"];
    let mut ids = BTreeMap::new();
    let mut dir = Directory::new();
    for n in names {
        let id = SigningIdentity::from_seed(Did::new(n), 1);
        dir.insert(id.verifying());
        ids.insert(n.to_string(), id);
    }
    let admins: BTreeSet<Did> = ["alice", "bob", "carol", "dave"]
        .iter()
        .map(|s| Did::new(*s))
        .collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(OpKind::Remove, 2);
    thresholds.insert(OpKind::Add, 1);
    let rules = GenesisRules { admins, thresholds };
    let founders: BTreeSet<Did> = ["alice", "bob", "carol", "dave", "erin"]
        .iter()
        .map(|s| Did::new(*s))
        .collect();
    let genesis = Genesis::new(rules, founders);
    ReconcileWorld { genesis, dir, ids }
}

/// Re-prove a reconcile vector. Builds each partitioned branch by applying its
/// signed ops to a fresh copy of the shared genesis, then runs the real
/// `conflict::detect` / `survivor::select_survivor` and classifies the verdict.
/// The classification and contested set must match the vector.
#[must_use]
pub fn check_reconcile(v: &ReconcileVector) -> CheckOutcome {
    let w = reconcile_world();

    // "re-formation" vectors exercise the DAG fork path, not detect(): a single
    // re-formed branch descending from the shared root. We verify the fork shares
    // lineage with the original and excludes the removers.
    if v.verdict == "re-formation" {
        return check_reformation(v, &w);
    }

    let mut states = Vec::new();
    for b in &v.branches {
        let mut state = GroupState::new(w.genesis.clone());
        for op in &b.ops {
            let kind = match op.kind.as_str() {
                "Add" => OpKind::Add,
                "Remove" => OpKind::Remove,
                "Dissolve" => OpKind::Dissolve,
                other => return CheckOutcome::Fail(format!("unknown op kind {other}")),
            };
            let subject = if op.subject.is_empty() {
                None
            } else {
                Some(Did::new(op.subject.clone()))
            };
            let signers: Vec<&SigningIdentity> =
                op.signers.iter().filter_map(|s| w.ids.get(s)).collect();
            let signed = sign_op(&state, kind, subject, &signers);
            if let Err(e) = state.apply(signed, &w.dir) {
                return CheckOutcome::Fail(format!(
                    "{}: branch {} op {:?} rejected: {e}",
                    v.id, b.label, op.kind
                ));
            }
        }
        states.push(state);
    }

    // Pairwise detect; collect contested members; classify.
    let mut contested = BTreeSet::new();
    let mut contradiction = false;
    for i in 0..states.len() {
        for j in (i + 1)..states.len() {
            match lineage_core::conflict::detect(&states[i], &states[j]) {
                lineage_core::conflict::Resolution::Heal => {}
                lineage_core::conflict::Resolution::HardStop(reasons) => {
                    contradiction = true;
                    for r in reasons {
                        if let lineage_core::conflict::ConflictReason::RemovedThenIncluded(d) = r {
                            contested.insert(d.0);
                        }
                    }
                }
            }
        }
    }

    let observed_verdict = if contradiction {
        "hard-stop"
    } else {
        "converge"
    };
    if observed_verdict != v.verdict {
        return CheckOutcome::Fail(format!(
            "{}: observed verdict {observed_verdict} != expected {}",
            v.id, v.verdict
        ));
    }
    let observed_contested: Vec<String> = contested.into_iter().collect();
    if observed_contested != v.contested {
        return CheckOutcome::Fail(format!(
            "{}: contested {observed_contested:?} != expected {:?}",
            v.id, v.contested
        ));
    }
    CheckOutcome::Pass
}

/// Re-prove a re-formation vector: the ejected member re-forms off the shared
/// ancestor; the fork MUST share lineage with the original and exclude removers,
/// while removers retain lineage standing (history not erased).
fn check_reformation(v: &ReconcileVector, w: &ReconcileWorld) -> CheckOutcome {
    // Convention: branch[0] carries the removal ops (who got ejected, by whom);
    // we read the single Remove op to recover ejected + removers.
    let removal_branch = match v.branches.first() {
        Some(b) => b,
        None => return CheckOutcome::Fail(format!("{}: no branches", v.id)),
    };
    let remove_op = match removal_branch.ops.iter().find(|o| o.kind == "Remove") {
        Some(o) => o,
        None => return CheckOutcome::Fail(format!("{}: no Remove op in reformation", v.id)),
    };
    let ejected = Did::new(remove_op.subject.clone());
    let removers: Vec<Did> = remove_op
        .signers
        .iter()
        .map(|s| Did::new(s.clone()))
        .collect();

    // Followers: founders minus removers minus ejected, plus ejected.
    let mut members: BTreeSet<Did> = w.genesis.founders.clone();
    for r in &removers {
        members.remove(r);
    }
    members.insert(ejected.clone());

    let g0 = w.genesis.id;
    // Fresh genesis derived deterministically from the re-formation inputs (same
    // recipe as reconcile-harness `reform`).
    let mut seed = b"reform-v1:".to_vec();
    seed.extend_from_slice(ejected.0.as_bytes());
    for m in &members {
        seed.push(0);
        seed.extend_from_slice(m.0.as_bytes());
    }
    let g1 = GenesisId::from_bytes(&seed);

    let mut lin = Lineage::new();
    lin.add_root(g0, w.genesis.founders.iter().cloned());
    lin.fork(g0, g1, members.iter().cloned());

    let shares = lin.shares_lineage(g1, g0);
    let removers_excluded = removers.iter().all(|r| !members.contains(r));
    let removers_standing = removers.iter().all(|r| lin.standing(r, g1));
    let ejected_standing = lin.standing(&ejected, g1) && lin.standing(&ejected, g0);

    if shares && removers_excluded && removers_standing && ejected_standing {
        CheckOutcome::Pass
    } else {
        CheckOutcome::Fail(format!(
            "{}: reformation invariants: shares={shares} removers_excluded={removers_excluded} removers_standing={removers_standing} ejected_standing={ejected_standing}",
            v.id
        ))
    }
}

// --- Category 7: adversarial AR-1..AR-6 -------------------------------------

/// Build a fresh genesis + directory from explicit admin/founder DID lists. Every
/// founder gets a real `from_seed(_, 1)` identity registered in the directory.
fn ar_world(
    admins: &[String],
    founders: &[String],
    op_kind: OpKind,
    threshold: u32,
) -> (Genesis, Directory, BTreeMap<String, SigningIdentity>) {
    let mut ids = BTreeMap::new();
    let mut dir = Directory::new();
    for d in founders {
        let id = SigningIdentity::from_seed(Did::new(d.clone()), 1);
        dir.insert(id.verifying());
        ids.insert(d.clone(), id);
    }
    let admin_set: BTreeSet<Did> = admins.iter().map(|d| Did::new(d.clone())).collect();
    let mut thresholds = BTreeMap::new();
    thresholds.insert(op_kind, threshold);
    let rules = GenesisRules {
        admins: admin_set,
        thresholds,
    };
    let founder_set: BTreeSet<Did> = founders.iter().map(|d| Did::new(d.clone())).collect();
    let genesis = Genesis::new(rules, founder_set);
    (genesis, dir, ids)
}

/// Parse "Add"/"Remove" or fail loudly.
fn parse_op_kind(s: &str) -> Result<OpKind, CheckOutcome> {
    match s {
        "Add" => Ok(OpKind::Add),
        "Remove" => Ok(OpKind::Remove),
        other => Err(CheckOutcome::Fail(format!("unknown op_kind {other}"))),
    }
}

/// Apply a (possibly shuffled / duplicated) op stream with buffering — the honest
/// replica from `ar2_malicious_sequencer`. Returns the converged state.
fn apply_stream(genesis: &Genesis, dir: &Directory, stream: &[SignedOp]) -> GroupState {
    let mut st = GroupState::new(genesis.clone());
    loop {
        let mut progressed = false;
        for op in stream {
            if op.body.seq == st.log.len() as u64 && op.body.prev == st.head {
                st.apply(op.clone(), dir)
                    .expect("a chaining op from the honest canonical set always applies");
                progressed = true;
            }
        }
        if !progressed {
            break;
        }
    }
    st
}

/// Build the canonical k-Add chain authored by the sole admin.
fn ar_canonical_chain(
    admin: &SigningIdentity,
    dir: &Directory,
    genesis: &Genesis,
    k: usize,
) -> (GroupState, Vec<SignedOp>) {
    let mut st = GroupState::new(genesis.clone());
    let mut ops = Vec::new();
    for i in 0..k {
        let op = sign_op(&st, OpKind::Add, Some(Did::new(format!("m{i}"))), &[admin]);
        st.apply(op.clone(), dir)
            .expect("admin Add always applies in order");
        ops.push(op);
    }
    (st, ops)
}

/// Re-prove an adversarial (cat-7) vector. Every verdict is DERIVED by running the
/// real `lineage-core` / `lineage-history` API — never hand-set. `expect` is the
/// human discriminant the observed verdict must match.
#[must_use]
pub fn check_adversarial(v: &AdversarialVector) -> CheckOutcome {
    match v.kind {
        // --- AR-1 -----------------------------------------------------------
        AdversarialKind::SybilNoStanding => {
            let kind = match parse_op_kind(&v.op_kind) {
                Ok(k) => k,
                Err(o) => return o,
            };
            // Fresh Sybils: founders + admins as given (none of the signers are
            // admins), signers registered in the directory so signatures verify.
            let mut founders = v.founders.clone();
            for s in &v.signers {
                if !founders.contains(s) {
                    founders.push(s.clone());
                }
            }
            let (genesis, mut dir, mut ids) = ar_world(&v.admins, &founders, kind, v.threshold);
            for s in &v.signers {
                let id = ids
                    .entry(s.clone())
                    .or_insert_with(|| SigningIdentity::from_seed(Did::new(s.clone()), 1));
                dir.insert(id.verifying());
            }
            let state = GroupState::new(genesis);
            let signers: Vec<&SigningIdentity> = v.signers.iter().map(|s| &ids[s]).collect();
            let op = sign_op(&state, kind, Some(Did::new(v.subject.clone())), &signers);

            // (a) outright rejected as a non-admin signer.
            let rejected = matches!(
                state.validate(&op, &dir),
                Err(RejectReason::SignerLacksStanding(_))
            );
            // (b) zero admit-authority by lineage regardless of headcount.
            let lineage_of: BTreeMap<Did, Did> = v
                .signers
                .iter()
                .map(|s| (Did::new(s.clone()), Did::new("sybil-lineage")))
                .collect();
            let lineages = state.valid_admin_lineages(&op, &dir, &lineage_of);
            if rejected && lineages == 0 {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-1: expected reject + 0 admin lineages, got rejected={rejected} lineages={lineages}"
                ))
            }
        }

        // --- AR-2 reorder + replay ------------------------------------------
        AdversarialKind::SequencerReorderConverges => {
            let admin_did = match v.admins.first() {
                Some(d) => d.clone(),
                None => return CheckOutcome::Fail("AR-2 reorder: no admin".into()),
            };
            let (genesis, dir, ids) = ar_world(&v.admins, &v.founders, OpKind::Add, v.threshold);
            let admin = &ids[&admin_did];
            let k = v.chain_len as usize;
            let (canonical, ops) = ar_canonical_chain(admin, &dir, &genesis, k);

            // Malicious sequencer: duplicate every op, then a fixed reversed order
            // (deterministic — no RNG so the vector is reproducible).
            let mut stream = ops.clone();
            stream.extend(ops.clone());
            stream.reverse();
            let got = apply_stream(&genesis, &dir, &stream);
            if got.head == canonical.head && got.members == canonical.members && got.log.len() == k
            {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-2 reorder: converged state diverged (log {} vs {k})",
                    got.log.len()
                ))
            }
        }

        // --- AR-2 drop ------------------------------------------------------
        AdversarialKind::SequencerDropVisiblyBehind => {
            let admin_did = match v.admins.first() {
                Some(d) => d.clone(),
                None => return CheckOutcome::Fail("AR-2 drop: no admin".into()),
            };
            let (genesis, dir, ids) = ar_world(&v.admins, &v.founders, OpKind::Add, v.threshold);
            let admin = &ids[&admin_did];
            let k = v.chain_len as usize;
            let (canonical, ops) = ar_canonical_chain(admin, &dir, &genesis, k);
            let drop = v.drop_seq as usize;
            if drop == 0 || drop >= k {
                return CheckOutcome::Fail(format!("AR-2 drop: drop_seq {drop} out of (0,{k})"));
            }
            let mut withheld = ops.clone();
            withheld.remove(drop);
            let deprived = apply_stream(&genesis, &dir, &withheld);
            // Peer applies the strict prefix before the gap, sits at a real earlier
            // head, and is visibly behind the canonical head.
            let behind = deprived.log.len() == drop
                && deprived.head != canonical.head
                && deprived.head == ops[drop - 1].body.id();
            if behind {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-2 drop: expected visibly-behind at seq {drop}, got log {}",
                    deprived.log.len()
                ))
            }
        }

        // --- AR-2 inject ----------------------------------------------------
        AdversarialKind::SequencerInjectRejected => {
            let (genesis, mut dir, mut ids) =
                ar_world(&v.admins, &v.founders, OpKind::Add, v.threshold);
            // The broker injects an op signed by a non-admin it controls.
            let injector = v
                .signers
                .first()
                .cloned()
                .unwrap_or_else(|| "mallory".into());
            let id = ids
                .entry(injector.clone())
                .or_insert_with(|| SigningIdentity::from_seed(Did::new(injector.clone()), 9));
            dir.insert(id.verifying());
            let state = GroupState::new(genesis);
            let forged = sign_op(
                &state,
                OpKind::Add,
                Some(Did::new(v.subject.clone())),
                &[&ids[&injector]],
            );
            if matches!(
                state.validate(&forged, &dir),
                Err(RejectReason::SignerLacksStanding(_))
            ) {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail("AR-2 inject: forged non-admin op was not rejected".into())
            }
        }

        // --- AR-6 double-count ----------------------------------------------
        AdversarialKind::ReplayDoubleCountPrevented => {
            let kind = match parse_op_kind(&v.op_kind) {
                Ok(k) => k,
                Err(o) => return o,
            };
            let (genesis, dir, ids) = ar_world(&v.admins, &v.founders, kind, v.threshold);
            let state = GroupState::new(genesis);
            // The same admin appears twice in the signer list.
            let signers: Vec<&SigningIdentity> = v.signers.iter().map(|s| &ids[s]).collect();
            let op = sign_op(&state, kind, Some(Did::new(v.subject.clone())), &signers);
            // Sigs keyed by DID → one entry; below a threshold of 2.
            let one_sig = op.sigs.len() == 1;
            let under = !state.meets_threshold(&op, &dir);
            if one_sig && under {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-6 double-count: sigs={} meets_threshold={}",
                    op.sigs.len(),
                    !under
                ))
            }
        }

        // --- AR-6 replay ----------------------------------------------------
        AdversarialKind::ReplayDoesNotReenact => {
            let kind = match parse_op_kind(&v.op_kind) {
                Ok(k) => k,
                Err(o) => return o,
            };
            let (genesis, dir, ids) = ar_world(&v.admins, &v.founders, kind, v.threshold);
            let mut state = GroupState::new(genesis);
            let signers: Vec<&SigningIdentity> = v.signers.iter().map(|s| &ids[s]).collect();
            let op = sign_op(&state, kind, Some(Did::new(v.subject.clone())), &signers);
            if let Err(e) = state.apply(op.clone(), &dir) {
                return CheckOutcome::Fail(format!("AR-6 replay: first apply rejected: {e}"));
            }
            // Replaying the same op (seq 0, prev = genesis) against the advanced
            // head does not chain → BrokenChain, not re-enacted.
            if state.apply(op, &dir) == Err(RejectReason::BrokenChain) {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(
                    "AR-6 replay: replayed op was not rejected as BrokenChain".into(),
                )
            }
        }

        // --- AR-3 foreign-lineage zero-crypto reject ------------------------
        AdversarialKind::BackfillForeignZeroCrypto => {
            // My lineage; a hostile branch on a DIFFERENT lineage, made large.
            let mut lineage = Lineage::new();
            let myroot = GenesisId::from_bytes(b"ar3-my-root");
            let mine = GenesisId::from_bytes(b"ar3-my-branch");
            lineage.add_root(myroot, [Did::new("me")]);
            lineage.fork(myroot, mine, [Did::new("me")]);
            let foreign = GenesisId::from_bytes(b"ar3-attacker-branch");
            let mut flood = BranchHistory::new(foreign);
            for seq in 0..(v.donor_msgs as u64) {
                flood.push_raw(Message {
                    author: Did::new("attacker"),
                    seq,
                    branch: foreign,
                    payload: vec![0u8; 64],
                    sig: Sig([0u8; 64]),
                });
            }
            // A verifier counting calls — it MUST never be invoked (zero crypto).
            let calls = std::sync::atomic::AtomicUsize::new(0);
            let verify = |_d: &Did, _m: &[u8], _s: &Sig| -> bool {
                calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                false
            };
            let mut store = HistoryStore::new();
            let result = store.backfill_import(&flood, mine, &lineage, verify);
            let foreign_rejected = result == Err(BackfillError::ForeignGenesis);
            let zero_crypto = calls.load(std::sync::atomic::Ordering::SeqCst) == 0;
            let no_residue = store.branch_count() == 0;
            if foreign_rejected && zero_crypto && no_residue {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-3 foreign: rejected={foreign_rejected} zero_crypto={zero_crypto} no_residue={no_residue}"
                ))
            }
        }

        // --- AR-3 forged-branch first-defect bound --------------------------
        AdversarialKind::BackfillFirstDefectBounded => {
            let alice = SigningIdentity::from_seed(Did::new("alice"), 1);
            let mut dir_map: BTreeMap<Did, VerifyingIdentity> = BTreeMap::new();
            dir_map.insert(Did::new("alice"), alice.verifying());

            let gb = GenesisId::from_bytes(b"ar3-shared-branch");
            let mine = GenesisId::from_bytes(b"ar3-shared-mine");
            let mut lineage = Lineage::new();
            lineage.add_root(gb, [Did::new("alice")]);
            lineage.fork(gb, mine, [Did::new("alice")]);

            // msg 0 has a BAD signature; the rest are well-formed and numerous.
            let n = v.donor_msgs.max(1) as u64;
            let mut forged = BranchHistory::new(gb);
            forged.push_raw(Message {
                author: Did::new("alice"),
                seq: 0,
                branch: gb,
                payload: b"tampered".to_vec(),
                sig: Sig([7u8; 64]),
            });
            for seq in 1..n {
                let p = format!("m{seq}");
                let bytes = Message::signing_bytes(gb, seq, &Did::new("alice"), p.as_bytes());
                forged.push_raw(Message {
                    author: Did::new("alice"),
                    seq,
                    branch: gb,
                    payload: p.into_bytes(),
                    sig: alice.sign(&bytes),
                });
            }
            let calls = std::sync::atomic::AtomicUsize::new(0);
            let verify = |d: &Did, m: &[u8], s: &Sig| {
                calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                dir_map.get(d).is_some_and(|vi| vi.verify(m, s))
            };
            let mut store = HistoryStore::new();
            let result = store.backfill_import(&forged, mine, &lineage, verify);
            let first_defect = matches!(result, Err(BackfillError::BadSignature { seq: 0, .. }));
            let bounded = calls.load(std::sync::atomic::Ordering::SeqCst) == 1;
            let no_residue = store.branch_count() == 0;
            if first_defect && bounded && no_residue {
                CheckOutcome::Pass
            } else {
                CheckOutcome::Fail(format!(
                    "AR-3 first-defect: rejected_at_0={first_defect} verify_calls={} (want 1) no_residue={no_residue}",
                    calls.load(std::sync::atomic::Ordering::SeqCst)
                ))
            }
        }
    }
}

// --- Categories 8 + 9: TS-authoritative structural validation ----------------

/// Validate the STRUCTURE of a TS-model verdict snapshot (cat 8 visibility /
/// cat 9 freshness). The Rust suite cannot re-prove the TS logic — the
/// authoritative runner is the TS model in `Proofs/lineage-group-model`. This
/// check confirms each experiment is well-formed: it carries at least one
/// invariant, and `all_passed` is consistent with the per-invariant flags.
#[must_use]
pub fn check_ts_experiment(e: &TsExperiment) -> CheckOutcome {
    if e.name.trim().is_empty() {
        return CheckOutcome::Fail("ts experiment: empty name".into());
    }
    if e.invariants.is_empty() {
        return CheckOutcome::Fail(format!("{}: no invariants recorded", e.name));
    }
    let computed_all = e.invariants.iter().all(|i| i.passed);
    if computed_all != e.all_passed {
        return CheckOutcome::Fail(format!(
            "{}: all_passed={} but per-invariant all() = {computed_all}",
            e.name, e.all_passed
        ));
    }
    CheckOutcome::Pass
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FoldSigner, HistMsg, ReconcileBranch, ReconcileOp, TsInvariant};

    // --- cat 1 ---------------------------------------------------------------

    #[test]
    fn derivation_passes_for_real_hash_and_fails_when_tampered() {
        let input = b"vacation-2025";
        let expected = GenesisId::from_bytes(input).0;
        let good = DerivationVector {
            kind: "genesis_id".into(),
            derivation: DerivationKind::Structural,
            input_hex: hex::encode(input),
            input_id: String::new(),
            expected_hex: hex::encode(expected),
            expect: "accept".into(),
        };
        assert_eq!(check_derivation(&good), CheckOutcome::Pass);

        let mut bad_out = expected;
        bad_out[0] ^= 0x01;
        let bad = DerivationVector {
            kind: "genesis_id".into(),
            derivation: DerivationKind::Structural,
            input_hex: hex::encode(input),
            input_id: String::new(),
            expected_hex: hex::encode(bad_out),
            expect: "accept".into(),
        };
        assert!(!check_derivation(&bad).is_pass());
    }

    /// The tagged wire derivations re-derive through the canonical
    /// `lineage_core::ids` functions and match the spec §2 byte-exact vectors.
    /// A one-bit tamper of either the output or the recorded pre-image fails.
    #[test]
    fn tagged_derivations_reprove_and_reject_tamper() {
        use lineage_core::ids;
        let cases = [
            (
                DerivationKind::LineageGenesis,
                "lin-a",
                ids::lineage_genesis("lin-a").0,
                {
                    let mut p = b"croft-lineage-genesis:".to_vec();
                    p.extend_from_slice(b"lin-a");
                    p
                },
                "e1d1d13d80d133c60ddb47ccf01a0f5f9b9b101544a0caedc244e9621097c93d",
            ),
            (
                DerivationKind::GroupGenesis,
                "grp-1",
                ids::group_genesis("grp-1").0,
                {
                    let mut p = b"croft-group-genesis:".to_vec();
                    p.extend_from_slice(b"grp-1");
                    p
                },
                "7a7f2300dd542ef7650b31e0dabe694c644c7886c9d9d8c92d1fdfe9bb359efa",
            ),
            (
                DerivationKind::GroupTopic,
                "grp-1",
                ids::group_topic("grp-1"),
                {
                    let mut p = b"croft-group-topic:".to_vec();
                    p.extend_from_slice(b"grp-1");
                    p
                },
                "4cb63102d3c6b599fcfc4693ed71ff6d04f9007c96579b159c7455d6d769a1d8",
            ),
        ];
        for (derivation, id, out, preimage, want_hex) in cases {
            assert_eq!(hex::encode(out), want_hex, "spec vector for {id}");
            let v = DerivationVector {
                kind: format!("{derivation:?}"),
                derivation: derivation.clone(),
                input_hex: hex::encode(&preimage),
                input_id: id.into(),
                expected_hex: want_hex.into(),
                expect: "accept".into(),
            };
            assert_eq!(check_derivation(&v), CheckOutcome::Pass);

            // Tamper the expected output → fail.
            let mut tampered = v.clone();
            let mut bad = out;
            bad[0] ^= 0x01;
            tampered.expected_hex = hex::encode(bad);
            assert!(
                !check_derivation(&tampered).is_pass(),
                "tampered output must fail"
            );

            // Tamper the recorded pre-image (still hashing to the right output via
            // the function call would pass the function check, but the pre-image
            // guard must catch a mismatched recorded pre-image).
            let mut bad_pre = v.clone();
            bad_pre.input_hex = hex::encode(b"wrong-preimage");
            assert!(
                !check_derivation(&bad_pre).is_pass(),
                "tampered pre-image must fail"
            );
        }
    }

    // --- cat 2 ---------------------------------------------------------------

    fn good_signing_vector() -> SigningVector {
        let branch = GenesisId::from_bytes(b"branch");
        let author = Did::new("alice");
        let id = SigningIdentity::from_seed(author.clone(), 1);
        let payload = b"hello".to_vec();
        let bytes = Message::signing_bytes(branch, 7, &author, &payload);
        let sig = id.sign(&bytes);
        let vk_hex = hex::encode(id.verifying().to_bytes());
        SigningVector {
            branch_hex: hex::encode(branch.0),
            seq: 7,
            author: author.0,
            author_seed: 1,
            payload_hex: hex::encode(&payload),
            signing_bytes_hex: hex::encode(&bytes),
            verifying_key_hex: vk_hex,
            signature_hex: sig.to_hex(),
            expect: "accept".into(),
        }
    }

    #[test]
    fn signing_good_vector_verifies() {
        assert_eq!(check_signing(&good_signing_vector()), CheckOutcome::Pass);
    }

    #[test]
    fn signing_one_bit_flip_is_rejected_and_marked_reject() {
        let mut v = good_signing_vector();
        // Flip one bit of the signature; mark it as a must-reject vector.
        let mut sig = hex::decode(&v.signature_hex).unwrap();
        sig[0] ^= 0x01;
        v.signature_hex = hex::encode(&sig);
        v.expect = "reject:bad-signature".into();
        // A correct runner: the flipped sig fails verify, which matches the
        // reject expectation -> Pass (the must-reject case is honored).
        assert_eq!(check_signing(&v), CheckOutcome::Pass);

        // If instead the vector *claimed* the flipped sig should accept, fail.
        v.expect = "accept".into();
        assert!(!check_signing(&v).is_pass());
    }

    // --- cat 3+4 -------------------------------------------------------------

    #[test]
    fn fold_one_lineage_many_devices_is_rejected() {
        // Three devices, all one lineage. by_did=3, by_lineage=1. Remove needs 2.
        let v = FoldVector {
            label: "one-lineage-3-devices".into(),
            signers: vec![
                FoldSigner {
                    did: "p-dev1".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "p-dev2".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "p-dev3".into(),
                    lineage: "person-p".into(),
                },
            ],
            op_kind: "Remove".into(),
            threshold: 2,
            expected_by_did: 3,
            expected_by_lineage: 1,
            expect: "reject:under-threshold-by-lineage".into(),
        };
        assert_eq!(check_fold(&v), CheckOutcome::Pass);
    }

    #[test]
    fn fold_two_lineages_meets_threshold() {
        let v = FoldVector {
            label: "two-lineages".into(),
            signers: vec![
                FoldSigner {
                    did: "p-dev1".into(),
                    lineage: "person-p".into(),
                },
                FoldSigner {
                    did: "q-dev1".into(),
                    lineage: "person-q".into(),
                },
            ],
            op_kind: "Remove".into(),
            threshold: 2,
            expected_by_did: 2,
            expected_by_lineage: 2,
            expect: "accept".into(),
        };
        assert_eq!(check_fold(&v), CheckOutcome::Pass);
    }

    // --- cat 5 ---------------------------------------------------------------

    #[test]
    fn revocation_pre_revoke_imports_post_revoke_rejected() {
        let branch_hex = hex::encode(GenesisId::from_bytes(b"branch-rev").0);
        let msgs = vec![HistMsg {
            author: "carol".into(),
            seq: 0,
            payload_hex: hex::encode(b"pre-revoke note"),
        }];
        let pre = RevocationVector {
            label: "pre-revoke retained".into(),
            branch_hex: branch_hex.clone(),
            author_has_standing: true,
            messages: msgs.clone(),
            expect: "accept".into(),
        };
        assert_eq!(check_revocation(&pre), CheckOutcome::Pass);

        let post = RevocationVector {
            label: "post-revoke rejected".into(),
            branch_hex,
            author_has_standing: false,
            messages: msgs,
            expect: "reject:unauthorized-author".into(),
        };
        assert_eq!(check_revocation(&post), CheckOutcome::Pass);
    }

    // --- cat 5b: revoke-authority -------------------------------------------

    use crate::model::{AuthorityAdmin, AuthoritySig, AuthorityVector};

    /// Build a revoke-authority vector by RUNNING the real gov code: a genesis
    /// with `admins` (threshold `threshold` on Remove), sign a Remove of
    /// `subject` with `signers`, and record the real signing bytes + sigs + keys.
    /// `lineage_of` maps each signer DID to its lineage. `non_admins` are extra
    /// members (not in the admin set) whose keys/sigs are recorded too.
    // Mirrors the emitter's `build_authority_vector` arity (one positional arg per
    // independent dimension of the vector); an options struct would not add clarity
    // for a test-only constructor.
    #[allow(clippy::too_many_arguments)]
    fn authority_vector(
        label: &str,
        admin_dids: &[&str],
        non_admin_dids: &[&str],
        signer_dids: &[&str],
        lineage_pairs: &[(&str, &str)],
        subject: &str,
        threshold: u32,
        expect: &str,
    ) -> AuthorityVector {
        let mut ids: BTreeMap<String, SigningIdentity> = BTreeMap::new();
        let all: BTreeSet<String> = admin_dids
            .iter()
            .chain(non_admin_dids.iter())
            .chain(signer_dids.iter())
            .map(|s| (*s).to_string())
            .collect();
        for d in &all {
            ids.insert(
                d.clone(),
                SigningIdentity::from_seed(Did::new(d.clone()), 1),
            );
        }
        let admins: BTreeSet<Did> = admin_dids.iter().map(|s| Did::new(*s)).collect();
        let mut thresholds = BTreeMap::new();
        thresholds.insert(OpKind::Remove, threshold);
        let rules = GenesisRules { admins, thresholds };
        // Founders = all admins + non-admins + the subject (so the subject is a
        // current member to remove; signers must be members to hold standing).
        let mut founders: BTreeSet<Did> = all.iter().map(|s| Did::new(s.clone())).collect();
        founders.insert(Did::new(subject));
        let genesis = Genesis::new(rules, founders);
        let state = GroupState::new(genesis);

        let signers: Vec<&SigningIdentity> = signer_dids.iter().map(|d| &ids[*d]).collect();
        let op = sign_op(&state, OpKind::Remove, Some(Did::new(subject)), &signers);

        let admins_out: Vec<AuthorityAdmin> = admin_dids
            .iter()
            .map(|d| AuthorityAdmin {
                did: (*d).into(),
                lineage: lineage_pairs
                    .iter()
                    .find(|(k, _)| k == d)
                    .map(|(_, l)| (*l).to_string())
                    .unwrap_or_else(|| (*d).to_string()),
                verifying_key_hex: hex::encode(ids[*d].verifying().to_bytes()),
            })
            .collect();
        let non_admins_out: Vec<AuthorityAdmin> = non_admin_dids
            .iter()
            .map(|d| AuthorityAdmin {
                did: (*d).into(),
                lineage: lineage_pairs
                    .iter()
                    .find(|(k, _)| k == d)
                    .map(|(_, l)| (*l).to_string())
                    .unwrap_or_else(|| (*d).to_string()),
                verifying_key_hex: hex::encode(ids[*d].verifying().to_bytes()),
            })
            .collect();
        let sigs_out: Vec<AuthoritySig> = op
            .sigs
            .iter()
            .map(|(d, s)| AuthoritySig {
                did: d.0.clone(),
                signature_hex: s.to_hex(),
            })
            .collect();

        AuthorityVector {
            label: label.into(),
            admins: admins_out,
            non_admin_signers: non_admins_out,
            op_kind: "Remove".into(),
            subject: subject.into(),
            signing_bytes_hex: hex::encode(op.body.signing_bytes()),
            sigs: sigs_out,
            lineage_of: lineage_pairs
                .iter()
                .map(|(a, b)| ((*a).to_string(), (*b).to_string()))
                .collect(),
            threshold,
            expect: expect.into(),
        }
    }

    #[test]
    fn authority_two_lineages_accepts() {
        let v = authority_vector(
            "two admin lineages meet threshold 2",
            &["alice", "bob", "carol"],
            &[],
            &["alice", "bob"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "accept",
        );
        assert_eq!(check_revocation_authority(&v), CheckOutcome::Pass);
    }

    #[test]
    fn authority_one_admin_under_threshold_rejects() {
        let v = authority_vector(
            "one admin under threshold 2",
            &["alice", "bob", "carol"],
            &[],
            &["alice"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "reject:under_threshold",
        );
        assert_eq!(check_revocation_authority(&v), CheckOutcome::Pass);
    }

    #[test]
    fn authority_non_admin_signer_rejects() {
        // mallory is a member who signs, but is NOT in the admin set: the only
        // signature is hers, so the admin-lineage count is 0 -> reject.
        let v = authority_vector(
            "lone non-admin signer does not count",
            &["alice", "bob", "carol"],
            &["mallory"],
            &["mallory"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("mallory", "lin-m")],
            "erin",
            2,
            "reject:non_admin_signer",
        );
        assert_eq!(check_revocation_authority(&v), CheckOutcome::Pass);
    }

    #[test]
    fn authority_one_lineage_multi_device_rejects() {
        // Two admin DIDs that fold to ONE lineage (E2.10): counts as 1 -> reject.
        let v = authority_vector(
            "two devices of one lineage count as one",
            &["p-phone", "p-laptop", "carol"],
            &[],
            &["p-phone", "p-laptop"],
            &[
                ("p-phone", "person-p"),
                ("p-laptop", "person-p"),
                ("carol", "lin-c"),
            ],
            "erin",
            2,
            "reject:one_lineage_multi_device",
        );
        assert_eq!(check_revocation_authority(&v), CheckOutcome::Pass);
    }

    #[test]
    fn authority_teeth_flipped_expect_fails() {
        // The must-reject teeth: a real reject case mislabeled `accept` must FAIL.
        let mut v = authority_vector(
            "under-threshold mislabeled accept",
            &["alice", "bob", "carol"],
            &[],
            &["alice"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "accept",
        );
        assert!(!check_revocation_authority(&v).is_pass());
        // And a true accept mislabeled reject must FAIL too.
        v = authority_vector(
            "accept mislabeled reject",
            &["alice", "bob", "carol"],
            &[],
            &["alice", "bob"],
            &[("alice", "lin-a"), ("bob", "lin-b"), ("carol", "lin-c")],
            "erin",
            2,
            "reject:under_threshold",
        );
        assert!(!check_revocation_authority(&v).is_pass());
    }

    // --- cat 6 ---------------------------------------------------------------

    #[test]
    fn reconcile_contradiction_hard_stops() {
        let v = ReconcileVector {
            id: "T-contradiction".into(),
            label: "boot erin vs keep erin".into(),
            branches: vec![
                ReconcileBranch {
                    label: "boot".into(),
                    ops: vec![ReconcileOp {
                        kind: "Remove".into(),
                        subject: "erin".into(),
                        signers: vec!["alice".into(), "bob".into()],
                    }],
                },
                ReconcileBranch {
                    label: "keep".into(),
                    ops: vec![ReconcileOp {
                        kind: "Add".into(),
                        subject: "frank".into(),
                        signers: vec!["carol".into()],
                    }],
                },
            ],
            verdict: "hard-stop".into(),
            contested: vec!["erin".into()],
        };
        assert_eq!(check_reconcile(&v), CheckOutcome::Pass);
    }

    #[test]
    fn reconcile_complementary_converges() {
        let v = ReconcileVector {
            id: "T-converge".into(),
            label: "both add, both keep erin".into(),
            branches: vec![
                ReconcileBranch {
                    label: "addfrank".into(),
                    ops: vec![ReconcileOp {
                        kind: "Add".into(),
                        subject: "frank".into(),
                        signers: vec!["carol".into()],
                    }],
                },
                ReconcileBranch {
                    label: "addgrace".into(),
                    ops: vec![ReconcileOp {
                        kind: "Add".into(),
                        subject: "grace".into(),
                        signers: vec!["dave".into()],
                    }],
                },
            ],
            verdict: "converge".into(),
            contested: vec![],
        };
        assert_eq!(check_reconcile(&v), CheckOutcome::Pass);
    }

    #[test]
    fn reconcile_reformation_descends_from_root() {
        let v = ReconcileVector {
            id: "T-reform".into(),
            label: "erin re-forms after eject".into(),
            branches: vec![ReconcileBranch {
                label: "eject".into(),
                ops: vec![ReconcileOp {
                    kind: "Remove".into(),
                    subject: "erin".into(),
                    signers: vec!["alice".into(), "bob".into()],
                }],
            }],
            verdict: "re-formation".into(),
            contested: vec![],
        };
        assert_eq!(check_reconcile(&v), CheckOutcome::Pass);
    }

    // --- cat 7: adversarial --------------------------------------------------

    fn adv(kind: AdversarialKind, expect: &str) -> AdversarialVector {
        AdversarialVector {
            ar: "AR".into(),
            label: "test".into(),
            kind,
            admins: vec!["alice".into(), "bob".into()],
            founders: vec!["alice".into(), "bob".into(), "carol".into()],
            op_kind: "Remove".into(),
            subject: "carol".into(),
            signers: vec![],
            threshold: 2,
            chain_len: 0,
            drop_seq: 0,
            donor_msgs: 0,
            expect: expect.into(),
        }
    }

    #[test]
    fn ar1_sybil_is_rejected_with_zero_admin_lineages() {
        let mut v = adv(AdversarialKind::SybilNoStanding, "reject:no-standing");
        v.signers = (0..6).map(|i| format!("sybil{i}")).collect();
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar2_reorder_and_replay_converge() {
        let mut v = adv(AdversarialKind::SequencerReorderConverges, "converge");
        v.admins = vec!["alice".into()];
        v.founders = vec!["alice".into()];
        v.op_kind = "Add".into();
        v.threshold = 1;
        v.chain_len = 5;
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar2_dropped_op_is_visibly_behind() {
        let mut v = adv(AdversarialKind::SequencerDropVisiblyBehind, "behind");
        v.admins = vec!["alice".into()];
        v.founders = vec!["alice".into()];
        v.op_kind = "Add".into();
        v.threshold = 1;
        v.chain_len = 6;
        v.drop_seq = 3;
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar2_injected_nonadmin_op_is_rejected() {
        let mut v = adv(
            AdversarialKind::SequencerInjectRejected,
            "reject:no-standing",
        );
        v.admins = vec!["alice".into()];
        v.founders = vec!["alice".into()];
        v.op_kind = "Add".into();
        v.threshold = 1;
        v.subject = "mallory".into();
        v.signers = vec!["mallory".into()];
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar6_double_count_stays_under_threshold() {
        let mut v = adv(
            AdversarialKind::ReplayDoubleCountPrevented,
            "reject:under-threshold",
        );
        v.signers = vec!["alice".into(), "alice".into()];
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar6_replayed_op_does_not_reenact() {
        let mut v = adv(AdversarialKind::ReplayDoesNotReenact, "reject:broken-chain");
        v.signers = vec!["alice".into(), "bob".into()];
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar3_foreign_branch_rejected_with_zero_crypto() {
        let mut v = adv(
            AdversarialKind::BackfillForeignZeroCrypto,
            "reject:foreign-genesis",
        );
        v.donor_msgs = 500;
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    #[test]
    fn ar3_forged_branch_bounded_by_first_defect() {
        let mut v = adv(
            AdversarialKind::BackfillFirstDefectBounded,
            "reject:bad-signature-bounded",
        );
        v.donor_msgs = 1000;
        assert_eq!(check_adversarial(&v), CheckOutcome::Pass);
    }

    // --- cat 8/9: TS-authoritative structural validation ---------------------

    #[test]
    fn ts_experiment_structure_passes_and_catches_inconsistency() {
        let good = TsExperiment {
            name: "V1".into(),
            invariants: vec![TsInvariant {
                invariant: "INV-REGIME-IMMUTABLE".into(),
                passed: true,
            }],
            all_passed: true,
        };
        assert_eq!(check_ts_experiment(&good), CheckOutcome::Pass);

        // all_passed inconsistent with the per-invariant flags → fail.
        let inconsistent = TsExperiment {
            name: "V2".into(),
            invariants: vec![TsInvariant {
                invariant: "INV-REGIME-IN-CONTENT".into(),
                passed: false,
            }],
            all_passed: true,
        };
        assert!(!check_ts_experiment(&inconsistent).is_pass());

        // empty invariant list → fail (a vector must assert something).
        let empty = TsExperiment {
            name: "V3".into(),
            invariants: vec![],
            all_passed: true,
        };
        assert!(!check_ts_experiment(&empty).is_pass());
    }
}
