//! RUN-ATTEST-02 EXP-PA4 — anonymity-set measurement (instrumented, not
//! pass/fail — but the HARNESS is proven red-first on a hand-computed fixture
//! before it touches the generated populations, per §2).
//!
//! M-PA4.2 / M-PA4.3 recompute the measured tables and assert they equal the
//! values committed in `FINDINGS-ANONYMITY-SETS.md`, so the published numbers
//! cannot drift from the harness.

mod common;

use attest_family::anonymity::{pool, pool_size, present, tabulate};
use attest_family::fixtures::*;
use attest_family::issuer::{mint, IssuerState, MintEntropy, MintOutput};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// Mint + fold a whole co-op population; returns the folded state plus each
/// persona's published envelopes.
fn folded_coop(coop: &CoopFixture) -> (attest_family::fold::AttestState, Vec<(PersonaId, MintOutput)>) {
    let mut state = IssuerState::new(u32::MAX);
    let mut outs: Vec<(PersonaId, MintOutput)> = Vec::new();
    let mut k = 0u64;
    for (i, m) in coop.members.iter().enumerate() {
        for (j, persona) in m.personas.iter().enumerate() {
            let out = mint(
                &mut state,
                &coop.issuer,
                m.member,
                persona,
                &generated_kinds(i, j),
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed(coop.tag, 7_000 + k, 1)),
            )
            .expect("population mint succeeds");
            k += 1;
            outs.push((persona.id, out));
        }
    }
    let mut envs: Vec<Envelope> = Vec::new();
    for (_, out) in &outs {
        envs.push(out.vetting.clone());
        envs.extend(out.credentials.iter().cloned());
    }
    (log_from(&envs).fold(), outs)
}

// ---------------------------------------------------------------------------
// T-PA4.1 — the harness is correct on a hand-computed fixture (red-first)
// ---------------------------------------------------------------------------

#[test]
fn harness_correct_on_known_fixture() {
    // Six personas, hand-assigned credentials:
    //   A: vetted, over_18, phone, payment
    //   B: vetted, over_18, phone
    //   C: vetted, over_18
    //   D: vetted, over_18, phone, payment
    //   E: vetted, phone
    //   F: vetted
    // Hand-computed pools: vetted=6, over_18=4, phone=4, payment=2.
    // Hand-computed bundles: {over_18}=4; {over_18,phone}=3 (A,B,D);
    // {over_18,phone,payment}=2 (A,D).
    let issuer = PersonaFixture::new("COOP", Holder("I"), derived_seed("t-pa4-1", 0, 0), true);
    let assignments: [(&str, &[PredicateKind]); 6] = [
        ("A", &[PredicateKind::VettedHolder, PredicateKind::Over18, PredicateKind::PhoneVerified, PredicateKind::PaymentVerified]),
        ("B", &[PredicateKind::VettedHolder, PredicateKind::Over18, PredicateKind::PhoneVerified]),
        ("C", &[PredicateKind::VettedHolder, PredicateKind::Over18]),
        ("D", &[PredicateKind::VettedHolder, PredicateKind::Over18, PredicateKind::PhoneVerified, PredicateKind::PaymentVerified]),
        ("E", &[PredicateKind::VettedHolder, PredicateKind::PhoneVerified]),
        ("F", &[PredicateKind::VettedHolder]),
    ];
    let mut state = IssuerState::new(u32::MAX);
    let mut envs: Vec<Envelope> = Vec::new();
    let mut ids: Vec<PersonaId> = Vec::new();
    for (k, (_, kinds)) in assignments.iter().enumerate() {
        let persona =
            PersonaFixture::new("gen", Holder("GEN"), derived_seed("t-pa4-1-p", 0, k as u64), false);
        let out = mint(
            &mut state,
            &issuer,
            attest_family::issuer::MemberRef(derived_seed("t-pa4-1-m", 0, k as u64)),
            &persona,
            kinds,
            d(2026, 7, 17),
            MintEntropy::from_seed(derived_seed("t-pa4-1-e", 0, k as u64)),
        )
        .expect("fixture mint succeeds");
        envs.push(out.vetting.clone());
        envs.extend(out.credentials.iter().cloned());
        ids.push(persona.id);
    }
    let state = log_from(&envs).fold();

    // Exact hand-computed sizes — the harness must reproduce them before it
    // is allowed near COOP-S / COOP-L.
    assert_eq!(pool_size(&state, &issuer.id, &[PredicateKind::VettedHolder]), 6);
    assert_eq!(pool_size(&state, &issuer.id, &[PredicateKind::Over18]), 4);
    assert_eq!(pool_size(&state, &issuer.id, &[PredicateKind::PhoneVerified]), 4);
    assert_eq!(pool_size(&state, &issuer.id, &[PredicateKind::PaymentVerified]), 2);
    assert_eq!(
        pool_size(&state, &issuer.id, &[PredicateKind::Over18, PredicateKind::PhoneVerified]),
        3
    );
    assert_eq!(
        pool_size(
            &state,
            &issuer.id,
            &[PredicateKind::Over18, PredicateKind::PhoneVerified, PredicateKind::PaymentVerified]
        ),
        2
    );

    // Exact membership too, not just size: {over_18, phone, payment} = {A, D}.
    let p = pool(
        &state,
        &issuer.id,
        &[PredicateKind::Over18, PredicateKind::PhoneVerified, PredicateKind::PaymentVerified],
    );
    assert_eq!(p, [ids[0], ids[3]].into_iter().collect());

    // A pool never counts a persona whose credential is merely pending: drop
    // the vetting facts and every pool is empty.
    let bare: Vec<Envelope> = envs
        .iter()
        .filter(|e| matches!(e.payload, Payload::Credential(_)))
        .cloned()
        .collect();
    let state_bare = log_from(&bare).fold();
    assert_eq!(pool_size(&state_bare, &issuer.id, &[PredicateKind::VettedHolder]), 0);

    // And tabulate() agrees with pool_size per predicate.
    let table = tabulate(&state, &issuer.id);
    for (kind, n) in table {
        assert_eq!(n, pool_size(&state, &issuer.id, &[kind]), "{}", kind.as_str());
    }
}

