//! `Havok SDK/hkprigidmotionutil.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Both functions are cdecl with a motion state list: `motions` points to an array of pointers
//! and each pointer plus `offset` is a motion state. The float arithmetic of the x87 is done in
//! `f64` and rounded to `f32` at every store (the guide's convention), the vector arithmetic is
//! SSE. Neither function has a /GS cookie or an exception frame.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, store_bits, store_vec, sum3, zip_with, Vec4,
};

/// The CRT square root (`00ec6040`, argument and result in `ST0`).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkQuaternion::setMul` (`00c66320`, thiscall on the result: two quaternions; `RET 8`),
/// the quaternion normalisation (`005611c0`, thiscall) and the rotation set from a quaternion
/// (`00cb2d90`, thiscall on the matrix: the quaternion; `RET 4`).
const QUATERNION_SET_MUL: u32 = 0x00c6_6320;
const QUATERNION_NORMALIZE: u32 = 0x0056_11c0;
const SET_ROTATION: u32 = 0x00cb_2d90;
/// `hkMatrix3::setTranspose` (`00cd4ba0`, thiscall on the result: the source; `RET 4`).
const MATRIX3_SET_TRANSPOSE: u32 = 0x00cd_4ba0;

/// The absolute-value mask and the limit vector of the validity test (`1e6` per lane), and the
/// vector the velocities are reset to (`01268370`).
const ABS_MASK: u32 = 0x0101_7d10;
const VELOCITY_LIMIT: u32 = 0x010c_bf80;
const VELOCITY_RESET: u32 = 0x0126_8370;
/// The table of floats indexed by a quality byte (`010c7948`).
const QUALITY_TABLE: u32 = 0x010c_7948;
/// Constants: `0.5` (double), the factor `4 / pi^2` (double `010ce4b0`), the threshold `0.9`
/// (double `0106b9e8`) and the float it is replaced by (`010177e8`), the series coefficients
/// (doubles `010ce4a8`, `010ce4a0`, `010ce498`) and pi (double `0101ff40`).
const HALF: u32 = 0x0101_1588;
const ANGLE_FACTOR: u32 = 0x010c_e4b0;
const ANGLE_LIMIT: u32 = 0x0106_b9e8;
const ANGLE_LIMIT_FLOAT: u32 = 0x0101_77e8;
const SERIES_FIRST: u32 = 0x010c_e4a8;
const SERIES_SECOND: u32 = 0x010c_e4a0;
const SERIES_THIRD: u32 = 0x010c_e498;
const PI: u32 = 0x0101_ff40;
/// The vector of ones (`010ec7f0`).
const ONES: u32 = 0x010e_c7f0;

/// Splat of one lane, the `SHUFPS x, x, 0` selection.
fn splat(value: f32) -> Vec4 {
    [value; 4]
}

fn mul(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x * y)
}

fn add(a: Vec4, b: Vec4) -> Vec4 {
    zip_with(a, b, |x, y| x + y)
}

/// The `MOVMSKPS` of the lanes of `bits` (masked with `mask`) that are below `limit`.
fn mask_below(bits: [u32; 4], mask: [u32; 4], limit: Vec4) -> u32 {
    (0..4).fold(0, |all, lane| {
        if f32::from_bits(bits[lane] & mask[lane]) < limit[lane] {
            all | 1 << lane
        } else {
            all
        }
    })
}

/// The `MOVMSKPS` of the lanes of `bits` that are not NaN.
fn mask_ordered(bits: [u32; 4]) -> u32 {
    (0..4).fold(0, |all, lane| {
        let value = f32::from_bits(bits[lane]);
        if value == value {
            all | 1 << lane
        } else {
            all
        }
    })
}

/// The CRT square root of an `f32` through the x87 (`ST0` in and out).
fn x87_root(e: &mut Engine, value: f32) -> f32 {
    x87_quiet(e.call(X87_SQRT, &args![f64::from(value)]).f64() as f32)
}

