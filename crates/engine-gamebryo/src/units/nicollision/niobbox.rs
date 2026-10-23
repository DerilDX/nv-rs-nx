//! `NiCollision/niobbox.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! An `NiOBBox` holds its model box (center at +0, axes at +0xc, +0x18 and +0x24, extents at
//! +0x30) and its world box in the same layout from +0x3c (center +0x3c, axes +0x48, +0x54 and
//! +0x60, extents +0x6c).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiPoint3::Unitize` (`004a0c10`, thiscall on the vector).
const UNITIZE: u32 = 0x004a_0c10;
/// The `double` 0.0001 at `01032980` below which a unitized extent counts as flat.
const FLAT_EXTENT_01032980: u32 = 0x0103_2980;
/// The float at `01015f5c` (`-FLT_MAX`) the ray's entry starts at.
const RAY_START_01015F5C: u32 = 0x0101_5f5c;
/// The float at `01016970` (`FLT_MAX`) the ray's exit starts at.
const RAY_END_01016970: u32 = 0x0101_6970;
/// The `double` 1e-7 at `0109a3a0` below which the ray runs parallel to a slab.
const PARALLEL_0109A3A0: u32 = 0x0109_a3a0;

fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// A stored float loaded back (`FSTP` then `FLD`).
fn round(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The product of the float at `loaded` (`FLD`) and the float operand at `multiplier`.
fn product(e: &Engine, loaded: u32, multiplier: u32) -> f64 {
    x87_mul(load(e, loaded), operand(e, multiplier))
}

/// `(first + second) + third` of three such products.
fn sum3(e: &Engine, terms: [(u32, u32); 3]) -> f64 {
    let [a, b, c] = terms.map(|(loaded, multiplier)| product(e, loaded, multiplier));
    x87_add(x87_add(a, b), c)
}

// Translated from 00eba480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBox::Transform` (Xbox PDB, thiscall): the world box from the model box and the transform
/// `transform` (rotation rows at +0, +0xc and +0x18, translation at +0x24, scale at +0x30). The
/// center is `translation + scale * (rotation * center)` (the rotated center, the scaled values
/// and the sums each stored), each axis `rotation * axis` and each extent `extent * scale`.
pub fn ni_obb_box_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    let b = this.addr();
    let t = transform.addr();
    let rotated = [
        sum3(e, [(t, b), (t + 4, b + 4), (t + 8, b + 8)]),
        sum3(e, [(t + 0xc, b), (t + 0x10, b + 4), (t + 0x14, b + 8)]),
        sum3(e, [(t + 0x1c, b + 4), (t + 0x18, b), (t + 0x20, b + 8)]),
    ]
    .map(x87_store32);
    let scale = round(load(e, t + 0x30));
    let scaled = [
        x87_store32(x87_mul(x87_load32(rotated[0]), scale)),
        x87_store32(x87_mul(x87_load32(rotated[1]), scale)),
        x87_store32(x87_mul(scale, x87_m32(rotated[2]))),
    ];
    for (axis, word) in scaled.iter().enumerate() {
        let offset = 4 * axis as u32;
        let center = x87_add(load(e, t + 0x24 + offset), x87_m32(*word));
        e.mem.set_u32(b + 0x3c + offset, x87_store32(center));
    }
    let x0 = sum3(e, [(t, b + 0xc), (b + 0x10, t + 4), (t + 8, b + 0x14)]);
    let y0 = sum3(
        e,
        [
            (t + 0xc, b + 0xc),
            (b + 0x10, t + 0x10),
            (t + 0x14, b + 0x14),
        ],
    );
    let z0 = sum3(
        e,
        [
            (b + 0x10, t + 0x1c),
            (t + 0x18, b + 0xc),
            (t + 0x20, b + 0x14),
        ],
    );
    for (offset, value) in [(0x48u32, x0), (0x4c, y0), (0x50, z0)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    let x1 = sum3(e, [(b + 0x1c, t + 4), (b + 0x18, t), (b + 0x20, t + 8)]);
    let y1 = sum3(
        e,
        [
            (b + 0x1c, t + 0x10),
            (b + 0x18, t + 0xc),
            (t + 0x14, b + 0x20),
        ],
    );
    let z1 = sum3(
        e,
        [
            (b + 0x1c, t + 0x1c),
            (t + 0x18, b + 0x18),
            (b + 0x20, t + 0x20),
        ],
    );
    for (offset, value) in [(0x54u32, x1), (0x58, y1), (0x5c, z1)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    let x2 = sum3(e, [(b + 0x24, t), (b + 0x28, t + 4), (b + 0x2c, t + 8)]);
    let y2 = sum3(
        e,
        [
            (b + 0x24, t + 0xc),
            (b + 0x28, t + 0x10),
            (t + 0x14, b + 0x2c),
        ],
    );
    let z2 = sum3(
        e,
        [
            (t + 0x1c, b + 0x28),
            (t + 0x18, b + 0x24),
            (b + 0x2c, t + 0x20),
        ],
    );
    for (offset, value) in [(0x60u32, x2), (0x64, y2), (0x68, z2)] {
        e.mem.set_u32(b + offset, x87_store32(value));
    }
    for axis in 0..3u32 {
        let extent = product(e, b + 0x30 + 4 * axis, t + 0x30);
        e.mem.set_u32(b + 0x6c + 4 * axis, x87_store32(extent));
    }
}

// Translated from 00ebf4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`fn_00ebf4a0`, thiscall on the box: `point`, `normal`): the world box's normal at
/// `point`. The world extents are copied and unitized (`004a0c10`); when one of them is at or
/// below the `double` 0.0001 at `01032980` (checked in axis order) the normal is that world
/// axis, copied. Otherwise with `d = point - center` (stored) each component is the box-frame
/// coordinate `axis . d` (stored) divided by the extent squared, and the result is unitized.
pub fn fn_00ebf4a0(e: &mut Engine, this: Ptr, point: Ptr, normal: Ptr) {
    let b = this.addr();
    let out = normal.addr();
    let unitized = e.with_stack(12, |e, local| {
        for offset in [0u32, 4, 8] {
            let extent = x87_store32(load(e, b + 0x6c + offset));
            e.mem.set_u32(local.addr() + offset, extent);
        }
        e.call(UNITIZE, &args![local]);
        [0u32, 4, 8].map(|offset| load(e, local.addr() + offset))
    });
    let flat = x87_load64(e.mem.u64(FLAT_EXTENT_01032980));
    for (axis, extent) in unitized.iter().enumerate() {
        if *extent <= flat {
            let source = b + 0x48 + 0xc * axis as u32;
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(source + offset);
                e.mem.set_u32(out.wrapping_add(offset), word);
            }
            return;
        }
    }
    let p = point.addr();
    let [dx, dy, dz] = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, p.wrapping_add(offset)),
            operand(e, b + 0x3c + offset),
        ))
    });
    let along = |e: &Engine, axis: u32| {
        let x = x87_mul(load(e, axis), dx);
        let y = x87_mul(load(e, axis + 4), dy);
        let z = x87_mul(load(e, axis + 8), dz);
        x87_add(x87_add(x, y), z)
    };
    let first = round(along(e, b + 0x48));
    let e0 = load(e, b + 0x6c);
    e.mem
        .set_u32(out, x87_store32(x87_div(first, x87_mul(e0, e0))));
    let second = x87_store32(along(e, b + 0x54));
    let e1 = load(e, b + 0x70);
    let value = x87_div(x87_m32(second), x87_mul(e1, e1));
    e.mem.set_u32(out.wrapping_add(4), x87_store32(value));
    let third = x87_store32(along(e, b + 0x60));
    let e2 = load(e, b + 0x74);
    let value = x87_div(x87_m32(third), x87_mul(e2, e2));
    e.mem.set_u32(out.wrapping_add(8), x87_store32(value));
    e.call(UNITIZE, &args![normal]);
}

