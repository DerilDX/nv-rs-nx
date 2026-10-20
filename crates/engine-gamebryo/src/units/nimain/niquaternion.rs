//! `NiMain/niquaternion.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Every function here is x87 code on `float` quaternions. Each `FADD`,
//! `FSUB`, `FMUL` and `FDIV` goes through the `x87_*` helpers in the exe's
//! operand order (destination first); `FLD dword` is `x87_load32`, a `float`
//! memory operand `x87_m32`, `FSTP dword` `x87_store32`.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `NiQuaternion` (Xbox PDB), 0x10 bytes; the PC offsets match.
    pub struct NiQuaternion: 0x10 {
        /// `m_fW` (Xbox PDB).
        0x00 m_fW: f32,
        /// `m_fX` (Xbox PDB).
        0x04 m_fX: f32,
        /// `m_fY` (Xbox PDB).
        0x08 m_fY: f32,
        /// `m_fZ` (Xbox PDB).
        0x0C m_fZ: f32,
    }
}

/// CRT `sqrt` helper taking and returning ST0 (`00ec6040`).
const CRT_SQRT: u32 = 0x00ec_6040;
/// CRT `cos` helper taking and returning ST0 (`00ec9f70`).
const CRT_COS: u32 = 0x00ec_9f70;
/// CRT `sin` helper taking and returning ST0 (`00eca0a0`).
const CRT_SIN: u32 = 0x00ec_a0a0;
/// CRT `acos` taking and returning ST0 (`00eca1d0`).
const CRT_ACOS: u32 = 0x00ec_a1d0;
/// `NiQuaternion::FastNormalize` (Xbox PDB), `00a4ff00`.
const FAST_NORMALIZE: u32 = 0x00a4_ff00;

/// `double` 1.0.
const ONE_DOUBLE: u32 = 0x0101_2070;
/// `double` 0.5.
const HALF_DOUBLE: u32 = 0x0101_1588;
/// `double` -1.0.
const MINUS_ONE_DOUBLE: u32 = 0x0101_a6b0;
/// `double` 3.0.
const THREE_DOUBLE: u32 = 0x0102_1928;
/// `float` 0.5.
const HALF_FLOAT: u32 = 0x0101_6248;
/// `float` pi (`0x40490fdb`).
const PI_FLOAT: u32 = 0x0109_c9c4;
/// `float` 0.001: below it the sine counts as zero in the logarithm.
const SINE_EPSILON: u32 = 0x0109_c9c0;
/// `double` factor applied to the cosine in the slerp correction
/// (`0x3fea545a20000000`, about 0.8228).
const SLERP_COSINE_FACTOR: u32 = 0x0109_c9d8;
/// `double` factor of the squared term in the slerp correction
/// (`0x3fe2bc5a20000000`, about 0.5855).
const SLERP_SQUARE_FACTOR: u32 = 0x0109_c9d0;
/// The next-index table `FromRotation` reads (three `int`s, 1, 2, 0 in the
/// exe's data).
const NEXT_INDEX: u32 = 0x011a_9e98;

/// `FLD dword [a]`.
fn ld(e: &Engine, a: u32) -> f64 {
    x87_load32(e.mem.u32(a))
}

/// A `float` memory operand at `a`.
fn m32(e: &Engine, a: u32) -> f64 {
    x87_m32(e.mem.u32(a))
}

/// `FSTP dword [a]`.
fn fst(e: &mut Engine, a: u32, v: f64) {
    e.mem.set_u32(a, x87_store32(v));
}

/// `FSTP dword tmp; FLD dword tmp`: the value rounded to a float and loaded
/// back.
fn spill(v: f64) -> f64 {
    x87_load32(x87_store32(v))
}

/// A `double` constant of the exe's data (an `FMUL qword` operand).
fn double(e: &Engine, a: u32) -> f64 {
    e.mem.f64(a)
}

// Translated from 00a6dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::operator-` (Xbox PDB, unary), returned by value: writes
/// the negated quaternion (each component through `FLD`/`FCHS`/`FSTP`) to
/// `result` and returns `result`.
pub fn niquaternion_negate(
    e: &mut Engine,
    this: Ptr<NiQuaternion>,
    result: Ptr<NiQuaternion>,
) -> Ptr<NiQuaternion> {
    for i in 0..4 {
        let v = ld(e, this.addr() + 4 * i);
        fst(e, result.addr() + 4 * i, -v);
    }
    result
}

