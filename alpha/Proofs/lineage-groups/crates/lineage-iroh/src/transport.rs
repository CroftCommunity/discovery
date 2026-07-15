//! Transport abstraction.
//!
//! The thesis-critical logic must not know whether bytes travel over iroh,
//! a relay, or an in-process queue. Everything crosses this seam as opaque
//! ciphertext addressed to a node on a group topic — exactly what a real iroh
//! gossip topic (Delta Chat pattern: a random topic id, lazy P2P bootstrap)
//! would carry.

use sha2::{Digest, Sha256};

/// A transport-level address. This is routing metadata a relay legitimately
/// sees; it is not a group-membership statement.
pub type NodeId = String;

/// A group topic id. Mirrors the Delta Chat pattern: a random, opaque id shared
/// out-of-band among members. It deliberately does NOT encode membership, so a
/// relay routing by topic learns nothing about who is in the group.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct GroupTopic(pub [u8; 32]);

impl GroupTopic {
    /// The canonical group topic (CROFT-PROTOCOL §2): `sha256("croft-group-topic:" ‖ group_id)`,
    /// via the single-source-of-truth derivation in `lineage_core::ids`.
    pub fn from_group_id(group_id: &str) -> Self {
        Self(lineage_core::ids::group_topic(group_id))
    }

    /// Test stand-in only: a stable opaque topic id from a numeric seed (NOT the §2 wire derivation —
    /// use [`from_group_id`] for that). Kept for the e3 end-to-end harness's deterministic seeds.
    pub fn from_seed(seed: u64) -> Self {
        let mut h = Sha256::new();
        h.update(b"lineage-topic-v1");
        h.update(seed.to_le_bytes());
        let mut id = [0u8; 32];
        id.copy_from_slice(&h.finalize());
        Self(id)
    }

    pub fn short(&self) -> String {
        hex::encode(self.0)[..12].to_string()
    }
}

/// The coarse kind of an MLS payload. A relay sees this framing tag (as any
/// transport sees message framing) but never the plaintext or who it concerns.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnvKind {
    /// A Welcome (newcomer bootstrap).
    Welcome,
    /// A handshake/commit message (epoch change).
    Handshake,
    /// An application message.
    App,
}

/// One opaque, addressed payload on the wire.
#[derive(Clone)]
pub struct Envelope {
    pub to: NodeId,
    pub topic: GroupTopic,
    pub kind: EnvKind,
    pub ciphertext: Vec<u8>,
}
