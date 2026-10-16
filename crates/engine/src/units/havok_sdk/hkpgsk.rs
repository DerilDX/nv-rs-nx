//! `Havok SDK/hkpgsk.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesobjectrefr::rsqrt_estimate;

/// A four-lane single-precision vector (one SSE register).
type Vec4 = [f32; 4];

/// Edge tables in the exe image (`010cb440`, signed bytes): entry `i` of the first table and
/// of the second (`010cb442`) are the two vertex numbers an edge test uses.
const EDGE_TABLE_FIRST: u32 = 0x010c_b440;
const EDGE_TABLE_SECOND: u32 = 0x010c_b442;
/// Signed bytes in the exe image (`010cb438`) indexed by a three-bit sign mask.
const MASK_TABLE: u32 = 0x010c_b438;
/// `-1.0f` (`01012054`).
const MINUS_ONE: u32 = 0x0101_2054;
/// The large float `0x7f7fffee` (`0104da48`) used as "no distance".
const BIG_FLOAT: u32 = 0x0104_da48;
/// A vector of four absolute-value masks (`01017d10`), one of four sign bits (`010c44d0`) and
/// one of four `1.0f` (`010ec7f0`).
const ABS_MASK: u32 = 0x0101_7d10;
const SIGN_MASK: u32 = 0x010c_44d0;
const ONES: u32 = 0x010e_c7f0;
/// A `float` threshold of about `1e-6` (`010c6090`) and a `double` factor of `1.1`
/// (`010290f8`) used by `findClosestTriangle`.
const NEAR_ZERO: u32 = 0x010c_6090;
const TIE_FACTOR: u32 = 0x0102_90f8;
/// `hkLineSegmentUtil::closestLineSegLineSeg` (Xbox PDB, `00d22e40`, cdecl `(pointA, dirA,
/// pointB, dirB, out)`, returns a bit mask).
const CLOSEST_LINE_SEG_LINE_SEG: u32 = 0x00d2_2e40;
/// `hkCalculatePenetrationDepth` (Xbox PDB, `00dfaac0`, cdecl, nine words).
const CALCULATE_PENETRATION_DEPTH: u32 = 0x00df_aac0;

fn ld(e: &Engine, at: u32) -> Vec4 {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn st(e: &mut Engine, at: u32, v: Vec4) {
    for (i, lane) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

/// A 16-byte copy that keeps the bits (integer moves in the exe).
fn copy16(e: &mut Engine, from: u32, to: u32) {
    for i in 0..4 {
        let word = e.mem.u32(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// An `FLD` / `FSTP` round trip of a float in memory (it quiets a signalling NaN).
fn load_float(e: &Engine, at: u32) -> f32 {
    x87_quiet(e.mem.f32(at))
}

fn vsub(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] - b[i])
}

fn vadd(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] + b[i])
}

fn vmul(a: Vec4, b: Vec4) -> Vec4 {
    [0, 1, 2, 3].map(|i| a[i] * b[i])
}

/// `SHUFPS dst, src, imm`: lanes 0 and 1 from `dst`, lanes 2 and 3 from `src`.
fn shufps(dst: Vec4, src: Vec4, imm: u8) -> Vec4 {
    let pick = |v: Vec4, shift: u8| v[usize::from((imm >> shift) & 3)];
    [pick(dst, 0), pick(dst, 2), pick(src, 4), pick(src, 6)]
}

/// `UNPCKLPS dst, src`.
fn unpcklps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], src[0], dst[1], src[1]]
}

/// `UNPCKHPS dst, src`.
fn unpckhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[2], src[2], dst[3], src[3]]
}

/// `MOVLHPS dst, src`.
fn movlhps(dst: Vec4, src: Vec4) -> Vec4 {
    [dst[0], dst[1], src[0], src[1]]
}

/// `MOVHLPS dst, src`.
fn movhlps(dst: Vec4, src: Vec4) -> Vec4 {
    [src[2], src[3], dst[2], dst[3]]
}

/// The horizontal sum of the first three lanes the way the exe forms it: lane 1 plus lane 0
/// (`ADDSS`), then lane 2 plus that.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// The bit-wise AND of each lane with a mask.
fn andps(v: Vec4, mask: [u32; 4]) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() & mask[i]))
}

fn xorps(v: Vec4, mask: [u32; 4]) -> Vec4 {
    [0, 1, 2, 3].map(|i| f32::from_bits(v[i].to_bits() ^ mask[i]))
}

fn mask_at(e: &Engine, at: u32) -> [u32; 4] {
    [0, 4, 8, 12].map(|o| e.mem.u32(at + o))
}

fn signed_byte(e: &Engine, at: u32) -> i32 {
    i32::from(e.mem.u8(at) as i8)
}

// Translated from 00da8390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findClosestTriangleBackup` (Xbox PDB). Register convention: `vertices` is EDI (four
/// 16-byte vectors; the first three are the triangle's, the fourth the apex), the other two
/// words are cdecl stack arguments: `point` (a vector) and `weights` (three floats). For
/// each of the three edges `i` (vertex numbers from the two edge tables) whose two weights
/// are both at least zero, the sign of a cross-product-like determinant of the point and the
/// apex against the edge decides which of the two weights is cleared to `-1.0`. Returns the
/// number of the first weight that is above zero, or -1 when none is.
pub fn find_closest_triangle_backup(
    e: &mut Engine,
    vertices: Ptr,
    point: Ptr,
    weights: Ptr,
) -> i32 {
    let v = vertices.addr();
    let w = weights.addr();
    let minus_one = load_float(e, MINUS_ONE);
    for i in 0..3u32 {
        let second = signed_byte(e, EDGE_TABLE_SECOND + i) as u32;
        let first = signed_byte(e, EDGE_TABLE_FIRST + i) as u32;
        let w_second = e.mem.f32(w + 4 * second);
        let w_first = e.mem.f32(w + 4 * first);
        if w_second >= 0.0 && w_first >= 0.0 {
            let base = ld(e, v + 16 * i);
            let apex = ld(e, v + 0x30);
            let edge_second = vsub(ld(e, v + 16 * second), base);
            let edge_first = vsub(ld(e, v + 16 * first), base);
            let to_point = vsub(ld(e, point.addr()), base);
            let to_apex = vsub(apex, base);
            let point_first = vmul(to_point, edge_first);
            let point_second = vmul(to_point, edge_second);
            let apex_second = vmul(to_apex, edge_second);
            let apex_first = vmul(to_apex, edge_first);
            let determinant =
                sum3(point_second) * sum3(apex_first) - sum3(point_first) * sum3(apex_second);
            // The exe tests the sign bit of the scalar result.
            let negative = determinant.to_bits() >> 31 != 0;
            if negative {
                e.mem.set_f32(w + 4 * first, minus_one);
            } else {
                e.mem.set_f32(w + 4 * second, minus_one);
            }
        }
    }
    for i in 0..3u32 {
        if 0.0 < e.mem.f32(w + 4 * i) {
            return i as i32;
        }
    }
    -1
}

// Translated from 00da8500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::checkTriangleBoundaries` (Xbox PDB), thiscall `(this, query, triangle, update)`.
/// `triangle` holds three vertices at +0/+0x10/+0x20 (and parallel vectors at +0x40/+0x50).
/// Computes the triangle's normal `N` (edge 1-2 cross edge 2-0) and, for the query point,
/// three scaled cross-product sums (the boundary tests against the edges 1-2, 2-0 and 0-1),
/// stores them in `this + 0x160` and returns a three-bit mask of the ones that are negative.
/// With `update` non-zero it also stores `N` at `this + 0x120`; when all three tests are
/// negative and the dot product of `N` with the difference of the vectors at `this + 0x20`
/// and `this + 0xa0` is negative, it flips `N`, swaps vertices 0 and 1 of the triangle (and
/// their parallel vectors at +0x40/+0x50), swaps the first two stored sums and sets
/// `this + 0x14` to 1.
pub fn hkp_gsk_check_triangle_boundaries(
    e: &mut Engine,
    this: Ptr,
    query: Ptr,
    triangle: Ptr,
    update: i32,
) -> u32 {
    let t = this.addr();
    let tr = triangle.addr();
    let p2 = ld(e, tr + 0x20);
    let p1 = ld(e, tr + 0x10);
    let p0 = ld(e, tr);
    let e12 = vsub(p2, p1);
    let e12_zxy = shufps(e12, e12, 0xd2);
    let e12_yzx = shufps(e12, e12, 0xc9);
    let e20 = vsub(p0, p2);
    let e20_yzx = shufps(e20, e20, 0xc9);
    let e20_zxy = shufps(e20, e20, 0xd2);
    let e01 = vsub(p1, p0);
    let q = ld(e, query.addr());
    let normal = vsub(vmul(e12_yzx, e20_zxy), vmul(e12_zxy, e20_yzx));
    let q_p1 = vsub(q, p1);
    let q_p2 = vsub(q, p2);
    let q_p0 = vsub(q, ld(e, tr));
    let c1 = vsub(
        vmul(shufps(q_p1, q_p1, 0xc9), e12_zxy),
        vmul(shufps(q_p1, q_p1, 0xd2), e12_yzx),
    );
    let c2 = vsub(
        vmul(shufps(q_p2, q_p2, 0xc9), e20_zxy),
        vmul(shufps(q_p2, q_p2, 0xd2), e20_yzx),
    );
    let c0 = vsub(
        vmul(shufps(q_p0, q_p0, 0xc9), shufps(e01, e01, 0xd2)),
        vmul(shufps(q_p0, q_p0, 0xd2), shufps(e01, e01, 0xc9)),
    );
    let n1 = vmul(c1, normal);
    let n2 = vmul(c2, normal);
    let n0 = vmul(c0, normal);
    let low = unpcklps(n1, n2);
    let high = unpckhps(n1, n2);
    let x3 = movlhps(low, n0);
    let x2 = shufps(movhlps(x3, low), n0, 0xd4);
    let x0 = shufps(high, n0, 0xe4);
    let sums = vadd(x0, vadd(x2, x3));
    st(e, t + 0x160, sums);
    let mask = (0..3).fold(0u32, |m, i| if sums[i] < 0.0 { m | (1 << i) } else { m });
    if update != 0 {
        st(e, t + 0x120, normal);
        if mask == 7 {
            let toward = vmul(vsub(ld(e, t + 0x20), ld(e, t + 0xa0)), normal);
            if sum3(toward).to_bits() >> 31 != 0 {
                let flipped = xorps(normal, mask_at(e, SIGN_MASK));
                st(e, t + 0x120, flipped);
                for (a, b) in [(tr, tr + 0x10), (tr + 0x40, tr + 0x50)] {
                    let first = [0, 1, 2, 3].map(|i| e.mem.u32(a + 4 * i));
                    copy16(e, b, a);
                    for (i, word) in first.into_iter().enumerate() {
                        e.mem.set_u32(b + 4 * i as u32, word);
                    }
                }
                let kept = load_float(e, t + 0x160);
                let second = load_float(e, t + 0x164);
                e.mem.set_f32(t + 0x160, second);
                e.mem.set_f32(t + 0x164, kept);
                e.mem.set_u32(t + 0x14, 1);
            }
        }
    }
    mask
}

