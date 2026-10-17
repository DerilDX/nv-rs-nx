//! `Havok SDK/hkaskeletonmapperutils.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkaSkeletonMapperUtils` (Xbox PDB name) builds the mapping between two skeletons. A
//! transform is 0x30 bytes: translation `+0`, rotation quaternion `+0x10`, scale `+0x20` (four
//! lanes each, the last lane of the scale is not used). The chain mapping the functions fill
//! is 0x70 bytes: four `u16` bone indices (start and end bone of each skeleton), a transform at
//! `+0x10` and a second one at `+0x40`; its class is unconfirmed. The class `Params` has its
//! copy constructor here (its field names are not in the Xbox PDB, so the fields are described
//! by offset). SSE code computes lane by lane in `f32`; a sum of three squares is
//! `z + (y + x)` the way the `SHUFPS`/`ADDSS` sequences add it; `RCPPS` and `RSQRTSS` are the
//! host CPU's estimates. x87 code computes in `f64` and rounds at each `float` store; a float
//! loaded through `FLD` goes through [`x87_quiet`].
//!
//! Functions that report an error build the message in a report builder (`00cb1600`, `00cc01c0`,
//! `00cc00e0`) and send it to the sink at `01268360` (slot `0xc`).
//!
//! Not translated: none of these functions has a C++ exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkp3axissweep::{ARRAY_RESERVE, MEMORY_ROUTER};
use crate::units::havok_sdk::hkpboxmotion::{add, cross, mul, splat, sub};
use crate::units::havok_sdk::hkpconvexradiusbuilder::normalising_scale;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, refined_reciprocal, shufps, store_bits, store_vec, sum3, unpckhps, Vec4,
    HALF_F32, ONES, SIGN_MASK, THREE_F32, ZERO_F32,
};

/// `(0, 0, 0, 1)`, the identity quaternion, and the `float` `1.0` the scales start from.
pub(crate) const IDENTITY_QUATERNION: u32 = 0x010c_71b0;
const FLOAT_ONE: u32 = 0x0101_2068;
/// Four `-0.5`: the term added to `w * w` in the quaternion rotation.
pub(crate) const ROTATION_BIAS: u32 = 0x010c_71e0;
/// Sign mask of the first three lanes (the conjugate), and the all-ones mask of the first three
/// lanes (lane 3 zero).
pub(crate) const CONJUGATE_MASK: u32 = 0x010c_4c00;
pub(crate) const XYZ_MASK: u32 = 0x010c_4c10;
/// The `float` the chain mapping uses as its length tolerance.
const CHAIN_TOLERANCE: u32 = 0x0107_18c0;
/// `hkQuaternion::setMul` (`00c66320`, `RET 8`): `this` = result, `(left, right)`.
pub(crate) const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
/// The quaternion operation `00c74ac0` (`RET 8`, no name in the engine map): `this` = result,
/// `(first, second)`.
const QUATERNION_SET_MUL_2: u32 = 0x00c7_4ac0;
/// The quaternion operation `00c74b70` (`RET 8`, no name in the engine map): `this` = result,
/// `(first, second)`.
const QUATERNION_SET_MUL_3: u32 = 0x00c7_4b70;
/// The rotation between two directions (`00cd9d10`, `RET 8`, no name in the engine map): `this` =
/// result, `(first, second)`.
const ROTATION_BETWEEN: u32 = 0x00cd_9d10;
/// The chain end point of a bone (`00ce5560`, cdecl, no name in the engine map): `(bones, index,
/// first output vector, second output vector)`.
const BONE_POINTS: u32 = 0x00ce_5560;
/// The mapping validity check (`00ce50e0`, `RET 4`, `hkqstransform.cpp`, no name in the engine
/// map): `this` = the mapping, `(byte)`; answers a pointer to the byte.
const MAPPING_IS_VALID: u32 = 0x00ce_50e0;
/// The report builder: constructor (`00cb1600`, `this`, buffer, size), text append (`00cc01c0`,
/// `this`, text) and destructor (`00cc00e0`, `this`); the global at `01268360` holds the sink
/// object whose slot `0xc` takes `(1, id, text, file, line)`.
const REPORT_BUILDER_CONSTRUCT: u32 = 0x00cb_1600;
const REPORT_BUILDER_TEXT: u32 = 0x00cc_01c0;
const REPORT_BUILDER_DESTRUCT: u32 = 0x00cc_00e0;
const REPORT_SINK: u32 = 0x0126_8360;
/// `".\Mapper\hkaSkeletonMapperUtils.cpp"` and the messages of `computeMappingTransform`.
const REPORT_FILE: u32 = 0x010c_bab0;
const MESSAGE_TOLERANCE: u32 = 0x010c_bbd8;
const MESSAGE_ZERO_LENGTH: u32 = 0x010c_bb78;
const MESSAGE_UNSUPPORTED: u32 = 0x010c_bb58;
const MESSAGE_INVALID: u32 = 0x010c_bb44;

/// The report the functions of this unit write: a builder over a 0x200-byte buffer holding
/// `message`, sent to the sink at `01268360` (slot `0xc`) with `(1, id, text, file, line)`,
/// then destroyed.
fn report(e: &mut Engine, message: u32, id: u32, line: u32) {
    e.with_stack(0x40, |e, builder| {
        e.with_stack(0x20c, |e, buffer| {
            let (builder, buffer) = (builder.addr(), buffer.addr());
            e.call(REPORT_BUILDER_CONSTRUCT, &args![builder, buffer, 0x200u32]);
            e.call(REPORT_BUILDER_TEXT, &args![builder, message]);
            let sink = e.global::<u32>(REPORT_SINK);
            e.vcall(sink, 0xc, &args![1u32, id, buffer, REPORT_FILE, line]);
            e.call(REPORT_BUILDER_DESTRUCT, &args![builder]);
        });
    });
}

/// `XORPS`: the bits of each lane xor the mask.
pub(crate) fn xor_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// `ANDPS`: the bits of each lane and the mask.
pub(crate) fn and_bits(v: Vec4, mask: [u32; 4]) -> Vec4 {
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() & mask[lane]))
}

