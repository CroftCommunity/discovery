//! RUN-HIST-02 rev B — hist-atproto live-leg spike.
//!
//! `LiveLeg` abstracts the XRPC surface we exercise (session, repo writes/reads,
//! sync CAR/blocks, blob upload/fetch, DID resolution). Two impls: `StubLeg`
//! (fixture replay, runs in CI) and `LiveLeg` (real HTTP against a bsky-hosted
//! PDS, cred-gated by env). Every call is metered through `Budget` and paced
//! single-flight through `Pacer` — the gentleness contract is enforced by the
//! harness, not by convention.

pub mod budget;
pub mod canonical;
pub mod car;
pub mod did;
pub mod fold;
pub mod leg;
pub mod live;
pub mod record;
pub mod stub;

pub use budget::{Budget, BudgetCaps, BudgetLedger, Pacer};
pub use canonical::{
    canonical_dag_cbor, cid_v1_dag_cbor, dag_cbor_to_atproto_json, ipld_to_json, CidString,
};
pub use car::{parse_car, CarBlock};
pub use fold::{fold_by_antecedent_hashes, fold_by_commit_order_NEGATIVE_CONTROL, FoldState};
pub use leg::{
    ApplyWritesOp, ApplyWritesResp, BlobRef, CommitMeta, GetRecordResp, LiveLegTrait,
    ListRecordsPage, RecordRef, SessionTokens, SyncRecordResp, XrpcError,
};
pub use live::HttpLeg;
pub use record::{HistEntry, Rkey, Subspace, HIST_ENTRY_TYPE};
pub use stub::StubLeg;
