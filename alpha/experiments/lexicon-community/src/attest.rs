//! The clean-room verifier (and minimal builder) for the CID-First Attestation
//! Specification — inline and remote patterns.
//!
//! Built from `SPEC-BADGE-BLUE.md` alone. Verification is a pure function over
//! bytes plus, for remote, a `Resolver` that fetches the proof record. Each
//! failure has a distinct `VerifyError` variant so the adversarial fixtures can
//! assert the *reason* they fail, not merely that they fail.
//!
//! The verifier optionally enforces two disciplines the spec underdetermines
//! (see AMBIGUITIES.md): low-S canonical signatures (A-3), and issuer↔key
//! binding via the issuer's DID document (A-1). Both default to the strict,
//! safe posture; the lax posture exists to demonstrate the gap under test.

use crate::cidfirst::{attestation_cid, plain_cid};
use crate::didkey::PubKey;
use crate::sign::SignKey;

/// A resolver supplies the two off-record facts remote/binding checks need.
pub trait Resolver {
    /// Fetch a record's JSON value by AT-URI (`at://did/collection/rkey`).
    fn get_record(&self, at_uri: &str) -> Result<serde_json::Value, String>;
    /// The `did:key` public keys authorized by a DID's document
    /// (`verificationMethod`). Empty if the DID cannot be resolved.
    fn authorized_keys(&self, did: &str) -> Vec<String>;
}

/// Verification policy. Defaults (via [`Default`]) are the strict, safe posture.
#[derive(Debug, Clone)]
pub struct VerifyOpts {
    /// Reject high-S (malleable) signatures (spec §6 mandates low-S).
    pub require_low_s: bool,
    /// Require the inline `key` to be listed in the `issuer` DID's document.
    /// This closes A-1 — without it an inline attestation proves only "some key
    /// signed", never "the issuer signed".
    pub require_issuer_binding: bool,
}

impl Default for VerifyOpts {
    fn default() -> Self {
        VerifyOpts { require_low_s: true, require_issuer_binding: true }
    }
}

impl VerifyOpts {
    /// The lax posture a naive reading of the spec yields: verify the embedded
    /// key's signature, check nothing about who that key belongs to.
    pub fn lax() -> Self {
        VerifyOpts { require_low_s: false, require_issuer_binding: false }
    }
}

/// Every distinct way verification can fail — the adversarial fixtures pin these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    NotAnObject,
    NoSignatures,
    MissingField(&'static str),
    UnknownEntryType(String),
    BadCidString(String),
    CidMismatch { claimed: String, recomputed: String },
    KeyResolve(String),
    HighS,
    BadSignature(String),
    IssuerBindingAbsent { key: String, issuer: String },
    MissingIssuer,
    ProofFetch(String),
    ProofCidMismatch { strongref: String, recomputed: String },
    ProofBindingMismatch { proof_cid: String, recomputed: String },
    Convert(String),
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for VerifyError {}

impl From<crate::ipld_json::ConvError> for VerifyError {
    fn from(e: crate::ipld_json::ConvError) -> Self {
        VerifyError::Convert(e.0)
    }
}

/// One verified attestation entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verified {
    Inline { key: String, issuer: Option<String>, cid: String },
    Remote { proof_uri: String, cid: String },
}

fn obj(v: &serde_json::Value) -> Result<&serde_json::Map<String, serde_json::Value>, VerifyError> {
    v.as_object().ok_or(VerifyError::NotAnObject)
}

fn strf(m: &serde_json::Map<String, serde_json::Value>, k: &'static str) -> Result<String, VerifyError> {
    m.get(k)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(VerifyError::MissingField(k))
}

// ---------------------------------------------------------------------------
// Verification
// ---------------------------------------------------------------------------

