//! RUN-ATTEST-02 EXP-PA6 — issuer covenant and no-record under multi-persona.
//! T-PA6.1, T-PA6.2 (commitment-audit leg; the covenant-rule-lineage leg is
//! in `t_pa_substrate.rs`), T-PA6.3, T-PA6.4.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::CredentialStatus;
use attest_family::issuer::{
    audit_lineage, mint, verifier_accepts, verify_credential, verify_status_response, AuditFailure,
    CheckDial, CredStanding, IssuerState, MintEntropy, MintOutput,
};
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;
use std::collections::BTreeMap;

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
    // or map/set of those. No substrate-capable type (no String, no free
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
    // multi-persona members is commitments + signed assertions — sweep the
    // export surface for persona bytes.
    let (w, state, outs) = minted_world();
    let mut exports: Vec<Vec<u8>> = vec![state.lineage_bytes()];
    for (_, out) in &outs {
        for env in &out.credentials {
            exports.push(state.status_check(&w.coop, env.object_id().0).to_canonical_bytes());
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
// T-PA6.2 — covenant audit without unmasking (commitment-audit leg)
// ---------------------------------------------------------------------------

#[test]
fn covenant_audit_without_unmasking() {
    let (w, mut state, _outs) = minted_world();
    // A second epoch so the audit walks a real lineage.
    let _out5 = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h4),
        &w.p4,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 18),
        MintEntropy::from_seed(derived_seed("t-pa6-2", 1, 1)),
    )
    .expect("second epoch mint succeeds");
    state.close_epoch(&w.coop);

    // Honest lineage: audit passes, resolving totals ONLY.
    let honest = state.lineage_bytes();
    let report = audit_lineage(&honest, &w.coop.id).expect("honest lineage audits clean");
    assert_eq!(report.epochs_audited, 2);
    assert_eq!(report.total_commitments, 9, "4 personas × 2 predicates + 1 × 1");

    // Zero unmasking: the audit's input contains no persona identifier (the
    // T-PA1.3 floor) and its OUTPUT is a totals-only report — the report
    // type has exactly two counter fields and nothing else.
    let report_src = crate_source("src/issuer.rs");
    let start = report_src.find("pub struct AuditReport {").unwrap();
    let end = report_src[start..].find("\n}").map(|e| start + e).unwrap();
    let fields: Vec<String> = code_lines(&report_src[start..end])
        .into_iter()
        .filter(|(_, l)| l.trim().starts_with("pub "))
        .map(|(_, l)| l)
        .collect();
    assert_eq!(fields.len(), 3, "AuditReport = the struct line + two counters: {fields:?}");

    // Tampered fixtures fail:
    // (1) a dropped commitment (declared total no longer matches) — signed
    //     freshly by the issuer so ONLY the count check can catch it;
    let decode = |bytes: &[u8]| -> Vec<BTreeMap<String, Ipld>> {
        let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).unwrap();
        let Ipld::List(l) = v else { panic!("lineage is a list") };
        l.into_iter()
            .map(|r| {
                let Ipld::Map(m) = r else { panic!("record is a map") };
                m
            })
            .collect()
    };
    let reencode = |records: Vec<BTreeMap<String, Ipld>>| -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Ipld::List(records.into_iter().map(Ipld::Map).collect()))
            .unwrap()
    };
    let resign = |m: &mut BTreeMap<String, Ipld>| {
        // Re-sign the tampered record with the REAL issuer key over the
        // c/e/t triple, mirroring the honest signing bytes.
        let mut unsigned = m.clone();
        unsigned.remove("g");
        let bytes = serde_ipld_dagcbor::to_vec(&Ipld::Map(unsigned)).unwrap();
        m.insert("g".into(), Ipld::Bytes(w.coop.sign_bytes(&bytes)));
    };

    let mut dropped = decode(&honest);
    {
        let m = &mut dropped[0];
        let Some(Ipld::List(c)) = m.get_mut("c") else { panic!("commitments list") };
        c.pop();
        resign(m);
    }
    assert_eq!(
        audit_lineage(&reencode(dropped), &w.coop.id).expect_err("dropped commitment must fail"),
        AuditFailure::TotalMismatch { epoch_no: 1 }
    );

    // (2) a malformed commitment shape (31 bytes), freshly signed;
    let mut malformed = decode(&honest);
    {
        let m = &mut malformed[1];
        let Some(Ipld::List(c)) = m.get_mut("c") else { panic!("commitments list") };
        c[0] = Ipld::Bytes(vec![0u8; 31]);
        resign(m);
    }
    assert_eq!(
        audit_lineage(&reencode(malformed), &w.coop.id).expect_err("malformed shape must fail"),
        AuditFailure::MalformedCommitment { epoch_no: 2 }
    );

    // (3) a flipped byte without re-signing — the signature catches it;
    let mut forged = decode(&honest);
    {
        let m = &mut forged[0];
        let Some(Ipld::Integer(t)) = m.get("t") else { panic!("total") };
        let t = *t + 1;
        m.insert("t".into(), Ipld::Integer(t));
    }
    assert_eq!(
        audit_lineage(&reencode(forged), &w.coop.id).expect_err("unsigned tamper must fail"),
        AuditFailure::BadSignature { epoch_no: 1 }
    );

    // (4) a gap in the epoch chain (lineage no longer intact).
    let gapped = decode(&honest).into_iter().skip(1).collect::<Vec<_>>();
    assert_eq!(
        audit_lineage(&reencode(gapped), &w.coop.id).expect_err("gapped lineage must fail"),
        AuditFailure::NonContiguousEpochs
    );
}

