//! Experiment-local record shapes (RUN-16 §A.2–A.5).
//!
//! Every fact — genesis, self-registration, request, grant, leave, revocation,
//! role grant/revoke, device attestation, supersession — and every message is
//! carried as the payload of a signed [`Envelope`]. The payload bytes are the
//! canonical encoding of a [`Record`], so a record inherits the envelope's
//! context binding (scope + antecedents) and its `H(envelope)` identity.
//!
//! These are experiment-local NSID-equivalents: clearly test-scoped shapes, not
//! published lexicon schema records (guardrail 5).

use serde::{Deserialize, Serialize};

use crate::envelope::{Envelope, SignedBody};
use crate::identity::Signer;

/// Who may post into a scope (the write-policy axis, §A.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WritePolicy {
    /// Anyone on the roster may post (a forum).
    Open,
    /// Only the scope author/steward may post (a newsletter).
    Single,
}

/// How one joins a scope (the tier axis, §A.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MembershipPolicy {
    /// Self-registration: one signature, the registrant's own (the open tier).
    Open,
    /// Two-sided: a steward grant answers a member request (the gated tier).
    Gated,
}

/// A scope genesis, carrying BOTH policy fields (§A.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genesis {
    /// The scope id this genesis founds.
    pub scope: String,
    /// Human title (owner-editable).
    pub title: String,
    /// Who may post.
    pub write_policy: WritePolicy,
    /// How one joins.
    pub membership_policy: MembershipPolicy,
    /// The steward DIDs (gated tier); empty for an open tier.
    pub steward_set: Vec<String>,
    /// Co-sign threshold over the steward set (gated tier); 0 for open.
    pub threshold: u32,
}

/// A record: one typed fact, carried as an envelope payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Record {
    /// Founds a scope with its policy fields.
    Genesis(Genesis),
    /// The open tier: the author registers themselves.
    SelfRegistration {
        /// The scope joined.
        scope: String,
    },
    /// The gated tier: the author asks to join.
    Request {
        /// The scope requested.
        scope: String,
    },
    /// The gated tier (R7 shape): a steward admits `subject`. The answered
    /// request's identity is among the envelope's antecedents.
    Grant {
        /// The scope.
        scope: String,
        /// The admitted member's DID.
        subject: String,
    },
    /// The author leaves (also expressed as deletion of their registration).
    Leave {
        /// The scope left.
        scope: String,
    },
    /// A causal cut: `subject` is revoked; messages with antecedents at/after
    /// this envelope's position are rejected, earlier ones still verify.
    Revocation {
        /// The scope.
        scope: String,
        /// The revoked member's DID.
        subject: String,
    },
    /// Grant a role to a subject.
    RoleGrant {
        /// The scope.
        scope: String,
        /// The subject DID.
        subject: String,
        /// The role name.
        role: String,
    },
    /// Revoke a role from a subject.
    RoleRevoke {
        /// The scope.
        scope: String,
        /// The subject DID.
        subject: String,
        /// The role name.
        role: String,
    },
    /// Delegate a device signing key for the author (§A.5, P5).
    DeviceAttestation {
        /// The scope (or `"*"` for account-wide).
        scope: String,
        /// The delegated device DID (a `did:key`).
        device_did: String,
    },
    /// Tier transition as re-plant (§ P7): a governed successor whose lineage
    /// names the predecessor genesis.
    Supersession {
        /// The scope.
        scope: String,
        /// The predecessor genesis identity.
        predecessor: String,
        /// The new write policy.
        write_policy: WritePolicy,
        /// The new membership policy.
        membership_policy: MembershipPolicy,
    },
    /// An application message (the payload the write-policy axis gates).
    Message {
        /// The scope posted into.
        scope: String,
        /// The message text.
        text: String,
    },
}

impl Record {
    /// The scope this record acts on.
    #[must_use]
    pub fn scope(&self) -> &str {
        match self {
            Record::Genesis(g) => &g.scope,
            Record::SelfRegistration { scope }
            | Record::Request { scope }
            | Record::Grant { scope, .. }
            | Record::Leave { scope }
            | Record::Revocation { scope, .. }
            | Record::RoleGrant { scope, .. }
            | Record::RoleRevoke { scope, .. }
            | Record::DeviceAttestation { scope, .. }
            | Record::Supersession { scope, .. }
            | Record::Message { scope, .. } => scope,
        }
    }
}

/// Seal a record into an envelope: payload = canonical(record), scope =
/// record.scope, author = signer.did, antecedents = the causal position.
///
/// # Panics
/// Never in practice: a `Record` of owned data always encodes.
#[must_use]
pub fn seal(signer: &Signer, antecedents: Vec<String>, record: &Record) -> Envelope {
    let payload =
        crate::canonical::to_canonical(record).expect("a Record of owned data always encodes");
    let body = SignedBody {
        scope: record.scope().to_string(),
        author: signer.did(),
        antecedents,
        payload,
    };
    Envelope::seal(body, signer).expect("body of owned data always encodes")
}

/// Decode the record from an envelope payload.
///
/// # Errors
/// Returns the ciborium error string if the payload is not a valid `Record`.
pub fn decode(env: &Envelope) -> Result<Record, String> {
    ciborium::from_reader(env.body.payload.as_slice()).map_err(|e| e.to_string())
}
