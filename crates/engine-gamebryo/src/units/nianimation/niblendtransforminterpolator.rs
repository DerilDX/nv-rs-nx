//! `NiAnimation/niblendtransforminterpolator.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiBlendTransformInterpolator::ms_RTTI` (the class's `NiRTTI` record).
const NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI: u32 = 0x011f_3e64;

// Translated from 00a40a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_blend_transform_interpolator_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI)
}

/// `00a37510`: the jump target of `00a40aa0`.
const JUMP_TARGET_00A40AA0: u32 = 0x00a3_7510;

// Translated from 00a40aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 5-byte jump to `00a37510`, passing on `this` and its two arguments. Return type (void): the exe's only callers of it are `CopyMembers`/`CreateClone` bodies (xrefs), it takes (this, destination, cloning process) and every `CopyMembers` in the Xbox PDB returns `void`; the class is unnamed in the engine map.
pub fn fn_00a40aa0(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) {
    e.call(JUMP_TARGET_00A40AA0, &args![this, first, second]);
}

/// `-FLT_MAX` (`double`, `010241b0`): the invalid marker the blend compares
/// transform parts with.
const LOWEST: u32 = 0x0102_41b0;
/// `NiInterpolator::INVALID_FLOAT` (Xbox PDB static; `-FLT_MAX`).
const INVALID_FLOAT: u32 = 0x0109_696c;
/// The float `-FLT_MAX` (`01015f5c`) written into invalid transform parts.
const INVALID_TRANSFORM_PART: u32 = 0x0101_5f5c;
/// The float `-FLT_MAX` (`01096b8c`) a fresh transform's scale starts as.
const INVALID_SCALE: u32 = 0x0109_6b8c;
/// The float `0.0` (`01011d78`) and the `double` `0.0001` (`01030f28`).
const ZERO: u32 = 0x0101_1d78;
const SMALLEST_SCALE_WEIGHT: u32 = 0x0103_0f28;
/// The translation (3 words, `011f426c`) and rotation (4 words, `011f49ec`)
/// the blend starts its sums from, and the translation (`011a8400`) and
/// rotation (`011f3704`) a fresh transform starts as (copied as words).
const START_TRANSLATE: u32 = 0x011f_426c;
const START_ROTATE: u32 = 0x011f_49ec;
const FRESH_TRANSLATE: u32 = 0x011a_8400;
const FRESH_ROTATE: u32 = 0x011f_3704;
/// `NiQuaternion::operator-` (`00a6dbd0`, `thiscall (this, &result)`,
/// answers the result's address).
const QUATERNION_NEGATE: u32 = 0x00a6_dbd0;
/// `NiQuaternion::Normalize` (`00a38560`, `thiscall`).
const QUATERNION_NORMALIZE: u32 = 0x00a3_8560;
/// The CRT square root `_CIsqrt` (`00ec6040`, argument and result in ST0).
const CRT_SQRT: u32 = 0x00ec_6040;
/// `NiBlendInterpolator::ComputeNormalizedWeights` (`00a37260`).
const COMPUTE_NORMALIZED_WEIGHTS: u32 = 0x00a3_7260;
/// The transform `Update` of an interpolator (vtable +0x8C, `(time,
/// object, NiQuatTransform*)`, answers whether it gave a value).
const UPDATE_TRANSFORM_SLOT: u32 = 0x8c;

