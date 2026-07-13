//! Pinning: a pin is a *reference*, never a copy and never a merge. It holds an
//! address (the native ID in its native pond) plus a type hint — never content.
//! When rendered, each pin asks its own pond's module to hydrate it from that
//! ID. This crate stays protocol-free: it knows pond identities and addresses
//! as opaque strings, nothing about how to fetch them.

use serde::{Deserialize, Serialize};

/// Which pond an address belongs to. Open set in spirit; Phase 2 has one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PondId {
    Bluesky,
}

/// A hint at what kind of thing the address points to, so the module can pick a
/// hydration strategy without the shell knowing the protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PinType {
    Post,
    Thread,
}

/// A pin: "this item matters." Address is the native ID (e.g. an at:// URI for
/// Bluesky); the shell never interprets it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pin {
    pub pond: PondId,
    pub address: String,
    pub type_hint: PinType,
}

impl Pin {
    pub fn new(pond: PondId, address: impl Into<String>, type_hint: PinType) -> Self {
        Pin {
            pond,
            address: address.into(),
            type_hint,
        }
    }
}
