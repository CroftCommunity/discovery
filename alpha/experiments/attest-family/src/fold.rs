//! The attestation log and its fold.
//!
//! `AttestLog` is an append-only set of signed attestation objects; `fold()`
//! computes the derived state as a **set-fold in cryptographic order**
//! (lamport, author bytes, object id — `types::fold_cmp`). Arrival order
//! cannot influence the fold because the fold never sees arrival order: it
//! folds the object SET (T-AT0.4, T-AT1.6).
//!
//! Supersede, never revoke: no operation here mutates or deletes a prior
//! object; `object_bytes` returns stored bytes unchanged forever (T-AT0.3).

use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{Signature, Verifier as _, VerifyingKey};

use crate::types::*;

// ---------------------------------------------------------------------------
// Log
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppendError {
    /// The envelope's signature does not verify against its author key.
    BadSignature,
    /// The author key bytes are not a valid Ed25519 verifying key.
    BadAuthorKey,
}

impl std::fmt::Display for AppendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppendError::BadSignature => write!(f, "signature does not verify"),
            AppendError::BadAuthorKey => write!(f, "author is not a valid verifying key"),
        }
    }
}
impl std::error::Error for AppendError {}

/// Append-only store of attestation objects, keyed by content address.
/// Duplicate appends are idempotent (same bytes, same id).
#[derive(Default)]
pub struct AttestLog {
    objects: BTreeMap<ObjectId, (Envelope, Vec<u8>)>,
}

impl AttestLog {
    pub fn new() -> Self {
        Self::default()
    }

    /// Verify the signature and store the object. Structural standing (does a
    /// vouch's base edge resolve? is a reply's author the subject?) is NOT
    /// judged here — the fold states those facts; append only refuses what is
    /// cryptographically not an attestation at all.
    pub fn append(&mut self, env: Envelope) -> Result<ObjectId, AppendError> {
        verify_signature(&env)?;
        let id = env.object_id();
        let bytes = env.canonical_bytes_with_sig();
        self.objects.entry(id).or_insert((env, bytes));
        Ok(id)
    }

    /// The stored canonical bytes of an object, unchanged since append
    /// (T-AT0.3's retrievability half).
    pub fn object_bytes(&self, id: &ObjectId) -> Option<&[u8]> {
        self.objects.get(id).map(|(_, b)| b.as_slice())
    }

    pub fn envelope(&self, id: &ObjectId) -> Option<&Envelope> {
        self.objects.get(id).map(|(e, _)| e)
    }

    pub fn len(&self) -> usize {
        self.objects.len()
    }

    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Fold the object set to derived state. Deterministic: same set, same
    /// state, regardless of append order.
    pub fn fold(&self) -> AttestState {
        Folder::new(self).run()
    }
}

/// Signature check used by append.
pub(crate) fn verify_signature(env: &Envelope) -> Result<(), AppendError> {
    let vk = VerifyingKey::from_bytes(&env.author.0).map_err(|_| AppendError::BadAuthorKey)?;
    let sig_bytes: [u8; 64] =
        env.signature.clone().try_into().map_err(|_| AppendError::BadSignature)?;
    let sig = Signature::from_bytes(&sig_bytes);
    vk.verify(&env.canonical_bytes(), &sig).map_err(|_| AppendError::BadSignature)
}

// ---------------------------------------------------------------------------
// Derived views (the fold's outputs)
// ---------------------------------------------------------------------------

/// Presentation metadata attached by fold/query. Markers annotate; they never
/// remove, hide, or reorder (no verdict by side effect).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marker {
    /// The attestation's base edge has been superseded (T-AT2.3).
    AntecedentSuperseded,
    /// Older than the governed freshness dial (T-AT5.5). Presentation only.
    Stale,
}

impl Marker {
    pub fn as_str(&self) -> &'static str {
        match self {
            Marker::AntecedentSuperseded => "antecedent_superseded",
            Marker::Stale => "stale",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EdgeStatus {
    Established,
    Superseded { by: ObjectId },
}

/// One side of an established edge: which persona, which half object, and the
/// side-local label (T-AT1.4).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeSide {
    pub persona: PersonaId,
    pub half: ObjectId,
    pub label: String,
}

/// An established (or superseded) edge: both halves matched on core hash.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeView {
    pub core_hash: [u8; 32],
    pub sides: [EdgeSide; 2],
    /// Provenance metadata only (T-AT1.7) — consumed by serialization/display,
    /// nothing else.
    pub grade: Grade,
    pub status: EdgeStatus,
}

