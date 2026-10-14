//! `fallout shared/tesspelllist.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0048d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this + 0xC` (`thiscall`, no arguments): the address of a member
/// at `+0xC`. The engine map gives this folded body no name; its many
/// callers use it on different classes (in `TESSpellList` that offset is
/// `leveledSpellList`, Xbox PDB).
pub fn fn_0048d150(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0xc))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0048d150, fn_0048d150(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_member_at_c() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x0048_d150, &args![0x0000_5000u32]).u32(),
            0x0000_500c
        );
    }
}
