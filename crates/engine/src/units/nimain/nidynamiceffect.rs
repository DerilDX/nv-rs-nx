//! `NiMain/nidynamiceffect.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDynamicEffect::ms_RTTI` (the class's `NiRTTI` record).
const NI_DYNAMIC_EFFECT_RTTI: u32 = 0x011f_4988;

// Translated from 00a69060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDynamicEffect::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dynamic_effect_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DYNAMIC_EFFECT_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a69060, ni_dynamic_effect_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dynamic_effect_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a69060, &args![]).u32(), NI_DYNAMIC_EFFECT_RTTI);
    }
}