// ---------------------------------------------------------------------------
// M-PA4.2 / M-PA4.3 — the measured tables (asserted equal to the FINDINGS doc)
// ---------------------------------------------------------------------------

/// The committed measurement tables. These constants are mirrored in
/// `FINDINGS-ANONYMITY-SETS.md`; this test recomputes them from the fixtures
/// so the document cannot drift. (Measurement, Modeled fixtures — the sizes
/// are properties of the generated populations, not protocol claims.)
mod committed {
    // (predicate, COOP-S pool, COOP-L pool)
    pub const PER_PREDICATE: [(&str, u64, u64); 4] = [
        ("vetted_holder", 15, 441),
        ("over_18", 14, 419),
        ("phone_verified", 10, 265),
        ("payment_verified", 7, 155),
    ];
    // (bundle, COOP-S pool, COOP-L pool)
    pub const BUNDLES: [(&str, u64, u64); 3] = [
        ("over_18", 14, 419),
        ("over_18+phone_verified", 9, 245),
        ("over_18+phone_verified+payment_verified", 5, 71),
    ];
}

#[test]
fn measure_anonymity_sets() {
    let small = coop_s();
    let large = coop_l();
    assert_eq!(small.members.len(), 12);
    assert_eq!(large.members.len(), 400);
    let (state_s, _) = folded_coop(&small);
    let (state_l, _) = folded_coop(&large);

    for (name, expect_s, expect_l) in committed::PER_PREDICATE {
        let kind = PredicateKind::from_str(name).unwrap();
        let got_s = pool_size(&state_s, &small.issuer.id, &[kind]);
        let got_l = pool_size(&state_l, &large.issuer.id, &[kind]);
        println!("M-PA4.2 {name}: COOP-S {got_s} COOP-L {got_l}");
        assert_eq!((got_s, got_l), (expect_s, expect_l), "{name}: doc drifted from harness");
    }

    for (bundle, expect_s, expect_l) in committed::BUNDLES {
        let kinds: Vec<PredicateKind> =
            bundle.split('+').map(|n| PredicateKind::from_str(n).unwrap()).collect();
        let got_s = pool_size(&state_s, &small.issuer.id, &kinds);
        let got_l = pool_size(&state_l, &large.issuer.id, &kinds);
        println!("M-PA4.3 {bundle}: COOP-S {got_s} COOP-L {got_l}");
        assert_eq!((got_s, got_l), (expect_s, expect_l), "{bundle}: doc drifted from harness");
    }

    // The deliverable exists and carries the same numbers.
    let doc = crate_source("FINDINGS-ANONYMITY-SETS.md");
    for (name, s, l) in committed::PER_PREDICATE {
        assert!(
            doc.contains(&format!("| `{name}` | {s} | {l} |")),
            "FINDINGS-ANONYMITY-SETS.md must carry the measured row for {name}"
        );
    }
    for (bundle, s, l) in committed::BUNDLES {
        assert!(
            doc.contains(&format!("| {s} | {l} |")) && doc.contains(bundle.split('+').next().unwrap()),
            "FINDINGS-ANONYMITY-SETS.md must carry the measured bundle row for {bundle}"
        );
    }
}

