# Handoff: engine port push (2026-10-10, evening)

For the next Claude session leading the nv-rs engine port. Read AGENTS.md,
docs/MILESTONES.md, docs/ENGINE_PORT_PLAN.md, docs/FRAME_SKELETON.md and
docs/ENGINE_CRATE.md first. This file says where things stand and how the
machinery runs.

## Resume here

### Current state, 2026-10-11 16:10 EDT

This section supersedes the historical notes below.

- #143 merged `a24bc31` after both current-head CI checks. Its shipping
  session 78829 is DONE; never rerun ship143.
- #144 merged `4116868` after both current-head CI checks passed.
  Rolling #145 remains open and attached, now rebased onto that merge with
  its ledger regenerated. Both fresh current-head checks must pass before
  shipping; checks for the prior `df1a5e4` head do not validate this rebase.
- HUD source `aabfc8b` passed 121 focused and 14,386 engine tests, the
  doctest, strict lint, formatting, 49 hygiene checks and release build.
  cycle03 session 72751 is DONE: collection `34ee119` passes 14,410
  combined engine tests, the doctest, strict lint and formatting. Seventeen
  reviewed compiler-only partials remain explicit (two legacy comments
  needed their missing addresses). All ordinary HUD bodies are translated.
  Fresh `/root/a03_round20` started immediately on b0223, owning only
  `fallout_misc/bgssaveloadchangesmap.rs` and `bgssaveloadfile.rs`.
  It is compiler-held and must not reset the slot.
- `/root/a04_water_complete` owns `fallout_shared/bgsautowater.rs`:
  thirty-seven normal bodies, five SEH-only partials, forty-four drafted
  tests and fifteen passing native comparisons. It is validating now;
  initial pointer type-annotation errors were corrected, tests still await
  results. `/root/a01_round19` now has the other compiler lane for its
  forty TaskQueueInterface bodies and forty-two tests. Four are complete;
  thirty-six retain reviewed compiler-only partials. Its forty-two focused
  and 14,445 engine tests pass; strict lint and release checks continue.
  After collection its
  fresh worker must continue ordinary bodies at `0087b650`.
- Root PR #144 merged from `94e709c`, branch
  `codex/phase2-reference-predicate`. Three helpers and four regressions
  pass 16,238 workspace tests, strict lint, formatting and release/debug
  builds. Eighteen native cases pass. Both final release probes load a
  selected original scripted inventory item and write it byte-identically,
  with both expected BEGIN handlers. One uses an official save's exact
  ten-plugin order and actual three-variable defaults. Its saved variable
  list is empty. Historical plugin contents, ordinary script execution,
  whole-save preservation and world/viewer integration remain unverified.
  Both current-head CI checks passed before merging.
- Root follow-on source checkpoints `8ebef74` and `e705c1d` on
  `codex/phase2-script-local-values` owns `fallout_shared/tesscript.rs`,
  `fallout_misc/bgsloadgamebuffer.rs`, `bgssavegamebuffer.rs` and topic
  `ENGINE_STATE.md`. It fixes
  raw flag copying, signaling-NaN stores and live list-item lookup, and adds
  two saved-form reader helpers and the numeric-form writer. SaveGame
  reloads the reference ID and effect flag/pointer after helper calls.
  Thirteen regressions are drafted but unrun; 102 native cases pass.
  Independent PC/source review passes, including the corrected test fixtures.
  It is rebased onto #144; run the prepared full check pipeline and
  final normal-registry release probes (nine LoadGame and eighteen SaveGame
  vectors) when a lane is free. An additional eighteen native three-leg
  SaveGame/LoadGame/SaveGame chains pass, carrying actual results forward;
  their prepared Rust release probe remains unrun. Its baseline stops at
  the now-added writer `00865db0`.
- Fresh a03's b0223 batch now has forty normal bodies and forty-three
  drafted tests. Thirty-three native cases and independent PC review pass.
  Eight compiler-only partials remain explicit. Cargo checks are queued.
- Limit full Cargo pipelines to two: water and taskqueue now own them.
  Root and fresh a03 are source-only. No deletion attempts after automatic
  review rejected cleanup and the user removed idle caches manually.
- Build 40 remains current, without a confirmed next-play date or new
  visible feature. Continue cycling and shipping until stopped. Preserve
  the user's 98% weekly stop/handoff rule (70% around 16:02 EDT).
### Active again, 2026-10-11 after the usage reset

This section supersedes the historical stopped state below. User requested
continuous work and manually removed the idle p3 build caches; free space
was 25,230,925,824 bytes afterward. Automatic review had rejected our
deletion attempt with only "blocked by policy"; no bypass was attempted.

