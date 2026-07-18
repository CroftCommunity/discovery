//! The JS-facing API of the wasm module — what a browser page (here: the
//! Node host, `SPEC-DELTA[run19-node-runner]`) calls. A thin byte-oriented
//! wrapper over the real `group-seal` `Sealer`; commits, welcomes, key
//! packages and sealed frames all cross as `Uint8Array`s.

use crate::seal::{msg, Sealer};
use wasm_bindgen::prelude::*;

fn js_err<E: std::fmt::Display>(e: E) -> JsError {
    JsError::new(&e.to_string())
}

/// The `(commit, welcome)` pair an invite produces.
#[wasm_bindgen]
pub struct CommitWelcome {
    commit: Vec<u8>,
    welcome: Vec<u8>,
}

#[wasm_bindgen]
impl CommitWelcome {
    /// The commit to fan out to existing members.
    #[wasm_bindgen(getter)]
    pub fn commit(&self) -> Vec<u8> {
        self.commit.clone()
    }

    /// The Welcome for the newcomers.
    #[wasm_bindgen(getter)]
    pub fn welcome(&self) -> Vec<u8> {
        self.welcome.clone()
    }
}

/// A decrypted chat message.
#[wasm_bindgen]
pub struct OpenedMessage {
    sender: String,
    text: String,
}

#[wasm_bindgen]
impl OpenedMessage {
    /// Who sent it.
    #[wasm_bindgen(getter)]
    pub fn sender(&self) -> String {
        self.sender.clone()
    }

    /// The body.
    #[wasm_bindgen(getter)]
    pub fn text(&self) -> String {
        self.text.clone()
    }
}

/// One member's sealing view, held inside the wasm module.
#[wasm_bindgen]
pub struct JsSealer {
    inner: Sealer,
}

#[wasm_bindgen]
impl JsSealer {
    /// Found a fresh sealed group (genesis).
    pub fn found(did: &str) -> Result<JsSealer, JsError> {
        Ok(Self {
            inner: Sealer::found(did).map_err(js_err)?,
        })
    }

    /// Enroll a prospective member (no group yet; joins from a Welcome).
    pub fn enroll(did: &str) -> Result<JsSealer, JsError> {
        Ok(Self {
            inner: Sealer::enroll(did).map_err(js_err)?,
        })
    }

    /// This member's key package as wire bytes.
    pub fn key_package(&self) -> Result<Vec<u8>, JsError> {
        let kp = self.inner.key_package().map_err(js_err)?;
        seal_wire::key_package_to_bytes(&kp).map_err(js_err)
    }

    /// Add one member by its wire key package → `(commit, welcome)`.
    pub fn invite(&mut self, kp_bytes: &[u8]) -> Result<CommitWelcome, JsError> {
        let kp = seal_wire::key_package_from_bytes(kp_bytes).map_err(js_err)?;
        let (commit, welcome) = self.inner.invite(&[kp]).map_err(js_err)?;
        Ok(CommitWelcome { commit, welcome })
    }

    /// Join a group from a Welcome.
    pub fn accept_welcome(&mut self, welcome: &[u8]) -> Result<(), JsError> {
        self.inner.accept_welcome(welcome).map_err(js_err)
    }

    /// Seal a chat message → MLS application ciphertext.
    pub fn seal(&mut self, sender: &str, text: &str) -> Result<Vec<u8>, JsError> {
        self.inner.seal(&msg(sender, text)).map_err(js_err)
    }

    /// Open a sealed frame → the chat message.
    pub fn open(&mut self, sealed: &[u8]) -> Result<OpenedMessage, JsError> {
        let m = self.inner.open(sealed).map_err(js_err)?;
        Ok(OpenedMessage {
            sender: m.sender,
            text: m.text,
        })
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

    /// The exported per-epoch secret as hex — a TEST-ONLY exposure for the P2
    /// cross-build state comparison (the I4 comparator); a product API would
    /// never export this.
    pub fn epoch_secret_hex(&self) -> Result<String, JsError> {
        Ok(hex::encode(
            self.inner.epoch_secret().map_err(js_err)?.as_bytes(),
        ))
    }
}
