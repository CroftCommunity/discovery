//! RUN-ATTEST-04 Part B — V5 + V6 + era-reissue: tree heads over keyed
//! commitments, holder stapling, and era-graded membership.
//! T-A4.6–T-A4.8, T-A4.10–T-A4.16 (T-A4.9, the governed cadence register,
//! lives in `t_pa_substrate.rs` with the other substrate-reuse tests).

mod common;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

use attest_family::fixtures::*;
use attest_family::fold::{CredentialStatus, CredentialView};
use attest_family::issuer::{
    audit_heads, mint, reissue, verifier_accepts, verify_staple, verify_tree_head, AuditFailure,
    CheckDial, IssuerState, MintEntropy, MintOutput, ReissueRefusal, Staple, TreeHead,
};
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

/// Mint `n_creds` credentials across distinct single-anchor personas, one
/// predicate each, in one issuer state. Returns the outputs.
fn mint_n(state: &mut IssuerState, coop: &PersonaFixture, tag: &str, n_creds: usize) -> Vec<MintOutput> {
    (0..n_creds)
        .map(|k| {
            let holder = Holder("HG");
            let subject =
                PersonaFixture::new("G", holder, derived_seed(tag, 100 + k as u64, 0), false);
            mint(
                state,
                coop,
                attest_family::issuer::MemberRef(derived_seed(tag, 200 + k as u64, 1)),
                &subject,
                &[PredicateKind::VettedHolder],
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed(tag, k as u64, 2)),
            )
            .expect("fixture mint succeeds")
        })
        .collect()
}

// ---------------------------------------------------------------------------
// T-A4.6 — the signed tree head is the ONLY publication
// ---------------------------------------------------------------------------

