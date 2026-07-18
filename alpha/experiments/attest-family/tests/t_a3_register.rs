//! RUN-ATTEST-03 Part A.1, register half: T-A3.4 `antecedent_register_governed`.
//!
//! The qualifying-antecedent-kind list (V1's closed class) is held as a
//! **governed register on the EXISTING R7 content-bound quorum machinery**
//! (`local_storage_projection` + `social-graph-core`, imported unchanged,
//! exactly as T-AT6.4 and T-PA3.2 drive them). Widening the class later is a
//! quorum act with lineage, not a code edit; the crate's fold only MIRRORS
//! the folded value (`AntecedentRegister`), it never governs it.
//!
//! Register stand-in (declared): substrate rule_key 2 (`role_change_threshold`)
//! reinterpreted as the **qualifying-kind bitmask** — bit 0 co_signed_edge,
//! bit 1 transaction, bit 2 ceremony; 7 = the full V1 class. (rule_key 0
//! remains the anchor dial, rule_key 1 the covenant register.) The substrate
//! genesis default of 1 is exactly the pre-V1 edge-only posture — V1's
//! establishment is itself a quorum act in this modeling. What is under test
//! is the machinery — content-bound approvals, thresholds, contradiction
//! hard-stop — not the register's name.
//!
//! Envelope-building scaffolding is copied from
//! `croft-chat/tests/common/mod.rs` via `t_at6_covenant.rs` (scaffolding
//! copied, machinery imported — nothing shadowed).

mod common;

use std::sync::Arc;

use local_storage_projection::fold_derived::{rule_change_approval_subject, DerivedFold};
use local_storage_projection::tables::Db;
use local_storage_projection::traits::Signer as _;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};
use social_graph_core::{
    Ed25519Signer, Ed25519Verifier, Identity, RegistryCredentialResolver, Session,
};

use attest_family::fixtures::*;
use attest_family::fold::{AntecedentRegister, VouchStatus};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

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

// --- the governed-register fixture ------------------------------------------

/// The antecedent-class register: substrate rule_key 2 (`role_change_threshold`),
/// reused as the qualifying-kind bitmask (declared stand-in).
const ANTECEDENT_REGISTER_KEY: u8 = 2;
/// The full V1 class: co_signed_edge | transaction | ceremony.
const FULL_CLASS: u32 = 0b111;
/// The pre-V1 posture: edge-only (also the substrate genesis default).
const EDGE_ONLY: u32 = 0b001;

struct Cast {
    o1: Identity,
    o2: Identity,
    a: Identity,
    b: Identity,
    group: GroupId,
}

fn cast() -> Cast {
    Cast {
        o1: Identity::from_seed([0xD0; 32]),
        o2: Identity::from_seed([0xD1; 32]),
        a: Identity::from_seed([0xD2; 32]),
        b: Identity::from_seed([0xD3; 32]),
        group: GroupId::new([0xA3; 32]),
    }
}

/// Genesis, two Owners, two Admins, RuleChange threshold raised to 2 (the R7
/// gate), then the register ESTABLISHED at the full V1 class under that gate
/// (quorum-met, causally chained — the F-AT-5 practice). Returns the setup
/// envelopes + the establishment hash.
fn register_setup(c: &Cast) -> (Vec<AssertionEnvelope>, Hash) {
    let o1_device = DeviceId::new(c.o1.device_id().0);
    let genesis = sign(
        &c.o1,
        base(&c.o1, c.group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o1_device)),
    );
    let add_o2 = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::MembershipAdd,
            2,
            vec![],
            membership_add_payload(PrincipalId::new(c.o2.principal_id().0), 0),
        ),
    );
    let add_a = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::MembershipAdd,
            3,
            vec![],
            membership_add_payload(PrincipalId::new(c.a.principal_id().0), 1),
        ),
    );
    let add_b = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::MembershipAdd,
            4,
            vec![],
            membership_add_payload(PrincipalId::new(c.b.principal_id().0), 1),
        ),
    );
    let raise = sign(
        &c.o1,
        base(&c.o1, c.group, AssertionType::RuleChange, 5, vec![], rule_change_payload(3, 2)),
    );
    let raise_h = envelope_hash(&raise);
    // V1's establishment: the register moves 1 (edge-only, the pre-V1
    // posture) → 7 (the full class) as a content-bound quorum act.
    let est_payload = rule_change_payload(ANTECEDENT_REGISTER_KEY, FULL_CLASS);
    let appr_est = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            1,
            vec![raise_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&est_payload)),
        ),
    );
    let est = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::RuleChange,
            6,
            vec![raise_h, envelope_hash(&appr_est)],
            est_payload,
        ),
    );
    let est_h = envelope_hash(&est);
    (vec![genesis, add_o2, add_a, add_b, raise, appr_est, est], est_h)
}

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

