//! Stapled status — the missing freshness layer, ported from the attest-family
//! lane (RUN-ATTEST-04 V5/V6) onto the spec's remote-attestation shape.
//!
//! The published attestation spec answers integrity and replay; it says nothing
//! about *freshness/revocation*. The web's answer to "is this still valid?" was
//! OCSP — status-by-callback — which centralizes and surveils (the issuer learns
//! who checks whom). The known cure is **stapling**: the issuer publishes ONE
//! signed tree head per era over keyed commitments (CT / RFC-9162 shape); the
//! holder staples an inclusion proof into its presentation; the verifier checks
//! with **zero callback to the issuer**.
//!
//! Composition with the spec is deliberate: the issuer signs the head record's
//! CID and the holder-binding record's CID with exactly the spec's CID-first
//! ECDSA. A staple is thus two CID-first attestations plus a Merkle audit path —
//! nothing new in the trust base.
//!
//! Design ported (not code): keyed commitment `HMAC-SHA256(key_e, cred_id)` so an
//! outsider cannot join commitments to credentials (T-A4.8); leaves in canonical
//! (byte-ascending) order so mint order is structurally absent (T-A4.7);
//! per-era superseded set for revocation (T-A4.10); the verifier is a pure
//! function with no state parameter (T-A4.11).

use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

use crate::cidfirst::plain_cid;
use crate::didkey::PubKey;
use crate::sign::SignKey;

type HmacSha256 = Hmac<Sha256>;

/// A 32-byte keyed commitment over a credential id.
pub type Commitment = [u8; 32];

fn bytes_val(b: &[u8]) -> serde_json::Value {
    use base64::Engine;
    let mut m = serde_json::Map::new();
    m.insert(
        "$bytes".into(),
        serde_json::Value::String(base64::engine::general_purpose::STANDARD.encode(b)),
    );
    serde_json::Value::Object(m)
}

fn read_bytes(v: &serde_json::Value) -> Option<Vec<u8>> {
    use base64::Engine;
    let s = v.get("$bytes")?.as_str()?;
    base64::engine::general_purpose::STANDARD.decode(s).ok()
}

/// `key_e = KDF(coop_secret, era_anchor)` — the modeled era key (owner-revisitable
/// choice, flagged exactly as in the lane). Here: HMAC-SHA256(coop_secret, era).
pub fn era_key(coop_secret: &[u8], era_anchor: &[u8; 32]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(coop_secret).expect("hmac key");
    mac.update(era_anchor);
    mac.finalize().into_bytes().into()
}

/// `commit = HMAC-SHA256(key_e, credential_cid_bytes)` — keyed so it cannot be
/// dictionary-joined to a credential without the era key.
pub fn commit(key_e: &[u8; 32], credential_cid: &str) -> Commitment {
    let mut mac = HmacSha256::new_from_slice(key_e).expect("hmac key");
    mac.update(credential_cid.as_bytes());
    mac.finalize().into_bytes().into()
}

// ---------------------------------------------------------------------------
// RFC-6962 Merkle tree over commitments (canonical byte-ascending order)
// ---------------------------------------------------------------------------

fn leaf_hash(c: &Commitment) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x00]);
    h.update(c);
    h.finalize().into()
}

fn node_hash(l: &[u8; 32], r: &[u8; 32]) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update([0x01]);
    h.update(l);
    h.update(r);
    h.finalize().into()
}

fn largest_pow2_below(n: usize) -> usize {
    let mut k = 1;
    while k << 1 < n {
        k <<= 1;
    }
    k
}

/// RFC-6962 Merkle Tree Hash over already-sorted leaves.
pub fn merkle_root(leaves: &[Commitment]) -> [u8; 32] {
    match leaves.len() {
        0 => Sha256::digest([]).into(),
        1 => leaf_hash(&leaves[0]),
        n => {
            let k = largest_pow2_below(n);
            node_hash(&merkle_root(&leaves[..k]), &merkle_root(&leaves[k..]))
        }
    }
}