// Translated from 00da8fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `findClosestTriangle` (Xbox PDB). Register convention: `triangle` is EAX (four 16-byte
/// vectors, the first three the triangle's and the fourth the apex), `point` is ESI, and
/// `scale` is a cdecl stack argument. Builds three vectors from cross products of the edges
/// and the scaled difference between the point and the apex, takes `|d| * d` of their combined
/// dot products with the difference, divides each by the squared length of its vector (the
/// large float `0x7f7fffee` when that is zero) and picks the largest. Returns -1 when the
/// largest is below about `1e-6`, its index when it exceeds 1.1 times the second largest,
/// otherwise the result of `findClosestTriangleBackup` (`00da8390`) run on the point with the
/// scaled dot products as weights.
pub fn find_closest_triangle(e: &mut Engine, triangle: Ptr, point: Ptr, scale: f32) -> i32 {
    let tr = triangle.addr();
    let p1 = ld(e, tr + 0x10);
    let p0 = ld(e, tr);
    let p3 = ld(e, tr + 0x30);
    let p2 = ld(e, tr + 0x20);
    let d02 = vsub(p0, p2);
    let d30 = vsub(p3, p0);
    let d21 = vsub(p2, p1);
    let d10 = vsub(p1, p0);
    let d32 = vsub(p3, p2);
    let d31 = vsub(p3, p1);
    let d21_zxy = shufps(d21, d21, 0xd2);
    let d21_yzx = shufps(d21, d21, 0xc9);
    let d31_yzx = shufps(d31, d31, 0xc9);
    let d31_zxy = shufps(d31, d31, 0xd2);
    let a = vsub(vmul(d31_zxy, d21_yzx), vmul(d31_yzx, d21_zxy));
    let d32_yzx = shufps(d32, d32, 0xc9);
    let d32_zxy = shufps(d32, d32, 0xd2);
    let d02_zxy = shufps(d02, d02, 0xd2);
    let d02_yzx = shufps(d02, d02, 0xc9);
    let b = vsub(vmul(d32_zxy, d02_yzx), vmul(d32_yzx, d02_zxy));
    let d30_yzx = shufps(d30, d30, 0xc9);
    let d30_zxy = shufps(d30, d30, 0xd2);
    let d10_zxy = shufps(d10, d10, 0xd2);
    let d10_yzx = shufps(d10, d10, 0xc9);
    let q = ld(e, point.addr());
    let c = vsub(vmul(d30_zxy, d10_yzx), vmul(d30_yzx, d10_zxy));
    let apex = ld(e, tr + 0x30);
    let w = vmul([scale; 4], vsub(q, apex));
    let low = unpcklps(a, b);
    let high = unpckhps(a, b);
    let x5 = movlhps(low, c);
    let x4 = shufps(movhlps(x5, low), c, 0xd4);
    let part_y = vmul(shufps(w, w, 0x55), x4);
    let part_x = vmul(shufps(w, w, 0), x5);
    let part_z = vmul(shufps(high, c, 0xe4), shufps(w, w, 0xaa));
    let d = vadd(vadd(part_y, part_x), part_z);
    let d_squared = vmul(andps(d, mask_at(e, ABS_MASK)), d);
    let big = load_float(e, BIG_FLOAT);
    let mut ratios = [0f32; 3];
    for (i, v) in [a, b, c].into_iter().enumerate() {
        let squares = vmul(v, v);
        let length_squared = x87_quiet(sum3(squares));
        ratios[i] = if length_squared == 0.0 {
            big
        } else {
            (f64::from(d_squared[i]) / f64::from(length_squared)) as f32
        };
    }
    let ratios = ratios.map(x87_quiet);
    let (small, largest_of_two, mut index) = if ratios[1] < ratios[0] {
        (ratios[1], ratios[0], 0)
    } else {
        (ratios[0], ratios[1], 1)
    };
    let (second, largest) = if ratios[2] < largest_of_two {
        if ratios[2] < small {
            (small, largest_of_two)
        } else {
            (ratios[2], largest_of_two)
        }
    } else {
        index = 2;
        (largest_of_two, ratios[2])
    };
    if e.mem.f32(NEAR_ZERO) > largest {
        return -1;
    }
    if f64::from(second) * e.mem.f64(TIE_FACTOR) < f64::from(largest) {
        return index;
    }
    e.with_stack(0x10, |e, weights| {
        st(e, weights.addr(), d);
        find_closest_triangle_backup(e, triangle, point, Ptr::new(weights.addr()))
    })
}

// Translated from 00da9790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::exitAndExportCacheImpl` (Xbox PDB), thiscall `(this, cache)`: writes the two
/// vertex counts as bytes at `cache + 8` and `cache + 9`, then the 16-bit vertex ids of the
/// first simplex (`this + 0x2c`, stride 0x10, `this[0]` of them) followed by those of the
/// second (`this + 0xec`, `this[1]`), each group ordered by the exe's small sorting
/// network (largest first; with three ids the other two are not always in order).
pub fn hkp_gsk_exit_and_export_cache_impl(e: &mut Engine, this: Ptr, cache: Ptr) {
    let t = this.addr();
    let out = cache.addr();
    let first_count = e.mem.u32(t) as i32;
    let second_count = e.mem.u32(t + 4) as i32;
    e.mem.set_u8(out + 8, first_count as u8);
    e.mem.set_u8(out + 9, second_count as u8);
    // The sorted ids of one simplex: (stored at 0, at 1, at 2).
    let sort = |e: &Engine, ids: u32, count: i32| -> (u16, u16, u16) {
        let x0 = e.mem.u16(ids);
        if count < 2 {
            return (x0, 0, 0);
        }
        let x1 = e.mem.u16(ids + 0x10);
        let (mut ax, mut si, mut swaps) = if x1 > x0 { (x1, x0, 1) } else { (x0, x1, 0) };
        let mut dx = 0;
        if count > 2 {
            let x2 = e.mem.u16(ids + 0x20);
            dx = x2;
            if x2 > ax {
                dx = ax;
                ax = x2;
                swaps += 1;
            }
            if swaps == 1 {
                std::mem::swap(&mut dx, &mut si);
            }
        }
        (ax, si, dx)
    };
    let (ax, si, dx) = sort(e, t + 0x2c, first_count);
    if first_count >= 2 {
        if first_count > 2 {
            e.mem.set_u16(out + 4, dx);
        }
        e.mem.set_u16(out + 2, si);
    }
    e.mem.set_u16(out, ax);
    let (ax, si, dx) = sort(e, t + 0xec, second_count);
    let base = out + 2 * first_count as u32;
    if second_count >= 2 {
        if second_count > 2 {
            e.mem.set_u16(base + 4, dx);
        }
        e.mem.set_u16(base + 2, si);
    }
    e.mem.set_u16(base, ax);
}

// Translated from 00da9880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::convertFeatureToClosestDistance` (Xbox PDB), thiscall `(this, input, out)`:
/// copies the 16 bytes at `input` to `out` and the float at `input + 0xc` to `out + 0x20`;
/// then fills `out + 0x10` (a vector) with `this + 0x20` when `this[0] == 1`, else with
/// `input.w * input.xyz + this + 0xa0` (lane-wise, including the fourth lane) when
/// `this[1] == 1`, else with `this + 0x130`.
pub fn hkp_gsk_convert_feature_to_closest_distance(
    e: &mut Engine,
    this: Ptr,
    input: Ptr,
    out: Ptr,
) {
    let t = this.addr();
    let (i, o) = (input.addr(), out.addr());
    let distance = load_float(e, i + 0xc);
    e.mem.set_f32(o + 0x20, distance);
    copy16(e, i, o);
    if e.mem.u32(t) == 1 {
        copy16(e, t + 0x20, o + 0x10);
    } else if e.mem.u32(t + 4) == 1 {
        let w = shufps(ld(e, i), ld(e, i), 0xff);
        let moved = vadd(vmul(w, ld(e, o)), ld(e, t + 0xa0));
        st(e, o + 0x10, moved);
    } else {
        copy16(e, t + 0x130, o + 0x10);
    }
}

fn decrement(e: &mut Engine, at: u32) {
    let value = e.mem.u32(at).wrapping_sub(1);
    e.mem.set_u32(at, value);
}

