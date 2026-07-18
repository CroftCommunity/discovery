//! Fixtures — built BEFORE features (standing directive; RUN-ATTEST-01 §3).
//!
//! Holders H1, H2, H3; personas P1a/P1b (both H1 — the linkage exists ONLY in
//! this module's bookkeeping, never in any payload), P2 (H2), P3 (H3). Issuer
//! persona COOP with process-provenance builders. Thing subjects BIZ1 (a
//! plumber) and WORK1 (a product). Canonical payload builders for every
//! attestation kind, plus the permutation harness for order-independence tests
//! (no shared harness crate exists in the workspace; this mirrors the
//! `croft-chat/tests/convergence_property.rs` pattern as a reusable module).
//!
//! Personas are deterministic-seed Ed25519 keypairs — the declared stand-in
//! for DID/atproto identity (an explicit non-goal, §9).

use std::cell::Cell;

use ed25519_dalek::{Signer as _, SigningKey};

use crate::types::*;

/// A holder — FIXTURE BOOKKEEPING ONLY. `Holder` appears in no payload type
/// and has no serializable identifier; it exists so tests can state "P1a and
/// P1b belong to the same holder" without that fact ever entering the protocol
/// surface (T-AT4.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Holder(pub &'static str);

/// A fixture persona: a named deterministic keypair plus its holder
/// bookkeeping and a per-persona lamport allocator.
pub struct PersonaFixture {
    pub name: &'static str,
    pub holder: Holder,
    sk: SigningKey,
    pub id: PersonaId,
    lamport: Cell<u64>,
    /// The co-op issuer role marker (§3 stand-in): fixture-level, mirrored in
    /// signed predicate payloads as `IssuerRole::CoopIssuer`.
    pub issuer: bool,
}

impl PersonaFixture {
    pub fn new(name: &'static str, holder: Holder, seed: [u8; 32], issuer: bool) -> Self {
        let sk = SigningKey::from_bytes(&seed);
        let id = PersonaId(sk.verifying_key().to_bytes());
        PersonaFixture { name, holder, sk, id, lamport: Cell::new(0), issuer }
    }

    /// Sign arbitrary canonical bytes (RUN-ATTEST-02: epoch records and
    /// status responses are issuer-signed objects outside the envelope shape).
    pub fn sign_bytes(&self, bytes: &[u8]) -> Vec<u8> {
        self.sk.sign(bytes).to_bytes().to_vec()
    }

    /// Next per-author lamport value (logical clock; the only time-like
    /// ordering input, and it is not wall-clock).
    pub fn next_lamport(&self) -> u64 {
        let v = self.lamport.get() + 1;
        self.lamport.set(v);
        v
    }

    /// Build and sign an envelope authored by this persona.
    pub fn sign(&self, lamport: u64, antecedents: Vec<ObjectId>, payload: Payload) -> Envelope {
        let mut env = Envelope {
            version: 1,
            author: self.id,
            lamport,
            antecedents,
            payload,
            signature: vec![],
        };
        env.signature = self.sk.sign(&env.canonical_bytes()).to_bytes().to_vec();
        env
    }

    /// Build and sign with an auto-allocated lamport.
    pub fn emit(&self, antecedents: Vec<ObjectId>, payload: Payload) -> Envelope {
        self.sign(self.next_lamport(), antecedents, payload)
    }
}

/// The standard fixture world: three holders, five personas, two things.
pub struct World {
    pub h1: Holder,
    pub h2: Holder,
    pub h3: Holder,
    pub p1a: PersonaFixture,
    pub p1b: PersonaFixture,
    pub p2: PersonaFixture,
    pub p3: PersonaFixture,
    pub coop: PersonaFixture,
    pub biz1: ThingId,
    pub work1: ThingId,
}

