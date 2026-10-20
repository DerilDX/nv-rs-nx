//! `fallout/ai/run_008ce180` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008ce390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x8C (`thiscall`; no Xbox PDB name).
pub fn fn_008ce390(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x8c)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c_e390, fn_008ce390(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_word_at_0x8c() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x8c, 0xdead_beef);
        assert_eq!(e.call(0x008c_e390, &args![this]).u32(), 0xdead_beef);
    }
}
