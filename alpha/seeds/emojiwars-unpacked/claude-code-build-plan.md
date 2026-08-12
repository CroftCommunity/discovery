# EMOJI WARS · BUILD PLAN (execution companion to SPEC.md)

This document tells Claude Code exactly how to execute the build: repo bootstrap, the port of the reference prototype, the world-dimensions refactor, and drop mode implemented behind a game-mode abstraction. SPEC.md holds the full product and schema definition; this file holds the order of operations, the module plan, and the acceptance checks. When the two disagree on behavior, `reference/levelforge.html` (forge v0.12) wins, then SPEC.md, then this file.

Scope of this plan: SPEC.md milestone 1 (faithful port, including the world refactor) plus the drop-mode portion of milestone 5. PWA, persistence, level select, and the rest stay in SPEC.md for later passes.

## Phase 0 · Repo bootstrap

1. `npm create vite@latest emoji-wars -- --template vanilla-ts`, then `npm i matter-js` and `npm i -D @types/matter-js vitest`.

2. tsconfig: `strict: true`, `noUncheckedIndexedAccess: true`.

3. Commit the three seed files at the paths below before writing any code:

```
/SPEC.md
/BUILD_PLAN.md                  (this file)
/reference/levelforge.html      (forge v0.12, schema v0.8)
/levels/demo/villain-house.json (fixture A, sling, wide; JSON below)
/levels/demo/first-descent.json (fixture B, drop, tall; JSON below)
/levels/demo/first-crossing.json (fixture C, bounce, wide; JSON below)
```

4. Vite serves `index.html` mounting a single full-viewport canvas plus the DOM chrome (topbar, tray, inspector, modals). Port the reference's CSS nearly verbatim; it encodes phone-tested sizing.

## Phase 1 · Schema module first

Create `src/schema.ts` before any rendering or physics. It is the source of truth.

- Types: `Level`, `Meta` (name, scene, gravity, note, hero, mode: 'sling' | 'drop' | 'bounce', background, backgroundImage), `World` (w, h, floorY), `Slingshot`, and a discriminated union `Obj` over shapes box, circle, tri, emoji, blob, with the common fields id, x, y, angle, material, anchored, path, note, and optional role ('goal').

- Constants: `WIDE = {w:1600, h:900}`, `TALL = {w:900, h:1600}`, `floorY = h - 40`, `SNAP = 10`, materials table exactly as SPEC.md.

- `migrate(raw: unknown): Level`: accept schema 0.2 through 0.8, fill defaults (hero, mode 'sling', background 'grid', world WIDE with floorY, backgroundImage null, drop custom background lacking an image). Re-seed the id counter from the max existing id on load.

- `emptyLevel(shape: 'wide' | 'tall')` mirroring the reference.

- Vitest: `migrate` round-trips both fixture files unchanged apart from filled defaults; `migrate(JSON.parse(JSON.stringify(level)))` is idempotent.

## Phase 2 · The world refactor (do it during the port, not after)

The rule: **no module reads a global world size**. Reference v0.12 already made world dimensions level data (`level.world`); the port's job is to make that structural.

- Everything that needs dimensions takes them as parameters or reads them from the `Level` it was handed: backdrop painters `(ctx, world)`, board renderer, thumbnail renderer, view fitting (`fit = min(cw/world.w, chh/world.h)`), magnet floor candidate (`world.floorY`), physics floor and side walls, out-of-bounds checks, the agent brief generator (dimensions and floor percentage are computed, see `agentBrief()` in the reference).

- Forbidden: exporting `WORLD_W`-style constants for consumption outside `schema.ts` presets. `WIDE`/`TALL` exist only as preset values the shape switcher writes into a level.

- ⚙ shape switch behavior (port from reference): snapshot for undo, write the other preset into `level.world`, clamp the slingshot into bounds, keep all object coordinates, toast a count of pieces now out of bounds.

- Acceptance: load fixture A (wide) then fixture B (tall) in one session; view fitting, backdrops, magnet floor, physics walls, and the agent brief all follow the level with no reload.

## Phase 3 · Port the forge (edit mode)

Port from reference v0.12 module by module. Suggested decomposition:

```
src/forge/view.ts        pan/zoom state, world<->screen transforms, resize
src/forge/render.ts      board, backdrops, objects, selection, start pad vs launcher
src/forge/gestures.ts    pointer state machine: stamp, paint, move+lift+magnet,
                         pinch/twist, path drag, sling drag, view pan/zoom
src/forge/magnet.ts      extents + candidate snapping (floor from level.world)
src/forge/tray.ts        shape arming, icons, afterPlace setting
src/forge/inspector.ts   materials, backdrop row, behavior chips incl 🏁, readout
src/forge/nudge.ts       pad, quadrant-opposite placement, burst-batched undo
src/forge/history.ts     snapshot undo/redo
src/forge/modals.ts      help, settings, schema, note+dictation, emoji, library, brief
```

