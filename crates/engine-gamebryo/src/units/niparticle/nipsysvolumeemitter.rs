//! `NiParticle/nipsysvolumeemitter.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c31c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x58 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_volume_emitter_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x58u32]);
    }
    this
}

// Translated from 00c31bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_27d8.
pub fn ni_p_sys_volume_emitter_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_27d8
}

use crate::units::niparticle::nipsysmodifier::{
    NiPSysModifier, CLONE_MAP_LOOKUP, GET_OBJECT_FROM_LINK_ID, READ_LINK_ID,
};

layout! {
    /// `NiPSysVolumeEmitter` (Xbox PDB), 0x58 bytes; the PC code uses the
    /// same offsets.
    pub struct NiPSysVolumeEmitter: 0x58 {
        /// `m_pkEmitterObj` (Xbox PDB): `NiAVObject*`.
        0x54 m_pkEmitterObj: u32,
    }
}

/// `NiPSysEmitter::LoadBinary` (`00c223b0`, one word).
const EMITTER_LOAD_BINARY: u32 = 0x00c2_23b0;
/// `00c2d6d0`: a 5-byte jump to `NiPSysModifier::LinkObject` (one word).
const MODIFIER_LINK_OBJECT_THUNK: u32 = 0x00c2_d6d0;
/// `NiPSysModifier::ProcessClone` (`00c317e0`, one word).
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
/// `NiTransform::InvertNonUniform` (`004b4880`, `thiscall`, the result's
/// address), `NiTransform::MakeIdentity` (`00a86bf0`, `thiscall`) and the
/// transform product `0062c250` (`thiscall` on the left operand, `(result,
/// right)`).
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MAKE_IDENTITY: u32 = 0x00a8_6bf0;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// The 0x34-byte `NiTransform` at +0x68 of the emitter object and of the
/// particle system that the code copies (their world transform by its use;
/// the PC offset is the code's).
const WORLD_TRANSFORM: u32 = 0x68;
const TRANSFORM_SIZE: u32 = 0x34;
/// `NiPSysVolumeEmitter::ComputeVolumeInitialPositionAndVelocity` (Xbox PDB
/// vtable slot +0xb0).
const COMPUTE_VOLUME_POSITION_AND_VELOCITY: u32 = 0xb0;

// Translated from 00c31b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::LoadBinary` (Xbox PDB): `NiPSysEmitter::LoadBinary`,
/// then a link id (`00a645b0`, the emitter object).
pub fn ni_p_sys_volume_emitter_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysVolumeEmitter>,
    stream: Ptr,
) {
    e.call(EMITTER_LOAD_BINARY, &args![this, stream]);
    e.call(READ_LINK_ID, &args![stream]);
}

// Translated from 00c31b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::LinkObject` (Xbox PDB): `NiPSysModifier::LinkObject`
/// (through the jump at `00c2d6d0`), then the linked object becomes the
/// emitter object.
pub fn ni_p_sys_volume_emitter_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysVolumeEmitter>,
    stream: Ptr,
) {
    e.call(MODIFIER_LINK_OBJECT_THUNK, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysVolumeEmitter::m_pkEmitterObj, object);
}

// Translated from 00c31c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::ProcessClone` (Xbox PDB): `NiPSysModifier`'s
/// `ProcessClone`, then the clone of `this` from the process's map (into a
/// stack word cleared first); with an emitter object, the clone's emitter
/// object is the object's clone when the map has one (the lookup answers
/// true), else the same object.
pub fn ni_p_sys_volume_emitter_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysVolumeEmitter>,
    process: Ptr,
) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::<NiPSysVolumeEmitter>::new(e.mem.u32(slot.addr()));
        let object = e.get(this, NiPSysVolumeEmitter::m_pkEmitterObj);
        if object == 0 {
            return;
        }
        let map = e.mem.u32(process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
            let object_clone = e.mem.u32(slot.addr());
            e.set(clone, NiPSysVolumeEmitter::m_pkEmitterObj, object_clone);
            return;
        }
        let object = e.get(this, NiPSysVolumeEmitter::m_pkEmitterObj);
        e.set(clone, NiPSysVolumeEmitter::m_pkEmitterObj, object);
    });
}

