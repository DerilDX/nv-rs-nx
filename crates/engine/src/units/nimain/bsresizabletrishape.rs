//! `NiMain/bsresizabletrishape.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSResizableTriShape::ms_RTTI` (the class's `NiRTTI` record).
const BS_RESIZABLE_TRI_SHAPE_RTTI: u32 = 0x011f_5b34;

// Translated from 00a86d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn bs_resizable_tri_shape_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(BS_RESIZABLE_TRI_SHAPE_RTTI)
}

/// `00a98eb0`: the jump target of `00a86d40`.
const JUMP_TARGET_00A86D40: u32 = 0x00a9_8eb0;

/// `00a74500`: the jump target of `00a87190`.
const JUMP_TARGET_00A87190: u32 = 0x00a7_4500;

/// `00a74510`: the jump target of `00a877f0`.
const JUMP_TARGET_00A877F0: u32 = 0x00a7_4510;

// Translated from 00a86d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::LinkObject` (Xbox PDB): a 5-byte jump to `00a98eb0` (engine map: `NiParticleMeshes::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSResizableTriShape@@UAAXAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A86D40, &args![this, stream]);
}

// Translated from 00a87190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a74500` (engine map: `NiParticleMeshes::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSResizableTriShape@@UAA_NAAVNiStream@@@Z`.
pub fn bs_resizable_tri_shape_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A87190, &args![this, stream]).bool()
}

// Translated from 00a877f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSResizableTriShape::IsEqual` (Xbox PDB): a 5-byte jump to `00a74510` (engine map: `NiParticleMeshes::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSResizableTriShape@@UAA_NPAVNiObject@@@Z`.
pub fn bs_resizable_tri_shape_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A877F0, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a86d20, bs_resizable_tri_shape_get_rtti() -> Ptr),
        entry!(0x00a8_6d40, bs_resizable_tri_shape_link_object(Ptr, Ptr)),
        entry!(0x00a8_7190, bs_resizable_tri_shape_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a8_77f0, bs_resizable_tri_shape_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_resizable_tri_shape_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a86d20, &args![]).u32(),
            BS_RESIZABLE_TRI_SHAPE_RTTI
        );
    }

    #[test]
    fn thunk_00a86d40_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_6d40;
        const TARGET: u32 = 0x00a9_8eb0;
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
    fn thunk_00a87190_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_7190;
        const TARGET: u32 = 0x00a7_4500;
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
    fn thunk_00a877f0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a8_77f0;
        const TARGET: u32 = 0x00a7_4510;
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
