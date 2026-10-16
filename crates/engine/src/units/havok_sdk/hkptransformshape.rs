//! `Havok SDK/hkptransformshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpTransformShape` wraps a child shape (`m_childShape`, the `hkpSingleShapeContainer` at
//! `+0x10`) in a rigid transform (`m_transform`, an `hkTransform` at `+0x30`: three rotation
//! columns at `+0x30`, `+0x40`, `+0x50` and the translation at `+0x60`). SSE code computes
//! lane by lane in `f32`. Taking a vector into the child's space is the `SHUFPS`/`MULPS`
//! sequence `(v.y * cy + v.x * cx) + v.z * cz` with `cx = (c0.x, c1.x, c2.x, c2.y)`,
//! `cy = (c0.y, c1.y, c2.y, c2.w)` and `cz = (c0.z, c1.z, c2.z, c2.w)`; the fourth lane is
//! garbage but is kept, because it is passed on. The profiling records use the host clock for
//! `RDTSC` (see `run_00cf5070`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::run_00cf5070::profile_record;

layout! {
    /// `hkpTransformShape` (Xbox PDB), 0x70 bytes; offsets checked against the code below.
    pub struct HkpTransformShape: 0x70 {
        /// `m_childShape` (Xbox PDB): the `hkpSingleShapeContainer`'s shape, at +0x14 (its
        /// vtable is at +0x10).
        0x14 m_childShape: Ptr,
    }
}

/// `m_transform` (Xbox PDB) in the shape.
const TRANSFORM: u32 = 0x30;
/// `hkTransform::setMul` (Xbox PDB, `this` = result, then the two transforms).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// `hkVector4::setRotatedDir` (Xbox PDB, `this` = result, then the rotation and the vector).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
/// Profiling labels: `"TtrcTransform"` and the shared end label.
const LABEL_TRANSFORM: u32 = 0x010c_c448;
const LABEL_END: u32 = 0x010c_41f8;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, *value);
    }
}

/// The vector `v` taken through the shape's rotation columns (the `UNPCKLPS`/`SHUFPS`/`MULPS`
/// sequence): every lane `i` is `(v.y * cy[i] + v.x * cx[i]) + v.z * cz[i]`.
fn through_columns(e: &Engine, this: Ptr<HkpTransformShape>, v: Vec4) -> Vec4 {
    let c0 = load_vec(e, this.addr() + TRANSFORM);
    let c1 = load_vec(e, this.addr() + TRANSFORM + 0x10);
    let c2 = load_vec(e, this.addr() + TRANSFORM + 0x20);
    let cx = [c0[0], c1[0], c2[0], c2[1]];
    let cy = [c0[1], c1[1], c2[1], c2[3]];
    let cz = [c0[2], c1[2], c2[2], c2[3]];
    std::array::from_fn(|i| (v[1] * cy[i] + v[0] * cx[i]) + v[2] * cz[i])
}

/// A point of the ray (`point` is the vector at `address`) taken into the child's space:
/// `(point - translation)` through the columns.
fn point_into_child_space(e: &Engine, this: Ptr<HkpTransformShape>, address: u32) -> Vec4 {
    let point = load_vec(e, address);
    let translation = load_vec(e, this.addr() + TRANSFORM + 0x30);
    let offset: Vec4 = std::array::from_fn(|lane| point[lane] - translation[lane]);
    through_columns(e, this, offset)
}

// Translated from 00cf58f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getContainer` (Xbox PDB): the shape container at `this + 0x10`.
pub fn get_container(_e: &mut Engine, this: Ptr<HkpTransformShape>) -> u32 {
    this.addr() + 0x10
}

// Translated from 00cf59e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getAabbImpl` (Xbox PDB): composes `transform` with the shape's
/// transform into a local `hkTransform` (`hkTransform::setMul`) and asks the child for its
/// AABB with it (child slot `+0x1C`: the local transform, the tolerance as passed through the
/// x87 stack, and `out`).
pub fn get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    e.with_stack(0x50, |e, local| {
        e.call(
            TRANSFORM_SET_MUL,
            &args![local, transform, this.addr() + TRANSFORM],
        );
        let child = e.get(this, HkpTransformShape::m_childShape);
        e.vcall(
            child.addr(),
            0x1c,
            &args![local, x87_quiet(tolerance).to_bits(), out],
        );
    });
}

