//! `BSHavok/hkpackednitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{load_vec, maxps, minps, store_vec, zip_with, Vec4};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00ca40f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00ca_3b90`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00ca40f0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00ca_3b90, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00ca3980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00ca3570`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x30 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn hk_packed_ni_tri_strips_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ca_3570;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

layout! {
    /// `hkPackedNiTriStripsData` (Xbox PDB), 0x30 bytes.
    pub struct HkPackedNiTriStripsData: 0x30 {
        /// `uiKeyCount` (Xbox PDB): the number of triangles (shape keys).
        0x08 uiKeyCount: u32,
        /// `uiVertCount` (Xbox PDB): the number of vertices.
        0x0c uiVertCount: u32,
        /// `usPartCount` (Xbox PDB): the number of sub-parts.
        0x10 usPartCount: u16,
        /// `paTriData` (Xbox PDB): `TriangleData*`, 8 bytes each (`us1`, `us2`, `us3`, unused).
        0x14 paTriData: u32,
        /// `paVertData` (Xbox PDB): `NiPoint3*`, 12 bytes each.
        0x18 paVertData: u32,
        /// `paSubPartData` (Xbox PDB): `SubPartData*`, 12 bytes each (`uiFilter`, `uiNumVerts`, `uiMaterial`).
        0x24 paSubPartData: u32,
    }

    /// `hkPackedNiTriStripsShape` (Xbox PDB), 0xb0 bytes. Its `hkpShapeContainer` sub-object
    /// (`hkpShapeCollection`'s second base) sits at +0x10: the container methods get that address
    /// as `this`.
    pub struct HkPackedNiTriStripsShape: 0xb0 {
        /// `m_disableWelding` (Xbox PDB, `hkBool`).
        0x14 m_disableWelding: u8,
        /// `fAabbScale` (Xbox PDB).
        0x80 fAabbScale: f32,
        /// `spTriStripsData` (Xbox PDB): `NiPointer<hkPackedNiTriStripsData>`.
        0x84 spTriStripsData: Ptr<HkPackedNiTriStripsData>,
        /// `fRadius` (Xbox PDB).
        0xa0 fRadius: f32,
    }
}

/// Offset of the `hkpShapeContainer` sub-object inside `hkPackedNiTriStripsShape`.
const CONTAINER_OFFSET: u32 = 0x10;
/// `AabbTransform` (Xbox PDB, `hkTransform`), `WorldAabb` (Xbox PDB, `hkAabb`: min then max)
/// and `vScale` (Xbox PDB, `hkVector4`).
const AABB_TRANSFORM: u32 = 0x20;
const WORLD_AABB: u32 = 0x60;
const SCALE: u32 = 0x90;

/// `hkTransform::isApproximatelyEqual` (`00cb2160`, `this` = the transform; stack
/// `(result, other, tolerance)`; answers the `result` address, holding an `hkBool`).
const TRANSFORM_IS_APPROXIMATELY_EQUAL: u32 = 0x00cb_2160;
/// `hkVector4::setTransformedPos` (`00c7f640`, `this` = out; `(transform, vector)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// The tolerance (`0.001f`) `getAabbImpl` compares the transforms with.
const TRANSFORM_TOLERANCE: u32 = 0x0101_7d00;
/// `FLT_MAX` as a `double`: a `WorldAabb` minimum equal to it marks the cache as unset.
const AABB_UNSET_LIMIT: u32 = 0x0104_e138;
/// The lane fill values of an empty box: the first three lanes of the minimum, of the maximum,
/// and the last lane of both.
const EMPTY_MIN_FILL: u32 = 0x0104_da48;
const EMPTY_MAX_FILL: u32 = 0x0104_e148;
const EMPTY_W_FILL: u32 = 0x0101_1d78;

/// The vtable of the triangle shape (`hkNormalTriangleShape` by its RTTI) that `getChildShape`
/// builds in the caller's buffer.
const TRIANGLE_SHAPE_VTABLE: u32 = 0x010c_74ec;

