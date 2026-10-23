//! `fallout/interface/menus/quantitymenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 007abdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether all six tile slots at +0x28 (one word each) are set: false at
/// the first null slot. The menu class is not confirmed here.
pub fn fn_007abdb0(e: &mut Engine, this: Ptr) -> bool {
    let mut index = 0u32;
    while index < 6 {
        let slot = this.addr().wrapping_add(0x28).wrapping_add(index * 4);
        if e.mem.u32(slot) == 0 {
            return false;
        }
        index += 1;
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007a_bdb0, fn_007abdb0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn true_only_with_all_six_tiles() {
        let mut e = Engine::new();
        let menu = e.mem.alloc(0x48);
        for i in 0..5u32 {
            e.mem.set_u32(menu + 0x28 + i * 4, 1);
        }
        assert!(!e.call(0x007a_bdb0, &args![menu]).bool());
        e.mem.set_u32(menu + 0x3c, 1);
        assert!(e.call(0x007a_bdb0, &args![menu]).bool());
        e.mem.set_u32(menu + 0x28, 0);
        assert!(!e.call(0x007a_bdb0, &args![menu]).bool());
    }
}
