# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

User resumed after the usage reset on 2026-10-11. Continuous translation
is active: collect each finished, committed slot with `cycle.sh NN 0 0`,
then immediately start a fresh agent on its new task. Never reset or cycle
a slot with unfinished work. Ship rolling PRs once both current-head checks
pass. The saved 98% usage stop/handoff rule still applies.

Rolling #121 passed both checks and merged as `eda98da`. The lead branch
`claude/engine-crate` is rebased onto it. The next rolling batch is ready
to publish: multibound `feac2df` and Explosion Initialize `e4de812` pass
13,756 combined engine tests, lint and formatting. Three multibound SEH
partials and Initialize's SEH partial remain explicit. Inspect the attached
rolling PR after publication; retain later local commits behind its head.
Merged rolling PRs: #109 `b995218`, #112 `d854fbe`, #114 `89e1a0b`,
#116 `c23f744`, #119 `951c0c7`, #120 `3a92d0a`, #121 `eda98da`.
Leave unrelated contributor PRs alone.

Slot state:

- a01 `/root/a01_loadedarea`, `agent-01`: fresh b0405 worker owns
  `fallout_shared_havok/loadedareabound.rs` (32 functions), prepared but
  validating with the exclusive compiler. All 32 bodies and 34 tests are written; three constructors
  or destructors retain explicit SEH partials. Reviewed argument arity,
  callback reloads and wrapping pointer arithmetic are corrected.
  cycle01 collected `a902f84`: `00699b60` normal path and the `0069ad40`
  immediate-vtable correction pass 121 focused and 13,724 combined engine
  tests, lint and formatting; the source also passed 49 hygiene tests.
  Exactly three NavMesh SEH partials remain: `006987d0`, `00699b60`,
  `006bd040`. Reviewed partial-only advancement retains them in the ledger.
- a04 `/root/a04_round11`, `agent-04`: fresh worker after cycle04 collected
  `d781c3b`. `009ae280` and `009b2940` normal paths remain SEH-partial;
  98 focused and 13,722 combined engine tests, lint and formatting pass.
  The source commit also passed 49 hygiene tests. Five larger paths remain
  untouched: `009acd60`, `009ad670`, `009ae6e0`, `009b00a0`, `009b1810`.
  `009acd60` is now collected as `e4de812`, source `6d0bb52`: five focused
  tests, 13,727 source engine tests, strict lint/fmt and 49 hygiene pass.
  Argument order, callback reloads, impact-object naming and x87 stores
  were reviewed against the PC instructions. It remains SEH partial.
  The fresh worker starts `009ad670`; the other three bodies remain
  untouched. Later-address evidence is in
  private `agents/scratch/a04/round9-009b1810-handoff.md`. Research/editing
  only until the compiler grant. Fourteen collected SEH partials remain.
- a03 `/root/a03_animgroup`, `agent-03`: fresh b0041 worker owns
  `fallout_shared/tesanimgroup.rs` (40 functions), prepared, compiler held.
  37 complete bodies and two SEH partials are registered. `005f3a20` remains
  unregistered/open; its full normal path is unfinished, with private
  `scratch/a03/005f3a20-handoff.md` for the next cycle. Reviews corrected
  array callback reloads, table stride, an erroneous constructor cleanup
  call, x87 stores and filename-parser callback ordering. Thirteen native
  leaf vectors passed; Rust tests are still uncompiled.
  cycle03 collected all 30 multibound entries as `20c6123`; source
  `0947e87` passed 27 focused / 13,744 engine / 49 hygiene tests, strict
  lint and formatting. Combined lead tests pass 13,751. Seven generated
  shift cases and one matrix signaling-NaN case also passed the native
  executable. Three exception-cleanup partials remain: `004391c0`,
  `00439410`, `00439490`. The previous pathmanager lane retains
  17 exception-cleanup partials in the ledger; cycle advanced only after
  confirming no untouched entries, using `ADVANCE_PARTIAL_ONLY=1`.
- a05 b0158 and a06 b0360 remain prepared but idle. Runtime permits three
  children currently; the user permits up to six.

Lead Phase 2 work:

- ExtraData list #115 merged `94165e8`; count #117 merged `cbca26e`;
  health #118 merged `a0ee61d`, each after both CI checks. The real-save
  payload probe checks 20 health and 14 count values across 3,319 lists /
  3,365 forms with the source unchanged. This is not a save round trip.