- #135 merged `7907420` and #134 merged `89bbde4`, each after both CI checks.
  #136 also merged `95ab97a` after both checks. #137 merged `935701e`, and
  root #138 merged `1614889`, each after both current-head checks passed.
  #139 merged `575378a` after both checks. Collection was active, so the
  remote merge happened first; private `finish-ship139.sh` then completed
  rebase/ledger/push and opened attached rolling #141 at `7a6cc48`. This
  continuation is DONE; never rerun ship139. Ship #141 when both checks pass.
  #140 merged `4166432` after both current-head checks passed; its head was
  `192b46e`. #141 then passed both checks and merged `c56a2d1`.
  ship141 is DONE. Attached #142 publishes the HUD collection at `ed11d75`
  and awaits both checks; its collected source commit is now `c865825`.
- cycle04 session 86301 completed: source `5c914204` collected as `ddeecbd`,
  all normal effect-item bodies through `00405400`. Ten verified SEH-only
  partials were retained with reviewed partial-only advancement. Fresh
  `/root/a04_round16` immediately started b0626, owning only
  `bsmain/iomanager.rs`; next forty entries `00456520` through `00c3fe80`.
- HUD source `ecf56da` passed 64 focused, 14,103 engine and 49 hygiene tests,
  strict lint and formatting. cycle03 session 41605 completed, collection
  `8aaf167`, with 14,177 combined engine tests and strict checks passing.
  Fresh `/root/a03_round18` immediately started its next forty HUD entries
  from `00778930`, compiler held; it is extending the compass marker paths.
  Movement source `598c064` passed 145 focused and 14,138 engine tests,
  strict lint, formatting and 49 hygiene checks. cycle01 completed with
  14,189 combined engine tests; collected commit is now `4cc1b3f` after
  shipping. Its seventeen verified SEH-only partials remain explicit.
  Fresh `/root/a01_round17` immediately started BSAudio task b0694.
  I/O collection is complete: source `a802ff8` became `75bd3b5`, with
  14,239 combined engine tests and strict checks passing. Fresh
  `/root/a04_round17` immediately continued b0626 from `00c3ffb0`.
  a03's intermediate focused pass succeeded (93 tests); HUD continues
  toward forty entries. I/O's remaining twenty-four bodies and callback
  mutation regressions are source-ready. Audio source `fae652b` passed all
  checks; cycle01 session 80637 collected `4c16784` with 14,279 combined
  engine tests passing. Fresh `/root/a01_round18` immediately began the
  remaining twenty-two audio bodies from `00ad7e90`. Root's two-helper
  checks completed: all 16,062 workspace tests (14,209 engine unit tests),
  strict lint, formatting and release/debug builds pass. I/O now owns the
  sole compiler checks all passed: 82 focused, 14,271 engine, strict lint,
  formatting, 49 hygiene checks and release build. Source `49b2fe1` collected
  through cycle04 session 29733 with 14,311 combined tests; collection is
  `1fc794c` after shipping rebase. Fresh `/root/a04_round18` immediately began
  BGSAutoWater b0046 (source-only). HUD source `9d9427b` passed 114 focused,
  14,227 engine, strict lint, formatting, 49 hygiene checks and release.
  cycle03 session 6725 is DONE: collection `faae60d` passes 14,379 combined
  engine tests. Fresh `/root/a03_round19` immediately began the remaining
  normal bodies from `0077f5e0`, compiler held. Audio now owns the sole
  compiler, followed by cycle01 and root's predicate follow-on. HUD's batch
  has forty normal bodies through `0077f5c0`, and audio
  twenty-two through `00ad87d0`. Audio also has seventeen passing native
  timer/empty-call cases. Review fixed HUD callback capture timing and a
  full-EAX predicate; I/O has additional pointer/flag reload regressions.
  I/O's eighteen and audio's nineteen remaining markers are SEH-only;
  reviewed partial-only lane advancement is allowed after validation.
