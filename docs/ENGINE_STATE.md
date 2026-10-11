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

Payload conversion, reference creation/ownership, arena lifetime, viewer state
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

## Health payload follow-on

`ExtraHealth` (Xbox PDB) is a 0x10-byte object with `fHealth` at +0x0c;
PC constructor `00431d10` confirms the layout. The health adapter delegates
to `004186f0` / `00419970`. An absent node reads the native -1.0 default,
whereas setting -1.0 creates or updates a node; presence remains distinct
from the float value. Zero and negative values are not clamped or removed.

Six generated native constructor vectors preserve signed zero, ordinary
values, padding and the following guard word; signaling NaNs become quiet
through FLD/FSTP. A private Rust registry probe reproduced the existing
constructor mismatch (7f800001 remained signaling instead of 7fc00001).
The constructor now uses the existing x87 bit conversion helper. Regression
tests cover the native vectors and list creation/update, presence, stable
node identity, count preservation and a read that leaves its source bytes
untouched. All 74 health-related tests pass, including these two new
regressions. The installed-executable registry probe now produces the
native quiet-NaN result `7fc00001` for constructor input `7f800001`.

A preliminary real-registry getter/setter probe passes ordinary values,
explicit defaults and signaling NaN handling; the setter already quieted
its argument before reaching the constructor. The early Doc Mitchell save
contains 20 health payloads. A read-only probe adapted all 20 health and
14 count payloads across 3,319 extra lists from 3,365 reference/actor forms.
Values and presence read back correctly, and removing count nodes retained
the health nodes. The source save SHA-256 remained unchanged. No
game-function doubles or save writes were used. On the merged list-adapter
base, all 15,315 core tests (13,462 engine), formatting, workspace clippy
with warnings denied and the release build pass. This also validates the
unchanged count source in the health follow-on. CI still gates merging.
No world/viewer consumer or save writer is added.

## Existing-reference boundary, 2026-10-11

`engine::reference::Reference` adapts an initialized `TESObjectREFR`
(Xbox PDB) in an explicitly supplied engine. Its `extra_data` accessor
dispatches to PC `005d43c0`, which returns the embedded list at +0x44.
Copies alias the same reference. Null is rejected; address validity,
object type and lifetime remain the caller's responsibility. No public
constructor or automatic destructor is added.

The PC reference constructor `0055a2f0` reaches form-header, map, array,
allocator and counter dependencies. The added translations retain their
individual PC addresses. Native generated vectors check five header
cases, two zero-capacity arrays and three allocator-header cases. Generated
tests check allocation requests separately from platform backing size:
requests of 4 and 13 bytes have 8- and 16-byte backing blocks. Callback
regressions verify reloading the form data-handler pointer and map bucket
count after calls that can change them. Compiler exception cleanup remains
explicitly partial in the affected form/reference constructors.

The private installed-executable probe initialized the allocator sequence
from PC `00866690` through `00aa2020`, then ran reference construction,
embedded-list access, count/health updates and count removal through the
real registry. No game-function doubles or game-file writes were used.
This covers that allocator sequence only: the later system-utility startup
has not been reproduced. `Engine::with_exe` maps executable data; it does
not perform that startup by itself.

On the merged #120 base, all 15,587 core tests pass (13,734 engine),
including the new reference alias/isolation regression. Workspace clippy
with warnings denied, formatting, release and debug builds also pass.
The updated installed-executable probe uses the new handle and passes
construction, embedded-list access, count/health changes and count removal
while preserving health. An independent source review found no concrete
issues in the adapter; it did not independently run the private probe.
No world/viewer integration or save round trip is established.

Reference PR #122 passed both core and viewer CI after rebasing onto #121,
and merged as `6bea9a1` on 2026-10-11. No play build was published for this
boundary because the viewer does not consume it yet.

## Buffered extra-data writing, 2026-10-11

The follow-on translates 18 constructor, accessor and writer dependencies
of the existing `00426a30` extra-data writer. `BGSSaveGameBuffer` (Xbox PDB)
has PC buffer/capacity/position/overhead fields at +4/+8/+0xc/+0x10.
`BGSSaveFormBuffer` (Xbox PDB) adds its nine-byte packed header at +0x14 and
form pointer at +0x20; construction clears nine header bytes and preserves
the three alignment bytes before the pointer. Six generated native
constructor vectors confirm the stores, vtables, return pointer and
surrounding guards. Exception cleanup
in `008659c0` remains explicitly partial.

Positioned writing (`00865ce0`) grows by 0x10000, appends the native '|'
separator unless suppressed, and updates the position and debug overhead.
It uses unsigned capacity comparisons; insertion (`00866140`) uses signed
comparisons and retains its original end across callbacks. Tests exercise
field/global changes across callbacks and wrapping arithmetic. Variable
counts (`00865f20`/`00865ff0`) patch a placeholder using one, two or four
tagged bytes. The checked move (`00ec7ce3`) uses the existing CRT memmove
boundary; three native cases pass for zero length/null pointers and overlap
in both directions. Error paths retain native TLS/error-handler calls;
these callees are unresolved and their unit tests use doubles.