// Translated from 00ebf610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiOBBox::TestIntersection` (Xbox PDB, the ray overload, thiscall: `origin`, `direction`):
/// the slab test of the ray against the world box. The entry starts at the float at `01015f5c`
/// and the exit at the float at `01016970`; per axis with `s = axis . (center - origin)` and
/// `f = axis . direction` (stored): `|f|` (stored) at or below the `double` 1e-7 at `0109a3a0`
/// misses when `|s|` exceeds `|extent|`; otherwise the two slab times `(s + |extent|) / f` and
/// `(s - |extent|) / f` (with `1 / f` stored, both stored) narrow the interval, which misses when
/// the entry passes the exit or the exit is behind the origin. Answers whether all three slabs
/// leave an interval.
pub fn ni_obb_box_test_intersection_ray(
    e: &mut Engine,
    this: Ptr,
    origin: Ptr,
    direction: Ptr,
) -> bool {
    let b = this.addr();
    let o = origin.addr();
    let u = direction.addr();
    let mut entry = x87_store32(load(e, RAY_START_01015F5C));
    let mut exit = round(load(e, RAY_END_01016970));
    let extents = [0x6cu32, 0x70, 0x74].map(|offset| x87_store32(round(load(e, b + offset).abs())));
    let [d0, d1, d2] = [0u32, 4, 8].map(|offset| {
        round(x87_sub(
            load(e, b + 0x3c + offset),
            operand(e, o.wrapping_add(offset)),
        ))
    });
    let parallel = x87_load64(e.mem.u64(PARALLEL_0109A3A0));
    for (index, extent) in extents.iter().enumerate() {
        let axis = b + 0x48 + 0xc * index as u32;
        let s = x87_add(
            x87_add(
                x87_mul(d1, operand(e, axis + 4)),
                x87_mul(d0, operand(e, axis)),
            ),
            x87_mul(d2, operand(e, axis + 8)),
        );
        let s = x87_store32(s);
        let f = round(x87_add(
            x87_add(product(e, u, axis), product(e, axis + 4, u.wrapping_add(4))),
            product(e, u.wrapping_add(8), axis + 8),
        ));
        if round(f.abs()) > parallel {
            let inverse = round(x87_div(1.0, f));
            let start = x87_load32(s);
            let near = round(x87_mul(x87_add(x87_load32(*extent), start), inverse));
            let far = round(x87_mul(x87_sub(start, x87_m32(*extent)), inverse));
            let (high, low) = if far < near {
                (round(near), round(far))
            } else {
                (far, near)
            };
            if x87_load32(entry) < low {
                entry = x87_store32(low);
            }
            if high < exit {
                exit = round(high);
            }
            if x87_load32(entry) > exit || 0.0 > exit {
                return false;
            }
        } else if x87_load32(*extent) < round(x87_load32(s).abs()) {
            return false;
        }
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00eb_a480, ni_obb_box_transform(Ptr, Ptr)),
        entry!(0x00eb_f4a0, fn_00ebf4a0(Ptr, Ptr, Ptr)),
        entry!(
            0x00eb_f610,
            ni_obb_box_test_intersection_ray(Ptr, Ptr, Ptr) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn floats(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        block
    }

    fn read(e: &Engine, address: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(address + 4 * i)).collect()
    }

    /// An `NiOBBox` whose world box is `center`, the unit axes and `extents`.
    fn world_box(e: &mut Engine, center: [f32; 3], extents: [f32; 3]) -> u32 {
        let mut words = vec![0.0f32; 15];
        words.extend_from_slice(&center);
        words.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
        words.extend_from_slice(&extents);
        floats(e, &words)
    }

    #[test]
    fn transform_rotates_scales_and_moves_the_box() {
        let mut e = Engine::new();
        // Model box: center (1, 2, 3), the unit axes, extents (1, 2, 3).
        let mut model = vec![1.0f32, 2.0, 3.0];
        model.extend_from_slice(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0]);
        model.extend_from_slice(&[0.0; 15]);
        let this = floats(&mut e, &model);
        // A quarter turn about z (x goes to y), translation (10, 20, 30), scale 2.
        let transform = floats(
            &mut e,
            &[
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0, 20.0, 30.0, 2.0,
            ],
        );
        e.call(0x00eb_a480, &args![this, transform]);
        assert_eq!(read(&e, this + 0x3c, 3), vec![6.0, 22.0, 36.0]);
        assert_eq!(
            read(&e, this + 0x48, 9),
            vec![0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0]
        );
        assert_eq!(read(&e, this + 0x6c, 3), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn normal_follows_the_extents() {
        let mut e = Engine::new();
        e.map(0x0103_2000, 0x1000);
        e.mem.set_f64(FLAT_EXTENT_01032980, 0.0001);
        e.register(UNITIZE, |e, a| {
            let v: Vec<f64> = (0..3).map(|i| f64::from(e.mem.f32(a[0] + 4 * i))).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for (i, c) in v.iter().enumerate() {
                e.mem.set_f32(a[0] + 4 * i as u32, (c / length) as f32);
            }
            Ret::default()
        });
        let out = e.mem.alloc(12);
        let point = floats(&mut e, &[1.0, 4.0, 0.0]);
        let off_axis = floats(&mut e, &[2.0, 0.0, 0.5]);
        let cube = world_box(&mut e, [1.0, 0.0, 0.0], [1.0, 2.0, 1.0]);
        e.call(0x00eb_f4a0, &args![cube, point, out]);
        assert_eq!(read(&e, out, 3), vec![0.0, 1.0, 0.0]);
        // A flat second extent answers its axis.
        let flat = world_box(&mut e, [1.0, 0.0, 0.0], [1.0, 0.0, 1.0]);
        e.call(0x00eb_f4a0, &args![flat, off_axis, out]);
        assert_eq!(read(&e, out, 3), vec![0.0, 1.0, 0.0]);
        let flat = world_box(&mut e, [1.0, 0.0, 0.0], [0.0, 1.0, 1.0]);
        e.call(0x00eb_f4a0, &args![flat, off_axis, out]);
        assert_eq!(read(&e, out, 3), vec![1.0, 0.0, 0.0]);
    }

    #[test]
    fn ray_slab_test() {
        let mut e = Engine::new();
        e.map(0x0101_5000, 0x2000);
        e.map(0x0109_a000, 0x1000);
        e.mem.set_u32(RAY_START_01015F5C, 0xff7f_ffff);
        e.mem.set_f32(RAY_END_01016970, f32::MAX);
        e.mem.set_f64(PARALLEL_0109A3A0, 1e-7);
        let cube = world_box(&mut e, [5.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        let origin = floats(&mut e, &[0.0, 0.0, 0.0]);
        let beside = floats(&mut e, &[0.0, 3.0, 0.0]);
        let inside = floats(&mut e, &[5.0, 0.5, 0.0]);
        let mut hits = |direction: [f32; 3], start: u32| {
            let d = floats(&mut e, &direction);
            e.call(0x00eb_f610, &args![cube, start, d]).bool()
        };
        assert!(hits([1.0, 0.0, 0.0], origin));
        assert!(hits([1.0, 0.15, 0.1], origin));
        // Behind, past the slab, and parallel outside a slab.
        assert!(!hits([-1.0, 0.0, 0.0], origin));
        assert!(!hits([1.0, 0.5, 0.0], origin));
        assert!(!hits([1.0, 0.0, 0.0], beside));
        assert!(hits([0.0, 0.0, 1.0], inside));
    }
}
