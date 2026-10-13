# The engine crate: translating FalloutNV.exe function by function

`crates/engine` holds FalloutNV.exe 1.4.0.525 translated into Rust, one
function at a time, on a model of the game's own memory. Decision and
alternatives: [ADR-0006](adr/0006-engine-crate-memory-model.md). The ledger
([LEDGER.md](LEDGER.md)) counts the translations; the engine map
([research/engine-map](../research/engine-map/README.md)) says which unit
each function belongs to.

## The model in one page

- **Memory** (`Mem`): the game's 32-bit address space, paged and sparse.
  Objects sit at their real field offsets; a pointer is the game's address
  ([`Ptr<T>`]). Reading memory nobody provided panics with the address.
- **The exe's data**: `Engine::with_exe(path)` maps `.rdata`, `.data`,
  `.tls` and `CONST` of the player's installed FalloutNV.exe (checked
  against 1.4.0.525) at their addresses. Constants, strings, vtables and
  initial globals are therefore the game's own; nothing from the exe is in
  the repository. Tests run without the exe and set what they need.
- **Functions**: every translation is a typed Rust function and is also
  registered under its exe address in a uniform form (argument words,
  `this` first; see `abi.rs`). `e.call(addr, &args![...])` calls any
  function by address, `e.vcall(this, slot, &args![...])` through the
  object's vtable. A call to a function nobody has translated panics
  with `open function <addr>`, so translations can be written in any
  order and connect as the coverage grows.
- **Classes**: `layout!` declares a class marker with its PC size and the
  fields used so far (`e.get(p, Class::field)`, `e.set(...)`).
- **Platform**: the allocator (`MemoryManager::Allocate`/`Deallocate`) is
  `Mem`'s heap; threads are one thread (`e.tls()` is the TLS block).
  Functions replaced this way are marked
  `// Platform: replaces <addr> (...)`; the ledger counts them as
  `platform`.

## One unit, one file, one owner

Translations live in `crates/engine/src/units/<subsystem>/<unit>.rs`, one
file per source unit of the Xbox 360 prototype (Xbox PDB, ADR-0002): for
example `fallout/ai/processlists.cpp` is `units/fallout_ai/processlists.rs`.
The lead creates the files (`ledger scaffold <unit or subsystem>`); the
`mod.rs` files are generated (`ledger units`) and never edited by hand.
**A translator edits only the unit files assigned to them.**

```sh
L="cargo run -q --release --manifest-path scripts/ledger/Cargo.toml --"
$L queue                          # open functions per unit, largest first
$L queue "fallout/ai/processlists.cpp"   # one unit's work list
$L scaffold "fallout/ai/processlists.cpp" # create its file (lead)
```

## How to translate a function

1. **Read it.** Decompile and disassemble it in the named Ghidra project
   (17,600 functions carry Xbox PDB names, so callees are often named).
   Where the C is ambiguous (stack slots, return registers, x87), the
   disassembly decides. Class layouts (names, offsets): the Xbox PDB
   (`pdbdump type <pdb> <Class>`), checked against the PC code's own
   offsets, since the 2010 Xbox build can differ.
2. **Write it** in the unit file:

   ```rust
   // Translated from 00d87410 (decompiled, FalloutNV.exe 1.4.0.525)
   /// `hkCachedHashMap<hkStringMapOperations,hkContainerHeapAllocator>::getIterator`
   /// (Xbox PDB): index of the first occupied slot, or `hashMod + 1`.
   pub fn get_iterator(e: &mut Engine, this: Ptr<HkCachedHashMap>) -> i32 {
       let hash_mod = e.get(this, HkCachedHashMap::m_hashMod);
       ...
   }
   ```

   - The marker comment goes directly above the function, with the
     function's entry address. The doc comment names the Xbox PDB name
     (marked `(Xbox PDB)`) and says what the function does.
   - Name: snake_case of the class and method, `tes_form_get_form_id`;
     overloads keep the map's `_ovN` (`garbage_collector_add_ov2`);
     unnamed functions are `fn_<addr>`. Check the body before trusting a
     map name: identical-code folding can put another method's name on a
     shared body (a destructor named `NiTArray<..>::SetSize`). If the name
     is wrong, use `fn_<addr>` and say what the body is.
   - Parameters in declaration order, `this` first. Types: `u32`, `i32`,
     `u16`, `i16`, `u8`, `i8`, `bool`, `f32`, `f64`, `u64`, `Ptr<T>`.
     A struct returned by value is an explicit pointer parameter.
   - Register it in the file's `funcs()`:
     `entry!(0x00d87410, get_iterator(Ptr<HkCachedHashMap>) -> i32)`.
