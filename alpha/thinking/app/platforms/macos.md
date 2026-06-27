# Platform design — macOS

date started: 2026-06-26 · status: **stub** (S4). Walk out the delta from the shared core
(`../client-architecture-adr.md`); fill the template over time.

- **Platform strengths** — _(stub)_ generous background execution vs iOS; Keychain; first-build target
  for the optional on-device-LLM assistant (per `../../seeds/apps-unpacked/on-device-llm-feasibility.md`,
  Android + macOS are first targets).
- **Platform constraints** — _(stub)_ notarization / Gatekeeper; App Store sandbox if distributed there
  (vs direct/Developer-ID); some entitlement friction for P2P networking.
- **What differs from the common core** — _(stub)_ Tauri-wrapped web shell; Keychain secret adapter;
  notification + lifecycle `effects.rs`.
- **Connectivity posture** — _(stub)_ strong P2P (desktop-class background); good meer-adjacent uptime if
  left running.
- **Open questions** — _(stub)_ distribution channel (App Store vs Developer-ID direct); on-device-LLM
  scope.
- **Provenance** — `../client-architecture-adr.md`; `../../seeds/apps-unpacked/on-device-llm-feasibility.md`;
  open-threads S4 / T6.
