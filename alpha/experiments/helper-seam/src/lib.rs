//! helper-seam — RUN-14 EXP-C: the content-helper seam.
//!
//! Proves the social-mapping *helper delegation* claim end to end: a content
//! helper admitted to a scope by grant holds cleartext by the same grant any
//! member holds keys by, feeds the AppView index, and goes forward-blind on
//! revocation — while never holding authority. Built on the real MLS mechanism
//! (`group-seal`, croft-group L2a) and the source-agnostic `NormalizedEvent`
//! boundary copied from `public-roundtrip`.

pub mod bridge;
pub mod helper;
pub mod index;

pub use bridge::NormalizedEvent;
pub use helper::ContentHelper;
pub use index::{Index, SearchHit};
