//! RUN-ATTEST-02 EXP-PA3 — fee as sybil friction (modeled): T-PA3.1, T-PA3.3.
//! (T-PA3.2, the governed dial on the reused R7 machinery, lives in
//! `t_pa_substrate.rs` alongside the covenant-lineage half of T-PA6.2 — both
//! need the substrate dev-deps.)
//!
//! No payment rail is built; what IS built is the structure that makes the
//! fee meaningful: no standing credential without a vetting-event antecedent,
//! and no public object revealing any member's anchor count.

mod common;

use attest_family::fixtures::*;
use attest_family::fold::CredentialStatus;
use attest_family::issuer::{mint, IssuerState, MintEntropy};
use attest_family::types::*;
use common::*;
use ipld_core::ipld::Ipld;

fn d(y: u16, m: u8, day: u8) -> DateClaim {
    DateClaim::new(y, m, day)
}

// ---------------------------------------------------------------------------
// T-PA3.1 — no credential without a vetting antecedent
// ---------------------------------------------------------------------------

#[test]
fn no_credential_without_vetting_antecedent() {
    let w = AnchorWorld::new();

    // The mint path always produces the antecedent: fold its output and the
    // credential stands.
    let mut state = IssuerState::new(u32::MAX);
    let out = mint(
        &mut state,
        &w.coop,
        member_ref(&w.h1),
        &w.p1a,
        &[PredicateKind::VettedHolder],
        d(2026, 7, 17),
        MintEntropy::from_seed(derived_seed("t-pa3-1", 0, 0)),
    )
    .expect("fixture mint succeeds");
    let mut envs = vec![out.vetting.clone()];
    envs.extend(out.credentials.iter().cloned());
    let folded = log_from(&envs).fold();
    let cred_id = out.credentials[0].object_id();
    assert_eq!(
        folded.credential(&cred_id).expect("credential folds").status,
        CredentialStatus::Standing
    );

    // (a) The same credential WITHOUT its vetting fact in the log: pending,
    // never standing — and pending is never partial (absent from no view,
    // demoted in none).
    let folded_bare = log_from(&out.credentials).fold();
    assert_eq!(
        folded_bare.credential(&cred_id).expect("still folds, as pending").status,
        CredentialStatus::Pending
    );

    // (b) A hand-built credential with NO antecedents at all: pending.
    let rogue_nonce = [0x77u8; 16];
    let bare = w.coop.sign(
        1,
        vec![],
        credential(
            PredicateKind::Over18,
            w.p1a.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 17)),
            rogue_nonce,
            None,
        ),
    );
    let s = log_from(std::slice::from_ref(&bare)).fold();
    assert_eq!(s.credential(&bare.object_id()).unwrap().status, CredentialStatus::Pending);

    // (c) A vetting fact naming a DIFFERENT subject does not stand a
    // credential up.
    let vet_other = w.coop.sign(1, vec![], vetting_fact(w.p1b.id, [0x78; 16], d(2026, 7, 17)));
    let cred_mismatch = w.coop.sign(
        1,
        vec![vet_other.object_id()],
        credential(
            PredicateKind::Over18,
            w.p1a.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 17)),
            [0x79; 16],
            None,
        ),
    );
    let s = log_from(&[vet_other, cred_mismatch.clone()]).fold();
    assert_eq!(
        s.credential(&cred_mismatch.object_id()).unwrap().status,
        CredentialStatus::Pending,
        "a vetting fact for another persona is not this persona's antecedent"
    );

    // (d) A vetting fact authored by someone OTHER than the credential's
    // issuer does not count (the antecedent must be the issuer's own act).
    let vet_rogue = w.p3.sign(1, vec![], vetting_fact(w.p1a.id, [0x7A; 16], d(2026, 7, 17)));
    let cred_rogue_vet = w.coop.sign(
        2,
        vec![vet_rogue.object_id()],
        credential(
            PredicateKind::Over18,
            w.p1a.id,
            coop_process(MethodKind::DocumentSighted, d(2026, 7, 17)),
            [0x7B; 16],
            None,
        ),
    );
    let s = log_from(&[vet_rogue, cred_rogue_vet.clone()]).fold();
    assert_eq!(
        s.credential(&cred_rogue_vet.object_id()).unwrap().status,
        CredentialStatus::Pending,
        "someone else's vetting fact is not an antecedent"
    );

    // (e) Negative API surface (the T-AT5.4 pattern, extended to the
    // RUN-ATTEST-02 modules): pin every public operation of issuer.rs and
    // anonymity.rs to an exact allowlist — there is no operation that could
    // stand a credential up while bypassing the antecedent, and any new
    // operation fails this test until reviewed.
    let allowlist: std::collections::BTreeSet<&str> = [
        // issuer state + protocol
        "new", "set_dial", "dial", "close_epoch", "lineage_bytes", "status_check",
        "supersede", "mint", "from_seed", "as_str",
        // verifier side
        "verify_credential", "verify_status_response", "verifier_accepts", "audit_lineage",
        // serializations
        "to_ipld", "to_canonical_bytes",
        // measurement harness
        "pool", "pool_size", "tabulate", "present",
    ]
    .into_iter()
    .collect();
    for file in ["src/issuer.rs", "src/anonymity.rs"] {
        let src = crate_source(file);
        for (line_no, line) in code_lines(&src) {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("pub fn ") {
                let name: String =
                    rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
                assert!(
                    allowlist.contains(name.as_str()),
                    "{file}:{line_no}: unreviewed public operation `{name}` — check it against \
                     the antecedent + no-suppression invariants"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// T-PA3.3 — no public object reveals any member's anchor count
// ---------------------------------------------------------------------------

#[test]
fn member_anchor_count_not_public() {
    // Two worlds with the SAME issuance totals but DIFFERENT anchor
    // distributions across members: A = [3, 2, 1, 1, 1], B = [2, 2, 2, 1, 1]
    // (8 personas each, same predicates). If any public byte partitioned
    // commitments per member — or revealed "some member holds 3" — the two
    // worlds' public surfaces would differ structurally. They must not: after
    // masking identifier values, the surfaces are identical.
    let build = |tag: &'static str, dist: &[usize]| -> (Vec<u8>, Vec<Vec<u8>>) {
        let mut state = IssuerState::new(u32::MAX);
        let issuer = PersonaFixture::new("COOP", Holder("I"), derived_seed(tag, 99, 99), true);
        let mut published: Vec<Vec<u8>> = Vec::new();
        let mut k = 0u64;
        for (i, &n) in dist.iter().enumerate() {
            let member = attest_family::issuer::MemberRef(derived_seed(tag, i as u64, u64::MAX));
            for j in 0..n {
                let subject = PersonaFixture::new(
                    "gen",
                    Holder("GEN"),
                    derived_seed(tag, i as u64, j as u64),
                    false,
                );
                let out = mint(
                    &mut state,
                    &issuer,
                    member,
                    &subject,
                    &[PredicateKind::VettedHolder],
                    d(2026, 7, 17),
                    MintEntropy::from_seed(derived_seed(tag, 1000 + k, 0)),
                )
                .expect("fixture mint succeeds");
                k += 1;
                for env in std::iter::once(&out.vetting).chain(out.credentials.iter()) {
                    published.push(env.canonical_bytes_with_sig());
                }
            }
        }
        state.close_epoch(&issuer);
        published.sort();
        (state.lineage_bytes(), published)
    };

    let (lineage_a, published_a) = build("t-pa3-3-a", &[3, 2, 1, 1, 1]);
    let (lineage_b, published_b) = build("t-pa3-3-b", &[2, 2, 2, 1, 1]);

    let mask = |bytes: &[u8]| -> String {
        let v: Ipld = serde_ipld_dagcbor::from_slice(bytes).expect("lineage decodes");
        masked_form(&v)
    };
    assert_eq!(
        mask(&lineage_a),
        mask(&lineage_b),
        "issuer lineage must not carry the anchor distribution across members"
    );

    let mask_all = |published: &[Vec<u8>]| -> Vec<String> {
        let mut forms: Vec<String> = published.iter().map(|b| mask(b)).collect();
        forms.sort();
        forms
    };
    assert_eq!(
        mask_all(&published_a),
        mask_all(&published_b),
        "the published-object population must not carry the anchor distribution"
    );

    // And commitment counts are total-only: the lineage's only numeric leaves
    // are epoch numbers and declared totals — nothing per-member exists.
    let v: Ipld = serde_ipld_dagcbor::from_slice(&lineage_a).unwrap();
    let mut numerics = Vec::new();
    ipld_numeric_leaves(&v, "", &mut numerics);
    for (path, _) in &numerics {
        let leaf_key = path.rsplit('.').next().unwrap_or(path.as_str());
        assert!(
            matches!(leaf_key, "e" | "t"),
            "lineage numeric outside epoch/total: {path}"
        );
    }
    assert_eq!(
        numerics.iter().filter(|(p, _)| p.as_str() == "t").count(),
        1,
        "one total per epoch — never partitioned"
    );
}
