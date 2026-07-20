//! HTTP-signature verification for inbound ActivityPub activities.
//!
//! Mastodon-shaped signature (draft-cavage-http-signatures): the sender
//! signs a canonical string built from `(request-target)`, `host`, `date`,
//! `digest`, and any additional `headers`-list entries; algorithm =
//! `rsa-sha256`; digest = `SHA-256=<base64>` of the raw body.
//!
//! Distinct error variants (RUN-14 EXP-A style, no collapse — P1 T-AP1.5):
//! - `SignatureMismatch`   — key resolves, digest ok, but signature does not verify
//! - `KeyResolutionFailed` — no key for keyId in the resolver's map
//! - `DigestMismatch`      — `Digest` header value ≠ SHA-256(body)
//! - `MalformedActivity`   — required signature-header parse failed, or
//!   the AP JSON body is not a well-formed activity
//! - `EvidenceRedacted`    — attempted verify on a redacted record (AP-V3);
//!   set by `redact.rs`, never returned by first verify (P4 T-AP4.3 pins
//!   the distinction).
//!
//! No variant collapses into another (T-AP1.5).

use std::collections::BTreeMap;

use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rsa::pkcs1v15::{Signature as RsaSignature, VerifyingKey};
use rsa::pkcs8::DecodePublicKey;
use rsa::signature::Verifier;
use rsa::RsaPublicKey;
use sha2::{Digest, Sha256};

use crate::types::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyError {
    /// The signature is well-formed and the digest matches the body, but the
    /// RSA verification against the resolved key failed.
    SignatureMismatch,
    /// The `keyId` field of the Signature header did not resolve to any
    /// public key via the provided resolver.
    KeyResolutionFailed,
    /// The `Digest` header value does not equal SHA-256 of the raw body.
    DigestMismatch,
    /// The AP JSON is not a well-formed activity (missing / wrong-typed
    /// required fields), or a required signature header is malformed / absent.
    MalformedActivity(String),
    /// The record's evidence has been redacted (AP-V3); no verification is
    /// possible. Distinct from `SignatureMismatch` — the degradation is
    /// honest, not an error masquerade.
    EvidenceRedacted,
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::SignatureMismatch => write!(f, "signature does not verify"),
            VerifyError::KeyResolutionFailed => write!(f, "key resolution failed"),
            VerifyError::DigestMismatch => write!(f, "digest header does not match body"),
            VerifyError::MalformedActivity(m) => write!(f, "malformed activity: {}", m),
            VerifyError::EvidenceRedacted => write!(f, "evidence redacted"),
        }
    }
}
impl std::error::Error for VerifyError {}

/// A key resolver: keyId → SPKI-DER public-key bytes. The runtime shape;
/// tests use `fixtures::FixtureKeyResolver`.
pub trait KeyResolver {
    fn resolve(&self, key_id: &KeyId) -> Option<Vec<u8>>;
}

/// A parsed Signature header — the fields Mastodon-style signatures carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureHeader {
    pub key_id: KeyId,
    pub algorithm: String,
    pub headers: Vec<String>, // e.g. ["(request-target)", "host", "date", "digest"]
    pub signature_b64: String,
}

/// A verified activity — the parsed AP activity plus the pinned public key
/// bytes at verify time (which the receipt captures in its evidence body).
#[derive(Debug, Clone)]
pub struct VerifiedActivity {
    pub activity: InboundActivity,
    pub actor_key_spki_der: Vec<u8>,
    pub actor_key_id: KeyId,
}

