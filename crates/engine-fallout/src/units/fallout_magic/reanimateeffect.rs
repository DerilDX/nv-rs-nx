//! `fallout/magic/reanimateeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Sets (`on`) or clears `bit` of the word at `this + 0x1c`.
fn set_flag(e: &mut Engine, this: Ptr, bit: u32, on: bool) {
    let flags = e.mem.u32(this.addr() + 0x1c);
    let flags = if on { flags | bit } else { flags & !bit };
    e.mem.set_u32(this.addr() + 0x1c, flags);
}

// Translated from 00826b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a `bool`): sets bit 1 (value 2) of the
/// word at `this + 0x1c` when the flag is set, clears it otherwise. What the bit means is
/// not confirmed.
pub fn fn_00826b40(e: &mut Engine, this: Ptr, on: bool) {
    set_flag(e, this, 2, on);
}

// Translated from 00826b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: a `bool`): sets bit 2 (value 4) of the
/// word at `this + 0x1c` when the flag is set, clears it otherwise. What the bit means is
/// not confirmed.
pub fn fn_00826b90(e: &mut Engine, this: Ptr, on: bool) {
    set_flag(e, this, 4, on);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_6b40, fn_00826b40(Ptr, bool)),
        entry!(0x0082_6b90, fn_00826b90(Ptr, bool)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(address: u32, bit: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x1c, 0x8000_0001);
        e.call(address, &[this, 1]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0001 | bit);
        // Only the low byte of the flag word counts.
        e.call(address, &[this, 0x100]);
        assert_eq!(e.mem.u32(this + 0x1c), 0x8000_0001);
        e.mem.set_u32(this + 0x1c, 0xffff_ffff);
        e.call(address, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x1c), !bit);
    }

    #[test]
    fn fn_00826b40_sets_or_clears_bit_value_2() {
        check(0x0082_6b40, 2);
    }

    #[test]
    fn fn_00826b90_sets_or_clears_bit_value_4() {
        check(0x0082_6b90, 4);
    }
}
