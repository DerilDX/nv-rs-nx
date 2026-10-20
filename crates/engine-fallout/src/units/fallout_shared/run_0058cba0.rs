//! `fallout shared/run_0058cba0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0058cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (callers include `HitMe` and `00403e60`; the class is not
/// confirmed): whether any bit of `mask` is set in the word at
/// `this + 0x58` (`NEG`/`SBB`/`NEG`: 1 or 0 in `EAX`); `RET 4`.
pub fn fn_0058cba0(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    e.mem.u32(this.addr() + 0x58) & mask != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0058_cba0, fn_0058cba0(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_bit_of_the_mask_in_the_word_at_0x58() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.mem.set_u32(this + 0x58, 0x8000_0010);
        for (mask, expected) in [(0x10, 1), (0x8000_0000, 1), (0x0f, 0), (0, 0)] {
            assert_eq!(e.call(0x0058_cba0, &[this, mask]).u32(), expected);
        }
    }
}
