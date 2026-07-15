//! `lineage-iroh` — Phase 3 end-to-end thin slice.
//!
//! Goal (plan §4, Phase 3): show genesis → fork → recombine working across
//! nodes over a transport, with an optional **blind broker** that carries
//! commits/rekeys when peers are not co-present and stores recovery snapshots.
//!
//! ## Real iroh: a documented negative result (the plan accepts this)
//! A feasibility probe (PHASE_3_FINDINGS.md) showed that vendoring `iroh` locks
//! ~395 packages including *pre-release* crypto crates that collide with our
//! pinned stable `ed25519-dalek`/`sha2`, and real P2P networking is outside this
//! environment's network policy. Rather than destabilise the dependency set for
//! a transport we cannot exercise here, transport is abstracted behind
//! [`broker::BlindBroker`] (and the `transport` types) so the iroh binding is a
//! drop-in for a network-enabled environment. The thesis-critical logic —
//! genesis/fork/recombine, partition/reconnect convergence, conflict hard-stop,
//! recovery, and broker blindness — is exercised end-to-end here over the
//! broker, wiring the *real* Phase 1 MLS and Phase 2 governance/history.
//!
//! What an iroh binding would add (and only that): replace the in-process
//! [`broker::BlindBroker`] queue with iroh `Endpoint`s + an `iroh-gossip` topic
//! per group (Delta Chat pattern), keeping the exact same [`transport`] seam.

pub mod broker;
pub mod node;
pub mod transport;

pub use broker::BlindBroker;
pub use node::Node;
pub use transport::{EnvKind, Envelope, GroupTopic, NodeId};
