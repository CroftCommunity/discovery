# EMOJI WARS · build spec for Claude Code

## What this is

Emoji Wars is a touch-first 2D physics game plus its level editor (the "forge"). The player flings a hero emoji from a launcher at villains, which are emoji stamped in a fragile target material. Destroy all villains to clear the level.

The product's real center is the **level schema**: a JSON format that a human edits visually on a phone and an AI collaborator reads and writes as text in chat. Every design decision protects that loop: human builds in the forge, copies schema JSON into a Claude conversation for feedback or co-design, pastes Claude's revisions back, and loads them. Both sides edit the same artifact.

`reference/levelforge.html` (forge v0.11, schema v0.7) is a working single-file prototype of the whole forge plus test-play. It was iterated on-device through many rounds of phone testing and encodes hard-won interaction decisions. **Treat it as the behavioral reference: port it faithfully first, then extend.** When this document and the reference disagree on a tuning value, the reference wins.

## Target and stack

- Deployment: static site, PWA (Progressive Web App: installable, offline-capable via service worker), on a subdomain of croft.ing. Own repo. No backend; levels are files in the repo.

- Stack: Vite + TypeScript. matter-js from npm, bundled (no CDN). Plain canvas rendering as in the reference; no framework needed for the forge, though a light one is acceptable for the level-select shell if it stays static-deployable.

- Phone-first: all interactions must work with touch as the primary input. Mouse works as a degraded fallback.

## Repo layout

```
/src
  schema.ts        source of truth: types, defaults, migrate()
  materials.ts     the materials table below
  forge/           editor: canvas, gestures, tray, inspector, modals
  play/            physics runtime: world build, break model, melt, movers
  shell/           level select, scenes, PWA plumbing
/levels
  /<scene>/
    <level>.json
    backdrop.png       optional, referenced by meta.backgroundSrc
    /sprites/          optional custom emoji images, referenced by object.sprite
/reference
  levelforge.html   the prototype; keep checked in, never load in prod
/public              manifest, icons, service worker registration
```

## Coordinate system and world constants

