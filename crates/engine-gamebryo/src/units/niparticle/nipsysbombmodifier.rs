//! `NiParticle/nipsysbombmodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c2e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00c31750`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x38 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_p_sys_bomb_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00c3_1750;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x38u32]);
    }
    this
}

// Translated from 00c2e540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2790.
pub fn ni_p_sys_bomb_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2790
}

use crate::units::niparticle::nipsysdata::NiPSysData;
use crate::units::niparticle::nipsysmodifier::{
    stream_read, NiPSysModifier, CLONE_MAP_LOOKUP, GET_OBJECT_FROM_LINK_ID, READ_LINK_ID,
};

layout! {
    /// `NiPSysBombModifier` (Xbox PDB), 0x38 bytes; the PC code uses the
    /// same offsets. Floats are declared as their bit patterns.
    pub struct NiPSysBombModifier: 0x38 {
        /// `m_pkBombObj` (Xbox PDB): `NiAVObject*`.
        0x18 m_pkBombObj: u32,
        /// `m_kBombAxis` (Xbox PDB): an `NiPoint3`.
        0x1C m_kBombAxis: u32,
        /// `m_fDecay` (Xbox PDB).
        0x28 m_fDecay: u32,
        /// `m_fDeltaV` (Xbox PDB).
        0x2C m_fDeltaV: u32,
        /// `m_eDecayType` (Xbox PDB): `Update` handles 0 (none), 1 and 2.
        0x30 m_eDecayType: u32,
        /// `m_eSymmType` (Xbox PDB): `Update` handles 0, 1 and 2.
        0x34 m_eSymmType: u32,
    }
}

/// The vtable of `NiPSysBombModifier` (RTTI `.?AVNiPSysBombModifier@@`,
/// `010c01ec`).
const VTABLE: u32 = 0x010c_01ec;
/// `NiPoint3::ZERO` (three `float`s at `011f426c`) and the axis the
/// constructor gives (`011a946c`), both copied by `MOV`.
const POINT_ZERO: u32 = 0x011f_426c;
const DEFAULT_AXIS: u32 = 0x011a_946c;
/// `00aa13e0` (unnamed, `cdecl`, one word): the object allocation.
const NI_NEW: u32 = 0x00aa_13e0;
/// `NiPSysModifier`'s constructor (`00c31720`), member copy (`00c31780`),
/// `LinkObject` (`00c31650`), `ProcessClone` (`00c317e0`) and `LoadBinary`
/// (`00c318b0`); `NiColor::LoadBinary` (`00a59be0`) reads the axis.
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const COLOR_LOAD_BINARY: u32 = 0x00a5_9be0;
/// `NiTransform::InvertNonUniform` (`004b4880`), the transform product
/// (`0062c250`), the matrix-times-vector (`004b4500`), the in-place
/// normalisation (`004a0c10`), the square root (`00ec6040`) and `exp`
/// (`00ecae6c`); see the gravity modifier for their forms.
const TRANSFORM_INVERT: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
const MATRIX_TIMES_VECTOR: u32 = 0x004b_4500;
const UNITIZE: u32 = 0x004a_0c10;
const SQRT: u32 = 0x00ec_6040;
const EXP: u32 = 0x00ec_ae6c;
/// The 0x34-byte `NiTransform` at +0x68 of the bomb object and of the
/// particle system, as the PC code reads them.
const OBJECT_TRANSFORM: u32 = 0x68;
const TRANSFORM_SIZE: u32 = 0x34;

// Translated from 00c2e520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::LinkObject` (Xbox PDB): `NiPSysModifier`'s
/// `LinkObject`, then the linked object becomes the bomb object.
pub fn ni_p_sys_bomb_modifier_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    e.set(this, NiPSysBombModifier::m_pkBombObj, object);
}