3. **Calls.**
   - Same unit: call the Rust function directly.
   - Anything else: `e.call(0x00abcdef, &args![a, b])` (result:
     `.u32()`, `.f32()`, `.bool()`, `.ptr::<T>()`, or
     `e.call_as::<T>(...)`). Never re-implement another unit's function
     inline, and never guess what a callee does.
   - Virtual: `e.vcall(this.addr(), 0x38, &args![...])`, with the byte
     offset of the slot, exactly as the code indexes the vtable.
   - CRT functions (`memcpy`, `strlen`, `sprintf`, ...) are called by
     address like any other; the runtime library is translated or
     replaced separately.
   - Imported Windows functions (`call dword ptr [0x00fdf0e4]`) are called
     by the address of their import slot: `e.call(0x00fdf0e4, ..)`.
     `units/platform.rs` provides the ones with a Rust stand-in
     (interlocked operations, critical sections, clocks); report others
     you need.
   - A value the code passes in x87 `ST0` (compiler helpers such as
     `_ftol2`, `00ec62c0`) is a leading `f64` argument (two words).
   - A local the game keeps on its stack and passes by address (a scope
     guard, a `NiPointer` temporary, an out parameter):
     `e.with_stack(size, |e, p| ...)`.
4. **Data.**
   - Fields of a class this unit owns (the class's own `.cpp` is this
     unit): declare them with `layout!` in this file, Xbox PDB names,
     PC offsets.
   - Containers, strings and reference-counted bases (`NiTArray`,
     `BSSimpleArray`, `BSSimpleList`, `BSStringT`, `NiFixedString`,
     `NiRefObject`) are in `crate::types`; use them, never redeclare them.
     Template instances that a unit contains (an `NiTArray<T>::SetSize`
     emitted in your `.cpp`) are your functions; their layout is still the
     shared one.
   - Fields of other classes: use their `layout!` if it exists
     (`crate::units::<sub>::<unit>::Class`); otherwise read at the offset
     (`e.mem.u32(p.addr() + 0x20)`) with a comment naming the class and
     field (`// TESForm::iFormID (Xbox PDB) +0x0C`). Do not edit another
     unit's file.
   - Globals and constants: read them from memory at their address
     (`e.global::<f32>(0x01012e10)`), as the game does; they come from the
     exe at run time. String literals are passed as their address.
   - Floats: compute in `f64` and round to `f32` at each `float` store,
     the closest match to x87 code (results can differ only in the last bit,
     in double-rounding cases). SSE code (`MOVSS`, `ADDSS`) computes in
     `f32`.
5. **Not translated**: C++ exception unwinding (`__CxxFrameHandler`
   states) and SEH frames are left out (say so if a function relies on
   them); the compiler-generated initializers and `atexit` destructors
   in the tail of `.text` are `library`.
6. **Test it** in the file's `#[cfg(test)] mod tests`: build the objects
   it reads (`e.new_object::<T>()`, `e.map`, `e.set_global`,
   `e.put_vtable`), stand in for callees with test doubles
   (`e.register(0x00abcdef, |e, a| ...)`, or `e.register_double` for a
   double that keeps state), and check results and calls
   (`e.call_log = Some(vec![])`). Inputs come from the decompiled
   logic, one case per branch that matters. Then:

   ```sh
   cargo test -p engine units::<subsystem>::<unit>
   cargo clippy -p engine --all-targets
   cargo fmt --all
   ```

7. **Report** per function: translated / partial / skipped, and why.
   Partial translations keep the marker only if the missing part is named
   in a comment (`// open: ...`). Also add `// Translation incomplete:
   <address> - <missing behavior>` on one line. The ledger keeps such
   addresses `partial` in the engine queue instead of `done`; remove that
   incomplete marker only when the documented gap is closed and tested.
   A panic-only placeholder has no translated branch: leave it skipped,
   without a translation marker or a new registry entry.

## Checking against the real exe

Pure functions can be compared with the real CPU through `nv-call`
(`research/nv-oracle`): the same inputs, the game's function and the
translation, bit for bit. The ledger will gain a column for that check.
Stateful code is checked by the save round trip (Phase 2) and the
acceptance routes once it is wired.

## The script interpreter (opt-in run on real scripts)