- World dimensions are level data, not constants: `world.w`, `world.h`, `world.floorY` in the schema, with two editor presets: wide 1600 x 900 and tall 900 x 1600 (floorY = h - 40 in both). Tall is first-class: it suits drop mode and portrait play, and everything (backdrops, magnet floor, physics walls, thumbnails, the agent brief's stated dimensions and floor percentage) must derive from the level's world, never from a global. Switching shape in ⚙ keeps object coordinates and warns if pieces land out of bounds.

- Origin top-left, y down. Grid snap 10. Angles in degrees in the schema (matter-js uses radians internally; convert at the boundary).

- Edit mode is a frozen god view: no gravity, objects sit exactly where placed. Test mode (▶) builds a fresh matter-js world from the schema; leaving Test discards it. The level data is never mutated by physics.

## Schema v0.7 (current)

```jsonc
{
  "schemaVersion": "0.7",
  "meta": {
    "name": "untitled",
    "scene": "",              // grouping key for the library and level select
    "gravity": 1,             // multiplier on engine gravity.y
    "note": "",               // level-wide intent note
    "hero": "🙂",             // emoji flung from the launcher; level override
    "background": "grid",     // grid|grass|cave|desert|night|sky|custom
    "backgroundImage": null   // prototype only: dataURL. Build: replace with backgroundSrc (see below)
  },
  "world": { "w": 1600, "h": 900, "floorY": 860 },
  "slingshot": { "x": 230, "y": 770 },
  "objects": [ /* see object shapes */ ]
}
```

Object common fields: `id` ("oN", N monotonically increasing; loader must re-seed the counter past the max), `x`, `y` (center), `angle` (degrees), `material`, `anchored` (bool, static body), `path` (null or `{x, y, speed}`: patrol endpoint plus speed in units/sec, ping-pong motion, implies static/kinematic), `note` (string, human intent that travels with the level).

Object shapes:

- `box`: `w`, `h`.

- `circle`: `r`.

- `tri`: `w`, `h`. Apex-up wedge with centroid at origin; vertices `(-w/2, h/3), (w/2, h/3), (0, -2h/3)`.

- `emoji`: `r`, `emoji` (glyph). Round physics body, emoji glyph as skin. An emoji in `target` material is a villain.

- `blob`: `pts` (array of `[dx, dy]` relative to center, decimated to >= 0.6 x brush spacing, max 70), `brushR`. A painted freeform stroke. Physics: compound body of overlapping circles of radius `brushR` at each point. One rigid piece; melts and breaks as a unit.

`migrate(level)` must accept every prior schema version (0.2 onward) and fill defaults: missing `meta.hero` -> "🙂", missing `meta.background` -> "grid", `background:"custom"` without an image -> "grid", missing per-object fields -> shape defaults. Bump `schemaVersion` on any additive change and extend `migrate`; never break the paste-and-load loop.

### Schema v0.8 (to implement during milestones 2 to 4)

- `meta.backgroundSrc`: filename of a backdrop image in the level folder, replacing the prototype's inline `backgroundImage` dataURL. `migrate` converts old inline images by writing the asset out (editor-side) or, at minimum, keeps rendering them.

- `object.sprite`: filename under the level's `sprites/` for custom emoji, i.e. user images used as stamps. Physics stays a circle of `r`; the image is the skin. This is the "custom emoji" feature.

- `object.group`: string weld-group key. Objects sharing a group are welded into one compound body in Test.

- `object.role` on target-material objects: `"destroy"` (default, villain) or `"protect"` (hostage variant: breaking it fails the level). Additionally `role: "goal"` on any object marks a goal zone for drop and bounce modes: physics isSensor (detects overlap without colliding), rendered as a catch tray in drop and as a goal flag in bounce.

- `meta.mode`: `"sling"` (default), `"drop"`, or `"bounce"`. See the Drop mode and Bounce mode sections. All three are first-class shipped modes.

- `object.hit`: optional per-emoji hit behavior key (e.g. `"explode"` for 💣). Registry of behaviors lives in code; schema stores only the key.

## Materials

Density, friction, restitution are matter-js body properties. `breakAt` is the impact threshold in the break model below; null means unbreakable.

| material | color   | density | friction | restitution | breakAt | special |
|----------|---------|---------|----------|-------------|---------|---------|
| wood     | #b5824c | 0.0010  | 0.40     | 0.20        | 9       | grain lines |
| stone    | #8e939c | 0.0025  | 0.60     | 0.10        | 20      | speckle |
| metal    | #a9b6c4 | 0.0040  | 0.30     | 0.05        | null    | riveted plate look |
| ice      | #9fd6ea | 0.0009  | 0.05     | 0.10        | 6.5     | melts in Test: Body.scale by 0.9985 per frame, removed below 30 percent |
| rubber   | #d94f5c | 0.0012  | 0.90     | 0.85        | null    | bounce |
| target   | #7bc86c | 0.0008  | 0.50     | 0.30        | 3.2     | the villain material; face drawn on plain circles |

## Break model (Test mode)

- Skip all break checks for the first 500 ms (settle grace). History: without this, ice pieces resting on the floor shattered at world start and looked like they "fell through the floor".

- On collisionStart, resolve each body to its compound parent if it is a part (`body.parent !== body`), then compute relative speed between the pair.

- Impact against a static body: `speed * 0.55`. Against a dynamic body: `speed * min(otherMass, 10) * 0.3`.

- If impact exceeds the material's `breakAt`, remove the body. If it was target material with role destroy, decrement the villain counter; at zero, LEVEL CLEAR.

## Forge interaction model (port checklist)

Modes and arming:

- ✋ adjust is the default. Tray tiles arm a shape; tapping the board stamps it centered under the finger in the current material. Setting `afterPlace` (⚙): `"adjust"` (default) disarms, selects the new piece, and lets the same touch continue as a drag; `"stamp"` keeps the tool armed.

- 🖌 paint arms the blob brush: drag paints a stroke (live ghost preview), release creates the blob centered on its centroid.

- 🔍 view toggles pan (one finger), pinch zoom 1x to 4x, double-tap reset. Zoom indicator shown when zoomed.

Selection and manipulation:

- Tap selects (generous hit pad, 12/zoom + 4). Drag moves; on touch, after 10 px the piece lifts 80/zoom units above the fingertip so the finger never hides it.

- Magnet: while dragging axis-aligned boxes, circles, and emoji, edges and centers snap flush to neighbors' edges/centers and to the floor, threshold 14/zoom + 4 units. Axes the magnet claimed skip grid snap on release so flush contact survives. Tris and blobs are exempt.

- Pinch resizes the selection; twist during pinch rotates freely. ⟳ (topbar and behavior row) rotates 90 degrees; with a plank armed and nothing selected, topbar ⟳ flips the stamp orientation.

- Nudge pad: appears only when a piece is selected, positioned in the screen corner diagonally opposite the piece's quadrant so it never covers it, re-evaluated only on selection change or gesture end so it stays put during a burst of taps. Each tap moves exactly one grid square with no magnet interference, preserving any flush off-grid offset. Nudges within 1.5 s batch into one undo step.

- Behavior row (right rail in landscape): ⟳ rotate, ⚓ anchored, ↔ moving (adds a patrol path with a draggable endpoint), ✎ note, ✕ delete, plus a live readout `x,y · size · angle`.

- Undo ↶ / redo ↷: full-level JSON snapshots taken at gesture start, stack capped (40 in the prototype because snapshots may embed the backdrop; with asset refs, 80+ is fine).

Rails and layout:

- Portrait: canvas, then shape tray, then inspector (materials row, backdrop row, behavior row).

- Landscape: left rail = ✋ plus shapes (select and place side), right rail = materials, backdrop, behavior (modify side). Known rough edge in the prototype: rails can overflow and scroll on some phones; the port must fit both rails without scrolling at common phone sizes.

Notes and dictation:

- ✎ opens a note editor per object; meta.note for the level. 🎤 uses the Web Speech API when the platform provides it, with graceful fallback messaging. Notes are the designer's intent in words and must round-trip through the schema untouched.

Emoji picker:

- 10 recents, a curated quick palette, and a free text input whose job is to open the OS emoji keyboard, which is the full-universe fallback.

- Milestone 4 replaces this with a searchable Slack-style picker: bundle an emoji name database (emojibase data or the emoji-picker-element web component) so search works offline, keeping recents on top and the OS keyboard input as a final fallback.

Backdrops:

- Six procedural styles drawn in canvas: grid, grass, cave, desert, night, sky. Selected in a swatch row; stored in `meta.background`; rendered in edit, Test, and library thumbnails. In edit mode a faint alignment grid overlays any non-grid backdrop.

- Custom: 🖼 upload. Prototype embeds a dataURL in `meta.backgroundImage`; the build stores a file and `meta.backgroundSrc`. Render cover-fit to 1600 x 900 and always draw a translucent floor strip plus line so the physics ground stays legible over any art.

- 📋 agent brief: a copyable prompt for image-generation agents that fixes form and function (exact 1600 x 900, ground reads at 96 percent height, low-contrast central band, detail toward top and edges, no text/logos/vignette, avoid hues near #7bc86c and #ff8a3d) and leaves a marked slot for the user's style notes. Copy the text verbatim from the `AGENT_BRIEF` constant in the reference.

Library and schema modals:

- 💾 library: named saves grouped by scene with canvas thumbnails, load and delete. Prototype uses window.storage or session memory; the build uses IndexedDB or localStorage, plus export/import of level JSON as files (this is how levels graduate into `/levels` in the repo).

- { } schema modal: pretty-printed JSON, Copy, Paste and Load (through `migrate`), Demo level, Clear. This is the collaboration surface; it must never lag behind the schema.

Test mode (▶):

- Fresh engine per entry, gravity from meta, floor plus side walls, bodies from objects (anchored or pathed implies static), movers ping-pong along their paths at `speed` via setPosition plus matching setVelocity per frame, ice melts, break model as above.

- The hero: a circular body skinned with `meta.hero`, loaded at the launcher; drag back to set the shot (capped at 200 units), dotted trajectory preview while aiming, auto-reload a few seconds after firing. Villain counter on screen; LEVEL CLEAR banner at zero. ■ returns to edit with the level untouched.

## Game shell (new in the build)

- Level select page: read `/levels/<scene>/*.json` at build time (Vite glob import), group by scene, thumbnail cards, play-only route that loads straight into Test mode with edit hidden.

- Hero progression: the player picks a hero once and it persists (their default across levels); `meta.hero` acts as a per-level override when a level demands a specific hero.

- Win/lose flow: clear banner, retry, next level in scene.

## Drop mode

A second game mode using the same schema and the same object vocabulary with inverted semantics. Reference inspiration: tap-to-hop descent games where a ball rolls down angled rails toward a basin. A rough but playable version exists in reference v0.12 (⚙ mode switch, 🏁 chip, Drop demo); its `HOP` and `COYOTE_MS` constants are placeholders pending phone tuning, and drop levels default to the tall world shape.

- `meta.mode: "drop"`. The launcher marker becomes the spawn point; the hero spawns there and gravity does the rest.

- Input: a tap anywhere applies a jump impulse, allowed only while grounded (the hero has an active contact whose collision normal points upward). This keeps jumps rhythmic instead of flappy. Impulse magnitude, the grounded tolerance window, and whether to add coyote time (a short grace period after rolling off an edge during which a jump still counts) are feel-critical: tune on a phone, do not trust desktop.

- Villains (target-material emoji) are hazards: any contact restarts the level. Nothing is destroyed in drop mode.

- Win: the hero overlaps an object with `role: "goal"`. Editors mark any object as the goal via a 🏁 behavior chip; it renders as a catch tray and is a physics sensor.

- Hero rendering while rolling: plain disc in the hero's color (yellow default), no glyph. The physics body spins; the skin does not show it. The face appears at spawn and on reaching the goal. Billboarding (face drawn upright regardless of body angle) is the fallback if the plain disc tests poorly.

- Level design implication: drop levels are built from the same planks, tris, blobs, and moving platforms; anchored is the norm. The forge needs no new tools, only the mode switch in ⚙ and the 🏁 chip.

## Bounce mode

The third game mode, and the Red Ball half of the game's parentage (the project began as "a cross between Red Ball and Angry Birds"; sling is the Angry Birds half). The hero is a ball you roll across the level, jumping over hazards and gaps in platforms, to reach a goal. Unlike drop mode there is no reference implementation; this section plus the build plan are authoritative, and the control feel is phone-gated the same way drop's hop was.

- `meta.mode: "bounce"`. Wide is the natural world shape. The launcher marker is the spawn point, typically at the left; the hero spawns there as a dynamic ball (same body as drop: r 22, friction 0.35, restitution 0.15) and the player traverses rightward, though nothing enforces direction.

- Input (proposed scheme, validate on phone): hold the left or right half of the screen to roll that way, implemented as torque toward a capped angular velocity so slopes and momentum stay physical rather than as direct velocity writes. A quick tap (press and release under ~180 ms with little movement) is a jump, allowed only when grounded, reusing drop mode's grounded-contact and coyote-time logic exactly. Rhythm should feel like drop's hop with steering added.

- Villains (target-material objects) are lethal on contact: red flash, attempt counter increments, respawn at the spawn point, same semantics and shared code path as drop. Falling out of the world counts the same. Roadmap option, not v1: landing on top of a villain destroys it, Mario style.

- Win: overlap an object with `role: "goal"`, marked with the same 🏁 chip. In bounce it renders as a goal flag on a post rather than a catch tray.

- No break system, same as drop; ice still melts; movers matter a lot here since the moving platform was Red Ball-inspired from the first demo.

- Hero rendering: the emoji glyph rotates with the body so rolling reads visually. If a spinning glyph tests poorly on the phone, fall back to drop's plain disc while moving.

- Tuning constants live in `tuning.ts` beside HOP and COYOTE_MS: roll torque (or max angular velocity) and jump velocity, which may simply share HOP. All are placeholders until phone testing.

## Milestones

1. **Faithful port.** TypeScript modules, bundled matter-js, feature parity with reference v0.11 per the checklist above, including the settle grace, magnet rules, lift offset, nudge pad placement, and afterPlace behavior. Fix the landscape rail overflow. Acceptance: a level built in the reference pastes into the port and behaves identically in Test.

2. **PWA and persistence.** Manifest, service worker, offline load. Library on IndexedDB. Export/import level JSON files. Backdrop becomes an asset file with `meta.backgroundSrc` (schema 0.8 partial, migrate handles inline images). Deploy docs for the croft.ing subdomain.

3. **Game shell.** Level select, scenes, play-only route, hero progression, win/lose flow.

4. **Collab depth.** Searchable emoji picker with bundled name data. Custom emoji sprites (`object.sprite`, files under `sprites/`). Weld groups (`object.group`). Protect-role targets. Blob fracture (split a compound at the impact point instead of vanishing). Per-emoji hit behaviors registry (💣 explode first). Trajectory preview via a short headless engine rollout. Audio notes as asset files if cheap.

5. **Additional modes.** Sling, drop, and bounce are all first-class and all ship polished; sling lands with the milestone 1 port, drop and bounce land here (each per its own section: mode switch in ⚙, spawn-point reinterpretation of the launcher, goal sensor, hazard semantics, mode-specific input and hero render). What was earlier sketched as a separate "drive" mode is bounce; it is no longer a distinct roadmap item. The GameMode strategy interface in the build plan exists so any future fourth mode is one new file plus a widened `meta.mode` union.

## Working agreements

- The schema is the contract. Any change bumps `schemaVersion`, extends `migrate`, and keeps Copy / Paste and Load working in the same release.

- Every milestone gates on phone testing, not desktop.

- Prefer honest physics over faked effects; where a simplification is chosen (e.g. blobs as circle compounds), document the seam in code comments.
