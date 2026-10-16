//! `NiMain/niscreenelementsdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiScreenElementsData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_ELEMENTS_DATA_RTTI: u32 = 0x011f_5b50;

// Translated from 00a87a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenElementsData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_elements_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_ELEMENTS_DATA_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a87a90, ni_screen_elements_data_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_elements_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a87a90, &args![]).u32(),
            NI_SCREEN_ELEMENTS_DATA_RTTI
        );
    }
}
