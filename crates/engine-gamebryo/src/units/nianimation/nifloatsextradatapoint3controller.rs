//! `NiAnimation/nifloatsextradatapoint3controller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatsExtraDataPoint3Controller::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI: u32 = 0x011f_3fc4;

// Translated from 00a4eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_floats_extra_data_point3_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI)
}

/// `00a57630`: the jump target of `00a4eaf0`.
const JUMP_TARGET_00A4EAF0: u32 = 0x00a5_7630;

// Translated from 00a4eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatsExtraDataPoint3Controller::LinkObject` (Xbox PDB): a 5-byte jump to `00a57630` (engine map: `NiPSysModifierCtlr::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiFloatsExtraDataPoint3Controller@@UAAXAAVNiStream@@@Z`.
pub fn ni_floats_extra_data_point3_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A4EAF0, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4eb40, ni_floats_extra_data_point3_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4_eaf0,
            ni_floats_extra_data_point3_controller_link_object(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_floats_extra_data_point3_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4eb40, &args![]).u32(),
            NI_FLOATS_EXTRA_DATA_POINT3_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4eaf0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_eaf0;
        const TARGET: u32 = 0x00a5_7630;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}
