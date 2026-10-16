//! `NiAnimation/nifloatinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4220;

// Translated from 00a57660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_INTERP_CONTROLLER_RTTI)
}

/// `00a38f80`: the jump target of `00a57c70`.
const JUMP_TARGET_00A57C70: u32 = 0x00a3_8f80;

// Translated from 00a57c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatInterpController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a38f80` (engine map: `NiSingleInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiFloatInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_float_interp_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57C70, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a57660, ni_float_interp_controller_get_rtti() -> Ptr),
        entry!(
            0x00a5_7c70,
            ni_float_interp_controller_load_binary(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a57660, &args![]).u32(),
            NI_FLOAT_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a57c70_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7c70;
        const TARGET: u32 = 0x00a3_8f80;
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
