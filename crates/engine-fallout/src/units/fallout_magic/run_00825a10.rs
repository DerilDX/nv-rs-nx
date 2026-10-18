//! `fallout/magic/run_00825a10` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00825c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x14` of `this` (a plain field getter).
pub fn fn_00825c00(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x14)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00825c00, fn_00825c00(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0x14() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 0x14, 0x1234_5678);
        assert_eq!(e.call(0x0082_5c00, &args![object]).u32(), 0x1234_5678);
    }
}
