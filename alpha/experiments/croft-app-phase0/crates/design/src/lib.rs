//! The design system: tokens plus the Leptos primitives that consume them.
//! Knows no protocol — it never depends on `bluesky` or any protocol type, and
//! every primitive takes plain display data as props.

pub mod primitives;
pub mod tokens;

pub use primitives::*;