/// `NiBlendInterpolator` fields (Xbox PDB): `m_fLastTime` (`NiInterpolator`)
/// +8, `m_uFlags` +0xC (bit 1: manager controlled), `m_ucArraySize` +0xD,
/// `m_ucInterpCount` +0xE, `m_pkInterpArray` +0x14, `m_pkSingleInterpolator`
/// +0x18, `m_fSingleTime` +0x20; items of 0x18 bytes (interpolator +0,
/// normalized weight +8, update time +0x14).
const LAST_TIME: u32 = 0x8;
const FLAGS: u32 = 0xc;
const ARRAY_SIZE: u32 = 0xd;
const INTERP_COUNT: u32 = 0xe;
const INTERP_ARRAY: u32 = 0x14;
const SINGLE_INTERPOLATOR: u32 = 0x18;
const SINGLE_TIME: u32 = 0x20;
const ITEM_SIZE: u32 = 0x18;
const ITEM_NORMALIZED_WEIGHT: u32 = 0x8;
const ITEM_UPDATE_TIME: u32 = 0x14;
/// `NiQuatTransform` (Xbox PDB): `m_kTranslate` +0, `m_kRotate` +0xC (four
/// floats), `m_fScale` +0x1C.
const ROTATE: u32 = 0xc;
const SCALE: u32 = 0x1c;

/// Writes the invalid marker into a transform's translation, rotation (its
/// float at +0x10) and scale.
fn invalidate_transform(e: &mut Engine, transform: u32) {
    let invalid = x87_quiet_bits(e.mem.u32(INVALID_TRANSFORM_PART));
    e.mem.set_u32(transform, invalid);
    e.mem.set_u32(transform + 0x10, invalid);
    e.mem.set_u32(transform + SCALE, invalid);
}

// Translated from 00a40b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::StoreSingleValue` (Xbox PDB): the single
/// interpolator's transform at `time` (the single time when manager
/// controlled). At the invalid time, or when its `Update` (vtable +0x8C)
/// gives nothing, the transform is marked invalid and the answer is false;
/// otherwise its rotation is normalized (`NiQuaternion::Normalize` on a
/// copy, unless the rotation's float at +0x10 is the invalid marker) and the
/// answer is true.
pub fn ni_blend_transform_interpolator_store_single_value(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    transform: Ptr,
) -> bool {
    let mut time_bits = time.to_bits();
    if e.mem.u8(this.addr() + FLAGS) & 1 != 0 {
        time_bits = x87_quiet_bits(e.mem.u32(this.addr() + SINGLE_TIME));
    }
    let time_value = x87_load32(time_bits);
    if x87_load32(e.mem.u32(INVALID_FLOAT)) == time_value {
        invalidate_transform(e, transform.addr());
        return false;
    }
    let single = e.mem.u32(this.addr() + SINGLE_INTERPOLATOR);
    let gave = e
        .vcall(
            single,
            UPDATE_TRANSFORM_SLOT,
            &args![x87_store32(time_value), object, transform],
        )
        .bool();
    if !gave {
        invalidate_transform(e, transform.addr());
        return false;
    }
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    let rotation = transform.addr() + ROTATE;
    if x87_load32(e.mem.u32(rotation + 4)) != lowest {
        e.with_stack(16, |e, copy| {
            for word in 0..4u32 {
                let value = e.mem.u32(rotation + 4 * word);
                e.mem.set_u32(copy.addr() + 4 * word, value);
            }
            e.call(QUATERNION_NORMALIZE, &args![copy]);
            for word in 0..4u32 {
                let value = e.mem.u32(copy.addr() + 4 * word);
                e.mem.set_u32(rotation + 4 * word, value);
            }
        });
    }
    true
}

