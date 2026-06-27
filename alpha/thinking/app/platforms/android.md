# Platform design — Android

date started: 2026-06-26 · status: **stub** (S4). Walk out the delta from the shared core
(`../client-architecture-adr.md`); fill the template over time.

- **Platform strengths** — _(stub)_ foreground services + WorkManager give a workable (if constrained)
  background story — far better than iOS; first-build target for the optional on-device-LLM assistant
  (`../../seeds/apps-unpacked/on-device-llm-feasibility.md`).
- **Platform constraints** — _(stub)_ Doze / app-standby / OEM battery killers throttle background sync;
  Play Store policy; per-OEM behavior variance.
- **What differs from the common core** — _(stub)_ foreground-service + WorkManager scheduling in
  `effects.rs`; Keystore secret adapter; push (FCM) origination as a host effect.
- **Connectivity posture** — _(stub)_ better-than-iOS but still not guaranteed background P2P; meer +
  push remain the dependable wake path.
- **Open questions** — _(stub)_ FCM dependency vs the "no central operator" stance; background-sync
  promise under Doze.
- **Provenance** — `../client-architecture-adr.md`;
  `../../seeds/apps-unpacked/on-device-llm-feasibility.md`; open-threads S4 / T6.