/// `SQRTSS` of the sum of the first three squares of `v`.
fn length_of(v: Vec4) -> f32 {
    sum3(mul(v, v)).sqrt()
}

/// `2 * (cross * w + ((w * w + bias) * v + dot * q))` where `w` is lane 3 of the quaternion `q`
/// and `dot` the sum of the first three lanes of `q * v`: the quaternion rotation of `v` the
/// code unrolls, `cross_term` being the cross product in the operand order of the caller.
pub(crate) fn rotate_twice(cross_term: Vec4, v: Vec4, q: Vec4, bias: Vec4) -> Vec4 {
    let dot = sum3(mul(q, v));
    let w = splat(q, 3);
    let weighted = add(mul(w, w), bias);
    let r = add(mul(cross_term, w), add(mul(weighted, v), mul([dot; 4], q)));
    add(r, r)
}

/// The identity transform: translation zero, rotation `(0, 0, 0, 1)` from `010c71b0`, scale
/// the `float` at `01012068` in all four lanes.
fn store_identity_transform(e: &mut Engine, at: u32) {
    store_bits(e, at, [0; 4]);
    let rotation = load_bits(e, IDENTITY_QUATERNION);
    store_bits(e, at + 0x10, rotation);
    let one = e.mem.u32(FLOAT_ONE);
    store_bits(e, at + 0x20, [one; 4]);
}

// Translated from 00ce5430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 0x70-byte chain mapping (unconfirmed class, no name in the engine map):
/// the four bone indices zero and two identity transforms, at `+0x10` and `+0x40`. Returns
/// `this`.
pub fn fn_00ce5430(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at, 0);
    e.mem.set_u16(at + 4, 0);
    e.mem.set_u16(at + 2, 0);
    e.mem.set_u16(at + 6, 0);
    store_identity_transform(e, at + 0x10);
    store_identity_transform(e, at + 0x40);
    this
}

// Translated from 00ce55d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scale factors between two skeletons for one mapping (no name in the engine map; cdecl, eight
/// words). Both outputs (`0x30` bytes each) start as identity transforms. `parameters` is the
/// mapper parameter block: `+0` the mode, `+4` and `+8` the bone indices of mode 1, `+0xC` and
/// `+0x10` two `float`s. Mode 0 measures the distance of the two points `00ce5560` gives for
/// each skeleton, mode 1 the length of the translation of one bone of each, any other mode uses
/// the two floats alone. The scale lanes (`+0x20`) of the outputs are `(L2 / L1 * +0x10) / +0xC`
/// and `(L1 / L2 * +0xC) / +0x10` (x87, rounded to `float`), or `+0x10 / +0xC` and `+0xC / +0x10`
/// without lengths. The second word is not read.
pub fn fn_00ce55d0(
    e: &mut Engine,
    parameters: Ptr,
    _unused_1: u32,
    bones_a: Ptr,
    index_a: u32,
    bones_b: Ptr,
    index_b: u32,
    out_a: Ptr,
    out_b: Ptr,
) {
    store_identity_transform(e, out_a.addr());
    store_identity_transform(e, out_b.addr());
    let params = parameters.addr();
    let mode = e.mem.u32(params);
    let lengths = match mode {
        0 => Some(e.with_stack(0x40, |e, block| {
            let at = block.addr();
            e.call(BONE_POINTS, &args![bones_a, index_a, at + 0x10, at]);
            e.call(BONE_POINTS, &args![bones_b, index_b, at + 0x30, at + 0x20]);
            let first = sub(load_vec(e, at), load_vec(e, at + 0x10));
            let second = sub(load_vec(e, at + 0x20), load_vec(e, at + 0x30));
            (length_of(first), length_of(second))
        })),
        1 => {
            let slot_a = e.mem.u32(params + 4).wrapping_mul(0x30);
            let slot_b = e.mem.u32(params + 8).wrapping_mul(0x30);
            let first = load_vec(e, bones_a.addr().wrapping_add(slot_a));
            let second = load_vec(e, bones_b.addr().wrapping_add(slot_b));
            Some((length_of(first), length_of(second)))
        }
        _ => None,
    };
    let c = f64::from(x87_quiet(e.mem.f32(params + 0xc)));
    let d = f64::from(x87_quiet(e.mem.f32(params + 0x10)));
    let (scale_a, scale_b) = match lengths {
        Some((first, second)) => {
            let first = f64::from(x87_quiet(first));
            let second = f64::from(x87_quiet(second));
            (
                (second / first * d / c) as f32,
                (first / second * c / d) as f32,
            )
        }
        None => ((d / c) as f32, (c / d) as f32),
    };
    store_bits(e, out_a.addr() + 0x20, [scale_a.to_bits(); 4]);
    store_bits(e, out_b.addr() + 0x20, [scale_b.to_bits(); 4]);
}

/// `out = inverse(a) * b` for two transforms (the mode 0 of `computeMappingTransform`, also used
/// by the pose conversions of `hkaSkeletonUtils`): translation
/// `rotate(conj(a.rotation), b.translation) - rotate(a.rotation, a.translation)` (the unrolled
/// quaternion rotation, doubled), rotation `conj(a.rotation) * b.rotation` (`00c66320`), scale
/// `b.scale * (1 / a.scale)` (`RCPPS` plus one Newton step, lane 3 cleared).
pub(crate) fn inverse_compose(e: &mut Engine, a: u32, b: u32, out: u32) {
    let bias = load_vec(e, ROTATION_BIAS);
    let translation = load_vec(e, a);
    let rotation = load_vec(e, a + 0x10);
    let scale = load_vec(e, a + 0x20);
    let forward = sub(
        mul(
            shufps(translation, translation, 0xc9),
            shufps(rotation, rotation, 0xd2),
        ),
        mul(
            shufps(translation, translation, 0xd2),
            shufps(rotation, rotation, 0xc9),
        ),
    );
    let moved = rotate_twice(forward, translation, rotation, bias);
    let negated = xor_bits(moved, load_bits(e, SIGN_MASK));
    let conjugate = xor_bits(load_vec(e, a + 0x10), load_bits(e, CONJUGATE_MASK));
    let ones = load_vec(e, ONES);
    let reciprocal_input = shufps(scale, unpckhps(scale, ones), 0xc4);
    let inverse_scale = and_bits(
        reciprocal_input.map(refined_reciprocal),
        load_bits(e, XYZ_MASK),
    );
    let b_translation = load_vec(e, b);
    let backward = rotate_twice(
        cross(conjugate, b_translation),
        b_translation,
        conjugate,
        bias,
    );
    store_vec(e, out, add(backward, negated));
    e.with_stack(0x10, |e, copy| {
        store_vec(e, copy.addr(), conjugate);
        e.call(
            QUATERNION_SET_MUL,
            &args![out + 0x10, copy.addr(), b + 0x10],
        );
    });
    let b_scale = load_vec(e, b + 0x20);
    store_vec(e, out + 0x20, mul(b_scale, inverse_scale));
}