// Translated from 00ca3550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getType` (`hkpShape` slot `0x34`, Xbox PDB): the constant `0x17`.
/// The linker folded this body with another class's slot (`010c75f8`).
pub fn hk_packed_ni_tri_strips_shape_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    0x17
}

// Translated from 00ca35d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getNextKey` (`hkpShapeContainer` slot `0x0c`, Xbox PDB); `this` is
/// the container sub-object. A shape key holds the triangle index in the low 24 bits and the
/// sub-part in the top byte. The next key is the next triangle, or `0xffffffff` past the last
/// one; its sub-part moves on when the next triangle's first vertex index (`us1`) is not below
/// the `uiNumVerts` of the sub-parts `0..=part` added up.
pub fn hk_packed_ni_tri_strips_shape_get_next_key(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let next = (key & 0x00ff_ffff).wrapping_add(1);
    let mut part = key >> 24;
    if next >= e.get(data, HkPackedNiTriStripsData::uiKeyCount) {
        return 0xffff_ffff;
    }
    let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
    // Pairs of sub-parts are added two at a time (even and odd running sums), then a last
    // single one when `part` is even.
    let mut even_sum = 0u32;
    let mut odd_sum = 0u32;
    let mut pair_end = 0u32;
    if (part as i32).wrapping_add(1) >= 2 {
        let pairs = part.wrapping_add(1) >> 1;
        for pair in 0..pairs {
            even_sum = even_sum.wrapping_add(e.mem.u32(sub_parts + 0x18 * pair + 4));
            odd_sum = odd_sum.wrapping_add(e.mem.u32(sub_parts + 0x18 * pair + 0x10));
        }
        pair_end = pairs * 2;
    }
    let last = if (pair_end as i32) <= (part as i32) {
        e.mem.u32(sub_parts + pair_end * 12 + 4)
    } else {
        0
    };
    let triangles = e.get(data, HkPackedNiTriStripsData::paTriData);
    let first_vertex = u32::from(e.mem.u16(triangles + next * 8));
    let total = odd_sum.wrapping_add(even_sum).wrapping_add(last);
    if first_vertex >= total {
        part = part.wrapping_add(1);
    }
    (next & 0x00ff_ffff) | (part << 24)
}

// Translated from 00ca3670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::getCollisionFilterInfo` (`hkpShapeContainer` slot `0x10`, Xbox
/// PDB); `this` is the container sub-object. The filter (`uiFilter`) of the key's sub-part
/// (the key's top byte), or 0 when that is not below `usPartCount`.
pub fn hk_packed_ni_tri_strips_shape_get_collision_filter_info(
    e: &mut Engine,
    this: Ptr,
    key: u32,
) -> u32 {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let part = key >> 24;
    if part >= u32::from(e.get(data, HkPackedNiTriStripsData::usPartCount)) {
        return 0;
    }
    let sub_parts = e.get(data, HkPackedNiTriStripsData::paSubPartData);
    e.mem.u32(sub_parts + part * 12)
}

// Translated from 00ca39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the triangle shape (`hkNormalTriangleShape` by its RTTI; `hkpTriangleShape`
/// layout, Xbox PDB) that this unit builds for `getChildShape`: radius at `+0x10`, reference
/// count 1, user data 0, shape type 3, welding info 0, welding type 6, not extruded, extrusion
/// zeroed, vtable `010c74ec`. Answers `this`.
pub fn hk_normal_triangle_shape_construct(e: &mut Engine, this: Ptr, radius: f32) -> Ptr {
    let me = this.addr();
    e.mem.set_u32(me + 0x10, x87_quiet_bits(radius.to_bits()));
    e.mem.set_u16(me + 6, 1);
    e.mem.set_u32(me + 8, 0);
    e.mem.set_u32(me + 0xc, 3);
    e.mem.set_u16(me + 0x14, 0);
    e.mem.set_u8(me + 0x16, 6);
    for lane in 0..4 {
        e.mem.set_u32(me + 0x50 + 4 * lane, 0);
    }
    e.mem.set_u8(me + 0x17, 0);
    e.mem.set_u32(me, TRIANGLE_SHAPE_VTABLE);
    this
}