// ---------------------------------------------------------------------------
// T-PA6.3 — the status-check protocol
// ---------------------------------------------------------------------------

#[test]
fn status_check_protocol() {
    let (w, state, outs) = minted_world();
    let cred = outs[0].1.credentials[0].clone();

    // Deterministic: the same question twice yields byte-identical signed
    // answers, derived from the issuer's own assertion lineage.
    let r1 = state.status_check(&w.coop, cred.object_id().0);
    let r2 = state.status_check(&w.coop, cred.object_id().0);
    assert_eq!(r1.to_canonical_bytes(), r2.to_canonical_bytes());
    assert_eq!(r1.standing, CredStanding::Current);
    assert_eq!(r1.credential_hash, cred.object_id().0, "the answer echoes the question");
    assert!(verify_status_response(&r1, &w.coop.id), "the answer is issuer-signed");
    // A forged standing does not verify.
    let mut forged = r1.clone();
    forged.standing = CredStanding::Superseded;
    assert!(!verify_status_response(&forged, &w.coop.id));

    // Unknown hash: deterministic `unknown`, signed — never an error, never
    // a fabricated verdict.
    let unknown = state.status_check(&w.coop, [0xEE; 32]);
    assert_eq!(unknown.standing, CredStanding::Unknown);
    assert!(verify_status_response(&unknown, &w.coop.id));

    // Staleness is presentation, never verdict: the protocol response has no
    // expiry and no timestamp (pinned byte-exactly in T-PA2.3); whether an
    // app requires a fresh answer is ITS dial, and choosing to require one
    // fails closed — no answer, no acceptance. No fail-open badge exists.
    let bytes = cred.canonical_bytes_with_sig();
    assert!(verifier_accepts(&bytes, &w.coop.id, Some(&r1), CheckDial::RequireCurrentStatus));
    assert!(
        !verifier_accepts(&bytes, &w.coop.id, None, CheckDial::RequireCurrentStatus),
        "requiring freshness without an answer fails CLOSED (app policy, not protocol verdict)"
    );
    assert!(
        verifier_accepts(&bytes, &w.coop.id, None, CheckDial::SignatureOnly),
        "accepting signature-only is equally an app choice; silence is not a verdict either way"
    );
}

// ---------------------------------------------------------------------------
// T-PA6.4 — a supersede reaches the verifier without any registry
// ---------------------------------------------------------------------------

#[test]
fn supersede_reaches_verifier_without_registry() {
    let (w, mut state, outs) = minted_world();
    let (pid, out) = &outs[0];
    assert_eq!(*pid, w.p1a.id);
    let phone = out
        .credentials
        .iter()
        .find(|e| {
            matches!(
                &e.payload,
                Payload::Credential(c) if c.predicate == PredicateKind::PhoneVerified
            )
        })
        .unwrap()
        .clone();
    let phone_bytes = phone.canonical_bytes_with_sig();

    // Before: a checking verifier accepts.
    let before = state.status_check(&w.coop, phone.object_id().0);
    assert!(verifier_accepts(&phone_bytes, &w.coop.id, Some(&before), CheckDial::RequireCurrentStatus));

    // The issuer supersedes P1a's phone_verified.
    let sup = state.supersede(&w.coop, &phone);

    // The verifier's next status check gets `superseded`, signed.
    let after = state.status_check(&w.coop, phone.object_id().0);
    assert_eq!(after.standing, CredStanding::Superseded);
    assert!(verify_status_response(&after, &w.coop.id));

    // P1a cannot present the old credential as current against a checking
    // verifier — with the fresh answer OR with the stale one already in hand
    // the acceptance is gone the moment checking happens.
    assert!(!verifier_accepts(&phone_bytes, &w.coop.id, Some(&after), CheckDial::RequireCurrentStatus));
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
