//! `(unplaced)/run_00502cd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Byte-swaps the four bytes at an address in place (`00401080`, cdecl:
/// the address, a flag byte).
const SWAP_4_BYTES: u32 = 0x0040_1080;

// Translated from 00503210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the word at `this` in place: `00401080` with `this` and a
/// zero flag byte. The quest target loader (`00610120`) calls it.
pub fn fn_00503210(e: &mut Engine, this: Ptr) {
    e.call(SWAP_4_BYTES, &args![this, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00503210, fn_00503210(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swaps_this_with_a_zero_flag() {
        let mut e = Engine::new();
        e.register(SWAP_4_BYTES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_3210, &[0x4000]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(SWAP_4_BYTES, vec![0x4000, 0])]);
    }
}
