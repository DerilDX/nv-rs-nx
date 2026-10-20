//! `fallout shared/pathfinding/navmeshmath.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Segment intersections in the XY plane and a segment-circle test. The
//! 2D cross products are computed as the code does, one x87 operation at a
//! time, each stored as a `float`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `00439ef0`: `this - other` into `out` (`thiscall`: out, other); returns
/// `out`.
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
/// `00439e90`: `this + other` into `out` (`thiscall`: out, other); returns
/// `out`.
const POINT_ADD: u32 = 0x0043_9e90;
/// `004a3760`: `vector * scale` into `out` (`cdecl`: out, scale, vector);
/// returns `out`.
const POINT_SCALE: u32 = 0x004a_3760;
/// `004a7290`: a point's squared length (`thiscall`, ST0).
const POINT_LENGTH_SQUARED: u32 = 0x004a_7290;
/// `NiPoint3::Dot` (engine map), `thiscall`, one word, ST0.
const POINT_DOT: u32 = 0x004b_6190;
/// `006815c0`: returns `this` (a point's trivial constructor).
const SELF: u32 = 0x0068_15c0;
/// `004390c0`: whether two points are equal (`thiscall`, one word).
const POINT_EQUALS: u32 = 0x0043_90c0;
/// The 2D point helpers: `004e8880` subtracts (`thiscall`: out, other),
/// `00532fd0` scales (`cdecl`: out, scale, vector), `004dfe30` adds
/// (`thiscall`: out, other), `00595c80` gives the squared length (`thiscall`, ST0).
const POINT2_SUBTRACT: u32 = 0x004e_8880;
const POINT2_SCALE: u32 = 0x0053_2fd0;
const POINT2_ADD: u32 = 0x004d_fe30;
const POINT2_LENGTH_SQUARED: u32 = 0x0059_5c80;

/// `(u.x - o.x) * (w.y - o.y) - (u.y - o.y) * (w.x - o.x)`, stored as a
/// `float` (its bits).
fn cross(e: &Engine, origin: u32, u: u32, w: u32) -> u32 {
    let m = |a: u32| e.mem.u32(a);
    let ux = x87_sub(x87_load32(m(u)), x87_m32(m(origin)));
    let wy = x87_sub(x87_load32(m(w + 4)), x87_m32(m(origin + 4)));
    let first = x87_mul(ux, wy);
    let uy = x87_sub(x87_load32(m(u + 4)), x87_m32(m(origin + 4)));
    let wx = x87_sub(x87_load32(m(w)), x87_m32(m(origin)));
    let second = x87_mul(uy, wx);
    x87_store32(x87_sub(first, second))
}

/// `FLD a; FMUL b; FCOMP 0.0; TEST AH,0x41`: whether `a * b <= 0`
/// (ordered).
fn product_not_positive(a: u32, b: u32, zero: f64) -> bool {
    x87_mul(x87_load32(a), x87_m32(b)) <= zero
}

/// `FLD a; FSUB b; FDIVR a`: `a / (a - b)`, stored as a `float`.
fn ratio(a: u32, b: u32) -> u32 {
    x87_store32(x87_div(x87_m32(a), x87_sub(x87_load32(a), x87_m32(b))))
}

fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

// Translated from 006ba240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshMath::Find2DIntersectionStable` (engine map), `cdecl`: whether
/// segments `a`-`b` and `c`-`d` cross in the XY plane (each pair of end
/// points on opposite sides of the other segment or on it, `a` and `d` not
/// on the other line), and then the crossing point on `a`-`b` into `out`.
pub fn nav_mesh_math_find_2d_intersection_stable(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    d: Ptr,
    out: Ptr,
) -> bool {
    let (a, b, c, d) = (a.addr(), b.addr(), c.addr(), d.addr());
    let side_d = cross(e, d, a, b);
    let side_c = cross(e, c, a, b);
    let side_a = cross(e, a, c, d);
    let side_b = cross(e, b, c, d);
    let zero: f64 = e.global(ZERO);
    if !product_not_positive(side_d, side_c, zero) {
        return false;
    }
    if !product_not_positive(side_a, side_b, zero) {
        return false;
    }
    // `TEST AH,0x44; JP`: false when exactly 0.
    if x87_load32(side_d) == zero {
        return false;
    }
    if x87_load32(side_b) == zero {
        return false;
    }
    let t = ratio(side_a, side_b);
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        let direction = e.call(POINT_SUBTRACT, &args![b, frame, a]).u32();
        let scaled = e
            .call(
                POINT_SCALE,
                &args![frame + 0xc, x87_quiet_bits(t), direction],
            )
            .u32();
        let point = e.call(POINT_ADD, &args![a, frame + 0x18, scaled]).u32();
        copy_words(e, out.addr(), point, 3);
    });
    true
}

