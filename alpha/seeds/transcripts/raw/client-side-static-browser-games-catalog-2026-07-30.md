# Raw: client-side / static-hostable browser games catalog (Gemini dialogue, 2026-07-30)

**Preservation status: preserved-condensed (cleaned-paste, content-faithful — NOT byte-pristine) —
PLAYBOOK §4.** Source: a pasted Google Gemini research dialogue, 2026-07-30. UI render chrome
stripped ("AI generated" image captions, inline citation chips like "GitHub Pages", "itch.io",
"PCMag", "Reddit", "YouTube" source tags). The user's questions are preserved close to verbatim;
Gemini's answers are reproduced content-faithfully (every named game / project / tool / claim kept;
marketing prose lightly compressed). No secrets.

**Axis / scope.** This dialogue is about **single-player, 100%-client-side, static-hostable**
browser games — games that run entirely on the user's machine (WebGL / WASM / JS) and can be served
as static files with **no backend**. It is a *distinct axis* from the two existing P2P-social games
catalogs (`../../thinking/app/ponds/games-pond-authoritative-list.md`,
`.../p2p-games-pond-launch-set.md`, both 2026-06-21), and it maps to the **as-built** `fun.croft.ing`
single-player shelf, not the (still-gated) P2P pond.

**Flags — ALL repo names, licenses, and port-liveness claims are Gemini-sourced and `[UNVERIFIED]`.**
Examples to confirm against primary sources before acting: TuxRacer.js repo `ebbejan/tux-racer-js`
+ itch publisher `0x00EB`; SuperTuxKart WASM/mobile ports; "Extreme Tux Racer is GPL"; the Frozen
Bubble HTML5/JS ports; individual project licenses throughout. **Legal claims** (95-year corporate
copyright rule; "no commercial game in public domain yet"; Paperboy → Atari Games → Midway → Warner
Bros. Discovery; Internet Archive's DMCA library exemption making iframe embeds legal) are relayed
from the dialogue and flagged for a fact-check pass. No atproto / iroh / iOS claims → the FACTCHECK
source-of-truth doesn't bear here.

**Distilled into** `../../thinking/app/ponds/client-side-static-game-candidates.md` (the catalog,
organized by integration pathway, not ranked). **Drove** two `fun/plans/` docs: the Tier-1 build-fresh
`2026-07-30-bubble-shooter.md` and the Tier-2 wrap-pathfinder `2026-07-30-tux-racer-wrap-spike.md`.
**Owner charter decision 2026-07-30** (recorded COHESION §62): widen `fun.croft.ing` to a **two-tier
shelf** — Tier 1 Croft-native (build-fresh, determinism-first, verifiable) + Tier 2 opportunistic
ethical wraps/ports (already-packaged, static, non-extractive, taken as-is, no verifiable record).

---

## Q — Does the Tux Racer game work in the browser?

Yes. Two routes today:

- **TuxRacer.js (modern web port).** A browser rewrite of the classic Extreme Tux Racer in
  **TypeScript + WebGL (HTML5)** — runs natively in modern browsers, no plugins/downloads.
  Optimized assets; desktop (keyboard/mouse) + mobile (touchscreen virtual joystick) controls.
  Playable on itch.io (publisher **0x00EB**) or self-hosted from source (`ebbejan/tux-racer-js`).
- **tuxracer-web (Docker/VNC streaming).** Runs the native Linux build inside a Docker container and
  streams video to the browser via a VNC→websocket bridge (noVNC): `docker run -p 80:80
  dtcooper/tuxracer-web`. Because it streams video/audio rather than rendering natively, it suffers
  latency/frame-drops. (Not the client-side route.)

## Q — Is there a Tux Racer that runs 100% client-side so it could be served as a static page?

Yes — **TuxRacer.js** (0x00EB). Built entirely in TypeScript + WebGL, it runs 100% client-side; no
backend game server or database. Serve it as static files on GitHub Pages / Netlify / S3 / nginx /
Apache. Pull `ebbejan/tux-racer-js`, run the Node build to compile TS → static JS bundles, drop into
any static dir. Self-contained, with touchscreen controls for mobile alongside desktop keyboard.

## Q — Other games that can be deployed that way in that ecosystem?

The OSS community has ported many classics to the web using **WebAssembly (WASM)**, **WebGL**, and
**Emscripten** (compiles C/C++ → JS/WASM). All run client-side → deployable as static files, zero
backend. Notable:

1. **Super Mario 64 (sm64-web)** — community decompiled SM64 to C, run through Emscripten; native-
   speed WebGL + Web Audio, gamepad support. *Copyright: static deploys usually require you to supply
   your own original ROM to extract assets.*
2. **Quake (WebQuake)** — id Tech 2 open-sourced; full HTML5/WebGL source ports at 60+ FPS; WebSocket
   multiplayer.
3. **Doom (JS-Doom)** — many static HTML5 Doom ports compiled from PrBoom / Chocolate Doom; tiny
   assets, instant load.
4. **Command & Conquer (HTML5 ports)** — Tiberian Dawn recreated in HTML5/JS; OpenRA has experimental
   WASM branches; client-side pathfinding/unit management.
5. **RollerCoaster Tycoon 2 (OpenRCT2)** — OSS re-implementation; WASM/Emscripten branches; guest-AI
   is CPU-heavy so WASM near-native speed matters.

All compile into `.html`/`.js`/`.wasm` and can be dropped into S3 or a GitHub repo, played from a URL.

## Q — Can we find more, esp. fun ones like TuxKart rather than deep strategy? 15 more?

Platformers & jump-'n'-runs:
- **Spelunky Classic HD** (GameMaker → HTML5; procedural generation in-browser)
- **Cave Story** (WASM via CSE2 / NXEngine ports; 60 FPS)
- **VVVVVV** (open-sourced → **VVVVVVwasm**)
- **Prince of Persia (PrinceJS)** — ground-up HTML5/JS recreation of the 1989 DOS classic
- **Celeste Classic (PICO-8)** — PICO-8 exports a static HTML5/JS wrapper
- **SuperTux** — OSS Mario clone; maintains a WASM build pipeline in its repo
- **Commander Keen (Commander Genius)** — DOS classic via the OSS engine, Emscripten→WASM

Racing / 3D / action:
- **SuperTuxKart** — community WASM + mobile ports; drift around tracks in-browser
- **Tomb Raider 1 (OpenLara)** — OSS engine recreation; WebGL/WASM, higher framerate than original;
  hostable on GitHub Pages
- **Half-Life 1 (webXash)** — Xash3D (GoldSrc rebuild) → Emscripten; supply your own asset folders
- **Re-Volt (RVGL Web)** — 1999 RC-car racer; active OSS community, WASM/WebGL
- **Teeworlds Web** — frantic 2D multiplayer arena shooter; C++ client+server → WASM, **WebRTC P2P
  multiplayer directly in-browser, no central server**

Retro & casual:
- **Sonic 1 & 2** — Retro Engine mobile-port decompiles → WASM (need ROMs/assets)
- **Blobby Volley 2** — LAN-party gummy-blob volleyball; lightweight WebGL/JS
- **Tux Paint** — classic OSS kids' drawing tool, HTML5, runs locally

## Q — Keep going on retro & casual fun, 15 more, no more that need ROMs.

Retro PC classics (freely-distributable assets):
- **3D Pinball for Windows – Space Cadet** — reverse-engineered → WASM; assets in OSS repos
- **SkiFree.js** — JS port of the 1991 Win3.1 classic (ski, dodge trees, the Yeti)
- **Gorillas.js** — web recreation of the 1991 QBasic banana-artillery game
- **Minesweeper Classic** — many pixel-perfect HTML5/JS recreations; zero backend

Casual sandbox & physics:
- **Sandspiel** — OSS falling-sand simulator built in **Rust + WASM**; paint water/fire/plant/gas
- **Free Rider JS** — OSS HTML5 track-drawing bike-physics game
- **WebLiero** — browser port of Liero (1998 real-time DOS Worms-like); assets included; local MP
- **Numpty Physics** — OSS crayon-physics; Emscripten port

Arcade & action:
- **Friday Night Funkin'** — HaxeFlixel → HTML5; OSS; hosted on Newgrounds/GitHub
- **OpenTyrian (WebTyrian)** — OSS port of Tyrian (1995 vertical shmup); assets made freeware →
  legal, no ROM
- **Pacman-Canvas** — ground-up OSS HTML5/JS recreation
- **Clumsy Bird** — OSS Flappy Bird clone on MelonJS; lightweight static-host example
- **T-Rex Runner (Chrome Dino)** — Chromium's offline game; source open; extractable as standalone

Twitch puzzles:
- **2048** — Gabriele Cirulli's OSS HTML/CSS/JS; no server
- **Hextris** — OSS HTML5 (Tetris meets Super Hexagon)
- **Sokoban (HTML5)** — countless OSS native-JS implementations, static

*(Also named in passing: **OpenLara** running in-browser as a WebGL showcase.)*

## Q — Those are awesome, find more new ones.

High-end WebGL racing/action:
- **HexGL** — Wipeout-style 3D futuristic racer; single-dev WebGL/HTML5 showcase, no plugins
- **Slow Roads** — procedurally-generated casual driving (Three.js); endless scenery
- **Swoop** — 3D biplane-through-rings arcade flight; OSS PlayCanvas showcase
- **Underrun** — fast twin-stick shooter; whole game (engine + audio) in a few KB of pure JS
- **osu!web** — OSS client-side web implementation of the osu! rhythm game; local beatmap cache

Modern indie hits (web prototypes):
- **Vampire Survivors (original prototype)** — first build was HTML5 on **Phaser**; still playable
  as a static webpage on itch.io
- **CrossCode (web demo)** — built on the HTML5 engine **ImpactJS**; standalone client-side demo
- **Celeste Classic 2 (Lani's Trek)** — PICO-8 sequel; static HTML5 wrapper

Addictive arcade & strategy:
- **Shapez.io (open-source edition)** — casual factory/shape-combining on an endless grid
- **BrowserQuest** — Mozilla's OSS HTML5 mini-MMO tech demo; self-contained arcade hack-and-slash
- **A Dark Room** — OSS text-based incremental; DOM elements only
- **Entanglement** — relaxing HTML5 tile-laying path puzzle (a Chrome experiment)

Casual sandbox & puzzle:
- **Blockrain.js** — OSS falling-block (Tetris-style) clone in pure JS; drop into any static page
- **Little Alchemy 2** — client-side element-combining (start from air/earth/fire/water); *not
  strictly OSS but free HTML5, self-contained*
- **Particle Clicker** — OSS incremental/idle game themed on particle physics (from CERN devs);
  static HTML/JS

## Q — Games for kids in this category?

Tux4Kids (web ports):
- **TuxMath (Tux of Math Command)** — arcade math (defend igloos from comet equations); C/SDL →
  Emscripten/WASM, client-side
- **Tux Math Scrabble** — build math equations on a Scrabble-like board; officially ported
  Python/PyGame → HTML5/JS by Charles Cosse; static
- **Tux Typing** — keyboarding game (Tux eats lettered fish); community WASM ports
- **Tux Paint** — kids' drawing tool, HTML5, local

Make-your-own — Scratch:
- **Scratch 3.0 (MIT)** — the whole editor is a client-side HTML5/JS app
- **TurboWarp Packager / HTMLifier** — OSS tools that package any Scratch project into a single
  self-contained HTML file (engine + logic + assets); host on GitHub Pages / S3, runs at 60 FPS
  without Scratch's servers

Browser-native logic/coding games:
- **Flexbox Froggy & Grid Garden** (Codepip) — OSS CSS-layout puzzles; fully client-side
- **Blockly Games** (Google) — OSS visual-block programming games; download as a static folder
- **CodeCombat (client-side modules)** — has a MP backend, but many OSS levels run client-side JS

Classic educational suites:
- **GCompris** — 150+ activities for ages 2–10; mostly native today, but OSS HTML5/JS
  implementations of classic mini-games exist for static portals
- **Little Alchemy** — free HTML5 element-combining; popular in schools; client-side, self-contained

## Q — Other fun games that can be deployed this way?

Arcade & action (highly visual):
- **Astray** — WebGL marble maze (Three.js + Box2D)
- **Captain Rogers: Asteroid Belt** — polished HTML5 arcade space-survival; OSS by Enclave Games
- **Monster Wants Candy** — kid-friendly HTML5 tapping game
- **Agent 8 Ball** — arcade HTML5 pool with bombs/power-ups

Physics & sandbox:
- **Numpty Physics** (Emscripten) · **Couch 2048** (physics 3D spin on 2048) · **Pond** (relaxing
  HTML5 swim-and-explore)

Puzzle & logic:
- **BlicblockJS** — OSS match-three/tetromino puzzle (based on the Sims 4 "Blicblock" minigame)
- **CSS Diner** — CSS-selector puzzle (serve plates/apples/bento) · **WarriorJS** — write raw JS to
  move a knight through a dungeon · **Cube-Composer** — logic puzzle inspired by functional
  programming · **TransCube** — transform-into-blocks 2D puzzle platformer

## Q — So Emscripten is a WASM porting library or what?

Not a library — a full **compiler toolchain** (a heavy-duty translator). Browsers can't run C/C++
(what ~99% of classic desktop games are written in); they run JS and WASM. Emscripten bridges the
gap:

1. **Translates the code (LLVM).** Uses the LLVM compiler infrastructure to output `.wasm` instead
   of a native `.exe`/binary.
2. **Fakes the hardware APIs.** Desktop games use OS libs (SDL, OpenGL) the browser lacks; Emscripten
   intercepts and translates on the fly: OpenGL → WebGL; desktop audio → Web Audio API; keyboard/
   mouse → HTML DOM events.
3. **Simulates a hard drive.** Browsers can't freely read the local filesystem; Emscripten packs the
   game's assets into a **Virtual File System (VFS)** living in browser RAM. The engine thinks it's
   reading `level1.dat` from `C:\`; it's really reading memory Emscripten set up.

Without it, every game above would need a from-scratch JS rewrite; with it, a 25-year-old C codebase
compiles for the web in an afternoon.

## Q — Where are the classics like Brickles, etc.? Any Mario-style games or retro classics?

**Brickles & early Mac classics.** Nobody rewrote Brickles' exact source in HTML5, but you can play
the original client-side via **in-browser operating systems**:
- **Infinite Mac** — Emscripten ports of the **Basilisk II / SheepShaver** Mac emulators to WASM;
  boots System 7 / Mac OS 9 in a browser tab in seconds, pre-loaded with shareware — the original
  **Brickles, Glider PRO, Crystal Quest, Lode Runner**, no install.
- **HTML5 Breakout clones** — block-breakers are the "Hello World" of web game dev; GitHub has
  thousands of pure JS/Canvas Breakout/Arkanoid clones for single-page static hosting.

**Mario-style platformers (client-side, no ROMs/emulators):**
- **MariOCaml** — the SMB physics/momentum/rendering engine rewritten in **OCaml**, transpiled to
  HTML5/JS; plays in a canvas, hostable anywhere
- **Super Mario Bro(w)s(er)** — modern web platformer on **PixiJS**; self-contained HTML5; standard
  Mario mechanics with procedurally-scrambled levels generated in-browser
- **Secret Maryo Chronicles (SMC)** — famous OSS Mario clone; community WASM experiments
- **Frogatto & Friends** — gorgeous OSS pixel-art platformer on the custom **Anura** engine (OSS →
  compiled to run in modern browsers via WebGL)

**The ultimate retro web hack — JS-DOS.** DOSBox ported to WASM. Put a classic DOS game's files in a
`.zip`, place it next to the static js-dos HTML page, and on visit the browser downloads the ZIP,
boots a virtual DOS machine locally in RAM, and launches the game — zero backend, 100% local. This is
how the Internet Archive serves thousands of MS-DOS games. (Named DOS titles: Jazz Jackrabbit,
Commander Keen, Duke Nukem II.)

## Q — Can I embed Internet Archive games on my own page? Top 10 in that category?

Yes — the Internet Archive supports native embedding of MS-DOS and Internet Arcade titles via a
standard HTML `<iframe>` (emulators DOSBox + JSMESS are JS/WASM, run inside the iframe, no plugins).
Steps: open the game's Archive page → Share / "Embed This Item" → copy the `<iframe src="...">` →
paste into your HTML (adjust width/height).

Top 10 iconic titles to embed:
- Arcade: **Street Fighter II**, **Galaga** (Namco, 1981), **Frogger** (1981), **Paperboy**
  (isometric newspaper delivery), **Space Harrier** (Sega, 1985 rail shooter)
- MS-DOS: **The Oregon Trail**, **DOOM** (id, 1993), **Prince of Persia** (1989), **Alone in the
  Dark** (1992, 3D survival-horror pioneer), **SimCity** (Maxis, 1989)

## Q — Are Atari games public domain? Is Paperboy?

**No** — neither Atari games nor Paperboy are public domain. Almost every commercial video game since
the 1970s is still under copyright.
- **95-year rule.** US corporate "works for hire" (studio games like Atari's) are protected for **95
  years from first publication**. The industry is young enough that **no commercial game has aged
  into the public domain yet** (first Atari games ~2070s).
- **Paperboy.** Released 1984 by Atari Games; copyright never lapsed. Atari's arcade division → Midway
  Games; Midway bankrupt 2009 → IP bought by Warner Bros. So **Warner Bros. Discovery** currently owns
  Paperboy.
- **Why it's on the Internet Archive.** The Archive is a 501(c)(3) digital library operating under
  hard-won **DMCA exemptions for software preservation**. You can safely embed the Archive's iframe
  because the Archive is the entity legally hosting/serving the files. You **cannot** legally extract
  the ROM, host it yourself, sell it, or remake it for profit.

## Q — Is there a version where you shoot colored balls up into others, match 3 touching?

Yes — the **"Bubble Shooter" / "Match-3 Bubble"** genre. The originator is Taito's 1994 arcade classic
**Puzzle Bobble** (aka **Bust-a-Move** in North America). In the OSS / static web ecosystem:

1. **Frozen Bubble** — the ultimate OSS clone of Puzzle Bobble (rounds out the Tux ecosystem: you
   play Tux, operating a bottom cannon shooting colored frozen bubbles upward; three-or-more same-
   color touching shatters). Originally written in **Perl**; community-ported to pure **HTML5/JS**;
   plays in-browser, hostable on static pages / itch.io.
2. **Bubble Shooter (HTML5 clones)** — the mechanic became a generic term; often endless survival
   variants (ceiling lowers to crush you). The hex-grid snapping math is a staple web-dev exercise,
   so GitHub is full of polished OSS HTML5 Bubble Shooter clones to self-host statically.
   (A "Build a Puzzle Bobble clone in JS" tutorial genre exists for the exact tile-matching mechanic.)
