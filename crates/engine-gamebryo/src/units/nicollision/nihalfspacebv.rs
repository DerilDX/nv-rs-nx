//! `NiCollision/nihalfspacebv.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e9e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nihalfspacebv.cpp`): a virtual slot (referenced only
/// from the vtables at `0109c5a0`, `010cb3d0` and `010f1734`, so `this` arrives in `ECX`
/// and is not read) that answers 5. Shared by identical-code folding; which method of
/// which class each slot is was not confirmed.
pub fn fn_00e9e670(_e: &mut Engine, _this: Ptr) -> u32 {
    5
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e9_e670, fn_00e9e670(Ptr) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_five() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00e9_e670, &args![0x5000u32]).u32(), 5);
    }
}