// Translated from 00a6dc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::UnitInverse` (Xbox PDB), returned by value (`cdecl`,
/// hidden `result` first): `W` copied, `X`, `Y`, `Z` negated. Returns
/// `result`.
pub fn niquaternion_unit_inverse(
    e: &mut Engine,
    result: Ptr<NiQuaternion>,
    q: Ptr<NiQuaternion>,
) -> Ptr<NiQuaternion> {
    let w = ld(e, q.addr());
    fst(e, result.addr(), w);
    for i in 1..4 {
        let v = ld(e, q.addr() + 4 * i);
        fst(e, result.addr() + 4 * i, -v);
    }
    result
}

// Translated from 00a6dc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The slerp parameter correction (`cdecl`, no Xbox PDB name): with
/// `f = 1 - cos * 0.8228`, `g = f * f * 0.5855` (each rounded to a float),
/// returns `t * (g + (1 + g * t * (2t - 3)))` rounded to a float, in ST0.
pub fn fn_00a6dc30(e: &mut Engine, t: f32, cos: f32) -> f64 {
    let scaled = x87_mul(x87_load32(cos.to_bits()), double(e, SLERP_COSINE_FACTOR));
    let f = spill(x87_sub(1.0, scaled));
    let f2 = spill(x87_mul(f, f));
    let g = spill(x87_mul(f2, double(e, SLERP_SQUARE_FACTOR)));
    let t = x87_load32(t.to_bits());
    let gt = x87_mul(g, t);
    let twice = x87_add(t, t);
    let a = x87_mul(gt, x87_sub(twice, double(e, THREE_DOUBLE)));
    let one_plus = x87_add(1.0, a);
    let sum = x87_add(g, one_plus);
    spill(x87_mul(t, sum))
}

// Translated from 00a6dc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quaternion logarithm, returned by value (`cdecl`, hidden `result`
/// first; no Xbox PDB name): the angle is pi for `W <= -1` (or NaN), 0 for
/// `W >= 1`, `acos(W)` otherwise; the vector part is scaled by
/// `angle / sin(angle)` (1 when `|sin| < 0.001`) and `W` becomes 0.
/// Returns `result`.
pub fn fn_00a6dc90(
    e: &mut Engine,
    result: Ptr<NiQuaternion>,
    q: Ptr<NiQuaternion>,
) -> Ptr<NiQuaternion> {
    let w = spill(ld(e, q.addr()));
    // FCOM qword -1.0; TEST AH,0x41: below, equal or unordered.
    let minus_one = double(e, MINUS_ONE_DOUBLE);
    let angle = if !(w > minus_one) {
        ld(e, PI_FLOAT)
    } else if !(1.0 > w) {
        // FLD1; FCOMP: 1 below or equal to W.
        0.0
    } else {
        spill(e.call(CRT_ACOS, &args![w]).f64())
    };
    let angle_bits = x87_store32(angle);
    let a = x87_load32(angle_bits);
    let sine = spill(spill(e.call(CRT_SIN, &args![a]).f64()));
    let magnitude = spill(sine.abs());
    // FCOMP 0.001; TEST AH,0x5; JP: not below (or unordered) divides.
    let epsilon = m32(e, SINE_EPSILON);
    let scale = if magnitude < epsilon {
        1.0
    } else {
        x87_div(x87_m32(angle_bits), sine)
    };
    let k = x87_load32(x87_store32(scale));
    fst(e, result.addr(), 0.0);
    let x = ld(e, q.addr() + 4);
    fst(e, result.addr() + 4, x87_mul(x, k));
    let y = ld(e, q.addr() + 8);
    fst(e, result.addr() + 8, x87_mul(y, k));
    let z = m32(e, q.addr() + 0xc);
    fst(e, result.addr() + 0xc, x87_mul(k, z));
    result
}

