//! `NiAnimation/niblendtransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3e64;

// Translated from 00a40a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI)
}

/// `00a37510`: the jump target of `00a40aa0`.
const JUMP_TARGET_00A40AA0: u32 = 0x00a3_7510;

// Translated from 00a40aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a37510`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00a40aa0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00A40AA0, &args![this, first, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a40a40, ni_blend_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4_0aa0, fn_00a40aa0(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a40a40, &args![]).u32(),
            NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a40aa0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_0aa0;
        const TARGET: u32 = 0x00a3_7510;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }
}
