//! `NiAnimation/bsblendtreadtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSBlendTreadTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ed0;

// Translated from 00a42fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_blend_tread_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI)
}

/// `00a37ec0`: the jump target of `00a40a10`.
const JUMP_TARGET_00A40A10: u32 = 0x00a3_7ec0;

/// `00a37980`: the jump target of `00a42fc0`.
const JUMP_TARGET_00A42FC0: u32 = 0x00a3_7980;

/// `00a36fd0`: the jump target of `00a42fd0`.
const JUMP_TARGET_00A42FD0: u32 = 0x00a3_6fd0;

/// `00a95ca0`: the jump target of `00a43330`.
const JUMP_TARGET_00A43330: u32 = 0x00a9_5ca0;

// Translated from 00a40a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a37ec0` (engine map: `NiBlendInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A40A10, &args![this, stream]);
}

// Translated from 00a42fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a37980` (engine map: `NiBlendInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A42FC0, &args![this, stream]);
}

// Translated from 00a42fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a36fd0` (engine map: `NiBlendInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSBlendTreadTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_blend_tread_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A42FD0, &args![this, other]).bool()
}

// Translated from 00a43330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::LinkObject` (Xbox PDB): a 5-byte jump to `00a95ca0` (engine map: `BSGrabIKNote::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSBlendTreadTransfInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn bs_blend_tread_transf_interpolator_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A43330, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a42fe0, bs_blend_tread_transf_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a4_0a10,
            bs_blend_tread_transf_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(
            0x00a4_2fc0,
            bs_blend_tread_transf_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a4_2fd0, bs_blend_tread_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a4_3330,
            bs_blend_tread_transf_interpolator_link_object(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_blend_tread_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a42fe0, &args![]).u32(),
            BS_BLEND_TREAD_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a40a10_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_0a10;
        const TARGET: u32 = 0x00a3_7ec0;
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
    fn thunk_00a42fc0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_2fc0;
        const TARGET: u32 = 0x00a3_7980;
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
    fn thunk_00a42fd0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_2fd0;
        const TARGET: u32 = 0x00a3_6fd0;
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
    fn thunk_00a43330_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_3330;
        const TARGET: u32 = 0x00a9_5ca0;
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
