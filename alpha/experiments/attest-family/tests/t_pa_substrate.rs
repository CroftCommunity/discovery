//! RUN-ATTEST-02, substrate-reuse half: T-PA3.2 (the anchor-count dial as a
//! governed R7 rule) and the covenant-lineage-intact leg of T-PA6.2. Both run
//! on the EXISTING content-bound-quorum machinery (`local_storage_projection`
//! and `social-graph-core`, imported unchanged, exactly as T-AT6.4 drives
//! them). The commitment-audit leg of T-PA6.2 lives in `t_pa6_issuer.rs`.
//!
//! Register stand-in (declared): substrate rule_key 0 (`add_member_threshold`)
//! reinterpreted as **max anchors per member**. rule_key 1 remains the
//! covenant register from T-AT6.4. What is under test is the machinery —
//! content-bound approvals, thresholds, contradiction hard-stop — not the
//! registers' names.
//!
//! Envelope-building scaffolding is copied from
//! `croft-chat/tests/common/mod.rs` via `t_at6_covenant.rs` (scaffolding
//! copied, machinery imported — nothing shadowed).

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

use attest_family::fixtures::{derived_seed, member_ref, AnchorWorld, Holder};
use attest_family::issuer::{mint, IssuerState, MintEntropy, MintRefusal};
use attest_family::types::{DateClaim, PredicateKind};

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

// --- the governed-dial fixture ----------------------------------------------

/// The anchor-dial register: substrate rule_key 0 (`add_member_threshold`),
/// reused as **max anchors per member** (declared stand-in).
const ANCHOR_DIAL_KEY: u8 = 0;

struct Cast {
    o1: Identity,
    o2: Identity,
    a: Identity,
    b: Identity,
    group: GroupId,
}

fn cast() -> Cast {
    Cast {
        o1: Identity::from_seed([0xE0; 32]),
        o2: Identity::from_seed([0xE1; 32]),
        a: Identity::from_seed([0xE2; 32]),
        b: Identity::from_seed([0xE3; 32]),
        group: GroupId::new([0xAD; 32]),
    }
}

