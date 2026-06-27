# Platform design — Linux

date started: 2026-06-26 · status: **stub** (S4). Walk out the delta from the shared core
(`../client-architecture-adr.md`); fill the template over time.

- **Platform strengths** — _(stub)_ no app-store gatekeeper; full background execution; can hold
  long-lived sockets; natural home for a headless / CLI peer and for self-hosting a meer.
- **Platform constraints** — _(stub)_ desktop-environment fragmentation; packaging spread
  (deb/rpm/Flatpak/AppImage); no single notification/keychain story.
- **What differs from the common core** — _(stub)_ shell + `effects.rs` deltas; secret storage adapter
  (Secret Service / file-backed); the desktop shell is the wrapped web app.
- **Connectivity posture** — _(stub)_ strongest P2P story (no background limits); candidate host for a
  self-run meer / relay.
- **Open questions** — _(stub)_ packaging target priority; headless-peer vs GUI-client split.
- **Provenance** — `../client-architecture-adr.md`; open-threads S4 / T6.
