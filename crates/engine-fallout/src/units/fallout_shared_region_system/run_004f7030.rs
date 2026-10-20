//! `fallout shared/region system/run_004f7030` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `006815c0` (`thiscall`): a no-op constructor step (answers `this`).
const TRIVIAL_CONSTRUCT: u32 = 0x0068_15c0;
/// `0052ce20` / `004f5d90` (`thiscall`, `RET 4`, a float): set a region
/// point's x / y.
const POINT_SET_X: u32 = 0x0052_ce20;
const POINT_SET_Y: u32 = 0x004f_5d90;
/// `006a7f50` / `006b9130` (`thiscall`): a point's x / y (x87 `ST0`).
const POINT_X: u32 = 0x006a_7f50;
const POINT_Y: u32 = 0x006b_9130;
/// The double constant the segment test compares with (`01012060`, 0.0 in
/// the exe).
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `0096a2d0` (`thiscall`): a `BSSimpleList` node constructor.
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// The float constants `004f7680` copies (`01016970` into +0x10 and +0x14,
/// `01015f5c` into +0x18 and +0x1c).
const BOUND_MIN: u32 = 0x0101_6970;
const BOUND_MAX: u32 = 0x0101_5f5c;
/// `004f7dd0` (`thiscall`): empties the point list; `00401030` frees;
/// `0046ffb0` the `BSSimpleList` destructor.
const POINT_LIST_CLEAR: u32 = 0x004f_7dd0;
const FREE: u32 = 0x0040_1030;
const SIMPLE_LIST_DESTRUCT: u32 = 0x0046_ffb0;

/// The x87 condition bits (`AH` after `FNSTSW AX`: C0 0x01, C2 0x04, C3
/// 0x40) of comparing `st0` with `source`.
fn fcom_status(st0: f64, source: f64) -> u8 {
    if st0.is_nan() || source.is_nan() {
        0x45
    } else if st0 < source {
        0x01
    } else if st0 == source {
        0x40
    } else {
        0
    }
}

/// `TEST AH,mask` then `JP`: the parity flag of the masked bits (set for an
/// even number of ones).
fn parity(status: u8, mask: u8) -> bool {
    (status & mask).count_ones() % 2 == 0
}

// Translated from 004f7030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a region point from another's two floats
/// (`thiscall`, `RET 4`, the source's address): `006815c0`, then x
/// (`0052ce20`) and y (`004f5d90`) set from the source's +0 and +4 (each a
/// `FLD`/`FSTP` copy). Answers `this`.
pub fn fn_004f7030(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    let x = x87_quiet_bits(e.mem.u32(source.addr()));
    e.call(POINT_SET_X, &args![this, x]);
    let y = x87_quiet_bits(e.mem.u32(source.addr().wrapping_add(4)));
    e.call(POINT_SET_Y, &args![this, y]);
    this
}

// Translated from 004f7070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a region point from x and y (`thiscall`,
/// `RET 8`): `006815c0`, then `0052ce20` and `004f5d90` with the two floats
/// (`FLD`/`FSTP` copies). Answers `this`.
pub fn fn_004f7070(e: &mut Engine, this: Ptr, x: f32, y: f32) -> Ptr {
    e.call(TRIVIAL_CONSTRUCT, &args![this]);
    e.call(POINT_SET_X, &args![this, x87_quiet_bits(x.to_bits())]);
    e.call(POINT_SET_Y, &args![this, x87_quiet_bits(y.to_bits())]);
    this
}

// Translated from 004f72a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: two points and a y): the x at which
/// the line through the points reaches `y`, from the second point:
/// `x1 + (x0 - x1) / (y0 - y1) * (y1 - y)` in the exe's x87 order (both
/// point coordinates of the second point stored as floats first). 0.0
/// (`FLDZ`) without either point.
pub fn fn_004f72a0(e: &mut Engine, first: Ptr, second: Ptr, y: f32) -> f32 {
    if first.is_null() || second.is_null() {
        return 0.0;
    }
    let x1 = x87_store32(e.call(POINT_X, &args![second]).f64());
    let y1 = x87_store32(e.call(POINT_Y, &args![second]).f64());
    let drop = x87_sub(x87_load32(y1), x87_m32(y.to_bits()));
    let dx = x87_sub(e.call(POINT_X, &args![first]).f64(), x87_m32(x1));
    let dy = x87_sub(e.call(POINT_Y, &args![first]).f64(), x87_m32(y1));
    let slope = x87_div(dx, dy);
    let offset = x87_mul(slope, drop);
    let x = x87_sub(x87_m32(x1), offset);
    f32::from_bits(x87_store32(x))
}

