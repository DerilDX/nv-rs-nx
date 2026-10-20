//! `fallout shared/pathfinding/run_006bc870` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bc870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies the word at `+0`, the half-word at `+4` and the
/// word at `+8` of `other` into `this` and returns `this`.
pub fn fn_006bc870(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let first = e.mem.u32(other.addr());
    e.mem.set_u32(this.addr(), first);
    let second = e.mem.u16(other.addr().wrapping_add(4));
    e.mem.set_u16(this.addr().wrapping_add(4), second);
    let third = e.mem.u32(other.addr().wrapping_add(8));
    e.mem.set_u32(this.addr().wrapping_add(8), third);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006b_c870, fn_006bc870(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod b035_tests {
    use super::*;

    #[test]
    fn fn_006bc870_copies_two_words_and_a_half_word() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        let other = e.mem.alloc(0xc);
        e.mem.write(this, &[0xee; 0xc]);
        for (offset, value) in [(0u32, 0x1111_2222u32), (4, 0x3333_4444), (8, 0x5555_6666)] {
            e.mem.set_u32(other + offset, value);
        }
        assert_eq!(e.call(0x006b_c870, &args![this, other]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x1111_2222);
        assert_eq!(e.mem.u32(this + 4), 0xeeee_4444);
        assert_eq!(e.mem.u32(this + 8), 0x5555_6666);
    }
}
