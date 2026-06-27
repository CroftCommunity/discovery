# Per-platform design files (the common-core / platform-difference walk-out)

date started: 2026-06-26

status: **scaffold — accreting.** Started per the 2026-06-26 open-threads review (structural decision
**S4**, beta `OPEN-THREADS` **T6 / T14**). One design file per platform, each walking out what is
**shared common core** vs what is **genuinely different** for that platform. Every not-yet-implemented
platform gets its own file; the thinking does not stay a one-off open thread.

## The settled spine these sit on (do not re-litigate here)

`../client-architecture-adr.md` already settled it (2026-06-22): **one shared functional core**
(`(state, intent) -> (state, effects)`, no I/O, WASM-clean) behind a **thin per-platform imperative
shell**, each shell supplying its own `effects.rs` callout; two callout axes (platform + an
implementation-behind-a-port). Web-first; desktop is the wrapped web app; the heavy P2P work is isolated
in the one pond Croft owns. The substrate is **iroh 1.0** (gossip / docs / blobs).

So each platform file answers only the **delta**: what this platform makes easy, what it makes hard, and
the trade-offs the shared core cannot paper over.

## Per-file template (copy into each)

- **Platform strengths** — what to lean into (system idioms, lifecycle, hardware).
- **Platform constraints** — what fights the architecture (background execution, sockets, sandboxing,
  store policy).
- **What differs from the common core** — the shell deltas, the `effects.rs` specifics, which ports get
  a platform-specific adapter.
- **Connectivity posture** — what this platform can promise about P2P / background sync / off-grid.
- **Open questions** — undecided, with the gate.
- **Provenance** — alpha sources + threads.

## Files

- `linux.md` · `macos.md` · `android.md` · `ios.md`

iOS is the sharpest case (T14): no background socket → opportunistic, not deterministic P2P → the meer
is the dependable backbone, not a bonus. See `ios.md`.

## Provenance

`beta/thinking/raw/open threads review Jun 26 at 8-17 PM.txt` (S4 / T6 / T14);
`../client-architecture-adr.md`; `../../ios-opportunistic-p2p.md`.
