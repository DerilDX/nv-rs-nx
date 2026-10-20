//! `NiMain/nimatrix3.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// `NiMatrix3` (Xbox PDB) is nine floats, row by row: m00 +0, m01 +4, m02 +8, m10 +0xc, m11
// +0x10, m12 +0x14, m20 +0x18, m21 +0x1c, m22 +0x20.

/// `NiMatrix3::operator*` (`thiscall` on the left matrix, (result, right); answers the
/// result).
const MATRIX_MULTIPLY: u32 = 0x0043_f8d0;
/// `NiFastATan2` (`cdecl`, two floats; answers in `ST0`).
const FAST_ATAN2: u32 = 0x00a8_11f0;
/// `00ec9650` and `00eca1d0` (the CRT's `asin` and `acos`; the argument and the result in
/// `ST0`).
const ASIN: u32 = 0x00ec_9650;
const ACOS: u32 = 0x00ec_a1d0;
/// The double -1 (`0101a6b0`), the float pi (`0109afc0`), the float pi/2 (`011a9444`) and the
/// double at `01012060` the angle extraction subtracts.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
const PI_FLOAT: u32 = 0x0109_afc0;
const HALF_PI_FLOAT: u32 = 0x011a_9444;
const ANGLE_OFFSET_DOUBLE: u32 = 0x0101_2060;

/// `sin` and `cos` of an angle as `FSINCOS` leaves them, each stored to `float` (computed in
/// `f64`, which can differ from the extended-precision result in the last bit of a float).
fn sin_cos(angle: f64) -> (u32, u32) {
    (x87_store32(angle.sin()), x87_store32(angle.cos()))
}

/// Writes nine words at `at`.
fn put9(e: &mut Engine, at: u32, words: [u32; 9]) {
    for (i, w) in words.iter().enumerate() {
        e.mem.set_u32(at + 4 * i as u32, *w);
    }
}

/// The rotation matrices the Euler constructors build from an angle's `float` sine and
/// cosine: about x `1 0 0 / 0 c s / 0 -s c`, about y `c 0 -s / 0 1 0 / s 0 c`, about z
/// `c s 0 / -s c 0 / 0 0 1`.
fn about_x(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [one, zero, zero, zero, cos, sin, zero, minus_sin, cos]
}

fn about_y(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [cos, zero, minus_sin, zero, one, zero, sin, zero, cos]
}

fn about_z(sin: u32, cos: u32) -> [u32; 9] {
    let (one, zero) = (x87_store32(1.0), x87_store32(0.0));
    let minus_sin = x87_store32(-x87_load32(sin));
    [cos, sin, zero, minus_sin, cos, zero, zero, zero, one]
}

/// `this = outer * (middle * inner)` through `NiMatrix3::operator*`, the middle matrix's
/// buffer taking the second product as the code does, then copied into `this`.
fn compose(e: &mut Engine, this: Ptr, outer: [u32; 9], middle: [u32; 9], inner: [u32; 9]) {
    e.with_stack(0x90, |e, locals| {
        let (o, m, i, r) = (
            locals.addr(),
            locals.addr() + 0x24,
            locals.addr() + 0x48,
            locals.addr() + 0x6c,
        );
        put9(e, o, outer);
        put9(e, m, middle);
        put9(e, i, inner);
        let first = e.call(MATRIX_MULTIPLY, &args![m, r, i]).u32();
        let second = e.call(MATRIX_MULTIPLY, &args![o, m, first]).u32();
        for word in 0..9 {
            let value = e.mem.u32(second + 4 * word);
            e.mem.set_u32(this.addr() + 4 * word, value);
        }
    });
}

