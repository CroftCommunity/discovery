//! RUN-ATTEST-02 EXP-PA5 — dual attachment: accountability per persona, unity
//! private. T-PA5.*.
//!
//! Claim: a persona carries its own record fully; siblings carry none of it.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::{CredentialStatus, ReviewStatus, VouchStatus};
use attest_family::issuer::{mint, IssuerState, MintEntropy, MintOutput};
use attest_family::types::*;
use common::*;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// The shared fixture: all three of H1's siblings hold anchor credentials;
/// P1a additionally has edges to P3 (so vouches/reviews can attach).
struct Fix {
    w: AnchorWorld,
    state: IssuerState,
    outs: Vec<(PersonaId, MintOutput)>,
    /// The base log: everything published before P1a's history accrues.
    base: Vec<Envelope>,
}

fn fix() -> Fix {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let mut outs = Vec::new();
    for (k, (subject, holder)) in
        [(&w.p1a, &w.h1), (&w.p1b, &w.h1), (&w.p1c, &w.h1)].iter().enumerate()
    {
        let out = mint(
            &mut state,
            &w.coop,
            member_ref(holder),
            subject,
            &[PredicateKind::VettedHolder, PredicateKind::PhoneVerified],
            d(2026, 7, 17),
            MintEntropy::from_seed(derived_seed("t-pa5", 0, k as u64)),
        )
        .expect("fixture mint succeeds");
        outs.push((subject.id, out));
    }
    state.close_epoch(&w.coop);

    let mut base: Vec<Envelope> = Vec::new();
    for (_, out) in &outs {
        base.push(out.vetting.clone());
        base.extend(out.credentials.iter().cloned());
    }
    // An edge P1a–P3, so P3 can vouch for / review P1a.
    let core = edge_core(w.p1a.id, w.p3.id, [0x51; 16], vec![]);
    base.push(w.p1a.emit(vec![], edge_half(core.clone(), "met at the co-op")));
    base.push(w.p3.emit(vec![], edge_half(core, "co-op member")));
    Fix { w, state, outs, base }
}

/// P1a's history: a vouch later superseded (narrowed), a review dispute
/// (review + P1a's signed reply), and a superseded credential. Returns the
/// appended envelopes plus the object ids of the record.
struct History {
    vouch1: ObjectId,
    vouch2: ObjectId,
    review: ObjectId,
    reply: ObjectId,
    superseded_credential: ObjectId,
}

fn accrue_history(f: &mut Fix) -> (Vec<Envelope>, History) {
    let w = &f.w;
    let core_hash = {
        let core = edge_core(w.p1a.id, w.p3.id, [0x51; 16], vec![]);
        core.core_hash()
    };
    let halves: Vec<ObjectId> = f
        .base
        .iter()
        .filter(|e| matches!(e.payload, Payload::EdgeHalf(_)))
        .map(|e| e.object_id())
        .collect();

    let v1 = w.p3.emit(
        halves.clone(),
        vouch(w.p1a.id, "would hire as contractor", "great work", core_hash, d(2026, 7, 1), None),
    );
    let v2 = w.p3.emit(
        vec![v1.object_id()],
        vouch(
            w.p1a.id,
            "would hire as contractor",
            "great work, small jobs only",
            core_hash,
            d(2026, 7, 10),
            Some(v1.object_id()),
        ),
    );
    let rv = w.p3.emit(
        vec![],
        review(SubjectRef::Persona(w.p1a.id), "punctuality", "late twice", d(2026, 7, 12), None),
    );
    let rp = w.p1a.emit(
        vec![rv.object_id()],
        reply(rv.object_id(), "the bus line was closed — disputed", d(2026, 7, 13)),
    );

    // The issuer supersedes P1a's phone_verified credential.
    let (_, out) = f.outs.iter().find(|(id, _)| *id == w.p1a.id).unwrap();
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
    let sup = f.state.supersede(&w.coop, &phone);

    let hist = History {
        vouch1: v1.object_id(),
        vouch2: v2.object_id(),
        review: rv.object_id(),
        reply: rp.object_id(),
        superseded_credential: phone.object_id(),
    };
    (vec![v1, v2, rv, rp, sup], hist)
}

// ---------------------------------------------------------------------------
// T-PA5.1 — the record stays with the persona, supersede-never-erase
// ---------------------------------------------------------------------------

