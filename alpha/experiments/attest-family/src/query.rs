//! The one query in the model: given (viewer, subject, scope), return the
//! **corroboration structure** — the set of standing vouches/reviews whose
//! scope matches and whose attester persona is resolvable to this viewer —
//! with grades and lineage pointers. Never an aggregate. Clients do the
//! weighting; the protocol computes provenance, never utility.

use std::collections::BTreeMap;

use ipld_core::ipld::Ipld;

use crate::fold::{AttestState, Marker};
use crate::types::*;

/// The governed freshness dial (T-AT5.5): entries whose date-claim is older
/// than this many days (relative to a caller-supplied `as_of` claim) gain a
/// `stale` presentation marker. Nothing is ever dropped or down-ranked by the
/// protocol — no verdict by timeout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshnessDial {
    pub stale_after_days: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    Vouch,
    Review,
}

impl EntryKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntryKind::Vouch => "vouch",
            EntryKind::Review => "review",
        }
    }
}

/// One standing attestation in a corroboration structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorroborationEntry {
    pub attestation: ObjectId,
    pub attester: PersonaId,
    pub kind: EntryKind,
    pub grade: Grade,
    /// Kind-derived grade set (V1, T-A3.3) — for a vouch, one grade per
    /// qualifying antecedent kind; for a review, the single review grade.
    /// Metadata only, exactly like `grade`.
    pub grades: Vec<Grade>,
    pub statement: String,
    pub made_on: DateClaim,
    pub markers: Vec<Marker>,
    pub lineage: Vec<ObjectId>,
    pub replies: Vec<ObjectId>,
}

/// The corroboration structure: a set with provenance, never a scalar.
/// Entries are in canonical-hash order (object id bytes ascending) — the only
/// ordering, and it is not computed from content, grade, or date.
///
/// No aggregate exists — a compile-boundary fact (T-AT0.2, T-AT3.1):
///
/// ```compile_fail
/// let s: attest_family::query::CorroborationStructure = todo!();
/// let _ = s.trust_score; // no such field exists, and never will
/// ```
///
/// ```compile_fail
/// let s: attest_family::query::CorroborationStructure = todo!();
/// let _ = s.entries.len() as f64 / s.total; // no denominator to aggregate over
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorroborationStructure {
    pub subject: SubjectRef,
    pub scope: Scope,
    pub entries: Vec<CorroborationEntry>,
}

/// The "N connections in common" disclosure: cardinality ONLY (T-AT3.5). The
/// serialized form carries no identifier, hash, or derivable value of the N
/// counterpart personas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutualCount {
    pub count: u64,
}

// ---------------------------------------------------------------------------
// Serialization (dag-cbor, the §4.6 path) — for leakage property tests
// ---------------------------------------------------------------------------

fn map(pairs: Vec<(&str, Ipld)>) -> Ipld {
    let mut m = BTreeMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    Ipld::Map(m)
}

fn subject_ipld(sr: &SubjectRef) -> Ipld {
    match sr {
        SubjectRef::Persona(p) => map(vec![
            ("i", Ipld::Bytes(p.0.to_vec())),
            ("k", Ipld::String("persona".into())),
        ]),
        SubjectRef::Thing(t) => map(vec![
            ("i", Ipld::Bytes(t.0.to_vec())),
            ("k", Ipld::String("thing".into())),
        ]),
    }
}

fn date_ipld(d: &DateClaim) -> Ipld {
    map(vec![
        ("d", Ipld::Integer(d.day as i128)),
        ("m", Ipld::Integer(d.month as i128)),
        ("y", Ipld::Integer(d.year as i128)),
    ])
}

impl CorroborationEntry {
    pub fn to_ipld(&self) -> Ipld {
        map(vec![
            ("a", Ipld::Bytes(self.attestation.0.to_vec())),
            ("d", date_ipld(&self.made_on)),
            ("e", Ipld::Bytes(self.attester.0.to_vec())),
            (
                "g",
                // Grade reaches serialization as a STRING — this is the one
                // sanctioned consumer of grade (T-AT1.7).
                Ipld::String(self.grade.as_str().into()),
            ),
            (
                "j",
                // The kind-derived grade set (V1), strings like "g".
                Ipld::List(self.grades.iter().map(|g| Ipld::String(g.as_str().into())).collect()),
            ),
            ("k", Ipld::String(self.kind.as_str().into())),
            (
                "l",
                Ipld::List(self.lineage.iter().map(|o| Ipld::Bytes(o.0.to_vec())).collect()),
            ),
            (
                "m",
                Ipld::List(
                    self.markers.iter().map(|mk| Ipld::String(mk.as_str().into())).collect(),
                ),
            ),
            (
                "r",
                Ipld::List(self.replies.iter().map(|o| Ipld::Bytes(o.0.to_vec())).collect()),
            ),
            ("t", Ipld::String(self.statement.clone())),
        ])
    }
}

impl CorroborationStructure {
    pub fn to_ipld(&self) -> Ipld {
        map(vec![
            ("e", Ipld::List(self.entries.iter().map(|e| e.to_ipld()).collect())),
            ("o", Ipld::String(self.scope.0.clone())),
            ("s", subject_ipld(&self.subject)),
        ])
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure map encode cannot fail")
    }
}

impl MutualCount {
    pub fn to_ipld(&self) -> Ipld {
        map(vec![("c", Ipld::Integer(self.count as i128))])
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure map encode cannot fail")
    }
}

