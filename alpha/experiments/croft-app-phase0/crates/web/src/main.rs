//! The web shell entry point: mount the Leptos app. The app hosts the
//! composable frame (pinned strip + contributed feed panels) over the same
//! intent/effect loop the CLI proved — intents from DOM events, view models to
//! the DOM, effects via the browser's fetch.

mod app;
mod effects;
mod frame;
mod gallery;
mod layout_store;
mod panel;
mod pinned;
mod runtime;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(app::App);
}
