# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

Latest, 04:22 local: HUD collection `5c12df2` passed combined engine tests,
strict lint and formatting, including the reviewed radiation audio correction.
Fresh `/root/a03_round16` immediately continues large `00775a00` in b0236.
The publication hold is released: publish the accumulated movement, HUD and
explosion batch after regenerating the ledger. a01 now owns the compiler for
its forty-body continuation; then cycle01/fresh worker, root's two map helpers,
then a04's forty effect-item routines. Effect-item review includes thirty-six
matching native float vectors and callback reload/argument ownership fixes.
Reader #130 core CI passes; viewer CI remains pending at `2b81e2a`.
Build 40 remains current, without a confirmed next play-build date or feature
list. Inventory stack preservation still has no playable consumer. Usage is
75%; retain the 98% stop/handoff rule.

Latest, 04:09 local: explosion collection `5380adf` passes 13,990 combined
engine tests, doctest, strict lint and formatting. Reviewed partial-only
advancement retains all seventeen exception-cleanup gaps. Fresh
`/root/a04_round14` immediately started b0677, effectitem.cpp, compiler held.
a03 now validates its eight-body HUD batch plus radiation audio correction;
then cycle03/fresh worker, a01/cycle01 and root's map follow-on. Keep the
later rolling push held until the HUD correction is collected. Root's
separately compiled map draft passes the full item load and byte-identical
41-byte rewrite; formal generated-registry validation is still pending.

Latest, 04:00 local: rolling #129 passed both checks and merged `1389dab`
at 03:56:28. `ship.sh 129` completed with `HOLD_LATER_PUSH=1`: the later
movement/HUD collections are rebased locally but unpublished. Do not ship
the queued HUD until a03's correction to `007738c0` is collected: its audio
singleton getter had been given another callee's pending stack arguments.
After that collection, regenerate the ledger and publish the next rolling
PR (no rolling PR is currently open). HUD collection is now `f34752a`.

Inventory reader #130 is open/attached at `2b81e2a`, with fresh CI. Local
checks on merged #128 passed 15,771 workspace / 13,918 engine tests, strict
lint, fmt and release/debug builds; it rebased cleanly onto #129. The
buffered two-stack reader probe passes, consuming all 41 bytes. The fuller
ItemChange probe stops while inserting its generated form into the real
map: allocator `0043a010` is missing. No byte-identical rewrite result yet.
Root is now on `codex/phase2-inventory-form-map`, based on #130, with two
helpers and three tests drafted in `fallout_shared/form_map_support.rs`
and its generated `mod.rs`, uncompiled. The allocator's refill `0043a080`
is included; PC evidence is private `codex-audit/inventory-map-*`.
a01 is reviewing this draft. a04 owns the heavy compiler, then cycle04,
a01/cycle01, a03/cycle03; root's map follow-on waits. a03's next collection
is eight normal bodies plus the radiation audio fix; three new SEH partials
are explicit. Large `00775a00` remains wholly open for a fresh continuation.
Build 40 remains current. Usage was 66% at 03:48; stop/handoff at 98%.

Latest, 03:46 local: inventory writer #128 passed both checks and merged
`14c77da` at 03:39:58. Root reader checkout is rebased onto that merge;
scoped stash `9ca22a2` is applied and retained (do not apply again).
Cycle03 completed HUD collection `74cd13b`: 13,970 combined engine tests,
one doctest, strict lint and formatting pass. Fresh `/root/a03_round15`
immediately continues at `00774b00`; preserve Create/DoIdle SEH partials.
Root now owns the compiler for `check-inventory-load.ps1`, session `3847`,
then the buffered-reader and full ItemChange payload probes. Both probes
remain unrun. a04 follows, then a01; both have reviewed drafts ready.
a01 has forty new normal entries through `00933890`, with sound argument
arity and signaling-NaN store corrections. a04 has both remaining normal
explosion bodies; reviewed partial-only advancement remains authorized
after successful checks. Rolling #129 core passes and viewer is pending
at `12f6153`; ship when both pass. Build 40 remains current, without a
confirmed next play-build date or shipped inventory consumer.

