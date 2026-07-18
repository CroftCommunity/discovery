//! RUN-ATTEST-02 EXP-PA1 — the no-default invariant. T-PA1.*.
//!
//! Claim: no public object, field, or derivable value designates any persona
//! as primary, first, or ranked. An observer's total knowledge is "this
//! persona carries the predicate; that one does not."

mod common;

use attest_family::fixtures::*;
use attest_family::issuer::{mint, IssuerState, MintEntropy, MintOutput};
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

/// Mint one persona's anchor credentials in the given issuer state.
fn mint_anchor(
    state: &mut IssuerState,
    issuer: &PersonaFixture,
    holder: &Holder,
    subject: &PersonaFixture,
    kinds: &[PredicateKind],
    seed: [u8; 32],
) -> MintOutput {
    mint(
        state,
        issuer,
        member_ref(holder),
        subject,
        kinds,
        d(2026, 7, 17),
        MintEntropy::from_seed(seed),
    )
    .expect("fixture mint must succeed")
}

/// The 13-candidate world of T-PA1.2: H1's three siblings plus ten
/// single-anchor strangers, all minted `vetted_holder` in ONE open epoch, in
/// a seeded order. Returns each candidate's published bundle (vetting +
/// credential envelopes), labeled by persona id, plus the index of H1's
/// FIRST-minted persona among the candidates and the closed epoch lineage.
struct Cohort {
    /// (persona id, published bundle bytes, per-envelope ipld forms)
    bundles: Vec<([u8; 32], Vec<Vec<u8>>)>,
    first_minted_of_h1: usize,
    lineage: Vec<u8>,
}

fn cohort(case: u64) -> Cohort {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let strangers: Vec<PersonaFixture> = (0..10)
        .map(|i| {
            PersonaFixture::new(
                "S",
                Holder("HS"),
                derived_seed("t-pa1-2-stranger", case, i),
                false,
            )
        })
        .collect();

    // Mint order: the three siblings and ten strangers interleaved by a
    // seeded shuffle; every candidate gets the same predicate set.
    #[derive(Clone, Copy)]
    enum Who {
        Sib(usize),
        Str(usize),
    }
    let mut order: Vec<Who> = (0..3)
        .map(Who::Sib)
        .chain((0..10).map(Who::Str))
        .collect();
    seeded_shuffle(&mut order, case * 31 + 7);

    let sibs = [&w.p1a, &w.p1b, &w.p1c];
    let mut bundles: Vec<([u8; 32], Vec<Vec<u8>>)> = Vec::new();
    let mut first_minted_sib_id: Option<[u8; 32]> = None;
    for (k, who) in order.iter().enumerate() {
        let (subject, holder) = match who {
            Who::Sib(i) => (sibs[*i], &w.h1),
            Who::Str(i) => (&strangers[*i], &Holder("HS")),
        };
        let out = mint_anchor(
            &mut state,
            &w.coop,
            holder,
            subject,
            &[PredicateKind::VettedHolder],
            derived_seed("t-pa1-2-entropy", case, k as u64),
        );
        let mut bytes: Vec<Vec<u8>> = vec![out.vetting.canonical_bytes_with_sig()];
        bytes.extend(out.credentials.iter().map(|e| e.canonical_bytes_with_sig()));
        if matches!(who, Who::Sib(_)) && first_minted_sib_id.is_none() {
            first_minted_sib_id = Some(subject.id.0);
        }
        bundles.push((subject.id.0, bytes));
    }
    let _ = state.close_epoch(&w.coop);

    // The adversary receives the bundles in CANONICAL order (sorted by
    // persona id), never in mint order — presentation order must not encode
    // ceremony order. "Which one was H1's first mint" is fixture bookkeeping.
    bundles.sort_by(|a, b| a.0.cmp(&b.0));
    let target = first_minted_sib_id.expect("three siblings were minted");
    let first_minted_of_h1 = bundles.iter().position(|(id, _)| *id == target).unwrap();
    Cohort { bundles, first_minted_of_h1, lineage: state.lineage_bytes() }
}

