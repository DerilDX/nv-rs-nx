# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

Codex takeover, 2026-10-10: #109 passed both CI checks and was squash-merged
as `b995218`. The lead branch was rebased to that commit and pushed. There
was no new rolling PR immediately after the merge. The ship
script stopped after the merge/rebase; recovery verified main and recreated
the automatically deleted remote branch with a normal push.

Active in this session: a04 restarted from its partial file and a03 started
its original task. The interface/render agent owns p2 and is finishing
the WIP (now `5fc95c9` after rebase onto `b995218`). a05 and a06 are waiting
for an available agent and
still have their original task files: do not reset them before first restart.
The session supports three child agents plus the lead; rotate translation
slots while p2 runs. a01 was collected as `74f7fb1` (40 functions), and
`cycle.sh` prepared its next b0329 task (78 entries left, starting at
`00698680`; include `traced` entries). It is waiting for capacity after the
remaining original slots. Play build remains 39.

Validation: a01's 40 preserved translations pass their 38 focused tests
after correcting an expectation for `00697010`'s unusual distance
comparison; collection passed formatting, lint and 13,281 engine tests
plus doc tests on the lead branch. a04's 36 preserved
translations lacked tests, which are now being added. Native `nv-call`
spot checks cover five `00698620` point products and 512 `006a5da0` flag
combinations (private evidence under `phase0/codex-audit`), not whole units.
p2's 182 release-profile viewer tests and 15,108 core tests passed after
repairing WIP model/test errors. The missing model call
was voice pause `0070bbe0` at `0070bb03`; a queue mock also made a deferred
deletion loop run forever. Both workspaces passed lint; final release
builds and acceptance are pending. No new play build has been published.

Machine note: automatic policy blocked removal of idle p3's target cache;
nothing was deleted. Private `task.sh` now warns below 40 GB free instead
of deleting caches, and retains scratch evidence. Do not use another
deletion path to bypass that denial. Ending the hung test recovered space.

Next steps:

1. Ship the rolling PR from `claude/engine-crate` once both checks pass;
   `gh pr list --head claude/engine-crate` identifies the current number.
2. Collect completed slots with one `cycle.sh NN 0 0` at a time. Never
   cycle an active agent's worktree. Restart a05 and a06 from their existing
   task files before resetting them; then rotate a01's prepared next task.
3. Finish p2 validation, amend its WIP into a proper commit, push and open
   the interface/render PR. Rebase if another batch merges; retain both
   sides' tests and regenerate the ledger. Merge only with both checks
   passing. Detailed models and open wiring: FRAME_SKELETON.md.
4. After that PR merges: play build 40 from main, then Phase 2. Keep the
   early menu-input boundary recorded as a follow-up, not a new priority.

## Where things stand

- **Phase 0** (engine map, ledger) is merged (#72). `docs/LEDGER.md` is the
  coverage table; regenerate with
  `cargo run -q --release --manifest-path scripts/ledger/Cargo.toml`.
- **Engine crate** (`crates/engine`, ADR-0006): 10,303 of FalloutNV.exe's
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
- **In progress (translation lanes; see "Resume here" for current owners):**
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
  - PR 7 interface and render: resumed WIP in worktree
    `%USERPROFILE%\nv-re\work\agents\p2`, branch
    `claude/phase1-interface-stage` (see "Resume here"). Merge after both
    checks pass, rebasing first if translation PRs merged meanwhile (LEDGER.md
    and engine unit test modules can conflict; keep both sides' tests).
  - After that: Phase 2 (state model in engine memory, save round trip; see
    ENGINE_PORT_PLAN.md).
- **Rolling translation PR:** #109 merged; the next batch is on branch
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
