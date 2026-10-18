# Systems tracker

Every system the port must cover, so nothing is left out. Built from the
engine map (`research/engine-map/engine_map.tsv`, 66,259 functions in
FalloutNV.exe 1.4.0.525) and the translation markers on 2026-10-14. The
per-function truth is docs/LEDGER.md. This page groups functions into
systems and says what "done" means for each one.

A system is done when three things are true:
1. **Translated:** every function in its units carries a `Translated from`
   marker (or `Platform: replaces` for library code).
2. **Switched on:** the viewer runs the translation in play instead of the
   nv-rs rewrite.
3. **Verified:** an acceptance route or a recording from the original game
   matches.

## Order (maintainer, 2026-10-14)

Movement and physics first, then animation, audio, the renderer, and then
the gameplay layer. The plan is to finish the feel, look and play in about
six weeks and the whole game in about two months. Each step ends with a play
build. Game threads are stepped on the main thread behind one seam in
`units/platform.rs`, so real OS threads can replace them later.

## Coverage by code group

"Covered" counts translated, partial and table-run static initializer
functions. Regenerate the numbers before relying on them.

| Code group | Functions | Covered | % | System(s) | Phase |
|---|---:|---:|---:|---|---|
| compiler: dynamic initializers | 16,213 | 15,709 | 97 | Start-up (table-run) | done |
| compiler: atexit destructors | 5,020 | 4,791 | 95 | Shut-down (table-run: 4,698 `call` and 92 `tail` rows, 1 by hand; 229 refused, left for hand translation) | D |
| (unplaced: no unit in the map) | 10,142 | 1,218 | 12 | Spread across every system; assign by caller | all |
| fallout shared (forms, references, world data) | 8,761 | 7,043 | 80 | Forms, records, references, cells, extra data | C |
| fallout/ai | 4,344 | 2,952 | 68 | AI, processes, movement, combat AI | A (movement), C |
| fallout/interface | 2,519 | 1,261 | 50 | Menus, HUD, interface manager | C |
| Havok SDK | 5,081 | 1 | 0 | Physics engine (only 79 functions ported in crates/physics, B1) | A |
| BSHavok | 1,570 | 144 | 9 | Bethesda's Havok layer: character controller, ragdolls, constraints | A |
| NiCollision | 177 | 0 | 0 | Gamebryo collision | A |
| fallout shared/havok | 338 | 137 | 41 | Game-side physics: water, camera caster, listeners | A |
| fallout shared/pathfinding | 1,562 | 1,097 | 70 | Navmesh, pathfinding | C |
| NiAnimation | 1,102 | 0 | 0 | Animation: controllers, blending, keyframes | A |
| FaceGen + fallout shared/facegen | 593 | 13 | 2 | Faces, head morphs, lip sync | A |
| BSAudio + fallout/audio | 460 | 306 | 67 | Audio | A |
| NiMain | 1,489 | 131 | 9 | Scene graph, cameras, properties | B |
| BSShader | 1,548 | 3 | 0 | Shaders, passes, accumulators, image space (game code, ADR-0007; counts 2026-10-17) | B |
| NiXenonRenderer + d3d9/d3dx9 | 976 | 7 | 1 | Gamebryo Direct3D 9 renderer, translated onto our own Direct3D 9 layer (ADR-0007; counts 2026-10-17) | B |
| NiParticle | 633 | 0 | 0 | Particles, effects | B |
| SpeedTree + fallout shared/speedtree | 496 | 22 | 4 | Trees | B |
| BSMain | 819 | 237 | 29 | Scene helpers, culling, debris, scene graph | B |
| fallout shared/sky | 147 | 64 | 44 | Sky, weather, sun, clouds | B |
| fallout shared/distant terrain system | 174 | 46 | 26 | Distant land, LOD | B |
| fallout shared/tempeffects | 135 | 37 | 27 | Temporary effects: decals, blood, shaders on actors | B |
| fallout shared/region system | 113 | 34 | 30 | Regions: weather, sounds, objects | B/C |
| fallout/magic + fallout shared/magic | 809 | 161 | 20 | Magic, effects, chems, radiation, addictions | C |
| fallout/dialogue | 73 | 0 | 0 | Dialogue | C |
| fallout/misc (save/load and more) | 941 | 705 | 75 | Saves, misc game systems | D |
| BSMenu | 369 | 321 | 87 | Menu framework: tiles, fonts | C |
| BSCore + BSSystem + BSSystemUtilities | 334 | 76 | 23 | Memory, threads, files, strings | platform |
| BSMovie | 47 | 1 | 2 | Bink video playback (decoder in crates/bink) | D |
| NiSystem, BSDevices, BSDiag, XAPILIB, RemoteLog | 68 | 3 | 4 | Platform glue, input devices, diagnostics | platform |
| LIBCMT, libcpmt, zLib | 200 | 16 | 8 | C runtime, zlib (platform replacements or translation) | platform |
| fallout/debug, fallout shared/settings | 20 | 1 | 5 | Debug, settings | D |

