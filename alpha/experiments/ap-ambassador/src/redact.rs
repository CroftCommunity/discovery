//! The Delete custom rider (AP-V3 / P4).

use crate::records::ReceiptRecord;
use crate::store::EvidenceStore;
use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedactError {
    NoSuchReceipt,
    AlreadyRedacted,
}

/// Apply a Delete's redaction to every receipt whose actor matches the
/// target of the Delete.
pub fn apply_delete(_delete_receipt: &ReceiptRecord, _store: &mut EvidenceStore) -> Vec<ReceiptId> {
    unimplemented!("P4 GREEN: redact evidence body, keep skeleton, state → AttestedRedacted")
}
