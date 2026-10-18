//! `NiMain/nifloatsextradata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatsExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_RTTI: u32 = 0x011f_5b60;

// Translated from 00a89120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a89120, ni_floats_extra_data_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a89120, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_RTTI
        );
    }
}
