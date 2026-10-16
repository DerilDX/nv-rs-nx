//! `NiMain/niparticlemeshesdata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleMeshesData::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_MESHES_DATA_RTTI: u32 = 0x011f_5f0c;

// Translated from 00a98b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_meshes_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_MESHES_DATA_RTTI)
}

// Translated from 00a98dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshesData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00a98b90`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x78 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_particle_meshes_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a9_8b90;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x78u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a98b00, ni_particle_meshes_data_get_rtti() -> Ptr),
        entry!(0x00a98dc0, ni_particle_meshes_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_meshes_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a98b00, &args![]).u32(),
            NI_PARTICLE_MESHES_DATA_RTTI
        );
    }

    #[test]
    fn ni_particle_meshes_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a9_8b90, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00a98dc0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00a98dc0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x78]]);
        assert_eq!(calls_to(&e, 0x00a9_8b90), vec![vec![this]; 2]);
    }
}