- Root #138 is attached/published at `e7cf965` on
  `codex/phase2-script-action-parser`: `expression.rs`,
  `tesscriptrunmanager.rs` and topic `ENGINE_STATE.md` contain six helpers
  and eighteen new regressions. Thirty native cases pass (six parser, nine
  constructors, four entry and eleven evaluator), with raw evidence private
  under `codex-audit`. All 16,011 workspace tests, strict lint, formatting
  and release/debug builds pass. CExpression is 0x710 bytes,
  not a pointer wrapper. EvaluateLine `005e1550` now drafts BEGIN and action-list
  paths; non-BEGIN ordinary execution/parse stays explicit partial and panics.
  Independent PC review of all six helpers passed. #140 now publishes
  `004013e0` and `00500940` at `192b46e`, with forty-one native cases and
  all local checks passing. Both PRs are attached; #138 is already merged.
  The actual-bytecode normal-registry probe now passes both real BEGIN
  handlers and its expected action entry. The original item-payload probe
  ran but stopped at `008d0360`; the rewrite assertion has not run.
  Root now owns `fallout_ai/baseprocess.rs` and
  `fallout_shared/tesactorbasedata.rs` on
  `codex/phase2-reference-predicate`, rebased on merged #141 at `c56a2d1` (checkpoint `e89af6b`).
  Its source checkpoint is `3c3bedb`, with Rust checks still pending. It drafts that true
  predicate and the following false predicate `0047c850`, confirmed from
  the real reference vtable and load-buffer call chain. Twelve native cases
  pass; independent PC/vtable/test-wiring review found no mismatch. Three
  new Rust regressions have not run. Private
  `check-reference-probe.ps1` builds debug and reruns the item probe first;
  `check-reference-predicates.ps1` supplies the later full check chain.
  No whole-save claim.
- Water has 34 of 37 normal bodies drafted; `0049c930`, `0049db60` and
  `0049e410` are still being translated. The initial water agent is
  interrupted; `/root/a04_water_complete` owns and audits its uncommitted
  source, including a still-incomplete early-exit draft for `0049db60`.
  Do not collect or reset this slot before the full normal paths and checks
  finish. The replacement also restores the external predicate call in
  `0049c860`, which the first agent incorrectly inlined while editing its
  description. Fifteen native comparator/plane
  cases pass. Review corrected array callback reloads, the membership
  argument, fourth-corner Y selection and NaN classification. The apparent
  constructor-size and uninitialized-pair conflicts were decompiler
  artifacts resolved against PC instructions; no guessed substitute remains.
- a04's collected I/O management batch has forty bodies and fifty tests,
  with twelve SEH-only partials. All checks passed. Review
  corrected callback argument words, live queue/array/count reloads, signed
  construction loops and the incompatible Xbox layout. A suspect Xbox
  name for `005a8040` was removed in favor of a neutral PC accessor.
- Build 40 is current, with no confirmed next-play-build date. These leaf
  translations alone do not justify a play publication. Continue cycling
  until stopped; retain the user's 98% usage handoff rule.

### Stopped at the requested 98% usage limit, 2026-10-11 05:40 local

Translation is stopped for handoff. All three agents were interrupted;
no cargo/rustc process remains. Preserve their uncommitted files: do not
run `task.sh`, reset a slot, or cycle it before finishing its checks and
committing its owned file. On an authorized resume, restart the agents on
their existing task files with these continuation notes, then resume the
continuous collect/cycle/fresh-agent loop and ship passing rolling PRs.

1. Check attached rolling #134 at `a762d6f91915812c2dc8f31a71a54d57e51001fa`
   and persistence #135 at `86d19a0b2f8343a28afd9c334fdde36ed7b9aa69`.
   Both core/viewer CI checks were still running at 05:38; neither merged.
   Ship #134 through `ship.sh 134` once both current-head checks pass.
   Merge #135 only after both checks, using its exact head. #132 merged
   `4ba17e6` and #133 merged `c975484`, both verified after passing checks.
   Lead has later local documentation commits; the ship script preserves
   them during rebase. Do not push them over the in-flight PR unnecessarily.
2. a04, batch b0677: only `fallout_shared_magic/effectitem.rs` is dirty.
   All 34 remaining normal bodies are drafted through `00405400`.
   Focused tests 78/78, full engine 14,129/14,129, doctest, strict clippy and
   formatting passed. The hygiene test was interrupted without a result;
   rerun it, inspect the diff, then commit and cycle04 exactly once.
   No active session remains (70580/91380/28464 finished). Ten explicit
   SEH-only partials remain; partial-only advancement is reviewed/allowed.
   Immediately start a fresh agent on the new task after collection.
3. a03, batch b0236: only `fallout_interface/hudmainmenu.rs` is dirty.
   The remaining normal body of `00775a00` is drafted through `0077886d`,
   with reviewed callback reloads, float storage, signed comparison,
   constructor-return and NaN fixes plus regressions. Direct rustfmt,
   diff and name scans pass; cargo checks have NOT run. Validate, commit,
   cycle03 and immediately start the fresh continuation. SEH stays partial.