// Translated from 00cf5a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::getMaximumProjection` (Xbox PDB): takes `direction` into the child's
/// space, asks the child (slot `+0xC`) for its maximum projection `s` along it, and returns
/// the projection of the transformed support: `s * local` is taken back through the rotation
/// columns, the translation is added, and the result is dotted with `direction`
/// (`z + (y + x)`).
pub fn get_maximum_projection(e: &mut Engine, this: Ptr<HkpTransformShape>, direction: Ptr) -> f32 {
    let v = load_vec(e, direction.addr());
    let local = through_columns(e, this, v);
    let buffer = e.mem.alloc(16);
    store_vec(e, buffer, local);
    let child = e.get(this, HkpTransformShape::m_childShape);
    let projection = x87_quiet(e.vcall(child.addr(), 0xc, &args![buffer]).f32());
    e.mem.free(buffer);
    let scaled: Vec4 = std::array::from_fn(|lane| projection * local[lane]);
    let c0 = load_vec(e, this.addr() + TRANSFORM);
    let c1 = load_vec(e, this.addr() + TRANSFORM + 0x10);
    let c2 = load_vec(e, this.addr() + TRANSFORM + 0x20);
    let c3 = load_vec(e, this.addr() + TRANSFORM + 0x30);
    let direction_vec = load_vec(e, direction.addr());
    let product: Vec4 = std::array::from_fn(|i| {
        let back = (scaled[1] * c1[i] + scaled[0] * c0[i]) + (scaled[2] * c2[i] + c3[i]);
        back * direction_vec[i]
    });
    x87_quiet(product[2] + (product[1] + product[0]))
}

// Translated from 00cf5b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::castRayImpl` (Xbox PDB; the `hkBool` result goes through the hidden
/// pointer `result`, which is returned): casts the ray against the child in the child's
/// space. The 0x30-byte input is copied with its `from` (`+0`) and `to` (`+0x10`) points
/// taken into the child's space, the output's child depth (`output + 0x40`) is raised for the
/// child's cast (slot `+0x20`), and on a hit the output's normal (`output + 0`) is rotated
/// back (`hkVector4::setRotatedDir`) and the child key recorded at
/// `output + 0x20 + depth * 4` is set to 0. Brackets the work with the `"TtrcTransform"` and
/// end profiling records.
pub fn cast_ray_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, LABEL_TRANSFORM);
    let from = point_into_child_space(e, this, input.addr());
    let local = e.mem.alloc(0x30);
    let bytes = e.mem.bytes(input.addr(), 0x30);
    e.mem.write(local, &bytes);
    let to = point_into_child_space(e, this, input.addr() + 0x10);
    store_vec(e, local, from);
    store_vec(e, local + 0x10, to);
    let depth = e.mem.u32(output.addr() + 0x40);
    e.mem.set_u32(output.addr() + 0x40, depth.wrapping_add(1));
    let child = e.get(this, HkpTransformShape::m_childShape);
    e.vcall(child.addr(), 0x20, &args![result, local, output]);
    e.mem.free(local);
    let depth = e.mem.u32(output.addr() + 0x40).wrapping_sub(1);
    e.mem.set_u32(output.addr() + 0x40, depth);
    if e.mem.u8(result.addr()) != 0 {
        let normal = e.mem.bytes(output.addr(), 16);
        let copy = e.mem.alloc(16);
        e.mem.write(copy, &normal);
        e.call(
            SET_ROTATED_DIR,
            &args![output, this.addr() + TRANSFORM, copy],
        );
        e.mem.free(copy);
        let depth = e.mem.u32(output.addr() + 0x40);
        e.mem
            .set_u32(output.addr() + 0x20 + depth.wrapping_mul(4), 0);
    }
    profile_record(e, LABEL_END);
    result
}

