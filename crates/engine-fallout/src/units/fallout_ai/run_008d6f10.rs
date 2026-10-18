//! `fallout/ai/run_008d6f10` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008d6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `+0x40` of `this` (a plain field getter).
pub fn fn_008d6f30(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x40)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008d6f30, fn_008d6f30(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0x40() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u32(object.addr() + 0x40, 0x1234_5678);
        assert_eq!(e.call(0x008d_6f30, &args![object]).u32(), 0x1234_5678);
    }
}
