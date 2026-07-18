//! RUN-ATTEST-02 EXP-PA2 — sibling unlinkability. T-PA2.*.
//!
//! Claim: credentials issued to sibling personas share no correlator in
//! public data. (T-PA2.4 — the residue OUTSIDE the model's control — is a
//! FINDINGS entry, pinned here by a doc test.)

mod common;

use std::collections::BTreeSet;

use attest_family::fixtures::*;
use attest_family::issuer::{mint, IssuerState, MintEntropy, MintOutput, MintRefusal};
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

fn byte_leaves_of(bytes: &[u8]) -> Vec<Vec<u8>> {
    let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).expect("public object decodes");
    let mut leaves = Vec::new();
    ipld_byte_leaves(&v, &mut leaves);
    leaves
}

// ---------------------------------------------------------------------------
// T-PA2.1 — sibling credentials share no correlator in public data
// ---------------------------------------------------------------------------

#[test]
fn sibling_credentials_unlinkable() {
    // Property over seeded variations (the T-AT3.5 pattern): H1's three
    // siblings and ten single-anchor strangers are minted the same predicate
    // set. The pairwise intersection of two SIBLINGS' public surfaces must be
    // no larger than what any two STRANGERS share — i.e. exactly the values
    // every same-(issuer, predicate) credential carries (the issuer's key),
    // and nothing else: no serial, no batch id, no shared nonce or salt, no
    // derivable value that partitions the siblings from the population.
    for case in 0u64..12 {
        let w = AnchorWorld::new();
        let mut state = IssuerState::new(u32::MAX);
        let kinds = [PredicateKind::VettedHolder, PredicateKind::PhoneVerified];

        let strangers: Vec<PersonaFixture> = (0..10)
            .map(|i| {
                PersonaFixture::new("S", Holder("HS"), derived_seed("t-pa2-1-s", case, i), false)
            })
            .collect();
        let mut everyone: Vec<(&PersonaFixture, Holder)> = vec![
            (&w.p1a, w.h1),
            (&w.p1b, w.h1),
            (&w.p1c, w.h1),
        ];
        everyone.extend(strangers.iter().map(|s| (s, Holder("HS"))));
        seeded_shuffle(&mut everyone, case + 3);

        let mut outputs: Vec<(PersonaId, MintOutput)> = Vec::new();
        for (k, (subject, holder)) in everyone.iter().enumerate() {
            let out = mint(
                &mut state,
                &w.coop,
                member_ref(holder),
                subject,
                &kinds,
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed("t-pa2-1-e", case, k as u64)),
            )
            .expect("fixture mint succeeds");
            outputs.push((subject.id, out));
        }
        state.close_epoch(&w.coop);

        // Each persona's public surface: published envelopes + its status
        // responses (the full byte set a third-party viewer can obtain from
        // the issuer's read side for that persona's credentials).
        let surface = |id: &PersonaId| -> BTreeSet<Vec<u8>> {
            let (_, out) = outputs.iter().find(|(pid, _)| pid == id).unwrap();
            let mut leaves: BTreeSet<Vec<u8>> = BTreeSet::new();
            for env in std::iter::once(&out.vetting).chain(out.credentials.iter()) {
                leaves.extend(byte_leaves_of(&env.canonical_bytes_with_sig()));
                let resp = state.status_check(&w.coop, env.object_id().0);
                leaves.extend(byte_leaves_of(&resp.to_canonical_bytes()));
            }
            leaves.into_iter().filter(|l| l.len() >= 16).collect()
        };

        // The population floor: what EVERY same-(issuer, predicates) holder
        // shares. Computed over all thirteen surfaces.
        let all: Vec<BTreeSet<Vec<u8>>> =
            outputs.iter().map(|(id, _)| surface(id)).collect();
        let mut floor = all[0].clone();
        for s in &all[1..] {
            floor = floor.intersection(s).cloned().collect();
        }
        // The floor is exactly the issuer's public key (the envelope author).
        assert_eq!(
            floor,
            BTreeSet::from([w.coop.id.0.to_vec()]),
            "case {case}: the only population-wide shared value is the issuer key"
        );

        // Sibling pairwise intersections must not exceed the floor.
        let sib_ids = [w.p1a.id, w.p1b.id, w.p1c.id];
        for a in 0..3 {
            for b in (a + 1)..3 {
                let sa = surface(&sib_ids[a]);
                let sb = surface(&sib_ids[b]);
                let shared: BTreeSet<&Vec<u8>> = sa.intersection(&sb).collect();
                for leaf in shared {
                    assert!(
                        floor.contains(leaf),
                        "case {case}: siblings {a},{b} share a value beyond the population floor: {leaf:02x?}"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// T-PA2.2 — independent derivation is enforced
// ---------------------------------------------------------------------------

// The compile-boundary half is the compile_fail doc-test on
// `issuer::MintEntropy` (entropy is consumed by value; a second mint cannot
// reuse it). This test is the runtime half: RE-DERIVING from the same seed is
// rejected deterministically at the salt boundary, with state unchanged.
#[test]
fn independent_derivation_enforced() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let kinds = [PredicateKind::VettedHolder];
    let seed = derived_seed("t-pa2-2", 1, 1);

    let first = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &kinds,
        d(2026, 7, 17),
        MintEntropy::from_seed(seed),
    );
    assert!(first.is_ok(), "fresh entropy mints");

    // Same derivation state, different subject: refused whole, deterministic.
    let reused = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1b,
        &kinds,
        d(2026, 7, 17),
        MintEntropy::from_seed(seed),
    );
    assert_eq!(
        reused.expect_err("shared derivation state must be refused"),
        MintRefusal::SaltReused
    );

    // The refusal left no partial state: the epoch closes with exactly the
    // first mint's commitment, and a fresh-seed mint still succeeds.
    let ok2 = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1b,
        &kinds,
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-pa2-2", 2, 2)),
    );
    assert!(ok2.is_ok(), "fresh entropy after a refusal mints");
    let record = state.close_epoch(&w.coop);
    assert_eq!(record.declared_total, 2, "the refused mint left nothing behind");
    assert_eq!(record.commitments.len(), 2);
}

// ---------------------------------------------------------------------------
// T-PA2.3 — the status check leaks nothing across personas
// ---------------------------------------------------------------------------

#[test]
fn status_check_no_cross_leak() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let kinds = [PredicateKind::VettedHolder, PredicateKind::PhoneVerified];
    let mut outs: Vec<(PersonaId, MintOutput)> = Vec::new();
    for (k, (subject, holder)) in [
        (&w.p1a, &w.h1),
        (&w.p1b, &w.h1),
        (&w.p1c, &w.h1),
        (&w.p2a, &w.h2),
        (&w.p3, &w.h3),
    ]
    .iter()
    .enumerate()
    {
        let out = mint(
            &mut state,
            &w.coop,
            member_ref(holder),
            subject,
            &kinds,
            d(2026, 7, 17),
            MintEntropy::from_seed(derived_seed("t-pa2-3", 0, k as u64)),
        )
        .expect("fixture mint succeeds");
        outs.push((subject.id, out));
    }
    state.close_epoch(&w.coop);

    let queried = outs[0].1.credentials[0].object_id();
    let resp = state.status_check(&w.coop, queried.0);
    let bytes = resp.to_canonical_bytes();

    // The response's field set is EXACT: queried hash echo, standing string,
    // issuer signature — nothing else exists to leak through.
    let v: Ipld = serde_ipld_dagcbor::from_slice(&bytes).unwrap();
    let Ipld::Map(m) = &v else { panic!("status response must be a map") };
    let keys: Vec<&str> = m.keys().map(|k| k.as_str()).collect();
    assert_eq!(keys, vec!["g", "h", "s"], "response fields are exactly {{hash, standing, sig}}");

    // No numeric leaf at all — no counts, no positions, no timestamps.
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&v, "", &mut numerics);
    assert!(numerics.is_empty(), "a status response carries no numbers: {numerics:?}");

    // Nothing about ANY other object or persona: not the siblings'
    // credentials, not the same persona's other credential, not any persona
    // id, not any commitment.
    for (pid, out) in &outs {
        assert!(!contains_subslice(&bytes, &pid.0), "persona id in status response");
        for env in std::iter::once(&out.vetting).chain(out.credentials.iter()) {
            if env.object_id() != queried {
                assert!(
                    !contains_subslice(&bytes, &env.object_id().0),
                    "another object's id in status response"
                );
            }
        }
    }
    for rec in [state.lineage_bytes()] {
        // The response and the lineage share no 32-byte value (the echoed
        // credential hash is salted before it ever enters the lineage).
        let resp_leaves: BTreeSet<Vec<u8>> =
            byte_leaves_of(&bytes).into_iter().filter(|l| l.len() == 32).collect();
        let lineage_leaves: BTreeSet<Vec<u8>> =
            byte_leaves_of(&rec).into_iter().filter(|l| l.len() == 32).collect();
        assert!(
            resp_leaves.is_disjoint(&lineage_leaves),
            "a status response must not locate anything in the public lineage"
        );
    }
}

// ---------------------------------------------------------------------------
// T-PA2.4 — residual correlators are FINDINGS, not code (doc pin)
// ---------------------------------------------------------------------------

#[test]
fn residual_correlators_documented() {
    let findings = crate_source("FINDINGS.md");
    for needle in [
        "shared counterpart",
        "stylometric",
        "network-layer metadata",
        "client hygiene",
        "transport",
        "epoch membership",
        "Modeled by design",
        "presentation-unlinkability",
        "same credential shown twice is trivially linkable",
    ] {
        assert!(
            findings.contains(needle),
            "FINDINGS.md must record the RUN-ATTEST-02 residue `{needle}` \
             (out-of-protocol correlators + the deferred BBS distinction)"
        );
    }
}
