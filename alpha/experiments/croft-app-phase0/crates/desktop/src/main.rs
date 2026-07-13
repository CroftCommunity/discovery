//! The Tauri desktop entry point. Hosts the same Phase 1 web bundle in a native
//! window and registers the native effect-handler commands the WASM frontend
//! calls via `invoke` when it detects it is running under Tauri. "Desktop is the
//! wrapped web app": one UI codebase, a different effect performer.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod effects;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            effects::fetch_feed,
            effects::hydrate_pin
        ])
        .run(tauri::generate_context!())
        .expect("error while running the pond desktop app");
}
