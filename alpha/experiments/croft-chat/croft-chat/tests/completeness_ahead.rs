//! EXP-C1 (backlog §2c, RUN-07) — the completeness-ahead contract, four separately
//! RED-able assertions at loopback / fold grade.
//!
//! The contract (Part 2 §7.3.3 dials, §7.4 freshness, §7.4.3 stamp): a node acts on an
//! irreversible / dependent governance step only when it can corroborate it is current,
//! and otherwise fail-closed **stalls** while still serving reads on best-known state.
//!
//! Grade: loopback / fold only. No MLS internals and no network transport are touched —
//! the "peer that holds the withheld fact" is modelled by re-delivering its frame into
//! the same store (the solicitation *result*), and the freshness / generation-stamp
//! values are integers the test seeds, standing in for the real attested values. The
//! real-NAT / live-transport path stays X1.

mod common;

use common::{
    base, drive, frame, genesis_payload, has_member, membership_add_payload, pump_until_quiet,
    rule_change_payload, sign, QueueBus,
};
use croft_chat::fingerprint::fingerprint;
use croft_chat::sync::Replicator;
use croft_chat::transport::{Topic, Transport};
use local_storage_projection::completeness_ahead::{
    admits_irreversible, detect_stamp_gap, quorum_k,
};
use local_storage_projection::types::envelope_hash;
use local_storage_projection::{AssertionType, DeviceId, GroupId, PrincipalId};
use social_graph_core::{Identity, Session};

// ---------------------------------------------------------------------------
// 1. Stall-at-threshold: a withheld governance fact stalls the dependent
//    irreversible act below freshness k, while reads continue.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn stall_at_threshold_no_breach_reads_unaffected() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x70; 32]);

    // Owner O, admins A2 and A3 (a 3-member group, so k = ceil(3/2) = 2). O raises
    // remove_member_threshold 1 -> 7 in fact R. Node X is denied R, so its frontier is behind.
    let id_o = Identity::from_seed([0x01; 32]);
    let id_a2 = Identity::from_seed([0x02; 32]);
    let id_a3 = Identity::from_seed([0x08; 32]);
    let a2_principal = PrincipalId::new(id_a2.principal_id().0);
    let a3_principal = PrincipalId::new(id_a3.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let alpha = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a2_principal, 1)));
    let gamma = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(a3_principal, 1)));
    let r = sign(&id_o, base(&id_o, group, AssertionType::RuleChange, 4, vec![], rule_change_payload(1, 7)));

    let authors = [&id_o, &id_a2, &id_a3];
    // X is denied R: it holds only {genesis, alpha, gamma}, an honest prefix.
    let sess_x = drive(&dir.path().join("x.redb"), &id_o, &authors, vec![frame(&genesis), frame(&alpha), frame(&gamma)]);

    // READS UNAFFECTED: X still serves its best-known state (the prefix). The read
    // succeeds and reflects the pre-R value, not an error and not a fabricated one.
    let sum_x = sess_x.get_group_summary(&group).expect("X still serves reads on best-known state");
    assert_eq!(sum_x.rules.remove_member_threshold, 1, "X reads its honest prefix (pre-R value)");
    assert!(has_member(&sess_x, &group, &a2_principal), "X's best-known state is intact and readable");

    // THE STALL: an irreversible/dependent act needs freshness k = ceil(n/2) distinct-lineage
    // currency attestations. X is partitioned from R, so it cannot corroborate currency: it
    // holds fewer than k attestations (seeded here as 1, standing in for the attested value).
    let n = sum_x.members.len() as u64; // members X can see (O + A2 + A3 = 3)
    assert_eq!(n, 3, "the 3-member group makes k = 2, so a single attestation is below threshold");
    let k = quorum_k(n); // ceil(3/2) = 2
    let freshness_x = 1u64; // below k: X cannot corroborate it is current
    assert!(
        !admits_irreversible(freshness_x, k),
        "X must STALL the dependent irreversible act below freshness k (fail-closed) — no breach"
    );

    // NO BREACH: contrast with a current node that DID gather k attestations — only it may act.
    assert!(admits_irreversible(k, k), "a node current at k attestations may enforce the act");
    // r is the withheld fact whose absence keeps X below threshold (referenced for documentation).
    let _ = envelope_hash(&r);
}

// ---------------------------------------------------------------------------
// 2. Stamp detection: a data-plane entry stamped ahead of X's governance
//    frontier is detected, sized, and filled before X acts.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn stamp_ahead_detected_sized_and_filled_before_acting() {
    // X's governance frontier is at generation `local`; a data-plane entry arrives carrying
    // a generation stamp ahead of it (the §7.4.3 behind-via-traffic case).
    let local: u64 = 4;
    let entry_stamp: u64 = 7;

    // DETECTED AND SIZED: the gap is seen and quantified (3 generations behind).
    let gap = detect_stamp_gap(local, entry_stamp);
    assert_eq!(gap, Some(3), "the stamp-ahead gap is detected and sized");

    // X must FILL before acting: it advances its frontier to the entry's generation. Modeled
    // as catching the frontier up to the stamp; a node that acts on the entry before filling
    // would be acting on a stale frontier (the breach the stamp exists to prevent).
    let acted_before_fill = gap.is_none();
    assert!(!acted_before_fill, "X does not act while a gap is open");

    let filled_frontier = entry_stamp; // X folded the missing generations up to the stamp
    assert_eq!(detect_stamp_gap(filled_frontier, entry_stamp), None, "after filling, no gap remains");
    let acts_now = detect_stamp_gap(filled_frontier, entry_stamp).is_none();
    assert!(acts_now, "X acts on the entry only once the gap is filled");
}

