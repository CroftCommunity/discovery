//! Identity and provenance value types shared across the workspace.
//!
//! These are deliberately thin. The lineage DAG / governance log (Phase 2)
//! builds on top of `GenesisId` and `Did`; Phase 1 only needs stable,
//! comparable identifiers and a content-hash helper for genesis anchoring.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A logical device participating in a scenario.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub String);

impl DeviceId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

/// A decentralized identity. A member is a `Did`; it may be realized on
/// several devices (this is what makes total-device-loss recovery, E3.3,
/// even expressible). Phase 1 treats it as an opaque stable string.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Did(pub String);

impl Did {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// MLS credential identity bytes for this DID.
    pub fn credential_identity(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }

    /// A privacy-preserving rendering for error messages and logs. A DID is a
    /// contact identifier, so error strings surface a short stable hash rather
    /// than the raw value, keeping the identifier out of any logs an error is
    /// later written to. The full DID remains available on the typed value.
    pub fn redacted(&self) -> String {
        let mut h = Sha256::new();
        h.update(self.0.as_bytes());
        format!("did:{}", &hex::encode(h.finalize())[..10])
    }
}

/// A 32-byte content hash, hex-rendered for display. Used as the genesis
/// anchor and as a deterministic tiebreak in survivor selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct GenesisId(pub [u8; 32]);

impl GenesisId {
    /// Derive a genesis id by hashing arbitrary canonical bytes.
    pub fn from_bytes(input: &[u8]) -> Self {
        let mut h = Sha256::new();
        h.update(input);
        let out = h.finalize();
        let mut buf = [0u8; 32];
        buf.copy_from_slice(&out);
        Self(buf)
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

/// Domain separator tags for the wire identity derivations (CROFT-PROTOCOL §2).
/// SHA-256 over `tag ‖ id`. The tag is the version + domain separator, so a hash
/// input for one identifier kind can never collide with another's (or another
/// protocol's). This is the canonical home for the derivations the
/// `altdrive-spike-lineage-sync` spike pioneered; the spike's inline form is
/// byte-identical to these.
const LINEAGE_GENESIS_TAG: &[u8] = b"croft-lineage-genesis:";
const GROUP_GENESIS_TAG: &[u8] = b"croft-group-genesis:";
const GROUP_TOPIC_TAG: &[u8] = b"croft-group-topic:";

fn tagged32(tag: &[u8], id: &str) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(tag);
    h.update(id.as_bytes());
    h.finalize().into()
}

/// The genesis anchor for a lineage (per-actor): `sha256("croft-lineage-genesis:" ‖ lineage_id)`.
/// Every device of that lineage chains its branch from this. (CROFT-PROTOCOL §2.)
pub fn lineage_genesis(lineage_id: &str) -> GenesisId {
    GenesisId(tagged32(LINEAGE_GENESIS_TAG, lineage_id))
}

/// The genesis anchor for a group (shared): `sha256("croft-group-genesis:" ‖ group_id)`.
/// The topic and any group-scoped derivations hang off this. (CROFT-PROTOCOL §2.)
pub fn group_genesis(group_id: &str) -> GenesisId {
    GenesisId(tagged32(GROUP_GENESIS_TAG, group_id))
}

/// The gossip topic for a group: `sha256("croft-group-topic:" ‖ group_id)`.
/// Derived from the group so all member lineages share one topic; deliberately
/// does not encode membership (a relay routing by topic learns nothing about who
/// is in the group). (CROFT-PROTOCOL §2.)
pub fn group_topic(group_id: &str) -> [u8; 32] {
    tagged32(GROUP_TOPIC_TAG, group_id)
}

impl std::fmt::Display for GenesisId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_hex()[..16])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_id_is_deterministic() {
        let a = GenesisId::from_bytes(b"vacation-2025");
        let b = GenesisId::from_bytes(b"vacation-2025");
        let c = GenesisId::from_bytes(b"vacation-2024");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.to_hex().len(), 64);
    }

    #[test]
    fn tagged_wire_derivations_match_the_spec_pre_images() {
        // CROFT-PROTOCOL §2 byte-exact vectors (independently computed via sha256(tag ‖ id)).
        assert_eq!(
            lineage_genesis("lin-a").to_hex(),
            "e1d1d13d80d133c60ddb47ccf01a0f5f9b9b101544a0caedc244e9621097c93d"
        );
        assert_eq!(
            group_genesis("grp-1").to_hex(),
            "7a7f2300dd542ef7650b31e0dabe694c644c7886c9d9d8c92d1fdfe9bb359efa"
        );
        assert_eq!(
            hex::encode(group_topic("grp-1")),
            "4cb63102d3c6b599fcfc4693ed71ff6d04f9007c96579b159c7455d6d769a1d8"
        );
    }

    #[test]
    fn tagged_derivations_are_domain_separated() {
        // The three tags must never collide for the same id (domain separation), and the lineage
        // and group genesis must differ from the structural GenesisId of the raw id bytes.
        assert_ne!(lineage_genesis("x").0, group_genesis("x").0);
        assert_ne!(group_genesis("x").0, group_topic("x"));
        assert_ne!(lineage_genesis("x").0, GenesisId::from_bytes(b"x").0);
    }
}
