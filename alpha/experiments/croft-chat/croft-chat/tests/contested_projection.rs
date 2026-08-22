//! **E108 / canonical §7.3.2 — `CONTESTED` as a first-class membership state.**
//!
//! The step-5 merge ratified rule (c): the subject of an open contradiction projects as
//! neither member nor not-member but `CONTESTED`, carrying the conflicting **pair as data,
//! in a set** — two simultaneously open contradictions must be representable (the old
//! `ForkStatus::Contradiction(TypesHash)` single min-hash slot structurally cannot), and
//! **resolution needs its own fact type** (the hard-stop replay is not resolution).
//!
//! Resolution authorization (owner, 2026-08-21, closing the vet's R4): resolving a pair is a
//! governed act whose threshold lives in the charter (`GroupRules.resolution_threshold`), the
//! same k-of-n Approval machinery as every other governance threshold; product default **2**;
//! never silently single-author. The spec hard-floor question rides the §7.3.2 filing.
//!
//! Pins, RED-first (plan E117 Phase 1):
//!   1. arrival-order: both orders of a mutual expulsion project both subjects `CONTESTED`,
//!      byte-identically;
//!   2. two open contradictions are both representable, each carrying its pair;
//!   3. a single-author resolution is refused at the default threshold;
//!   4. a quorum resolution closes exactly the named pair, leaving the other open, and the
//!      post-resolution projection is deterministic across arrival orders.
//!
//! Fidelity: **Modeled** — real fold over a real store; governance facts are this
//! experiment's envelope type, not a wire-format-final Drystone encoding.

mod common;

use std::sync::Arc;

use common::{approval_payload, base, genesis_payload, membership_add_payload, sign};
use local_storage_projection::fold_derived::{DerivedFold, ForkStatus, MembershipView};
use local_storage_projection::tables::Db;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionEnvelope, AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::{Ed25519Verifier, Identity, RegistryCredentialResolver};

fn remove_payload(subject: PrincipalId) -> Vec<u8> {
    subject.as_bytes().to_vec()
}

/// Resolution payload: the pair's two content addresses, lexicographically ordered.
fn resolution_payload(
    a: &local_storage_projection::Hash,
    b: &local_storage_projection::Hash,
) -> Vec<u8> {
    let (lo, hi) = if a.as_bytes() <= b.as_bytes() { (a, b) } else { (b, a) };
    let mut p = Vec::with_capacity(64);
    p.extend_from_slice(lo.as_bytes());
    p.extend_from_slice(hi.as_bytes());
    p
}

struct Folded {
    _db: Arc<Db>,
    fold: DerivedFold<Ed25519Verifier, RegistryCredentialResolver>,
}

/// Ingest `order` into a fresh store and keep the fold open for direct state reads.
/// Every ingest outcome is surfaced (eprintln), never swallowed — a silently-rejected
/// fact would turn an order-dependence claim into a harness artifact.
fn fold_order(path: &std::path::Path, authors: &[&Identity], order: &[&AssertionEnvelope]) -> Folded {
    let db = Arc::new(Db::open(path).expect("open db"));
    let resolver = RegistryCredentialResolver::new();
    for a in authors {
        resolver.register(a.device_id(), a.principal_id());
    }
    let fold = DerivedFold::new(Arc::clone(&db), Ed25519Verifier, resolver);
    for env in order {
        match fold.ingest(env) {
            Ok(r) => eprintln!("    ingest {:?} -> {:?}", env.assertion_type, r),
            Err(e) => eprintln!("    ingest {:?} -> REJECTED: {e}", env.assertion_type),
        }
    }
    Folded { _db: db, fold }
}

/// The cast and facts shared by every pin: O owns the group; A, B, C admins; D a member.
/// Pair 1 (mutual expulsion): A removes B ⊗ B removes A — both concurrent.
/// Pair 2 (removed-then-included): C removes D ⊗ O re-adds D — concurrent on subject D.
struct Cast {
    group: GroupId,
    id_o: Identity,
    id_a: Identity,
    id_b: Identity,
    id_c: Identity,
    a: PrincipalId,
    b: PrincipalId,
    c: PrincipalId,
    d: PrincipalId,
    genesis: AssertionEnvelope,
    add_a: AssertionEnvelope,
    add_b: AssertionEnvelope,
    add_c: AssertionEnvelope,
    add_d: AssertionEnvelope,
    a_removes_b: AssertionEnvelope,
    b_removes_a: AssertionEnvelope,
    c_removes_d: AssertionEnvelope,
    o_readds_d: AssertionEnvelope,
}