/// Genesis, two Owners, two Admins, RuleChange threshold raised to 2 (the R7
/// gate). Returns the setup envelopes + the raise hash.
fn dial_setup(c: &Cast) -> (Vec<AssertionEnvelope>, Hash) {
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
    (vec![genesis, add_o2, add_a, add_b, raise], raise_h)
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

// ---------------------------------------------------------------------------
// T-PA3.2 — the anchor count is a governed dial (reused R7 machinery)
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-4 pending — fee semantics (flat per-anchor vs vetting-tier
// pricing) are pure policy on top of this dial; nothing here decides them.
#[test]
fn anchor_count_is_a_governed_dial() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, raise_h) = dial_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];

    let to_three = rule_change_payload(ANCHOR_DIAL_KEY, 3);

    // Under-quorum: a lone owner cannot move the dial — refused whole.
    let lone =
        sign(&c.o1, base(&c.o1, c.group, AssertionType::RuleChange, 6, vec![raise_h], to_three.clone()));

    // Quorum met on the canonical payload: Admin A approves THIS content, O1
    // enacts citing the approval (the R7 shape).
    let appr = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            1,
            vec![raise_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&to_three)),
        ),
    );
    let change = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::RuleChange,
            6,
            vec![raise_h, envelope_hash(&appr)],
            to_three.clone(),
        ),
    );

    let path = dir.path().join("dial.redb");
    let outcomes = run_fold(&path, &authors, &setup, &[&lone, &appr, &change]);
    assert!(outcomes[0].is_err(), "under-quorum dial change must be refused whole");
    assert!(outcomes[1].is_ok());
    assert!(outcomes[2].is_ok(), "quorum-met dial change applies: {:?}", outcomes[2]);

    // Visible in lineage: the change and its approval antecedent export.
    let session = Session::open(&path, &c.o1).expect("open session");
    let summary = session.get_group_summary(&c.group).expect("summary");
    assert_eq!(summary.rules.add_member_threshold, 3, "the dial register moved 1 → 3");
    let log = session.export_group_log(&c.group).expect("export lineage");
    let frame_of = |env: &AssertionEnvelope| {
        let mut b = vec![0x01];
        b.extend_from_slice(&env.canonical_bytes_with_sig());
        b
    };
    assert!(log.contains(&frame_of(&change)), "the dial change is visible in lineage");
    assert!(log.contains(&frame_of(&appr)), "its approval antecedent is visible in lineage");
    assert!(!log.contains(&frame_of(&lone)), "the refused change left no partial trace");

    // The issuer state MIRRORS the folded register and enforces it at the
    // seam: with the dial at 3, a member's fourth anchor is refused
    // deterministically; the first three mint.
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(1);
    state.set_dial(summary.rules.add_member_threshold);
    assert_eq!(state.dial(), 3);
    let subjects = [&w.p1a, &w.p1b, &w.p1c];
    for (k, s) in subjects.iter().enumerate() {
        assert!(
            mint(
                &mut state,
                &w.coop,
                member_ref(&w.h1),
                s,
                &[PredicateKind::VettedHolder],
                DateClaim::new(2026, 7, 17),
                MintEntropy::from_seed(derived_seed("t-pa3-2", 0, k as u64)),
            )
            .is_ok(),
            "anchor {} of 3 mints",
            k + 1
        );
    }
    let fourth = attest_family::fixtures::PersonaFixture::new(
        "P1d",
        Holder("H1"),
        derived_seed("t-pa3-2-4th", 0, 0),
        false,
    );
    assert_eq!(
        mint(
            &mut state,
            &w.coop,
            member_ref(&w.h1),
            &fourth,
            &[PredicateKind::VettedHolder],
            DateClaim::new(2026, 7, 17),
            MintEntropy::from_seed(derived_seed("t-pa3-2", 0, 9)),
        )
        .expect_err("the dial refuses the fourth anchor"),
        MintRefusal::DialExceeded
    );
    // A different member is unaffected by H1's count.
    assert!(mint(
        &mut state,
        &w.coop,
        member_ref(&w.h2),
        &w.p2a,
        &[PredicateKind::VettedHolder],
        DateClaim::new(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-pa3-2", 0, 10)),
    )
    .is_ok());
}

/// The concurrent-quorum hard-stop is INHERITED unchanged: two quorum-met
/// dial changes with no causal order contradiction-hard-stop exactly as
/// §7.6.1 prescribes (byte-head min), and neither contested value applies.
#[test]
fn dial_inherits_contradiction_hard_stop() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, raise_h) = dial_setup(&c);
    let authors = [&c.o1, &c.o2, &c.a, &c.b];

    let p2 = rule_change_payload(ANCHOR_DIAL_KEY, 2);
    let p5 = rule_change_payload(ANCHOR_DIAL_KEY, 5);
    let appr_a = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            1,
            vec![raise_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&p2)),
        ),
    );
    let appr_b = sign(
        &c.b,
        base(
            &c.b,
            c.group,
            AssertionType::Approval,
            1,
            vec![raise_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&p5)),
        ),
    );
    let change2 = sign(
        &c.o1,
        base(&c.o1, c.group, AssertionType::RuleChange, 6, vec![raise_h, envelope_hash(&appr_a)], p2),
    );
    let change5 = sign(
        &c.o2,
        base(&c.o2, c.group, AssertionType::RuleChange, 1, vec![raise_h, envelope_hash(&appr_b)], p5),
    );

    let expected_head = min_hash(envelope_hash(&change2), envelope_hash(&change5));
    let expected_status = format!("contradiction:{expected_head}");

    for (name, order) in [
        ("order1", vec![&appr_a, &appr_b, &change2, &change5]),
        ("order2", vec![&appr_b, &appr_a, &change5, &change2]),
    ] {
        let path = dir.path().join(format!("{name}.redb"));
        let _ = run_fold(&path, &authors, &setup, &order);
        let session = Session::open(&path, &c.o1).expect("open session");
        let summary = session.get_group_summary(&c.group).expect("summary");
        assert_eq!(
            summary.fork_status, expected_status,
            "{name}: the dial INHERITS the substrate contradiction hard-stop unchanged"
        );
        assert_eq!(
            summary.rules.add_member_threshold, 1,
            "{name}: neither contested value applies; the dial keeps its pre-conflict value"
        );
    }
}

