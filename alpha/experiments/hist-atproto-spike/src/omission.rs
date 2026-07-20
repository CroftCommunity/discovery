//! B6 — omission detection: a responder withholding one entry is caught by
//! the member regardless of responder honesty (rbsr-construction req. 5 on
//! this backend), and admission REUSES the proven A2.3 machinery — standing
//! plus contiguity (`lineage-history::backfill_import`, Proofs Phase 2.5) —
//! never a reimplementation.
//!
//! The shape: a span of history arrives as signed messages (author = the
//! device-subspace's persona; payload = the canonical §G envelope bytes).
//! Admission requires, before absorbing anything: (a) contiguous sequence,
//! (b) author standing on the lineage, (c) valid signatures — all three from
//! `backfill_import` — and then (d) the envelope-level predecessor chain
//! verifies, with any break NAMED by bounding digests (§I / B3's
//! `verify_span`). A signature-deep check alone is exactly the A2.3 gap.

use crate::envelope::{Digest, Envelope};
use crate::pages::{verify_span, ChainCheck};
use lineage_core::dag::Lineage;
use lineage_core::ids::{Did, GenesisId};
use lineage_core::keys::VerifyingIdentity;
use lineage_history::{BackfillError, BranchHistory, HistoryStore};

/// Why a served span was rejected (or what gap it names).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdmitError {
    /// Rejected by the reused A2.3 admission machinery (standing, contiguity,
    /// signature — see `lineage_history::BackfillError`).
    Backfill(BackfillError),
    /// Admitted by transport-level checks but the §G predecessor chain has a
    /// break, named by its bounding digests (the omission, made nameable).
    EnvelopeGap {
        subspace: Digest,
        after_digest: Digest,
        before_digest: Digest,
    },
    /// A payload that does not decode as a canonical envelope.
    BadEnvelope(String),
}

/// Admit a served span of envelope-carrying messages: the REUSED A2.3
/// admission (standing plus contiguity plus signature, via
/// `backfill_import`) and then the §G predecessor-chain verification. (The
/// B6 red run captured the signature-deep form — the exact check A2.3
/// proved insufficient — admitting a withheld span, a stranger's well-signed
/// history, and a broken predecessor chain before this went green.)
pub fn admit_span(
    donor: &BranchHistory,
    my_branch: GenesisId,
    lineage: &Lineage,
    verify: impl Fn(&Did, &[u8], &Sig) -> bool,
) -> Result<Vec<Envelope>, AdmitError> {
    admit_via_backfill(donor, my_branch, lineage, verify)
}

pub use lineage_core::keys::Sig;

/// Verify-closure over a fixed registry of verifying identities: unknown
/// authors fail verification (never "trusted because unknown").
pub fn registry_verify<'a>(
    registry: &'a [VerifyingIdentity],
) -> impl Fn(&Did, &[u8], &Sig) -> bool + 'a {
    move |did, bytes, sig| {
        registry
            .iter()
            .find(|v| v.did() == did)
            .is_some_and(|v| v.verify(bytes, sig))
    }
}

/// Green-path helper (used at green; kept separate so the staged red above is
/// a single function swap): run the REUSED `backfill_import` over a fresh
/// `HistoryStore`, then the envelope-level chain verification.
#[allow(dead_code)]
pub(crate) fn admit_via_backfill(
    donor: &BranchHistory,
    my_branch: GenesisId,
    lineage: &Lineage,
    verify: impl Fn(&Did, &[u8], &Sig) -> bool,
) -> Result<Vec<Envelope>, AdmitError> {
    let mut store = HistoryStore::new();
    store
        .backfill_import(donor, my_branch, lineage, verify)
        .map_err(AdmitError::Backfill)?;
    let admitted = store
        .branch(donor.branch)
        .expect("just imported")
        .messages()
        .iter()
        .map(|m| Envelope::from_canonical(&m.payload).map_err(AdmitError::BadEnvelope))
        .collect::<Result<Vec<_>, _>>()?;
    if let Some(first) = admitted.first() {
        let subspace = first.subspace;
        match verify_span(&subspace, &admitted) {
            ChainCheck::Complete(envs) => Ok(envs),
            ChainCheck::Gap {
                subspace,
                after_digest,
                before_digest,
            } => Err(AdmitError::EnvelopeGap {
                subspace,
                after_digest,
                before_digest,
            }),
        }
    } else {
        Ok(admitted)
    }
}
