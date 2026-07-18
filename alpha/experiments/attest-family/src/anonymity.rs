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
    let _ = (state, issuer, shown);
    unimplemented!("RUN-ATTEST-02: pool measurement pending")
}

pub fn pool_size(state: &AttestState, issuer: &PersonaId, shown: &[PredicateKind]) -> u64 {
    pool(state, issuer, shown).len() as u64
}

/// One row per predicate for `issuer` — the M-PA4.2 tabulation.
pub fn tabulate(state: &AttestState, issuer: &PersonaId) -> Vec<(PredicateKind, u64)> {
    let _ = (state, issuer);
    unimplemented!("RUN-ATTEST-02: tabulation pending")
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
    let _ = (published, kinds);
    unimplemented!("RUN-ATTEST-02: presentation composition pending")
}

impl Presentation {
    pub fn to_ipld(&self) -> Ipld {
        unimplemented!("RUN-ATTEST-02: presentation serialization pending")
    }

    pub fn to_canonical_bytes(&self) -> Vec<u8> {
        unimplemented!("RUN-ATTEST-02: presentation serialization pending")
    }
}