/// The public issuer surface is signed heads and nothing per-credential
/// (V5). Staged-violation red: a per-credential receipt on the public
/// surface (the pre-V5 commitment pile shape) fails every prong here.
#[test]
fn tree_head_is_the_only_publication() {
    let w = AnchorWorld::new();

    // Two worlds, same head count, very different issuance volumes.
    let surface = |n_creds: usize, tag: &str| -> (Vec<u8>, Vec<MintOutput>) {
        let mut state = IssuerState::new(u32::MAX);
        let outs = mint_n(&mut state, &w.coop, tag, n_creds);
        state.close_epoch(&w.coop);
        (state.lineage_bytes(), outs)
    };
    let (small, outs_small) = surface(3, "t-a4-6-s");
    let (large, outs_large) = surface(17, "t-a4-6-l");

    // Nothing per-credential: the byte-leaf population of the public surface
    // is a PER-HEAD constant — identical between a 3-credential epoch and a
    // 17-credential epoch (era anchor, root, superseded root, signature; an
    // empty superseded set). A receipt pile cannot hide in here.
    let leaves_small = byte_leaves_of(&small);
    let leaves_large = byte_leaves_of(&large);
    assert_eq!(
        leaves_small.len(),
        leaves_large.len(),
        "publication size must not scale with issuance count — heads only"
    );
    assert_eq!(leaves_small.len(), 4, "one head = era anchor, root, superseded root, signature");

    // The only numerics are the epoch number and the leaf COUNT (epoch-grain
    // volume is part of V5's honest value claim).
    for bytes in [&small, &large] {
        let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).unwrap();
        let mut numerics = Vec::new();
        ipld_numeric_leaves(&v, "", &mut numerics);
        for (path, _) in &numerics {
            let leaf_key = path.rsplit('.').next().unwrap_or(path.as_str());
            assert!(matches!(leaf_key, "e" | "n"), "head numeric outside epoch/count: {path}");
        }
    }

    // No credential id, derived credential value, or persona id exists on
    // the surface (the T-PA1.3 floor, inherited by the tree model).
    for (bytes, outs) in [(&small, &outs_small), (&large, &outs_large)] {
        for out in outs.iter() {
            for env in out.credentials.iter().chain(std::iter::once(&out.vetting)) {
                assert!(
                    !contains_subslice(bytes, &env.object_id().0),
                    "credential/vetting id on the public surface"
                );
                assert!(
                    !contains_subslice(bytes, blake3::hash(&env.object_id().0).as_bytes()),
                    "derived credential value on the public surface"
                );
            }
            // The holder-held commitment is NOT published outside a
            // superseded set (none exists here).
            for b in &out.bindings {
                assert!(
                    !contains_subslice(bytes, &b.commitment),
                    "a live commitment leaked to the public surface"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// T-A4.7 — leaves sort canonically by commitment value (successor of
// T-PA1.4 `commitment_fold_is_unordered_per_epoch`, replaced old → new)
// ---------------------------------------------------------------------------

// OWNER-CALL: PA OC-2 DECIDED (V6, 2026-07-18, owner-confirmed in chat):
// the ledger posture is inherited from V5 — canonical-order leaves, epoch
// grain, nothing finer published; ceremony spacing is USER GUIDANCE (the
// co-op informs, and honors either choice without friction — a PRIMITIVES
// sentence, not a mechanism); and issuer operational time is governance time
// (the era-anchoring move, T-A4.9's register + the era'd heads here).
#[test]
fn leaves_canonical_by_commitment() {
    let w = AnchorWorld::new();
    let sibs = [&w.p1a, &w.p1b, &w.p1c];
    let strangers: Vec<PersonaFixture> = (0..4)
        .map(|i| PersonaFixture::new("S", Holder("HS"), derived_seed("t-a4-7-s", 9, i), false))
        .collect();

    // The same mint SET in two different call orders (siblings batched first
    // vs interleaved), same per-subject entropy: the published head must be
    // byte-identical — mint order is structurally absent from the tree
    // (F-PA-3's rule as behavior: leaves sort by commitment value alone).
    let run = |order: &[usize]| -> (TreeHead, Vec<u8>) {
        let mut state = IssuerState::new(u32::MAX);
        for &k in order {
            let (subject, holder): (&PersonaFixture, &Holder) =
                if k < 3 { (sibs[k], &w.h1) } else { (&strangers[k - 3], &Holder("HS")) };
            mint(
                &mut state,
                &w.coop,
                member_ref(holder),
                subject,
                &[PredicateKind::VettedHolder],
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed("t-a4-7-e", 0, k as u64)),
            )
            .expect("fixture mint succeeds");
        }
        let head = state.close_epoch(&w.coop);
        (head, state.lineage_bytes())
    };

    let (head_batched, bytes_batched) = run(&[0, 1, 2, 3, 4, 5, 6]);
    let (head_interleaved, bytes_interleaved) = run(&[3, 0, 4, 5, 1, 6, 2]);
    assert_eq!(head_batched.root, head_interleaved.root, "permuted mint orders → identical tree");
    assert_eq!(
        bytes_batched, bytes_interleaved,
        "the whole public surface is mint-order-free (adjacency cannot exist)"
    );
    assert_eq!(head_batched.leaf_count, 7);
}

// ---------------------------------------------------------------------------
// T-A4.8 — keyed commitments resist the dictionary
// ---------------------------------------------------------------------------

/// An outsider holding a guessed (or even the real, published) credential id
/// cannot confirm membership without the era key: no un-keyed or
/// public-value-keyed derivation of the credential id appears anywhere on
/// the public surface — including the published superseded set.
#[test]
fn keyed_commitments_resist_dictionary() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let outs = mint_n(&mut state, &w.coop, "t-a4-8", 3);
    state.close_epoch(&w.coop);
    // One supersession, so the superseded set (commitments!) is public too.
    let target = outs[0].credentials[0].clone();
    state.supersede(&w.coop, &target);
    let head2 = state.close_epoch(&w.coop);
    let surface = state.lineage_bytes();
    let surface_leaves: BTreeSet<Vec<u8>> =
        byte_leaves_of(&surface).into_iter().filter(|l| l.len() == 32).collect();

    let era = attest_family::issuer::genesis_era();
    for out in &outs {
        for env in &out.credentials {
            let id = env.object_id().0;
            // The dictionary: every derivation an outsider can compute from
            // the credential id and PUBLIC values (the era anchor is public
            // in the credential payload). None may confirm membership.
            let mut guesses: Vec<[u8; 32]> = vec![
                id,
                *blake3::hash(&id).as_bytes(),
                *blake3::keyed_hash(&era, &id).as_bytes(),
                *blake3::keyed_hash(&id, &era).as_bytes(),
            ];
            let mut cat = Vec::new();
            cat.extend_from_slice(&id);
            cat.extend_from_slice(&era);
            guesses.push(*blake3::hash(&cat).as_bytes());
            cat.clear();
            cat.extend_from_slice(&era);
            cat.extend_from_slice(&id);
            guesses.push(*blake3::hash(&cat).as_bytes());
            for g in &guesses {
                assert!(
                    !surface_leaves.contains(g.as_slice()),
                    "an outsider-computable derivation of a credential id appears on the \
                     public surface — the commitment is not keyed"
                );
            }
        }
    }

    // Positive control: the ISSUER (holding the era key) did produce exactly
    // the keyed image — the superseded credential's holder-held commitment
    // is the one in the published superseded set.
    let binding = &outs[0].bindings[0];
    assert_eq!(binding.credential_hash, target.object_id().0);
    assert!(
        head2.superseded.contains(&binding.commitment),
        "the superseded set carries the keyed commitment (and only the key ties it back)"
    );
}

// ---------------------------------------------------------------------------
// T-A4.10 — a superseded credential cannot staple a fresh-head proof
// ---------------------------------------------------------------------------

#[test]
fn superseded_cannot_staple_fresh() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-a4-10", 0, 0)),
    )
    .expect("fixture mint succeeds");
    let head1 = state.close_epoch(&w.coop);
    let cred = out.credentials[0].clone();
    let cred_bytes = cred.canonical_bytes_with_sig();
    let binding = &out.bindings[0];

    // Standing: the holder staples against the published head; verification
    // is pure and passes.
    let staple1 = state.holder_staple(binding).expect("proof exists after the covering head");
    assert!(verify_staple(&cred_bytes, &w.coop.id, &head1, &staple1));

    // The issuer supersedes; the commitment enters the epoch's superseded
    // set, published with the NEXT head.
    state.supersede(&w.coop, &cred);
    let head2 = state.close_epoch(&w.coop);
    assert!(head2.superseded.contains(&binding.commitment));

    // Fresh-head staple: refused. The inclusion proof still reaches the root
    // (the leaf was minted), but the fresh head's superseded set kills it —
    // supersession travels to verifiers through publication, not through any
    // issuer round-trip.
    let staple2 = state.holder_staple(binding).expect("proof still derivable");
    assert!(
        !verify_staple(&cred_bytes, &w.coop.id, &head2, &staple2),
        "a superseded credential must not verify against a fresh head"
    );

    // The OLD head still verifies the old staple — era/epoch-stamped truth.
    // Whether that is acceptable is the app's freshness dial, fail-closed as
    // always (restated, not re-derived): no staple + RequireFreshStaple
    // fails CLOSED; SignatureOnly is equally the app's own choice.
    assert!(verify_staple(&cred_bytes, &w.coop.id, &head1, &staple1));
    assert!(!verifier_accepts(&cred_bytes, &w.coop.id, None, CheckDial::RequireFreshStaple));
    assert!(verifier_accepts(&cred_bytes, &w.coop.id, None, CheckDial::SignatureOnly));
    assert!(!verifier_accepts(
        &cred_bytes,
        &w.coop.id,
        Some((&head2, &staple2)),
        CheckDial::RequireFreshStaple
    ));
}

