//! `BSMain/bswarray.obj` (Xbox PDB source unit), subsystem `BSMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiIntegersExtraData::ms_RTTI` (the class's `NiRTTI` record).
const NI_INTEGERS_EXTRA_DATA_RTTI: u32 = 0x011f_5f24;

// Translated from 00a99940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiIntegersExtraData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_integers_extra_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_INTEGERS_EXTRA_DATA_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a99940, ni_integers_extra_data_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_integers_extra_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a99940, &args![]).u32(),
            NI_INTEGERS_EXTRA_DATA_RTTI
        );
    }
}
