//! Capstone: verify the cryptographic chain of custody of an atproto repo.
//!
//! An atproto repo is a signed Merkle Search Tree (MST). This module:
//!  1. parses the CAR export (`com.atproto.sync.getRepo`) into blocks,
//!  2. checks every block is content-addressed correctly (sha256(bytes) == CID),
//!  3. decodes the signed root commit,
//!  4. verifies the commit signature against the account's signing key, and
//!  5. walks the MST to collect the record CIDs the signed root commits to.
//!
//! Together that proves a specific record is part of a repo whose signed root
//! chains to a verified identity — fusing identity (V2) and content (V3).

use anyhow::{bail, Context, Result};
use cid::Cid;
use ipld_core::ipld::Ipld;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};

/// Decoded, signature-bearing repo commit.
pub struct Commit {
    pub did: String,
    pub rev: String,
    pub data: Cid, // MST root
    pub sig: Vec<u8>,
    pub unsigned_bytes: Vec<u8>, // canonical DAG-CBOR of the commit without `sig`
}

/// Parse a CAR v1 stream into (roots, blocks). Blocks are keyed by CID.
pub fn parse_car(data: &[u8]) -> Result<(Vec<Cid>, HashMap<Cid, Vec<u8>>)> {
    let mut pos = 0usize;
    let header_len = read_uvarint(data, &mut pos)? as usize;
    let header_bytes = &data[pos..pos + header_len];
    pos += header_len;
    let header: Ipld = serde_ipld_dagcbor::from_slice(header_bytes).context("CAR header")?;
    let mut roots = Vec::new();
    if let Ipld::Map(m) = &header {
        if let Some(Ipld::List(l)) = m.get("roots") {
            for x in l {
                if let Ipld::Link(c) = x {
                    roots.push(*c);
                }
            }
        }
    }
    let mut blocks = HashMap::new();
    while pos < data.len() {
        let block_len = read_uvarint(data, &mut pos)? as usize;
        let end = pos + block_len;
        let (cid, cid_len) = read_cid(&data[pos..end])?;
        blocks.insert(cid, data[pos + cid_len..end].to_vec());
        pos = end;
    }
    Ok((roots, blocks))
}

/// Verify every block hashes to its CID (sha2-256 multihash only). Returns
/// (ok_count, total).
pub fn verify_block_integrity(blocks: &HashMap<Cid, Vec<u8>>) -> (usize, usize) {
    let mut ok = 0;
    for (cid, bytes) in blocks {
        let mh = cid.hash();
        // 0x12 == sha2-256
        if mh.code() == 0x12 && mh.digest() == Sha256::digest(bytes).as_slice() {
            ok += 1;
        }
    }
    (ok, blocks.len())
}

/// Decode the signed root commit, capturing the canonical bytes that were signed.
pub fn decode_commit(root: &Cid, blocks: &HashMap<Cid, Vec<u8>>) -> Result<Commit> {
    let bytes = blocks.get(root).context("commit block missing from CAR")?;
    let ipld: Ipld = serde_ipld_dagcbor::from_slice(bytes).context("decode commit")?;
    let Ipld::Map(m) = ipld else { bail!("commit is not a map") };

    let did = match m.get("did") {
        Some(Ipld::String(s)) => s.clone(),
        _ => bail!("commit.did missing"),
    };
    let rev = match m.get("rev") {
        Some(Ipld::String(s)) => s.clone(),
        _ => bail!("commit.rev missing"),
    };
    let data = match m.get("data") {
        Some(Ipld::Link(c)) => *c,
        _ => bail!("commit.data missing"),
    };
    let sig = match m.get("sig") {
        Some(Ipld::Bytes(b)) => b.clone(),
        _ => bail!("commit.sig missing"),
    };

    // Re-encode the commit without `sig` — this is exactly what was signed.
    let mut unsigned = m.clone();
    unsigned.remove("sig");
    let unsigned_bytes = serde_ipld_dagcbor::to_vec(&Ipld::Map(unsigned)).context("re-encode unsigned commit")?;

    Ok(Commit { did, rev, data, sig, unsigned_bytes })
}

/// Verify the commit signature against a `did:key`-style multibase public key
/// (as found in the DID document's `#atproto` verificationMethod).
pub fn verify_commit_sig(commit: &Commit, public_key_multibase: &str) -> Result<&'static str> {
    verify_signature(&commit.unsigned_bytes, &commit.sig, public_key_multibase)
}

/// Verify an ECDSA signature over `msg` (the verifier hashes with sha2-256)
/// using a multibase `did:key` public key. Supports secp256k1 (multicodec
/// 0xe7) and P-256 (0x1200) — the two curves atproto signing keys use.
pub fn verify_signature(msg: &[u8], sig: &[u8], public_key_multibase: &str) -> Result<&'static str> {
    let (_base, decoded) = multibase::decode(public_key_multibase).context("decode multibase key")?;
    let mut pos = 0usize;
    let codec = read_uvarint(&decoded, &mut pos)?;
    let key_bytes = &decoded[pos..];
    match codec {
        0xe7 => {
            use k256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
            let vk = VerifyingKey::from_sec1_bytes(key_bytes).context("k256 pubkey")?;
            let sig = Signature::from_slice(sig).context("k256 sig")?;
            vk.verify(msg, &sig).context("k256 signature verification failed")?;
            Ok("secp256k1")
        }
        0x1200 => {
            use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
            let vk = VerifyingKey::from_sec1_bytes(key_bytes).context("p256 pubkey")?;
            let sig = Signature::from_slice(sig).context("p256 sig")?;
            vk.verify(msg, &sig).context("p256 signature verification failed")?;
            Ok("p256")
        }
        other => bail!("unsupported signing-key multicodec 0x{other:x}"),
    }
}

/// Walk the MST from `root`, collecting every leaf value CID (i.e. the record
/// CIDs the signed tree commits to).
pub fn collect_mst_values(root: &Cid, blocks: &HashMap<Cid, Vec<u8>>) -> HashSet<Cid> {
    let mut out = HashSet::new();
    let mut seen = HashSet::new();
    walk(root, blocks, &mut out, &mut seen);
    out
}

fn walk(node: &Cid, blocks: &HashMap<Cid, Vec<u8>>, out: &mut HashSet<Cid>, seen: &mut HashSet<Cid>) {
    if !seen.insert(*node) {
        return;
    }
    let Some(bytes) = blocks.get(node) else { return };
    let Ok(Ipld::Map(m)) = serde_ipld_dagcbor::from_slice::<Ipld>(bytes) else { return };
    if let Some(Ipld::Link(l)) = m.get("l") {
        walk(l, blocks, out, seen);
    }
    if let Some(Ipld::List(entries)) = m.get("e") {
        for e in entries {
            if let Ipld::Map(em) = e {
                if let Some(Ipld::Link(v)) = em.get("v") {
                    out.insert(*v);
                }
                if let Some(Ipld::Link(t)) = em.get("t") {
                    walk(t, blocks, out, seen);
                }
            }
        }
    }
}

fn read_uvarint(data: &[u8], pos: &mut usize) -> Result<u64> {
    let mut result: u64 = 0;
    let mut shift = 0;
    loop {
        let byte = *data.get(*pos).context("varint: unexpected end")?;
        *pos += 1;
        result |= ((byte & 0x7f) as u64) << shift;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            bail!("varint too long");
        }
    }
    Ok(result)
}

fn read_cid(buf: &[u8]) -> Result<(Cid, usize)> {
    let mut cur = std::io::Cursor::new(buf);
    let cid = Cid::read_bytes(&mut cur).context("read CID")?;
    Ok((cid, cur.position() as usize))
}
