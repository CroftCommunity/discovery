# Experiment: composable social client (Phase 0)

An app-design experiment, parked here under `experiments/`. It builds the
architectural spine described in the two companion docs and nothing more.

- [`design-philosophy.md`](./design-philosophy.md) — the *why*: principles,
  values, and the binding decisions.
- [`BUILD-SPEC.md`](./BUILD-SPEC.md) — the *what* and *in what order*: the Phase 0
  build spec and acceptance criteria.

## What Phase 0 is

A pure Rust **functional core** and a **command-line shell** that drives it,
proving the architecture: `(state, intent) -> (state, effects)`, effects-as-data,
view models that render as text, a module behind a swappable port held by the
shell.

```text
crates/
  core/      pure logic; no I/O, no async, no clock (the heart)
  bluesky/   the first ecosystem module: native types, the port trait, a fake
  cli/       the imperative shell: runtime loop, effects, text renderer
```

## Status against the milestones

| Milestone | State |
|-----------|-------|
| M1 — workspace & core types | ✅ done |
| M2 — core behavior, test-first (A1–D2) | ✅ done — 13 tests pass |
| M3 — projection, test-first (P1–P7) | ✅ done — 7 tests pass |
| M4 — Bluesky port & fake | ✅ done — real fixtures committed; fake-mode tests pass |
| M5 — CLI against the fake | ✅ done — loop + all render states; `open`/`more`/`empty`/`error` |
| M6 — real adapter | ✅ done — live fetch behind the same port; parsing tested vs fixtures |

**Phase 0 is complete (M1–M6).** 22 core/CLI tests + 8 feature-gated bluesky
tests pass; `cargo clippy --all-features` is clean.

## Running it

```sh
cd experiments
cargo test                                  # core + projection + render coverage
cargo test -p bluesky --features adapter    # wire-parsing tests vs the fixtures
cargo test -p bluesky --features fake       # fake-mode determinism tests

cargo run --bin pond -- open                # fixtures: page 1, "more below" footer
cargo run --bin pond -- more                #   then append page 2
cargo run --bin pond -- empty               #   the empty-feed view
cargo run --bin pond -- error               #   fake error mode -> error view

cargo run --bin pond -- --real open         # live: fetch a real feed over HTTP
cargo run --bin pond -- --real --actor <handle> more
```

## Fixtures (real recordings, not fabricated)

`BUILD-SPEC.md` §0/§6 require the fixtures to be **real recorded responses**,
never hand-authored. The committed files in `crates/bluesky/fixtures/`
(`timeline_page_1.json`, `timeline_page_2.json`, `timeline_empty.json`) were
recorded from the public AppView's `app.bsky.feed.getAuthorFeed`, which returns
the identical `FeedViewPost` shape as `getTimeline`
(`{ "feed": [ { "post": { … } } ], "cursor": "…" }`). `getTimeline` itself is
per-user and needs auth; the author-feed recording is genuine protocol data of
the same shape, so the parser is tested against shapes the protocol really
produces. Page 1's `cursor` chains into page 2, and the empty file is a real
end-of-feed response.

## The real adapter (M6)

`crates/bluesky/src/adapter.rs` fetches the public AppView feed endpoint behind
the same `BlueskyPort` as the fake — so swapping fake ↔ real changes nothing in
the core or the shell loop.

It performs the HTTPS GET by shelling out to the system **`curl`** rather than
linking a Rust TLS stack. That is deliberate: every Rust TLS backend pulled in a
transitive crate CroftC's license policy (Cycode) flags as non-permissive
(rustls → `webpki-roots`, CDLA-Permissive-2.0; native-tls → `r-efi`, LGPL). A
native client is exactly what the shell is meant to use to perform effects
(philosophy §6), it keeps the dependency tree clean, and the choice stays
reversible behind the port. It is blocking, so the shell's minimal single-poll
executor drives it with no async runtime. (`curl` must be on `PATH`.)

`BUILD-SPEC` names Jacquard as the eventual library; the port keeps that choice
reversible, and the moderation/labeling question is deferred with it (see
`BUILD-SPEC.md` M6).