/// Verify a single inline entry against the record + repository DID.
pub fn verify_inline(
    record: &serde_json::Value,
    entry: &serde_json::Value,
    repository_did: &str,
    resolver: &dyn Resolver,
    opts: &VerifyOpts,
) -> Result<Verified, VerifyError> {
    let e = obj(entry)?;
    let claimed_cid = strf(e, "cid")?;
    let key = strf(e, "key")?;
    let issuer = e.get("issuer").and_then(|v| v.as_str()).map(|s| s.to_string());

    // Rebuild $sig metadata from the entry. We EXCLUDE `cid`, `signature`, AND
    // `key`: the reference impl computes the CID without a `key` field in $sig
    // (it does not even emit one) — so to interoperate, `key` must NOT be part of
    // the signed bytes. This is AMBIGUITIES.md A-2 (spec lists `key` as $sig
    // metadata; reference excludes it) surfaced as a load-bearing decision, and
    // it sharpens A-1: `key` being outside the signature means only the
    // issuer↔key DID-document binding authenticates *who* signed.
    let mut meta = serde_json::Map::new();
    for (k, v) in e {
        if k != "cid" && k != "signature" && k != "key" {
            meta.insert(k.clone(), v.clone());
        }
    }
    let recomputed = attestation_cid(record, &serde_json::Value::Object(meta), repository_did)?;
    if recomputed.to_string_b32() != claimed_cid {
        return Err(VerifyError::CidMismatch {
            claimed: claimed_cid,
            recomputed: recomputed.to_string_b32(),
        });
    }

    // Signature bytes from the `$bytes` wrapper.
    let sig_b64 = e
        .get("signature")
        .and_then(|s| s.get("$bytes"))
        .and_then(|b| b.as_str())
        .ok_or(VerifyError::MissingField("signature.$bytes"))?;
    let sig_bytes = decode_sig(sig_b64)?;

    let pk = PubKey::parse_did_key(&key).map_err(|e| VerifyError::KeyResolve(e.0))?;

    if opts.require_low_s && !pk.is_low_s(&sig_bytes).map_err(|e| VerifyError::KeyResolve(e.0))? {
        return Err(VerifyError::HighS);
    }

    pk.verify_raw(&recomputed.to_bytes(), &sig_bytes)
        .map_err(|e| VerifyError::BadSignature(e.0))?;

    if opts.require_issuer_binding {
        let iss = issuer.clone().ok_or(VerifyError::MissingIssuer)?;
        let authorized = resolver.authorized_keys(&iss);
        if !authorized.iter().any(|k| k == &key) {
            return Err(VerifyError::IssuerBindingAbsent { key: key.clone(), issuer: iss });
        }
    }

    Ok(Verified::Inline { key, issuer, cid: claimed_cid })
}

/// Verify a single remote (strongRef) entry against the source record.
pub fn verify_remote(
    record: &serde_json::Value,
    entry: &serde_json::Value,
    source_repository_did: &str,
    resolver: &dyn Resolver,
) -> Result<Verified, VerifyError> {
    let e = obj(entry)?;
    let uri = strf(e, "uri")?;
    let strongref_cid = strf(e, "cid")?;

    // Fetch the proof record and check its own plain CID matches the strongRef.
    let proof = resolver.get_record(&uri).map_err(VerifyError::ProofFetch)?;
    let proof_plain = plain_cid(&proof)?;
    if proof_plain.to_string_b32() != strongref_cid {
        return Err(VerifyError::ProofCidMismatch {
            strongref: strongref_cid,
            recomputed: proof_plain.to_string_b32(),
        });
    }

    // The proof's `cid` field is the attestation CID over the source record.
    let p = obj(&proof)?;
    let proof_att_cid = strf(p, "cid")?;
    let mut meta = serde_json::Map::new();
    for (k, v) in p {
        if k != "cid" && k != "signature" {
            meta.insert(k.clone(), v.clone());
        }
    }
    let recomputed = attestation_cid(record, &serde_json::Value::Object(meta), source_repository_did)?;
    if recomputed.to_string_b32() != proof_att_cid {
        return Err(VerifyError::ProofBindingMismatch {
            proof_cid: proof_att_cid,
            recomputed: recomputed.to_string_b32(),
        });
    }

    Ok(Verified::Remote { proof_uri: uri, cid: strongref_cid })
}

