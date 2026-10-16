//! `BSHavok/bhknicollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a810b0`: the jump target of `00c6b5e0`.
const JUMP_TARGET_00C6B5E0: u32 = 0x00a8_10b0;

// Translated from 00c6b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SetSceneGraphObject` (Xbox PDB): a 5-byte jump to `00a810b0` (engine map: `NiCollisionObject::SetSceneGraphObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetSceneGraphObject@bhkNiCollisionObject@@UAAXPAVNiAVObject@@@Z`.
pub fn bhk_ni_collision_object_set_scene_graph_object(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C6B5E0, &args![this, arg]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c6_b5e0,
        bhk_ni_collision_object_set_scene_graph_object(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c6b5e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_b5e0;
        const TARGET: u32 = 0x00a8_10b0;
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