// Translated from 006ba3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: the 2D point (two floats) where segment `a`-`b`
/// meets line `c`-`d` into `result`: `c + (d - c) * t` with `t` from the
/// cross products, when `a` and `b` are on opposite sides of `c`-`d` (or
/// one on it); otherwise the end point of `a`-`b` nearer that point (`b`
/// when not farther). Returns `result`.
pub fn fn_006ba3d0(e: &mut Engine, result: Ptr, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> Ptr {
    let (a, b, c, d) = (a.addr(), b.addr(), c.addr(), d.addr());
    let side_b = cross(e, b, c, d);
    let side_a = cross(e, a, c, d);
    let side_c = cross(e, c, a, b);
    let sum = x87_store32(x87_sub(
        x87_add(x87_load32(side_c), x87_m32(side_a)),
        x87_m32(side_b),
    ));
    let t = ratio(side_c, sum);
    e.with_stack(0x44, |e, frame| {
        let frame = frame.addr();
        let point = frame + 0x3c;
        let direction = e.call(POINT2_SUBTRACT, &args![d, frame + 0x20, c]).u32();
        let scaled = e
            .call(
                POINT2_SCALE,
                &args![frame + 0x18, x87_quiet_bits(t), direction],
            )
            .u32();
        e.call(POINT2_ADD, &args![c, point, scaled]);
        let zero: f64 = e.global(ZERO);
        if product_not_positive(side_b, side_a, zero) {
            copy_words(e, result.addr(), point, 2);
            return;
        }
        let to_a = e
            .call(POINT2_SUBTRACT, &args![point, frame + 0x10, a])
            .u32();
        let distance_a = e.call(POINT2_LENGTH_SQUARED, &args![to_a]).f64();
        let to_b = e.call(POINT2_SUBTRACT, &args![point, frame + 0x8, b]).u32();
        let distance_b = e.call(POINT2_LENGTH_SQUARED, &args![to_b]).f64();
        // `TEST AH,0x41; JNZ`: `b` unless it is farther (ordered).
        if distance_b > distance_a {
            copy_words(e, result.addr(), a, 2);
        } else {
            copy_words(e, result.addr(), b, 2);
        }
    });
    result
}

// Translated from 006ba550 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name, `cdecl`: the point where segment `a`-`b` meets line
/// `c`-`d` in the XY plane into `result` (three floats): `a` when `a`
/// equals `b` (`004390c0`); else `a + (b - a) * t` from the cross products
/// when `a` and `b` are on opposite sides (or one on it); otherwise the end
/// point nearer that point (`b` when not farther). Returns `result`.
pub fn fn_006ba550(e: &mut Engine, result: Ptr, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> Ptr {
    let (a, b, c, d) = (a.addr(), b.addr(), c.addr(), d.addr());
    if e.call(POINT_EQUALS, &args![a, b]).bool() {
        copy_words(e, result.addr(), a, 3);
        return result;
    }
    let side_b = cross(e, b, c, d);
    let side_a = cross(e, a, c, d);
    let t = ratio(side_a, side_b);
    e.with_stack(0x50, |e, frame| {
        let frame = frame.addr();
        let point = frame + 0x44;
        let direction = e.call(POINT_SUBTRACT, &args![b, frame + 0x2c, a]).u32();
        let scaled = e
            .call(
                POINT_SCALE,
                &args![frame + 0x20, x87_quiet_bits(t), direction],
            )
            .u32();
        e.call(POINT_ADD, &args![a, point, scaled]);
        let zero: f64 = e.global(ZERO);
        if product_not_positive(side_b, side_a, zero) {
            copy_words(e, result.addr(), point, 3);
            return;
        }
        let to_a = e.call(POINT_SUBTRACT, &args![point, frame + 0x14, a]).u32();
        let distance_a = e.call(POINT_LENGTH_SQUARED, &args![to_a]).f64();
        let to_b = e.call(POINT_SUBTRACT, &args![point, frame + 0x8, b]).u32();
        let distance_b = e.call(POINT_LENGTH_SQUARED, &args![to_b]).f64();
        if distance_b > distance_a {
            copy_words(e, result.addr(), a, 3);
        } else {
            copy_words(e, result.addr(), b, 3);
        }
    });
    result
}

// Translated from 006ba9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshMath::SegmentCircleIntersection` (engine map), `cdecl`: whether
/// segment `a`-`b` comes closer than `radius` to `center`: the closest
/// point is `a` when the projection is not past `a`, `b` when it is at or
/// past `b`, else `a + (b - a) * t`; true when its squared distance is
/// below `radius * radius`.
pub fn nav_mesh_math_segment_circle_intersection(
    e: &mut Engine,
    a: Ptr,
    b: Ptr,
    center: Ptr,
    radius: f32,
) -> bool {
    let (a, b, center) = (a.addr(), b.addr(), center.addr());
    e.with_stack(0x58, |e, frame| {
        let frame = frame.addr();
        let segment = frame + 0x34;
        let to_center = frame + 0x4c;
        let closest = frame + 0x40;
        e.call(POINT_SUBTRACT, &args![b, segment, a]);
        e.call(POINT_SUBTRACT, &args![center, to_center, a]);
        e.call(SELF, &args![closest]);
        let along = e.call(POINT_DOT, &args![segment, to_center]).f64();
        let along = x87_store32(along);
        let zero: f64 = e.global(ZERO);
        if x87_load32(along) <= zero {
            copy_words(e, closest, a, 3);
        } else {
            let length = e.call(POINT_DOT, &args![segment, segment]).f64();
            let length = x87_store32(length);
            // `TEST AH,1; JNZ`: interpolated when below the length or NaN.
            if x87_load32(along) >= x87_load32(length) {
                copy_words(e, closest, b, 3);
            } else {
                let t = x87_store32(x87_div(x87_load32(along), x87_m32(length)));
                let scaled = e
                    .call(
                        POINT_SCALE,
                        &args![frame + 0x20, x87_quiet_bits(t), segment],
                    )
                    .u32();
                let point = e.call(POINT_ADD, &args![a, frame + 0x14, scaled]).u32();
                copy_words(e, closest, point, 3);
            }
        }
        let offset = e
            .call(POINT_SUBTRACT, &args![center, frame + 8, closest])
            .u32();
        let distance = e.call(POINT_LENGTH_SQUARED, &args![offset]).f64();
        let limit = x87_mul(x87_load32(radius.to_bits()), x87_m32(radius.to_bits()));
        limit > distance
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006b_a240, nav_mesh_math_find_2d_intersection_stable(Ptr, Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006b_a3d0, fn_006ba3d0(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006b_a550, fn_006ba550(Ptr, Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006b_a9e0, nav_mesh_math_segment_circle_intersection(Ptr, Ptr, Ptr, f32) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(e: &mut Engine, values: &[f32]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        p
    }

    fn read(e: &Engine, p: u32, n: u32) -> Vec<f64> {
        (0..n).map(|i| f64::from(e.mem.f32(p + 4 * i))).collect()
    }

    fn write(e: &mut Engine, p: u32, v: &[f64]) {
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x as f32);
        }
    }

    /// Point doubles over plain floats (`n` components).
    fn points(e: &mut Engine) {
        e.map(0x0101_2000, 0x1000);
        e.set_global(ZERO, 0.0f64);
        fn sub(e: &mut Engine, a: &[u32], n: u32) -> Ret {
            let (p, q) = (read(e, a[0], n), read(e, a[2], n));
            let r: Vec<f64> = p.iter().zip(q).map(|(x, y)| x - y).collect();
            write(e, a[1], &r);
            a[1].into_ret()
        }
        fn add(e: &mut Engine, a: &[u32], n: u32) -> Ret {
            let (p, q) = (read(e, a[0], n), read(e, a[2], n));
            let r: Vec<f64> = p.iter().zip(q).map(|(x, y)| x + y).collect();
            write(e, a[1], &r);
            a[1].into_ret()
        }
        fn scale(e: &mut Engine, a: &[u32], n: u32) -> Ret {
            let s = f64::from(f32::from_bits(a[1]));
            let r: Vec<f64> = read(e, a[2], n).iter().map(|x| x * s).collect();
            write(e, a[0], &r);
            a[0].into_ret()
        }
        e.register(POINT_SUBTRACT, |e, a| sub(e, a, 3));
        e.register(POINT2_SUBTRACT, |e, a| sub(e, a, 2));
        e.register(POINT_ADD, |e, a| add(e, a, 3));
        e.register(POINT2_ADD, |e, a| add(e, a, 2));
        e.register(POINT_SCALE, |e, a| scale(e, a, 3));
        e.register(POINT2_SCALE, |e, a| scale(e, a, 2));
        e.register(POINT_LENGTH_SQUARED, |e, a| Ret {
            st0: read(e, a[0], 3).iter().map(|x| x * x).sum(),
            ..Ret::default()
        });
        e.register(POINT2_LENGTH_SQUARED, |e, a| Ret {
            st0: read(e, a[0], 2).iter().map(|x| x * x).sum::<f64>(),
            ..Ret::default()
        });
        e.register(POINT_DOT, |e, a| Ret {
            st0: read(e, a[0], 3)
                .iter()
                .zip(read(e, a[1], 3))
                .map(|(x, y)| x * y)
                .sum(),
            ..Ret::default()
        });
        e.register(SELF, |_, a| a[0].into_ret());
        e.register(POINT_EQUALS, |e, a| {
            (read(e, a[0], 3) == read(e, a[1], 3)).into_ret()
        });
    }

    #[test]
    fn crossing_segments_meet() {
        let mut e = Engine::new();
        points(&mut e);
        let a = point(&mut e, &[0.0, 0.0, 0.0]);
        let b = point(&mut e, &[4.0, 4.0, 8.0]);
        let c = point(&mut e, &[0.0, 4.0, 0.0]);
        let d = point(&mut e, &[4.0, 0.0, 0.0]);
        let out = e.mem.alloc(12);
        assert!(e.call(0x006b_a240, &args![a, b, c, d, out]).bool());
        assert_eq!(read(&e, out, 3), vec![2.0, 2.0, 4.0]);
        // Apart: false.
        let far = point(&mut e, &[10.0, 10.0, 0.0]);
        let far2 = point(&mut e, &[12.0, 9.0, 0.0]);
        assert!(!e.call(0x006b_a240, &args![a, b, far, far2, out]).bool());
        // `d` on the line a-b: false.
        let on = point(&mut e, &[2.0, 2.0, 0.0]);
        assert!(!e.call(0x006b_a240, &args![a, b, c, on, out]).bool());
    }

    #[test]
    fn segment_meets_line_or_nearer_end() {
        let mut e = Engine::new();
        points(&mut e);
        let a = point(&mut e, &[0.0, 0.0, 1.0]);
        let b = point(&mut e, &[4.0, 4.0, 1.0]);
        let c = point(&mut e, &[0.0, 4.0, 0.0]);
        let d = point(&mut e, &[4.0, 0.0, 0.0]);
        let result = e.mem.alloc(12);
        assert_eq!(
            e.call(0x006b_a3d0, &args![result, a, b, c, d]).u32(),
            result
        );
        assert_eq!(read(&e, result, 2), vec![2.0, 2.0]);
        assert_eq!(
            e.call(0x006b_a550, &args![result, a, b, c, d]).u32(),
            result
        );
        assert_eq!(read(&e, result, 3), vec![2.0, 2.0, 1.0]);
        // The line beyond b: the nearer end, b.
        let c2 = point(&mut e, &[0.0, 10.0, 0.0]);
        let d2 = point(&mut e, &[10.0, 0.0, 0.0]);
        e.call(0x006b_a3d0, &args![result, a, b, c2, d2]);
        assert_eq!(read(&e, result, 2), vec![4.0, 4.0]);
        e.call(0x006b_a550, &args![result, a, b, c2, d2]);
        assert_eq!(read(&e, result, 3), vec![4.0, 4.0, 1.0]);
        // Equal ends: a.
        e.call(0x006b_a550, &args![result, a, a, c, d]);
        assert_eq!(read(&e, result, 3), vec![0.0, 0.0, 1.0]);
    }

    #[test]
    fn circle_hits_by_closest_point() {
        let mut e = Engine::new();
        points(&mut e);
        let a = point(&mut e, &[0.0, 0.0, 0.0]);
        let b = point(&mut e, &[10.0, 0.0, 0.0]);
        let hit = |e: &mut Engine, c: &[f32], r: f32| {
            let center = point(e, c);
            e.call(0x006b_a9e0, &args![a, b, center, r]).bool()
        };
        assert!(hit(&mut e, &[5.0, 1.0, 0.0], 2.0));
        assert!(!hit(&mut e, &[5.0, 3.0, 0.0], 2.0));
        assert!(!hit(&mut e, &[5.0, 2.0, 0.0], 2.0));
        // Before a, past b.
        assert!(hit(&mut e, &[-1.0, 0.0, 0.0], 1.5));
        assert!(!hit(&mut e, &[-2.0, 0.0, 0.0], 1.5));
        assert!(hit(&mut e, &[11.0, 0.0, 0.0], 1.5));
        assert!(!hit(&mut e, &[12.0, 0.0, 0.0], 1.5));
    }
}
