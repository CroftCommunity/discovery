//! Top-level router: the composable frame (pinned strip + contributed panels),
//! or the deterministic state gallery when the URL carries `?gallery` (used by
//! the snapshot harness).

use crate::frame::Frame;
use crate::gallery::Gallery;
use design::tokens::root_css;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    if is_gallery() {
        view! { <style>{root_css()}</style> <Gallery /> }.into_any()
    } else {
        view! { <style>{root_css()}</style> <Frame /> }.into_any()
    }
}

fn is_gallery() -> bool {
    web_sys::window()
        .and_then(|w| w.location().search().ok())
        .map(|s| s.contains("gallery"))
        .unwrap_or(false)
}
