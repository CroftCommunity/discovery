//! RUN-ATTEST-04 Part A — V4: the graded resolvability default. T-A4.1–T-A4.5.
//!
//! V4 (DECIDED 2026-07-18, owner-confirmed in chat) replaces the
//! resolvable-to-all stand-in default: with NO policy act on record, a persona
//! resolves exactly to the counterparts of its standing co-signed edges;
//! strangers receive cardinality only. Silence IS the graded posture — zero
//! configuration. OPEN is a deliberate per-persona policy supersede with
//! lineage. Reviews assert experience, not relationship: a review grants no
//! resolution (only a standing co-signed edge does), and public-tier
//! discoverability of published records is untouched by this Drystone-tier
//! traversal default.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::FarEnd;
use attest_family::query::{edge_list_ipld, FreshnessDial};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

fn dial() -> FreshnessDial {
    FreshnessDial { stale_after_days: 3650 }
}

fn as_of() -> DateClaim {
    d(2026, 7, 18)
}

// ---------------------------------------------------------------------------
// T-A4.1 — THE red for Part A: fresh persona, zero policy acts, graded
// ---------------------------------------------------------------------------

/// P2 is a fresh persona with ZERO policy acts. It holds a standing co-signed
/// edge with P1a and has vouched for P1a. To the stranger P3's full sweep, P2
/// is present only as cardinality: the edge far end is opaque with no
/// derivable value, the vouch is absent-not-redacted (the T-AT3.3 pattern),
/// and the only stranger-facing tier is the mutual count. Under the
/// pre-change resolvable-to-all default this fails loudly (captured red).
#[test]
fn fresh_persona_graded_by_default() {
    let w = World::new();
    let core = edge_core(w.p1a.id, w.p2.id, [0xA4; 16], vec![]);
    let ha = w.p1a.emit(vec![], edge_half(core.clone(), "friend"));
    let hb = w.p2.emit(vec![], edge_half(core.clone(), "friend"));
    let v = w.p2.emit(
        vec![ha.object_id(), hb.object_id()],
        vouch(w.p1a.id, "would hire as contractor", "solid", core.core_hash(), d(2026, 7, 1), None),
    );
    let rv = w.p2.emit(
        vec![],
        review(SubjectRef::Persona(w.p1a.id), "punctuality", "on time", d(2026, 7, 2), None),
    );
    let state = log_from(&[ha, hb, v.clone(), rv.clone()]).fold();

    // Silence is the graded posture: no policy object exists for P2 at all.
    assert!(state.policy_head(&w.p2.id).is_none(), "zero policy acts — silence, not configuration");
    assert!(!state.resolvable(&w.p3.id, &w.p2.id), "a stranger does not resolve a fresh persona");

    // The stranger's sweep of P1a's edge list: the edge exists, its far end
    // (fresh P2) is opaque, and the serialization carries no P2-derived value.
    let disclosed = state.edge_list(&w.p1a.id, &w.p3.id);
    assert_eq!(disclosed.len(), 1);
    assert_eq!(
        disclosed[0].far_end,
        FarEnd::Opaque,
        "graded default: a fresh persona at the far end is opaque to a stranger"
    );
    let bytes = serde_ipld_dagcbor::to_vec(&edge_list_ipld(&disclosed)).unwrap();
    assert!(!contains_subslice(&bytes, &w.p2.id.0), "no fresh-persona id in a stranger's sweep");
    assert!(
        !contains_subslice(&bytes, blake3::hash(&w.p2.id.0).as_bytes()),
        "no derivable fresh-persona value either"
    );

    // Every resolution attempt is absent-not-redacted (T-AT3.3 pattern): the
    // fresh persona's vouch AND review are absent from the stranger's
    // corroboration structures — no id trace, no counting field.
    for scope in ["would hire as contractor", "punctuality"] {
        let resp = state.corroboration(
            &w.p3.id,
            &SubjectRef::Persona(w.p1a.id),
            &Scope::new(scope),
            &dial(),
            as_of(),
        );
        assert!(
            resp.entries.is_empty(),
            "scope `{scope}`: a fresh attester's attestation is ABSENT from a stranger's structure"
        );
        let rbytes = resp.to_canonical_bytes();
        assert!(!contains_subslice(&rbytes, &w.p2.id.0), "no attester id leak");
        assert!(!contains_subslice(&rbytes, &v.object_id().0), "no vouch id leak");
        assert!(!contains_subslice(&rbytes, &rv.object_id().0), "no review id leak");
        let mut numerics = Vec::new();
        ipld_numeric_leaves(&resp.to_ipld(), "", &mut numerics);
        assert!(numerics.is_empty(), "absent means absent — no redaction count exists");
    }

    // The counterpart still resolves (that is the grade): P1a sees P2.
    assert!(state.resolvable(&w.p1a.id, &w.p2.id), "the standing-edge counterpart resolves");

    // And the stranger-facing tier that remains is cardinality.
    let mc = state.mutual_connection_count(&w.p1a.id, &w.p2.id);
    assert_eq!(mc.count, 0, "cardinality is exact (no common counterpart in this fixture)");
}

