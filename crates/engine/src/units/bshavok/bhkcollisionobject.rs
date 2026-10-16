//! `BSHavok/bhkcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c6c330`: the jump target of `00c65800`.
const JUMP_TARGET_00C65800: u32 = 0x00c6_c330;

/// `00c6bb70`: the jump target of `00c65810`.
const JUMP_TARGET_00C65810: u32 = 0x00c6_bb70;

/// `00c6c380`: the jump target of `00c65820`.
const JUMP_TARGET_00C65820: u32 = 0x00c6_c380;

/// `00c6b730`: the jump target of `00c87fa0`.
const JUMP_TARGET_00C87FA0: u32 = 0x00c6_b730;

/// `00c6b760`: the jump target of `00c87fb0`.
const JUMP_TARGET_00C87FB0: u32 = 0x00c6_b760;

// Translated from 00c65800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::LoadBinary` (Xbox PDB): a 5-byte jump to `00c6c330` (engine map: `bhkNiCollisionObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65800, &args![this, stream]);
}

// Translated from 00c65810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::LinkObject` (Xbox PDB): a 5-byte jump to `00c6bb70` (engine map: `bhkNiCollisionObject::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65810, &args![this, stream]);
}

// Translated from 00c65820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::SaveBinary` (Xbox PDB): a 5-byte jump to `00c6c380` (engine map: `bhkNiCollisionObject::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@bhkCollisionObject@@UAAXAAVNiStream@@@Z`.
pub fn bhk_collision_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C65820, &args![this, stream]);
}

// Translated from 00c87fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c6b730` (engine map: `bhkNiCollisionObject::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@bhkCollisionObject@@UAA_NAAVNiStream@@@Z`.
pub fn bhk_collision_object_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00C87FA0, &args![this, stream]).bool()
}

// Translated from 00c87fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkCollisionObject::IsEqual` (Xbox PDB): a 5-byte jump to `00c6b760` (engine map: `bhkNiCollisionObject::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@bhkCollisionObject@@UAA_NPAVNiObject@@@Z`.
pub fn bhk_collision_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C87FB0, &args![this, other]).bool()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6_5800, bhk_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00c6_5810, bhk_collision_object_link_object(Ptr, Ptr)),
        entry!(0x00c6_5820, bhk_collision_object_save_binary(Ptr, Ptr)),
        entry!(0x00c8_7fa0, bhk_collision_object_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c8_7fb0, bhk_collision_object_is_equal(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c65800_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5800;
        const TARGET: u32 = 0x00c6_c330;
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
    fn thunk_00c65810_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5810;
        const TARGET: u32 = 0x00c6_bb70;
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
    fn thunk_00c65820_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_5820;
        const TARGET: u32 = 0x00c6_c380;
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
    fn thunk_00c87fa0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_7fa0;
        const TARGET: u32 = 0x00c6_b730;
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
    fn thunk_00c87fb0_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c8_7fb0;
        const TARGET: u32 = 0x00c6_b760;
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