// ---------------------------------------------------------------------------
// T-A4.11 — the verifier never contacts the issuer
// ---------------------------------------------------------------------------

/// API-surface negative test in the T-AT5.4 style (successor of T-PA2.3
/// `status_check_no_cross_leak` and the deleted T-PA6.3 status-check
/// protocol): no public operation accepts a verifier query naming a subject —
/// the (verifier, subject) capture leak is structurally gone.
#[test]
fn verifier_never_contacts_issuer() {
    // (a) The status-check machinery is GONE from the source, not merely
    // unused: no OCSP-shaped operation, response type, or standing enum
    // exists anywhere in the issuer module.
    let src = crate_source("src/issuer.rs");
    for (line_no, line) in code_lines(&src) {
        for banned in ["status_check", "StatusResponse", "CredStanding"] {
            assert!(
                !line.contains(banned),
                "src/issuer.rs:{line_no}: status-check machinery survived the V5 rework: {line}"
            );
        }
    }

    // (b) Signature pin over the read/verify surface: the issuer-state read
    // operations take no persona at all, and the pure verifier functions
    // take exactly ONE `PersonaId` — the issuer key — and no subject.
    let signature_of = |name: &str| -> String {
        let needle = format!("pub fn {name}(");
        let start = src.find(&needle).unwrap_or_else(|| panic!("missing pub fn {name}"));
        let end = src[start..].find(')').map(|e| start + e).unwrap_or(src.len());
        src[start..end].to_string()
    };
    for read_op in ["lineage_bytes", "holder_proof", "holder_staple"] {
        let sig = signature_of(read_op);
        assert!(
            !sig.contains("PersonaId") && !sig.contains("subject"),
            "read operation `{read_op}` must name no persona: {sig}"
        );
    }
    for verify_op in ["verify_credential", "verify_tree_head", "verify_staple", "verifier_accepts", "audit_heads"]
    {
        let sig = signature_of(verify_op);
        assert_eq!(
            sig.matches("PersonaId").count(),
            1,
            "verifier function `{verify_op}` takes exactly the issuer key: {sig}"
        );
        assert!(sig.contains("issuer: &PersonaId") && !sig.contains("subject"));
    }

    // (c) Behavioral: verification is pure — the issuer state can be DROPPED
    // before the verifier ever runs. Nothing to contact exists.
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-a4-11", 0, 0)),
    )
    .expect("fixture mint succeeds");
    let head = state.close_epoch(&w.coop);
    let staple = state.holder_staple(&out.bindings[0]).unwrap();
    let cred_bytes = out.credentials[0].canonical_bytes_with_sig();
    drop(state); // the issuer is unreachable from here on
    assert!(verify_staple(&cred_bytes, &w.coop.id, &head, &staple));
}

