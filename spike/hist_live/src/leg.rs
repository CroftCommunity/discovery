//! `LiveLegTrait` — the XRPC surface this spike exercises.
//!
//! Every method returns `Result<T, XrpcError>`; the impls (stub, live) meter
//! calls through a shared `Budget` and pace them through a shared `Pacer`.

use crate::budget::BudgetError;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum XrpcError {
    /// The endpoint returned an XRPC-shaped error (`{"error", "message"}`);
    /// http_status may be a rate-limit tell — inspect `rate_limit_headers`.
    Xrpc {
        http_status: u16,
        error: Option<String>,
        message: Option<String>,
        rate_limit_headers: RateLimitHeaders,
    },
    Http(String),
    Budget(BudgetError),
    Decode(String),
    /// A test-mode assertion (stub only): the harness called something the
    /// fixture didn't cover.
    StubMiss(String),
}

impl std::fmt::Display for XrpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            XrpcError::Xrpc {
                http_status,
                error,
                message,
                ..
            } => write!(
                f,
                "xrpc {}: {} — {}",
                http_status,
                error.as_deref().unwrap_or(""),
                message.as_deref().unwrap_or("")
            ),
            XrpcError::Http(s) => write!(f, "http: {}", s),
            XrpcError::Budget(b) => write!(f, "{}", b),
            XrpcError::Decode(s) => write!(f, "decode: {}", s),
            XrpcError::StubMiss(s) => write!(f, "stub miss: {}", s),
        }
    }
}

impl std::error::Error for XrpcError {}

impl From<BudgetError> for XrpcError {
    fn from(b: BudgetError) -> Self {
        XrpcError::Budget(b)
    }
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateLimitHeaders {
    pub limit: Option<String>,
    pub remaining: Option<String>,
    pub reset: Option<String>,
    pub policy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionTokens {
    pub did: String,
    pub handle: String,
    pub access_jwt: String,
    pub refresh_jwt: String,
    /// PDS endpoint the didDoc resolves to; may differ from the entry host.
    pub pds_endpoint: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "$type")]
pub enum ApplyWritesOp {
    #[serde(rename = "com.atproto.repo.applyWrites#create")]
    Create {
        collection: String,
        rkey: String,
        value: serde_json::Value,
    },
    #[serde(rename = "com.atproto.repo.applyWrites#delete")]
    Delete { collection: String, rkey: String },
}

impl ApplyWritesOp {
    /// For the budget: a create OR a delete each counts as one write.
    pub fn write_count(&self) -> usize {
        1
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplyWritesResp {
    pub commit: Option<CommitMeta>,
    #[serde(default)]
    pub results: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMeta {
    pub cid: String,
    pub rev: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetRecordResp {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}

/// Response of `com.atproto.sync.getRecord`: a CAR containing the proof path
/// from the signed commit down to the record leaf.  We expose the raw bytes
/// AND the parsed leaf so tests can compare bytes-for-bytes.
#[derive(Debug, Clone)]
pub struct SyncRecordResp {
    pub car_bytes: Vec<u8>,
    /// The leaf block bytes (dag-cbor of the record).  Extracted from the CAR
    /// by matching the leaf CID that the MST resolves to.
    pub leaf_bytes: Option<Vec<u8>>,
    pub leaf_cid: Option<cid::Cid>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListRecordsPage {
    pub cursor: Option<String>,
    pub records: Vec<RecordRef>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RecordRef {
    pub uri: String,
    pub cid: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlobRef {
    pub blob: serde_json::Value,
}

pub trait LiveLegTrait {
    /// Create session (or return a cached one).  Live impl refreshes when the
    /// accessJwt is close to expiring.
    fn session(&self) -> Result<SessionTokens, XrpcError>;

    fn describe_repo(&self, did: &str) -> Result<serde_json::Value, XrpcError>;

    /// `com.atproto.repo.applyWrites` — the batched write path.  Charges the
    /// budget by the number of ops, single-flighted through the pacer.
    fn apply_writes(&self, ops: Vec<ApplyWritesOp>) -> Result<ApplyWritesResp, XrpcError>;

    /// `com.atproto.repo.getRecord` (JSON path).
    fn get_record(
        &self,
        collection: &str,
        rkey: &str,
    ) -> Result<GetRecordResp, XrpcError>;

    /// `com.atproto.sync.getRecord` (CAR / proof path).
    fn sync_get_record(
        &self,
        collection: &str,
        rkey: &str,
    ) -> Result<SyncRecordResp, XrpcError>;

    /// `com.atproto.repo.listRecords` — cursored.
    fn list_records(
        &self,
        collection: &str,
        limit: u32,
        cursor: Option<&str>,
        reverse: bool,
    ) -> Result<ListRecordsPage, XrpcError>;

    /// `com.atproto.repo.uploadBlob` (returns the blob ref for embedding).
    fn upload_blob(&self, mime: &str, bytes: Vec<u8>) -> Result<BlobRef, XrpcError>;

    /// `com.atproto.sync.getBlob` — returns Some(bytes) if the blob is
    /// retrievable, None if the server returns 404 or a NotFound XRPC error
    /// (the interesting signal for E4's dereference test).
    fn sync_get_blob(&self, did: &str, cid: &str) -> Result<Option<Vec<u8>>, XrpcError>;

    /// `com.atproto.sync.getRepo` — full CAR.
    fn sync_get_repo(&self, did: &str) -> Result<Vec<u8>, XrpcError>;

    /// DID document lookup via the plc directory (or web).
    fn resolve_did_doc(&self, did: &str) -> Result<serde_json::Value, XrpcError>;

    /// Ledger read for the results doc.
    fn budget_snapshot(&self) -> crate::budget::BudgetLedger;

    /// Last rate-limit headers observed on any response.  Default returns
    /// empty (stub); the live impl captures real headers.
    fn last_rate_limit_headers(&self) -> RateLimitHeaders {
        RateLimitHeaders::default()
    }
}
