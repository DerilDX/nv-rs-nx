//! `fallout/interface/menumanager.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00726070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the word at +4. Folded callers include a list's next-node pointer
/// and an inventory entry's signed count; the accessor preserves all bits.
pub fn word_at_four(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr().wrapping_add(4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00726070, word_at_four(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Three generated vectors match native 00726070 on the executable/CPU
    // configuration recorded in docs/ENGINE_STATE.md, including untouched guards.
    fn folded_word_getter_preserves_next_pointer_or_signed_count_bits() {
        let mut e = Engine::new();
        let object = e.mem.alloc(12);
        for word in [0u32, 0x1234_5678, (-2i32) as u32] {
            e.mem.set_u32(object, 0xaaaa_aaaa);
            e.mem.set_u32(object + 4, word);
            e.mem.set_u32(object + 8, 0xbbbb_bbbb);
            let before = e.mem.bytes(object, 12);
            assert_eq!(e.call(0x00726070, &args![object]).u32(), word);
            assert_eq!(e.mem.bytes(object, 12), before);
        }
    }
}
