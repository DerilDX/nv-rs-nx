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

## Count payload dependency

`ExtraList::item_count` and `set_item_count` delegate to
`ExtraDataList::GetCount` / `SetCount` (Xbox PDB, PC `00418770` / `00419ad0`).
This is the signed 16-bit `ExtraCount::iCount` at +0x0c, in a 0x10-byte
object; Xbox PDB layout agrees with the PC constructor `00431c20`.
It is distinct from the number of extra nodes and from an inventory
entry's signed 32-bit change against its base record.

The PC setter removes an existing type 0x24 node for counts 0 and 1;
the getter then returns 1. Other values create or update the payload.
The required base destructor `0040ecb0` only resets the vtable to
`01014210`; it does not change the type, link or payload. A generated
16-byte sentinel case matches native output, including untouched bytes.
The native executable identity and private evidence location are as above.

The installed-executable real-registry probe now passes counts 2, 5, -2,
0, 1, 32767 and 1, including removal and reallocation. No game-function
doubles or installation writes are used. The general subclass destruction
boundary remains open; this establishes only the count dependency chain.
The focused ExtraData filter passed 172 tests, including the new destructor
and count lifecycle checks. The full core suite then passed 15,156 tests
on the #113 base, including explicit allocation-release assertions. Count
source is unchanged after rebasing onto #114 plus the list-adapter PR.
The repeat compile exhausted host memory alongside other engine compilers;
it failed before tests. Validation is now serialized. On the merged
list-adapter base, the health follow-on with unchanged count source passed
all 15,315 core tests, clippy, formatting and the release build. Fresh CI
still gates this count PR. The generated ledger is refreshed by the
rolling integration batch after merge, avoiding competing snapshots.

A read-only probe of the user's early Doc Mitchell autosave decoded 3,365
reference/actor change forms and visited 3,319 extra lists. Its 14 count
payloads (values 2 through 40) all adapted and read back correctly through
the real registry, then removed successfully. Lists without a count read
as 1. The save SHA-256 remained unchanged. This checks count payloads only;
other extras are not imported by the probe and no save bytes are written.

Next action: merge after CI, then continue the health payload adapter.
World ownership and full save
round trips remain open.