// Translated from 00ce5a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::computeMappingTransform` (Xbox PDB name; cdecl): the transform `out`
/// that maps the bone transform `a` onto `b`. Mode 0: translation
/// `rotate(conj(a.rotation), b.translation) - rotate(a.rotation, a.translation)` (the unrolled
/// quaternion rotation, doubled), rotation `conj(a.rotation) * b.rotation` (`00c66320`), scale
/// `b.scale * (1 / a.scale)` (`RCPPS` plus one Newton step, lane 3 cleared). Mode 1 scales by
/// the ratio of the lengths of the two translations (`fallback_scale` when a length is not above
/// `tolerance`): rotation by `00c74ac0`, translation `b.translation - scale * a.translation`;
/// a negative tolerance, one zero length only, and any other mode are reported (ids
/// `abba39c3`, `abba2b94`, `abbaef91`). The transform is then checked (`00ce50e0`) and a
/// report (`abba10fc`) is sent when it is not valid.
pub fn hka_skeleton_mapper_utils_compute_mapping_transform(
    e: &mut Engine,
    mode: u32,
    a: Ptr,
    b: Ptr,
    out: Ptr,
    tolerance: f32,
    fallback_scale: f32,
) {
    let (a, b, out) = (a.addr(), b.addr(), out.addr());
    if mode == 0 {
        inverse_compose(e, a, b, out);
    } else if mode == 1 {
        let tolerance = x87_quiet(tolerance);
        if tolerance < 0.0 {
            report(e, MESSAGE_TOLERANCE, 0xabba_39c3, 0x34e);
        }
        let length_a = length_of(load_vec(e, a));
        let length_b = length_of(load_vec(e, b));
        let small_a = length_a <= tolerance;
        let small_b = length_b <= tolerance;
        let scale = if !small_a && !small_b {
            (f64::from(length_b) / f64::from(length_a)) as f32
        } else {
            if small_a != small_b {
                report(e, MESSAGE_ZERO_LENGTH, 0xabba_2b94, 0x35c);
            }
            x87_quiet(fallback_scale)
        };
        store_bits(e, out + 0x20, [scale.to_bits(); 4]);
        e.call(QUATERNION_SET_MUL_2, &args![out + 0x10, b + 0x10, a + 0x10]);
        let negated = f32::from_bits(scale.to_bits() ^ 0x8000_0000);
        let moved = mul([negated; 4], load_vec(e, a));
        store_vec(e, out, add(moved, load_vec(e, b)));
    } else {
        report(e, MESSAGE_UNSUPPORTED, 0xabba_ef91, 0x36c);
    }
    let valid = e.with_stack(4, |e, flag| {
        let answer = e.call(MAPPING_IS_VALID, &args![out, flag.addr()]).u32();
        e.mem.u8(answer)
    });
    if valid == 0 {
        report(e, MESSAGE_INVALID, 0xabba_10fc, 0x371);
    }
}

// Translated from 00ce5e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks a bone chain of a skeleton from bone `start` up its parents (the `i16` array whose data
/// pointer is at `skeleton + 0xC`) while the bone is above `stop` (signed compare). Starting
/// from a zero vector, each bone's transform (`transforms`, 0x30 bytes each) rotates the vector
/// by its quaternion and adds its translation (`out_vector`); `out_length` is the sum of the
/// translation lengths (x87 sum, stored as `float`). No name in the engine map.
pub fn fn_00ce5e60(
    e: &mut Engine,
    skeleton: Ptr,
    transforms: Ptr,
    stop: i16,
    start: i16,
    out_length: Ptr,
    out_vector: Ptr,
) {
    let (length_at, vector_at) = (out_length.addr(), out_vector.addr());
    e.mem.set_u32(length_at, 0);
    store_bits(e, vector_at, [0; 4]);
    let mut index = start;
    while index > stop {
        let slot = i32::from(index).wrapping_mul(0x30) as u32;
        let at = transforms.addr().wrapping_add(slot);
        let current = load_vec(e, vector_at);
        let rotation = load_vec(e, at + 0x10);
        let bias = load_vec(e, ROTATION_BIAS);
        let moved = rotate_twice(cross(rotation, current), current, rotation, bias);
        store_vec(e, vector_at, moved);
        let translation = load_vec(e, at);
        store_vec(e, vector_at, add(moved, translation));
        let total = f64::from(x87_quiet(e.mem.f32(length_at)));
        let step = f64::from(x87_quiet(length_of(load_vec(e, at))));
        e.mem.set_f32(length_at, (total + step) as f32);
        let parents = e.mem.u32(skeleton.addr() + 0xc);
        let parent_slot = (i32::from(index) as u32).wrapping_mul(2);
        index = e.mem.u16(parents.wrapping_add(parent_slot)) as i16;
    }
}

