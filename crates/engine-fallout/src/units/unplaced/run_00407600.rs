//! `(unplaced)/run_00407600` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004077c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 14 (`0x4000`) of the flag word at `+8` of `this` is set (a
/// plain flag getter).
pub fn fn_004077c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 8) & 0x4000 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004077c0, fn_004077c0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_0x4000_of_the_word_at_8() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x10));
        // Every other bit set: still false.
        e.mem.set_u32(object.addr() + 8, !0x4000);
        assert!(!e.call(0x0040_77c0, &args![object]).bool());
        e.mem.set_u32(object.addr() + 8, 0x4000);
        assert!(e.call(0x0040_77c0, &args![object]).bool());
    }
}