- Managed checkout `%USERPROFILE%/.codex/worktrees/phase2-extra-data/wt-phase0`
  now uses follow-on branch `codex/phase2-extra-save-buffer`.
  PR #122 (`codex/phase2-reference-state`) is open and attached, rebased
  onto #121 as `8e11001`, with fresh CI. Its ten form helpers,
  nine allocator/counter helpers, form callback-reload corrections and the
  new Reference adapter pass 15,587 core tests (13,734 engine), workspace
  clippy with warnings denied, formatting, release and debug builds.
  Native spot checks: five header, two zero-capacity array and three
  allocator-header vectors. Core/viewer CI now gates merging #122.
- The allocator test failure was a fixture expectation: `Mem::alloc` rounds
  to eight bytes. The fixed test verifies requests of 4/13 separately from
  backing blocks of 8/16. Twenty allocator-focused tests pass.
- Private `phase0/codex-audit/reference-bootstrap-allocator.rs` passes real
  registry construction of TESObjectREFR, embedded list, count/health updates
  and count removal while preserving health. It initializes only the PC
  allocator sequence through `00aa2020`, not full BSSystemUtility startup.
  No game-function doubles or game-file writes. No public creation API yet.
- `reference.rs` plus `lib.rs` add an existing-reference handle and embedded
  list accessor. Its generated isolation/aliasing regression and updated
  installed-executable probe pass. Independent source review found no
  concrete adapter issues. World consumers, payload conversion and FOS
  writing remain open. Next research traces buffered extra-data saving
  (`00426a30`) and the currently scaffolded save-buffer constructors.
- The follow-on has five uncommitted files: form_support, bgsloadformbuffer,
  bgssaveloadinitialdata, bgssavegamebuffer and bgssaveformbuffer. Nine
  constructor/getter dependencies and generated tests are prepared but
  uncompiled; six guarded constructor vectors passed the native executable.
  Byte-writing/growth dependencies remain open. Private evidence uses the
  `codex-audit/save-boundary-*` and `save-buffer-headers-native*` files.

ONE heavy engine compiler across all worktrees, `-j2`, existing debug cache,
`CARGO_PROFILE_DEV_DEBUG=0`. a01 owns the compiler now; root validation
session 87210 and cycle04 session 14042 completed successfully. Then
cycle01, a03, cycle03, root save-buffer checks when prepared. Disk is about
3 GB free. Automatic approval review previously blocked
idle p3 target deletion; nothing was deleted. Do not bypass that denial.
Private task.sh warns rather than deleting caches and retains scratch.
Private ship.sh now safely recreates an absent merged branch with an empty
lease, refuses an unexpected nonempty remote head, and avoids a push when
there are no later commits. After a transient query failure during #120
shipping, it also explicitly stops if the remote-ref query fails. #120's
merge and #121's published head were independently verified. Syntax passes.

Build 40 remains the published play copy from merged Phase 1 PR 7 (#113,
`f917219`). Core/viewer checks and three scripted routes passed: Doc 63 s,
VCG02 410 s, VMS16 339 s. App/source SHA-256 matched; WHATS-NEW is LF.
Evidence: `%USERPROFILE%/nv-re/acceptance/build40-main-20261010`.
No original-game comparison was performed. Exact menu-input/GPU renderer
boundaries remain in FRAME_SKELETON.md. These translation batches do not
establish new playable features.

Next action: publish the checked rolling batch and collect a01 after
validation. Merge #122 after both current-head checks pass, and continue
collecting and immediately restarting finished translation slots.

## Where things stand

- **Phase 0** (engine map, ledger) is merged (#72). `docs/LEDGER.md` is the
  coverage table; regenerate with
  `cargo run -q --release --manifest-path scripts/ledger/Cargo.toml`.
- **Engine crate** (`crates/engine`, ADR-0006): current function coverage
  and explicit partial counts are in LEDGER.md, each translation with a
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
  loadedareabound.cpp (a01, prepared), tesanimgroup.cpp (a03, active), explosion.cpp
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
- **Rolling translation PR:** #109, #112, #114, #116, #119, #120 and #121 merged; next batch on branch
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