impl World {
    pub fn new() -> Self {
        let h1 = Holder("H1");
        let h2 = Holder("H2");
        let h3 = Holder("H3");
        World {
            h1,
            h2,
            h3,
            // P1a and P1b: independent seeds, same holder — the linkage lives
            // here and only here.
            p1a: PersonaFixture::new("P1a", h1, [0x11; 32], false),
            p1b: PersonaFixture::new("P1b", h1, [0x12; 32], false),
            p2: PersonaFixture::new("P2", h2, [0x21; 32], false),
            p3: PersonaFixture::new("P3", h3, [0x31; 32], false),
            coop: PersonaFixture::new("COOP", h3, [0xC0; 32], true),
            biz1: ThingId([0xB1; 32]),
            work1: ThingId([0xE1; 32]),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// RUN-ATTEST-02 fixtures (§3) — anchor-persona world + generated co-ops
// ---------------------------------------------------------------------------

/// The anchor-persona fixture world (RUN-ATTEST-02 §3): holders H1..H5. H1
/// holds anchor personas P1a/P1b/P1c (the "3 legit anchors" case), H2 holds
/// P2a/P2b, H3–H5 hold one each. As always, the holder↔persona linkage lives
/// ONLY here — never in any payload or issuer public object.
pub struct AnchorWorld {
    pub h1: Holder,
    pub h2: Holder,
    pub h3: Holder,
    pub h4: Holder,
    pub h5: Holder,
    pub p1a: PersonaFixture,
    pub p1b: PersonaFixture,
    pub p1c: PersonaFixture,
    pub p2a: PersonaFixture,
    pub p2b: PersonaFixture,
    pub p3: PersonaFixture,
    pub p4: PersonaFixture,
    pub p5: PersonaFixture,
    pub coop: PersonaFixture,
}

impl AnchorWorld {
    pub fn new() -> Self {
        let h1 = Holder("H1");
        let h2 = Holder("H2");
        let h3 = Holder("H3");
        let h4 = Holder("H4");
        let h5 = Holder("H5");
        AnchorWorld {
            h1,
            h2,
            h3,
            h4,
            h5,
            p1a: PersonaFixture::new("P1a", h1, [0xA1; 32], false),
            p1b: PersonaFixture::new("P1b", h1, [0xA2; 32], false),
            p1c: PersonaFixture::new("P1c", h1, [0xA3; 32], false),
            p2a: PersonaFixture::new("P2a", h2, [0xB1; 32], false),
            p2b: PersonaFixture::new("P2b", h2, [0xB2; 32], false),
            p3: PersonaFixture::new("P3", h3, [0xC3; 32], false),
            p4: PersonaFixture::new("P4", h4, [0xC4; 32], false),
            p5: PersonaFixture::new("P5", h5, [0xC5; 32], false),
            coop: PersonaFixture::new("COOP", h5, [0xD0; 32], true),
        }
    }
}

impl Default for AnchorWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Fixture-side derivation of an opaque seam handle from holder bookkeeping.
/// The derivation lives HERE (fixtures) so the issuer state itself never sees
/// a holder name — only the opaque handle.
pub fn member_ref(h: &Holder) -> crate::issuer::MemberRef {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"member:");
    hasher.update(h.0.as_bytes());
    crate::issuer::MemberRef(*hasher.finalize().as_bytes())
}

/// Deterministic 32-byte fixture seed (no wall-clock entropy anywhere).
pub fn derived_seed(tag: &str, i: u64, j: u64) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(tag.as_bytes());
    hasher.update(&i.to_be_bytes());
    hasher.update(&j.to_be_bytes());
    *hasher.finalize().as_bytes()
}

/// One generated co-op member: an opaque seam handle plus its personas.
pub struct CoopMember {
    pub member: crate::issuer::MemberRef,
    pub personas: Vec<PersonaFixture>,
}

/// A generated issuer population for EXP-PA4 (same code paths as the anchor
/// world; only the population differs).
pub struct CoopFixture {
    pub tag: &'static str,
    pub issuer: PersonaFixture,
    pub members: Vec<CoopMember>,
}

fn generated_coop(tag: &'static str, member_counts: &[usize]) -> CoopFixture {
    let issuer = PersonaFixture::new("COOP", Holder("ISSUER"), derived_seed(tag, u64::MAX, 0), true);
    let members = member_counts
        .iter()
        .enumerate()
        .map(|(i, &n)| {
            let member = crate::issuer::MemberRef(derived_seed(tag, i as u64, u64::MAX));
            let personas = (0..n)
                .map(|j| {
                    PersonaFixture::new("gen", Holder("GEN"), derived_seed(tag, i as u64, j as u64), false)
                })
                .collect();
            CoopMember { member, personas }
        })
        .collect();
    CoopFixture { tag, issuer, members }
}

/// COOP-S (RUN-ATTEST-02 §3): 12 member-holders — one 3-anchor member, one
/// 2-anchor member, ten single-anchor members (15 personas).
pub fn coop_s() -> CoopFixture {
    let mut counts = vec![3, 2];
    counts.extend(std::iter::repeat_n(1, 10));
    generated_coop("coop-s", &counts)
}

/// COOP-L (RUN-ATTEST-02 §3): 400 member-holders — one 3-anchor member,
/// thirty-nine 2-anchor members, 360 single-anchor members (441 personas).
pub fn coop_l() -> CoopFixture {
    let mut counts = vec![3];
    counts.extend(std::iter::repeat_n(2, 39));
    counts.extend(std::iter::repeat_n(1, 360));
    generated_coop("coop-l", &counts)
}

/// Deterministic predicate assignment for generated personas (member i,
/// persona j). Every anchor persona carries `vetted_holder`; the rest follow
/// fixed congruences so both co-ops use the same rule.
pub fn generated_kinds(i: usize, j: usize) -> Vec<PredicateKind> {
    let mut kinds = vec![PredicateKind::VettedHolder];
    if !(i + j).is_multiple_of(20) {
        kinds.push(PredicateKind::Over18);
    }
    if (i * 7 + j) % 5 < 3 {
        kinds.push(PredicateKind::PhoneVerified);
    }
    if (i * 13 + j) % 20 < 7 {
        kinds.push(PredicateKind::PaymentVerified);
    }
    kinds
}

// ---------------------------------------------------------------------------
// Canonical payload builders (one per attestation kind)
// ---------------------------------------------------------------------------

/// Order two personas canonically (byte-ascending) for an [`EdgeCore`].
pub fn ordered_pair(x: PersonaId, y: PersonaId) -> (PersonaId, PersonaId) {
    if x.0 <= y.0 {
        (x, y)
    } else {
        (y, x)
    }
}

/// The shared core both halves must reference (T-AT1.2).
pub fn edge_core(
    x: PersonaId,
    y: PersonaId,
    edge_nonce: [u8; 16],
    ceremony: Vec<ObjectId>,
) -> EdgeCore {
    let (persona_a, persona_b) = ordered_pair(x, y);
    let mut ceremony = ceremony;
    ceremony.sort();
    EdgeCore { persona_a, persona_b, edge_nonce, consent: ConsentMode::Mutual, ceremony }
}

pub fn edge_half(core: EdgeCore, label: &str) -> Payload {
    Payload::EdgeHalf(EdgeHalf { core, label: label.to_string() })
}

pub fn edge_dissolve(core_hash: [u8; 32], supersedes: Vec<ObjectId>) -> Payload {
    Payload::EdgeDissolve(EdgeDissolve { core_hash, supersedes })
}

pub fn ceremony_fact(
    session: [u8; 16],
    x: PersonaId,
    y: PersonaId,
    sighted_on: DateClaim,
) -> Payload {
    let (a, b) = ordered_pair(x, y);
    Payload::CeremonyFact(CeremonyFact { session, participants: [a, b], sighted_on })
}

pub fn transaction_fact(payer: PersonaId, payee: SubjectRef, occurred_on: DateClaim) -> Payload {
    Payload::TransactionFact(TransactionFact { payer, payee, occurred_on })
}

pub fn thing_decl(thing: ThingId, kind: ThingKind, controller: PersonaId) -> Payload {
    Payload::ThingDecl(ThingDecl { thing, kind, controller })
}

pub fn vouch(
    subject: PersonaId,
    scope: &str,
    statement: &str,
    base_edge: [u8; 32],
    made_on: DateClaim,
    supersedes: Option<ObjectId>,
) -> Payload {
    Payload::Vouch(Vouch {
        subject,
        scope: Scope::new(scope),
        statement: statement.to_string(),
        base_edge,
        made_on,
        supersedes,
    })
}

pub fn vouch_withdraw(supersedes: ObjectId) -> Payload {
    Payload::VouchWithdraw(VouchWithdraw { supersedes })
}

pub fn review(
    subject: SubjectRef,
    scope: &str,
    statement: &str,
    made_on: DateClaim,
    supersedes: Option<ObjectId>,
) -> Payload {
    Payload::Review(Review {
        subject,
        scope: Scope::new(scope),
        statement: statement.to_string(),
        consent: ConsentMode::UnilateralNotice,
        made_on,
        supersedes,
    })
}

pub fn reply(review: ObjectId, statement: &str, made_on: DateClaim) -> Payload {
    Payload::Reply(Reply { review, statement: statement.to_string(), made_on })
}

/// Process-provenance builder for the COOP issuer (§3).
pub fn coop_process(method: MethodKind, performed_on: DateClaim) -> ProcessProvenance {
    ProcessProvenance { method, performed_on, role: IssuerRole::CoopIssuer }
}

pub fn predicate(
    kind: PredicateKind,
    subject: PersonaId,
    process: ProcessProvenance,
    supersedes: Option<ObjectId>,
) -> Payload {
    Payload::Predicate(Predicate { predicate: kind, subject, process, supersedes })
}

pub fn policy(persona: PersonaId, rule: PolicyRule, supersedes: Option<ObjectId>) -> Payload {
    Payload::ResolvabilityPolicy(ResolvabilityPolicy { persona, rule, supersedes })
}

/// RUN-ATTEST-02 payload builders.
pub fn vetting_fact(subject: PersonaId, vetting_nonce: [u8; 16], performed_on: DateClaim) -> Payload {
    Payload::VettingFact(VettingFact {
        subject,
        vetting_nonce,
        performed_on,
        role: IssuerRole::CoopIssuer,
    })
}

pub fn credential(
    kind: PredicateKind,
    subject: PersonaId,
    process: ProcessProvenance,
    mint_nonce: [u8; 16],
    supersedes: Option<ObjectId>,
) -> Payload {
    Payload::Credential(Credential { predicate: kind, subject, process, mint_nonce, supersedes })
}

pub fn credential_supersede(supersedes: ObjectId) -> Payload {
    Payload::CredentialSupersede(CredentialSupersede { supersedes })
}

// ---------------------------------------------------------------------------
// Permutation harness (order-independence tests)
// ---------------------------------------------------------------------------

/// All permutations of `items` (Heap's algorithm) — exhaustive for small sets
/// (T-AT1.6 permutes 3–4 arrivals; 24 orders is cheap).
pub fn permutations<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    let mut out = Vec::new();
    let mut work: Vec<T> = items.to_vec();
    let n = work.len();
    heap_recurse(n, &mut work, &mut out);
    out
}

fn heap_recurse<T: Clone>(k: usize, work: &mut Vec<T>, out: &mut Vec<Vec<T>>) {
    if k <= 1 {
        out.push(work.clone());
        return;
    }
    for i in 0..k {
        heap_recurse(k - 1, work, out);
        if k.is_multiple_of(2) {
            work.swap(i, k - 1);
        } else {
            work.swap(0, k - 1);
        }
    }
}

/// Deterministic seeded shuffle (xorshift64*) for larger corpora — no `rand`
/// dep, no wall-clock entropy (the permutation harness must itself obey the
/// no-wall-clock rule).
pub fn seeded_shuffle<T>(items: &mut [T], seed: u64) {
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = move || {
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        state = state.wrapping_mul(0x2545_F491_4F6C_DD1D);
        state
    };
    for i in (1..items.len()).rev() {
        let j = (next() % (i as u64 + 1)) as usize;
        items.swap(i, j);
    }
}