// ---------------------------------------------------------------------------
// T-PA6.2 (covenant-lineage leg) — the covenant rule's lineage is intact and
// auditable without unmasking anyone
// ---------------------------------------------------------------------------

/// The commitment-audit leg lives in `t_pa6_issuer.rs`; this leg audits the
/// COVENANT RULE's lineage on the reused machinery: the covenant register's
/// establishment and its quorum antecedents are present and chained in the
/// exported log, and that export contains only GOVERNOR principals (the
/// issuer org's own identities) — no subject persona bytes exist anywhere in
/// the governed lineage.
#[test]
fn covenant_rule_lineage_intact_without_unmasking() {
    let dir = tempfile::tempdir().expect("tempdir");
    let c = cast();
    let (setup, raise_h) = dial_setup(&c);
    // Establish the covenant register (rule_key 1 — the T-AT6.4 stand-in) at
    // full strength under the quorum gate, causally chained on the raise.
    let set_payload = rule_change_payload(1, 3);
    let appr = sign(
        &c.a,
        base(
            &c.a,
            c.group,
            AssertionType::Approval,
            1,
            vec![raise_h],
            approval_payload(AssertionType::RuleChange, rc_subject(&set_payload)),
        ),
    );
    let set_full = sign(
        &c.o1,
        base(
            &c.o1,
            c.group,
            AssertionType::RuleChange,
            6,
            vec![raise_h, envelope_hash(&appr)],
            set_payload,
        ),
    );
    let authors = [&c.o1, &c.o2, &c.a, &c.b];
    let path = dir.path().join("covenant-lineage.redb");
    // (The RED stage omitted `appr` here — an under-quorum establishment,
    // which the reused machinery refused whole: no intact covenant lineage
    // can exist without its quorum antecedents.)
    let outcomes = run_fold(&path, &authors, &setup, &[&appr, &set_full]);
    assert!(outcomes.iter().all(|o| o.is_ok()), "covenant establishment folds: {outcomes:?}");

    let session = Session::open(&path, &c.o1).expect("open session");
    let summary = session.get_group_summary(&c.group).expect("summary");
    assert_eq!(summary.rules.remove_member_threshold, 3, "covenant at full strength");

    let log = session.export_group_log(&c.group).expect("export lineage");
    let frame_of = |env: &AssertionEnvelope| {
        let mut b = vec![0x01];
        b.extend_from_slice(&env.canonical_bytes_with_sig());
        b
    };
    // Intact: the establishment AND its approval antecedent are in the export,
    // and the establishment names the approval causally (the chained shape
    // F-AT-5 requires).
    assert!(log.contains(&frame_of(&set_full)), "covenant establishment in lineage");
    assert!(log.contains(&frame_of(&appr)), "its approval antecedent in lineage");
    assert!(set_full.antecedents.contains(&envelope_hash(&appr)), "chained, not concurrent");

    // Without unmasking: no anchor-holder persona exists in this lineage —
    // sweep the export for the AnchorWorld persona keys (subject personas
    // never touch the governed covenant log; only governor principals do).
    let w = AnchorWorld::new();
    for p in [&w.p1a, &w.p1b, &w.p1c, &w.p2a, &w.p2b, &w.p3, &w.p4, &w.p5] {
        assert!(
            !log.iter().any(|frame| contains_subslice_bytes(frame, &p.id.0)),
            "a subject persona id appeared in the covenant lineage"
        );
    }
}

fn contains_subslice_bytes(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}