// Translated from 00a40c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::BlendValues` (Xbox PDB): blends the
/// transforms of the items with a positive normalized weight. Each item's
/// interpolator gives a transform at `time` (its update time when manager
/// controlled) through `Update` (vtable +0x8C) into a fresh local
/// transform; at the invalid time or without a value its weight is taken
/// off both the translation and the scale weight totals (from 1). Each
/// valid part is summed weighted: the translation (else its weight comes
/// off the translation total), the rotation (each after the first flipped
/// with `NiQuaternion::operator-` when its dot product with the running sum
/// is negative), the scale (else its weight comes off the scale total).
/// The result starts invalid; a summed translation is divided by its total
/// (unless that is 0), the rotation sum normalized (`1 / sqrt` of its
/// squared length, `_CIsqrt`), the scale sum divided by its total (unless
/// that is below 0.0001). Answers whether any part of the result is valid.
/// The float roundings follow the exe's stores.
pub fn ni_blend_transform_interpolator_blend_values(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    out: Ptr,
) -> bool {
    let lowest = f64::from_bits(e.global::<u64>(LOWEST));
    let load = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let one = x87_store32(1.0);
    let mut translate_weight = one;
    let mut scale_weight = one;
    let mut scale_sum = 0u32;
    let mut translate = [0u32; 3];
    for (i, word) in translate.iter_mut().enumerate() {
        *word = e.mem.u32(START_TRANSLATE + 4 * i as u32);
    }
    let mut rotate = [0u32; 4];
    for (i, word) in rotate.iter_mut().enumerate() {
        *word = e.mem.u32(START_ROTATE + 4 * i as u32);
    }
    let mut translated = false;
    let mut rotated = false;
    let mut scaled = false;
    let mut first_rotation = true;
    let mut index = 0u8;
    while index < e.mem.u8(this.addr() + ARRAY_SIZE) {
        let item = e
            .mem
            .u32(this.addr() + INTERP_ARRAY)
            .wrapping_add(u32::from(index) * ITEM_SIZE);
        index = index.wrapping_add(1);
        let interpolator = e.mem.u32(item);
        if interpolator == 0 {
            continue;
        }
        let weight_bits = e.mem.u32(item + ITEM_NORMALIZED_WEIGHT);
        if !(0.0 < x87_m32(weight_bits)) {
            continue;
        }
        let subtract = |e: &Engine, total: u32| {
            x87_store32(x87_sub(
                x87_load32(total),
                x87_m32(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT)),
            ))
        };
        let mut item_time = x87_quiet_bits(time.to_bits());
        let skip = if 0.0 == x87_m32(weight_bits) {
            true
        } else {
            if e.mem.u8(this.addr() + FLAGS) & 1 != 0 {
                item_time = x87_quiet_bits(e.mem.u32(item + ITEM_UPDATE_TIME));
            }
            load(e, INVALID_FLOAT) == x87_load32(item_time)
        };
        if skip {
            translate_weight = subtract(e, translate_weight);
            scale_weight = subtract(e, scale_weight);
            continue;
        }
        let part = e.with_stack(0x20, |e, local| {
            let local = local.addr();
            for word in 0..3u32 {
                let value = e.mem.u32(FRESH_TRANSLATE + 4 * word);
                e.mem.set_u32(local + 4 * word, value);
            }
            for word in 0..4u32 {
                let value = e.mem.u32(FRESH_ROTATE + 4 * word);
                e.mem.set_u32(local + ROTATE + 4 * word, value);
            }
            let scale = x87_quiet_bits(e.mem.u32(INVALID_SCALE));
            e.mem.set_u32(local + SCALE, scale);
            let gave = e
                .vcall(
                    interpolator,
                    UPDATE_TRANSFORM_SLOT,
                    &args![item_time, object, local],
                )
                .bool();
            let words: Vec<u32> = (0..8).map(|i| e.mem.u32(local + 4 * i)).collect();
            (gave, words)
        });
        let (gave, transform) = part;
        if !gave {
            translate_weight = subtract(e, translate_weight);
            scale_weight = subtract(e, scale_weight);
            continue;
        }
        let x = x87_load32(transform[0]);
        if x == lowest {
            translate_weight = subtract(e, translate_weight);
        } else {
            let weight = x87_load32(x87_quiet_bits(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT)));
            translated = true;
            let px = x87_store32(x87_mul(x, weight));
            let py = x87_store32(x87_mul(x87_load32(transform[1]), weight));
            let pz = x87_store32(x87_mul(weight, x87_load32(transform[2])));
            translate[0] = x87_store32(x87_add(x87_load32(px), x87_m32(translate[0])));
            translate[1] = x87_store32(x87_add(x87_load32(translate[1]), x87_m32(py)));
            translate[2] = x87_store32(x87_add(x87_load32(translate[2]), x87_m32(pz)));
        }
        let r1 = x87_load32(transform[4]);
        if r1 != lowest {
            let mut quaternion = [transform[3], transform[4], transform[5], transform[6]];
            if first_rotation {
                first_rotation = false;
            } else {
                let dot = x87_mul(r1, x87_m32(rotate[1]));
                let dot = x87_add(dot, x87_mul(x87_load32(transform[3]), x87_m32(rotate[0])));
                let dot = x87_add(dot, x87_mul(x87_load32(transform[5]), x87_m32(rotate[2])));
                let dot = x87_add(dot, x87_mul(x87_load32(transform[6]), x87_m32(rotate[3])));
                let dot = x87_load32(x87_store32(dot));
                if dot < x87_m32(e.mem.u32(ZERO)) {
                    quaternion = e.with_stack(0x20, |e, temp| {
                        let source = temp.addr();
                        for (i, word) in quaternion.iter().enumerate() {
                            e.mem.set_u32(source + 4 * i as u32, *word);
                        }
                        let result = e
                            .call(QUATERNION_NEGATE, &args![source, source + 0x10])
                            .u32();
                        [0u32, 1, 2, 3].map(|i| e.mem.u32(result + 4 * i))
                    });
                }
            }
            let weight = x87_load32(x87_quiet_bits(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT)));
            rotated = true;
            let q0 = x87_store32(x87_mul(x87_load32(quaternion[0]), weight));
            let q1 = x87_store32(x87_mul(x87_load32(quaternion[1]), weight));
            let q2 = x87_store32(x87_mul(x87_load32(quaternion[2]), weight));
            rotate[0] = x87_store32(x87_add(x87_load32(q0), x87_m32(rotate[0])));
            rotate[1] = x87_store32(x87_add(x87_load32(rotate[1]), x87_m32(q1)));
            rotate[2] = x87_store32(x87_add(x87_load32(rotate[2]), x87_m32(q2)));
            let q3 = x87_store32(x87_mul(weight, x87_m32(quaternion[3])));
            rotate[3] = x87_store32(x87_add(x87_load32(q3), x87_m32(rotate[3])));
        }
        let scale = x87_load32(transform[7]);
        if scale == lowest {
            scale_weight = subtract(e, scale_weight);
        } else {
            scaled = true;
            let weighted = x87_mul(scale, x87_m32(e.mem.u32(item + ITEM_NORMALIZED_WEIGHT)));
            scale_sum = x87_store32(x87_add(weighted, x87_m32(scale_sum)));
        }
    }
    let out = out.addr();
    invalidate_transform(e, out);
    if translated || rotated || scaled {
        let total = x87_load32(translate_weight);
        if total == 0.0 {
            translated = false;
        }
        let smallest = f64::from_bits(e.global::<u64>(SMALLEST_SCALE_WEIGHT));
        if x87_load32(scale_weight) < smallest {
            scaled = false;
        }
        if translated {
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, total)));
            translate[0] = x87_store32(x87_mul(reciprocal, x87_m32(translate[0])));
            e.mem.set_u32(out, translate[0]);
            translate[1] = x87_store32(x87_mul(x87_load32(translate[1]), reciprocal));
            e.mem.set_u32(out + 4, translate[1]);
            translate[2] = x87_store32(x87_mul(reciprocal, x87_m32(translate[2])));
            e.mem.set_u32(out + 8, translate[2]);
        }
        if rotated {
            let q1 = x87_load32(rotate[1]);
            let q0 = x87_load32(rotate[0]);
            let q2 = x87_load32(rotate[2]);
            let q3 = x87_load32(rotate[3]);
            let sum = x87_add(x87_mul(q1, q1), x87_mul(q0, q0));
            let sum = x87_add(sum, x87_mul(q2, q2));
            let sum = x87_add(sum, x87_mul(q3, q3));
            let length_squared = x87_load32(x87_store32(sum));
            let root = e.call(CRT_SQRT, &args![length_squared]).f64();
            let root = x87_load32(x87_store32(root));
            let reciprocal = x87_load32(x87_store32(x87_div(1.0, root)));
            rotate[0] = x87_store32(x87_mul(reciprocal, x87_m32(rotate[0])));
            e.mem.set_u32(out + ROTATE, rotate[0]);
            rotate[1] = x87_store32(x87_mul(x87_load32(rotate[1]), reciprocal));
            e.mem.set_u32(out + ROTATE + 4, rotate[1]);
            rotate[2] = x87_store32(x87_mul(x87_load32(rotate[2]), reciprocal));
            e.mem.set_u32(out + ROTATE + 8, rotate[2]);
            rotate[3] = x87_store32(x87_mul(reciprocal, x87_m32(rotate[3])));
            e.mem.set_u32(out + ROTATE + 12, rotate[3]);
        }
        if scaled {
            let scale = x87_div(x87_load32(scale_sum), x87_m32(scale_weight));
            e.mem.set_u32(out + SCALE, x87_store32(scale));
        }
    }
    load(e, out + SCALE) != lowest || load(e, out + 0x10) != lowest || load(e, out) != lowest
}

