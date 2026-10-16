//! `Havok SDK/hkplistshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpListShape` (Xbox PDB, RTTI `.?AVhkpListShape@@` confirmed at the vtable `010cc3d8`) is
//! a shape collection of child shapes: `m_childInfo` (an `hkArray` of 0x10-byte
//! `ChildInfo`, the child shape pointer first) at +0x18, `m_aabbHalfExtents` at +0x30 and
//! `m_aabbCenter` at +0x40 (offsets agree with the Xbox PDB). A shape's `getAabbImpl` is
//! vtable slot `0x1c`: `(transform, tolerance, out aabb)`, the aabb being two four-float
//! vectors, minimum then maximum.
//!
//! SSE code computes lane by lane in `f32`; `MINPS`/`MAXPS` answer the second operand when
//! the comparison is false or unordered, as `minimum` and `maximum` here do.

#[allow(unused_imports)]
use crate::prelude::*;

/// The identity transform global `getAabb` of the children is called with when the list
/// shape recomputes its extents (`01268370`).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
/// The four-lane vector of `0.5` (`010c71d0`).
const HALF_VECTOR: u32 = 0x010c_71d0;
/// The four-lane vector the extents are compared against (`01267e30`).
const EXTENT_THRESHOLD: u32 = 0x0126_7e30;
/// `float` the minimum corner starts from (`010cc3b0`, `0x7effffee`); the maximum starts
/// from the same value with the sign bits of `SIGN_MASK` flipped.
const START_MINIMUM: u32 = 0x010c_c3b0;
/// Sign mask of four lanes (`010c44d0`).
const SIGN_MASK: u32 = 0x010c_44d0;

layout! {
    /// `hkpListShape` (Xbox PDB), 0x70 bytes: the fields the translations use.
    pub struct HkpListShape: 0x70 {
        /// `m_childInfo.m_data` (Xbox PDB): the `ChildInfo` array, 0x10 bytes per entry.
        0x18 m_childInfo: Ptr,
        /// `m_childInfo.m_size` (Xbox PDB).
        0x1c m_childInfoSize: i32,
        /// `m_aabbHalfExtents` (Xbox PDB): `hkVector4`.
        0x30 m_aabbHalfExtents: Inline<[f32; 4]>,
        /// `m_aabbCenter` (Xbox PDB): `hkVector4`.
        0x40 m_aabbCenter: Inline<[f32; 4]>,
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cf4fb0, fn_00cf4fb0(Ptr<HkpListShape>, Ptr)),
        entry!(
            0x00cf5420,
            hkp_list_shape_recalc_aabb_extents(Ptr<HkpListShape>)
        ),
        entry!(
            0x00cf5590,
            hkp_list_shape_get_aabb_impl(Ptr<HkpListShape>, Ptr, f32, Ptr)
        ),
    ]
}

fn lanes(e: &Engine, at: u32) -> [f32; 4] {
    [0, 1, 2, 3].map(|i| e.mem.f32(at + 4 * i))
}

fn set_lanes(e: &mut Engine, at: u32, values: [f32; 4]) {
    for (i, value) in values.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, value);
    }
}

/// `MINPS dst, src` lane: `dst` when `dst < src`, otherwise `src`.
fn minimum(dst: f32, src: f32) -> f32 {
    if dst < src {
        dst
    } else {
        src
    }
}

/// `MAXPS dst, src` lane: `dst` when `dst > src`, otherwise `src`.
fn maximum(dst: f32, src: f32) -> f32 {
    if dst > src {
        dst
    } else {
        src
    }
}

/// The child shape of entry `index`: the first word of the 0x10-byte `ChildInfo`.
fn child_shape(e: &Engine, this: Ptr<HkpListShape>, index: i32) -> u32 {
    let data = e.get(this, HkpListShape::m_childInfo);
    e.mem
        .u32(data.addr().wrapping_add((index as u32).wrapping_mul(0x10)))
}

/// Merges the aabb at `from` into the one at `into`: lane-wise minimum of the minimum
/// corners and maximum of the maximum corners (`MINPS`, `MAXPS` with the accumulated one as
/// the first operand).
fn merge_aabb(e: &mut Engine, into: u32, from: u32) {
    let minimum_corner = lanes(e, into);
    let new_minimum = lanes(e, from);
    set_lanes(
        e,
        into,
        [0, 1, 2, 3].map(|i| minimum(minimum_corner[i], new_minimum[i])),
    );
    let maximum_corner = lanes(e, into + 0x10);
    let new_maximum = lanes(e, from + 0x10);
    set_lanes(
        e,
        into + 0x10,
        [0, 1, 2, 3].map(|i| maximum(maximum_corner[i], new_maximum[i])),
    );
}

