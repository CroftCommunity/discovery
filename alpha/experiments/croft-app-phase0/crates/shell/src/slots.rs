//! Slots: the named regions the UI exposes. A panel declares which slot it
//! contributes into; the shell renders slots without knowing the panels. This
//! is the contribution model, kept minimal — the data half lives here (which
//! panel kind, which slot); the render half (a registry of view builders) lives
//! in the platform shell, so adding a panel type needs no change here.

use serde::{Deserialize, Serialize};

/// A named region of the frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SlotId {
    /// The top-level pinned strip, above the panels.
    PinnedStrip,
    /// The main column area where feed panels live.
    Column,
    /// A side region (reserved; nothing contributes here in Phase 2).
    Sidebar,
}

impl SlotId {
    pub fn as_str(self) -> &'static str {
        match self {
            SlotId::PinnedStrip => "pinned-strip",
            SlotId::Column => "column",
            SlotId::Sidebar => "sidebar",
        }
    }
}