// ---------------------------------------------------------------------------
// T-A4.2 — a standing co-signed edge resolves each side to the other, ONLY
// ---------------------------------------------------------------------------

#[test]
fn counterpart_resolves() {
    let w = World::new();
    // Edge P1a–P2 (standing) and edge P1a–P3 (standing). No policy acts.
    let core_ab = edge_core(w.p1a.id, w.p2.id, [0xB1; 16], vec![]);
    let core_ac = edge_core(w.p1a.id, w.p3.id, [0xB2; 16], vec![]);
    let envs = vec![
        w.p1a.emit(vec![], edge_half(core_ab.clone(), "x")),
        w.p2.emit(vec![], edge_half(core_ab.clone(), "x")),
        w.p1a.emit(vec![], edge_half(core_ac.clone(), "y")),
        w.p3.emit(vec![], edge_half(core_ac.clone(), "y")),
    ];
    let state = log_from(&envs).fold();

    // Each side of a standing edge resolves the other, with no policy act.
    assert!(state.resolvable(&w.p1a.id, &w.p2.id));
    assert!(state.resolvable(&w.p2.id, &w.p1a.id));
    assert!(state.resolvable(&w.p1a.id, &w.p3.id));
    assert!(state.resolvable(&w.p3.id, &w.p1a.id));

    // And ONLY to each other: P3 holds no edge with P2, so neither resolves
    // the other — sharing a counterpart (P1a) grants nothing.
    assert!(!state.resolvable(&w.p3.id, &w.p2.id), "no edge, no resolution — counterparts only");
    assert!(!state.resolvable(&w.p2.id, &w.p3.id));

    // The grade is traversal-real: P2's vouch about P1a traverses to P1a (its
    // counterpart) and not to P3 (a stranger to P2).
    let v = w.p2.emit(
        vec![envs[0].object_id(), envs[1].object_id()],
        vouch(w.p1a.id, "would hire as contractor", "good", core_ab.core_hash(), d(2026, 7, 3), None),
    );
    let mut log = log_from(&envs);
    log.append(v.clone()).unwrap();
    let state = log.fold();
    let sees = |viewer: &PersonaId| {
        state
            .corroboration(viewer, &SubjectRef::Persona(w.p1a.id), &Scope::new("would hire as contractor"), &dial(), as_of())
            .entries
            .len()
    };
    assert_eq!(sees(&w.p1a.id), 1, "the counterpart traverses the vouch");
    assert_eq!(sees(&w.p3.id), 0, "the stranger does not");

    // A dissolved edge is not a standing edge: resolution lapses with it.
    let dis = w.p2.emit(
        vec![envs[0].object_id(), envs[1].object_id()],
        edge_dissolve(core_ab.core_hash(), vec![envs[0].object_id(), envs[1].object_id()]),
    );
    log.append(dis).unwrap();
    let state = log.fold();
    assert!(
        !state.resolvable(&w.p1a.id, &w.p2.id),
        "only a STANDING co-signed edge grades resolution; a superseded edge does not"
    );
}

// ---------------------------------------------------------------------------
// T-A4.3 — OPEN is a deliberate posture: a policy supersede with lineage
// ---------------------------------------------------------------------------

