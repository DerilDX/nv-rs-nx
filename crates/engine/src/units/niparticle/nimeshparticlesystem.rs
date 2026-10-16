//! `NiParticle/nimeshparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1ad80`: the jump target of `00c21900`.
const JUMP_TARGET_00C21900: u32 = 0x00c1_ad80;

// Translated from 00c21900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::SetModelData` (Xbox PDB): a 5-byte jump to `00c1ad80` (engine map: `NiParticleSystem::SetModelData`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetModelData@NiMeshParticleSystem@@UAAXPAVNiGeometryData@@@Z`.
pub fn ni_mesh_particle_system_set_model_data(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C21900, &args![this, arg]);
}

// Translated from 00c21e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshParticleSystem::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2538.
pub fn ni_mesh_particle_system_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2538
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c21e10, ni_mesh_particle_system_get_r_t_t_i() -> u32),
        entry!(
            0x00c2_1900,
            ni_mesh_particle_system_set_model_data(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c21900_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_1900;
        const TARGET: u32 = 0x00c1_ad80;
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
    fn ni_mesh_particle_system_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_1e10, &args![]).u32(), 0x0120_2538);
    }
}
