//! `fallout shared/run_00590d00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00590d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the flag word at `+0x38` of `this` is set. `ShowMessage`
/// calls it on a message form to tell a message box from a plain message;
/// the engine map has no name for it.
pub fn fn_00590d00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x38) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00590d00, fn_00590d00(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_bit_zero_of_the_word_at_0x38() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0b10);
        assert!(!e.call(0x0059_0d00, &args![object]).bool());
        e.mem.set_u32(object + 0x38, 0xffff_ffff);
        assert!(e.call(0x0059_0d00, &args![object]).bool());
    }
}