// ---------------------------------------------------------------------------
// T-A4.12 — the holder verifies its own inclusion (the promised pin)
// ---------------------------------------------------------------------------

#[test]
fn holder_verifies_own_inclusion() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let outs = mint_n(&mut state, &w.coop, "t-a4-12", 5);
    let head = state.close_epoch(&w.coop);
    assert!(verify_tree_head(&head, &w.coop.id));

    // Every holder + its staple + the published head verifies — holder
    // self-verifiability is V5's second value claim.
    for out in &outs {
        let staple = state.holder_staple(&out.bindings[0]).expect("covered by the head");
        let bytes = out.credentials[0].canonical_bytes_with_sig();
        assert!(verify_staple(&bytes, &w.coop.id, &head, &staple));

        // Tampered head (unsigned flip): fails.
        let mut forged = head.clone();
        forged.leaf_count += 1;
        assert!(!verify_tree_head(&forged, &w.coop.id));
        assert!(!verify_staple(&bytes, &w.coop.id, &forged, &staple));

        // Tampered proof: fails.
        let mut bad = staple.clone();
        if let Some((_, h)) = bad.proof.first_mut() {
            h[0] ^= 0xFF;
        }
        assert!(!verify_staple(&bytes, &w.coop.id, &head, &bad));

        // Someone else's credential with this staple: the issuer's binding
        // signature refuses the swap.
        let other_bytes = outs
            .iter()
            .find(|o| o.credentials[0].object_id() != out.credentials[0].object_id())
            .unwrap()
            .credentials[0]
            .canonical_bytes_with_sig();
        assert!(!verify_staple(&other_bytes, &w.coop.id, &head, &staple));
    }
}

// ---------------------------------------------------------------------------
// T-A4.13 — the covenant audit over heads (successor of T-PA6.2's
// commitment-audit leg, replaced old → new)
// ---------------------------------------------------------------------------

