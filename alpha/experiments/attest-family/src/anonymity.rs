//! RUN-ATTEST-02 — EXP-PA4: the anonymity-set measurement harness and
//! presentation-side bundle composition.
//!
//! A credential partitions personas into the pool of same-(issuer, predicate)
//! holders; showing several predicates together intersects the pools. This
//! module MEASURES those pool sizes (instrumented, not pass/fail — but the
//! harness itself is proven red-first on a hand-computed fixture, T-PA4.1).

use std::collections::BTreeSet;

use ipld_core::ipld::Ipld;

use crate::fold::AttestState;
use crate::types::*;

/// The pool of personas that hold a STANDING credential from `issuer` for
/// EVERY predicate in `shown` — the anonymity set of a persona presenting
/// exactly that bundle.
pub fn pool(state: &AttestState, issuer: &PersonaId, shown: &[PredicateKind]) -> BTreeSet<PersonaId> {
    let holds = |persona: &PersonaId, kind: &PredicateKind| -> bool {
        state.credentials().iter().any(|c| {
            &c.issuer == issuer
                && &c.subject == persona
                && c.predicate == *kind
                && c.status == crate::fold::CredentialStatus::Standing
        })
    };
    let subjects: BTreeSet<PersonaId> = state
        .credentials()
        .iter()
        .filter(|c| &c.issuer == issuer)
        .map(|c| c.subject)
        .collect();
    subjects
        .into_iter()
        .filter(|p| shown.iter().all(|k| holds(p, k)))
        .collect()
}

pub fn pool_size(state: &AttestState, issuer: &PersonaId, shown: &[PredicateKind]) -> u64 {
    pool(state, issuer, shown).len() as u64
}

/// One row per predicate for `issuer` — the M-PA4.2 tabulation.
pub fn tabulate(state: &AttestState, issuer: &PersonaId) -> Vec<(PredicateKind, u64)> {
    [
        PredicateKind::VettedHolder,
        PredicateKind::Over18,
        PredicateKind::PhoneVerified,
        PredicateKind::PaymentVerified,
    ]
    .into_iter()
    .map(|k| (k, pool_size(state, issuer, &[k])))
    .collect()
}

/// A presentation: the SUBSET of published credential envelopes a persona
/// chooses to show. Because single-predicate credentials are the unit (§3),
/// any subset is presentable without revealing unpresented predicates
/// (T-PA4.4) — the serialized presentation carries no trace of them.
#[derive(Debug, Clone)]
pub struct Presentation {
    pub shown: Vec<Envelope>,
}

/// Compose a presentation from a persona's published credentials: exactly the
/// envelopes whose predicate is in `kinds`, nothing else.
pub fn present(published: &[Envelope], kinds: &[PredicateKind]) -> Presentation {
    let shown = published
        .iter()
        .filter(|e| {
            matches!(&e.payload, Payload::Credential(c) if kinds.contains(&c.predicate))
        })
        .cloned()
        .collect();
    Presentation { shown }
}

impl Presentation {
    /// The presentation's wire form: exactly the shown envelopes' canonical
    /// bytes, in canonical (byte-ascending) order — no count of anything
    /// unpresented, no other field at all.
    pub fn to_ipld(&self) -> Ipld {
        let mut bytes: Vec<Vec<u8>> =
            self.shown.iter().map(|e| e.canonical_bytes_with_sig()).collect();
        bytes.sort();
        Ipld::List(bytes.into_iter().map(Ipld::Bytes).collect())
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        serde_ipld_dagcbor::to_vec(&self.to_ipld()).expect("pure value encode cannot fail")
    }
}
