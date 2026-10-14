# initgen: the static-initializer table

Before `WinMain`, FalloutNV.exe's `__cinit` (`00eccd14`) runs two tables of
function pointers: `_initterm_e(010113c4, 010113e8)` (8 C initializers) and
`_initterm(00fdf4a4, 010113c0)` (17,728 non-null C++ dynamic initializers).
They build thousands of game globals: the `Setting`s with their names and
defaults, the NiAnimation function tables, computed float constants, RTTI
records. The engine crate runs them at start-up
(`crates/engine/src/static_init.rs`, called by `host::Host::start`).

Most entries are a few instructions of one of three shapes. Instead of one
Rust function per entry, initgen writes one row per entry into
`crates/engine/data/static_initializers.tsv`, and the engine has one
translation per shape that executes the rows in `__cinit`'s order. The
ledger counts each row as the translation of its entry.

```text
cargo run --release -- --exe "<...>/FalloutNV.exe" --cache <private dir> \
    --out ../../crates/engine/data/static_initializers.tsv --report <refused.tsv>
```

- The entries come from the executable's own tables (`--exe`, read-only).
- Each entry's instructions come from the read-only Ghidra server
  (`disassemble_function`, default `http://127.0.0.1:8090`). For the ~1,400
  entries Ghidra made no function of, the entry's bytes (`read_memory`, the
  unpacked image) are decoded by `decode`, which knows only the encodings the
  shapes use and refuses any other byte. Both are cached in `--cache`;
  disassembly and bytes are never committed.
- An entry becomes a row only if its whole body matches one shape,
  instruction by instruction. Everything else is written to `--report` with
  the reason and stays for hand translation.

## Shapes and rows

Each row has three tab-separated columns: `entry`, `shape` and `parameters`.
Addresses are 8 hex digits. Values from the executable's data (defaults,
constants) are given as the addresses they are loaded from, so the table
holds no bytes of the executable.

| Shape | Code (between `PUSH EBP; MOV EBP,ESP` and `POP EBP; RET`, or frameless up to `RET`) | Parameters |
| --- | --- | --- |
| `reg` | `CALL f; MOV [g],EAX` | `f g` |
| `object` | pushed arguments; `MOV ECX,g; CALL ctor`; optionally `PUSH dtor; CALL _atexit; ADD ESP,4` (or `POP ECX`) | `ctor g dtor\|- args...` in the constructor's order: `w:<word>`, `f:<src>` (a float pushed through `PUSH ECX; <load>; FSTP float ptr [ESP]`), `d:<src>` (a double through `SUB ESP,8; <load>; FSTP double ptr [ESP]`); `<src>` is `s.<addr>` (`FLD float`), `d.<addr>` (`FLD double`), `1` (`FLD1`) or `0` (`FLDZ`) |
| `const` | straight-line x87 and moves, the x87 stack empty at both ends | `;`-separated ops: `fld.s/fld.d <a>`, `fld1`, `fldz`, `fchs`, `fmul/fdiv/fdivr/fadd/fsub/fsubr.s/.d <a>`, `fmulp/fdivp/fdivrp/faddp/fsubp/fsubrp`, `fstp.s/fstp.d <g>`, `sqrt <a>` (`PUSH ECX; FLD float ptr [a]; FSTP float ptr [ESP]; CALL 004019b0; ADD ESP,4`), `call <f>` (no arguments), `st.eax <g>` (after a call), `st.w/st.b <g> <imm>`, `cp.d <g> <a>` (`MOV reg,[a]; MOV [g],reg`) |

Not shapes: a call to `00a2e180` (it answers the incoming `ECX`, which
`_initterm` leaves from the previous initializer), anything with a branch,
stack frame locals or another instruction.

`--rust <addr,...>` prints Rust translations of the NiAnimation registration
functions the `reg` rows call and of their indexed table helpers, each
matched against its exact form (used once to write the functions in
`crates/engine/src/units/nianimation/run_*.rs` and
`units/unplaced/run_00a29470.rs`).

## Checks

`cargo test` checks each shape's matcher and the byte decoder; the engine's
`static_init` tests check the parser and each shape's translation (x87
precision, NaN rules, argument order); `crates/host/tests/static_initializers.rs`
(opt-in, `NV_EXE`) runs the whole table on the real executable and checks
settings against their initializers' values.