#[test]
fn audit_over_heads() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    // Era 1: two epochs — 3 mints, head; 2 more + 1 supersession, head.
    let outs = mint_n(&mut state, &w.coop, "t-a4-13-a", 3);
    state.close_epoch(&w.coop);
    let _more = mint_n(&mut state, &w.coop, "t-a4-13-b", 2);
    state.supersede(&w.coop, &outs[0].credentials[0]);
    state.close_epoch(&w.coop);
    // Era roll (publishes era 1's final head), then era 2: one mint, head.
    let era2 = *blake3::hash(b"t-a4-13:era-2").as_bytes();
    state.roll_era(&w.coop, era2);
    mint_n(&mut state, &w.coop, "t-a4-13-c", 1);
    state.close_epoch(&w.coop);

    // Honest surface audits clean: totals only, per-era leaf counts summed.
    let honest = state.lineage_bytes();
    let report = audit_heads(&honest, &w.coop.id).expect("honest heads audit clean");
    assert_eq!(report.heads_audited, 4, "2 era-1 heads + the roll's closing head + 1 era-2 head");
    assert_eq!(report.total_commitments, 6, "era 1 contributes 5, era 2 contributes 1");

    // The report type is counters only — nothing else exists to leak
    // (the T-PA6.2 shape, carried over).
    let src = crate_source("src/issuer.rs");
    let start = src.find("pub struct AuditReport {").unwrap();
    let end = src[start..].find("\n}").map(|e| start + e).unwrap();
    let fields: Vec<String> = code_lines(&src[start..end])
        .into_iter()
        .filter(|(_, l)| l.trim().starts_with("pub "))
        .map(|(_, l)| l)
        .collect();
    assert_eq!(fields.len(), 3, "AuditReport = the struct line + two counters: {fields:?}");
    // Zero identities: no persona id appears in the audited bytes.
    for p in [&w.p1a, &w.p1b, &w.p2a, &w.p3] {
        assert!(!contains_subslice(&honest, &p.id.0), "persona id in the head lineage");
    }

    // Tampered fixtures are caught. Helpers mirror T-PA6.2's decode/resign.
    let decode = |bytes: &[u8]| -> Vec<BTreeMap<String, Ipld>> {
        let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).unwrap();
        let Ipld::List(l) = v else { panic!("lineage is a list") };
        l.into_iter()
            .map(|r| {
                let Ipld::Map(m) = r else { panic!("head is a map") };
                m
            })
            .collect()
    };
    let reencode = |records: Vec<BTreeMap<String, Ipld>>| -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&Ipld::List(records.into_iter().map(Ipld::Map).collect()))
            .unwrap()
    };
    // Re-sign the signed portion {a,e,n,r,u} with the REAL issuer key, so
    // only the targeted audit check can catch the tamper.
    let resign = |m: &mut BTreeMap<String, Ipld>| {
        let mut unsigned = m.clone();
        unsigned.remove("g");
        unsigned.remove("d");
        let bytes = serde_ipld_dagcbor::to_vec(&Ipld::Map(unsigned)).unwrap();
        m.insert("g".into(), Ipld::Bytes(w.coop.sign_bytes(&bytes)));
    };

    // (1) An unsigned flip — the signature catches it.
    let mut forged = decode(&honest);
    {
        let m = &mut forged[0];
        let Some(Ipld::Integer(n)) = m.get("n") else { panic!("leaf count") };
        let n = *n + 1;
        m.insert("n".into(), Ipld::Integer(n));
    }
    assert_eq!(
        audit_heads(&reencode(forged), &w.coop.id).expect_err("unsigned tamper must fail"),
        AuditFailure::BadSignature { epoch_no: 1 }
    );

    // (2) A dropped superseded entry, freshly signed — the published set no
    // longer matches its signed root.
    let mut dropped = decode(&honest);
    {
        let m = &mut dropped[1];
        let Some(Ipld::List(dl)) = m.get_mut("d") else { panic!("superseded list") };
        assert!(!dl.is_empty(), "fixture epoch 2 has a supersession");
        dl.pop();
        resign(m);
    }
    assert_eq!(
        audit_heads(&reencode(dropped), &w.coop.id).expect_err("hidden supersession must fail"),
        AuditFailure::SupersededRootMismatch { epoch_no: 2 }
    );

    // (3) A gap in the epoch chain.
    let gapped = decode(&honest).into_iter().skip(1).collect::<Vec<_>>();
    assert_eq!(
        audit_heads(&reencode(gapped), &w.coop.id).expect_err("gapped lineage must fail"),
        AuditFailure::NonContiguousEpochs
    );

    // (4) An era that reappears after being left — era references must be
    // contiguous (governance time moves forward): flipping head 2 into era 2
    // makes head 3's genuine era-1 anchor a REUSE.
    let mut era_bounce = decode(&honest);
    {
        let m = &mut era_bounce[1];
        m.insert("a".into(), Ipld::Bytes(era2.to_vec()));
        resign(m);
    }
    assert_eq!(
        audit_heads(&reencode(era_bounce), &w.coop.id).expect_err("era reuse must fail"),
        AuditFailure::EraReused
    );

    // (5) A leaf count regressing within an era, freshly signed.
    let mut regressed = decode(&honest);
    {
        let m = &mut regressed[1];
        m.insert("n".into(), Ipld::Integer(1));
        resign(m);
    }
    assert_eq!(
        audit_heads(&reencode(regressed), &w.coop.id).expect_err("regressed count must fail"),
        AuditFailure::LeafCountRegressed { epoch_no: 2 }
    );
}

// ---------------------------------------------------------------------------
// T-A4.14 — a reissue chains the original vetting; no new antecedent
// ---------------------------------------------------------------------------