// Translated from 00a41110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiBlendTransformInterpolator::Update` (Xbox PDB): with one interpolator
/// `StoreSingleValue`, with more `ComputeNormalizedWeights` then
/// `BlendValues`, with none nothing (false); `m_fLastTime` becomes `time`.
/// Answers what the store or blend answered.
pub fn ni_blend_transform_interpolator_update(
    e: &mut Engine,
    this: Ptr,
    time: f32,
    object: Ptr,
    transform: Ptr,
) -> bool {
    let time_bits = x87_quiet_bits(time.to_bits());
    let count = e.mem.u8(this.addr() + INTERP_COUNT);
    let answer = if count == 1 {
        ni_blend_transform_interpolator_store_single_value(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            transform,
        )
    } else if count > 0 {
        e.call(COMPUTE_NORMALIZED_WEIGHTS, &args![this]);
        ni_blend_transform_interpolator_blend_values(
            e,
            this,
            f32::from_bits(time_bits),
            object,
            transform,
        )
    } else {
        false
    };
    e.mem.set_u32(this.addr() + LAST_TIME, time_bits);
    answer
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a40a40, ni_blend_transform_interpolator_get_rtti() -> Ptr),
        entry!(0x00a4_0aa0, fn_00a40aa0(Ptr, Ptr, Ptr)),
        entry!(
            0x00a40b30,
            ni_blend_transform_interpolator_store_single_value(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a40c10,
            ni_blend_transform_interpolator_blend_values(Ptr, f32, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a41110,
            ni_blend_transform_interpolator_update(Ptr, f32, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_blend_transform_interpolator_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a40a40, &args![]).u32(),
            NI_BLEND_TRANSFORM_INTERPOLATOR_RTTI
        );
    }

    #[test]
    fn thunk_00a40aa0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a4_0aa0;
        const TARGET: u32 = 0x00a3_7510;
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

#[cfg(test)]
mod b014_tests {
    use super::*;

    const VTABLE: u32 = 0x7000_0000;
    const CHILD_UPDATE: u32 = 0x7200_008c;

    /// An engine with the constants mapped (start sums 0, fresh transforms
    /// invalid) and the children's `Update` a double: a child object holds
    /// its answer at +0x30 and the transform it gives at +0x10.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0102_4000u32,
            0x0109_6000,
            0x0101_5000,
            0x0101_1000,
            0x0103_0000,
            0x011f_4000,
            0x011a_8000,
            0x011f_3000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(LOWEST, -3.4028234663852886e38f64);
        e.mem.set_f32(INVALID_FLOAT, f32::MIN);
        e.mem.set_f32(INVALID_SCALE, f32::MIN);
        e.mem.set_f32(INVALID_TRANSFORM_PART, f32::MIN);
        e.set_global(SMALLEST_SCALE_WEIGHT, 0.0001f64);
        for i in 0..3 {
            e.mem.set_f32(FRESH_TRANSLATE + 4 * i, f32::MIN);
        }
        for i in 0..4 {
            e.mem.set_f32(FRESH_ROTATE + 4 * i, f32::MIN);
        }
        let mut slots = vec![0u32; 0x90 / 4];
        slots[0x8c / 4] = CHILD_UPDATE;
        e.put_vtable(VTABLE, &slots);
        e.register(CHILD_UPDATE, |e, a| {
            let child = a[0];
            for i in 0..8 {
                let word = e.mem.u32(child + 0x10 + 4 * i);
                e.mem.set_u32(a[3] + 4 * i, word);
            }
            (e.mem.u32(child + 0x30) != 0).into_ret()
        });
        e
    }

    /// A child giving `transform` (translate x, rotation w, x, y, z, scale)
    /// or nothing.
    fn child(e: &mut Engine, gives: bool, transform: [f32; 8]) -> u32 {
        let child = e.mem.alloc(0x40);
        e.mem.set_u32(child, VTABLE);
        for (i, v) in transform.iter().enumerate() {
            e.mem.set_f32(child + 0x10 + 4 * i as u32, *v);
        }
        e.mem.set_u32(child + 0x30, u32::from(gives));
        child
    }

    fn floats(e: &Engine, at: u32) -> Vec<f32> {
        (0..8).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    const M: f32 = f32::MIN;

    #[test]
    fn store_single_value_normalizes_a_valid_rotation() {
        let mut e = engine();
        e.register(QUATERNION_NORMALIZE, |e, a| {
            for i in 0..4 {
                let v = e.mem.f32(a[0] + 4 * i) / 2.0;
                e.mem.set_f32(a[0] + 4 * i, v);
            }
            Ret::default()
        });
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        let single = child(&mut e, true, [1.0, 2.0, 3.0, 2.0, 4.0, 6.0, 8.0, 5.0]);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        let out = e.mem.alloc(0x20);
        assert!(e.call(0x00a40b30, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(floats(&e, out), [1.0, 2.0, 3.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        // An invalid rotation is left as it is.
        let single = child(&mut e, true, [1.0, 2.0, 3.0, 2.0, M, 6.0, 8.0, 5.0]);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        assert!(e.call(0x00a40b30, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(floats(&e, out)[3..7], [2.0, M, 6.0, 8.0]);
    }

    #[test]
    fn store_single_value_marks_invalid_at_the_invalid_time_or_without_a_value() {
        let mut e = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        let single = child(&mut e, false, [1.0; 8]);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        let out = e.mem.alloc(0x20);
        assert!(!e.call(0x00a40b30, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(
            (e.mem.f32(out), e.mem.f32(out + 0x10), e.mem.f32(out + 0x1c)),
            (M, M, M)
        );
        // Manager controlled: the single time (invalid here) is used.
        let single = child(&mut e, true, [1.0; 8]);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, single);
        e.mem.set_u8(this.addr() + FLAGS, 1);
        e.mem.set_f32(this.addr() + SINGLE_TIME, M);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a40b30, &args![this, 1.0f32, 0u32, out]).bool());
        assert!(e
            .call_log
            .take()
            .unwrap()
            .iter()
            .all(|c| c.0 != CHILD_UPDATE));
        assert_eq!(e.mem.f32(out + 0x10), M);
    }

    /// A blend of two children with normalized weights 0.25 and 0.75.
    fn blend_of(e: &mut Engine, a: u32, b: u32) -> Ptr {
        let this: Ptr = Ptr::new(e.mem.alloc(0x30));
        let array = e.mem.alloc(2 * ITEM_SIZE);
        e.mem.set_u32(array, a);
        e.mem.set_f32(array + ITEM_NORMALIZED_WEIGHT, 0.25);
        e.mem.set_u32(array + ITEM_SIZE, b);
        e.mem
            .set_f32(array + ITEM_SIZE + ITEM_NORMALIZED_WEIGHT, 0.75);
        e.mem.set_u32(this.addr() + INTERP_ARRAY, array);
        e.mem.set_u8(this.addr() + ARRAY_SIZE, 2);
        this
    }

    #[test]
    fn blend_values_weights_every_part() {
        let mut e = engine();
        let a = child(&mut e, true, [4.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0]);
        let b = child(&mut e, true, [8.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 4.0]);
        let this = blend_of(&mut e, a, b);
        let out = e.mem.alloc(0x20);
        assert!(e.call(0x00a40c10, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(floats(&e, out), [7.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 3.5]);
    }

    #[test]
    fn blend_values_flips_an_opposite_rotation() {
        let mut e = engine();
        e.register(QUATERNION_NEGATE, |e, a| {
            for i in 0..4 {
                let v = -e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v);
            }
            a[1].into_ret()
        });
        let a = child(&mut e, true, [M, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, M]);
        let b = child(&mut e, true, [M, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, M]);
        let this = blend_of(&mut e, a, b);
        let out = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a40c10, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(
            e.call_log
                .take()
                .unwrap()
                .iter()
                .filter(|c| c.0 == QUATERNION_NEGATE)
                .count(),
            1
        );
        assert_eq!(floats(&e, out), [M, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, M]);
    }

    #[test]
    fn blend_values_drops_failed_children_and_reports_nothing_valid() {
        let mut e = engine();
        let a = child(&mut e, true, [4.0, 0.0, 0.0, M, M, M, M, 2.0]);
        let b = child(&mut e, false, [8.0; 8]);
        let this = blend_of(&mut e, a, b);
        let out = e.mem.alloc(0x20);
        assert!(e.call(0x00a40c10, &args![this, 1.0f32, 0u32, out]).bool());
        let got = floats(&e, out);
        assert_eq!((got[0], got[4], got[7]), (4.0, M, 2.0));
        // Neither gives anything: all invalid, false.
        let a = child(&mut e, false, [1.0; 8]);
        let this = blend_of(&mut e, a, b);
        let out = e.mem.alloc(0x20);
        assert!(!e.call(0x00a40c10, &args![this, 1.0f32, 0u32, out]).bool());
        assert_eq!(
            (e.mem.f32(out), e.mem.f32(out + 0x10), e.mem.f32(out + 0x1c)),
            (M, M, M)
        );
    }

    #[test]
    fn update_by_interpolator_count() {
        let mut e = engine();
        e.register(COMPUTE_NORMALIZED_WEIGHTS, |_, _| Ret::default());
        let a = child(&mut e, true, [4.0, 0.0, 0.0, M, M, M, M, 2.0]);
        let b = child(&mut e, true, [8.0, 0.0, 0.0, M, M, M, M, 2.0]);
        let this = blend_of(&mut e, a, b);
        let out = e.mem.alloc(0x20);
        // None: false, only the time.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a41110, &args![this, 2.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 2.5);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Two: the weights, then the blend.
        e.mem.set_u8(this.addr() + INTERP_COUNT, 2);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a41110, &args![this, 3.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 7.0);
        assert_eq!(e.mem.f32(this.addr() + LAST_TIME), 3.5);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|c| c.0 == COMPUTE_NORMALIZED_WEIGHTS));
        // One: the single interpolator.
        e.mem.set_u8(this.addr() + INTERP_COUNT, 1);
        e.mem.set_u32(this.addr() + SINGLE_INTERPOLATOR, b);
        assert!(e.call(0x00a41110, &args![this, 4.5f32, 0u32, out]).bool());
        assert_eq!(e.mem.f32(out), 8.0);
    }
}
