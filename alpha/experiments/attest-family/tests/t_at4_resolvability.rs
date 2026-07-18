//! Part 4 — EXP-AT4: resolvability governed by the named party + correlation
//! resistance. T-AT4.*.
//!
//! Resolvability of a persona named at the far end of an edge is governed by
//! THAT party's policy, never by the edge holder's disclosure choices.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{EdgeStatus, FarEnd};
use attest_family::query::edge_list_ipld;
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

// ---------------------------------------------------------------------------
// T-AT4.1 — the edge holder cannot grant far-end resolvability
// ---------------------------------------------------------------------------

#[test]
fn edge_holder_cannot_grant_far_end() {
    let w = World::new();
    // Edge P1a–P2; P2's policy excludes everyone but P1a.
    let core = edge_core(w.p1a.id, w.p2.id, [0x41; 16], vec![]);
    let ha = w.p1a.emit(vec![], edge_half(core.clone(), "friend"));
    let hb = w.p2.emit(vec![], edge_half(core.clone(), "friend"));
    let pol = w.p2.emit(vec![], policy(w.p2.id, PolicyRule::AllowOnly(vec![w.p1a.id]), None));
    let state = log_from(&[ha, hb, pol]).fold();

    // P1a "discloses their edge list to viewer P3" — the disclosure API takes
    // ONLY (of, viewer): there is deliberately no parameter through which the
    // holder could grant more than the far party's own policy allows.
    let disclosed = state.edge_list(&w.p1a.id, &w.p3.id);
    assert_eq!(disclosed.len(), 1, "the edge itself is visible as existing");
    assert_eq!(disclosed[0].status, EdgeStatus::Established);
    assert_eq!(
        disclosed[0].far_end,
        FarEnd::Opaque,
        "the far end stays opaque: P2's policy excludes P3, and P1a cannot override it"
    );

    // The serialized disclosure carries no P2-derived value at all.
    let bytes = serde_ipld_dagcbor::to_vec(&edge_list_ipld(&disclosed)).unwrap();
    assert!(!contains_subslice(&bytes, &w.p2.id.0), "no far-end id under an excluding policy");
    assert!(
        !contains_subslice(&bytes, blake3::hash(&w.p2.id.0).as_bytes()),
        "no derivable far-end value either"
    );

    // To P1a (allowed by P2's policy), the same list resolves the far end.
    let to_self = state.edge_list(&w.p1a.id, &w.p1a.id);
    assert_eq!(to_self[0].far_end, FarEnd::Resolved(w.p2.id));
}

// ---------------------------------------------------------------------------
// T-AT4.2 — a policy change is a superseding fact with lineage
// ---------------------------------------------------------------------------

#[test]
fn policy_change_is_supersede() {
    let w = World::new();
    let p1 = w.p2.emit(vec![], policy(w.p2.id, PolicyRule::AllowAll, None));
    let mut log = log_from(std::slice::from_ref(&p1));
    let p1_bytes = log.object_bytes(&p1.object_id()).unwrap().to_vec();

    let head0 = log.fold();
    assert_eq!(head0.policy_head(&w.p2.id).unwrap().rule, PolicyRule::AllowAll);

    // Change = supersede, citing the prior policy.
    let p2 = w.p2.emit(
        vec![p1.object_id()],
        policy(w.p2.id, PolicyRule::AllowOnly(vec![w.p1a.id]), Some(p1.object_id())),
    );
    log.append(p2.clone()).unwrap();

    let state = log.fold();
    assert_eq!(
        state.policy_head(&w.p2.id).unwrap().rule,
        PolicyRule::AllowOnly(vec![w.p1a.id]),
        "the head policy is the superseding fact"
    );
    assert_eq!(
        state.policy_lineage(&w.p2.id),
        vec![p1.object_id(), p2.object_id()],
        "the change carries lineage, oldest first"
    );
    // Not a mutation: the prior policy's bytes are unchanged (T-AT0.3).
    assert_eq!(log.object_bytes(&p1.object_id()).unwrap(), &p1_bytes[..]);

    // A persona cannot set another persona's policy: an envelope whose author
    // differs from the named persona does not become that persona's policy.
    let rogue = w.p3.emit(vec![], policy(w.p2.id, PolicyRule::AllowOnly(vec![w.p3.id]), None));
    log.append(rogue).unwrap();
    let state2 = log.fold();
    assert_eq!(
        state2.policy_head(&w.p2.id).unwrap().rule,
        PolicyRule::AllowOnly(vec![w.p1a.id]),
        "only the named party's own facts govern their resolvability"
    );
}

// ---------------------------------------------------------------------------
// T-AT4.3 — persona correlation resistance (property over the public surface)
// ---------------------------------------------------------------------------