4. a01, batch b0155: only `fallout_ai/mobileobject.rs` is dirty. Nine normal
   bodies are drafted from `00935f60` through `00cbc510`; seventeen total
   SEH-only partials remain. Reviewed mode reloads and face/activator call
   argument fixes have regressions. Source-only checks pass; cargo checks
   have NOT run. Validate before commit/cycle01, and review the remaining
   queue before opting into partial-only advancement.
5. Root's managed checkout `%USERPROFILE%\.codex\worktrees\phase2-extra-data\wt-phase0`
   is clean on `codex/phase2-inventory-script-locals`, commit `86d19a0`.
   #135 contains five helpers/ten regressions; all 15,876 workspace tests,
   strict lint/fmt, release/debug builds and ten native cases passed.
   Applied retained stash `c11039c` must never be applied again. Next work
   is the real Script form/save-ID fixture and interpreter action-list
   path, starting `005e1160`/`005e13a0`/`005e1550`. Six generated native
   parser cases pass; private `codex-audit/inventory-script-next.md` has
   the details. a04 owns its required TLS guards; do not duplicate them.

Build 40 remains the current play copy. Item payload evidence (10,201
count/health entries, eighteen multi-stack entries, three full-record decode
failures excluded) does not establish a whole-save round trip or playable
integration. No confirmed next-build date or feature list. Prior automatic
approval review blocked idle p3 target cleanup with only "blocked by policy";
no deletion was performed for that rejected action. Do not retry/bypass it.

Latest, 05:35 local: root's five script-loading helpers are committed and
pushed as `86d19a0` on `codex/phase2-inventory-script-locals`; attached
PR #135 is open with fresh core/viewer CI. The checkout is clean. All local
checks passed: 1,320 focused tests, 15,876 workspace tests including doctests
(14,023 engine unit tests), strict lint, formatting, release and debug builds.
Session `33662` is complete. Native evidence and the next parser's six cases
are recorded in `ENGINE_STATE.md` and private `inventory-script-next.md`.

`/root/a04_round15` now owns the compiler for its frozen 34-body batch.
After a clean commit, cycle04 once with reviewed `ADVANCE_PARTIAL_ONLY=1`
if only its ten verified SEH-only partials remain; immediately start a fresh
worker, compiler-held. Then grant `/root/a03_round17` its remaining normal
HUD body, followed by `/root/a01_round16`'s nine remaining normal movement
bodies. Those two drafts are uncompiled and restricted to their own unit
files. a01 now has seventeen total SEH-only partials; the earlier estimate
of six remaining normal bodies was incorrect, superseded by the ledger.
Its callback mode reloads and pending argument corrections are drafted
with regressions. The source-only formatting/hygiene scans pass.

Rolling #134 remains open at `a762d6f`, both CI checks still running.
Ship with `ship.sh 134` when both current-head checks pass. #135 also needs
both checks before merging. Usage is 97%; stop and hand off at 98%.
No new play copy is warranted by these unconnected engine dependencies.
Build 40 remains current, with whole-save/world integration still pending.

Latest, 05:26 local: #132 merged `4ba17e6` and rolling #133 merged
`c975484`, each after core and viewer CI passed. `ship.sh 133` completed
and created attached #134 at `a762d6f`, covering actor dialogue and HUD
target information. Its fresh CI must pass before shipping. Movement source
`d3269c3` was collected through cycle01; the rebased collection is
`c95399b`, with 14,126 combined engine tests, doctest, strict lint and
formatting passing. HUD collection is now `c9a17b4` after that rebase.
Fresh `/root/a01_round16` immediately continued at `00935f60`; sixteen
earlier SEH-only partials remain explicit and six normal bodies remain.

Root owns the heavy compiler, session `33662`, running the private
`codex-audit/check-inventory-locals.ps1`. Focused tests pass; workspace,
lint, formatting and release/debug builds are still in progress. The root
checkout is on `codex/phase2-inventory-script-locals`, based on #132's
merge, with only `tesscript.rs`, `tesscriptrunmanager.rs` and
`ENGINE_STATE.md` changed. Scoped stash `c11039c` is applied and retained;
never apply it again. a04's frozen 34-body batch gets the compiler next,
then a03's remaining HUD body. Both are drafted and uncompiled. The HUD
review fixes cover callback reloads, float storage, signed target types,
constructor return pointers and unordered comparisons.

Six further native cases for the next parser `005e13a0` pass, checking
signed opcode/length words, reference-prefix behavior and RET24; this
parser is still untranslated. Private `inventory-script-next.md` records
the evidence. Usage 94%; stop and hand off at 98% as requested. Build 40
remains current, without a confirmed next-build date or feature list.