- Port the interaction checklist in SPEC.md item by item; the tuning numbers there (lift 80/zoom, magnet 14/zoom+4, hit pad 12/zoom+4, decimation 0.6 x brush, 70-point cap, 1.5 s nudge batching) are load-bearing.

- Fix the known rough edge while porting: landscape rails must fit without scrolling at common phone sizes.

- Acceptance: every row of the Phase 5 verification matrix passes on a phone; a level built in the reference pastes into the port and renders identically.

## Phase 4 · Play runtime with a mode strategy

Refactor Test mode so game modes are pluggable rather than branched inline (the reference branches inline with `isDrop()`; do not copy that shape).

`src/play/runtime.ts` owns what modes share:

- fresh engine per entry, gravity from meta, floor and side walls from `level.world`

- body construction per shape (compound blobs, goal bodies static + isSensor)

- movers (ping-pong via setPosition plus matching setVelocity)

- ice melt (skip goal bodies), compound-parent resolution helper

- draw pass for bodies and backdrop, HUD frame

`src/play/modes/types.ts`:

```ts
interface GameMode {
  init(rt: Runtime): void;                       // spawn hero, counters
  onPointerDown(p: Vec): void;
  onPointerMove(p: Vec): void;
  onPointerUp(): void;
  onCollisionStart(pairs: Pair[]): void;
  onCollisionActive(pairs: Pair[]): void;
  tick(dtMs: number): void;                      // respawns, out-of-bounds
  drawHero(ctx: CanvasRenderingContext2D): void;
  drawOverlay(ctx: CanvasRenderingContext2D): void;  // aim UI, HUD text
  hint(): string;                                // playhint copy
}
```

`src/play/modes/sling.ts` (parity port from reference):

- hero static at launcher until released; drag capped at 200; launch velocity = pull x 0.16; dotted trajectory preview; auto-reload ~3.8 s

- break system registered by this mode only: 500 ms settle grace, impact vs static = speed x 0.55, vs dynamic = speed x min(mass,10) x 0.3, parent-resolved pairs

- villain counter from target-material non-goal objects; zero -> LEVEL CLEAR

`src/play/modes/drop.ts`:

- hero spawns dynamic at the start point (the slingshot field reinterpreted); friction 0.35, restitution 0.15, r 22

- grounded tracking: on collisionStart/Active pairs involving the hero (parent-resolved, sensors excluded), if any contact support point lies below hero center + 0.35 r, record `lastGround = now`

- tap anywhere: if `now - lastGround < COYOTE_MS`, set velocity y to `-HOP` keeping x, clear lastGround. No aiming, no pull

- hazards: contact with target-material body -> red flash, attempt++, teleport hero to start with zeroed velocity and spin. Falling past `world.h + 200` counts the same

- goal: contact with a `role:'goal'` sensor -> cleared, LEVEL CLEAR banner; input ignored after

- no break system in drop mode; melt still runs

- hero render: plain disc `#ffd93d` with dark outline and small highlight while speed >= 1.2; the emoji face at rest and after clearing

`src/play/modes/bounce.ts` (no reference implementation; SPEC.md Bounce mode section is authoritative):

- hero spawns dynamic at the start point with the same body parameters as drop (r 22, friction 0.35, restitution 0.15); wide is the expected world shape but nothing depends on it

- extract drop's grounded-contact and coyote logic into a shared helper (e.g. `src/play/grounded.ts`) and use it from both modes; do not duplicate it

- input: track held pointers by screen half. While a pointer is held left of center, drive the hero toward `-MAX_ROLL` angular velocity via `Body.setAngularVelocity` stepped by `ROLL_ACCEL` per tick (torque-style ramp, not an instant write, so slopes and momentum stay physical); right half mirrors. A press released within `TAP_MS` with under ~12 px of movement is a jump: if grounded-or-coyote, set velocity y to `-JUMP` keeping x

- villains and out-of-world falls: identical semantics to drop (flash, attempt++, respawn at start), shared code path, not a copy

- goal: `role:'goal'` sensor contact clears the level, same as drop; runtime renders goal bodies as a flag on a post when mode is bounce, catch tray when drop

- no break system; melt runs

- hero render: emoji glyph rotating with the body so rolling reads; if it tests poorly on the phone, reuse drop's moving-disc render

- `src/play/tuning.ts` holds every feel constant with a comment that these are phone-tuned placeholders: `HOP = 13`, `COYOTE_MS = 130` (shared by both jump modes), `JUMP = HOP`, `MAX_ROLL = 0.45` (rad/tick-scale angular velocity cap), `ROLL_ACCEL = 0.05`, `TAP_MS = 180`. Nothing outside this file hardcodes them

