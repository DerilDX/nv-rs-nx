//! `NiAnimation/bsmultitargettreadtransfcontroller.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSMultiTargetTreadTransfController::ms_RTTI` (the class's `NiRTTI` record).
const BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI: u32 = 0x011f_36e4;

// Translated from 00a35950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_multi_target_tread_transf_controller_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI)
}

/// `00a2f910`: the jump target of `00a35840`.
const JUMP_TARGET_00A35840: u32 = 0x00a2_f910;

/// `00c27120`: the jump target of `00a35870`.
const JUMP_TARGET_00A35870: u32 = 0x00c2_7120;

/// `00c283f0`: the jump target of `00a35880`.
const JUMP_TARGET_00A35880: u32 = 0x00c2_83f0;

// Translated from 00a35840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::LoadBinary` (Xbox PDB): a 5-byte jump to `00a2f910` (engine map: `NiInterpController::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSMultiTargetTreadTransfController@@UAAXAAVNiStream@@@Z`.
pub fn bs_multi_target_tread_transf_controller_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35840, &args![this, stream]);
}

// Translated from 00a35870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::SaveBinary` (Xbox PDB): a 5-byte jump to `00c27120` (engine map: `NiPSysResetOnLoopCtlr::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSMultiTargetTreadTransfController@@UAAXAAVNiStream@@@Z`.
pub fn bs_multi_target_tread_transf_controller_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A35870, &args![this, stream]);
}

// Translated from 00a35880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSMultiTargetTreadTransfController::IsEqual` (Xbox PDB): a 5-byte jump to `00c283f0` (engine map: `NiPSysResetOnLoopCtlr::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSMultiTargetTreadTransfController@@UAA_NPAVNiObject@@@Z`.
pub fn bs_multi_target_tread_transf_controller_is_equal(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A35880, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a35950, bs_multi_target_tread_transf_controller_get_rtti() -> Ptr),
        entry!(
            0x00a3_5840,
            bs_multi_target_tread_transf_controller_load_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a3_5870,
            bs_multi_target_tread_transf_controller_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a3_5880, bs_multi_target_tread_transf_controller_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_multi_target_tread_transf_controller_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a35950, &args![]).u32(),
            BS_MULTI_TARGET_TREAD_TRANSF_CONTROLLER_RTTI
        );
    }

    #[test]
    fn thunk_00a35840_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5840;
        const TARGET: u32 = 0x00a2_f910;
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
    fn thunk_00a35870_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a3_5870;
        const TARGET: u32 = 0x00c2_7120;
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
    fn thunk_00a35880_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a3_5880;
        const TARGET: u32 = 0x00c2_83f0;
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
