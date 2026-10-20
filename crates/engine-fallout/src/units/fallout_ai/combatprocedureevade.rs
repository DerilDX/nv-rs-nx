//! `fallout/ai/combat/combatprocedureevade.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0081b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 4 (`thiscall`, `this` unread). The engine map names it
/// `CombatProcedureEvade::GetType`; the body is shared by the linker
/// (several unrelated vtables point to it), so it is named by address.
pub fn fn_0081b6c0(_e: &mut Engine, _this: Ptr) -> u32 {
    4
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0081_b6c0, fn_0081b6c0(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0081b6c0_returns_four() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0081_b6c0, &args![0x1234u32]).u32(), 4);
    }
}