/// One motion of `hkRigidMotionUtilStep`; `frame` is the stack frame (0x60 bytes: the
/// quaternion at `+0x30`, the rotation delta at `+0x40`, a copy of the velocity at `+0x50`).
fn step_motion(e: &mut Engine, info: u32, state: u32, frame: u32) {
    let angular = load_bits(e, state + 0xe0);
    let linear = load_bits(e, state + 0xd0);
    let abs_mask = load_bits(e, ABS_MASK);
    let limit = load_vec(e, VELOCITY_LIMIT);
    let valid = mask_below(angular, abs_mask, limit)
        & mask_below(linear, abs_mask, limit)
        & mask_ordered(angular)
        & mask_ordered(linear)
        & 7;
    if valid != 7 {
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xd0, reset);
        let reset = load_bits(e, VELOCITY_RESET);
        store_bits(e, state + 0xe0, reset);
    }
    // The previous position is the position, with the first float of the step info in the
    // fourth lane.
    let position = load_bits(e, state + 0x60);
    store_bits(e, state + 0x50, position);
    let first = x87_quiet_bits(e.mem.u32(info));
    e.mem.set_u32(state + 0x5c, first);
    // A velocity above the limit of its quality is scaled down to it.
    let velocity = load_vec(e, state + 0xd0);
    let quality = u32::from(e.mem.u8(state + 0xbc));
    store_vec(e, frame + 0x50, velocity);
    let speed_squared = sum3(mul(velocity, velocity));
    let top_speed = e.mem.f32(QUALITY_TABLE + quality * 4);
    let limit_squared = f64::from(top_speed) * f64::from(top_speed);
    if f64::from(speed_squared) > limit_squared {
        let speed = x87_root(e, speed_squared);
        let inverse = (1.0 / f64::from(speed)) as f32;
        let factor = (f64::from(inverse) * f64::from(top_speed)) as f32;
        let scaled = mul(splat(factor), load_vec(e, frame + 0x50));
        store_vec(e, state + 0xd0, scaled);
    }
    // The position moves by the velocity times the time step.
    let step = x87_quiet(e.mem.f32(info + 8));
    let velocity = load_vec(e, state + 0xd0);
    let moved = add(mul(splat(step), velocity), load_vec(e, state + 0x60));
    store_vec(e, state + 0x60, moved);
    let second = x87_quiet_bits(e.mem.u32(info + 0xc));
    e.mem.set_u32(state + 0x6c, second);
    // The rotation: the current quaternion is kept (twice).
    let rotation = load_bits(e, state + 0x80);
    store_bits(e, frame + 0x30, rotation);
    let angular_quality = u32::from(e.mem.u8(state + 0xbd));
    let kept = load_bits(e, frame + 0x30);
    store_bits(e, state + 0x70, kept);
    // The half-angle vector: half the time step times the angular velocity.
    let raw_step = e.mem.f32(info + 8);
    let half = e.mem.f64(HALF);
    let angular_velocity = load_vec(e, state + 0xe0);
    let half_step = (f64::from(raw_step) * half) as f32;
    store_vec(e, frame + 0x50, angular_velocity);
    let half_angle = mul(splat(half_step), angular_velocity);
    store_vec(e, frame + 0x40, half_angle);
    let angle_squared = sum3(mul(half_angle, half_angle));
    let angle_estimate = (f64::from(angle_squared) * e.mem.f64(ANGLE_FACTOR)) as f32;
    // The largest angle step the quality allows, capped at the float constant when above
    // `0.9`.
    let allowed = e.mem.f32(QUALITY_TABLE + angular_quality * 4);
    let mut allowed = (f64::from(allowed) * f64::from(raw_step)) as f32;
    if f64::from(allowed) > e.mem.f64(ANGLE_LIMIT) {
        allowed = x87_quiet(e.global::<f32>(ANGLE_LIMIT_FLOAT));
    }
    let allowed_squared = (f64::from(allowed) * f64::from(allowed)) as f32;
    let first_coefficient = e.mem.f64(SERIES_FIRST);
    let second_coefficient = e.mem.f64(SERIES_SECOND);
    let third_coefficient = e.mem.f64(SERIES_THIRD);
    let kept_angle;
    let cosine;
    if allowed_squared >= angle_estimate {
        // Small step: the series of the cosine of the estimate.
        let estimate = f64::from(angle_estimate);
        let squared = (estimate * estimate) as f32;
        let first = 1.0 - first_coefficient * estimate;
        let second = first - second_coefficient * f64::from(squared);
        let third = f64::from(squared) * (estimate * third_coefficient);
        cosine = (second - third) as f32;
        kept_angle = angle_estimate;
    } else {
        // Large step: the angular velocity and the half angle are scaled down so that the
        // angle is the allowed one.
        let root = x87_root(e, angle_estimate);
        let inverse = (1.0 / f64::from(root)) as f32;
        let factor = (f64::from(inverse) * f64::from(allowed)) as f32;
        let scaled_velocity = mul(splat(factor), load_vec(e, frame + 0x50));
        store_vec(e, state + 0xe0, scaled_velocity);
        let fourth = (f64::from(allowed_squared) * f64::from(allowed_squared)) as f32;
        let scaled_angle = mul(splat(factor), load_vec(e, frame + 0x40));
        store_vec(e, frame + 0x40, scaled_angle);
        let squared = f64::from(allowed_squared);
        let first = 1.0 - first_coefficient * squared;
        let second = first - second_coefficient * f64::from(fourth);
        let third = f64::from(fourth) * (squared * third_coefficient);
        cosine = (second - third) as f32;
        kept_angle = allowed_squared;
    }
    e.mem.set_f32(frame + 0x4c, cosine);
    e.call(
        QUATERNION_SET_MUL,
        &args![frame + 0x30, frame + 0x40, frame + 0x30],
    );
    e.call(QUATERNION_NORMALIZE, &args![frame + 0x30]);
    // The rotation delta doubled, with the angle in the fourth lane.
    let delta = load_vec(e, frame + 0x40);
    store_vec(e, state + 0xa0, add(delta, delta));
    let angle = x87_root(e, kept_angle);
    let rotated = load_bits(e, frame + 0x30);
    let angle_pi = (f64::from(angle) * e.mem.f64(PI)) as f32;
    e.mem.set_f32(state + 0xac, angle_pi);
    store_bits(e, state + 0x80, rotated);
    e.call(SET_ROTATION, &args![state + 0x10, state + 0x80]);
    // The transform's translation: the position minus the centre of mass (`+0x90`) rotated
    // by the transform's three columns, summed as `(y * c1 + x * c0) + z * c2`.
    let centre = load_vec(e, state + 0x90);
    let column_1 = load_vec(e, state + 0x20);
    let column_0 = load_vec(e, state + 0x10);
    let first = mul(splat(centre[1]), column_1);
    let second = mul(splat(centre[0]), column_0);
    let sum = add(first, second);
    let column_2 = load_vec(e, state + 0x30);
    let third = mul(splat(centre[2]), column_2);
    let position = load_vec(e, state + 0x60);
    let rotated_centre = add(sum, third);
    let translation = zip_with(position, rotated_centre, |a, b| a - b);
    store_vec(e, state + 0x40, translation);
}

