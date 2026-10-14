# nv-rs

[![Rust](https://img.shields.io/badge/Rust-2021_edition-orange.svg)](https://www.rust-lang.org/)
[![Engine](https://img.shields.io/badge/Engine-Bevy_0.16-blue.svg)](https://bevyengine.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE-APACHE)
[![Platform](https://img.shields.io/badge/Platform-Windows-purple.svg)](docs/PLAYTESTING.md)
[![Status](https://img.shields.io/badge/Status-experimental-lightgrey.svg)](docs/MILESTONES.md)

**Fallout: New Vegas, rebuilt in Rust, aiming to play exactly like the
original.**

nv-rs is a new engine for Fallout: New Vegas, written in Rust with the Bevy
engine. It runs the game from your own copy's files, and its goal is a
faithful 1:1 match, down to the original's quirks. The goal is not to make a
"remake" that feels close. It's to make the same game, on a modern,
open-source engine that can be fixed, ported and extended.

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
- **The engine port:** about a quarter of the original game's code has been
  carefully ported so far. Since Build 42 it runs inside the game, starting
  with quests, which now load and run on the original's own logic.

Not yet: a full playthrough, loading original saves, script-extender (NVSE)
plugins, Linux and macOS, and VR.

## What's next

The original game's systems are switched on one at a time, each in its own
playable build:

1. **Movement and physics:** how walking, jumping and collisions feel.
2. **Animation and faces.**
3. **Audio.**
4. **Graphics:** lighting, weather and effects matching the original.
5. **Gameplay:** combat, AI, quests, dialogue, menus, V.A.T.S., perks,
   crafting and minigames.
6. **Saves and the open world.**

After that come optional bug fixes, script-extender compatibility and more
platforms. The full list of every system is in
[docs/SYSTEMS.md](docs/SYSTEMS.md).

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

Original code is dual-licensed under [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option. The licence covers only original
work. Fallout and Fallout: New Vegas are trademarks of their respective owners.
Read [LEGAL.md](LEGAL.md) for the full notice.
