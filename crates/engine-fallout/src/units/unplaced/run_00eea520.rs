//! `(unplaced)/run_00eea520` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00eea530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A getter (unnamed in the map; `thiscall`, no arguments; referenced from two vtables at
/// `010af3b4` and `010af81c`): the word at `this + 0x28`.
pub fn fn_00eea530(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(0x28))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00eea530, fn_00eea530(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00eea530_reads_the_word_at_0x28() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x2c);
        e.mem.set_u32(this + 0x28, 0xabcd);
        assert_eq!(e.call(0x00eea530, &args![this]).u32(), 0xabcd);
    }
}
