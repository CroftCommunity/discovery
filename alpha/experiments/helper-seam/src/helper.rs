//! The content helper — a croft-group MEMBER admitted to the scope by grant.
//!
//! social-mapping helper delegation: "a *content* helper that a persona or a
//! k-of-n group deliberately admits to the scope (a search or index helper, for
//! example) may hold clear text, because it holds it by the same grant any member
//! holds keys by, and it is revocable … In neither case does the helper gain
//! authority." This type is that sentence in code: it wraps a `group-seal`
//! `Sealer` (a real MLS member) but exposes ONLY the join + ingest surface —
//! never invite, remove, grant, or any governance knob. Its sole outward write is
//! a `NormalizedEvent` handed to the index (EXP-C step 3, the no-authority check).

use anyhow::{anyhow, Result};
use group_seal::Sealer;

use crate::bridge::NormalizedEvent;

/// A content/index helper. Holds a private MLS member view; its outward surface
/// is `ingest` (decrypt → normalize) and nothing else.
pub struct ContentHelper {
    did: String,
    sealer: Sealer,
}

impl ContentHelper {
    /// Enroll the helper as a prospective member (credential, no group yet — it
    /// joins from a Welcome, exactly as any member does).
    pub fn enroll(did: &str) -> Result<Self> {
        let sealer = Sealer::enroll(did).map_err(|e| anyhow!(e.to_string()))?;
        Ok(Self { did: did.to_string(), sealer })
    }

    /// The helper's key package, so the group authority can admit it by grant.
    pub fn key_package(&self) -> Result<lineage_mls::KeyPackage> {
        self.sealer.key_package().map_err(|e| anyhow!(e.to_string()))
    }

    /// Accept the Welcome that grants scope access (the admission).
    pub fn accept_welcome(&mut self, welcome: &[u8]) -> Result<()> {
        self.sealer
            .accept_welcome(welcome)
            .map_err(|e| anyhow!(e.to_string()))
    }

    /// Apply a membership control message (an epoch roll the helper is still
    /// entitled to see). Note: this consumes control, it does not ISSUE it — the
    /// helper cannot originate a membership change.
    pub fn apply_control(&mut self, control: &[u8]) -> Result<()> {
        self.sealer
            .apply_control(control)
            .map_err(|e| anyhow!(e.to_string()))
    }

    /// INGEST: open a sealed group frame as the member it is, and normalize the
    /// plaintext into the source-agnostic event the index consumes. Returns Err
    /// if the helper cannot decrypt (e.g. it was revoked and the frame is from a
    /// later epoch — MLS forward secrecy), so a revoked helper produces no rows.
    pub fn ingest(&mut self, group_id: &str, seq: i64, sealed: &[u8]) -> Result<NormalizedEvent> {
        // Open as the member it is — decryption is by the SAME grant any member
        // holds. If the helper has been revoked, this frame is from an epoch it
        // no longer holds a key for and `open` fails (MLS forward secrecy),
        // so a revoked helper yields no event and therefore no index row.
        let msg = self
            .sealer
            .open(sealed)
            .map_err(|e| anyhow!("helper cannot decrypt (revoked / no key): {e}"))?;
        Ok(NormalizedEvent::from_group_message(group_id, seq, &msg.sender, &msg))
    }

    /// The helper's own DID.
    pub fn did(&self) -> &str {
        &self.did
    }
}
