//! `BSHavok/bhkblendcollisionobjectaddrotation.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c818f0`: the jump target of `00c8ff80`.
const JUMP_TARGET_00C8FF80: u32 = 0x00c8_18f0;

// Translated from 00c8ff80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObjectAddRotation::LoadBinary` (Xbox PDB): a 5-byte jump to `00c818f0` (engine map: `bhkBlendCollisionObject::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@bhkBlendCollisionObjectAddRotation@@UAAXAAVNiStream@@@Z`.
pub fn bhk_blend_collision_object_add_rotation_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C8FF80, &args![this, stream]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00c8_ff80,
        bhk_blend_collision_object_add_rotation_load_binary(Ptr, Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c8ff80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c8_ff80;
        const TARGET: u32 = 0x00c8_18f0;
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