// Translated from 00da9340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::processEdgeTriangle` (Xbox PDB), thiscall `(this, simplex, triangle, counter_a,
/// counter_b, edge_mode, value)`; the callee pops six words after `this`. `simplex` holds
/// two vectors at +0 and +0x10 (with parallel vectors at +0x40/+0x50), `triangle` three at
/// +0/+0x10/+0x20 (parallel ones at +0x40/+0x50/+0x60); the counters are int pointers.
/// With `edge_mode` non-zero: the signed distances of the two simplex points from the
/// triangle's plane are compared; when they differ in sign the crossing point goes through
/// `checkTriangleBoundaries` (`00da8500`) and a full result (7) returns 1; otherwise the
/// simplex is reordered (nearer point first), checked again with the update flag, and a
/// full result decrements `counter_a` and returns 0. If neither settles it, the two
/// segment-to-edge distances (`00d22e40` with the triangle's edges 1-2 and 0-2) pick which
/// triangle vertex (0 or 1) is replaced by vertex 2, `counter_b` is decremented and 2 is
/// returned. With `edge_mode` zero the stored sums at `this + 0x160` and the stored normal
/// at `this + 0x120` are used instead: opposite-side points are checked for a full result
/// (returns 1), the second point is checked (a full result drops the first point, decrements
/// `counter_a`, stores `value` at `this + 0x174` and returns 0), and otherwise the mask picks
/// which triangle vertex is replaced by vertex 2 (a comparison of saved against current
/// sums decides the ambiguous masks 1, 2 and 4), `counter_b` is decremented and 2 returned.
#[allow(clippy::too_many_arguments)]
pub fn hkp_gsk_process_edge_triangle(
    e: &mut Engine,
    this: Ptr,
    simplex: Ptr,
    triangle: Ptr,
    counter_a: Ptr,
    counter_b: Ptr,
    edge_mode: u8,
    value: u32,
) -> u32 {
    let t = this.addr();
    let s = simplex.addr();
    let tr = triangle.addr();
    e.with_stack(0xc0, |e, frame| {
        let f = frame.addr();
        if edge_mode != 0 {
            let t0 = ld(e, tr);
            let t2 = ld(e, tr + 0x20);
            let t1 = ld(e, tr + 0x10);
            let s0 = ld(e, s);
            let e12 = vsub(t1, t2);
            st(e, f + 0x40, e12);
            let e02 = vsub(t0, t2);
            let e12_zxy = shufps(e12, e12, 0xd2);
            let e12_yzx = shufps(e12, e12, 0xc9);
            let e02_yzx = shufps(e02, e02, 0xc9);
            let e02_zxy = shufps(e02, e02, 0xd2);
            let normal = vsub(vmul(e02_zxy, e12_yzx), vmul(e02_yzx, e12_zxy));
            let d0 = sum3(vmul(vsub(s0, t0), normal));
            st(e, f + 0x30, e02);
            let s1 = ld(e, s + 0x10);
            let d1 = sum3(vmul(vsub(s1, t0), normal));
            if (d0.to_bits() ^ d1.to_bits()) >> 31 != 0 {
                let fraction = d0 / (d0 - d1);
                let ones = ld(e, ONES);
                let point = vadd(vmul(vsub(ones, [fraction; 4]), s0), vmul(s1, [fraction; 4]));
                st(e, f + 0x10, point);
                let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(f + 0x10), triangle, 0);
                if r == 7 {
                    return 1;
                }
            } else {
                if x87_quiet(d1.abs()) < x87_quiet(d0.abs()) {
                    for (a, b) in [(s, s + 0x10), (s + 0x40, s + 0x50)] {
                        let first = [0, 1, 2, 3].map(|i| e.mem.u32(a + 4 * i));
                        copy16(e, b, a);
                        for (i, word) in first.into_iter().enumerate() {
                            e.mem.set_u32(b + 4 * i as u32, word);
                        }
                    }
                }
                let r = hkp_gsk_check_triangle_boundaries(e, this, simplex, triangle, 1);
                if r == 7 {
                    decrement(e, counter_a.addr());
                    return 0;
                }
            }
            let s0 = ld(e, s);
            let s1 = ld(e, s + 0x10);
            st(e, f + 0x10, vsub(s1, s0));
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![s, f + 0x10, tr + 0x20, f + 0x40, f + 0x50],
            );
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![s, f + 0x10, tr + 0x20, f + 0x30, f + 0x80],
            );
            let first_distance = load_float(e, f + 0x70);
            let second_distance = load_float(e, f + 0xa0);
            let which = u32::from(second_distance < first_distance);
            copy16(e, tr + 0x20, tr + 16 * which);
            copy16(e, tr + 0x60, tr + 16 * (which + 4));
            decrement(e, counter_b.addr());
            return 2;
        }
        let t0 = ld(e, tr);
        let s1 = ld(e, s + 0x10);
        let normal = ld(e, t + 0x120);
        let saved = [0, 4, 8, 12].map(|o| e.mem.u32(t + 0x160 + o));
        let e1 = sum3(vmul(vsub(s1, t0), normal));
        let s0 = ld(e, s);
        let e0 = sum3(vmul(vsub(s0, t0), normal));
        if (e0 * e1).to_bits() >> 31 != 0 {
            let fraction = e0 / (e0 - e1);
            let ones = ld(e, ONES);
            let point = vadd(vmul(vsub(ones, [fraction; 4]), s0), vmul(s1, [fraction; 4]));
            st(e, f + 0x30, point);
            let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(f + 0x30), triangle, 0);
            if r == 7 {
                return 1;
            }
        }
        let r = hkp_gsk_check_triangle_boundaries(e, this, Ptr::new(s + 0x10), triangle, 0);
        if r == 7 {
            decrement(e, counter_a.addr());
            copy16(e, s + 0x10, s);
            copy16(e, s + 0x50, s + 0x40);
            e.mem.set_u32(t + 0x174, value);
            return 0;
        }
        let s10 = f32::from_bits(saved[0]);
        let s14 = f32::from_bits(saved[1]);
        let s18 = f32::from_bits(saved[2]);
        let c160 = load_float(e, t + 0x160);
        let c164 = load_float(e, t + 0x164);
        let c168 = load_float(e, t + 0x168);
        // The three exits: replace vertex 0, replace vertex 1, or replace nothing.
        let diff = |current: f32, old: f32| (f64::from(current) - f64::from(old)) as f32;
        let wide = |lhs: f32, a: f32, rhs: f32, b: f32| {
            f64::from(lhs) * f64::from(a) > f64::from(rhs) * f64::from(b)
        };
        // 0: vertex 0, 1: vertex 1, 2: neither.
        let exit = match r {
            6 => 0,
            5 => 1,
            3 => 2,
            1 => {
                let f1 = diff(c164, s14);
                let f2 = diff(c168, s18);
                if wide(f1, s18, s14, f2) {
                    2
                } else {
                    1
                }
            }
            2 => {
                let f1 = diff(c168, s18);
                let f2 = diff(c160, s10);
                if wide(f1, s10, s18, f2) {
                    0
                } else {
                    2
                }
            }
            4 => {
                let f1 = diff(c160, s10);
                let f2 = diff(c164, s14);
                if wide(f1, s14, s10, f2) {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        };
        match exit {
            0 => {
                copy16(e, tr + 0x20, tr);
                copy16(e, tr + 0x60, tr + 0x40);
            }
            1 => {
                copy16(e, tr + 0x20, tr + 0x10);
                copy16(e, tr + 0x60, tr + 0x50);
            }
            _ => {}
        }
        decrement(e, counter_b.addr());
        2
    })
}

/// The floating-point tolerance (`010141cc`) passed to `hkCalculatePenetrationDepth`.
const PENETRATION_TOLERANCE: u32 = 0x0101_41cc;

fn count(e: &Engine, at: u32) -> i32 {
    e.mem.i32(at)
}

// Translated from 00da98f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::reduceDimensionExtended` (Xbox PDB), thiscall with no arguments. The two vertex
/// lists are `this + 0x20` (count at `this`) and `this + 0xa0` (count at `this + 4`, with the
/// matching untransformed vectors at `this + 0xe0`). When one list has three vertices and the
/// other point lies inside it (`checkTriangleBoundaries` returns 7), the other list shrinks to
/// that single point. Otherwise the closest pair of edges (`00d22e40`, over all edge pairs of the
/// three-vertex lists and the single points of the others) is found and the vertex of each
/// three-vertex list the pair's edge starts at is replaced by the list's last vertex while the
/// count drops by one.
pub fn hkp_gsk_reduce_dimension_extended(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let first = |e: &Engine, at: u32| signed_byte(e, at) as u32;
    if count(e, t + 4) == 3 {
        let mut i = 0;
        while i < count(e, t) {
            let r = hkp_gsk_check_triangle_boundaries(
                e,
                this,
                Ptr::new(t + 0x20 + 16 * i as u32),
                Ptr::new(t + 0xa0),
                0,
            );
            if r == 7 {
                copy16(e, t + 0x20 + 16 * i as u32, t + 0x20);
                e.mem.set_i32(t, 1);
                return;
            }
            i += 1;
        }
    }
    if count(e, t) == 3 {
        let mut j = 0;
        while j < count(e, t + 4) {
            let r = hkp_gsk_check_triangle_boundaries(
                e,
                this,
                Ptr::new(t + 0xa0 + 16 * j as u32),
                Ptr::new(t + 0x20),
                0,
            );
            if r == 7 {
                copy16(e, t + 0xe0 + 16 * j as u32, t + 0xe0);
                copy16(e, t + 0xa0 + 16 * j as u32, t + 0xa0);
                e.mem.set_i32(t + 4, 1);
                return;
            }
            j += 1;
        }
    }
    let count_a = if count(e, t) == 3 { 3u32 } else { 1 };
    let count_b = if count(e, t + 4) == 3 { 3u32 } else { 1 };
    let mut best_a = 0u32;
    let mut best_b = 0u32;
    e.with_stack(0x90, |e, frame| {
        let f = frame.addr();
        let mut best = load_float(e, BIG_FLOAT);
        for i in 0..count_a {
            for j in 0..count_b {
                let next_a = first(e, EDGE_TABLE_SECOND + i);
                let next_b = first(e, EDGE_TABLE_SECOND + j);
                let a = ld(e, t + 0x20 + 16 * i);
                let a_next = ld(e, t + 0x20 + 16 * next_a);
                st(e, f + 0x50, vsub(a_next, a));
                let b = ld(e, t + 0xa0 + 16 * j);
                let b_next = ld(e, t + 0xa0 + 16 * next_b);
                st(e, f + 0x40, vsub(b_next, b));
                e.call(
                    CLOSEST_LINE_SEG_LINE_SEG,
                    &args![
                        t + 0x20 + 16 * i,
                        f + 0x50,
                        t + 0xa0 + 16 * j,
                        f + 0x40,
                        f + 0x60
                    ],
                );
                let distance = load_float(e, f + 0x80);
                if best > distance {
                    best = distance;
                    best_a = i;
                    best_b = j;
                }
            }
        }
    });
    if count_a == 3 {
        let n = count(e, t) - 1;
        e.mem.set_i32(t, n);
        let slot = first(e, EDGE_TABLE_FIRST + best_a);
        copy16(e, t + 0x20 + 16 * n as u32, t + 0x20 + 16 * slot);
    }
    if count_b == 3 {
        let n = count(e, t + 4) - 1;
        e.mem.set_i32(t + 4, n);
        let slot = first(e, EDGE_TABLE_FIRST + best_b);
        copy16(e, t + 0xe0 + 16 * n as u32, t + 0xe0 + 16 * slot);
        copy16(e, t + 0xa0 + 16 * n as u32, t + 0xa0 + 16 * slot);
    }
}

