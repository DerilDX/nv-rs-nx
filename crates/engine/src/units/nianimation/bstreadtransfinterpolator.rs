//! `NiAnimation/bstreadtransfinterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSTreadTransfInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const BS_TREAD_TRANSF_INTERPOLATOR_RTTI: u32 = 0x011f_3ed8;

// Translated from 00a43360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_tread_transf_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_TREAD_TRANSF_INTERPOLATOR_RTTI)
}

/// `00c85800`: the jump target of `00a43340`.
const JUMP_TARGET_00A43340: u32 = 0x00c8_5800;

// Translated from 00a43340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTreadTransfInterpolator::IsEqual` (Xbox PDB): a 5-byte jump to `00c85800`, passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSTreadTransfInterpolator@@UAA_NPAVNiObject@@@Z`.
pub fn bs_tread_transf_interpolator_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A43340, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a43360, bs_tread_transf_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4_3340, bs_tread_transf_interpolator_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_tread_transf_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a43360, &args![]).u32(),
            BS_TREAD_TRANSF_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a43340_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a4_3340;
        const TARGET: u32 = 0x00c8_5800;
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
