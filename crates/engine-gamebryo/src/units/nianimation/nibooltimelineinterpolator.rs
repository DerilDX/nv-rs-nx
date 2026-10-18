//! `NiAnimation/nibooltimelineinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBoolTimelineInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BOOL_TIMELINE_INTERPOLATOR_RTTI: u32 = 0x011f_4044;

// Translated from 00a52d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_bool_timeline_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BOOL_TIMELINE_INTERPOLATOR_RTTI)
}

/// `00a43330`: the jump target of `00a514d0`.
const JUMP_TARGET_00A514D0: u32 = 0x00a4_3330;

/// `00a53600`: the jump target of `00a52ce0`.
const JUMP_TARGET_00A52CE0: u32 = 0x00a5_3600;

/// `00a3c5b0`: the jump target of `00a52cf0`.
const JUMP_TARGET_00A52CF0: u32 = 0x00a3_c5b0;

/// `00a53680`: the jump target of `00a52d00`.
const JUMP_TARGET_00A52D00: u32 = 0x00a5_3680;

/// `00a53220`: the jump target of `00a52d10`.
const JUMP_TARGET_00A52D10: u32 = 0x00a5_3220;

// Translated from 00a514d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::LinkObject` (Xbox PDB): a 5-byte jump to `00a43330` (engine map: `BSBlendTreadTransfInterpolator::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A514D0, &args![this, stream]);
}

// Translated from 00a52ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::LoadBinary` (Xbox PDB): a 5-byte jump to `00a53600` (engine map: `NiBoolInterpolator::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A52CE0, &args![this, stream]);
}

// Translated from 00a52cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a3c5b0` (engine map: `NiBoolInterpolator::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiBoolTimelineInterpolator@@UAA_NAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00A52CF0, &args![this, stream]).bool()
}

// Translated from 00a52d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::SaveBinary` (Xbox PDB): a 5-byte jump to `00a53680` (engine map: `NiBoolInterpolator::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiBoolTimelineInterpolator@@UAAXAAVNiStream@@@Z`.
pub fn ni_bool_timeline_interpolator_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A52D00, &args![this, stream]);
}

// Translated from 00a52d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBoolTimelineInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00a53220` (engine map: `NiBoolInterpolator::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiBoolTimelineInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn ni_bool_timeline_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A52D10, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a52d70, ni_bool_timeline_interpolator_get_rtti() -> Ptr),
        entry!(
            0x00a5_14d0,
            ni_bool_timeline_interpolator_link_object(Ptr, Ptr)
        ),
        entry!(
            0x00a5_2ce0,
            ni_bool_timeline_interpolator_load_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_2cf0, ni_bool_timeline_interpolator_register_streamables(Ptr, Ptr) -> bool),
        entry!(
            0x00a5_2d00,
            ni_bool_timeline_interpolator_save_binary(Ptr, Ptr)
        ),
        entry!(0x00a5_2d10, ni_bool_timeline_interpolator_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_bool_timeline_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a52d70, &args![]).u32(),
            NI_BOOL_TIMELINE_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a514d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_14d0;
        const TARGET: u32 = 0x00a4_3330;
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
    fn thunk_00a52ce0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_2ce0;
        const TARGET: u32 = 0x00a5_3600;
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
    fn thunk_00a52cf0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_2cf0;
        const TARGET: u32 = 0x00a3_c5b0;
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
    fn thunk_00a52d00_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a5_2d00;
        const TARGET: u32 = 0x00a5_3680;
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
    fn thunk_00a52d10_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a5_2d10;
        const TARGET: u32 = 0x00a5_3220;
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
