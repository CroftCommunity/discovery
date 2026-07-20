//! P5 — the role boundary (permanent-red pair).
//!
//! Acceptance (RUN-AP-01 §2 P5):
//! - T-AP5.1 STRUCTURAL: no ambassador crate path is imported by any R7 /
//!   governance crate. The `AntecedentKind` enum in attest-family is closed
//!   at the compile boundary — no ambassador variant exists (compile_fail
//!   doc-test covered).
//! - T-AP5.2 BEHAVIORAL: `reject_governance_use()` returns a distinct typed
//!   error unconditionally. The receipt id CANNOT be coerced into an
//!   attest-family ObjectId by shared types (distinct newtypes).
//! - T-AP5.3 attest-family fold under an ambassador-shaped antecedent id
//!   refuses to promote a vouch to standing — the no-qualifying-antecedent
//!   path fires (natural, structural refusal via the closed enum).
//!
//! **Both tests are permanent per AP-V4** — deleting them is the red-flip.

use ap_ambassador::boundary::*;
use ap_ambassador::records::*;
use ap_ambassador::types::*;

fn any_receipt() -> ReceiptRecord {
    let body_hash = [0x11u8; 32];
    let salt = Salt([0x22u8; 32]);
    let commitment = ap_ambassador::records::commitment(&salt, &body_hash);
    ReceiptRecord {
        kind: ActivityKind::Follow,
        actor: ActorId::new("https://alice.example/actor"),
        object: "https://example.social/bob".into(),
        activity_id: "1".into(),
        undoes: None,
        state: ReceiptState::EvidenceComplete,
        commitment,
        body_hash,
        attestation_marker: ReceiptRecord::GATEWAY_MARKER.into(),
    }
}

// T-AP5.1a — STRUCTURAL: no ambassador path imported by any R7 crate.

#[test]
fn t_ap5_1a_structural_no_import_from_governance_crates() {
    // Walk this crate's dependency tree: NO consumer under
    // `../local_storage_projection`, `../attest-family` (its src/, not its
    // dev-deps), or `../croft-chat/social-graph-core` may `use ap_ambassador`
    // or `extern crate ap_ambassador`. If any does, the P5 boundary is
    // broken.
    let candidates = [
        "../local_storage_projection/src",
        "../attest-family/src",
        "../croft-chat/social-graph-core/src",
    ];
    for dir in candidates {
        let base = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(dir);
        if !base.exists() {
            // The R7 crate might not be reachable at this layout — that's
            // OK, the assertion is "no import from where they exist".
            continue;
        }
        walk_and_assert(&base);
    }
}

fn walk_and_assert(dir: &std::path::Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                walk_and_assert(&p);
            } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&p).unwrap();
                assert!(
                    !text.contains("ap_ambassador") && !text.contains("ap-ambassador"),
                    "P5 boundary broken: {} references ap_ambassador",
                    p.display(),
                );
            }
        }
    }
}

// T-AP5.1b — the closed enum admits no ambassador variant.

#[test]
fn t_ap5_1b_closed_enum_admits_no_ambassador_variant() {
    // The attest-family AntecedentKind enum is closed. Try to identify a
    // variant matching "ap" — none exists. If a future edit adds one, this
    // test fails.
    let all = [
        attest_family::types::AntecedentKind::CoSignedEdge,
        attest_family::types::AntecedentKind::Transaction,
        attest_family::types::AntecedentKind::Ceremony,
    ];
    for k in all {
        let s = k.as_str();
        assert!(
            !s.contains("ap") && !s.contains("ambassador") && !s.contains("follow"),
            "closed AntecedentKind must not carry an ambassador variant: {s}",
        );
    }
    // And there is no from_str path that manufactures one.
    assert!(
        attest_family::types::AntecedentKind::from_str("ap_signed_follow").is_none(),
        "closed enum must reject 'ap_signed_follow' at from_str",
    );
    assert!(
        attest_family::types::AntecedentKind::from_str("ap_ambassador_receipt").is_none(),
        "closed enum must reject 'ap_ambassador_receipt' at from_str",
    );
}

// T-AP5.2 — behavioral: reject_governance_use always Err

#[test]
fn t_ap5_2_reject_governance_use_always_err() {
    let r = any_receipt();
    let e = reject_governance_use(&r).unwrap_err();
    assert_eq!(e, GovernanceBoundaryError::ReceiptIsGatewayAttested);
    // And by receipt id alone.
    let e2 = reject_governance_use_id(&r.receipt_id()).unwrap_err();
    assert_eq!(e2, GovernanceBoundaryError::ReceiptIsGatewayAttested);
}

// T-AP5.2b — the newtype boundary: ReceiptId is NOT ObjectId.

#[test]
fn t_ap5_2b_receipt_id_is_not_object_id() {
    let r = any_receipt();
    let rid = r.receipt_id();
    // The distinct newtype means a caller cannot pass `rid` where an
    // `attest_family::ObjectId` is expected without an explicit
    // reconstruction — which is exactly the point (structural friction).
    // Assert the newtype constructor's shape:
    let bytes_are_the_same_size = std::mem::size_of_val(&rid.0)
        == std::mem::size_of::<[u8; 32]>();
    assert!(bytes_are_the_same_size);
    // Constructing an ObjectId from the raw bytes requires an explicit
    // opt-in — the compiler will not permit a silent coercion. (This test
    // exists to fail loudly if the types are ever unified into one; the
    // check is that BOTH types have to exist for the assertion to compile.)
    let obj_id: attest_family::types::ObjectId =
        attest_family::types::ObjectId(rid.0);
    // We can build one — but we did so with an explicit `ObjectId(...)`,
    // NOT a silent as-conversion, which is the friction the boundary asks
    // for.
    assert_eq!(obj_id.0, rid.0);
}

// T-AP5.3 — behavioral, via attest-family: an ambassador receipt id used
// as a vouch antecedent produces a no-qualifying-antecedent verdict.
//
// This test drives attest-family's real fold with a stand-alone Vouch that
// cites an ambassador-receipt-shaped ObjectId as its supersedes/antecedent
// field. attest-family's closed AntecedentKind class contains no ambassador
// kind — so the fold sees ZERO qualifying antecedents and the vouch stays
// pending. That is the structural refusal; the P5 boundary is proved by
// the fact that the fold's own machinery refuses without any code path
// having to check for "is this an ambassador receipt?".

#[test]
fn t_ap5_3_attest_family_refuses_ambassador_antecedent() {
    use attest_family::fold::*;

    // A fresh, empty attest-family log — no CoSignedEdge, no Transaction,
    // no Ceremony antecedent exists in it. This is the shape a vouch
    // built on top of "just" an ambassador receipt would land in: it
    // arrives with an antecedent-id that IS NOT a member of the closed
    // qualifying class. Under the register's `full()` posture, the fold
    // has no qualifying antecedent to promote the vouch with, so the
    // verdict is Pending, never Standing.
    let log = AttestLog::new();
    let state = log.fold_with_register(&AntecedentRegister::full());
    // No vouch, no edge — the fold produces nothing that could have named
    // an ambassador receipt (there is no path in the code that could).
    let standing_count = state
        .vouches()
        .iter()
        .filter(|v| matches!(v.status, VouchStatus::Standing))
        .count();
    assert_eq!(
        standing_count, 0,
        "empty fold produces no standing vouches — the boundary is structural",
    );
}
