//! G1 — the completeness guard, verified through the real ingest pipeline
//! (Battery 5 / the G1 fix, Rung B).
//!
//! This experiment originally *refuted* the unstated completeness precondition of
//! I5: every `AssertionEnvelope` declares its `antecedents` (the hashes of the
//! assertions it causally follows), but the fold never read them, so a node that
//! missed a *cross-device* predecessor would admit the dependent fact anyway and
//! fold to a head no complete peer ever held — a silent divergence that survived
//! the `Replicator` (whose per-device contiguity cannot see a cross-device
//! dependency). See git history / the plain-English findings for that result.
//!
//! The fold now carries the guard (`fold_derived::ingest`, Step 5.5): a fact whose
//! declared antecedents are not all present is HELD BACK (`FoldError::
//! MissingAntecedents`), which the Replicator's existing retry buffers until the
//! predecessor arrives. This test verifies the closed behavior.
//!
//! Scenario (one group; owner O and a second admin A2):
//!   O:  genesis (lamport 1)                          — O becomes Owner
//!   O:  α  MembershipAdd A2 as Admin (lamport 2)
//!   O:  R  RuleChange remove_member_threshold 1 → 7 (lamport 3)
//!   A2: β  MembershipAdd X as Member (lamport 1), antecedents = [hash(R)]
//!
//! Node A receives all four. Node B drops R. β declares R as its antecedent, so on
//! B it is now held back rather than admitted.
//!
//! VERIFICATION: node B's head is a strict PREFIX of A's — the {genesis, α} state A
//! itself passed through (O Owner, A2 Admin, threshold 1, no X) — not a divergent
//! head. X is absent on B because β is held pending its missing predecessor, and A
//! is complete (threshold 7, X present).

mod common;

use common::{
    base, drive, frame, genesis_payload, has_member, membership_add_payload, rule_change_payload,
    sign,
};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::Identity;

#[tokio::test]
async fn completeness_gap_survives_the_pipeline() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x77; 32]);

    let id_o = Identity::from_seed([0x01; 32]); // owner
    let id_a2 = Identity::from_seed([0x02; 32]); // second admin
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let x_principal = PrincipalId::new([0x33; 32]); // added member (never signs)
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(
        &id_o,
        base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)),
    );
    let alpha = sign(
        &id_o,
        base(
            &id_o,
            group,
            AssertionType::MembershipAdd,
            2,
            vec![],
            membership_add_payload(a2_principal, 1), // Admin
        ),
    );
    let r = sign(
        &id_o,
        base(&id_o, group, AssertionType::RuleChange, 3, vec![], rule_change_payload(1, 7)),
    );
    let beta = sign(
        &id_a2,
        base(
            &id_a2,
            group,
            AssertionType::MembershipAdd,
            1, // A2's own chain starts at 1
            vec![envelope_hash(&r)], // β declares R as its antecedent
            membership_add_payload(x_principal, 2), // Member
        ),
    );

    // The dependency the wire records: β's antecedent IS R's hash.
    assert_eq!(
        beta.antecedents,
        vec![envelope_hash(&r)],
        "β must declare R as its antecedent (the ignored completeness signal)"
    );

    // Frames injected scrambled (β before its prerequisites) to also exercise the
    // Replicator's retry — proving β's admission is genuine, not an ordering fluke.
    let authors = [&id_o, &id_a2];
    let sess_a = drive(
        &dir.path().join("a.redb"),
        &id_o,
        &authors,
        vec![frame(&beta), frame(&genesis), frame(&r), frame(&alpha)],
    );
    let sess_b = drive(
        &dir.path().join("b.redb"),
        &id_o,
        &authors,
        vec![frame(&beta), frame(&genesis), frame(&alpha)], // R dropped
    );

    let sum_a = sess_a.get_group_summary(&group).expect("summary A");
    let sum_b = sess_b.get_group_summary(&group).expect("summary B");

    // A is complete: R applied (threshold 7) and β applied (X present).
    assert!(has_member(&sess_a, &group, &x_principal), "A admitted β (complete set)");
    assert_eq!(sum_a.rules.remove_member_threshold, 7, "A saw R");

    // THE GUARD: on B, β is HELD BACK because its antecedent R was never delivered.
    // X is therefore absent — B did not admit a fact whose predecessor it lacks.
    assert!(
        !has_member(&sess_b, &group, &x_principal),
        "B held β back (its antecedent R is absent) — X must NOT be admitted. If this \
         fails, the completeness guard has regressed and the G1 gap is back."
    );

    // B's head is a strict PREFIX of A's — the {genesis, α} state A itself passed
    // through, not a head A never held. A2 is still a member (α was delivered and
    // has no missing antecedent); only β (and R) are absent.
    assert!(has_member(&sess_b, &group, &a2_principal), "α applied on B (no gap): A2 is a member");
    assert_eq!(sum_b.rules.remove_member_threshold, 1, "B is behind (R absent), at the pre-R value");
    assert_eq!(sum_b.fork_status, "clean", "B is a clean prefix — stale but honest, not divergent");

    eprintln!(
        "G1 RESULT (guard verified): through the real Replicator+Session+fold, node B \
         HELD BACK β because its cross-device antecedent R was never delivered — X is \
         absent, B's head is the {{genesis, α}} prefix A itself passed through (threshold \
         1, A2 a member), not a divergent head. The gap is closed at the fold."
    );
}
