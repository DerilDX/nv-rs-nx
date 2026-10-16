//! `NiAnimation/nirollcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiFloatController::ms_RTTI` (the class's `NiRTTI` record).
const NI_FLOAT_CONTROLLER_RTTI: u32 = 0x011f_4230;

/// `NiRollController::ms_RTTI` (the class's `NiRTTI` record).
const NI_ROLL_CONTROLLER_RTTI: u32 = 0x011f_3f58;

// Translated from 00a485a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiFloatController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_float_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_FLOAT_CONTROLLER_RTTI)
}

// Translated from 00a485b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_roll_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_ROLL_CONTROLLER_RTTI)
}

/// `00a57970`: the jump target of `00a48550`.
const JUMP_TARGET_00A48550: u32 = 0x00a5_7970;

/// `00a57a70`: the jump target of `00a48560`.
const JUMP_TARGET_00A48560: u32 = 0x00a5_7a70;

/// `00a57ad0`: the jump target of `00a48570`.
const JUMP_TARGET_00A48570: u32 = 0x00a5_7ad0;

/// `00a57b00`: the jump target of `00a48580`.
const JUMP_TARGET_00A48580: u32 = 0x00a5_7b00;

/// `00a57b20`: the jump target of `00a48590`.
const JUMP_TARGET_00A48590: u32 = 0x00a5_7b20;

// Translated from 00a48550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a57970` (engine map: `NiFloatController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48550, &args![this, stream]);
}

// Translated from 00a48560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::LinkObject` (Xbox PDB): a 5-byte jump to `00a57a70` (engine map: `NiFloatController::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48560, &args![this, stream]);
}

// Translated from 00a48570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a57ad0` (engine map: `NiFloatController::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiRollController@@UAA_NAAVNiStream@@@Z`.
pub fn ni_roll_controller_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A48570, &args![this, stream]).bool()
}

// Translated from 00a48580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::SaveBinary` (Xbox PDB): a 5-byte jump to `00a57b00` (engine map: `NiFloatController::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiRollController@@UAAXAAVNiStream@@@Z`.
pub fn ni_roll_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A48580, &args![this, stream]);
}

// Translated from 00a48590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRollController::IsEqual` (Xbox PDB): a 5-byte jump to `00a57b20` (engine map: `NiFloatController::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiRollController@@UAA_NPAVNiObject@@@Z`.
pub fn ni_roll_controller_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A48590, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a485a0, ni_float_controller_get_rtti() -> Ptr),
        entry!(0x00a485b0, ni_roll_controller_get_rtti() -> Ptr),
        entry!(0x00a4_8550, ni_roll_controller_load_binary(Ptr, Ptr)),
        entry!(0x00a4_8560, ni_roll_controller_link_object(Ptr, Ptr)),
        entry!(0x00a4_8570, ni_roll_controller_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a4_8580, ni_roll_controller_save_binary(Ptr, Ptr)),
        entry!(0x00a4_8590, ni_roll_controller_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_float_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a485a0, &args![]).u32(), NI_FLOAT_CONTROLLER_RTTI);
    }

    #[test]
    fn ni_roll_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a485b0, &args![]).u32(), NI_ROLL_CONTROLLER_RTTI);
    }

    #[test]
    fn thunk_00a48550_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8550;
        const TARGET: u32 = 0x00a5_7970;
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
    fn thunk_00a48560_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8560;
        const TARGET: u32 = 0x00a5_7a70;
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
    fn thunk_00a48570_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_8570;
        const TARGET: u32 = 0x00a5_7ad0;
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

    #[test]
    fn thunk_00a48580_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_8580;
        const TARGET: u32 = 0x00a5_7b00;
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
    fn thunk_00a48590_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_8590;
        const TARGET: u32 = 0x00a5_7b20;
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
