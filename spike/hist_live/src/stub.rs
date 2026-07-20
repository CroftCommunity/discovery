//! `StubLeg` — deterministic fixture-replay implementation of `LiveLegTrait`.
//!
//! Two roles:
//!  1. In CI (no PDS creds), tests target StubLeg and exercise the fold /
//!     canonical / budget / pacer harness end-to-end.
//!  2. Once the live impl records fixtures (`fixtures/*.json`), tests replay
//!     the recorded responses against StubLeg to keep the assertions live
//!     without hitting the network on every run.
//!
//! The stub charges budget and paces exactly like the live impl — the
//! gentleness contract is exercised by CI so a regression in the counters is
//! caught before it hits the network.

use crate::budget::{Budget, BudgetLedger, Pacer};
use crate::leg::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct StubLeg {
    pub budget: Budget,
    pub pacer: Pacer,
    inner: Mutex<StubState>,
}

struct StubState {
    session: SessionTokens,
    /// (collection, rkey) → (cid, canonical bytes, value json).
    records: HashMap<(String, String), StoredRecord>,
    /// blob cid → bytes.  If `None`, the blob was deleted (E4 semantic).
    blobs: HashMap<String, Option<Vec<u8>>>,
    /// Full-repo CAR, if a test set one up.
    repo_car: Option<Vec<u8>>,
    /// DID docs by DID string.
    did_docs: HashMap<String, serde_json::Value>,
    /// Scripted per-rkey sync.getRecord responses (raw CAR bytes).
    sync_records: HashMap<(String, String), Vec<u8>>,
    describe: Option<serde_json::Value>,
    /// Deliberately-invalid records for E8's oversize / rejection tests.
    xrpc_errors: HashMap<String, (u16, String, String)>,
}

struct StoredRecord {
    cid: String,
    #[allow(dead_code)]
    canonical: Vec<u8>,
    value: serde_json::Value,
}

impl StubLeg {
    pub fn new_with_pacer(pacer: Pacer) -> Self {
        let session = SessionTokens {
            did: "did:plc:stub".to_string(),
            handle: "stub.invalid".to_string(),
            access_jwt: "stub-access".to_string(),
            refresh_jwt: "stub-refresh".to_string(),
            pds_endpoint: "https://stub.invalid".to_string(),
        };
        StubLeg {
            budget: Budget::new(crate::budget::BudgetCaps::GENTLE),
            pacer,
            inner: Mutex::new(StubState {
                session,
                records: HashMap::new(),
                blobs: HashMap::new(),
                repo_car: None,
                did_docs: HashMap::new(),
                sync_records: HashMap::new(),
                describe: None,
                xrpc_errors: HashMap::new(),
            }),
        }
    }

    /// Convenience for tests.
    pub fn new_zero_paced() -> Self {
        Self::new_with_pacer(Pacer::zero())
    }

    pub fn set_repo_car(&self, car: Vec<u8>) {
        self.inner.lock().unwrap().repo_car = Some(car);
    }
    pub fn set_did_doc(&self, did: &str, doc: serde_json::Value) {
        self.inner.lock().unwrap().did_docs.insert(did.to_string(), doc);
    }
    pub fn set_describe(&self, v: serde_json::Value) {
        self.inner.lock().unwrap().describe = Some(v);
    }
    pub fn preload_sync_record(&self, collection: &str, rkey: &str, car: Vec<u8>) {
        self.inner
            .lock()
            .unwrap()
            .sync_records
            .insert((collection.to_string(), rkey.to_string()), car);
    }
    pub fn preload_blob(&self, cid: &str, bytes: Vec<u8>) {
        self.inner
            .lock()
            .unwrap()
            .blobs
            .insert(cid.to_string(), Some(bytes));
    }
    pub fn dereference_blob(&self, cid: &str) {
        // Simulate the GC-after-deref path: mark it as "logically deleted".
        self.inner.lock().unwrap().blobs.insert(cid.to_string(), None);
    }
    pub fn inject_xrpc_error(
        &self,
        method: &str,
        status: u16,
        error: &str,
        message: &str,
    ) {
        self.inner
            .lock()
            .unwrap()
            .xrpc_errors
            .insert(method.to_string(), (status, error.into(), message.into()));
    }
}

