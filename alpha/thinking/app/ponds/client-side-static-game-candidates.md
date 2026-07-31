# Client-side / static game candidates — the single-player shelf axis

author: distilled from the 2026-07-30 Gemini dialogue
(`../../seeds/transcripts/raw/client-side-static-browser-games-catalog-2026-07-30.md`)

date: 2026-07-30

scope: candidate games that run **100% client-side** (WebGL / WASM / JS) and can be served as
**static files with no backend** — the axis that maps to the **as-built** `fun.croft.ing`
single-player shelf. **Not ranked** (owner instruction): organized by *integration pathway*, which
is what determines effort and which shelf tier a candidate lands in.

related: `games-pond-authoritative-list.md` and `p2p-games-pond-launch-set.md` (both 2026-06-21) are
the **P2P / group-chat-social** catalogs — a different axis (they rank for friend-to-friend, present-
together fun over iroh/webxdc transport). This doc is the single-player/static complement; it extends
that family, it does not replace or re-rank it.

> **All repo/license/liveness claims below are Gemini-sourced and `[UNVERIFIED]`.** License and
> "already-packaged non-extractive" status are the two gates that actually decide inclusion, and both
> must be confirmed against primary sources per-candidate before any adoption.

---

## The charter decision (owner, 2026-07-30) — a two-tier shelf

`fun.croft.ing` **widens** from "determinism-first-verifiable only" to a **two-tier shelf**. Both
tiers share the non-negotiables: **client-side, static, non-extractive, accessible-where-possible**.
They differ on whether the game carries a *verifiable outcome*.

```
┌─ TIER 1 — Croft-native ──────────────────────┐  ┌─ TIER 2 — opportunistic wrap / port ─────────┐
│ build-fresh, determinism-first.               │  │ already-packaged ethical games, taken AS-IS.  │
│ Rust core → wasm; move-list replay →          │  │ We do NOT rebuild them. NO verifiable record  │
│ state_hash; verifiable outcome + re-verifying │  │ — and the shelf SAYS SO (honest representation)│
│ ?r= share; tap-first, core decides legality.  │  │ Fit our UI/UX + drawer/URL chrome.            │
│ solitaire · match-3 · (bubble shooter, next)  │  │ TuxRacer.js · HexGL · Sandspiel · Astray · …  │
└────────────────────────────────────────────────┘  └────────────────────────────────────────────────┘
   verifiable | tiny bundle | tap-first              opportunistic | may be large | native input
```

"Determinism-first-verifiable" (Tier 1) means: same seed + same moves always yields the same result
bit-for-bit (native == wasm), so a finished game emits a tiny `(seed, moves, result, hash)` record
that **anyone can re-replay to check** — a win is proven, not trusted, with no server. It is a
property of the *result*, orthogonal to packaging/webxdc (how a game is bundled/transported). A Tier-2
game can be beautifully packaged yet have no verifiable outcome; a Tier-1 game is a tiny bundle that
is fully verifiable.

## The Tier-2 inclusion filter (the "opportunistic" gate)

Be opportunistic about games **already packaged in a way that fits our style**, even ones we would
never build Tier-1 from scratch. A candidate is admissible to Tier 2 iff **all** hold:

1. **Already fully client-side / static** — runs in-browser, no backend, servable as static files.
2. **Non-extractive** — no ads, no tracking / telemetry-home, no account-as-data-grab, no dark
   patterns. This is the ethical gate and the whole point of the shelf.
3. **Redistribution-licensed** — an OSS or freeware-assets license that lets us vendor, host, and
   attribute (copyleft is allowed but changes obligations; flag it).
4. **Fits our UI/UX chrome** — mounts in the `GameModule` drawer contract, gets its own `/<id>/` URL,
   works in all three chrome modes; themeable is a bonus, cleanly-framed is the floor.
5. **Honestly represented** — if it has no verifiable outcome, the shelf must not imply it does.

**Bundle weight is NOT a hard disqualifier — it is a disclosed class (owner decision 2026-07-30).**
A large one-time download that then runs **fully offline with no further bandwidth** is a legitimate
shelf class (the "big-download-then-offline" games), *provided the download size is disclosed up
front, before the user commits to it.* So leg 1's "static" bar is about *architecture* (no backend),
not *size*. The instant-start ideal stays the default; heavy titles are admitted as their own labelled
class with an honest "≈N MB download, then offline" notice. (SuperTuxKart, ~120 MB, is the first of
this class — see bucket B.)

