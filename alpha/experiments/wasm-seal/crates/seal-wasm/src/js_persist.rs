//! JS bindings for the persistence-capable member (P3/P5). The encrypted
//! state blob crosses to the JS host as bytes; the HOST owns the at-rest
//! backing (`SPEC-DELTA[run19-storage-shim]` — Node fs here; IndexedDB/OPFS
//! with a WebCrypto-wrapped key in the browser mapping).

use group_core::ChatMessage;
use seal_persist::PersistSealer;
use wasm_bindgen::prelude::*;

use crate::js::{CommitWelcome, OpenedMessage};

fn js_err<E: std::fmt::Display>(e: E) -> JsError {
    JsError::new(&e.to_string())
}

fn key16(key: &[u8]) -> Result<[u8; 16], JsError> {
    key.try_into()
        .map_err(|_| JsError::new("at-rest key must be 16 bytes"))
}

/// A persistence-capable member held inside the wasm module.
#[wasm_bindgen]
pub struct JsPersistSealer {
    inner: PersistSealer,
}

#[wasm_bindgen]
impl JsPersistSealer {
    /// Found a fresh sealed group (genesis).
    pub fn found(did: &str) -> Result<JsPersistSealer, JsError> {
        Ok(Self {
            inner: PersistSealer::found(did).map_err(js_err)?,
        })
    }

    /// Enroll a prospective member.
    pub fn enroll(did: &str) -> Result<JsPersistSealer, JsError> {
        Ok(Self {
            inner: PersistSealer::enroll(did).map_err(js_err)?,
        })
    }

    /// This member's key package as wire bytes.
    pub fn key_package(&self) -> Result<Vec<u8>, JsError> {
        self.inner.key_package().map_err(js_err)
    }

    /// Add one member by wire key package → `(commit, welcome)`.
    pub fn invite(&mut self, kp_bytes: &[u8]) -> Result<CommitWelcome, JsError> {
        let (commit, welcome) = self.inner.invite(kp_bytes).map_err(js_err)?;
        Ok(CommitWelcome::new(commit, welcome))
    }

    /// Join a group from a Welcome.
    pub fn accept_welcome(&mut self, welcome: &[u8]) -> Result<(), JsError> {
        self.inner.accept_welcome(welcome).map_err(js_err)
    }

    /// Seal a chat message → MLS application ciphertext.
    pub fn seal(&mut self, sender: &str, text: &str) -> Result<Vec<u8>, JsError> {
        self.inner
            .seal(&ChatMessage {
                sender: sender.to_string(),
                text: text.to_string(),
            })
            .map_err(js_err)
    }

    /// Open a sealed frame → the chat message.
    pub fn open(&mut self, sealed: &[u8]) -> Result<OpenedMessage, JsError> {
        let m = self.inner.open(sealed).map_err(js_err)?;
        Ok(OpenedMessage::new(m.sender, m.text))
    }

    /// Apply an inbound commit, advancing the epoch.
    pub fn apply_control(&mut self, control: &[u8]) -> Result<(), JsError> {
        self.inner.apply_control(control).map_err(js_err)
    }

    /// Remove a member by DID → the commit to fan out.
    pub fn remove_member(&mut self, did: &str) -> Result<Vec<u8>, JsError> {
        self.inner.remove_member(did).map_err(js_err)
    }

    /// Current epoch number.
    pub fn epoch(&self) -> Result<u64, JsError> {
        self.inner.epoch().map_err(js_err)
    }

    /// The exported per-epoch secret as hex (test comparator only).
    pub fn epoch_secret_hex(&self) -> Result<String, JsError> {
        Ok(hex::encode(self.inner.epoch_secret().map_err(js_err)?))
    }

    /// This member's entire MLS state as an AES-128-GCM encrypted blob.
    pub fn snapshot(&self, key: &[u8]) -> Result<Vec<u8>, JsError> {
        self.inner.snapshot(&key16(key)?).map_err(js_err)
    }

    /// Rebuild a member from an encrypted blob (fails on wrong key /
    /// corrupt / destroyed state — the eviction drill's expected refusal).
    pub fn restore(key: &[u8], blob: &[u8]) -> Result<JsPersistSealer, JsError> {
        Ok(Self {
            inner: PersistSealer::restore(&key16(key)?, blob).map_err(js_err)?,
        })
    }
}
