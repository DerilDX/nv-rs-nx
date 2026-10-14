//! `fallout/ai/actorpackage.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 008c6060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores 0 in the word at `this + 4`. The engine map names it
/// `ActorPackage::NullInstanceData` (Xbox PDB), but the body is shared by
/// identical-code folding (constructors such as `00487420` call it on their
/// own objects, as do `00475350`, `0048e7a0`, ...), so it is named by
/// address.
pub fn fn_008c6060(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x008c6060, fn_008c6060(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clears_only_the_word_at_4() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0xc);
        for offset in [0, 4, 8] {
            e.mem.set_u32(object + offset, 0xdead_0000 + offset);
        }
        e.call(0x008c_6060, &[object]);
        assert_eq!(e.mem.u32(object), 0xdead_0000);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0xdead_0008);
    }
}
