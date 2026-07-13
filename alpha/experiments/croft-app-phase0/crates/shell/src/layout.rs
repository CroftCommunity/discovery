//! The layout document: one serializable value describing which panels exist,
//! where, their sizes and visibility — plus the pin list. The user edits it by
//! arranging the UI; the shell persists and restores it (the Obsidian-workspace
//! model). Pins live in the same document.

use crate::pinning::Pin;
use crate::slots::SlotId;
use serde::{Deserialize, Serialize};

/// What a panel shows. An open set: adding a kind here (and a renderer in the
/// platform shell's registry) is the only change needed for a new panel type —
/// the layout/slot machinery is untouched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum PanelKind {
    /// A single pond feed (Phase 2: Bluesky public author feed).
    Feed { actor: String },
}

/// One panel placed in a slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelConfig {
    /// Stable id, so the same panel survives reordering/persistence.
    pub id: String,
    #[serde(flatten)]
    pub kind: PanelKind,
    pub slot: SlotId,
    /// Relative size weight within its slot.
    pub size: u32,
    pub visible: bool,
}

/// The whole persisted workspace: panels + pins. Versioned so a future schema
/// change can migrate rather than silently misread an old document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layout {
    #[serde(default = "default_version")]
    pub version: u32,
    pub panels: Vec<PanelConfig>,
    #[serde(default)]
    pub pins: Vec<Pin>,
}

fn default_version() -> u32 {
    1
}

impl Default for Layout {
    /// The starting workspace: one Bluesky feed panel in the column, no pins.
    fn default() -> Self {
        Layout {
            version: default_version(),
            panels: vec![PanelConfig {
                id: "feed-bsky".to_string(),
                kind: PanelKind::Feed {
                    actor: "bsky.app".to_string(),
                },
                slot: SlotId::Column,
                size: 1,
                visible: true,
            }],
            pins: Vec::new(),
        }
    }
}

impl Layout {
    /// Serialize to JSON for persistence.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Layout serializes")
    }

    /// Parse a persisted document, falling back to the default if it is absent
    /// or unreadable (a corrupt/older blob should never brick the app).
    pub fn from_json_or_default(raw: &str) -> Self {
        serde_json::from_str(raw).unwrap_or_default()
    }

    /// Panels contributed into a given slot, in order, visible only.
    pub fn panels_in(&self, slot: SlotId) -> impl Iterator<Item = &PanelConfig> {
        self.panels
            .iter()
            .filter(move |p| p.slot == slot && p.visible)
    }

    /// Add a pin if an identical one is not already present (idempotent).
    pub fn add_pin(&mut self, pin: Pin) {
        if !self.pins.contains(&pin) {
            self.pins.push(pin);
        }
    }

    /// Remove a pin by address.
    pub fn remove_pin(&mut self, address: &str) {
        self.pins.retain(|p| p.address != address);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinning::{PinType, PondId};

    #[test]
    fn default_layout_has_one_feed_in_the_column() {
        let l = Layout::default();
        assert_eq!(l.panels_in(SlotId::Column).count(), 1);
        assert!(l.pins.is_empty());
    }

    #[test]
    fn round_trips_through_json() {
        let mut l = Layout::default();
        l.add_pin(Pin::new(PondId::Bluesky, "at://did:plc:x/app.bsky.feed.post/1", PinType::Post));
        let back = Layout::from_json_or_default(&l.to_json());
        assert_eq!(l, back);
    }

    #[test]
    fn corrupt_document_falls_back_to_default() {
        assert_eq!(Layout::from_json_or_default("{ not json"), Layout::default());
    }

    #[test]
    fn pinning_is_idempotent_and_removable() {
        let mut l = Layout::default();
        let p = Pin::new(PondId::Bluesky, "at://x/1", PinType::Post);
        l.add_pin(p.clone());
        l.add_pin(p.clone());
        assert_eq!(l.pins.len(), 1);
        l.remove_pin("at://x/1");
        assert!(l.pins.is_empty());
    }

    #[test]
    fn hidden_panels_are_excluded_from_their_slot() {
        let mut l = Layout::default();
        l.panels[0].visible = false;
        assert_eq!(l.panels_in(SlotId::Column).count(), 0);
    }
}