/// Parse a Mastodon-shaped Signature header value. On any missing field or
/// bad quoting, returns `MalformedActivity`.
pub fn parse_signature_header(value: &str) -> Result<SignatureHeader, VerifyError> {
    // Format: keyId="…",algorithm="rsa-sha256",headers="(request-target) host date digest",signature="…"
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut rest = value.trim();
    while !rest.is_empty() {
        let eq = rest
            .find('=')
            .ok_or_else(|| VerifyError::MalformedActivity("signature header: no =".into()))?;
        let key = rest[..eq].trim().to_string();
        let after_eq = &rest[eq + 1..];
        let after_eq = after_eq.trim_start();
        let val;
        let consumed;
        if let Some(stripped) = after_eq.strip_prefix('"') {
            let end = stripped
                .find('"')
                .ok_or_else(|| VerifyError::MalformedActivity("signature header: unclosed quote".into()))?;
            val = stripped[..end].to_string();
            consumed = 1 + end + 1; // opening quote + content + closing quote
        } else {
            // unquoted (unusual but tolerated)
            let end = after_eq.find(',').unwrap_or(after_eq.len());
            val = after_eq[..end].trim().to_string();
            consumed = end;
        }
        map.insert(key, val);
        rest = &after_eq[consumed..];
        rest = rest.trim_start();
        if let Some(stripped) = rest.strip_prefix(',') {
            rest = stripped.trim_start();
        }
    }
    let key_id = map
        .remove("keyId")
        .ok_or_else(|| VerifyError::MalformedActivity("signature header: missing keyId".into()))?;
    let algorithm = map
        .remove("algorithm")
        .ok_or_else(|| VerifyError::MalformedActivity("signature header: missing algorithm".into()))?;
    let headers = map
        .remove("headers")
        .ok_or_else(|| VerifyError::MalformedActivity("signature header: missing headers".into()))?;
    let signature_b64 = map
        .remove("signature")
        .ok_or_else(|| VerifyError::MalformedActivity("signature header: missing signature".into()))?;
    Ok(SignatureHeader {
        key_id: KeyId::new(&key_id),
        algorithm,
        headers: headers.split_whitespace().map(|s| s.to_string()).collect(),
        signature_b64,
    })
}

/// Build the canonical signing string per Mastodon convention.
pub fn build_signing_string(
    req: &SignedRequest,
    covered: &[String],
) -> Result<String, VerifyError> {
    let mut lines = Vec::with_capacity(covered.len());
    for name in covered {
        let lc = name.to_ascii_lowercase();
        if lc == "(request-target)" {
            lines.push(format!(
                "(request-target): {} {}",
                req.method.to_ascii_lowercase(),
                req.path
            ));
        } else {
            let v = req.header(&lc).ok_or_else(|| {
                VerifyError::MalformedActivity(format!("covered header missing: {lc}"))
            })?;
            lines.push(format!("{}: {}", lc, v));
        }
    }
    Ok(lines.join("\n"))
}

/// Parse the AP JSON body into an `InboundActivity`. Returns
/// `MalformedActivity` on any structural mismatch.
pub fn parse_ap_activity(raw_body: &[u8]) -> Result<InboundActivity, VerifyError> {
    // Minimal JSON parsing: we only need `type`, `actor`, `object`, `id`.
    // A tiny hand-written parser keeps the dep surface minimal (and any
    // parse failure lands in the MalformedActivity variant).
    let text = std::str::from_utf8(raw_body)
        .map_err(|_| VerifyError::MalformedActivity("body is not utf-8".into()))?;
    let obj = parse_json_object(text)
        .ok_or_else(|| VerifyError::MalformedActivity("body is not a JSON object".into()))?;
    let kind_str = obj
        .get("type")
        .ok_or_else(|| VerifyError::MalformedActivity("activity missing type".into()))?;
    let kind = match kind_str.as_str() {
        "Follow" => ActivityKind::Follow,
        "Undo" => ActivityKind::UndoFollow,
        "Delete" => ActivityKind::Delete,
        other => {
            return Err(VerifyError::MalformedActivity(format!(
                "unsupported activity type: {other}"
            )))
        }
    };
    let actor = obj
        .get("actor")
        .ok_or_else(|| VerifyError::MalformedActivity("activity missing actor".into()))?
        .clone();
    let activity_id = obj
        .get("id")
        .ok_or_else(|| VerifyError::MalformedActivity("activity missing id".into()))?
        .clone();

    // For Follow: object is a URL string. For UndoFollow: object is a nested
    // Follow object with its own id. For Delete: object is a URL string
    // (actor or object being deleted). We parse conservatively.
    let (object, undoes) = match kind {
        ActivityKind::Follow | ActivityKind::Delete => {
            let o = obj
                .get("object")
                .ok_or_else(|| VerifyError::MalformedActivity("activity missing object".into()))?
                .clone();
            (o, None)
        }
        ActivityKind::UndoFollow => {
            let raw = obj.get("object_raw").cloned();
            match raw {
                Some(nested_text) => {
                    let nested = parse_json_object(&nested_text).ok_or_else(|| {
                        VerifyError::MalformedActivity("Undo.object not a JSON object".into())
                    })?;
                    let nested_id = nested.get("id").cloned().ok_or_else(|| {
                        VerifyError::MalformedActivity("Undo.object missing id".into())
                    })?;
                    let nested_object = nested.get("object").cloned().unwrap_or_default();
                    (nested_object, Some(nested_id))
                }
                None => {
                    return Err(VerifyError::MalformedActivity(
                        "Undo requires an object".into(),
                    ))
                }
            }
        }
    };

    Ok(InboundActivity {
        raw_body: raw_body.to_vec(),
        kind,
        actor: ActorId::new(&actor),
        object,
        activity_id,
        undoes,
    })
}