// Translated from 00cf4fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; `RET 4`: the bounding box of all children at the identity
/// transform with tolerance 0 (first child's `getAabbImpl` straight into `aabb`, the others
/// merged in), after which `m_aabbCenter` is `(max + min) * 0.5` and `m_aabbHalfExtents` is
/// `(max - min) * 0.5`. The first child is read even when there are none, as the exe does.
pub fn fn_00cf4fb0(e: &mut Engine, this: Ptr<HkpListShape>, aabb: Ptr) {
    let first = child_shape(e, this, 0);
    e.vcall(first, 0x1c, &args![IDENTITY_TRANSFORM, 0u32, aabb]);
    let count = e.get(this, HkpListShape::m_childInfoSize);
    if count > 1 {
        e.with_stack(0x20, |e, scratch| {
            for index in 1..count {
                let child = child_shape(e, this, index);
                e.vcall(child, 0x1c, &args![IDENTITY_TRANSFORM, 0u32, scratch]);
                merge_aabb(e, aabb.addr(), scratch.addr());
            }
        });
    }
    let minimum_corner = lanes(e, aabb.addr());
    let maximum_corner = lanes(e, aabb.addr() + 0x10);
    let half = lanes(e, HALF_VECTOR);
    let center = [0, 1, 2, 3].map(|i| (maximum_corner[i] + minimum_corner[i]) * half[i]);
    set_lanes(e, this.addr() + 0x40, center);
    let extents = [0, 1, 2, 3].map(|i| (maximum_corner[i] - minimum_corner[i]) * half[i]);
    set_lanes(e, this.addr() + 0x30, extents);
}

// Translated from 00cf5420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::recalcAabbExtents` (Xbox PDB): recomputes the center and half extents
/// (`00cf4fb0`) into a scratch aabb that is thrown away.
pub fn hkp_list_shape_recalc_aabb_extents(e: &mut Engine, this: Ptr<HkpListShape>) {
    e.with_stack(0x20, |e, scratch| fn_00cf4fb0(e, this, scratch));
}