fn cast() -> Cast {
    let group = GroupId::new([0xC7; 32]);
    let id_o = Identity::from_seed([0xA0; 32]);
    let id_a = Identity::from_seed([0xA1; 32]);
    let id_b = Identity::from_seed([0xA2; 32]);
    let id_c = Identity::from_seed([0xA3; 32]);
    let id_d = Identity::from_seed([0xA4; 32]);
    let a = PrincipalId::new(id_a.principal_id().0);
    let b = PrincipalId::new(id_b.principal_id().0);
    let c = PrincipalId::new(id_c.principal_id().0);
    let d = PrincipalId::new(id_d.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let add_a = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a, 1)),
    );
    let add_b = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b, 1)),
    );
    let add_c = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(c, 1)),
    );
    let add_d = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 5, vec![], membership_add_payload(d, 2)),
    );

    // Pair 1: mutually-expelling concurrent removes (each antecedes only setup facts).
    let a_removes_b = sign(
        &id_a,
        base(&id_a, group, AssertionType::MembershipRemove, 10, vec![envelope_hash(&add_b)], remove_payload(b)),
    );
    let b_removes_a = sign(
        &id_b,
        base(&id_b, group, AssertionType::MembershipRemove, 10, vec![envelope_hash(&add_a)], remove_payload(a)),
    );

    // Pair 2: an add/remove race on D — both antecede add_d, neither sees the other.
    let c_removes_d = sign(
        &id_c,
        base(&id_c, group, AssertionType::MembershipRemove, 12, vec![envelope_hash(&add_d)], remove_payload(d)),
    );
    let o_readds_d = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 12, vec![envelope_hash(&add_d)], membership_add_payload(d, 2)),
    );

    Cast {
        group, id_o, id_a, id_b, id_c,
        a, b, c, d,
        genesis, add_a, add_b, add_c, add_d,
        a_removes_b, b_removes_a, c_removes_d, o_readds_d,
    }
}

fn is_contested(v: &MembershipView) -> bool {
    matches!(v, MembershipView::Contested(_))
}

