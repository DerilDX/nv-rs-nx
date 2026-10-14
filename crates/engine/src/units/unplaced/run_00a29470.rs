//! `(unplaced)/run_00a29470` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a294f0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f367e` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a294f0(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_367e) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_367e, 0x1);
    e.mem.set_u32(0x011f_3e28, 0xe7cd40);
    e.mem.set_u32(0x011f_3ac8, 0xa29680);
    e.mem.set_u32(0x011f_3a38, 0xa29680);
    e.mem.set_u32(0x011f_39a8, 0xe7cd40);
    e.mem.set_u32(0x011f_3918, 0xa29680);
    e.mem.set_u8(0x011f_3776, 0x8);
    e.call(0x00a2_91f0, &args![0x5u32, 0x0u32]);
    1
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a294f0, fn_00a294f0() -> u32)]
}