Editor tie-ins (small; drop parts port from reference, bounce parts are new but symmetric): ⚙ mode switch cycles sling → drop → bounce (switching to drop moves a low start point near the top; switching to bounce moves it near the left floor), start-pad rendering in edit and thumbnails for both non-sling modes, 🏁 behavior chip toggling `role:'goal'`, mode-aware playhint ("hold a side to roll · tap to jump" for bounce).

## Phase 5 · Verification matrix

Run on a phone. Each row is a manual test; fixtures are the two demo files.

| # | Action | Expect |
|---|--------|--------|
| 1 | Load fixture A, ▶, fling at the house | wood breaks, villains pop, LEVEL CLEAR at 0 |
| 2 | Fixture A: wait ~15 s in Test with an ice piece added | ice shrinks then vanishes; nothing shatters in the first 0.5 s |
| 3 | Stamp a plank, drag near another | lifts above finger; edges snap flush; release keeps flush contact |
| 4 | Select a piece in each screen quadrant | nudge pad appears diagonally opposite every time |
| 5 | Nudge 6 times fast, then ↶ once | all six squares revert together |
| 6 | Paint a blob, ▶ | one rigid piece; topples as a unit |
| 7 | 🖼 upload an image | cover-fit backdrop, floor line still visible, 7th swatch appears |
| 8 | 📋 on a tall level | brief states 900 x 1600 and the tall floor percentage |
| 9 | ⚙ shape switch on a populated level | coordinates kept; toast counts out-of-bounds pieces |
| 10 | Load fixture B, ▶ | hero rolls from start pad; plain yellow disc while moving |
| 11 | Fixture B: tap while airborne | nothing (no double jump) |
| 12 | Fixture B: tap just after rolling off an edge | hop still fires (coyote) |
| 13 | Fixture B: touch the cactus | red flash, respawn at start, attempt counter +1 |
| 14 | Fixture B: land in the catch tray | LEVEL CLEAR |
| 15 | Copy schema from port, paste into reference v0.12, Load | identical render and behavior (sling and drop levels; bounce has no reference) |
| 16 | Paste a schema 0.4 level (no mode/world fields) | migrate fills defaults, loads clean |
| 17 | Load fixture C, ▶, hold right half of screen | hero rolls right, accelerates smoothly, climbs the ramp with momentum |
| 18 | Fixture C: quick tap while rolling | jump only when grounded; coyote works off the platform edge; holding a side never triggers a jump |
| 19 | Fixture C: roll into the cactus | red flash, respawn at start, attempt +1 |
| 20 | Fixture C: reach the flag at the right edge | LEVEL CLEAR |

## Fixture A · levels/demo/villain-house.json

```json
{
  "schemaVersion": "0.8",
  "meta": {"name":"villain-house","scene":"demo","gravity":1,"note":"","hero":"🙂","mode":"sling","background":"grass","backgroundImage":null},
  "world": {"w":1600,"h":900,"floorY":860},
  "slingshot": {"x":230,"y":770},
  "objects": [
    {"id":"o1","shape":"box","x":1100,"y":820,"w":30,"h":80,"angle":0,"material":"wood","anchored":false,"path":null,"note":""},
    {"id":"o2","shape":"box","x":1240,"y":820,"w":30,"h":80,"angle":0,"material":"wood","anchored":false,"path":null,"note":""},
    {"id":"o3","shape":"box","x":1170,"y":765,"w":220,"h":26,"angle":0,"material":"wood","anchored":false,"path":null,"note":""},
    {"id":"o4","shape":"blob","x":960,"y":740,"angle":0,"material":"stone","anchored":true,"path":null,"brushR":26,"pts":[[-60,110],[-40,40],[-10,-30],[20,-90],[45,-110]],"note":"painted stone spire"},
    {"id":"o5","shape":"emoji","x":1170,"y":727,"r":26,"angle":0,"material":"target","emoji":"👿","anchored":false,"path":null,"note":"villain on the roof"},
    {"id":"o6","shape":"emoji","x":1170,"y":822,"r":26,"angle":0,"material":"target","emoji":"🎃","anchored":false,"path":null,"note":"villain in the house"},
    {"id":"o7","shape":"box","x":560,"y":600,"w":180,"h":22,"angle":0,"material":"metal","anchored":true,"path":{"x":820,"y":600,"speed":90},"note":"moving platform"}
  ]
}
```

## Fixture B · levels/demo/first-descent.json

