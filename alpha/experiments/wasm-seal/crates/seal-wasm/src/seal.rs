//! The seal interface — GREEN: the real croft-group L2a stack, re-exported.
//!
//! No re-implementation: `Sealer`/`EpochSecret`/`SealError` are croft-group's
//! `group-seal` types verbatim, the frame is `group-core`'s `ChatMessage`, and
//! the key package is `lineage-mls`'s (openmls's) — the exact stack a native
//! croft client uses, now compiled to and exercised on wasm32-unknown-unknown.
//! The only additions are the two test conveniences the RED stub promised:
//! [`msg`] and [`ciphersuite_name`].

pub use group_core::ChatMessage;
pub use group_seal::{EpochSecret, SealError, Sealer};
pub use lineage_mls::KeyPackage;

/// Build a [`ChatMessage`] (test convenience, stable across RED/GREEN).
#[must_use]
pub fn msg(sender: &str, text: &str) -> ChatMessage {
    ChatMessage {
        sender: sender.to_string(),
        text: text.to_string(),
    }
}

/// The ciphersuite the stack seals under, as a stable name for the P1
/// prediction pin (PRED-CS).
#[must_use]
pub fn ciphersuite_name() -> String {
    format!("{:?}", lineage_mls::CIPHERSUITE)
}
