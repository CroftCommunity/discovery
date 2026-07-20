//! Ambassador store — in-memory (declared stand-in this run).

use crate::records::{EvidenceBody, ReceiptRecord};
use crate::redact::RedactError;
use crate::types::*;

#[derive(Debug, Clone)]
pub struct StoredEntry {
    pub record: ReceiptRecord,
    pub body: Option<EvidenceBody>,
}

#[derive(Debug, Default)]
pub struct EvidenceStore {
    _entries: (),
}

impl EvidenceStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, _record: ReceiptRecord, _body: EvidenceBody) -> ReceiptId {
        unimplemented!("P2 GREEN: insert record + body")
    }

    pub fn receipt(&self, _id: &ReceiptId) -> Option<&ReceiptRecord> {
        unimplemented!("P2 GREEN: fetch receipt by id")
    }

    pub fn body(&self, _id: &ReceiptId) -> Option<&EvidenceBody> {
        unimplemented!("P2 GREEN: fetch body by id (None if redacted)")
    }

    pub fn receipts(&self) -> Box<dyn Iterator<Item = &ReceiptRecord> + '_> {
        unimplemented!("P2 GREEN: iterate all held receipts")
    }

    pub fn len(&self) -> usize {
        unimplemented!("P2 GREEN")
    }
    pub fn is_empty(&self) -> bool {
        unimplemented!("P2 GREEN")
    }

    pub fn redact(&mut self, _id: &ReceiptId) -> Result<(), RedactError> {
        unimplemented!("P4 GREEN: redact body, mark state AttestedRedacted")
    }
}