fn masked_id(e: &Engine, at: u32) -> u32 {
    e.mem.u32(at) & 0xc0ff_ffff
}

// Translated from 00daa7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::handlePenetration` (Xbox PDB), thiscall `(this, shape_a, shape_b, transform)`;
/// the callee pops three words. Pads the two vertex lists (`this + 0x20` with count `this`,
/// `this + 0xa0` with count `this + 4` and the untransformed vectors at `this + 0xe0`) to a
/// simplex of up to four points by duplicating vertices, builds the differences of the lists
/// and calls `hkCalculatePenetrationDepth` (`00dfaac0`) with the tolerance float at
/// `010141cc`. A result of 1 stores counts of 1, the "converged" flag at `this + 0x14` and the
/// result vector at `this + 0x120`. Otherwise the counts are set to three and reduced for
/// repeated vertex ids (the low 24 bits plus the top two, mask `c0ffffff`), the second list is
/// re-transformed from its untransformed vectors with the 4x4 `transform`,
/// `reduceDimensionExtended` is called when the counts exceed four in total, two lists of two
/// get the segment-to-segment closest points at `this + 0x130`, and the result vector and flag
/// are stored.
pub fn hkp_gsk_handle_penetration(
    e: &mut Engine,
    this: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
) {
    let t = this.addr();
    let count_a = count(e, t);
    let count_b = count(e, t + 4);
    let mut n = 4u32;
    if count_b >= 3 {
        if count_b > 3 {
            copy16(e, t + 0x20, t + 0x50);
        } else if count_a > 1 {
            copy16(e, t + 0x30, t + 0x50);
            copy16(e, t + 0xe0, t + 0x110);
            copy16(e, t + 0xa0, t + 0xd0);
        } else {
            n = 3;
        }
        copy16(e, t + 0x20, t + 0x30);
        copy16(e, t + 0x20, t + 0x40);
    } else if count_a >= 3 {
        if count_a > 3 {
            copy16(e, t + 0xe0, t + 0x110);
            copy16(e, t + 0xa0, t + 0xd0);
        } else if count_b > 1 {
            copy16(e, t + 0x20, t + 0x50);
            copy16(e, t + 0xf0, t + 0x110);
            copy16(e, t + 0xb0, t + 0xd0);
        } else {
            n = 3;
        }
        copy16(e, t + 0xe0, t + 0xf0);
        copy16(e, t + 0xa0, t + 0xb0);
        copy16(e, t + 0xe0, t + 0x100);
        copy16(e, t + 0xa0, t + 0xc0);
    } else {
        let (na, nb) = (count_a as u32, count_b as u32);
        copy16(e, t + 0x20, t + 0x20 + 16 * na);
        copy16(e, t + 0x20, t + 0x30 + 16 * na);
        copy16(e, t + 0xe0, t + 0xe0 + 16 * nb);
        copy16(e, t + 0xe0, t + 0xf0 + 16 * nb);
        copy16(e, t + 0xa0, t + 0xa0 + 16 * nb);
        copy16(e, t + 0xa0, t + 0xb0 + 16 * nb);
        n = 1;
    }
    e.with_stack(0xd0, |e, frame| {
        let f = frame.addr();
        for k in 0..n {
            let a = ld(e, t + 0x20 + 16 * k);
            let b = ld(e, t + 0xa0 + 16 * k);
            st(e, f + 0x80 + 16 * k, vsub(a, b));
        }
        let tolerance = x87_quiet_bits(e.mem.u32(PENETRATION_TOLERANCE));
        let r = e
            .call(
                CALCULATE_PENETRATION_DEPTH,
                &args![
                    shape_a.addr(),
                    shape_b.addr(),
                    transform.addr(),
                    tolerance,
                    t + 0x20,
                    t + 0xe0,
                    f + 0x80,
                    n,
                    f + 0x50
                ],
            )
            .u32();
        if r == 1 {
            e.mem.set_i32(t, 1);
            e.mem.set_i32(t + 4, 1);
            e.mem.set_u32(t + 0x14, 1);
            copy16(e, f + 0x50, t + 0x120);
            return;
        }
        let id_a2 = masked_id(e, t + 0x4c);
        let id_a1 = masked_id(e, t + 0x3c);
        e.mem.set_i32(t, 3);
        e.mem.set_i32(t + 4, 3);
        if id_a1 == id_a2 || masked_id(e, t + 0x2c) == id_a2 {
            e.mem.set_i32(t, 2);
        }
        if masked_id(e, t + 0x2c) == id_a1 {
            let n = count(e, t) - 1;
            e.mem.set_i32(t, n);
            copy16(e, t + 0x20 + 16 * n as u32, t + 0x20);
        }
        let id_b2 = masked_id(e, t + 0x10c);
        let id_b1 = masked_id(e, t + 0xfc);
        if id_b1 == id_b2 || masked_id(e, t + 0xec) == id_b2 {
            let n = count(e, t + 4) - 1;
            e.mem.set_i32(t + 4, n);
        }
        let nb = count(e, t + 4);
        if nb >= 2 && masked_id(e, t + 0xec) == id_b1 {
            e.mem.set_i32(t + 4, nb - 1);
            copy16(e, t + 0xe0 + 16 * (nb - 1) as u32, t + 0xe0);
        }
        let rows = [0, 0x10, 0x20, 0x30].map(|o| ld(e, transform.addr() + o));
        let mut k = count(e, t + 4) - 1;
        while k >= 0 {
            let at = t + 0xe0 + 16 * k as u32;
            let v = ld(e, at);
            let xy = vadd(vmul([v[1]; 4], rows[1]), vmul([v[0]; 4], rows[0]));
            let zw = vadd(vmul([v[2]; 4], rows[2]), rows[3]);
            st(e, at - 0x40, vadd(xy, zw));
            k -= 1;
        }
        if count(e, t) + count(e, t + 4) > 4 {
            hkp_gsk_reduce_dimension_extended(e, this);
        }
        if count(e, t) == 2 && count(e, t + 4) == 2 {
            let a0 = ld(e, t + 0x20);
            let a1 = ld(e, t + 0x30);
            st(e, f + 0x20, vsub(a1, a0));
            let b0 = ld(e, t + 0xa0);
            let b1 = ld(e, t + 0xb0);
            st(e, f + 0x30, vsub(b1, b0));
            e.call(
                CLOSEST_LINE_SEG_LINE_SEG,
                &args![t + 0x20, f + 0x20, t + 0xa0, f + 0x30, t + 0x130],
            );
        }
        e.mem.set_u32(t + 0x14, 1);
        copy16(e, f + 0x50, t + 0x120);
    });
}

/// The starting tolerance (`010d51c4`) doubled on every pass of `getClosestFeature`.
const FEATURE_TOLERANCE: u32 = 0x010d_51c4;
/// The float constants `0.0` (`01011d78`), `3.0` (`01017718`) and `0.5` (`01016248`) the final
/// normalisation reads.
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
/// The case number (the exe's byte table at `00dabc7c`) for `count_a * 8 | count_b` minus 9.
const FEATURE_CASE: [u8; 25] = [
    0, 1, 2, 3, 10, 10, 10, 10, 4, 5, 6, 10, 10, 10, 10, 10, 7, 8, 10, 10, 10, 10, 10, 10, 9,
];

// Program counters of `getClosestFeature` (the exe's addresses).
const PC_TOP: u32 = 0x00da_ae60;
const PC_DISPATCH: u32 = 0x00da_ae9f;
const PC_EDGE_A2_B3: u32 = 0x00da_aeae;
const PC_EDGE_A3_B2: u32 = 0x00da_aecd;
const PC_SEGMENTS: u32 = 0x00da_af03;
const PC_B4: u32 = 0x00da_afb2;
const PC_B3: u32 = 0x00da_aff5;
const PC_A4: u32 = 0x00da_b125;
const PC_A3: u32 = 0x00da_b158;
const PC_A2: u32 = 0x00da_b677;
const PC_B2: u32 = 0x00da_b54a;
const PC_POINTS: u32 = 0x00da_b75d;
const PC_STORE_KIND: u32 = 0x00da_b771;
const PC_POST: u32 = 0x00da_b260;
const PC_CONVERGED: u32 = 0x00da_bb07;
const PC_FINISH: u32 = 0x00da_bb28;

/// `value * |value|` the way the x87 code forms it (the product in extended precision, stored
/// as a float).
fn signed_square(value: f32) -> f32 {
    let value = x87_quiet(value);
    (f64::from(value) * f64::from(value.abs())) as f32
}