/// The crate-side vouch corpus the mirrored register is exercised against:
/// one transaction-backed edge-free vouch, one edge-backed vouch.
fn vouch_corpus() -> (Vec<Envelope>, ObjectId, ObjectId) {
    let w = World::new();
    let tx = w.p1a.emit(
        vec![],
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 6, 20)),
    );
    let v_tx = w.p1a.emit(
        vec![tx.object_id()],
        vouch_edge_free(w.p2.id, "would hire as plumber", "paid", d(2026, 7, 1), None),
    );
    let core = edge_core(w.p1a.id, w.p2.id, [0xA4; 16], vec![]);
    let core_hash = core.core_hash();
    let ha = w.p1a.emit(vec![], edge_half(core.clone(), "client"));
    let hb = w.p2.emit(vec![], edge_half(core, "client"));
    let v_edge = w.p1a.emit(
        vec![ha.object_id(), hb.object_id()],
        vouch(w.p2.id, "would trust with keys", "neighbor", core_hash, d(2026, 7, 2), None),
    );
    let v_tx_id = v_tx.object_id();
    let v_edge_id = v_edge.object_id();
    (vec![tx, v_tx, ha, hb, v_edge], v_tx_id, v_edge_id)
}

// ---------------------------------------------------------------------------
// T-A3.4 — the qualifying-kind register is governed (reused R7 machinery)
// ---------------------------------------------------------------------------

#[test]
fn antecedent_register_governed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, est_h) = register_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];

    // --- the establishment itself folds, and the genesis default was the
    // pre-V1 edge-only posture ------------------------------------------------
    let path_est = dir.path().join("register-est.redb");
    let _ = run_fold(&path_est, &authors, &setup, &[]);
    let session = Session::open(&path_est, &c.o1).expect("open session");
    let summary = session.get_group_summary(&c.group).expect("summary");
    assert_eq!(
        summary.rules.role_change_threshold, FULL_CLASS,
        "the register holds the full V1 class after the quorum establishment"
    );

    // Mirrored into the crate fold (the T-PA3.2 mirroring pattern): under the
    // full class, the transaction-backed edge-free vouch STANDS.
    let (corpus, v_tx_id, v_edge_id) = vouch_corpus();
    let log = log_from(&corpus);
    let full = AntecedentRegister::from_mask(summary.rules.role_change_threshold);
    let state_full = log.fold_with_register(&full);
    assert_eq!(state_full.vouch(&v_tx_id).unwrap().status, VouchStatus::Standing);
    assert_eq!(state_full.vouch(&v_edge_id).unwrap().status, VouchStatus::Standing);

    // --- narrowing the class back is a quorum act ----------------------------
    let narrow_payload = rule_change_payload(ANTECEDENT_REGISTER_KEY, EDGE_ONLY);

    // Under-quorum: a lone owner cannot move the register — refused whole.
    let lone = sign(
        &c.o1,
        base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![est_h], narrow_payload.clone()),
    );
    // Quorum met on the canonical payload: Admin A approves THIS content, O1
    // enacts citing the approval (the R7 shape), chained on the establishment.
    let appr = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            2,
            vec![est_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&narrow_payload)),
        ),
    );
    let narrow = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::RuleChange,
            7,
            vec![est_h, envelope_hash(&appr)],
            narrow_payload.clone(),
        ),
    );

    let path = dir.path().join("register.redb");
    let outcomes = run_fold(&path, &authors, &setup, &[&lone, &appr, &narrow]);
    assert!(outcomes[0].is_err(), "under-quorum register change must be refused whole");
    assert!(outcomes[1].is_ok());
    assert!(outcomes[2].is_ok(), "quorum-met register change applies: {:?}", outcomes[2]);

    // Visible in lineage: the change and its approval antecedent export; the
    // refused change left no partial trace.
    let session = Session::open(&path, &c.o1).expect("open session");
    let summary = session.get_group_summary(&c.group).expect("summary");
    assert_eq!(summary.rules.role_change_threshold, EDGE_ONLY, "the register moved 7 → 1");
    let log_export = session.export_group_log(&c.group).expect("export lineage");
    let frame_of = |env: &AssertionEnvelope| {
        let mut b = vec![0x01];
        b.extend_from_slice(&env.canonical_bytes_with_sig());
        b
    };
    assert!(log_export.contains(&frame_of(&narrow)), "the register change is visible in lineage");
    assert!(log_export.contains(&frame_of(&appr)), "its approval antecedent is visible in lineage");
    assert!(!log_export.contains(&frame_of(&lone)), "the refused change left no partial trace");
    assert!(
        narrow.antecedents.contains(&envelope_hash(&appr)),
        "the change names its approval as antecedent (the R7 shape)"
    );

    // Mirrored: under the narrowed (edge-only) register the SAME transaction-
    // backed vouch folds pending — no code edit happened, only a quorum act —
    // while the edge-backed vouch still stands.
    let narrowed = AntecedentRegister::from_mask(summary.rules.role_change_threshold);
    let state_narrow = log.fold_with_register(&narrowed);
    assert_eq!(
        state_narrow.vouch(&v_tx_id).unwrap().status,
        VouchStatus::Pending,
        "a kind outside the governed register does not qualify"
    );
    assert_eq!(state_narrow.vouch(&v_edge_id).unwrap().status, VouchStatus::Standing);
}

