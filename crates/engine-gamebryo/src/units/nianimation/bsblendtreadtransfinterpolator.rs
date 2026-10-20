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

/// `BSBlendTreadTransfInterpolator`'s vtable (`01097904`).
const BLEND_TREAD_VTABLE: u32 = 0x0109_7904;
/// `NiBlendInterpolator::~NiBlendInterpolator` (`00a37170`, `thiscall`) and
/// `ComputeNormalizedWeights` (`00a37260`, `thiscall`).
const BLEND_INTERPOLATOR_DESTRUCTOR: u32 = 0x00a3_7170;
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// `BSBlendTreadTransfInterpolator::StoreSingleValue` (`00a43040`) and
/// `BlendValues` (`00a43160`), both `thiscall (this, time, target,
/// &value)` answering a `bool`.
const STORE_SINGLE_VALUE: u32 = 0x00a4_3040;
const BLEND_VALUES: u32 = 0x00a4_3160;
/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` +8,
/// `m_ucInterpCount` +0xE.
const BLEND_LAST_TIME: u32 = 0x8;
const BLEND_INTERP_COUNT: u32 = 0xe;

// Translated from 00a42f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::~BSBlendTreadTransfInterpolator` (Xbox
/// PDB): the class's vtable, then the `NiBlendInterpolator` destructor (a
/// tail jump).
pub fn bs_blend_tread_transf_interpolator_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), BLEND_TREAD_VTABLE);
    e.call(BLEND_INTERPOLATOR_DESTRUCTOR, &args![this]);
}

// Translated from 00a432d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSBlendTreadTransfInterpolator::Update` (Xbox PDB; `thiscall (time,
/// target, &value)`, `RET 0xC`): with one interpolator its value is stored
/// (`StoreSingleValue`); with more, the weights are normalized and the
/// values blended (`BlendValues`); with none nothing happens and false is
/// answered. The last time takes `time` (through the FPU) in every case;
/// the answer is the called function's.
pub fn bs_blend_tread_transf_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    target: Ptr,
    value: Ptr,
) -> bool {
    let object = this.addr();
    let count = e.mem.u8(object + BLEND_INTERP_COUNT);
    let time_bits = x87_quiet_bits(time.to_bits());
    let result = if count == 1 {
        e.call(STORE_SINGLE_VALUE, &args![this, time_bits, target, value])
            .bool()
    } else if count != 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        e.call(BLEND_VALUES, &args![this, time_bits, target, value])
            .bool()
    } else {
        false
    };
    e.mem.set_u32(object + BLEND_LAST_TIME, time_bits);
    result
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
        entry!(
            0x00a4_2f80,
            bs_blend_tread_transf_interpolator_destructor(Ptr)
        ),
        entry!(0x00a4_32d0, bs_blend_tread_transf_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool),
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

    fn blend_calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    #[test]
    fn blend_tread_destructor_sets_the_vtable_and_runs_the_base() {
        let mut e = Engine::new();
        e.register(BLEND_INTERPOLATOR_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x34);
        e.call(0x00a4_2f80, &args![this]);
        assert_eq!(e.mem.u32(this), BLEND_TREAD_VTABLE);
        assert_eq!(
            blend_calls_to(&e, BLEND_INTERPOLATOR_DESTRUCTOR),
            vec![vec![this]]
        );
    }

    #[test]
    fn blend_tread_update_stores_blends_or_does_nothing_by_count() {
        let mut e = Engine::new();
        e.register(STORE_SINGLE_VALUE, |_, _| 1u32.into_ret());
        e.register(BLEND_VALUES, |_, _| 0x101u32.into_ret());
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let this = e.mem.alloc(0x34);
        let time = 0.5f32.to_bits();
        for (count, answer) in [(1u8, true), (3, true), (0, false)] {
            e.call_log = Some(vec![]);
            e.mem.set_u8(this + BLEND_INTERP_COUNT, count);
            e.mem.set_f32(this + BLEND_LAST_TIME, 0.0);
            let result = e
                .call(0x00a4_32d0, &args![this, 0.5f32, 0x6000u32, 0x7000u32])
                .bool();
            assert_eq!(result, answer);
            assert_eq!(e.mem.f32(this + BLEND_LAST_TIME), 0.5);
            let single = blend_calls_to(&e, STORE_SINGLE_VALUE);
            let blended = blend_calls_to(&e, BLEND_VALUES);
            match count {
                1 => {
                    assert_eq!(single, vec![vec![this, time, 0x6000, 0x7000]]);
                    assert!(blended.is_empty());
                }
                3 => {
                    assert_eq!(blended, vec![vec![this, time, 0x6000, 0x7000]]);
                    assert_eq!(
                        blend_calls_to(&e, COMPUTE_NORMALIZED_WEIGHTS),
                        vec![vec![this]]
                    );
                }
                _ => assert!(single.is_empty() && blended.is_empty()),
            }
        }
    }
}
