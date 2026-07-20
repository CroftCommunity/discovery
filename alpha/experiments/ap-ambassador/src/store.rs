//! Ambassador store — in-memory this run (declared stand-in). Holds one
//! entry per receipt: the (canonical, dag-cbor-encoded) record and the
//! evidence body OR the redaction marker if the body has been redacted
//! (AP-V3).

use std::collections::BTreeMap;

use crate::records::{EvidenceBody, ReceiptRecord};
use crate::redact::RedactError;
use crate::types::*;

/// A stored entry — record plus body-or-redacted.
#[derive(Debug, Clone)]
pub struct StoredEntry {
    pub record: ReceiptRecord,
    pub body: Option<EvidenceBody>, // None ⇔ AttestedRedacted
}

/// In-memory evidence store keyed by ReceiptId.
#[derive(Debug, Default)]
pub struct EvidenceStore {
    entries: BTreeMap<[u8; 32], StoredEntry>,
}

impl EvidenceStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert an evidence-complete receipt with its body.
    pub fn insert(&mut self, record: ReceiptRecord, body: EvidenceBody) -> ReceiptId {
        let id = record.receipt_id();
        self.entries.insert(
            id.0,
            StoredEntry {
                record,
                body: Some(body),
            },
        );
        id
    }

    /// Fetch a receipt by id.
    pub fn receipt(&self, id: &ReceiptId) -> Option<&ReceiptRecord> {
        self.entries.get(&id.0).map(|e| &e.record)
    }

    /// Fetch a body by id (None ⇔ redacted).
    pub fn body(&self, id: &ReceiptId) -> Option<&EvidenceBody> {
        self.entries.get(&id.0).and_then(|e| e.body.as_ref())
    }

    /// Iterate over all held receipts (record only, no body).
    pub fn receipts(&self) -> impl Iterator<Item = &ReceiptRecord> {
        self.entries.values().map(|e| &e.record)
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Redact an entry: body → None, record state → AttestedRedacted.
    /// Idempotent-informing: a second redact returns `AlreadyRedacted`.
    pub fn redact(&mut self, id: &ReceiptId) -> Result<(), RedactError> {
        let entry = self.entries.get_mut(&id.0).ok_or(RedactError::NoSuchReceipt)?;
        if entry.body.is_none() {
            return Err(RedactError::AlreadyRedacted);
        }
        entry.body = None;
        entry.record.state = ReceiptState::AttestedRedacted;
        Ok(())
    }
}
