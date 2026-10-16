//! `BSHavok/run_00ca4120` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `float` constants in the exe image: the starting corners of a box (`0104da48` and
/// `0104e148`, the largest and smallest values the code starts from), zero (`01011d78`), a
/// scale (`012682c8`) and the tolerance `0.001` (`01017d00`), plus `3.4028234663852886e38`
/// as a `double` (`0104e138`).
const BOX_START_MIN: u32 = 0x0104_da48;
const BOX_START_MAX: u32 = 0x0104_e148;
const ZERO: u32 = 0x0101_1d78;
const POINT_SCALE: u32 = 0x0126_82c8;
const TOLERANCE: u32 = 0x0101_7d00;
const FLOAT_MAX_DOUBLE: u32 = 0x0104_e138;
/// `00cb2160(this, flag, a, tolerance)` (`RET 0xc`): writes a byte at `flag`, 1 when two
/// sets of vectors agree within the tolerance, and returns `flag`. Outside this unit.
const NEARLY_EQUAL: u32 = 0x00cb_2160;
/// `00c7f640(out, transform, vector)` (`RET 8`, ECX = out): transforms a point.
const TRANSFORM_POINT: u32 = 0x00c7_f640;

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, value: [f32; 4]) {
    for (i, lane) in value.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, lane);
    }
}