#[test]
fn record_stays_with_persona() {
    let mut f = fix();
    let (extra, h) = accrue_history(&mut f);
    let mut log = log_from(&f.base);
    let phone_bytes = log.object_bytes(&h.superseded_credential).unwrap().to_vec();
    for e in &extra {
        log.append(e.clone()).unwrap();
    }
    let state = log.fold();

    // The superseded vouch is still there, superseded, with forward lineage.
    let v1 = state.vouch(&h.vouch1).expect("superseded vouch persists");
    assert_eq!(v1.status, VouchStatus::Superseded { by: h.vouch2 });
    assert_eq!(v1.lineage, vec![h.vouch1, h.vouch2]);
    let v2 = state.vouch(&h.vouch2).expect("narrowing vouch stands");
    assert_eq!(v2.status, VouchStatus::Standing);

    // The dispute: review standing, P1a's signed reply attached as a peer
    // object — both visible to a third viewer's corroboration.
    let rv = state.review(&h.review).expect("review persists");
    assert_eq!(rv.status, ReviewStatus::Standing);
    assert_eq!(rv.replies, vec![h.reply]);
    let resp = state.corroboration(
        &f.w.p4.id,
        &SubjectRef::Persona(f.w.p1a.id),
        &Scope::new("punctuality"),
        &attest_family::query::FreshnessDial { stale_after_days: 3650 },
        d(2026, 7, 18),
    );
    assert_eq!(resp.entries.len(), 1);
    assert_eq!(resp.entries[0].replies, vec![h.reply]);

    // The superseded credential: status moved, bytes unchanged, lineage
    // intact (supersede-never-erase, T-AT0.3 inherited).
    let cred = state.credential(&h.superseded_credential).expect("superseded credential persists");
    assert!(matches!(cred.status, CredentialStatus::Superseded { .. }));
    assert_eq!(log.object_bytes(&h.superseded_credential).unwrap(), &phone_bytes[..]);

    // And the persona's OTHER credential still stands — supersede is
    // per-object, never per-persona.
    let (_, out) = f.outs.iter().find(|(id, _)| *id == f.w.p1a.id).unwrap();
    let vetted = out
        .credentials
        .iter()
        .find(|e| {
            matches!(
                &e.payload,
                Payload::Credential(c) if c.predicate == PredicateKind::VettedHolder
            )
        })
        .unwrap();
    assert_eq!(
        state.credential(&vetted.object_id()).unwrap().status,
        CredentialStatus::Standing
    );
}

// ---------------------------------------------------------------------------
// T-PA5.2 — siblings are unaffected, byte-for-byte
// ---------------------------------------------------------------------------

/// One sibling's full public surface: fold-side sweep + published envelopes +
/// the issuer's read side for its credentials.
fn surface_of(
    state: &attest_family::fold::AttestState,
    sid: &PersonaId,
    f: &Fix,
    viewer: &PersonaId,
    scopes: &[&str],
) -> Vec<u8> {
    let (_, out) = f.outs.iter().find(|(id, _)| id == sid).unwrap();
    let mut published: Vec<Envelope> = vec![out.vetting.clone()];
    published.extend(out.credentials.iter().cloned());
    let mut bytes = persona_surface_bytes(state, sid, viewer, scopes, &published);
    for env in &out.credentials {
        bytes.extend(f.state.status_check(&f.w.coop, env.object_id().0).to_canonical_bytes());
    }
    bytes
}

#[test]
fn siblings_unaffected() {
    let mut f = fix();
    let scopes = ["would hire as contractor", "punctuality"];

    // Sweep P1b's and P1c's full public surfaces BEFORE P1a's history.
    let state_before = log_from(&f.base).fold();
    let viewer = f.w.p4.id;
    let (p1a, p1b, p1c) = (f.w.p1a.id, f.w.p1b.id, f.w.p1c.id);
    let b_before = surface_of(&state_before, &p1b, &f, &viewer, &scopes);
    let c_before = surface_of(&state_before, &p1c, &f, &viewer, &scopes);
    let a_before = surface_of(&state_before, &p1a, &f, &viewer, &scopes);

    // Accrue P1a's full history (including the issuer-side supersede).
    let (extra, _) = accrue_history(&mut f);
    let mut all = f.base.clone();
    all.extend(extra);
    let state_after = log_from(&all).fold();

    let b_after = surface_of(&state_after, &p1b, &f, &viewer, &scopes);
    let c_after = surface_of(&state_after, &p1c, &f, &viewer, &scopes);

    // Byte-level: from every third-party viewer's sweep, the siblings'
    // surfaces are IDENTICAL before and after P1a's history.
    assert_eq!(b_before, b_after, "P1b's public surface must be untouched by P1a's record");
    assert_eq!(c_before, c_after, "P1c's public surface must be untouched by P1a's record");

    // Sanity: P1a's own surface DID change (the record attached somewhere).
    let a_after = surface_of(&state_after, &p1a, &f, &viewer, &scopes);
    assert_ne!(a_before, a_after, "P1a's own surface carries the record");
}

