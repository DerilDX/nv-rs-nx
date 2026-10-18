//! `Havok SDK/hkalookatiksolver.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT helpers `00ec9f70` and `00eca0a0` (value in and out in `ST0`; the other
/// translations use them as cosine and sine, not confirmed here), the unit's helper `00ce0c40`
/// (cdecl, one float; result in `ST0`) and `00ce00f0` (cdecl, two floats; result in `ST0`).
const CRT_FIRST_TRIG: u32 = 0x00ec_9f70;
const CRT_SECOND_TRIG: u32 = 0x00ec_a0a0;
const RATIO_ANGLE: u32 = 0x00ce_0c40;
const PAIR_ANGLE: u32 = 0x00ce_00f0;
/// The float stored in the last lane of both results (`01011d78`, zero in the exe) and the
/// double a length is compared against (`01032980`, a float 0.0001 as a double).
const LAST_LANE: u32 = 0x0101_1d78;
const MINIMUM_LENGTH: u32 = 0x0103_2980;

// Translated from 00ce0110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one stack word `vector`; no PDB name): turns the vector `(r, a, b)` at
/// `vector` (the first three floats; each is copied through a load/store pair, which quiets a
/// signalling NaN) into `(cos(a) * (cos(b) * r), sin(a) * (cos(b) * r), sin(b) * r, w)` in
/// place, with `cos` and `sin` the two CRT helpers and `w` the float at `01011d78`. Every
/// product is rounded to a float.
pub fn fn_00ce0110(e: &mut Engine, vector: Ptr) {
    let at = vector.addr();
    let radius = x87_quiet(e.mem.f32(at));
    let azimuth = x87_quiet(e.mem.f32(at + 4));
    let elevation = x87_quiet(e.mem.f32(at + 8));
    // x87 `FMUL` with the x87 NaN choice (#199), rounded to a float at the store.
    let multiply = |first: f32, second: f32| {
        x87_mul(x87_m32(first.to_bits()), x87_m32(second.to_bits())) as f32
    };
    let cos_elevation = e.call(CRT_FIRST_TRIG, &args![f64::from(elevation)]).f32();
    let horizontal = multiply(cos_elevation, radius);
    let cos_azimuth = e.call(CRT_FIRST_TRIG, &args![f64::from(azimuth)]).f32();
    let x = multiply(cos_azimuth, horizontal);
    let sin_azimuth = e.call(CRT_SECOND_TRIG, &args![f64::from(azimuth)]).f32();
    let y = multiply(sin_azimuth, horizontal);
    let sin_elevation = e.call(CRT_SECOND_TRIG, &args![f64::from(elevation)]).f32();
    let z = multiply(sin_elevation, radius);
    let last = e.mem.u32(LAST_LANE);
    e.mem.set_f32(at, x);
    e.mem.set_f32(at + 4, y);
    e.mem.set_f32(at + 8, z);
    e.mem.set_u32(at + 12, last);
}

