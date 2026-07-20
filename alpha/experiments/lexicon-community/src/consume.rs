//! Live-network consumption (EXP-LEX-02) — parse real records, resolve strong
//! refs, and prove lossless DAG-CBOR round-trips with matching CIDs.
//!
//! The default path is offline: recorded PDS captures under `fixtures/recorded/`
//! (JSON via `getRecord`, plus the authoritative DAG-CBOR block from the CAR).
//! A live fetch is available behind the `LEXCOMM_LIVE=1` flag through a
//! caller-supplied fetcher — the crate itself makes no network calls.

use crate::cidfirst::{dag_cbor_of_json, plain_cid};
use crate::ipld_json::{ipld_to_json, json_to_ipld, ConvError};

/// A `com.atproto.repo.strongRef` (`{uri, cid}`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrongRef {
    pub uri: String,
    pub cid: String,
}

/// Extract a strongRef from a value shaped `{uri, cid}` (e.g. an RSVP `subject`).
pub fn strong_ref(v: &serde_json::Value) -> Option<StrongRef> {
    Some(StrongRef {
        uri: v.get("uri")?.as_str()?.to_string(),
        cid: v.get("cid")?.as_str()?.to_string(),
    })
}

/// Recompute the plain CID of a record's `value` (as returned by `getRecord`)
/// and compare to the authoritative `cid` the PDS reported.
pub fn cid_matches(get_record: &serde_json::Value) -> Result<bool, ConvError> {
    let value = get_record
        .get("value")
        .ok_or_else(|| ConvError("getRecord: no value".into()))?;
    let claimed = get_record
        .get("cid")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ConvError("getRecord: no cid".into()))?;
    Ok(plain_cid(value)?.to_string_b32() == claimed)
}

/// Prove the JSON→IPLD→DAG-CBOR path is byte-identical to the authoritative
/// block bytes the PDS served (extracted from the CAR).
pub fn reserializes_identically(
    get_record: &serde_json::Value,
    authoritative_block: &[u8],
) -> Result<bool, ConvError> {
    let value = get_record
        .get("value")
        .ok_or_else(|| ConvError("getRecord: no value".into()))?;
    Ok(dag_cbor_of_json(value)? == authoritative_block)
}

/// Full IPLD-in-JSON round-trip: JSON → IPLD → JSON is stable, and the second
/// DAG-CBOR encode is byte-identical to the first (idempotent canonical form).
pub fn round_trip_stable(value: &serde_json::Value) -> Result<bool, ConvError> {
    let ipld = json_to_ipld(value)?;
    let back = ipld_to_json(&ipld);
    let reencoded = json_to_ipld(&back)?;
    Ok(serde_ipld_dagcbor::to_vec(&ipld).map_err(|e| ConvError(e.to_string()))?
        == serde_ipld_dagcbor::to_vec(&reencoded).map_err(|e| ConvError(e.to_string()))?)
}