// Translated from 00c2e570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::CreateClone` (Xbox PDB): a 0x38-byte object from
/// `00aa13e0` (constructed: the `NiPSysModifier` constructor, the vtable
/// `010c01ec`, no bomb object, the axis from `011a946c`, decay and delta-v
/// 0.0, both types 0; 0 when the allocation fails), the `NiPSysModifier`
/// member copy into it (it still runs into a null clone), then the axis,
/// decay and delta-v (through the x87 stack) and the two types; the bomb
/// object is not copied. Returns the clone.
pub fn ni_p_sys_bomb_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    process: Ptr,
) -> Ptr<NiPSysBombModifier> {
    let clone = Ptr::<NiPSysBombModifier>::new(e.call(NI_NEW, &args![0x38u32]).u32());
    if !clone.is_null() {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(clone.addr(), VTABLE);
        e.set(clone, NiPSysBombModifier::m_pkBombObj, 0);
        for word in 0..3 {
            let value = e.global::<u32>(DEFAULT_AXIS + 4 * word);
            e.mem.set_u32(
                clone.addr() + NiPSysBombModifier::m_kBombAxis.off + 4 * word,
                value,
            );
        }
        let zero = 0.0f32.to_bits();
        e.set(clone, NiPSysBombModifier::m_fDecay, zero);
        e.set(clone, NiPSysBombModifier::m_fDeltaV, zero);
        e.set(clone, NiPSysBombModifier::m_eDecayType, 0);
        e.set(clone, NiPSysBombModifier::m_eSymmType, 0);
    }
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    for word in 0..3 {
        let off = NiPSysBombModifier::m_kBombAxis.off + 4 * word;
        let value = e.mem.u32(this.addr() + off);
        e.mem.set_u32(clone.addr() + off, value);
    }
    for field in [NiPSysBombModifier::m_fDecay, NiPSysBombModifier::m_fDeltaV] {
        let value = x87_quiet_bits(e.get(this, field));
        e.set(clone, field, value);
    }
    for field in [
        NiPSysBombModifier::m_eDecayType,
        NiPSysBombModifier::m_eSymmType,
    ] {
        let value = e.get(this, field);
        e.set(clone, field, value);
    }
    clone
}

// Translated from 00c2e610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::ProcessClone` (Xbox PDB): `NiPSysModifier`'s
/// `ProcessClone`, then the clone of `this` from the process's map (into a
/// stack word cleared first); the clone's bomb object is the bomb object's
/// clone when the map has one (the lookup answers true), else the same
/// object (looked up even when it is null).
pub fn ni_p_sys_bomb_modifier_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    process: Ptr,
) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(process.addr());
        e.mem.set_u32(slot.addr(), 0);
        e.call(CLONE_MAP_LOOKUP, &args![map, this, slot]);
        let clone = Ptr::<NiPSysBombModifier>::new(e.mem.u32(slot.addr()));
        let object = e.get(this, NiPSysBombModifier::m_pkBombObj);
        let map = e.mem.u32(process.addr());
        if e.call(CLONE_MAP_LOOKUP, &args![map, object, slot]).bool() {
            let object_clone = e.mem.u32(slot.addr());
            e.set(clone, NiPSysBombModifier::m_pkBombObj, object_clone);
            return;
        }
        let object = e.get(this, NiPSysBombModifier::m_pkBombObj);
        e.set(clone, NiPSysBombModifier::m_pkBombObj, object);
    });
}

/// `FLD y; FLD x; FLD z; FLD ST1; FMULP ST2; FLD ST2; FMULP ST3; FXCH;
/// FADDP ST2,ST0; FMUL ST0; FADDP`: the squared length, then its square
/// root (`00ec6040`) stored, reloaded and stored again.
fn length(e: &mut Engine, x: u32, y: u32, z: u32) -> u32 {
    let (x, y, z) = (x87_load32(x), x87_load32(y), x87_load32(z));
    let sum = x87_add(x87_mul(y, y), x87_mul(x, x));
    let squared = x87_store32(x87_add(sum, x87_mul(z, z)));
    let root = e.call(SQRT, &args![x87_load32(squared)]).f64();
    x87_quiet_bits(x87_store32(root))
}

