# Rapier `enhanced-determinism`: does native == wasm?

**Verdict: YES**, on the axis tested — and the feature is load-bearing, not decorative.

This spike exists to settle whether emoji-wars can carry a **verifiable outcome**
(`fun`'s Tier-1 bar) on a float physics engine, rather than needing a fixed-point
rewrite. It is the cheapest measurement that could have invalidated the largest
piece of the plan, so it was run first.

## Result

```
                            native (aarch64-apple-darwin)   wasm32-unknown-unknown
enhanced-determinism ON      2135883295078246327             2135883295078246327    MATCH
enhanced-determinism OFF    14104404486998819895             2135883295078246327    DIVERGE
```

Digest = FNV-1a-64 over the **bit patterns** of every dynamic body's final
`(x, y, angle)` after 600 fixed steps at `dt = 1/60`.

## What the control actually showed

The interesting cell is bottom-left. Read across the bottom row: **wasm produces
the same digest either way** — only *native* moves when the feature is turned off.

That is the mechanism working exactly as documented. `enhanced-determinism`
expands to `[simba/libm_force, parry2d/enhanced-determinism]`, and `libm_force`
routes transcendental math through the `libm` crate instead of the platform's
own. `wasm32-unknown-unknown` has no platform math library, so it was *already*
using that libm; the feature's real job is to drag **native** onto the same
implementation macOS otherwise replaces with its own.

Consequence worth carrying forward: the divergence risk lives on the native side.
A browser-only build would likely have agreed with itself without the feature —
but `fun`'s Tier-1 convention is specifically a `native == wasm` cross-check, and
that is precisely the axis that fails without it.

## Why the match is believable

A digest that agrees proves nothing on its own — it could be insensitive, or the
scenario could be inert. Both were tested rather than assumed:

- `a_one_ulp_change_to_the_launch_moves_the_digest` — perturbing the hero's launch
  velocity by **one ULP** (the smallest representable f32 change) produces a
  different digest. The hash responds to the simulation.
- `the_simulation_actually_does_work` — asserts the hero travelled from its spawn
  and that stack pieces were displaced >0.25 units. A pile that never moved would
  hash stably and mean nothing.
- `scenario_hash_is_stable_across_runs_in_one_process` — rules out the boring
  failure (global state, unstable iteration order) masquerading as a platform result.
- The golden vector was **RED at `0` first**, so the recorded value is measured,
  not assumed.

The scenario is contact-rich on purpose: a launched, spinning hero into a 3×4
stack of unstable boxes beside an angled ramp. Restitution, friction, and rotation
all participate, and 10 simulated seconds gives divergence room to compound rather
than cancel.

## What this does NOT establish

- **One wasm engine only.** The wasm side ran under Node 22 (V8). iOS Safari is
  JavaScriptCore and was not tested. The result is *likely* to hold — wasm f32
  arithmetic is spec-deterministic, and the transcendental functions are compiled
  **into the module** rather than supplied by the engine — but "likely" is not
  "measured." Testing a real iOS device is the obvious follow-up, and note that
  `fun`'s existing gate is chromium-only, so it would not catch a JSC difference either.
- **One machine.** Only this aarch64 Mac. An x86_64 native build was not tested.
- **Not the whole determinism problem.** A verifiable outcome also needs a fixed
  timestep, captured and quantised inputs, a canonical state serialisation, and
  bit-exact level-JSON → world construction. This spike settles the *engine*; those
  four remain, and they are the same work under any engine.
- **The cost is real and unmeasured here.** `enhanced-determinism` is mutually
  exclusive with `parallel`, `simd-stable`, and `simd-nightly`. No performance
  comparison was run. For a phone game with modest body counts this is likely
  fine — likely, again, not measured.

## Provenance

| | |
|---|---|
| rustc / cargo | 1.97.1 (pinned in `rust-toolchain.toml`) |
| rapier2d | `=0.35.1` (pinned; `enhanced-determinism`) |
| parry2d / glamx | 0.30.2 / 0.3.0 (resolved) |
| native target | aarch64-apple-darwin |
| wasm target | wasm32-unknown-unknown, run under Node v22.23.2 |
| date | 2026-08-09 |

Rapier 0.35 uses **glam `Vec2`**, not nalgebra vectors, and `PhysicsPipeline::step`
takes gravity **by value** with 12 arguments. Written from the crate source, after
an initial guess against the older nalgebra-style API failed to compile.

## Reproducing

```
cargo test --release                                   # native + the four guards
cargo build --release --target wasm32-unknown-unknown  # wasm module
node verify.mjs                                        # cross-check
```

Resolve cargo through `rustup which cargo` — Homebrew's cargo shadows rustup on
PATH here and has no wasm std, the same trap `fun/tools/build-wasm.sh` documents.