// Translated from 00ca3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkPackedNiTriStripsShape::isWeldingEnabled` (`hkpShapeContainer` slot `0x18`, Xbox PDB);
/// `this` is the container sub-object, so the byte at `this + 4` is `m_disableWelding`. True when
/// it is 0. The linker folded this body with 12 other vtable slots.
pub fn hk_packed_ni_tri_strips_shape_is_welding_enabled(e: &mut Engine, this: Ptr) -> bool {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    e.get(shape, HkPackedNiTriStripsShape::m_disableWelding) == 0
}

// Translated from 00ca3c40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca3c40 - /GS stack-cookie check (00ec408c) is not modeled
/// `hkPackedNiTriStripsShape::getChildShape` (`hkpShapeContainer` slot `0x14`, Xbox PDB);
/// `this` is the container sub-object; stack `(key, buffer)`. Answers 0 when the key's triangle
/// index (low 24 bits) is not below `uiKeyCount`. Otherwise it constructs a triangle shape in
/// `buffer` (`00ca39a0`, radius `fRadius`), copies the triangle's three vertices (`NiPoint3`s
/// through the x87, last lane 0) into `+0x20`, `+0x30` and `+0x40`, and, when the first lane of
/// `vScale` is not 1.0 (NaN included), multiplies each vertex lane by lane by `vScale`.
/// Answers `buffer`. A null `buffer` is not checked by the exe (it would write at address 0x20).
pub fn hk_packed_ni_tri_strips_shape_get_child_shape(
    e: &mut Engine,
    this: Ptr,
    key: u32,
    buffer: Ptr,
) -> Ptr {
    let shape = Ptr::<HkPackedNiTriStripsShape>::new(this.addr().wrapping_sub(CONTAINER_OFFSET));
    let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
    let index = key & 0x00ff_ffff;
    if e.get(data, HkPackedNiTriStripsData::uiKeyCount) <= index {
        return Ptr::new(0);
    }
    let triangle = e.get(data, HkPackedNiTriStripsData::paTriData) + index * 8;
    let out = if buffer.addr() == 0 {
        0
    } else {
        let radius = e.get(shape, HkPackedNiTriStripsShape::fRadius);
        hk_normal_triangle_shape_construct(e, buffer, x87_quiet(radius)).addr()
    };
    for corner in 0..3u32 {
        let vertex_index = u32::from(e.mem.u16(triangle + 2 * corner));
        let data = e.get(shape, HkPackedNiTriStripsShape::spTriStripsData);
        let vertex = e.get(data, HkPackedNiTriStripsData::paVertData) + vertex_index * 12;
        for lane in 0..3 {
            let bits = x87_quiet_bits(e.mem.u32(vertex + 4 * lane));
            e.mem.set_u32(out + 0x20 + 0x10 * corner + 4 * lane, bits);
        }
        e.mem.set_u32(out + 0x20 + 0x10 * corner + 0xc, 0);
    }
    let scale = load_vec(e, shape.addr() + SCALE);
    if scale[0] != 1.0 {
        for corner in 0..3u32 {
            let at = out + 0x20 + 0x10 * corner;
            let vertex = load_vec(e, at);
            store_vec(e, at, zip_with(vertex, scale, |v, s| v * s));
        }
    }
    Ptr::new(out)
}

