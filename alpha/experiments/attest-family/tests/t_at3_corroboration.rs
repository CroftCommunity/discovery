//! Part 3 — EXP-AT3: scoped corroboration, no scalar, viewer-relative. T-AT3.*.
//!
//! The only query in the model: (viewer, subject, scope) → the corroboration
//! STRUCTURE — standing vouches/reviews with exact scope match whose attester
//! is resolvable to this viewer, with grades and lineage pointers. Never an
//! aggregate. Clients do the weighting.

mod common;

use attest_family::fixtures::*;
use attest_family::query::FreshnessDial;
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

/// Two edges onto P2 (from P1a and P3), a contractor vouch from each, one
/// babysitter vouch, and P1a's policy restricting resolvability to P2 only.
struct Fix {
    w: World,
    envs: Vec<Envelope>,
    vouch_p1a: ObjectId,
    vouch_p3: ObjectId,
    vouch_babysitter: ObjectId,
}

fn fixture(with_policy: bool) -> Fix {
    let w = World::new();
    let core_a = edge_core(w.p1a.id, w.p2.id, [0x31; 16], vec![]);
    let ha1 = w.p1a.emit(vec![], edge_half(core_a.clone(), "friend"));
    let ha2 = w.p2.emit(vec![], edge_half(core_a.clone(), "friend"));
    let core_b = edge_core(w.p3.id, w.p2.id, [0x32; 16], vec![]);
    let hb1 = w.p3.emit(vec![], edge_half(core_b.clone(), "cousin"));
    let hb2 = w.p2.emit(vec![], edge_half(core_b.clone(), "cousin"));

    let v1 = w.p1a.emit(
        vec![ha1.object_id(), ha2.object_id()],
        vouch(w.p2.id, "would hire as contractor", "rebuilt my porch", core_a.core_hash(), d(2026, 7, 1), None),
    );
    let v2 = w.p3.emit(
        vec![hb1.object_id(), hb2.object_id()],
        vouch(w.p2.id, "would hire as contractor", "fixed my roof", core_b.core_hash(), d(2026, 7, 2), None),
    );
    let v3 = w.p3.emit(
        vec![hb1.object_id(), hb2.object_id()],
        vouch(w.p2.id, "would trust as babysitter", "watched my kids", core_b.core_hash(), d(2026, 7, 3), None),
    );

    let mut envs = vec![ha1, ha2, hb1, hb2, v1.clone(), v2.clone(), v3.clone()];
    if with_policy {
        // P1a is resolvable ONLY to P2.
        envs.push(w.p1a.emit(vec![], policy(w.p1a.id, PolicyRule::AllowOnly(vec![w.p2.id]), None)));
    }
    Fix {
        w,
        envs,
        vouch_p1a: v1.object_id(),
        vouch_p3: v2.object_id(),
        vouch_babysitter: v3.object_id(),
    }
}

// ---------------------------------------------------------------------------
// T-AT3.1 — structure, not scalar; canonical-hash order only
// ---------------------------------------------------------------------------

#[test]
fn returns_structure_not_scalar() {
    let f = fixture(false);
    let state = log_from(&f.envs).fold();
    let resp = state.corroboration(
        &f.w.p2.id,
        &SubjectRef::Persona(f.w.p2.id),
        &Scope::new("would hire as contractor"),
        &dial(),
        as_of(),
    );
    assert_eq!(resp.entries.len(), 2);

    // No aggregate numeric field: every numeric leaf in the serialization is a
    // date-claim component (ties to T-AT0.2's crate-wide scan).
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&resp.to_ipld(), "", &mut numerics);
    for (path, val) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path);
        assert!(
            matches!(leaf_key, "d" | "m" | "y"),
            "aggregate/numeric leaf in corroboration response: {path} = {val}"
        );
    }

    // No computed ordering: entries are in canonical-hash order (object id
    // bytes ascending), which is unrelated to content, grade, or date.
    let mut ids: Vec<[u8; 32]> = resp.entries.iter().map(|e| e.attestation.0).collect();
    let sorted = {
        let mut s = ids.clone();
        s.sort();
        s
    };
    assert_eq!(ids, sorted, "entries must be in canonical-hash order");
    ids.sort();

    // Grades and lineage pointers are present per entry (the structure IS the
    // response).
    for e in &resp.entries {
        assert!(!e.lineage.is_empty(), "every entry carries lineage pointers");
    }
}

// ---------------------------------------------------------------------------
// T-AT3.2 — exact scope filter; adjacent scopes never bleed
// ---------------------------------------------------------------------------

#[test]
fn scope_filter_exact() {
    let f = fixture(false);
    let state = log_from(&f.envs).fold();

    let contractor = state.corroboration(
        &f.w.p2.id,
        &SubjectRef::Persona(f.w.p2.id),
        &Scope::new("would hire as contractor"),
        &dial(),
        as_of(),
    );
    let got: Vec<ObjectId> = contractor.entries.iter().map(|e| e.attestation).collect();
    assert!(got.contains(&f.vouch_p1a) && got.contains(&f.vouch_p3));
    assert!(
        !got.contains(&f.vouch_babysitter),
        "adjacent scope must not bleed into contractor"
    );

    let babysitter = state.corroboration(
        &f.w.p2.id,
        &SubjectRef::Persona(f.w.p2.id),
        &Scope::new("would trust as babysitter"),
        &dial(),
        as_of(),
    );
    let got_b: Vec<ObjectId> = babysitter.entries.iter().map(|e| e.attestation).collect();
    assert_eq!(got_b, vec![f.vouch_babysitter]);

    // A prefix is not a match — scope equality is exact.
    let prefix = state.corroboration(
        &f.w.p2.id,
        &SubjectRef::Persona(f.w.p2.id),
        &Scope::new("would hire"),
        &dial(),
        as_of(),
    );
    assert!(prefix.entries.is_empty());
}