/// RFC-6962 audit path for the leaf at `index`.
pub fn audit_path(leaves: &[Commitment], index: usize) -> Vec<[u8; 32]> {
    let n = leaves.len();
    if n <= 1 {
        return vec![];
    }
    let k = largest_pow2_below(n);
    if index < k {
        let mut p = audit_path(&leaves[..k], index);
        p.push(merkle_root(&leaves[k..]));
        p
    } else {
        let mut p = audit_path(&leaves[k..], index - k);
        p.push(merkle_root(&leaves[..k]));
        p
    }
}

/// Recompute the root from a leaf commitment, its audit path and tree position —
/// the exact RFC-6962 §2.1.1 inclusion-proof verification.
pub fn root_from_path(commitment: &Commitment, index: usize, tree_size: usize, path: &[[u8; 32]]) -> [u8; 32] {
    let mut r = leaf_hash(commitment);
    if tree_size <= 1 {
        return r;
    }
    let (mut fnv, mut sn) = (index, tree_size - 1);
    for p in path {
        if fnv & 1 == 1 || fnv == sn {
            r = node_hash(p, &r);
            if fnv & 1 == 0 {
                while fnv & 1 == 0 && fnv != 0 {
                    fnv >>= 1;
                    sn >>= 1;
                }
            }
        } else {
            r = node_hash(&r, p);
        }
        fnv >>= 1;
        sn >>= 1;
    }
    r
}

// ---------------------------------------------------------------------------
// Signed head + holder binding (each a CID-first signature by the issuer)
// ---------------------------------------------------------------------------

/// One signed tree head per era/epoch. `record` is the published JSON; `sig` is
/// the issuer's CID-first ECDSA over `plain_cid(record)`.
#[derive(Debug, Clone)]
pub struct SignedHead {
    pub record: serde_json::Value,
    pub sig: Vec<u8>,
}

/// Issuer-signed `{credential ↔ commitment}` pairing, holder-held (never
/// published) — lets a verifier map credential→commitment without the era key.
#[derive(Debug, Clone)]
pub struct SignedBinding {
    pub record: serde_json::Value,
    pub sig: Vec<u8>,
}

/// The holder-stapled proof handed to a verifier alongside the credential.
#[derive(Debug, Clone)]
pub struct Staple {
    pub commitment: Commitment,
    pub index: usize,
    pub tree_size: usize,
    pub path: Vec<[u8; 32]>,
    pub binding: SignedBinding,
    pub head: SignedHead,
}

/// Build a signed head record over a set of credential CIDs for one era.
/// `superseded` are credential CIDs revoked this era. Returns the head plus the
/// sorted commitment list (issuer-side; the holder gets only its own path).
pub fn build_head(
    issuer: &SignKey,
    coop_secret: &[u8],
    era_anchor: &[u8; 32],
    epoch: u64,
    credential_cids: &[String],
    superseded_cids: &[String],
) -> (SignedHead, Vec<Commitment>) {
    let key_e = era_key(coop_secret, era_anchor);
    let mut commits: Vec<Commitment> = credential_cids.iter().map(|c| commit(&key_e, c)).collect();
    commits.sort();
    commits.dedup();
    let root = merkle_root(&commits);

    let mut superseded: Vec<Commitment> = superseded_cids.iter().map(|c| commit(&key_e, c)).collect();
    superseded.sort();
    superseded.dedup();
    let superseded_root = merkle_root(&superseded);

    let record = serde_json::json!({
        "$type": "community.lexicon.attest.treeHead",
        "epochNo": epoch,
        "eraAnchor": bytes_val(era_anchor),
        "leafCount": commits.len() as u64,
        "root": bytes_val(&root),
        "superseded": superseded.iter().map(|c| bytes_val(c)).collect::<Vec<_>>(),
        "supersededRoot": bytes_val(&superseded_root),
    });
    let cid = plain_cid(&record).expect("head record encodes");
    let sig = issuer.sign_cid(&cid);
    (SignedHead { record, sig }, commits)
}

/// Build the holder binding for one credential (issuer signs credential↔commitment).
pub fn build_binding(issuer: &SignKey, credential_cid: &str, commitment: &Commitment) -> SignedBinding {
    let record = serde_json::json!({
        "$type": "community.lexicon.attest.holderBinding",
        "credential": credential_cid,
        "commitment": bytes_val(commitment),
    });
    let cid = plain_cid(&record).expect("binding record encodes");
    let sig = issuer.sign_cid(&cid);
    SignedBinding { record, sig }
}

