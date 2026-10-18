//! `NiAnimation/nilightdimmercontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiLightDimmerController::ms_RTTI` (the class's `NiRTTI` record).
const NI_LIGHT_DIMMER_CONTROLLER_RTTI: u32 = 0x011f_3fa0;

// Translated from 00a4cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_light_dimmer_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_LIGHT_DIMMER_CONTROLLER_RTTI)
}

/// `00a57c70`: the jump target of `00a4cf40`.
const JUMP_TARGET_00A4CF40: u32 = 0x00a5_7c70;

// Translated from 00a4cf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiLightDimmerController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a57c70` (engine map: `NiFloatInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiLightDimmerController@@UAAXAAVNiStream@@@Z`.
pub fn ni_light_dimmer_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A4CF40, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a4cf90, ni_light_dimmer_controller_get_rtti() -> Ptr),
        entry!(
            0x00a4_cf40,
            ni_light_dimmer_controller_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_light_dimmer_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a4cf90, &args![]).u32(),
            NI_LIGHT_DIMMER_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a4cf40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_cf40;
        const TARGET: u32 = 0x00a5_7c70;
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
