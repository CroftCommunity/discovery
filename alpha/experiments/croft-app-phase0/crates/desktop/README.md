# desktop — the Tauri shell (Phase 2, M2.1)

"Desktop is the wrapped web app." Tauri hosts the **same** Phase 1 web bundle
(`../web/dist`) in a native macOS window. There is no second UI codebase — the
only difference from the web shell is the effect handler: `FetchFeed` is
performed by a native Rust HTTP client (`reqwest`, platform TLS) via Tauri
commands (`src/effects.rs`), instead of the browser's fetch.

The WASM frontend (`crates/web`) detects Tauri at runtime (`window.__TAURI__`)
and routes effects through `invoke("fetch_feed" | "hydrate_pin")`; off Tauri it
uses the browser's fetch. Parsing is shared (`bluesky::wire`) on both paths.

## Build & run (desktop host only)

This crate is **excluded from the Cargo workspace** on purpose: Tauri needs
system `webkit2gtk` and pulls a large dependency tree, which we keep out of the
scanned workspace lock / CI. It builds on a desktop host:

```sh
# 1. Build the shared web bundle the desktop wraps:
cd experiments/crates/web && trunk build

# 2. Run / build the desktop app (macOS, or Linux with webkit2gtk + libsoup):
cd ../desktop
cargo tauri dev          # or: cargo tauri build
```

(Requires the Tauri CLI: `cargo install tauri-cli --version '^2'`.)

## Not verifiable in the CI sandbox

The headless CI container has no `webkit2gtk`, so this crate **cannot be compiled
or launched here** — it is written as standard Tauri v2 boilerplate and verified
on a real desktop host. The web shell it wraps *is* fully verified headlessly
(see `../web`). `Cargo.lock` is generated on the build host (not committed).
