// Reference fold v2 (R1-R4), NOT a production fold implementation.
// No production fold was found in `croftc/upstream-repo`. See RESULTS.md.

use drystone_convergence::fold::fold;
use drystone_convergence::types::{AuthorityState, Fact, FactId, FactPayload};

use proptest::prelude::*;
use rand::seq::SliceRandom;
use rand::{SeedableRng, rngs::SmallRng};

// ─── Fixed pools ─────────────────────────────────────────────────────────────

const MEMBERS: &[&str] = &["alice", "bob", "charlie"];
const ROLES:   &[&str] = &["admin", "auditor"];
const AUTHORS: &[&str] = &["node0", "node1", "node2"];

// ─── Generator ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct RawOp {
    author_idx:   u8,
    payload_kind: u8,
    member_idx:   u8,
    role_idx:     u8,
    k:            u8,
    n:            u8,
}

fn raw_op_strategy() -> impl Strategy<Value = RawOp> {
    (any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>())
        .prop_map(|(a, p, m, r, k, n)| RawOp {
            author_idx: a, payload_kind: p, member_idx: m, role_idx: r, k, n,
        })
}

/// Convert RawOps into a causally-structured fact set.
///
/// Within each author, successive facts list the prior fact as a predecessor,
/// giving a causal chain per author. Facts from different authors with no
/// cross-references are concurrent. Small member/role pools maximise the
/// probability of conflicting concurrent facts.
fn materialize(ops: &[RawOp]) -> Vec<Fact> {
    let mut counters:  [u64; 3]          = [0; 3];
    let mut last_ids:  [Option<FactId>; 3] = [None; 3];
    let mut facts = Vec::with_capacity(ops.len());

    for op in ops {
        let ai = (op.author_idx % AUTHORS.len() as u8) as usize;
        counters[ai] += 1;

        let mi = (op.member_idx % MEMBERS.len() as u8) as usize;
        let ri = (op.role_idx   % ROLES.len()   as u8) as usize;
        let n  = (op.n % 3 + 2) as u32;
        let k  = ((op.k % 3 + 1) as u32).min(n);

        let payload = match op.payload_kind % 5 {
            0 => FactPayload::AddMember(MEMBERS[mi].to_string()),
            1 => FactPayload::RemoveMember(MEMBERS[mi].to_string()),
            2 => FactPayload::GrantRole(MEMBERS[mi].to_string(), ROLES[ri].to_string()),
            3 => FactPayload::RevokeRole(MEMBERS[mi].to_string(), ROLES[ri].to_string()),
            _ => FactPayload::SetThreshold(ROLES[ri].to_string(), k, n),
        };

        let preds: Vec<FactId> = last_ids[ai].iter().copied().collect();
        let fact = Fact::new(AUTHORS[ai].to_string(), counters[ai], preds, payload);
        last_ids[ai] = Some(fact.id);
        facts.push(fact);
    }
    facts
}