// Translated from 00ce5f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::computeChainMapping` (Xbox PDB name; cdecl): the mapping transform
/// of bone `index_a` of `bones_a` onto bone `index_b` of `bones_b` (0x30-byte transforms);
/// `tolerance` and `fallback_scale` go through `FLD`/`FSTP`.
pub fn hka_skeleton_mapper_utils_compute_chain_mapping(
    e: &mut Engine,
    mode: u32,
    bones_a: Ptr,
    index_a: i16,
    bones_b: Ptr,
    index_b: i16,
    out: Ptr,
    tolerance: f32,
    fallback_scale: f32,
) {
    let slot_a = i32::from(index_a).wrapping_mul(0x30) as u32;
    let slot_b = i32::from(index_b).wrapping_mul(0x30) as u32;
    hka_skeleton_mapper_utils_compute_mapping_transform(
        e,
        mode,
        Ptr::new(bones_a.addr().wrapping_add(slot_a)),
        Ptr::new(bones_b.addr().wrapping_add(slot_b)),
        out,
        x87_quiet(tolerance),
        x87_quiet(fallback_scale),
    );
}

// Translated from 00ce5fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills one chain mapping `out` (no name in the engine map; cdecl, ten words) for a chain of
/// skeleton A from bone `start_a` to `end_a` and one of skeleton B from `start_b` to `end_b`:
/// the four bone indices (low words), the mapping transform of the start bones at `+0x10`
/// (tolerance from `010718c0`, fallback scale 1.0), and for the end: the rotation between the
/// two normalised chain vectors (`00cd9d10`) at `+0x40`, a quaternion from `00c74b70` over the
/// end bones' rotations at `+0x50`, and the reciprocal of A's chain length (x87, as a
/// `float`) in all lanes at `+0x60`. The chain vectors come from `00ce5e60`; the normalisation
/// is the refined `RSQRTSS`, zero for a zero sum.
pub fn fn_00ce5fe0(
    e: &mut Engine,
    mode: u32,
    skeleton_a: Ptr,
    bones_a: Ptr,
    start_a: u32,
    end_a: u32,
    skeleton_b: Ptr,
    bones_b: Ptr,
    start_b: u32,
    end_b: u32,
    out: Ptr,
) {
    let at = out.addr();
    e.mem.set_u16(at + 4, start_b as u16);
    e.mem.set_u16(at + 6, end_b as u16);
    e.mem.set_u16(at, start_a as u16);
    e.mem.set_u16(at + 2, end_a as u16);
    let tolerance = f32::from_bits(x87_quiet_bits(e.mem.u32(CHAIN_TOLERANCE)));
    hka_skeleton_mapper_utils_compute_chain_mapping(
        e,
        mode,
        bones_a,
        start_a as i16,
        bones_b,
        start_b as i16,
        Ptr::new(at + 0x10),
        tolerance,
        1.0,
    );
    e.with_stack(0x50, |e, block| {
        let base = block.addr();
        let (length_a, length_b) = (base, base + 4);
        let (vector_a, vector_b, rotation) = (base + 0x10, base + 0x20, base + 0x30);
        fn_00ce5e60(
            e,
            skeleton_a,
            bones_a,
            start_a as i16,
            end_a as i16,
            Ptr::new(length_a),
            Ptr::new(vector_a),
        );
        fn_00ce5e60(
            e,
            skeleton_b,
            bones_b,
            start_b as i16,
            end_b as i16,
            Ptr::new(length_b),
            Ptr::new(vector_b),
        );
        let zero = e.mem.f32(ZERO_F32);
        let half = e.mem.f32(HALF_F32);
        let three = e.mem.f32(THREE_F32);
        let chain_a = load_vec(e, vector_a);
        let scale = normalising_scale(sum3(mul(chain_a, chain_a)), zero, half, three);
        store_vec(e, vector_a, mul([scale; 4], chain_a));
        let chain_b = load_vec(e, vector_b);
        let scale = normalising_scale(sum3(mul(chain_b, chain_b)), zero, half, three);
        store_vec(e, vector_b, mul([scale; 4], chain_b));
        e.call(ROTATION_BETWEEN, &args![rotation, vector_b, vector_a]);
        let reciprocal = (1.0 / f64::from(x87_quiet(e.mem.f32(length_a)))) as f32;
        let rotation_value = load_bits(e, rotation);
        store_bits(e, at + 0x40, rotation_value);
        store_bits(e, at + 0x60, [reciprocal.to_bits(); 4]);
        let slot_end_a = i32::from(end_a as i16).wrapping_mul(0x30) as u32;
        let slot_end_b = i32::from(end_b as i16).wrapping_mul(0x30) as u32;
        let rotation_a = bones_a.addr().wrapping_add(slot_end_a) + 0x10;
        let rotation_b = bones_b.addr().wrapping_add(slot_end_b) + 0x10;
        e.call(
            QUATERNION_SET_MUL_3,
            &args![at + 0x50, rotation_a, rotation_b],
        );
    });
}

// Translated from 00ce9580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkaSkeletonMapperUtils::Params::Params` (Xbox PDB name), the copy constructor (`RET 4`,
/// `this` and the source): `+0..0xC` copied, the floats at `+0xC` and `+0x10` through
/// `FLD`/`FSTP`, three empty `hkArray`s (data, size, capacity and flags `0x80000000`) at
/// `+0x14`, `+0x20` and `+0x2C` filled with the source's 8-byte elements (reserved through
/// `hkArrayUtil::_reserve`), the bytes at `+0x38` and `+0x39` and the words `+0x3C..0x50`.
/// Returns `this`.
pub fn hka_skeleton_mapper_utils_params_params(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    for offset in [0, 4, 8] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0xc, 0x10] {
        let word = x87_quiet_bits(e.mem.u32(from + offset));
        e.mem.set_u32(to + offset, word);
    }
    for array in [0x14, 0x20, 0x2c] {
        e.mem.set_u32(to + array, 0);
        e.mem.set_u32(to + array + 4, 0);
        e.mem.set_u32(to + array + 8, 0x8000_0000);
    }
    let flag = e.mem.u8(from + 0x38);
    e.mem.set_u8(to + 0x38, flag);
    let flag = e.mem.u8(from + 0x39);
    e.mem.set_u8(to + 0x39, flag);
    for offset in [0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for array in [0x14, 0x20, 0x2c] {
        copy_pair_array(e, to + array, from + array);
    }
    this
}

