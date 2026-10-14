//! `fallout/ai/combat/combatprocedurebeincover.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 009d2510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers 7 (`ECX` not read, no stack words). The map names it
/// `CombatProcedureBeInCover::GetType` (through that class's vtable), but
/// the body is shared by identical-code folding: several vtables point at
/// it, and `TESAttributes`' save and load (`00480100`, `00480130`) call it
/// for the size of the attribute data.
pub fn fn_009d2510(_e: &mut Engine, _this: Ptr) -> u32 {
    7
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x009d2510, fn_009d2510(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_seven() {
        let mut e = Engine::new();
        let back = e.call(0x009d_2510, &args![Ptr::<()>::new(0x1234)]).u32();
        assert_eq!(back, 7);
    }
}