/// The entry point (P1). Verify an incoming HTTP-signed AP request against
/// the resolver's key material, and return the parsed activity + pinned key
/// bytes on success. All four tamper variants map to a distinct error
/// (T-AP1.5 no-collapse).
pub fn verify_ap_http_signature<R: KeyResolver>(
    req: &SignedRequest,
    resolver: &R,
) -> Result<VerifiedActivity, VerifyError> {
    let sig_header_value = req
        .header("signature")
        .ok_or_else(|| VerifyError::MalformedActivity("missing Signature header".into()))?;
    let sig = parse_signature_header(sig_header_value)?;
    if sig.algorithm != "rsa-sha256" {
        return Err(VerifyError::MalformedActivity(format!(
            "unsupported algorithm: {}",
            sig.algorithm
        )));
    }

    // 1) Digest check must precede signature — a matching signature over a
    //    canonical string that includes a wrong digest is still a
    //    DigestMismatch (the *body* is tampered), and per §P1 no variant
    //    collapses: an on-purpose mismatched-digest fixture must return
    //    DigestMismatch, not SignatureMismatch. See T-AP1.3.
    let digest_header = req
        .header("digest")
        .ok_or_else(|| VerifyError::MalformedActivity("missing Digest header".into()))?;
    if !digest_matches(digest_header, &req.body) {
        return Err(VerifyError::DigestMismatch);
    }

    // 2) Resolve the actor key.
    let spki = resolver
        .resolve(&sig.key_id)
        .ok_or(VerifyError::KeyResolutionFailed)?;

    // 3) Build the signing string and RSA-verify.
    let signing_string = build_signing_string(req, &sig.headers)?;
    let sig_bytes = B64
        .decode(sig.signature_b64.as_bytes())
        .map_err(|_| VerifyError::MalformedActivity("signature not base64".into()))?;
    let pub_key = RsaPublicKey::from_public_key_der(&spki)
        .map_err(|_| VerifyError::MalformedActivity("actor key not SPKI DER".into()))?;
    let verifying_key = VerifyingKey::<Sha256>::new(pub_key);
    let signature = RsaSignature::try_from(sig_bytes.as_slice())
        .map_err(|_| VerifyError::SignatureMismatch)?;
    verifying_key
        .verify(signing_string.as_bytes(), &signature)
        .map_err(|_| VerifyError::SignatureMismatch)?;

    // 4) Parse the AP activity now that the byte-signature is confirmed.
    let activity = parse_ap_activity(&req.body)?;

    Ok(VerifiedActivity {
        activity,
        actor_key_spki_der: spki,
        actor_key_id: sig.key_id,
    })
}

fn digest_matches(header_value: &str, body: &[u8]) -> bool {
    // "SHA-256=<b64>"
    let Some((algo, b64)) = header_value.split_once('=') else {
        return false;
    };
    if !algo.eq_ignore_ascii_case("SHA-256") {
        return false;
    }
    let Ok(expect) = B64.decode(b64.as_bytes()) else {
        return false;
    };
    let got = Sha256::digest(body);
    expect.as_slice() == got.as_slice()
}