// Translated from 00a6df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::FromRotation` (Xbox PDB): sets this quaternion from the
/// 3x3 rotation matrix `m` (row-major floats). With a positive trace the
/// root is `sqrt(trace + 1)`; otherwise the largest diagonal element `i`
/// (with `j`, `k` from the exe's next-index table) gives the root
/// `sqrt(m_ii - m_jj - m_kk + 1)` and the vector components are written
/// through a local table of pointers to `X`, `Y`, `Z`, indexed as the exe
/// indexes it.
pub fn niquaternion_from_rotation(e: &mut Engine, this: Ptr<NiQuaternion>, m: Ptr) {
    let q = this.addr();
    let m = m.addr();
    let at = |n: u32| m.wrapping_add(n.wrapping_mul(4));
    let s1 = x87_add(ld(e, at(0)), m32(e, at(4)));
    let trace = spill(x87_add(s1, m32(e, at(8))));
    // FCOM against 0; TEST AH,0x41: below, equal or unordered take the
    // other branch.
    if trace > 0.0 {
        let x = spill(x87_add(trace, double(e, ONE_DOUBLE)));
        let r = spill(spill(e.call(CRT_SQRT, &args![x]).f64()));
        let h = double(e, HALF_DOUBLE);
        fst(e, q, x87_mul(r, h));
        let k = spill(x87_div(h, r));
        let d1 = x87_sub(ld(e, at(7)), m32(e, at(5)));
        fst(e, q + 4, x87_mul(d1, k));
        let d2 = x87_sub(ld(e, at(2)), m32(e, at(6)));
        fst(e, q + 8, x87_mul(d2, k));
        let d3 = x87_sub(ld(e, at(3)), m32(e, at(1)));
        fst(e, q + 0xc, x87_mul(k, d3));
        return;
    }
    // FCOMPP; TEST AH,0x5; JP: only "below" picks the other element.
    let mut i = 0u32;
    if ld(e, at(0)) < ld(e, at(4)) {
        i = 1;
    }
    let diag = |n: u32| m.wrapping_add(n << 4);
    if ld(e, diag(i)) < ld(e, at(8)) {
        i = 2;
    }
    let j = e.mem.u32(NEXT_INDEX.wrapping_add(i.wrapping_mul(4)));
    let k = e.mem.u32(NEXT_INDEX.wrapping_add(j.wrapping_mul(4)));
    let a = x87_sub(ld(e, diag(i)), m32(e, diag(j)));
    let b = x87_sub(a, m32(e, diag(k)));
    let x = spill(x87_add(b, double(e, ONE_DOUBLE)));
    let r = spill(spill(e.call(CRT_SQRT, &args![x]).f64()));
    let index = |row: u32, column: u32| at(row.wrapping_mul(3).wrapping_add(column));
    e.with_stack(12, |e, slots| {
        let slots = slots.addr();
        e.mem.set_u32(slots, q + 4);
        e.mem.set_u32(slots + 4, q + 8);
        e.mem.set_u32(slots + 8, q + 0xc);
        let slot = |n: u32| slots.wrapping_add(n.wrapping_mul(4));
        let h = double(e, HALF_DOUBLE);
        let qi = e.mem.u32(slot(i));
        let half_root = spill(x87_mul(r, h));
        fst(e, qi, half_root);
        let kf = spill(x87_div(h, r));
        let wd = x87_sub(ld(e, index(k, j)), m32(e, index(j, k)));
        fst(e, q, x87_mul(wd, kf));
        let sj = x87_add(ld(e, index(i, j)), m32(e, index(j, i)));
        let qj = e.mem.u32(slot(j));
        fst(e, qj, spill(x87_mul(sj, kf)));
        let sk = x87_add(ld(e, index(i, k)), m32(e, index(k, i)));
        let qk = e.mem.u32(slot(k));
        fst(e, qk, spill(x87_mul(kf, sk)));
    });
}