The interpreter's trunk (`Script::Run` → `ScriptRunManager::Run` →
`ScriptRunner::Run` → `EvaluateLine` → `CExpression::Eval`) is translated;
`INTERPRETER_TRUNK` in `units/fallout_shared/tesscriptrunmanager.rs` lists
it and fails when a member is not registered. CI runs it on generated
compiled scripts only. With the player's installation, an ignored test runs
every quest script of `FalloutNV.esm` once through it on generated forms
and prints which untranslated functions stop them:

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p engine --test real_scda -- --ignored --nocapture
```

The executable is found next to `Data` (`exe::installed_exe`) and refused
unless it is 1.4.0.525 (`exe::verify`), the check a game start-up can reuse.

## In the running game

The `host` crate starts the engine for the running game. The viewer calls
`host::Host::start(<Data>)` before loading anything: it opens the
`FalloutNV.exe` next to `Data` (read-only), refuses every build but
1.4.0.525 with the file and the reason, maps the data sections and runs the
allocator start-up the probes run (`00401000`, `00aa2230`, `00aa21d0`,
holder at `011f6118`, `00aa2020`); the rest of `WinMain`'s start-up is not
reproduced. The startup line reads `Game code: <exe> (1.4.0.525), <n>
translated functions, allocator started.` The `Host` is a Bevy non-send
resource (main thread only; the engine is not `Send`).

Nothing runs through it yet (script track step S4): the game's scripts
still run through `crates/script`.

`host::script_form` (step S5) makes a script record into a `Script` form
in the game's memory and carries a script instance's variables between
the world (`script::interp::Locals`, by name) and the game's
`ScriptLocals` (by `SLSD` id, through `ScriptLocals::SetVariable` and
`GetVariable`; references as form ids in the low word). `Script::Load`
itself is bridged: its file reading (`TESFile`, `004726f0`, `00401660`,
`ScriptVariable::Load` `005e2a20`) is not translated, so the record's
bytes are put where it puts them; referenced forms stay unresolved.
With the installation, an ignored test takes all 322 quest scripts of
`FalloutNV.esm` (3,642 variables, 178 references) through the game's
locals and back unchanged:

```sh
NV_DATA="<...>/Fallout New Vegas/Data" cargo test -p host --release --test real_quest_locals -- --ignored --nocapture
```

`host::global_form` makes a global (`CreateFormOfType(6)`, the `TESGlobal`
constructor `005a6320`, then `FNAM`/`FLTV` where `TESGlobal::Load` puts
them) and carries its value to and from the float at +0x24 that the
interpreter reads (`00526ac0`); the same ignored test checks every global
of `FalloutNV.esm`.

Open: the quest half (a quest's locals at `TESQuest` +0x5c, read by
`0059e300` and `0060d4d0`). The `TESQuest` constructor (`0060c4b0`) flags
a change through `TESForm::ForceChange` and `BGSSaveLoadGame::AddChange`
(`0084a690`), which needs the save/load game singleton (`011ddf38`,
made by `BGSSaveLoadGame::Create` `00847480` in the `TES` constructor).
`Create` stops at `006b3eb0`; measured by answering each untranslated
callee with 0 (so later paths are approximate), it reaches eight more
(`007d6bd0`, `00845550`, `00852a70`, `008651d0`, `00822600`, `0084b5c0`,
`008433d0`, `008535e0`), and the quest constructor then `009a4250`.
(`008c71b0`, the one-word store `ForceChange` builds its flags with, is
translated in `fallout_misc/bgschangeflags.rs`.) S6 (quest scripts
through the game's interpreter in the live game) needs the quest forms
and the other referenced forms (see the script track plan).

### Static initializers (not run yet)

The game constructs many globals before `WinMain`: `__cinit` (`00eccd14`)
runs `_initterm_e(010113c4, 010113e8)` (8 C initializers) and
`_initterm(00fdf4a4, 010113c0)` (17,728 non-null C++ dynamic initializers,
each usually followed by its `atexit` destructor; the engine map counts
16,315 `init` and 5,020 `atexit` functions as "compiler: static
init/exit"). `host::Host::start` does not run them, so those globals read
as zero in the game's memory: a `Setting` such as `fGunShellLifetime`
(`011ce7c0`) has neither its name nor its default. Measured on 2026-10-13
(callees of every entry, Ghidra):

| Entries | Shape | Data |
| ---: | --- | --- |
| 7,254 | `CALL <one-shot registration>; MOV [global], EAX`: 25 NiAnimation functions (`00a26e20`, `00a299f0`, ...) included by about 329 units each; the first call fills function-pointer tables, later calls return 0 | game (Gamebryo dispatch tables) |
| 4,409 | no calls: constants computed with x87 (`FLD`/`FMUL`/`FSTP`) into globals | game |
| 4,386 | a `Setting` constructor (`0040e0b0`, `0040c150`, `0044f440`, `0045cf30`, `0045cfe0`, `004a4a40`, `0045d060`, `0044fae0`) with the global, its name and default, which also adds it to its collection (`00404a70` and others), then `atexit` | game (settings) |
| 1,066 | a math helper (`004019b0`) into a global constant | game |
| 67 | `NiRTTI` constructors (`00a7b610`) | game (RTTI chain) |
| 554 | 140 other shapes (`0044f570` model loader, sky `00633c90`/`00633000`, lists `0096a2d0`, strings, CRT `00ec6054`, ...) | mostly game |
| 8 | the C initializers | CRT |

One initializer is translated. Nearly all of them build game data, so they
can't stay "library". Proposal: `Host::start` runs both tables in `__cinit`'s
order by address (`_atexit` `00ec658f` is already `platform`). The four
regular shapes (about 17,100 entries) are best translated by a generator
that checks each body against its shape and refuses anything else; the
callees (the Setting constructors and their bases `0040e130`/`00404920`/
`0045cfb0`, the collections' `Add`, the 25 registration functions,
`004019d0`) and the 554 others are translated by hand. Until every entry is
translated, the run would skip the untranslated ones and print how many
were skipped. Entries can depend on earlier ones (collections before
settings), so the order is kept. `crates/host/tests/static_initializers.rs`
measures the tables and holds the acceptance test: three sample settings
(`fGunShellLifetime`, `fPlayerMaxResistance`, `fMaxArmorRating`) must have
the name and default their initializers pass. It is ignored and fails
today.

## Lessons from the first sessions (2026-10-09)

- **Test doubles that keep state.** `e.register` takes a plain `fn`; a double that
  captures anything goes through `e.register_double`. `Engine::new()` registers
  every translated function, so a test doubles every callee outside its file
  or that callee runs for real.
- **Variable-argument functions** (cdecl `...`, e.g. `sprintf_s`, `PrintLine`,
  `Script::ParseParameters`): write the typed function taking the extra words
  as a slice, and register a small entry `fn(&mut Engine, &[u32]) -> Ret` as
  `(addr, entry as AbiFn)` (see `parse_parameters_entry` in `tesscript.rs`).
  Where the callee receives a `va_list`, copy the words into game memory and
  pass that address.
- **Structs passed by value** (points, matrices, `BSString`) are consecutive
  words in `args!`, in push order (first member at the lowest address).
- **Compiler-folded masks.** Blocks guarded by `flags & 0` (the compiler folded
  the mask to zero) never run; leave them out and say so in the doc comment.
- **Arguments come from the disassembly.** The decompiler often hangs a
  pushed word on the wrong call (`PUSH x; CALL getter; MOV ECX,EAX; CALL
  method`: `x` belongs to `method`), drops `this`, and mis-sizes `this`
  (indexing in `wchar_t` units). A function's parameters are exactly the
  words its `RET n` (or its callers' cleanup, for cdecl) shows; a word it
  never reads is still a parameter, named `_unused_<n>`.
- **PC layouts differ from the Xbox PDB in known ways.** `TESForm` is 0x18
  on PC (no editor ID or version-control fields), so fields of every class
  derived from it sit 0x10 lower than the PDB says (`TESObjectREFR`,
  `TESObjectCELL`, ...). Some classes lose the MemDebug build's leading
  accounting words (`Model`: 8 bytes). Gamebryo transforms are 16-byte
  aligned on Xbox only. Always confirm offsets against the PC code.
- **Map names can be wrong for folded code.** Identical-code folding puts
  one method's name on another's body (`005f36f0`, named
  `ActorMover::GetPreferredMoveMode`, returns `TES::pInteriorCell`). Name
  by the body.
- **Script command bodies** share one cdecl signature of 8 words
  (`ScriptArgs` in `units/fallout_shared/tesscriptfunctions.rs`).
- **Split units.** The largest units are split into part files
  (`<stem>_p2.rs`, ...), each with its own owner and address range; the
  main file's owner declares shared constants and helpers `pub(crate)`.
- **Tests.** `e.call_log` includes the top-level call the test makes;
  calls between functions of one file are direct and not logged. Doubles
  that keep state use `register_double`. Tests that read exe data map the
  page and write the values the code reads.
- **Scratch files** never go to `/tmp` or a shared folder: translators run
  in parallel.
