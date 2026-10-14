//! `(unplaced)/run_00ec36a6` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ec3bda (decompiled, FalloutNV.exe 1.4.0.525)
/// Swaps the two bytes of the low half (unnamed in the map; a runtime
/// library function, `cdecl`, one word; the half-word swap `00407a90`
/// calls it). The whole of `EAX` is the result: the low half is the swapped
/// half (`(value << 8) + (value >> 8)` in 16 bits), the high half is bits 8
/// to 23 of the argument, which callers ignore.
pub fn fn_00ec3bda(_e: &mut Engine, value: u32) -> u32 {
    let shifted = value << 8;
    let low = (shifted as u16).wrapping_add((value as u16) >> 8);
    (shifted & 0xffff_0000) | low as u32
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ec3bda, fn_00ec3bda(u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_the_low_half_and_keeps_the_shifted_high_half() {
        let mut e = Engine::new();
        let mut swap = |value: u32| e.call(0x00ec_3bda, &args![value]).u32();
        assert_eq!(swap(0x0000_1234), 0x0012_3412);
        assert_eq!(swap(0x0000_ff01), 0x00ff_01ff);
        assert_eq!(swap(0xabcd_1234), 0xcd12_3412);
        assert_eq!(swap(0), 0);
        assert_eq!(swap(0x0000_00ff) as u16, 0xff00);
    }
}