/// Assemble a staple for `credential_cid` given the era's full commitment list.
pub fn build_staple(
    issuer: &SignKey,
    coop_secret: &[u8],
    era_anchor: &[u8; 32],
    credential_cid: &str,
    head: &SignedHead,
    commits: &[Commitment],
) -> Option<Staple> {
    let key_e = era_key(coop_secret, era_anchor);
    let c = commit(&key_e, credential_cid);
    let index = commits.iter().position(|x| x == &c)?;
    let path = audit_path(commits, index);
    let binding = build_binding(issuer, credential_cid, &c);
    Some(Staple {
        commitment: c,
        index,
        tree_size: commits.len(),
        path,
        binding,
        head: head.clone(),
    })
}

// ---------------------------------------------------------------------------
// The pure verifier — zero callback
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StapleError {
    HeadSignature,
    BindingSignature,
    BindingCredentialMismatch,
    BindingCommitmentMismatch,
    Superseded,
    ForgedInclusion,
    Malformed(String),
}

impl std::fmt::Display for StapleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for StapleError {}

fn head_field_bytes(head: &serde_json::Value, k: &str) -> Result<Vec<u8>, StapleError> {
    read_bytes(head.get(k).ok_or_else(|| StapleError::Malformed(format!("head missing {k}")))?)
        .ok_or_else(|| StapleError::Malformed(format!("head {k} not $bytes")))
}

/// Verify a staple with **no network access**: (1) issuer signed the head CID;
/// (2) issuer signed the binding CID and it names this credential+commitment;
/// (3) the commitment is not in this head's superseded set; (4) the audit path
/// reproduces the head's root. Pure over its arguments — there is no resolver,
/// no issuer endpoint, nothing to leak.
pub fn verify_staple(
    credential_cid: &str,
    issuer: &PubKey,
    staple: &Staple,
) -> Result<(), StapleError> {
    // (1) head signature over its CID.
    let head_cid = plain_cid(&staple.head.record).map_err(|e| StapleError::Malformed(e.0))?;
    issuer
        .verify_raw(&head_cid.to_bytes(), &staple.head.sig)
        .map_err(|_| StapleError::HeadSignature)?;

    // (2) binding signature + it binds THIS credential to THIS commitment.
    let bind_cid = plain_cid(&staple.binding.record).map_err(|e| StapleError::Malformed(e.0))?;
    issuer
        .verify_raw(&bind_cid.to_bytes(), &staple.binding.sig)
        .map_err(|_| StapleError::BindingSignature)?;
    let b = &staple.binding.record;
    if b.get("credential").and_then(|v| v.as_str()) != Some(credential_cid) {
        return Err(StapleError::BindingCredentialMismatch);
    }
    let bound_commit = b
        .get("commitment")
        .and_then(read_bytes)
        .ok_or_else(|| StapleError::Malformed("binding commitment".into()))?;
    if bound_commit.as_slice() != staple.commitment.as_slice() {
        return Err(StapleError::BindingCommitmentMismatch);
    }

    // (3) superseded set (inside the signed head).
    let head = &staple.head.record;
    if let Some(sup) = head.get("superseded").and_then(|v| v.as_array()) {
        for s in sup {
            if read_bytes(s).as_deref() == Some(&staple.commitment) {
                return Err(StapleError::Superseded);
            }
        }
    }

    // (4) inclusion: recompute root, compare to the signed head's root.
    let root = head_field_bytes(head, "root")?;
    let recomputed = root_from_path(&staple.commitment, staple.index, staple.tree_size, &staple.path);
    if recomputed.as_slice() != root.as_slice() {
        return Err(StapleError::ForgedInclusion);
    }
    Ok(())
}

/// Freshness policy (verifier-side, fail-closed): the staple's head epoch must be
/// at least `min_epoch`. This is app policy, never a protocol timeout — exactly
/// the lane's posture. A staple against a frozen (older) era fails this when a
/// newer era has rolled.
pub fn is_fresh(staple: &Staple, min_epoch: u64) -> bool {
    staple
        .head
        .record
        .get("epochNo")
        .and_then(|v| v.as_u64())
        .map(|e| e >= min_epoch)
        .unwrap_or(false)
}
