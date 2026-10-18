//! `NiAnimation/bsrotaccumtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSRotAccumTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ee0;

// Translated from 00a43ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_rot_accum_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI)
}

/// `00a3f7b0`: the jump target of `00a43a70`.
const JUMP_TARGET_00A43A70: u32 = 0x00a3_f7b0;

/// `00a3f870`: the jump target of `00a43a80`.
const JUMP_TARGET_00A43A80: u32 = 0x00a3_f870;

/// `00a3f8a0`: the jump target of `00a43a90`.
const JUMP_TARGET_00A43A90: u32 = 0x00a3_f8a0;

/// `00a3f8d0`: the jump target of `00a43aa0`.
const JUMP_TARGET_00A43AA0: u32 = 0x00a3_f8d0;

// Translated from 00a43a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a3f7b0` (engine map: `NiTransformInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSRotAccumTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43A70, &args![this, stream]);
}

// Translated from 00a43a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a3f870` (engine map: `NiTransformInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSRotAccumTransfInterpolator@@UAA_NAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A43A80, &args![this, stream]).bool()
}

// Translated from 00a43a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a3f8a0` (engine map: `NiTransformInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSRotAccumTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_rot_accum_transf_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43A90, &args![this, stream]);
}

// Translated from 00a43aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRotAccumTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a3f8d0` (engine map: `NiTransformInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSRotAccumTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_rot_accum_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A43AA0, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a43ab0, bs_rot_accum_transf_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_3a70,
            bs_rot_accum_transf_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_3a80, bs_rot_accum_transf_interpolator_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_3a90,
            bs_rot_accum_transf_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_3aa0, bs_rot_accum_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_rot_accum_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a43ab0, &args![]).u32(),
            BS_ROT_ACCUM_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a43a70_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3a70;
        const TARGET: u32 = 0x00a3_f7b0;
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
    fn thunk_00a43a80_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3a80;
        const TARGET: u32 = 0x00a3_f870;
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
    fn thunk_00a43a90_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3a90;
        const TARGET: u32 = 0x00a3_f8a0;
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
    fn thunk_00a43aa0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3aa0;
        const TARGET: u32 = 0x00a3_f8d0;
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