Latest, 03:30 local: #127 passed both CI checks and merged `72003ab` at
03:23:48. `ship.sh 127` completed; #129 is attached at `12f6153`, titled
"Engine: translate explosion effect processing", with fresh CI. That
explosion collection is now `dc89abd` after rebasing. #128 core passed;
viewer remains pending. It can merge at its exact checked head independently.
Movement source `6ae7a25` passed 65 focused / 13,889 source engine / doc /
strict lint / fmt / 49 hygiene tests. Cycle01 completed as `c757035` with
13,925 combined engine tests plus doc, lint and fmt. Fresh `/root/a01_round14`
immediately started the next normal entries at `00931880`; four reviewed
SEH-only partials remain in the ledger. a03 now owns the compiler, then
cycle03, root reader/probes, and a04. a04 has both remaining normal bodies
drafted and reviewed, with a full blocked-then-clear ray test; all seventeen
remaining partials have verified PC exception frames. After its checks,
partial-only lane advancement is reviewed/allowed. Root's fuller ItemChange
probe now requires byte-identical payloads after loading and writing again;
it has not run. Usage is 60% at 03:27; the 98% stop rule remains active.

Rolling #126 passed both checks and merged as `97e7572` at 02:47 local.
`ship.sh 126` completed; attached rolling #127 is at `cb42c4d` with fresh
CI, containing MobileObject collection now `7a1f056` and loader collection
now `43a8ee1`. Save-buffer #125 passed both checks and merged `68387b4`
at 03:04 local. Inventory writer follow-on is open/attached as #128 at
`4b68ea0`, with fresh CI after rebasing onto #125. Its branch is
`codex/phase2-inventory-payload`. Final session `30664` passed 15,691 core tests (13,838
engine), strict lint, formatting and release/debug builds. Seventeen
dependencies now let the real-registry writer produce a 41-byte payload
with delta -2 and two separate count/condition stacks (2/42.5 and 7/17.5).
The payload decoder consumes it fully and the save-ID map resolves the
item correctly. No FOS file, native load round trip or world consumer yet.
Private evidence: `codex-audit/inventory-buffer-*` and
`inventory-next-native*`; ten generated native vectors pass.
The managed phase2-extra-data checkout now uses `codex/phase2-inventory-load`,
based on #128. Its two dirty unit files are `bgsloadgamebuffer.rs` and
`bgsloadformbuffer.rs`: eleven reader helpers and eight tests, uncompiled.
Peer review is clean and ten generated native reader vectors pass.
`inventory-load-probe.rs` and its runner are prepared, unrun. The fuller
`inventory-item-load-probe.rs` also exercises the ItemChange loader through
the real form map and installed RTTI, with a generated TESBoundObject header;
it is prepared but unrun. `check-inventory-load.ps1` serializes root checks.
Scoped stash
`9e0dd23` has been applied and retained; do not apply it again.

Latest batch: animation loader source `99fb056` collected as `6c569b7`;
13,879 combined engine tests, strict lint and formatting pass. All 44
unit entries have normal behavior; only SEH cleanup at `005f21c0`,
`005f22d0`, `005f3a20` remains partial. Reviewed partial-only advancement
retains these gaps. Fresh `/root/a03_hudmainmenu` immediately started b0236,
HUDMainMenu, compiler held. Explosion `009ae6e0` source `64750e3` was
collected as `b9871e9` at 03:13 local: 113 focused, 13,830 source engine,
49 hygiene and 13,885 combined engine tests plus doc, lint and fmt pass.
Only compiler SEH cleanup is partial. Fresh `/root/a04_round13` immediately
started `009b00a0`, then `009b1810`, compiler held. a01 now owns the compiler;
then a03 and root. All six larger movement bodies have drafts and tests;
Move is now registered after the vector/control-flow review and 65 focused
tests pass; full engine checks are underway. a03 now has 40 registered HUD
bodies and 45 tests, Create and DoIdle SEH-only partials, and awaits its
compiler grant. Both remaining explosion bodies are drafted and under review.
ProcessTargets has no PC exception frame; its mistaken SEH partial marker
is being removed. RunLOSPick retains its actual compiler-cleanup partial.
Review corrections include vector
receiver/output argument order, post-callback globals and x87 boundaries.
Root reader validation waits after the slots. Build 40 remains
current; no reliable next-build date or confirmed feature list exists yet.

User resumed after the usage reset on 2026-10-11. Continuous translation
is active: collect each finished, committed slot with `cycle.sh NN 0 0`,
then immediately start a fresh agent on its new task. Never reset or cycle
a slot with unfinished work. Ship rolling PRs once both current-head checks
pass. The saved 98% usage stop/handoff rule still applies.

