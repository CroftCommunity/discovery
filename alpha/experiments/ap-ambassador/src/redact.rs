//! The Delete custom rider (AP-V3 / P4).
//!
//! On a verified `Delete` naming an actor held in the store, the ambassador
//! redacts the evidence body and keeps the fact skeleton — the commitment
//! and interval boundaries survive. Masked never-was-world equality: except
//! for the commitment and marker, the post-redaction state is byte-equal to
//! a world that never held the body.
//!
//! Undo Follow does NOT trigger redaction (only Delete does — the rider's
//! selective-respect assertion, T-AP4.4).

use crate::records::ReceiptRecord;
use crate::store::EvidenceStore;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedactError {
    /// The record referenced is not in the store (no evidence to redact).
    NoSuchReceipt,
    /// The record is already redacted; a second Delete is a no-op but the
    /// caller is told (not an error, but a distinct variant so caller
    /// knows).
    AlreadyRedacted,
}

/// Apply a Delete's redaction to every receipt whose actor matches the
/// target of the Delete. The Delete record itself is NOT redacted — it is
/// itself a receipt (a Delete-receipt) and stays evidence-complete unless a
/// later Delete targets its own actor.
///
/// Returns the list of ReceiptIds whose state moved from EvidenceComplete
/// → AttestedRedacted.
pub fn apply_delete(
    delete_receipt: &ReceiptRecord,
    store: &mut EvidenceStore,
) -> Vec<ReceiptId> {
    assert_eq!(
        delete_receipt.kind,
        ActivityKind::Delete,
        "apply_delete requires a Delete kind"
    );
    let target = &delete_receipt.object;
    let mut redacted = Vec::new();
    let ids: Vec<ReceiptId> = store.receipts().map(|r| r.receipt_id()).collect();
    for id in ids {
        // Never redact the Delete itself — see the doc-comment.
        if id == delete_receipt.receipt_id() {
            continue;
        }
        let Some(r) = store.receipt(&id) else {
            continue;
        };
        // The target of the redaction is the actor identity as a URL. A
        // Delete of an actor URL redacts every held receipt authored by
        // that actor. A Delete of an object URL that isn't an actor is a
        // no-op this run (AP-OC-9 territory).
        if r.actor.0 == *target {
            match store.redact(&id) {
                Ok(()) => redacted.push(id),
                Err(RedactError::AlreadyRedacted) | Err(RedactError::NoSuchReceipt) => {}
            }
        }
    }
    redacted
}