// Translated from 00a6e1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets this quaternion from three angles (no Xbox PDB name): the half
/// angles `-a/2`, `b/2`, `-c/2` give cosines `c0, c1, c2` and sines
/// `s0, s1, s2`, and with `P = c2 c1`, `Q = s2 s1` (floats):
/// `W = P c0 + Q s0`, `X = P s0 - Q c0`, `Y = s0 c1 s2 + s1 c0 c2`,
/// `Z = s2 (c0 c1) - c2 (s0 s1)`.
pub fn fn_00a6e1f0(e: &mut Engine, this: Ptr<NiQuaternion>, a: f32, b: f32, c: f32) {
    let q = this.addr();
    let half = double(e, HALF_DOUBLE);
    let h0 = spill(x87_mul(-x87_load32(a.to_bits()), half));
    let c0 = spill(spill(e.call(CRT_COS, &args![h0]).f64()));
    let h1 = spill(x87_mul(x87_load32(b.to_bits()), half));
    let c1 = spill(spill(e.call(CRT_COS, &args![h1]).f64()));
    let h2 = spill(x87_mul(-x87_load32(c.to_bits()), half));
    let c2 = spill(spill(e.call(CRT_COS, &args![h2]).f64()));
    let s0 = spill(spill(e.call(CRT_SIN, &args![h0]).f64()));
    let s1 = spill(spill(e.call(CRT_SIN, &args![h1]).f64()));
    let s2 = spill(spill(e.call(CRT_SIN, &args![h2]).f64()));
    let p = spill(x87_mul(c2, c1));
    let qq = spill(x87_mul(s2, s1));
    let qs0 = x87_mul(qq, s0);
    let pc0 = x87_mul(p, c0);
    fst(e, q, x87_add(pc0, qs0));
    let ps0 = x87_mul(p, s0);
    let qc0 = x87_mul(qq, c0);
    fst(e, q + 4, x87_sub(ps0, qc0));
    let x = x87_mul(x87_mul(s0, c1), s2);
    let y = x87_mul(x87_mul(s1, c0), c2);
    fst(e, q + 8, x87_add(x, y));
    let c0c1 = x87_mul(c0, c1);
    let s2c0c1 = x87_mul(s2, c0c1);
    let s0s1 = x87_mul(s0, s1);
    let c2s0s1 = x87_mul(c2, s0s1);
    fst(e, q + 0xc, x87_sub(s2c0c1, c2s0s1));
}

// Translated from 00a6e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::Slerp` (Xbox PDB): with `cos = p . q`, the parameter is
/// corrected by [`fn_00a6dc30`] (`t <= 0.5`: on `t`; otherwise, a NaN `t`
/// included, `1 - fn(1 - t)`), each component becomes `p + k (q - p)`, and
/// the result is normalized (`NiQuaternion::FastNormalize`, `00a4ff00`).
pub fn niquaternion_slerp(
    e: &mut Engine,
    this: Ptr<NiQuaternion>,
    t: f32,
    p: Ptr<NiQuaternion>,
    q: Ptr<NiQuaternion>,
) {
    let (p, q, out) = (p.addr(), q.addr(), this.addr());
    let d0 = x87_mul(ld(e, p), m32(e, q));
    let d1 = x87_mul(ld(e, p + 4), m32(e, q + 4));
    let d2 = x87_mul(ld(e, p + 8), m32(e, q + 8));
    let d3 = x87_mul(ld(e, p + 0xc), m32(e, q + 0xc));
    let dot_bits = x87_store32(x87_add(x87_add(x87_add(d0, d1), d2), d3));
    let half = ld(e, HALF_FLOAT);
    let tv = x87_load32(t.to_bits());
    let cos = f32::from_bits(x87_quiet_bits(dot_bits));
    // FCOM; TEST AH,0x41; JP: exactly one of C0/C3 (below or equal) calls
    // on `t`; above or unordered take the mirrored path.
    let corrected = if tv <= half {
        fn_00a6dc30(e, f32::from_bits(x87_store32(tv)), cos)
    } else {
        let mirrored = spill(x87_sub(1.0, tv));
        let r = fn_00a6dc30(e, f32::from_bits(x87_store32(mirrored)), cos);
        x87_sub(1.0, r)
    };
    let k = spill(corrected);
    for i in 0..3u32 {
        let pi = spill(ld(e, p + 4 * i));
        let d = x87_sub(ld(e, q + 4 * i), pi);
        let r = if i == 0 {
            x87_add(pi, x87_mul(d, k))
        } else {
            x87_add(x87_mul(d, k), pi)
        };
        fst(e, out + 4 * i, r);
    }
    let p3 = spill(ld(e, p + 0xc));
    let d3 = x87_sub(ld(e, q + 0xc), p3);
    fst(e, out + 0xc, x87_add(x87_mul(k, d3), p3));
    e.call(FAST_NORMALIZE, &args![this]);
}

