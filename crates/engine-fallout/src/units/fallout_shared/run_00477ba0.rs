//! `fallout shared/run_00477ba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00477ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether bit 0x00800000 of the flag word at
/// `this + 8` is set.
pub fn fn_00477ba0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr().wrapping_add(8)) & 0x0080_0000 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0047_7ba0, fn_00477ba0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00477ba0_tests_bit_23_of_the_word_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this + 8, 0xff7f_ffff);
        assert!(!e.call(0x0047_7ba0, &args![this]).bool());
        e.mem.set_u32(this + 8, 0x0080_0000);
        assert!(e.call(0x0047_7ba0, &args![this]).bool());
    }
}
