//! The seal: real OpenMLS application messages.
//!
//! **Nothing here is stood in.** The methodology's canonical forbidden move is substituting a
//! placeholder cipher for the MLS seal, and every claim in this spike that touches
//! confidentiality or byte-identity runs through this module.
//!
//! Group *construction* is reused from `mls-replant` (the Rung-A ancestor); what is new here
//! is the application-message layer, which `mls-replant` does not have — it was built for the
//! re-plant/re-key mechanics, not for carrying messages.
//!
//! **Deliberate deviation from the plan's stated signature.** The plan wrote
//! `open(...) -> Vec<u8>`. It returns a `Result` instead, because S7 (Carol carries and learns
//! nothing) must record *the named error a non-member receives*, and an unwinding panic cannot
//! be recorded. "It failed" and "it was rejected at <this> point with <this> error" are
//! different security stories, and S7's whole job is to tell them apart.

use mls_replant::Persona;
use openmls::prelude::*;
use tls_codec::{Deserialize as _, Serialize as _};

/// What can go wrong sealing or opening. Carries the library's own error so a caller can
/// report it verbatim rather than paraphrasing it.
#[derive(Debug, thiserror::Error)]
pub enum MlsError {
    /// The sender could not create the application message.
    #[error("create_message failed: {0}")]
    Create(String),
    /// The bytes did not parse as an MLS message.
    #[error("message did not parse as MLS: {0}")]
    Parse(String),
    /// The bytes parsed, but were not a protocol message a group can process.
    #[error("not a processable protocol message: {0}")]
    NotProtocol(String),
    /// The group refused to process the message. **This is the S7 case** — the error text is
    /// the library's own and is recorded, not summarised.
    #[error("process_message refused: {0}")]
    Process(String),
    /// The message processed, but carried something other than application content.
    #[error("expected an application message, got {0}")]
    NotApplication(&'static str),
}

/// The exact library versions this spike resolved, for the result record.
///
/// Hand-written, and therefore guarded: `w1_mls_roundtrip` asserts this string still agrees
/// with `Cargo.toml`, so a bumped pin cannot silently leave a stale banner behind. The
/// methodology requires every result to print exact resolved versions.
#[must_use]
pub fn resolved_versions() -> String {
    [
        "openmls =0.8.1",
        "openmls_rust_crypto =0.5.1",
        "openmls_basic_credential =0.5.0",
        "openmls_traits =0.5.0",
        "tls_codec 0.4",
    ]
    .join(", ")
}

/// Seal `plaintext` to `group` as `sender`, returning the wire bytes.
///
/// These bytes are what the meer stores and forwards, unchanged, and what M2's digest chain
/// is computed over.
///
/// # Errors
/// Returns [`MlsError::Create`] if the library refuses to build the message (e.g. the sender
/// has been evicted, or pending proposals block it).
pub fn seal(group: &mut MlsGroup, sender: &Persona, plaintext: &[u8]) -> Result<Vec<u8>, MlsError> {
    let out = group
        .create_message(&sender.provider, &sender.signer, plaintext)
        .map_err(|e| MlsError::Create(e.to_string()))?;
    out.tls_serialize_detached()
        .map_err(|e| MlsError::Create(e.to_string()))
}

/// Open `sealed` against `group` as `member`, returning the plaintext.
///
/// # Errors
/// Every failure mode is a distinct variant so a caller can say *where* it failed:
/// [`MlsError::Parse`] (not MLS bytes), [`MlsError::NotProtocol`] (MLS, but not processable),
/// [`MlsError::Process`] (the group refused it — S7's case), or
/// [`MlsError::NotApplication`] (processed, but not application content).
pub fn open(
    group: &mut MlsGroup,
    member: &Persona,
    sealed: &[u8],
) -> Result<Vec<u8>, MlsError> {
    let msg = MlsMessageIn::tls_deserialize_exact(sealed)
        .map_err(|e| MlsError::Parse(e.to_string()))?;
    let protocol: ProtocolMessage = msg
        .try_into_protocol_message()
        .map_err(|e| MlsError::NotProtocol(e.to_string()))?;
    let processed = group
        .process_message(&member.provider, protocol)
        .map_err(|e| MlsError::Process(e.to_string()))?;
    match processed.into_content() {
        ProcessedMessageContent::ApplicationMessage(app) => Ok(app.into_bytes()),
        ProcessedMessageContent::ProposalMessage(_) => Err(MlsError::NotApplication("a proposal")),
        ProcessedMessageContent::ExternalJoinProposalMessage(_) => {
            Err(MlsError::NotApplication("an external join proposal"))
        }
        ProcessedMessageContent::StagedCommitMessage(_) => {
            Err(MlsError::NotApplication("a staged commit"))
        }
    }
}

/// **The forbidden move, made available only under the `reframe` feature.**
///
/// Decode an `MlsMessage` and re-encode it without changing semantic content — what a
/// misbehaving forwarder would do. It exists solely so M2 can construct the thing the spec
/// forbids and measure what actually happens.
///
/// Two facts about it, both established by Phase 0's D3 probe and both load-bearing:
///
/// 1. **It does not compile in a default build.** openmls gates
///    `From<MlsMessageIn> for MlsMessageOut` behind `test-utils`
///    (`framing/message_out.rs:195-211`), with the comment *"break abstraction layers and MUST
///    NOT be made available outside of tests"*. The library enforces the property Part 2 §6.6.2
///    states, independently of our discipline.
/// 2. **The result is byte-identical to the input**, because TLS-codec serialization is
///    canonical. Re-framing is therefore not a route to a different-but-valid copy. The
///    dangerous operation is *re-sealing*, which requires a group key.
///
/// # Errors
/// [`MlsError::Parse`] if the input is not an MLS message; [`MlsError::Create`] if
/// re-serialization fails.
#[cfg(feature = "reframe")]
pub fn reframe(sealed: &[u8]) -> Result<Vec<u8>, MlsError> {
    let msg_in = MlsMessageIn::tls_deserialize_exact(sealed)
        .map_err(|e| MlsError::Parse(e.to_string()))?;
    let msg_out: MlsMessageOut = msg_in.into();
    msg_out
        .tls_serialize_detached()
        .map_err(|e| MlsError::Create(e.to_string()))
}
