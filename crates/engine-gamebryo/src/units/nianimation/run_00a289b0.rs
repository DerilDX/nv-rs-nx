//! `NiAnimation/run_00a289b0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a28cb0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3669` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28cb0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3669) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d50, 0x0);
    e.mem.set_u32(0x011f_3c30, 0x0);
    e.mem.set_u32(0x011f_3ba0, 0x0);
    e.mem.set_u32(0x011f_3840, 0x0);
    e.mem.set_u8(0x011f_3669, 0x1);
    e.mem.set_u32(0x011f_3de0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a80, 0xa29680);
    e.mem.set_u32(0x011f_39f0, 0xa29680);
    e.mem.set_u32(0x011f_3960, 0xe7cd40);
    e.mem.set_u32(0x011f_38d0, 0xa29680);
    e.mem.set_u8(0x011f_3764, 0x14);
    e.mem.set_u32(0x011f_3cc0, 0xa286e0);
    e.mem.set_u32(0x011f_3b10, 0xa28a80);
    e.mem.set_u32(0x011f_37b0, 0xa28ab0);
    1
}

// Translated from 00a29110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a29110(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d58, 0x0);
    e.mem.set_u32(0x011f_3c38, 0x0);
    e.mem.set_u32(0x011f_3ba8, 0x0);
    e.mem.set_u8(0x011f_367c, 0x1);
    e.mem.set_u32(0x011f_3de8, 0xa290a0);
    e.mem.set_u32(0x011f_3a88, 0xa28ed0);
    e.mem.set_u32(0x011f_39f8, 0xa28d40);
    e.mem.set_u32(0x011f_3968, 0xa28f00);
    e.mem.set_u32(0x011f_38d8, 0xa2a600);
    e.mem.set_u8(0x011f_3766, 0x24);
    e.mem.set_u32(0x011f_3cc8, 0xa28d90);
    e.mem.set_u32(0x011f_3b18, 0xa28ea0);
    e.mem.set_u32(0x011f_37b8, 0xa28dc0);
    e.mem.set_u32(0x011f_3848, 0xa28f50);
    1
}

// Translated from 00a291f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a291f0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa291d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa291a0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a28cb0, fn_00a28cb0() -> u32),
        entry!(0x00a29110, fn_00a29110() -> u32),
        entry!(0x00a291f0, fn_00a291f0(u32, u32)),
    ]
}