// Translated from 00d28680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilStep` (engine map), cdecl with the step info (the first float at `+0`, the
/// time step at `+8`, another float at `+0xC`), the motion list, the number of motions and the
/// offset of the motion state in each listed object. Motions of type 5 (`+8`) are skipped.
/// For each other motion:
///
/// * a velocity pair (`+0xD0` linear, `+0xE0` angular) with a lane whose absolute value is not
///   below 1e6 (a NaN included) in its first three lanes is reset to the vector at `01268370`;
/// * the position (`+0x60`) is copied to `+0x50` with the step info's first float in its fourth
///   lane; a linear velocity whose squared length is above the square of the limit
///   `table[quality at +0xBC]` is scaled to the limit (the CRT root); the position moves by the
///   velocity times the time step and gets the info's `+0xC` float in the fourth lane;
/// * the rotation (`+0x80`) is kept at `+0x70`; the half angle vector (`half the step times
///   the angular velocity`) gets its squared length scaled by `4 / pi^2` as the estimate; the
///   largest step is `table[quality at +0xBD] * step` (replaced by the float `010177e8` when
///   above `0.9`); a larger estimate scales the angular velocity and the half angle to the
///   largest step; the cosine series gives the fourth lane; the rotation is multiplied by the
///   delta (`hkQuaternion::setMul`), normalised, stored at `+0x80` and made the transform's
///   rotation (`00cb2d90`); `+0xA0` takes twice the delta and `+0xAC` the angle times pi;
/// * the transform's translation (`+0x40`) is the position minus the centre of mass (`+0x90`)
///   rotated by the transform's columns.
///
/// The code keeps three vectors in its stack frame; the frame here has the same offsets.
pub fn hk_rigid_motion_util_step(e: &mut Engine, info: Ptr, motions: Ptr, count: i32, offset: u32) {
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return;
    }
    let mut list = motions.addr();
    e.with_stack(0x60, |e, frame| loop {
        let state = e.mem.u32(list).wrapping_add(offset);
        if e.mem.u8(state + 8) != 5 {
            step_motion(e, info.addr(), state, frame.addr());
        }
        list = list.wrapping_add(4);
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
    });
}

/// The damping factor `1 - damping * step` (rounded to a float), zero when it is below zero
/// (a NaN stays).
fn damping_factor(damping: f32, step: f32) -> f32 {
    let factor = (1.0 - f64::from(damping) * f64::from(step)) as f32;
    if factor < 0.0 {
        0.0
    } else {
        factor
    }
}

/// The damping of a motion state's velocities (linear `+0xD0` by the factor of `+0xB4`,
/// angular `+0xE0` by the factor of `+0xB8`); answers nothing. The time step is the info's
/// (`+8`) passed through the x87.
fn damp_velocities(e: &mut Engine, info: u32, state: u32) {
    let step = x87_quiet(e.mem.f32(info + 8));
    let linear = damping_factor(e.mem.f32(state + 0xb4), step);
    let velocity = mul(splat(linear), load_vec(e, state + 0xd0));
    store_vec(e, state + 0xd0, velocity);
    let angular = damping_factor(e.mem.f32(state + 0xb8), step);
    let velocity = mul(splat(angular), load_vec(e, state + 0xe0));
    store_vec(e, state + 0xe0, velocity);
}

/// The position blended between the previous (`+0x50`) and the new one (`+0x60`) by
/// `(info float 0 - previous w) * new w`, written to `target`.
fn blend_position(e: &mut Engine, info: u32, state: u32, target: u32) {
    let blend = ((f64::from(e.mem.f32(info)) - f64::from(e.mem.f32(state + 0x5c)))
        * f64::from(e.mem.f32(state + 0x6c))) as f32;
    let ones = load_vec(e, ONES);
    let previous = load_vec(e, state + 0x50);
    let current = load_vec(e, state + 0x60);
    let kept = mul(zip_with(ones, splat(blend), |a, b| a - b), previous);
    let taken = mul(current, splat(blend));
    store_vec(e, target, add(kept, taken));
}

/// The identity inertia: zero vectors at `+0x50`, `+0x60`, `+0x70` of the accumulator with
/// `1.0` on the diagonal (`+0x50`, `+0x64`, `+0x78`).
fn identity_inertia(e: &mut Engine, accumulator: u32) {
    for offset in [0x50, 0x60, 0x70] {
        store_bits(e, accumulator + offset, [0; 4]);
    }
    for offset in [0x50, 0x64, 0x78] {
        e.mem.set_f32(accumulator + offset, 1.0);
    }
}

/// The word at `+8` of the accumulator: the short at `+0x11E` of the state shifted up 16 bits.
fn store_group(e: &mut Engine, state: u32, accumulator: u32) {
    let group = (e.mem.i16(state + 0x11e) as i32) << 16;
    e.mem.set_i32(accumulator + 8, group);
}

