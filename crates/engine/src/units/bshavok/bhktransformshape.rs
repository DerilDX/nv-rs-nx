//! `BSHavok/bhktransformshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c84dc0`: the jump target of `00c8bf40`.
const JUMP_TARGET_00C8BF40: u32 = 0x00c8_4dc0;

// Translated from 00c8bf40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkTransformShape::IsEqual` (Xbox PDB): a 5-byte jump to `00c84dc0` (engine map: `bhkShape::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@bhkTransformShape@@UAA_NPAVNiObject@@@Z`.
pub fn bhk_transform_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C8BF40, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c8_bf40, bhk_transform_shape_is_equal(Ptr, Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c8bf40_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_bf40;
        const TARGET: u32 = 0x00c8_4dc0;
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
