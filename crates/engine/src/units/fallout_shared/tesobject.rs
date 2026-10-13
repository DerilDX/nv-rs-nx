//! `fallout shared/tesobject.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0050f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a word at +0x20 (`RET 4`). The PC body is shared by otherwise
/// unrelated callers; the inventory buffer save `004d4090` uses it to switch
/// a save buffer's mode word, as `007037c0` does for the word at +0x24 when
/// loading.
pub fn fn_0050f9c0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(0x20), value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0050f9c0, fn_0050f9c0(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_setter_keeps_neighbors_and_accepts_all_word_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x28);
        e.mem.write(object, &[0xa5; 0x28]);
        for value in [0, 0x2000_0040, u32::MAX] {
            e.call(0x0050f9c0, &[object, value]);
            assert_eq!(e.mem.u32(object + 0x20), value);
            assert_eq!(e.mem.bytes(object, 0x20), vec![0xa5; 0x20]);
            assert_eq!(e.mem.u32(object + 0x24), 0xa5a5_a5a5);
        }
    }
}
