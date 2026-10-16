//! `BSHavok/bhkblendcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c81570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkBlendCollisionObject::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_7e64.
pub fn bhk_blend_collision_object_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_7e64
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00c81570, bhk_blend_collision_object_get_r_t_t_i() -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bhk_blend_collision_object_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c8_1570, &args![]).u32(), 0x0126_7e64);
    }
}