/// A point transformed by four row vectors: `(y*r1 + x*r0) + (z*r2 + r3)`.
fn transform_point(rows: &[Vec4; 4], v: Vec4) -> Vec4 {
    let xy = vadd(vmul([v[1]; 4], rows[1]), vmul([v[0]; 4], rows[0]));
    let zw = vadd(vmul([v[2]; 4], rows[2]), rows[3]);
    vadd(xy, zw)
}

fn negative(value: f32) -> bool {
    value.to_bits() >> 31 != 0
}

/// Asks shape A (or B) for a support point opposite to the contact normal and adds it to the
/// first (second) simplex list when it is far enough out. Shared by the two exe copies of the
/// code for shape A. `dir_at` is where the search direction (the negated normal) is stored.
fn extend_first_list(e: &mut Engine, t: u32, shape: u32, dir_at: u32, threshold: f32) -> bool {
    let normal = ld(e, t + 0x120);
    st(e, dir_at, xorps(normal, mask_at(e, SIGN_MASK)));
    let count = e.mem.u32(t);
    e.vcall(shape, 0x38, &args![dir_at, t + 0x20 + 16 * count]);
    let count = e.mem.u32(t);
    let dir = ld(e, dir_at);
    let new_vertex = ld(e, t + 0x20 + 16 * count);
    let reach = |e: &Engine, k: u32| {
        signed_square(sum3(vmul(vsub(new_vertex, ld(e, t + 0x20 + 16 * k)), dir)))
    };
    let far_enough = threshold < reach(e, 0);
    if !far_enough {
        return false;
    }
    match count {
        2 => {
            if reach(e, 1) < threshold {
                return false;
            }
        }
        3 => {
            if reach(e, 2) < threshold || reach(e, 1) < threshold {
                return false;
            }
        }
        _ => {}
    }
    e.mem.set_u32(t, count.wrapping_add(1));
    e.mem.set_u32(t + 0x14, 1);
    true
}

/// The same for shape B: the search direction is the normal rotated back through the
/// transform's rows, the support point is found in B's own space and then transformed.
fn extend_second_list(
    e: &mut Engine,
    t: u32,
    shape: u32,
    transform: u32,
    dir_at: u32,
    threshold: f32,
) -> bool {
    let r0 = ld(e, transform);
    let r1 = ld(e, transform + 0x10);
    let r2 = ld(e, transform + 0x20);
    let normal = ld(e, t + 0x120);
    let low = unpcklps(r0, r1);
    let high = unpckhps(r0, r1);
    let col_x = movlhps(low, r2);
    let col_y = shufps(movhlps(col_x, low), r2, 0xd4);
    let col_z = shufps(high, r2, 0xe4);
    let dir = vadd(
        vadd(vmul([normal[1]; 4], col_y), vmul([normal[0]; 4], col_x)),
        vmul(col_z, [normal[2]; 4]),
    );
    let count = e.mem.u32(t + 4);
    st(e, dir_at, dir);
    e.vcall(shape, 0x38, &args![dir_at, t + 0xe0 + 16 * count]);
    let count = e.mem.u32(t + 4);
    let dir = ld(e, dir_at);
    let new_vertex = ld(e, t + 0xe0 + 16 * count);
    let reach = |e: &Engine, k: u32| {
        signed_square(sum3(vmul(vsub(new_vertex, ld(e, t + 0xe0 + 16 * k)), dir)))
    };
    let far_enough = threshold < reach(e, 0);
    if far_enough {
        match count {
            2 => {
                if reach(e, 1) < threshold {
                    return false;
                }
            }
            3 => {
                if reach(e, 2) < threshold || reach(e, 1) < threshold {
                    return false;
                }
            }
            _ => {}
        }
        let rows = [0, 0x10, 0x20, 0x30].map(|o| ld(e, transform + o));
        let moved = transform_point(&rows, ld(e, t + 0xe0 + 16 * count));
        st(e, t + 0xa0 + 16 * count, moved);
        let n = e.mem.u32(t + 4).wrapping_add(1);
        e.mem.set_u32(t + 4, n);
        e.mem.set_u32(t + 0x14, 1);
        return true;
    }
    false
}

