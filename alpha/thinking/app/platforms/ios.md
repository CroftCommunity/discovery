# Platform design — iOS

date started: 2026-06-26 · status: **stub** (S4) — the sharpest platform case. Walk out the delta from
the shared core (`../client-architecture-adr.md`); couples beta `OPEN-THREADS` **T14**.

The driving constraint (from `../../ios-opportunistic-p2p.md`, T14): **you cannot hold a background
socket on iOS**, so device-to-device P2P is **opportunistic, not deterministic**, and spontaneous
off-grid meshing is aspirational/unproven. This structurally argues the **meer is the dependable
backbone, not a bonus**. The iOS app must play to iOS strengths.

- **Platform strengths** — _(stub)_ event-driven from the system; push (APNs) as the wake primitive;
  BackgroundTasks for opportunistic catch-up; tight Keychain; strong UX expectations.
- **Platform constraints** — _(stub)_ no long-lived background socket; aggressive suspension; App Store
  review; background execution budgeted, not guaranteed.
- **What differs from the common core** — _(stub)_ the shell is **push-triggered + sync-on-foreground**,
  not socket-resident; `effects.rs` wraps APNs + BackgroundTasks + Keychain; lean hard on the meer for
  delivery while suspended.
- **Connectivity posture** — _(stub)_ **opportunistic** P2P only; off-grid/background sync is NOT
  promised deterministically (T14 product gate); the meer + push is the reliable path. State this as a
  named product limitation (T14 → 08 §9).
- **Open questions** — _(stub)_ exactly what off-grid/background behavior Croft promises on iOS (the T14
  product decision, undecided); APNs dependency vs the "no central operator" stance.
- **Provenance** — `../../ios-opportunistic-p2p.md`; `../client-architecture-adr.md`; beta `OPEN-THREADS`
  T14; open-threads S4.
