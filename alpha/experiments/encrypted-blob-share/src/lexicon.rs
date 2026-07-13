//! A *targeted* validator for the `com.example.groupshare.attachment` record.
//!
//! The brief calls for "a targeted validator that checks the record before it's
//! accepted" rather than a general-purpose lexicon engine. This checks exactly
//! the constraints the lexicon JSON declares for our record type.

use crate::doc::AttachmentRef;

/// Validate an [`AttachmentRef`] against the lexicon constraints.
/// Returns `Ok(())` if valid, or an error describing the first failure.
pub fn validate(att: &AttachmentRef) -> anyhow::Result<()> {
    // $type const "blob"
    if att.type_ != "blob" {
        anyhow::bail!("$type must be \"blob\", got {:?}", att.type_);
    }
    // ciphertext_ref: 64 lowercase hex chars (BLAKE3-256)
    if att.ciphertext_ref.len() != 64 || !is_lower_hex(&att.ciphertext_ref) {
        anyhow::bail!(
            "ciphertext_ref must be 64 lowercase hex chars, got {} chars",
            att.ciphertext_ref.len()
        );
    }
    // nonce: 24 lowercase hex chars (12 bytes)
    if att.nonce.len() != 24 || !is_lower_hex(&att.nonce) {
        anyhow::bail!(
            "nonce must be 24 lowercase hex chars, got {} chars",
            att.nonce.len()
        );
    }
    // mime_type: non-empty
    if att.mime_type.is_empty() {
        anyhow::bail!("mime_type must be non-empty");
    }
    // size: integer >= 0 (u64 is always >= 0; documented for completeness)
    let _ = att.size;
    Ok(())
}

fn is_lower_hex(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
