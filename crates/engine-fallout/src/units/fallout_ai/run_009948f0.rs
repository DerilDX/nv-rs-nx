//! `fallout/ai/run_009948f0` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00994ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` and one stack word, `RET 4`): stores `value` in the
/// word at +0xa4.
pub fn fn_00994ef0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xa4, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0099_4ef0, fn_00994ef0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00994ef0_stores_the_word() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xac);
        e.call(0x0099_4ef0, &args![object, 0xcafe_f00du32]);
        assert_eq!(e.mem.u32(object + 0xa4), 0xcafe_f00d);
        assert_eq!(e.mem.u32(object + 0xa0), 0);
        assert_eq!(e.mem.u32(object + 0xa8), 0);
    }
}