/// Verify every entry in a record's `signatures` array.
pub fn verify_record(
    record: &serde_json::Value,
    repository_did: &str,
    resolver: &dyn Resolver,
    opts: &VerifyOpts,
) -> Result<Vec<Verified>, VerifyError> {
    let r = obj(record)?;
    let sigs = r.get("signatures").and_then(|v| v.as_array()).ok_or(VerifyError::NoSignatures)?;
    if sigs.is_empty() {
        return Err(VerifyError::NoSignatures);
    }
    let mut out = Vec::new();
    for entry in sigs {
        let ty = entry.get("$type").and_then(|v| v.as_str()).unwrap_or("");
        let v = if ty == "com.atproto.repo.strongRef" {
            verify_remote(record, entry, repository_did, resolver)?
        } else if entry.get("key").is_some() {
            verify_inline(record, entry, repository_did, resolver, opts)?
        } else {
            return Err(VerifyError::UnknownEntryType(ty.to_string()));
        };
        out.push(v);
    }
    Ok(out)
}

fn decode_sig(s: &str) -> Result<Vec<u8>, VerifyError> {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(s)
        .or_else(|_| base64::engine::general_purpose::STANDARD_NO_PAD.decode(s))
        .map_err(|e| VerifyError::BadSignature(format!("base64: {e}")))
}

// ---------------------------------------------------------------------------
// Builders (mint golden fixtures)
// ---------------------------------------------------------------------------

/// Build an inline attestation: returns the record with the entry appended to
/// `signatures`. `metadata` is the extra `$sig` fields (`$type`, `issuer`, …);
/// `key`/`cid`/`signature`/`repository` are filled in.
pub fn build_inline(
    record: &serde_json::Value,
    signer: &SignKey,
    metadata: &serde_json::Value,
    repository_did: &str,
) -> Result<serde_json::Value, VerifyError> {
    // `key` is NOT part of the signed $sig (interop with the reference impl;
    // AMBIGUITIES A-2). Compute the CID over the bare metadata, then emit `key`
    // in the entry only.
    let meta = obj(metadata)?.clone();
    let cid = attestation_cid(record, &serde_json::Value::Object(meta.clone()), repository_did)?;
    let sig = signer.sign_cid(&cid);
    use base64::Engine;
    let sig_b64 = base64::engine::general_purpose::STANDARD.encode(&sig);

    let mut entry = meta;
    entry.insert("key".into(), serde_json::Value::String(signer.did_key()));
    entry.insert("cid".into(), serde_json::Value::String(cid.to_string_b32()));
    let mut sigobj = serde_json::Map::new();
    sigobj.insert("$bytes".into(), serde_json::Value::String(sig_b64));
    entry.insert("signature".into(), serde_json::Value::Object(sigobj));

    let mut out = obj(record)?.clone();
    let arr = out
        .entry("signatures")
        .or_insert_with(|| serde_json::Value::Array(vec![]));
    arr.as_array_mut().unwrap().push(serde_json::Value::Object(entry));
    Ok(serde_json::Value::Object(out))
}

/// Build a remote attestation: returns `(proof_record, attested_record)`.
/// `attestor_rkey` makes the strongRef URI deterministic for fixtures.
pub fn build_remote(
    record: &serde_json::Value,
    metadata: &serde_json::Value,
    source_repository_did: &str,
    attestor_did: &str,
    attestor_rkey: &str,
) -> Result<(serde_json::Value, serde_json::Value), VerifyError> {
    let att_cid = attestation_cid(record, metadata, source_repository_did)?;
    let mut proof = obj(metadata)?.clone();
    proof.insert("cid".into(), serde_json::Value::String(att_cid.to_string_b32()));
    let proof_val = serde_json::Value::Object(proof);

    let proof_plain = plain_cid(&proof_val)?;
    let ty = proof_val.get("$type").and_then(|v| v.as_str()).unwrap_or("com.example.attestation");
    let uri = format!("at://{attestor_did}/{ty}/{attestor_rkey}");

    let mut strongref = serde_json::Map::new();
    strongref.insert("$type".into(), serde_json::Value::String("com.atproto.repo.strongRef".into()));
    strongref.insert("uri".into(), serde_json::Value::String(uri));
    strongref.insert("cid".into(), serde_json::Value::String(proof_plain.to_string_b32()));

    let mut attested = obj(record)?.clone();
    let arr = attested.entry("signatures").or_insert_with(|| serde_json::Value::Array(vec![]));
    arr.as_array_mut().unwrap().push(serde_json::Value::Object(strongref));

    Ok((proof_val, serde_json::Value::Object(attested)))
}