// Translated from 00a582f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::TransformVertices` (Xbox PDB; `cdecl`, (matrix, translation, count, source,
/// destination)): each destination vertex is the matrix times the source vertex plus the
/// translation, `x = m00 sx + tx + m01 sy + sz m02`, `y = m10 sx + ty + m11 sy + m12 sz`,
/// `z = m20 sx + tz + sy m21 + m22 sz` in the exe's order, each component read from memory
/// when used (so a transform in place sees the components already written). The exe does
/// four vertices per loop pass while at least four remain (a signed count test), the rest
/// one by one; the order of reads and writes is the same.
pub fn ni_matrix3_transform_vertices(
    e: &mut Engine,
    matrix: Ptr,
    translation: Ptr,
    count: u32,
    source: Ptr,
    destination: Ptr,
) {
    let (m, t) = (matrix.addr(), translation.addr());
    let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let md = |e: &Engine, at: u32| x87_m32(e.mem.u32(at));
    // The unrolled passes (the largest multiple of four vertices, when the count is at least
    // four as a signed number) and the tail (up to the count, unsigned) visit every vertex in
    // order: one loop here.
    for index in 0..count {
        let s = source.addr().wrapping_add(index.wrapping_mul(12));
        let d = destination.addr().wrapping_add(index.wrapping_mul(12));
        let x = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m), md(e, s)), md(e, t)),
                x87_mul(ld(e, m + 4), md(e, s + 4)),
            ),
            x87_mul(ld(e, s + 8), md(e, m + 8)),
        );
        e.mem.set_u32(d, x87_store32(x));
        let y = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m + 0xc), md(e, s)), md(e, t + 4)),
                x87_mul(ld(e, m + 0x10), md(e, s + 4)),
            ),
            x87_mul(ld(e, m + 0x14), md(e, s + 8)),
        );
        e.mem.set_u32(d + 4, x87_store32(y));
        let z = x87_add(
            x87_add(
                x87_add(x87_mul(ld(e, m + 0x18), md(e, s)), md(e, t + 8)),
                x87_mul(ld(e, s + 4), md(e, m + 0x1c)),
            ),
            x87_mul(ld(e, m + 0x20), md(e, s + 8)),
        );
        e.mem.set_u32(d + 8, x87_store32(z));
    }
}

// Translated from 00a58510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nimatrix3.cpp`; `cdecl`, one float; answers in `ST0`):
/// the arc cosine clamped to its domain: pi (the float at `0109afc0`) for a value at or
/// below -1 (the double at `0101a6b0`) or NaN, 0 at or above 1, otherwise `acos`
/// (`00eca1d0`) stored as a `float`.
pub fn fn_00a58510(e: &mut Engine, value: f32) -> f32 {
    let v = x87_load32(value.to_bits());
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    if !(v > minus_one) {
        return f32::from_bits(x87_quiet_bits(e.mem.u32(PI_FLOAT)));
    }
    if !(1.0 > v) {
        return 0.0;
    }
    let angle = e.call(ACOS, &args![v]).f64();
    f32::from_bits(x87_store32(angle))
}