/// One test serving two verdicts: the era-reissue chains the ORIGINAL
/// vetting event (era-reissue semantics) and touches neither the seam nor
/// the dial — which is V8's "era-reissues are free" STRUCTURALLY pinned:
/// there is no new vetting antecedent and no mint act for a fee to attach to.
#[test]
fn reissue_chains_vetting_no_new_antecedent() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(1); // dial: ONE anchor per member
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-a4-14", 0, 0)),
    )
    .expect("first anchor mints");
    state.close_epoch(&w.coop);

    // The member is AT the dial cap: any further mint act is refused...
    assert!(mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1b,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 18),
        MintEntropy::from_seed(derived_seed("t-a4-14", 1, 0)),
    )
    .is_err());

    // ...and an era roll happens (a governance fact opens era 2).
    let era2 = *blake3::hash(b"t-a4-14:era-2").as_bytes();
    state.roll_era(&w.coop, era2);

    let old = out.credentials[0].clone();
    let request = w.p1a.emit(vec![old.object_id()], reissue_request(old.object_id(), era2));

    // The reissue succeeds AT THE CAP — no dial check, no seam write, no new
    // vetting: structurally free (V8), holder-signed and unilateral.
    let re = reissue(
        &mut state,
        &w.coop,
        &request,
        &old,
        MintEntropy::from_seed(derived_seed("t-a4-14", 2, 0)),
    )
    .expect("era-reissue is free — the dial does not even run");

    // The fresh credential chains the ORIGINAL vetting event and the
    // holder's request — and nothing else. No new VettingFact exists.
    assert_eq!(
        re.credential.antecedents,
        vec![out.vetting.object_id(), request.object_id()],
        "antecedents = the original vetting + the holder's own request"
    );
    let Payload::Credential(new_c) = &re.credential.payload else { panic!("a credential") };
    assert_eq!(new_c.era, era2, "issued under the new era");
    assert_eq!(new_c.supersedes, Some(old.object_id()));
    let Payload::Credential(old_c) = &old.payload else { panic!("a credential") };
    assert_eq!(new_c.process, old_c.process, "the original process provenance carries over");

    // Refusals: a request signed by anyone but the holder; a wrong era; an
    // unknown credential. Reissue is the HOLDER's unilateral act or nothing.
    let rogue = w.p3.emit(vec![old.object_id()], reissue_request(old.object_id(), era2));
    assert_eq!(
        reissue(&mut state, &w.coop, &rogue, &old, MintEntropy::from_seed(derived_seed("t-a4-14", 3, 0)))
            .expect_err("only the holder's signature stands a request up"),
        ReissueRefusal::NotHolderSigned
    );
    let stale_era = w.p1a.emit(
        vec![old.object_id()],
        reissue_request(old.object_id(), attest_family::issuer::genesis_era()),
    );
    assert_eq!(
        reissue(&mut state, &w.coop, &stale_era, &old, MintEntropy::from_seed(derived_seed("t-a4-14", 4, 0)))
            .expect_err("a request must target the current era"),
        ReissueRefusal::WrongEra
    );

    // Fold view: the reissued credential STANDS on the original vetting
    // (T-PA3.1's rule, satisfied by the chained antecedent), carries its era
    // facts, and the old credential is superseded with intact lineage.
    let state_folded = log_from(&[
        out.vetting.clone(),
        old.clone(),
        request.clone(),
        re.credential.clone(),
    ])
    .fold();
    let new_view = state_folded.credential(&re.credential.object_id()).unwrap();
    assert_eq!(new_view.status, CredentialStatus::Standing);
    assert_eq!(new_view.era, era2);
    assert!(new_view.holder_requested, "the fold states the holder asked — an era fact");
    let old_view = state_folded.credential(&old.object_id()).unwrap();
    assert_eq!(old_view.status, CredentialStatus::Superseded { by: re.credential.object_id() });
    assert_eq!(old_view.lineage, vec![old.object_id(), re.credential.object_id()]);
    assert!(!old_view.holder_requested, "the original mint was not a reissue");
}

// ---------------------------------------------------------------------------
// T-A4.15 — cross-era commitments are unlinkable
// ---------------------------------------------------------------------------

