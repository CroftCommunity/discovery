//! Ed25519 adapters — re-exported from the core's port realizations.
//!
//! The implementations lived here until E117 P3 relocated them into
//! `social-tree-core::ports::ed25519` (the fold's authorship rung wanted one
//! real path, not two). This module is now a shim so the facade's public
//! surface is unchanged; the behavior pins travel with the core.

pub use social_tree_core::ports::ed25519::{
    Ed25519Signer, Ed25519Verifier, MonotonicLamport, RegistryCredentialResolver,
};