impl LiveLegTrait for StubLeg {
    fn session(&self) -> Result<SessionTokens, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        Ok(self.inner.lock().unwrap().session.clone())
    }

    fn describe_repo(&self, _did: &str) -> Result<serde_json::Value, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        self.inner
            .lock()
            .unwrap()
            .describe
            .clone()
            .ok_or_else(|| XrpcError::StubMiss("describe_repo not preloaded".into()))
    }

    fn apply_writes(&self, ops: Vec<ApplyWritesOp>) -> Result<ApplyWritesResp, XrpcError> {
        let n: usize = ops.iter().map(|o| o.write_count()).sum();
        // Charge FIRST — if budget denies, the pacer isn't acquired either.
        self.budget.charge_writes(n)?;
        let _g = self.pacer.acquire();
        if let Some((s, e, m)) = self
            .inner
            .lock()
            .unwrap()
            .xrpc_errors
            .get("com.atproto.repo.applyWrites")
            .cloned()
        {
            return Err(XrpcError::Xrpc {
                http_status: s,
                error: Some(e),
                message: Some(m),
                rate_limit_headers: RateLimitHeaders::default(),
            });
        }
        let mut inner = self.inner.lock().unwrap();
        let mut results = Vec::with_capacity(ops.len());
        for op in ops {
            match op {
                ApplyWritesOp::Create {
                    collection,
                    rkey,
                    value,
                } => {
                    let canonical = crate::canonical::canonical_dag_cbor(&value);
                    let cid = crate::canonical::cid_v1_dag_cbor(&canonical).to_string();
                    let key = (collection.clone(), rkey.clone());
                    inner.records.insert(
                        key,
                        StoredRecord {
                            cid: cid.clone(),
                            canonical,
                            value,
                        },
                    );
                    results.push(serde_json::json!({
                        "$type": "com.atproto.repo.applyWrites#createResult",
                        "uri": format!("at://did:plc:stub/{}/{}", collection, rkey),
                        "cid": cid,
                    }));
                }
                ApplyWritesOp::Delete { collection, rkey } => {
                    inner.records.remove(&(collection.clone(), rkey.clone()));
                    results.push(serde_json::json!({
                        "$type": "com.atproto.repo.applyWrites#deleteResult",
                    }));
                }
            }
        }
        Ok(ApplyWritesResp {
            commit: Some(CommitMeta {
                cid: "bafyrei-stub-commit".into(),
                rev: format!("stub-rev-{}", inner.records.len()),
            }),
            results,
        })
    }

    fn get_record(&self, collection: &str, rkey: &str) -> Result<GetRecordResp, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let inner = self.inner.lock().unwrap();
        let rec = inner
            .records
            .get(&(collection.to_string(), rkey.to_string()))
            .ok_or_else(|| XrpcError::Xrpc {
                http_status: 404,
                error: Some("RecordNotFound".into()),
                message: Some("record not found".into()),
                rate_limit_headers: RateLimitHeaders::default(),
            })?;
        Ok(GetRecordResp {
            uri: format!("at://{}/{}/{}", inner.session.did, collection, rkey),
            cid: rec.cid.clone(),
            value: rec.value.clone(),
        })
    }

    fn sync_get_record(
        &self,
        collection: &str,
        rkey: &str,
    ) -> Result<SyncRecordResp, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let car = self
            .inner
            .lock()
            .unwrap()
            .sync_records
            .get(&(collection.to_string(), rkey.to_string()))
            .cloned()
            .ok_or_else(|| XrpcError::StubMiss(format!("sync_get_record {}/{}", collection, rkey)))?;
        // Best-effort leaf extraction: parse the CAR and pick the block whose
        // dag-cbor decodes to a map containing "$type" == our expected type.
        let parsed = crate::car::parse_car(&car).map_err(XrpcError::Decode)?;
        let mut leaf = None;
        for b in &parsed.blocks {
            if let Ok(v) = serde_ipld_dagcbor::from_slice::<ipld_core::ipld::Ipld>(&b.data) {
                if let ipld_core::ipld::Ipld::Map(m) = &v {
                    if let Some(ipld_core::ipld::Ipld::String(t)) = m.get("$type") {
                        if t == crate::record::HIST_ENTRY_TYPE {
                            leaf = Some((b.cid, b.data.clone()));
                            break;
                        }
                    }
                }
            }
        }
        Ok(SyncRecordResp {
            car_bytes: car,
            leaf_cid: leaf.as_ref().map(|(c, _)| *c),
            leaf_bytes: leaf.map(|(_, d)| d),
        })
    }

    fn list_records(
        &self,
        collection: &str,
        limit: u32,
        cursor: Option<&str>,
        reverse: bool,
    ) -> Result<ListRecordsPage, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        let inner = self.inner.lock().unwrap();
        let mut rks: Vec<&(String, String)> = inner
            .records
            .keys()
            .filter(|(c, _)| c == collection)
            .collect();
        rks.sort_by(|a, b| a.1.cmp(&b.1));
        if reverse {
            rks.reverse();
        }
        // Cursor is the last rkey we returned last time — start AFTER it.
        let start = match cursor {
            None => 0,
            Some(c) => rks
                .iter()
                .position(|(_, r)| r == c)
                .map(|i| i + 1)
                .unwrap_or(rks.len()),
        };
        let end = (start + limit as usize).min(rks.len());
        let page: Vec<RecordRef> = rks[start..end]
            .iter()
            .map(|(collection, rkey)| {
                let rec = inner.records.get(&((*collection).clone(), (*rkey).clone())).unwrap();
                RecordRef {
                    uri: format!("at://{}/{}/{}", inner.session.did, collection, rkey),
                    cid: rec.cid.clone(),
                    value: rec.value.clone(),
                }
            })
            .collect();
        let next = if end < rks.len() {
            Some(rks[end - 1].1.clone())
        } else {
            None
        };
        Ok(ListRecordsPage {
            cursor: next,
            records: page,
        })
    }

    fn upload_blob(&self, _mime: &str, bytes: Vec<u8>) -> Result<BlobRef, XrpcError> {
        self.budget.charge_blob(bytes.len())?;
        let _g = self.pacer.acquire();
        let cid = crate::canonical::cid_v1_dag_cbor(&bytes).to_string();
        self.inner
            .lock()
            .unwrap()
            .blobs
            .insert(cid.clone(), Some(bytes));
        Ok(BlobRef {
            blob: serde_json::json!({
                "$type": "blob",
                "ref": {"$link": cid},
                "mimeType": "application/octet-stream",
            }),
        })
    }

    fn sync_get_blob(&self, _did: &str, cid: &str) -> Result<Option<Vec<u8>>, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        Ok(self
            .inner
            .lock()
            .unwrap()
            .blobs
            .get(cid)
            .cloned()
            .flatten())
    }

    fn sync_get_repo(&self, _did: &str) -> Result<Vec<u8>, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        self.inner
            .lock()
            .unwrap()
            .repo_car
            .clone()
            .ok_or_else(|| XrpcError::StubMiss("sync_get_repo not preloaded".into()))
    }

    fn resolve_did_doc(&self, did: &str) -> Result<serde_json::Value, XrpcError> {
        let _g = self.pacer.acquire();
        self.budget.charge_read();
        self.inner
            .lock()
            .unwrap()
            .did_docs
            .get(did)
            .cloned()
            .ok_or_else(|| XrpcError::StubMiss(format!("did doc for {}", did)))
    }

    fn budget_snapshot(&self) -> BudgetLedger {
        self.budget.snapshot()
    }
}
