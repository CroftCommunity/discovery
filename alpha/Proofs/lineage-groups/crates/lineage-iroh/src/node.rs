//! A node: one device's MLS state wired to ship/receive through a transport.
//!
//! This is where Phase 1 (MLS) meets Phase 3 (transport). A node never talks to
//! peers directly; everything is an opaque [`Envelope`] handed to the broker.
//! Group operations (add/remove/external-commit) and application messages all
//! cross the same seam, so the broker only ever sees ciphertext + routing.

use lineage_core::ids::Did;
use lineage_mls::{Device, KeyPackage, MlsError, Received};

use crate::broker::BlindBroker;
use crate::transport::{EnvKind, Envelope, GroupTopic, NodeId};

/// Pack a (group_info, ratchet_tree) pair into one opaque snapshot blob.
fn pack_snapshot(group_info: &[u8], ratchet_tree: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + group_info.len() + ratchet_tree.len());
    out.extend_from_slice(&(group_info.len() as u32).to_le_bytes());
    out.extend_from_slice(group_info);
    out.extend_from_slice(ratchet_tree);
    out
}

fn unpack_snapshot(blob: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    if blob.len() < 4 {
        return None;
    }
    let n = u32::from_le_bytes(blob[0..4].try_into().ok()?) as usize;
    let rest = &blob[4..];
    if rest.len() < n {
        return None;
    }
    Some((rest[..n].to_vec(), rest[n..].to_vec()))
}

pub struct Node {
    pub id: NodeId,
    pub did: Did,
    pub topic: GroupTopic,
    dev: Device,
}

impl Node {
    pub fn new(id: &str, did: &str, topic: GroupTopic) -> Result<Self, MlsError> {
        Ok(Self {
            id: id.to_string(),
            did: Did::new(did),
            topic,
            dev: Device::new(Did::new(did))?,
        })
    }

    pub fn key_package(&self) -> Result<KeyPackage, MlsError> {
        self.dev.key_package()
    }

    pub fn epoch(&self) -> Result<u64, MlsError> {
        self.dev.epoch()
    }

    pub fn member_count(&self) -> Result<usize, MlsError> {
        self.dev.member_count()
    }

    pub fn epoch_proof(&self) -> Result<Vec<u8>, MlsError> {
        self.dev.epoch_proof()
    }

    pub fn leaf_index_of(&self, did: &Did) -> Result<Option<lineage_mls::LeafNodeIndex>, MlsError> {
        self.dev.leaf_index_of(did)
    }

    /// Found a new group (genesis).
    pub fn found_group(&mut self) -> Result<(), MlsError> {
        self.dev.create_group()
    }

    fn ship(&self, broker: &mut BlindBroker, to: &NodeId, kind: EnvKind, ciphertext: Vec<u8>) {
        broker.relay(Envelope {
            to: to.clone(),
            topic: self.topic,
            kind,
            ciphertext,
        });
    }

    /// Add a member: welcome → the newcomer, commit → existing peers.
    pub fn add_member(
        &mut self,
        broker: &mut BlindBroker,
        newcomer: &NodeId,
        kp: KeyPackage,
        existing_peers: &[NodeId],
    ) -> Result<(), MlsError> {
        let (commit, welcome) = self.dev.add(&[kp])?;
        self.ship(broker, newcomer, EnvKind::Welcome, welcome);
        for p in existing_peers {
            self.ship(broker, p, EnvKind::Handshake, commit.clone());
        }
        Ok(())
    }

    /// Remove a member by DID; commit → the given peers.
    pub fn remove_member(
        &mut self,
        broker: &mut BlindBroker,
        did: &Did,
        peers: &[NodeId],
    ) -> Result<(), MlsError> {
        let idx = self
            .dev
            .leaf_index_of(did)?
            .ok_or_else(|| MlsError::Op(format!("not a member: {}", did.redacted())))?;
        let (commit, _welcome) = self.dev.remove(&[idx])?;
        for p in peers {
            self.ship(broker, p, EnvKind::Handshake, commit.clone());
        }
        Ok(())
    }

    /// Send an application message to peers.
    pub fn broadcast(
        &mut self,
        broker: &mut BlindBroker,
        peers: &[NodeId],
        text: &[u8],
    ) -> Result<(), MlsError> {
        let ct = self.dev.send(text)?;
        for p in peers {
            self.ship(broker, p, EnvKind::App, ct.clone());
        }
        Ok(())
    }

    /// Process all currently-deliverable mail. Welcomes bootstrap the group;
    /// handshakes advance the epoch; application messages are returned decrypted.
    pub fn pump(&mut self, broker: &mut BlindBroker) -> Result<Vec<Vec<u8>>, MlsError> {
        let mut out = Vec::new();
        for env in broker.drain_topic(&self.id, self.topic) {
            match env.kind {
                EnvKind::Welcome => {
                    self.dev.join_from_welcome(&env.ciphertext, None)?;
                }
                EnvKind::Handshake | EnvKind::App => match self.dev.recv(&env.ciphertext)? {
                    Received::Application(bytes) => out.push(bytes),
                    Received::CommitMerged => {}
                },
            }
        }
        Ok(out)
    }

    /// Encrypt an application message and return the ciphertext without
    /// shipping it (used by tests to capture epoch-bound traffic).
    pub fn send_raw(&mut self, text: &[u8]) -> Result<Vec<u8>, MlsError> {
        self.dev.send(text)
    }

    /// Attempt to decrypt a raw ciphertext directly (bypassing the broker).
    /// Used to assert forward-only secrecy: a recovered device must NOT decrypt
    /// traffic from epochs it was never part of.
    pub fn try_recv(&mut self, ciphertext: &[u8]) -> Result<Received, MlsError> {
        self.dev.recv(ciphertext)
    }

    /// Publish this node's current epoch as an opaque snapshot the broker can
    /// hand to a recovering/reconnecting peer (E3.1 / E3.3).
    pub fn publish_snapshot(&self, broker: &mut BlindBroker) -> Result<(), MlsError> {
        let (group_info, ratchet_tree) = self.dev.publish_group_info()?;
        broker.put_snapshot(self.topic, pack_snapshot(&group_info, &ratchet_tree));
        Ok(())
    }

    /// Join (or re-key into) the survivor epoch using a broker-held snapshot via
    /// an external commit; the resulting commit is shipped to `peers` so they
    /// admit this node (the survivor re-key / recovery primitive).
    pub fn join_via_broker(
        &mut self,
        broker: &mut BlindBroker,
        peers: &[NodeId],
    ) -> Result<(), MlsError> {
        let blob = broker
            .get_snapshot(self.topic)
            .ok_or_else(|| MlsError::Op("no snapshot for topic".into()))?;
        let (group_info, ratchet_tree) =
            unpack_snapshot(&blob).ok_or_else(|| MlsError::Op("malformed snapshot".into()))?;
        let commit = self.dev.external_commit_join(&group_info, &ratchet_tree)?;
        for p in peers {
            self.ship(broker, p, EnvKind::Handshake, commit.clone());
        }
        Ok(())
    }
}