// ---------------------------------------------------------------------------
// T-PA5.3 — an anchor is NOT uniqueness; the holder question is unaskable
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-3 pending — whether `sole_anchor(context)` ever ships, and
// for which contexts, is an owner call. It is defined in vocabulary
// (PRIMITIVES-ATTEST.md) and deliberately NOT built here.
#[test]
fn anchor_is_not_uniqueness() {
    // (a) Vocabulary: PRIMITIVES-ATTEST.md carries the explicit sentences.
    let prim = crate_source("PRIMITIVES-ATTEST.md");
    for needle in [
        "NOT proof of unique personhood",
        "one human may hold several",
        "sole_anchor(context)",
        "NOT built",
        "vetted_holder",
        "reality anchor",
    ] {
        assert!(
            prim.contains(needle),
            "PRIMITIVES-ATTEST.md must state `{needle}` (anchor ≠ uniqueness; \
             sole_anchor is vocabulary only)"
        );
    }

    // (b) Type level: holder bookkeeping and the seam handle are not
    // representable in any serialized public object — the `Holder`,
    // `MemberRef`, and `SeamBoundary` types never appear in the
    // serialization or fold modules (exact-token match, so the
    // `vetted_holder` PREDICATE — which asserts vetting, not holder identity
    // — does not trip it).
    for file in ["src/canonical.rs", "src/query.rs", "src/fold.rs", "src/types.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let tokens: Vec<&str> = line
                .split(|c: char| !c.is_alphanumeric() && c != '_')
                .filter(|t| !t.is_empty())
                .collect();
            for banned in ["Holder", "MemberRef", "SeamBoundary"] {
                assert!(
                    !tokens.contains(&banned),
                    "{file}:{line_no}: holder-linkage type `{banned}` reached the \
                     public-object surface: {line}"
                );
            }
        }
    }
    // And the seam type exposes nothing: no public accessor in its region.
    let issuer_src = crate_source("src/issuer.rs");
    let start = issuer_src.find("pub struct SeamBoundary").expect("seam type exists");
    let end = issuer_src[start..]
        .find("\n// ---")
        .map(|e| start + e)
        .unwrap_or(issuer_src.len());
    for (line_no, line) in code_lines(&issuer_src[start..end]) {
        let t = line.trim();
        assert!(
            !t.starts_with("pub fn "),
            "SeamBoundary region line {line_no} exposes a public operation: {line}"
        );
    }

    // (c) Behavioral: two worlds — same shape, but in world SAME the two
    // personas share a holder and in world SPLIT they don't. Every public
    // byte a viewer can sweep is structurally identical: the question "do X
    // and Y share a holder?" has no public answer surface at all.
    let build = |tag: &'static str, same_holder: bool| -> Vec<String> {
        let issuer = PersonaFixture::new("COOP", Holder("I"), derived_seed(tag, 50, 50), true);
        let mut state = IssuerState::new(u32::MAX);
        let x = PersonaFixture::new("X", Holder("HX"), derived_seed(tag, 0, 0), false);
        let y = PersonaFixture::new(
            "Y",
            if same_holder { Holder("HX") } else { Holder("HY") },
            derived_seed(tag, 0, 1),
            false,
        );
        let mx = member_ref(&Holder("HX"));
        let my = member_ref(if same_holder { &Holder("HX") } else { &Holder("HY") });
        let mut published: Vec<Vec<u8>> = Vec::new();
        for (k, (p, m)) in [(&x, mx), (&y, my)].iter().enumerate() {
            let out = mint(
                &mut state,
                &issuer,
                *m,
                p,
                &[PredicateKind::VettedHolder],
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed(tag, 9, k as u64)),
            )
            .expect("fixture mint succeeds");
            published.push(out.vetting.canonical_bytes_with_sig());
            for e in &out.credentials {
                published.push(e.canonical_bytes_with_sig());
            }
        }
        state.close_epoch(&issuer);
        published.push(state.lineage_bytes());
        let mut forms: Vec<String> = published
            .iter()
            .map(|b| {
                let v: ipld_core::ipld::Ipld =
                    serde_ipld_dagcbor::from_slice(b).expect("public object decodes");
                masked_form(&v)
            })
            .collect();
        forms.sort();
        forms
    };
    assert_eq!(
        build("t-pa5-3-same", true),
        build("t-pa5-3-split", false),
        "shared-holder and split-holder worlds must be structurally indistinguishable"
    );
}
