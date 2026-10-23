//! `NiParticle/bsstripparticlesystem.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00c1b0b0`: the jump target of `00c21940`.
const JUMP_TARGET_00C21940: u32 = 0x00c1_b0b0;

/// `00c1b1d0`: the jump target of `00c21950`.
const JUMP_TARGET_00C21950: u32 = 0x00c1_b1d0;

/// `00a7fd90`: the jump target of `00c256f0`.
const JUMP_TARGET_00C256F0: u32 = 0x00a7_fd90;

/// `00c1b3a0`: the jump target of `00c25700`.
const JUMP_TARGET_00C25700: u32 = 0x00c1_b3a0;

/// `00c1b810`: the jump target of `00c25710`.
const JUMP_TARGET_00C25710: u32 = 0x00c1_b810;

/// `00c1b400`: the jump target of `00c25720`.
const JUMP_TARGET_00C25720: u32 = 0x00c1_b400;

// Translated from 00c21940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::RegisterStreamables` (Xbox PDB): a 5-byte jump to `00c1b0b0` (engine map: `NiParticleSystem::RegisterStreamables`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?RegisterStreamables@BSStripParticleSystem@@UAA_NAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_register_streamables(
    e: &mut Engine,
    this: Ptr,
    stream: Ptr,
) -> bool {
    e.call(JUMP_TARGET_00C21940, &args![this, stream]).bool()
}

// Translated from 00c21950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::IsEqual` (Xbox PDB): a 5-byte jump to `00c1b1d0` (engine map: `NiParticleSystem::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@BSStripParticleSystem@@UAA_NPAVNiObject@@@Z`.
pub fn bs_strip_particle_system_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00C21950, &args![this, other]).bool()
}

// Translated from 00c256f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::OnVisible` (Xbox PDB): a 5-byte jump to `00a7fd90` (engine map: `NiGeometry::OnVisible`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?OnVisible@BSStripParticleSystem@@UAAXAAVNiCullingProcess@@@Z`.
pub fn bs_strip_particle_system_on_visible(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C256F0, &args![this, arg]);
}

// Translated from 00c25700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::LoadBinary` (Xbox PDB): a 5-byte jump to `00c1b3a0` (engine map: `NiParticleSystem::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25700, &args![this, stream]);
}

// Translated from 00c25710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::LinkObject` (Xbox PDB): a 5-byte jump to `00c1b810` (engine map: `NiParticleSystem::LinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LinkObject@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25710, &args![this, stream]);
}

// Translated from 00c25720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::SaveBinary` (Xbox PDB): a 5-byte jump to `00c1b400` (engine map: `NiParticleSystem::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@BSStripParticleSystem@@UAAXAAVNiStream@@@Z`.
pub fn bs_strip_particle_system_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00C25720, &args![this, stream]);
}

// Translated from 00c257c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2560.
pub fn b_s_strip_particle_system_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2560
}

/// `NiTPointerListBase<..., NiPointer<NiPSysModifier>>::RemoveAll` (engine map, `00e74d40`,
/// `thiscall` on the list, no stack words).
const MODIFIER_LIST_REMOVE_ALL: u32 = 0x00e7_4d40;
/// `NiGeometry::~NiGeometry` (engine map, `00a80580`, `thiscall`, no stack words).
const GEOMETRY_DESTRUCT: u32 = 0x00a8_0580;
/// The sized Ni free `00aa1460` (`cdecl`: block, size).
const NI_DELETE: u32 = 0x00aa_1460;

// Translated from 00c1b9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStripParticleSystem::_scalar_deleting_destructor_` (engine map): `thiscall`, one stack
/// word, `RET 4`. Empties the modifier list at +0xc8, runs `NiGeometry::~NiGeometry`, then
/// frees the 0x110 bytes of `this` when bit 0 of `flags` is set. Answers `this`.
pub fn bs_strip_particle_system_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(MODIFIER_LIST_REMOVE_ALL, &args![this.byte_add(0xc8)]);
    e.call(GEOMETRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x110u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c257c0, b_s_strip_particle_system_get_r_t_t_i() -> u32),
        entry!(
            0x00c1_b9b0,
            bs_strip_particle_system_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00c2_1940, bs_strip_particle_system_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00c2_1950, bs_strip_particle_system_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00c2_56f0, bs_strip_particle_system_on_visible(Ptr, Ptr)),
        entry!(0x00c2_5700, bs_strip_particle_system_load_binary(Ptr, Ptr)),
        entry!(0x00c2_5710, bs_strip_particle_system_link_object(Ptr, Ptr)),
        entry!(0x00c2_5720, bs_strip_particle_system_save_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thunk_00c21940_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_1940;
        const TARGET: u32 = 0x00c1_b0b0;
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
    fn thunk_00c21950_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00c2_1950;
        const TARGET: u32 = 0x00c1_b1d0;
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
    fn thunk_00c256f0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_56f0;
        const TARGET: u32 = 0x00a7_fd90;
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
    fn thunk_00c25700_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5700;
        const TARGET: u32 = 0x00c1_b3a0;
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
    fn thunk_00c25710_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5710;
        const TARGET: u32 = 0x00c1_b810;
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
    fn thunk_00c25720_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c2_5720;
        const TARGET: u32 = 0x00c1_b400;
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
    fn b_s_strip_particle_system_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_57c0, &args![]).u32(), 0x0120_2560);
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn scalar_deleting_destructor_empties_the_list_destructs_and_frees_on_bit_zero() {
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            let mut e = Engine::new();
            e.register(MODIFIER_LIST_REMOVE_ALL, |_, _| Ret::default());
            e.register(GEOMETRY_DESTRUCT, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x00c1_b9b0, &args![0x0e00_3000u32, flags]).u32(),
                0x0e00_3000
            );
            let log = e.call_log.take().unwrap();
            let mut expected = vec![
                (MODIFIER_LIST_REMOVE_ALL, vec![0x0e00_30c8]),
                (GEOMETRY_DESTRUCT, vec![0x0e00_3000]),
            ];
            if frees {
                expected.push((NI_DELETE, vec![0x0e00_3000, 0x110]));
            }
            assert_eq!(log[1..].to_vec(), expected);
        }
    }
}
