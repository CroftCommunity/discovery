//! RUN-ATTEST-03 Part C — the `atproto_map` mapping spike (pure, no network).
//! T-A3.7 `lexicon_roundtrip_lossless` and T-A3.8
//! `fields_without_lexicon_home_documented`.

mod common;

use attest_family::atproto_map::*;
use attest_family::fixtures::*;
use attest_family::issuer::{mint, IssuerState, MintEntropy};
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// One payload of every MAPPED kind, with optional fields exercised both
/// present and absent.
fn mapped_fixtures(w: &World) -> Vec<Payload> {
    let core = edge_core(w.p1a.id, w.p2.id, [0xC1; 16], vec![ObjectId([0xC2; 32])]);
    let core_hash = core.core_hash();
    vec![
        edge_half(core, "friend from school"),
        // Edge-based vouch with supersede pointer.
        vouch(w.p2.id, "would hire as contractor", "solid", core_hash, d(2026, 7, 1), Some(ObjectId([0xC3; 32]))),
        // Edge-free vouch (V1), no supersede.
        vouch_edge_free(w.p2.id, "would hire as plumber", "paid twice", d(2026, 7, 2), None),
        // Persona-subject review, no supersede.
        review(SubjectRef::Persona(w.p2.id), "communication", "responsive", d(2026, 7, 3), None),
        // Thing-subject review with supersede.
        review(SubjectRef::Thing(w.biz1), "plumbing", "fixed it", d(2026, 7, 4), Some(ObjectId([0xC4; 32]))),
        reply(ObjectId([0xC5; 32]), "our side of it", d(2026, 7, 5)),
        credential(
            PredicateKind::VettedHolder,
            w.p1a.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 6)),
            [0xC6; 16],
            None,
        ),
        credential(
            PredicateKind::PhoneVerified,
            w.p1b.id,
            coop_process(MethodKind::SmsRoundTrip, d(2026, 7, 7)),
            [0xC7; 16],
            Some(ObjectId([0xC8; 32])),
        ),
    ]
}

// ---------------------------------------------------------------------------
// T-A3.7 — payload ↔ lexicon record shape, lossless per kind
// ---------------------------------------------------------------------------

#[test]
fn lexicon_roundtrip_lossless() {
    let w = World::new();
    for p in mapped_fixtures(&w) {
        let record = to_record(&p)
            .unwrap_or_else(|| panic!("{}: mapped kind must produce a record", p.kind_str()));
        // $type is the draft lexicon id.
        let Ipld::Map(m) = &record else { panic!("record is a map") };
        let Some(Ipld::String(ty)) = m.get("$type") else { panic!("record carries $type") };
        assert!(
            ty.starts_with("ing.croft.attest."),
            "{}: $type must be a draft lexicon id, got {ty}",
            p.kind_str()
        );
        // Lossless: record → payload reproduces the payload exactly.
        let back = from_record(&record)
            .unwrap_or_else(|e| panic!("{}: from_record failed: {e}", p.kind_str()));
        assert_eq!(back, p, "{}: round-trip must be lossless", p.kind_str());
        // And stable: re-mapping yields the identical record shape.
        assert_eq!(to_record(&back).unwrap(), record, "{}: map is deterministic", p.kind_str());

        // The T-AT0.2 invariant at the record layer: no numeric leaf outside
        // date-claim components (mirroring the schema's absence of any score
        // field).
        let mut numerics = Vec::new();
        ipld_numeric_leaves(&record, "", &mut numerics);
        for (path, val) in &numerics {
            let leaf_key = path.rsplit('.').next().unwrap_or(path);
            assert!(
                matches!(leaf_key, "day" | "month" | "year"),
                "{}: numeric leaf outside a date claim: {path} = {val}",
                p.kind_str()
            );
        }
    }

    // The treeHead mapping (V5 — supersedes the commitmentEpoch mapping, the
    // T-A3.7 family extended per RUN-ATTEST-04 B.4): lossless over the
    // signed content + published superseded set; the detached signature is
    // deliberately unmapped (repo commit signature replaces it — a
    // documented no-home surface).
    let aw = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let mut outs = Vec::new();
    for (k, subject) in [&aw.p1a, &aw.p2a, &aw.p3].iter().enumerate() {
        outs.push(
            mint(
                &mut state,
                &aw.coop,
                member_ref(&aw.h1),
                subject,
                &[PredicateKind::VettedHolder],
                d(2026, 7, 17),
                MintEntropy::from_seed(derived_seed("t-a3-map", 0, k as u64)),
            )
            .expect("fixture mint succeeds"),
        );
    }
    // A supersession, so the superseded set round-trips non-trivially.
    state.supersede(&aw.coop, &outs[0].credentials[0]);
    let head = state.close_epoch(&aw.coop);
    let record = head_to_record(&head);
    let back = head_from_record(&record).expect("treeHead record decodes");
    assert_eq!(back.epoch_no, head.epoch_no);
    assert_eq!(back.era_anchor, head.era_anchor);
    assert_eq!(back.leaf_count, head.leaf_count);
    assert_eq!(back.root, head.root);
    assert_eq!(back.superseded_root, head.superseded_root);
    assert_eq!(back.superseded, head.superseded, "the superseded set survives exactly");
    assert!(back.signature.is_empty(), "the detached signature has no lexicon home");
    // Head numerics are epoch number + leaf count only (T-A4.6's rule).
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&record, "", &mut numerics);
    for (path, _) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path.as_str());
        assert!(
            matches!(leaf_key, "epochNo" | "leafCount"),
            "head numeric outside epoch/count: {path}"
        );
    }
}

