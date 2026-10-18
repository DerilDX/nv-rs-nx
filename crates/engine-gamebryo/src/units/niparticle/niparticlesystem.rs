//! `NiParticle/niparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleSystem::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_SYSTEM_RTTI: u32 = 0x0120_24e0;

// Translated from 00c1b5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_system_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_SYSTEM_RTTI)
}

/// `00a80680`: the jump target of `00c1ad80`.
const JUMP_TARGET_00C1AD80: u32 = 0x00a8_0680;

// Translated from 00c1ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleSystem::SetModelData` (Xbox PDB): a 5-byte jump to `00a80680` (engine map: `NiGeometry::SetModelData`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetModelData@NiParticleSystem@@UAAXPAVNiGeometryData@@@Z`.
pub fn ni_particle_system_set_model_data(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C1AD80, &args![this, arg]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c1b5b0, ni_particle_system_get_rtti() -> Ptr),
        entry!(0x00c1_ad80, ni_particle_system_set_model_data(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_system_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c1b5b0, &args![]).u32(), NI_PARTICLE_SYSTEM_RTTI);
    }

    #[test]
    fn thunk_00c1ad80_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c1_ad80;
        const TARGET: u32 = 0x00a8_0680;
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
