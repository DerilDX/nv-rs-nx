//! `Havok SDK/hkpextendedmeshshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpExtendedMeshShape` holds triangle subparts (0x60 bytes each) and convex-shape subparts
//! (0x40 bytes each); a shape key packs the subpart index in its high bits (the number of low
//! bits is `m_numBitsForSubpartIndex`) and the child index in the low bits, and bit 31 selects
//! the convex-shape subparts. The container functions (`getFirstKey`-style, vtable at
//! `010cfd20`) take the container embedded at shape `+0x10` as `this`. The field names below
//! follow the Havok SDK's structure; the PDB dump has no entry for the class, so they and the
//! container function roles (read off the vtable layout) are unconfirmed.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `hkpExtendedMeshShape` (Havok SDK; not in the PDB dump, offsets checked against the
    /// code below).
    pub struct HkpExtendedMeshShape: 0x100 {
        /// The number of low key bits that hold the child index.
        0xb0 m_numBitsForSubpartIndex: u32,
        /// The data of the triangle subpart array (0x60-byte entries).
        0xb4 m_trianglesSubpartsData: u32,
        /// Its element count.
        0xb8 m_trianglesSubpartsSize: u32,
        /// The data of the convex-shape subpart array (0x40-byte entries).
        0xc0 m_shapesSubpartsData: u32,
        /// Its element count.
        0xc4 m_shapesSubpartsSize: u32,
        /// The data of the per-triangle welding info (`u16` each).
        0xcc m_weldingInfoData: u32,
        /// Its element count.
        0xd0 m_weldingInfoSize: u32,
        /// The welding type byte copied into every triangle.
        0xd8 m_weldingType: u8,
        /// The collision filter info used for keys without a material.
        0xdc m_defaultCollisionFilterInfo: u32,
        /// The convex radius copied into every triangle.
        0xe4 m_radius: u32,
    }
}

/// `hkpTriangleUtil::isDegenerate` (Xbox PDB, cdecl: result byte pointer, three vertices,
/// tolerance; returns the result byte pointer).
const IS_DEGENERATE: u32 = 0x00ce_b8d0;
/// The degeneracy tolerance (a float).
const DEGENERACY_TOLERANCE: u32 = 0x011b_2e90;
/// The default convex radius of a triangle (a float).
const TRIANGLE_RADIUS: u32 = 0x011b_153c;
/// The vtable of a triangle shape, the vtable of a convex-transform shape and the mask that
/// clears the fourth lane of a vector (`ffffffff ffffffff ffffffff 00000000`).
const TRIANGLE_VTABLE: u32 = 0x010c_749c;
const CONVEX_TRANSFORM_VTABLE: u32 = 0x010d_0e5c;
const LANE_MASK: u32 = 0x010c_4c10;
/// `hkpConvexTransformShapeBase` constructor (Xbox PDB), `hkpConvexTransformShape`
/// constructor (Xbox PDB) and the routine that builds a transform from a rotation.
const CONVEX_TRANSFORM_BASE: u32 = 0x00c9_e340;
const CONVEX_TRANSFORM: u32 = 0x00d5_d7a0;
const SET_TRANSFORM: u32 = 0x00cb_2d90;
/// The shape type of a triangle shape.
const TRIANGLE_TYPE: u32 = 3;

/// The container (shape `+0x10`) seen as the shape.
fn shape_of(container: Ptr) -> Ptr<HkpExtendedMeshShape> {
    Ptr::new(container.addr().wrapping_sub(0x10))
}

/// Splits a key into its subpart index (above bit 31, below the child bits) and child index.
fn split_key(bits: u32, key: u32) -> (u32, u32) {
    (
        (key & 0x7fff_ffff) >> (32u32.wrapping_sub(bits) & 31),
        (0xffff_ffffu32 >> (bits & 31)) & key,
    )
}

