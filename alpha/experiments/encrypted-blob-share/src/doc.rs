//! The Automerge CRDT document holding the group's shared mutable state.
//!
//! Large immutable binaries live in the iroh-blobs content-addressed store; this
//! document holds only a *reference* to each attachment (its ciphertext hash,
//! the AEAD nonce, mime type, plaintext size, filename). The bytes never live
//! here.
//!
//! ## The 4-tuple address (Willow data model)
//!
//! Conceptually each write to this document carries a
//! `(namespace, subspace, path, timestamp)` address mirroring the Willow data
//! model:
//!   * `namespace` = group id,
//!   * `subspace`  = author identity (e.g. "alice"),
//!   * `path`      = `/attachments/<rkey>`,
//!   * `timestamp` = write time.
//!
//! This experiment does not build a full addressing engine, but
//! [`AttachmentAddress`] records where the attachment reference *would* be
//! addressed.

use anyhow::Context;
use automerge::transaction::Transactable;
use automerge::{Automerge, ObjType, ReadDoc, ScalarValue, Value, ROOT};
use serde::{Deserialize, Serialize};

/// Conceptual Willow-style address for an attachment write. Recorded for
/// documentation; not used as a storage key in this experiment.
#[derive(Debug, Clone)]
pub struct AttachmentAddress {
    pub namespace: String, // group id
    pub subspace: String,  // author identity
    pub path: String,      // /attachments/<rkey>
    pub timestamp: u64,    // write time (unix seconds)
}

impl AttachmentAddress {
    pub fn new(group_id: &str, author: &str, rkey: &str, timestamp: u64) -> Self {
        Self {
            namespace: group_id.to_string(),
            subspace: author.to_string(),
            path: format!("/attachments/{rkey}"),
            timestamp,
        }
    }
}

/// The attachment-reference record.
///
/// Modeled on atproto's own `blob` type convention (atproto blobs carry
/// `$type: "blob"`, a `ref` link, `mimeType`, and `size`) so the shape is
/// structurally familiar. Where we diverge:
///   * our `ref` is the BLAKE3 hash of the *ciphertext*, not the plaintext;
///   * we additionally store the AEAD `nonce` needed to decrypt;
///   * we deliberately do NOT store the decryption key (members hold it via MLS).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AttachmentRef {
    /// Always "blob" — mirrors atproto's blob `$type`.
    #[serde(rename = "$type")]
    pub type_: String,
    /// BLAKE3 hash (hex) of the CIPHERTEXT as stored in iroh-blobs. (atproto's
    /// `ref` is a CID/link; here it is the ciphertext content hash.)
    pub ciphertext_ref: String,
    /// Hex-encoded AEAD nonce. NOT present in atproto blobs — required here
    /// because the content hash is over ciphertext, so the nonce must travel
    /// with the reference.
    pub nonce: String,
    /// MIME type of the (plaintext) media.
    pub mime_type: String,
    /// Size in bytes of the PLAINTEXT (atproto `size` is of the stored blob;
    /// we report plaintext size since that is what the recipient reconstructs).
    pub size: u64,
    /// Optional original filename.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

impl AttachmentRef {
    pub fn new(
        ciphertext_hash_hex: String,
        nonce_hex: String,
        mime_type: &str,
        plaintext_size: u64,
        filename: Option<String>,
    ) -> Self {
        Self {
            type_: "blob".to_string(),
            ciphertext_ref: ciphertext_hash_hex,
            nonce: nonce_hex,
            mime_type: mime_type.to_string(),
            size: plaintext_size,
            filename,
        }
    }
}

/// A tiny wrapper over an Automerge document representing the group's state.
pub struct GroupDoc {
    pub doc: Automerge,
}

impl GroupDoc {
    /// Create an empty document with an `attachments` map at the root.
    pub fn new() -> anyhow::Result<Self> {
        let mut doc = Automerge::new();
        doc.transact::<_, _, automerge::AutomergeError>(|tx| {
            tx.put_object(ROOT, "attachments", ObjType::Map)?;
            Ok(())
        })
        .map_err(|e| anyhow::anyhow!("init attachments map: {:?}", e.error))?;
        Ok(Self { doc })
    }

    /// Insert an attachment reference under `attachments/<rkey>`.
    pub fn put_attachment(&mut self, rkey: &str, att: &AttachmentRef) -> anyhow::Result<()> {
        self.doc
            .transact::<_, _, automerge::AutomergeError>(|tx| {
                let attachments = tx
                    .get(ROOT, "attachments")?
                    .ok_or(automerge::AutomergeError::Fail)?
                    .1;
                let rec = tx.put_object(&attachments, rkey, ObjType::Map)?;
                tx.put(&rec, "$type", att.type_.as_str())?;
                tx.put(&rec, "ciphertext_ref", att.ciphertext_ref.as_str())?;
                tx.put(&rec, "nonce", att.nonce.as_str())?;
                tx.put(&rec, "mime_type", att.mime_type.as_str())?;
                tx.put(&rec, "size", att.size as i64)?;
                if let Some(name) = &att.filename {
                    tx.put(&rec, "filename", name.as_str())?;
                }
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("put attachment: {:?}", e.error))?;
        Ok(())
    }

    /// Read an attachment reference back out of the document.
    pub fn get_attachment(&self, rkey: &str) -> anyhow::Result<AttachmentRef> {
        let (_, attachments) = self
            .doc
            .get(ROOT, "attachments")
            .context("get attachments")?
            .context("attachments map missing")?;
        let (_, rec) = self
            .doc
            .get(&attachments, rkey)
            .context("get attachment record")?
            .with_context(|| format!("attachment {rkey} missing"))?;

        let s = |k: &str| -> anyhow::Result<String> {
            let (v, _) = self
                .doc
                .get(&rec, k)
                .with_context(|| format!("get field {k}"))?
                .with_context(|| format!("field {k} missing"))?;
            match v {
                Value::Scalar(sc) => match sc.as_ref() {
                    ScalarValue::Str(st) => Ok(st.to_string()),
                    other => anyhow::bail!("field {k} not a string: {other:?}"),
                },
                other => anyhow::bail!("field {k} not scalar: {other:?}"),
            }
        };
        let filename = match self.doc.get(&rec, "filename")? {
            Some((Value::Scalar(sc), _)) => match sc.as_ref() {
                ScalarValue::Str(st) => Some(st.to_string()),
                _ => None,
            },
            _ => None,
        };
        let size = {
            let (v, _) = self
                .doc
                .get(&rec, "size")?
                .context("size missing")?;
            match v {
                Value::Scalar(sc) => match sc.as_ref() {
                    ScalarValue::Int(i) => {
                        if *i < 0 {
                            anyhow::bail!("size is negative: {i}");
                        }
                        *i as u64
                    }
                    ScalarValue::Uint(u) => *u,
                    other => anyhow::bail!("size not int: {other:?}"),
                },
                other => anyhow::bail!("size not scalar: {other:?}"),
            }
        };

        Ok(AttachmentRef {
            type_: s("$type")?,
            ciphertext_ref: s("ciphertext_ref")?,
            nonce: s("nonce")?,
            mime_type: s("mime_type")?,
            size,
            filename,
        })
    }

    /// Serialize the document for handoff to another member (stands in for full
    /// CRDT sync — see the README note on document-sync scope).
    pub fn save(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// Load a document handed over by another member.
    pub fn load(bytes: &[u8]) -> anyhow::Result<Self> {
        let doc = Automerge::load(bytes).context("load automerge doc")?;
        Ok(Self { doc })
    }
}