// ---------------------------------------------------------------------------
// T-PA1.1 — no rank is representable (compile-boundary, extends T-AT0.2)
// ---------------------------------------------------------------------------

#[test]
fn no_rank_representable() {
    // The whole source tree (every module, discovered — not a hardcoded
    // list), code lines only. The RUN-ATTEST-01 banned set carries over and
    // the anchor-persona set is added: nothing anywhere may introduce an
    // ordinal, primary, rank, or sequence field — there is no default and no
    // first among a holder's personas.
    let banned = [
        "score", "rating", "rank", "trust", "weight", "reputation", // T-AT0.2
        "ordinal", "primary", "sequence", // T-PA1.1
    ];
    let files = crate_src_files();
    assert!(
        files.iter().any(|f| f.ends_with("issuer.rs")),
        "scan must actually see the RUN-ATTEST-02 modules"
    );
    for file in &files {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let lower = line.to_lowercase();
            for b in &banned {
                assert!(
                    !lower.contains(b),
                    "{file}:{line_no}: banned no-default token `{b}` in code line: {line}"
                );
            }
        }
    }

    // Serialization walk: a minted credential's public bytes contain no
    // numeric leaf outside date claims and the envelope's (version, lamport)
    // — in particular nothing that could carry a mint position.
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let out = mint_anchor(
        &mut state,
        &w.coop,
        &w.h1,
        &w.p1a,
        &[PredicateKind::VettedHolder, PredicateKind::Over18],
        derived_seed("t-pa1-1", 0, 0),
    );
    for env in out.credentials.iter().chain(std::iter::once(&out.vetting)) {
        let decoded: Ipld = serde_ipld_dagcbor::from_slice(&env.canonical_bytes_with_sig())
            .expect("published object must decode");
        let mut numerics = Vec::new();
        ipld_numeric_leaves(&decoded, "", &mut numerics);
        for (path, val) in &numerics {
            let leaf_key = path.rsplit('.').next().unwrap_or(path);
            assert!(
                matches!(leaf_key, "d" | "m" | "y" | "l" | "v"),
                "numeric leaf outside date/lamport/version in published object: {path} = {val}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// T-PA1.2 — sibling indistinguishability (property over the public bytes)
// ---------------------------------------------------------------------------

// ADVERSARY CLASS (stated honestly, reproduced verbatim in the run summary):
// this test models a PUBLIC-DATA-ONLY adversary. It sees every byte a
// third-party viewer can sweep — the candidates' published credentials and
// vetting facts and the issuer's epoch lineage — but has no issuer insider
// state (salts, seam ledger, mint order), no network/traffic metadata, and no
// behavioral/stylometric features. Within that class the test proves (a)
// structural indistinguishability: after masking identifier values, every
// candidate's published bundle is byte-identical; and (b) a battery of
// concrete deterministic selectors over the raw public bytes picks H1's
// first-minted persona no better than chance across seeded worlds. It does
// not — and cannot — quantify over every deterministic function; the
// structural half is what makes the battery representative.

#[test]
fn sibling_indistinguishability() {
    let candidates = 13u64; // 3 siblings + 10 strangers
    let cases = 16u64;

    // The selector battery: deterministic functions from public bytes to a
    // candidate. Chance is 1/13 per case; with 16 seeded cases a selector
    // that beats 4 hits is far outside chance (P[X >= 5 | p = 1/13] < 1%).
    type Selector = fn(&Cohort) -> usize;
    let selectors: Vec<(&str, Selector)> = vec![
        ("min-object-bytes", |c| {
            (0..c.bundles.len()).min_by_key(|i| c.bundles[*i].1.clone()).unwrap()
        }),
        ("max-object-bytes", |c| {
            (0..c.bundles.len()).max_by_key(|i| c.bundles[*i].1.clone()).unwrap()
        }),
        ("min-persona-id", |c| {
            (0..c.bundles.len()).min_by_key(|i| c.bundles[*i].0).unwrap()
        }),
        ("nearest-to-lineage-head", |c| {
            // The first 32 bytes of serialized lineage as an anchor value;
            // pick the candidate whose persona id is closest to it.
            let mut head = [0u8; 32];
            for (k, b) in c.lineage.iter().take(32).enumerate() {
                head[k] = *b;
            }
            (0..c.bundles.len())
                .min_by_key(|i| {
                    c.bundles[*i]
                        .0
                        .iter()
                        .zip(head.iter())
                        .map(|(a, b)| (*a ^ *b) as u64)
                        .sum::<u64>()
                })
                .unwrap()
        }),
    ];

    let mut hits = vec![0u64; selectors.len()];
    for case in 0..cases {
        let c = cohort(case);
        assert_eq!(c.bundles.len(), candidates as usize);
        assert!(c.first_minted_of_h1 < c.bundles.len());

        // (a) Structural indistinguishability: every candidate's bundle has
        // the identical masked form — the shape carries nothing.
        let masked: Vec<String> = c
            .bundles
            .iter()
            .map(|(_, bytes)| {
                let forms: Vec<String> = bytes
                    .iter()
                    .map(|b| {
                        let v: Ipld =
                            serde_ipld_dagcbor::from_slice(b).expect("published object decodes");
                        masked_form(&v)
                    })
                    .collect();
                forms.join("|")
            })
            .collect();
        for m in &masked[1..] {
            assert_eq!(
                m, &masked[0],
                "case {case}: a candidate's published bundle differs structurally"
            );
        }

        // (b) The selector battery.
        for (s, (_, f)) in selectors.iter().enumerate() {
            if f(&c) == c.first_minted_of_h1 {
                hits[s] += 1;
            }
        }
    }
    for (s, (name, _)) in selectors.iter().enumerate() {
        assert!(
            hits[s] <= 4,
            "selector `{name}` picked H1's first-minted persona {} / {cases} times — \
             beats the 1/{candidates} chance bound",
            hits[s]
        );
    }
}

// ---------------------------------------------------------------------------
// T-PA1.3 — issuer lineage carries commitments, not identities
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-1 pending — issuer public-lineage content. This run
// implements the narrowest option (blinded commitments); the alternatives
// (publish nothing / publish full issuance facts) are recorded in §8 of the
// instruction and decided by no test here.
#[test]
fn issuer_lineage_carries_commitments_not_identities() {
    let w = AnchorWorld::new();
    let mut state = IssuerState::new(u32::MAX);
    let mut published: Vec<MintOutput> = Vec::new();
    for (k, (subject, holder)) in
        [(&w.p1a, &w.h1), (&w.p1b, &w.h1), (&w.p2a, &w.h2), (&w.p3, &w.h3)].iter().enumerate()
    {
        published.push(mint_anchor(
            &mut state,
            &w.coop,
            holder,
            subject,
            &[PredicateKind::VettedHolder, PredicateKind::Over18],
            derived_seed("t-pa1-3", 0, k as u64),
        ));
    }
    let record = state.close_epoch(&w.coop);
    let lineage = state.lineage_bytes();

    // No subject persona identifier — raw or derived — appears in lineage.
    for p in [&w.p1a, &w.p1b, &w.p2a, &w.p3] {
        assert!(!contains_subslice(&lineage, &p.id.0), "persona id in issuer lineage");
        assert!(
            !contains_subslice(&lineage, blake3::hash(&p.id.0).as_bytes()),
            "derived persona value in issuer lineage"
        );
    }
    // Nor any credential object id (commitments are salted — an observer who
    // has the persona's published credential still cannot locate it in the
    // lineage).
    for out in &published {
        for env in &out.credentials {
            assert!(
                !contains_subslice(&lineage, &env.object_id().0),
                "unblinded credential id in issuer lineage"
            );
            assert!(
                !contains_subslice(&lineage, blake3::hash(&env.object_id().0).as_bytes()),
                "unsalted credential hash in issuer lineage"
            );
        }
    }
    // The epoch record accounts for every issuance: 4 personas × 2 predicates.
    assert_eq!(record.declared_total, 8);
    assert_eq!(record.commitments.len(), 8);

    // Verification needs ONLY the issuer's signature on the credential bytes
    // — the function takes no registry, no lineage, no state.
    let cred = &published[0].credentials[0];
    let verified =
        attest_family::issuer::verify_credential(&cred.canonical_bytes_with_sig(), &w.coop.id)
            .expect("credential verifies from bytes + issuer key alone");
    assert_eq!(verified.object_id(), cred.object_id());
    // A different issuer key refuses.
    assert!(attest_family::issuer::verify_credential(
        &cred.canonical_bytes_with_sig(),
        &w.p3.id
    )
    .is_err());
}

// ---------------------------------------------------------------------------
// T-PA1.4 — commitment fold is unordered per epoch
// ---------------------------------------------------------------------------

// OWNER-CALL: OC-2 pending — sibling-batching mitigation. This run implements
// unordered per-epoch commitment folds; ceremony-spacing policy (or both) is
// an owner call. NOTE the honest residue: epoch MEMBERSHIP is still public
// quantization — the anonymity set of "minted in epoch E" is everyone minted
// in E (recorded in FINDINGS).
#[test]
fn commitment_fold_is_unordered_per_epoch() {
    let w = AnchorWorld::new();
    let sibs = [&w.p1a, &w.p1b, &w.p1c];
    let strangers: Vec<PersonaFixture> = (0..4)
        .map(|i| PersonaFixture::new("S", Holder("HS"), derived_seed("t-pa1-4-s", 9, i), false))
        .collect();

    // The same mint SET in two different call orders (siblings batched first
    // vs interleaved), same per-subject entropy: the closed epoch's public
    // lineage must be byte-identical — within an epoch there IS no order.
    let run = |order: &[usize]| -> Vec<u8> {
        let mut state = IssuerState::new(u32::MAX);
        for &k in order {
            let (subject, holder): (&PersonaFixture, &Holder) = if k < 3 {
                (sibs[k], &w.h1)
            } else {
                (&strangers[k - 3], &Holder("HS"))
            };
            mint_anchor(
                &mut state,
                &w.coop,
                holder,
                subject,
                &[PredicateKind::VettedHolder],
                derived_seed("t-pa1-4-e", 0, k as u64),
            );
        }
        state.close_epoch(&w.coop);
        state.lineage_bytes()
    };

    let batched = run(&[0, 1, 2, 3, 4, 5, 6]);
    let interleaved = run(&[3, 0, 4, 5, 1, 6, 2]);
    assert_eq!(
        batched, interleaved,
        "epoch lineage must not depend on mint order — adjacency cannot exist"
    );

    // And the serialized commitment order is canonical (byte-ascending), i.e.
    // computed from the commitment values alone, never from ceremony order.
    let mut state = IssuerState::new(u32::MAX);
    for k in 0..7usize {
        let (subject, holder): (&PersonaFixture, &Holder) =
            if k < 3 { (sibs[k], &w.h1) } else { (&strangers[k - 3], &Holder("HS")) };
        mint_anchor(
            &mut state,
            &w.coop,
            holder,
            subject,
            &[PredicateKind::VettedHolder],
            derived_seed("t-pa1-4-e", 0, k as u64),
        );
    }
    let record = state.close_epoch(&w.coop);
    let listed: Vec<[u8; 32]> = record.commitments.iter().copied().collect();
    let mut sorted = listed.clone();
    sorted.sort();
    assert_eq!(listed, sorted, "commitments fold as an unordered (canonically sorted) set");
}