// Translated from 00c31cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysVolumeEmitter::ComputeInitialPositionAndVelocity` (Xbox PDB):
/// `thiscall`, `RET 8` (position, particle info). With an emitter object,
/// the transform from the emitter object's space into the particle
/// system's is the inverse of the system's world transform
/// (`InvertNonUniform`) times the object's world transform (both copied to
/// the stack first); without one it is the identity. That transform goes
/// with both arguments to `ComputeVolumeInitialPositionAndVelocity`
/// (vtable slot +0xb0). The stack frame keeps the exe's layout: the result
/// at +0, the object's transform at +0x34, the inverse at +0x68 and the
/// system's transform at +0x9c.
pub fn ni_p_sys_volume_emitter_compute_initial_position_and_velocity(
    e: &mut Engine,
    this: Ptr<NiPSysVolumeEmitter>,
    position: Ptr,
    info: Ptr,
) {
    e.with_stack(0xd0, |e, frame| {
        let result = frame.addr();
        let object = e.get(this, NiPSysVolumeEmitter::m_pkEmitterObj);
        if object == 0 {
            e.call(TRANSFORM_MAKE_IDENTITY, &args![result]);
        } else {
            let object_world = frame.addr() + 0x34;
            let inverse = frame.addr() + 0x68;
            let system_world = frame.addr() + 0x9c;
            let bytes = e.mem.bytes(object + WORLD_TRANSFORM, TRANSFORM_SIZE);
            e.mem.write(object_world, &bytes);
            let system = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
            let bytes = e
                .mem
                .bytes(system.wrapping_add(WORLD_TRANSFORM), TRANSFORM_SIZE);
            e.mem.write(system_world, &bytes);
            e.call(TRANSFORM_INVERT, &args![system_world, inverse]);
            e.call(TRANSFORM_MULTIPLY, &args![inverse, result, object_world]);
        }
        e.vcall(
            this.addr(),
            COMPUTE_VOLUME_POSITION_AND_VELOCITY,
            &args![result, position, info],
        );
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c3_1b20,
            ni_p_sys_volume_emitter_load_binary(Ptr<NiPSysVolumeEmitter>, Ptr)
        ),
        entry!(
            0x00c3_1b40,
            ni_p_sys_volume_emitter_link_object(Ptr<NiPSysVolumeEmitter>, Ptr)
        ),
        entry!(
            0x00c3_1c90,
            ni_p_sys_volume_emitter_process_clone(Ptr<NiPSysVolumeEmitter>, Ptr)
        ),
        entry!(
            0x00c3_1cf0,
            ni_p_sys_volume_emitter_compute_initial_position_and_velocity(
                Ptr<NiPSysVolumeEmitter>,
                Ptr,
                Ptr,
            )
        ),
        entry!(0x00c31bc0, ni_p_sys_volume_emitter_get_r_t_t_i() -> u32),
        entry!(0x00c31c70, ni_p_sys_volume_emitter_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_volume_emitter_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00c3_1750, |_, _| Ret::default());
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
        assert_eq!(e.call(0x00c31c70, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c31c70, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x58]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_volume_emitter_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c3_1bc0, &args![]).u32(), 0x0120_27d8);
    }

    fn log_of(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn load_binary_reads_the_emitter_then_a_link() {
        let mut e = Engine::new();
        e.register(EMITTER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00c3_1b20, &args![0x5000u32, 0x7000u32]);
        assert_eq!(log_of(&e, EMITTER_LOAD_BINARY), vec![vec![0x5000, 0x7000]]);
        assert_eq!(log_of(&e, READ_LINK_ID), vec![vec![0x7000]]);
    }

    #[test]
    fn link_object_takes_the_linked_emitter_object() {
        let mut e = Engine::new();
        e.register(MODIFIER_LINK_OBJECT_THUNK, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysVolumeEmitter>();
        e.call_log = Some(vec![]);
        e.call(0x00c3_1b40, &args![this, 0x7000u32]);
        assert_eq!(
            log_of(&e, MODIFIER_LINK_OBJECT_THUNK),
            vec![vec![this.addr(), 0x7000]]
        );
        assert_eq!(e.get(this, NiPSysVolumeEmitter::m_pkEmitterObj), 0x6600);
    }

    #[test]
    fn process_clone_maps_or_keeps_the_emitter_object() {
        for (found, expected) in [(true, 0x6100u32), (false, 0x6000)] {
            let mut e = Engine::new();
            e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysVolumeEmitter>();
            let clone = e.new_object::<NiPSysVolumeEmitter>();
            e.set(this, NiPSysVolumeEmitter::m_pkEmitterObj, 0x6000);
            let map = e.mem.alloc(8);
            e.mem.set_u32(map, clone.addr());
            e.mem.set_u32(map + 4, found as u32);
            let process = e.mem.alloc(4);
            e.mem.set_u32(process, map);
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                if a[1] == 0x6000 {
                    let found = e.mem.u32(a[0] + 4);
                    if found != 0 {
                        e.mem.set_u32(a[2], 0x6100);
                    }
                    return Ret {
                        eax: found,
                        ..Ret::default()
                    };
                }
                let clone = e.mem.u32(a[0]);
                e.mem.set_u32(a[2], clone);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            });
            e.call(0x00c3_1c90, &args![this, process]);
            assert_eq!(e.get(clone, NiPSysVolumeEmitter::m_pkEmitterObj), expected);
        }
    }

    const VOLUME_FN: u32 = 0x0f50_0000;

    #[test]
    fn initial_position_uses_the_identity_without_an_emitter_object() {
        let mut e = Engine::new();
        e.register(TRANSFORM_MAKE_IDENTITY, |e, a| {
            e.mem.set_u32(a[0], 0x1d);
            Ret::default()
        });
        e.register(VOLUME_FN, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1d);
            Ret::default()
        });
        let this = e.new_object::<NiPSysVolumeEmitter>();
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + COMPUTE_VOLUME_POSITION_AND_VELOCITY, VOLUME_FN);
        e.mem.set_u32(this.addr(), vtable);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1cf0, &args![this, 0x7100u32, 0x7200u32]);
        let calls = log_of(&e, VOLUME_FN);
        assert_eq!(calls.len(), 1);
        assert_eq!(
            (calls[0][0], calls[0][2], calls[0][3]),
            (this.addr(), 0x7100, 0x7200)
        );
        assert_eq!(log_of(&e, TRANSFORM_MAKE_IDENTITY), vec![vec![calls[0][1]]]);
    }

    #[test]
    fn initial_position_maps_from_the_emitter_object_into_the_system() {
        let mut e = Engine::new();
        e.register(TRANSFORM_INVERT, |e, a| {
            // The system's copy: its first word, plus one, into the inverse.
            let first = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], first + 1);
            Ret::default()
        });
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            let value = e.mem.u32(a[0]) * 100 + e.mem.u32(a[2]);
            e.mem.set_u32(a[1], value);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(VOLUME_FN, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 8 * 100 + 3);
            Ret::default()
        });
        let this = e.new_object::<NiPSysVolumeEmitter>();
        let vtable = e.mem.alloc(0x100);
        e.mem
            .set_u32(vtable + COMPUTE_VOLUME_POSITION_AND_VELOCITY, VOLUME_FN);
        e.mem.set_u32(this.addr(), vtable);
        let object = e.mem.alloc(0xa0);
        e.mem.set_u32(object + WORLD_TRANSFORM, 3);
        let system = e.mem.alloc(0xa0);
        e.mem.set_u32(system + WORLD_TRANSFORM, 7);
        e.mem.set_u32(this.addr() + 0x10, system);
        e.set(this, NiPSysVolumeEmitter::m_pkEmitterObj, object);
        e.call_log = Some(vec![]);
        e.call(0x00c3_1cf0, &args![this, 0x7100u32, 0x7200u32]);
        let volume = log_of(&e, VOLUME_FN);
        let result = volume[0][1];
        assert_eq!(
            log_of(&e, TRANSFORM_INVERT),
            vec![vec![result + 0x9c, result + 0x68]]
        );
        assert_eq!(
            log_of(&e, TRANSFORM_MULTIPLY),
            vec![vec![result + 0x68, result, result + 0x34]]
        );
    }
}