impl EdgeView {
    pub fn participants(&self) -> [PersonaId; 2] {
        [self.sides[0].persona, self.sides[1].persona]
    }

    pub fn side(&self, persona: &PersonaId) -> Option<&EdgeSide> {
        self.sides.iter().find(|s| &s.persona == persona)
    }
}

/// A half with no matching counterpart: pending. Pending is never partial —
/// no query surfaces it as an edge (T-AT1.1, T-AT1.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingHalf {
    pub object: ObjectId,
    pub author: PersonaId,
    pub core_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VouchStatus {
    Standing,
    /// Base edge not resolvable as an established edge (OWNER-CALL: OC-2
    /// pending — narrowest option: edge-antecedent required).
    Pending,
    Withdrawn { by: ObjectId },
    Superseded { by: ObjectId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VouchView {
    pub object: ObjectId,
    pub author: PersonaId,
    pub subject: PersonaId,
    pub scope: Scope,
    pub statement: String,
    pub made_on: DateClaim,
    pub base_edge: [u8; 32],
    pub grade: Grade,
    pub status: VouchStatus,
    /// Presentation metadata (T-AT2.3): annotates, never withdraws.
    pub markers: Vec<Marker>,
    /// Supersede ancestry, oldest first, through this object to the chain's
    /// latest successor (withdraws included).
    pub lineage: Vec<ObjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewStatus {
    Standing,
    Superseded { by: ObjectId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewView {
    pub object: ObjectId,
    pub author: PersonaId,
    pub subject: SubjectRef,
    pub scope: Scope,
    pub statement: String,
    pub made_on: DateClaim,
    pub grade: Grade,
    pub status: ReviewStatus,
    /// Signed replies attached as peer objects (T-AT5.3).
    pub replies: Vec<ObjectId>,
    pub lineage: Vec<ObjectId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PredicateStatus {
    Current,
    Superseded { by: ObjectId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredicateView {
    pub object: ObjectId,
    /// The issuer IS the envelope author — a predicate cannot be detached from
    /// who asserted it (T-AT6.2).
    pub issuer: PersonaId,
    pub subject: PersonaId,
    pub predicate: PredicateKind,
    pub process: ProcessProvenance,
    pub status: PredicateStatus,
    pub lineage: Vec<ObjectId>,
}

/// RUN-ATTEST-02: the standing of a folded credential.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredentialStatus {
    Standing,
    /// No resolvable vetting-event antecedent from the same issuer naming the
    /// same subject (T-PA3.1) — pending, never partially standing.
    Pending,
    Superseded { by: ObjectId },
}

/// RUN-ATTEST-02: a folded single-predicate credential. Like
/// [`PredicateView`], the issuer IS the envelope author and the process block
/// is inseparable. Deliberate absences (T-PA1.1): no field designates a mint
/// position among a holder's personas — not here, not anywhere public.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredentialView {
    pub object: ObjectId,
    pub issuer: PersonaId,
    pub subject: PersonaId,
    pub predicate: PredicateKind,
    pub process: ProcessProvenance,
    pub status: CredentialStatus,
    pub lineage: Vec<ObjectId>,
}

impl CredentialView {
    /// Serialized public form (the T-PA2.1 leakage surface).
    pub fn to_ipld(&self) -> ipld_core::ipld::Ipld {
        unimplemented!("RUN-ATTEST-02: credential view serialization pending")
    }
}

/// The deterministic notice fact a folded review emits (T-AT5.2). Delivery is
/// out of scope; existence and determinism are in scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NoticeFact {
    pub review: ObjectId,
    pub subject: SubjectRef,
    /// The persona the notice is addressed to: the subject persona, or the
    /// thing's declared controller (None if no ThingDecl resolves).
    pub addressed_to: Option<PersonaId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyView {
    pub object: ObjectId,
    pub persona: PersonaId,
    pub rule: PolicyRule,
}

/// A far end in a disclosed edge list: resolvable → the persona id; not
/// resolvable to this viewer → opaque, carrying NOTHING (no hash, no alias —
/// absence of a derivable value is the point, T-AT4.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FarEnd {
    Resolved(PersonaId),
    Opaque,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeDisclosure {
    pub core_hash: [u8; 32],
    pub far_end: FarEnd,
    pub status: EdgeStatus,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// The folded state. All accessors are read-only; nothing here can remove,
/// hide, or demote a stored attestation (T-AT5.4).
pub struct AttestState {
    pub(crate) fold_order: Vec<ObjectId>,
    pub(crate) edges: Vec<EdgeView>,
    pub(crate) pending_halves: Vec<PendingHalf>,
    pub(crate) vouches: Vec<VouchView>,
    pub(crate) reviews: Vec<ReviewView>,
    pub(crate) predicates: Vec<PredicateView>,
    pub(crate) credentials: Vec<CredentialView>,
    pub(crate) notices: Vec<NoticeFact>,
    pub(crate) policies: BTreeMap<PersonaId, (PolicyView, Vec<ObjectId>)>,
}

impl AttestState {
    /// The deterministic fold order: (lamport, author, id) — logical +
    /// cryptographic inputs only (T-AT0.4).
    pub fn fold_order(&self) -> &[ObjectId] {
        &self.fold_order
    }

    pub fn edges(&self) -> &[EdgeView] {
        &self.edges
    }

    pub fn edge_by_core(&self, core_hash: &[u8; 32]) -> Option<&EdgeView> {
        self.edges.iter().find(|e| &e.core_hash == core_hash)
    }

    pub fn pending_halves(&self) -> &[PendingHalf] {
        &self.pending_halves
    }

    pub fn vouches(&self) -> &[VouchView] {
        &self.vouches
    }

    pub fn vouch(&self, id: &ObjectId) -> Option<&VouchView> {
        self.vouches.iter().find(|v| &v.object == id)
    }

    pub fn reviews(&self) -> &[ReviewView] {
        &self.reviews
    }

    pub fn review(&self, id: &ObjectId) -> Option<&ReviewView> {
        self.reviews.iter().find(|r| &r.object == id)
    }

    pub fn predicates(&self) -> &[PredicateView] {
        &self.predicates
    }

    /// RUN-ATTEST-02: folded credentials, in fold order. Read-only, like every
    /// accessor here — reviewed against the T-AT5.4 suppression invariant.
    pub fn credentials(&self) -> &[CredentialView] {
        &self.credentials
    }

    pub fn credential(&self, id: &ObjectId) -> Option<&CredentialView> {
        self.credentials.iter().find(|c| &c.object == id)
    }

    pub fn notices(&self) -> &[NoticeFact] {
        &self.notices
    }

    /// The current (head) resolvability policy for a persona, if any.
    pub fn policy_head(&self, persona: &PersonaId) -> Option<&PolicyView> {
        self.policies.get(persona).map(|(head, _)| head)
    }

    /// The policy supersede lineage for a persona, oldest first.
    pub fn policy_lineage(&self, persona: &PersonaId) -> Vec<ObjectId> {
        self.policies.get(persona).map(|(_, l)| l.clone()).unwrap_or_default()
    }

    /// Is `persona` resolvable to `viewer`? Governed by the NAMED party's own
    /// policy head (T-AT4.1). Default with no policy: resolvable (stand-in
    /// default, declared in PRIMITIVES-ATTEST.md). Self is always resolvable.
    pub fn resolvable(&self, viewer: &PersonaId, persona: &PersonaId) -> bool {
        if viewer == persona {
            return true;
        }
        match self.policy_head(persona) {
            None => true,
            Some(head) => match &head.rule {
                PolicyRule::AllowAll => true,
                PolicyRule::AllowOnly(list) => list.contains(viewer),
            },
        }
    }

    /// The edge list of persona `of`, as disclosed to `viewer`: each far end
    /// resolves ONLY per that far party's own policy — the disclosing holder
    /// has no input (there is deliberately no parameter through which `of`
    /// could grant more, T-AT4.1).
    pub fn edge_list(&self, of: &PersonaId, viewer: &PersonaId) -> Vec<EdgeDisclosure> {
        let mut out = Vec::new();
        for e in &self.edges {
            let parts = e.participants();
            let far = match () {
                _ if &parts[0] == of => parts[1],
                _ if &parts[1] == of => parts[0],
                _ => continue,
            };
            let far_end = match self.resolvable(viewer, &far) {
                true => FarEnd::Resolved(far),
                false => FarEnd::Opaque,
            };
            out.push(EdgeDisclosure { core_hash: e.core_hash, far_end, status: e.status.clone() });
        }
        out.sort_by(|a, b| a.core_hash.cmp(&b.core_hash));
        out
    }
}

// ---------------------------------------------------------------------------
// The fold itself
// ---------------------------------------------------------------------------

struct Folder<'a> {
    /// Envelopes in fold order: (lamport, author bytes, object id) ASC —
    /// logical + cryptographic inputs only; no wall-clock exists here.
    ordered: Vec<(ObjectId, &'a Envelope)>,
}

impl<'a> Folder<'a> {
    fn new(log: &'a AttestLog) -> Self {
        let mut ordered: Vec<(ObjectId, &Envelope)> =
            log.objects.iter().map(|(id, (env, _))| (*id, env)).collect();
        ordered.sort_by(|(ia, a), (ib, b)| {
            a.lamport
                .cmp(&b.lamport)
                .then_with(|| a.author.0.cmp(&b.author.0))
                .then_with(|| ia.0.cmp(&ib.0))
        });
        Folder { ordered }
    }

    fn run(self) -> AttestState {
        let fold_order: Vec<ObjectId> = self.ordered.iter().map(|(id, _)| *id).collect();

        // --- pass 1: index the primitive facts -----------------------------
        let mut ceremony: BTreeMap<ObjectId, (PersonaId, &CeremonyFact)> = BTreeMap::new();
        let mut transactions: BTreeSet<ObjectId> = BTreeSet::new();
        let mut things: BTreeMap<ThingId, PersonaId> = BTreeMap::new();
        for (id, env) in &self.ordered {
            match &env.payload {
                Payload::CeremonyFact(c) => {
                    let author_is_participant = c.participants.contains(&env.author);
                    if author_is_participant {
                        ceremony.insert(*id, (env.author, c));
                    }
                }
                Payload::TransactionFact(_) => {
                    transactions.insert(*id);
                }
                Payload::ThingDecl(t) => {
                    if env.author == t.controller {
                        things.entry(t.thing).or_insert(t.controller);
                    }
                }
                _ => {}
            }
        }

        // --- pass 2: halves → edges + pendings ------------------------------
        // Per core hash, the first half per participant (fold order) counts.
        let mut halves_by_core: BTreeMap<[u8; 32], Vec<(ObjectId, &Envelope, &EdgeHalf)>> =
            BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::EdgeHalf(h) = &env.payload {
                halves_by_core.entry(h.core.core_hash()).or_default().push((*id, env, h));
            }
        }
        let mut dissolves_by_core: BTreeMap<[u8; 32], Vec<(ObjectId, PersonaId)>> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::EdgeDissolve(d) = &env.payload {
                dissolves_by_core.entry(d.core_hash).or_default().push((*id, env.author));
            }
        }

        let mut edges: Vec<EdgeView> = Vec::new();
        let mut pending_halves: Vec<PendingHalf> = Vec::new();
        for (core_hash, halves) in &halves_by_core {
            let core = &halves[0].2.core;
            let [pa, pb] = core.participants();
            let first_of = |persona: PersonaId| {
                halves.iter().find(|(_, env, h)| {
                    env.author == persona && h.core.participants().contains(&persona)
                })
            };
            let side_a = first_of(pa);
            let side_b = first_of(pb);
            match (side_a, side_b) {
                (Some((ida, _, ha)), Some((idb, _, hb))) => {
                    let copresent = has_copresence(core, &ceremony);
                    let prov = if copresent {
                        Grade::InPerson
                    } else {
                        Grade::Remote
                    };
                    let status = dissolves_by_core
                        .get(core_hash)
                        .and_then(|ds| {
                            ds.iter().find(|(_, author)| [pa, pb].contains(author))
                        })
                        .map(|(id, _)| EdgeStatus::Superseded { by: *id })
                        .unwrap_or(EdgeStatus::Established);
                    edges.push(EdgeView {
                        core_hash: *core_hash,
                        sides: [
                            EdgeSide { persona: pa, half: *ida, label: ha.label.clone() },
                            EdgeSide { persona: pb, half: *idb, label: hb.label.clone() },
                        ],
                        grade: prov,
                        status,
                    });
                }
                _ => {
                    for (id, env, _) in halves {
                        pending_halves.push(PendingHalf {
                            object: *id,
                            author: env.author,
                            core_hash: *core_hash,
                        });
                    }
                }
            }
        }

        // --- pass 3: vouch lineage ------------------------------------------
        // Successor of a vouch = the first (fold order) same-author object
        // superseding it: a narrowing vouch or a withdraw.
        let mut vouch_envs: BTreeMap<ObjectId, (&Envelope, &Vouch)> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Vouch(v) = &env.payload {
                vouch_envs.insert(*id, (env, v));
            }
        }
        let mut vouch_successor: BTreeMap<ObjectId, (ObjectId, bool)> = BTreeMap::new(); // target -> (successor, is_withdraw)
        for (id, env) in &self.ordered {
            match &env.payload {
                Payload::Vouch(v) => {
                    if let Some(target) = v.supersedes {
                        let valid = vouch_envs
                            .get(&target)
                            .map(|(tenv, _)| tenv.author == env.author)
                            .unwrap_or(false);
                        if valid {
                            vouch_successor.entry(target).or_insert((*id, false));
                        }
                    }
                }
                Payload::VouchWithdraw(wd) => {
                    let valid = vouch_envs
                        .get(&wd.supersedes)
                        .map(|(tenv, _)| tenv.author == env.author)
                        .unwrap_or(false);
                    if valid {
                        vouch_successor.entry(wd.supersedes).or_insert((*id, true));
                    }
                }
                _ => {}
            }
        }

        let mut vouches: Vec<VouchView> = Vec::new();
        for (id, env) in &self.ordered {
            let Payload::Vouch(v) = &env.payload else { continue };
            // Standing requires: the base edge folded (both halves), the
            // author is one of its participants, and the subject is the other.
            let base = edges.iter().find(|e| e.core_hash == v.base_edge);
            let standing = base
                .map(|e| {
                    let parts = e.participants();
                    parts.contains(&env.author)
                        && parts.contains(&v.subject)
                        && env.author != v.subject
                })
                .unwrap_or(false);
            let status = match (standing, vouch_successor.get(id)) {
                (false, _) => VouchStatus::Pending,
                (true, None) => VouchStatus::Standing,
                (true, Some((by, true))) => VouchStatus::Withdrawn { by: *by },
                (true, Some((by, false))) => VouchStatus::Superseded { by: *by },
            };
            let mut markers = Vec::new();
            let base_superseded =
                base.map(|e| matches!(e.status, EdgeStatus::Superseded { .. })).unwrap_or(false);
            if base_superseded {
                markers.push(Marker::AntecedentSuperseded);
            }
            let has_tx = env.antecedents.iter().any(|a| transactions.contains(a));
            let base_prov = base.map(|e| e.grade).unwrap_or(Grade::Remote);
            let prov = if has_tx {
                Grade::TransactionBacked
            } else {
                base_prov
            };
            let lineage = chain_of(*id, &vouch_envs, &vouch_successor);
            vouches.push(VouchView {
                object: *id,
                author: env.author,
                subject: v.subject,
                scope: v.scope.clone(),
                statement: v.statement.clone(),
                made_on: v.made_on,
                base_edge: v.base_edge,
                grade: prov,
                status,
                markers,
                lineage,
            });
        }

        // --- pass 4: reviews, replies, notices ------------------------------
        let mut review_envs: BTreeMap<ObjectId, (&Envelope, &Review)> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Review(r) = &env.payload {
                review_envs.insert(*id, (env, r));
            }
        }
        let mut review_successor: BTreeMap<ObjectId, (ObjectId, bool)> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Review(r) = &env.payload {
                if let Some(target) = r.supersedes {
                    let valid = review_envs
                        .get(&target)
                        .map(|(tenv, _)| tenv.author == env.author)
                        .unwrap_or(false);
                    if valid {
                        review_successor.entry(target).or_insert((*id, false));
                    }
                }
            }
        }
        // The persona a subject's replies must be signed by.
        let subject_persona = |sr: &SubjectRef| -> Option<PersonaId> {
            match sr {
                SubjectRef::Persona(p) => Some(*p),
                SubjectRef::Thing(t) => things.get(t).copied(),
            }
        };
        let mut replies_by_review: BTreeMap<ObjectId, Vec<ObjectId>> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Reply(rp) = &env.payload {
                let addressee = review_envs
                    .get(&rp.review)
                    .and_then(|(_, review)| subject_persona(&review.subject));
                if addressee == Some(env.author) {
                    replies_by_review.entry(rp.review).or_default().push(*id);
                }
            }
        }

        let mut reviews: Vec<ReviewView> = Vec::new();
        let mut notices: Vec<NoticeFact> = Vec::new();
        for (id, env) in &self.ordered {
            let Payload::Review(r) = &env.payload else { continue };
            let status = match review_successor.get(id) {
                None => ReviewStatus::Standing,
                Some((by, _)) => ReviewStatus::Superseded { by: *by },
            };
            let has_tx = env.antecedents.iter().any(|a| transactions.contains(a));
            let prov = if has_tx {
                Grade::TransactionBacked
            } else {
                Grade::Remote
            };
            let review_chain = review_envs
                .iter()
                .map(|(k, (e, r))| (*k, (*e, RSup(r.supersedes))))
                .collect::<BTreeMap<_, _>>();
            let lineage = chain_of_generic(*id, &review_chain, &review_successor);
            reviews.push(ReviewView {
                object: *id,
                author: env.author,
                subject: r.subject,
                scope: r.scope.clone(),
                statement: r.statement.clone(),
                made_on: r.made_on,
                grade: prov,
                status,
                replies: replies_by_review.get(id).cloned().unwrap_or_default(),
                lineage,
            });
            // T-AT5.2: folding a review deterministically emits a notice fact
            // addressed to the subject. Existence + determinism are in scope;
            // delivery is not.
            notices.push(NoticeFact {
                review: *id,
                subject: r.subject,
                addressed_to: subject_persona(&r.subject),
            });
        }
        notices.sort();

        // --- pass 5: predicates ---------------------------------------------
        let mut pred_envs: BTreeMap<ObjectId, (&Envelope, &Predicate)> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Predicate(p) = &env.payload {
                pred_envs.insert(*id, (env, p));
            }
        }
        let mut pred_successor: BTreeMap<ObjectId, (ObjectId, bool)> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::Predicate(p) = &env.payload {
                if let Some(target) = p.supersedes {
                    let valid = pred_envs
                        .get(&target)
                        .map(|(tenv, _)| tenv.author == env.author)
                        .unwrap_or(false);
                    if valid {
                        pred_successor.entry(target).or_insert((*id, false));
                    }
                }
            }
        }
        let mut predicates: Vec<PredicateView> = Vec::new();
        for (id, env) in &self.ordered {
            let Payload::Predicate(p) = &env.payload else { continue };
            let status = match pred_successor.get(id) {
                None => PredicateStatus::Current,
                Some((by, _)) => PredicateStatus::Superseded { by: *by },
            };
            let pred_chain = pred_envs
                .iter()
                .map(|(k, (e, p))| (*k, (*e, RSup(p.supersedes))))
                .collect::<BTreeMap<_, _>>();
            let lineage = chain_of_generic(*id, &pred_chain, &pred_successor);
            predicates.push(PredicateView {
                object: *id,
                issuer: env.author,
                subject: p.subject,
                predicate: p.predicate,
                process: p.process,
                status,
                lineage,
            });
        }

        // --- pass 6: resolvability policies ---------------------------------
        // Only the NAMED party's own facts govern their resolvability: a
        // policy is valid iff its envelope author is the persona it names.
        let mut policy_envs: BTreeMap<ObjectId, (&Envelope, &ResolvabilityPolicy)> =
            BTreeMap::new();
        let mut policies_by_persona: BTreeMap<PersonaId, Vec<ObjectId>> = BTreeMap::new();
        for (id, env) in &self.ordered {
            if let Payload::ResolvabilityPolicy(rp) = &env.payload {
                if env.author == rp.persona {
                    policy_envs.insert(*id, (env, rp));
                    policies_by_persona.entry(rp.persona).or_default().push(*id);
                }
            }
        }
        let mut policy_successor: BTreeMap<ObjectId, (ObjectId, bool)> = BTreeMap::new();
        for (id, (_, rp)) in &policy_envs {
            if let Some(target) = rp.supersedes {
                if policy_envs.contains_key(&target) {
                    policy_successor.entry(target).or_insert((*id, false));
                }
            }
        }
        let mut policies: BTreeMap<PersonaId, (PolicyView, Vec<ObjectId>)> = BTreeMap::new();
        for (persona, ids) in &policies_by_persona {
            // Head: the last (fold-order) policy with no successor.
            let head_id = ids
                .iter()
                .rev()
                .find(|id| !policy_successor.contains_key(id))
                .copied()
                .unwrap_or(*ids.last().unwrap());
            // Lineage: walk back from the head via supersede pointers, oldest first.
            let mut lineage = vec![head_id];
            let mut cur = head_id;
            while let Some(target) = policy_envs.get(&cur).and_then(|(_, rp)| rp.supersedes) {
                if !policy_envs.contains_key(&target) || lineage.contains(&target) {
                    break;
                }
                lineage.push(target);
                cur = target;
            }
            lineage.reverse();
            let (_, rp) = policy_envs[&head_id];
            policies.insert(
                *persona,
                (
                    PolicyView { object: head_id, persona: *persona, rule: rp.rule.clone() },
                    lineage,
                ),
            );
        }

        // RUN-ATTEST-02 credential pass — pending (RED stage): no credential
        // ever folds, so every credential test fails against the empty view.
        let credentials: Vec<CredentialView> = Vec::new();

        AttestState {
            fold_order,
            edges,
            pending_halves,
            vouches,
            reviews,
            predicates,
            credentials,
            notices,
            policies,
        }
    }
}

