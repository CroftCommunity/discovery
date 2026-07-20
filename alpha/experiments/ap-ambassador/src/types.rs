//! Types shared across the ambassador crate.
//!
//! Design constraint (AP-V4): none of these types implement any co-sign or
//! vouch antecedent trait, and there is no closed-enum variant in
//! `attest-family` that names an ambassador shape (the P5 structural
//! evidence). The closed `AntecedentKind` enum in attest-family admits no
//! ambassador variant by construction; the boundary is structural.

use std::fmt;

/// The identity of an ambassador receipt envelope — `H(canonical_bytes)`
/// (BLAKE3), the same shape as attest-family's `ObjectId`.
///
/// This is deliberately a **distinct newtype** from `attest_family::ObjectId`:
/// a mismatched-newtype cannot flow into an attest-family antecedent slot by
/// accident, making the P5 role boundary type-checked rather than only
/// gentleman's-agreement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiptId(pub [u8; 32]);

impl fmt::Display for ReceiptId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

/// An ActivityPub actor identity — a URL string treated as bytes. The
/// ambassador uses the actor id as the addressable persona in the receipt;
/// it does not attempt to resolve to a Croft persona (no DID linkage without
/// a P6 dual-proof binding).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ActorId(pub String);

impl ActorId {
    pub fn new(url: &str) -> Self {
        ActorId(url.to_string())
    }
}

/// A `keyId` from an HTTP-signature header — typically an actor URL with a
/// fragment (e.g. `https://mastodon.example/users/alice#main-key`).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyId(pub String);

impl KeyId {
    pub fn new(s: &str) -> Self {
        KeyId(s.to_string())
    }
}

/// The three ambassador-observable AP activity kinds this run supports.
/// (Other inbound kinds — replies, likes — are AP-OC-9, out of scope here.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivityKind {
    /// `Follow` — opens a follower interval when verified.
    Follow,
    /// `Undo Follow` — closes the open interval opened by a matching Follow.
    UndoFollow,
    /// `Delete` — the custom-respect rider (AP-V3): redact the evidence body,
    /// keep the fact skeleton.
    Delete,
}

impl ActivityKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ActivityKind::Follow => "Follow",
            ActivityKind::UndoFollow => "Undo",
            ActivityKind::Delete => "Delete",
        }
    }
}

/// The receipt record's state marker (AP-V2 / AP-V3).
///
/// - `EvidenceComplete` — the full activity JSON, HTTP-signature headers as
///   received, and the pinned actor public key are all held in the store; the
///   receipt is byte-verifiable from those bytes alone.
/// - `AttestedRedacted` — the evidence body has been redacted (AP-V3
///   `Delete` custom rider). The commitment and interval boundaries survive;
///   the receipt is no longer byte-verifiable. Re-verification returns the
///   distinct `EvidenceRedacted` error variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptState {
    EvidenceComplete,
    AttestedRedacted,
}

impl ReceiptState {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReceiptState::EvidenceComplete => "evidence-complete",
            ReceiptState::AttestedRedacted => "attested-redacted",
        }
    }
}

/// An inbound ActivityPub activity as the ambassador received it — raw JSON
/// bytes plus the parsed cross-cutting fields. The raw bytes are what get
/// signed and what get stored in the evidence body; the parsed fields are for
/// dispatch only.
#[derive(Debug, Clone)]
pub struct InboundActivity {
    pub raw_body: Vec<u8>,
    pub kind: ActivityKind,
    /// The actor URL from the activity's `actor` field (the sender).
    pub actor: ActorId,
    /// For Follow / Undo Follow: the object URL being followed (the target).
    /// For Delete: the actor or object URL being deleted (may equal `actor`).
    pub object: String,
    /// The activity's `id` field — used to identify UndoFollow's target
    /// Follow, and Delete's target actor/object.
    pub activity_id: String,
    /// For UndoFollow: the `id` of the Follow being undone (from
    /// `object.id`). For Follow/Delete: `None`.
    pub undoes: Option<String>,
}

/// An HTTP-signature-signed request as the ambassador received it: the
/// canonical method / path / headers / body used to verify.
///
/// The signature scheme is Mastodon-shaped (draft-cavage-http-signatures over
/// `(request-target)`, `host`, `date`, `digest`; RSA-SHA256; keyId points at
/// an actor's `#main-key`). No support for other algorithms this run.
#[derive(Debug, Clone)]
pub struct SignedRequest {
    pub method: String,
    pub path: String,
    /// Header name/value pairs as received (lower-case names). Preserved as a
    /// list, not a map, so the receipt records the arrival order faithfully
    /// (AP-V2 evidence-complete carries the headers as received).
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl SignedRequest {
    /// Convenience accessor: first header with the given (lower-case) name.
    pub fn header<'a>(&'a self, name: &str) -> Option<&'a str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }
}