// Translated from 00c2e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::Update` (Xbox PDB): `thiscall`, `RET 8` (time,
/// particle data). The bomb's position starts as `NiPoint3::ZERO` and its
/// axis as `m_kBombAxis`; with particles and a bomb object, the position is
/// the object's in the particle system's space (inverse of the system's
/// transform at +0x68 times the object's) and the axis that transform's
/// rotation of `m_kBombAxis`, normalised (`004a0c10`). Each particle whose
/// last update is before `time` (unordered skips) and, with a decay type,
/// within `m_fDecay` of the bomb gets `m_fDeltaV * falloff * (time - last
/// update)` along a direction:
/// - symmetry 0: from the bomb, normalised (the distance kept);
/// - symmetry 1: away from the axis line (the offset minus its part along
///   the axis), normalised unless its length is zero; that length is the
///   distance;
/// - symmetry 2: the axis, or its opposite when the offset's dot product
///   with it is negative; the (positive) dot product is the distance;
/// - other: the direction the three stack words hold (the previous
///   particle's, or the frame's prior contents for the first; the engine's
///   frame starts zeroed).
///
/// The falloff is 1, `(decay - distance) / decay` (type 1) or
/// `exp(-distance / decay)` (type 2). The x87 operations are done
/// instruction by instruction; the stack frame keeps the exe's layout (the
/// transforms at +0x88, +0xbc, +0xf0, the axis at +0x20, the turned axis at
/// +0x58 and the direction at +0x14).
pub fn ni_p_sys_bomb_modifier_update(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    time: f32,
    data: Ptr<NiPSysData>,
) {
    e.with_stack(0x124, |e, frame| {
        let f = frame.addr();
        let mut bomb = [0u32; 3];
        for (i, word) in bomb.iter_mut().enumerate() {
            *word = e.global::<u32>(POINT_ZERO + 4 * i as u32);
        }
        let axis_field = this.addr() + NiPSysBombModifier::m_kBombAxis.off;
        for word in 0..3 {
            let value = e.mem.u32(axis_field + 4 * word);
            e.mem.set_u32(f + 0x20 + 4 * word, value);
        }
        let object = e.get(this, NiPSysBombModifier::m_pkBombObj);
        if e.get(data, NiPSysData::m_usActive) != 0 && object != 0 {
            let bytes = e.mem.bytes(object + OBJECT_TRANSFORM, TRANSFORM_SIZE);
            e.mem.write(f + 0xbc, &bytes);
            let system = e.mem.u32(this.addr() + NiPSysModifier::m_pkTarget.off);
            let bytes = e
                .mem
                .bytes(system.wrapping_add(OBJECT_TRANSFORM), TRANSFORM_SIZE);
            e.mem.write(f + 0x88, &bytes);
            e.call(TRANSFORM_INVERT, &args![f + 0x88, f + 0xf0]);
            e.call(TRANSFORM_MULTIPLY, &args![f + 0xf0, f + 0x88, f + 0xbc]);
            for (i, word) in bomb.iter_mut().enumerate() {
                *word = e.mem.u32(f + 0xac + 4 * i as u32);
            }
            let turned = e
                .call(MATRIX_TIMES_VECTOR, &args![f + 0x88, f + 0x58, axis_field])
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(turned + 4 * word);
                e.mem.set_u32(f + 0x20 + 4 * word, value);
            }
            e.call(UNITIZE, &args![f + 0x20]);
        }
        let mut i: u16 = 0;
        if i >= e.get(data, NiPSysData::m_usActive) {
            return;
        }
        let direction = f + 0x14;
        loop {
            bomb_particle(e, this, data, time, bomb, f, direction, u32::from(i));
            i = i.wrapping_add(1);
            if i >= e.get(data, NiPSysData::m_usActive) {
                break;
            }
        }
    });
}

