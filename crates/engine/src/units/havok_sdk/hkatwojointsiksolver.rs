//! `Havok SDK/hkatwojointsiksolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SSE code computes lane by lane in `f32`; x87 code computes in `f64` and rounds when a `float`
//! is stored. `RSQRTSS` and `RCPPS` are the host CPU's estimates.

use crate::prelude::*;
use crate::units::havok_sdk::hkpboxmotion::{cross, mul};
use crate::units::havok_sdk::hkpconvexradiusbuilder::{normalising_scale, splat};
use crate::units::havok_sdk::hkpsphereshape::{
    load_bits, load_vec, store_bits, store_vec, sum3, HALF_F32, THREE_F32, ZERO_F32,
};

/// The CRT square root (`00ec6040`, argument in ST0, answers in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `0.5` as a `double` and the two `double` bounds on the dot product (about `-0.999999` and
/// `0.999999`) of `00cdb910`.
const HALF_F64: u32 = 0x0101_1588;
const LOWER_BOUND: u32 = 0x010c_b650;
const UPPER_BOUND: u32 = 0x0109_deb0;
/// The vector `(0, 0, 0, 1)`.
const IDENTITY_ROTATION: u32 = 0x010c_71b0;
/// The function `00cb2700` (`hkquaternion.cpp`, `RET 4`, no name in the engine map): `this` =
/// result, `(vector)`; `00cdb910` hands the result to it when the two directions are (nearly)
/// opposite.
const OPPOSITE_DIRECTIONS: u32 = 0x00cb_2700;