Latest, 05:19 local: HUD source `6c4df88` was collected as `0eb6b3c`
through cycle03, passing 14,100 combined engine tests, doctest, strict lint
and formatting. Fresh `/root/a03_round17` immediately continued and has
drafted the remaining normal `00775a00` paths; review is correcting late
HUD reloads and float storage boundaries. No cargo grant for that worker.
`/root/a01_round15` owns the compiler for twenty movement/dialogue bodies:
lint and formatting pass; one fixture correction is undergoing tests.
After commit: cycle01 exactly once, immediately fresh worker, then root
compiler for `codex-audit/check-inventory-locals.ps1`, followed by a04/a03.
Root has five helpers and ten Rust regressions, still uncompiled; all ten
native constructor/manager cases pass. Both #132 (`6513c26`) and #133
(`f264324`) have passing core CI and pending viewer CI. Ship when both
current-head checks pass. Build 40 remains current with no confirmed ETA.

Latest: rolling #131 passed both CI checks and merged `7cda9bf` at 04:57
local. `ship.sh 131` completed and created attached #133 at `f264324`,
covering movement persistence and effect-item routines. Its CI is fresh.
Effect collection is now `6cdf2b2` (source `7cc22ff`), with 14,095 combined
engine tests plus doctest, strict lint and formatting passing. Fresh
`/root/a04_round15` immediately started the remaining normal effect-item
entries at `00404a80`; its 34-body draft is ready and compiler-held.

a03 owns the compiler for its large HUD partial, followed by cycle03/fresh
worker, a01's twenty-body dialogue/movement batch and cycle01, root's
persistence checks, then a04. Root's script follow-on now contains five
dependencies and nine prepared tests in `tesscript.rs` and
`tesscriptrunmanager.rs`, plus `ENGINE_STATE.md`. Six native constructor
vectors pass; independent source review found no mismatch. Rust checks
are pending. Interpreter `005e1160` and action-list semantics remain open.
PR #132 core passes, viewer CI is pending. Build 40 remains current.

Latest, 04:49 local: persistence follow-on #132 is published and attached at
`6513c26`, with three dependency helpers and four regressions. All 15,781
workspace tests (13,928 engine), strict lint, formatting and release/debug
builds pass. Both probes now pass using the normal generated registry:
the synthetic two-stack payload and 10,201 original count/health-only item
payloads from eight saves, including eighteen multi-stack entries. Three
containing records that fail full decoding remain excluded. Generated form,
map, mode and partial allocator startup are explicit boundaries; this is
still not whole-save or playable inventory acceptance.

Root's clean published map branch has a new follow-on branch,
`codex/phase2-inventory-script-locals`: constructor `005a8b80` and two tests
are drafted in `tesscript.rs`, uncompiled. Three guarded native vectors pass.
Do not claim script extras work: their actual script forms and save-ID maps
are not yet supplied by the broader exploratory fixture. a04 owns the
compiler; a03, a01, then root follow their respective collect/cycle steps.
Rolling #131 core passes and viewer CI remains pending at `d6dcc8e`.
Build 40 remains current, with no confirmed next-build date or feature list.
Usage 84%; retain the 98% stop/handoff rule.

Latest, 04:33 local: movement collection `757af70` passes 14,051 combined
engine tests, doctest, strict lint and formatting. Fresh `/root/a01_round15`
immediately starts its remaining normal bodies at `009339a0`; ten SEH-only
partials remain explicit. Root now owns the compiler, session `57033`, for
the persistence follow-on; a04 then a03 follow. a03's next HUD body remains
explicitly partial beyond `00776731`; reviewed argument/global reload fixes
are in its draft. a04's forty-body effect-item batch is ready, uncompiled.

Reader #130 merged `bad2f96` after both CI checks. Rolling #131 is attached,
head `d6dcc8e`, rebased onto that merge with only generated-ledger conflicts;
fresh core/viewer CI is required. Later movement `757af70` remains local.
Root branch `codex/phase2-inventory-form-map` is based on merged #130;
stash `a4d2a67` is applied and retained, never apply it again. Its three
helpers/four tests cover form-map allocation and a shared buffer setter.
The draft count/health probe reads eight real saves and rewrites all 10,201
selected item payloads byte-identically, including eighteen multi-stack
entries; three full-record decode failures are excluded. This is not a FOS
round trip. The broader inventory probe reaches missing `005a8b80` in script
loading; investigate next after this batch. Final generated-registry probes
remain pending after the build. Build 40 remains current. Usage 78%.

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
