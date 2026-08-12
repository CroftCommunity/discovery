# Emoji Wars / Levelforge — the three modes, then the reshape and the tier decision (Claude, 2026-08-09 → 08-11)

- **Source:** a claude.ai dialogue the owner pasted into a Claude Code session in `CroftC/` on
  2026-08-09, followed by a live design conversation in that same session running to 2026-08-11.
- **Preservation status:** **preserved-condensed (cleaned-paste, content-faithful — §4)** for Part 1
  (the pasted claude.ai turns; UI chrome stripped — collapsed thinking-summary headers, "Ran a
  command", "Claude's response was interrupted.", and the artifact cards `Levelforge spec ·
  Document · MD`, `Claude code build plan · Document · MD`, `Levelforge · Code · HTML`).
  **Preserved-verbatim (in-session)** for Part 2, the live design conversation, whose owner turns are
  byte-exact.
- **Credentials:** none present; none redacted.
- **The three deliverables are filed separately and verbatim** as `seeds/emojiwars-unpacked/` (from
  `emojiwars.zip`) — referenced, not re-embedded, per §4.
- **Why this file matters:** Part 2 contains **two owner decisions that changed the shelf's
  standards** — Emoji Wars moves into `fun`, and the shelf gains a **third tier**. Both are recorded
  in `fun/docs/BUILDING-GAMES.md` §11.

---

## Part 1 — the pasted claude.ai dialogue (cleaned-paste)

### 1a. The build plan

> Ok give me comprehensive Claude code instructions to execute and build this mode with needed
> refactor

**Assistant.** The build plan is the third and final handoff file, deliberately operational where the
spec is definitional. What it adds:

- **The refactor, made structural.** Phase 2's rule is that **no module may read a global world
  size** — dimensions flow from the level object or as parameters, everywhere. The reference already
  behaves this way as of v0.12, so Claude Code's job is enforcing it architecturally, not re-deriving
  it. Phase 4 does the same for modes: instead of the prototype's **inline `isDrop()` branches**
  (which it is explicitly told not to copy), sling and drop become implementations of a **`GameMode`
  interface**, so drive mode later is a third *file*, not a third *branch*.
- **Fixtures baked in.** Both demo levels are inlined as exact JSON — the wide sling house and the
  tall drop descent — so acceptance testing does not depend on anyone regenerating them. **Row 15 of
  the verification matrix is the round-trip oracle:** a level from the port pasted into the reference
  must behave identically.
- **A 16-row verification matrix** covering everything phone-tested that session, from magnet flush
  contact and nudge-batch undo through coyote-window hops and the tall-aware agent brief, with the
  kickoff prompt asking Claude Code to separate what it can unit-test from what needs the owner's
  thumbs.

> Your handoff is now: create the repo, commit SPEC.md, BUILD_PLAN.md, reference/levelforge.html, and
> the two fixture files at the paths phase 0 lists, then paste the kickoff prompt from the bottom of
> the build plan.

**One honest caveat** (assistant's own): the fixture JSONs were **derived from the demo-generator
code rather than from a live export**, so before committing them, load each via Paste & Load in v0.12
once — if both load clean and play, the fixtures are verified oracles.

### 1b. Bounce mode was dropped, and is not optional

> I do note this demo html didn't keep bounce mode and we want to
>
> And all 3 modes are still important, drop, bounce, sling

**Assistant.** *(Acknowledged glossing over bounce when the three were first listed, and went to check
the transcript rather than speculate. The response was interrupted mid-search.)*

> Bounce mode is basically red ball style play

**Assistant.** Both files are patched.

**Spec.** `meta.mode` is now **`"sling" | "drop" | "bounce"`**, all three first-class. There is a full
**Bounce mode** section: Red Ball style traversal, launcher as left-side spawn, lethal villains with
drop's respawn semantics, **🏁 goal rendered as a flag** instead of a catch tray, movers emphasised
since they were Red Ball-inspired from the first demo. One tidy discovery while patching: **the spec
already had this mode sketched in milestone 5 under the name "drive"** — folded into bounce rather
than leaving two names for one idea. Milestone 5 now says all three ship polished.

**Build plan.** Phase 4 gains **`modes/bounce.ts`** with the rule that **drop's grounded/coyote logic
is extracted into a shared helper both modes use**. Tuning constants (`JUMP`, `MAX_ROLL`,
`ROLL_ACCEL`, `TAP_MS`) join `HOP` and `COYOTE_MS` in `tuning.ts`. **Fixture C** (first-crossing, wide
desert: cactus to jump, ramp to a high road over a lurking villain, descending elevator, flag) is
inlined, plus matrix rows 17–20 and an updated kickoff prompt. A new guardrail tells Claude Code that
**bounce has no reference oracle**, so it must not invent beyond the spec and should mirror drop where
the spec is silent.

**The one design decision the assistant made for the owner:** the control scheme is
**hold-left/right-half to roll** (torque-ramped, so momentum and slopes stay physical) with a **quick
tap for jump**. Marked as a proposal to validate on the phone, and the mode-strategy interface means
swapping schemes touches one file. Same derived-not-phone-verified caveat applies to fixture C.

---

## Part 2 — the live reshape conversation (verbatim owner turns)

### 2a. The reframe

> see emojiwars.zip
>
> this one I def want to talk about, the build has gone on, but I'm realizing the initial shape needs
> to change and want to talk on preservering existing things and creating a better foundation and
> porting over

Grounding established in-session against `CroftCommunity/levelforge` (cloned at `e4f5adc`):

| Finding | Evidence |
|---|---|
| **Build-plan Phase 2 is already done** — "No module reads a global world size" | `FOLLOWUP.md`, verified by grep; `WIDE`/`TALL` are schema presets, backdrops take `(W,H,fy)` |
| `grounded.ts` already extracted, commented **"shared-ready for bounce"**; bounce tuning constants already placeheld | `src/play/grounded.ts`, `src/play/tuning.ts` |
| **`src/main.ts` is 3,721 lines** — 39% of a 9,642-line TS codebase | `wc -l` |
| **14 inline `level.meta.mode ===` branches; no `GameMode` interface exists** | `grep` |
| Repo modes are **`slingshot \| drop \| drive`**; spec says `sling \| drop \| bounce`; `drive` reads a `meta.goal` zone where spec says `role:'goal'` | `src/schema.ts:58`, `FOLLOWUP.md` naming note |
| **No lobby / level-select module** — it lives inside `main.ts` | `grep` found none |
| Schema at **v0.8** with a `migrate()`; deploys to `levelforge.croft.ing` | `FOLLOWUP.md` |

**The reframe that followed:** the problem is narrower than "the initial shape." The structural half
of the refactor is banked. What remains is (1) extracting modes out of `main.ts` behind an interface,
and (2) reconciling the names, which `FOLLOWUP.md` had already deferred to "a future pass."

### 2b. The owner's actual shape

> explain the port quesiton again in plain english and emojiwars is already under way at
> levelforge.croft.ing, but I realized as I was building it that it belons in fun. and taht it was
> not just a level builder but that the player and builder were inseparable and that I needed a
> frontstage and backstage separation and a lobby too so the talk is how to take what's good about
> what we have and put it back together in a firmer shape, esp since I think I have 2 other canvas
> draw/media use cases in mind, maybe we separate that out entirely and build a dialect for it

Three things named here, recorded as distinct threads:

1. **Emoji Wars belongs in `fun`.** (Decided.)
2. **Frontstage / backstage / lobby.** This names the `main.ts` defect better than "extract a
   `GameMode` interface" did: the file is that size **because the distinction was never in the
   code**. Note the product/code split — player and builder are inseparable *as a product*, entirely
   separable *as code*.
3. **A canvas/media dialect across three use cases.** Grounded: paint/canvas concerns already touch
   **9 files, ~246 references**, concentrated in `main.ts`, `editor/render.ts`, `editor/backdrops.ts`,
   `editor/geometry.ts`, plus the paint-tool kit from commit `e4f5adc`. It is **a smear, not yet a
   layer**. Recommendation given: **extract a library now** (concrete, from working code); **defer the
   dialect** until the second use case shows what is actually common, since designing a contract from
   one real use case plus two imagined ones fits the imagined ones badly.

### 2c. The tier question, and the measurement that settled it

Emoji Wars fits **neither** existing shelf tier: not Tier-2 (that is an existing game taken as-is;
this one is ours) and not Tier-1 (matter-js is float JS physics and cannot carry §3's verifiable
outcome). `docs/BUILDING-GAMES.md:54` forbids faking one.

The owner raised **Rapier** as a path — correctly noting the assistant's "fixed-point" framing was too
narrow, since Rapier reaches cross-platform determinism via strict IEEE-754 plus its opt-in
`enhanced-determinism`, at the cost of `simd-*` and `parallel`.

> fine let's run the spike now

**Result** (`discovery/alpha/experiments/rapier-determinism`, commit `eb70cff`):

```
                          native (aarch64-darwin)   wasm32-unknown-unknown
enhanced-determinism ON    2135883295078246327       2135883295078246327    MATCH
enhanced-determinism OFF  14104404486998819895       2135883295078246327    DIVERGE
```

The control is the more useful half: **wasm produces the same digest either way; only native moves.**
`wasm32-unknown-unknown` has no platform math library, so it was already using the `libm` crate —
`libm_force`'s real job is dragging *native* onto the implementation wasm already had. **The
divergence risk lives on the native side**, which is exactly the axis `fun`'s Tier-1 `native == wasm`
cross-check tests.

### 2d. The decision

> I changed my mind, I don't want to tyr to build a determinstic engine for emoji wars, let's use the
> one we have that is pixel based, and this is anotehr tier of games based on non-determinstic libs
> but that are not straight ports

**Ratified as `fun/docs/BUILDING-GAMES.md` §11 — Tier-3, engine-backed originals** (commit `10df5fe`
on `claude/tier3-engine-backed`). Two rules carry it:

- **Share inputs, never outcomes.** A level, seed, or challenge is data and reproduces; a result
  presented as a record cannot be re-derived downstream. This is the implementable form of §9's
  "honestly represented."
- **The data/sim line must be visible in the directory structure.** Tier-3 is not Tier-1 minus
  rigour — it is Tier-1 discipline applied to the half that can carry it. Schema, migrations and pure
  rules keep golden vectors and mutation testing; only the engine's integration gives that up, and it
  pins behaviour with **tolerance probes** recorded from the engine's own run-to-run variance.

Tier-3 is **stricter** than Tier-2 where Tier-2 is exempt (tap-first, settings, full-surface
accessibility), because nothing foreign is executing and there is nothing to contain.

**Recorded in §11 and worth repeating:** the code does not yet know Tier-3 exists. `src/contract.ts`
is a union of `tier?: 1` and `tier: 2`, so `tier: 3` will not typecheck, and `wrapped-banner.ts`
returns `null` unless `tier === 2` — shipping a Tier-3 game without widening that check would put an
**unmarked non-verifiable game on the shelf**, the exact failure the honesty rule prevents. Both are
test-first work for the first Tier-3 game, not speculative groundwork.

## Still open (owner's, not resolved here)

- **The two other canvas/media use cases** — unnamed. This blocks scoping the library extraction.
- **Frontstage / backstage / lobby** — agreed as the right cut; no plan written.
- **The physical move into `fun`** — decided in principle. Emoji Wars has no Rust core, so `fun`'s
  per-game layout (`crates/<game>-core`, `crates/<game>-wasm`) does not fit it as written; §11 covers
  the standards, not the file layout.
- **The `slingshot→sling` / `drive→bounce` rename + schema migration** — scheduled by §11's existence
  but not planned.

## Distilled to

- `fun/docs/BUILDING-GAMES.md` **§11** + the three-tier framing + a Tier-3 checklist; `fun/CLAUDE.md`
  shelf-model section (commit `10df5fe`).
- `discovery/alpha/experiments/rapier-determinism/RESULT.md` (commit `eb70cff`) — the evidence.
- `fun/plans/2026-08-09-emojiwars-physics-parity-harness.md` — **SUPERSEDED**, kept because the road
  not taken is what makes the tier a decision rather than a default.
- `ROADMAP_TODO.md` **E99**; `COHESION.md` **§71**.
