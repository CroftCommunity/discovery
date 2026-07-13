//! Persisting the layout document. The shell holds one serializable value
//! (`shell::Layout`); here we load it from and save it to `localStorage`, so a
//! user-arranged workspace (panels, pins) survives a restart. A missing or
//! unreadable blob falls back to the default layout.

use shell::Layout;

const KEY: &str = "pond.layout";

fn storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

/// Load the persisted layout, or the default if absent/unreadable.
pub fn load() -> Layout {
    match storage().and_then(|s| s.get_item(KEY).ok().flatten()) {
        Some(raw) => Layout::from_json_or_default(&raw),
        None => Layout::default(),
    }
}

/// Persist the layout. Best-effort: a storage failure must not break the UI.
pub fn save(layout: &Layout) {
    if let Some(s) = storage() {
        let _ = s.set_item(KEY, &layout.to_json());
    }
}