// Translated from 00cf5590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpListShape::getAabbImpl` (the Xbox PDB name of slot `0x1c` in the vtable `010cc3d8`,
/// where this body sits), `RET 0xc`: `(transform, tolerance, out aabb)`. Recomputes the
/// extents first when every half extent is at or below the threshold vector `01267e30`
/// (unordered counts as above), then starts the box inverted (minimum `0x7effffee`, maximum
/// its sign-flipped twin) and merges in the `getAabbImpl` of every child with the given
/// transform and tolerance (the tolerance passes through `FLD`/`FSTP`).
pub fn hkp_list_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkpListShape>,
    transform: Ptr,
    tolerance: f32,
    aabb: Ptr,
) {
    let extents = lanes(e, this.addr() + 0x30);
    let threshold = lanes(e, EXTENT_THRESHOLD);
    if (0..4).all(|i| extents[i] <= threshold[i]) {
        hkp_list_shape_recalc_aabb_extents(e, this);
    }
    let start = e.mem.u32(START_MINIMUM);
    for lane in 0..4 {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane);
        e.mem.set_u32(aabb.addr() + 4 * lane, start);
        e.mem.set_u32(aabb.addr() + 0x10 + 4 * lane, start ^ mask);
    }
    let count = e.get(this, HkpListShape::m_childInfoSize);
    if count > 0 {
        let tolerance_bits = x87_quiet_bits(tolerance.to_bits());
        e.with_stack(0x20, |e, scratch| {
            for index in 0..count {
                let child = child_shape(e, this, index);
                e.vcall(child, 0x1c, &args![transform, tolerance_bits, scratch]);
                merge_aabb(e, aabb.addr(), scratch.addr());
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHILD_VTABLE: u32 = 0x0200_0000;
    const CHILD_GET_AABB: u32 = 0x0200_0100;

    /// Children are objects `[vtable, min x4, max x4]`; their `getAabbImpl` copies that box
    /// out and logs the transform and tolerance words it was given.
    fn rig(boxes: &[([f32; 4], [f32; 4])]) -> (Engine, Ptr<HkpListShape>) {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.map(0x0100_0000, 0x0030_0000);
        e.put_vtable(CHILD_VTABLE, &[0, 0, 0, 0, 0, 0, 0, CHILD_GET_AABB]);
        e.register(CHILD_GET_AABB, |e, a| {
            for lane in 0..8 {
                let value = e.mem.u32(a[0] + 4 + 4 * lane);
                e.mem.set_u32(a[3] + 4 * lane, value);
            }
            Ret::default()
        });
        set_lanes(&mut e, HALF_VECTOR, [0.5; 4]);
        set_lanes(&mut e, EXTENT_THRESHOLD, [0.0; 4]);
        e.mem.set_u32(START_MINIMUM, 0x7eff_ffee);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        let list = e.new_object::<HkpListShape>();
        let infos = e.mem.alloc(0x10 * boxes.len() as u32 + 0x10);
        for (i, (low, high)) in boxes.iter().enumerate() {
            let child = e.mem.alloc(0x24);
            e.mem.set_u32(child, CHILD_VTABLE);
            set_lanes(&mut e, child + 4, *low);
            set_lanes(&mut e, child + 0x14, *high);
            e.mem.set_u32(infos + 0x10 * i as u32, child);
        }
        e.set(list, HkpListShape::m_childInfo, Ptr::new(infos));
        e.set(list, HkpListShape::m_childInfoSize, boxes.len() as i32);
        (e, list)
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const TWO_BOXES: [([f32; 4], [f32; 4]); 2] = [
        ([0.0, 0.0, 0.0, 0.0], [1.0, 2.0, 3.0, 0.0]),
        ([-1.0, 1.0, 0.5, 0.0], [2.0, 3.0, 1.0, 0.0]),
    ];

    #[test]
    fn extents_come_from_the_merged_children() {
        let (mut e, list) = rig(&TWO_BOXES);
        let out = e.mem.alloc(0x20);
        e.call(0x00cf_4fb0, &args![list, Ptr::<()>::new(out)]);
        assert_eq!(lanes(&e, out), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [2.0, 3.0, 3.0, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x40), [0.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
        let children = calls(&e, CHILD_GET_AABB);
        assert_eq!(children.len(), 2);
        // Identity transform, tolerance zero.
        assert_eq!(children[1][1..3], [IDENTITY_TRANSFORM, 0]);
    }

    #[test]
    fn recalc_runs_the_same_computation_on_a_scratch_box() {
        let (mut e, list) = rig(&TWO_BOXES);
        e.call(0x00cf_5420, &args![list]);
        assert_eq!(lanes(&e, list.addr() + 0x40), [0.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
    }

    #[test]
    fn get_aabb_recalculates_only_for_empty_extents() {
        let (mut e, list) = rig(&TWO_BOXES);
        let out = e.mem.alloc(0x20);
        let transform = e.mem.alloc(0x40);
        e.call(
            0x00cf_5590,
            &args![
                list,
                Ptr::<()>::new(transform),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        // The extents were all at or below the threshold, so they were recomputed.
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.5, 1.5, 1.5, 0.0]);
        assert_eq!(lanes(&e, out), [-1.0, 0.0, 0.0, 0.0]);
        assert_eq!(lanes(&e, out + 0x10), [2.0, 3.0, 3.0, 0.0]);
        let children = calls(&e, CHILD_GET_AABB);
        // Two calls for the recomputation, two for the box itself, with the given arguments.
        assert_eq!(children.len(), 4);
        assert_eq!(children[2][1..3], [transform, 0.25f32.to_bits()]);

        // With non-empty extents the recomputation is skipped.
        let (mut e, list) = rig(&TWO_BOXES);
        set_lanes(&mut e, list.addr() + 0x30, [1.0, 1.0, 1.0, 1.0]);
        e.call(
            0x00cf_5590,
            &args![
                list,
                Ptr::<()>::new(transform),
                0.25f32,
                Ptr::<()>::new(out)
            ],
        );
        assert_eq!(calls(&e, CHILD_GET_AABB).len(), 2);
        assert_eq!(lanes(&e, list.addr() + 0x30), [1.0; 4]);
    }

    #[test]
    fn get_aabb_of_an_empty_list_is_the_inverted_box() {
        let (mut e, list) = rig(&[]);
        set_lanes(&mut e, list.addr() + 0x30, [1.0; 4]);
        let out = e.mem.alloc(0x20);
        e.call(
            0x00cf_5590,
            &args![list, Ptr::<()>::new(0), 0.0f32, Ptr::<()>::new(out)],
        );
        assert_eq!(e.mem.u32(out), 0x7eff_ffee);
        assert_eq!(e.mem.u32(out + 0x1c), 0xfeff_ffee);
    }
}
