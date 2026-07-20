//! CID-first computation — the spec's seven-step deterministic algorithm.
//!
//! Reproduced from `SPEC-BADGE-BLUE.md` §1: strip `signatures` → prepare
//! metadata (drop `cid`/`signature`) → add `repository` → insert as `$sig` →
//! DAG-CBOR → SHA-256 → CIDv1(dag-cbor, sha-256). The CID string form is
//! `base32lower` with the multibase `b` prefix. Grounded against real PDS
//! records in EXP-LEX-02 (byte-identical, CID-identical).

use sha2::{Digest, Sha256};

use crate::ipld_json::{json_to_ipld, ConvError};

/// A CIDv1 over DAG-CBOR (codec `0x71`) with a SHA-256 (`0x12`) 32-byte digest —
/// the only CID shape the spec and atproto records use here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordCid {
    digest: [u8; 32],
}

const B32: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

impl RecordCid {
    /// The 4-byte prefix of a CIDv1 dag-cbor sha-256: version, codec, hash fn, len.
    const PREFIX: [u8; 4] = [0x01, 0x71, 0x12, 0x20];

    /// SHA-256 the DAG-CBOR bytes and wrap as a CIDv1.
    pub fn of_dag_cbor(dag_cbor: &[u8]) -> Self {
        let digest: [u8; 32] = Sha256::digest(dag_cbor).into();
        RecordCid { digest }
    }

    /// The full 36-byte binary CID (prefix ‖ digest) — the bytes the spec's
    /// signature covers (our chosen disambiguation; see AMBIGUITIES.md A-4).
    pub fn to_bytes(&self) -> [u8; 36] {
        let mut out = [0u8; 36];
        out[..4].copy_from_slice(&Self::PREFIX);
        out[4..].copy_from_slice(&self.digest);
        out
    }

    /// The `bafy…` base32lower multibase string.
    pub fn to_string_b32(&self) -> String {
        let raw = self.to_bytes();
        let (mut bits, mut val, mut out) = (0u32, 0u32, String::from("b"));
        for &byte in raw.iter() {
            val = (val << 8) | byte as u32;
            bits += 8;
            while bits >= 5 {
                bits -= 5;
                out.push(B32[((val >> bits) & 31) as usize] as char);
            }
        }
        if bits > 0 {
            out.push(B32[((val << (5 - bits)) & 31) as usize] as char);
        }
        out
    }

    /// Parse a `bafy…` string back to a `RecordCid`, validating the prefix.
    pub fn parse(s: &str) -> Result<Self, ConvError> {
        let s = s.strip_prefix('b').ok_or_else(|| ConvError("cid: expected multibase 'b'".into()))?;
        let (mut bits, mut val) = (0u32, 0u32);
        let mut raw = Vec::with_capacity(36);
        for c in s.bytes() {
            let d = B32
                .iter()
                .position(|&x| x == c)
                .ok_or_else(|| ConvError("cid: bad base32 char".into()))? as u32;
            val = (val << 5) | d;
            bits += 5;
            if bits >= 8 {
                bits -= 8;
                raw.push(((val >> bits) & 0xff) as u8);
            }
        }
        if raw.len() != 36 || raw[..4] != Self::PREFIX {
            return Err(ConvError("cid: not a v1 dag-cbor sha-256 CID".into()));
        }
        let mut digest = [0u8; 32];
        digest.copy_from_slice(&raw[4..]);
        Ok(RecordCid { digest })
    }
}

impl std::fmt::Display for RecordCid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_b32())
    }
}

/// DAG-CBOR bytes of an IPLD-in-JSON value (used for record and proof-record CIDs).
pub fn dag_cbor_of_json(v: &serde_json::Value) -> Result<Vec<u8>, ConvError> {
    let ipld = json_to_ipld(v)?;
    serde_ipld_dagcbor::to_vec(&ipld).map_err(|e| ConvError(format!("dag-cbor: {e}")))
}

/// The plain DAG-CBOR CID of a record as-is (no `$sig` injection) — the CID a PDS
/// assigns. This is what EXP-LEX-02 checks and what a remote proof record's own
/// `cid` in a strongRef refers to.
pub fn plain_cid(record: &serde_json::Value) -> Result<RecordCid, ConvError> {
    Ok(RecordCid::of_dag_cbor(&dag_cbor_of_json(record)?))
}

/// The seven-step attestation CID over `(record, metadata, repository_did)`.
///
/// `metadata` is the `$sig` object minus the auto/stripped fields; we clone and
/// enforce the spec's stripping so callers can pass a full entry or a bare
/// metadata object interchangeably.
pub fn attestation_cid(
    record: &serde_json::Value,
    metadata: &serde_json::Value,
    repository_did: &str,
) -> Result<RecordCid, ConvError> {
    let mut rec = record.clone();
    let obj = rec
        .as_object_mut()
        .ok_or_else(|| ConvError("record must be a JSON object".into()))?;

    // Step 1 — strip signatures.
    obj.remove("signatures");

    // Steps 2–3 — prepare metadata: drop cid/signature, add repository.
    let mut meta = metadata.clone();
    let mobj = meta
        .as_object_mut()
        .ok_or_else(|| ConvError("metadata must be a JSON object".into()))?;
    mobj.remove("cid");
    mobj.remove("signature");
    mobj.insert(
        "repository".into(),
        serde_json::Value::String(repository_did.to_string()),
    );

    // Step 4 — insert $sig.
    obj.insert("$sig".into(), meta);

    // Steps 5–7.
    Ok(RecordCid::of_dag_cbor(&dag_cbor_of_json(&rec)?))
}
