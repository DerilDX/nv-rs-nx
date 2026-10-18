//! `fallout/interface/bgsmenupacker.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

use crate::prelude::*;

// Translated from 007037c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x24. The PC body is shared by otherwise unrelated
/// callers, including BGSLoadFormBuffer::SetForm (Xbox PDB); the map's
/// BGSMenuPacker::RecomputePacking (Xbox PDB) name does not describe this store.
pub fn fn_007037c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x24), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007037c0, fn_007037c0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_setter_keeps_neighbors_and_accepts_all_word_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x2c);
        e.mem.write(object, &[0xa5; 0x2c]);
        for value in [0, 0x2000_0040, u32::MAX] {
            e.call(0x007037c0, &[object, value]);
            assert_eq!(e.mem.u32(object + 0x24), value);
            assert_eq!(e.mem.bytes(object, 0x24), vec![0xa5; 0x24]);
            assert_eq!(e.mem.u32(object + 0x28), 0xa5a5_a5a5);
        }
    }
}