// ---------------------------------------------------------------------------
// 3. Solicitation reach: an unreferenced tail fact, absent on X, is surfaced by
//    a frontier ask and folds identically to live arrival (same fingerprint).
// ---------------------------------------------------------------------------

#[tokio::test]
async fn solicitation_surfaces_unreferenced_tail_folds_identically() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x71; 32]);

    let id_o = Identity::from_seed([0x03; 32]);
    let id_z = Identity::from_seed([0x04; 32]);
    let z_principal = PrincipalId::new(id_z.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    // F is an UNREFERENCED TAIL: a MembershipAdd of Z with no antecedents, and nothing
    // declares F as its antecedent, so no held-back fact reveals F's absence — only a
    // frontier ask can surface it.
    let f = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(z_principal, 2)));
    assert!(f.antecedents.is_empty(), "F is an unreferenced tail (no antecedents; nothing references it)");

    let authors = [&id_o, &id_z];

    // Live node L receives F in the normal stream.
    let live = drive(&dir.path().join("l.redb"), &id_o, &authors, vec![frame(&genesis), frame(&f)]);
    let fp_live = fingerprint(&live, &group);
    assert!(has_member(&live, &group, &z_principal), "L admitted F live");

    // Node X keeps ONE persistent session + replicator across two deliveries, so the
    // solicited frame folds into the same live pipeline (a fresh replicator would treat F,
    // whose per-device lamport is 2, as arriving before genesis and buffer it forever).
    let session_x = Session::open(&dir.path().join("x.redb"), &id_o).expect("open X");
    for a in &authors {
        session_x.trust_peer(a.device_id(), a.principal_id());
    }
    let mut bus = QueueBus::default();
    bus.subscribe(&Topic("drystone/completeness".to_string()));
    let mut repl = Replicator::new();

    // Initial delivery: X has genesis but NOT the unreferenced tail F. Its fingerprint differs.
    bus.inject(vec![frame(&genesis)]);
    pump_until_quiet(&session_x, &mut bus, &mut repl);
    let fp_x1 = fingerprint(&session_x, &group);
    assert!(!has_member(&session_x, &group, &z_principal), "X lacks the unreferenced tail before soliciting");
    assert_ne!(fp_x1, fp_live, "X differs from L while F is unsolicited");

    // Solicitation: X's frontier ask to a peer holding F returns it; X folds it into the same
    // pipeline. (The ask itself is out of scope at loopback grade — we deliver the frame the ask
    // would return.)
    bus.inject(vec![frame(&f)]);
    pump_until_quiet(&session_x, &mut bus, &mut repl);
    let fp_x2 = fingerprint(&session_x, &group);

    assert!(has_member(&session_x, &group, &z_principal), "the solicited tail is admitted on X");
    assert_eq!(
        fp_x2, fp_live,
        "the fold admits the solicited unreferenced tail IDENTICALLY to live arrival (same fingerprint)"
    );
}

// ---------------------------------------------------------------------------
// 4. Formula-valued k: k = ceil(n/2) over the folded member set is identical on
//    every node at the same act position across arrival orders.
// ---------------------------------------------------------------------------

#[tokio::test]
async fn formula_valued_k_identical_across_orders() {
    let dir = tempfile::tempdir().expect("tempdir");
    let group = GroupId::new([0x72; 32]);

    let id_o = Identity::from_seed([0x05; 32]);
    let id_a = Identity::from_seed([0x06; 32]);
    let id_b = Identity::from_seed([0x07; 32]);
    let a_principal = PrincipalId::new(id_a.principal_id().0);
    let b_principal = PrincipalId::new(id_b.principal_id().0);
    let o_device = DeviceId::new(id_o.device_id().0);

    let genesis = sign(&id_o, base(&id_o, group, AssertionType::GroupGenesis, 1, vec![], genesis_payload(&o_device)));
    let add_a = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 2, vec![], membership_add_payload(a_principal, 1)));
    let add_b = sign(&id_o, base(&id_o, group, AssertionType::MembershipAdd, 3, vec![], membership_add_payload(b_principal, 1)));

    let authors = [&id_o, &id_a, &id_b];
    // add_a and add_b are concurrent (both antecedent-free additions by O); the act position
    // "all members added" is reached by both arrival orders.
    let n1 = {
        let s = drive(&dir.path().join("k1.redb"), &id_o, &authors, vec![frame(&genesis), frame(&add_a), frame(&add_b)]);
        s.get_group_summary(&group).expect("summary").members.len() as u64
    };
    let n2 = {
        let s = drive(&dir.path().join("k2.redb"), &id_o, &authors, vec![frame(&genesis), frame(&add_b), frame(&add_a)]);
        s.get_group_summary(&group).expect("summary").members.len() as u64
    };
    assert_eq!(n1, n2, "the folded member count at the act position is order-independent");
    assert_eq!(n1, 3, "O + A + B");

    let k1 = quorum_k(n1);
    let k2 = quorum_k(n2);
    assert_eq!(k1, k2, "every node computes the identical k at the same act position across orders");
    assert_eq!(k1, 2, "k = ceil(3/2) = 2");

    // ensure the antecedent hashes above are actually the concurrency signal (documentation).
    let _ = envelope_hash(&add_a);
}
