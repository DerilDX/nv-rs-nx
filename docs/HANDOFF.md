# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

Codex takeover, 2026-10-10: rolling PRs #109, #112 and #114 passed both CI
checks and were squash-merged as `b995218`, `d854fbe` and `89e1a0b`.
Current rolling PR #116 contains the later navigation continuation and
explosion helpers, ledger and handoff. The explosion review corrections have been collected and checked;
#116 needs fresh CI on the updated head before merging. The lead branch is `claude/engine-crate`.

Phase 1 PR 7 (#113) passed both checks and merged as `f917219`.
The lead branch is rebased onto current main `89e1a0b`.
Only ledger/milestone conflicts needed resolution. Local validation
passed 15,108 core tests, 182 viewer release tests, formatting, lint and
release builds. Doc Mitchell, Back in the Saddle and Ghost Town Gunfight
acceptance routes passed; private evidence is under
`%USERPROFILE%/nv-re/acceptance/pr7-resume-20261010`. This is not a
comparison with the original game. Exact menu-input placement and GPU
renderer work remain open in FRAME_SKELETON.md. Build 40 was published
from merged main at 19:47 local after a fresh release build and Doc route
(63 s). Source and app copy SHA-256 match; WHATS-NEW uses LF. The other
two routes also passed (VCG02 410 s, VMS16 339 s); logs are in
`%USERPROFILE%/nv-re/acceptance/build40-main-20261010`.

Continuous translation remains active until the user says stop: collect each
finished slot with cycle.sh, immediately start its prepared next task, and
ship rolling PRs as soon as both checks pass. Runtime currently permits only
three children and rejects fresh threads because of stale pending slots;
idle workers are reused for new prepared tasks.

Slot state: a01 navigation continuation is owned by worker `/root/a04`
(not the explosion worker). Its previous 36 complete / 4 skipped batch was
collected as `a8b4daf`; the new task starts with open `006987d0`.
a03 b0331 pathmanager.cpp is running under `/root/a03`.
a04 b0160 explosion.cpp is running under fresh worker `/root/a04_round4`.
Its 24-function continuation and all reviewed ABI/NaN/accessor corrections
were collected as `57fdb72`; 13,541 combined engine tests, lint and formatting
pass. Native copy vectors and missing branch tests are included. 37 queue
entries remain, starting `009ac200`. a04 is researching with cargo held.
a06's next b0360 and a05's next b0158 (12 entries starting `008ca620`)
are prepared and waiting for capacity. Never collect/reset an active slot.
a05's collected batch has 28 complete / 12 partial functions; gaps remain
explicit in its file header.
Collection validation: a01 passed 13,281 engine tests; a04 added 31
focused tests and passed 13,312 combined engine tests; a03 passed 13,355
combined engine tests. a06 passed 13,397 combined engine tests.
a05 passed 13,438 combined engine tests before the PR 7 rebase.
The navigation continuation passed 13,487 combined engine tests.
Formatting, engine lint and doc tests passed for
each collection. Native spot checks cover five `00698620` point products
and 512 `006a5da0` flag combinations, not whole units. These translations
are not newly playable behavior.

Lead Phase 2 PR #115 is open at `c819461` after rebasing onto #114, branch
`codex/phase2-extra-data`, in managed worktree
`%USERPROFILE%/.codex/worktrees/phase2-extra-data/wt-phase0`.
It adds the engine-owned ExtraData list handle and four traced helpers.
After rebasing onto merged #113, all 15,154 core tests (13,301 engine),
clippy, formatting and release build passed; CI gates its merge.
The installed-executable real-registry probe passes construction, add,
lookup and detach. No world/viewer consumer or save writer is added.
Private probes/native vectors and the read-only 11-save baseline are in
`phase0/codex-audit`; FOS_SAVES.md records the decoder limits.
The lead has now switched that managed checkout to a separate follow-on
branch `codex/phase2-extra-count`, closing the count destruction dependency;
commit `ec6a70e` is separate from #115. Its pre-rebase full core test suite
passed 15,156 tests. The rebased attempt (session 75073) failed during
rustc memory allocation, before tests; no assertion failure. It is closed.
Retry tests, clippy and release serially; private logs are
`phase2-count-rebased-*`.
Its real-save count probe passes 14 payloads across 3,319 extra lists.
The count follow-on is draft PR #117 at `117d104`, stacked on #115; retarget
onto main after #115 merges. No root cargo process is active. The managed
checkout is now `codex/phase2-extra-health`, with uncommitted adapter and
constructor-test changes. A native vector caught signaling-NaN handling in
`00431d10`; the fix and health adapter tests are not yet compiled. Simultaneous engine test
compilers exhausted host memory; ONE heavy engine compile may run at a
time now. a01 has the current validation window; a03, a04 and root wait,
continuing research/review. The lead grants each next window explicitly.
Machine note: automatic policy blocked removal of idle p3's target cache;
nothing was deleted. Private `task.sh` now warns below 40 GB free instead
of deleting caches, and retains scratch evidence. Do not use another
deletion path to bypass that denial.

Next steps:

1. Continue the count-payload dependency and real-save validation; build 40
   is published and all three acceptance routes have passed.
2. Collect completed slots with one `cycle.sh NN 0 0` at a time. Never
   cycle an active agent's worktree. Rotate prepared tasks as capacity opens.
3. Ship updated #116 when both checks pass; rebase affected branches
   after another merge, retaining tests and regenerating the ledger.
4. Rebase #115 after the next main merge and require both CI checks.
   Payload conversion, world ownership and save round trips
   remain open; decoding a save does not prove a lossless round trip.

## Where things stand

- **Phase 0** (engine map, ledger) is merged (#72). `docs/LEDGER.md` is the
  coverage table; regenerate with
  `cargo run -q --release --manifest-path scripts/ledger/Cargo.toml`.
- **Engine crate** (`crates/engine`, ADR-0006): 10,454 of FalloutNV.exe's
  functions are translated (15.8% of all, see LEDGER.md), each with a
  `// Translated from <addr>` marker and tests. They run in the crate's
  emulated 32-bit memory, not yet in play.
- **Previously translated units (dependency closure is not established):** main.cpp, actor.cpp (all 7 parts), playercharacter.cpp
  (all parts), interface.cpp, interfacemanager.cpp, processlists.cpp,
  tesobjectrefr.cpp, extradatalist.cpp, extradataobjects.cpp, tesscript*.cpp,
  tesform, tesfile, tesland, tesworldspace, teswater, tesnpc, package,
  inventorychanges, bipedanim, animation, modelloader, tesobjectcell,
  bgsdecalmanager, navmeshobstaclemanager, globalfunc, pathfind.cpp,
  BSMenu/tile.cpp, highprocess.cpp parts 1, 2, 4 and 5, navmesh.cpp,
  pathinglocation.cpp, bgssceneinfo.cpp, bgssaveloadgame.cpp, and others.
- **In progress (translation lanes; see "Resume here" for current owners):**
  navmesh_util.cpp (a01, active), pathmanager.cpp (a03, prepared), explosion.cpp
  (a04, active), aitaskmanager.cpp lane 2 (a05, prepared; partial behavior
  listed in the file header), bhkworld.obj (a06, prepared).
- **Phase 1 (frame skeleton) wiring:**
  - PR 1 frame map (#84), PR 2 `world::frame` (#86), PR 3 Bevy order from the
    frame (#90): merged.
  - PR 4 player stage (#101): merged. It added `world::frame::player` and
    `frame_order::PlayerSet`.
  - PR 5 world and time stage (#105): merged.
  - PR 6 AI task stage (#107): merged.
  - PR 7 interface and render (#113): merged as `f917219` after both checks.
    Acceptance passed; exact open boundaries remain in FRAME_SKELETON.md.
  - Phase 2 (state model in engine memory, save round trip; see
    ENGINE_PORT_PLAN.md).
- **Rolling translation PR:** #109, #112 and #114 merged; #116 is open on branch
  `claude/engine-crate` (see "Resume here").
- **Play build** (the user's `Desktop\nv-rs-play`): Build 40 = main `f917219` with #113.
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