// Translated from 00ca4280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4280 - the /GS cookie check 00ec408c is not modeled
/// Computes a bounding box (`bounds`: minimum at +0, maximum at +0x10) of the point lists
/// kept by `this` (ECX; the class is unconfirmed): `this + 0x98` is an array of 8-byte
/// entries (count at +0xa0) whose first word points to a record with a point array at +0x20
/// (12-byte points) and a 16-bit count at +8. Each point is scaled lane-wise by the vector
/// at `this + 0x80` (after a multiplication by the scale constant at `012682c8`), moved by
/// the transform `transform` through `00c7f640`, and folded into the box. When the float at
/// `this + 0x60` is not the largest float, the first step is a shortcut: if
/// `00cb2160(this + 0x20, transform, 0.001)` reports the vectors equal, the box is simply the
/// pair of vectors at `this + 0x60` and `this + 0x70`. Finally the box is grown by the float
/// at `this + 0x90` plus `margin` on every lane. The empty box starts at the two largest
/// floats. The stack protector check at the end is not modeled.
pub fn fn_00ca4280(e: &mut Engine, this: Ptr, transform: u32, margin: f32, bounds: Ptr) {
    let this = this.addr();
    let bounds = bounds.addr();
    let reference = e.mem.f32(this + 0x60);
    let largest = e.mem.f64(FLOAT_MAX_DOUBLE);
    let mut shortcut = false;
    if f64::from(reference) != largest {
        let tolerance = e.mem.u32(TOLERANCE);
        let equal = e.with_stack(4, |e, flag| {
            e.call(
                NEARLY_EQUAL,
                &args![this + 0x20, flag, transform, x87_quiet_bits(tolerance)],
            );
            e.mem.u8(flag.addr()) != 0
        });
        if equal {
            let low = lanes(e, this + 0x60);
            let high = lanes(e, this + 0x70);
            set_lanes(e, bounds, low);
            set_lanes(e, bounds + 0x10, high);
            shortcut = true;
        }
    }
    if !shortcut {
        let start_min = e.mem.f32(BOX_START_MIN);
        let start_max = e.mem.f32(BOX_START_MAX);
        let zero = e.mem.f32(ZERO);
        set_lanes(e, bounds, [start_min, start_min, start_min, zero]);
        set_lanes(e, bounds + 0x10, [start_max, start_max, start_max, zero]);
        let count = e.mem.u32(this + 0xa0);
        let mut index = 0u32;
        while index < count {
            let entry = e.mem.u32(e.mem.u32(this + 0x98) + index * 8);
            let mut point = e.mem.u32(entry + 0x20) + 8;
            let mut remaining = u32::from(e.mem.u16(entry + 8));
            while remaining != 0 {
                let scale = e.mem.f32(POINT_SCALE);
                let weights = lanes(e, this + 0x80);
                let scaled = [
                    (f64::from(e.mem.f32(point - 8)) * f64::from(scale)) as f32,
                    (f64::from(e.mem.f32(point - 4)) * f64::from(scale)) as f32,
                    (f64::from(scale) * f64::from(e.mem.f32(point))) as f32,
                    0.0,
                ];
                let product = [0, 1, 2, 3].map(|i| weights[i] * scaled[i]);
                e.with_stack(0x30, |e, local| {
                    let l = local.addr();
                    set_lanes(e, l, product);
                    e.call(TRANSFORM_POINT, &args![l + 0x10, transform, l]);
                    let world = lanes(e, l + 0x10);
                    let low = lanes(e, bounds);
                    let high = lanes(e, bounds + 0x10);
                    // MINPS / MAXPS: the second operand wins unless the first is strictly
                    // smaller / larger (so NaN lanes take the new point).
                    let low =
                        [0, 1, 2, 3].map(|i| if low[i] < world[i] { low[i] } else { world[i] });
                    let high = [0, 1, 2, 3].map(|i| {
                        if high[i] > world[i] {
                            high[i]
                        } else {
                            world[i]
                        }
                    });
                    set_lanes(e, bounds, low);
                    set_lanes(e, bounds + 0x10, high);
                });
                point += 12;
                remaining -= 1;
            }
            index += 1;
        }
    }
    let grow = (f64::from(e.mem.f32(this + 0x90)) + f64::from(margin)) as f32;
    let low = lanes(e, bounds);
    let high = lanes(e, bounds + 0x10);
    set_lanes(e, bounds, low.map(|lane| lane - grow));
    set_lanes(e, bounds + 0x10, high.map(|lane| lane + grow));
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ca_4280, fn_00ca4280(Ptr, u32, f32, Ptr))]
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A box fed from one list of two points; the transform double adds 10 to every lane
    /// and the nearly-equal double reports `equal`.
    fn rig(equal: u8) -> (Engine, u32, u32) {
        let mut e = test_engine();
        e.mem.set_f32(BOX_START_MIN, 3.0e38);
        e.mem.set_f32(BOX_START_MAX, -3.0e38);
        e.mem.set_f32(ZERO, 0.0);
        e.mem.set_f32(POINT_SCALE, 2.0);
        e.mem.set_f64(FLOAT_MAX_DOUBLE, f64::from(f32::MAX));
        e.register_double(NEARLY_EQUAL, move |e, a| {
            e.mem.set_u8(a[1], equal);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(TRANSFORM_POINT, |e, a| {
            let v = lanes(e, a[2]);
            set_lanes(e, a[0], v.map(|lane| lane + 10.0));
            Ret::default()
        });
        let this = e.mem.alloc(0x100);
        e.mem.set_f32(this + 0x60, 1.0);
        set_lanes(&mut e, this + 0x60, [1.0, 2.0, 3.0, 4.0]);
        set_lanes(&mut e, this + 0x70, [5.0, 6.0, 7.0, 8.0]);
        set_lanes(&mut e, this + 0x80, [1.0, 1.0, 1.0, 1.0]);
        e.mem.set_f32(this + 0x90, 0.5);
        let points = e.mem.alloc(0x40);
        for (i, v) in [1.0f32, 2.0, 3.0, -1.0, 0.0, 5.0].into_iter().enumerate() {
            e.mem.set_f32(points + 4 * i as u32, v);
        }
        let record = e.mem.alloc(0x40);
        e.mem.set_u32(record + 0x20, points);
        e.mem.set_u16(record + 8, 2);
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, record);
        e.mem.set_u32(this + 0x98, list);
        e.mem.set_u32(this + 0xa0, 1);
        let bounds = e.mem.alloc(0x20);
        (e, this, bounds)
    }

    #[test]
    fn folds_the_transformed_points_and_grows_the_box() {
        let (mut e, this, bounds) = rig(0);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        // points scaled by 2 then moved by 10: (12, 14, 16) and (8, 10, 20); grown by 0.75.
        assert_eq!(lanes(&e, bounds), [7.25, 9.25, 15.25, -0.75]);
        assert_eq!(lanes(&e, bounds + 0x10), [12.75, 14.75, 20.75, 10.75]);
    }

    #[test]
    fn the_shortcut_copies_the_stored_box() {
        let (mut e, this, bounds) = rig(1);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        assert_eq!(lanes(&e, bounds), [0.25, 1.25, 2.25, 3.25]);
        assert_eq!(lanes(&e, bounds + 0x10), [5.75, 6.75, 7.75, 8.75]);
        // The largest float skips the shortcut test altogether.
        let (mut e, this, bounds) = rig(1);
        e.mem.set_f32(this + 0x60, f32::MAX);
        e.call(0x00ca_4280, &args![this, 0x1000u32, 0.25f32, bounds]);
        assert_eq!(lanes(&e, bounds)[0], 7.25);
    }
}
