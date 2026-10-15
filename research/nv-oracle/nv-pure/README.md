# nv-pure: automatic oracle check for pure translations

Runs every pure translation on the real CPU through [`nv-call`](../README.md#nv-call)
and in the engine crate with the same inputs, and compares the results bit for
bit (docs/HANDOFF.md next task 10, #182). A wrong translation of a pure function
fails by itself, without a hand-written test.

It is a local check: CI has no copy of the game. Without `NV_ORACLE_EXE` it
prints that it skipped and exits 0.

## Pieces

| Step | What | Output |
| --- | --- | --- |
| 1 | `research/ghidra/NvPurity.java` on your own analysed project (read-only): per function, stores outside its own frame, indirect and imported calls, FS use, direct calls, `RET n`, ECX/EDX read on entry, stack-argument bytes. | `purity.tsv`, private |
| 2 | `nv-pure list --purity purity.tsv`: joins those facts with the `// Translated from` markers and `entry!` signatures of `crates/engine`. | `pure.tsv` (this folder): addresses, conventions, signatures |
| 3 | `nv-pure check`: generates inputs, runs them through `nv-call` and the engine, compares. | `%USERPROFILE%\nv-re\oracle\pure\` (vectors, results, `failures.tsv`), private |

Only `pure.tsv` is committed: addresses and signatures, no exe bytes or outputs.
The inputs are regenerated from a fixed seed (the function's address), so the
same vectors run everywhere.

## Which functions are checked

A translation is on the list when all of these hold:

- it has a `Translated from` marker and no `Translation incomplete` marker, and an
  `entry!` registration whose types the check knows;
- in the exe it is pure, transitively: it and everything it calls store only to
  their own stack frame, make no indirect or imported calls and don't touch FS
  (TLS, SEH frames);
- its arguments and result are scalars (`u8`..`u64`, `bool`, `f32`, `f64`), and
  neither it nor its callees read through a pointer (so no object has to be built
  for it);
- the calling convention follows from the exe and agrees with the signature:
  ECX/EDX read on entry (thiscall, fastcall), the `RET n` size (stdcall/thiscall
  pop exactly the stack words the signature leaves), and the stack-argument bytes
  the body reads (never more than the signature passes). Functions where nothing
  fits are listed by `nv-pure list` as convention mismatches; each is worth a look,
  since a missing or extra parameter in a translation shows up this way;
- it doesn't take a leading `f64` in x87 ST0 (`nv-call` can't preset ST0).

`check` also skips a listed function when it or one of its callees is not
registered in the engine at the time of the run.

## Inputs

1. **Per parameter**, every value below with the other parameters at a plain
   value (1.5 or 3):
   - `f32`/`f64`: ±0, ±1, 0.5, 2, 0.001, 1000, π, -π/2, 360, 0.1, ±1e30, max,
     smallest normal, smallest and largest denormal, ±inf, ±qNaN, qNaN with
     payload, sNaN; rounding ties and conversion limits (±0.5, ±1.5, ±2.5, 255.5,
     65535.5, the floats on either side of 0.5, 2^23, 2^24, the floats around
     ±2^31, 2^32, ±1e10).
   - integers: 0, 1, -1, 2, 3, 7, INT_MAX, INT_MIN, 0xff, 0x100, 0xffff, 0x10000,
     0xffffff00, 0x400000.
   - `u8`/`u16`/`bool`: their own edges, passed as the zero- or sign-extended word
     the uniform form uses.
   - **The function's own constants** (NvPurity mines them, `consts` in
     `pure.tsv`, callees' included): c-1, c and c+1 of every immediate it compares
     or computes with (`CMP EAX,0x14`), and every float/double it loads from
     memory with its two neighbouring values. Branch boundaries that no generic
     list hits come from here.
2. **For two or three parameters, every combination** of a small special set per
   type (floats: ±0, ±1, 1.5, qNaN, ±inf; integers 0, 1, 3, -1, INT_MIN): signed
   zeros against each other, equal operands, NaN on either side.
3. **Seeded random lists** up to `--count` (default 64, at least 16): floats half
   raw bit patterns (any class), half in ±1000; integers mostly small, sometimes
   any pattern.

The FPU runs with nv-call's defaults (control word 0x027F: 53-bit precision,
round to nearest; MXCSR 0x1F80), matching the engine's "compute in f64, round at
float stores" rule.

## Verdicts

| Kind | Meaning | Fails the check |
| --- | --- | --- |
| `Match` | Same bits (`bool`: AL zero/non-zero; `u8`/`u16`: low byte/word; floats: ST0 rounded to the type). | no |
| `BothFailed` | Both failed the same way: the exe's access violation and the engine's unmapped read at the same address (or both divided by zero). | no |
| `OutOfImageRead` | The engine stopped at memory it never maps (outside `.rdata`..`CONST`, e.g. a random table index landing in `.text`), which nv-call's process happened to have mapped. | no |
| `Mismatch` | Different results. | yes |
| `NanBits` | Both NaN, different sign or payload. | yes |
| `ExeFaultOnly` / `RustPanicOnly` | One side failed. | yes |
| `FaultMismatch` | Both failed, but at different addresses (inside the engine's mapped data) or in different ways. | yes |
| `Convention` | The exe popped a different number of bytes than the listed convention: the list is wrong, not the translation. | yes |

The engine side runs every vector on a fresh engine (the exe's data sections
mapped anew), as nv-call restores the image's writable memory before every
vector, so no vector sees another's state.

The summary also prints how many functions give **varying** exe results over
their inputs. A function whose results are all the same value (no parameters, or
a result that depends on globals the game fills at run time, zero in the file) is
only checked for that one value; they are listed in `coverage.tsv` in the output
folder.

## Validation

- **Mutation test**, fifteen realistic bugs planted in translations, one per
  function, in two rounds:

  | Genre | Planted bug | Caught |
  | --- | --- | --- |
  | range boundary | `10..=20` → `10..20` (TESFurniture::IsSitMarker) | yes, only by the mined constant 20 |
  | float compare strictness | `>` → `>=` in a float min (only +0/-0 differ) | yes, by the cross-parameter ±0 pair |
  | compare in a bool result | `>` → `>=` in an epsilon compare | yes |
  | wrong offset/constant | member +8 → +0xc | yes |
  | rounding mode | round-half-to-even → half-down | yes, by the rounding ties |
  | argument order | `second / first` → `first / second` | yes |
  | signed vs unsigned | `u32` max compared as `i32` | yes |
  | shift kind | arithmetic `>> 5` → logical | yes |
  | sign extension | `y as u16` → `y as i32` in a packed key | yes |
  | mask width | `& 0xffffff00` → `& 0x0000ff00` | yes |
  | float precision | intermediate `float` rounding dropped | yes |
  | integer overflow | wrapping `i8` add → saturating | yes |
  | float→int overflow | NaN/range guard deleted | yes |
  | wrong callee | `_ftol2` (truncate) → the rounding helper | yes |
  | NaN quieting | `FLD`/`FSTP` sNaN quieting dropped from an `f32` result | **no**, see limits |

  The first two were missed until the constant boundaries, the cross-parameter
  combinations and the rounding ties were added, which is why those exist.
- **Determinism.** Two runs give the same counts and the same failures; only
  which out-of-range reads fault can vary with nv-call's process layout, and
  those are never failures.
- **Non-vacuity.** 108 of 324 functions vary with their inputs and match bit for
  bit, which cannot happen with arguments in the wrong places; the `Convention`
  check (stack bytes the exe actually popped) passes for every listed function.

## Known limits

- **sNaN quieting in `f32` results is invisible.** The uniform form returns
  floats as `Ret::st0: f64`, and `f32 as f64` quiets a signalling NaN on x86-64,
  so a translation that forgets the `FLD`/`FSTP` quieting reads back the same
  quiet NaN as the exe. A Rust caller of the typed function would still see the
  sNaN. Fix: carry the raw `f32` bits in `Ret`. (`f64` results are not
  affected.)
- **Only pure functions with scalar arguments** (335 of 16,311 translations).
  Functions that read through pointer arguments need buffers; ones that touch
  globals need state.
- **Runtime-initialised globals are zero.** Neither nv-call nor the engine runs
  the CRT's static initialisers, so a function reading a table the game fills at
  start-up (the 512-entry sine tables) returns 0 on both sides; such matches are
  listed in `coverage.tsv`.
- **One FPU mode.** Vectors run with control word 0x027F (53-bit), the engine's
  model; results that depend on the game thread's actual precision mode are not
  compared across modes.

## Running it

```powershell
# once: build nv-call (32-bit)
cd research\nv-oracle
cargo build --release --target i686-pc-windows-msvc

# purity facts from your project (see research/ghidra/README.md for the .cmd wrapper)
analyzeHeadless <proj> FalloutNV -process FalloutNV_unpacked.exe -noanalysis -readOnly `
  -scriptPath research\ghidra -postScript NvPurity.java out=<private>\purity.tsv

cd research\nv-oracle\nv-pure
cargo run --release -- list --purity <private>\purity.tsv
$env:NV_ORACLE_EXE = '<path to the unpacked FalloutNV.exe 1.4.0.525>'
cargo run --release -- check            # all listed functions
cargo run --release -- check --limit 50 # the pilot
cargo run --release -- check --only 00ec62c0,00a01a20
```

`NV_CALL` overrides the nv-call.exe path (default:
`research/nv-oracle/target/i686-pc-windows-msvc/release/nv-call.exe`),
`NV_ORACLE_OUT` the output folder. Exit status 1 when any vector fails.

The MSVC build of nv-call was run here on Windows 11 (i9-12900H) against the
unpacked 1.4.0.525 exe.