// Translated from 004f7310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: four points): whether the segment
/// from the first to the second point meets the one from the third to the
/// fourth. False when a point is null. The bounding ranges are tested first
/// in x, then in y (each segment's extent from the sign of its difference,
/// the other's ends compared with the exe's `FCOMPP` tests, NaN passing);
/// then with `denominator = dy1 * dx2 - dx1 * dy2` (the differences stored
/// as floats, `dx1 = x1 - x0`, `dx2 = x2 - x3`, and the same for y) a zero
/// denominator answers true, and otherwise both numerators
/// (`dx1 * ey - dy1 * ex` and `dy2 * ex - dx2 * ey`, `ex = x0 - x2`,
/// `ey = y0 - y2`) must lie between 0 and the denominator on its side.
pub fn fn_004f7310(e: &mut Engine, p0: Ptr, p1: Ptr, p2: Ptr, p3: Ptr) -> bool {
    if p0.is_null() || p1.is_null() || p2.is_null() || p3.is_null() {
        return false;
    }
    let zero = f64::from_bits(e.mem.u64(ZERO_DOUBLE));
    // Ranges per axis: `getter` is the x or y getter.
    let mut differences = [0u32; 4];
    for (axis, getter) in [POINT_X, POINT_Y].into_iter().enumerate() {
        let a1 = e.call(getter, &args![p1]).f64();
        let a0 = e.call(getter, &args![p0]).f64();
        let d1 = x87_store32(x87_sub(a1, a0));
        let a2 = e.call(getter, &args![p2]).f64();
        let a3 = e.call(getter, &args![p3]).f64();
        let d2 = x87_store32(x87_sub(a2, a3));
        differences[axis * 2] = d1;
        differences[axis * 2 + 1] = d2;
        let (high, low) = if parity(fcom_status(x87_load32(d1), zero), 0x05) {
            let high = x87_store32(e.call(getter, &args![p1]).f64());
            let low = x87_store32(e.call(getter, &args![p0]).f64());
            (high, low)
        } else {
            let high = x87_store32(e.call(getter, &args![p0]).f64());
            let low = x87_store32(e.call(getter, &args![p1]).f64());
            (high, low)
        };
        let (first, second) = if fcom_status(x87_load32(d2), zero) & 0x41 == 0 {
            (p3, p2)
        } else {
            (p2, p3)
        };
        let end = e.call(getter, &args![first]).f64();
        if !parity(fcom_status(x87_load32(high), end), 0x05) {
            return false;
        }
        let end = e.call(getter, &args![second]).f64();
        if fcom_status(x87_load32(low), end) & 0x41 == 0 {
            return false;
        }
    }
    let [dx1, dx2, dy1, dy2] = differences;
    let x0 = e.call(POINT_X, &args![p0]).f64();
    let x2 = e.call(POINT_X, &args![p2]).f64();
    let ex = x87_store32(x87_sub(x0, x2));
    let y0 = e.call(POINT_Y, &args![p0]).f64();
    let y2 = e.call(POINT_Y, &args![p2]).f64();
    let ey = x87_store32(x87_sub(y0, y2));
    let product = |a: u32, b: u32| x87_mul(x87_load32(a), x87_m32(b));
    let denominator = x87_store32(x87_sub(product(dy1, dx2), product(dx1, dy2)));
    let first = x87_store32(x87_sub(product(dx1, ey), product(dy1, ex)));
    let second = x87_store32(x87_sub(product(dy2, ex), product(dx2, ey)));
    let denominator_value = x87_load32(denominator);
    let status = fcom_status(denominator_value, zero);
    if !parity(status, 0x44) {
        return true;
    }
    for numerator in [second, first] {
        let value = x87_load32(numerator);
        if status & 0x41 == 0 {
            if !parity(fcom_status(value, zero), 0x05) {
                return false;
            }
            if !parity(fcom_status(denominator_value, value), 0x05) {
                return false;
            }
        } else {
            if fcom_status(value, zero) & 0x41 == 0 {
                return false;
            }
            if fcom_status(denominator_value, value) & 0x41 == 0 {
                return false;
            }
        }
    }
    true
}