/// Extends T-PA2.1 across eras: old- and new-era commitments for the same
/// holder share no derivable correlator in the full public sweep. Proven two
/// ways: (a) "who reissued" has no public answer — two worlds differing only
/// in WHICH holder reissued have masked-identical issuer surfaces; (b) the
/// old era's and new era's public byte populations are disjoint, and so are
/// one holder's old- and new-era staples.
#[test]
fn cross_era_commitments_unlinkable() {
    let era2 = *blake3::hash(b"t-a4-15:era-2").as_bytes();

    struct EraWorld {
        surface: Vec<u8>,
        old_staple: Staple,
        new_staple: Staple,
        old_head: TreeHead,
        new_head: TreeHead,
    }
    let build = |case: u64, who_reissues: usize| -> EraWorld {
        let coop = PersonaFixture::new(
            "COOP",
            Holder("I"),
            derived_seed("t-a4-15-coop", case, 0),
            true,
        );
        let mut state = IssuerState::new(u32::MAX);
        let holders: Vec<PersonaFixture> = (0..3)
            .map(|i| {
                PersonaFixture::new("H", Holder("HH"), derived_seed("t-a4-15-h", case, i), false)
            })
            .collect();
        let outs: Vec<MintOutput> = holders
            .iter()
            .enumerate()
            .map(|(i, subject)| {
                mint(
                    &mut state,
                    &coop,
                    attest_family::issuer::MemberRef(derived_seed("t-a4-15-m", case, i as u64)),
                    subject,
                    &[PredicateKind::VettedHolder],
                    d(2026, 7, 17),
                    MintEntropy::from_seed(derived_seed("t-a4-15-e", case, i as u64)),
                )
                .expect("fixture mint succeeds")
            })
            .collect();
        let old_head = state.close_epoch(&coop);
        let old_staple = state.holder_staple(&outs[who_reissues].bindings[0]).unwrap();
        state.roll_era(&coop, era2);

        let old = outs[who_reissues].credentials[0].clone();
        let request = holders[who_reissues]
            .emit(vec![old.object_id()], reissue_request(old.object_id(), era2));
        let re = reissue(
            &mut state,
            &coop,
            &request,
            &old,
            MintEntropy::from_seed(derived_seed("t-a4-15-r", case, 9)),
        )
        .expect("reissue succeeds");
        let new_head = state.close_epoch(&coop);
        let new_staple = state.holder_staple(&re.binding).unwrap();
        EraWorld { surface: state.lineage_bytes(), old_staple, new_staple, old_head, new_head }
    };

    for case in 0u64..6 {
        // (a) Who reissued? The issuer's public surface cannot say: worlds
        // where holder 0 vs holder 1 reissued are structurally identical.
        let wx = build(case, 0);
        let wy = build(case, 1);
        let mask = |bytes: &[u8]| -> String {
            let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).unwrap();
            masked_form(&v)
        };
        assert_eq!(
            mask(&wx.surface),
            mask(&wy.surface),
            "case {case}: which holder reissued must have no public answer"
        );

        // (b) Era disjointness: the old-era and new-era heads share no
        // 32-byte value (different keys → different commitments, roots,
        // supersession sets; the anchors themselves differ by construction).
        // The one permitted shared value is the STRUCTURAL constant every
        // supersession-free head carries: the digest of the empty superseded
        // set — content-free, holder-free, era-free.
        let empty_set_digest = *blake3::hash(
            &serde_ipld_dagcbor::to_vec(&Ipld::List(vec![])).unwrap(),
        )
        .as_bytes();
        let head_leaves = |h: &TreeHead| -> BTreeSet<Vec<u8>> {
            byte_leaves_of(&h.to_canonical_bytes())
                .into_iter()
                .filter(|l| l.len() == 32 && l.as_slice() != empty_set_digest.as_slice())
                .collect()
        };
        assert!(
            head_leaves(&wx.old_head).is_disjoint(&head_leaves(&wx.new_head)),
            "case {case}: old-era and new-era public values must be disjoint"
        );

        // And the same holder's old- and new-era staples share nothing: two
        // colluding verifiers (one shown each) cannot join the eras.
        let staple_values = |s: &Staple| -> BTreeSet<Vec<u8>> {
            let mut vals: BTreeSet<Vec<u8>> = s.proof.iter().map(|(_, h)| h.to_vec()).collect();
            vals.insert(s.commitment.to_vec());
            vals.insert(s.binding_sig.clone());
            vals
        };
        assert!(
            staple_values(&wx.old_staple).is_disjoint(&staple_values(&wx.new_staple)),
            "case {case}: one holder's cross-era staples share a derivable value"
        );
    }
}

// ---------------------------------------------------------------------------
// T-A4.16 — no standing computation exists (the guard with teeth)
// ---------------------------------------------------------------------------