// Translated from 00ce01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one stack word `vector`; no PDB name): the inverse of `fn_00ce0110`. With
/// `length` the square root of `z*z + (y*y + x*x)` (the products and sums as the SSE code
/// orders them), a length below the double at `01032980` (0.0001) makes the vector all zero;
/// otherwise it becomes `(length, pair_angle(y, x), ratio_angle(z / length), w)` in place,
/// with `w` the float at `01011d78`, `pair_angle` the helper at `00ce00f0` and `ratio_angle`
/// the one at `00ce0c40`. A NaN length is not below the limit.
pub fn fn_00ce01c0(e: &mut Engine, vector: Ptr) {
    let at = vector.addr();
    let [x, y, z] = [0, 4, 8].map(|offset| e.mem.f32(at + offset));
    let squared_sum = z * z + (y * y + x * x);
    let length = x87_quiet(squared_sum.sqrt());
    let limit = e.mem.f64(MINIMUM_LENGTH);
    let below_limit = matches!(
        f64::from(length).partial_cmp(&limit),
        Some(std::cmp::Ordering::Less)
    );
    if below_limit {
        for offset in [0, 4, 8, 12] {
            e.mem.set_f32(at + offset, 0.0);
        }
        return;
    }
    let x = x87_quiet(e.mem.f32(at));
    let y = x87_quiet(e.mem.f32(at + 4));
    // `z / length` with the x87 NaN choice (#199), rounded to a float.
    let ratio = x87_div(x87_m32(e.mem.u32(at + 8)), x87_m32(length.to_bits())) as f32;
    let phi = e.call(RATIO_ANGLE, &args![ratio]).f32();
    let theta = e.call(PAIR_ANGLE, &args![y, x]).f32();
    let last = e.mem.u32(LAST_LANE);
    e.mem.set_f32(at, length);
    e.mem.set_f32(at + 4, theta);
    e.mem.set_f32(at + 8, phi);
    e.mem.set_u32(at + 12, last);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ce_0110, fn_00ce0110(Ptr)),
        entry!(0x00ce_01c0, fn_00ce01c0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the four helpers as doubles: the first two return the cosine and sine
    /// of their `double` argument, `00ce0c40` returns twice its float argument and `00ce00f0`
    /// its first argument minus its second, all logged.
    fn engine() -> (Engine, Calls) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_f64(MINIMUM_LENGTH, f64::from(0.0001f32));
        let calls: Calls = Rc::default();
        for address in [CRT_FIRST_TRIG, CRT_SECOND_TRIG, RATIO_ANGLE, PAIR_ANGLE] {
            let calls = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                let wide = f64::from_bits(
                    u64::from(a[0]) | (u64::from(a.get(1).copied().unwrap_or(0)) << 32),
                );
                let first = f32::from_bits(a[0]);
                let value = match address {
                    CRT_FIRST_TRIG => wide.cos(),
                    CRT_SECOND_TRIG => wide.sin(),
                    RATIO_ANGLE => f64::from(first * 2.0),
                    _ => f64::from(first - f32::from_bits(a[1])),
                };
                Ret {
                    st0: value,
                    ..Ret::default()
                }
            });
        }
        (e, calls)
    }

    fn vector(e: &mut Engine, lanes: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (lane, value) in lanes.iter().enumerate() {
            e.mem.set_f32(at + 4 * lane as u32, *value);
        }
        at
    }

    fn lanes(e: &Engine, at: u32) -> [u32; 4] {
        [0, 4, 8, 12].map(|offset| e.mem.u32(at + offset))
    }

    #[test]
    fn spherical_to_cartesian_uses_the_helpers_in_the_exe_order() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [2.0, 0.5, 0.25, 9.0]);
        e.mem.set_u32(LAST_LANE, 0x3f00_0000);
        fn_00ce0110(&mut e, Ptr::new(at));
        let cos_elevation = f64::from(0.25f32).cos() as f32;
        let horizontal = cos_elevation * 2.0;
        let x = f64::from(0.5f32).cos() as f32 * horizontal;
        let y = f64::from(0.5f32).sin() as f32 * horizontal;
        let z = f64::from(0.25f32).sin() as f32 * 2.0;
        assert_eq!(
            lanes(&e, at),
            [x.to_bits(), y.to_bits(), z.to_bits(), 0x3f00_0000]
        );
        let order: Vec<u32> = calls.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                CRT_FIRST_TRIG,
                CRT_FIRST_TRIG,
                CRT_SECOND_TRIG,
                CRT_SECOND_TRIG
            ]
        );
        // The helpers get the angles as doubles: elevation, azimuth, azimuth, elevation.
        let angles: Vec<f64> = calls
            .borrow()
            .iter()
            .map(|(_, a)| f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)))
            .collect();
        assert_eq!(angles, vec![0.25, 0.5, 0.5, 0.25]);
    }

    #[test]
    fn spherical_to_cartesian_quiets_signalling_nans() {
        let (mut e, _) = engine();
        let at = e.mem.alloc(16);
        e.mem.set_u32(at, 0x7f80_0001);
        e.mem.set_f32(at + 4, 0.0);
        e.mem.set_f32(at + 8, 0.0);
        fn_00ce0110(&mut e, Ptr::new(at));
        // cos(0) * radius is NaN; the quiet copy of the radius comes out in x, y and z.
        let result = lanes(&e, at);
        assert_eq!(result[0], 0x7fc0_0001);
        assert_eq!(result[1], 0x7fc0_0001);
        assert_eq!(result[2], 0x7fc0_0001);
    }

    #[test]
    fn cartesian_to_spherical_computes_length_and_angles() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [3.0, 4.0, 12.0, 77.0]);
        e.mem.set_u32(LAST_LANE, 0x3f80_0000);
        fn_00ce01c0(&mut e, Ptr::new(at));
        let length = (12.0f32 * 12.0 + (4.0 * 4.0 + 3.0 * 3.0)).sqrt();
        assert_eq!(length, 13.0);
        // The ratio helper doubles its argument, the pair helper subtracts.
        let ratio = 12.0f32 / 13.0;
        assert_eq!(
            lanes(&e, at),
            [
                13.0f32.to_bits(),
                (4.0f32 - 3.0).to_bits(),
                (ratio * 2.0).to_bits(),
                0x3f80_0000
            ]
        );
        assert_eq!(
            *calls.borrow(),
            vec![
                (RATIO_ANGLE, vec![ratio.to_bits()]),
                (PAIR_ANGLE, vec![4.0f32.to_bits(), 3.0f32.to_bits()]),
            ]
        );
    }

    #[test]
    fn a_short_vector_becomes_zero() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [0.00005, 0.0, 0.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(lanes(&e, at), [0; 4]);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn a_vector_just_above_the_limit_is_converted() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [0.0002, 0.0, 0.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(e.mem.f32(at), 0.0002f32.abs());
    }

    #[test]
    fn a_nan_length_is_not_below_the_limit() {
        let (mut e, calls) = engine();
        let at = vector(&mut e, [f32::NAN, 1.0, 1.0, 5.0]);
        fn_00ce01c0(&mut e, Ptr::new(at));
        assert_eq!(calls.borrow().len(), 2);
        assert!(e.mem.f32(at).is_nan());
    }
}