// Translated from 00d57090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x60-byte triangle subpart from `source` to `dest`, four words at a time (cdecl;
/// the function is unnamed and its class unconfirmed).
pub fn fn_00d57090(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..24 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00d570d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x40-byte convex-shape subpart from `source` to `dest` (cdecl; the function is
/// unnamed and its class unconfirmed).
pub fn fn_00d570d0(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..16 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00d55530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::getMeshMaterial` (Xbox PDB): the address of the material of the
/// triangle or convex shape `key`, or 0 when there is none. The subpart is the embedded one
/// (`+0x20`) when there is exactly one triangle subpart, else entry `index` of the subpart
/// array (convex-shape subparts are not bounds checked). A subpart holds the material
/// base (`+4`), element type (`+1`: 1 is a byte, else a `u16`), the record stride (`+8`), the
/// material stride (`+2`, signed) and the first material (`+0xC`).
pub fn get_mesh_material(e: &mut Engine, this: Ptr<HkpExtendedMeshShape>, key: u32) -> u32 {
    let bits = e.get(this, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (index, child) = split_key(bits, key);
    let subpart = if (key as i32) < 0 {
        let array = e.get(this, HkpExtendedMeshShape::m_shapesSubpartsData);
        (index << 6).wrapping_add(array)
    } else {
        let count = e.get(this, HkpExtendedMeshShape::m_trianglesSubpartsSize);
        if count == 1 {
            this.addr() + 0x20
        } else if index >= count {
            return 0;
        } else {
            let array = e.get(this, HkpExtendedMeshShape::m_trianglesSubpartsData);
            index.wrapping_mul(0x60).wrapping_add(array)
        }
    };
    let materials = e.mem.u32(subpart + 4);
    let first = e.mem.u32(subpart + 0xc);
    let stride = i32::from(e.mem.u16(subpart + 2) as i16) as u32;
    if materials == 0 {
        return 0;
    }
    let record = u32::from(e.mem.u16(subpart + 8))
        .wrapping_mul(child)
        .wrapping_add(materials);
    let value = if e.mem.u8(subpart + 1) == 1 {
        u32::from(e.mem.u8(record))
    } else {
        u32::from(e.mem.u16(record))
    };
    value.wrapping_mul(stride).wrapping_add(first)
}

// Translated from 00d555d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's collision filter info of `key` (unconfirmed role): the first word of the
/// key's material (`getMeshMaterial` of the shape), or the shape's default collision filter
/// info when it has none.
pub fn fn_00d555d0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = shape_of(this);
    let material = e.call(0x00d5_5530, &args![shape, key]).u32();
    if material != 0 {
        e.mem.u32(material)
    } else {
        e.get(shape, HkpExtendedMeshShape::m_defaultCollisionFilterInfo)
    }
}

/// Whether the triangle shape `triangle` (its vertices at `+0x20`, `+0x30` and `+0x40`) is
/// degenerate; `result` is the scratch byte `isDegenerate` writes.
fn is_degenerate(e: &mut Engine, triangle: u32, result: u32) -> bool {
    let tolerance = x87_quiet_bits(e.mem.u32(DEGENERACY_TOLERANCE));
    let flag = e
        .call(
            IS_DEGENERATE,
            &args![
                result,
                triangle + 0x20,
                triangle + 0x30,
                triangle + 0x40,
                tolerance
            ],
        )
        .u32();
    e.mem.u8(flag) != 0
}

// Translated from 00d55b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's first shape key (unconfirmed role): 0xffffffff when there are no subparts,
/// else the key 0 of the first triangle subpart (or `0x80000000`, the first convex shape,
/// when there are no triangle subparts). A triangle child that is degenerate is skipped
/// through the container's slot `+0xC`.
pub fn fn_00d55b40(e: &mut Engine, this: Ptr) -> u32 {
    let shape = shape_of(this);
    let triangles = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize);
    let shapes = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize);
    if shapes.wrapping_add(triangles) == 0 {
        return 0xffff_ffff;
    }
    let key = if triangles != 0 { 0 } else { 0x8000_0000 };
    e.with_stack(0x210, |e, buffer| {
        let child = e.vcall(this.addr(), 0x14, &args![key, buffer]).u32();
        if e.mem.u32(child + 0xc) == TRIANGLE_TYPE && is_degenerate(e, child, buffer.addr() + 0x200)
        {
            e.vcall(this.addr(), 0xc, &args![key]).u32()
        } else {
            key
        }
    })
}

// Translated from 00d55bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The container's next shape key after `key` (unconfirmed role): the next child of the
/// subpart, else child 0 of the next subpart, else the first convex shape after the triangle
/// subparts, else 0xffffffff. A triangle child that is degenerate is skipped.
pub fn fn_00d55bd0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let shape = shape_of(this);
    let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (mut index, mut child) = split_key(bits, key);
    let mut sign = key & 0x8000_0000;
    e.with_stack(0x210, |e, buffer| loop {
        child = child.wrapping_add(1);
        if sign == 0 {
            let array = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsData);
            let count = e
                .mem
                .u32(array.wrapping_add(index.wrapping_mul(0x60)) + 0x14);
            if (child as i32) >= (count as i32) {
                index = index.wrapping_add(1);
                child = 0;
                if index >= e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize) {
                    if e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize) == 0 {
                        return 0xffff_ffff;
                    }
                    index = 0;
                    sign = 0x8000_0000;
                    child = 0xffff_ffff;
                    continue;
                }
            }
        } else {
            let array = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsData);
            let count = e.mem.u32(array.wrapping_add(index << 6) + 0x18);
            if (child as i32) >= (count as i32) {
                index = index.wrapping_add(1);
                if index >= e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsSize) {
                    return 0xffff_ffff;
                }
                child = 0;
            }
        }
        let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
        let next = (index << (32u32.wrapping_sub(bits) & 31)) | sign | child;
        let found = e.vcall(this.addr(), 0x14, &args![next, buffer]).u32();
        if e.mem.u32(found + 0xc) != TRIANGLE_TYPE
            || !is_degenerate(e, found, buffer.addr() + 0x200)
        {
            return next;
        }
    })
}

/// The three vertex indices of the triangle whose index record starts at `record`, read as
/// bytes (type 1), `u16` (2) or `u32` (3); `selector` is the
/// record's winding bit. Other index types give zeros.
fn read_triangle_indices(e: &Engine, index_type: i32, record: u32, selector: i32) -> [u32; 3] {
    let second = i32::from(selector.wrapping_add(1) as i16) as u32;
    let third = i32::from((selector ^ 1).wrapping_add(1) as i16) as u32;
    match index_type {
        1 => [
            u32::from(e.mem.u8(record)),
            u32::from(e.mem.u8(record.wrapping_add(second))),
            u32::from(e.mem.u8(record.wrapping_add(third))),
        ],
        2 => [
            u32::from(e.mem.u16(record)),
            u32::from(e.mem.u16(record.wrapping_add(second.wrapping_mul(2)))),
            u32::from(e.mem.u16(record.wrapping_add(third.wrapping_mul(2)))),
        ],
        3 => [
            e.mem.u32(record),
            e.mem.u32(record.wrapping_add(second.wrapping_mul(4))),
            e.mem.u32(record.wrapping_add(third.wrapping_mul(4))),
        ],
        _ => [0, 0, 0],
    }
}

/// The triangle part of `getChildShape`: fills the triangle shape `out` from the triangle
/// `child` of `subpart`.
fn fill_triangle(
    e: &mut Engine,
    shape: Ptr<HkpExtendedMeshShape>,
    subpart: u32,
    child: u32,
    out: u32,
) {
    let vertices = e.mem.u32(subpart + 0x18);
    let vertex_stride = e.mem.u32(subpart + 0x1c);
    let record = e
        .mem
        .u32(subpart + 0x44)
        .wrapping_mul(child)
        .wrapping_add(e.mem.u32(subpart + 0x40));
    let selector = i32::from(e.mem.u8(subpart + 0x49) as i8) & (child as i32);
    let index_type = i32::from(e.mem.u8(subpart + 0x48) as i8);
    let indices = read_triangle_indices(e, index_type, record, selector);
    // The triangle shape's constructor defaults, then the shape's own values.
    e.mem.set_u16(out + 6, 1);
    let radius = x87_quiet_bits(e.mem.u32(TRIANGLE_RADIUS));
    e.mem.set_u32(out + 0x10, radius);
    e.mem.set_u32(out + 8, 0);
    e.mem.set_u32(out + 0xc, TRIANGLE_TYPE);
    e.mem.set_u32(out, TRIANGLE_VTABLE);
    e.mem.set_u16(out + 0x14, 0);
    e.mem.set_u8(out + 0x16, 6);
    for lane in 0..4 {
        e.mem.set_u32(out + 0x50 + 4 * lane, 0);
    }
    e.mem.set_u8(out + 0x17, 0);
    let welding_type = e.get(shape, HkpExtendedMeshShape::m_weldingType);
    e.mem.set_u8(out + 0x16, welding_type);
    let shape_radius = x87_quiet_bits(e.get(shape, HkpExtendedMeshShape::m_radius));
    e.mem.set_u32(out + 0x10, shape_radius);
    // The extrusion vector and whether it is non-zero (`0 < x*x + (y*y + z*z)` in this order).
    let extrusion: [f32; 4] =
        std::array::from_fn(|lane| e.mem.f32(subpart + 0x30 + 4 * lane as u32));
    let squares = extrusion.map(|lane| lane * lane);
    let length = squares[2] + (squares[1] + squares[0]);
    e.mem.set_u32(out + 0xc, TRIANGLE_TYPE);
    e.mem.set_u8(out + 0x17, u8::from(0.0 < length));
    for lane in 0..4 {
        let word = e.mem.u32(subpart + 0x30 + 4 * lane);
        e.mem.set_u32(out + 0x50 + 4 * lane, word);
    }
    // Welding info of the triangle.
    let welding = if e.get(shape, HkpExtendedMeshShape::m_weldingInfoSize) == 0 {
        0
    } else {
        let first = e.mem.u32(subpart + 0x4c).wrapping_add(child);
        let data = e.get(shape, HkpExtendedMeshShape::m_weldingInfoData);
        e.mem.u16(data.wrapping_add(first.wrapping_mul(2)))
    };
    e.mem.set_u16(out + 0x14, welding);
    // The scaled vertices; the fourth lane of every vertex is a stale stack word (scaled), so
    // it is read from a scratch block that is never written there.
    let scale: [f32; 4] =
        std::array::from_fn(|lane| e.mem.f32(shape.addr() + 0x80 + 4 * lane as u32));
    e.with_stack(0x20, |e, scratch| {
        let stale = e.mem.f32(scratch.addr() + 0xc);
        for (slot, index) in indices.into_iter().enumerate() {
            let address = index.wrapping_mul(vertex_stride).wrapping_add(vertices);
            let mut vertex = [0.0f32; 4];
            for (lane, value) in vertex.iter_mut().take(3).enumerate() {
                let bits = x87_quiet_bits(e.mem.u32(address + 4 * lane as u32));
                *value = f32::from_bits(bits);
            }
            vertex[3] = stale;
            for (lane, value) in vertex.iter().enumerate() {
                e.mem.set_f32(
                    out + 0x20 + 0x10 * slot as u32 + 4 * lane as u32,
                    value * scale[lane],
                );
            }
        }
    });
}

/// The convex-shape part of `getChildShape`: `child` of the (copied) subpart `local`; returns
/// the shape that is the answer, wrapped in a convex transform shape in `out` when the
/// subpart's kind asks for it.
fn convex_child(e: &mut Engine, local: u32, child: u32, out: Ptr) -> Ptr {
    let list = e.mem.u32(local + 0x14);
    let shape = e.mem.u32(list.wrapping_add(child.wrapping_mul(4)));
    let kind = e.mem.u32(local + 0x3c) & 0xc0ff_ffff;
    if kind == 0 {
        return Ptr::new(shape);
    }
    if kind == 1 {
        if out.is_null() {
            return Ptr::NULL;
        }
        let radius = x87_quiet(e.mem.f32(shape + 0x10)).to_bits();
        e.call(
            CONVEX_TRANSFORM_BASE,
            &args![out, 0xbu32, radius, shape, 0u32],
        );
        for lane in 0..4 {
            let word = e.mem.u32(local + 0x30 + 4 * lane) & e.mem.u32(LANE_MASK + 4 * lane);
            e.mem.set_u32(out.addr() + 0x20 + 4 * lane, word);
        }
        e.mem.set_u32(out.addr(), CONVEX_TRANSFORM_VTABLE);
        e.mem.set_u32(out.addr() + 0x1c, 0);
        return out;
    }
    e.with_stack(0x40, |e, transform| {
        for lane in 0..4 {
            let word = e.mem.u32(local + 0x30 + 4 * lane);
            e.mem.set_u32(transform.addr() + 0x30 + 4 * lane, word);
        }
        e.call(SET_TRANSFORM, &args![transform, local + 0x20]);
        if out.is_null() {
            Ptr::NULL
        } else {
            Ptr::new(
                e.call(CONVEX_TRANSFORM, &args![out, shape, transform, 0u32])
                    .u32(),
            )
        }
    })
}

// Translated from 00d55ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpExtendedMeshShape::getChildShape` (Xbox PDB; `this` is the container at shape `+0x10`):
/// the shape of `key`. A triangle key fills the triangle shape `out` (type 3, from the three
/// vertices of the subpart's index record, scaled by the shape's scaling at `+0x80`) and
/// returns it; a null `out` crashes in the exe (it is written through) and is not supported.
/// A convex-shape key returns the stored shape itself (kind 0), or wraps it in a convex
/// transform shape built in `out` (kind 1: translation only, else with a rotation); the
/// wrapped forms return null for a null `out`.
pub fn get_child_shape(e: &mut Engine, this: Ptr, key: u32, out: Ptr) -> Ptr {
    let shape = shape_of(this);
    let bits = e.get(shape, HkpExtendedMeshShape::m_numBitsForSubpartIndex);
    let (index, child) = split_key(bits, key);
    if (key as i32) >= 0 {
        return e.with_stack(0x60, |e, local| {
            let subpart = if e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsSize) == 1 {
                this.addr() + 0x10
            } else {
                let array = e.get(shape, HkpExtendedMeshShape::m_trianglesSubpartsData);
                e.call(
                    0x00d5_7090,
                    &args![local, index.wrapping_mul(0x60).wrapping_add(array)],
                );
                local.addr()
            };
            fill_triangle(e, shape, subpart, child, out.addr());
            out
        });
    }
    e.with_stack(0x40, |e, local| {
        let array = e.get(shape, HkpExtendedMeshShape::m_shapesSubpartsData);
        e.call(0x00d5_70d0, &args![local, (index << 6).wrapping_add(array)]);
        convex_child(e, local.addr(), child, out)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d5_5530,
            get_mesh_material(Ptr<HkpExtendedMeshShape>, u32) -> u32
        ),
        entry!(0x00d5_55d0, fn_00d555d0(Ptr, u32) -> u32),
        entry!(0x00d5_5b40, fn_00d55b40(Ptr) -> u32),
        entry!(0x00d5_5bd0, fn_00d55bd0(Ptr, u32) -> u32),
        entry!(0x00d5_5ce0, get_child_shape(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00d5_7090, fn_00d57090(u32, u32)),
        entry!(0x00d5_70d0, fn_00d570d0(u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const CHILD_DOUBLE: u32 = 0x0900_0400;
    const NEXT_DOUBLE: u32 = 0x0900_0410;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(TRIANGLE_RADIUS, 0x3d4c_cccd);
        e.mem.set_u32(DEGENERACY_TOLERANCE, 0x33d6_bf95);
        for (lane, word) in [!0u32, !0, !0, 0].into_iter().enumerate() {
            e.mem.set_u32(LANE_MASK + 4 * lane as u32, word);
        }
        e
    }

    fn put(e: &mut Engine, address: u32, words: &[u32]) {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(address + 4 * i as u32, *word);
        }
    }

    /// A shape with 8 key bits for the subpart index, scaling (1, 2, 3, 1) and the given
    /// triangle subpart count; the container is at `+0x10`.
    fn new_shape(e: &mut Engine, triangle_subparts: u32) -> u32 {
        let shape = e.mem.alloc(0x100);
        e.mem.set_u32(shape + 0xb0, 8);
        e.mem.set_u32(shape + 0xb8, triangle_subparts);
        put(
            e,
            shape + 0x80,
            &[0x3f80_0000, 0x4000_0000, 0x4040_0000, 0x3f80_0000],
        );
        e.mem.set_u8(shape + 0xd8, 3);
        e.mem.set_u32(shape + 0xdc, 0xf1f1);
        e.mem.set_u32(shape + 0xe4, 0x3dcc_cccd);
        shape
    }

    /// Fills the triangle subpart at `subpart`.
    fn triangle_subpart(e: &mut Engine, subpart: u32, vertices: u32, indices: (u32, u32, u8, u8)) {
        let (base, stride, index_type, mask) = indices;
        e.mem.set_u32(subpart + 0x18, vertices);
        e.mem.set_u32(subpart + 0x1c, 12);
        e.mem.set_u32(subpart + 0x40, base);
        e.mem.set_u32(subpart + 0x44, stride);
        e.mem.set_u8(subpart + 0x48, index_type);
        e.mem.set_u8(subpart + 0x49, mask);
    }

    /// Eight vertices (i, 10 + i, 20 + i).
    fn vertices(e: &mut Engine) -> u32 {
        let vertices = e.mem.alloc(0x60);
        for i in 0..8u32 {
            for lane in 0..3 {
                e.mem
                    .set_f32(vertices + 12 * i + 4 * lane, (i + 10 * lane) as f32);
            }
        }
        vertices
    }

    fn vertex(e: &Engine, out: u32, slot: u32) -> [f32; 3] {
        std::array::from_fn(|lane| e.mem.f32(out + 0x20 + 0x10 * slot + 4 * lane as u32))
    }

    #[test]
    fn copy_functions_copy_the_subpart_words() {
        let mut e = engine();
        let source = e.mem.alloc(0x60);
        for i in 0..24 {
            e.mem.set_u32(source + 4 * i, 0x1000 + i);
        }
        let dest = e.mem.alloc(0x70);
        e.call(0x00d5_7090, &args![dest, source]);
        assert_eq!(e.mem.bytes(dest, 0x60), e.mem.bytes(source, 0x60));
        assert_eq!(e.mem.u32(dest + 0x60), 0);
        let dest = e.mem.alloc(0x50);
        e.call(0x00d5_70d0, &args![dest, source]);
        assert_eq!(e.mem.bytes(dest, 0x40), e.mem.bytes(source, 0x40));
        assert_eq!(e.mem.u32(dest + 0x40), 0);
    }

    #[test]
    fn get_mesh_material_reads_the_subpart_material_table() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        // The embedded subpart: byte materials, record stride 1, material stride 4.
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x0303_0201]);
        e.mem.set_u8(shape + 0x21, 1);
        e.mem.set_u16(shape + 0x22, 4);
        e.mem.set_u32(shape + 0x24, table);
        e.mem.set_u16(shape + 0x28, 1);
        e.mem.set_u32(shape + 0x2c, 0x1000);
        // Child 2 holds the material 3: 3 * 4 + 0x1000.
        assert_eq!(e.call(0x00d5_5530, &args![shape, 2u32]).u32(), 0x100c);
        // 16-bit materials with a negative stride.
        e.mem.set_u8(shape + 0x21, 2);
        e.mem.set_u16(shape + 0x22, 0xfffe);
        e.mem.set_u16(shape + 0x28, 2);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 1u32]).u32(),
            0x1000u32.wrapping_add(0x0303u32.wrapping_mul(0xffff_fffe))
        );
        // No material table: none.
        e.mem.set_u32(shape + 0x24, 0);
        assert_eq!(e.call(0x00d5_5530, &args![shape, 1u32]).u32(), 0);
    }

    #[test]
    fn get_mesh_material_picks_the_subpart_by_the_key() {
        let mut e = engine();
        let shape = new_shape(&mut e, 2);
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x0000_0007]);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        // Triangle subpart 1.
        e.mem.set_u8(subparts + 0x60 + 1, 1);
        e.mem.set_u16(subparts + 0x60 + 2, 2);
        e.mem.set_u32(subparts + 0x60 + 4, table);
        e.mem.set_u16(subparts + 0x60 + 8, 1);
        e.mem.set_u32(subparts + 0x60 + 0xc, 0x2000);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 0x0100_0000u32]).u32(),
            0x200e
        );
        // A subpart index past the array has no material.
        assert_eq!(e.call(0x00d5_5530, &args![shape, 0x0200_0000u32]).u32(), 0);
        // Convex-shape subparts are indexed by the key without a bounds check.
        let convex = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, convex);
        e.mem.set_u8(convex + 0x40 + 1, 1);
        e.mem.set_u16(convex + 0x40 + 2, 1);
        e.mem.set_u32(convex + 0x40 + 4, table);
        e.mem.set_u16(convex + 0x40 + 8, 1);
        e.mem.set_u32(convex + 0x40 + 0xc, 0x10);
        assert_eq!(
            e.call(0x00d5_5530, &args![shape, 0x8100_0000u32]).u32(),
            0x17
        );
    }

    #[test]
    fn collision_filter_info_falls_back_to_the_default() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        let table = e.mem.alloc(0x10);
        put(&mut e, table, &[0x77]);
        e.mem.set_u8(shape + 0x21, 1);
        e.mem.set_u32(shape + 0x24, 0);
        e.mem.set_u16(shape + 0x28, 1);
        assert_eq!(
            e.call(0x00d5_55d0, &args![shape + 0x10, 0u32]).u32(),
            0xf1f1
        );
        // With a material table the answer is the material's first word.
        e.mem.set_u32(shape + 0x24, table);
        e.mem.set_u32(shape + 0x2c, table);
        assert_eq!(e.call(0x00d5_55d0, &args![shape + 0x10, 0u32]).u32(), 0x77);
    }

    #[test]
    fn get_child_shape_builds_a_scaled_triangle_from_an_embedded_list_subpart() {
        let mut e = engine();
        let shape = new_shape(&mut e, 1);
        let vertices = vertices(&mut e);
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0102_0100, 0x0000_0302]);
        // Byte index records of three with the list mask 0: the bytes
        // 0 1 2 | 1 2 3 make record 1 the triangle (1, 2, 3).
        triangle_subpart(&mut e, shape + 0x20, vertices, (indices, 3, 1, 0));
        put(&mut e, shape + 0x50, &[0x3f80_0000, 0, 0, 0x7777_7777]);
        let out = e.mem.alloc(0x60);
        let got = e.call(0x00d5_5ce0, &args![shape + 0x10, 1u32, out]).u32();
        assert_eq!(got, out);
        assert_eq!(vertex(&e, out, 0), [1.0, 22.0, 63.0]);
        assert_eq!(vertex(&e, out, 1), [2.0, 24.0, 66.0]);
        assert_eq!(vertex(&e, out, 2), [3.0, 26.0, 69.0]);
        assert_eq!(e.mem.u16(out + 6), 1);
        assert_eq!(e.mem.u32(out), TRIANGLE_VTABLE);
        assert_eq!(e.mem.u32(out + 0xc), 3);
        assert_eq!(e.mem.u32(out + 0x10), 0x3dcc_cccd);
        assert_eq!(e.mem.u8(out + 0x16), 3);
        assert_eq!(e.mem.u8(out + 0x17), 1);
        assert_eq!(e.mem.u16(out + 0x14), 0);
        assert_eq!(e.mem.bytes(out + 0x50, 16), e.mem.bytes(shape + 0x50, 16));
    }

    #[test]
    fn get_child_shape_reads_strips_welding_info_and_wide_indices() {
        let mut e = engine();
        let shape = new_shape(&mut e, 2);
        let vertices = vertices(&mut e);
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        // Subpart 1: 16-bit strip, one index per record; triangle 3 is (3, 1, 0).
        let indices = e.mem.alloc(0x10);
        put(&mut e, indices, &[0x0001_0000, 0x0003_0002, 0x0001_0000]);
        triangle_subpart(&mut e, subparts + 0x60, vertices, (indices, 2, 2, 1));
        e.mem.set_u32(subparts + 0x60 + 0x4c, 2);
        let welding = e.mem.alloc(0x20);
        put(&mut e, welding, &[0, 0, 0x1234_0000, 0]);
        e.mem.set_u32(shape + 0xcc, welding);
        e.mem.set_u32(shape + 0xd0, 8);
        let out = e.mem.alloc(0x60);
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0x0100_0003u32, out]);
        assert_eq!(vertex(&e, out, 0), [3.0, 26.0, 69.0]);
        assert_eq!(vertex(&e, out, 1), [1.0, 22.0, 63.0]);
        assert_eq!(vertex(&e, out, 2), [0.0, 20.0, 60.0]);
        // Welding entry (2 + 3) of the shape.
        assert_eq!(e.mem.u16(out + 0x14), 0x1234);
        assert_eq!(e.mem.u8(out + 0x17), 0);
        // 32-bit indices in an embedded subpart.
        let shape = new_shape(&mut e, 1);
        let wide = e.mem.alloc(0x10);
        put(&mut e, wide, &[5, 6, 7]);
        triangle_subpart(&mut e, shape + 0x20, vertices, (wide, 12, 3, 0));
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0u32, out]);
        assert_eq!(vertex(&e, out, 0), [5.0, 30.0, 75.0]);
        assert_eq!(vertex(&e, out, 2), [7.0, 34.0, 81.0]);
        // An unknown index type reads vertex 0 three times.
        e.mem.set_u8(shape + 0x68, 9);
        e.call(0x00d5_5ce0, &args![shape + 0x10, 0u32, out]);
        assert_eq!(vertex(&e, out, 1), [0.0, 20.0, 60.0]);
    }

    #[test]
    fn get_child_shape_returns_convex_shapes_plain_or_wrapped() {
        let mut e = engine();
        let shape = new_shape(&mut e, 0);
        let subparts = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, subparts);
        e.mem.set_u32(shape + 0xc4, 2);
        let list = e.mem.alloc(0x10);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(list + 4, child);
        e.mem.set_f32(child + 0x10, 1.5);
        // Subpart 1, kind 0: the stored shape itself.
        let subpart = subparts + 0x40;
        e.mem.set_u32(subpart + 0x14, list);
        put(
            &mut e,
            subpart + 0x30,
            &[0x3f80_0000, 0xc000_0000, 0x4040_0000, 0],
        );
        let out = e.mem.alloc(0x60);
        let key = 0x8100_0001u32;
        assert_eq!(
            e.call(0x00d5_5ce0, &args![shape + 0x10, key, out]).u32(),
            child
        );
        // Kind 1: a convex transform shape with a translation (lane 3 cleared).
        e.mem.set_u32(subpart + 0x3c, 1);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CONVEX_TRANSFORM_BASE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let got = e.call(0x00d5_5ce0, &args![shape + 0x10, key, out]).u32();
        assert_eq!(got, out);
        assert_eq!(*seen.borrow(), vec![vec![out, 0xb, 0x3fc0_0000, child, 0]]);
        assert_eq!(e.mem.u32(out), CONVEX_TRANSFORM_VTABLE);
        assert_eq!(e.mem.u32(out + 0x1c), 0);
        assert_eq!(e.mem.bytes(out + 0x20, 12), e.mem.bytes(subpart + 0x30, 12));
        assert_eq!(e.mem.u32(out + 0x2c), 0);
        // A null output has nothing to build in.
        assert_eq!(
            e.call(0x00d5_5ce0, &args![shape + 0x10, key, 0u32]).u32(),
            0
        );
    }

    #[test]
    fn get_child_shape_builds_a_transformed_convex_shape_for_other_kinds() {
        let mut e = engine();
        let shape = new_shape(&mut e, 0);
        let subparts = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, subparts);
        let list = e.mem.alloc(0x10);
        let child = e.mem.alloc(0x20);
        e.mem.set_u32(list, child);
        e.mem.set_u32(subparts + 0x14, list);
        e.mem.set_u32(subparts + 0x3c, 0x4000_0002);
        put(&mut e, subparts + 0x30, &[0x11, 0x22, 0x33, 0x4000_0002]);
        let transforms = Rc::new(RefCell::new(Vec::new()));
        let transform_log = Rc::clone(&transforms);
        e.register_double(SET_TRANSFORM, move |_, a| {
            transform_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let built = Rc::new(RefCell::new(Vec::new()));
        let built_log = Rc::clone(&built);
        e.register_double(CONVEX_TRANSFORM, move |e, a| {
            let mut words = a.to_vec();
            words.extend((0..4).map(|i| e.mem.u32(a[2] + 0x30 + 4 * i)));
            built_log.borrow_mut().push(words);
            Ret {
                eax: 0x5151,
                ..Ret::default()
            }
        });
        let out = e.mem.alloc(0x60);
        let got = e
            .call(0x00d5_5ce0, &args![shape + 0x10, 0x8000_0000u32, out])
            .u32();
        assert_eq!(got, 0x5151);
        {
            let transforms = transforms.borrow();
            assert_eq!(transforms.len(), 1);
            let built = built.borrow();
            assert_eq!(built[0][0..2], [out, child]);
            assert_eq!(built[0][2], transforms[0][0]);
            assert_eq!(built[0][3], 0);
            assert_eq!(built[0][4..8], [0x11, 0x22, 0x33, 0x4000_0002]);
        }
        // Without an output there is nothing to construct.
        let got = e
            .call(0x00d5_5ce0, &args![shape + 0x10, 0x8000_0000u32, 0u32])
            .u32();
        assert_eq!(got, 0);
        assert_eq!(built.borrow().len(), 1);
    }

    /// A container whose slot `+0x14` hands out triangle (or other) shapes and whose slot
    /// `+0xC` answers `0x4242`; `isDegenerate` answers from `degenerate`.
    fn container(
        e: &mut Engine,
        shape_type: u32,
        degenerate: Vec<bool>,
    ) -> (u32, Rc<RefCell<Vec<Vec<u32>>>>) {
        let shape = new_shape(e, 0);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x14, CHILD_DOUBLE);
        e.mem.set_u32(vtable + 0xc, NEXT_DOUBLE);
        e.mem.set_u32(shape + 0x10, vtable);
        let triangle = e.mem.alloc(0x60);
        e.mem.set_u32(triangle + 0xc, shape_type);
        let asked = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&asked);
        e.register_double(CHILD_DOUBLE, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: triangle,
                ..Ret::default()
            }
        });
        e.register_double(NEXT_DOUBLE, |_, _| Ret {
            eax: 0x4242,
            ..Ret::default()
        });
        let flag = e.mem.alloc(4);
        let answers = RefCell::new(degenerate.into_iter());
        e.register_double(IS_DEGENERATE, move |e, a| {
            let answer = answers
                .borrow_mut()
                .next()
                .expect("unexpected isDegenerate");
            e.mem.set_u8(flag, u8::from(answer));
            assert_eq!(a[1..4], [triangle + 0x20, triangle + 0x30, triangle + 0x40]);
            assert_eq!(a[4], 0x33d6_bf95);
            Ret {
                eax: flag,
                ..Ret::default()
            }
        });
        (shape + 0x10, asked)
    }

    #[test]
    fn first_key_is_the_first_child_unless_it_is_degenerate() {
        let mut e = engine();
        let (this, asked) = container(&mut e, 3, vec![false, true]);
        e.mem.set_u32(this - 0x10 + 0xb8, 1);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0);
        assert_eq!(asked.borrow()[0][..2], [this, 0]);
        // A degenerate first triangle defers to the next-key slot.
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0x4242);
        // With only convex subparts the first key is the first convex shape.
        let (this, asked) = container(&mut e, 1, vec![]);
        e.mem.set_u32(this - 0x10 + 0xc4, 3);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0x8000_0000);
        assert_eq!(asked.borrow()[0][1], 0x8000_0000);
        // No subparts at all: no key.
        e.mem.set_u32(this - 0x10 + 0xc4, 0);
        assert_eq!(e.call(0x00d5_5b40, &args![this]).u32(), 0xffff_ffff);
    }

    #[test]
    fn next_key_walks_children_subparts_then_convex_shapes() {
        let mut e = engine();
        let (this, asked) = container(&mut e, 3, vec![false, false, true, false, false, false]);
        let shape = this - 0x10;
        let subparts = e.mem.alloc(0xc0);
        e.mem.set_u32(shape + 0xb4, subparts);
        e.mem.set_u32(shape + 0xb8, 2);
        e.mem.set_u32(subparts + 0x14, 2);
        e.mem.set_u32(subparts + 0x60 + 0x14, 1);
        let convex = e.mem.alloc(0x80);
        e.mem.set_u32(shape + 0xc0, convex);
        e.mem.set_u32(convex + 0x18, 2);
        // Within a subpart.
        assert_eq!(e.call(0x00d5_5bd0, &args![this, 0u32]).u32(), 1);
        // Past its last child: the next subpart's first child.
        assert_eq!(e.call(0x00d5_5bd0, &args![this, 1u32]).u32(), 0x0100_0000);
        // Past the last triangle subpart there are no convex shapes yet: none.
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x0100_0000u32]).u32(),
            0xffff_ffff
        );
        assert_eq!(asked.borrow().len(), 2);
        // A degenerate candidate (the third answer) is skipped.
        e.mem.set_u32(shape + 0xc4, 1);
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x0100_0000u32]).u32(),
            0x8000_0001
        );
        // The convex shapes are keyed by the high bit.
        e.mem.set_u32(shape + 0xc4, 2);
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x8000_0001u32]).u32(),
            0x8100_0000
        );
        assert_eq!(
            e.call(0x00d5_5bd0, &args![this, 0x8100_0001u32]).u32(),
            0xffff_ffff
        );
    }
}
