//! `NiAnimation/run_00a276a0` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00a27b00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3656` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a27b00(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3656) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3656, 0x1);
    e.mem.set_u32(0x011f_3dc8, 0xe7cd40);
    e.mem.set_u32(0x011f_3a68, 0xa29680);
    e.mem.set_u32(0x011f_39d8, 0xa29680);
    e.mem.set_u32(0x011f_3948, 0xe7cd40);
    e.mem.set_u32(0x011f_38b8, 0xa29680);
    e.mem.set_u8(0x011f_375e, 0x10);
    e.call(0x00a2_7aa0, &args![0x1u32, 0x0u32]);
    1
}

// Translated from 00a28680 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A one-shot registration of animation function tables (no Xbox PDB
/// name; called by the static initializers of every unit that includes
/// its header): the first call (the guard byte `011f3668` clear)
/// stores the table entries and answers 1; later calls answer 0.
pub fn fn_00a28680(e: &mut Engine) -> u32 {
    if e.mem.u8(0x011f_3668) != 0 {
        return 0;
    }
    e.mem.set_u8(0x011f_3668, 0x1);
    e.mem.set_u32(0x011f_3dd0, 0xa281e0);
    e.mem.set_u32(0x011f_3a70, 0xa28260);
    e.mem.set_u32(0x011f_39e0, 0xa27b60);
    e.mem.set_u32(0x011f_3950, 0xa27c40);
    e.mem.set_u32(0x011f_38c0, 0xa2a600);
    e.mem.set_u8(0x011f_3760, 0x40);
    e.call(0x00a2_8620, &args![0x1u32, 0x2u32]);
    1
}

// Translated from 00a27aa0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a27aa0(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa291e0);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27470);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27760);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa29680);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0x0);
}

// Translated from 00a28620 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Stores animation function-table entries for one (row, column) pair
/// (`cdecl`; no Xbox PDB name): the offset is `(row * 6 + column) * 4`
/// into each table.
pub fn fn_00a28620(e: &mut Engine, row: u32, column: u32) {
    let at = row
        .wrapping_mul(3)
        .wrapping_mul(2)
        .wrapping_add(column)
        .wrapping_mul(4);
    e.mem.set_u32(at.wrapping_add(0x011f_3d20), 0xa28060);
    e.mem.set_u32(at.wrapping_add(0x011f_3c90), 0xa27c80);
    e.mem.set_u32(at.wrapping_add(0x011f_3c00), 0xa27d60);
    e.mem.set_u32(at.wrapping_add(0x011f_3b70), 0xa27e40);
    e.mem.set_u32(at.wrapping_add(0x011f_3ae0), 0xa27bd0);
    e.mem.set_u32(at.wrapping_add(0x011f_3780), 0xa27ee0);
    e.mem.set_u32(at.wrapping_add(0x011f_3810), 0xa282a0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a27b00, fn_00a27b00() -> u32),
        entry!(0x00a28680, fn_00a28680() -> u32),
        entry!(0x00a27aa0, fn_00a27aa0(u32, u32)),
        entry!(0x00a28620, fn_00a28620(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_registration_runs_once_and_stores_through_its_helper() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 1);
        assert_eq!(e.mem.u8(0x011f_3656), 1);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0x00e7_cd40);
        assert_eq!(e.mem.u8(0x011f_375e), 0x10);
        // The helper with row 1, column 0: (1 * 6 + 0) * 4 = 0x18 into
        // each of its tables.
        assert_eq!(e.mem.u32(0x011f_3d20 + 0x18), 0x00a2_91e0);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x18), 0);
        // Once only: the guard is set, later calls answer 0 and store nothing.
        e.mem.set_u32(0x011f_3dc8, 0);
        assert_eq!(e.call(0x00a27b00, &[]).u32(), 0);
        assert_eq!(e.mem.u32(0x011f_3dc8), 0);
    }

    #[test]
    fn the_helper_indexes_by_row_and_column() {
        let mut e = Engine::new();
        e.map(0x011f_3600, 0x900);
        e.mem.set_u32(0x011f_3810 + 0x2c, 0x5555);
        e.call(0x00a27aa0, &[1, 5]);
        // (1 * 6 + 5) * 4 = 0x2c.
        assert_eq!(e.mem.u32(0x011f_3c90 + 0x2c), 0x00a2_7470);
        assert_eq!(e.mem.u32(0x011f_3810 + 0x2c), 0);
    }
}
