//! `stone-alpha` topology config: the four-node test layout + loader.
//!
//! Pure data + parsing (no iroh), so it builds without the `iroh-it` feature.
//! The binary uses it to start as a named node and to know which peers to
//! bootstrap from; the `EndpointAddr` JSON itself is exchanged out-of-band
//! (written/read by the iroh path) since addresses are only known at runtime.

use std::path::Path;

use serde::Deserialize;
use social_graph_core::{Identity, PrincipalId};
use thiserror::Error;

/// Errors loading or resolving a topology.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// The file could not be read.
    #[error("reading topology {path}: {source}")]
    Read {
        /// Offending path.
        path: String,
        /// Underlying IO error.
        source: std::io::Error,
    },
    /// The TOML could not be parsed.
    #[error("parsing topology: {0}")]
    Parse(#[from] toml::de::Error),
    /// No node by that name.
    #[error("no node named {0} in topology")]
    UnknownNode(String),
}

/// The whole topology.
#[derive(Debug, Clone, Deserialize)]
pub struct Topology {
    /// Relay mode (`"n0"` for the n0 relays).
    pub relay_mode: String,
    /// The nodes, in file order.
    #[serde(rename = "node")]
    pub nodes: Vec<NodeConfig>,
}

/// One node in the topology.
#[derive(Debug, Clone, Deserialize)]
pub struct NodeConfig {
    /// Stable node name (used on the CLI: `--node <name>`).
    pub name: String,
    /// Host (public IP, internal IP, or `"nat"` for an off-VPC workstation).
    pub host: String,
    /// UDP/QUIC bind port.
    pub port: u16,
    /// Deterministic demo-identity seed.
    pub seed: u8,
    /// Whether the node is publicly reachable (has a routable IP).
    pub public: bool,
}

impl NodeConfig {
    /// This node's deterministic demo identity.
    #[must_use]
    pub fn identity(&self) -> Identity {
        Identity::from_seed([self.seed; 32])
    }

    /// This node's principal id.
    #[must_use]
    pub fn principal(&self) -> PrincipalId {
        PrincipalId::new(self.identity().principal_id().0)
    }
}

impl Topology {
    /// Load and parse a topology file.
    ///
    /// # Errors
    /// [`ConfigError::Read`] / [`ConfigError::Parse`] on IO or syntax failure.
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let text = std::fs::read_to_string(path).map_err(|source| ConfigError::Read {
            path: path.display().to_string(),
            source,
        })?;
        Self::parse(&text)
    }

    /// Parse a topology from TOML text.
    ///
    /// # Errors
    /// [`ConfigError::Parse`] on a syntax/schema error.
    pub fn parse(text: &str) -> Result<Self, ConfigError> {
        let topology: Topology = toml::from_str(text)?;
        tracing::info!(
            nodes = topology.nodes.len(),
            relay_mode = %topology.relay_mode,
            "topology loaded"
        );
        Ok(topology)
    }

    /// Resolve a node by name.
    ///
    /// # Errors
    /// [`ConfigError::UnknownNode`] if no node matches.
    pub fn node(&self, name: &str) -> Result<&NodeConfig, ConfigError> {
        self.nodes
            .iter()
            .find(|n| n.name == name)
            .ok_or_else(|| ConfigError::UnknownNode(name.to_string()))
    }

    /// The peers `name` should bootstrap from: every *other* node it can plausibly
    /// reach. A non-public node (NAT/internal) is dropped from peer lists for
    /// nodes that can't route to it; public nodes are always included.
    #[must_use]
    pub fn bootstrap_peers(&self, name: &str) -> Vec<&NodeConfig> {
        self.nodes
            .iter()
            .filter(|n| n.name != name && n.public)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Topology;

    const STONE_ALPHA: &str = include_str!("../../stone-alpha.toml");

    #[test]
    fn parses_the_checked_in_topology() {
        let topo = Topology::parse(STONE_ALPHA).expect("parse stone-alpha");
        assert_eq!(topo.relay_mode, "n0");
        assert_eq!(topo.nodes.len(), 4, "four nodes");
    }

    #[test]
    fn resolves_a_node_and_its_principal_and_peers() {
        let topo = Topology::parse(STONE_ALPHA).expect("parse");

        // Resolve node4 (the workstation, NAT).
        let node = topo.node("node4-workstation").expect("node4");
        assert_eq!(node.seed, 4);
        assert!(!node.public, "workstation is NAT");

        // Its principal is derived deterministically from the seed.
        let expected =
            social_graph_core::PrincipalId::new(
                social_graph_core::Identity::from_seed([4; 32]).principal_id().0,
            );
        assert_eq!(node.principal(), expected);

        // Bootstrap peers are the public boxes (node1, node2) — not the
        // internal-only node3, not itself.
        let peers: Vec<&str> = topo
            .bootstrap_peers("node4-workstation")
            .iter()
            .map(|n| n.name.as_str())
            .collect();
        assert!(peers.contains(&"secroute-testing-one"));
        assert!(peers.contains(&"secroute-testing-two"));
        assert!(!peers.contains(&"node3"), "internal-only node excluded");
        assert!(!peers.contains(&"node4-workstation"), "self excluded");
    }

    #[test]
    fn unknown_node_errors() {
        let topo = Topology::parse(STONE_ALPHA).expect("parse");
        assert!(topo.node("nope").is_err());
    }
}