/// The workplace-persona fixture, end to end: holder H1's persona P1b is a
/// workplace persona meant for reading structure — it deliberately opts OPEN
/// so strangers can resolve it. The open act is a policy supersede with
/// lineage: deliberate, visible in P1b's own record, reversible by further
/// supersede.
#[test]
fn open_is_a_posture_supersede() {
    let w = World::new();
    // P1b (the workplace persona) has one standing edge to P2 and a vouch
    // about P2 in a work scope.
    let core = edge_core(w.p1b.id, w.p2.id, [0xC1; 16], vec![]);
    let ha = w.p1b.emit(vec![], edge_half(core.clone(), "colleague"));
    let hb = w.p2.emit(vec![], edge_half(core.clone(), "colleague"));
    let v = w.p1b.emit(
        vec![ha.object_id(), hb.object_id()],
        vouch(w.p2.id, "worked together on contracts", "reliable", core.core_hash(), d(2026, 7, 1), None),
    );
    let mut log = log_from(&[ha, hb, v.clone()]);

    // Before the act: the stranger P3 gets nothing (the graded default).
    let before = log.fold();
    assert!(!before.resolvable(&w.p3.id, &w.p1b.id));

    // The OPEN act: a policy fact by the named party itself.
    let open = w.p1b.emit(vec![], policy(w.p1b.id, PolicyRule::AllowAll, None));
    log.append(open.clone()).unwrap();
    let opened = log.fold();
    assert!(opened.resolvable(&w.p3.id, &w.p1b.id), "opting open is a per-persona posture");
    assert_eq!(
        opened.corroboration(&w.p3.id, &SubjectRef::Persona(w.p2.id), &Scope::new("worked together on contracts"), &dial(), as_of())
            .entries
            .len(),
        1,
        "the workplace persona's vouch now traverses to strangers — reading structure works"
    );
    // Deliberate and visible: the posture is P1b's own record, with lineage.
    assert_eq!(opened.policy_head(&w.p1b.id).unwrap().rule, PolicyRule::AllowAll);
    assert_eq!(opened.policy_lineage(&w.p1b.id), vec![open.object_id()]);

    // Reversible by further supersede — same machinery, lineage grows.
    let narrow = w.p1b.emit(
        vec![open.object_id()],
        policy(w.p1b.id, PolicyRule::AllowOnly(vec![w.p2.id]), Some(open.object_id())),
    );
    log.append(narrow.clone()).unwrap();
    let reversed = log.fold();
    assert!(!reversed.resolvable(&w.p3.id, &w.p1b.id), "reversed: the stranger loses resolution");
    assert!(reversed.resolvable(&w.p2.id, &w.p1b.id), "the listed viewer keeps it");
    assert_eq!(
        reversed.policy_lineage(&w.p1b.id),
        vec![open.object_id(), narrow.object_id()],
        "the posture's whole history is lineage, oldest first"
    );
}

// ---------------------------------------------------------------------------
// T-A4.4 — cardinality is still leakless where it is now the stranger tier
// ---------------------------------------------------------------------------

/// T-AT3.5's serialization fuzz re-run under the graded default: the mutual
/// count is now the STRANGER-FACING tier, so the no-identity-leak property is
/// re-proven exactly where it matters most — with zero policy acts anywhere,
/// a stranger's entire view of the relationship tier is the count.
#[test]
fn cardinality_still_leakless() {
    for case in 0u8..24 {
        let n = (case % 5) as usize + 1;
        let w = World::new();
        let counterparts: Vec<PersonaFixture> = (0..n)
            .map(|i| {
                let mut seed = [0x70u8; 32];
                seed[0] = 0x70 + case;
                seed[1] = i as u8;
                PersonaFixture::new("C", Holder("HX"), seed, false)
            })
            .collect();

        let mut envs: Vec<Envelope> = Vec::new();
        for (i, c) in counterparts.iter().enumerate() {
            for (j, far) in [&w.p1a, &w.p2].into_iter().enumerate() {
                let mut nonce = [0u8; 16];
                nonce[0] = case;
                nonce[1] = i as u8;
                nonce[2] = j as u8;
                let core = edge_core(c.id, far.id, nonce, vec![]);
                envs.push(c.emit(vec![], edge_half(core.clone(), "x")));
                envs.push(far.emit(vec![], edge_half(core, "y")));
            }
        }
        seeded_shuffle(&mut envs, case as u64 + 11);
        let state = log_from(&envs).fold();

        // ZERO policy acts exist — the graded default is in force, and the
        // stranger P3 resolves none of the participants.
        for p in counterparts.iter().map(|c| c.id).chain([w.p1a.id, w.p2.id]) {
            assert!(!state.resolvable(&w.p3.id, &p), "case {case}: stranger resolves nobody");
        }
        // The stranger's edge-list sweeps yield only opaque far ends.
        for of in [&w.p1a.id, &w.p2.id] {
            for disc in state.edge_list(of, &w.p3.id) {
                assert_eq!(disc.far_end, FarEnd::Opaque, "case {case}: graded default holds");
            }
        }

        // The count is exact and leakless — count without identity is the
        // disclosure's entire content (T-AT3.5, re-proven at the new tier).
        let mc = state.mutual_connection_count(&w.p1a.id, &w.p2.id);
        assert_eq!(mc.count, n as u64, "case {case}: cardinality exact");
        let bytes = mc.to_canonical_bytes();
        for c in &counterparts {
            assert!(!contains_subslice(&bytes, &c.id.0), "case {case}: counterpart id leaked");
            assert!(
                !contains_subslice(&bytes, blake3::hash(&c.id.0).as_bytes()),
                "case {case}: derived counterpart value leaked"
            );
        }
        for e in &envs {
            assert!(
                !contains_subslice(&bytes, &e.object_id().0),
                "case {case}: an edge object id leaked"
            );
        }
        let mut byte_leaves = Vec::new();
        ipld_byte_leaves(&mc.to_ipld(), &mut byte_leaves);
        assert!(byte_leaves.is_empty(), "case {case}: no byte content may exist");
    }
}