// Translated from 00d29830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkRigidMotionUtilApplyForcesAndBuildAccumulators` (engine map), cdecl with the step info,
/// the motion list, the number of motions, the offset of the motion state in each listed
/// object and the accumulator array (0x80 bytes per motion); answers the accumulator after the
/// last one. By the motion type (`+8`):
///
/// * 0, 3, 6: the velocities are damped (see `damping_factor`), the state's matrix (`+0x10`)
///   is transposed into the accumulator at `+0x50` (`hkMatrix3::setTranspose`), the accumulator
///   gets the type 0, the vectors `+0x30` (from `+0xC0`), `+0x10` (from `+0xD0`), the blended
///   position at `+0x40`, the angular velocity transformed by the transposed columns at `+0x20`,
///   the byte at `+0xBE` as a word at `+4` and the group word at `+8`;
/// * 2 and 7: the velocities are damped, the accumulator gets the type 0 (2) or 2 (7), `+0x30`
///   from `+0xC0`, the blended position at `+0x40`, the angular and linear velocities at
///   `+0x20` and `+0x10`, the identity inertia, the byte at `+0xBE` and the group word;
/// * 4: type 1, zero at `+0x30`, the position `+0x60` at `+0x40`, the velocities, the identity
///   inertia and the group word;
/// * 5: type 1 and zeros (the word at `+8` is 0.0);
/// * any other (1, above 7): nothing is written.
///
/// The code keeps the constants `1, 0, 0, 1` on the x87 stack throughout and only stores them.
pub fn hk_rigid_motion_util_apply_forces_and_build_accumulators(
    e: &mut Engine,
    info: Ptr,
    motions: Ptr,
    count: i32,
    offset: u32,
    accumulators: Ptr,
) -> Ptr {
    let mut remaining = count.wrapping_sub(1);
    if remaining < 0 {
        return accumulators;
    }
    let info = info.addr();
    let mut list = motions.addr();
    let mut accumulator = accumulators.addr();
    loop {
        let state = e.mem.u32(list).wrapping_add(offset);
        match e.mem.u8(state + 8) {
            0 | 3 | 6 => {
                damp_velocities(e, info, state);
                e.call(
                    MATRIX3_SET_TRANSPOSE,
                    &args![accumulator + 0x50, state + 0x10],
                );
                e.mem.set_u8(accumulator, 0);
                let word = load_bits(e, state + 0xc0);
                store_bits(e, accumulator + 0x30, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                blend_position(e, info, state, accumulator + 0x40);
                let angular = load_vec(e, state + 0xe0);
                let column_1 = load_vec(e, accumulator + 0x60);
                let column_0 = load_vec(e, accumulator + 0x50);
                let sum = add(
                    mul(splat(angular[1]), column_1),
                    mul(splat(angular[0]), column_0),
                );
                let column_2 = load_vec(e, accumulator + 0x70);
                let rotated = add(sum, mul(splat(angular[2]), column_2));
                store_vec(e, accumulator + 0x20, rotated);
                e.mem
                    .set_u32(accumulator + 4, u32::from(e.mem.u8(state + 0xbe)));
                store_group(e, state, accumulator);
            }
            2 | 7 => {
                let kind = if e.mem.u8(state + 8) == 7 { 2 } else { 0 };
                e.mem.set_u8(accumulator, kind);
                damp_velocities(e, info, state);
                let word = load_bits(e, state + 0xc0);
                store_bits(e, accumulator + 0x30, word);
                blend_position(e, info, state, accumulator + 0x40);
                let word = load_bits(e, state + 0xe0);
                store_bits(e, accumulator + 0x20, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                identity_inertia(e, accumulator);
                e.mem
                    .set_u32(accumulator + 4, u32::from(e.mem.u8(state + 0xbe)));
                store_group(e, state, accumulator);
            }
            4 => {
                e.mem.set_u8(accumulator, 1);
                store_bits(e, accumulator + 0x30, [0; 4]);
                let word = load_bits(e, state + 0x60);
                store_bits(e, accumulator + 0x40, word);
                let word = load_bits(e, state + 0xe0);
                store_bits(e, accumulator + 0x20, word);
                let word = load_bits(e, state + 0xd0);
                store_bits(e, accumulator + 0x10, word);
                identity_inertia(e, accumulator);
                store_group(e, state, accumulator);
            }
            5 => {
                e.mem.set_u32(accumulator + 8, 0);
                e.mem.set_u8(accumulator, 1);
                for offset in [0x30, 0x10, 0x20, 0x40, 0x50, 0x60, 0x70] {
                    store_bits(e, accumulator + offset, [0; 4]);
                }
            }
            _ => {}
        }
        list = list.wrapping_add(4);
        accumulator = accumulator.wrapping_add(0x80);
        remaining = remaining.wrapping_sub(1);
        if remaining < 0 {
            break;
        }
    }
    Ptr::new(accumulator)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d2_8680, hk_rigid_motion_util_step(Ptr, Ptr, i32, u32)),
        entry!(
            0x00d2_9830,
            hk_rigid_motion_util_apply_forces_and_build_accumulators(Ptr, Ptr, i32, u32, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The `4 / pi^2` factor, the series coefficients and pi as the exe stores them (doubles).
    const FACTOR: f64 = 0.405_284_7;
    const COEFFICIENTS: [f64; 3] = [0.822_948, 0.130_529, 0.044_408];

    /// The offset of the motion state inside each listed object.
    const OFFSET: u32 = 0x10;

    fn close(got: f32, expected: f64) {
        let error = (f64::from(got) - expected).abs();
        assert!(
            error <= 1e-5 * expected.abs().max(1e-3),
            "{got} is not {expected}"
        );
    }

    fn set_constants(e: &mut Engine) {
        for lane in 0..4 {
            e.set_global(ABS_MASK + 4 * lane, 0x7fff_ffffu32);
            e.set_global(VELOCITY_LIMIT + 4 * lane, 1.0e6f32);
            e.set_global(VELOCITY_RESET + 4 * lane, 7.0f32);
            e.set_global(ONES + 4 * lane, 1.0f32);
        }
        e.set_global(HALF, 0.5f64);
        e.set_global(ANGLE_FACTOR, FACTOR);
        e.set_global(ANGLE_LIMIT, f64::from(0.9f32));
        e.set_global(ANGLE_LIMIT_FLOAT, 0.9f32);
        e.set_global(SERIES_FIRST, COEFFICIENTS[0]);
        e.set_global(SERIES_SECOND, COEFFICIENTS[1]);
        e.set_global(SERIES_THIRD, COEFFICIENTS[2]);
        e.set_global(PI, f64::from(std::f32::consts::PI));
        // Quality 1 allows a speed of 10 and a step of 0.5; quality 2 a step of 5.
        e.set_global(QUALITY_TABLE + 4, 10.0f32);
        e.set_global(QUALITY_TABLE + 8, 0.5f32);
    }

    /// The step info: the first float, the time step and the other float.
    fn info(e: &mut Engine, first: f32, step: f32, second: f32) -> u32 {
        let info = e.mem.alloc(0x10);
        e.mem.set_f32(info, first);
        e.mem.set_f32(info + 8, step);
        e.mem.set_f32(info + 0xc, second);
        info
    }

    /// A motion state of `kind` (filled with `0xab`) inside an object `OFFSET` bytes before it,
    /// and a list holding the objects. Returns (list, states).
    fn motions(e: &mut Engine, kinds: &[u8]) -> (u32, Vec<u32>) {
        let list = e.mem.alloc(4 * kinds.len() as u32 + 4);
        let mut states = Vec::new();
        for (index, kind) in kinds.iter().enumerate() {
            let object = e.mem.alloc(0x180);
            e.mem.write(object, &[0xab; 0x180]);
            let state = object + OFFSET;
            e.mem.set_u8(state + 8, *kind);
            e.mem.set_u32(list + 4 * index as u32, object);
            states.push(state);
        }
        (list, states)
    }

    fn quiet_state(e: &mut Engine, state: u32) {
        // Valid velocities and zero everything the step reads as a vector.
        for offset in [0x10, 0x20, 0x30, 0x50, 0x60, 0x80, 0x90, 0xd0, 0xe0] {
            store_vec(e, state + offset, [0.0; 4]);
        }
        e.mem.set_u8(state + 0xbc, 1);
        e.mem.set_u8(state + 0xbd, 2);
    }

    /// The doubles of the step: the root (host), the quaternion product (writes `product`),
    /// the normalisation and the rotation set (identity columns). The roots asked for and the
    /// deltas the product saw are returned.
    struct Doubles {
        log: Log,
        roots: Rc<RefCell<Vec<f64>>>,
        deltas: Rc<RefCell<Vec<Vec4>>>,
    }

    const PRODUCT: Vec4 = [0.1, 0.2, 0.3, 0.9];

    fn step_doubles(e: &mut Engine) -> Doubles {
        let log = new_log();
        let roots: Rc<RefCell<Vec<f64>>> = Rc::default();
        let seen = Rc::clone(&roots);
        e.register_double(X87_SQRT, move |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            seen.borrow_mut().push(value);
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        let deltas: Rc<RefCell<Vec<Vec4>>> = Rc::default();
        let seen = Rc::clone(&deltas);
        let logged = Rc::clone(&log);
        e.register_double(QUATERNION_SET_MUL, move |e, a| {
            logged.borrow_mut().push((QUATERNION_SET_MUL, a.to_vec()));
            seen.borrow_mut().push(load_vec(e, a[1]));
            store_vec(e, a[0], PRODUCT);
            Ret::default()
        });
        log_double(e, &log, QUATERNION_NORMALIZE, 0);
        let logged = Rc::clone(&log);
        e.register_double(SET_ROTATION, move |e, a| {
            logged.borrow_mut().push((SET_ROTATION, a.to_vec()));
            store_vec(e, a[0], [1.0, 0.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x10, [0.0, 1.0, 0.0, 0.0]);
            store_vec(e, a[0] + 0x20, [0.0, 0.0, 1.0, 0.0]);
            Ret::default()
        });
        Doubles { log, roots, deltas }
    }

    #[test]
    fn a_motion_of_type_5_is_left_alone_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 1.0, 0.5, 2.0);
        let (list, states) = motions(&mut e, &[5]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        for offset in 0..0x140 {
            assert_eq!(e.mem.u8(states[0] + offset + 9), 0xab, "offset {offset:#x}");
        }
        assert!(doubles.log.borrow().is_empty());
        assert!(doubles.roots.borrow().is_empty());
    }

    #[test]
    fn no_motions_do_nothing_00d28680() {
        for count in [0i32, -3] {
            let mut e = engine();
            set_constants(&mut e);
            let doubles = step_doubles(&mut e);
            let info = info(&mut e, 1.0, 0.5, 2.0);
            let (list, states) = motions(&mut e, &[0]);
            e.call(0x00d2_8680, &args![info, list, count, OFFSET]);
            assert_eq!(e.mem.u8(states[0] + 9), 0xab);
            assert!(doubles.roots.borrow().is_empty());
        }
    }

    #[test]
    fn a_slow_motion_moves_by_its_velocity_and_turns_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 0.5, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, state + 0x60, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, state + 0x80, [0.5; 4]);
        store_vec(&mut e, state + 0x90, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        // The previous position carries the first float; the position moved by 0.5 * 3 and
        // carries the other float.
        assert_eq!(load_vec(&e, state + 0x50), [1.0, 2.0, 3.0, 7.0]);
        assert_eq!(load_vec(&e, state + 0x60), [2.5, 2.0, 3.0, 9.0]);
        assert_eq!(load_vec(&e, state + 0x70), [0.5; 4]);
        // No angular velocity: the delta is (0, 0, 0, cos = 1) and the angle is 0.
        assert_eq!(*doubles.deltas.borrow(), vec![[0.0, 0.0, 0.0, 1.0]]);
        assert_eq!(*doubles.roots.borrow(), vec![0.0]);
        assert_eq!(load_vec(&e, state + 0xa0), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, state + 0x80), PRODUCT);
        let frame_calls = calls(&doubles.log, QUATERNION_SET_MUL);
        assert_eq!(frame_calls.len(), 1);
        let frame = frame_calls[0][0];
        assert_eq!(frame_calls[0][1..], [frame + 0x10, frame]);
        assert_eq!(calls(&doubles.log, QUATERNION_NORMALIZE), vec![vec![frame]]);
        assert_eq!(
            calls(&doubles.log, SET_ROTATION),
            vec![vec![state + 0x10, state + 0x80]]
        );
        // The translation is the position minus the centre rotated by the identity columns.
        assert_eq!(load_vec(&e, state + 0x40), [1.5, 0.0, 0.0, 9.0]);
        // Bytes the function does not touch keep their fill.
        assert_eq!(e.mem.u8(state + 0xb0), 0xab);
    }

    #[test]
    fn a_fast_motion_is_scaled_to_its_speed_limit_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        e.set_global(QUALITY_TABLE + 4, 1.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 4.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        let inverse = (1.0f64 / 5.0) as f32;
        let factor = (f64::from(inverse) * 1.0) as f32;
        assert_eq!(
            load_vec(&e, state + 0xd0),
            [factor * 3.0, factor * 4.0, factor * 0.0, factor * 0.0]
        );
        // The position moved by the scaled velocity (time step 1).
        assert_eq!(
            load_vec(&e, state + 0x60)[..3],
            [factor * 3.0, factor * 4.0, 0.0]
        );
        assert_eq!(*doubles.roots.borrow(), vec![25.0, 0.0]);
    }

    #[test]
    fn a_speed_exactly_at_the_limit_is_kept_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        e.set_global(QUALITY_TABLE + 4, 5.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xd0, [3.0, 4.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        assert_eq!(load_vec(&e, state + 0xd0), [3.0, 4.0, 0.0, 0.0]);
        assert_eq!(*doubles.roots.borrow(), vec![0.0]);
    }

    #[test]
    fn invalid_velocities_are_reset_00d28680() {
        let cases: [(Vec4, Vec4, bool); 6] = [
            ([0.0; 4], [f32::NAN, 0.0, 0.0, 0.0], true),
            ([0.0; 4], [0.0, 0.0, 2.0e6, 0.0], true),
            ([0.0, -1.0e6, 0.0, 0.0], [0.0; 4], true),
            ([0.0; 4], [0.0, 0.0, f32::NEG_INFINITY, 0.0], true),
            // The fourth lane is not looked at.
            ([0.0, 0.0, 0.0, 5.0e6], [0.0, 0.0, 0.0, f32::NAN], false),
            ([1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0], false),
        ];
        for (linear, angular, reset) in cases {
            let mut e = engine();
            set_constants(&mut e);
            // A quality whose limits do not rescale anything.
            e.set_global(QUALITY_TABLE + 4, 1.0e9f32);
            let _doubles = step_doubles(&mut e);
            let info = info(&mut e, 7.0, 0.0, 9.0);
            let (list, states) = motions(&mut e, &[1]);
            let state = states[0];
            quiet_state(&mut e, state);
            store_vec(&mut e, state + 0xd0, linear);
            store_vec(&mut e, state + 0xe0, angular);
            e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
            let bits = |v: Vec4| v.map(f32::to_bits);
            if reset {
                assert_eq!(
                    load_vec(&e, state + 0xd0),
                    [7.0; 4],
                    "{linear:?} {angular:?}"
                );
                assert_eq!(load_vec(&e, state + 0xe0), [7.0; 4]);
            } else {
                assert_eq!(bits(load_vec(&e, state + 0xd0)), bits(linear));
            }
        }
    }

    #[test]
    fn a_fast_turn_is_scaled_to_the_allowed_angle_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        // Time step 1: half angle (2, 0, 0), estimate 4 * 0.4052847 = 1.62; quality 2 allows
        // 0.5, so the square 0.25 is below the estimate.
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        let estimate = (4.0 * FACTOR) as f32;
        let roots = doubles.roots.borrow().clone();
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[0], f64::from(estimate));
        assert_eq!(roots[1], 0.25);
        let inverse = (1.0 / f64::from(estimate).sqrt() as f32 as f64) as f32;
        let factor = (f64::from(inverse) * 0.5) as f32;
        let angular = load_vec(&e, state + 0xe0);
        assert_eq!(angular[0], factor * 4.0);
        assert_eq!(angular[1], factor * 0.0);
        // The delta: the scaled half angle and the cosine of the allowed angle.
        let deltas = doubles.deltas.borrow();
        assert_eq!(deltas[0][0], factor * 2.0);
        let squared = 0.25f64;
        close(
            deltas[0][3],
            1.0 - COEFFICIENTS[0] * squared
                - COEFFICIENTS[1] * squared * squared
                - squared * squared * squared * COEFFICIENTS[2],
        );
        // Twice the delta, and the angle (the root of the allowed square, 0.5) times pi.
        close(load_vec(&e, state + 0xa0)[0], f64::from(factor * 2.0 * 2.0));
        close(load_vec(&e, state + 0xa0)[3], 0.5 * std::f64::consts::PI);
    }

    #[test]
    fn the_allowed_step_is_capped_at_0_9_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        // Quality 2 now allows 5 per time unit, over 0.9.
        e.set_global(QUALITY_TABLE + 8, 5.0f32);
        let doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [4.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        // The estimate 1.62 is above the capped square (0.81) but below 25 (uncapped).
        let capped = (f64::from(0.9f32) * f64::from(0.9f32)) as f32;
        let roots = doubles.roots.borrow().clone();
        assert_eq!(roots.len(), 2);
        assert_eq!(roots[1], f64::from(capped));
    }

    #[test]
    fn a_small_turn_keeps_the_angular_velocity_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let doubles = step_doubles(&mut e);
        // Time step 0.1: half angle (0.05, 0, 0), estimate 0.0025 * 0.4052847; quality 2
        // allows 0.05, whose square 0.0025 is above the estimate.
        let info = info(&mut e, 7.0, 0.1, 9.0);
        let (list, states) = motions(&mut e, &[3]);
        let state = states[0];
        quiet_state(&mut e, state);
        store_vec(&mut e, state + 0xe0, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 1i32, OFFSET]);
        assert_eq!(load_vec(&e, state + 0xe0), [1.0, 0.0, 0.0, 0.0]);
        let half = (f64::from(0.1f32) * 0.5) as f32;
        assert_eq!(doubles.deltas.borrow()[0][0], half * 1.0);
        let estimate = f64::from((f64::from(half * 1.0 * (half * 1.0)) * FACTOR) as f32);
        close(
            doubles.deltas.borrow()[0][3],
            1.0 - COEFFICIENTS[0] * estimate - COEFFICIENTS[1] * estimate * estimate,
        );
        // The angle is the root of the estimate times pi.
        assert_eq!(doubles.roots.borrow().len(), 1);
    }

    #[test]
    fn several_motions_are_taken_from_the_list_in_order_00d28680() {
        let mut e = engine();
        set_constants(&mut e);
        let _doubles = step_doubles(&mut e);
        let info = info(&mut e, 7.0, 1.0, 9.0);
        let (list, states) = motions(&mut e, &[0, 5, 6]);
        for state in [states[0], states[2]] {
            quiet_state(&mut e, state);
        }
        store_vec(&mut e, states[0] + 0xd0, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, states[2] + 0xd0, [0.0, 2.0, 0.0, 0.0]);
        e.call(0x00d2_8680, &args![info, list, 3i32, OFFSET]);
        assert_eq!(load_vec(&e, states[0] + 0x60)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u8(states[1] + 0x60), 0xab);
        assert_eq!(load_vec(&e, states[2] + 0x60)[..3], [0.0, 2.0, 0.0]);
    }

    /// The accumulator block for `count` motions, filled with `0xcd`.
    fn accumulators(e: &mut Engine, count: u32) -> u32 {
        let block = e.mem.alloc(0x80 * count.max(1));
        e.mem
            .write(block, &vec![0xcd; 0x80 * count.max(1) as usize]);
        block
    }

    /// The matrix transpose double: it writes three known columns at its destination.
    fn transpose_double(e: &mut Engine, log: &Log) {
        let logged = Rc::clone(log);
        e.register_double(MATRIX3_SET_TRANSPOSE, move |e, a| {
            logged
                .borrow_mut()
                .push((MATRIX3_SET_TRANSPOSE, a.to_vec()));
            store_vec(e, a[0], [1.0, 2.0, 3.0, 0.0]);
            store_vec(e, a[0] + 0x10, [4.0, 5.0, 6.0, 0.0]);
            store_vec(e, a[0] + 0x20, [7.0, 8.0, 9.0, 0.0]);
            Ret::default()
        });
    }

    /// A state for the force functions: positions, velocities, damping and the two bytes.
    fn force_state(e: &mut Engine, state: u32) {
        store_vec(e, state + 0x50, [1.0, 1.0, 1.0, 2.0]);
        store_vec(e, state + 0x60, [3.0, 5.0, 7.0, 0.5]);
        store_vec(e, state + 0xc0, [-1.0, -2.0, -3.0, -4.0]);
        store_vec(e, state + 0xd0, [10.0, 20.0, 30.0, 40.0]);
        store_vec(e, state + 0xe0, [1.0, 10.0, 100.0, 0.0]);
        e.mem.set_f32(state + 0xb4, 0.0);
        e.mem.set_f32(state + 0xb8, 0.0);
        e.mem.set_u8(state + 0xbe, 3);
        e.mem.set_i16(state + 0x11e, -2);
    }

    #[test]
    fn the_motion_types_0_3_and_6_build_the_same_accumulator_00d29830() {
        for kind in [0u8, 3, 6] {
            let mut e = engine();
            set_constants(&mut e);
            let log = new_log();
            transpose_double(&mut e, &log);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            assert_eq!(
                calls(&log, MATRIX3_SET_TRANSPOSE),
                vec![vec![acc + 0x50, state + 0x10]]
            );
            assert_eq!(e.mem.u8(acc), 0);
            assert_eq!(e.mem.u8(acc + 1), 0xcd);
            assert_eq!(e.mem.u32(acc + 4), 3);
            assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
            assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
            assert_eq!(load_vec(&e, acc + 0x30), [-1.0, -2.0, -3.0, -4.0]);
            // t = (5 - 2) * 0.5 = 1.5: (1 - t) * previous + current * t.
            assert_eq!(load_vec(&e, acc + 0x40), [4.0, 7.0, 10.0, -0.25]);
            // The angular velocity (1, 10, 100) through the transposed columns.
            assert_eq!(load_vec(&e, acc + 0x20), [741.0, 852.0, 963.0, 0.0]);
            // The transposed columns stay where the transpose wrote them.
            assert_eq!(load_vec(&e, acc + 0x50), [1.0, 2.0, 3.0, 0.0]);
            // The state's velocities were damped by factors of 1.
            assert_eq!(load_vec(&e, state + 0xd0), [10.0, 20.0, 30.0, 40.0]);
        }
    }

    #[test]
    fn the_damping_factors_come_from_the_time_step_00d29830() {
        for kind in [0u8, 2, 7] {
            let mut e = engine();
            set_constants(&mut e);
            let log = new_log();
            transpose_double(&mut e, &log);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            e.mem.set_f32(state + 0xb4, 0.2);
            e.mem.set_f32(state + 0xb8, 4.0);
            let acc = accumulators(&mut e, 1);
            e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
            let linear = (1.0 - f64::from(0.2f32) * 0.5) as f32;
            assert_eq!(
                load_vec(&e, state + 0xd0),
                [linear * 10.0, linear * 20.0, linear * 30.0, linear * 40.0]
            );
            // 1 - 4 * 0.5 is below zero: the angular velocity is zeroed (not negated).
            let angular = load_vec(&e, state + 0xe0);
            assert_eq!(angular.map(f32::to_bits), [0; 4]);
        }
    }

    #[test]
    fn a_nan_time_step_gives_nan_velocities_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, f32::NAN, 0.0);
        let (list, states) = motions(&mut e, &[2]);
        let state = states[0];
        force_state(&mut e, state);
        e.mem.set_f32(state + 0xb4, 0.2);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert!(load_vec(&e, state + 0xd0).iter().all(|lane| lane.is_nan()));
        assert!(load_vec(&e, state + 0xe0).iter().all(|lane| lane.is_nan()));
    }

    #[test]
    fn the_motion_types_2_and_7_keep_the_velocities_and_the_identity_inertia_00d29830() {
        for (kind, flag) in [(2u8, 0u8), (7, 2)] {
            let mut e = engine();
            set_constants(&mut e);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            let state = states[0];
            force_state(&mut e, state);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            assert_eq!(e.mem.u8(acc), flag);
            assert_eq!(e.mem.u32(acc + 4), 3);
            assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
            assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
            assert_eq!(load_vec(&e, acc + 0x20), [1.0, 10.0, 100.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x30), [-1.0, -2.0, -3.0, -4.0]);
            assert_eq!(load_vec(&e, acc + 0x40), [4.0, 7.0, 10.0, -0.25]);
            assert_eq!(load_vec(&e, acc + 0x50), [1.0, 0.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x60), [0.0, 1.0, 0.0, 0.0]);
            assert_eq!(load_vec(&e, acc + 0x70), [0.0, 0.0, 1.0, 0.0]);
        }
    }

    #[test]
    fn the_motion_type_4_copies_the_position_and_velocities_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[4]);
        let state = states[0];
        force_state(&mut e, state);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert_eq!(e.mem.u8(acc), 1);
        // The word at +4 is not written for this type.
        assert_eq!(e.mem.u32(acc + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(acc + 8), 0xfffe_0000);
        assert_eq!(load_vec(&e, acc + 0x10), [10.0, 20.0, 30.0, 40.0]);
        assert_eq!(load_vec(&e, acc + 0x20), [1.0, 10.0, 100.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x30), [0.0; 4]);
        assert_eq!(load_vec(&e, acc + 0x40), [3.0, 5.0, 7.0, 0.5]);
        assert_eq!(load_vec(&e, acc + 0x50), [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x60), [0.0, 1.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, acc + 0x70), [0.0, 0.0, 1.0, 0.0]);
        // The state itself is not damped.
        assert_eq!(load_vec(&e, state + 0xd0), [10.0, 20.0, 30.0, 40.0]);
    }

    #[test]
    fn the_motion_type_5_clears_the_accumulator_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[5]);
        force_state(&mut e, states[0]);
        let acc = accumulators(&mut e, 1);
        e.call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc]);
        assert_eq!(e.mem.u8(acc), 1);
        assert_eq!(e.mem.u8(acc + 1), 0xcd);
        assert_eq!(e.mem.u32(acc + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(acc + 8), 0);
        for offset in [0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70] {
            assert_eq!(load_vec(&e, acc + offset), [0.0; 4], "{offset:#x}");
        }
        assert_eq!(e.mem.u32(acc + 0xc), 0xcdcd_cdcd);
    }

    #[test]
    fn the_motion_types_1_and_above_7_leave_their_accumulator_alone_00d29830() {
        for kind in [1u8, 8, 0xff] {
            let mut e = engine();
            set_constants(&mut e);
            let info = info(&mut e, 5.0, 0.5, 0.0);
            let (list, states) = motions(&mut e, &[kind]);
            force_state(&mut e, states[0]);
            let acc = accumulators(&mut e, 1);
            let end = e
                .call(0x00d2_9830, &args![info, list, 1i32, OFFSET, acc])
                .u32();
            assert_eq!(end, acc + 0x80);
            for offset in 0..0x80 {
                assert_eq!(e.mem.u8(acc + offset), 0xcd);
            }
            assert_eq!(load_vec(&e, states[0] + 0xd0), [10.0, 20.0, 30.0, 40.0]);
        }
    }

    #[test]
    fn the_accumulators_follow_the_list_and_no_motions_answer_the_start_00d29830() {
        let mut e = engine();
        set_constants(&mut e);
        let info = info(&mut e, 5.0, 0.5, 0.0);
        let (list, states) = motions(&mut e, &[5, 1, 4]);
        for state in &states {
            force_state(&mut e, *state);
        }
        let acc = accumulators(&mut e, 3);
        let end = e
            .call(0x00d2_9830, &args![info, list, 3i32, OFFSET, acc])
            .u32();
        assert_eq!(end, acc + 0x180);
        assert_eq!(e.mem.u8(acc), 1);
        assert_eq!(e.mem.u8(acc + 0x80), 0xcd);
        assert_eq!(e.mem.u8(acc + 0x100), 1);
        for count in [0i32, -1] {
            let end = e
                .call(0x00d2_9830, &args![info, list, count, OFFSET, acc + 0x40])
                .u32();
            assert_eq!(end, acc + 0x40);
        }
    }
}