// Translated from 004f7680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a point-list object's constructor
/// (`thiscall`, `RET 4`, a byte): the list (`0096a2d0`), the two corners
/// (`006815c0` at +0x10 and +0x18), the byte at +0xc, 0x400 at +0x20, 0 at
/// +0x24, the corner floats from `01016970` (+0x14, +0x10) and `01015f5c`
/// (+0x1c, +0x18) as `FLD`/`FSTP` copies, and no array (+8). Answers `this`.
// Translation incomplete: 004f7680 - the SEH frame (FS:[0], unwind of the list) is not translated.
pub fn fn_004f7680(e: &mut Engine, this: Ptr, kind: u8) -> Ptr {
    let base = this.addr();
    e.call(SIMPLE_LIST_CONSTRUCT, &args![this]);
    e.call(TRIVIAL_CONSTRUCT, &args![base.wrapping_add(0x10)]);
    e.call(TRIVIAL_CONSTRUCT, &args![base.wrapping_add(0x18)]);
    e.mem.set_u8(base.wrapping_add(0xc), kind);
    e.mem.set_u32(base.wrapping_add(0x20), 0x400);
    e.mem.set_u32(base.wrapping_add(0x24), 0);
    for (offset, constant) in [
        (0x14u32, BOUND_MIN),
        (0x10, BOUND_MIN),
        (0x1c, BOUND_MAX),
        (0x18, BOUND_MAX),
    ] {
        let value = x87_quiet_bits(e.global::<u32>(constant));
        e.mem.set_u32(base.wrapping_add(offset), value);
    }
    e.mem.set_u32(base.wrapping_add(8), 0);
    this
}

// Translated from 004f7740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: that object's destructor body: empties the
/// list (`004f7dd0`), frees the array at +8 when set (`00401030`), then
/// destroys the list (`0046ffb0`).
// Translation incomplete: 004f7740 - the SEH frame (FS:[0], unwind of the list) is not translated.
pub fn fn_004f7740(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(POINT_LIST_CLEAR, &args![this]);
    let array = e.mem.u32(base.wrapping_add(8));
    if array != 0 {
        e.call(FREE, &args![array]);
    }
    e.call(SIMPLE_LIST_DESTRUCT, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004f_7030, fn_004f7030(Ptr, Ptr) -> Ptr),
        entry!(0x004f_7070, fn_004f7070(Ptr, f32, f32) -> Ptr),
        entry!(0x004f_72a0, fn_004f72a0(Ptr, Ptr, f32) -> f32),
        entry!(0x004f_7310, fn_004f7310(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x004f_7680, fn_004f7680(Ptr, u8) -> Ptr),
        entry!(0x004f_7740, fn_004f7740(Ptr)),
    ]
}