// Translated from 00a592c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::ToEulerAnglesXYZ` (Xbox PDB): the y angle `-asin(m02)` (the arc sine through
/// `00ec9650`, `m02` clamped: pi/2, the float at `011a9444`, at or below -1 or NaN, -pi/2 at
/// or above 1); strictly between -pi/2 and pi/2 the x angle `-atan2(-m12, m22)` and the z
/// angle `-atan2(-m01, m00)` (`NiFastATan2`) and the answer 1; at -pi/2 or below the z angle
/// 0 and the x angle `atan2(m10, m11)` minus the double at `01012060`; at pi/2 or above (or
/// NaN) the z angle 0 and the x angle `0 - atan2(m10, m11)`; those answer 0.
pub fn ni_matrix3_to_euler_angles_xyz(
    e: &mut Engine,
    this: Ptr,
    x_angle: Ptr,
    y_angle: Ptr,
    z_angle: Ptr,
) -> bool {
    let m = this.addr();
    let ld = |e: &Engine, at: u32| x87_load32(e.mem.u32(at));
    let a = x87_load32(x87_store32(ld(e, m + 8)));
    let minus_one = f64::from_bits(e.mem.u64(MINUS_ONE_DOUBLE));
    let half_pi_bits = e.mem.u32(HALF_PI_FLOAT);
    let local = if !(a > minus_one) {
        x87_store32(-x87_load32(half_pi_bits))
    } else if !(1.0 > a) {
        x87_store32(x87_load32(half_pi_bits))
    } else {
        let angle = e.call(ASIN, &args![a]).f64();
        x87_store32(x87_load32(x87_store32(angle)))
    };
    let half_pi = x87_load32(e.mem.u32(HALF_PI_FLOAT));
    let y = x87_load32(x87_store32(-x87_load32(local)));
    e.mem.set_u32(y_angle.addr(), x87_store32(y));
    let atan2 =
        |e: &mut Engine, first: u32, second: u32| e.call(FAST_ATAN2, &args![first, second]).f64();
    let negated = |e: &Engine, at: u32| x87_store32(-x87_load32(e.mem.u32(at)));
    let copied = |e: &Engine, at: u32| x87_store32(x87_load32(e.mem.u32(at)));
    if y < half_pi {
        if -half_pi < y {
            let (first, second) = (negated(e, m + 0x14), copied(e, m + 0x20));
            let x = atan2(e, first, second);
            e.mem.set_u32(x_angle.addr(), x87_store32(-x));
            let (first, second) = (negated(e, m + 4), copied(e, m));
            let z = atan2(e, first, second);
            e.mem.set_u32(z_angle.addr(), x87_store32(-z));
            return true;
        }
        let (first, second) = (copied(e, m + 0xc), copied(e, m + 0x10));
        let r = x87_store32(atan2(e, first, second));
        e.mem.set_u32(z_angle.addr(), x87_store32(0.0));
        let offset = f64::from_bits(e.mem.u64(ANGLE_OFFSET_DOUBLE));
        e.mem
            .set_u32(x_angle.addr(), x87_store32(x87_sub(x87_load32(r), offset)));
        return false;
    }
    let (first, second) = (copied(e, m + 0xc), copied(e, m + 0x10));
    let r = x87_store32(atan2(e, first, second));
    e.mem.set_u32(z_angle.addr(), x87_store32(0.0));
    e.mem
        .set_u32(x_angle.addr(), x87_store32(x87_sub(0.0, x87_load32(r))));
    false
}

// Translated from 00a59540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::FromEulerAnglesXYZ` (Xbox PDB): the rotations about x, y and z by the three
/// angles (sines and cosines by `FSINCOS`, stored as floats), combined as `X * (Y * Z)`
/// through `NiMatrix3::operator*`, copied into `this`.
pub fn ni_matrix3_from_euler_angles_xyz(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    let (sx, cx) = sin_cos(x87_load32(x.to_bits()));
    let (sy, cy) = sin_cos(x87_load32(y.to_bits()));
    let (sz, cz) = sin_cos(x87_load32(z.to_bits()));
    compose(e, this, about_x(sx, cx), about_y(sy, cy), about_z(sz, cz));
}

// Translated from 00a59780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::FromEulerAnglesZYX` (Xbox PDB; the angles about z, y and x in that order):
/// the three rotations combined as `Z * (Y * X)` through `NiMatrix3::operator*`, copied into
/// `this`.
pub fn ni_matrix3_from_euler_angles_zyx(e: &mut Engine, this: Ptr, z: f32, y: f32, x: f32) {
    let (sx, cx) = sin_cos(x87_load32(x.to_bits()));
    let (sy, cy) = sin_cos(x87_load32(y.to_bits()));
    let (sz, cz) = sin_cos(x87_load32(z.to_bits()));
    compose(e, this, about_z(sz, cz), about_y(sy, cy), about_x(sx, cx));
}

