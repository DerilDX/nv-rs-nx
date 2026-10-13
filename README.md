# nv-rs

[![Rust](https://img.shields.io/badge/Rust-2021_edition-orange.svg)](https://www.rust-lang.org/)
[![Engine](https://img.shields.io/badge/Engine-Bevy_0.16-blue.svg)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Platform](https://img.shields.io/badge/Platform-Windows-purple.svg)](docs/PLAYTESTING.md)
[![Status](https://img.shields.io/badge/Status-experimental-lightgrey.svg)](docs/MILESTONES.md)

A from-scratch reimplementation of **Fallout: New Vegas** in **Rust** with the
**Bevy** engine. It runs the game from the `Data` folder of an installation you
own and aims to behave exactly like the original (1:1).

> [!IMPORTANT]
> nv-rs is an independent project, not affiliated with or endorsed by
> Bethesda Softworks, ZeniMax Media, Obsidian Entertainment or Microsoft. It
> contains no game files; you need your own legally obtained copy of the game.
> See [LEGAL.md](LEGAL.md).

---

## How it works

- **Original files at runtime.** nv-rs reads the game's own plugins, archives,
  meshes, textures, sounds, menus and shaders directly. There is no
  conversion step, and the installation is never modified. The game's
  executable is read too (version 1.4.0.525 only; any other build is refused
  at start-up with the reason), for the constants and tables of the
  translated code.
- **Behaviour from the original program.** Rules, constants and formulas come
  from the game's program (`FalloutNV.exe`, read in Ghidra), its data, and
  recordings of the original game. Nothing is invented or tuned by eye. The
  method is described in [docs/METHODOLOGY.md](docs/METHODOLOGY.md) and the
  decisions behind it in [docs/adr/](docs/adr/README.md).
- **Ported system by system.** Every function in `FalloutNV.exe` is mapped
  and tracked in a ledger ([docs/LEDGER.md](docs/LEDGER.md)). The game's code
  is being translated into Rust function by function in the `engine` crate,
  each piece marked with the address it came from and tested, in the order
  that matters most for running the game. The plan is in
  [docs/ENGINE_PORT_PLAN.md](docs/ENGINE_PORT_PLAN.md).
- **Checked against the original.** Tools in [`research/`](research/) record
  what the original program actually does (function results on the real CPU,
  live arguments and state) so that Rust code can be tested against it.
- **Simulation separate from presentation.** Game rules live in plain Rust
  crates with no external libraries. The Bevy app only draws, plays sound
  and reads input. That split also keeps the planned VR support possible
  ([docs/VR.md](docs/VR.md)).

---

## Repository layout

```
nv-rs/
├── Cargo.toml         # core workspace (no external crates)
├── crates/            # readers, game rules, tools and test fixtures (table below)
├── viewer/            # the Bevy app (its own workspace, Bevy 0.16)
├── research/          # Ghidra scripts, the engine map and native oracle tools
│   ├── ghidra/        # identity, complete export, function cards, name import
│   ├── engine-map/    # every function of the exe, by unit and subsystem
│   └── nv-oracle/     # nv-call, nv-probe, nv-inject (own workspace, 32-bit Windows)
├── docs/              # roadmap, method, decisions, ledger, topic findings
├── scripts/           # acceptance routes, the ledger, packaging
└── distribution/      # files shipped in playtest packages
```

| Group | Crates | What they do |
| --- | --- | --- |
| **Formats** | `esm`, `bsa`, `nif`, `dds`, `mp3`, `bink`, `speedtree`, `shaders`, `assets`, `fos` | Read the game's files: plugins and records, archives, meshes and animations, textures, audio, movies, trees, compiled shaders, the loose-file/archive lookup, and the original game's saves (read-only). |
| **Engine port** | `engine`, `host` | `FalloutNV.exe` translated function by function, running on a model of the game's memory ([ADR-0006](docs/adr/0006-engine-crate-memory-model.md), [docs/ENGINE_CRATE.md](docs/ENGINE_CRATE.md)). `host` starts it in the running game on the player's own `FalloutNV.exe`. |
| **Game** | `world`, `physics`, `script`, `scriptgen`, `ui`, `preview`, `cellview` | The game's rules: world state and the per-frame order, actors, AI, combat, dialogue, quests, inventory, saves, collision, the script engine and the game's scripts translated to Rust, menus built from the game's XML, and per-cell scene data. |
| **Tools** | `nvinspect` | Command-line inspection of real game data: records, cells, meshes, collision walks, coverage tables and more. |
| **Test content** | `testdata` | Builds synthetic plugins, archives and meshes for tests, so tests never need game files. |
| **App** | `viewer` (binary `nv-viewer`) | The Bevy app: rendering, audio and input on top of the game crates. |

---

## Roadmap

### Engine port

The main work right now. Progress is measured in the ledger
([docs/LEDGER.md](docs/LEDGER.md)).

- [x] **Phase 0: the map and the ledger.** Every function in the exe has a
  unit, a subsystem and a status.
- [ ] **Translation** (ongoing): over 10,000 of the exe's functions are
  translated so far (about 15% of the whole program; counting only game
  code, about 35% is translated or traced). Most are not yet switched on
  in play.
- [ ] **Phase 1: the frame skeleton** (nearly done): the game's per-frame
  order, from the main loop down, drives the order of the app's systems
  ([docs/FRAME_SKELETON.md](docs/FRAME_SKELETON.md)).
- [ ] **Phase 2: the state model and saves:** the game's state lives in the
  translated code's memory, checked by reading a real save and writing it
  back identically.
- [ ] **Phase 3 onward:** whole systems switched over one at a time: the
  script engine, conditions, AI, animation, menus, the rest of Havok and
  audio.

### Milestones

Each milestone has an acceptance gate checked against the original game.
Status and evidence are in [docs/MILESTONES.md](docs/MILESTONES.md).

- [ ] **M1: Opening and persistent world** (active): the opening movie and
  wakeup, Doc Mitchell's character creation, leaving for Goodsprings, and
  saving and reloading with all state intact.
- [ ] **M2: Core gameplay loop** (active alongside M1): Sunny's tutorial
  and a Goodsprings quest branch: movement, weapons, damage, AI, dialogue,
  loot, trade, progression and V.A.T.S.
- [ ] **M3: Base-game systems and campaign:** coverage of quests, creatures,
  weapons, effects, factions, companions, travel and menus, up to campaign
  completion.
- [ ] **M4: Stability and performance:** long play without crashes or lost
  state, with measured frame times and streaming.
- [ ] **M5: DLC and mods:** official DLC and documented plugin, archive and
  script-extension behaviour.
- [ ] **M6: VR:** shared simulation with independent aim and views, tested on
  a headset.

What plays today (October 2026): Doc Mitchell's house and his walk to the
door, Goodsprings with its people going about their routines, and two
Goodsprings quests driven by their own scripts: Back in the Saddle (Sunny's
tutorial) and Ghost Town Gunfight. Along the way: NPC combat, animation and
navigation, dialogue, barter, the Pip-Boy, radio, physics and ragdolls,
terminals and hacking; crafting, Caravan and parts of Dead Money are in but
mostly checked by tests so far. These
three routes are replayed automatically on every change
(`scripts/acceptance.ps1`). Much of this was built before the engine port
and will be replaced, system by system, by the translated code.

What it isn't yet: a complete playthrough. Parts of the routes are still
driven by console lines, the face editor isn't built, and nothing has been
compared side by side with the original game across a whole route. Native
NVSE plugin DLLs are not supported, original-game saves can't be loaded,
and VR and performance targets have not been validated. The open work is
in [docs/TASKS.md](docs/TASKS.md).

---

## Quick start

### Play a test build

Download a Windows x64 ZIP from
[Releases](https://github.com/slaterain/nv-rs/releases), extract it, and run
`Play.cmd`. Point it at your own `Fallout New Vegas\Data` folder. Read
[docs/PLAYTESTING.md](docs/PLAYTESTING.md) first.

### Build from source

Install stable Rust, then:

```sh
# Core workspace: readers, game rules, nvinspect
cargo build --release
cargo test --workspace

# The Bevy app (separate workspace)
cd viewer
cargo run --release -- "<path to Fallout New Vegas\Data>" GSDocMitchellHouse
```

The core crates and their tests need no game files. Running the viewer and
`nvinspect` on real assets needs your `Data` folder. To replay the
acceptance routes (Windows, PowerShell):

```powershell
powershell -File scripts\acceptance.ps1 -Data "<path to Fallout New Vegas\Data>" -Build
```

Windows is the tested platform. Native Linux and macOS (Metal) support are
in open pull requests.

---

## Documentation

| Read this | For |
| --- | --- |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute: claiming tasks, pull requests, forks |
| [docs/TASKS.md](docs/TASKS.md) | Open tasks: major systems (claim on GitHub) and the maintainer's list |
| [docs/MILESTONES.md](docs/MILESTONES.md) | Active work, blockers, evidence and the next action |
| [docs/ENGINE_PORT_PLAN.md](docs/ENGINE_PORT_PLAN.md), [docs/LEDGER.md](docs/LEDGER.md) | The engine port: plan, phases, and what is translated so far |
| [docs/ENGINE_CRATE.md](docs/ENGINE_CRATE.md), [docs/FRAME_SKELETON.md](docs/FRAME_SKELETON.md) | How translated code is written and tested; the per-frame order |
| [docs/METHODOLOGY.md](docs/METHODOLOGY.md) | How the game's program is read and reimplemented, with sources |
| [docs/adr/](docs/adr/README.md) | Decisions: method, symbols, decompiled code, tools, the engine memory model |
| [docs/RESEARCH_WORKFLOW.md](docs/RESEARCH_WORKFLOW.md) | Day-to-day research steps and provenance |
| [research/ghidra](research/ghidra/README.md), [research/nv-oracle](research/nv-oracle/README.md) | The research tools and how to run them |
| [docs/TECHNICAL_REFERENCE.md](docs/TECHNICAL_REFERENCE.md) | Detailed feature descriptions |
| [docs/OPENING.md](docs/OPENING.md), [docs/OPENING_LOOK_IK.md](docs/OPENING_LOOK_IK.md), [docs/FACE_CREATION.md](docs/FACE_CREATION.md), [docs/PERSISTENCE.md](docs/PERSISTENCE.md), [docs/VIGOR.md](docs/VIGOR.md) | M1 topic findings |
| [docs/GOODSPRINGS_ROUTE.md](docs/GOODSPRINGS_ROUTE.md), [docs/PACKAGES.md](docs/PACKAGES.md), [docs/PATHING.md](docs/PATHING.md), [docs/NPC_COMBAT.md](docs/NPC_COMBAT.md), [docs/ANIMATION.md](docs/ANIMATION.md), [docs/DIALOGUE.md](docs/DIALOGUE.md), [docs/PHYSICS.md](docs/PHYSICS.md), [docs/WEAPON_EFFECTS.md](docs/WEAPON_EFFECTS.md), [docs/PIPBOY.md](docs/PIPBOY.md) | M2 topic findings (each says what's traced, tested and missing) |
| [docs/CONTRIB_CHAZM.md](docs/CONTRIB_CHAZM.md), [docs/CONTRIB_PLAYCON.md](docs/CONTRIB_PLAYCON.md), [docs/DEAD_MONEY.md](docs/DEAD_MONEY.md) | Contributors' merged work |
| [docs/VR.md](docs/VR.md) | VR boundaries and proposals |
| [docs/PLAYTESTING.md](docs/PLAYTESTING.md), [docs/MAINTAINING.md](docs/MAINTAINING.md) | Testing builds and maintaining releases |

---

## Contributing

nv-rs is maintained by **slaterain** and welcomes contributors (and their
AI agents). The way in:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md), and
   give both to your agent.
2. Claim a task: the major systems in [docs/TASKS.md](docs/TASKS.md) are
   GitHub issues titled `[task] …`; comment `Claiming this` before you
   start.
3. Work in a fork on a topic branch from the latest `main`, and open a
   small pull request against `main` that passes the checks and the
   acceptance routes.

**Where help matters most now:** with the game's own code being ported
system by system, hand-written gameplay logic (quest scripts, AI, combat
rules, menus) will be replaced as the translated systems come online. The
most useful contributions are platform support, acceptance routes and
playthrough tests, research into how the original behaves, tools, and bug
reports from test builds, especially anything that behaves differently from
the original game.

**Already have work in a fork?** Sync your fork first, then follow
["Already working in a fork?"](CONTRIBUTING.md#already-working-in-a-fork-start-here)
to get your work claimed, rebased and into a pull request.

Final decisions rest with the maintainer
([docs/GOVERNANCE.md](docs/GOVERNANCE.md)).

## Legal and license

Original code is dual-licensed under [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option. The licence covers only original
work. Fallout and Fallout: New Vegas are trademarks of their respective owners.
Read [LEGAL.md](LEGAL.md) for the full notice, including how material learned
from the game's program is handled.