#[test]
fn persona_correlation_resistance() {
    // P1a and P1b (same holder H1 — the linkage exists ONLY in fixture
    // bookkeeping) each mint edges to P2. Property over seeded variations:
    // the entire public folded surface reachable by a third-party viewer
    // exposes no shared identifier, key material, or derivable value linking
    // P1a and P1b other than the shared counterpart P2 itself.
    for case in 0u64..16 {
        let w = World::new();
        assert_eq!(w.p1a.holder, w.p1b.holder, "fixture: same holder, bookkeeping only");

        let mut envs = Vec::new();
        for (i, near) in [&w.p1a, &w.p1b].into_iter().enumerate() {
            let mut nonce = [0u8; 16];
            nonce[0] = 0x43 + i as u8;
            nonce[1] = case as u8;
            let core = edge_core(near.id, w.p2.id, nonce, vec![]);
            let ha = near.emit(vec![], edge_half(core.clone(), "friend"));
            let hb = w.p2.emit(vec![], edge_half(core.clone(), "friend"));
            let v = near.emit(
                vec![ha.object_id(), hb.object_id()],
                vouch(
                    w.p2.id,
                    "would hire as contractor",
                    "good",
                    core.core_hash(),
                    d(2026, 7, 1 + case as u8 % 20),
                    None,
                ),
            );
            envs.extend([ha, hb, v]);
        }
        seeded_shuffle(&mut envs, case + 7);
        let state = log_from(&envs).fold();

        // The public surface a third-party viewer (P3) can reach, split into
        // the P1a-attributed part and the P1b-attributed part.
        let viewer = w.p3.id;
        let mut surfaces: Vec<Vec<Vec<u8>>> = Vec::new();
        for near in [&w.p1a, &w.p1b] {
            let mut leaves = Vec::new();
            ipld_byte_leaves(&edge_list_ipld(&state.edge_list(&near.id, &viewer)), &mut leaves);
            let resp = state.corroboration(
                &viewer,
                &SubjectRef::Persona(w.p2.id),
                &Scope::new("would hire as contractor"),
                &attest_family::query::FreshnessDial { stale_after_days: 3650 },
                d(2026, 7, 18),
            );
            // Only this near-side's entries attribute to this surface. (P2's
            // own edge list is deliberately NOT split per side: it reveals
            // that P2 has two counterparts — the shared-counterpart residue
            // this test documents rather than solves; see FINDINGS.md.)
            for e in resp.entries.iter().filter(|e| e.attester == near.id) {
                ipld_byte_leaves(&e.to_ipld(), &mut leaves);
            }
            surfaces.push(leaves);
        }

        // Intersect the two surfaces' 16-byte-or-longer leaves (identifiers,
        // hashes, keys — anything derivable). The ONLY permitted shared value
        // is P2's persona id (the shared counterpart), plus P2-authored
        // object ids that are shared precisely because P2 is shared.
        let p2_owned: std::collections::BTreeSet<Vec<u8>> = envs
            .iter()
            .filter(|e| e.author == w.p2.id)
            .map(|e| e.object_id().0.to_vec())
            .collect();
        let set_a: std::collections::BTreeSet<&Vec<u8>> =
            surfaces[0].iter().filter(|l| l.len() >= 16).collect();
        let set_b: std::collections::BTreeSet<&Vec<u8>> =
            surfaces[1].iter().filter(|l| l.len() >= 16).collect();
        for shared in set_a.intersection(&set_b) {
            let is_p2_id = shared.as_slice() == w.p2.id.0.as_slice();
            let is_p2_owned_object = p2_owned.contains(shared.as_slice());
            assert!(
                is_p2_id || is_p2_owned_object,
                "case {case}: shared leaf links P1a and P1b beyond the shared counterpart: {shared:02x?}"
            );
        }

        // No holder identifier exists anywhere on the surface (it is not even
        // representable in a payload).
        for leaves in &surfaces {
            for l in leaves {
                assert_ne!(l.as_slice(), b"H1", "case {case}: holder bookkeeping leaked");
            }
        }
    }
    // Residual behavioral/metadata correlation (shared counterpart, timing-
    // shaped graph structure) is EXPECTED and recorded, not solved — see
    // FINDINGS.md (T-AT4.3 residue).
}

// ---------------------------------------------------------------------------
// T-AT4.4 — the issuer linkage seam is documented (FINDINGS entry, Modeled)
// ---------------------------------------------------------------------------

#[test]
fn issuer_linkage_seam_documented() {
    let findings = crate_source("FINDINGS.md");
    assert!(
        findings.contains("issuer linkage seam"),
        "FINDINGS.md must state the co-op-as-issuer linkage seam"
    );
    for needle in [
        "per-persona optional issuance",
        "no-record covenant",
        "unlinkable presentations",
        "BBS",
        "Modeled",
    ] {
        assert!(
            findings.contains(needle),
            "FINDINGS.md issuer-seam entry must mention `{needle}` (v1 posture + deferred direction)"
        );
    }
}
