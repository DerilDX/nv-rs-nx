//! `NiParticle/nipsysspawnmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c23ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2550.
pub fn ni_p_sys_spawn_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2550
}

use crate::units::niparticle::nipsysmodifier::stream_read;

layout! {
    /// `NiPSysSpawnModifier` (Xbox PDB), 0x34 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysSpawnModifier: 0x34 {
        /// `m_usNumSpawnGenerations` (Xbox PDB).
        0x18 m_usNumSpawnGenerations: u16,
        /// `m_fPercentageSpawned` (Xbox PDB).
        0x1C m_fPercentageSpawned: u32,
        /// `m_usMinNumToSpawn` (Xbox PDB).
        0x20 m_usMinNumToSpawn: u16,
        /// `m_usMaxNumToSpawn` (Xbox PDB).
        0x22 m_usMaxNumToSpawn: u16,
        /// `m_fSpawnSpeedChaos` (Xbox PDB).
        0x24 m_fSpawnSpeedChaos: u32,
        /// `m_fSpawnDirChaos` (Xbox PDB).
        0x28 m_fSpawnDirChaos: u32,
        /// `m_fLifeSpan` (Xbox PDB).
        0x2C m_fLifeSpan: u32,
        /// `m_fLifeSpanVar` (Xbox PDB).
        0x30 m_fLifeSpanVar: u32,
    }
}

/// The vtable of `NiPSysSpawnModifier` (RTTI `.?AVNiPSysSpawnModifier@@`,
/// `010be124`).
const VTABLE: u32 = 0x010b_e124;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`), member copy (`00c31780`)
/// and `LoadBinary` (`00c318b0`).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;

// Translated from 00c23d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::CreateClone` (Xbox PDB): a 0x34-byte object from
/// `00aa13e0` (constructed: the `NiPSysModifier` constructor, the vtable
/// `010be124`, one generation, 100% spawned (1.0), one to one particles,
/// chaos and life span 0.0; 0 when the allocation fails), the
/// `NiPSysModifier` member copy into it (it still runs into a null clone),
/// then every field (the floats through the x87 stack). Returns the clone.
pub fn ni_p_sys_spawn_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    process: Ptr,
) -> Ptr<NiPSysSpawnModifier> {
    let clone = Ptr::<NiPSysSpawnModifier>::new(e.call(NI_NEW, &args![0x34u32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        let zero = 0.0f32.to_bits();
        e.set(
            clone,
            NiPSysSpawnModifier::m_fPercentageSpawned,
            1.0f32.to_bits(),
        );
        e.set(clone, NiPSysSpawnModifier::m_fSpawnSpeedChaos, zero);
        e.set(clone, NiPSysSpawnModifier::m_fSpawnDirChaos, zero);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysSpawnModifier::m_fLifeSpan, zero);
        e.set(clone, NiPSysSpawnModifier::m_usNumSpawnGenerations, 1);
        e.set(clone, NiPSysSpawnModifier::m_fLifeSpanVar, zero);
        e.set(clone, NiPSysSpawnModifier::m_usMinNumToSpawn, 1);
        e.set(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn, 1);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let copy_float = |e: &mut Engine, field| {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    };
    let generations = e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations);
    e.set(
        clone,
        NiPSysSpawnModifier::m_usNumSpawnGenerations,
        generations,
    );
    copy_float(e, NiPSysSpawnModifier::m_fPercentageSpawned);
    let min = e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn);
    e.set(clone, NiPSysSpawnModifier::m_usMinNumToSpawn, min);
    let max = e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn);
    e.set(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn, max);
    copy_float(e, NiPSysSpawnModifier::m_fSpawnSpeedChaos);
    copy_float(e, NiPSysSpawnModifier::m_fSpawnDirChaos);
    copy_float(e, NiPSysSpawnModifier::m_fLifeSpan);
    copy_float(e, NiPSysSpawnModifier::m_fLifeSpanVar);
    clone
}