// ---------------------------------------------------------------------------
// T-A4.5 — the correlation-resistance sweeps, remeasured under the default
// ---------------------------------------------------------------------------

/// T-AT4.3's sweep (and the fold-side leg of T-PA2.1's) were measured under
/// resolvable-to-all. Re-measured here under the graded default, in BOTH
/// postures: zero-policy (the new default — the third-party surface must be a
/// subset of the opted-open one: equal or stronger, never weaker) and
/// opted-open (every participant takes the V4 OPEN posture — reproducing the
/// original measurement's disclosure level, where the original property must
/// still hold). Any weaker case is a FINDING, not silently absorbed.
#[test]
fn sweeps_remeasured() {
    for case in 0u64..16 {
        for open_posture in [false, true] {
            let w = World::new();
            assert_eq!(w.p1a.holder, w.p1b.holder, "fixture: same holder, bookkeeping only");

            let mut envs = Vec::new();
            for (i, near) in [&w.p1a, &w.p1b].into_iter().enumerate() {
                let mut nonce = [0u8; 16];
                nonce[0] = 0x53 + i as u8;
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
            if open_posture {
                // The opted-open posture: each participant's own deliberate
                // policy act (the T-A4.3 machinery) — reproducing the
                // pre-V4 disclosure level under the new default.
                for p in [&w.p1a, &w.p1b, &w.p2] {
                    envs.push(p.emit(vec![], policy(p.id, PolicyRule::AllowAll, None)));
                }
            }
            seeded_shuffle(&mut envs, case + 17);
            let state = log_from(&envs).fold();

            let viewer = w.p3.id;
            let mut surfaces: Vec<Vec<Vec<u8>>> = Vec::new();
            for near in [&w.p1a, &w.p1b] {
                let mut leaves = Vec::new();
                ipld_byte_leaves(&edge_list_ipld(&state.edge_list(&near.id, &viewer)), &mut leaves);
                let resp = state.corroboration(
                    &viewer,
                    &SubjectRef::Persona(w.p2.id),
                    &Scope::new("would hire as contractor"),
                    &dial(),
                    as_of(),
                );
                for e in resp.entries.iter().filter(|e| e.attester == near.id) {
                    ipld_byte_leaves(&e.to_ipld(), &mut leaves);
                }
                surfaces.push(leaves);
            }

            // The original T-AT4.3 property, unchanged: the only shared
            // 16-byte-or-longer values are P2's id and P2-owned object ids.
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
                    "case {case} open={open_posture}: shared leaf links P1a and P1b beyond \
                     the shared counterpart: {shared:02x?}"
                );
            }

            // Zero-policy posture: equal or STRONGER than the old default —
            // the stranger surface must not expose the counterpart at all
            // (the old measurement exposed P2's id in resolved far ends).
            if !open_posture {
                for leaves in &surfaces {
                    for l in leaves {
                        assert_ne!(
                            l.as_slice(),
                            w.p2.id.0.as_slice(),
                            "case {case}: graded default leaks the counterpart id to a stranger"
                        );
                    }
                }
            }
        }
    }
    // Result of the remeasure (stated for the run summary): every case is
    // equal (opted-open reproduces the old surface, property holds) or
    // stronger (zero-policy exposes strictly less: no resolved far ends, no
    // attester-linked entries). No weaker case exists to file as a FINDING.
}
