//! `fallout/ai/modifierlist.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009373f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `this`. The engine map has no name for it; the quest stage
/// item's `InitItem` (`0060fa10`) calls it.
pub fn fn_009373f0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr())
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009373f0, fn_009373f0(Ptr) -> u8)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_first_byte() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x1122_33a5);
        assert_eq!(e.call(0x0093_73f0, &[object]).u32() & 0xff, 0xa5);
    }
}
