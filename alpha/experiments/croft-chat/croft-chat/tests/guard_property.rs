//! G1-guard as a property: prefix-closure under random cross-device loss
//! (Battery 6, Rung B).
//!
//! G1/G2 verify the antecedent guard on one hand-built scenario. This lifts it to a
//! property. Build a linear causal chain of governance facts f0→f1→…→f(N-1), where
//! each f_i declares f_(i-1) as its antecedent, authored by *alternating* devices so
//! every antecedent link crosses a device boundary — exactly the case the
//! Replicator's per-device contiguity cannot cover, so the fold's guard is what has
//! to hold it.
//!
//! Chain: f0 = genesis(O); f1 = O adds A2 as Admin; f_i (i≥2) = the current author
//! (O if i odd, A2 if i even) adds member M_i. f0 and f1 are always delivered (they
//! carry the authority the rest needs).
//!
//! For every drop point d and many random delivery orders of the remaining subset,
//! the node must land at a **contiguous prefix**: members M_2…M_(d-1) present, and
//! M_d…M_(N-1) ABSENT — because f_d is missing and every later fact transitively
//! depends on it, so the guard holds them all back. Plus a no-drop case that must
//! admit every member (liveness).
//!
//! Falsifies if: any node admits a member whose fact transitively depends on a
//! dropped one (the guard leaked), or a complete delivery fails to admit all (the
//! guard over-blocks).

mod common;

use common::{base, drive, frame, genesis_payload, has_member, membership_add_payload, sign};
use croft_chat::transport::Frame;
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{
    AssertionEnvelope, AssertionType, DeviceId, GroupId, Hash, PrincipalId,
};
use social_graph_core::Identity;

/// Deterministic PRNG (splitmix64) — no wall-clock, fully reproducible.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    fn shuffle<T>(&mut self, v: &mut [T]) {
        for i in (1..v.len()).rev() {
            let j = (self.next() % (i as u64 + 1)) as usize;
            v.swap(i, j);
        }
    }
}

const N: usize = 8; // f0..f7

/// Build the causal chain. Returns (facts in index order, member principal per index).
fn build_chain(
    group: GroupId,
    id_o: &Identity,
    id_a2: &Identity,
) -> (Vec<AssertionEnvelope>, Vec<Option<PrincipalId>>) {
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let mut facts: Vec<AssertionEnvelope> = Vec::with_capacity(N);
    let mut members: Vec<Option<PrincipalId>> = Vec::with_capacity(N);
    let mut o_lam = 0u64;
    let mut a2_lam = 0u64;
    let mut prev: Option<Hash> = None;

    for i in 0..N {
        let antecedents: Vec<Hash> = prev.into_iter().collect();
        let env = if i == 0 {
            o_lam += 1;
            members.push(None);
            sign(
                id_o,
                base(id_o, group, AssertionType::GroupGenesis, o_lam, antecedents, genesis_payload(&o_device)),
            )
        } else if i == 1 {
            o_lam += 1;
            members.push(Some(a2_principal)); // f1 makes A2 a member (Admin)
            sign(
                id_o,
                base(id_o, group, AssertionType::MembershipAdd, o_lam, antecedents, membership_add_payload(a2_principal, 1)),
            )
        } else {
            // i >= 2: O authors odd indices, A2 authors even indices.
            let author_is_o = i % 2 == 1;
            let m = PrincipalId::new([0x50 + i as u8; 32]);
            members.push(Some(m));
            let payload = membership_add_payload(m, 2); // Member
            if author_is_o {
                o_lam += 1;
                sign(id_o, base(id_o, group, AssertionType::MembershipAdd, o_lam, antecedents, payload))
            } else {
                a2_lam += 1;
                sign(id_a2, base(id_a2, group, AssertionType::MembershipAdd, a2_lam, antecedents, payload))
            }
        };
        prev = Some(envelope_hash(&env));
        facts.push(env);
    }
    (facts, members)
}

#[tokio::test]
async fn guard_holds_prefix_closure_under_random_loss() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0xC7; 32]);
    let id_o = Identity::from_seed([0x70; 32]);
    let id_a2 = Identity::from_seed([0x71; 32]);
    let authors = [&id_o, &id_a2];

    let (facts, members) = build_chain(group, &id_o, &id_a2);
    let mut rng = Rng(0x5EED_1234_ABCD_0001);
    let mut cases = 0u32;

    // Drop each f_d for d in 2..N; deliver the rest in several random orders.
    for d in 2..N {
        for _ in 0..8 {
            let mut delivered: Vec<Frame> = facts
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != d)
                .map(|(_, e)| frame(e))
                .collect();
            rng.shuffle(&mut delivered);

            let sess = drive(
                &dir.path().join(format!("d{d}_{cases}.redb")),
                &id_o,
                &authors,
                delivered,
            );

            // Prefix-closure: members below the drop are present; the drop and
            // everything after it (transitive dependents) are absent.
            for (i, m) in members.iter().enumerate() {
                let Some(pid) = m else { continue };
                let present = has_member(&sess, &group, pid);
                if i < d {
                    assert!(present, "case d={d}: M_{i} (below drop) must be present");
                } else {
                    assert!(
                        !present,
                        "case d={d}: M_{i} (at/after drop) must be ABSENT — the guard \
                         must hold back any fact transitively depending on the dropped f_{d}"
                    );
                }
            }
            cases += 1;
        }
    }

    // Liveness: a complete delivery (no drop), random order, admits every member.
    for _ in 0..8 {
        let mut all: Vec<Frame> = facts.iter().map(frame).collect();
        rng.shuffle(&mut all);
        let sess = drive(&dir.path().join(format!("full_{cases}.redb")), &id_o, &authors, all);
        for m in members.iter().flatten() {
            assert!(has_member(&sess, &group, m), "complete delivery must admit every member");
        }
        cases += 1;
    }

    eprintln!(
        "GUARD-PROPERTY RESULT (corroboration): across {cases} cases (random \
         cross-device drops + random orders), every node's admitted membership is a \
         contiguous prefix of the causal chain — no fact whose antecedent chain \
         reaches a dropped fact is ever admitted, and complete deliveries admit all. \
         Prefix-closure holds as a property, not just on the G1 scenario."
    );
}
