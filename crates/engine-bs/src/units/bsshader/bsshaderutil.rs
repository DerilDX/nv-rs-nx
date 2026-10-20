//! `BSShader/bsshaderutil.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSShaderProperty::SetFlagRecurse` (Xbox PDB, `00ba91a0`): `cdecl`, three stack words (the
/// object, the flag index, the value).
const SHADER_PROPERTY_SET_FLAG_RECURSE: u32 = 0x00ba_91a0;

// Translated from 00b6b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderUtil::RecursiveSetPropertyWireframe` (Xbox PDB): `cdecl`, two stack words (the
/// object, the value): `BSShaderProperty::SetFlagRecurse` with flag index `0x30`.
pub fn bs_shader_util_recursive_set_property_wireframe(e: &mut Engine, object: Ptr, value: u32) {
    e.call(
        SHADER_PROPERTY_SET_FLAG_RECURSE,
        &args![object, 0x30u32, value],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00b6_b910,
        bs_shader_util_recursive_set_property_wireframe(Ptr, u32)
    )]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_00b6b910_sets_flag_0x30_recursively() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ba_91a0, |_, _| Ret::default());
        e.call(0x00b6_b910, &args![0x8000u32, 1u32]);
        e.call(0x00b6_b910, &args![0x9000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let sets: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x00ba_91a0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(sets, vec![&vec![0x8000, 0x30, 1], &vec![0x9000, 0x30, 0]]);
    }
}