Rolling #124 passed both checks and merged as `c6ac7eb` at 02:12 local.
`ship.sh 124` succeeded; rolling #126 is open/attached at `9343898`, with
animation collection now `00a246f` and Init3D now `ca298df`. Fresh CI gates it.
Save-buffer PR #125 is based on #124, now at `685c26c`, with fresh CI.
Its generated-ledger delta was removed so rolling integration owns that
shared snapshot and independent source PRs can merge without count conflicts.
Source is unchanged. Follow-on safety stash `a7e3ae0` was reapplied and
retained; do not apply it again.
Its 18 dependencies pass 15,641 core tests (13,788 engine), strict workspace
lint, formatting, release and debug builds. The real-registry probe writes
count 5 / health 42.5 and the FOS payload decoder reads exactly both back.
It uses the executable's flag table, a generated mode receiver and partial
allocator startup; no game-function doubles, FOS file or full round trip.
Private evidence is in `phase0/codex-audit/save-buffer-probe-result.txt`.
Root session `37952` finished. Init3D source `d4fabeb` collected as
`407caf3`: 107 focused / 13,760 source / 13,824 combined engine tests,
49 hygiene tests, strict lint and formatting pass. Cycle04 session
`77634` completed and fresh `/root/a04_round12` immediately started
`009ae6e0`. a01 now owns the compiler; then cycle01, a03/cycle03, root.
Cycle01 has since completed as `8f92810`: 34 MobileObject functions,
13,864 combined engine tests, strict lint and formatting pass. Source
`f7e4c04` passed 40 focused, 13,830 engine and 49 hygiene tests. Fresh
`/root/a01_mobileobject_cont` immediately started the six remaining larger
bodies in address order, with compiler held. a03 now has the exclusive
compiler for its five-body loader batch; then cycle03, then root inventory.
a04's `009ae6e0` normal-path draft remains unregistered during review:
callback snapshots, x87 stores, distance comparison and contact-sort
pointer fixes have regressions prepared, uncompiled. Start fresh workers
after each cycle. Root's 16-helper/15-test inventory follow-on and private
two-stack payload probe remain prepared, uncompiled.

Reference #122 and rolling #123 passed both checks and merged as `6bea9a1`
and `8b5eebe`. Rolling #124 is open/attached at `9c519be`, containing loaded
area bounds (`e61c6a9`, formerly `86886d9`): 13,790 combined engine tests,
strict lint and formatting passed before rebasing. Fresh core/viewer CI
gates this head. Later animation collection `6dbf98f` stays local behind it
and passes 13,820 combined engine tests, strict lint and formatting.
Merged rolling PRs: #109 `b995218`, #112 `d854fbe`, #114 `89e1a0b`,
#116 `c23f744`, #119 `951c0c7`, #120 `3a92d0a`, #121 `eda98da`,
#123 `8b5eebe`.
Leave unrelated contributor PRs alone.

Slot state:

- a01 `/root/a01_mobileobject`, `agent-01`: fresh b0155 worker owns
  `fallout_ai/mobileobject.rs`, 34 functions/tests prepared with compiler held.
  Six larger bodies remain open: `0092ec40`, `0092ed60`, `0092f260`,
  `009308f0`, `00930c70`, `00931710`. Reviews corrected getter results,
  callback field reloads, a spurious vcall argument and x87 quieting;
  eligible cleanup and controller show/hide tests were added.
  LoadedAreaBound source `44c5bda` was collected as `86886d9`: 29 complete,
  three SEH partials, 34 focused / 13,758 source engine / 49 hygiene tests,
  strict lint and formatting pass. Reviewed argument arity, callback
  reloads and wrapping pointer arithmetic are corrected. Its cycle wrote
  the new MobileObject task and a fresh agent started immediately.
  cycle01 collected `a902f84`: `00699b60` normal path and the `0069ad40`
  immediate-vtable correction pass 121 focused and 13,724 combined engine
  tests, lint and formatting; the source also passed 49 hygiene tests.
  Exactly three NavMesh SEH partials remain: `006987d0`, `00699b60`,
  `006bd040`. Reviewed partial-only advancement retains them in the ledger.
- a04 `/root/a04_round12`, `agent-04`: fresh worker after cycle04 collected
  `d781c3b`. `009ae280` and `009b2940` normal paths remain SEH-partial;
  98 focused and 13,722 combined engine tests, lint and formatting pass.
  The source commit also passed 49 hygiene tests. Five larger paths remain
  untouched: `009acd60`, `009ad670`, `009ae6e0`, `009b00a0`, `009b1810`.
  `009acd60` is now collected as `e4de812`, source `6d0bb52`: five focused
  tests, 13,727 source engine tests, strict lint/fmt and 49 hygiene pass.
  Argument order, callback reloads, impact-object naming and x87 stores
  were reviewed against the PC instructions. It remains SEH partial.
  `009ad670` is now collected as `407caf3` with four tests; its fresh
  worker starts `009ae6e0` with the compiler held. `009b00a0` and
  `009b1810` remain untouched. Later-address evidence is in
  private `agents/scratch/a04/round9-009b1810-handoff.md`. Research/editing
  only until the compiler grant. Root PC review caught action-record
  offsets and CString argument arity; both are corrected, and x87 stores
  now have signaling-NaN coverage. The draft adds a fifteenth SEH partial.
  Prior worker reviewed root save-buffer code read-only with no further issue.