```json
{
  "schemaVersion": "0.8",
  "meta": {"name":"first-descent","scene":"demo","gravity":1,"note":"tap-to-hop descent; reach the tray without touching a villain","hero":"🙂","mode":"drop","background":"night","backgroundImage":null},
  "world": {"w":900,"h":1600,"floorY":1560},
  "slingshot": {"x":140,"y":110},
  "objects": [
    {"id":"o1","shape":"box","x":260,"y":300,"w":420,"h":22,"angle":12,"material":"wood","anchored":true,"path":null,"note":""},
    {"id":"o2","shape":"emoji","x":430,"y":266,"r":22,"angle":0,"material":"target","emoji":"🌵","anchored":true,"path":null,"note":"hop this"},
    {"id":"o3","shape":"box","x":620,"y":520,"w":440,"h":22,"angle":-14,"material":"wood","anchored":true,"path":null,"note":""},
    {"id":"o4","shape":"emoji","x":470,"y":478,"r":22,"angle":0,"material":"target","emoji":"👿","anchored":true,"path":null,"note":""},
    {"id":"o5","shape":"box","x":280,"y":760,"w":440,"h":22,"angle":12,"material":"wood","anchored":true,"path":null,"note":""},
    {"id":"o6","shape":"emoji","x":410,"y":722,"r":22,"angle":0,"material":"target","emoji":"🎃","anchored":true,"path":null,"note":""},
    {"id":"o7","shape":"box","x":600,"y":1000,"w":420,"h":22,"angle":-12,"material":"wood","anchored":true,"path":null,"note":""},
    {"id":"o8","shape":"box","x":450,"y":1180,"w":200,"h":22,"angle":0,"material":"metal","anchored":true,"path":{"x":700,"y":1180,"speed":80},"note":"moving step"},
    {"id":"o9","shape":"box","x":450,"y":1548,"w":240,"h":24,"angle":0,"material":"stone","anchored":true,"path":null,"note":"catch tray","role":"goal"}
  ]
}
```

## Fixture C · levels/demo/first-crossing.json

```json
{
  "schemaVersion": "0.8",
  "meta": {"name":"first-crossing","scene":"demo","gravity":1,"note":"roll right, jump the cactus, take the ramp over the villain, reach the flag","hero":"🙂","mode":"bounce","background":"desert","backgroundImage":null},
  "world": {"w":1600,"h":900,"floorY":860},
  "slingshot": {"x":120,"y":770},
  "objects": [
    {"id":"o1","shape":"emoji","x":560,"y":838,"r":22,"angle":0,"material":"target","emoji":"🌵","anchored":true,"path":null,"note":"jump over this"},
    {"id":"o2","shape":"box","x":860,"y":800,"w":240,"h":22,"angle":-18,"material":"wood","anchored":true,"path":null,"note":"ramp up"},
    {"id":"o3","shape":"box","x":1090,"y":720,"w":260,"h":22,"angle":0,"material":"wood","anchored":true,"path":null,"note":"high road"},
    {"id":"o4","shape":"emoji","x":1090,"y":838,"r":22,"angle":0,"material":"target","emoji":"👿","anchored":true,"path":null,"note":"lurks under the high road"},
    {"id":"o5","shape":"box","x":1330,"y":640,"w":180,"h":22,"angle":0,"material":"metal","anchored":true,"path":{"x":1330,"y":800,"speed":70},"note":"elevator down, time the hop off"},
    {"id":"o6","shape":"box","x":1530,"y":848,"w":100,"h":24,"angle":0,"material":"stone","anchored":true,"path":null,"note":"goal flag","role":"goal"}
  ]
}
```

## Guardrails

- The Copy / Paste and Load loop must work after every phase; it is the product.

- Never mutate level data from physics; Test builds and discards its own world.

- Any schema-affecting change bumps `schemaVersion` and extends `migrate` in the same commit, with a fixture test.

- Keep every feel constant (`HOP`, `COYOTE_MS`, `JUMP`, `MAX_ROLL`, `ROLL_ACCEL`, `TAP_MS`) in `tuning.ts` only; nothing else hardcodes them. They are placeholders until phone testing confirms them.

- Bounce mode has no reference implementation. Do not invent behavior beyond SPEC.md's Bounce mode section; where that section is silent, mirror drop mode's decision.

- Do not delete or "improve" the reference file; it is the behavioral oracle.

## Kickoff prompt (paste into Claude Code at the repo root)

"Read SPEC.md, BUILD_PLAN.md, and reference/levelforge.html in full before writing code. Execute BUILD_PLAN.md phases 0 through 4 in order, committing per phase with the phase name in the commit message. Phase 4 ships all three game modes behind the GameMode interface: sling and drop match the reference file, which is the behavioral oracle for them; bounce has no reference, so implement it strictly from SPEC.md's Bounce mode section and this plan, mirroring drop where they're silent. After phase 4, print the Phase 5 verification matrix (rows 1 through 20) as a checklist for me to run on my phone against the three demo fixtures, noting which rows you could verify yourself with unit tests (schema round-trips, migrate idempotence, magnet candidate math, break-model thresholds, grounded/coyote logic) and which need my manual pass. Do not start PWA, persistence, or level-select work; those are later milestones in SPEC.md."
