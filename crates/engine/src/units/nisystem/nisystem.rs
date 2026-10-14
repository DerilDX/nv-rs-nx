//! `NiSystem/nisystem.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__stricmp` (LIBCMT, `00ec68e4`).
const STRICMP: u32 = 0x00ec_68e4;

// Translated from 00aa1360 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `NiStricmp` (Xbox PDB; `cdecl`): a jump to `__stricmp`.
pub fn ni_stricmp(e: &mut Engine, a: u32, b: u32) -> i32 {
    e.call(STRICMP, &args![a, b]).u32() as i32
}

// Translated from 00ec1c90 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// A string map's key comparison (`thiscall`, `RET 8`; `this` unused; the
/// ledger files it under `BSMovie/movieplayer_xenon.obj`): whether the two
/// keys are equal ignoring case ([`ni_stricmp`] answers 0), as 1 or 0.
pub fn fn_00ec1c90(e: &mut Engine, _this: Ptr, a: u32, b: u32) -> bool {
    ni_stricmp(e, a, b) == 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa1360, ni_stricmp(u32, u32) -> i32),
        entry!(0x00ec1c90, fn_00ec1c90(Ptr, u32, u32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_compare_equal_ignoring_case() {
        let mut e = Engine::new();
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.write(a, b"fAbc\0");
        e.mem.write(b, b"FabC\0");
        assert_eq!(e.call(0x00ec1c90, &[0, a, b]).u32() & 0xff, 1);
        e.mem.write(b, b"Fabd\0");
        assert_eq!(e.call(0x00ec1c90, &[0, a, b]).u32() & 0xff, 0);
        assert!((e.call(0x00aa1360, &[a, b]).u32() as i32) < 0);
    }
}