## Systems and what each needs

### Phase A: how it feels
- **Movement and physics.** Character controller (`BSHavok/bhkcharactercontroller`, in progress), player mover, actor mover, middle/high process (in progress), camera collision, Havok SDK (about 5,000 functions: collision detection, the solver, character proxy, phantoms, ray casts), ragdolls and constraints, dismemberment, water physics.
- **Input.** Controls, keyboard, mouse and gamepad through `BSDevices` and the interface manager's input handling. Phase 1 already follows the game's order.
- **Animation.** All of `NiAnimation`, the actor animation data, idle selection, weapon animations, blending and timing, and how animation drives movement.
- **Faces.** FaceGen head morphs, lip sync and expressions.
- **Audio.** The `BSAudio` manager, 3D sound, footsteps, weapon and impact sounds, music, the radio, voice and ambient region sounds. Audio threads use the stepped-thread seam, and decoding uses platform replacements (`crates/mp3`, ogg, wav).

### Phase B: how it looks
- **Bevy upgrade**, one version per PR, before the renderer work.
- **Scene graph** (`NiMain`, `BSMain`): node updates, culling, LOD.
- **Renderer** (`NiXenonRenderer`, `BSShader`): translate the Gamebryo Direct3D 9 renderer, the shader classes and passes, render targets, image space and HDR; their Direct3D 9 calls go to our own Direct3D 9 layer (ADR-0007, "Renderer units" below).
- **Sky and weather**: sun, clouds, fog, precipitation, transitions.
- **Terrain, trees and distant land**: SpeedTree, LOD, grass, water rendering.
- **Effects**: particles, decals, blood, effect shaders, VATS and hit effects.
- **Optional real-time shadows** (off by default, maintainer decision).

### Renderer units (ADR-0007, 2026-10-17)

The ledger used to count 2,115 functions as `platform`. Each one was read
in the exe and sorted (the list stays in the private research tree):

- **1,871 moved to `open`** (game code to translate). 1,823 are the
  renderer: 926 in `NiXenonRenderer` (the Xbox name of the PC
  `NiDX9Renderer`: device set-up and reset, render states, buffers,
  textures, shader programs and their constant maps) and 897 in `BSShader`
  (shader classes and passes, accumulators, batch rendering, image-space
  effects). A scan of the disassembly finds COM-style calls (the interface
  pointer pushed, as Direct3D 9 calls are made) in 206 of them; those calls
  stay calls, to our own Direct3D 9 layer. The other 48 are game code inside
  platform subsystems: archive and file helpers without OS calls in
  `BSSystem` (archive name hashing, buffered reads,
  `ArchiveManager::TrimFileName`), the `Archive` and `BSFile` destructors, a
  `NiFile` method the map had put in the renderer, a `BSFileCache` slot the
  map had named into `xaudio2`, the `NiTMap<const char*, char*>` template
  code of the movie player (two of its slots named into `BSDiag`), and a
  Havok stub named into `RemoteLog`. The ledger lists the ones whose unit
  stays platform in `GAME_FUNCS`.
- **244 stay `platform`**: 188 operating-system and service glue (Win32
  file finding, the tasklet threads, the `BSFile` OS reads, the Xbox system
  utilities, which use Steam on PC, Bink movie playback, input devices, the
  allocator), 46 C runtime functions that Rust already stands in for, 5
  D3DX import thunks (our Direct3D 9 layer supplies D3DX) and 5 Xbox-only
  functions (no callers, and they byte-swap big-endian archive data).

