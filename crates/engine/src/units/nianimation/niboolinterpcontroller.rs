//! `NiAnimation/niboolinterpcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolInterpController::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_INTERP_CONTROLLER_RTTI: u32 = 0x011f_4228;

// Translated from 00a577b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_interp_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_INTERP_CONTROLLER_RTTI)
}

/// `00a39010`: the jump target of `00a57c00`.
const JUMP_TARGET_00A57C00: u32 = 0x00a3_9010;

/// `00a39030`: the jump target of `00a57c10`.
const JUMP_TARGET_00A57C10: u32 = 0x00a3_9030;

// Translated from 00a57c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a39010` (engine map: `NiSingleInterpController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiBoolInterpController@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_interp_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A57C00, &args![this, stream]);
}

// Translated from 00a57c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolInterpController::IsEqual` (Xbox PDB): a 5-byte jump to `00a39030` (engine map: `NiSingleInterpController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiBoolInterpController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_bool_interp_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A57C10, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a577b0, ni_bool_interp_controller_get_rtti() -> Ptr),
        entry!(0x00a5_7c00, ni_bool_interp_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a5_7c10, ni_bool_interp_controller_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_interp_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a577b0, &args![]).u32(),
            NI_BOOL_INTERP_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a57c00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_7c00;
        const TARGET: u32 = 0x00a3_9010;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn thunk_00a57c10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_7c10;
        const TARGET: u32 = 0x00a3_9030;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }
}
