//! `NiAnimation/run_00a2bde0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a2c4c0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3696` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2c4c0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3696) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d5c, 0x0);
    e.mem.set_u32(0x011f_3c3c, 0x0);
    e.mem.set_u32(0x011f_3bac, 0x0);
    e.mem.set_u8(0x011f_3696, 0x1);
    e.mem.set_u32(0x011f_3dec, 0xa2c420);
    e.mem.set_u32(0x011f_3a8c, 0xa2c490);
    e.mem.set_u32(0x011f_39fc, 0xa2bbf0);
    e.mem.set_u32(0x011f_396c, 0xa2be60);
    e.mem.set_u32(0x011f_38dc, 0xa2a600);
    e.mem.set_u8(0x011f_3767, 0x40);
    e.mem.set_u32(0x011f_3ccc, 0xa2bc60);
    e.mem.set_u32(0x011f_3b1c, 0xa2bde0);
    e.mem.set_u32(0x011f_37bc, 0xa2c130);
    e.mem.set_u32(0x011f_384c, 0xa2c1b0);
    1
}

// Translated from 00a2cb80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3697` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2cb80(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3697) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d2c, 0x0);
    e.mem.set_u32(0x011f_3c0c, 0x0);
    e.mem.set_u32(0x011f_3b7c, 0x0);
    e.mem.set_u8(0x011f_3697, 0x1);
    e.mem.set_u32(0x011f_3dbc, 0xa2caf0);
    e.mem.set_u32(0x011f_3a5c, 0xa2cb50);
    e.mem.set_u32(0x011f_39cc, 0xa2c5d0);
    e.mem.set_u32(0x011f_393c, 0xa2c660);
    e.mem.set_u32(0x011f_38ac, 0xa2a600);
    e.mem.set_u8(0x011f_375b, 0x1c);
    e.mem.set_u32(0x011f_3c9c, 0xa2c610);
    e.mem.set_u32(0x011f_3aec, 0xa2c550);
    e.mem.set_u32(0x011f_378c, 0xa2c7a0);
    e.mem.set_u32(0x011f_381c, 0xa2c890);
    1
}

// Translated from 00a2ce20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3698` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2ce20(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3698) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3dac, 0x0);
    e.mem.set_u32(0x011f_3c8c, 0x0);
    e.mem.set_u32(0x011f_3bfc, 0x0);
    e.mem.set_u8(0x011f_3698, 0x1);
    e.mem.set_u32(0x011f_3e3c, 0xa2cdc0);
    e.mem.set_u32(0x011f_3adc, 0xa2cc60);
    e.mem.set_u32(0x011f_3a4c, 0xa2cc10);
    e.mem.set_u32(0x011f_39bc, 0xa2cc90);
    e.mem.set_u32(0x011f_392c, 0xa2a600);
    e.mem.set_u8(0x011f_377b, 0x8);
    e.mem.set_u32(0x011f_3d1c, 0xa2cc40);
    e.mem.set_u32(0x011f_3b6c, 0xa2cc30);
    e.mem.set_u32(0x011f_380c, 0xa29680);
    e.mem.set_u32(0x011f_389c, 0xa2ccd0);
    1
}

// Translated from 00a2d140 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3699` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d140(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3699) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d7c, 0x0);
    e.mem.set_u32(0x011f_3c5c, 0x0);
    e.mem.set_u32(0x011f_3bcc, 0x0);
    e.mem.set_u32(0x011f_37dc, 0x0);
    e.mem.set_u8(0x011f_3699, 0x1);
    e.mem.set_u32(0x011f_3e0c, 0xa2cf60);
    e.mem.set_u32(0x011f_3aac, 0xa2cfd0);
    e.mem.set_u32(0x011f_3a1c, 0xa2ab10);
    e.mem.set_u32(0x011f_398c, 0xa2ced0);
    e.mem.set_u32(0x011f_38fc, 0xa2a600);
    e.mem.set_u8(0x011f_376f, 0x14);
    e.mem.set_u32(0x011f_3cec, 0xa2cf20);
    e.mem.set_u32(0x011f_3b3c, 0xa2a1a0);
    e.mem.set_u32(0x011f_386c, 0xa2d000);
    1
}

// Translated from 00a2d360 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369a` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d360(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369a) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d34, 0x0);
    e.mem.set_u32(0x011f_3c14, 0x0);
    e.mem.set_u32(0x011f_3b84, 0x0);
    e.mem.set_u8(0x011f_369a, 0x1);
    e.mem.set_u32(0x011f_3dc4, 0xa2a5a0);
    e.mem.set_u32(0x011f_3a64, 0xa2d230);
    e.mem.set_u32(0x011f_39d4, 0xa2d1e0);
    e.mem.set_u32(0x011f_3944, 0xa2cc90);
    e.mem.set_u32(0x011f_38b4, 0xa2a600);
    e.mem.set_u8(0x011f_375d, 0x8);
    e.mem.set_u32(0x011f_3ca4, 0xa2d200);
    e.mem.set_u32(0x011f_3af4, 0xa2d1d0);
    e.mem.set_u32(0x011f_3794, 0xa29680);
    e.mem.set_u32(0x011f_3824, 0xa2d260);
    1
}

// Translated from 00a2d610 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369b` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d610(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369b) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_369b, 0x1);
    e.mem.set_u32(0x011f_3ddc, 0xa2a8e0);
    e.mem.set_u32(0x011f_3a7c, 0xa2d470);
    e.mem.set_u32(0x011f_39ec, 0xa2a7b0);
    e.mem.set_u32(0x011f_395c, 0xa27040);
    e.mem.set_u32(0x011f_38cc, 0xa2a600);
    e.mem.set_u8(0x011f_3763, 0x10);
    e.call(0x00a2_d5b0, &args![0x1u32, 0x5u32]);
    1
}

// Translated from 00a2d810 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f369c` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2d810(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_369c) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d64, 0x0);
    e.mem.set_u32(0x011f_3c44, 0x0);
    e.mem.set_u32(0x011f_3bb4, 0x0);
    e.mem.set_u8(0x011f_369c, 0x1);
    e.mem.set_u32(0x011f_3df4, 0xa2abc0);
    e.mem.set_u32(0x011f_3a94, 0xa2d690);
    e.mem.set_u32(0x011f_3a04, 0xa2ab10);
    e.mem.set_u32(0x011f_3974, 0xa2ab40);
    e.mem.set_u32(0x011f_38e4, 0xa2a600);
    e.mem.set_u8(0x011f_3769, 0x14);
    e.mem.set_u32(0x011f_3cd4, 0xa2cf20);
    e.mem.set_u32(0x011f_3b24, 0xa2d680);
    e.mem.set_u32(0x011f_37c4, 0xa2d670);
    e.mem.set_u32(0x011f_3854, 0xa2d6c0);
    1
}

// Translated from 00a2d5b0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2d5b0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2d3f0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2d420);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2a7e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2d4a0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a2c4c0, fn_00a2c4c0() -> u32),
        entry!(0x00a2cb80, fn_00a2cb80() -> u32),
        entry!(0x00a2ce20, fn_00a2ce20() -> u32),
        entry!(0x00a2d140, fn_00a2d140() -> u32),
        entry!(0x00a2d360, fn_00a2d360() -> u32),
        entry!(0x00a2d610, fn_00a2d610() -> u32),
        entry!(0x00a2d810, fn_00a2d810() -> u32),
        entry!(0x00a2d5b0, fn_00a2d5b0(u32, u32)),
    ]
}