// Translated from 00a59a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::LoadBinary` (Xbox PDB): the nine 4-byte entries, row by row, from the
/// stream's input (`NiStream` +0x24c, its read function +8 and position +4), the
/// byte-count word a local set to 4.
pub fn ni_matrix3_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.with_stack(4, |e, count| {
        for word in 0..9 {
            let input = e.mem.u32(stream.addr() + 0x24c);
            let function = e.mem.u32(input + 8);
            e.mem.set_u32(count.addr(), 4);
            let destination = this.addr() + 4 * word;
            let done = e
                .call(function, &args![input, destination, 4u32, count, 1u32])
                .u32();
            let position = e.mem.u32(input + 4).wrapping_add(done);
            e.mem.set_u32(input + 4, position);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a5_82f0,
            ni_matrix3_transform_vertices(Ptr, Ptr, u32, Ptr, Ptr)
        ),
        entry!(0x00a5_8510, fn_00a58510(f32) -> f32),
        entry!(
            0x00a5_92c0,
            ni_matrix3_to_euler_angles_xyz(Ptr, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00a5_9540,
            ni_matrix3_from_euler_angles_xyz(Ptr, f32, f32, f32)
        ),
        entry!(
            0x00a5_9780,
            ni_matrix3_from_euler_angles_zyx(Ptr, f32, f32, f32)
        ),
        entry!(0x00a5_9a30, ni_matrix3_load_binary(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// An engine with the constants and a matrix product double (row by row, in `f32`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_a000, 0x1000);
        e.map(0x0109_a000, 0x1000);
        e.map(0x011a_9000, 0x1000);
        e.mem.set_f64(MINUS_ONE_DOUBLE, -1.0);
        e.mem.set_f32(PI_FLOAT, std::f32::consts::PI);
        e.mem.set_f32(HALF_PI_FLOAT, std::f32::consts::FRAC_PI_2);
        e.mem.set_f64(ANGLE_OFFSET_DOUBLE, 0.0);
        e.register(MATRIX_MULTIPLY, |e, a| {
            let get = |e: &Engine, m: u32, r: u32, c: u32| e.mem.f32(m + 12 * r + 4 * c);
            let mut out = [0.0f32; 9];
            for r in 0..3 {
                for c in 0..3 {
                    out[(r * 3 + c) as usize] = (0..3)
                        .map(|k| get(e, a[0], r, k) * get(e, a[2], k, c))
                        .sum();
                }
            }
            for (i, v) in out.iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * i as u32, *v);
            }
            a[1].into_ret()
        });
        e
    }

    #[test]
    fn transform_vertices_applies_the_matrix_and_translation() {
        let mut e = engine();
        let m = e.mem.alloc(0x24);
        put(&mut e, m, &[0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 2.0]);
        let t = e.mem.alloc(0xc);
        put(&mut e, t, &[10.0, 20.0, 30.0]);
        let count = 5u32;
        let src = e.mem.alloc(12 * count);
        for i in 0..count {
            put(&mut e, src + 12 * i, &[i as f32, 1.0, 1.0]);
        }
        let dst = e.mem.alloc(12 * count);
        e.call(0x00a5_82f0, &args![m, t, count, src, dst]);
        for i in 0..count {
            assert_eq!(
                floats(&e, dst + 12 * i, 3),
                vec![9.0, 20.0 + i as f32, 32.0]
            );
        }
        // In place, the later components see the earlier ones already written.
        let v = e.mem.alloc(12);
        put(&mut e, v, &[3.0, 1.0, 1.0]);
        e.call(0x00a5_82f0, &args![m, t, 1u32, v, v]);
        assert_eq!(floats(&e, v, 3), vec![9.0, 29.0, 32.0]);
    }

    #[test]
    fn clamped_arc_cosine() {
        let mut e = engine();
        e.register(ACOS, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: v.acos(),
                ..Ret::default()
            }
        });
        let acos = |e: &mut Engine, v: f32| e.call(0x00a5_8510, &args![v]).f32();
        assert_eq!(acos(&mut e, -1.0), std::f32::consts::PI);
        assert_eq!(acos(&mut e, f32::NAN), std::f32::consts::PI);
        assert_eq!(acos(&mut e, 1.0), 0.0);
        assert_eq!(acos(&mut e, 2.0), 0.0);
        assert_eq!(acos(&mut e, 0.5), (0.5f64.acos()) as f32);
    }

    #[test]
    fn euler_angles_round_trip_and_the_gimbal_cases() {
        let mut e = engine();
        e.register(ASIN, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            Ret {
                st0: v.asin(),
                ..Ret::default()
            }
        });
        e.register(FAST_ATAN2, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])).atan2(f64::from(f32::from_bits(a[1]))),
            ..Ret::default()
        });
        let m = e.mem.alloc(0x24);
        let out = e.mem.alloc(0xc);
        e.call(0x00a5_9540, &args![m, 0.25f32, 0.5f32, -0.75f32]);
        let ok = e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]).bool();
        assert!(ok);
        let angles = floats(&e, out, 3);
        for (got, want) in angles.iter().zip([0.25f32, 0.5, -0.75]) {
            assert!((got - want).abs() < 1e-5, "{got} {want}");
        }
        // m02 = -1: y = pi/2, z 0, x = 0 - atan2(m10, m11).
        put(&mut e, m, &[0.0, 0.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0]);
        let ok = e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]).bool();
        assert!(!ok);
        assert_eq!(
            floats(&e, out, 3),
            vec![0.0, std::f32::consts::FRAC_PI_2, 0.0]
        );
        // m02 = 1: y = -pi/2, x = atan2(m10, m11) - the offset (0 here).
        put(&mut e, m, &[0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        e.call(0x00a5_92c0, &args![m, out, out + 4, out + 8]);
        assert_eq!(
            floats(&e, out, 3),
            vec![
                std::f32::consts::FRAC_PI_2,
                -std::f32::consts::FRAC_PI_2,
                0.0
            ]
        );
    }

    #[test]
    fn from_euler_angles_zyx_composes_in_reverse() {
        let mut e = engine();
        let m = e.mem.alloc(0x24);
        // A quarter turn about z only.
        let quarter = std::f32::consts::FRAC_PI_2;
        e.call(0x00a5_9780, &args![m, quarter, 0.0f32, 0.0f32]);
        let (s, c) = (
            (f64::from(quarter)).sin() as f32,
            (f64::from(quarter)).cos() as f32,
        );
        assert_eq!(floats(&e, m, 9), vec![c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0]);
        // A quarter turn about x only (the third angle).
        e.call(0x00a5_9780, &args![m, 0.0f32, 0.0f32, quarter]);
        assert_eq!(floats(&e, m, 9), vec![1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c]);
        // XYZ with only y.
        e.call(0x00a5_9540, &args![m, 0.0f32, quarter, 0.0f32]);
        assert_eq!(floats(&e, m, 9), vec![c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c]);
    }

    #[test]
    fn load_binary_reads_nine_words() {
        let mut e = engine();
        let stream = e.mem.alloc(0x300);
        let input = e.mem.alloc(0x10);
        e.mem.set_u32(stream + 0x24c, input);
        e.mem.set_u32(input + 8, 0x0300_0000);
        let mut next = 1u32;
        e.register_double(0x0300_0000, move |e, a| {
            assert_eq!((a[2], e.mem.u32(a[3]), a[4]), (4, 4, 1));
            e.mem.set_u32(a[1], next);
            next += 1;
            4u32.into_ret()
        });
        let m = e.mem.alloc(0x24);
        e.call(0x00a5_9a30, &args![m, stream]);
        let words: Vec<u32> = (0..9).map(|i| e.mem.u32(m + 4 * i)).collect();
        assert_eq!(words, (1..=9).collect::<Vec<u32>>());
        assert_eq!(e.mem.u32(input + 4), 36);
    }
}
