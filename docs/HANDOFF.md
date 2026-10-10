# Handoff: engine port push (2026-10-10, morning)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

The previous lead stopped because the account hit its weekly usage limit at
about 04:30 on 2026-10-10. All six agents died mid-task; nothing is running.
Their transcripts belong to the other account, so they cannot be resumed with
SendMessage: start fresh agents instead. In order:

1. **Rolling PR.** #106 is merged. The commits after it (a05, a06 and a03
   results and this file) were rebased onto main and opened as the next
   rolling PR from `claude/engine-crate` (see `gh pr list`). Ship it with
   `bin/ship.sh <pr>` once both checks pass.
2. **Restart the five translation slots.** Do NOT run `cycle.sh` or `task.sh`
   for them first: their task files in `agents\tasks\aNN.md` are already
   written, and `task.sh` resets the worktree, which would throw away the
   partial work below. Launch one Agent per slot (model sonnet, background)
   with the standard prompt (see below), adding for a01 and a04 the sentence
   "Your file already holds uncommitted partial work from an interrupted
   session: check that it builds and is right, keep it, and continue from
   where it stops."
   - a01: `navmesh_util.cpp` (b0329), partial uncommitted work in
     `navmesh_util.rs`.
   - a03: `exteriorcellloader.cpp` (b0043), nothing done yet.
   - a04: `navmeshrender.cpp` (b0341), partial uncommitted work in
     `navmeshrender.rs`.
   - a05: `aitaskmanager.cpp` lane 2 (b0158, 52 left), nothing done yet.
   - a06: `bhkworld.obj` (b0360), nothing done yet.
   When each finishes, collect it with `cycle.sh` as usual.
