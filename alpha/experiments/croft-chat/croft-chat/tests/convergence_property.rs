//! V1′ — convergence is a property, not a scenario (Battery 6, Rung B).
//!
//! `convergence.rs` (P7) proves order-insensitive convergence for one hand-built
//! two-node interleaving. This lifts it to a property: over many random delivery
//! orders of a multi-device governance DAG that includes *concurrent* cross-device
//! facts, every independent folder must reach the identical head (fingerprint).
//!
//! The concurrency is the point. O and A2 (both admins) each add members with no
//! causal link between their chains, so the fold must place these concurrent facts
//! in a canonical order that does not depend on arrival — the causal-then-
//! cryptographic total order (§7.3.1), whose tiebreak is the content hash. If that
//! order were arrival-dependent, the member list (and thus the fingerprint) would
//! differ across permutations. It does not.
//!
//! Falsifies if: any permutation of a complete set yields a different fingerprint.

mod common;

use std::collections::HashSet;

use common::{base, drive, frame, genesis_payload, membership_add_payload, sign};
use croft_chat::fingerprint::fingerprint;
use croft_chat::transport::Frame;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::Identity;

/// Deterministic PRNG (splitmix64) — no `rand`, no wall-clock, fully reproducible.
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Fisher–Yates in place.
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

#[tokio::test]
async fn convergence_is_order_independent_over_random_permutations() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC1; 32]);

    let id_o = Identity::from_seed([0x40; 32]);
    let id_a2 = Identity::from_seed([0x41; 32]);
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);
    let authors = [&id_o, &id_a2];

    // A complete DAG with concurrency:
    //   O:  genesis(1), add A2 Admin(2), add X1(3), add X2(4)
    //   A2: add Y1(1), add Y2(2)      — concurrent with O's X-adds
    let members: Vec<PrincipalId> = (0u8..4).map(|i| PrincipalId::new([0x50 + i; 32])).collect();
    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let grant_a2 = sign(
        &id_o,
        base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2_principal, 1)),
    );
    let o_add_x1 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(members[0], 2)));
    let o_add_x2 = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 4, vec![], membership_add_payload(members[1], 2)));
    let a2_add_y1 = sign(&id_a2, base(&id_a2, group, AssertionType::MembershipAdd, 1, vec![], membership_add_payload(members[2], 2)));
    let a2_add_y2 = sign(&id_a2, base(&id_a2, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(members[3], 2)));

    let dag: Vec<Frame> = [
        &genesis, &grant_a2, &o_add_x1, &o_add_x2, &a2_add_y1, &a2_add_y2,
    ]
    .iter()
    .map(|e| frame(e))
    .collect();

    // Feed 64 random permutations to 64 independent folders; collect their heads.
    let mut rng = SplitMix64(0x0DDB_A11C_0FFE_E123);
    let mut heads: HashSet<String> = HashSet::new();
    let mut reference: Option<String> = None;
    const PERMUTATIONS: usize = 64;
    for k in 0..PERMUTATIONS {
        let mut order = dag.clone();
        rng.shuffle(&mut order);
        let sess = drive(&dir.path().join(format!("perm{k}.redb")), &id_o, &authors, order);

        // Every member must be present regardless of arrival order.
        let sum = sess.get_group_summary(&group).expect("summary");
        for m in &members {
            assert!(
                sum.members.iter().any(|x| &x.principal == m),
                "permutation {k}: member {m:?} missing"
            );
        }
        let fp = fingerprint(&sess, &group);
        reference.get_or_insert_with(|| fp.clone());
        heads.insert(fp);
    }

    assert_eq!(
        heads.len(),
        1,
        "all {PERMUTATIONS} permutations must converge to one head; got {} distinct: the \
         canonical (causal-then-cryptographic) order is arrival-dependent",
        heads.len()
    );

    eprintln!(
        "V1′ RESULT (corroboration): {PERMUTATIONS} random delivery orders of a \
         6-fact, 2-device DAG with concurrent cross-device adds all converge to one \
         head ({}). Order-independence holds as a property, and the concurrent facts \
         are placed by a canonical order that does not depend on arrival.",
        reference.unwrap_or_default()
    );
}
