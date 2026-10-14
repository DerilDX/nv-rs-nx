//! `fallout/misc/saveload/bgsunloadedformbuffer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00841100 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// ORs `flags` into the change-flags word at `this` (`thiscall`, `RET 4`);
/// `BGSSaveLoadChangesMap::AddChangeFlags` (`0084585d`) adds a form's change
/// flags with it. (The engine map's name for this body,
/// `BGSUnloadedFormBuffer::BGSUnloadedFormBuffer_ov2`, comes from folded
/// code.)
pub fn fn_00841100(e: &mut Engine, this: Ptr, flags: u32) {
    let word = e.mem.u32(this.addr());
    e.mem.set_u32(this.addr(), word | flags);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00841100, fn_00841100(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ors_the_flags_into_the_word() {
        let mut e = Engine::new();
        let w = e.mem.alloc(4);
        e.mem.set_u32(w, 0x0000_0101);
        e.call(0x00841100, &[w, 0x4000_0001]);
        assert_eq!(e.mem.u32(w), 0x4000_0101);
    }
}
