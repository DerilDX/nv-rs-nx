<p align="center">
  <img src="docs/media/logo-ranger.png" alt="nv-rs logo: an NCR Ranger in front of a Mojave sunset, inside a gear" width="260">
</p>

<h1 align="center">nv-rs</h1>

[![Rust](https://img.shields.io/badge/Rust-2021_edition-orange.svg)](https://www.rust-lang.org/)
[![Engine](https://img.shields.io/badge/Engine-Bevy_0.16-blue.svg)](https://bevyengine.org/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS-purple.svg)](docs/PLAYTESTING.md)
[![Status](https://img.shields.io/badge/Status-experimental-lightgrey.svg)](docs/MILESTONES.md)

**Fallout: New Vegas, rebuilt in Rust, aiming to play exactly like the
original.**

nv-rs is a new engine for Fallout: New Vegas, written in Rust with the Bevy
engine. It runs the game from your own copy's files, and its goal is a
faithful 1:1 match, down to the original's quirks. The goal is not to make a
"remake" that feels close. It's to make the same game, on a modern,
open-source engine that can be fixed, ported and extended.

<p align="center">
  <img src="docs/media/goodsprings-aerial.jpg" alt="Goodsprings from above, rendered by nv-rs" width="100%">
  <br><sub><i>Goodsprings, rendered by nv-rs from the game's own files.</i></sub>
</p>

> [!IMPORTANT]
> nv-rs is an independent project, not affiliated with or endorsed by
> Bethesda Softworks, ZeniMax Media, Obsidian Entertainment or Microsoft. It
> contains no game files; you need your own legally obtained copy of the game.
> See [LEGAL.md](LEGAL.md).

---

## Why it's different

- **Behaviour comes from the original, not from guesswork.** Rules, formulas
  and timings are taken from the original game itself and checked against
  it. Nothing is tuned by eye.
- **Your game, your files.** nv-rs reads the game's own plugins, meshes,
  textures, sounds and menus directly from your installation. It never
  modifies them.
- **Built to last.** Open source, memory-safe Rust, with the game's rules
  kept separate from graphics, so new platforms, fixes and even VR are
  possible later.
- **Mods keep working.** Regular `.esp` and `.esm` mods load the way they do
  in the original.

## Where it stands (October 2026)

Experimental, but already playable in places:

- **The opening and Goodsprings:** Doc Mitchell's house, character creation,
  Goodsprings with its people going about their day, and the quests *Back in
  the Saddle* and *Ghost Town Gunfight*.
- **Systems in play:** combat, NPC AI and navigation, dialogue, barter, the
  Pip-Boy, the radio, physics and ragdolls, terminals and hacking.
- **The engine port:** nearly all of the original program's functions are
  translated (see the bar below). Since Build 42 the translation runs inside
  the game, starting with quests. In the separate play build, the game's own
  code now starts a new game in Doc Mitchell's house, plays the opening
  (Doc's speech, the name box) and reaches the race menu. It is not yet the
  default way to play.

<table>
  <tr>
    <td width="50%"><img src="docs/media/doc-mitchell.jpg" alt="Doc Mitchell in his chair"></td>
    <td width="50%"><img src="docs/media/ghost-town-gunfight.jpg" alt="Ghost Town Gunfight"></td>
  </tr>
  <tr>
    <td align="center"><sub><i>Waking up in Doc Mitchell's house.</i></sub></td>
    <td align="center"><sub><i>Ghost Town Gunfight: the Powder Gangers come to town.</i></sub></td>
  </tr>
</table>

Not yet: a full playthrough, loading original saves, script-extender (NVSE)
plugins, packaged Linux and macOS builds, and VR.

## Progress

<p align="center">
  <img src="docs/media/progress.svg" alt="Engine port progress: 99.8% of the original program's functions covered" width="100%">
</p>

The original game's systems switch over one at a time, each in its own
playable build:

| | System | Status |
| :---: | --- | --- |
| ✅ | **Foundations:** the map of the original program, the frame order, the engine running in play | Done (Build 42) |
| 🔨 | **Movement and physics:** how walking, jumping and collisions feel | In progress: every physics function translated (all 9,015 Havok functions; 739 still have a marked untranslated part, mostly C++ exception frames). In the play build the game's own code starts a new game, loads Doc Mitchell's house, plays the opening (Doc's speech, the name box) and reaches the race menu, taking its input from the game window; next: the movement checks against the original |
| 🔨 | **Animation and faces** | In progress: every function translated (some still lack their C++ exception frames), not yet switched on |
| 🔨 | **Audio:** sounds, music and the radio | Every function translated; DirectSound, the music graph (DirectShow and MP3), Vorbis and the mixer run on our own layer. In the play build, `NV_SOUND=out` plays the game's own sound on the real audio device; the resampling, 3D sound and distortion are approximate for now ([docs/SOUND.md](docs/SOUND.md)) |
| 🔨 | **Graphics:** lighting, weather and effects matching the original | In progress: every function translated. In the play build on Windows, `NV_D3D9=real` sends the game's own renderer to the real Direct3D 9 in its own window ([ADR-0008](docs/adr/0008-d3d9-layer-forwarded-to-the-real-runtime.md)); the cross-platform layer on wgpu ([ADR-0007](docs/adr/0007-renderer-translated-on-own-d3d9-layer.md)) is not built yet. Not yet compared against the original |
| ⬜ | **Gameplay:** combat, AI, quests, dialogue, menus, V.A.T.S., perks, crafting and minigames | Every function translated, not yet switched on |
| ⬜ | **Saves and the open world:** original saves, world streaming | Planned |

After that come optional bug fixes, script-extender compatibility and more
platforms. Every system, with numbers, is in
[docs/SYSTEMS.md](docs/SYSTEMS.md), and the milestones are in
[docs/MILESTONES.md](docs/MILESTONES.md).

---

## Try it

1. Download the latest Windows build from
   [Releases](https://github.com/slaterain/nv-rs/releases).
2. Extract it and run `Play.cmd`.
3. Point it at your own `Fallout New Vegas\Data` folder.

nv-rs needs Fallout: New Vegas version 1.4.0.525 installed. Tips for testing
and reporting are in [docs/PLAYTESTING.md](docs/PLAYTESTING.md). If anything
behaves differently from the original game, that's a bug worth reporting.

### Build from source

Install stable Rust, then:

```sh
cargo build --release && cargo test --workspace
cd viewer
cargo run --release -- "<path to Fallout New Vegas\Data>" GSDocMitchellHouse
```

### Build and run on macOS

The source viewer supports native macOS and selects Metal. Install stable
Rust and the Xcode Command Line Tools, then pass the Data folder from your
own PC installation:

```sh
cd viewer
cargo run --release -- "/path/to/Fallout New Vegas/Data" GSDocMitchellHouse
```

Apple Silicon builds a native arm64 viewer by default. The game executable
and Windows release packages remain Windows-specific.

### Build and run on Linux (Vulkan and Wayland)

The viewer builds from source on Linux and supports native Wayland and X11
windows. On Fedora, install `alsa-lib-devel`, `libudev-devel`,
`wayland-devel`, and `libxkbcommon-devel`. Debian and Ubuntu need
`libasound2-dev`, `libudev-dev`, `libwayland-dev`, and `libxkbcommon-dev`.
Install a Vulkan loader and a working GPU driver for your system.

From a Wayland session, pass your own game data folder. When
`WAYLAND_DISPLAY` is set, winit prefers Wayland. Unsetting `DISPLAY` is
optional and forces the native Wayland path; `WGPU_BACKEND=vulkan` selects
Vulkan rendering:

```sh
cd viewer
env -u DISPLAY WGPU_BACKEND=vulkan cargo run --release -- \
  "/path/to/Fallout New Vegas/Data" GSDocMitchellHouse
```

For Steam/Proton installations, pass the active plugin list and INI when
needed with `--plugins` and `--ini`. Linux builds currently run from source;
Windows remains the packaged playtest platform.

---

## How it's made

To be upfront: nv-rs is built almost entirely with AI coding agents (mainly
Claude, with some Codex). The maintainer directs the project: setting the plan
and priorities, playtesting builds and deciding what gets merged. The agents
write nearly all of the code.

What keeps it honest is the method. Agents don't get to invent behaviour:
every ported piece of logic is traced to the original game's code and marked
with where it came from. It is then checked by more than 20,000 automated
tests, by automated play-throughs of the opening and the Goodsprings quests on
every change, and against the original game itself. Anything that can't be
traced to the original doesn't go in. Everything is in the open, so judge it by
how it plays.

---

## Get involved

Contributions are welcome, from people and their AI agents alike:

1. Read [CONTRIBUTING.md](CONTRIBUTING.md) (and give [AGENTS.md](AGENTS.md)
   to your agent).
2. Open an issue saying what you'd like to work on, and wait for a
   go-ahead from the maintainer.
3. Open a small pull request against `main`.

The most useful help right now: testing builds and reporting differences from
the original, platform support, playthrough routes and research into how the
original behaves.

**Curious how it works?** Start with
[docs/ENGINE_PORT_PLAN.md](docs/ENGINE_PORT_PLAN.md) and
[docs/METHODOLOGY.md](docs/METHODOLOGY.md). Progress is tracked in
[docs/LEDGER.md](docs/LEDGER.md) and [docs/MILESTONES.md](docs/MILESTONES.md),
and every other document is linked from [docs/](docs/).

Final decisions rest with the maintainer, **slaterain**
([docs/GOVERNANCE.md](docs/GOVERNANCE.md)).

## Legal and license

Original code is licensed under the [GNU GPL v3 or later](LICENSE). The
licence covers only original work. Fallout and Fallout: New Vegas are trademarks of their respective owners.
Read [LEGAL.md](LEGAL.md) for the full notice.
