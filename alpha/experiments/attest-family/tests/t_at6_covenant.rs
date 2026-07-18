//! Part 6 (reuse half) — T-AT6.4 `covenant_is_r7_rule`.
//!
//! The no-monetization covenant is modeled as a **governed rule** on the
//! EXISTING R7 content-bound quorum machinery — `local_storage_projection`'s
//! fold, reused UNCHANGED (reuse is a condition of considered compatibility;
//! nothing here reimplements or shadows the substrate). Asserts:
//!   (a) weakening the covenant requires quorum met on the canonical payload
//!       hash (an approval naming different content does not authorize);
//!   (b) an under-quorum change is pending (refused whole), never partial;
//!   (c) the admitted change and its approval antecedents are visible in the
//!       exported lineage;
//! plus one test confirming two concurrent quorum-met changes inherit the
//! substrate's §7.6.1 contradiction hard-stop unchanged.
//!
//! The covenant register is a DECLARED STAND-IN: the substrate's rule_key 1
//! (`remove_member_threshold` register) reinterpreted as "covenant strength"
//! (3 = full covenant; lowering = weakening). What is under test is the
//! machinery — content-bound approvals, thresholds, contradiction — not the
//! register's name.
//!
//! Envelope-building helpers are copied from `croft-chat/tests/common/mod.rs`
//! (test scaffolding; the machinery itself is imported, not copied).

use std::sync::Arc;

use local_storage_projection::fold_derived::{rule_change_approval_subject, DerivedFold};
use local_storage_projection::tables::Db;
use local_storage_projection::traits::Signer as _;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};
use social_graph_core::{Ed25519Signer, Ed25519Verifier, Identity, RegistryCredentialResolver, Session};

// --- scaffolding copied from croft-chat/tests/common/mod.rs ----------------

fn sign(identity: &Identity, mut env: AssertionEnvelope) -> AssertionEnvelope {
    let canonical = env.canonical_bytes();
    env.signature = Ed25519Signer::new(identity).sign(&canonical);
    env
}

fn base(
    identity: &Identity,
    group: GroupId,
    ty: AssertionType,
    lamport: u64,
    antecedents: Vec<Hash>,
    payload: Vec<u8>,
) -> AssertionEnvelope {
    AssertionEnvelope {
        version: 0x01,
        assertion_type: ty,
        author_device: DeviceId::new(identity.device_id().0),
        author_principal: PrincipalId::new(identity.principal_id().0),
        group,
        antecedents,
        lamport,
        timestamp: 1_700_000_000 + lamport,
        payload,
        signature: vec![],
    }
}

fn genesis_payload(device: &DeviceId) -> Vec<u8> {
    let mut p = Vec::with_capacity(50);
    p.extend_from_slice(&1u16.to_be_bytes());
    for _ in 0..4 {
        p.extend_from_slice(&1u32.to_be_bytes());
    }
    p.extend_from_slice(device.as_bytes());
    p
}

fn membership_add_payload(subject: PrincipalId, role_byte: u8) -> Vec<u8> {
    let mut p = Vec::with_capacity(33);
    p.extend_from_slice(subject.as_bytes());
    p.push(role_byte);
    p
}

fn rule_change_payload(rule_key: u8, new_value: u32) -> Vec<u8> {
    let mut p = Vec::with_capacity(5);
    p.push(rule_key);
    p.extend_from_slice(&new_value.to_be_bytes());
    p
}

fn approval_payload(act_type: AssertionType, subject: PrincipalId) -> Vec<u8> {
    let mut p = act_type.to_u16().to_be_bytes().to_vec();
    p.extend_from_slice(subject.as_bytes());
    p
}

fn rc_subject(payload: &[u8]) -> PrincipalId {
    PrincipalId::new(rule_change_approval_subject(payload))
}

fn min_hash(a: Hash, b: Hash) -> Hash {
    if a.as_bytes() <= b.as_bytes() {
        a
    } else {
        b
    }
}

// --- the covenant fixture ---------------------------------------------------

/// The covenant register: substrate rule_key 1, reused as covenant strength.
const COVENANT_KEY: u8 = 1;
const COVENANT_FULL: u32 = 3;

struct Cast {
    o1: Identity,
    o2: Identity,
    a: Identity,
    b: Identity,
    group: GroupId,
}

fn cast() -> Cast {
    Cast {
        o1: Identity::from_seed([0xF0; 32]),
        o2: Identity::from_seed([0xF1; 32]),
        a: Identity::from_seed([0xF2; 32]),
        b: Identity::from_seed([0xF3; 32]),
        group: GroupId::new([0xCE; 32]),
    }
}