- a03 `/root/a03_loadanimgroup`, `agent-03`: fresh b0041 worker owns
  `fallout_shared/tesanimgroup.rs`, continuing the full `005f3a20` normal
  path with compiler held. Five new bodies and 14 tests are prepared;
  static checks pass (15 new tests, 28 total in the file). Peer review corrected note-getter callback reloads,
  empty-note log arity, sound error scanning and tail diagnostic reloads.
  The file has 41 complete bodies and three SEH partials including the
  loader; runtime checks remain queued. Its predecessor committed `f85e4d8` and cycle03
  collected `6dbf98f`: focused 13 / full 13,764 / hygiene 49 pass, strict
  lint/fmt pass. Seven queue entries remain, including two SEH partials.
  37 complete bodies and two SEH partials are registered. `005f3a20` remains
  unregistered/open; its full normal path is unfinished, with private
  `scratch/a03/005f3a20-handoff.md` for the next cycle. Reviews corrected
  array callback reloads, table stride, an erroneous constructor cleanup
  call, x87 stores and filename-parser callback ordering. Thirteen native
  leaf vectors passed; every registered address has test coverage.
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
  now uses follow-on branch `codex/phase2-inventory-payload`, based on #125.
  The checked save-buffer branch remains pushed separately at `574c3c5`.
  Inventory follow-on has 16 new helpers and 15 tests prepared but
  uncompiled: form-ID encoding, bidirectional maps, constructors and
  callback allocation/release. Root owns its twelve unit files; no other
  agent may edit them. `git status` lists the exact files. Seven native
  generated vectors pass (created-ID decoding and key equality), and
  independent static review of the first fifteen bodies found no concrete
  mismatch. The last helper constructs a list node (`00470440`), checked
  against PC assembly. Private `inventory-buffer-probe.rs` is prepared but
  not run; it writes one entry with delta -2 and two distinct count/health
  stacks, then uses the FOS inventory decoder. No world changes.
  Safety stash `063e7833f97201d3ff909bb96784ddd3f0050afb` was applied after
  rebasing the follow-on onto updated #125; retain but do not reapply it.
  PR #122 (`codex/phase2-reference-state`) merged as `6bea9a1` after both
  checks passed on head `8e11001`. Its ten form helpers,
  nine allocator/counter helpers, form callback-reload corrections and the
  new Reference adapter pass 15,587 core tests (13,734 engine), workspace
  clippy with warnings denied, formatting, release and debug builds.
  Native spot checks: five header, two zero-capacity array and three
  allocator-header vectors. The follow-on is now based on merged #123.
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
- The follow-on is committed in #125: form_support, bgsloadformbuffer,
  bgssaveloadinitialdata, bgssavegamebuffer, bgssaveformbuffer,
  bgsloadgamesubbuffer, run_008660b0, run_008661e0 and run_00ec7ce3. Eighteen
  constructor/getter/writer dependencies and generated tests pass validation
  in completed root session `37952`; six guarded constructor vectors passed the native executable.
  Byte writing, growth, insertion and variable-size value paths are checked;
  CRT checked movement at `00ec7ce3` is checked with three native
  generated cases passing. Its error handler/TLS callees remain untraced;
  error-path tests use doubles. Private evidence uses the
  `codex-audit/save-boundary-*` and `save-buffer-headers-native*` files.
  `ENGINE_STATE.md` and the ledger are committed. Private `save-buffer-probe.rs` uses real
  executable tables, a generated mode receiver and the translated writer,
  then checks the bytes through the FOS payload decoder; it passes.

ONE heavy engine compiler across all worktrees, `-j2`, existing debug cache,
`CARGO_PROFILE_DEV_DEBUG=0`. a01 MobileObject owns the compiler; root
save-buffer session `37952` and cycle04 `77634` completed. Next: cycle01,
then a03/cycle03 and root inventory follow-on. Disk is about 2.6 GB free.
Automatic approval review previously blocked
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