// Translated from 00a6e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Spherical quadrangle interpolation (no Xbox PDB name): `a = Slerp(t, p,
/// q)` and `b = Slerp(t, s1, s2)` into two locals, then this =
/// `Slerp(2t (1 - t), a, b)`.
pub fn fn_00a6e430(
    e: &mut Engine,
    this: Ptr<NiQuaternion>,
    t: f32,
    p: Ptr<NiQuaternion>,
    s1: Ptr<NiQuaternion>,
    s2: Ptr<NiQuaternion>,
    q: Ptr<NiQuaternion>,
) {
    e.with_stack(0x20, |e, locals| {
        let first: Ptr<NiQuaternion> = Ptr::new(locals.addr());
        let second: Ptr<NiQuaternion> = Ptr::new(locals.addr() + 0x10);
        let tv = x87_load32(t.to_bits());
        let t_arg = f32::from_bits(x87_store32(tv));
        niquaternion_slerp(e, second, t_arg, p, q);
        niquaternion_slerp(e, first, t_arg, s1, s2);
        let one_minus = x87_sub(1.0, tv);
        let twice = x87_add(tv, tv);
        let u = spill(x87_mul(one_minus, twice));
        niquaternion_slerp(e, this, f32::from_bits(x87_store32(u)), second, first);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00a6dbd0,
            niquaternion_negate(Ptr<NiQuaternion>, Ptr<NiQuaternion>) -> Ptr<NiQuaternion>
        ),
        entry!(
            0x00a6dc00,
            niquaternion_unit_inverse(Ptr<NiQuaternion>, Ptr<NiQuaternion>) -> Ptr<NiQuaternion>
        ),
        entry!(0x00a6dc30, fn_00a6dc30(f32, f32) -> f64),
        entry!(
            0x00a6dc90,
            fn_00a6dc90(Ptr<NiQuaternion>, Ptr<NiQuaternion>) -> Ptr<NiQuaternion>
        ),
        entry!(
            0x00a6df40,
            niquaternion_from_rotation(Ptr<NiQuaternion>, Ptr)
        ),
        entry!(0x00a6e1f0, fn_00a6e1f0(Ptr<NiQuaternion>, f32, f32, f32)),
        entry!(
            0x00a6e330,
            niquaternion_slerp(Ptr<NiQuaternion>, f32, Ptr<NiQuaternion>, Ptr<NiQuaternion>)
        ),
        entry!(
            0x00a6e430,
            fn_00a6e430(
                Ptr<NiQuaternion>,
                f32,
                Ptr<NiQuaternion>,
                Ptr<NiQuaternion>,
                Ptr<NiQuaternion>,
                Ptr<NiQuaternion>,
            )
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quat(e: &mut Engine, v: [u32; 4]) -> Ptr<NiQuaternion> {
        let q: Ptr<NiQuaternion> = e.new_object();
        for (i, w) in v.iter().enumerate() {
            e.mem.set_u32(q.addr() + 4 * i as u32, *w);
        }
        q
    }

    fn words(e: &Engine, q: Ptr<NiQuaternion>) -> [u32; 4] {
        [0, 4, 8, 12].map(|o| e.mem.u32(q.addr() + o))
    }

    fn floats(e: &Engine, q: Ptr<NiQuaternion>) -> [f32; 4] {
        words(e, q).map(f32::from_bits)
    }

    /// The exe's constants this unit reads, and doubles for the CRT math
    /// helpers and `FastNormalize`.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_a000,
            0x0102_1000,
            0x0109_c000,
            0x011a_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(MINUS_ONE_DOUBLE, -1.0f64);
        e.set_global(THREE_DOUBLE, 3.0f64);
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(PI_FLOAT, 0x4049_0fdbu32);
        e.set_global(SINE_EPSILON, 0x3a83_126fu32);
        e.set_global(SLERP_COSINE_FACTOR, f64::from_bits(0x3fea_545a_2000_0000));
        e.set_global(SLERP_SQUARE_FACTOR, f64::from_bits(0x3fe2_bc5a_2000_0000));
        e.set_global(NEXT_INDEX, 1u32);
        e.set_global(NEXT_INDEX + 4, 2u32);
        e.set_global(NEXT_INDEX + 8, 0u32);
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(CRT_SIN, |_, a| f64::take(a, &mut 0).sin().into_ret());
        e.register(CRT_COS, |_, a| f64::take(a, &mut 0).cos().into_ret());
        e.register(CRT_ACOS, |_, a| f64::take(a, &mut 0).acos().into_ret());
        e.register(FAST_NORMALIZE, |_, _| Ret::default());
        e
    }

    #[test]
    fn negate_flips_every_component_and_quiets_signalling_nans() {
        let mut e = engine();
        let q = quat(&mut e, [0x3f80_0000, 0xc000_0000, 0x7f80_0001, 0]);
        let out: Ptr<NiQuaternion> = e.new_object();
        let r = e.call(0x00a6_dbd0, &args![q, out]).u32();
        assert_eq!(r, out.addr());
        assert_eq!(
            words(&e, out),
            [0xbf80_0000, 0x4000_0000, 0xffc0_0001, 0x8000_0000]
        );
    }

    #[test]
    fn unit_inverse_keeps_w_and_negates_the_vector() {
        let mut e = engine();
        let q = quat(&mut e, [0x7f80_0002, 0x3f80_0000, 0x8000_0000, 0x4040_0000]);
        let out: Ptr<NiQuaternion> = e.new_object();
        let r = e.call(0x00a6_dc00, &args![out, q]).u32();
        assert_eq!(r, out.addr());
        assert_eq!(words(&e, out), [0x7fc0_0002, 0xbf80_0000, 0, 0xc040_0000]);
    }

    #[test]
    fn slerp_correction_follows_the_polynomial() {
        let mut e = engine();
        let (t, cos) = (0.25f32, 0.5f32);
        let factor = f64::from_bits(0x3fea_545a_2000_0000);
        let square = f64::from_bits(0x3fe2_bc5a_2000_0000);
        let f = (1.0 - cos as f64 * factor) as f32 as f64;
        let f2 = (f * f) as f32 as f64;
        let g = (f2 * square) as f32 as f64;
        let t64 = t as f64;
        let a = g * t64 * (t64 + t64 - 3.0);
        let expected = (t64 * (g + (1.0 + a))) as f32;
        let r = e.call(0x00a6_dc30, &args![t, cos]).f64();
        assert_eq!(r, expected as f64);
    }

    #[test]
    fn log_takes_pi_zero_or_the_arc_cosine() {
        let mut e = engine();
        let out: Ptr<NiQuaternion> = e.new_object();
        // W <= -1: angle pi; the float pi's sine (about -8.7e-8) is below
        // 0.001, so the scale is 1.
        let q = quat(&mut e, [0xbf80_0000, 0x4000_0000, 0x4040_0000, 0x4080_0000]);
        e.call(0x00a6_dc90, &args![out, q]);
        assert_eq!(floats(&e, out), [0.0, 2.0, 3.0, 4.0]);
        // W >= 1: angle 0, sine 0, scale 1.
        let q = quat(&mut e, [0x3f80_0000, 0x4000_0000, 0x4040_0000, 0x4080_0000]);
        e.call(0x00a6_dc90, &args![out, q]);
        assert_eq!(floats(&e, out), [0.0, 2.0, 3.0, 4.0]);
        // Between: angle acos(0) as a float, scaled by angle / sin(angle).
        let q = quat(&mut e, [0, 0x4000_0000, 0x4040_0000, 0x4080_0000]);
        let r = e.call(0x00a6_dc90, &args![out, q]).u32();
        assert_eq!(r, out.addr());
        let angle = std::f64::consts::FRAC_PI_2 as f32;
        let sine = (angle as f64).sin() as f32;
        let k = (angle as f64 / sine as f64) as f32 as f64;
        assert_eq!(
            floats(&e, out),
            [0.0, (2.0 * k) as f32, (3.0 * k) as f32, (k * 4.0) as f32]
        );
        // A NaN W takes the pi branch.
        e.register(CRT_SIN, |_, _| 0.5f64.into_ret());
        let q = quat(&mut e, [0x7fc0_0000, 0x3f80_0000, 0, 0]);
        e.call(0x00a6_dc90, &args![out, q]);
        let k = (f32::from_bits(0x4049_0fdb) as f64 / 0.5) as f32;
        assert_eq!(floats(&e, out), [0.0, k, 0.0, 0.0]);
    }

    fn matrix(e: &mut Engine, m: [f32; 9]) -> Ptr {
        let p = Ptr::new(e.mem.alloc(0x24));
        for (i, v) in m.iter().enumerate() {
            e.mem.set_f32(p.addr() + 4 * i as u32, *v);
        }
        p
    }

    #[test]
    fn from_rotation_uses_the_trace_when_positive() {
        let mut e = engine();
        let q: Ptr<NiQuaternion> = e.new_object();
        let m = matrix(&mut e, [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a6_df40, &args![q, m]);
        assert_eq!(floats(&e, q), [1.0, 0.0, 0.0, 0.0]);
        // A quarter turn about Z: trace 1.
        let m = matrix(&mut e, [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a6_df40, &args![q, m]);
        let r = 2f64.sqrt() as f32 as f64;
        let k = (0.5 / r) as f32 as f64;
        assert_eq!(
            floats(&e, q),
            [(r * 0.5) as f32, 0.0, 0.0, (2.0 * k) as f32]
        );
    }

    #[test]
    fn from_rotation_picks_the_largest_diagonal_otherwise() {
        let mut e = engine();
        let q: Ptr<NiQuaternion> = e.new_object();
        // Half turn about Z: i = 2, j = 0, k = 1.
        let m = matrix(&mut e, [-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0]);
        e.call(0x00a6_df40, &args![q, m]);
        assert_eq!(floats(&e, q), [0.0, 0.0, 0.0, 1.0]);
        // Half turn about Y: m0 < m4 gives i = 1 (j = 2, k = 0).
        let m = matrix(&mut e, [-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0]);
        e.call(0x00a6_df40, &args![q, m]);
        assert_eq!(floats(&e, q), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn from_angles_halves_and_negates_the_first_and_third() {
        let mut e = engine();
        let q: Ptr<NiQuaternion> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00a6_e1f0, &args![q, 2.0f32, 4.0f32, 6.0f32]);
        let log = e.call_log.take().unwrap();
        let inputs: Vec<f64> = log
            .iter()
            .filter(|(a, _)| *a == CRT_COS)
            .map(|(_, w)| f64::take(w, &mut 0))
            .collect();
        assert_eq!(inputs, vec![-1.0, 2.0, -3.0]);
        let c = [-1.0f64, 2.0, -3.0].map(|h| h.cos() as f32 as f64);
        let s = [-1.0f64, 2.0, -3.0].map(|h| h.sin() as f32 as f64);
        let p = (c[2] * c[1]) as f32 as f64;
        let qq = (s[2] * s[1]) as f32 as f64;
        assert_eq!(
            floats(&e, q),
            [
                (p * c[0] + qq * s[0]) as f32,
                (p * s[0] - qq * c[0]) as f32,
                (s[0] * c[1] * s[2] + s[1] * c[0] * c[2]) as f32,
                (s[2] * (c[0] * c[1]) - c[2] * (s[0] * s[1])) as f32,
            ]
        );
        e.call(0x00a6_e1f0, &args![q, 0.0f32, 0.0f32, 0.0f32]);
        assert_eq!(floats(&e, q), [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn slerp_mirrors_the_correction_above_one_half() {
        let mut e = engine();
        let p = quat(&mut e, [0x3f80_0000, 0, 0, 0]);
        let q = quat(&mut e, [0, 0x3f80_0000, 0, 0]);
        let out: Ptr<NiQuaternion> = e.new_object();
        let k = fn_00a6dc30(&mut e, 0.25, 0.0) as f32;
        e.call_log = Some(vec![]);
        e.call(0x00a6_e330, &args![out, 0.25f32, p, q]);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(a, w)| *a == FAST_NORMALIZE && w == &vec![out.addr()]));
        assert_eq!(floats(&e, out), [(1.0 - k as f64) as f32, k, 0.0, 0.0]);
        e.call(0x00a6_e330, &args![out, 0.75f32, p, q]);
        let mirrored = (1.0 - k as f64) as f32;
        assert_eq!(
            floats(&e, out),
            [(1.0 - mirrored as f64) as f32, mirrored, 0.0, 0.0]
        );
        // Exactly one half takes the direct path.
        let half = fn_00a6dc30(&mut e, 0.5, 0.0) as f32;
        e.call(0x00a6_e330, &args![out, 0.5f32, p, q]);
        assert_eq!(floats(&e, out)[1], half);
    }

    #[test]
    fn squad_slerps_the_two_pairs_then_their_results() {
        let mut e = engine();
        let id = [0x3f80_0000, 0, 0, 0];
        let p = quat(&mut e, id);
        let a = quat(&mut e, id);
        let b = quat(&mut e, id);
        let q = quat(&mut e, id);
        let out: Ptr<NiQuaternion> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00a6_e430, &args![out, 0.25f32, p, a, b, q]);
        let log = e.call_log.take().unwrap();
        let normalized: Vec<u32> = log
            .iter()
            .filter(|(f, _)| *f == FAST_NORMALIZE)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(normalized.len(), 3);
        assert_eq!(normalized[2], out.addr());
        assert_eq!(normalized[1] + 0x10, normalized[0]);
        assert_eq!(floats(&e, out), [1.0, 0.0, 0.0, 0.0]);
    }
}