3. **Phase 1 PR 7 (interface and render).** Its unfinished work is saved as a
   WIP commit (4576013) on `claude/phase1-interface-stage` in worktree
   `agents\p2`, pushed, no PR yet. It was written on top of #107's branch:
   rebase onto main first (`git rebase origin/main`; drop the commits #107
   already contains if git does not skip them itself). Then start one agent
   (opus) in p2 to finish it: the stage is the interface and render steps of
   `docs/FRAME_SKELETON.md`, modelled on PR 6 (#107, `world::frame::ai_stage`)
   and PR 5 (#105). The WIP adds `crates/world/src/frame/interface.rs` (and a
   folder), `coverage.rs`, `research/engine-map/frame_wiring.tsv`, and edits
   to the interface, interfacemanager and main engine units. It was not yet
   built or tested. The agent amends the WIP into a real commit, passes fmt,
   clippy and tests, and opens the PR.
4. After PR 7 merges: play build 40, then Phase 2.

## Where things stand

- **Phase 0** (engine map, ledger) is merged (#72). `docs/LEDGER.md` is the
  coverage table; regenerate with
  `cargo run -q --release --manifest-path scripts/ledger/Cargo.toml`.
- **Engine crate** (`crates/engine`, ADR-0006): about 10,250 of FalloutNV.exe's
  functions are translated (15.5% of all, see LEDGER.md), each with a
  `// Translated from <addr>` marker and tests. They run in the crate's
  emulated 32-bit memory, not yet in play.
- **Units finished:** main.cpp, actor.cpp (all 7 parts), playercharacter.cpp
  (all parts), interface.cpp, interfacemanager.cpp, processlists.cpp,
  tesobjectrefr.cpp, extradatalist.cpp, extradataobjects.cpp, tesscript*.cpp,
  tesform, tesfile, tesland, tesworldspace, teswater, tesnpc, package,
  inventorychanges, bipedanim, animation, modelloader, tesobjectcell,
  bgsdecalmanager, navmeshobstaclemanager, globalfunc, pathfind.cpp,
  BSMenu/tile.cpp, highprocess.cpp parts 1, 2, 4 and 5, navmesh.cpp,
  pathinglocation.cpp, bgssceneinfo.cpp, bgssaveloadgame.cpp, and others.
- **In progress (translation lanes, all interrupted; see "Resume here"):**
  navmesh_util.cpp (a01), exteriorcellloader.cpp (a03), navmeshrender.cpp
  (a04), aitaskmanager.cpp lane 2 (a05; MovementTaskData still needs its
  layout, see the file header), bhkworld.obj (a06).
- **Phase 1 (frame skeleton) wiring:**
  - PR 1 frame map (#84), PR 2 `world::frame` (#86), PR 3 Bevy order from the
    frame (#90): merged.
  - PR 4 player stage (#101): merged. It added `world::frame::player` and
    `frame_order::PlayerSet`.
  - PR 5 world and time stage (#105): merged.
  - PR 6 AI task stage (#107): merged.
  - PR 7 interface and render: unfinished WIP in worktree
    `%USERPROFILE%\nv-re\work\agents\p2`, branch
    `claude/phase1-interface-stage` (see "Resume here"). Merge after both
    checks pass, rebasing first if translation PRs merged meanwhile (LEDGER.md
    and engine unit test modules can conflict; keep both sides' tests).
  - After that: Phase 2 (state model in engine memory, save round trip; see
    ENGINE_PORT_PLAN.md).
- **Rolling translation PR:** #106 merged; the next one is open from branch
  `claude/engine-crate` (see "Resume here").
- **Play build** (the user's `Desktop\nv-rs-play`): Build 39 = main with #107.
  After the next wiring PR merges, build `viewer` in release from main
  (`cargo build --release --manifest-path viewer/Cargo.toml`), copy
  `viewer/target/release/nv-viewer.exe` into `app` (or `next` if the viewer is
  running) and add a plain-language entry at the top of `WHATS-NEW.txt` (LF
  line endings).

## How the translation machinery runs (private, not in the repo)

Everything private lives in `%USERPROFILE%\nv-re\work\phase0` (state notes in
`STATE.md`, scripts in `bin/`).

- Lead worktree: `%USERPROFILE%\nv-re\work\wt-phase0` (branch
  `claude/engine-crate`). Agent worktrees: `%USERPROFILE%\nv-re\work\agents\aNN`,
  task files `agents\tasks\aNN.md`, scratch `agents\scratch\aNN`.
- Private read-only Ghidra server for agents: 127.0.0.1:8090
  (`bin/start-srv.ps1`). Never write to the shared server on 8089.
- When an agent finishes: `cd ...\phase0 && bin/cycle.sh NN <tokens> <ms>`
  (collects its file, checks names, fmt, clippy, tests, commits, logs to
  `progress.tsv`, writes its next task), then launch an Agent (model sonnet,
  background) with: "Read the task file
  %USERPROFILE%\nv-re\work\agents\tasks\aNN.md and carry out the task it
  describes, exactly as written. It names your working directory, your files
  and the rules."
- Ship the rolling PR when both checks pass: `bin/ship.sh <pr>` (squash-merges,
  rebases, regenerates the ledger, opens the next PR).
- After any other PR merges, rebase `claude/engine-crate` onto main: on each
  stop where only `docs/LEDGER.md` conflicts, rerun the ledger, `git add` it,
  `GIT_EDITOR=true git rebase --continue`; check the rebase succeeded before
  running anything else; then rerun the ledger, commit, build, force-push.
- Concurrency: at most 6 agents (5 translators + 1 wiring agent). The machine
  has 31 GB and each engine compile takes 3-4 GB; agents pass `-j 2` to cargo.
  Slots listed in `parked.txt` are collected but not relaunched.
- `plan.tsv` is ordered by reachability from `Main::OnIdle` (0086e650).
  `pending_lanes.txt` holds lanes handed back; cycle.sh takes those first.
- Usage limit: the user's rule is to stop at 98% weekly usage, merge finished
  work and update this file. If agents die on a session limit, resume each
  with SendMessage after the reset (their uncommitted work stays in the
  worktree).

## Open items for the lead

- Platform stand-ins missing for Win32 imports the translated code calls
  (file search, registry, window, message box, semaphores, time; full list in
  phase0/STATE.md).
- 00aa13e0/00aa1070 allocate through the Gamebryo allocator object at
  `**0x011f6080`; the platform must provide it before running translated code.
- Shared layouts and helpers agents keep asking for: one `Actor`,
  `PlayerCharacter`, process (`HighProcess`/`MiddleHighProcess`) layout shared
  across part files (each part declares its own today), a `BSSimpleList`
  accessor set, `NiPoint3`/`NiMatrix3` helpers, x87 rounding helpers,
  `BSStringT`.
- Six functions Ghidra never defined (00435ac0, 00435bb0, 0041fdb0, 00420060,
  004203b0, 00420630): define them in the private project and rerun the engine
  map.
- 0084b720 (hash map base destructor body, vtable 0107f4c8) is called from
  bgssaveloadgame.cpp but sits in no unit's queue; it needs an owner.
- ADR-0006 awaits maintainer review.
- Stray scratch files agents left in the Git install folder
  (`C:/Program Files/Git/tmp_dummy`, `tmp_p3.rs`, `tmp_tests.rs`); deleting
  them there needs the user.
