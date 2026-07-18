//! RUN-ATTEST-02 EXP-PA6 — issuer covenant and no-record under multi-persona,
//! reworked to the V5 tree-head model (RUN-ATTEST-04 Part B).
//!
//! Replacements, named old → new:
//! - T-PA6.2's commitment-audit leg → T-A4.13 `audit_over_heads`
//!   (`t_a4_issuer_tree.rs`); the covenant-rule-lineage leg stays in
//!   `t_pa_substrate.rs` unchanged.
//! - T-PA6.3 `status_check_protocol` → DELETED with its machinery; successors
//!   are T-A4.11 `verifier_never_contacts_issuer` and T-A4.12
//!   `holder_verifies_own_inclusion`.
//! - T-PA6.4 `supersede_reaches_verifier_without_registry` → reworked below to
//!   the staple flow: supersession reaches verifiers through PUBLICATION (the
//!   next head's superseded set), never through an issuer round-trip.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::CredentialStatus;
use attest_family::issuer::{
    mint, verifier_accepts, verify_credential, verify_staple, CheckDial, IssuerState, MintEntropy,
    MintOutput,
};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

fn minted_world() -> (AnchorWorld, IssuerState, Vec<(PersonaId, MintOutput)>) {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let mut outs = Vec::new();
    for (k, (subject, holder)) in [
        (&w.p1a, &w.h1),
        (&w.p1b, &w.h1),
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
            &[PredicateKind::VettedHolder, PredicateKind::PhoneVerified],
            d(2026, 7, 17),
            MintEntropy::from_seed(derived_seed("t-pa6", 0, k as u64)),
        )
        .expect("fixture mint succeeds");
        outs.push((subject.id, out));
    }
    state.close_epoch(&w.coop);
    (w, state, outs)
}

// ---------------------------------------------------------------------------
// T-PA6.1 — issuer state is assertions + process only
// ---------------------------------------------------------------------------