/// Does the core's ceremony reference a co-presence session signed by BOTH
/// participants? (One fact from each, same session, naming this pair.)
fn has_copresence(
    core: &EdgeCore,
    ceremony: &BTreeMap<ObjectId, (PersonaId, &CeremonyFact)>,
) -> bool {
    let pair = core.participants();
    let mut sessions_a: BTreeSet<[u8; 16]> = BTreeSet::new();
    let mut sessions_b: BTreeSet<[u8; 16]> = BTreeSet::new();
    for cid in &core.ceremony {
        let Some((author, fact)) = ceremony.get(cid) else { continue };
        if fact.participants != pair {
            continue;
        }
        if *author == pair[0] {
            sessions_a.insert(fact.session);
        }
        if *author == pair[1] {
            sessions_b.insert(fact.session);
        }
    }
    sessions_a.intersection(&sessions_b).next().is_some()
}

/// Marker for generic supersede chains (the review/predicate reuse of the
/// vouch chain-walk).
struct RSup(Option<ObjectId>);

/// The full supersede chain containing `id`: walk back to the root via each
/// object's own supersede pointer, then forward via successors (withdraws
/// included). Oldest first.
fn chain_of(
    id: ObjectId,
    envs: &BTreeMap<ObjectId, (&Envelope, &Vouch)>,
    successor: &BTreeMap<ObjectId, (ObjectId, bool)>,
) -> Vec<ObjectId> {
    let generic = envs
        .iter()
        .map(|(k, (e, v))| (*k, (*e, RSup(v.supersedes))))
        .collect::<BTreeMap<_, _>>();
    chain_of_generic(id, &generic, successor)
}

fn chain_of_generic(
    id: ObjectId,
    envs: &BTreeMap<ObjectId, (&Envelope, RSup)>,
    successor: &BTreeMap<ObjectId, (ObjectId, bool)>,
) -> Vec<ObjectId> {
    // Back to root.
    let mut back = Vec::new();
    let mut cur = id;
    while let Some((_, RSup(Some(target)))) = envs.get(&cur) {
        if !envs.contains_key(target) || back.contains(target) || *target == id {
            break;
        }
        back.push(*target);
        cur = *target;
    }
    back.reverse();
    // Forward through successors (may end on a withdraw object outside `envs`).
    let mut chain = back;
    chain.push(id);
    let mut cur = id;
    while let Some((next, _)) = successor.get(&cur) {
        if chain.contains(next) {
            break;
        }
        chain.push(*next);
        cur = *next;
    }
    chain
}