#[cfg(test)]
mod b025_tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.register(POINT_X, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        e.register(POINT_Y, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 4)),
            ..Ret::default()
        });
        e.register(TRIVIAL_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(POINT_SET_X, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(POINT_SET_Y, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e
    }

    fn point(e: &mut Engine, x: f32, y: f32) -> u32 {
        let p = e.mem.alloc(8);
        e.mem.set_f32(p, x);
        e.mem.set_f32(p + 4, y);
        p
    }

    #[test]
    fn region_points_from_floats() {
        let mut e = engine();
        let source = point(&mut e, 1.5, -2.0);
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x004f_7030, &args![this, source]).u32(), this);
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (1.5, -2.0));
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.mem.set_u32(source, 0x7f80_0001);
        e.call(0x004f_7030, &args![this, source]);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(
            e.call(0x004f_7070, &args![this, 3.0f32, 4.0f32]).u32(),
            this
        );
        assert_eq!((e.mem.f32(this), e.mem.f32(this + 4)), (3.0, 4.0));
    }

    #[test]
    fn fn_004f72a0_interpolates_x_at_y() {
        let mut e = engine();
        let a = point(&mut e, 0.0, 0.0);
        let b = point(&mut e, 10.0, 20.0);
        // From b down the line to y = 5: x = 10 + (0 - 10) / (0 - 20) * (20 - 5).
        let r = e.call(0x004f_72a0, &args![a, b, 5.0f32]).f64();
        assert_eq!(r, 2.5);
        assert_eq!(e.call(0x004f_72a0, &args![0u32, b, 5.0f32]).f64(), 0.0);
        assert_eq!(e.call(0x004f_72a0, &args![a, 0u32, 5.0f32]).f64(), 0.0);
    }

    fn crosses(e: &mut Engine, s: [(f32, f32); 4]) -> bool {
        let p: Vec<u32> = s.iter().map(|(x, y)| point(e, *x, *y)).collect();
        e.call(0x004f_7310, &args![p[0], p[1], p[2], p[3]]).bool()
    }

    #[test]
    fn fn_004f7310_tests_segment_intersection() {
        let mut e = engine();
        // Crossing diagonals.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (0.0, 2.0), (2.0, 0.0)]
        ));
        // The same with the segments' ends swapped.
        assert!(crosses(
            &mut e,
            [(2.0, 2.0), (0.0, 0.0), (2.0, 0.0), (0.0, 2.0)]
        ));
        // Disjoint in x.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (3.0, 1.0)]
        ));
        // Disjoint in y.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (0.0, 2.0), (1.0, 3.0)]
        ));
        // Boxes overlap but the segments miss.
        assert!(!crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (1.5, 0.0), (3.0, 1.0)]
        ));
        // Parallel (zero denominator) with overlapping boxes: true.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (2.0, 2.0), (1.0, 1.0), (3.0, 3.0)]
        ));
        // Touching at an end.
        assert!(crosses(
            &mut e,
            [(0.0, 0.0), (1.0, 1.0), (1.0, 1.0), (2.0, 0.0)]
        ));
        // A null point.
        let p = point(&mut e, 0.0, 0.0);
        assert!(!e.call(0x004f_7310, &args![p, p, p, 0u32]).bool());
    }

    #[test]
    fn point_list_object_construct_and_destroy() {
        let mut e = engine();
        e.map(0x0101_5000, 0x2000);
        e.set_global::<f32>(BOUND_MIN, 3.0e38);
        e.set_global::<f32>(BOUND_MAX, -3.0e38);
        e.register(SIMPLE_LIST_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x28);
        e.mem.write(this, &[0xee; 0x28]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_7680, &args![this, 7u32]).u32(), this);
        assert_eq!(
            calls_to(&e, TRIVIAL_CONSTRUCT),
            vec![vec![this + 0x10], vec![this + 0x18]]
        );
        assert_eq!(e.mem.u8(this + 0xc), 7);
        assert_eq!(e.mem.u32(this + 0x20), 0x400);
        assert_eq!(e.mem.u32(this + 0x24), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(
            [e.mem.f32(this + 0x10), e.mem.f32(this + 0x14)],
            [3.0e38; 2]
        );
        assert_eq!(
            [e.mem.f32(this + 0x18), e.mem.f32(this + 0x1c)],
            [-3.0e38; 2]
        );
        for addr in [POINT_LIST_CLEAR, FREE, SIMPLE_LIST_DESTRUCT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.mem.set_u32(this + 8, 0x888);
        e.call_log = Some(vec![]);
        e.call(0x004f_7740, &args![this]);
        let order: Vec<(u32, Vec<u32>)> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            order,
            vec![
                (POINT_LIST_CLEAR, vec![this]),
                (FREE, vec![0x888]),
                (SIMPLE_LIST_DESTRUCT, vec![this]),
            ]
        );
        e.mem.set_u32(this + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_7740, &args![this]);
        assert!(calls_to(&e, FREE).is_empty());
    }
}
