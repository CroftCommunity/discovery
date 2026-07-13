//! Validation #3 helper: recompute a record's CID exactly as a PDS does —
//! canonical DAG-CBOR encode → sha2-256 → multihash → CIDv1 → base32 (multibase
//! 'b'). If our independently-computed CID matches the server's, the server's
//! CID is a genuine content hash (real content-addressing / tamper-evidence).

use anyhow::Result;
use serde_json::Value;
use sha2::{Digest, Sha256};

const B32_LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz234567";

/// CIDv1 with the dag-cbor codec (0x71) and sha2-256 multihash (0x12), encoded
/// as multibase base32-lower (the form atproto uses: `bafyrei…`).
pub fn cid_v1_dagcbor(value: &Value) -> Result<String> {
    let cbor = serde_ipld_dagcbor::to_vec(value)?;
    let digest = Sha256::digest(&cbor);
    let mut bytes = Vec::with_capacity(36);
    bytes.push(0x01); // CIDv1
    bytes.push(0x71); // codec: dag-cbor
    bytes.push(0x12); // multihash: sha2-256
    bytes.push(0x20); // digest length: 32 bytes
    bytes.extend_from_slice(&digest);
    Ok(format!("b{}", base32_lower(&bytes)))
}

/// RFC 4648 base32 (lower-case alphabet), no padding — as used by multibase 'b'.
fn base32_lower(data: &[u8]) -> String {
    let mut out = String::new();
    let mut buf: u64 = 0;
    let mut bits: u32 = 0;
    for &b in data {
        buf = (buf << 8) | b as u64;
        bits += 8;
        while bits >= 5 {
            bits -= 5;
            out.push(B32_LOWER[((buf >> bits) & 0x1f) as usize] as char);
        }
        buf &= (1u64 << bits) - 1;
    }
    if bits > 0 {
        out.push(B32_LOWER[((buf << (5 - bits)) & 0x1f) as usize] as char);
    }
    out
}
