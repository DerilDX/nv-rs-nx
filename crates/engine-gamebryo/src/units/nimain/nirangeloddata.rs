//! `NiMain/nirangeloddata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRangeLODData::ms_RTTI` (the class's `NiRTTI` record).
const NI_RANGE_LOD_DATA_RTTI: u32 = 0x011f_5ef8;

// Translated from 00a976f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_range_lod_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RANGE_LOD_DATA_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a976f0, ni_range_lod_data_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_range_lod_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a976f0, &args![]).u32(), NI_RANGE_LOD_DATA_RTTI);
    }
}