// Translated from 00cf5cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpTransformShape::castRayWithCollectorImpl` (Xbox PDB): casts the ray against the child
/// in the child's space, as `castRayImpl` does, and gives the child (slot `+0x24`) a body
/// record `(child, 0, composed transform, body)` where the transform is the body's transform
/// (`body + 8`) composed with the shape's (`hkTransform::setMul`). Brackets the work with the
/// `"TtrcTransform"` and end profiling records.
pub fn cast_ray_with_collector_impl(
    e: &mut Engine,
    this: Ptr<HkpTransformShape>,
    input: Ptr,
    body: Ptr,
    collector: u32,
) {
    profile_record(e, LABEL_TRANSFORM);
    let from = point_into_child_space(e, this, input.addr());
    let local = e.mem.alloc(0x30);
    let to = point_into_child_space(e, this, input.addr() + 0x10);
    let bytes = e.mem.bytes(input.addr(), 0x30);
    e.mem.write(local, &bytes);
    store_vec(e, local, from);
    store_vec(e, local + 0x10, to);
    let composed = e.mem.alloc(0x40);
    let body_transform = e.mem.u32(body.addr() + 8);
    e.call(
        TRANSFORM_SET_MUL,
        &args![composed, body_transform, this.addr() + TRANSFORM],
    );
    let child = e.get(this, HkpTransformShape::m_childShape);
    let record = e.mem.alloc(16);
    e.mem.set_u32(record, child.addr());
    e.mem.set_u32(record + 4, 0);
    e.mem.set_u32(record + 8, composed);
    e.mem.set_u32(record + 12, body.addr());
    e.vcall(child.addr(), 0x24, &args![local, record, collector]);
    e.mem.free(record);
    e.mem.free(composed);
    e.mem.free(local);
    profile_record(e, LABEL_END);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf_58f0, get_container(Ptr<HkpTransformShape>) -> u32),
        entry!(
            0x00cf_59e0,
            get_aabb_impl(Ptr<HkpTransformShape>, Ptr, f32, Ptr)
        ),
        entry!(
            0x00cf_5a20,
            get_maximum_projection(Ptr<HkpTransformShape>, Ptr) -> f32
        ),
        entry!(
            0x00cf_5b20,
            cast_ray_impl(Ptr<HkpTransformShape>, Ptr, Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cf_5cc0,
            cast_ray_with_collector_impl(Ptr<HkpTransformShape>, Ptr, Ptr, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00cf5070::{PROFILE_SLOT, TLS_GET_VALUE};
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_DOUBLE: u32 = 0x0900_0200;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_8000, 0x1000);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x80);
        e.mem.set_u32(PROFILE_SLOT, 2);
        e.register_double(TLS_GET_VALUE, move |_, _| Ret {
            eax: stream,
            ..Ret::default()
        });
        e
    }

    /// A transform shape whose child has `CHILD_DOUBLE` in vtable slot `slot`; the rotation
    /// columns are c0 = (0, 1, 0, 0), c1 = (-1, 0, 0, 0), c2 = (0, 0, 1, 0) and the
    /// translation (1, 2, 3, 0).
    fn shape(e: &mut Engine, slot: u32) -> (u32, u32) {
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + slot, CHILD_DOUBLE);
        let child = e.mem.alloc(0x10);
        e.mem.set_u32(child, vtable);
        let shape = e.mem.alloc(0x70);
        e.mem.set_u32(shape + 0x14, child);
        store_vec(e, shape + 0x30, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, shape + 0x40, [-1.0, 0.0, 0.0, 0.0]);
        store_vec(e, shape + 0x50, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, shape + 0x60, [1.0, 2.0, 3.0, 0.0]);
        (shape, child)
    }

    #[test]
    fn get_container_is_this_plus_0x10() {
        let mut e = engine();
        assert_eq!(e.call(0x00cf_58f0, &args![0x1000u32]).u32(), 0x1010);
    }

    #[test]
    fn get_aabb_impl_composes_the_transforms_and_asks_the_child() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x1c);
        let set_mul = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&set_mul);
        e.register_double(TRANSFORM_SET_MUL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let child_log = Rc::clone(&seen);
        e.register_double(CHILD_DOUBLE, move |_, a| {
            child_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let transform = e.mem.alloc(0x40);
        let out = e.mem.alloc(0x20);
        // A signalling NaN tolerance comes out quieted.
        e.call(0x00cf_59e0, &args![shape, transform, 0x7f80_0001u32, out]);
        let set_mul = set_mul.borrow();
        assert_eq!(set_mul.len(), 1);
        assert_eq!((set_mul[0][1], set_mul[0][2]), (transform, shape + 0x30));
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0][0], child);
        assert_eq!(seen[0][1], set_mul[0][0]);
        assert_eq!(seen[0][2], 0x7fc0_0001);
        assert_eq!(seen[0][3], out);
    }

    #[test]
    fn get_maximum_projection_scales_the_childs_answer() {
        let mut e = engine();
        let (shape, _) = shape(&mut e, 0xc);
        let local = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&local);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            log.borrow_mut()
                .extend((0..4).map(|i| e.mem.f32(a[1] + 4 * i)));
            Ret {
                st0: 2.0,
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 2.0, 3.0, 0.0]);
        let got = e.call(0x00cf_5a20, &args![shape, direction]).f32();
        // local = (2, -1, 3, 0); scaled = (4, -2, 6, 0); back (rotated by the columns, plus
        // the translation) = (-2 * -1 + 4 * 0 + 6 * 0 + 1, -2 * 0 + 4 * 1 + 0 + 2,
        // 0 + 0 + 6 + 3, ...) = (3, 6, 9); dotted with the direction: 3 + 12 + 27.
        assert_eq!(*local.borrow(), vec![2.0, -1.0, 3.0, 0.0]);
        assert_eq!(got, 42.0);
    }

    #[test]
    fn cast_ray_impl_transforms_the_ray_and_rotates_the_normal_on_a_hit() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x20);
        let seen = Rc::new(RefCell::new(Vec::new()));
        e.register_double(CHILD_DOUBLE, {
            let log = Rc::clone(&seen);
            move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..12).map(|i| e.mem.u32(a[2] + 4 * i)));
                words.push(e.mem.u32(a[3] + 0x40));
                log.borrow_mut().push(words);
                e.mem.set_u8(a[1], 1);
                Ret::default()
            }
        });
        let rotated = Rc::new(RefCell::new(Vec::new()));
        let rotated_log = Rc::clone(&rotated);
        e.register_double(SET_ROTATED_DIR, move |e, a| {
            rotated_log.borrow_mut().push((a.to_vec(), e.mem.u32(a[2])));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [2.0, 3.0, 4.0, 0.0]);
        store_vec(&mut e, input + 0x10, [1.0, 2.0, 3.0, 0.0]);
        e.mem.set_u32(input + 0x24, 0x1234);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output, 0x3f80_0000);
        e.mem.set_u32(output + 0x40, 1);
        e.mem.set_u32(output + 0x20 + 4, 9);
        let result = e.mem.alloc(4);
        let got = e
            .call(0x00cf_5b20, &args![shape, result, input, output])
            .u32();
        assert_eq!(got, result);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let call = &seen[0];
        assert_eq!((call[0], call[1], call[3]), (child, result, output));
        // from - translation = (1, 1, 1, 0) -> (1, -1, 1, 0); to - translation = 0.
        let floats: Vec<f32> = call[4..16].iter().map(|w| f32::from_bits(*w)).collect();
        assert_eq!(&floats[0..4], &[1.0, -1.0, 1.0, 0.0]);
        assert_eq!(&floats[4..8], &[0.0, 0.0, 0.0, 0.0]);
        assert_eq!(call[4 + 9], 0x1234);
        assert_eq!(call[16], 2);
        assert_eq!(e.mem.u32(output + 0x40), 1);
        let rotated = rotated.borrow();
        assert_eq!(rotated.len(), 1);
        assert_eq!(
            &rotated[0].0[0..3],
            &[output, shape + 0x30, rotated[0].0[2]]
        );
        assert_eq!(rotated[0].1, 0x3f80_0000);
        assert_eq!(e.mem.u32(output + 0x20 + 4), 0);
    }

    #[test]
    fn cast_ray_impl_leaves_the_output_alone_on_a_miss() {
        let mut e = engine();
        let (shape, _) = shape(&mut e, 0x20);
        e.register_double(CHILD_DOUBLE, |e, a| {
            e.mem.set_u8(a[1], 0);
            Ret::default()
        });
        e.register_double(SET_ROTATED_DIR, |_, _| panic!("no hit, no rotation"));
        let input = e.mem.alloc(0x30);
        let output = e.mem.alloc(0x50);
        e.mem.set_u32(output + 0x20, 5);
        let result = e.mem.alloc(4);
        e.mem.set_u8(result, 7);
        e.call(0x00cf_5b20, &args![shape, result, input, output]);
        assert_eq!(e.mem.u32(output + 0x20), 5);
        assert_eq!(e.mem.u32(output + 0x40), 0);
    }

    #[test]
    fn cast_ray_with_collector_composes_the_body_transform() {
        let mut e = engine();
        let (shape, child) = shape(&mut e, 0x24);
        let composed = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&composed);
        e.register_double(TRANSFORM_SET_MUL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let seen = Rc::new(RefCell::new(Vec::new()));
        let child_log = Rc::clone(&seen);
        e.register_double(CHILD_DOUBLE, move |e, a| {
            let ray: Vec<f32> = (0..8).map(|i| e.mem.f32(a[1] + 4 * i)).collect();
            let record: Vec<u32> = (0..4).map(|i| e.mem.u32(a[2] + 4 * i)).collect();
            child_log.borrow_mut().push((a.to_vec(), ray, record));
            Ret::default()
        });
        let input = e.mem.alloc(0x30);
        store_vec(&mut e, input, [2.0, 3.0, 4.0, 0.0]);
        store_vec(&mut e, input + 0x10, [3.0, 2.0, 1.0, 0.0]);
        let body = e.mem.alloc(0x10);
        let body_transform = e.mem.alloc(0x40);
        e.mem.set_u32(body + 8, body_transform);
        e.call(0x00cf_5cc0, &args![shape, input, body, 0x5555u32]);
        let composed = composed.borrow();
        assert_eq!(composed.len(), 1);
        assert_eq!(
            (composed[0][1], composed[0][2]),
            (body_transform, shape + 0x30)
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (words, ray, record) = &seen[0];
        assert_eq!(words[0], child);
        assert_eq!(words[3], 0x5555);
        assert_eq!(&ray[0..4], &[1.0, -1.0, 1.0, 0.0]);
        assert_eq!(&ray[4..8], &[0.0, -2.0, -2.0, 0.0]);
        assert_eq!(*record, vec![child, 0, composed[0][0], body]);
    }
}