The PC change-flag table at `01183dc0` and `01183dc4` uses mask `0x400` for
count and health, not the generic mask used by older synthetic writer
fixtures. A new generated regression prepares both payloads through the
real registry and exercises the buffered writer with matching and filtered
change flags. The installed-executable probe passes with a generated
save-mode receiver and the previously tested allocator startup fragment.
It writes count 5 and health 42.5 through the real registry and native
change-flag table, then decodes the resulting 14 bytes as exactly those
two extras with no unread bytes. No game-function doubles or game-file
writes are used. This checks a payload, not a complete FOS file.

On the merged #123 base, all 15,641 core tests pass (13,788 engine),
including the generated writer regression. Workspace clippy with warnings
denied, formatting, release and debug build checks pass.
No FOS file writing, full save-manager startup, save round trip or viewer
integration is established.

Next action: publish the bounded writer batch,
then trace per-stack state ownership before adding a world consumer.

## Inventory payload dependency chain, 2026-10-11

`ItemChange` (Xbox PDB) is a 0x0c-byte object: the extra-list chain at +0,
signed item-count delta at +4, and item form at +8. Its PC buffered writer
`004bed60` writes the form ID, signed delta, number of non-null stacks and
each stack's extra list separately. These counts must not be collapsed:
the entry delta is against its base record, whereas an `ExtraCount` is the
signed 16-bit count in one stack. The current world importer merges item
counts and overwrites per-item weapon condition as it visits the stacks;
this follow-on does not change that importer.

Seventeen dependencies are implemented for this writer: form-ID encoding,
the forward/reverse save-ID maps, map construction and virtual allocation,
lookup/insertion/release, and the list-node constructor needed to retain
multiple stacks, and the shared word-at-four getter. `BGSSaveLoadFormIDMap`
(Xbox PDB) has maps at +0/+0x10 and
the next index at +0x20, size 0x24, checked against the PC constructors.
Constructor `00846b60` still records omitted compiler exception cleanup.

Ten generated cases match the native executable: three created-form-ID
decodes with unchanged surrounding bytes, four key-equality AL results,
and three word-at-four results with unchanged surrounding bytes.
The native cases use the same executable hash and CPU configuration cited
above. Static peer review of all seventeen bodies found no concrete mismatch;
the list-node constructor's review also checked the aliased-source regression
against its PC load-before-store order and one-word stack argument.
Generated tests cover callback changes, collisions, index reuse and the
real registry's encode/decode chain. The first 16-helper batch passed all
15,690 core tests (13,837 engine), strict workspace lint, formatting and
release/debug builds. The installed-executable probe then reached the
untranslated `00726070` getter while advancing to the second stack. After
adding that dependency, the real-registry probe passes: its 41-byte payload
decodes to entry delta -2, count 2 / health 42.5 in one stack, and count 7 /
health 17.5 in the other. The save-ID map resolves the encoded item back to
its original generated form ID, with no unread payload bytes. Generated form
header, manager/mode receivers and partial allocator startup are explicit;
no game-function doubles or game-file writes are used. Final checks with all
seventeen dependencies pass: 15,691 core tests (13,838 engine), strict
workspace lint, formatting, and release/debug builds.
No FOS file, world consumer or full save round trip is implemented.

Next action: validate native buffer loading before connecting the state
boundary to the world.

## Native inventory buffer reader, 2026-10-11

Eleven reader dependencies are implemented in `bgsloadgamebuffer.rs` and
`bgsloadformbuffer.rs`: buffer construction/attachment, raw reads, variable
counts, form-ID decoding, and form/reference/actor access. The PC raw reader
does not add a bounds check: it skips only when the size field is zero and
reloads its buffer and cursor after callbacks. Tag three leaves the cursor
unchanged; the other tags decode one, two or four bytes and consume a
separator. The derived constructor retains its three header-padding bytes;
its compiler exception cleanup remains explicitly partial.

Peer review against PC disassembly found no mismatch. Ten generated native
cases pass on the same executable/CPU configuration above: two guarded
constructors, four flag-predicate AL results, and four variable-count tags
with cursor checks. Eight focused regressions and all 15,771 workspace
tests (13,918 engine) pass on merged #128, as do strict workspace lint,
formatting, release and debug builds.

The installed-executable probe writes and loads two independent stacks:
count 2 / health 42.5 and count 7 / health 17.5, with signed entry delta -2.
The translated buffered reader consumes all 41 bytes. This uses real
registered translations, generated receivers and partial allocator startup,
without game-function doubles or game-file writes.

The fuller ItemChange load/write probe stops before loading: inserting its
generated form into the real form map reaches untranslated allocator
`0043a010`. Its byte-identical rewrite assertion has not run. Static review
of `004bee00` confirms the existing normal loader path; compiler exception
cleanup remains omitted there. Neither probe establishes a complete FOS
file round trip or a world/viewer consumer.

Next action: translate the form-map allocation dependency and rerun the
full ItemChange load/write probe before connecting world state.
