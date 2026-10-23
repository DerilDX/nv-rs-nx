//! `fallout/magic/run_008070d0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00807770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: an index): the address of element
/// `index` of an array of 0x70-byte elements whose base is the word at `this`
/// (`base + index * 0x70`, 32-bit wrap-around).
pub fn fn_00807770(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x70)
        .wrapping_add(e.mem.u32(this.addr()))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_7770, fn_00807770(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_0x70_byte_elements() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0010_0000);
        assert_eq!(e.call(0x0080_7770, &[this, 0]).u32(), 0x0010_0000);
        assert_eq!(e.call(0x0080_7770, &[this, 3]).u32(), 0x0010_0150);
        assert_eq!(e.call(0x0080_7770, &[this, 0xffff_ffff]).u32(), 0x000f_ff90);
    }
}