Placement: 101 functions joined an existing unit through
`research/engine-map/unit_overrides.tsv` because they are RTTI vtable slots
of that unit's class: 98 the map had put in a subsystem but no unit (44
`NiDX9Renderer` slots went to `nixenonrenderer.cpp`, for example) and 3 the
map had put in a wrong unit by name. 1,080 others were grouped into 148 new
run units (`<subsystem>/run_<first address>`), 932 of them in the renderer.
214 unit files were scaffolded (90 in `units/nixenonrenderer`, 110 in
`units/bsshader`, 14 in `bssystem`, `bsmovie` and `remotelog`). No
translation yet.

### Phase C: how it plays
- **Actor values and combat math**: damage, hit chance, skills, SPECIAL, derived stats.
- **AI**: packages and daily schedules, detection, pathfinding, combat AI and group tactics, followers and the companion wheel.
- **Quests, dialogue and scripts**: all script commands connected to the world, dialogue topics and results, console commands. (Live since Build 42; widening continues.)
- **Menus**: Pip-Boy, inventory, barter, containers, map and fast travel, wait and sleep, character creation, level-up, the start menu, loading screens.
- **VATS**: targeting, action points, slow motion, the VATS camera.
- **Magic and effects**: chems, radiation, addictions, perk effects, abilities.
- **Character progression**: levelling, perks, karma, reputation, factions, crime.
- **Crafting and survival**: repair, recipes, campfires and workbenches, hardcore mode (needs, healing, ammo weight).
- **Minigames**: lockpicking, hacking, Caravan, casino games (blackjack, roulette, slots).
- **World streaming**: cells loading and unloading, references, the order and timing of attach and detach.

### Phase D: the rest
- **Saves**: through the game's own save code, so old saves work.
- **Shut-down**: the 5,020 atexit destructors. 4,790 are rows of `crates/engine-core/data/atexit_destructors.tsv` (research/initgen `--atexit`, run by `engine::static_exit`), one is translated by hand, 229 are refused for hand translation. The CRT exit path is translated too: `_atexit` records its function and `units::crt::doexit` (`00eccd99`) runs the exit functions last first, then the pre-terminator and terminator tables. Not yet connected to the host's shut-down; the exit functions' own destructors run only as far as they are translated.
- **Video**: intro and ending Bink videos (`BSMovie` plus `crates/bink`).
- **Settings and INI files**, debug, diagnostics.
- **Real threads**: swap the stepped-thread seam for real OS threads to replicate the original.
- **Partial markers**: turn exception-frame and cookie partials into full translations where behaviour depends on them.
- **The 10,142 unplaced functions**: assign each to a system by its callers; the ledger tracks them.

### Platform (needed by all phases)
- Windows stand-ins: memory, files (`BSFile` and `ReadF`, needed to finish the real quest loader), events, semaphores, threads (the seam), timers.
- Direct3D 9 calls go to our own Direct3D 9 layer on wgpu (ADR-0007), with D3DX supplied by that layer; DirectInput becomes Bevy input; XAudio2 and DirectSound become the audio back end.
- The exe check: 1.4.0.525 now; GOG and Epic versions later (maintainer decision).

### After the port (planned, not part of 1:1)
- **Fixes layer**: engine-side bug fixes switchable on and off, with the original behaviour always available for comparison. Data fixes (YUP and similar) stay as installed mods unless their authors give permission.
- **Script extender compatibility**: xNVSE and JIP LN functions and events as a compatibility layer. Check each project's license first. Their binary patches of the exe have to be reimplemented in code.
- **GOG and Epic exe support.**

## Known corrections and open items
- The Havok SDK is not ported. Earlier notes counted the B1 solver work (79 functions in `crates/physics`) as "Havok translated".
- The private x87 NaN helpers in several units disagree (`expression.rs`, `actionpoints.rs`, `static_init.rs` and newer files). Consolidate them into `crate::x87` against recordings from the real CPU.
- Shared layouts that translators keep asking for are listed in docs/HANDOFF.md.
