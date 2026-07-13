# web — the Leptos web shell (Phase 1)

The proven core, compiled to WASM, rendering the same view models the CLI
rendered to text — now to the DOM, against real public Bluesky data, through the
`design` system. Same intent/effect loop as the CLI; only the edges differ
(intents from DOM events, effects via the browser's `fetch`).

## Run it

```sh
cd experiments/crates/web
trunk serve --open          # live feed (real public getAuthorFeed)
# gallery of every state:  http://localhost:8080/?gallery
```

Build a static bundle: `trunk build` → `dist/`.

## Snapshot harness (the per-state DoD gate)

`snapshots/harness.mjs` loads the deterministic `?gallery` and captures one PNG
per required state into `snapshots/images/`. Every state in its `REQUIRED` list
must be present and visible or the harness exits non-zero — so an undesigned
state is a failing run. It also loads the live feed and asserts real public data
renders.

```sh
trunk build
python3 -m http.server 8088 --directory dist &     # serve the bundle
node snapshots/harness.mjs http://127.0.0.1:8088    # capture + verify
```

The committed images in `snapshots/images/` are the reference set.

## Notes / environment caveats

- **No Rust TLS, no auth.** Effects use the browser's `fetch` against the public
  AppView `getAuthorFeed` (same shape as `getTimeline`); the browser does TLS.
  Swapping to authed read later changes only the effect handler (spec 0a/0b).
- **Token-contract:** primitives never write raw values; enforced by the test in
  `crates/design`. Webview tells handled in `style/reset.css`; reduced motion and
  visible keyboard focus both respected.
- **Sandbox capture caveat:** in the CI sandbox, a TLS-intercepting proxy isn't
  trusted by headless Chromium, so the harness sets `ignoreHTTPSErrors` (a
  harness-only accommodation) and web fonts fall back to system sans. On a real
  machine the fonts (Hanken Grotesk + IBM Plex Mono) load and TLS validates
  normally — the app code is unchanged either way.