/// One particle of `NiPSysBombModifier::Update` (the loop body).
fn bomb_particle(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    data: Ptr<NiPSysData>,
    time: f32,
    bomb: [u32; 3],
    frame: u32,
    direction: u32,
    i: u32,
) {
    let now = x87_load32(time.to_bits());
    let info = e.get(data, NiPSysData::m_pkParticleInfo) + i * 0x1c;
    let last = x87_load32(x87_quiet_bits(e.mem.u32(info + 0x14)));
    // `FLD time; FLD last; FCOM; TEST AH,5; JP`: on only when the last
    // update is before the time.
    if !(last < now) {
        return;
    }
    let elapsed = x87_store32(x87_sub(now, last));
    let position = e.get(data, NiPSysData::m_pkVertex) + i * 0xc;
    let offset: [u32; 3] = [0, 1, 2].map(|k| {
        x87_store32(x87_sub(
            x87_load32(e.mem.u32(position + 4 * k)),
            x87_m32(bomb[k as usize]),
        ))
    });
    let [dx, dy, dz] = offset;
    let mut distance = x87_load32(length(e, dx, dy, dz));
    let decay_type = e.get(this, NiPSysBombModifier::m_eDecayType);
    if decay_type != 0 {
        let decay = x87_load32(e.get(this, NiPSysBombModifier::m_fDecay));
        // `FLD decay; FCOMP; TEST AH,1; JNZ`: skipped when the decay is
        // below the distance (or they are unordered).
        if decay < distance || decay.is_nan() || distance.is_nan() {
            return;
        }
    }
    let axis = [0, 1, 2].map(|k| e.mem.u32(frame + 0x20 + 4 * k));
    let a = axis.map(x87_load32);
    match e.get(this, NiPSysBombModifier::m_eSymmType) {
        0 => {
            let inverse = x87_load32(x87_store32(x87_div(1.0, distance)));
            let d = [
                x87_store32(x87_mul(inverse, x87_m32(dx))),
                x87_store32(x87_mul(x87_load32(dy), inverse)),
                x87_store32(x87_mul(inverse, x87_m32(dz))),
            ];
            write_direction(e, direction, d);
        }
        1 => {
            let (lx, ly, lz) = (x87_load32(dx), x87_load32(dy), x87_load32(dz));
            let first = x87_add(x87_mul(a[1], ly), x87_mul(lx, x87_m32(axis[0])));
            let dot = x87_store32(x87_add(x87_mul(a[2], lz), first));
            let along = x87_load32(dot);
            let px = x87_store32(x87_mul(a[0], along));
            let py = x87_store32(x87_mul(a[1], along));
            let pz = x87_store32(x87_mul(along, a[2]));
            let rx = x87_store32(x87_sub(lx, x87_load32(px)));
            let ry = x87_store32(x87_sub(ly, x87_load32(py)));
            let rz = x87_store32(x87_sub(lz, x87_m32(pz)));
            write_direction(e, direction, [rx, ry, rz]);
            let len = length(e, rx, ry, rz);
            distance = x87_load32(len);
            // `FLDZ; FLD len; FUCOM; TEST AH,0x44; JNP`: not normalised
            // when the length is zero.
            if distance != 0.0 {
                let inverse = x87_load32(x87_store32(x87_div(1.0, distance)));
                let y = e.mem.u32(direction + 4);
                let z = e.mem.u32(direction + 8);
                let d = [
                    x87_store32(x87_mul(inverse, x87_m32(rx))),
                    x87_store32(x87_mul(x87_load32(y), inverse)),
                    x87_store32(x87_mul(inverse, x87_m32(z))),
                ];
                write_direction(e, direction, d);
            }
        }
        2 => {
            let first = x87_add(x87_mul(x87_load32(dx), a[0]), x87_mul(a[1], x87_m32(dy)));
            let dot = x87_store32(x87_add(first, x87_mul(a[2], x87_m32(dz))));
            let along = x87_load32(dot);
            if along < 0.0 {
                let flipped = x87_store32(-along);
                let d = [x87_store32(-a[0]), x87_store32(-a[1]), x87_store32(-a[2])];
                write_direction(e, direction, d);
                distance = x87_load32(flipped);
            } else {
                write_direction(e, direction, axis);
                distance = along;
            }
        }
        _ => {}
    }
    let decay = e.get(this, NiPSysBombModifier::m_fDecay);
    let falloff = match decay_type {
        1 => x87_store32(x87_div(x87_sub(x87_m32(decay), distance), x87_m32(decay))),
        2 => {
            let exponent = x87_store32(x87_div(-distance, x87_m32(decay)));
            let result = e.call(EXP, &args![x87_load32(exponent)]).f64();
            x87_quiet_bits(x87_store32(result))
        }
        _ => 1.0f32.to_bits(),
    };
    let k = x87_store32(x87_mul(
        x87_mul(
            x87_load32(e.get(this, NiPSysBombModifier::m_fDeltaV)),
            x87_m32(falloff),
        ),
        x87_m32(elapsed),
    ));
    let k = x87_load32(k);
    let d = [0, 1, 2].map(|j| e.mem.u32(direction + 4 * j));
    let push = [
        x87_store32(x87_mul(k, x87_m32(d[0]))),
        x87_store32(x87_mul(x87_load32(d[1]), k)),
        x87_store32(x87_mul(k, x87_m32(d[2]))),
    ];
    for (j, value) in push.iter().enumerate() {
        let slot = info + 4 * j as u32;
        let sum = x87_add(x87_load32(e.mem.u32(slot)), x87_m32(*value));
        e.mem.set_u32(slot, x87_store32(sum));
    }
}