/// The fold exposes ERA FACTS only — issued-under (`era`), holder-requested
/// (`holder_requested`) — and object supersede-lineage status. No type,
/// field, or derivable value expresses active / lapsed / current-member /
/// in-good-standing. (RED was staged as a `current_member: bool` on
/// `CredentialView`, serialized; all three prongs failed; staging deleted at
/// green.)
#[test]
fn no_standing_computation() {
    // (a) Compile-boundary prong: the CredentialView field set is EXACTLY the
    // identity + process + object-status + era-fact fields.
    let src = crate_source("src/fold.rs");
    let start = src.find("pub struct CredentialView {").unwrap();
    let end = src[start..].find("\n}").map(|e| start + e).unwrap();
    let fields: Vec<String> = code_lines(&src[start..end])
        .into_iter()
        .filter_map(|(_, l)| {
            let t = l.trim().strip_prefix("pub ")?.to_string();
            Some(t.split(':').next().unwrap_or("").trim().to_string())
        })
        .filter(|f| !f.contains("struct"))
        .collect();
    assert_eq!(
        fields,
        vec![
            "object",
            "issuer",
            "subject",
            "predicate",
            "process",
            "status",
            "era",
            "holder_requested",
            "lineage"
        ],
        "CredentialView carries identity, process, object status, and ERA FACTS — nothing else"
    );

    // (b) Vocabulary prong (the T-AT0.2 pattern extended with the standing
    // vocabulary): no source code line in the crate introduces member-
    // standing language. Exact-token match, so object-lineage words like
    // `Standing`/`Current` (supersede-chain state, T-PA3.1/T-AT6.3) stay
    // legal while member-standing vocabulary is unrepresentable.
    let banned = [
        "active",
        "lapsed",
        "in_good_standing",
        "good_standing",
        "current_member",
        "membership",
        "expiry",
        "expires",
        "valid_until",
    ];
    for file in crate_src_files() {
        let src = crate_source(&file);
        for (line_no, line) in code_lines(&src) {
            let tokens: Vec<String> = line
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .filter(|t| !t.is_empty())
                .map(|t| t.to_lowercase())
                .collect();
            for b in &banned {
                assert!(
                    !tokens.contains(&b.to_string()),
                    "{file}:{line_no}: member-standing vocabulary `{b}` in code line: {line}"
                );
            }
        }
    }

    // (c) Serialization prong: a folded mint + reissue world serializes era
    // facts and nothing standing-shaped — key set pinned, status strings
    // pinned to the object-lineage triple.
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-a4-16", 0, 0)),
    )
    .expect("fixture mint succeeds");
    state.close_epoch(&w.coop);
    let era2 = *blake3::hash(b"t-a4-16:era-2").as_bytes();
    state.roll_era(&w.coop, era2);
    let old = out.credentials[0].clone();
    let request = w.p1a.emit(vec![old.object_id()], reissue_request(old.object_id(), era2));
    let re = reissue(
        &mut state,
        &w.coop,
        &request,
        &old,
        MintEntropy::from_seed(derived_seed("t-a4-16", 1, 0)),
    )
    .expect("reissue succeeds");

    let folded =
        log_from(&[out.vetting.clone(), old.clone(), request, re.credential.clone()]).fold();
    for view in folded.credentials() {
        let Ipld::Map(m) = view.to_ipld() else { panic!("credential view is a map") };
        let keys: Vec<&str> = m.keys().map(|k| k.as_str()).collect();
        assert_eq!(
            keys,
            vec!["a", "e", "i", "l", "m", "p", "q", "s", "u"],
            "the serialized view is ids + era fact + process + predicate + holder-requested + \
             object status — no standing field exists"
        );
        let Some(Ipld::String(u)) = m.get("u") else { panic!("status string") };
        assert!(
            matches!(u.as_str(), "standing" | "pending" | "superseded"),
            "object supersede-lineage status only — never a member-standing value: {u}"
        );
    }
    // The era facts themselves read as facts: the old credential remains a
    // valid fact of era 1 forever (silence carries no penalty — its
    // supersession here is the holder's own requested reissue, not decay).
    let old_view = folded.credential(&old.object_id()).unwrap();
    assert_eq!(old_view.era, attest_family::issuer::genesis_era());
    let new_view = folded.credential(&re.credential.object_id()).unwrap();
    assert_eq!((new_view.era, new_view.holder_requested), (era2, true));
    let _: &CredentialView = new_view;
}

// ---------------------------------------------------------------------------
// The T-AT0.4 wall-clock scan, extended over the reworked issuer module
// ---------------------------------------------------------------------------

/// The head cadence is the seed-rule shape (epoch roll OR N facts): no
/// wall-clock input exists anywhere in the pipeline — enforced as a source
/// scan over EVERY crate module (discovered, not listed), so the new issuer
/// machinery cannot dodge it.
#[test]
fn no_wall_clock_in_issuer_pipeline() {
    let files = crate_src_files();
    assert!(files.iter().any(|f| f.ends_with("issuer.rs")), "scan must see the issuer module");
    for file in &files {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            for banned in
                ["SystemTime", "Instant", "std::time", "chrono", "wall_clock", "now()", "Duration"]
            {
                assert!(
                    !line.contains(banned),
                    "{file}:{line_no}: wall-clock machinery `{banned}` in code line: {line}"
                );
            }
        }
    }
}