// ---------------------------------------------------------------------------
// T-AT3.3 — resolvability filters traversal: absent, not redacted-but-counted
// ---------------------------------------------------------------------------

#[test]
fn resolvability_filters_traversal() {
    let f = fixture(true); // P1a resolvable only to P2
    let state = log_from(&f.envs).fold();

    // Viewer P3: P1a is not resolvable → P1a's vouch is ABSENT entirely.
    let resp = state.corroboration(
        &f.w.p3.id,
        &SubjectRef::Persona(f.w.p2.id),
        &Scope::new("would hire as contractor"),
        &dial(),
        as_of(),
    );
    let got: Vec<ObjectId> = resp.entries.iter().map(|e| e.attestation).collect();
    assert_eq!(got, vec![f.vouch_p3], "only the resolvable attester's vouch traverses");

    // Absent means absent: the serialized response carries no trace of the
    // filtered attester or her attestation — and no count that would reveal a
    // redaction happened.
    let bytes = resp.to_canonical_bytes();
    assert!(!contains_subslice(&bytes, &f.w.p1a.id.0), "no attester id leak");
    assert!(!contains_subslice(&bytes, &f.vouch_p1a.0), "no attestation id leak");
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&resp.to_ipld(), "", &mut numerics);
    for (path, _) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path);
        assert!(matches!(leaf_key, "d" | "m" | "y"), "no counting field may exist: {path}");
    }
}

// ---------------------------------------------------------------------------
// T-AT3.4 — viewer relativity: different policies, provably different views
// ---------------------------------------------------------------------------

#[test]
fn viewer_relativity() {
    let f = fixture(true);
    let state = log_from(&f.envs).fold();
    let scope = Scope::new("would hire as contractor");

    let view_p2 = state.corroboration(
        &f.w.p2.id,
        &SubjectRef::Persona(f.w.p2.id),
        &scope,
        &dial(),
        as_of(),
    );
    let view_p3 = state.corroboration(
        &f.w.p3.id,
        &SubjectRef::Persona(f.w.p2.id),
        &scope,
        &dial(),
        as_of(),
    );

    // Same (subject, scope), two viewers, provably different structures.
    assert_ne!(view_p2.to_canonical_bytes(), view_p3.to_canonical_bytes());
    assert_eq!(view_p2.entries.len(), 2, "P2 sees both attesters (P1a allows P2)");
    assert_eq!(view_p3.entries.len(), 1, "P3 sees only P3's own vouch");

    // Each internally consistent: every returned attester is resolvable to
    // THAT viewer.
    for e in &view_p2.entries {
        assert!(state.resolvable(&f.w.p2.id, &e.attester));
    }
    for e in &view_p3.entries {
        assert!(state.resolvable(&f.w.p3.id, &e.attester));
    }
}

// ---------------------------------------------------------------------------
// T-AT3.5 — mutual count without identity (fuzzed for leakage)
// ---------------------------------------------------------------------------

#[test]
fn mutual_count_without_identity() {
    // Property-style over seeded variations (the convergence_property.rs
    // pattern): N common counterparts, varying N and key material per case.
    for case in 0u8..24 {
        let n = (case % 5) as usize + 1;
        let w = World::new();
        // Counterpart personas C_i: fresh deterministic keypairs per case.
        let counterparts: Vec<PersonaFixture> = (0..n)
            .map(|i| {
                let mut seed = [0x60u8; 32];
                seed[0] = 0x60 + case;
                seed[1] = i as u8;
                PersonaFixture::new("C", Holder("HX"), seed, false)
            })
            .collect();

        let mut envs: Vec<Envelope> = Vec::new();
        // Each C_i has an established edge to BOTH P1a and P2.
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
        // A decoy: an edge from P3 to P2 only (not mutual with P1a).
        let core_decoy = edge_core(w.p3.id, w.p2.id, [0xD0, case, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![]);
        envs.push(w.p3.emit(vec![], edge_half(core_decoy.clone(), "d")));
        envs.push(w.p2.emit(vec![], edge_half(core_decoy, "d")));

        seeded_shuffle(&mut envs, case as u64 + 1);
        let state = log_from(&envs).fold();

        let mc = state.mutual_connection_count(&w.p1a.id, &w.p2.id);
        assert_eq!(mc.count, n as u64, "case {case}: cardinality only, and exact");

        // Leakage fuzz: the serialized disclosure contains no identifier,
        // hash, or derivable value of the N counterparts beyond the count.
        let bytes = mc.to_canonical_bytes();
        for c in &counterparts {
            assert!(!contains_subslice(&bytes, &c.id.0), "case {case}: counterpart id leaked");
            assert!(
                !contains_subslice(&bytes, blake3::hash(&c.id.0).as_bytes()),
                "case {case}: derived hash of counterpart id leaked"
            );
        }
        for e in &envs {
            assert!(
                !contains_subslice(&bytes, &e.object_id().0),
                "case {case}: an edge object id leaked"
            );
        }
        // The whole disclosure is just the cardinality: one numeric leaf, no
        // byte leaves at all.
        let mut byte_leaves = Vec::new();
        ipld_byte_leaves(&mc.to_ipld(), &mut byte_leaves);
        assert!(byte_leaves.is_empty(), "case {case}: no byte content may exist");
    }
}