fn write_direction(e: &mut Engine, direction: u32, d: [u32; 3]) {
    for (j, value) in d.iter().enumerate() {
        e.mem.set_u32(direction + 4 * j as u32, *value);
    }
}

// Translated from 00c2eb80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysBombModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, a link id (`00a645b0`, the bomb object), the axis
/// (`NiColor::LoadBinary`), decay and delta-v (4 bytes each), then the
/// decay and symmetry types (4 bytes each, read into a local and stored).
/// The component-size word is the stream argument's stack word.
pub fn ni_p_sys_bomb_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysBombModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_LOAD_BINARY, &args![this, s]);
    e.call(READ_LINK_ID, &args![s]);
    e.call(
        COLOR_LOAD_BINARY,
        &args![this.addr() + NiPSysBombModifier::m_kBombAxis.off, s],
    );
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        e.mem.set_u32(argument_slot, s);
        for field in [NiPSysBombModifier::m_fDecay, NiPSysBombModifier::m_fDeltaV] {
            stream_read(e, s, this.addr() + field.off, 4, 4, argument_slot);
        }
        for field in [
            NiPSysBombModifier::m_eDecayType,
            NiPSysBombModifier::m_eSymmType,
        ] {
            stream_read(e, s, local, 4, 4, argument_slot);
            let value = e.mem.u32(local);
            e.set(this, field, value);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_e520,
            ni_p_sys_bomb_modifier_link_object(Ptr<NiPSysBombModifier>, Ptr)
        ),
        entry!(
            0x00c2_e570,
            ni_p_sys_bomb_modifier_create_clone(
                Ptr<NiPSysBombModifier>,
                Ptr,
            ) -> Ptr<NiPSysBombModifier>
        ),
        entry!(
            0x00c2_e610,
            ni_p_sys_bomb_modifier_process_clone(Ptr<NiPSysBombModifier>, Ptr)
        ),
        entry!(
            0x00c2_e670,
            ni_p_sys_bomb_modifier_update(Ptr<NiPSysBombModifier>, f32, Ptr<NiPSysData>)
        ),
        entry!(
            0x00c2_eb80,
            ni_p_sys_bomb_modifier_load_binary(Ptr<NiPSysBombModifier>, Ptr)
        ),
        entry!(0x00c2e540, ni_p_sys_bomb_modifier_get_r_t_t_i() -> u32),
        entry!(0x00c2e550, ni_p_sys_bomb_modifier_scalar_deleting_destructor(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_p_sys_bomb_modifier_scalar_deleting_destructor_deletes_only_on_bit_0() {
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
        assert_eq!(e.call(0x00c2e550, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c2e550, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x38]]);
        assert_eq!(calls_to(&e, 0x00c3_1750), vec![vec![this]; 2]);
    }

    #[test]
    fn ni_p_sys_bomb_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_e540, &args![]).u32(), 0x0120_2790);
    }

    use crate::units::niparticle::nipsysmodifier::test_stream;

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
    fn link_object_takes_the_bomb_object() {
        let mut e = Engine::new();
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_OBJECT_FROM_LINK_ID, |_, _| Ret {
            eax: 0x6600,
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysBombModifier>();
        e.call(0x00c2_e520, &args![this, 0x7000u32]);
        assert_eq!(e.get(this, NiPSysBombModifier::m_pkBombObj), 0x6600);
    }

    #[test]
    fn create_clone_constructs_and_copies_all_but_the_object() {
        let mut e = Engine::new();
        e.map(DEFAULT_AXIS & !0xfff, 0x1000);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        let this = e.new_object::<NiPSysBombModifier>();
        e.set(this, NiPSysBombModifier::m_pkBombObj, 0x6600);
        e.set(this, NiPSysBombModifier::m_fDeltaV, 0x7f80_0001);
        e.set(this, NiPSysBombModifier::m_eSymmType, 2);
        e.mem.set_f32(this.addr() + 0x24, 5.0);
        let clone = e
            .call(0x00c2_e570, &args![this, 0x4000u32])
            .ptr::<NiPSysBombModifier>();
        assert_eq!(e.mem.u32(clone.addr()), VTABLE);
        assert_eq!(e.get(clone, NiPSysBombModifier::m_pkBombObj), 0);
        assert_eq!(e.get(clone, NiPSysBombModifier::m_fDeltaV), 0x7fc0_0001);
        assert_eq!(e.get(clone, NiPSysBombModifier::m_eSymmType), 2);
        assert_eq!(e.mem.f32(clone.addr() + 0x24), 5.0);
    }

    #[test]
    fn process_clone_maps_or_keeps_the_bomb_object() {
        for (found, expected) in [(true, 0x6100u32), (false, 0x6000)] {
            let mut e = Engine::new();
            e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
            let this = e.new_object::<NiPSysBombModifier>();
            let clone = e.new_object::<NiPSysBombModifier>();
            e.set(this, NiPSysBombModifier::m_pkBombObj, 0x6000);
            let map = e.mem.alloc(8);
            e.mem.set_u32(map, clone.addr());
            e.mem.set_u32(map + 4, found as u32);
            let process = e.mem.alloc(4);
            e.mem.set_u32(process, map);
            e.register(CLONE_MAP_LOOKUP, |e, a| {
                let found = if a[1] == 0x6000 {
                    let found = e.mem.u32(a[0] + 4);
                    if found != 0 {
                        e.mem.set_u32(a[2], 0x6100);
                    }
                    found
                } else {
                    let clone = e.mem.u32(a[0]);
                    e.mem.set_u32(a[2], clone);
                    1
                };
                Ret {
                    eax: found,
                    ..Ret::default()
                }
            });
            e.call(0x00c2_e610, &args![this, process]);
            assert_eq!(e.get(clone, NiPSysBombModifier::m_pkBombObj), expected);
        }
    }

    struct Scene {
        e: Engine,
        this: Ptr<NiPSysBombModifier>,
        data: Ptr<NiPSysData>,
    }

    /// A bomb at the origin (no bomb object) over particles at `positions`,
    /// last updated at 0; the square root and `exp` are the host's.
    fn scene(positions: &[[f32; 3]], decay_type: u32, symmetry: u32) -> Scene {
        let mut e = Engine::new();
        e.map(POINT_ZERO & !0xfff, 0x1000);
        e.register(SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e.register(EXP, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).exp(),
            ..Ret::default()
        });
        let this = e.new_object::<NiPSysBombModifier>();
        e.set(this, NiPSysBombModifier::m_fDeltaV, 2.0f32.to_bits());
        e.set(this, NiPSysBombModifier::m_fDecay, 10.0f32.to_bits());
        e.set(this, NiPSysBombModifier::m_eDecayType, decay_type);
        e.set(this, NiPSysBombModifier::m_eSymmType, symmetry);
        e.mem.set_f32(this.addr() + 0x24, 1.0);
        let data = e.new_object::<NiPSysData>();
        let n = positions.len() as u32;
        let vertices = e.mem.alloc(12 * n);
        for (i, p) in positions.iter().enumerate() {
            for (k, v) in p.iter().enumerate() {
                e.mem.set_f32(vertices + 12 * i as u32 + 4 * k as u32, *v);
            }
        }
        let info = e.mem.alloc(0x1c * n);
        e.set(data, NiPSysData::m_pkVertex, vertices);
        e.set(data, NiPSysData::m_pkParticleInfo, info);
        e.set(data, NiPSysData::m_usActive, n as u16);
        Scene { e, this, data }
    }

    impl Scene {
        fn update(&mut self, time: f32) {
            let (this, data) = (self.this, self.data);
            self.e.call(0x00c2_e670, &args![this, time, data]);
        }

        fn velocity(&self, i: u32) -> [f32; 3] {
            let info = self.e.get(self.data, NiPSysData::m_pkParticleInfo) + 0x1c * i;
            [0, 4, 8].map(|o| self.e.mem.f32(info + o))
        }
    }

    fn f(v: f64) -> f64 {
        f64::from(v as f32)
    }

    #[test]
    fn spherical_push_away_from_the_bomb() {
        let mut s = scene(&[[3.0, 4.0, 0.0]], 0, 0);
        s.update(1.0);
        let inverse = f(1.0 / 5.0);
        let v = s.velocity(0);
        assert_eq!(v[0], (2.0 * f(inverse * 3.0)) as f32);
        assert_eq!(v[1], (2.0 * f(4.0 * inverse)) as f32);
        assert_eq!(v[2], 0.0);
        // A particle already updated at the time is left alone.
        let mut s = scene(&[[3.0, 4.0, 0.0]], 0, 0);
        let info = s.e.get(s.data, NiPSysData::m_pkParticleInfo);
        s.e.mem.set_f32(info + 0x14, 1.0);
        s.update(1.0);
        assert_eq!(s.velocity(0), [0.0; 3]);
    }

    #[test]
    fn linear_decay_and_its_range() {
        let mut s = scene(&[[3.0, 4.0, 0.0], [30.0, 40.0, 0.0]], 1, 0);
        s.update(1.0);
        let falloff = f((10.0 - 5.0) / 10.0);
        let k = f(2.0 * falloff);
        let inverse = f(1.0 / 5.0);
        assert_eq!(s.velocity(0)[0], (k * f(inverse * 3.0)) as f32);
        // Beyond the decay distance: nothing.
        assert_eq!(s.velocity(1), [0.0; 3]);
    }

    #[test]
    fn cylindrical_pushes_away_from_the_axis_line() {
        let mut s = scene(&[[3.0, 4.0, 7.0]], 0, 1);
        s.update(1.0);
        let inverse = f(1.0 / 5.0);
        let v = s.velocity(0);
        assert_eq!(v[0], (2.0 * f(inverse * 3.0)) as f32);
        assert_eq!(v[1], (2.0 * f(4.0 * inverse)) as f32);
        assert_eq!(v[2], 0.0);
        // On the axis: the zero offset is not normalised.
        let mut s = scene(&[[0.0, 0.0, 7.0]], 0, 1);
        s.update(1.0);
        assert_eq!(s.velocity(0), [0.0; 3]);
    }

    #[test]
    fn planar_pushes_along_the_axis_by_side_with_exponential_decay() {
        let mut s = scene(&[[1.0, 2.0, -3.0], [0.0, 0.0, 3.0]], 2, 2);
        s.e.call_log = Some(vec![]);
        s.update(1.0);
        let falloff = f((-3.0f64 / 10.0).exp());
        let k = (2.0 * falloff) as f32;
        assert_eq!(s.velocity(0), [0.0, 0.0, -k]);
        assert_eq!(s.velocity(1), [0.0, 0.0, k]);
        let exp = log_of(&s.e, EXP);
        assert_eq!(exp.len(), 2);
    }

    #[test]
    fn other_symmetry_uses_the_stack_direction() {
        let mut s = scene(&[[3.0, 4.0, 0.0]], 0, 3);
        s.update(1.0);
        assert_eq!(s.velocity(0), [0.0; 3]);
    }

    #[test]
    fn bomb_object_gives_the_position_and_the_turned_axis() {
        let mut s = scene(&[[3.0, 4.0, 5.0]], 0, 0);
        let object = s.e.mem.alloc(0xa0);
        let system = s.e.mem.alloc(0xa0);
        s.e.mem.set_u32(s.this.addr() + 0x10, system);
        s.e.set(s.this, NiPSysBombModifier::m_pkBombObj, object);
        s.e.register(TRANSFORM_INVERT, |_, _| Ret::default());
        s.e.register(TRANSFORM_MULTIPLY, |e, a| {
            e.mem.set_f32(a[1] + 0x2c, 5.0);
            Ret::default()
        });
        s.e.register(MATRIX_TIMES_VECTOR, |e, a| {
            let v = e.mem.bytes(a[2], 12);
            e.mem.write(a[1], &v);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        s.e.register(UNITIZE, |_, _| Ret::default());
        s.e.call_log = Some(vec![]);
        s.update(1.0);
        let turn = log_of(&s.e, MATRIX_TIMES_VECTOR);
        assert_eq!(turn[0][2], s.this.addr() + 0x1c);
        assert_eq!(log_of(&s.e, UNITIZE).len(), 1);
        // The bomb sits at (0, 0, 5): the particle is 5 away along x and y.
        let inverse = f(1.0 / 5.0);
        assert_eq!(s.velocity(0)[0], (2.0 * f(inverse * 3.0)) as f32);
        assert_eq!(s.velocity(0)[2], 0.0);
    }

    #[test]
    fn load_binary_reads_the_bomb_fields() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_LINK_ID, |_, _| Ret::default());
        e.register(COLOR_LOAD_BINARY, |_, _| Ret::default());
        let mut bytes: Vec<u8> = [0.5f32, 2.0].iter().flat_map(|v| v.to_le_bytes()).collect();
        bytes.extend_from_slice(&1u32.to_le_bytes());
        bytes.extend_from_slice(&2u32.to_le_bytes());
        let stream = test_stream::new(&mut e, 0x1402_0007, &bytes);
        let this = e.new_object::<NiPSysBombModifier>();
        e.call_log = Some(vec![]);
        e.call(0x00c2_eb80, &args![this, stream]);
        assert_eq!(log_of(&e, READ_LINK_ID), vec![vec![stream]]);
        assert_eq!(
            log_of(&e, COLOR_LOAD_BINARY),
            vec![vec![this.addr() + 0x1c, stream]]
        );
        assert_eq!(e.get(this, NiPSysBombModifier::m_fDeltaV), 2.0f32.to_bits());
        assert_eq!(e.get(this, NiPSysBombModifier::m_eDecayType), 1);
        assert_eq!(e.get(this, NiPSysBombModifier::m_eSymmType), 2);
    }
}