/// The covenant modeled as an R7 rule: genesis, two Owners, two Admins, the
/// RuleChange threshold raised to 2 (the R7 gate — THE modeling step), and
/// the covenant register set to full strength under that gate. Returns the
/// setup envelopes; the caller ingests them in order.
fn covenant_setup(c: &Cast) -> (Vec<AssertionEnvelope>, Hash) {
    let o1_device = DeviceId::new(c.o1.device_id().0);
    let genesis = sign(&c.o1, base(&c.o1, c.group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)));
    let add_o2 = sign(&c.o1, base(&c.o1, c.group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(PrincipalId::new(c.o2.principal_id().0), 0)));
    let add_a = sign(&c.o1, base(&c.o1, c.group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(PrincipalId::new(c.a.principal_id().0), 1)));
    let add_b = sign(&c.o1, base(&c.o1, c.group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(PrincipalId::new(c.b.principal_id().0), 1)));
    // The R7 modeling step: rule changes themselves now need a 2-quorum.
    // (The RED stage of this test omitted this raise — an ungoverned covenant
    // register, weakenable by a lone owner — and the tests failed against it.)
    let raise = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 5, vec![], rule_change_payload(3, 2)));
    let raise_h = envelope_hash(&raise);
    // Set the covenant register to full strength, under the 2-quorum gate:
    // Admin A approves the specific content, O1 enacts citing the approval.
    let set_payload = rule_change_payload(COVENANT_KEY, COVENANT_FULL);
    let appr_set = sign(&c.a, base(&c.a, c.group, AssertionType::Approval, 1, vec![raise_h], approval_payload(AssertionType::RuleChange, rc_subject(&set_payload))));
    let set_full = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 6, vec![raise_h, envelope_hash(&appr_set)], set_payload));
    (vec![genesis, add_o2, add_a, add_b, raise, appr_set, set_full], raise_h)
}

/// Open a fresh substrate fold, ingest `setup`, then run `attempts` and
/// return each attempt's outcome. The fold is the REUSED machinery.
fn run_fold(
    path: &std::path::Path,
    authors: &[&Identity],
    setup: &[AssertionEnvelope],
    attempts: &[&AssertionEnvelope],
) -> Vec<Result<(), String>> {
    let db = Arc::new(Db::open(path).expect("open db"));
    let resolver = RegistryCredentialResolver::new();
    for a in authors {
        resolver.register(a.device_id(), a.principal_id());
    }
    let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
    for env in setup {
        fold.ingest(env).unwrap_or_else(|e| panic!("setup ingest failed: {e}"));
    }
    attempts
        .iter()
        .map(|env| fold.ingest(env).map(|_| ()).map_err(|e| e.to_string()))
        .collect()
}

// ---------------------------------------------------------------------------
// (a) + (b) + (c): content-bound quorum, pending-never-partial, lineage
// ---------------------------------------------------------------------------

