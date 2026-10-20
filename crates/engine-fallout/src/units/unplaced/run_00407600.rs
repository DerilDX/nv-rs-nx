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

// Translated from 004077e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the member at `+0xA0` of `this` (`this + 0xA0`); the
/// engine map has no name and the class is unconfirmed.
pub fn fn_004077e0(_e: &mut Engine, this: Ptr) -> Ptr {
    this.byte_add(0xa0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004077c0, fn_004077c0(Ptr) -> bool),
        entry!(0x0040_77e0, fn_004077e0(Ptr) -> Ptr),
    ]
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

#[cfg(test)]
mod member_tests {
    use super::*;

    #[test]
    fn fn_004077e0_adds_0xa0_to_this() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0040_77e0, &args![0x1000u32]).u32(), 0x10a0);
        assert_eq!(e.call(0x0040_77e0, &args![0xffff_ff80u32]).u32(), 0x20);
    }
}
