//! Part 6 (lib half) — EXP-AT6: issuer predicates. T-AT6.1–6.3.
//!
//! The co-op issuer asserts predicates ("over_18", "phone_verified",
//! "payment_verified") about a persona. The payload is predicate + subject +
//! issuer (envelope author) + process-provenance metadata. The substrate (ID
//! number, card number) is unrepresentable.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{Marker, PredicateStatus};
use attest_family::query::FreshnessDial;
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

// ---------------------------------------------------------------------------
// T-AT6.1 — the substrate is unrepresentable (compile boundary, EXP-B style)
// ---------------------------------------------------------------------------

/// The compile_fail doc-tests on [`attest_family::types::Predicate`] show that
/// attempting to attach substrate does not compile. This test pins the same
/// boundary at the type-definition level: the predicate payload types consist
/// exclusively of closed enums and fixed-shape claims — no field type capable
/// of carrying free-form substrate exists.
#[test]
fn substrate_unrepresentable() {
    let src = crate_source("src/types.rs");

    // Extract the struct/enum bodies of the predicate family.
    let mut region = String::new();
    for marker in [
        "pub struct Predicate {",
        "pub enum PredicateKind {",
        "pub struct ProcessProvenance {",
        "pub enum MethodKind {",
        "pub enum IssuerRole {",
    ] {
        let start = src.find(marker).unwrap_or_else(|| panic!("missing {marker}"));
        let end = src[start..].find("\n}").map(|e| start + e).unwrap();
        region.push_str(&src[start..end]);
        region.push('\n');
    }

    // No free-form or open-ended field type may appear in the predicate family.
    for banned in ["String", "Vec<u8>", "&'static str", "&str", "Box<", "serde_json", "Ipld"] {
        for (line_no, line) in code_lines(&region) {
            assert!(
                !line.contains(banned),
                "predicate family line {line_no} carries a substrate-capable type `{banned}`: {line}"
            );
        }
    }

    // The closed enums are exactly the declared vocabulary.
    assert_eq!(PredicateKind::Over18.as_str(), "over_18");
    assert_eq!(PredicateKind::PhoneVerified.as_str(), "phone_verified");
    assert_eq!(PredicateKind::PaymentVerified.as_str(), "payment_verified");
    assert!(PredicateKind::from_str("ssn").is_none());
    assert!(MethodKind::from_str("free_text").is_none());
}

// ---------------------------------------------------------------------------
// T-AT6.2 — a predicate is inseparable from its issuer and process
// ---------------------------------------------------------------------------

#[test]
fn predicate_inseparable_from_issuer_and_process() {
    let w = World::new();
    assert!(w.coop.issuer, "COOP carries the issuer role marker");
    let pr = w.coop.emit(
        vec![],
        predicate(
            PredicateKind::Over18,
            w.p2.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 17)),
            None,
        ),
    );

    // Every serialization of the predicate carries issuer + process: decode
    // the canonical bytes and check the envelope is the only form (author =
    // issuer, signed), and the payload embeds the process block.
    let bytes = pr.canonical_bytes_with_sig();
    let decoded = attest_family::canonical::decode_envelope(&bytes).unwrap();
    assert_eq!(decoded.author, w.coop.id, "the issuer IS the signed author");
    match &decoded.payload {
        Payload::Predicate(p) => {
            assert_eq!(p.process.method, MethodKind::DocumentSighted);
            assert_eq!(p.process.role, IssuerRole::CoopIssuer);
            assert_eq!(p.process.performed_on, d(2026, 7, 17));
        }
        other => panic!("expected predicate, got {}", other.kind_str()),
    }

    // The folded view carries issuer + process too — no code path yields a
    // bare "over_18: true" detached from who asserted it and how.
    let state = log_from(std::slice::from_ref(&pr)).fold();
    let view = &state.predicates()[0];
    assert_eq!(view.issuer, w.coop.id);
    assert_eq!(view.subject, w.p2.id);
    assert_eq!(view.predicate, PredicateKind::Over18);
    assert_eq!(view.process.method, MethodKind::DocumentSighted);
    assert_eq!(view.process.role, IssuerRole::CoopIssuer);

    // And the crate exposes no function returning a predicate value without
    // the view (source-scan: nothing public returns PredicateKind).
    for file in ["src/fold.rs", "src/query.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let t = line.trim();
            if t.starts_with("pub fn ") {
                assert!(
                    !t.contains("-> PredicateKind") && !t.contains("-> Option<PredicateKind>"),
                    "{file}:{line_no}: a public operation detaches the predicate from its provenance: {line}"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// T-AT6.3 — refresh by supersede; staleness is presentation, never expiry
// ---------------------------------------------------------------------------

#[test]
fn expiring_predicate_via_supersede() {
    let w = World::new();
    let v1 = w.coop.emit(
        vec![],
        predicate(
            PredicateKind::PhoneVerified,
            w.p2.id,
            coop_process(MethodKind::SmsRoundTrip, d(2025, 1, 10)),
            None,
        ),
    );
    let mut log = log_from(std::slice::from_ref(&v1));
    let v1_bytes = log.object_bytes(&v1.object_id()).unwrap().to_vec();

    // Refresh = supersede, citing the stale predicate.
    let v2 = w.coop.emit(
        vec![v1.object_id()],
        predicate(
            PredicateKind::PhoneVerified,
            w.p2.id,
            coop_process(MethodKind::SmsRoundTrip, d(2026, 7, 1)),
            Some(v1.object_id()),
        ),
    );
    log.append(v2.clone()).unwrap();

    let state = log.fold();
    let v1_view = state.predicates().iter().find(|p| p.object == v1.object_id()).unwrap().clone();
    let v2_view = state.predicates().iter().find(|p| p.object == v2.object_id()).unwrap().clone();
    assert_eq!(v1_view.status, PredicateStatus::Superseded { by: v2.object_id() });
    assert_eq!(v2_view.status, PredicateStatus::Current);
    assert_eq!(v1_view.lineage, vec![v1.object_id(), v2.object_id()], "stale predicate persists in lineage");
    assert_eq!(log.object_bytes(&v1.object_id()).unwrap(), &v1_bytes[..], "bytes unchanged (T-AT0.3)");

    // Staleness under the governed dial is PRESENTATION (per T-AT5.5): the
    // stale claim gains a marker; nothing expires by timeout.
    let dial = FreshnessDial { stale_after_days: 180 };
    let as_of = d(2026, 7, 18);
    assert_eq!(
        state.predicate_presentation(&v1_view, &dial, as_of),
        vec![Marker::Stale],
        "old process date → staleness presentation"
    );
    assert_eq!(state.predicate_presentation(&v2_view, &dial, as_of), Vec::<Marker>::new());
    // Both remain in the state regardless of any dial — never expiry-by-timeout.
    assert_eq!(state.predicates().len(), 2);
}