// Translated from 00cdb910 (decompiled, FalloutNV.exe 1.4.0.525)
/// A fractional rotation between two directions (thiscall, `RET 0xc`; no name in the engine
/// map): `this` receives the quaternion that rotates `vector_b` towards `vector_a` by the
/// fraction `t` of the way. With `d` the dot product of the two vectors (the first three
/// lanes, `z + (y + x)`), `h = (((1 - t) + t * d) + 1) * 0.5` is computed on the x87 stack and
/// rounded to a `float`; when `h <= 0` or `d` is below the lower bound (a double) the result is
/// left to `00cb2700` (`this`, `vector_a`). When `d` is above the upper bound the result is the
/// vector `(0, 0, 0, 1)` at `010c71b0`. Otherwise `s` is the CRT square root of `h`
/// (`00ec6040`, rounded to a `float`), `q = (t * 0.5) / s` is rounded to a `float`, and the
/// result is the vector `(cross * q, s)` (`cross` = `vector_b x vector_a`) normalised by the
/// refined `RSQRTSS` scale, zero when the length is zero, in all four lanes.
pub fn fn_00cdb910(e: &mut Engine, this: Ptr, t: f32, vector_a: u32, vector_b: u32) {
    let this = this.addr();
    let a = load_vec(e, vector_a);
    let b = load_vec(e, vector_b);
    let d = sum3(mul(b, a));
    let t64 = f64::from(t);
    let x = ((1.0 - t64) + t64 * f64::from(d)) as f32;
    let half = e.mem.f64(HALF_F64);
    let h = ((1.0 + f64::from(x)) * half) as f32;
    if h <= 0.0 {
        e.call(OPPOSITE_DIRECTIONS, &args![this, vector_a]);
        return;
    }
    if f64::from(d) < e.mem.f64(LOWER_BOUND) {
        e.call(OPPOSITE_DIRECTIONS, &args![this, vector_a]);
        return;
    }
    if f64::from(d) > e.mem.f64(UPPER_BOUND) {
        let identity = load_bits(e, IDENTITY_ROTATION);
        store_bits(e, this, identity);
        return;
    }
    let root = e.call(X87_SQRT, &args![f64::from(h)]).f64() as f32;
    let factor = ((t64 * half) / f64::from(root)) as f32;
    let scaled = mul(cross(a, b), splat(factor));
    let rotation = [scaled[0], scaled[1], scaled[2], root];
    let squares = mul(rotation, rotation);
    let sum = (squares[3] + squares[1]) + (squares[2] + squares[0]);
    let zero = e.mem.f32(ZERO_F32);
    let three = e.mem.f32(THREE_F32);
    let half32 = e.mem.f32(HALF_F32);
    let scale = normalising_scale(sum, zero, half32, three);
    store_vec(e, this, mul(splat(scale), rotation));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cd_b910, fn_00cdb910(Ptr, f32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::FRAC_1_SQRT_2;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(HALF_F64, 0.5);
        e.mem.set_f64(LOWER_BOUND, -0.999999);
        e.mem.set_f64(UPPER_BOUND, 0.999999);
        e.mem.set_f32(ZERO_F32, 0.0);
        e.mem.set_f32(THREE_F32, 3.0);
        e.mem.set_f32(HALF_F32, 0.5);
        for (lane, value) in [0.0f32, 0.0, 0.0, 1.0].into_iter().enumerate() {
            e.mem.set_f32(IDENTITY_ROTATION + 4 * lane as u32, value);
        }
        e.call_log = Some(vec![]);
        e.register_double(X87_SQRT, |_, a| {
            let value = f64::from_bits((u64::from(a[1]) << 32) | u64::from(a[0]));
            Ret {
                st0: value.sqrt(),
                ..Ret::default()
            }
        });
        e.register_double(OPPOSITE_DIRECTIONS, |_, _| Ret::default());
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

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        store_vec(e, at, lanes);
        at
    }

    #[test]
    fn opposite_directions_are_left_to_the_other_function() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [-1.0, 0.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // Half way: h = 0.5 is positive, the dot product -1 is below the lower bound.
        e.call(0x00cd_b910, &args![out, 0.5f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS), vec![vec![out, a]]);
        assert!(calls(&e, X87_SQRT).is_empty());
        assert_eq!(load_vec(&e, out), [9.0; 4]);
        // The whole way: h = 0, the same answer.
        e.call(0x00cd_b910, &args![out, 1.0f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS).len(), 2);
    }

    #[test]
    fn a_negative_h_is_left_to_the_other_function_too() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // t = 4: x = -3 + 0 = -3, h = (1 - 3) * 0.5 = -1.
        e.call(0x00cd_b910, &args![out, 4.0f32, a, b]);
        assert_eq!(calls(&e, OPPOSITE_DIRECTIONS), vec![vec![out, a]]);
    }

    #[test]
    fn parallel_directions_give_the_identity() {
        let mut e = engine();
        let a = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let b = vector(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        e.call(0x00cd_b910, &args![out, 0.5f32, a, b]);
        assert_eq!(load_vec(&e, out), [0.0, 0.0, 0.0, 1.0]);
        assert!(calls(&e, OPPOSITE_DIRECTIONS).is_empty());
        assert!(calls(&e, X87_SQRT).is_empty());
    }

    #[test]
    fn a_quarter_turn_gives_the_half_angle_quaternion() {
        let mut e = engine();
        let a = vector(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = vector(&mut e, [0.0, 1.0, 0.0, 0.0]);
        let out = vector(&mut e, [9.0; 4]);
        // d = 0, t = 1: h = 0.5 and s = sqrt(0.5); the cross term in lane 2 is
        // b.y * a.x - b.x * a.y = 1.
        e.call(0x00cd_b910, &args![out, 1.0f32, a, b]);
        assert_eq!(
            calls(&e, X87_SQRT),
            vec![vec![
                0.5f64.to_bits() as u32,
                (0.5f64.to_bits() >> 32) as u32
            ]]
        );
        let result = load_vec(&e, out);
        let expected = [0.0, 0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2];
        for lane in 0..4 {
            assert!(
                (result[lane] - expected[lane]).abs() < 1e-5,
                "lane {lane}: {result:?}"
            );
        }
    }
}