// ---------------------------------------------------------------------------
// T-A3.8 — the no-lexicon-home list is documented AND mechanically exact
// ---------------------------------------------------------------------------

/// The two-tier boundary made mechanical: every payload kind without a record
/// mapping is named in `fields_without_lexicon_home` with the tier that holds
/// it, and the three V-series boundary surfaces are stated verbatim.
#[test]
fn fields_without_lexicon_home_documented() {
    let w = World::new();
    let list = fields_without_lexicon_home();

    // The instruction-named boundary surfaces, each with its holding tier.
    for (needle, tier_needle) in [
        ("resolvability_policy", "drystone"),
        ("ceremony_fact", "drystone/private"),
        ("issuer retained state", "issuer seam"),
    ] {
        let entry = list
            .iter()
            .find(|e| e.surface.contains(needle))
            .unwrap_or_else(|| panic!("no-home list must name `{needle}`"));
        assert!(
            entry.tier.contains(tier_needle),
            "`{needle}` must be held by `{tier_needle}`, got `{}`",
            entry.tier
        );
        assert!(!entry.why.is_empty());
    }

    // Mechanical exactness: one payload of EVERY kind; to_record is Some iff
    // the kind is not named in the no-home list. (The list also carries
    // non-payload surfaces — envelope fields, the epoch signature — which
    // have no Payload variant to check.)
    let session = [0x5E; 16];
    let core = edge_core(w.p1a.id, w.p2.id, [0xC9; 16], vec![]);
    let all_kinds: Vec<Payload> = vec![
        edge_half(core.clone(), "x"),
        edge_dissolve(core.core_hash(), vec![ObjectId([0xCA; 32])]),
        ceremony_fact(session, w.p1a.id, w.p2.id, d(2026, 7, 1)),
        transaction_fact(w.p1a.id, SubjectRef::Persona(w.p2.id), d(2026, 7, 2)),
        thing_decl(w.biz1, ThingKind::Business, w.p3.id),
        vouch(w.p2.id, "s", "t", core.core_hash(), d(2026, 7, 3), None),
        vouch_withdraw(ObjectId([0xCB; 32])),
        review(SubjectRef::Thing(w.biz1), "s", "t", d(2026, 7, 4), None),
        reply(ObjectId([0xCC; 32]), "t", d(2026, 7, 5)),
        predicate(PredicateKind::Over18, w.p2.id, coop_process(MethodKind::DocumentSighted, d(2026, 7, 6)), None),
        policy(w.p2.id, PolicyRule::AllowAll, None),
        vetting_fact(w.p1a.id, [0xCD; 16], d(2026, 7, 7)),
        credential(PredicateKind::Over18, w.p1a.id, coop_process(MethodKind::DocumentSighted, d(2026, 7, 8)), [0xCE; 16], None),
        credential_supersede(ObjectId([0xCF; 32])),
        reissue_request(ObjectId([0xD1; 32]), [0xD2; 32]),
    ];
    assert_eq!(all_kinds.len(), 15, "one payload of every kind");

    let mapped: &[&str] = &["edge_half", "vouch", "review", "reply", "credential"];
    for p in &all_kinds {
        let kind = p.kind_str();
        let has_record = to_record(p).is_some();
        assert_eq!(
            has_record,
            mapped.contains(&kind),
            "{kind}: record mapping must match the B.2 six-lexicon scope"
        );
        if !has_record {
            assert!(
                list.iter().any(|e| e.surface.contains(kind)),
                "{kind}: unmapped kind must be NAMED in fields_without_lexicon_home"
            );
        }
    }

    // Every listed entry names a tier — no unhoused surface exists.
    for e in list {
        assert!(!e.tier.is_empty(), "`{}` must name its holding tier", e.surface);
    }
}