/// **Pin 1 — arrival-order: mutual expulsion projects both subjects CONTESTED, byte-identically.**
///
/// This is E108's ratified rule (c) made executable: while the pair is open, A and B are
/// neither member nor not-member — the projection stops pretending it knows — and the two
/// arrival orders produce byte-identical `GroupState`.
#[test]
fn mutual_expulsion_projects_contested_both_orders() {
    let dir = tempfile::tempdir().expect("tempdir");
    let k = cast();
    let authors = [&k.id_o, &k.id_a, &k.id_b, &k.id_c];
    let setup = [&k.genesis, &k.add_a, &k.add_b, &k.add_c, &k.add_d];

    let order1: Vec<&AssertionEnvelope> =
        setup.iter().copied().chain([&k.a_removes_b, &k.b_removes_a]).collect();
    let order2: Vec<&AssertionEnvelope> =
        setup.iter().copied().chain([&k.b_removes_a, &k.a_removes_b]).collect();

    let f1 = fold_order(&dir.path().join("o1.redb"), &authors, &order1);
    let f2 = fold_order(&dir.path().join("o2.redb"), &authors, &order2);

    let s1 = f1.fold.read_group_state(&k.group).expect("read 1").expect("state 1");
    let s2 = f2.fold.read_group_state(&k.group).expect("read 2").expect("state 2");

    // The subject of an open contradiction is CONTESTED — in both orders, for both parties.
    for (label, s) in [("order1", &s1), ("order2", &s2)] {
        assert!(
            is_contested(&s.membership(&k.a)),
            "{label}: A must project CONTESTED, got {:?}",
            s.membership(&k.a)
        );
        assert!(
            is_contested(&s.membership(&k.b)),
            "{label}: B must project CONTESTED, got {:?}",
            s.membership(&k.b)
        );
        // Totality: an uninvolved admin stays a plain member; a stranger stays NotMember.
        assert!(
            matches!(s.membership(&k.c), MembershipView::Member(_)),
            "{label}: C must stay Member"
        );
        assert!(
            matches!(s.membership(&PrincipalId::new([0x5A; 32])), MembershipView::NotMember),
            "{label}: a stranger must project NotMember"
        );
    }

    // The artifact carries the pair AS DATA: both content addresses, order-independent.
    let want_pair = {
        let (ha, hb) = (envelope_hash(&k.a_removes_b), envelope_hash(&k.b_removes_a));
        if ha.as_bytes() <= hb.as_bytes() { (ha, hb) } else { (hb, ha) }
    };
    for (label, s) in [("order1", &s1), ("order2", &s2)] {
        match &s.fork_status {
            ForkStatus::Contested(entries) => {
                assert_eq!(entries.len(), 1, "{label}: exactly one open contradiction");
                assert_eq!(entries[0].pair, want_pair, "{label}: the pair travels as data");
            }
            other => panic!("{label}: expected Contested, got {other:?}"),
        }
    }

    // Byte-identical convergence — the C4/G1 property survives the schema change.
    // `computed_at_gov_head` records which fact a node folded LAST — a locator, never
    // resolution content (§7.4.3's locator-not-authorization discipline) — so it is
    // normalized; everything else (members, rules, fork entries, projection) must be
    // byte-identical across arrival orders.
    let normalized = |s: &local_storage_projection::fold_derived::GroupState| {
        let mut c = s.clone();
        c.computed_at_gov_head = local_storage_projection::Hash::new([0u8; 32]);
        c.to_bytes()
    };
    assert_eq!(normalized(&s1), normalized(&s2), "orders must converge byte-identically");
}

/// **Pin 2 — two simultaneously open contradictions are representable, each with its pair.**
///
/// The review's schema finding made executable: the old single min-hash slot cannot hold
/// pair 1 (mutual expulsion, subjects A and B) and pair 2 (removed-then-included, subject D)
/// at once. The set-valued artifact must carry both, and the projection must contest
/// exactly {A, B, D} while C remains a member.
#[test]
fn two_open_contradictions_are_both_representable() {
    let dir = tempfile::tempdir().expect("tempdir");
    let k = cast();
    let authors = [&k.id_o, &k.id_a, &k.id_b, &k.id_c];

    let order: Vec<&AssertionEnvelope> = [
        &k.genesis, &k.add_a, &k.add_b, &k.add_c, &k.add_d,
        &k.a_removes_b, &k.b_removes_a, // pair 1 opens
        &k.c_removes_d, &k.o_readds_d,  // pair 2 opens
    ]
    .to_vec();

    let f = fold_order(&dir.path().join("two.redb"), &authors, &order);
    let s = f.fold.read_group_state(&k.group).expect("read").expect("state");

    match &s.fork_status {
        ForkStatus::Contested(entries) => {
            assert_eq!(
                entries.len(),
                2,
                "both contradictions must be open simultaneously, got {entries:?}"
            );
            let pairs: Vec<_> = entries.iter().map(|e| e.pair).collect();
            let p1 = {
                let (x, y) = (envelope_hash(&k.a_removes_b), envelope_hash(&k.b_removes_a));
                if x.as_bytes() <= y.as_bytes() { (x, y) } else { (y, x) }
            };
            let p2 = {
                let (x, y) = (envelope_hash(&k.c_removes_d), envelope_hash(&k.o_readds_d));
                if x.as_bytes() <= y.as_bytes() { (x, y) } else { (y, x) }
            };
            assert!(pairs.contains(&p1), "pair 1 (mutual expulsion) must be carried");
            assert!(pairs.contains(&p2), "pair 2 (removed-then-included) must be carried");
        }
        other => panic!("expected Contested with two entries, got {other:?}"),
    }

    assert!(is_contested(&s.membership(&k.a)), "A contested (pair 1)");
    assert!(is_contested(&s.membership(&k.b)), "B contested (pair 1)");
    assert!(is_contested(&s.membership(&k.d)), "D contested (pair 2)");
    assert!(
        matches!(s.membership(&k.c), MembershipView::Member(_)),
        "C must remain a plain member"
    );
}