// Translated from 00ca37e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca37e0 - /GS stack-cookie check (00ec408c) is not modeled
/// `hkPackedNiTriStripsShape::getAabbImpl` (`hkpShape` slot `0x1c`, Xbox PDB); stack
/// `(transform, tolerance, out)`, `out` an `hkAabb` (min then max). When the cached `WorldAabb`
/// minimum's first lane is not `FLT_MAX` (compared as a `double`, NaN counts as not equal), the
/// stored `AabbTransform` is approximately equal to `transform` (`hkTransform::isApproximatelyEqual`
/// `00cb2160`, tolerance `0.001`) and `vScale`'s first lane equals `fAabbScale` (x87 compare, NaN
/// fails), the cached box is copied. Otherwise the box starts empty (minimum `+max`, maximum `-max`
/// fills, last lane 0) and takes the lane-wise `MINPS`/`MAXPS` of every vertex of the data
/// (loaded through the x87, last lane 0, multiplied by `vScale`, moved by `transform` through
/// `hkVector4::setTransformedPos` `00c7f640`). Finally the box grows by `fRadius + tolerance` (x87,
/// rounded to `float`) on every lane.
pub fn hk_packed_ni_tri_strips_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr<HkPackedNiTriStripsShape>,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let me = this.addr();
    let out = out.addr();
    e.with_stack(0x30, |e, scratch| {
        let vertex = scratch.addr();
        let moved = vertex + 0x10;
        let flag = vertex + 0x20;
        let unset_limit = e.mem.f64(AABB_UNSET_LIMIT);
        let mut cached = false;
        if f64::from(e.mem.f32(me + WORLD_AABB)) != unset_limit {
            let transform_tolerance = x87_quiet_bits(e.mem.u32(TRANSFORM_TOLERANCE));
            let result = e
                .call(
                    TRANSFORM_IS_APPROXIMATELY_EQUAL,
                    &args![me + AABB_TRANSFORM, flag, transform, transform_tolerance],
                )
                .u32();
            if e.mem.u8(result) != 0 {
                let scale = f64::from(e.mem.f32(me + SCALE));
                let aabb_scale = f64::from(e.get(this, HkPackedNiTriStripsShape::fAabbScale));
                cached = scale == aabb_scale;
            }
        }
        if cached {
            for lane in 0..8 {
                let word = e.mem.u32(me + WORLD_AABB + 4 * lane);
                e.mem.set_u32(out + 4 * lane, word);
            }
        } else {
            let (min_fill, max_fill, w_fill) = (
                e.mem.u32(EMPTY_MIN_FILL),
                e.mem.u32(EMPTY_MAX_FILL),
                e.mem.u32(EMPTY_W_FILL),
            );
            for (lane, word) in [min_fill, min_fill, min_fill, w_fill]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(out + 4 * lane as u32, word);
            }
            for (lane, word) in [max_fill, max_fill, max_fill, w_fill]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(out + 0x10 + 4 * lane as u32, word);
            }
            let scale = load_vec(e, me + SCALE);
            let mut data = e.get(this, HkPackedNiTriStripsShape::spTriStripsData);
            if e.get(data, HkPackedNiTriStripsData::uiVertCount) != 0 {
                let mut offset = 0u32;
                let mut done = 0u32;
                loop {
                    let source = e.get(data, HkPackedNiTriStripsData::paVertData) + offset;
                    let raw: Vec4 = [
                        x87_quiet(e.mem.f32(source)),
                        x87_quiet(e.mem.f32(source + 4)),
                        x87_quiet(e.mem.f32(source + 8)),
                        0.0,
                    ];
                    store_vec(e, vertex, zip_with(scale, raw, |s, v| s * v));
                    e.call(SET_TRANSFORMED_POS, &args![moved, transform, vertex]);
                    let corner = load_vec(e, moved);
                    let low = zip_with(load_vec(e, out), corner, minps);
                    store_vec(e, out, low);
                    let high = zip_with(load_vec(e, out + 0x10), corner, maxps);
                    store_vec(e, out + 0x10, high);
                    offset = offset.wrapping_add(0xc);
                    data = e.get(this, HkPackedNiTriStripsShape::spTriStripsData);
                    done += 1;
                    if done >= e.get(data, HkPackedNiTriStripsData::uiVertCount) {
                        break;
                    }
                }
            }
        }
        let radius = x87_quiet(e.get(this, HkPackedNiTriStripsShape::fRadius));
        let margin = (f64::from(radius) + f64::from(x87_quiet(tolerance))) as f32;
        let low = load_vec(e, out).map(|lane| lane - margin);
        store_vec(e, out, low);
        let high = load_vec(e, out + 0x10).map(|lane| lane + margin);
        store_vec(e, out + 0x10, high);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ca_40f0, deleting_destructor_00ca40f0(Ptr, u8) -> Ptr),
        entry!(0x00ca3980, hk_packed_ni_tri_strips_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00ca3550, hk_packed_ni_tri_strips_shape_get_type(Ptr) -> u32),
        entry!(0x00ca35d0, hk_packed_ni_tri_strips_shape_get_next_key(Ptr, u32) -> u32),
        entry!(0x00ca3670, hk_packed_ni_tri_strips_shape_get_collision_filter_info(Ptr, u32) -> u32),
        entry!(
            0x00ca37e0,
            hk_packed_ni_tri_strips_shape_get_aabb_impl(
                Ptr<HkPackedNiTriStripsShape>,
                Ptr,
                f32,
                Ptr,
            )
        ),
        entry!(0x00ca39a0, hk_normal_triangle_shape_construct(Ptr, f32) -> Ptr),
        entry!(0x00ca3b10, hk_packed_ni_tri_strips_shape_is_welding_enabled(Ptr) -> bool),
        entry!(0x00ca3c40, hk_packed_ni_tri_strips_shape_get_child_shape(Ptr, u32, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00ca40f0_frees_only_when_bit_zero_is_set() {
        check(0x00ca_40f0, 0x00ca_3b90);
    }

    #[test]
    fn hk_packed_ni_tri_strips_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ca_3570, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ca3980, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ca3980, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x30]]);
        assert_eq!(calls_to(&e, 0x00ca_3570), vec![vec![this]; 2]);
    }

    /// A shape (0xb0 bytes) pointing at a data block (0x30 bytes), both zeroed.
    fn shape_with_data(e: &mut Engine) -> (u32, u32) {
        let shape = e.mem.alloc(0xb0);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(shape + 0x84, data);
        (shape, data)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn get_type_is_0x17() {
        let mut e = engine();
        assert_eq!(e.call(0x00ca_3550, &args![0x1234u32]).u32(), 0x17);
    }

    #[test]
    fn is_welding_enabled_reads_the_disable_byte_of_the_container() {
        let mut e = engine();
        let (shape, _) = shape_with_data(&mut e);
        let container = shape + 0x10;
        assert!(e.call(0x00ca_3b10, &args![container]).bool());
        e.mem.set_u8(shape + 0x14, 1);
        assert!(!e.call(0x00ca_3b10, &args![container]).bool());
    }

    #[test]
    fn get_collision_filter_info_picks_the_sub_part_or_zero() {
        let mut e = engine();
        let (shape, data) = shape_with_data(&mut e);
        let sub_parts = e.mem.alloc(0x24);
        e.mem.set_u16(data + 0x10, 2);
        e.mem.set_u32(data + 0x24, sub_parts);
        e.mem.set_u32(sub_parts, 0x11);
        e.mem.set_u32(sub_parts + 0xc, 0x22);
        let container = shape + 0x10;
        let filter = |e: &mut Engine, key: u32| e.call(0x00ca_3670, &args![container, key]).u32();
        assert_eq!(filter(&mut e, 0x0000_0007), 0x11);
        assert_eq!(filter(&mut e, 0x0100_0005), 0x22);
        assert_eq!(filter(&mut e, 0x0200_0000), 0);
        assert_eq!(filter(&mut e, 0xff00_0000), 0);
    }

    #[test]
    fn get_next_key_advances_the_sub_part_past_its_vertex_count() {
        let mut e = engine();
        let (shape, data) = shape_with_data(&mut e);
        let sub_parts = e.mem.alloc(0x24);
        let triangles = e.mem.alloc(0x50);
        e.mem.set_u32(data + 8, 10);
        e.mem.set_u16(data + 0x10, 3);
        e.mem.set_u32(data + 0x14, triangles);
        e.mem.set_u32(data + 0x24, sub_parts);
        // uiNumVerts of the three sub-parts: 3, 4 and 5.
        for (part, count) in [3u32, 4, 5].into_iter().enumerate() {
            e.mem.set_u32(sub_parts + 12 * part as u32 + 4, count);
        }
        let container = shape + 0x10;
        let next = |e: &mut Engine, key: u32, first_vertex: u16| {
            e.mem
                .set_u16(triangles + 8 * ((key & 0xff_ffff) + 1), first_vertex);
            e.call(0x00ca_35d0, &args![container, key]).u32()
        };
        // Sub-part 0: limit 3.
        assert_eq!(next(&mut e, 0, 2), 0x0000_0001);
        assert_eq!(next(&mut e, 0, 3), 0x0100_0001);
        // Sub-part 1: limit 3 + 4.
        assert_eq!(next(&mut e, 0x0100_0004, 6), 0x0100_0005);
        assert_eq!(next(&mut e, 0x0100_0004, 7), 0x0200_0005);
        // Sub-part 2 (a pair and a single one): limit 3 + 4 + 5.
        assert_eq!(next(&mut e, 0x0200_0000, 11), 0x0200_0001);
        assert_eq!(next(&mut e, 0x0200_0000, 12), 0x0300_0001);
        // Past the last triangle.
        assert_eq!(
            e.call(0x00ca_35d0, &args![container, 9u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(
            e.call(0x00ca_35d0, &args![container, 0x0500_0009u32]).u32(),
            0xffff_ffff
        );
    }

    #[test]
    fn triangle_constructor_fills_the_header_and_zeroes_the_extrusion() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(this + 4 * word, 0xffff_ffff);
        }
        let got = e.call(0x00ca_39a0, &args![this, 0.5f32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), 0x010c_74ec);
        assert_eq!(e.mem.u16(this + 6), 1);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.f32(this + 0x10), 0.5);
        assert_eq!(e.mem.u16(this + 0x14), 0);
        assert_eq!(e.mem.u8(this + 0x16), 6);
        assert_eq!(e.mem.u8(this + 0x17), 0);
        for lane in 0..4 {
            assert_eq!(e.mem.u32(this + 0x50 + 4 * lane), 0);
        }
        // The vertices and the words in between are not touched.
        assert_eq!(e.mem.u32(this + 0x20), 0xffff_ffff);
        // A signalling NaN radius is quieted by the x87 copy.
        e.call(0x00ca_39a0, &args![this, f32::from_bits(0x7f80_0001)]);
        assert_eq!(e.mem.u32(this + 0x10), 0x7fc0_0001);
    }

    /// A shape with two triangles, three vertices, radius 0.25 and the given scale.
    fn shape_with_triangles(e: &mut Engine, scale: [f32; 4]) -> u32 {
        let (shape, data) = shape_with_data(e);
        let triangles = e.mem.alloc(0x10);
        let vertices = e.mem.alloc(0x24);
        e.mem.set_u32(data + 8, 2);
        e.mem.set_u32(data + 0x0c, 3);
        e.mem.set_u32(data + 0x14, triangles);
        e.mem.set_u32(data + 0x18, vertices);
        for (slot, vertex) in [0u16, 1, 2, 2, 0, 1].into_iter().enumerate() {
            e.mem
                .set_u16(triangles + 2 * slot as u32 + 2 * (slot as u32 / 3), vertex);
        }
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vertices + 4 * lane as u32, value);
        }
        e.mem.set_f32(shape + 0xa0, 0.25);
        for (lane, value) in scale.into_iter().enumerate() {
            e.mem.set_f32(shape + 0x90 + 4 * lane as u32, value);
        }
        shape
    }

    fn read_vertices(e: &Engine, buffer: u32) -> Vec<f32> {
        (0..3u32)
            .flat_map(|corner| (0..4u32).map(move |lane| (corner, lane)))
            .map(|(corner, lane)| e.mem.f32(buffer + 0x20 + 0x10 * corner + 4 * lane))
            .collect()
    }

    #[test]
    fn get_child_shape_builds_the_triangle_and_scales_only_when_needed() {
        let mut e = engine();
        let shape = shape_with_triangles(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let container = shape + 0x10;
        let buffer = e.mem.alloc(0x60);
        for word in 0..0x18 {
            e.mem.set_u32(buffer + 4 * word, 0xffff_ffff);
        }
        let got = e.call(0x00ca_3c40, &args![container, 0u32, buffer]).u32();
        assert_eq!(got, buffer);
        assert_eq!(e.mem.u32(buffer), 0x010c_74ec);
        assert_eq!(e.mem.f32(buffer + 0x10), 0.25);
        assert_eq!(
            read_vertices(&e, buffer),
            [1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0, 7.0, 8.0, 9.0, 0.0]
        );
        // The second triangle (vertices 2, 0, 1), the sub-part byte of the key is ignored.
        e.call(0x00ca_3c40, &args![container, 0x0700_0001u32, buffer]);
        assert_eq!(
            read_vertices(&e, buffer),
            [7.0, 8.0, 9.0, 0.0, 1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0]
        );
        // A scale whose first lane is not 1 multiplies all four lanes of every vertex.
        e.mem.set_f32(shape + 0x90, 2.0);
        e.mem.set_f32(shape + 0x94, 3.0);
        e.mem.set_f32(shape + 0x98, 4.0);
        e.mem.set_f32(shape + 0x9c, 5.0);
        e.call(0x00ca_3c40, &args![container, 0u32, buffer]);
        assert_eq!(
            read_vertices(&e, buffer),
            [2.0, 6.0, 12.0, 0.0, 8.0, 15.0, 24.0, 0.0, 14.0, 24.0, 36.0, 0.0]
        );
    }

    #[test]
    fn get_child_shape_answers_null_past_the_last_triangle() {
        let mut e = engine();
        let shape = shape_with_triangles(&mut e, [1.0, 1.0, 1.0, 1.0]);
        let buffer = e.mem.alloc(0x60);
        e.mem.set_u32(buffer, 0xdead_beef);
        let got = e
            .call(0x00ca_3c40, &args![shape + 0x10, 2u32, buffer])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u32(buffer), 0xdead_beef);
    }

    const FLT_MAX_FILL: u32 = 0x7f7f_ffee;

    /// Constants the AABB function reads, the two callee doubles and a shape with a cached box
    /// `(1, 2, 3, 4)` to `(5, 6, 7, 8)`, `AabbTransform` scale 2 and the given vertices.
    fn aabb_setup(e: &mut Engine, vertices: &[[f32; 3]], flag: u8) -> (u32, u32) {
        e.call_log = Some(vec![]);
        e.mem.set_f64(0x0104_e138, f64::from(f32::MAX));
        e.mem.set_u32(0x0101_7d00, 0x3a83_126f);
        e.mem.set_u32(0x0104_da48, FLT_MAX_FILL);
        e.mem.set_u32(0x0104_e148, 0xff7f_ffee);
        e.mem.set_u32(0x0101_1d78, 0);
        e.register_double(0x00cb_2160, move |e, a| {
            e.mem.set_u8(a[1], flag);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        // The "transform" of the test moves every point by 100 along x.
        e.register(0x00c7_f640, |e, a| {
            for lane in 0..4 {
                let value = e.mem.f32(a[2] + 4 * lane);
                let moved = if lane == 0 { value + 100.0 } else { value };
                e.mem.set_f32(a[0] + 4 * lane, moved);
            }
            Ret::default()
        });
        let (shape, data) = shape_with_data(e);
        let list = e.mem.alloc(0x40);
        for (index, vertex) in vertices.iter().enumerate() {
            for (lane, value) in vertex.iter().enumerate() {
                e.mem
                    .set_f32(list + 12 * index as u32 + 4 * lane as u32, *value);
            }
        }
        e.mem.set_u32(data + 0x0c, vertices.len() as u32);
        e.mem.set_u32(data + 0x18, list);
        for (lane, value) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(shape + 0x60 + 4 * lane as u32, value);
        }
        e.mem.set_f32(shape + 0x80, 2.0);
        for lane in 0..4 {
            e.mem.set_f32(shape + 0x90 + 4 * lane, 2.0);
        }
        e.mem.set_f32(shape + 0xa0, 0.25);
        let transform = e.mem.alloc(0x40);
        (shape, transform)
    }

    fn read_box(e: &Engine, out: u32) -> [f32; 8] {
        std::array::from_fn(|lane| e.mem.f32(out + 4 * lane as u32))
    }

    #[test]
    fn get_aabb_copies_the_cached_box_when_transform_and_scale_match() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert_eq!(
            read_box(&e, out),
            [0.25, 1.25, 2.25, 3.25, 5.75, 6.75, 7.75, 8.75]
        );
        let checks = calls_to(&e, 0x00cb_2160);
        assert_eq!(checks.len(), 1);
        assert_eq!(checks[0][0], shape + 0x20);
        assert_eq!(checks[0][2..], [transform, 0x3a83_126f]);
        assert!(calls_to(&e, 0x00c7_f640).is_empty());
    }

    #[test]
    fn get_aabb_rebuilds_from_the_vertices_when_the_transforms_differ() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0], [-4.0, 5.0, 0.0]], 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        // Scaled by 2 and moved by 100 along x: (102, 4, 6, 0) and (92, 10, 0, 0).
        assert_eq!(
            read_box(&e, out),
            [91.25, 3.25, -0.75, -0.75, 102.75, 10.75, 6.75, 0.75]
        );
        assert_eq!(calls_to(&e, 0x00c7_f640).len(), 2);
    }

    #[test]
    fn get_aabb_skips_the_cache_check_when_the_minimum_is_float_max() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        e.mem.set_f32(shape + 0x60, f32::MAX);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert!(calls_to(&e, 0x00cb_2160).is_empty());
        assert_eq!(
            read_box(&e, out),
            [101.25, 3.25, 5.25, -0.75, 102.75, 4.75, 6.75, 0.75]
        );
    }

    #[test]
    fn get_aabb_rebuilds_when_the_scale_differs_from_the_cached_one() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[[1.0, 2.0, 3.0]], 1);
        e.mem.set_f32(shape + 0x80, 3.0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        assert_eq!(calls_to(&e, 0x00c7_f640).len(), 1);
        assert_eq!(read_box(&e, out)[0], 101.25);
    }

    #[test]
    fn get_aabb_without_vertices_keeps_the_empty_fills_grown_by_the_margin() {
        let mut e = engine();
        let (shape, transform) = aabb_setup(&mut e, &[], 0);
        let out = e.mem.alloc(0x20);
        e.call(0x00ca_37e0, &args![shape, transform, 0.5f32, out]);
        let got = read_box(&e, out);
        assert_eq!(got[0].to_bits(), FLT_MAX_FILL);
        assert_eq!(got[3], -0.75);
        assert_eq!(got[4].to_bits(), 0xff7f_ffee);
        assert_eq!(got[7], 0.75);
        assert!(calls_to(&e, 0x00c7_f640).is_empty());
    }
}
