//! `NiMain/niditherproperty.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiDitherProperty::ms_RTTI` (the class's `NiRTTI` record).
const NI_DITHER_PROPERTY_RTTI: u32 = 0x011f_5f48;

// Translated from 00a9a5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiDitherProperty::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_dither_property_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_DITHER_PROPERTY_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a9a5e0, ni_dither_property_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_dither_property_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a9a5e0, &args![]).u32(), NI_DITHER_PROPERTY_RTTI);
    }
}
