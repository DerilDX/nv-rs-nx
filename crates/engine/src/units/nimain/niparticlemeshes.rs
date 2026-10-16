//! `NiMain/niparticlemeshes.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiParticleMeshes::ms_RTTI` (the class's `NiRTTI` record).
const NI_PARTICLE_MESHES_RTTI: u32 = 0x011f_5f14;

// Translated from 00a98ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_particle_meshes_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_PARTICLE_MESHES_RTTI)
}

/// `00a9b1a0`: the jump target of `00a71d10`.
const JUMP_TARGET_00A71D10: u32 = 0x00a9_b1a0;

/// `00a9b190`: the jump target of `00a74500`.
const JUMP_TARGET_00A74500: u32 = 0x00a9_b190;

/// `00a89770`: the jump target of `00a74510`.
const JUMP_TARGET_00A74510: u32 = 0x00a8_9770;

/// `00a9b180`: the jump target of `00a98ea0`.
const JUMP_TARGET_00A98EA0: u32 = 0x00a9_b180;

/// `00a74780`: the jump target of `00a98eb0`.
const JUMP_TARGET_00A98EB0: u32 = 0x00a7_4780;

/// `00a80400`: the jump target of `00a9b140`.
const JUMP_TARGET_00A9B140: u32 = 0x00a8_0400;

// Translated from 00a71d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::SaveBinary` (Xbox PDB): a 5-byte jump to `00a9b1a0` (engine map: `NiLines::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A71D10, &args![this, stream]);
}

// Translated from 00a74500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00a9b190` (engine map: `NiLines::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@NiParticleMeshes@@UAA_NAAVNiStream@@@Z`.
pub fn ni_particle_meshes_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    e.call(JUMP_TARGET_00A74500, &args![this, stream]).bool()
}

// Translated from 00a74510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::IsEqual` (Xbox PDB): a 5-byte jump to `00a89770` (engine map: `NiLines::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiParticleMeshes@@UAA_NPAVNiObject@@@Z`.
pub fn ni_particle_meshes_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A74510, &args![this, other]).bool()
}

// Translated from 00a98ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::LoadBinary` (Xbox PDB): a 5-byte jump to `00a9b180` (engine map: `NiLines::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A98EA0, &args![this, stream]);
}

// Translated from 00a98eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::LinkObject` (Xbox PDB): a 5-byte jump to `00a74780` (engine map: `NiLines::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@NiParticleMeshes@@UAAXAAVNiStream@@@Z`.
pub fn ni_particle_meshes_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A98EB0, &args![this, stream]);
}

// Translated from 00a9b140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiParticleMeshes::CopyMembers` (Xbox PDB): a 5-byte jump to `00a80400` (engine map: `NiLines::CopyMembers`), passing on `this` and its two arguments. Return type (void): Xbox PDB public name `?CopyMembers@NiParticleMeshes@@IAAXPAV1@AAVNiCloningProcess@@@Z`.
pub fn ni_particle_meshes_copy_members(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00A9B140, &args![this, first, second]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a98ec0, ni_particle_meshes_get_rtti() -> Ptr),
        entry!(0x00a7_1d10, ni_particle_meshes_save_binary(Ptr, Ptr)),
        entry!(0x00a7_4500, ni_particle_meshes_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a7_4510, ni_particle_meshes_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_8ea0, ni_particle_meshes_load_binary(Ptr, Ptr)),
        entry!(0x00a9_8eb0, ni_particle_meshes_link_object(Ptr, Ptr)),
        entry!(0x00a9_b140, ni_particle_meshes_copy_members(Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_particle_meshes_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a98ec0, &args![]).u32(), NI_PARTICLE_MESHES_RTTI);
    }

    #[test]
    fn thunk_00a71d10_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_1d10;
        const TARGET: u32 = 0x00a9_b1a0;
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
    fn thunk_00a74500_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_4500;
        const TARGET: u32 = 0x00a9_b190;
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
    fn thunk_00a74510_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a7_4510;
        const TARGET: u32 = 0x00a8_9770;
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
    fn thunk_00a98ea0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_8ea0;
        const TARGET: u32 = 0x00a9_b180;
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
    fn thunk_00a98eb0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_8eb0;
        const TARGET: u32 = 0x00a7_4780;
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
    fn thunk_00a9b140_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_b140;
        const TARGET: u32 = 0x00a8_0400;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32, 0x3000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32, 0x3000u32]);
    }
}