// ---------------------------------------------------------------------------
// Inheritance: concurrent quorum-met register changes hard-stop (§7.6.1)
// ---------------------------------------------------------------------------

/// The contradiction hard-stop is INHERITED unchanged: two concurrent
/// quorum-met changes to the antecedent register contradiction-hard-stop
/// exactly as §7.6.1 prescribes (byte-head min), and neither contested value
/// applies — the register keeps the established full class.
#[test]
fn antecedent_register_inherits_contradiction_hard_stop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, est_h) = register_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];

    let p3 = rule_change_payload(ANTECEDENT_REGISTER_KEY, 0b011);
    let p5 = rule_change_payload(ANTECEDENT_REGISTER_KEY, 0b101);
    let appr_a = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            2,
            vec![est_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&p3)),
        ),
    );
    let appr_b = sign(
        &c.b,
        base(
            &c.b,
            c.group,
            AssertionType::Approval,
            1,
            vec![est_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&p5)),
        ),
    );
    let change3 = sign(
        &c.o1,
        base(&c.o1, c.group, AssertionType::RuleChange, 7, vec![est_h, envelope_hash(&appr_a)], p3),
    );
    let change5 = sign(
        &c.o2,
        base(&c.o2, c.group, AssertionType::RuleChange, 1, vec![est_h, envelope_hash(&appr_b)], p5),
    );

    let expected_head = min_hash(envelope_hash(&change3), envelope_hash(&change5));
    let expected_status = format!("contradiction:{expected_head}");

    for (name, order) in [
        ("order1", vec![&appr_a, &appr_b, &change3, &change5]),
        ("order2", vec![&appr_b, &appr_a, &change5, &change3]),
    ] {
        let path = dir.path().join(format!("{name}.redb"));
        let _ = run_fold(&path, &authors, &setup, &order);
        let session = Session::open(&path, &c.o1).expect("open session");
        let summary = session.get_group_summary(&c.group).expect("summary");
        assert_eq!(
            summary.fork_status, expected_status,
            "{name}: the antecedent register INHERITS the substrate contradiction hard-stop unchanged"
        );
        assert_eq!(
            summary.rules.role_change_threshold, FULL_CLASS,
            "{name}: neither contested value applies; the register keeps the established class"
        );
    }
}
