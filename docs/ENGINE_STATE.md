# Engine-owned state: Phase 2

Phase 2's gate remains a real `.fos` round trip, compared change form by
change form. The first bounded step is `engine::extra_data::ExtraList`:
a handle to a list in an explicitly supplied `Engine`. It is not yet used
by `world` or the viewer. Current save-reader evidence and limitations are
in [FOS_SAVES.md](FOS_SAVES.md); overall priority stays in
[MILESTONES.md](MILESTONES.md) and [HANDOFF.md](HANDOFF.md).

## List boundary

The handle delegates construction, count, bitmap lookup, cached lookup,
insertion, detach and clearing to the existing PC translations
`0040f740`, `0040fe20`, `0040fe80`, `00410220`, `0040ff60`, `00410020` and
`0040fae0`. No new list algorithm is introduced. Insertion retains the
game's special head-insertion types and appends other types. Detach looks
up the predecessor by type, so callers must supply the first matching
node. Duplicate types retain the game's shared-bitmap caveat.

Handles refer to one engine's arena. Copying a handle does not copy its
objects; dropping it does not destroy them. Node construction and lifetime
remain explicit. The adapter exposes detach and forget, not virtual
destruction: subclass destructor dependency closure is not established.

`Engine` currently contains `Rc<RefCell<...>>` test-double state and is not
`Send`, `Sync` or `Clone`. `GameState` has different ownership requirements.
The explicit engine parameter lets this boundary be checked before
changing world/viewer ownership. It does not resolve that integration.

## Dependency evidence, 2026-10-10

Private Ghidra GET queries on port 8090 identified four missing helpers:

| PC address | Behavior | Evidence |
| --- | --- | --- |
| `0040ec80` | Base extra constructor: vtable, type byte, null next | PC disassembly, native generated vectors |
| `004f1540` | Read the type byte, returning AL | PC disassembly, native generated vectors |
| `00403550` | Write the next pointer at +8 | PC disassembly, native generated vectors |
| `0040fc90` | Call the GetCurrentThreadId import at `00fdf1d0` | PC decompilation and disassembly |

`BSExtraData` and its fields are Xbox PDB names. The prototype layout is
0x0c bytes, with the type at +4 and the next pointer at +8; these are checked
against the PC instructions. The engine map does not assign these helpers
a source-unit name, so `extra_data_support` is a dependency module rather
than an asserted original source unit. No executable, symbol database or
raw decompiler output is committed.

Eight generated header cases matched the native executable, including
constructor argument truncation, untouched padding and next-pointer
stores. The executable is FalloutNV.exe 1.4.0.525, unpacked SHA-256
`19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f`.
Private probe inputs and outputs remain in
`%USERPROFILE%/nv-re/work/phase0/codex-audit`.

Integration tests use generated globals and real platform imports, with
no doubles for game functions. They exercise insertion order, the bitmap,
cache changes between lists, removal of head and tail, cleared membership
with a retained old chain, and a real `ExtraNorthRotation` constructor.
The installed-executable probe also passes through the real registry:
construct a list, construct `ExtraNorthRotation`, add, count, find, detach
without deletion, and check empty membership. It maps the user's installed
executable data with `Engine::with_exe`; no game-function doubles or game
file writes are used. This proves that bounded dependency chain, not a
save round trip or original-game gameplay comparison. Full core checks
passed after rebasing onto merged PR #113: 15,154 tests across suites
(13,301 engine tests), clippy with warnings denied, formatting and the
release build. The four ledger-tool tests also pass. A separate read-only
source review found no actionable issues; that review did not independently
verify the private probes. CI still gates merging the PR.

## Open boundary

Payload conversion, reference/form adapters, arena lifetime, viewer state
ownership and save serialization are not implemented by this step. The
save reader's `Extra::Other` representation discards payloads, so successful
decoding cannot prove preservation of unsupported data. Do not replace the
existing world import or claim a save round trip from these list tests.

A separate private probe identifies a concrete next payload dependency.
`ExtraDataList::SetCount` / `GetCount` (Xbox PDB, PC `00419ad0` / `00418770`)
successfully create and update counts 2, 5 and -2 through the real registry.
Setting 0 reaches the open destructor helper `0040ecb0`; the PC body resets
the base vtable. The probe stops there, so removal and later cases are not
validated. PC code treats counts 0/1 as removing the count extra, with the
getter returning 1 for an absent node. No count adapter has been added.

Next action: close the count destruction dependency and validate a
count-payload adapter against a real save.
