//! `NiAnimation/run_00a298f0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a299f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367f` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a299f0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367f) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d68, 0x0);
    e.mem.set_u32(0x011f_3c48, 0x0);
    e.mem.set_u32(0x011f_3bb8, 0x0);
    e.mem.set_u32(0x011f_37c8, 0x0);
    e.mem.set_u32(0x011f_3858, 0x0);
    e.mem.set_u8(0x011f_367f, 0x1);
    e.mem.set_u32(0x011f_3df8, 0xe7cd40);
    e.mem.set_u32(0x011f_3a98, 0xa29680);
    e.mem.set_u32(0x011f_3a08, 0xa29680);
    e.mem.set_u32(0x011f_3978, 0xe7cd40);
    e.mem.set_u32(0x011f_38e8, 0xa29680);
    e.mem.set_u8(0x011f_376a, 0x14);
    e.mem.set_u32(0x011f_3cd8, 0xa29690);
    e.mem.set_u32(0x011f_3b28, 0xa299c0);
    1
}

// Translated from 00a2a110 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3690` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a110(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3690) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d60, 0x0);
    e.mem.set_u32(0x011f_3c40, 0x0);
    e.mem.set_u32(0x011f_3bb0, 0x0);
    e.mem.set_u8(0x011f_3690, 0x1);
    e.mem.set_u32(0x011f_3df0, 0xa2a070);
    e.mem.set_u32(0x011f_3a90, 0xa2a0e0);
    e.mem.set_u32(0x011f_3a00, 0xa29cf0);
    e.mem.set_u32(0x011f_3970, 0xa29db0);
    e.mem.set_u32(0x011f_38e0, 0xa2a600);
    e.mem.set_u8(0x011f_3768, 0x48);
    e.mem.set_u32(0x011f_3cd0, 0xa29b10);
    e.mem.set_u32(0x011f_3b20, 0xa29c20);
    e.mem.set_u32(0x011f_37c0, 0xa29e00);
    e.mem.set_u32(0x011f_3850, 0xa29e40);
    1
}

// Translated from 00a2a4d0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3691` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a4d0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3691) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d6c, 0x0);
    e.mem.set_u32(0x011f_3c4c, 0x0);
    e.mem.set_u32(0x011f_3bbc, 0x0);
    e.mem.set_u32(0x011f_37cc, 0x0);
    e.mem.set_u8(0x011f_3691, 0x1);
    e.mem.set_u32(0x011f_3dfc, 0xa2a320);
    e.mem.set_u32(0x011f_3a9c, 0xa2cfd0);
    e.mem.set_u32(0x011f_3a0c, 0xa2ab10);
    e.mem.set_u32(0x011f_397c, 0xa2a1b0);
    e.mem.set_u32(0x011f_38ec, 0xa2a600);
    e.mem.set_u8(0x011f_376b, 0x14);
    e.mem.set_u32(0x011f_3cdc, 0xa2a200);
    e.mem.set_u32(0x011f_3b2c, 0xa2a1a0);
    e.mem.set_u32(0x011f_385c, 0xa2a390);
    1
}

// Translated from 00a2a720 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3692` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2a720(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3692) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d24, 0x0);
    e.mem.set_u32(0x011f_3c04, 0x0);
    e.mem.set_u32(0x011f_3b74, 0x0);
    e.mem.set_u8(0x011f_3692, 0x1);
    e.mem.set_u32(0x011f_3db4, 0xa2a5a0);
    e.mem.set_u32(0x011f_3a54, 0xa2d230);
    e.mem.set_u32(0x011f_39c4, 0xa2d1e0);
    e.mem.set_u32(0x011f_3934, 0xa2cc90);
    e.mem.set_u32(0x011f_38a4, 0xa2a600);
    e.mem.set_u8(0x011f_3759, 0x8);
    e.mem.set_u32(0x011f_3c94, 0xa2a560);
    e.mem.set_u32(0x011f_3ae4, 0xa2d1d0);
    e.mem.set_u32(0x011f_3784, 0xa29680);
    e.mem.set_u32(0x011f_3814, 0xa2a620);
    1
}

// Translated from 00a2aab0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3693` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2aab0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3693) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3693, 0x1);
    e.mem.set_u32(0x011f_3dcc, 0xa2a8e0);
    e.mem.set_u32(0x011f_3a6c, 0xa2d470);
    e.mem.set_u32(0x011f_39dc, 0xa2a7b0);
    e.mem.set_u32(0x011f_394c, 0xa27040);
    e.mem.set_u32(0x011f_38bc, 0xa2a600);
    e.mem.set_u8(0x011f_375f, 0x10);
    e.call(0x00a2_aa50, &args![0x1u32, 0x1u32]);
    1
}

// Translated from 00a2ad80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3694` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2ad80(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3694) != 0 {
        return 0;
    }
    e.mem.set_u32(0x011f_3d54, 0x0);
    e.mem.set_u32(0x011f_3c34, 0x0);
    e.mem.set_u32(0x011f_3ba4, 0x0);
    e.mem.set_u8(0x011f_3694, 0x1);
    e.mem.set_u32(0x011f_3de4, 0xa2abc0);
    e.mem.set_u32(0x011f_3a84, 0xa2d690);
    e.mem.set_u32(0x011f_39f4, 0xa2ab10);
    e.mem.set_u32(0x011f_3964, 0xa2ab40);
    e.mem.set_u32(0x011f_38d4, 0xa2a600);
    e.mem.set_u8(0x011f_3765, 0x14);
    e.mem.set_u32(0x011f_3cc4, 0xa2ab90);
    e.mem.set_u32(0x011f_3b14, 0xa2d680);
    e.mem.set_u32(0x011f_37b4, 0xa2d670);
    e.mem.set_u32(0x011f_3844, 0xa2ac30);
    1
}

// Translated from 00a2bb90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3695` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a2bb90(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3695) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3695, 0x1);
    e.mem.set_u32(0x011f_3dd4, 0xa2bb00);
    e.mem.set_u32(0x011f_3a74, 0xa2bb60);
    e.mem.set_u32(0x011f_39e4, 0xa2ae10);
    e.mem.set_u32(0x011f_3954, 0xa2af30);
    e.mem.set_u32(0x011f_38c4, 0xa2a600);
    e.mem.set_u8(0x011f_3761, 0x4c);
    e.call(0x00a2_baa0, &args![0x1u32, 0x3u32]);
    1
}

// Translated from 00a2aa50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2aa50(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2a7f0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2a890);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2a7e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2a940);
}

// Translated from 00a2baa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a2baa0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa2b6d0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa2af70);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa2b050);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa2b130);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa2ae90);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa2b410);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa2b850);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a299f0, fn_00a299f0() -> u32),
        entry!(0x00a2a110, fn_00a2a110() -> u32),
        entry!(0x00a2a4d0, fn_00a2a4d0() -> u32),
        entry!(0x00a2a720, fn_00a2a720() -> u32),
        entry!(0x00a2aab0, fn_00a2aab0() -> u32),
        entry!(0x00a2ad80, fn_00a2ad80() -> u32),
        entry!(0x00a2bb90, fn_00a2bb90() -> u32),
        entry!(0x00a2aa50, fn_00a2aa50(u32, u32)),
        entry!(0x00a2baa0, fn_00a2baa0(u32, u32)),
    ]
}