/// **Pin 3 — a single-author resolution is refused at the default threshold.**
///
/// The one-signature verdict the fold refused to manufacture must not be purchasable with a
/// new fact type: `resolution_threshold` defaults to 2, so a Resolution with no approvals is
/// refused (`ThresholdNotMet`) and the pair stays open.
#[test]
fn single_author_resolution_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let k = cast();
    let authors = [&k.id_o, &k.id_a, &k.id_b, &k.id_c];
    let order: Vec<&AssertionEnvelope> = [
        &k.genesis, &k.add_a, &k.add_b, &k.add_c, &k.add_d,
        &k.a_removes_b, &k.b_removes_a,
    ]
    .to_vec();

    let f = fold_order(&dir.path().join("solo.redb"), &authors, &order);

    let pair = resolution_payload(&envelope_hash(&k.a_removes_b), &envelope_hash(&k.b_removes_a));
    let solo = sign(
        &k.id_o,
        base(&k.id_o, k.group, AssertionType::Resolution, 7, vec![], pair),
    );

    let err = f.fold.ingest(&solo).expect_err("single-author resolution must be refused");
    let msg = format!("{err}");
    assert!(
        msg.to_lowercase().contains("threshold"),
        "refusal must be the threshold gate, got: {msg}"
    );

    let s = f.fold.read_group_state(&k.group).expect("read").expect("state");
    assert!(is_contested(&s.membership(&k.a)), "pair must stay open: A still CONTESTED");
    assert!(is_contested(&s.membership(&k.b)), "pair must stay open: B still CONTESTED");
}

/// **Pin 4 — a quorum resolution closes exactly the named pair, deterministically.**
///
/// With both pairs open, a Resolution carrying one Approval (two distinct personae — the
/// default threshold) closes pair 1 only: A and B return to the deterministic replay
/// projection (members — the contested removes stay withheld; no retroactive verdict;
/// further governance re-decides), D stays CONTESTED, and the post-resolution state is
/// byte-identical across the arrival orders of the resolved pair.
#[test]
fn quorum_resolution_closes_named_pair_only() {
    let dir = tempfile::tempdir().expect("tempdir");
    let k = cast();
    let authors = [&k.id_o, &k.id_a, &k.id_b, &k.id_c];

    let pair_bytes =
        resolution_payload(&envelope_hash(&k.a_removes_b), &envelope_hash(&k.b_removes_a));
    // The approval names (Resolution, H(payload)) — the RuleChange-style content-hash
    // subject, computable by the approver before the act exists.
    let subject = PrincipalId::new(
        local_storage_projection::fold_derived::rule_change_approval_subject(&pair_bytes),
    );
    let approve = sign(
        &k.id_c,
        base(&k.id_c, k.group, AssertionType::Approval, 13, vec![], approval_payload(AssertionType::Resolution, subject)),
    );
    let resolve = sign(
        &k.id_o,
        base(&k.id_o, k.group, AssertionType::Resolution, 13, vec![envelope_hash(&approve)], pair_bytes.clone()),
    );

    let run = |name: &str, first: &AssertionEnvelope, second: &AssertionEnvelope| {
        let order: Vec<&AssertionEnvelope> = vec![
            &k.genesis, &k.add_a, &k.add_b, &k.add_c, &k.add_d,
            first, second,
            &k.c_removes_d, &k.o_readds_d,
            &approve, &resolve,
        ];
        let f = fold_order(&dir.path().join(format!("{name}.redb")), &authors, &order);
        f.fold.read_group_state(&k.group).expect("read").expect("state")
    };

    let s1 = run("r1", &k.a_removes_b, &k.b_removes_a);
    let s2 = run("r2", &k.b_removes_a, &k.a_removes_b);

    for (label, s) in [("r1", &s1), ("r2", &s2)] {
        // Pair 1 is closed: A and B are back to the deterministic replay projection.
        assert!(
            matches!(s.membership(&k.a), MembershipView::Member(_)),
            "{label}: A returns to Member after resolution, got {:?}",
            s.membership(&k.a)
        );
        assert!(
            matches!(s.membership(&k.b), MembershipView::Member(_)),
            "{label}: B returns to Member after resolution, got {:?}",
            s.membership(&k.b)
        );
        // Pair 2 was NOT named and stays open.
        assert!(
            is_contested(&s.membership(&k.d)),
            "{label}: D must stay CONTESTED — the resolution names one pair only"
        );
        match &s.fork_status {
            ForkStatus::Contested(entries) => {
                assert_eq!(entries.len(), 1, "{label}: exactly the unresolved pair remains");
            }
            other => panic!("{label}: expected Contested(1 entry), got {other:?}"),
        }
    }

    assert_eq!(
        s1.to_bytes(),
        s2.to_bytes(),
        "post-resolution state must be byte-identical across arrival orders"
    );
}

