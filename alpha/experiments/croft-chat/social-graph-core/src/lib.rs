//! Tenant-agnostic social-graph substrate facade (the Drystone protocol layer).
//!
//! This crate is the thin domain layer over the redb-backed
//! `local_storage_projection` surface. It owns session/identity and the
//! tenant-agnostic graph operations (groups, members, channels, timeline) so
//! that tenants (chat, notes, games) attach to one substrate without reaching
//! around it.
//!
//! Phases populate this crate: P4 adds the ed25519 adapters
//! (`crypto`/`identity`), P5 adds the `Session` facade.
#![warn(missing_docs)]

pub mod crypto;
pub mod identity;
pub mod session;

pub use crypto::{
    Ed25519Signer, Ed25519Verifier, MonotonicLamport, RegistryCredentialResolver,
};
pub use identity::Identity;
pub use session::{ApplyOutcome, ChannelRef, Session, SessionError};

// Re-export the substrate view types tenants render, so they depend on
// social-graph-core (never reaching around it into the substrate directly).
pub use local_storage_projection::surface::{
    assertion_order_key, GroupListItemView, GroupSummaryView, MemberView, MessageView,
    TimelineEntry, TimelineView, TimelineWindow,
};
pub use local_storage_projection::types::{GroupId, Hash, KindTag, PrincipalId, Role, TypedId};