/// Fills the empty `hkArray` of 8-byte elements at `array` (data, size, capacity and flags)
/// from the one at `source`: the capacity is raised to the source's size (`_reserve` with
/// twice the capacity when that is larger), the first `min(size, source size)` elements and then
/// the rest are copied (a null destination is skipped), and the size is the source's.
fn copy_pair_array(e: &mut Engine, array: u32, source: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let source_size = e.mem.i32(source + 4);
    let size = e.mem.i32(array + 4);
    let common = if source_size > size {
        size
    } else {
        source_size
    };
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < source_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if source_size < doubled {
            doubled
        } else {
            source_size
        };
        e.call(ARRAY_RESERVE, &args![allocator, array, wanted, 8u32]);
    }
    let source_data = e.mem.u32(source);
    let data = e.mem.u32(array);
    if common > 0 {
        for index in 0..common as u32 {
            let word = e.mem.u32(source_data.wrapping_add(8 * index));
            e.mem.set_u32(data.wrapping_add(8 * index), word);
            let word = e.mem.u32(source_data.wrapping_add(8 * index + 4));
            e.mem.set_u32(data.wrapping_add(8 * index + 4), word);
        }
    }
    let mut from = source_data.wrapping_add((common as u32).wrapping_mul(8));
    let mut to = data.wrapping_add((common as u32).wrapping_mul(8));
    let mut remaining = source_size.wrapping_sub(common);
    while remaining > 0 {
        if to != 0 {
            let word = e.mem.u32(from);
            e.mem.set_u32(to, word);
            let word = e.mem.u32(from + 4);
            e.mem.set_u32(to + 4, word);
        }
        from = from.wrapping_add(8);
        to = to.wrapping_add(8);
        remaining -= 1;
    }
    e.mem.set_i32(array + 4, source_size);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_5430, fn_00ce5430(Ptr) -> Ptr),
        entry!(
            0x00ce_55d0,
            fn_00ce55d0(Ptr, u32, Ptr, u32, Ptr, u32, Ptr, Ptr)
        ),
        entry!(
            0x00ce_5a70,
            hka_skeleton_mapper_utils_compute_mapping_transform(u32, Ptr, Ptr, Ptr, f32, f32)
        ),
        entry!(0x00ce_5e60, fn_00ce5e60(Ptr, Ptr, i16, i16, Ptr, Ptr)),
        entry!(
            0x00ce_5f90,
            hka_skeleton_mapper_utils_compute_chain_mapping(u32, Ptr, i16, Ptr, i16, Ptr, f32, f32)
        ),
        entry!(
            0x00ce_5fe0,
            fn_00ce5fe0(u32, Ptr, Ptr, u32, u32, Ptr, Ptr, u32, u32, Ptr)
        ),
        entry!(0x00ce_9580, hka_skeleton_mapper_utils_params_params(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SINK_SLOT: u32 = 0x0900_00c0;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        store_bits(&mut e, IDENTITY_QUATERNION, [0, 0, 0, 0x3f80_0000]);
        e.mem.set_f32(FLOAT_ONE, 1.0);
        store_vec(&mut e, ROTATION_BIAS, [-0.5; 4]);
        store_vec(&mut e, ONES, [1.0; 4]);
        store_bits(&mut e, SIGN_MASK, [0x8000_0000; 4]);
        store_bits(
            &mut e,
            CONJUGATE_MASK,
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0],
        );
        store_bits(&mut e, XYZ_MASK, [u32::MAX, u32::MAX, u32::MAX, 0]);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(HALF_F32, 0.5);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_u32(CHAIN_TOLERANCE, 0x3727_c5ac);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The report builder and the sink at `01268360` as doubles.
    fn install_report(e: &mut Engine) {
        for address in [
            REPORT_BUILDER_CONSTRUCT,
            REPORT_BUILDER_TEXT,
            REPORT_BUILDER_DESTRUCT,
            SINK_SLOT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, SINK_SLOT);
        let sink = e.mem.alloc(0x10);
        e.mem.set_u32(sink, vtable);
        e.mem.set_u32(REPORT_SINK, sink);
    }

    /// The reports sent to the sink, as `(id, line)`.
    fn reports(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, SINK_SLOT)
            .into_iter()
            .map(|args| {
                assert_eq!(args[1], 1);
                assert_eq!(args[4], REPORT_FILE);
                (args[2], args[5])
            })
            .collect()
    }

    /// The validity check answers `valid` through the byte it is given.
    fn install_validity(e: &mut Engine, valid: bool) {
        if valid {
            e.register(MAPPING_IS_VALID, |e, a| {
                e.mem.set_u8(a[1], 1);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        } else {
            e.register(MAPPING_IS_VALID, |e, a| {
                e.mem.set_u8(a[1], 0);
                Ret {
                    eax: a[1],
                    ..Ret::default()
                }
            });
        }
    }

    /// `00c74ac0` and `00c66320` write their first operand into `this`.
    fn install_quaternion_doubles(e: &mut Engine) {
        e.register(QUATERNION_SET_MUL, |e, a| {
            let left = load_vec(e, a[1]);
            store_vec(e, a[0], left);
            Ret::default()
        });
        e.register(QUATERNION_SET_MUL_2, |e, a| {
            let left = load_vec(e, a[1]);
            store_vec(e, a[0], left);
            Ret::default()
        });
    }

    fn assert_identity(e: &Engine, at: u32) {
        assert_eq!(load_bits(e, at), [0; 4]);
        assert_eq!(load_bits(e, at + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_bits(e, at + 0x20), [0x3f80_0000; 4]);
    }

    fn assert_close(got: Vec4, want: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (got[lane] - want[lane]).abs() <= tolerance,
                "lane {lane}: {got:?} against {want:?}"
            );
        }
    }

    #[test]
    fn fn_00ce5430_makes_an_empty_chain_mapping() {
        let mut e = engine();
        let this = e.mem.alloc(0x70);
        for word in 0..0x1c {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00ce_5430, &args![this]).u32(), this);
        for offset in [0, 2, 4, 6] {
            assert_eq!(e.mem.u16(this + offset), 0);
        }
        assert_identity(&e, this + 0x10);
        assert_identity(&e, this + 0x40);
    }

    #[test]
    fn fn_00ce55d0_scales_by_the_lengths_of_each_mode() {
        let mut e = engine();
        e.register(BONE_POINTS, |e, a| {
            let first = load_vec(e, a[0]);
            let second = load_vec(e, a[0] + 0x10);
            store_vec(e, a[2], first);
            store_vec(e, a[3], second);
            Ret::default()
        });
        let params = e.mem.alloc(0x14);
        e.mem.set_f32(params + 0xc, 2.0);
        e.mem.set_f32(params + 0x10, 4.0);
        let bones_a = e.mem.alloc(0x100);
        let bones_b = e.mem.alloc(0x100);
        let out_a = e.mem.alloc(0x30);
        let out_b = e.mem.alloc(0x30);
        // Mode 0: the two points of A are 5 apart, the two of B are 10 apart.
        store_vec(&mut e, bones_a, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, bones_a + 0x10, [4.0, 6.0, 3.0, 0.0]);
        store_vec(&mut e, bones_b, [0.0; 4]);
        store_vec(&mut e, bones_b + 0x10, [0.0, 0.0, 10.0, 0.0]);
        e.mem.set_u32(params, 0);
        let words = args![params, 0x1111u32, bones_a, 7u32, bones_b, 9u32, out_a, out_b];
        e.call(0x00ce_55d0, &words);
        let points = calls(&e, BONE_POINTS);
        assert_eq!(points.len(), 2);
        assert_eq!(points[0][..2], [bones_a, 7]);
        assert_eq!(points[1][..2], [bones_b, 9]);
        assert_eq!(load_bits(&e, out_a), [0; 4]);
        assert_eq!(load_bits(&e, out_a + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(&e, out_a + 0x20), [4.0; 4]);
        assert_eq!(load_bits(&e, out_b), [0; 4]);
        assert_eq!(load_bits(&e, out_b + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(&e, out_b + 0x20), [0.25; 4]);
        // Mode 1: bone 1 of A (translation length 5) and bone 0 of B (length 10).
        e.mem.set_u32(params, 1);
        e.mem.set_u32(params + 4, 1);
        e.mem.set_u32(params + 8, 0);
        store_vec(&mut e, bones_a + 0x30, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, bones_b, [0.0, 0.0, 10.0, 0.0]);
        e.call(0x00ce_55d0, &words);
        assert_identity_with_scale(&e, out_a, 4.0);
        assert_identity_with_scale(&e, out_b, 0.25);
        // Any other mode: the two floats alone.
        e.mem.set_u32(params, 5);
        e.call(0x00ce_55d0, &words);
        assert_identity_with_scale(&e, out_a, 2.0);
        assert_identity_with_scale(&e, out_b, 0.5);
        assert_eq!(calls(&e, BONE_POINTS).len(), 2);
    }

    fn assert_identity_with_scale(e: &Engine, at: u32, scale: f32) {
        assert_eq!(load_bits(e, at), [0; 4]);
        assert_eq!(load_bits(e, at + 0x10), [0, 0, 0, 0x3f80_0000]);
        assert_eq!(load_vec(e, at + 0x20), [scale; 4]);
    }

    /// Two transforms: `a` with translation (1, 2, 3), `b` with (5, 6, 7), both with the identity
    /// rotation; `a` scaled (2, 4, 8), `b` by one.
    fn two_transforms(e: &mut Engine) -> (u32, u32, u32) {
        let a = e.mem.alloc(0x30);
        let b = e.mem.alloc(0x30);
        let out = e.mem.alloc(0x30);
        for at in [a, b] {
            store_bits(e, at + 0x10, [0, 0, 0, 0x3f80_0000]);
        }
        store_vec(e, a, [1.0, 2.0, 3.0, 0.0]);
        store_vec(e, a + 0x20, [2.0, 4.0, 8.0, 1.0]);
        store_vec(e, b, [5.0, 6.0, 7.0, 0.0]);
        store_vec(e, b + 0x20, [1.0; 4]);
        (a, b, out)
    }

    #[test]
    fn compute_mapping_transform_mode_zero_inverts_the_first_transform() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        e.call(0x00ce_5a70, &args![0u32, a, b, out, 1.0e-5f32, 1.0f32]);
        // The translation is rotate(conjugate, b.t) - rotate(a.rotation, a.t) = (5, 6, 7) - (1, 2, 3).
        assert_close(load_vec(&e, out), [4.0, 4.0, 4.0, 0.0], 0.0);
        // The rotation is what `00c66320` made of the conjugate.
        let set_mul = calls(&e, QUATERNION_SET_MUL);
        assert_eq!(set_mul.len(), 1);
        assert_eq!(set_mul[0][0], out + 0x10);
        assert_eq!(set_mul[0][2], b + 0x10);
        assert_eq!(
            load_bits(&e, out + 0x10),
            [0x8000_0000, 0x8000_0000, 0x8000_0000, 0x3f80_0000]
        );
        // The scale is b.scale / a.scale (reciprocal estimate with one Newton step), lane 3 zero.
        let scale = load_vec(&e, out + 0x20);
        assert_close(scale, [0.5, 0.25, 0.125, 0.0], 1.0e-6);
        assert_eq!(scale[3].to_bits(), 0);
        assert!(reports(&e).is_empty());
        assert_eq!(calls(&e, MAPPING_IS_VALID).len(), 1);
        assert_eq!(calls(&e, MAPPING_IS_VALID)[0][0], out);
    }

    #[test]
    fn compute_mapping_transform_mode_one_scales_by_the_lengths_and_reports() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        store_vec(&mut e, a, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, b, [0.0, 0.0, 10.0, 0.0]);
        // Lengths 5 and 10: scale 2, translation b - 2 a.
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(load_vec(&e, out), [-6.0, -8.0, 10.0, 0.0]);
        let set_mul = calls(&e, QUATERNION_SET_MUL_2);
        assert_eq!(set_mul, vec![vec![out + 0x10, b + 0x10, a + 0x10]]);
        assert!(reports(&e).is_empty());
        // A zero length against a non-zero one is reported and the fallback scale is used.
        store_vec(&mut e, a, [0.0; 4]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [7.0; 4]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 10.0, 0.0]);
        assert_eq!(reports(&e), vec![(0xabba_2b94, 0x35c)]);
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[0][1], MESSAGE_ZERO_LENGTH);
        // Two zero lengths use the fallback without a report.
        store_vec(&mut e, b, [0.0; 4]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [7.0; 4]);
        assert_eq!(reports(&e).len(), 1);
        // A negative tolerance is reported, the work goes on with it (no length is small).
        store_vec(&mut e, a, [3.0, 4.0, 0.0, 0.0]);
        store_vec(&mut e, b, [0.0, 0.0, 10.0, 0.0]);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, -1.0f32, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(reports(&e)[1], (0xabba_39c3, 0x34e));
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[1][1], MESSAGE_TOLERANCE);
        // NaN tolerance: no report, no length is small.
        e.call(0x00ce_5a70, &args![1u32, a, b, out, f32::NAN, 7.0f32]);
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(reports(&e).len(), 2);
    }

    #[test]
    fn compute_mapping_transform_reports_other_modes_and_invalid_results() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let (a, b, out) = two_transforms(&mut e);
        e.mem.set_u32(out, 0x1234_5678);
        e.call(0x00ce_5a70, &args![2u32, a, b, out, 1.0e-5f32, 1.0f32]);
        assert_eq!(e.mem.u32(out), 0x1234_5678);
        assert_eq!(reports(&e), vec![(0xabba_ef91, 0x36c)]);
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[0][1], MESSAGE_UNSUPPORTED);
        // A mapping the validity check refuses is reported after the work.
        install_validity(&mut e, false);
        e.call(0x00ce_5a70, &args![1u32, a, b, out, 1.0e-5f32, 1.0f32]);
        assert_eq!(reports(&e)[1], (0xabba_10fc, 0x371));
        assert_eq!(calls(&e, REPORT_BUILDER_TEXT)[1][1], MESSAGE_INVALID);
        assert_eq!(calls(&e, REPORT_BUILDER_CONSTRUCT)[1][2], 0x200);
    }

    /// Parent indices `parents` as the `i16` array a skeleton points to at `+0xC`.
    fn make_skeleton(e: &mut Engine, parents: &[i16]) -> u32 {
        let data = e.mem.alloc(2 * parents.len() as u32);
        for (index, parent) in parents.iter().enumerate() {
            e.mem.set_i16(data + 2 * index as u32, *parent);
        }
        let skeleton = e.mem.alloc(0x10);
        e.mem.set_u32(skeleton + 0xc, data);
        skeleton
    }

    /// Sets bone `index` of the transforms at `bones`.
    fn set_bone(e: &mut Engine, bones: u32, index: u32, translation: Vec4, rotation: Vec4) {
        store_vec(e, bones + 0x30 * index, translation);
        store_vec(e, bones + 0x30 * index + 0x10, rotation);
        store_vec(e, bones + 0x30 * index + 0x20, [1.0; 4]);
    }

    #[test]
    fn fn_00ce5e60_sums_the_chain_up_to_the_stop_bone() {
        let mut e = engine();
        let skeleton = make_skeleton(&mut e, &[-1, 0, 1]);
        let bones = e.mem.alloc(0x90);
        let identity = [0.0, 0.0, 0.0, 1.0];
        set_bone(&mut e, bones, 0, [9.0, 9.0, 9.0, 0.0], identity);
        set_bone(&mut e, bones, 1, [0.0, 2.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones, 2, [1.0, 0.0, 0.0, 0.0], identity);
        let length = e.mem.alloc(4);
        let vector = e.mem.alloc(0x10);
        e.mem.set_f32(length, 5.0);
        store_vec(&mut e, vector, [8.0; 4]);
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, 2i16, length, vector],
        );
        assert_eq!(e.mem.f32(length), 3.0);
        assert_eq!(load_vec(&e, vector), [1.0, 2.0, 0.0, 0.0]);
        // A start at or below the stop bone leaves zeros.
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 2i16, 2i16, length, vector],
        );
        assert_eq!(e.mem.f32(length), 0.0);
        assert_eq!(load_bits(&e, vector), [0; 4]);
        // The stop bone is compared as a signed 16-bit value: a start of -1 never runs.
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, -1i16, length, vector],
        );
        assert_eq!(load_bits(&e, vector), [0; 4]);
        // A bone rotated about z (quaternion (0, 0, 0.6, 0.8)) turns the vector gathered so far.
        set_bone(&mut e, bones, 1, [0.0; 4], [0.0, 0.0, 0.6, 0.8]);
        e.call(
            0x00ce_5e60,
            &args![skeleton, bones, 0i16, 2i16, length, vector],
        );
        assert_close(load_vec(&e, vector), [0.28, 0.96, 0.0, 0.0], 1.0e-6);
        assert_eq!(e.mem.f32(length), 1.0);
    }

    #[test]
    fn compute_chain_mapping_picks_the_bones_by_signed_index() {
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let bones_a = e.mem.alloc(0x90);
        let bones_b = e.mem.alloc(0x90);
        let identity = [0.0, 0.0, 0.0, 1.0];
        set_bone(&mut e, bones_a, 1, [3.0, 4.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_b, 0, [0.0, 0.0, 10.0, 0.0], identity);
        let out = e.mem.alloc(0x30);
        // Bone -1 of the array that starts one bone later is its first bone.
        e.call(
            0x00ce_5f90,
            &args![
                1u32,
                bones_a,
                1i16,
                bones_b + 0x30,
                -1i16,
                out,
                1.0e-5f32,
                7.0f32
            ],
        );
        assert_eq!(load_vec(&e, out + 0x20), [2.0; 4]);
        assert_eq!(load_vec(&e, out), [-6.0, -8.0, 10.0, 0.0]);
        assert_eq!(
            calls(&e, QUATERNION_SET_MUL_2),
            vec![vec![out + 0x10, bones_b + 0x10, bones_a + 0x30 + 0x10]]
        );
    }

    #[test]
    fn fn_00ce5fe0_fills_a_chain_mapping() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        install_report(&mut e);
        install_validity(&mut e, true);
        install_quaternion_doubles(&mut e);
        let identity = [0.0, 0.0, 0.0, 1.0];
        let skeleton_a = make_skeleton(&mut e, &[-1, 0, 1]);
        let skeleton_b = make_skeleton(&mut e, &[-1, 0]);
        let bones_a = e.mem.alloc(0x90);
        let bones_b = e.mem.alloc(0x60);
        set_bone(&mut e, bones_a, 0, [3.0, 4.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_a, 1, [0.0, 3.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_a, 2, [4.0, 0.0, 0.0, 0.0], identity);
        set_bone(&mut e, bones_b, 0, [0.0, 0.0, 10.0, 0.0], identity);
        set_bone(&mut e, bones_b, 1, [0.0, 6.0, 0.0, 0.0], identity);
        let chains = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&chains);
        e.register_double(ROTATION_BETWEEN, move |e, a| {
            seen.borrow_mut()
                .push((load_vec(e, a[1]), load_vec(e, a[2])));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 4.0]);
            Ret::default()
        });
        e.register(QUATERNION_SET_MUL_3, |e, a| {
            store_vec(e, a[0], [9.0; 4]);
            Ret::default()
        });
        let out = e.mem.alloc(0x70);
        e.call(
            0x00ce_5fe0,
            &args![
                1u32,
                skeleton_a,
                bones_a,
                0x0001_0000u32,
                2u32,
                skeleton_b,
                bones_b,
                0xffff_0000u32,
                1u32,
                out
            ],
        );
        // The bone indices are the low words of the arguments.
        for (offset, want) in [(0, 0), (2, 2), (4, 0), (6, 1)] {
            assert_eq!(e.mem.u16(out + offset), want);
        }
        // The start bones give scale 10 / 5 (the chains of 0x10000 and 0xffff0000 are bones 0).
        assert_eq!(load_vec(&e, out + 0x30), [2.0; 4]);
        // The chain vectors (4, 3, 0) and (0, 6, 0), normalised, in the order (B, A).
        let chains = chains.borrow();
        assert_eq!(chains.len(), 1);
        assert_close(chains[0].0, [0.0, 1.0, 0.0, 0.0], 1.0e-5);
        assert_close(chains[0].1, [0.8, 0.6, 0.0, 0.0], 1.0e-5);
        assert_eq!(load_vec(&e, out + 0x40), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(load_vec(&e, out + 0x50), [9.0; 4]);
        let reciprocal = (1.0f64 / 7.0f64) as f32;
        assert_eq!(load_bits(&e, out + 0x60), [reciprocal.to_bits(); 4]);
        let set_mul = calls(&e, QUATERNION_SET_MUL_3);
        assert_eq!(
            set_mul,
            vec![vec![
                out + 0x50,
                bones_a + 0x60 + 0x10,
                bones_b + 0x30 + 0x10
            ]]
        );
    }

    #[test]
    fn params_copy_constructor_copies_the_arrays() {
        let mut e = engine();
        let router_block = e.mem.alloc(0x20);
        e.mem.set_u32(router_block + 0x10, 0x00a1_1000);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router_block,
            ..Ret::default()
        });
        e.register(ARRAY_RESERVE, |e, a| {
            let data = e.mem.alloc(8 * a[2]);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let source = e.mem.alloc(0x54);
        for word in 0..0x15 {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.mem.set_u32(source + 0xc, 0x7f80_0001);
        e.mem.set_u32(source + 0x10, 0x3fc0_0000);
        e.mem.set_u8(source + 0x38, 1);
        e.mem.set_u8(source + 0x39, 2);
        // Array 1: two pairs; array 2: empty; array 3: one pair.
        let first = e.mem.alloc(16);
        let third = e.mem.alloc(8);
        for (index, value) in [11u32, 12, 13, 14].into_iter().enumerate() {
            e.mem.set_u32(first + 4 * index as u32, value);
        }
        e.mem.set_u32(third, 31);
        e.mem.set_u32(third + 4, 32);
        e.mem.set_u32(source + 0x14, first);
        e.mem.set_u32(source + 0x18, 2);
        e.mem.set_u32(source + 0x1c, 2);
        e.mem.set_u32(source + 0x20, 0);
        e.mem.set_u32(source + 0x24, 0);
        e.mem.set_u32(source + 0x28, 0x8000_0000);
        e.mem.set_u32(source + 0x2c, third);
        e.mem.set_u32(source + 0x30, 1);
        e.mem.set_u32(source + 0x34, 1);
        let this = e.mem.alloc(0x54);
        let got = e.call(0x00ce_9580, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(
            [e.mem.u32(this), e.mem.u32(this + 4), e.mem.u32(this + 8)],
            [0x100, 0x101, 0x102]
        );
        // The floats go through FLD/FSTP: the signalling NaN comes back quiet.
        assert_eq!(e.mem.u32(this + 0xc), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x10), 0x3fc0_0000);
        assert_eq!([e.mem.u8(this + 0x38), e.mem.u8(this + 0x39)], [1, 2]);
        for offset in [0x3c, 0x40, 0x44, 0x48, 0x4c, 0x50] {
            assert_eq!(e.mem.u32(this + offset), e.mem.u32(source + offset));
        }
        let data = e.mem.u32(this + 0x14);
        assert_ne!(data, first);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0x1c), 2);
        let copied: Vec<u32> = (0..4).map(|word| e.mem.u32(data + 4 * word)).collect();
        assert_eq!(copied, vec![11, 12, 13, 14]);
        // The empty array stays empty with its flags.
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 0x28), 0x8000_0000);
        let data = e.mem.u32(this + 0x2c);
        assert_eq!(e.mem.u32(this + 0x30), 1);
        assert_eq!([e.mem.u32(data), e.mem.u32(data + 4)], [31, 32]);
        // The arrays were reserved to their sizes through the allocator the router names.
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![
                vec![0x00a1_1000, this + 0x14, 2, 8],
                vec![0x00a1_1000, this + 0x2c, 1, 8]
            ]
        );
    }
}
