# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

Codex takeover, 2026-10-10: rolling PRs #109, #112, #114 and #116 passed
both CI checks and were squash-merged as `b995218`, `d854fbe`, `89e1a0b`
and `c23f744`. The lead branch is `claude/engine-crate`. Its later batches
include explosion collection `a2fe87a`: the owner/weapon callback-reload
correction and nine more functions pass 13,695 combined engine tests,
lint and formatting. Rolling PR #119 is open and attached, now rebased
onto health merge `a0ee61d` with the checked navigation continuation
included. Fresh CI gates this combined head. The old remote branch was absent
after #116 merged; an empty-ref lease safely recreated it.

Phase 1 PR 7 (#113) passed both checks and merged as `f917219`.
The lead branch is rebased onto current main `a0ee61d` (health PR #118).
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
three children. Fresh agent creation works again; each collected slot gets
a fresh worker, with heavy compilation serialized across all worktrees.

Slot state: a01 navigation continuation is owned by fresh worker
`/root/a01_round7`. Collection `b50add8` adds three complete functions and
fixes `0069b030` to reload its buffer after a callback that may relocate it.
The regression verifies the old storage is untouched; 13,638 combined
engine tests, lint and formatting pass. Two normal-path geometry translations
(`006987d0`, `006bd040`) were collected as `d24c517`, with explicit
exception-cleanup partials and callback regressions. All 13,698 combined
engine tests, lint and formatting pass. This follow-on is included in
rebased PR #119. `00699b60` remains unregistered: its edge/list ownership and
repair paths are unresolved; the fresh worker owns that investigation.
a03 b0331 pathmanager.cpp is running under fresh `/root/a03_round4`.
Collection `7321030` adds 29 complete and six partial functions plus all
reviewed corrections to earlier argument-slot ownership, output ordering,
callback reloads and completion pointers. The combined 13,680 engine
tests, lint and formatting pass. Forty entries remain: 15 explicit
exception-cleanup partials and 25 untouched, starting `006ec5b0`.
a04 b0160 explosion.cpp is running under fresh worker `/root/a04_round6`.
Its latest 21-function continuation was collected as `00532d6`; 13,633
combined engine tests, lint and formatting pass. Native sort permutations
cover duplicate keys, NaN, signed zero and infinities. Sixteen entries
remain, starting `009ac5a0`. New PC review requires `009b0f80` to reload
the owner after its virtual IsActor callback; the fresh worker owns that
correction and a mutation regression. It must also mark the already
documented omitted exception cleanup as explicit partials. Keep this local
batch was held until corrected; merged #116 excludes it. Collection
`a2fe87a` now passes all checks: six new complete functions, three new
partials and eight older partials explicitly marked. The fresh task has
18 entries: eleven exception-cleanup partials and seven untranslated.
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

Lead Phase 2 PR #115 passed both CI checks and merged as `94165e8`, branch
`codex/phase2-extra-data`, in managed worktree
`%USERPROFILE%/.codex/worktrees/phase2-extra-data/wt-phase0`.
It adds the engine-owned ExtraData list handle and four traced helpers.
Local core tests, clippy, formatting, release build and both CI checks passed.
The installed-executable real-registry probe passes construction, add,
lookup and detach. No world/viewer consumer or save writer is added.
Private probes/native vectors and the read-only 11-save baseline are in
`phase0/codex-audit`; FOS_SAVES.md records the decoder limits.
The count follow-on #117 passed both checks and merged as `cbca26e` at
21:56 local time. Health PR #118 passed both fresh checks and merged as
`a0ee61d` at 22:32 local time. The health implementation is unchanged.
Their milestone notes are separate
from rolling history, and their generated ledger changes are deferred to
rolling integration. Merge-tree checks against #116 pass without conflicts.
Health checks passed: 74 focused tests, all 15,315 core tests, lint, formatting
and release build. The real-save probe passes all 20 health and 14 count
payloads with the source hash unchanged; count source is unchanged in
this validated follow-on. No root process is active. The managed checkout
is now `codex/phase2-reference-state`, commit `9469f51`, based on #115.
Ten form-construction helpers and two callback-reload fixes pass 124
focused tests, all 13,466 engine tests, lint, formatting and debug build.
Five native header and two zero-capacity array vectors pass. The actual
reference probe now reaches `00aa1070` and stops at the uninitialized
allocator. Nine allocator and counter-import helpers and their tests are
prepared but uncompiled in `nisystem/allocator_support.rs`, its module
registration, `nisystem/bsniallocator.rs`, `bscore/memorymanager.rs`
and `fallout_shared/form_support.rs`.
Three native allocator-header vectors pass. No public reference adapter
exists yet; next validate and continue the real startup dependency chain.

Simultaneous engine compilers exhausted host memory; ONE heavy engine
compile may run at a time. a03 has the current validation window;
root is next. Other workers wait,
continuing research/review. The lead grants each next window explicitly.
Machine note: automatic policy blocked removal of idle p3's target cache;
nothing was deleted. Private `task.sh` now warns below 40 GB free instead
of deleting caches, and retains scratch evidence. Do not use another
deletion path to bypass that denial.

Next steps:

1. Continue reference-constructor dependency research/validation while
   health PR #118 awaits CI. Build 40 and all three routes passed.
2. Collect completed slots with one `cycle.sh NN 0 0` at a time. Never
   cycle an active agent's worktree. Rotate prepared tasks as capacity opens.
3. After the explosion correction passes collection, push and open the
   next rolling PR; ship when both checks pass. Preserve local review holds.
4. Merge #118 after both fresh checks pass on its new main-based head.
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