// Translated from 00c247f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysSpawnModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, then the generations (2 bytes), percentage (4), minimum
/// and maximum counts (2 each), speed and direction chaos, life span and
/// its variation (4 each). The component-size word is the stream
/// argument's stack word.
pub fn ni_p_sys_spawn_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysSpawnModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.with_stack(4, |e, slot| {
        for (off, size) in [
            (NiPSysSpawnModifier::m_usNumSpawnGenerations.off, 2),
            (NiPSysSpawnModifier::m_fPercentageSpawned.off, 4),
            (NiPSysSpawnModifier::m_usMinNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_usMaxNumToSpawn.off, 2),
            (NiPSysSpawnModifier::m_fSpawnSpeedChaos.off, 4),
            (NiPSysSpawnModifier::m_fSpawnDirChaos.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpan.off, 4),
            (NiPSysSpawnModifier::m_fLifeSpanVar.off, 4),
        ] {
            stream_read(e, s, this.addr() + off, size, size, slot.addr());
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_3d10,
            ni_p_sys_spawn_modifier_create_clone(
                Ptr<NiPSysSpawnModifier>,
                Ptr,
            ) -> Ptr<NiPSysSpawnModifier>
        ),
        entry!(
            0x00c2_47f0,
            ni_p_sys_spawn_modifier_load_binary(Ptr<NiPSysSpawnModifier>, Ptr)
        ),
        entry!(0x00c23ce0, ni_p_sys_spawn_modifier_get_r_t_t_i() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_spawn_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_3ce0, &args![]).u32(), 0x0120_2550);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

    #[test]
    fn create_clone_constructs_then_copies_every_field() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.set(this, NiPSysSpawnModifier::m_usNumSpawnGenerations, 3);
        e.set(this, NiPSysSpawnModifier::m_fPercentageSpawned, 0x7f80_0001);
        e.set(this, NiPSysSpawnModifier::m_usMaxNumToSpawn, 9);
        e.set(this, NiPSysSpawnModifier::m_fLifeSpanVar, 2.0f32.to_bits());
        e.call_log = Some(vec![]);
        let clone = e
            .call(0x00c2_3d10, &args![this, 0x4000u32])
            .ptr::<NiPSysSpawnModifier>();
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(
            |c| c.0 == MODIFIER_COPY_MEMBERS && c.1 == vec![this.addr(), clone.addr(), 0x4000]
        ));
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_usNumSpawnGenerations),
            3
        );
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_fPercentageSpawned),
            0x7fc0_0001
        );
        assert_eq!(e.get(clone, NiPSysSpawnModifier::m_usMinNumToSpawn), 0);
        assert_eq!(e.get(clone, NiPSysSpawnModifier::m_usMaxNumToSpawn), 9);
        assert_eq!(
            e.get(clone, NiPSysSpawnModifier::m_fLifeSpanVar),
            2.0f32.to_bits()
        );
    }

    #[test]
    fn create_clone_defaults_before_the_copy() {
        let mut e = Engine::new();
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        // The copy double checks the defaults the constructor left.
        e.register(MODIFIER_COPY_MEMBERS, |e, a| {
            let clone = a[1];
            assert_eq!(e.mem.u16(clone + 0x18), 1);
            assert_eq!(e.mem.f32(clone + 0x1c), 1.0);
            assert_eq!(e.mem.u16(clone + 0x20), 1);
            assert_eq!(e.mem.u16(clone + 0x22), 1);
            Ret::default()
        });
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.call(0x00c2_3d10, &args![this, 0x4000u32]);
    }

    #[test]
    fn load_binary_reads_every_field() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        let mut bytes = vec![2u8, 0];
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        bytes.extend_from_slice(&[3, 0, 4, 0]);
        for v in [1.0f32, 2.0, 3.0, 4.0] {
            bytes.extend_from_slice(&v.to_le_bytes());
        }
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysSpawnModifier>();
        e.call(0x00c2_47f0, &args![this, stream]);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usNumSpawnGenerations), 2);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usMinNumToSpawn), 3);
        assert_eq!(e.get(this, NiPSysSpawnModifier::m_usMaxNumToSpawn), 4);
        assert_eq!(
            e.get(this, NiPSysSpawnModifier::m_fLifeSpanVar),
            4.0f32.to_bits()
        );
        assert_eq!(test_stream::position(&e, stream), bytes.len() as u32);
    }
}