/// **Pin 5 — closing a pair is not un-deciding it: resolved exclusions persist through
/// every LATER replay.** (Added by the P1 mutation sweep: `resolved_excluded` had no
/// killer — nothing replayed after a resolution, so a fold that forgot closed pairs
/// would only diverge at the *next* contradiction or resolution.)
///
/// Both pairs are opened, then resolved in sequence. The second resolution triggers a
/// full deterministic replay whose exclusion set must still carry pair 1's facts —
/// derived from the log's admitted Resolution facts, not from any state memory. If
/// that derivation breaks, pair 1's mutually-expelling removes re-apply and A or B
/// loses membership silently.
#[test]
fn resolved_exclusions_persist_through_later_replays() {
    let dir = tempfile::tempdir().expect("tempdir");
    let k = cast();
    let authors = [&k.id_o, &k.id_a, &k.id_b, &k.id_c];

    let pair1 =
        resolution_payload(&envelope_hash(&k.a_removes_b), &envelope_hash(&k.b_removes_a));
    let subject1 = PrincipalId::new(
        local_storage_projection::fold_derived::rule_change_approval_subject(&pair1),
    );
    let approve1 = sign(
        &k.id_c,
        base(&k.id_c, k.group, AssertionType::Approval, 13, vec![], approval_payload(AssertionType::Resolution, subject1)),
    );
    let resolve1 = sign(
        &k.id_o,
        base(&k.id_o, k.group, AssertionType::Resolution, 13, vec![envelope_hash(&approve1)], pair1),
    );

    let pair2 =
        resolution_payload(&envelope_hash(&k.c_removes_d), &envelope_hash(&k.o_readds_d));
    let subject2 = PrincipalId::new(
        local_storage_projection::fold_derived::rule_change_approval_subject(&pair2),
    );
    let approve2 = sign(
        &k.id_c,
        base(&k.id_c, k.group, AssertionType::Approval, 14, vec![], approval_payload(AssertionType::Resolution, subject2)),
    );
    let resolve2 = sign(
        &k.id_o,
        base(&k.id_o, k.group, AssertionType::Resolution, 14, vec![envelope_hash(&approve2)], pair2),
    );

    let order: Vec<&AssertionEnvelope> = vec![
        &k.genesis, &k.add_a, &k.add_b, &k.add_c, &k.add_d,
        &k.a_removes_b, &k.b_removes_a, // pair 1 opens
        &k.c_removes_d, &k.o_readds_d,  // pair 2 opens
        &approve1, &resolve1,           // pair 1 closes
        &approve2, &resolve2,           // pair 2 closes — replay must STILL exclude pair 1
    ];
    let f = fold_order(&dir.path().join("persist.redb"), &authors, &order);
    let s = f.fold.read_group_state(&k.group).expect("read").expect("state");

    assert!(
        matches!(s.fork_status, ForkStatus::Clean),
        "both pairs closed — the group is clean, got {:?}",
        s.fork_status
    );
    for (who, name) in [(&k.a, "A"), (&k.b, "B"), (&k.c, "C"), (&k.d, "D")] {
        assert!(
            matches!(s.membership(who), MembershipView::Member(_)),
            "{name} must be a member after both resolutions (resolved removes stay \
             withheld; nothing re-applies), got {:?}",
            s.membership(who)
        );
    }
}