// ---------------------------------------------------------------------------
// T-PA4.4 — presentation is subset-capable
// ---------------------------------------------------------------------------

#[test]
fn presentation_is_subset_capable() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[
            PredicateKind::VettedHolder,
            PredicateKind::Over18,
            PredicateKind::PhoneVerified,
            PredicateKind::PaymentVerified,
        ],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-pa4-4", 0, 0)),
    )
    .expect("fixture mint succeeds");

    // Present over_18 ALONE.
    let shown = present(&out.credentials, &[PredicateKind::Over18]);
    assert_eq!(shown.shown.len(), 1);
    let bytes = shown.to_canonical_bytes();

    // The presented credential verifies from the presentation alone.
    let verified = attest_family::issuer::verify_credential(
        &shown.shown[0].canonical_bytes_with_sig(),
        &w.coop.id,
    )
    .expect("presented credential verifies");
    assert!(matches!(
        verified.payload,
        Payload::Credential(Credential { predicate: PredicateKind::Over18, .. })
    ));

    // NO trace of the unpresented credentials: not their object ids, not
    // their mint nonces, not any derivable value — and no count of how many
    // other credentials exist (the only numerics are the shown envelope's
    // own date/lamport/version fields).
    for env in &out.credentials {
        let Payload::Credential(c) = &env.payload else { panic!("mint returns credentials") };
        if c.predicate == PredicateKind::Over18 {
            continue;
        }
        assert!(
            !contains_subslice(&bytes, &env.object_id().0),
            "unpresented credential id leaked ({})",
            c.predicate.as_str()
        );
        assert!(
            !contains_subslice(&bytes, &c.mint_nonce),
            "unpresented mint nonce leaked ({})",
            c.predicate.as_str()
        );
        assert!(
            !contains_subslice(&bytes, blake3::hash(&env.object_id().0).as_bytes()),
            "derivable value of unpresented credential leaked ({})",
            c.predicate.as_str()
        );
    }
    let v: ipld_core::ipld::Ipld = serde_ipld_dagcbor::from_slice(&bytes).unwrap();
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&v, "", &mut numerics);
    for (path, val) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path.as_str());
        assert!(
            matches!(leaf_key, "d" | "m" | "y" | "l" | "v"),
            "presentation numeric outside the shown envelope's own fields: {path} = {val}"
        );
    }

    // Subset choice is free: any single predicate presents alone the same way.
    for kind in [PredicateKind::VettedHolder, PredicateKind::PaymentVerified] {
        let p = present(&out.credentials, &[kind]);
        assert_eq!(p.shown.len(), 1);
    }
    // And a pair presents exactly the pair.
    let pair = present(&out.credentials, &[PredicateKind::Over18, PredicateKind::PhoneVerified]);
    assert_eq!(pair.shown.len(), 2);
}