// Translated from 00daad40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpGsk::getClosestFeature` (Xbox PDB), thiscall `(this, shape_a, shape_b, transform,
/// result)`; the callee pops four words after `this`. The GJK main loop. `this + 0x20` holds
/// shape A's simplex vertices (count at `this`), `this + 0xa0` the shape B vertices already in
/// A's space (count at `this + 4`), `this + 0xe0` the same vertices in B's own space, and
/// `this + 0x120` is the contact normal. The B vertices are first rebuilt from the `transform`
/// rows (a 4x4 of 16-byte rows). Each pass doubles the tolerance and dispatches on the two
/// counts: the sizes reduce the simplex (nearest-feature tests, `findClosestTriangle`,
/// `processEdgeTriangle`, `checkTriangleBoundaries`, segment-to-segment tests), set the
/// normal and kind (`this + 0x170`) and either end the pass as converged or ask the shapes'
/// support functions (virtual call at slot `0x38`, arguments direction and output vertex) for a
/// new vertex. A converged simplex with `this + 0x10` clear is handed to `handlePenetration`;
/// with it set the function returns 4 after trimming the counts. Otherwise the normal is
/// normalised (refined `RSQRTSS`, zero for a zero normal), copied to `result` with the
/// distance in its fourth lane (also stored at `this + 0x12c`) and 0 is returned.
pub fn hkp_gsk_get_closest_feature(
    e: &mut Engine,
    this: Ptr,
    shape_a: Ptr,
    shape_b: Ptr,
    transform: Ptr,
    result: Ptr,
) -> u32 {
    let t = this.addr();
    let ta = t + 0x20;
    let tb = t + 0xa0;
    let rows = [0u32, 0x10, 0x20, 0x30].map(|o| ld(e, transform.addr() + o));
    let mut k = e.mem.u32(t + 4).wrapping_sub(1);
    loop {
        let v = ld(e, t.wrapping_add(0xe0).wrapping_add(k.wrapping_mul(16)));
        st(
            e,
            t.wrapping_add(0xa0).wrapping_add(k.wrapping_mul(16)),
            transform_point(&rows, v),
        );
        k = k.wrapping_sub(1);
        if (k as i32) < 0 {
            break;
        }
    }
    let a = move |k: u32| ta.wrapping_add(k.wrapping_mul(16));
    let b = move |k: u32| tb.wrapping_add(k.wrapping_mul(16));
    let bp = move |k: u32| (tb + 0x40).wrapping_add(k.wrapping_mul(16));
    let mut tolerance = load_float(e, FEATURE_TOLERANCE);
    e.with_stack(0x100, |e, frame| {
        let f = frame.addr();
        let mut iteration = 0u32;
        #[allow(unused_assignments)]
        let mut previous_sum = e.mem.u32(t).wrapping_add(e.mem.u32(t + 4));
        let mut converged = false;
        let mut pc = PC_TOP;
        loop {
            match pc {
                PC_TOP => {
                    tolerance = x87_quiet(tolerance) * 2.0;
                    iteration += 1;
                    previous_sum = e.mem.u32(t).wrapping_add(e.mem.u32(t + 4));
                    e.mem.set_u32(t + 0x174, 0);
                    pc = PC_DISPATCH;
                }
                PC_DISPATCH => {
                    let key = (e.mem.u32(t).wrapping_mul(8) | e.mem.u32(t + 4)).wrapping_sub(9);
                    pc = if key > 0x18 {
                        converged = true;
                        PC_POST
                    } else {
                        match FEATURE_CASE[key as usize] {
                            0 => PC_POINTS,
                            1 => PC_B2,
                            2 => PC_B3,
                            3 => PC_B4,
                            4 => PC_A2,
                            5 => PC_SEGMENTS,
                            6 => PC_EDGE_A2_B3,
                            7 => PC_A3,
                            8 => PC_EDGE_A3_B2,
                            9 => PC_A4,
                            _ => {
                                converged = true;
                                PC_POST
                            }
                        }
                    };
                }
                PC_EDGE_A2_B3 | PC_EDGE_A3_B2 => {
                    let flag = u8::from(e.mem.u32(t + 0x170) == 2);
                    let r = if pc == PC_EDGE_A2_B3 {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(a(0)),
                            Ptr::new(b(0)),
                            Ptr::new(t),
                            Ptr::new(t + 4),
                            flag,
                            1,
                        )
                    } else {
                        hkp_gsk_process_edge_triangle(
                            e,
                            this,
                            Ptr::new(b(0)),
                            Ptr::new(a(0)),
                            Ptr::new(t + 4),
                            Ptr::new(t),
                            flag,
                            u32::MAX,
                        )
                    };
                    pc = match r {
                        1 => {
                            converged = true;
                            PC_POST
                        }
                        2 => PC_SEGMENTS,
                        _ => PC_STORE_KIND,
                    };
                }
                PC_SEGMENTS => {
                    let a0 = ld(e, a(0));
                    let a1 = ld(e, a(1));
                    st(e, f + 0xa0, vsub(a1, a0));
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    st(e, f + 0xc0, vsub(b1, b0));
                    let mask = e
                        .call(
                            CLOSEST_LINE_SEG_LINE_SEG,
                            &args![a(0), f + 0xa0, b(0), f + 0xc0, t + 0x130],
                        )
                        .u32();
                    if mask == 0 {
                        // The segments are parallel to nothing nearer: the plane normal.
                        let dir_b = ld(e, f + 0xc0);
                        let dir_a = ld(e, f + 0xa0);
                        let normal = vsub(
                            vmul(shufps(dir_b, dir_b, 0xd2), shufps(dir_a, dir_a, 0xc9)),
                            vmul(shufps(dir_b, dir_b, 0xc9), shufps(dir_a, dir_a, 0xd2)),
                        );
                        st(e, t + 0x120, normal);
                        let side = sum3(vmul(vsub(ld(e, a(0)), ld(e, b(0))), normal));
                        let sign = mask_at(e, SIGN_MASK);
                        let keep = [0, 1, 2, 3].map(|i| side.to_bits() & sign[i]);
                        st(e, t + 0x120, xorps(normal, keep));
                        pc = PC_STORE_KIND;
                    } else {
                        if mask & 1 != 0 {
                            copy16(e, a(1), a(0));
                        }
                        if mask & 3 != 0 {
                            e.mem.set_u32(t, 1);
                        }
                        if mask & 4 != 0 {
                            copy16(e, b(1), b(0));
                            copy16(e, bp(1), bp(0));
                            e.mem.set_u32(t + 4, 1);
                        } else if mask & 8 != 0 {
                            e.mem.set_u32(t + 4, 1);
                        }
                        pc = PC_DISPATCH;
                    }
                }
                PC_B4 => {
                    let r = find_closest_triangle(e, Ptr::new(b(0)), Ptr::new(a(0)), 1.0);
                    if r < 0 {
                        converged = true;
                        pc = PC_POST;
                    } else {
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(3), b(r as u32));
                        copy16(e, bp(3), bp(r as u32));
                        pc = PC_B3;
                    }
                }
                PC_B3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(a(0)),
                        Ptr::new(b(0)),
                        1,
                    );
                    if r == 7 {
                        pc = PC_STORE_KIND;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        copy16(e, b(2), b(index));
                        copy16(e, bp(2), bp(index));
                        pc = PC_B2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, a(0));
                    let base = ld(e, b(m as u32));
                    let p_second = ld(e, b(second));
                    let p_first = ld(e, b(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let b2 = ld(e, b(2));
                    e.mem.set_u32(t + 4, 2);
                    if negative(side_first) {
                        st(e, b(first), b2);
                        copy16(e, bp(2), bp(first));
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t + 4);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t + 4, n);
                            copy16(e, b(n), b(keep));
                            copy16(e, bp(n), bp(keep));
                        }
                    } else {
                        st(e, b(second), b2);
                        copy16(e, bp(2), bp(second));
                    }
                    pc = PC_DISPATCH;
                }
                PC_A4 => {
                    let scale = load_float(e, MINUS_ONE);
                    let r = find_closest_triangle(e, Ptr::new(a(0)), Ptr::new(b(0)), scale);
                    if r < 0 {
                        converged = true;
                        pc = PC_POST;
                    } else {
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(3), a(r as u32));
                        pc = PC_A3;
                    }
                }
                PC_A3 => {
                    let r = hkp_gsk_check_triangle_boundaries(
                        e,
                        this,
                        Ptr::new(b(0)),
                        Ptr::new(a(0)),
                        -1,
                    );
                    if r == 7 {
                        pc = PC_STORE_KIND;
                        continue;
                    }
                    let m = signed_byte(e, MASK_TABLE + r);
                    if m < 0 {
                        let mut index = (m + 8) as u32;
                        if index as i32 > 3 {
                            index = 1;
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        copy16(e, a(2), a(index));
                        pc = PC_A2;
                        continue;
                    }
                    let first = signed_byte(e, EDGE_TABLE_FIRST + m as u32) as u32;
                    let second = signed_byte(e, EDGE_TABLE_SECOND + m as u32) as u32;
                    let query = ld(e, b(0));
                    let base = ld(e, a(m as u32));
                    let p_second = ld(e, a(second));
                    let p_first = ld(e, a(first));
                    let to_query = vsub(query, base);
                    let side_first = sum3(vmul(vsub(p_first, base), to_query));
                    let a2 = ld(e, a(2));
                    e.mem.set_u32(t, 2);
                    if negative(side_first) {
                        st(e, a(first), a2);
                        let side_second = sum3(vmul(vsub(p_second, base), to_query));
                        if negative(side_second) {
                            let mut keep = second;
                            let mut n = e.mem.u32(t);
                            if keep == n {
                                keep = first;
                            }
                            n = n.wrapping_sub(1);
                            e.mem.set_u32(t, n);
                            copy16(e, a(n), a(keep));
                        }
                    } else {
                        st(e, a(second), a2);
                    }
                    pc = PC_DISPATCH;
                }
                PC_B2 => {
                    let b0 = ld(e, b(0));
                    let b1 = ld(e, b(1));
                    let a0 = ld(e, a(0));
                    let edge = vsub(b1, b0);
                    let from_b0 = vsub(b0, a0);
                    let from_b1 = vsub(b1, a0);
                    let negative_b1 = negative(sum3(vmul(from_b1, edge)));
                    let negative_b0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_b1 != negative_b0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let direction = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(
                                shufps(from_b1, from_b1, 0xc9),
                                shufps(from_b0, from_b0, 0xd2),
                            ),
                            vmul(
                                shufps(from_b1, from_b1, 0xd2),
                                shufps(from_b0, from_b0, 0xc9),
                            ),
                        );
                        let normal = vsub(
                            vmul(
                                shufps(direction, direction, 0xc9),
                                shufps(cross, cross, 0xd2),
                            ),
                            vmul(
                                shufps(direction, direction, 0xd2),
                                shufps(cross, cross, 0xc9),
                            ),
                        );
                        st(e, t + 0x120, normal);
                        pc = PC_STORE_KIND;
                    } else {
                        if negative_b1 {
                            st(e, b(0), b1);
                            copy16(e, bp(1), bp(0));
                        }
                        let n = e.mem.u32(t + 4).wrapping_sub(1);
                        e.mem.set_u32(t + 4, n);
                        pc = PC_POINTS;
                    }
                }
                PC_A2 => {
                    let a1 = ld(e, a(1));
                    let a0 = ld(e, a(0));
                    let b0 = ld(e, b(0));
                    let edge = vsub(a1, a0);
                    let from_b0 = vsub(a0, b0);
                    let to_a1 = vsub(a1, b0);
                    let negative_a1 = negative(sum3(vmul(to_a1, edge)));
                    let negative_a0 = negative(sum3(vmul(from_b0, edge)));
                    if negative_a1 != negative_a0 {
                        let length_squared = sum3(vmul(edge, edge));
                        let direction = vmul([rsqrt_estimate(length_squared); 4], edge);
                        let cross = vsub(
                            vmul(shufps(from_b0, from_b0, 0xd2), shufps(to_a1, to_a1, 0xc9)),
                            vmul(shufps(from_b0, from_b0, 0xc9), shufps(to_a1, to_a1, 0xd2)),
                        );
                        let normal = vsub(
                            vmul(
                                shufps(cross, cross, 0xc9),
                                shufps(direction, direction, 0xd2),
                            ),
                            vmul(
                                shufps(cross, cross, 0xd2),
                                shufps(direction, direction, 0xc9),
                            ),
                        );
                        st(e, t + 0x120, normal);
                        pc = PC_STORE_KIND;
                    } else {
                        if negative_a1 {
                            let parallel = ld(e, a(5));
                            st(e, a(0), a1);
                            st(e, a(4), parallel);
                        }
                        let n = e.mem.u32(t).wrapping_sub(1);
                        e.mem.set_u32(t, n);
                        pc = PC_POINTS;
                    }
                }
                PC_POINTS => {
                    let normal = vsub(ld(e, a(0)), ld(e, b(0)));
                    st(e, t + 0x120, normal);
                    pc = PC_STORE_KIND;
                }
                PC_STORE_KIND => {
                    let n = e.mem.u32(t + 4);
                    e.mem.set_u32(t + 0x170, n);
                    converged = false;
                    pc = PC_POST;
                }
                PC_POST => {
                    let normal = ld(e, t + 0x120);
                    let normal_squared = sum3(vmul(normal, normal));
                    let na = e.mem.u32(t);
                    let nb = e.mem.u32(t + 4);
                    let grown = na.wrapping_sub(previous_sum).wrapping_add(nb);
                    let flags = e.mem.u32(t + 0x14) | grown;
                    e.mem.set_u32(t + 0x14, flags);
                    let along = sum3(vmul(vsub(ld(e, a(0)), ld(e, b(0))), normal));
                    if converged {
                        pc = PC_CONVERGED;
                        continue;
                    }
                    let nn = f64::from(x87_quiet(normal_squared));
                    let dd = x87_quiet(along);
                    let scaled = nn * f64::from(tolerance);
                    if f64::from(dd) * f64::from(dd.abs()) <= scaled || f64::from(tolerance) > nn {
                        pc = PC_CONVERGED;
                        continue;
                    }
                    let threshold = scaled as f32;
                    let odd = iteration & 1 != 0;
                    if odd
                        && na != e.mem.u32(t + 8)
                        && e.mem.u32(t + 0x174) != 1
                        && extend_first_list(e, t, shape_a.addr(), f + 0x80, threshold)
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    let nb = e.mem.u32(t + 4);
                    if nb != e.mem.u32(t + 0xc)
                        && e.mem.u32(t + 0x174) as i32 != -1
                        && extend_second_list(
                            e,
                            t,
                            shape_b.addr(),
                            transform.addr(),
                            f + 0x60,
                            threshold,
                        )
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    let na = e.mem.u32(t);
                    if !odd
                        && na != e.mem.u32(t + 8)
                        && e.mem.u32(t + 0x174) != 1
                        && extend_first_list(e, t, shape_a.addr(), f + 0xe0, threshold)
                    {
                        pc = PC_TOP;
                        continue;
                    }
                    pc = PC_FINISH;
                }
                PC_CONVERGED => {
                    if e.mem.u32(t + 0x10) == 0 {
                        hkp_gsk_handle_penetration(e, this, shape_a, shape_b, transform);
                        pc = PC_FINISH;
                    } else {
                        let nb = e.mem.u32(t + 4);
                        let na = e.mem.u32(t);
                        if (na.wrapping_add(nb) as i32) > 4 {
                            if (na as i32) > (nb as i32) {
                                e.mem.set_u32(t, 3);
                                e.mem.set_u32(t + 4, 1);
                            } else {
                                e.mem.set_u32(t, 1);
                                e.mem.set_u32(t + 4, 3);
                            }
                        }
                        return 4;
                    }
                }
                _ => {
                    // PC_FINISH
                    let normal = ld(e, t + 0x120);
                    let difference = vsub(ld(e, a(0)), ld(e, b(0)));
                    let zero = e.mem.f32(ZERO);
                    let three = e.mem.f32(THREE);
                    let half = e.mem.f32(HALF);
                    let length_squared = sum3(vmul(normal, normal));
                    let estimate = rsqrt_estimate(length_squared);
                    let step = three - (length_squared * estimate) * estimate;
                    let refined = (half * estimate) * step;
                    let scale = if length_squared == zero { 0.0 } else { refined };
                    st(e, t + 0x120, vmul([scale; 4], normal));
                    copy16(e, t + 0x120, result.addr());
                    let distance = x87_quiet(sum3(vmul(ld(e, t + 0x120), difference)));
                    e.mem.set_f32(result.addr() + 0xc, distance);
                    e.mem.set_f32(t + 0x12c, distance);
                    return 0;
                }
            }
        }
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00da_8390, find_closest_triangle_backup(Ptr, Ptr, Ptr) -> i32),
        entry!(
            0x00da_8500,
            hkp_gsk_check_triangle_boundaries(Ptr, Ptr, Ptr, i32) -> u32
        ),
        entry!(0x00da_8fb0, find_closest_triangle(Ptr, Ptr, f32) -> i32),
        entry!(
            0x00da_9340,
            hkp_gsk_process_edge_triangle(Ptr, Ptr, Ptr, Ptr, Ptr, u8, u32) -> u32
        ),
        entry!(0x00da_9790, hkp_gsk_exit_and_export_cache_impl(Ptr, Ptr)),
        entry!(
            0x00da_9880,
            hkp_gsk_convert_feature_to_closest_distance(Ptr, Ptr, Ptr)
        ),
        entry!(0x00da_98f0, hkp_gsk_reduce_dimension_extended(Ptr)),
        entry!(0x00da_a7e0, hkp_gsk_handle_penetration(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00da_ad40,
            hkp_gsk_get_closest_feature(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;
    type SegmentLog = Rc<RefCell<Vec<(Vec<u32>, Vec4, Vec4)>>>;

    fn constants(e: &mut Engine) {
        e.mem.write(
            MASK_TABLE,
            &[0xff, 0, 1, 0xfa, 2, 0xf9, 0xf8, 0xff, 2, 0, 1, 2, 0],
        );
        for i in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * i, 0x8000_0000);
            e.mem.set_u32(ABS_MASK + 4 * i, 0x7fff_ffff);
            e.mem.set_f32(ONES + 4 * i, 1.0);
        }
        e.mem.set_f32(MINUS_ONE, -1.0);
        e.mem.set_f32(BIG_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(THREE, 3.0);
        e.mem.set_f32(HALF, 0.5);
    }

    fn triangle(e: &mut Engine, at: u32, p0: Vec4, p1: Vec4, p2: Vec4) {
        st(e, at, p0);
        st(e, at + 0x10, p1);
        st(e, at + 0x20, p2);
    }

    const BASE: Vec4 = [0.0, 0.0, 0.0, 0.0];
    const RIGHT: Vec4 = [1.0, 0.0, 0.0, 0.0];
    const UP: Vec4 = [0.0, 1.0, 0.0, 0.0];

    #[test]
    fn the_backup_search_clears_weights_by_edge_sign_and_returns_the_first_positive() {
        let mut e = test_engine();
        constants(&mut e);
        let v = e.mem.alloc(0x40);
        triangle(&mut e, v, BASE, RIGHT, UP);
        st(&mut e, v + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(0x10);
        st(&mut e, point, [0.25, 0.25, -1.0, 0.0]);
        let w = e.mem.alloc(0x10);
        for i in 0..3 {
            e.mem.set_f32(w + 4 * i, 1.0);
        }
        assert_eq!(e.call(0x00da_8390, &args![v, point, w]).i32(), 0);
        let weights = [0, 1, 2].map(|i| e.mem.f32(w + 4 * i));
        assert_eq!(weights, [1.0, -1.0, -1.0]);
        for i in 0..3 {
            e.mem.set_f32(w + 4 * i, -1.0);
        }
        assert_eq!(e.call(0x00da_8390, &args![v, point, w]).i32(), -1);
    }

    #[test]
    fn the_boundary_check_masks_negative_sums_and_flips_a_backwards_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let tri = e.mem.alloc(0x80);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        let q = e.mem.alloc(0x10);
        // Outside: the first and third sums are negative.
        st(&mut e, q, [-1.0, 0.5, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_8500, &args![this, q, tri, 0u32]).u32(), 5);
        // Inside with the flag: all three negative, and the shapes' points lie on the wrong
        // side of the normal, so the triangle is turned around.
        st(&mut e, q, [0.25, 0.25, 0.0, 0.0]);
        st(&mut e, this + 0xa0, [0.0, 0.0, 1.0, 0.0]);
        st(&mut e, tri + 0x40, [7.0, 0.0, 0.0, 0.0]);
        st(&mut e, tri + 0x50, [8.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00da_8500, &args![this, q, tri, 1u32]).u32(), 7);
        assert_eq!(ld(&e, this + 0x120), [-0.0, -0.0, -1.0, -0.0]);
        assert_eq!(ld(&e, tri), RIGHT);
        assert_eq!(ld(&e, tri + 0x10), BASE);
        assert_eq!(ld(&e, tri + 0x40)[0], 8.0);
        assert_eq!(ld(&e, tri + 0x50)[0], 7.0);
        assert_eq!(e.mem.f32(this + 0x160), -0.25);
        assert_eq!(e.mem.f32(this + 0x164), -0.5);
        assert_eq!(e.mem.u32(this + 0x14), 1);
    }

    #[test]
    fn the_triangle_search_picks_the_dominant_face() {
        let mut e = test_engine();
        constants(&mut e);
        e.mem.set_f32(NEAR_ZERO, 1e-6);
        e.mem.set_f64(TIE_FACTOR, 1.1);
        let tri = e.mem.alloc(0x40);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, tri + 0x30, [0.0, 0.0, 1.0, 0.0]);
        let point = e.mem.alloc(0x10);
        let one = 1.0f32.to_bits();
        // On the apex: nothing is closer than the threshold.
        st(&mut e, point, [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), -1);
        // Straight above it: the first face wins clearly.
        st(&mut e, point, [0.0, 0.0, 2.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), 0);
        // Towards -x: the second face.
        st(&mut e, point, [-1.0, 0.0, 1.0, 0.0]);
        assert_eq!(e.call(0x00da_8fb0, &args![tri, point, one]).i32(), 1);
    }

    #[test]
    fn an_edge_pair_that_crosses_the_triangle_plane_inside_it_is_reported_inside() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let s = e.mem.alloc(0x80);
        let tri = e.mem.alloc(0x80);
        let ca = e.mem.alloc(8);
        let cb = ca + 4;
        e.mem.set_u32(ca, 3);
        e.mem.set_u32(cb, 3);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, s, [0.25, 0.25, 1.0, 0.0]);
        st(&mut e, s + 0x10, [0.25, 0.25, -1.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 1u32, 0u32]);
        assert_eq!(got.u32(), 1);
        assert_eq!(e.mem.u32(ca), 3);
        // Both points on one side and the first inside: the first counter drops.
        st(&mut e, s + 0x10, [0.25, 0.25, 2.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 1u32, 0u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(e.mem.u32(ca), 2);
        assert_eq!(e.mem.u32(cb), 3);
    }

    #[test]
    fn the_stored_normal_mode_drops_the_first_point_when_the_second_is_inside() {
        let mut e = test_engine();
        constants(&mut e);
        let this = e.mem.alloc(0x200);
        let s = e.mem.alloc(0x80);
        let tri = e.mem.alloc(0x80);
        let ca = e.mem.alloc(8);
        let cb = ca + 4;
        e.mem.set_u32(ca, 2);
        e.mem.set_u32(cb, 3);
        triangle(&mut e, tri, BASE, RIGHT, UP);
        st(&mut e, this + 0x120, [0.0, 0.0, 1.0, 0.0]);
        st(&mut e, s, [0.25, 0.25, 1.0, 0.0]);
        st(&mut e, s + 0x10, [0.25, 0.25, 2.0, 0.0]);
        st(&mut e, s + 0x50, [5.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_9340, &args![this, s, tri, ca, cb, 0u32, 9u32]);
        assert_eq!(got.u32(), 0);
        assert_eq!(e.mem.u32(ca), 1);
        assert_eq!(ld(&e, s), [0.25, 0.25, 2.0, 0.0]);
        assert_eq!(ld(&e, s + 0x40), [5.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x174), 9);
    }

    #[test]
    fn the_cache_export_writes_counts_and_sorted_ids() {
        let mut e = test_engine();
        let this = e.mem.alloc(0x200);
        let cache = e.mem.alloc(0x20);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u16(this + 0x2c, 5);
        e.mem.set_u16(this + 0x3c, 9);
        e.mem.set_u16(this + 0xec, 7);
        e.call(0x00da_9790, &args![this, cache]);
        assert_eq!(e.mem.u8(cache + 8), 2);
        assert_eq!(e.mem.u8(cache + 9), 1);
        assert_eq!(e.mem.u16(cache), 9);
        assert_eq!(e.mem.u16(cache + 2), 5);
        assert_eq!(e.mem.u16(cache + 4), 7);
    }

    #[test]
    fn the_feature_conversion_picks_the_point_by_which_list_has_one_vertex() {
        let mut e = test_engine();
        let this = e.mem.alloc(0x200);
        let input = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x30);
        st(&mut e, input, [1.0, 2.0, 3.0, 2.0]);
        st(&mut e, this + 0x20, [4.0, 4.0, 4.0, 4.0]);
        st(&mut e, this + 0xa0, [10.0, 10.0, 10.0, 10.0]);
        st(&mut e, this + 0x130, [6.0, 6.0, 6.0, 6.0]);
        e.mem.set_u32(this, 1);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out), [1.0, 2.0, 3.0, 2.0]);
        assert_eq!(e.mem.f32(out + 0x20), 2.0);
        assert_eq!(ld(&e, out + 0x10), [4.0; 4]);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 1);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out + 0x10), [12.0, 14.0, 16.0, 14.0]);
        e.mem.set_u32(this + 4, 2);
        e.call(0x00da_9880, &args![this, input, out]);
        assert_eq!(ld(&e, out + 0x10), [6.0; 4]);
    }

    fn segment_double(e: &mut Engine, distances: Vec<f32>) -> SegmentLog {
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        let next = Cell::new(0usize);
        e.register_double(CLOSEST_LINE_SEG_LINE_SEG, move |e, a| {
            sink.borrow_mut()
                .push((a.to_vec(), ld(e, a[1]), ld(e, a[3])));
            let distance = distances[next.get() % distances.len()];
            next.set(next.get() + 1);
            e.mem.set_f32(a[4] + 0x20, distance);
            Ret::default()
        });
        log
    }

    #[test]
    fn the_reduction_shrinks_a_list_whose_point_lies_inside_the_other_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        let log = segment_double(&mut e, vec![1.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 3);
        st(&mut e, this + 0x20, [0.25, 0.25, 0.0, 0.0]);
        triangle(&mut e, this + 0xa0, BASE, RIGHT, UP);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 3);
        assert!(log.borrow().is_empty());
        // The second list is the single point, the first the triangle: the point inside it
        // is found at index 1 of the second list, which moves to the front.
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 3);
        e.mem.set_u32(this + 4, 2);
        triangle(&mut e, this + 0x20, BASE, RIGHT, UP);
        st(&mut e, this + 0xa0, [5.0, 5.0, 0.0, 0.0]);
        st(&mut e, this + 0xb0, [0.25, 0.25, 0.0, 0.0]);
        st(&mut e, this + 0xe0, [100.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0xf0, [101.0, 0.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(ld(&e, this + 0xa0), [0.25, 0.25, 0.0, 0.0]);
        assert_eq!(ld(&e, this + 0xe0), [101.0, 0.0, 0.0, 0.0]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_reduction_drops_the_vertex_at_the_start_of_the_closest_edge_pair() {
        let mut e = test_engine();
        constants(&mut e);
        // Two single points: one pair, and the edge vectors are the second vertices minus the
        // first.
        let log = segment_double(&mut e, vec![5.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 2);
        e.mem.set_u32(this + 4, 2);
        st(&mut e, this + 0x20, [1.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0x30, [3.0, 0.0, 0.0, 0.0]);
        st(&mut e, this + 0xa0, [0.0, 1.0, 0.0, 0.0]);
        st(&mut e, this + 0xb0, [0.0, 4.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        {
            let calls = log.borrow();
            assert_eq!(calls.len(), 1);
            assert_eq!(calls[0].0[0], this + 0x20);
            assert_eq!(calls[0].0[2], this + 0xa0);
            assert_eq!(calls[0].1, [2.0, 0.0, 0.0, 0.0]);
            assert_eq!(calls[0].2, [0.0, 3.0, 0.0, 0.0]);
        }
        assert_eq!(e.mem.u32(this), 2);
    }

    #[test]
    fn the_reduction_replaces_the_start_vertex_of_the_closest_edge_of_a_triangle() {
        let mut e = test_engine();
        constants(&mut e);
        // A triangle and a lone point away from it: the closest pair is the second one (index
        // 1), so the vertex the edge table gives for it (vertex 0) is replaced by vertex 2.
        let log = segment_double(&mut e, vec![3.0, 1.0, 2.0]);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 3);
        e.mem.set_u32(this + 4, 1);
        triangle(&mut e, this + 0x20, BASE, RIGHT, UP);
        st(&mut e, this + 0xa0, [5.0, 5.0, 0.0, 0.0]);
        e.call(0x00da_98f0, &args![this]);
        assert_eq!(log.borrow().len(), 3);
        assert_eq!(e.mem.u32(this), 2);
        assert_eq!(ld(&e, this + 0x20), UP);
        assert_eq!(e.mem.u32(this + 4), 1);
    }

    fn identity_rows(e: &mut Engine, at: u32, translation: Vec4) {
        st(e, at, [1.0, 0.0, 0.0, 0.0]);
        st(e, at + 0x10, [0.0, 1.0, 0.0, 0.0]);
        st(e, at + 0x20, [0.0, 0.0, 1.0, 0.0]);
        st(e, at + 0x30, translation);
    }

    #[test]
    fn the_penetration_handler_stores_a_direct_hit_and_keeps_going_after_a_miss() {
        let mut e = test_engine();
        constants(&mut e);
        e.mem.set_f32(PENETRATION_TOLERANCE, 1e-4);
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&calls);
        let hit = Rc::new(Cell::new(true));
        let hit_flag = Rc::clone(&hit);
        let diff = Rc::new(RefCell::new(BASE));
        let diff_sink = Rc::clone(&diff);
        e.register_double(CALCULATE_PENETRATION_DEPTH, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            *diff_sink.borrow_mut() = ld(e, a[6]);
            st(e, a[8], [0.0, 1.0, 0.0, 0.0]);
            Ret {
                eax: u32::from(hit_flag.get()),
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x200);
        let m = e.mem.alloc(0x40);
        identity_rows(&mut e, m, [10.0, 20.0, 30.0, 0.0]);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        st(&mut e, this + 0x20, [3.0, 3.0, 3.0, 0.0]);
        st(&mut e, this + 0xa0, [1.0, 1.0, 1.0, 0.0]);
        e.call(0x00da_a7e0, &args![this, 0x111u32, 0x222u32, m]);
        {
            let log = calls.borrow();
            assert_eq!(log.len(), 1);
            assert_eq!(log[0][0..3], [0x111, 0x222, m]);
            assert_eq!(log[0][3], 1e-4f32.to_bits());
            assert_eq!(log[0][4..6], [this + 0x20, this + 0xe0]);
            assert_eq!(log[0][7], 1);
        }
        assert_eq!(*diff.borrow(), [2.0, 2.0, 2.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(ld(&e, this + 0x120), [0.0, 1.0, 0.0, 0.0]);
        // A miss: the vertex ids are all equal, so both lists shrink to one vertex and the
        // second is moved by the transform.
        hit.set(false);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 0x2c, 5);
        e.mem.set_u32(this + 0xec, 7);
        st(&mut e, this + 0xe0, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(this + 0xec, 7);
        e.call(0x00da_a7e0, &args![this, 0x111u32, 0x222u32, m]);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 1);
        assert_eq!(ld(&e, this + 0xa0), [11.0, 22.0, 33.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x14), 1);
        assert_eq!(ld(&e, this + 0x120), [0.0, 1.0, 0.0, 0.0]);
    }

    fn shapes(e: &mut Engine, support_a: Vec4, support_b: Vec4) -> (u32, u32) {
        let mut make = |address: u32, vertex: Vec4| {
            let table = e.mem.alloc(0x40);
            e.mem.set_u32(table + 0x38, address);
            let shape = e.mem.alloc(0x10);
            e.mem.set_u32(shape, table);
            e.register_double(address, move |e, a| {
                st(e, a[2], vertex);
                Ret::default()
            });
            shape
        };
        let a = make(0x00ee_0001, support_a);
        let b = make(0x00ee_0002, support_b);
        (a, b)
    }

    fn feature_setup(e: &mut Engine) -> (u32, u32) {
        constants(e);
        e.mem.set_f32(FEATURE_TOLERANCE, 1e-6);
        let this = e.mem.alloc(0x200);
        e.mem.set_u32(this + 8, 4);
        e.mem.set_u32(this + 0xc, 4);
        let m = e.mem.alloc(0x40);
        identity_rows(e, m, BASE);
        (this, m)
    }

    #[test]
    fn a_zero_gap_converges_and_returns_four() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        let (shape_a, shape_b) = shapes(&mut e, BASE, BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        e.mem.set_u32(this + 0x10, 1);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 4);
        assert_eq!(e.mem.u32(this), 1);
        assert_eq!(e.mem.u32(this + 4), 1);
    }

    #[test]
    fn two_single_points_whose_supports_add_nothing_end_with_the_normalised_gap() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        // The support points the shapes return are the vertices already held.
        let (shape_a, shape_b) = shapes(&mut e, [1.0, 0.0, 0.0, 0.0], BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 1);
        st(&mut e, this + 0x20, [1.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 0);
        let normal = ld(&e, result);
        assert!((normal[0] - 1.0).abs() < 1e-5);
        assert_eq!(normal[1], 0.0);
        let distance = e.mem.f32(result + 0xc);
        assert!((distance - 1.0).abs() < 1e-5);
        assert_eq!(e.mem.f32(this + 0x12c), distance);
        assert_eq!(e.mem.u32(this + 0x170), 1);
    }

    #[test]
    fn a_point_beside_a_segment_gets_the_segment_normal_toward_it() {
        let mut e = test_engine();
        let (this, m) = feature_setup(&mut e);
        let (shape_a, shape_b) = shapes(&mut e, [1.0, 1.0, 0.0, 0.0], BASE);
        let result = e.mem.alloc(0x10);
        e.mem.set_u32(this, 1);
        e.mem.set_u32(this + 4, 2);
        st(&mut e, this + 0x20, [1.0, 1.0, 0.0, 0.0]);
        st(&mut e, this + 0xe0, BASE);
        st(&mut e, this + 0xf0, [2.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00da_ad40, &args![this, shape_a, shape_b, m, result]);
        assert_eq!(got.u32(), 0);
        let normal = ld(&e, result);
        assert!(normal[0].abs() < 1e-3);
        assert!((normal[1] - 1.0).abs() < 1e-3);
        assert!((e.mem.f32(result + 0xc) - 1.0).abs() < 1e-3);
        // The B vertices were rebuilt from the untransformed ones.
        assert_eq!(ld(&e, this + 0xb0), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x170), 2);
    }
}