/// A tiny JSON-object parser sufficient for AP activities' top-level fields.
///
/// Handles: `{ "k": "s", "k": "s" }` where values are strings OR nested
/// objects (nested objects are captured as raw substrings so we can re-parse
/// them for Undo's nested Follow). Whitespace-tolerant. Returns `None` on any
/// structural surprise (numbers, arrays, escapes beyond `\"` and `\\` are
/// not needed here and refuse rather than mislead).
///
/// For nested objects the parent stores the substring under the key with
/// `"_raw"` appended (so `object` becomes `object_raw`), so a caller who
/// wanted a string value cannot silently be given an object.
pub(crate) fn parse_json_object(text: &str) -> Option<BTreeMap<String, String>> {
    let bytes = text.as_bytes();
    let mut i = 0;
    fn skip_ws(bytes: &[u8], i: &mut usize) {
        while *i < bytes.len() {
            match bytes[*i] {
                b' ' | b'\t' | b'\n' | b'\r' => *i += 1,
                _ => break,
            }
        }
    }
    fn parse_string(bytes: &[u8], i: &mut usize) -> Option<String> {
        if *i >= bytes.len() || bytes[*i] != b'"' {
            return None;
        }
        *i += 1;
        let mut out = String::new();
        while *i < bytes.len() {
            match bytes[*i] {
                b'"' => {
                    *i += 1;
                    return Some(out);
                }
                b'\\' => {
                    *i += 1;
                    if *i >= bytes.len() {
                        return None;
                    }
                    match bytes[*i] {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        _ => return None,
                    }
                    *i += 1;
                }
                _ => {
                    out.push(bytes[*i] as char);
                    *i += 1;
                }
            }
        }
        None
    }
    fn parse_object_raw(bytes: &[u8], i: &mut usize) -> Option<String> {
        // Return the raw substring of an object literal, `{` through `}`.
        if *i >= bytes.len() || bytes[*i] != b'{' {
            return None;
        }
        let start = *i;
        let mut depth = 0i32;
        let mut in_string = false;
        while *i < bytes.len() {
            let c = bytes[*i];
            if in_string {
                if c == b'\\' {
                    *i += 2;
                    continue;
                }
                if c == b'"' {
                    in_string = false;
                }
                *i += 1;
                continue;
            }
            match c {
                b'"' => in_string = true,
                b'{' => depth += 1,
                b'}' => {
                    depth -= 1;
                    if depth == 0 {
                        *i += 1;
                        return Some(std::str::from_utf8(&bytes[start..*i]).ok()?.to_string());
                    }
                }
                _ => {}
            }
            *i += 1;
        }
        None
    }

    skip_ws(bytes, &mut i);
    if i >= bytes.len() || bytes[i] != b'{' {
        return None;
    }
    i += 1;
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    loop {
        skip_ws(bytes, &mut i);
        if i < bytes.len() && bytes[i] == b'}' {
            return Some(out);
        }
        let key = parse_string(bytes, &mut i)?;
        skip_ws(bytes, &mut i);
        if i >= bytes.len() || bytes[i] != b':' {
            return None;
        }
        i += 1;
        skip_ws(bytes, &mut i);
        if i >= bytes.len() {
            return None;
        }
        if bytes[i] == b'"' {
            let v = parse_string(bytes, &mut i)?;
            out.insert(key, v);
        } else if bytes[i] == b'{' {
            let raw = parse_object_raw(bytes, &mut i)?;
            out.insert(format!("{key}_raw"), raw);
        } else if bytes[i] == b'[' {
            // Skip past the balanced array — its contents don't participate
            // in the top-level fields we care about (`type`, `actor`,
            // `object`, `id`), but the parser must not choke on it. This
            // is required for Delete(Actor)'s `"to":[…]` field per
            // fed-shim specimen `mastodon-delete-actor-observed-shape.md`.
            let mut depth = 0i32;
            let mut in_string = false;
            while i < bytes.len() {
                let c = bytes[i];
                if in_string {
                    if c == b'\\' {
                        i += 2;
                        continue;
                    }
                    if c == b'"' {
                        in_string = false;
                    }
                    i += 1;
                    continue;
                }
                match c {
                    b'"' => in_string = true,
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if depth != 0 {
                return None;
            }
            // Do NOT record the array under `key` — the caller doesn't
            // need it, and recording under a stringly key would look
            // like a string field to consumers of `parse_json_object`.
        } else {
            return None;
        }
        skip_ws(bytes, &mut i);
        if i < bytes.len() && bytes[i] == b',' {
            i += 1;
            continue;
        }
        // let the next loop iteration match `}`
    }
}