/// Serialize a disclosed edge list (T-AT4.1/4.3 leakage surface). An opaque
/// far end serializes as the bare string "opaque" — no bytes, no alias.
pub fn edge_list_ipld(list: &[crate::fold::EdgeDisclosure]) -> Ipld {
    Ipld::List(
        list.iter()
            .map(|d| {
                let far = match &d.far_end {
                    crate::fold::FarEnd::Resolved(p) => Ipld::Bytes(p.0.to_vec()),
                    crate::fold::FarEnd::Opaque => Ipld::String("opaque".into()),
                };
                let status = match &d.status {
                    crate::fold::EdgeStatus::Established => Ipld::String("established".into()),
                    crate::fold::EdgeStatus::Superseded { by } => map(vec![
                        ("b", Ipld::Bytes(by.0.to_vec())),
                        ("k", Ipld::String("superseded".into())),
                    ]),
                };
                map(vec![
                    ("f", far),
                    ("h", Ipld::Bytes(d.core_hash.to_vec())),
                    ("u", status),
                ])
            })
            .collect(),
    )
}

// ---------------------------------------------------------------------------
// The query
// ---------------------------------------------------------------------------

impl AttestState {
    /// The only query in the model. Returns corroboration STRUCTURE,
    /// viewer-relative: standing vouches/reviews with exact scope match whose
    /// attester is resolvable to `viewer`. A non-resolvable attester's
    /// attestation is ABSENT — not redacted-but-counted (T-AT3.3).
    pub fn corroboration(
        &self,
        viewer: &PersonaId,
        subject: &SubjectRef,
        scope: &Scope,
        dial: &FreshnessDial,
        as_of: DateClaim,
    ) -> CorroborationStructure {
        let mut entries: Vec<CorroborationEntry> = Vec::new();

        for v in self.vouches() {
            let subject_matches = matches!(subject, SubjectRef::Persona(p) if *p == v.subject);
            // A withdrawn (author-superseded) vouch is ABSENT — no tombstone
            // field, no count, no residue of any kind (T-A3.6, V2).
            let standing = v.status == crate::fold::VouchStatus::Standing;
            let traverses = subject_matches
                && standing
                && v.scope == *scope
                && self.resolvable(viewer, &v.author);
            if traverses {
                let mut markers = v.markers.clone();
                markers.extend(stale_marker(&v.made_on, dial, &as_of));
                entries.push(CorroborationEntry {
                    attestation: v.object,
                    attester: v.author,
                    kind: EntryKind::Vouch,
                    grade: v.grade,
                    grades: v.grades.clone(),
                    statement: v.statement.clone(),
                    made_on: v.made_on,
                    markers,
                    lineage: v.lineage.clone(),
                    replies: Vec::new(),
                });
            }
        }

        for r in self.reviews() {
            let traverses = r.subject == *subject
                && r.status == crate::fold::ReviewStatus::Standing
                && r.scope == *scope
                && self.resolvable(viewer, &r.author);
            if traverses {
                let markers = stale_marker(&r.made_on, dial, &as_of);
                entries.push(CorroborationEntry {
                    attestation: r.object,
                    attester: r.author,
                    kind: EntryKind::Review,
                    grade: r.grade,
                    grades: vec![r.grade],
                    statement: r.statement.clone(),
                    made_on: r.made_on,
                    markers,
                    lineage: r.lineage.clone(),
                    replies: r.replies.clone(),
                });
            }
        }

        // Canonical-hash order — the ONLY ordering, computed from nothing but
        // the content address.
        entries.sort_by(|a, b| a.attestation.0.cmp(&b.attestation.0));
        CorroborationStructure { subject: *subject, scope: scope.clone(), entries }
    }

    /// Presentation markers for a predicate view under the governed freshness
    /// dial (T-AT6.3, same dial semantics as T-AT5.5): an old process date
    /// gains `stale`. Presentation only — never expiry, never removal.
    pub fn predicate_presentation(
        &self,
        view: &crate::fold::PredicateView,
        dial: &FreshnessDial,
        as_of: DateClaim,
    ) -> Vec<Marker> {
        stale_marker(&view.process.performed_on, dial, &as_of)
    }

    /// Cardinality-only mutual-connections disclosure (T-AT3.5): the number of
    /// personas holding an established edge to both `a` and `b`. Identity of
    /// the counterparts is not returned, resolvable or not — count without
    /// identity is the disclosure's entire content.
    pub fn mutual_connection_count(&self, a: &PersonaId, b: &PersonaId) -> MutualCount {
        let mut counterparts_a = std::collections::BTreeSet::new();
        let mut counterparts_b = std::collections::BTreeSet::new();
        for e in self.edges() {
            if e.status != crate::fold::EdgeStatus::Established {
                continue;
            }
            let parts = e.participants();
            if parts.contains(a) {
                counterparts_a.extend(parts.iter().filter(|p| *p != a).copied());
            }
            if parts.contains(b) {
                counterparts_b.extend(parts.iter().filter(|p| *p != b).copied());
            }
        }
        counterparts_a.remove(b);
        counterparts_b.remove(a);
        let count = counterparts_a.intersection(&counterparts_b).count() as u64;
        MutualCount { count }
    }
}

/// The freshness dial's ONLY effect: an entry whose asserted date-claim is
/// older than the dial (relative to the caller-supplied `as_of` claim) gains a
/// `stale` marker. Nothing is dropped, nothing is reordered — presentation
/// only, no verdict by timeout (T-AT5.5, T-AT6.3).
fn stale_marker(made_on: &DateClaim, dial: &FreshnessDial, as_of: &DateClaim) -> Vec<Marker> {
    let age_days = as_of.approx_days() - made_on.approx_days();
    if age_days > dial.stale_after_days as i64 {
        vec![Marker::Stale]
    } else {
        Vec::new()
    }
}
