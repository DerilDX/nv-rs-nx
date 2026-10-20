//! `fallout shared/settings/xgamesetting.cpp` (Xbox PDB source unit), subsystem `fallout shared/settings`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004f8960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the class is unconfirmed). `__fastcall` getter: the 32-bit word at
/// `+0x108` of `this`.
pub fn fn_004f8960(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x108))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f8960, fn_004f8960(Ptr) -> u32)]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_004f8960_answers_the_word_at_offset_0x108() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        e.mem.set_u32(this + 0x104, 0x1111_1111);
        e.mem.set_u32(this + 0x108, 0xcafe_f00d);
        e.mem.set_u32(this + 0x10c, 0x2222_2222);
        assert_eq!(e.call(0x004f_8960, &args![this]).u32(), 0xcafe_f00d);
    }
}
