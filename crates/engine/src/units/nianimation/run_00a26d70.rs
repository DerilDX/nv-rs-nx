//! `NiAnimation/run_00a26d70` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a26e20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3654` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a26e20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3654) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d20, 0x0);
    e.mem.set_u32(0x011f_3c00, 0x0);
    e.mem.set_u32(0x011f_3b70, 0x0);
    e.mem.set_u32(0x011f_3810, 0x0);
    e.mem.set_u8(0x011f_3654, 0x1);
    e.mem.set_u32(0x011f_3db0, 0xe7cd40);
    e.mem.set_u32(0x011f_3a50, 0xa29680);
    e.mem.set_u32(0x011f_39c0, 0xa29680);
    e.mem.set_u32(0x011f_3930, 0xe7cd40);
    e.mem.set_u32(0x011f_38a0, 0xa29680);
    e.mem.set_u8(0x011f_3758, 0x8);
    e.mem.set_u32(0x011f_3c90, 0xa29680);
    e.mem.set_u32(0x011f_3ae0, 0xa26b00);
    e.mem.set_u32(0x011f_3780, 0xa29680);
    1
}

// Translated from 00a273a0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3655` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a273a0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3655) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d28, 0x0);
    e.mem.set_u32(0x011f_3c08, 0x0);
    e.mem.set_u32(0x011f_3b78, 0x0);
    e.mem.set_u8(0x011f_3655, 0x1);
    e.mem.set_u32(0x011f_3db8, 0xa27250);
    e.mem.set_u32(0x011f_3a58, 0xa27320);
    e.mem.set_u32(0x011f_39c8, 0xa26f80);
    e.mem.set_u32(0x011f_3938, 0xa27040);
    e.mem.set_u32(0x011f_38a8, 0xa2a600);
    e.mem.set_u8(0x011f_375a, 0x10);
    e.mem.set_u32(0x011f_3c98, 0xa26fe0);
    e.mem.set_u32(0x011f_3ae8, 0xa26f30);
    e.mem.set_u32(0x011f_3788, 0xa29680);
    e.mem.set_u32(0x011f_3818, 0xa27080);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a26e20, fn_00a26e20() -> u32),
        entry!(0x00a273a0, fn_00a273a0() -> u32),
    ]
}