#[test]
fn issuer_state_is_assertions_plus_process_only() {
    // Compile-boundary leg (the T-AT6.1 style, inherited): scan the retained
    // state's type region — `IssuerState`, `AssertionHead`, `SeamBoundary`,
    // `MemberRef`. Every field is a closed enum, fixed-size array, counter,
    // or map/set/list of those (the V5 rework adds the era anchor, the
    // confidential era-key seed, keyed commitments, and published heads —
    // all fixed-size values). No substrate-capable type (no String, no free
    // bytes, no Ipld), and — crucially — NO PersonaId: post-mint retained
    // state holds zero persona identifiers; holder linkage exists only as
    // the opaque member handle inside the type named `SeamBoundary`.
    let src = crate_source("src/issuer.rs");
    let mut region = String::new();
    for marker in [
        "pub struct IssuerState {",
        "pub(crate) struct AssertionHead {",
        "pub struct SeamBoundary {",
        "pub struct MemberRef(",
    ] {
        let start = src.find(marker).unwrap_or_else(|| panic!("missing {marker}"));
        let end = src[start..].find("\n}").map(|e| start + e).unwrap_or(src.len());
        region.push_str(&src[start..end]);
        region.push('\n');
    }
    for banned in [
        "String", "Vec<u8>", "&'static str", "&str", "Box<", "serde_json", "Ipld", "PersonaId",
        "Envelope", "Payload",
    ] {
        for (line_no, line) in code_lines(&region) {
            assert!(
                !line.contains(banned),
                "issuer retained-state region line {line_no} carries a substrate-capable or \
                 identity-capable type `{banned}`: {line}"
            );
        }
    }
    // The seam is present, exactly once, under its own name — the known
    // linkage point is named in the type system, so it cannot silently
    // spread to other fields.
    assert_eq!(
        region.matches("seam: SeamBoundary").count(),
        1,
        "the payment-bookkeeping stand-in must be typed as SeamBoundary"
    );

    // Runtime leg: everything the issuer EXPORTS after minting for several
    // multi-persona members is signed heads (V4 rework: the ONLY public
    // surface) plus the holder-channel staples — sweep both for persona
    // bytes.
    let (w, state, outs) = minted_world();
    let mut exports: Vec<Vec<u8>> = vec![state.lineage_bytes()];
    for (_, out) in &outs {
        for binding in &out.bindings {
            let staple = state.holder_staple(binding).expect("covered by the closed head");
            exports.push(binding.to_canonical_bytes());
            exports.push(
                serde_ipld_dagcbor::to_vec(&ipld_core::ipld::Ipld::List(
                    staple.proof.iter().map(|(_, h)| ipld_core::ipld::Ipld::Bytes(h.to_vec())).collect(),
                ))
                .unwrap(),
            );
        }
    }
    for surface in &exports {
        for p in [&w.p1a, &w.p1b, &w.p2a, &w.p3] {
            assert!(
                !contains_subslice(surface, &p.id.0),
                "issuer export carries a persona id"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// T-PA6.4 — a supersede reaches the verifier without any registry (staple
// model: through the NEXT published head, never an issuer round-trip)
// ---------------------------------------------------------------------------

#[test]
fn supersede_reaches_verifier_without_registry() {
    let (w, mut state, outs) = minted_world();
    let (pid, out) = &outs[0];
    assert_eq!(*pid, w.p1a.id);
    let (idx, phone) = out
        .credentials
        .iter()
        .enumerate()
        .find(|(_, e)| {
            matches!(
                &e.payload,
                Payload::Credential(c) if c.predicate == PredicateKind::PhoneVerified
            )
        })
        .map(|(i, e)| (i, e.clone()))
        .unwrap();
    let phone_bytes = phone.canonical_bytes_with_sig();
    let binding = &out.bindings[idx];

    // Before: the holder's staple against the published head verifies.
    let head1 = state.close_epoch(&w.coop); // republish current head (no new facts)
    let staple = state.holder_staple(binding).unwrap();
    assert!(verify_staple(&phone_bytes, &w.coop.id, &head1, &staple));
    assert!(verifier_accepts(
        &phone_bytes,
        &w.coop.id,
        Some((&head1, &staple)),
        CheckDial::RequireFreshStaple
    ));

    // The issuer supersedes P1a's phone_verified; the NEXT head publishes it.
    let sup = state.supersede(&w.coop, &phone);
    let head2 = state.close_epoch(&w.coop);

    // P1a cannot present the credential against the fresh head — the
    // supersession reached the verifier through publication alone.
    let staple2 = state.holder_staple(binding).unwrap();
    assert!(!verify_staple(&phone_bytes, &w.coop.id, &head2, &staple2));
    assert!(!verifier_accepts(
        &phone_bytes,
        &w.coop.id,
        Some((&head2, &staple2)),
        CheckDial::RequireFreshStaple
    ));
    // The old object ITSELF still verifies as a signature (it is intact) —
    // status, not existence, is what moved.
    assert!(verify_credential(&phone_bytes, &w.coop.id).is_ok());

    // And in any shared log, the fold shows the supersede with the old
    // object's bytes unchanged (T-AT0.3 invariant, end to end).
    let mut envs: Vec<Envelope> = Vec::new();
    for (_, o) in &outs {
        envs.push(o.vetting.clone());
        envs.extend(o.credentials.iter().cloned());
    }
    let mut log = log_from(&envs);
    log.append(sup.clone()).unwrap();
    let folded = log.fold();
    let view = folded.credential(&phone.object_id()).expect("superseded credential persists");
    assert_eq!(view.status, CredentialStatus::Superseded { by: sup.object_id() });
    assert_eq!(
        log.object_bytes(&phone.object_id()).unwrap(),
        &phone_bytes[..],
        "the superseded object's bytes are unchanged in lineage"
    );
    // The persona's sibling credentials are untouched by the supersede.
    let (sib_id, sib_out) = &outs[1];
    assert_eq!(*sib_id, w.p1b.id);
    for env in &sib_out.credentials {
        assert_eq!(
            folded.credential(&env.object_id()).unwrap().status,
            CredentialStatus::Standing
        );
    }
}