#[test]
fn covenant_weakening_requires_content_bound_quorum() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, _raise_h) = covenant_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];
    // Every attempt causally FOLLOWS the covenant's establishment (cites it),
    // so a successful weakening is a supersede in lineage, not a concurrent
    // competitor (that case is the second test).
    let set_full_h = envelope_hash(&setup[6]);

    let weaken_payload = rule_change_payload(COVENANT_KEY, 1);
    let other_payload = rule_change_payload(COVENANT_KEY, 2);

    // (b) A lone weakening — no approvals. Under the R7 gate this is
    // under-quorum: refused whole, never partially applied.
    let lone = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![set_full_h], weaken_payload.clone()));

    // (a) A weakening citing an approval for DIFFERENT content (covenant → 2,
    // not → 1). Content-bound means this approval cannot authorize it.
    let appr_wrong = sign(&c.b, base(&c.b, c.group, AssertionType::Approval, 1, vec![set_full_h], approval_payload(AssertionType::RuleChange, rc_subject(&other_payload))));
    let mismatched = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![set_full_h, envelope_hash(&appr_wrong)], weaken_payload.clone()));

    // The proper weakening: Admin A approves THIS content; O1 enacts citing it.
    let appr_right = sign(&c.a, base(&c.a, c.group, AssertionType::Approval, 2, vec![set_full_h], approval_payload(AssertionType::RuleChange, rc_subject(&weaken_payload))));
    let weaken = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![set_full_h, envelope_hash(&appr_right)], weaken_payload.clone()));

    let path = dir.path().join("covenant.redb");
    let outcomes = run_fold(
        &path,
        &authors,
        &setup,
        &[&lone, &appr_wrong, &mismatched, &appr_right, &weaken],
    );

    assert!(outcomes[0].is_err(), "(b) under-quorum weakening must be refused whole: {:?}", outcomes[0]);
    assert!(outcomes[1].is_ok(), "the wrong-content approval itself is a valid fact");
    assert!(
        outcomes[2].is_err(),
        "(a) an approval naming different content must not authorize the weakening: {:?}",
        outcomes[2]
    );
    assert!(outcomes[3].is_ok(), "the right-content approval lands");
    assert!(outcomes[4].is_ok(), "(a) quorum met on the canonical payload hash admits the weakening: {:?}", outcomes[4]);

    // Read back through the real Session surface: the register moved 3 → 1
    // exactly once, and the lineage shows the change WITH its approval
    // antecedents (c).
    let session = Session::open(&path, &c.o1).expect("open session");
    let summary = session.get_group_summary(&c.group).expect("summary");
    assert_eq!(summary.rules.remove_member_threshold, 1, "the admitted weakening applied");
    assert!(!summary.fork_status.starts_with("contradiction"), "clean fold");

    let log = session.export_group_log(&c.group).expect("export lineage");
    let frame_of = |env: &AssertionEnvelope| {
        let mut b = vec![0x01];
        b.extend_from_slice(&env.canonical_bytes_with_sig());
        b
    };
    assert!(
        log.contains(&frame_of(&weaken)),
        "(c) the admitted covenant change is visible in lineage"
    );
    assert!(
        log.contains(&frame_of(&appr_right)),
        "(c) its approval antecedent is visible in lineage"
    );
    assert!(
        weaken.antecedents.contains(&envelope_hash(&appr_right)),
        "(c) the change names its approval as antecedent (the R7 shape)"
    );
    // Refused acts left no partial trace in the lineage (pending ≠ partial).
    assert!(!log.contains(&frame_of(&lone)));
    assert!(!log.contains(&frame_of(&mismatched)));
}

// ---------------------------------------------------------------------------
// Inheritance: concurrent quorum-met covenant changes hard-stop (§7.6.1)
// ---------------------------------------------------------------------------

#[test]
fn covenant_inherits_contradiction_hard_stop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, _raise_h) = covenant_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];
    let set_full_h = envelope_hash(&setup[6]);

    // Two CONCURRENT quorum-met changes to the covenant register: O1+A set it
    // to 5, O2+B set it to 9. Neither cites the other.
    let p5 = rule_change_payload(COVENANT_KEY, 5);
    let p9 = rule_change_payload(COVENANT_KEY, 9);
    let appr_a = sign(&c.a, base(&c.a, c.group, AssertionType::Approval, 2, vec![set_full_h], approval_payload(AssertionType::RuleChange, rc_subject(&p5))));
    let appr_b = sign(&c.b, base(&c.b, c.group, AssertionType::Approval, 1, vec![set_full_h], approval_payload(AssertionType::RuleChange, rc_subject(&p9))));
    let change5 = sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![set_full_h, envelope_hash(&appr_a)], p5));
    let change9 = sign(&c.o2, base(&c.o2, c.group, AssertionType::RuleChange, 1, vec![set_full_h, envelope_hash(&appr_b)], p9));

    let expected_head = min_hash(envelope_hash(&change5), envelope_hash(&change9));
    let expected_status = format!("contradiction:{expected_head}");

    // Both arrival orders.
    for (name, order) in [
        ("order1", vec![&appr_a, &appr_b, &change5, &change9]),
        ("order2", vec![&appr_b, &appr_a, &change9, &change5]),
    ] {
        let path = dir.path().join(format!("{name}.redb"));
        let _ = run_fold(&path, &authors, &setup, &order);
        let session = Session::open(&path, &c.o1).expect("open session");
        let summary = session.get_group_summary(&c.group).expect("summary");
        assert_eq!(
            summary.fork_status, expected_status,
            "{name}: the attest covenant INHERITS the substrate contradiction hard-stop, \
             byte-head min(H(F),H(G)) — not a shadowed reimplementation"
        );
        assert_eq!(
            summary.rules.remove_member_threshold, COVENANT_FULL,
            "{name}: neither contested value applies; the covenant keeps its pre-conflict strength"
        );
    }
}
