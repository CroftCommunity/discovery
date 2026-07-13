//! The composable shell's platform-agnostic core: slots (named regions), the
//! serializable layout document, and pinning. Pure data + serde; no Leptos, no
//! protocol types. The web and desktop shells render from this and persist it
//! as one value. A second pond gets slots/layout/pinning for free.

pub mod layout;
pub mod pinning;
pub mod slots;

pub use layout::{Layout, PanelConfig, PanelKind};
pub use pinning::{Pin, PinType, PondId};
pub use slots::SlotId;
