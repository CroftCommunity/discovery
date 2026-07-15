//! Real atproto-style content IDs — closing the `cid` gap the Phase 3a README
//! flagged. atproto addresses records by a CIDv1 with the DAG-CBOR codec
//! (0x71) over a SHA-256 multihash of the record's canonical DAG-CBOR encoding.
//! Phase 3a used a `b3-<blake3>` stand-in; here we compute the genuine CID.

use cid::Cid;
use multihash_codetable::{Code, MultihashDigest};
use serde::Serialize;

/// DAG-CBOR multicodec.
const DAG_CBOR: u64 = 0x71;

/// The atproto CIDv1 (DAG-CBOR / SHA-256) of a record, as a `bafy…` string.
pub fn record_cid<T: Serialize>(value: &T) -> String {
    let bytes = serde_ipld_dagcbor::to_vec(value).expect("dag-cbor encode failed");
    let mh = Code::Sha2_256.digest(&bytes);
    Cid::new_v1(DAG_CBOR, mh).to_string()
}

/// True iff `s` parses as a CIDv1 with the DAG-CBOR codec (what atproto uses).
pub fn is_atproto_cid(s: &str) -> bool {
    match Cid::try_from(s) {
        Ok(c) => c.version() == cid::Version::V1 && c.codec() == DAG_CBOR,
        Err(_) => false,
    }
}
