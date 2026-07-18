//! Validate-before-relay (RUN-16 §A.8; chaining check RUN-18 B1).
//!
//! A relay re-emits an envelope only if it passes its checks against the
//! folded state: the **signature** verifies, the **author is on the roster** of
//! the scope, the **write policy** admits the author, and — in
//! write-restricted scopes (GROUPS.md A.2, reception completeness) — the
//! envelope is **chained**: its FIRST antecedent is the author's previous
//! envelope (the scope genesis for the author's first). An envelope failing
//! any check is dropped and never re-emitted — the relay carries only facts it
//! has itself validated, so a downstream reader inherits validity without
//! re-trusting the sender.
//!
//! This gates message envelopes (the write-policy axis). Governance records
//! (genesis/grant/revocation) are folded, not relayed as chat.

use std::collections::BTreeMap;

use crate::envelope::Envelope;
use crate::fold::FoldState;
use crate::records::{self, Record, WritePolicy};

/// (scope, author) → chain head, as a relay processing a stream tracks it.
type RunningHeads = BTreeMap<(String, String), String>;

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
    /// A write-restricted-scope envelope whose first antecedent is not the
    /// author's previous envelope (or the scope genesis for the first) —
    /// the chaining check, RUN-18 B1.
    Unchained,
}

/// Decide whether the relay accepts `env` given the folded `state`.
///
/// # Errors
/// Returns the specific [`RelayReject`] reason on refusal.
pub fn accepts(state: &FoldState, env: &Envelope) -> Result<(), RelayReject> {
    accepts_with_heads(state, env, &RunningHeads::new())
}

/// The full check, consulting `heads` (chain heads the relay has itself
/// advanced while processing the current stream) before the folded state.
fn accepts_with_heads(
    state: &FoldState,
    env: &Envelope,
    heads: &RunningHeads,
) -> Result<(), RelayReject> {
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
    // own the scope) may post, and the envelope must be chained (RUN-18 B1).
    match cat.write_policy {
        WritePolicy::Open => {
            if state.roster_members(&scope).contains(&env.body.author) {
                Ok(())
            } else {
                Err(RelayReject::NotOnRoster)
            }
        }
        WritePolicy::Single => {
            if env.body.author != cat.owner {
                return Err(RelayReject::WritePolicy);
            }
            let expected = heads
                .get(&(scope.clone(), env.body.author.clone()))
                .cloned()
                .or_else(|| state.chain_head(&scope, &env.body.author))
                .or_else(|| state.genesis_id(&scope));
            if env.body.antecedents.first() == expected.as_ref() {
                Ok(())
            } else {
                Err(RelayReject::Unchained)
            }
        }
    }
}

/// Re-emit only the envelopes the relay accepts, in input order, advancing the
/// author's chain head as chained envelopes pass so a chained stream validates
/// as a stream. Dropped envelopes never appear in the output and never move a
/// head (validate-before-relay).
#[must_use]
pub fn relay(state: &FoldState, incoming: &[Envelope]) -> Vec<Envelope> {
    let mut heads = RunningHeads::new();
    let mut out = Vec::new();
    for env in incoming {
        if accepts_with_heads(state, env, &heads).is_ok() {
            heads.insert(
                (env.body.scope.clone(), env.body.author.clone()),
                env.identity_hex(),
            );
            out.push(env.clone());
        }
    }
    out
}
