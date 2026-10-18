//! `NiAnimation/niextradatacontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiExtraDataController::ms_RTTI` (the class's `NiRTTI` record).
const NI_EXTRA_DATA_CONTROLLER_RTTI: u32 = 0x011f_4218;

// Translated from 00a57290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiExtraDataController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_extra_data_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_EXTRA_DATA_CONTROLLER_RTTI)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a57290, ni_extra_data_controller_get_rtti() -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_extra_data_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57290, &args![]).u32(),
            NI_EXTRA_DATA_CONTROLLER_RTTI
        );
    }
}