fn fact_set_strategy() -> impl Strategy<Value = Vec<Fact>> {
    proptest::collection::vec(raw_op_strategy(), 1..=20)
        .prop_map(|ops| materialize(&ops))
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY A (retained from v1): permutation invariance
// ═══════════════════════════════════════════════════════════════════════════════

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// For any complete fact-set S and any permutation π, fold(S) == fold(π(S)).
    ///
    /// Tested against: faithful R1-R4 reference fold (NOT production).
    #[test]
    fn prop_a_permutation_invariance(
        facts        in fact_set_strategy(),
        shuffle_seed in any::<u64>(),
    ) {
        let baseline = fold(&facts).unwrap().fingerprint();
        let mut rng = SmallRng::seed_from_u64(shuffle_seed);
        for round in 0..5u8 {
            let mut permuted = facts.clone();
            permuted.shuffle(&mut rng);
            let result = fold(&permuted).unwrap().fingerprint();
            prop_assert_eq!(
                baseline, result,
                "Fold is order-dependent (shuffle round {}, seed={}).\n\
                 Facts ({} total): {:?}",
                round, shuffle_seed, facts.len(),
                facts.iter().map(|f| &f.payload).collect::<Vec<_>>()
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY B (new, R1): causal precedence beats FactId order
//
// The discriminating property: a causally-later fact must win even when it
// carries a smaller FactId. The v1 highest-id-wins stub fails this property.
//
// R1 correctness must hold for any id assignment; explicit synthetic ids are
// legitimate precisely because they exercise that guarantee directly.
// ═══════════════════════════════════════════════════════════════════════════════

/// Minimal hand case from the brief:
///   Add(m)       [id=5, no preds]
///   Grant(m,r)   [id=4, preds={5}]   ← causally after Add
///   Revoke(m,r)  [id=1, preds={4}]   ← causally last, smallest id
///
/// The revoke is causally last and has the smallest id.
/// An id-only fold (highest-FactId wins) would keep the grant (id 4 > id 1).
/// The R1 fold must keep the revoke; (m, r) must not be effective.
#[test]
fn prop_b_hand_case_minimal() {
    let id5 = FactId::explicit(5);
    let id4 = FactId::explicit(4);
    let id1 = FactId::explicit(1);

    let add   = Fact::with_explicit_id(id5, "node0".into(), 1, vec![],
                    FactPayload::AddMember("alice".into()));
    let grant = Fact::with_explicit_id(id4, "node0".into(), 2, vec![id5],
                    FactPayload::GrantRole("alice".into(), "admin".into()));
    let revoke = Fact::with_explicit_id(id1, "node0".into(), 3, vec![id4],
                    FactPayload::RevokeRole("alice".into(), "admin".into()));

    // All six orderings must agree.
    let all = [add.clone(), grant.clone(), revoke.clone()];
    let perms: &[&[usize]] = &[
        &[0,1,2], &[0,2,1], &[1,0,2], &[1,2,0], &[2,0,1], &[2,1,0],
    ];
    let baseline = fold(&all).unwrap();
    for perm in perms {
        let ordered: Vec<Fact> = perm.iter().map(|&i| all[i].clone()).collect();
        let s = fold(&ordered).unwrap();
        assert_eq!(s.fingerprint(), baseline.fingerprint(),
            "Permutation {:?} diverged", perm);
    }

    // The revoke (causally last) must win despite having id=1 < id=4 (grant).
    assert!(baseline.members.contains("alice"), "alice should be a member");
    assert!(!baseline.effective_roles.contains(&("alice".into(), "admin".into())),
        "role must not be effective: revoke is causally after grant");
}

/// Broken id-only fold fails Property B (documented; broken variant not shipped).
///
/// This test demonstrates that the v1 highest-id-wins rule is wrong.
/// We inline the broken logic to confirm the failure without keeping broken code.
#[test]
fn prop_b_broken_id_only_fold_fails() {
    // Build the minimal Property B case.
    let id5 = FactId::explicit(5);
    let id4 = FactId::explicit(4);
    let id1 = FactId::explicit(1);
    let add   = Fact::with_explicit_id(id5, "node0".into(), 1, vec![],
                    FactPayload::AddMember("alice".into()));
    let grant = Fact::with_explicit_id(id4, "node0".into(), 2, vec![id5],
                    FactPayload::GrantRole("alice".into(), "admin".into()));
    let revoke = Fact::with_explicit_id(id1, "node0".into(), 3, vec![id4],
                    FactPayload::RevokeRole("alice".into(), "admin".into()));

    // Simulate the broken id-only fold: highest FactId wins per slot.
    let role_winner_id = id4.max(id1); // id4 wins (broken: ignores causal order)
    let broken_role_granted = role_winner_id == id4;  // true — grant "wins"

    // The broken fold would say the grant wins the role slot.
    // Under the real fold, the revoke wins (causally later).
    assert!(broken_role_granted,
        "The id-only rule selects the grant (id4 > id1), which is wrong");

    // The correct fold gives the opposite result.
    let correct = fold(&[add, grant, revoke]).unwrap();
    assert!(!correct.effective_roles.contains(&("alice".into(), "admin".into())),
        "The R1-R4 fold correctly selects the causally-later revoke");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// For any two distinct explicit ids (hi > lo):
    ///   early fact: id=hi, no preds
    ///   late fact:  id=lo, preds={hi}   ← causally after early, smaller id
    /// The late fact must win the member slot. Both orderings must agree.
    #[test]
    fn prop_b_causal_beats_id_proptest(a in 1u64..u64::MAX, b in 0u64..u64::MAX) {
        prop_assume!(a != b);
        let (hi, lo) = if a > b { (a, b) } else { (b, a) };
        let early_id = FactId::explicit(hi);
        let late_id  = FactId::explicit(lo);

        let early = Fact::with_explicit_id(early_id, "node0".into(), 1, vec![],
                        FactPayload::AddMember("m".into()));
        let late  = Fact::with_explicit_id(late_id,  "node0".into(), 2, vec![early_id],
                        FactPayload::RemoveMember("m".into()));

        // late is causally after early; despite lo < hi, late must win.
        let s1 = fold(&[early.clone(), late.clone()]).unwrap();
        let s2 = fold(&[late.clone(), early.clone()]).unwrap();

        prop_assert_eq!(s1.fingerprint(), s2.fingerprint(), "Not permutation-invariant");
        prop_assert!(!s1.members.contains("m"),
            "Causally-later RemoveMember (id={lo}) must beat AddMember (id={hi})");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY C (new, R1): concurrent tiebreak determinism
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn prop_c_hand_case_concurrent_tiebreak() {
    // Two concurrent SetThreshold facts on the same role (both tier 1).
    // Among concurrent same-tier facts, winner = greater FactId (R1 tiebreak).
    let id_hi = FactId::explicit(10);
    let id_lo = FactId::explicit(3);

    let t_hi = Fact::with_explicit_id(id_hi, "node0".into(), 1, vec![],
                   FactPayload::SetThreshold("admin".into(), 2, 3));
    let t_lo = Fact::with_explicit_id(id_lo, "node1".into(), 1, vec![],
                   FactPayload::SetThreshold("admin".into(), 1, 2));

    let s1 = fold(&[t_hi.clone(), t_lo.clone()]).unwrap();
    let s2 = fold(&[t_lo.clone(), t_hi.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint(), "Must be permutation-invariant");
    // id_hi (10) > id_lo (3) → t_hi wins → threshold is (2, 3)
    assert_eq!(s1.thresholds.get("admin"), Some(&(2, 3)),
        "C: greater-id SetThreshold wins same-tier concurrent tiebreak");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// Concurrent same-tier facts on the same slot resolve to the tiebreak winner
    /// (greatest FactId) and are permutation-invariant.
    ///
    /// Uses two concurrent SetThreshold facts (both tier 1) on the same role;
    /// the greater-id fact's (k, n) values must appear in the result.
    #[test]
    fn prop_c_concurrent_tiebreak_proptest(a: u64, b: u64, ka: u8, na: u8, kb: u8, nb: u8) {
        prop_assume!(a != b);
        let id1 = FactId::explicit(a);
        let id2 = FactId::explicit(b);
        let na32 = (na % 3 + 2) as u32;
        let ka32 = ((ka % 3 + 1) as u32).min(na32);
        let nb32 = (nb % 3 + 2) as u32;
        let kb32 = ((kb % 3 + 1) as u32).min(nb32);

        // Both concurrent — no predecessors — same tier (SetThreshold = tier 1).
        let t1 = Fact::with_explicit_id(id1, "node0".into(), 1, vec![],
                     FactPayload::SetThreshold("r".into(), ka32, na32));
        let t2 = Fact::with_explicit_id(id2, "node1".into(), 1, vec![],
                     FactPayload::SetThreshold("r".into(), kb32, nb32));

        let s1 = fold(&[t1.clone(), t2.clone()]).unwrap();
        let s2 = fold(&[t2.clone(), t1.clone()]).unwrap();
        prop_assert_eq!(s1.fingerprint(), s2.fingerprint(), "C: not permutation-invariant");

        let (exp_k, exp_n) = if id1 > id2 { (ka32, na32) } else { (kb32, nb32) };
        prop_assert_eq!(s1.thresholds.get("r"), Some(&(exp_k, exp_n)),
            "C: greater-id SetThreshold must win same-tier concurrent tiebreak");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY D (new, R2): role cascade via projection
// ═══════════════════════════════════════════════════════════════════════════════

fn all_permutations_agree(facts: &[Fact]) {
    if facts.is_empty() { return; }
    let baseline = fold(facts).unwrap().fingerprint();
    let n = facts.len();
    // For small n, test all n! orderings via Heap's algorithm via indices.
    let mut indices: Vec<usize> = (0..n).collect();
    heap_permutations(&mut indices, n, &mut |perm: &[usize]| {
        let ordered: Vec<Fact> = perm.iter().map(|&i| facts[i].clone()).collect();
        assert_eq!(fold(&ordered).unwrap().fingerprint(), baseline,
            "Ordering {:?} diverged", perm);
    });
}

fn heap_permutations(arr: &mut Vec<usize>, k: usize, visit: &mut impl FnMut(&[usize])) {
    if k == 1 {
        visit(arr);
        return;
    }
    for i in 0..k {
        heap_permutations(arr, k - 1, visit);
        if k % 2 == 0 { arr.swap(i, k - 1); } else { arr.swap(0, k - 1); }
    }
}

/// D1: causal chain Add → Grant → Remove. m not a member; (m,r) not effective.
#[test]
fn prop_d1_causal_grant_then_remove() {
    let add    = Fact::new("n0".into(), 1, vec![],
                     FactPayload::AddMember("alice".into()));
    let grant  = Fact::new("n0".into(), 2, vec![add.id],
                     FactPayload::GrantRole("alice".into(), "admin".into()));
    let remove = Fact::new("n0".into(), 3, vec![grant.id],
                     FactPayload::RemoveMember("alice".into()));

    let facts = [add, grant, remove];
    all_permutations_agree(&facts);
    let s = fold(&facts).unwrap();
    assert!(!s.members.contains("alice"), "D1: alice should not be a member");
    assert!(!s.effective_roles.contains(&("alice".into(), "admin".into())),
        "D1: role must not be effective after remove");
}

/// D2: concurrent Grant(m,r) and Remove(m), both causally after Add(m).
/// m not a member; (m,r) not effective regardless of which wins the role tiebreak.
#[test]
fn prop_d2_concurrent_grant_and_remove() {
    let add = Fact::new("n0".into(), 1, vec![],
                  FactPayload::AddMember("alice".into()));

    // Test both id orderings for the concurrent pair (grant wins, then remove wins).
    for (grant_id_val, remove_id_val) in [(20u64, 5u64), (5u64, 20u64)] {
        let grant = Fact::with_explicit_id(
            FactId::explicit(grant_id_val), "n1".into(), 1, vec![add.id],
            FactPayload::GrantRole("alice".into(), "admin".into()));
        let remove = Fact::with_explicit_id(
            FactId::explicit(remove_id_val), "n2".into(), 1, vec![add.id],
            FactPayload::RemoveMember("alice".into()));

        let facts = [add.clone(), grant, remove];
        all_permutations_agree(&facts);
        let s = fold(&facts).unwrap();
        assert!(!s.members.contains("alice"),
            "D2 (grant_id={grant_id_val}): alice must not be a member");
        assert!(!s.effective_roles.contains(&("alice".into(), "admin".into())),
            "D2 (grant_id={grant_id_val}): role must not be effective (projection filters it)");
    }
}

/// D3: Add → Grant → Remove → Add2. m is a member; (m,r) NOT effective.
/// Re-add does not restore the prior grant.
#[test]
fn prop_d3_readd_does_not_restore_role() {
    let add1   = Fact::new("n0".into(), 1, vec![],
                     FactPayload::AddMember("alice".into()));
    let grant  = Fact::new("n0".into(), 2, vec![add1.id],
                     FactPayload::GrantRole("alice".into(), "admin".into()));
    let remove = Fact::new("n0".into(), 3, vec![grant.id],
                     FactPayload::RemoveMember("alice".into()));
    let add2   = Fact::new("n0".into(), 4, vec![remove.id],
                     FactPayload::AddMember("alice".into()));

    let facts = [add1, grant, remove, add2];
    all_permutations_agree(&facts);
    let s = fold(&facts).unwrap();
    assert!(s.members.contains("alice"), "D3: alice is a member after re-add");
    assert!(!s.effective_roles.contains(&("alice".into(), "admin".into())),
        "D3: prior grant was revoked by remove; re-add does not restore it");
}

/// D4: extends D3 with Grant2 causally after Add2. m is a member; (m,r) IS effective.
#[test]
fn prop_d4_regrant_after_readd_works() {
    let add1   = Fact::new("n0".into(), 1, vec![],
                     FactPayload::AddMember("alice".into()));
    let grant1 = Fact::new("n0".into(), 2, vec![add1.id],
                     FactPayload::GrantRole("alice".into(), "admin".into()));
    let remove = Fact::new("n0".into(), 3, vec![grant1.id],
                     FactPayload::RemoveMember("alice".into()));
    let add2   = Fact::new("n0".into(), 4, vec![remove.id],
                     FactPayload::AddMember("alice".into()));
    let grant2 = Fact::new("n0".into(), 5, vec![add2.id],
                     FactPayload::GrantRole("alice".into(), "admin".into()));

    let facts = [add1, grant1, remove, add2, grant2];
    all_permutations_agree(&facts);
    let s = fold(&facts).unwrap();
    assert!(s.members.contains("alice"), "D4: alice is a member");
    assert!(s.effective_roles.contains(&("alice".into(), "admin".into())),
        "D4: fresh grant after re-add should be effective");
}

// D5: permutation-invariance for each D scenario is already asserted inside
// D1-D4 via `all_permutations_agree`. This proptest is an additional sweep
// over randomly generated sets to confirm the property at scale.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    #[test]
    fn prop_d5_role_cascade_permutation_invariance(
        facts        in fact_set_strategy(),
        shuffle_seed in any::<u64>(),
    ) {
        let baseline = fold(&facts).unwrap().fingerprint();
        let mut rng = SmallRng::seed_from_u64(shuffle_seed);
        for _ in 0..5u8 {
            let mut permuted = facts.clone();
            permuted.shuffle(&mut rng);
            let result = fold(&permuted).unwrap().fingerprint();
            prop_assert_eq!(baseline, result,
                "effective_roles not permutation-invariant");
        }
    }
}

/// D6: A12 type precedence — concurrent RemoveMember (tier 2) beats AddMember
/// (tier 5) regardless of FactId order. Discriminating test against flat fold.
///
///   Add(m)    [tier 5, explicit id=100]  — concurrent with Remove
///   Remove(m) [tier 2, explicit id=5]   — concurrent with Add
///
/// A12: RemoveMember (tier 2) wins despite id=5 < id=100. m not a member.
/// Flat id-only fold: Add (id=100) would win. m is a member. — WRONG.
#[test]
fn prop_d6_type_precedence_beats_id_order() {
    let add_id = FactId::explicit(100);
    let rem_id = FactId::explicit(5);

    let add    = Fact::with_explicit_id(add_id, "n0".into(), 1, vec![],
                     FactPayload::AddMember("alice".into()));
    let remove = Fact::with_explicit_id(rem_id, "n1".into(), 1, vec![],
                     FactPayload::RemoveMember("alice".into()));

    let s1 = fold(&[add.clone(), remove.clone()]).unwrap();
    let s2 = fold(&[remove.clone(), add.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint(), "D6: must be permutation-invariant");

    // A12: RemoveMember (tier 2) wins over AddMember (tier 5) despite id=100 > id=5.
    assert!(!s1.members.contains("alice"),
        "D6: type precedence must win; RemoveMember (tier 2) beats AddMember (tier 5)");

    // Confirm what a flat id-only fold would do — the discriminating check.
    let flat_is_member = add_id > rem_id; // id=100 > id=5 → AddMember would win
    assert!(flat_is_member,
        "D6: flat fold would incorrectly select AddMember (id=100 > id=5)");
}

/// D7: A12 cascade precedence — concurrent RemoveMember cascade (tier 2) beats
/// GrantRole (tier 4) in the role slot even when the member is later re-added.
///
///   Add1(m)    [no preds]
///   Grant(m,r) [preds={Add1}, tier 4, id=20] — concurrent with Remove
///   Remove(m)  [preds={Add1}, tier 2, id=5]  — concurrent with Grant; cascades to role slot
///   Add2(m)    [preds={Remove}]              — causally after Remove; re-adds m
///
/// Member slot: Add2 causally last → alice IS a member.
/// Role slot:   Remove cascade (tier 2) beats Grant (tier 4) → NOT granted.
/// Effective:   member but role not granted → NOT effective.
///
/// Flat id-only fold (no A12): Grant (id=20 > id=5) wins role → (alice,admin) IS effective — WRONG.
#[test]
fn prop_d7_remove_cascade_tier_beats_grant_id() {
    let add1 = Fact::new("n0".into(), 1, vec![],
                    FactPayload::AddMember("alice".into()));

    let grant_id = FactId::explicit(20);
    let rem_id   = FactId::explicit(5);

    let grant  = Fact::with_explicit_id(grant_id, "n1".into(), 1, vec![add1.id],
                     FactPayload::GrantRole("alice".into(), "admin".into()));
    let remove = Fact::with_explicit_id(rem_id,   "n2".into(), 1, vec![add1.id],
                     FactPayload::RemoveMember("alice".into()));
    let add2   = Fact::new("n3".into(), 1, vec![remove.id],
                     FactPayload::AddMember("alice".into()));

    let facts = [add1, grant, remove, add2];
    all_permutations_agree(&facts);
    let s = fold(&facts).unwrap();

    assert!(s.members.contains("alice"),
        "D7: alice is a member (Add2 causally after Remove)");
    assert!(!s.effective_roles.contains(&("alice".into(), "admin".into())),
        "D7: RemoveMember cascade (tier 2) beats GrantRole (tier 4) by A12; \
         role not effective despite alice being a member");

    // Confirm what a flat fold would do — the discriminating check.
    let flat_is_granted = grant_id > rem_id; // id=20 > id=5 → Grant would win
    assert!(flat_is_granted,
        "D7: flat fold would incorrectly select GrantRole (id=20 > id=5)");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY E (new, R3): idempotent no-ops
// ═══════════════════════════════════════════════════════════════════════════════

/// E1: fold { Remove(m) } with no Add(m). m not a member; no error.
#[test]
fn prop_e1_remove_never_added() {
    let remove = Fact::new("n0".into(), 1, vec![],
                     FactPayload::RemoveMember("ghost".into()));
    let s = fold(&[remove]).unwrap();
    assert!(!s.members.contains("ghost"), "E1: never-added member must not appear");
}

/// E2: fold { Revoke(m,r) } with no Grant. (m,r) not effective; no error.
#[test]
fn prop_e2_revoke_never_granted() {
    let revoke = Fact::new("n0".into(), 1, vec![],
                     FactPayload::RevokeRole("ghost".into(), "admin".into()));
    let s = fold(&[revoke]).unwrap();
    assert!(!s.effective_roles.contains(&("ghost".into(), "admin".into())),
        "E2: never-granted role must not be effective");
}

/// E3: fold { Remove(m) } and fold { Add(m), Remove(m) } agree on membership.
#[test]
fn prop_e3_remove_after_add_same_as_remove_only() {
    let add_then_remove = {
        let add    = Fact::new("n0".into(), 1, vec![],
                         FactPayload::AddMember("eve".into()));
        let remove = Fact::new("n0".into(), 2, vec![add.id],
                         FactPayload::RemoveMember("eve".into()));
        fold(&[add, remove]).unwrap()
    };
    let remove_only = {
        let remove = Fact::new("n0".into(), 1, vec![],
                         FactPayload::RemoveMember("eve".into()));
        fold(&[remove]).unwrap()
    };

    assert_eq!(add_then_remove.members.contains("eve"),
               remove_only.members.contains("eve"),
        "E3: both cases must agree that eve is not a member");
    assert!(!add_then_remove.members.contains("eve"), "eve must not be a member");
}

// ═══════════════════════════════════════════════════════════════════════════════
// PROPERTY F (new, R4): threshold LWW via R1
// ═══════════════════════════════════════════════════════════════════════════════

/// F1: causal chain SetThreshold(2,3) → SetThreshold(3,5). Result: (3,5).
#[test]
fn prop_f1_causal_threshold_lww() {
    let t1 = Fact::new("n0".into(), 1, vec![],
                  FactPayload::SetThreshold("admin".into(), 2, 3));
    let t2 = Fact::new("n0".into(), 2, vec![t1.id],
                  FactPayload::SetThreshold("admin".into(), 3, 5));

    let s1 = fold(&[t1.clone(), t2.clone()]).unwrap();
    let s2 = fold(&[t2.clone(), t1.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint(), "F1: must be permutation-invariant");
    assert_eq!(s1.thresholds.get("admin"), Some(&(3, 5)),
        "F1: causally-later threshold must win");
}

/// F1 explicit-id variant: causally-later threshold has smaller id.
#[test]
fn prop_f1_causal_threshold_smaller_id_wins() {
    let id_hi = FactId::explicit(100);
    let id_lo = FactId::explicit(1);

    let t1 = Fact::with_explicit_id(id_hi, "n0".into(), 1, vec![],
                 FactPayload::SetThreshold("admin".into(), 2, 3));
    let t2 = Fact::with_explicit_id(id_lo, "n0".into(), 2, vec![id_hi],
                 FactPayload::SetThreshold("admin".into(), 3, 5));

    let s1 = fold(&[t1.clone(), t2.clone()]).unwrap();
    let s2 = fold(&[t2.clone(), t1.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint());
    assert_eq!(s1.thresholds.get("admin"), Some(&(3, 5)),
        "F1-explicit: causally-later (id=1) must beat id=100");
}

/// F2: concurrent SetThreshold — tiebreak winner's value, permutation-invariant.
#[test]
fn prop_f2_concurrent_threshold_tiebreak() {
    let t1 = Fact::new("n0".into(), 1, vec![],
                  FactPayload::SetThreshold("admin".into(), 2, 3));
    let t2 = Fact::new("n1".into(), 1, vec![],
                  FactPayload::SetThreshold("admin".into(), 1, 2));

    let s_ab = fold(&[t1.clone(), t2.clone()]).unwrap();
    let s_ba = fold(&[t2.clone(), t1.clone()]).unwrap();
    assert_eq!(s_ab.fingerprint(), s_ba.fingerprint(), "F2: must be permutation-invariant");

    let (exp_k, exp_n) = if t1.id > t2.id { (2, 3) } else { (1, 2) };
    assert_eq!(s_ab.thresholds.get("admin"), Some(&(exp_k, exp_n)),
        "F2: tiebreak winner (greatest FactId) must determine the threshold");
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(500))]

    /// Concurrent SetThreshold on the same role resolves to the greater-id winner.
    #[test]
    fn prop_f2_threshold_tiebreak_proptest(a: u64, b: u64, ka: u8, na: u8, kb: u8, nb: u8) {
        prop_assume!(a != b);
        let id1 = FactId::explicit(a);
        let id2 = FactId::explicit(b);
        let na32 = (na % 3 + 2) as u32;
        let ka32 = ((ka % 3 + 1) as u32).min(na32);
        let nb32 = (nb % 3 + 2) as u32;
        let kb32 = ((kb % 3 + 1) as u32).min(nb32);

        let t1 = Fact::with_explicit_id(id1, "n0".into(), 1, vec![],
                     FactPayload::SetThreshold("r".into(), ka32, na32));
        let t2 = Fact::with_explicit_id(id2, "n1".into(), 1, vec![],
                     FactPayload::SetThreshold("r".into(), kb32, nb32));

        let s1 = fold(&[t1.clone(), t2.clone()]).unwrap();
        let s2 = fold(&[t2.clone(), t1.clone()]).unwrap();
        prop_assert_eq!(s1.fingerprint(), s2.fingerprint(), "F2: not permutation-invariant");

        let (exp_k, exp_n) = if id1 > id2 { (ka32, na32) } else { (kb32, nb32) };
        prop_assert_eq!(s1.thresholds.get("r"), Some(&(exp_k, exp_n)),
            "F2: greater-id winner must determine threshold");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Retained hand cases (updated for v2 semantics)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn empty_facts_yields_empty_state() {
    assert_eq!(fold(&[]).unwrap(), AuthorityState::empty());
}

#[test]
fn single_add_member() {
    let fact = Fact::new("n0".into(), 1, vec![],
                   FactPayload::AddMember("alice".into()));
    let s = fold(&[fact]).unwrap();
    assert!(s.members.contains("alice"));
    assert_eq!(s.members.len(), 1);
    assert!(s.effective_roles.is_empty());
    assert!(s.thresholds.is_empty());
}

#[test]
fn two_independent_adds() {
    let f1 = Fact::new("n0".into(), 1, vec![], FactPayload::AddMember("alice".into()));
    let f2 = Fact::new("n1".into(), 1, vec![], FactPayload::AddMember("bob".into()));
    let s = fold(&[f1, f2]).unwrap();
    assert!(s.members.contains("alice"));
    assert!(s.members.contains("bob"));
    assert_eq!(s.members.len(), 2);
}

/// Concurrent add/remove for the same member must resolve deterministically.
#[test]
fn concurrent_add_remove_same_member() {
    let add    = Fact::new("n0".into(), 1, vec![], FactPayload::AddMember("bob".into()));
    let remove = Fact::new("n1".into(), 1, vec![], FactPayload::RemoveMember("bob".into()));
    let s1 = fold(&[add.clone(), remove.clone()]).unwrap();
    let s2 = fold(&[remove.clone(), add.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint());
}

/// Two SetThreshold on different roles — both appear in the final state.
#[test]
fn two_concurrent_thresholds_different_roles() {
    let f1 = Fact::new("n0".into(), 1, vec![],
                  FactPayload::SetThreshold("admin".into(), 2, 3));
    let f2 = Fact::new("n1".into(), 1, vec![],
                  FactPayload::SetThreshold("auditor".into(), 1, 3));
    let s1 = fold(&[f1.clone(), f2.clone()]).unwrap();
    let s2 = fold(&[f2.clone(), f1.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint());
    assert_eq!(s1.thresholds.get("admin"),   Some(&(2, 3)));
    assert_eq!(s1.thresholds.get("auditor"), Some(&(1, 3)));
}

/// Causally-ordered add then remove is deterministic (R1: remove wins as causally later).
#[test]
fn causally_ordered_add_then_remove() {
    let add    = Fact::new("n0".into(), 1, vec![],
                     FactPayload::AddMember("charlie".into()));
    let remove = Fact::new("n0".into(), 2, vec![add.id],
                     FactPayload::RemoveMember("charlie".into()));
    let s1 = fold(&[add.clone(), remove.clone()]).unwrap();
    let s2 = fold(&[remove.clone(), add.clone()]).unwrap();
    assert_eq!(s1.fingerprint(), s2.fingerprint());
    assert!(!s1.members.contains("charlie"), "Remove is causally after add; must win");
}

/// Grant without AddMember: (m,r) not effective (R2 projection: m not a member).
#[test]
fn grant_without_prior_add_not_effective() {
    let grant = Fact::new("n0".into(), 1, vec![],
                    FactPayload::GrantRole("alice".into(), "admin".into()));
    let s = fold(&[grant]).unwrap();
    assert!(!s.effective_roles.contains(&("alice".into(), "admin".into())),
        "Role must not be effective without membership");
    // alice is not even in members
    assert!(!s.members.contains("alice"));
}
