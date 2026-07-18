//! Validate-before-relay (RUN-16 §A.8).
//!
//! A relay re-emits an envelope only if it passes three checks against the
//! folded state: the **signature** verifies, the **author is on the roster** of
//! the scope, and the **write policy** admits the author. An envelope failing
//! any check is dropped and never re-emitted — the relay carries only facts it
//! has itself validated, so a downstream reader inherits validity without
//! re-trusting the sender.
//!
//! This gates message envelopes (the write-policy axis). Governance records
//! (genesis/grant/revocation) are folded, not relayed as chat.

use crate::envelope::Envelope;
use crate::fold::FoldState;
use crate::records::{self, Record, WritePolicy};

/// Why the relay refused an envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelayReject {
    /// The signature did not verify (tampered or forged).
    BadSignature,
    /// The payload was not a decodable record.
    Undecodable,
    /// The envelope is not a message (governance records are not relayed here).
    NotAMessage,
    /// The scope has no genesis in the folded state.
    UnknownScope,
    /// The author is not a current roster member of the scope.
    NotOnRoster,
    /// The write policy does not admit this author (e.g. not the single writer).
    WritePolicy,
}

/// Decide whether the relay accepts `env` given the folded `state`.
///
/// # Errors
/// Returns the specific [`RelayReject`] reason on refusal.
pub fn accepts(state: &FoldState, env: &Envelope) -> Result<(), RelayReject> {
    if env.verify().is_err() {
        return Err(RelayReject::BadSignature);
    }
    let rec = records::decode(env).map_err(|_| RelayReject::Undecodable)?;
    let (scope, _text) = match &rec {
        Record::Message { scope, text } => (scope.clone(), text.clone()),
        _ => return Err(RelayReject::NotAMessage),
    };
    let cat = state
        .catalogue
        .get(&scope)
        .ok_or(RelayReject::UnknownScope)?;

    // Write-policy check. Under `Open`, any current roster member may post;
    // under `Single`, only the scope owner (who need not self-register — they
    // own the scope) may post.
    match cat.write_policy {
        WritePolicy::Open => {
            if state.roster_members(&scope).contains(&env.body.author) {
                Ok(())
            } else {
                Err(RelayReject::NotOnRoster)
            }
        }
        WritePolicy::Single => {
            if env.body.author == cat.owner {
                Ok(())
            } else {
                Err(RelayReject::WritePolicy)
            }
        }
    }
}

/// Re-emit only the envelopes the relay accepts, in input order. Dropped
/// envelopes never appear in the output (validate-before-relay).
#[must_use]
pub fn relay(state: &FoldState, incoming: &[Envelope]) -> Vec<Envelope> {
    incoming
        .iter()
        .filter(|e| accepts(state, e).is_ok())
        .cloned()
        .collect()
}