Explicitly **disqualified by the filter:** anything that needs an original **ROM / copyrighted asset**
the user must supply, and any **emulator/iframe-host** path where the game is not ours to
redistribute (bucket D below). Fun, but off-ethic to self-host and/or not license-clean.

---

## Candidates by integration pathway

### A — Build-fresh, determinism-first (Tier 1; native to the current recipe)

Rules simpler than an integration; clean license; verifiable outcome native. Same path as solitaire /
match-3.

- **Bubble shooter** — Puzzle Bobble / Bust-a-Move mechanic; **Frozen Bubble** is the OSS homage
  (build fresh, don't port its GPL/Perl lineage). *The extracted Tier-1 candidate → `fun/plans/
  2026-07-30-bubble-shooter.md`.*
- **2048** · **Sokoban** · **Minesweeper** · **Hextris** · **Tetris-family** (Blockrain.js) ·
  **BlicblockJS** (match-3 / tetromino).

### B — Wrap a self-contained web game (Tier 2; the opportunistic path)

Run a JS/WebGL bundle as-is behind the drawer contract. Real-time / arcade; no move-list verification;
per-game license + bundle weight are the filters. *Pathfinder → `fun/plans/2026-07-30-tux-racer-wrap-
spike.md`.*

- **Racing / 3D:** **TuxRacer.js** *(the extracted Tier-2 candidate)* · **SuperTuxKart — ADOPTED
  (owner, 2026-07-30) as the first "big-download-then-offline" class member.** Verified: real WASM
  port (`ading2210/stk-code` wasm branch; live at supertuxkart.pages.dev), **GPL**, **~120 MB initial
  download / ~500 MB RAM, experimental (networking off)** — a one-time download that then runs
  offline, so it ships with an up-front size disclosure rather than being disqualified on weight. ·
  HexGL (MIT, complete — the lowest-risk *first* Tier-2 exemplar) · Slow Roads · Swoop · Astray ·
  OpenLara · Re-Volt (RVGL) · Captain Rogers.
- **Platform / action:** Prince of Persia (PrinceJS) · Friday Night Funkin' · Pacman-Canvas · Clumsy
  Bird · T-Rex Runner (Chrome Dino) · Underrun · Agent 8 Ball · Monster Wants Candy · osu!web.
- **Physics / sandbox:** Sandspiel (Rust + WASM) · Free Rider JS · Numpty Physics · Couch 2048 ·
  WebLiero.
- **Relaxing / incremental:** A Dark Room · Particle Clicker · Shapez.io (OSS edition) · BrowserQuest
  · Pond · Vampire Survivors (original Phaser prototype) · CrossCode (ImpactJS web demo) · Little
  Alchemy / 2 *(free but not OSS — check the filter)*.
- **Retro PC (freeware assets):** 3D Pinball Space Cadet · SkiFree.js · Gorillas.js.

### C — Emscripten C/C++ port (Tier 2 if no ROM; heavier)

Engine + asset bundles compiled C/C++ → WASM; per-project license.

- **No ROM (OSS / freeware assets) — filter-passable:** SuperTux · Cave Story (CSE2 / NXEngine) ·
  VVVVVV (wasm) · Celeste Classic + Celeste Classic 2 (PICO-8) · Commander Keen (Commander Genius) ·
  OpenTyrian · Spelunky Classic HD · **Teeworlds Web** *(WebRTC P2P — a natural bridge to the P2P
  pond)* · Blobby Volley 2 · Frogatto · Secret Maryo Chronicles.
- **Needs original ROM/assets — DISQUALIFIED by filter #2/#3:** Super Mario 64 (sm64-web) · Quake
  (WebQuake) · Doom (JS-Doom) · Half-Life (webXash) · Sonic 1 & 2 · C&C / OpenRA · OpenRCT2.

### D — Emulator / iframe host (mostly DISQUALIFIED)

Hosts *others'* games; ROM + legal caveats. Off-ethic to self-host; legal only as an Archive-hosted
embed.

- Infinite Mac (System 7 / OS 9 → Brickles, Glider PRO, Crystal Quest, Lode Runner) · JS-DOS
  (DOSBox / WASM) · **Internet Archive embeds** (DOSBox + JSMESS; legal *only* as Archive-hosted
  iframes — Paperboy/Atari are **not** public domain; 95-yr corporate-copyright rule) · MariOCaml ·
  Super Mario Bro(w)s(er) (PixiJS).

### E — Kids / educational (cross-cuts A–C; the values-showcase subset)

- **Tux4Kids:** TuxMath · Tux Math Scrabble · Tux Typing · Tux Paint.
- **Coding / logic:** Blockly Games · Flexbox Froggy · Grid Garden · CSS Diner · WarriorJS ·
  Cube-Composer · CodeCombat.
- **Make-your-own:** Scratch 3.0 + TurboWarp Packager / HTMLifier (static export). **Suite:** GCompris
  (HTML5 experiments).

---

## Toolchain / engines named (reference)

**Emscripten** (LLVM; C/C++ → WASM; fakes SDL/OpenGL → WebGL, desktop audio → Web Audio, input → DOM
events, disk → a RAM virtual filesystem) · **PICO-8** (fantasy console, exports static HTML5) ·
**Phaser** · **MelonJS** · **PixiJS** · **PlayCanvas** · **HaxeFlixel** · **ImpactJS** · **Three.js**
· **Box2D**.

## Legal facts asserted (flagged for a fact-check pass)

`[UNVERIFIED]`: the 95-year corporate-copyright term; "no commercial video game is public domain
yet"; Paperboy → Atari Games → Midway → Warner Bros. Discovery ownership chain; the Internet
Archive's DMCA library exemption making iframe embeds legal while self-hosting the ROM is not.

## What this drove

- **Charter decision** → COHESION §62 (two-tier shelf); this doc's tier model + inclusion filter.
- **Tier-1 build** → `fun/plans/2026-07-30-bubble-shooter.md` (ungated, single-player, verifiable).
- **Tier-2 pathfinder** → `fun/plans/2026-07-30-tux-racer-wrap-spike.md` (license/weight/fit gates +
  the reusable wrap + honest-representation standard).
- **SHIPPED 2026-07-31** → `fun/plans/2026-07-31-tier2-wraps.md`. Tier 2 went from zero shipping
  games + a drafted standard to **three live wraps** + the reusable machinery. The follow-on below
  is done: the wrapped-game standard is **ratified** in `fun/docs/BUILDING-GAMES.md` §9 (plus a
  step-by-step porting recipe + mobile/desktop guidance), with **Astray** as the reference impl.
- **Open follow-on (DONE 2026-07-31):** the wrapped-game addendum to `fun/docs/BUILDING-GAMES.md`
  now encodes the Tier-2 filter + honest-representation rule as shelf standards (§9), shipped with
  the first wraps rather than deferred.

## Tier-2 shipped — recon corrections to the candidates above (2026-07-31)

The candidate licenses/postures were `[UNVERIFIED]` (Gemini-sourced). Confirmed against primary
sources when building the first three wraps (`fun/plans/2026-07-31-tier2-wraps.md`):

- **Astray** — VERIFIED **The Unlicense** (public domain), self-contained ~10 static files, zero
  egress. Promoted to the **pathfinder** (cleaner than HexGL), shipped at `/astray/`.
- **HexGL** — VERIFIED **MIT**, self-contained WebGL; 2012 Three.js runs with no shim. Bundled
  Google Analytics — **stripped at vendor time**. Shipped at `/hexgl/` (~17 MB, disclosed).
- **Clumsy Bird** — VERIFIED **GPL-3.0** (copyleft; source-offer recorded), prebuilt MelonJS
  bundle, zero patches, zero egress. Shipped at `/clumsybird/` as the third wrap.
- **REJECTED on the inclusion filter, not license:** **Pacman-canvas** (Pac-Man is a Namco/Bandai
  **trademark**; also ships `ads.txt` + a highscore backend + hot-linked sounds) and **T-Rex
  Runner** (Google's Chromium asset/brand). Trademark on a name/character disqualifies a wrap even
  when its code license is clean — a filter refinement worth carrying forward.
- **Containment posture (as built):** `iframe[sandbox="allow-scripts"]` (opaque origin, no
  `allow-same-origin`) + a same-origin egress allowlist, proven by a real-browser gate. Every wrap
  ships a `tier2.meta.json` (provenance + posture). Avoid the Emscripten + runtime-asset-untar class
  (the SuperTuxKart cut, `fun/plans/2026-07-31-supertuxkart-wrap.md`).
