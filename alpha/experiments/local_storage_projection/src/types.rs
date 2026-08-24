//! Adapter shim (E117 P2): the model moved to `social-tree-core` (croft).
//! This re-export keeps the corpus's import paths stable; the adapter adds
//! nothing model-shaped of its own.
pub use social_tree_core::model::*;
