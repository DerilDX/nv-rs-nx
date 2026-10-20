//! `fallout shared/run_00518f80` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00519020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`fallout shared`; the class is unconfirmed). `thiscall`, `RET 4`:
/// stores its stack word at `+0x688` of `this`, a plain 32-bit move (the word is not loaded as a
/// float).
pub fn fn_00519020(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x688), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00519020, fn_00519020(Ptr, u32))]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00519020_stores_the_word_at_offset_0x688() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x700);
        e.mem.set_u32(this + 0x684, 0x1111_1111);
        e.mem.set_u32(this + 0x68c, 0x2222_2222);
        e.call(0x0051_9020, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this + 0x688), 0xdead_beef);
        assert_eq!(e.mem.u32(this + 0x684), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x68c), 0x2222_2222);
    }
}
