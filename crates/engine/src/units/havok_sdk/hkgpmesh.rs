//! `Havok SDK/hkgpmesh.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00e24f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00e1_8a50`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00e24f70(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00e1_8a50, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// ---------------------------------------------------------------------------------------------
// Constants, helpers and the object layout this unit's functions read (offsets are read from
// the code; Havok's own field names are not available for these classes, so fields are named
// by what the code does with them).
// ---------------------------------------------------------------------------------------------

use super::hkaabbtree::rcp_estimate;

/// `operator delete` (`0x00401030`, one stack word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `hkArrayUtil::_reserve` (`0x00c908c0`, cdecl `(allocator, array, wanted, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// The array growth function `00c90940` (cdecl `(allocator, array, element size)`).
const ARRAY_GROW: u32 = 0x00c9_0940;
/// `hkVector4::setTransformedPos` (`0x00c7f640`, thiscall `(this = result, transform, position)`).
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// A 16-byte constant the unit copies where it needs an all-zero vector (`01267e30`).
const ZERO_VECTOR: u32 = 0x0126_7e30;
/// A second 16-byte constant, zero in lane 0 (`010c7160`), the plane offset starts from.
const ZERO_LANES: u32 = 0x010c_7160;
/// A `float` constant read as a scale (`0106c2e0`).
const THIRD: u32 = 0x0106_c2e0;
/// The identity transform the unit copies words out of (`01268370`, 16 words).
const IDENTITY_TRANSFORM: u32 = 0x0126_8370;
/// The error reporter object (a pointer at `01268360`) the pool's assertion calls through
/// its slot `0x0c`, and the three string-buffer functions that build its message.
const ERROR_REPORTER: u32 = 0x0126_8360;
const STRING_BUFFER_INIT: u32 = 0x00cb_1600;
const STRING_BUFFER_APPEND: u32 = 0x00cc_01c0;
const STRING_BUFFER_EXIT: u32 = 0x00cc_00e0;
/// The pool assertion's message text, source file name and line (`0xaf`) and message hash.
const POOL_MESSAGE_TEXT: u32 = 0x010d_365c;
const POOL_SOURCE_FILE: u32 = 0x010d_35f0;
const POOL_MESSAGE_HASH: u32 = 0x9035_5365;

/// Mesh (`hkgpMesh`) fields. The vertex pool starts at `+0x08` and the triangle pool at
/// `+0x20` (both are `hkgpAbstractMesh` pool allocators, see [`POOL_HEAD`]).
const MESH_VERTEX_POOL: u32 = 0x08;
const MESH_TRIANGLE_POOL: u32 = 0x20;
/// Head of the vertex list and the number of vertices (the pool's `+0x0c`, `+0x10`).
const MESH_VERTEX_HEAD: u32 = 0x14;
const MESH_VERTEX_COUNT: u32 = 0x18;
/// Head of the triangle list and the number of triangles.
const MESH_TRIANGLE_HEAD: u32 = 0x2c;
/// The convex hull (`hkgpConvexHull`, built by `rebuildConvexHull` `00e13750`) that `00e114c0` deletes and `00e114a0` returns.
const MESH_CONVEX_HULL: u32 = 0xe4;
/// A byte the three one-line callbacks `00e114f0`..`00e11520` set to 1.
const MESH_PROBLEM_FLAG: u32 = 0x100;
/// A byte the mesh clears when its triangles change.
const MESH_DIRTY_FLAG: u32 = 0x101;
/// `float` tolerances the plane computation compares against.
const MESH_EDGE_TOLERANCE: u32 = 0x104;
const MESH_NORMAL_TOLERANCE: u32 = 0x108;

/// Pool (`hkGeometryProcessing::PoolAllocator`) fields: an `hkArray` of blocks (data, size,
/// capacity and flags), the head of the doubly linked list of live elements and its length.
const POOL_HEAD: u32 = 0x0c;
const POOL_COUNT: u32 = 0x10;

/// Element links: next at `+0`, previous at `+4`.
const ELEMENT_NEXT: u32 = 0;
const ELEMENT_PREVIOUS: u32 = 4;
/// Vertex position (a 16-byte vector).
const VERTEX_POSITION: u32 = 0x10;
/// Triangle fields: three vertex pointers, three neighbour links (a triangle pointer with
/// the neighbour's edge index in the low two bits), the plane (16 bytes) and two words.
const TRIANGLE_VERTEX_0: u32 = 0x08;
const TRIANGLE_VERTEX_1: u32 = 0x0c;
const TRIANGLE_VERTEX_2: u32 = 0x10;
const TRIANGLE_LINK_0: u32 = 0x14;
const TRIANGLE_PLANE: u32 = 0x20;
const TRIANGLE_INDEX: u32 = 0x30;
const TRIANGLE_PART: u32 = 0x34;

/// Four `float` lanes.
type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, value: Vec4) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, word);
    }
}

/// `MOVAPS` between memory operands: the four words exactly.
fn copy_vec(e: &mut Engine, destination: u32, source: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(source + 4 * lane);
        e.mem.set_u32(destination + 4 * lane, word);
    }
}

fn mul_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] * b[lane])
}

fn add_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] + b[lane])
}

fn sub_ps(a: Vec4, b: Vec4) -> Vec4 {
    std::array::from_fn(|lane| a[lane] - b[lane])
}

/// `MINPS dst, src` per lane: `dst` when it is less than `src`, else `src`.
fn min_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] < src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// `MAXPS dst, src` per lane: `dst` when it is greater than `src`, else `src`.
fn max_ps(dst: Vec4, src: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        if dst[lane] > src[lane] {
            dst[lane]
        } else {
            src[lane]
        }
    })
}

/// What the `SHUFPS 0x55` / `ADDSS` / `SHUFPS 0xaa` / `ADDSS` sequence leaves in lane 0 for
/// the vector `p`: `p[2] + (p[1] + p[0])`.
fn sum_of_three(p: Vec4) -> f32 {
    p[2] + (p[1] + p[0])
}

/// The bits `FNSTSW AX` leaves in `AH` after comparing `st0` with `st1`: C0 (`0x01`) when
/// `st0 < st1`, C3 (`0x40`) when they are equal, and C0, C2 (`0x04`) and C3 when unordered.
fn fcom_ah(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 > st1 {
        0
    } else if st0 < st1 {
        0x01
    } else {
        0x40
    }
}

/// The parity flag a `TEST AH, mask` leaves: set when the masked byte has an even number of
/// one bits (`JP` jumps, `JNP` does not).
fn parity_even(masked: u8) -> bool {
    masked.count_ones() % 2 == 0
}

/// The allocator object of the memory router (`CALL 00c85750`, then the word at `+0x10`).
fn memory_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.mem.u32(router + 0x10)
}

/// Leaves `vtable` in the object and, when bit 0 of `flags` is set, frees the block with
/// `operator delete`. Returns `this`.
fn scalar_deleting_with_operator_delete(e: &mut Engine, this: Ptr, flags: u8, vtable: u32) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// Leaves `vtable` in the object and, when bit 0 of `flags` is set, returns the block of
/// `size` bytes to the memory router's allocator (slot `8`). Returns `this`.
fn scalar_deleting_with_router(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
    vtable: u32,
    size: u32,
) -> Ptr {
    e.mem.set_u32(this.addr(), vtable);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArray` of words, `pushBack` as the exe inlines it: the router first, room made through
/// `00c90940` when the size equals the capacity, then the element.
fn array_push_word(e: &mut Engine, array: u32, value: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(data + size * 4, value);
    e.mem.set_u32(array + 4, size + 1);
}

/// Frees the storage of an `hkArray` that owns it (bit 31 of the capacity word clear)
/// through the allocator's slot `0x10` with `bytes` bytes, as the exe's inlined `clearAndDeallocate`.
fn array_free_storage(e: &mut Engine, allocator: u32, array: u32, element_bytes: u32) {
    let capacity = e.mem.u32(array + 8);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_bytes);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00d1ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::PrismPolicy::_scalar_deleting_destructor_` (Xbox PDB; one stack word `flags`):
/// leaves vtable `010cd658` in the object and, when bit 0 of `flags` is set, frees it with
/// `operator delete`. Returns `this`.
pub fn hkgp_mesh_prism_policy_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_operator_delete(e, this, flags, 0x010c_d658)
}

// Translated from 00d1bab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMeshInternals::TriangleOverlap::_scalar_deleting_destructor_` (Xbox PDB; one stack
/// word `flags`): leaves vtable `010cd664` and, when bit 0 of `flags` is set, frees the
/// object with `operator delete`. Returns `this`.
pub fn hkgp_mesh_internals_triangle_overlap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_operator_delete(e, this, flags, 0x010c_d664)
}

// Translated from 00e11480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::IConvexOverlap::IConvexShape::_scalar_deleting_destructor_` (Xbox PDB; one
/// stack word `flags`): leaves vtable `010eacf4` and, when bit 0 of `flags` is set, frees
/// the object with `operator delete`. Returns `this`.
pub fn hkgp_mesh_iconvex_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_operator_delete(e, this, flags, 0x010e_acf4)
}

// Translated from 00e22450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGskShape::_scalar_deleting_destructor_` (Xbox PDB; one stack word `flags`): leaves
/// vtable `010eadfc` and, when bit 0 of `flags` is set, frees the object with
/// `operator delete`. Returns `this`.
pub fn hk_gsk_shape_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    scalar_deleting_with_operator_delete(e, this, flags, 0x010e_adfc)
}

// Translated from 00e22500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::IConvexOverlap::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): leaves vtable `010ead08` and, when bit 0 of `flags` is set, frees the object
/// with `operator delete`. Returns `this`.
pub fn hkgp_mesh_iconvex_overlap_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_operator_delete(e, this, flags, 0x010e_ad08)
}

// Translated from 00e21e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::LineShape::_scalar_deleting_destructor_` (Xbox PDB; one stack word `flags`):
/// leaves vtable `010eacf4` and, when bit 0 of `flags` is set, returns the `0x30`-byte block
/// to the memory router's allocator. Returns `this`.
pub fn hkgp_mesh_line_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_router(e, this, flags, 0x010e_acf4, 0x30)
}

// Translated from 00e21e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::TriangleShape::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): leaves vtable `010eacf4` and, when bit 0 of `flags` is set, returns the 8-byte
/// block to the memory router's allocator. Returns `this`.
pub fn hkgp_mesh_triangle_shape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_router(e, this, flags, 0x010e_acf4, 8)
}

// Translated from 00e22d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::WeldingPolicy::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): leaves vtable `010cd664` and, when bit 0 of `flags` is set, returns the
/// `0x30`-byte block to the memory router's allocator. Returns `this`.
pub fn hkgp_mesh_welding_policy_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_router(e, this, flags, 0x010c_d664, 0x30)
}

// Translated from 00e22d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::CollideShapeTriangle::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): leaves vtable `010cd664` and, when bit 0 of `flags` is set, returns the
/// `0x14`-byte block to the memory router's allocator. Returns `this`.
pub fn hkgp_mesh_collide_shape_triangle_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_router(e, this, flags, 0x010c_d664, 0x14)
}

// Translated from 00e22db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::NearestFeaturePolicy::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): leaves vtable `010cd664` and, when bit 0 of `flags` is set, returns the
/// `0x60`-byte block to the memory router's allocator. Returns `this`.
pub fn hkgp_mesh_nearest_feature_policy_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    scalar_deleting_with_router(e, this, flags, 0x010c_d664, 0x60)
}

// Translated from 00e114a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word at `this + 0xe4` of the mesh: the convex hull object (`hkgpConvexHull`, built
/// by `rebuildConvexHull`) that `00e114c0` deletes.
pub fn fn_00e114a0(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + MESH_CONVEX_HULL)
}

// Translated from 00e114c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the object at `this + 0xe4` through its slot 0 (the deleting destructor, with
/// flag 1) when there is one, then clears the field.
pub fn fn_00e114c0(e: &mut Engine, this: Ptr) {
    let hull = e.mem.u32(this.addr() + MESH_CONVEX_HULL);
    if hull != 0 {
        e.vcall(hull, 0, &args![1u32]);
    }
    e.mem.set_u32(this.addr() + MESH_CONVEX_HULL, 0);
}

// Translated from 00e114f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x100` to 1; the three stack words are not read.
pub fn fn_00e114f0(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32, _unused_3: u32) {
    e.mem.set_u8(this.addr() + MESH_PROBLEM_FLAG, 1);
}

// Translated from 00e11500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x100` to 1; the two stack words are not read.
pub fn fn_00e11500(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    e.mem.set_u8(this.addr() + MESH_PROBLEM_FLAG, 1);
}

// Translated from 00e11510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x100` to 1; the two stack words are not read.
pub fn fn_00e11510(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    e.mem.set_u8(this.addr() + MESH_PROBLEM_FLAG, 1);
}

// Translated from 00e11520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `this + 0x100` to 1; the two stack words are not read.
pub fn fn_00e11520(e: &mut Engine, this: Ptr, _unused_1: u32, _unused_2: u32) {
    e.mem.set_u8(this.addr() + MESH_PROBLEM_FLAG, 1);
}

// Translated from 00e11530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounding box of the mesh's vertex positions (`this` is ECX, one stack word `out`): writes
/// the per-lane minimum at `out` and the maximum at `out + 0x10`. The head vertex's position
/// is read before the list is tested, as the exe does, so an empty mesh reads address `0x10`.
pub fn fn_00e11530(e: &mut Engine, this: Ptr, out: Ptr) {
    let head = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    let first = load_vec(e, head + VERTEX_POSITION);
    store_vec(e, out.addr(), first);
    store_vec(e, out.addr() + 0x10, first);
    if head == 0 {
        return;
    }
    let mut minimum = first;
    let mut maximum = first;
    let mut vertex = head;
    while vertex != 0 {
        let position = load_vec(e, vertex + VERTEX_POSITION);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
        minimum = min_ps(minimum, position);
        maximum = max_ps(maximum, position);
    }
    store_vec(e, out.addr(), minimum);
    store_vec(e, out.addr() + 0x10, maximum);
}

// Translated from 00e11570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vertex furthest along a direction (`this` is ECX; stack: `out`, `direction`): copies
/// the head vertex's position to `out`, then for each vertex whose dot product with the
/// direction (three lanes) is greater than the best so far stores its position at `out`
/// with lane 3 replaced by the bits of `0.5f` OR-ed with the vertex index (`0x3f000000 | i`).
/// Returns `out`. The head vertex's position is read before the list is tested.
pub fn fn_00e11570(e: &mut Engine, this: Ptr, out: Ptr, direction: Ptr) -> Ptr {
    let head = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    for word in 0..4 {
        let value = e.mem.u32(head + VERTEX_POSITION + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    let direction = load_vec(e, direction.addr());
    let current = load_vec(e, out.addr());
    let mut best = sum_of_three(mul_ps(current, direction));
    let mut vertex = head;
    let mut index = 0u32;
    while vertex != 0 {
        let position = load_vec(e, vertex + VERTEX_POSITION);
        let candidate = sum_of_three(mul_ps(position, direction));
        // FLD candidate; FLD best; FCOMP; TEST AH,5; JP skips.
        let condition = fcom_ah(f64::from(best), f64::from(candidate));
        if !parity_even(condition & 0x05) {
            best = candidate;
            copy_vec(e, out.addr(), vertex + VERTEX_POSITION);
            e.mem.set_u32(out.addr() + 0xc, index | 0x3f00_0000);
        }
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
        index += 1;
    }
    out
}

// Translated from 00e11630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Centroid of a triangle (stack: `out`, `triangle`; no register input): the sum of the
/// three vertex positions, `(p1 + p0) + p2` on all four lanes, times the float constant at
/// `0106c2e0` splatted. Writes the vector to `out`.
pub fn fn_00e11630(e: &mut Engine, out: Ptr, triangle: Ptr) {
    let t = triangle.addr();
    let v0 = e.mem.u32(t + TRIANGLE_VERTEX_0);
    let v1 = e.mem.u32(t + TRIANGLE_VERTEX_1);
    let v2 = e.mem.u32(t + TRIANGLE_VERTEX_2);
    let p0 = load_vec(e, v0 + VERTEX_POSITION);
    let p1 = load_vec(e, v1 + VERTEX_POSITION);
    let scale = e.mem.f32(THIRD);
    let p2 = load_vec(e, v2 + VERTEX_POSITION);
    let sum = add_ps(add_ps(p1, p0), p2);
    store_vec(e, out.addr(), mul_ps(sum, [scale; 4]));
}

// Translated from 00e11670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Centre of the mesh's vertices (`this` is ECX, stack: `out`): `out` starts as the zero
/// vector `01267e30`; with a vertex count above zero it becomes the sum of the positions
/// times `1.0 / count` (an `FILD`, `FDIVR`, `FSTP` float); otherwise, when the object at
/// `this + 0xe4` exists, it is the vector `00d752d0` returns for that object. Returns `out`.
pub fn fn_00e11670(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let o = out.addr();
    for lane in 0..4 {
        let word = e.mem.u32(ZERO_VECTOR + 4 * lane);
        e.mem.set_u32(o + 4 * lane, word);
    }
    let count = e.mem.i32(this.addr() + MESH_VERTEX_COUNT);
    if count > 0 {
        let head = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
        if head != 0 {
            let mut sum = load_vec(e, o);
            let mut vertex = head;
            while vertex != 0 {
                let position = load_vec(e, vertex + VERTEX_POSITION);
                vertex = e.mem.u32(vertex + ELEMENT_NEXT);
                sum = add_ps(position, sum);
            }
            store_vec(e, o, sum);
        }
        let reciprocal = f32::from_bits(x87_store32(x87_div(1.0, f64::from(count))));
        let sum = load_vec(e, o);
        store_vec(e, o, mul_ps([reciprocal; 4], sum));
    } else {
        let hull = e.mem.u32(this.addr() + MESH_CONVEX_HULL);
        if hull != 0 {
            let vector = e.call(0x00d7_52d0, &args![hull]).u32();
            copy_vec(e, o, vector);
        }
    }
    out
}

// Translated from 00e11710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::setPlane` (Xbox PDB; `this` is ECX; stack: the three vertex positions `a`, `b`,
/// `c`, the plane `out` and a check byte). Writes the unit normal of the triangle to `out`
/// (the cross product of `c - a` and `b - a` taken lane by lane, times `1 / length`; the
/// length is first stored in lane 3) and the plane offset `0 - a . normal` to lane 3. Returns
/// 1, or 0 when `check` is non-zero and an edge is no longer than the tolerance at
/// `this + 0x104` (squared length, reported through `00e11500`), or the normal's length is
/// below the tolerance at `this + 0x108` (reported through `00e114f0`).
pub fn mesh_set_plane(
    e: &mut Engine,
    this: Ptr,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    out: Ptr,
    check: u8,
) -> u8 {
    let checking = check != 0;
    let tolerance = f64::from(x87_quiet(e.mem.f32(this.addr() + MESH_EDGE_TOLERANCE)));
    let pa = load_vec(e, a.addr());
    let pb = load_vec(e, b.addr());
    let mut ok = 1u8;
    let edge_b = sub_ps(pb, pa);
    if checking {
        let squared = sum_of_three(mul_ps(edge_b, edge_b));
        let condition = fcom_ah(f64::from(squared), tolerance);
        if !parity_even(condition & 0x41) {
            fn_00e11500(e, this, a.addr(), b.addr());
            ok = 0;
        }
    }
    let pc = load_vec(e, c.addr());
    let edge_c = sub_ps(pc, pa);
    if checking {
        let squared = sum_of_three(mul_ps(edge_c, edge_c));
        let condition = fcom_ah(f64::from(squared), tolerance);
        if !parity_even(condition & 0x41) {
            fn_00e11500(e, this, a.addr(), c.addr());
            ok = 0;
        }
    }
    let pb = load_vec(e, b.addr());
    let pc = load_vec(e, c.addr());
    let edge_bc = sub_ps(pc, pb);
    if checking {
        let squared = sum_of_three(mul_ps(edge_bc, edge_bc));
        let condition = fcom_ah(f64::from(squared), tolerance);
        if !parity_even(condition & 0x41) {
            fn_00e11500(e, this, c.addr(), b.addr());
            ok = 0;
        }
    }
    // SHUFPS 0xc9 gives lanes (1, 2, 0, 3) and 0xd2 gives (2, 0, 1, 3).
    let shuffle_c9 = |v: Vec4| [v[1], v[2], v[0], v[3]];
    let shuffle_d2 = |v: Vec4| [v[2], v[0], v[1], v[3]];
    let positive = mul_ps(shuffle_c9(edge_c), shuffle_d2(edge_b));
    let negative = mul_ps(shuffle_d2(edge_c), shuffle_c9(edge_b));
    let normal = sub_ps(negative, positive);
    store_vec(e, out.addr(), normal);
    let length = sum_of_three(mul_ps(normal, normal)).sqrt();
    // FLD length; FST [out + 0xc]: the store quiets a signalling NaN.
    let stored_length = x87_quiet(length);
    e.mem.set_f32(out.addr() + 0xc, stored_length);
    if checking {
        let normal_tolerance = f64::from(x87_quiet(e.mem.f32(this.addr() + MESH_NORMAL_TOLERANCE)));
        let condition = fcom_ah(normal_tolerance, f64::from(stored_length));
        if condition & 0x41 == 0 {
            fn_00e114f0(e, this, a.addr(), b.addr(), c.addr());
            ok = 0;
        }
    }
    let length = x87_quiet(e.mem.f32(out.addr() + 0xc));
    let reciprocal = f32::from_bits(x87_store32(x87_div(1.0, f64::from(length))));
    let normal = load_vec(e, out.addr());
    let scaled = mul_ps([reciprocal; 4], normal);
    store_vec(e, out.addr(), scaled);
    let along = mul_ps(load_vec(e, a.addr()), scaled);
    let offset = e.mem.f32(ZERO_LANES) - sum_of_three(along);
    e.mem.set_f32(out.addr() + 0xc, x87_quiet(offset));
    ok
}

// Translated from 00e118e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plane of a triangle (`this` is ECX; stack: `triangle`, `out`, `check`): `setPlane` (`00e11710`)
/// on the positions of the triangle's three vertices. Returns its result.
pub fn fn_00e118e0(e: &mut Engine, this: Ptr, triangle: Ptr, out: Ptr, check: u8) -> u8 {
    let t = triangle.addr();
    let v0 = e.mem.u32(t + TRIANGLE_VERTEX_0);
    let v1 = e.mem.u32(t + TRIANGLE_VERTEX_1);
    let v2 = e.mem.u32(t + TRIANGLE_VERTEX_2);
    mesh_set_plane(
        e,
        this,
        Ptr::new(v0 + VERTEX_POSITION),
        Ptr::new(v1 + VERTEX_POSITION),
        Ptr::new(v2 + VERTEX_POSITION),
        out,
        check,
    )
}

// Translated from 00e11910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Recomputes a triangle's plane (`this` is ECX; stack: `triangle`): `setPlane` with checking
/// into a temporary; when it succeeds the plane is copied to `triangle + 0x20`. Returns
/// whether it succeeded.
pub fn fn_00e11910(e: &mut Engine, this: Ptr, triangle: Ptr) -> bool {
    let t = triangle.addr();
    let v0 = e.mem.u32(t + TRIANGLE_VERTEX_0);
    let v1 = e.mem.u32(t + TRIANGLE_VERTEX_1);
    let v2 = e.mem.u32(t + TRIANGLE_VERTEX_2);
    e.with_stack(0x20, |e, plane| {
        let ok = mesh_set_plane(
            e,
            this,
            Ptr::new(v0 + VERTEX_POSITION),
            Ptr::new(v1 + VERTEX_POSITION),
            Ptr::new(v2 + VERTEX_POSITION),
            plane,
            1,
        );
        if ok == 0 {
            return false;
        }
        copy_vec(e, t + TRIANGLE_PLANE, plane.addr());
        true
    })
}

// Translated from 00e11960 (decompiled, FalloutNV.exe 1.4.0.525)
/// One more than the largest signed word at `+0x30` over the mesh's triangles (`this` is ECX);
/// 0 when there are none.
pub fn fn_00e11960(e: &mut Engine, this: Ptr) -> i32 {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    let mut largest = -1i32;
    while triangle != 0 {
        let index = e.mem.i32(triangle + TRIANGLE_INDEX);
        if index > largest {
            largest = index;
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    largest + 1
}

// Translated from 00e11990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Classifies the mesh's vertices against a plane (`this` is ECX; stack: `plane` (a vector:
/// normal in lanes 0 to 2, offset in lane 3) and `slot`). For each vertex the signed distance
/// `(z + (y + x)) + offset` of its position is computed; when its square is not above the
/// tolerance at `this + 0x104` the distance is taken as 0; it is stored in the float at
/// `vertex + 0x30 + 4 * slot`. Returns bit 0 when any distance is above zero and bit 1 when any
/// is below zero (a NaN counts as neither).
pub fn fn_00e11990(e: &mut Engine, this: Ptr, plane: Ptr, slot: u32) -> u32 {
    let mut vertex = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    let mut positive = false;
    let mut negative = false;
    while vertex != 0 {
        let plane_vector = load_vec(e, plane.addr());
        let position = load_vec(e, vertex + VERTEX_POSITION);
        let sum = sum_of_three(mul_ps(position, plane_vector));
        let distance = sum + plane_vector[3];
        let mut kept = f64::from(x87_quiet(distance));
        let squared = x87_mul(kept, kept) as f32;
        let tolerance = f64::from(x87_quiet(e.mem.f32(this.addr() + MESH_EDGE_TOLERANCE)));
        let condition = fcom_ah(tolerance, f64::from(x87_quiet(squared)));
        if condition & 0x01 == 0 {
            kept = 0.0;
        }
        let stored = kept as f32;
        e.mem.set_f32(vertex + 0x30 + 4 * slot, stored);
        let condition = fcom_ah(f64::from(stored), 0.0);
        if !parity_even(condition & 0x05) {
            negative = true;
        } else {
            let condition = fcom_ah(f64::from(stored), 0.0);
            if condition & 0x41 == 0 {
                positive = true;
            }
        }
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
    u32::from(positive) + if negative { 2 } else { 0 }
}

// Translated from 00e11a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::updatePlaneEquations` (Xbox PDB; `this` is ECX): for each triangle, resets the
/// plane at `+0x20` to the zero vector `01267e30` and recomputes it (`00e11910`).
pub fn mesh_update_plane_equations(e: &mut Engine, this: Ptr) {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        copy_vec(e, triangle + TRIANGLE_PLANE, ZERO_VECTOR);
        fn_00e11910(e, this, Ptr::new(triangle));
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
}

// Translated from 00e11a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::applyTransform` (Xbox PDB; `this` is ECX; stack: `transform`): replaces every
/// vertex position by `hkVector4::setTransformedPos` of the transform on a copy of it, then
/// updates the plane equations (`00e11a60`) and deletes the convex hull (`00e114c0`).
pub fn mesh_apply_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    let mut vertex = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    while vertex != 0 {
        e.with_stack(0x10, |e, local| {
            copy_vec(e, local.addr(), vertex + VERTEX_POSITION);
            e.call(
                SET_TRANSFORMED_POS,
                &args![vertex + VERTEX_POSITION, transform, local],
            );
        });
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
    mesh_update_plane_equations(e, this);
    fn_00e114c0(e, this);
}

// Translated from 00e11b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::applyScale` (Xbox PDB; `this` is ECX; stack: `scale`): multiplies every vertex
/// position by the vector at `scale` lane by lane, then updates the plane equations and
/// deletes the convex hull.
pub fn mesh_apply_scale(e: &mut Engine, this: Ptr, scale: Ptr) {
    let mut vertex = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    while vertex != 0 {
        let position = load_vec(e, vertex + VERTEX_POSITION);
        let factor = load_vec(e, scale.addr());
        store_vec(e, vertex + VERTEX_POSITION, mul_ps(position, factor));
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
    mesh_update_plane_equations(e, this);
    fn_00e114c0(e, this);
}

// Translated from 00e11b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a rotation matrix (`this` is ECX, passed on to `applyTransform`; stack: `rotation`,
/// three 16-byte columns): builds a transform from the three columns and the translation
/// words of the identity transform (`012683a0`..`012683ac`) and applies it (`00e11a90`).
pub fn fn_00e11b40(e: &mut Engine, this: Ptr, rotation: Ptr) {
    e.with_stack(0x40, |e, transform| {
        let t = transform.addr();
        for word in 0..4 {
            let value = e.mem.u32(IDENTITY_TRANSFORM + 0x30 + 4 * word);
            e.mem.set_u32(t + 0x30 + 4 * word, value);
        }
        copy_vec(e, t, rotation.addr());
        copy_vec(e, t + 0x10, rotation.addr() + 0x10);
        copy_vec(e, t + 0x20, rotation.addr() + 0x20);
        mesh_apply_transform(e, this, transform);
    });
}

// Translated from 00e11ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a rotation given as a quaternion (`this` is ECX; stack: `quaternion`): the identity
/// transform (`01268370`, 16 words) with its rotation part set from the quaternion by
/// `00cb2d90`, applied with `applyTransform`.
pub fn fn_00e11ba0(e: &mut Engine, this: Ptr, quaternion: Ptr) {
    e.with_stack(0x40, |e, transform| {
        for word in 0..16 {
            let value = e.mem.u32(IDENTITY_TRANSFORM + 4 * word);
            e.mem.set_u32(transform.addr() + 4 * word, value);
        }
        e.call(0x00cb_2d90, &args![transform, quaternion]);
        mesh_apply_transform(e, this, transform);
    });
}

// Translated from 00e11ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies a translation (`this` is ECX; stack: `translation`, a vector): the identity
/// rotation (words 0 to 11 of `01268370`) with the vector as the fourth column, applied with
/// `applyTransform`.
pub fn fn_00e11ca0(e: &mut Engine, this: Ptr, translation: Ptr) {
    e.with_stack(0x40, |e, transform| {
        for word in 0..12 {
            let value = e.mem.u32(IDENTITY_TRANSFORM + 4 * word);
            e.mem.set_u32(transform.addr() + 4 * word, value);
        }
        copy_vec(e, transform.addr() + 0x30, translation.addr());
        mesh_apply_transform(e, this, transform);
    });
}

// Translated from 00e11d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a float in slot `index` of every vertex (`this` is ECX; stack: `index`, `value`): the
/// float at `vertex + 0x30 + 4 * index` becomes `value` (loaded and stored through the x87
/// stack, which quiets a signalling NaN). Nothing is loaded when there are no vertices.
pub fn fn_00e11d80(e: &mut Engine, this: Ptr, index: u32, value: f32) {
    let mut vertex = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    if vertex == 0 {
        return;
    }
    let stored = x87_quiet(value);
    while vertex != 0 {
        e.mem.set_f32(vertex + 0x30 + 4 * index, stored);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
}

// Translated from 00e11da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the word at `+0x44` of every vertex to `value` (`this` is ECX; stack: `value`).
pub fn fn_00e11da0(e: &mut Engine, this: Ptr, value: u32) {
    let mut vertex = e.mem.u32(this.addr() + MESH_VERTEX_HEAD);
    while vertex != 0 {
        e.mem.set_u32(vertex + 0x44, value);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
}

// ---------------------------------------------------------------------------------------------
// The pools (`hkGeometryProcessing::PoolAllocator`) and the small array and list functions.
// A pool is an `hkArray` of blocks (data, size, capacity and flags at `+0`, `+4`, `+8`), the
// head of its list of live elements at `+0x0c` and their number at `+0x10`. A block holds 32
// elements of one stride, then the head of its list of free elements (`block_size - 8`) and
// its number of live elements (`block_size - 4`); free elements are chained through their
// first word.
// ---------------------------------------------------------------------------------------------

/// Makes a new block of `block_size` bytes for the pool (`this` of the exe functions): the
/// allocator's slot 4, the 32 elements of `stride` bytes chained from element 0 to element
/// 31, no live elements, the block pushed on the pool's array and swapped with the first
/// entry when there is more than one. Returns the block.
fn pool_new_block(e: &mut Engine, pool: u32, block_size: u32, stride: u32) -> u32 {
    let allocator = memory_allocator(e);
    let block = e.vcall(allocator, 4, &args![block_size]).u32();
    let mut previous = 0u32;
    let mut element = block + block_size - 8 - stride;
    let mut head = 0u32;
    for _ in 0..32 {
        e.mem.set_u32(element, previous);
        head = element;
        previous = element;
        element = element.wrapping_sub(stride);
    }
    e.mem.set_u32(block + block_size - 4, 0);
    e.mem.set_u32(block + block_size - 8, head);
    array_push_word(e, pool, block);
    let size = e.mem.i32(pool + 4);
    if size > 1 {
        let data = e.mem.u32(pool);
        let last_slot = data + (size as u32) * 4 - 4;
        let last = e.mem.u32(last_slot);
        let first = e.mem.u32(data);
        e.mem.set_u32(data, last);
        e.mem.set_u32(last_slot, first);
    }
    block
}

/// Takes the next free element of the pool's first block, making a block with `new_block`
/// when the pool has none or its first block has no free element: the block's live count is
/// incremented and its free head moves to the element's chain word. Returns the element.
fn pool_take_element(
    e: &mut Engine,
    pool: u32,
    block_size: u32,
    new_block: impl FnOnce(&mut Engine) -> u32,
) -> u32 {
    let free_head = block_size - 8;
    let mut block = 0;
    let mut usable = false;
    if e.mem.i32(pool + 4) != 0 {
        let data = e.mem.u32(pool);
        block = e.mem.u32(data);
        usable = e.mem.u32(block + free_head) != 0;
    }
    if !usable {
        block = new_block(e);
    }
    let element = e.mem.u32(block + free_head);
    let next_free = e.mem.u32(element);
    let live = e.mem.u32(block + free_head + 4);
    e.mem.set_u32(block + free_head + 4, live.wrapping_add(1));
    e.mem.set_u32(block + free_head, next_free);
    element
}

/// Pushes the new element at the front of the pool's list of live elements (previous 0, next
/// the old head, whose previous link becomes the element) and counts it.
fn pool_link_front(e: &mut Engine, pool: u32, element: u32) {
    e.mem.set_u32(element + ELEMENT_PREVIOUS, 0);
    let head = e.mem.u32(pool + POOL_HEAD);
    e.mem.set_u32(element + ELEMENT_NEXT, head);
    let head = e.mem.u32(pool + POOL_HEAD);
    if head != 0 {
        e.mem.set_u32(head + ELEMENT_PREVIOUS, element);
    }
    let count = e.mem.u32(pool + POOL_COUNT);
    e.mem.set_u32(pool + POOL_COUNT, count.wrapping_add(1));
    e.mem.set_u32(pool + POOL_HEAD, element);
}

/// Returns every block to the allocator (slot 8, with `block_size`), then clears the block
/// count, the list head and the live count.
fn pool_release_blocks(e: &mut Engine, pool: u32, block_size: u32) {
    let mut index = 0;
    while index < e.mem.i32(pool + 4) {
        let data = e.mem.u32(pool);
        let block = e.mem.u32(data + 4 * index as u32);
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![block, block_size]);
        index += 1;
    }
    e.mem.set_u32(pool + 4, 0);
    e.mem.set_u32(pool + POOL_HEAD, 0);
    e.mem.set_u32(pool + POOL_COUNT, 0);
}

/// The pool's destructor: frees every block (slot 8, `block_size`), sets the block count to 0,
/// frees the array's storage when it owns it and leaves it empty (capacity word `0x80000000`).
fn pool_destroy(e: &mut Engine, pool: u32, block_size: u32) {
    let mut index = 0;
    while index < e.mem.i32(pool + 4) {
        let data = e.mem.u32(pool);
        let block = e.mem.u32(data + 4 * index as u32);
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![block, block_size]);
        index += 1;
    }
    e.mem.set_u32(pool + 4, 0);
    let allocator = memory_allocator(e);
    e.mem.set_u32(pool + 4, 0);
    array_free_storage(e, allocator, pool, 4);
    e.mem.set_u32(pool + 8, 0x8000_0000);
    e.mem.set_u32(pool, 0);
}

/// The assertion the unit raises: builds the message `text` in a 0x200-byte stack buffer
/// (`00cb1600`, `00cc01c0`), reports it through slot `0x0c` of the reporter object at
/// `01268360` (severity 3, `hash`, the buffer, source `file` and `line`) and executes `INT3`
/// when the report returns non-zero, otherwise ends the message (`00cc00e0`). `int3_address` is
/// the address of that instruction in the calling function.
fn assertion_report(e: &mut Engine, text: u32, hash: u32, file: u32, line: u32, int3_address: u32) {
    e.with_stack(0x20c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        e.call(STRING_BUFFER_INIT, &args![frame, buffer, 0x200u32]);
        e.call(STRING_BUFFER_APPEND, &args![frame, text]);
        let reporter = e.mem.u32(ERROR_REPORTER);
        let handled = e
            .vcall(reporter, 0x0c, &args![3u32, hash, buffer, file, line])
            .u32();
        if handled != 0 {
            panic!("INT3 at {int3_address:08x}");
        }
        e.call(STRING_BUFFER_EXIT, &args![frame]);
    });
}

/// The pool assertion (an element in none of the pool's blocks), see [`assertion_report`].
fn pool_assertion(e: &mut Engine, int3_address: u32) {
    assertion_report(
        e,
        POOL_MESSAGE_TEXT,
        POOL_MESSAGE_HASH,
        POOL_SOURCE_FILE,
        0xaf,
        int3_address,
    );
}

/// Gives an element back to its pool: finds the block that holds it (`block <= element <=
/// block + block_size - 8 - stride`, unsigned), decrements the block's live count and, when it
/// reaches zero, returns the block to the allocator (slot 8) and fills its slot in the pool's
/// array with the last block. The element is not put on a free list. An element in no block
/// raises [`pool_assertion`]. A null element does nothing.
fn pool_release(
    e: &mut Engine,
    pool: u32,
    element: u32,
    block_size: u32,
    stride: u32,
    int3_address: u32,
) {
    if element == 0 {
        return;
    }
    let last_element = block_size - 8 - stride;
    let count = e.mem.i32(pool + 4);
    let data = e.mem.u32(pool);
    let mut found = None;
    let mut index = 0;
    while index < count {
        let block = e.mem.u32(data + 4 * index as u32);
        if element >= block && element <= block + last_element {
            found = Some(index as u32);
            break;
        }
        index += 1;
    }
    let Some(index) = found else {
        pool_assertion(e, int3_address);
        return;
    };
    let block = e.mem.u32(data + 4 * index);
    let live = e.mem.u32(block + block_size - 4).wrapping_sub(1);
    e.mem.set_u32(block + block_size - 4, live);
    if live != 0 {
        return;
    }
    let data = e.mem.u32(pool);
    let block = e.mem.u32(data + 4 * index);
    let allocator = memory_allocator(e);
    e.vcall(allocator, 8, &args![block, block_size]);
    let size = e.mem.i32(pool + 4).wrapping_sub(1);
    e.mem.set_i32(pool + 4, size);
    if size as u32 != index {
        let data = e.mem.u32(pool);
        let last = e.mem.u32(data + 4 * size as u32);
        e.mem.set_u32(data + 4 * index, last);
    }
}

/// The growth the exe inlines before appending `count` elements to an `hkArray`: when the
/// capacity is below the new size, reserves the larger of the new size and twice the
/// capacity (`00c908c0` with `allocator` and `element_size`). Returns the new size.
fn array_reserve_for_append(
    e: &mut Engine,
    allocator: u32,
    array: u32,
    count: i32,
    element_size: u32,
) -> i32 {
    let new_size = e.mem.i32(array + 4).wrapping_add(count);
    let capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, wanted, element_size],
        );
    }
    new_size
}

// Translated from 00e22970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkArrayBase<hkgpMeshInternals::RawTriangle>::_append` (Xbox PDB; `this` is the array in ECX;
/// stack: `allocator`, `source`, `count`): makes room for `count` elements of `0x30` bytes
/// (see [`array_reserve_for_append`]), copies them from `source` to the end (each copy is
/// skipped when its destination address is 0) and adds `count` to the size.
pub fn hk_array_raw_triangle_append(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    source: u32,
    count: i32,
) {
    let array = this.addr();
    let new_size = array_reserve_for_append(e, allocator, array, count, 0x30);
    let size = e.mem.u32(array + 4);
    let mut destination = size.wrapping_mul(0x30).wrapping_add(e.mem.u32(array));
    let mut from = source;
    let mut remaining = count;
    while remaining > 0 {
        if destination != 0 {
            for word in 0..12 {
                let value = e.mem.u32(from + 4 * word);
                e.mem.set_u32(destination + 4 * word, value);
            }
        }
        destination = destination.wrapping_add(0x30);
        from = from.wrapping_add(0x30);
        remaining -= 1;
    }
    e.mem.set_i32(array + 4, new_size);
}

// Translated from 00e229f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `count` words to an `hkArray` of words (`this` is the array in ECX; stack:
/// `allocator`, `source`, `count`), growing it as `00e22970` does with an element size of 4;
/// the words are copied to the end and the size increased.
pub fn fn_00e229f0(e: &mut Engine, this: Ptr, allocator: u32, source: u32, count: i32) {
    let array = this.addr();
    let new_size = array_reserve_for_append(e, allocator, array, count, 4);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let mut destination = data.wrapping_add(size.wrapping_mul(4));
    let mut from = source;
    let mut remaining = count;
    while remaining > 0 {
        let value = e.mem.u32(from);
        e.mem.set_u32(destination, value);
        destination = destination.wrapping_add(4);
        from = from.wrapping_add(4);
        remaining -= 1;
    }
    e.mem.set_i32(array + 4, new_size);
}

// Translated from 00e22640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `count` eight-byte elements to an `hkArray` (`this` is the array in ECX; stack:
/// `allocator`, `source`, `count`), growing it with an element size of 8; each copy is
/// skipped when its destination address is 0.
pub fn fn_00e22640(e: &mut Engine, this: Ptr, allocator: u32, source: u32, count: i32) {
    let array = this.addr();
    let new_size = array_reserve_for_append(e, allocator, array, count, 8);
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    let mut destination = data.wrapping_add(size.wrapping_mul(8));
    let mut from = source;
    let mut remaining = count;
    while remaining > 0 {
        if destination != 0 {
            let low = e.mem.u32(from);
            e.mem.set_u32(destination, low);
            let high = e.mem.u32(from + 4);
            e.mem.set_u32(destination + 4, high);
        }
        destination = destination.wrapping_add(8);
        from = from.wrapping_add(8);
        remaining -= 1;
    }
    e.mem.set_i32(array + 4, new_size);
}

// Translated from 00e22c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a three-word record (`this` is ECX; stack: `first`, `pair`): `this[0] = first`,
/// `this[1] = pair[0]`, `this[2] = pair[1]`. Returns `this`.
pub fn fn_00e22c80(e: &mut Engine, this: Ptr, first: u32, pair: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), first);
    let low = e.mem.u32(pair.addr());
    let high = e.mem.u32(pair.addr() + 4);
    e.mem.set_u32(this.addr() + 4, low);
    e.mem.set_u32(this.addr() + 8, high);
    this
}

/// The pair of words in the function-local static at `0126ebe8` (guard: bit 0 of `0126ebf0`):
/// the first time a function that uses it runs it sets the guard bit and zeroes the pair.
fn static_zero_pair(e: &mut Engine) -> (u32, u32) {
    if e.mem.u8(0x0126_ebf0) & 1 == 0 {
        let guard = e.mem.u32(0x0126_ebf0);
        e.mem.set_u32(0x0126_ebf0, guard | 1);
        e.mem.set_u32(0x0126_ebe8, 0);
        e.mem.set_u32(0x0126_ebec, 0);
    }
    (e.mem.u32(0x0126_ebe8), e.mem.u32(0x0126_ebec))
}

// Translated from 00e22ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initialises a 0x54-byte record (`this` is ECX): the zero vector (`01267e30`) at `+0`, `+0x10`
/// and `+0x20`, the float at `0104da48` at `+0x40`, zero at `+0x44` (byte) and `+0x48`, and at
/// `+0x4c` / `+0x50` the two words of a function-local static pair (`0126ebe8`, `0126ebec`),
/// which this function zeroes the first time it runs (guard bit 0 of `0126ebf0`). Returns
/// `this`.
pub fn fn_00e22ca0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    let large = e.mem.u32(0x0104_da48);
    for offset in [0u32, 0x10, 0x20] {
        for word in 0..4 {
            let value = e.mem.u32(ZERO_VECTOR + 4 * word);
            e.mem.set_u32(t + offset + 4 * word, value);
        }
    }
    e.mem.set_u32(t + 0x40, x87_quiet_bits(large));
    e.mem.set_u8(t + 0x44, 0);
    e.mem.set_u32(t + 0x48, 0);
    let (low, high) = static_zero_pair(e);
    e.mem.set_u32(t + 0x4c, low);
    e.mem.set_u32(t + 0x50, high);
    this
}

// Translated from 00e22ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 14 words `+0x10` to `+0x44` of the element `source` into `this` (a vertex's
/// data: ECX; stack: `source`). Returns `this`.
pub fn fn_00e22ec0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for word in 0..14u32 {
        let value = e.mem.u32(source.addr() + 0x10 + 4 * word);
        e.mem.set_u32(this.addr() + 0x10 + 4 * word, value);
    }
    this
}

// Translated from 00e22f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 15 words `+0x08` to `+0x40` of the element `source` into `this` (a triangle's
/// data: ECX; stack: `source`). Returns `this`.
pub fn fn_00e22f20(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    for word in 0..15u32 {
        let value = e.mem.u32(source.addr() + 8 + 4 * word);
        e.mem.set_u32(this.addr() + 8 + 4 * word, value);
    }
    this
}

// Translated from 00e23080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a 0x24-byte pool element's data (`this` is ECX; stack: `source`): the six words `+0x08`
/// to `+0x1c`, then the 16-bit field at `+0x20` (the exe assigns its bits 0 to 2, then bit 3,
/// then bits 0 to 3 again, which leaves the whole source value) and the 16-bit field at
/// `+0x22`. Returns `this`.
pub fn fn_00e23080(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let t = this.addr();
    let s = source.addr();
    for word in 0..6u32 {
        let value = e.mem.u32(s + 8 + 4 * word);
        e.mem.set_u32(t + 8 + 4 * word, value);
    }
    let destination = e.mem.u16(t + 0x20);
    let step_one = destination ^ ((destination ^ e.mem.u16(s + 0x20)) & 7);
    e.mem.set_u16(t + 0x20, step_one);
    let step_two = ((e.mem.u16(s + 0x20) ^ step_one) & 8) ^ step_one;
    e.mem.set_u16(t + 0x20, step_two);
    let step_three = ((e.mem.u16(s + 0x20) ^ step_two) & 0xf) ^ e.mem.u16(s + 0x20);
    e.mem.set_u16(t + 0x20, step_three);
    let tail = e.mem.u16(s + 0x22);
    e.mem.set_u16(t + 0x22, tail);
    this
}

// Translated from 00e234e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases every block of a pool of `0xa08`-byte blocks (`this` is the pool in ECX) and
/// clears its block count, list head and live count.
pub fn fn_00e234e0(e: &mut Engine, this: Ptr) {
    pool_release_blocks(e, this.addr(), 0xa08);
}

// Translated from 00e239c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e234e0` for a pool of `0x288`-byte blocks.
pub fn fn_00e239c0(e: &mut Engine, this: Ptr) {
    pool_release_blocks(e, this.addr(), 0x288);
}

// Translated from 00e23a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e234e0` for a pool of `0x488`-byte blocks.
pub fn fn_00e23a10(e: &mut Engine, this: Ptr) {
    pool_release_blocks(e, this.addr(), 0x488);
}

// Translated from 00e23530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes a block for a pool of 0x50-byte elements (`this` is the pool in ECX), see
/// [`pool_new_block`] (`0xa08` bytes). Returns the block.
pub fn fn_00e23530(e: &mut Engine, this: Ptr) -> u32 {
    pool_new_block(e, this.addr(), 0xa08, 0x50)
}

// Translated from 00e235d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e23530` for 0x1c-byte elements (`0x388`-byte blocks).
pub fn fn_00e235d0(e: &mut Engine, this: Ptr) -> u32 {
    pool_new_block(e, this.addr(), 0x388, 0x1c)
}

// Translated from 00e23670 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e23530` for 0x14-byte elements (`0x288`-byte blocks).
pub fn fn_00e23670(e: &mut Engine, this: Ptr) -> u32 {
    pool_new_block(e, this.addr(), 0x288, 0x14)
}

// Translated from 00e23710 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e23530` for 0x24-byte elements (`0x488`-byte blocks).
pub fn fn_00e23710(e: &mut Engine, this: Ptr) -> u32 {
    pool_new_block(e, this.addr(), 0x488, 0x24)
}

// Translated from 00e237b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMeshInternals::SimpleCollector::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`):
/// pushes the word at `node + 0x20` on the `hkArray` of words at `this + 4`. (The exe leaves
/// an `FLD1` on the x87 stack; nothing reads it.)
pub fn hkgp_mesh_internals_simple_collector_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) {
    let leaf = e.mem.u32(node.addr() + 0x20);
    array_push_word(e, this.addr() + 4, leaf);
}

// Translated from 00e23800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMeshInternals::SimpleCollector::_scalar_deleting_destructor_` (Xbox PDB; one stack word
/// `flags`): empties the `hkArray` of words at `this + 4` (size at `+8`, capacity at `+0xc`),
/// freeing its storage when it owns it, leaves vtable `010cd664` and, when bit 0 of `flags`
/// is set, frees the object with `operator delete`. Returns `this`.
pub fn hkgp_mesh_internals_simple_collector_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let t = this.addr();
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(t + 0xc);
    e.mem.set_u32(t + 8, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t + 4);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(t + 4, 0);
    e.mem.set_u32(t + 0xc, 0x8000_0000);
    e.mem.set_u32(t, 0x010c_d664);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00e23860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pool destructor for `0xa08`-byte blocks (`this` is the pool in ECX; the Xbox PDB name
/// the map gives is `hkgpAbstractMesh<..>::~hkgpAbstractMesh`, identical code folded): see
/// [`pool_destroy`].
pub fn fn_00e23860(e: &mut Engine, this: Ptr) {
    pool_destroy(e, this.addr(), 0xa08);
}

// Translated from 00e23c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pool destructor for `0x388`-byte blocks, see [`pool_destroy`].
pub fn fn_00e23c70(e: &mut Engine, this: Ptr) {
    pool_destroy(e, this.addr(), 0x388);
}

// Translated from 00e23cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pool destructor for `0x288`-byte blocks, see [`pool_destroy`].
pub fn fn_00e23cf0(e: &mut Engine, this: Ptr) {
    pool_destroy(e, this.addr(), 0x288);
}

// Translated from 00e23d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pool destructor for `0x488`-byte blocks, see [`pool_destroy`].
pub fn fn_00e23d70(e: &mut Engine, this: Ptr) {
    pool_destroy(e, this.addr(), 0x488);
}

// Translated from 00e238e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<Edge, Vertex, Triangle>::reset` (Xbox PDB; `this` is ECX): releases the
/// blocks of the pools at `this + 8` and `this + 0x20` (`00e234e0`).
pub fn hkgp_abstract_mesh_reset_00e238e0(e: &mut Engine, this: Ptr) {
    fn_00e234e0(e, Ptr::new(this.addr() + MESH_VERTEX_POOL));
    fn_00e234e0(e, Ptr::new(this.addr() + MESH_TRIANGLE_POOL));
}

// Translated from 00e24a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<hkgpTriangulatorType<..>>::reset` (Xbox PDB; `this` is ECX): releases the
/// blocks of the pool at `this + 8` (`0x288`-byte blocks, `00e239c0`) and of the pool at
/// `this + 0x20` (`0x488`-byte blocks, `00e23a10`).
pub fn hkgp_abstract_mesh_reset_00e24a80(e: &mut Engine, this: Ptr) {
    fn_00e239c0(e, Ptr::new(this.addr() + MESH_VERTEX_POOL));
    fn_00e23a10(e, Ptr::new(this.addr() + MESH_TRIANGLE_POOL));
}

// Translated from 00e23b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<Edge, Vertex, Triangle>::_scalar_deleting_destructor_` (Xbox PDB; one
/// stack word `flags`): leaves vtable `010eaf50`, releases and destroys the pools at `this +
/// 0x20` and `this + 8` (`0xa08`-byte blocks), leaves vtable `0102e388` and, when bit 0 of
/// `flags` is set, returns the object to the memory router's allocator (slot 8) with the 16-bit
/// size at `this + 4`. Returns `this`.
pub fn hkgp_abstract_mesh_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x010e_af50);
    fn_00e234e0(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e23860(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e234e0(e, Ptr::new(t + MESH_VERTEX_POOL));
    fn_00e23860(e, Ptr::new(t + MESH_VERTEX_POOL));
    e.mem.set_u32(t, 0x0102_e388);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e24de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpAbstractMesh<hkgpTriangulatorType<..>>::_scalar_deleting_destructor_` (Xbox PDB; one
/// stack word `flags`): leaves vtable `010eb088`, releases and destroys the pool at `this +
/// 0x20` (`0x488`-byte blocks) and the pool at `this + 8` (`0x288`-byte blocks), leaves vtable
/// `0102e388` and, when bit 0 of `flags` is set, frees the object as `00e23b50` does. Returns
/// `this`.
pub fn hkgp_abstract_mesh_triangulator_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u8,
) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x010e_b088);
    fn_00e23a10(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e23d70(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e239c0(e, Ptr::new(t + MESH_VERTEX_POOL));
    fn_00e23cf0(e, Ptr::new(t + MESH_VERTEX_POOL));
    e.mem.set_u32(t, 0x0102_e388);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e23af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of a 12-byte `hkArray` of 0x40-byte elements (`this` is ECX; one stack
/// word `flags`; the class is unconfirmed): sets the size to 0, frees the storage when the
/// array owns it, leaves it empty (data 0, capacity word `0x80000000`) and, when bit 0 of
/// `flags` is set, returns the 12-byte object to the memory router's allocator. Returns
/// `this`.
pub fn fn_00e23af0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(t + 8);
    e.mem.set_u32(t + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t);
        let bytes = (capacity & 0x3fff_ffff) << 6;
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 8, 0x8000_0000);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![this, 0xcu32]);
    }
    this
}

// Translated from 00e24760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the `hkArray` of words at `this + 4` (size at `+8`, capacity at `+0xc`; `this` is
/// ECX): size 0, storage freed when owned, data 0 and capacity word `0x80000000`.
pub fn fn_00e24760(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(t + 0xc);
    e.mem.set_u32(t + 8, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t + 4);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(t + 0xc, 0x8000_0000);
    e.mem.set_u32(t + 4, 0);
}

// Translated from 00e22f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGeometryProcessing::PoolAllocator<hkgpMeshBase::Vertex, 32>::release` (Xbox PDB, also the
/// triangle pool's: identical code; `this` is the pool in ECX; stack: `element`): see
/// [`pool_release`] for blocks of `0xa08` bytes and 0x50-byte elements. An element in no block
/// raises the pool assertion, whose `INT3` is at `00e23012`.
pub fn pool_allocator_release_00e22f90(e: &mut Engine, this: Ptr, element: u32) {
    pool_release(e, this.addr(), element, 0xa08, 0x50, 0x00e2_3012);
}

// Translated from 00e24e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Releases an element of a pool of `0x388`-byte blocks and 0x1c-byte elements (`this` is the
/// pool in ECX; stack: `element`): when the element is not null, first empties the `hkArray`
/// of words the element starts with (size 0, storage freed when owned, data 0, capacity word
/// `0x80000000`), then gives it back as [`pool_release`] does; an element in no block raises
/// the pool assertion, whose `INT3` is at `00e24f01`.
pub fn fn_00e24e40(e: &mut Engine, this: Ptr, element: u32) {
    if element == 0 {
        return;
    }
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(element + 8);
    e.mem.set_u32(element + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(element);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(element, 0);
    e.mem.set_u32(element + 8, 0x8000_0000);
    pool_release(e, this.addr(), element, 0x388, 0x1c, 0x00e2_4f01);
}

// Translated from 00e24140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a copy of a vertex to the pool of `0xa08`-byte blocks (`this` is the pool in ECX; stack:
/// `source`): takes the next free element of the first block (making a block with `00e23530`
/// when needed), copies the payload with `00e22ec0` and links it at the front of the live list.
/// Returns the element.
pub fn fn_00e24140(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    let pool = this.addr();
    let element = pool_take_element(e, pool, 0xa08, |e| fn_00e23530(e, this));
    fn_00e22ec0(e, Ptr::new(element), source);
    pool_link_front(e, pool, element);
    element
}

// Translated from 00e241a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00e24140` with the triangle copy `00e22f20`: adds a copy of the triangle `source` to the
/// pool of `0xa08`-byte blocks (`this` is the pool in ECX). Returns the element.
pub fn fn_00e241a0(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    let pool = this.addr();
    let element = pool_take_element(e, pool, 0xa08, |e| fn_00e23530(e, this));
    fn_00e22f20(e, Ptr::new(element), source);
    pool_link_front(e, pool, element);
    element
}

// Translated from 00e247c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a copy of a 0x14-byte element to the pool of `0x288`-byte blocks (`this` is the pool in
/// ECX; stack: `source`): the words `+0x08` and `+0x0c` are copied, and the word at `+0x10`
/// (the exe assigns its bit 0 twice, which leaves the whole source word); the element is
/// linked at the front of the live list. A new block is made with `00e23670` when needed.
pub fn fn_00e247c0(e: &mut Engine, this: Ptr, source: Ptr) {
    let pool = this.addr();
    let element = pool_take_element(e, pool, 0x288, |e| fn_00e23670(e, this));
    let s = source.addr();
    let first = e.mem.u32(s + 8);
    e.mem.set_u32(element + 8, first);
    let second = e.mem.u32(s + 0xc);
    e.mem.set_u32(element + 0xc, second);
    let destination = e.mem.u32(element + 0x10);
    let step_one = destination ^ ((destination ^ e.mem.u32(s + 0x10)) & 1);
    e.mem.set_u32(element + 0x10, step_one);
    let step_two = ((e.mem.u32(s + 0x10) ^ step_one) & 1) ^ e.mem.u32(s + 0x10);
    e.mem.set_u32(element + 0x10, step_two);
    pool_link_front(e, pool, element);
}

// Translated from 00e24850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a copy of a 0x24-byte element to the pool of `0x488`-byte blocks (`this` is the pool in
/// ECX; stack: `source`): free element from the first block (a new block with `00e23710`
/// when needed), data copied with `00e23080`, linked at the front of the live list. Returns
/// the element.
pub fn fn_00e24850(e: &mut Engine, this: Ptr, source: Ptr) -> u32 {
    let pool = this.addr();
    let element = pool_take_element(e, pool, 0x488, |e| fn_00e23710(e, this));
    fn_00e23080(e, Ptr::new(element), source);
    pool_link_front(e, pool, element);
    element
}

// Translated from 00e25f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor of an `hkArray` of 12-byte `hkArray`s, a 16-byte object (`this` is ECX;
/// one stack word `flags`): for each entry, size 0, storage freed when owned (12 bytes per
/// element of capacity), data 0 and capacity word `0x80000000`; then the outer array is
/// emptied the same way and, when bit 0 of `flags` is set, the 16-byte object is returned to
/// the memory router's allocator. Returns `this`.
pub fn fn_00e25f00(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    let allocator = memory_allocator(e);
    let size = e.mem.i32(t + 4);
    let data = e.mem.u32(t);
    if size > 0 {
        let mut entry = data;
        let mut remaining = size;
        while remaining != 0 {
            let entry_allocator = memory_allocator(e);
            let capacity = e.mem.u32(entry + 8);
            e.mem.set_u32(entry + 4, 0);
            if (capacity as i32) >= 0 {
                let storage = e.mem.u32(entry);
                let bytes = (capacity & 0x3fff_ffff).wrapping_mul(12);
                e.vcall(entry_allocator, 0x10, &args![storage, bytes]);
            }
            e.mem.set_u32(entry, 0);
            e.mem.set_u32(entry + 8, 0x8000_0000);
            entry += 12;
            remaining -= 1;
        }
    }
    let capacity = e.mem.u32(t + 8);
    e.mem.set_u32(t + 4, 0);
    if (capacity as i32) >= 0 {
        let storage = e.mem.u32(t);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(12);
        e.vcall(allocator, 0x10, &args![storage, bytes]);
    }
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 8, 0x8000_0000);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        e.vcall(allocator, 8, &args![this, 0x10u32]);
    }
    this
}

// Translated from 00e27400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the object at `this + 0x24c` when there is one (`this` is ECX): its destructor
/// `00e25f00` with flags 0, then the 16-byte block back to the memory router's allocator
/// (slot 8), and the field is cleared.
pub fn fn_00e27400(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + 0x24c) == 0 {
        return;
    }
    let allocator = memory_allocator(e);
    let object = e.mem.u32(t + 0x24c);
    fn_00e25f00(e, Ptr::new(object), 0);
    let object = e.mem.u32(t + 0x24c);
    e.vcall(allocator, 8, &args![object, 0x10u32]);
    e.mem.set_u32(t + 0x24c, 0);
}

// Translated from 00e27df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpTriangulatorType<..>::_scalar_deleting_destructor_` (Xbox PDB; one stack word `flags`):
/// leaves vtable `010eb0e0`, runs `00e275d0` with 1 (not yet translated here), destroys the pool
/// at `this + 0x250` (`0x388`-byte blocks), empties the `hkArray` of 8-byte elements at `this +
/// 0x40` (size at `+0x44`, capacity at `+0x48`), then the base mesh's part as `00e24de0` does
/// for the pools at `this + 0x28` and `this + 0x10` (vtable `010eb088` while it runs), leaves
/// vtable `0102e388` in both vtable words and, when bit 0 of `flags` is set, returns the object
/// to the memory router's allocator with the 16-bit size at `this + 4`. Returns `this`.
pub fn hkgp_triangulator_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0x010e_b0e0);
    e.call(0x00e2_75d0, &args![this, 1u32]);
    fn_00e23c70(e, Ptr::new(t + 0x250));
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(t + 0x48);
    e.mem.set_u32(t + 0x44, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(t + 0x40);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(8);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
    e.mem.set_u32(t + 0x40, 0);
    e.mem.set_u32(t + 0x48, 0x8000_0000);
    e.mem.set_u32(t + 8, 0x010e_b088);
    fn_00e23a10(e, Ptr::new(t + 0x28));
    fn_00e23d70(e, Ptr::new(t + 0x28));
    fn_00e239c0(e, Ptr::new(t + 0x10));
    fn_00e23cf0(e, Ptr::new(t + 0x10));
    e.mem.set_u32(t + 8, 0x0102_e388);
    e.mem.set_u32(t, 0x0102_e388);
    if flags & 1 != 0 {
        let allocator = memory_allocator(e);
        let size = u32::from(e.mem.u16(t + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

// Translated from 00e23900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks an edge up in the mesh's edge table (`this` is ECX; stack: `edge`, a pair (triangle,
/// edge index)): 0 when the triangle is null or the table pointer at `this + 0x24c` is null.
/// The key is the edge's two vertices, `a = triangle.vertex[index]` and `b =
/// triangle.vertex[(9 >> 2 * index) & 3]`; the bucket is `(a.w3 * 0x3442a5 + a.w2 * 0x21528000)
/// ^ (b.w3 * 0x1958e9 - b.w2 * 0x538b8000)` modulo the bucket count at `table + 4` (words
/// `+8` and `+0xc` of each vertex, 32-bit wrapping products, unsigned `DIV`), a 12-byte entry
/// (`a`, `b`, value) in the bucket's `hkArray` (data, size at `+4`); returns the value of the
/// matching entry, or 0.
pub fn fn_00e23900(e: &mut Engine, this: Ptr, edge: Ptr) -> u32 {
    let triangle = e.mem.u32(edge.addr());
    if triangle == 0 {
        return 0;
    }
    let table = e.mem.u32(this.addr() + 0x24c);
    if table == 0 {
        return 0;
    }
    let index = e.mem.u32(edge.addr() + 4);
    let vertex_a = e.mem.u32(triangle + 8 + 4 * index);
    let next = (9i32 >> ((2 * index) & 31)) as u32 & 3;
    let vertex_b = e.mem.u32(triangle + 8 + 4 * next);
    let first = e
        .mem
        .u32(vertex_a + 0xc)
        .wrapping_mul(0x0034_42a5)
        .wrapping_add(e.mem.u32(vertex_a + 8).wrapping_mul(0x2152_8000));
    let second = e
        .mem
        .u32(vertex_b + 0xc)
        .wrapping_mul(0x0019_58e9)
        .wrapping_sub(e.mem.u32(vertex_b + 8).wrapping_mul(0x538b_8000));
    let bucket_count = e.mem.u32(table + 4);
    let bucket = (first ^ second) % bucket_count;
    let bucket_address = e.mem.u32(table).wrapping_add(bucket.wrapping_mul(12));
    let size = e.mem.i32(bucket_address + 4);
    if size <= 0 {
        return 0;
    }
    let entries = e.mem.u32(bucket_address);
    let mut entry = entries;
    let mut position = 0;
    while position < size {
        if e.mem.u32(entry) == vertex_a && e.mem.u32(entry + 4) == vertex_b {
            return e.mem.u32(entry + 8);
        }
        position += 1;
        entry = entry.wrapping_add(12);
    }
    0
}

// ---------------------------------------------------------------------------------------------
// The shapes and policies the mesh's overlap and distance queries use (vtable methods).
// ---------------------------------------------------------------------------------------------

/// `hkAabbUtil::calcAabb` (`00cbeb70`, cdecl `(vectors, count, out)`): the bounding box of
/// `count` 16-byte vectors, written to `out` (minimum, then maximum).
const CALC_AABB: u32 = 0x00cb_eb70;

// Translated from 00e21ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::IConvexOverlap::IConvexShape::getMaxIndex` (Xbox PDB; `this` is not read): an
/// assertion (message `010ead80`, source `010ead20`, line `0x118`, hash `0xcfa7dfcc`); `INT3`
/// is at `00e21d2e`. Returns 0.
pub fn hkgp_mesh_iconvex_shape_get_max_index(e: &mut Engine, _unused_this: Ptr) -> u32 {
    assertion_report(e, 0x010e_ad80, 0xcfa7_dfcc, 0x010e_ad20, 0x118, 0x00e2_1d2e);
    0
}

// Translated from 00e22520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkGskShape::convertVertexIdsToVertices` (Xbox PDB; `this` and the three stack words are not
/// read): an assertion (message `010eae6c`, source `010eae18`, line `0x2c`, hash `0x50cc4da8`);
/// `INT3` is at `00e2256b`.
pub fn hk_gsk_shape_convert_vertex_ids_to_vertices(
    e: &mut Engine,
    _unused_this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
) {
    assertion_report(e, 0x010e_ae6c, 0x50cc_4da8, 0x010e_ae18, 0x2c, 0x00e2_256b);
}

// Translated from 00e21dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::LineShape::getBoundingBox` (Xbox PDB; `this` is ECX; stack: `out`): the lane-wise
/// minimum of the two end points (`this + 0x10`, `this + 0x20`) at `out` and the maximum at
/// `out + 0x10`.
pub fn hkgp_mesh_line_shape_get_bounding_box(e: &mut Engine, this: Ptr, out: Ptr) {
    let start = load_vec(e, this.addr() + 0x10);
    let end = load_vec(e, this.addr() + 0x20);
    store_vec(e, out.addr(), min_ps(start, end));
    store_vec(e, out.addr() + 0x10, max_ps(start, end));
}

// Translated from 00e21d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::LineShape::getSupportingVertex` (Xbox PDB; `this` is ECX; stack: `direction`,
/// `out`): writes the end point (`this + 0x10`, else `this + 0x20`) with the larger dot
/// product with the direction: the second one unless its dot product is less than the first's
/// (a NaN comparison picks the second).
pub fn hkgp_mesh_line_shape_get_supporting_vertex(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let direction = load_vec(e, direction.addr());
    let start = load_vec(e, this.addr() + 0x10);
    let first = sum_of_three(mul_ps(direction, start));
    let end = load_vec(e, this.addr() + 0x20);
    let second = sum_of_three(mul_ps(end, direction));
    // FCOMPP with ST0 = second, ST1 = first; TEST AH,5; JP picks the end point.
    let condition = fcom_ah(f64::from(second), f64::from(first));
    if !parity_even(condition & 0x05) {
        store_vec(e, out.addr(), start);
    } else {
        store_vec(e, out.addr(), end);
    }
}

// Translated from 00e21e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::WeldingPolicy::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`): the squared
/// distance between `this + 0x10` and the node's box centre `(node[0..] + node[0x10..]) *
/// s` (`s` the float at `01016248`, splatted); when the float at `this + 0x20` is not below it
/// (`FCOMPP`, `TEST AH,1`, `JNZ` skips), stores the word at `node + 0x20` at `this + 0x24`.
pub fn hkgp_mesh_welding_policy_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) {
    let low = load_vec(e, node.addr());
    let scale = e.mem.f32(0x0101_6248);
    let high = load_vec(e, node.addr() + 0x10);
    let centre = mul_ps(add_ps(high, low), [scale; 4]);
    let point = load_vec(e, this.addr() + 0x10);
    let offset = sub_ps(centre, point);
    let squared = sum_of_three(mul_ps(offset, offset));
    let best = x87_quiet(e.mem.f32(this.addr() + 0x20));
    let condition = fcom_ah(f64::from(best), f64::from(squared));
    if condition & 0x01 == 0 {
        let leaf = e.mem.u32(node.addr() + 0x20);
        e.mem.set_u32(this.addr() + 0x24, leaf);
    }
}

// Translated from 00e21ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::TriangleShape::getSupportingVertex` (Xbox PDB; `this` is ECX; stack: `direction`,
/// `out`): the vertex of the triangle at `this + 4` (vertex pointers at `+8`, `+0xc`, `+0x10`)
/// with the largest dot product with the direction, written to `out` with lane 3 replaced by
/// `0x3f000000 | index`. With the dot products `d0`, `d1`, `d2`: when `d1 < d0` the result is
/// vertex 0 if `d2 < d0` and vertex 2 otherwise; else vertex 1 if `d2 < d1` and vertex 2
/// otherwise (unordered comparisons give vertex 2).
pub fn hkgp_mesh_triangle_shape_get_supporting_vertex(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let triangle = e.mem.u32(this.addr() + 4);
    let direction = load_vec(e, direction.addr());
    let v0 = e.mem.u32(triangle + 8);
    let p0 = load_vec(e, v0 + VERTEX_POSITION);
    let v1 = e.mem.u32(triangle + 0xc);
    let v2 = e.mem.u32(triangle + 0x10);
    let d0 = sum_of_three(mul_ps(p0, direction));
    let p1 = load_vec(e, v1 + VERTEX_POSITION);
    let d1 = sum_of_three(mul_ps(p1, direction));
    let p2 = load_vec(e, v2 + VERTEX_POSITION);
    let d2 = sum_of_three(mul_ps(p2, direction));
    let index = if !parity_even(fcom_ah(f64::from(d1), f64::from(d0)) & 0x05) {
        if !parity_even(fcom_ah(f64::from(d2), f64::from(d0)) & 0x05) {
            0
        } else {
            2
        }
    } else if !parity_even(fcom_ah(f64::from(d2), f64::from(d1)) & 0x05) {
        1
    } else {
        2
    };
    let vertex = e.mem.u32(triangle + 8 + 4 * index);
    copy_vec(e, out.addr(), vertex + VERTEX_POSITION);
    e.mem.set_u32(out.addr() + 0xc, index | 0x3f00_0000);
}

// Translated from 00e21fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::TriangleShape::getBoundingBox` (Xbox PDB; `this` is ECX; stack: `out`): the
/// bounding box (`00cbeb70`) of the positions of the three vertices of the triangle at
/// `this + 4`, written to `out`. Returns `out`.
pub fn hkgp_mesh_triangle_shape_get_bounding_box(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let triangle = e.mem.u32(this.addr() + 4);
    e.with_stack(0x30, |e, local| {
        for slot in 0..3u32 {
            let vertex = e.mem.u32(triangle + 8 + 4 * slot);
            copy_vec(e, local.addr() + 0x10 * slot, vertex + VERTEX_POSITION);
        }
        e.call(CALC_AABB, &args![local, 3u32, out]);
    });
    out
}

// Translated from 00e221e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::NearestFeaturePolicy::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`; returns
/// a float): asks `00de9f10` for the distance between the point at `this + 0x10` and the
/// triangle at `node + 0x20` (its three vertex positions; two result vectors go to locals);
/// when the float at `this + 0x20` is above it (`FCOMP`, `TEST AH,0x41`, `JNZ` skips), stores
/// the distance there, the two vectors at `this + 0x30` and `this + 0x40` and the triangle at
/// `this + 0x50`. Returns the float at `this + 0x20`.
pub fn hkgp_mesh_nearest_feature_policy_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let t = this.addr();
    let triangle = e.mem.u32(node.addr() + 0x20);
    let v2 = e.mem.u32(triangle + 0x10);
    let v1 = e.mem.u32(triangle + 0xc);
    let v0 = e.mem.u32(triangle + 8);
    e.with_stack(0x20, |e, locals| {
        let result = e
            .call(
                0x00de_9f10,
                &args![
                    t + 0x10,
                    v0 + VERTEX_POSITION,
                    v1 + VERTEX_POSITION,
                    v2 + VERTEX_POSITION,
                    locals,
                    locals.addr() + 0x10
                ],
            )
            .f32();
        let result = x87_quiet(result);
        let best = x87_quiet(e.mem.f32(t + 0x20));
        let condition = fcom_ah(f64::from(best), f64::from(result));
        if condition & 0x41 == 0 {
            e.mem.set_f32(t + 0x20, result);
            copy_vec(e, t + 0x30, locals.addr());
            copy_vec(e, t + 0x40, locals.addr() + 0x10);
            e.mem.set_u32(t + 0x50, triangle);
        }
    });
    x87_quiet(e.mem.f32(t + 0x20))
}

// Translated from 00e22270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMeshInternals::TriangleOverlap::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`;
/// returns a float): `00deab10` tests the box at `this + 0x10` against the positions of the
/// three vertices of the triangle at `node + 0x20`; a non-zero answer sets the word at
/// `this + 4` (OR 1). Returns 0.0 when that word is non-zero, else 1.0.
pub fn hkgp_mesh_internals_triangle_overlap_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let t = this.addr();
    let triangle = e.mem.u32(node.addr() + 0x20);
    let hit = e.with_stack(0x30, |e, local| {
        for slot in 0..3u32 {
            let vertex = e.mem.u32(triangle + 8 + 4 * slot);
            copy_vec(e, local.addr() + 0x10 * slot, vertex + VERTEX_POSITION);
        }
        e.call(0x00de_ab10, &args![t + 0x10, local]).u32()
    });
    let flag = u32::from(hit != 0);
    let word = e.mem.u32(t + 4) | flag;
    e.mem.set_u32(t + 4, word);
    if word != 0 {
        0.0
    } else {
        1.0
    }
}

// Translated from 00e22480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IConvexOverlapImpl::ShapeBridge::convertVertexIdsToVertices` (Xbox PDB; `this` is ECX;
/// stack: two words that are not read and `out`): calls slot 4 of `this` with a temporary
/// vector (`1.0` from `01012068`, then three of `0.0` from `01011d78`) and `out`. Returns what
/// it returns.
pub fn shape_bridge_convert_vertex_ids_to_vertices(
    e: &mut Engine,
    this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    out: u32,
) -> u32 {
    let one = e.mem.f32(0x0101_2068);
    let zero = e.mem.f32(0x0101_1d78);
    e.with_stack(0x10, |e, vector| {
        store_vec(e, vector.addr(), [one, zero, zero, zero]);
        e.vcall(this.addr(), 4, &args![vector, out]).u32()
    })
}

// Translated from 00e224d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IConvexOverlapImpl::checkOverlap` (Xbox PDB; `this` is ECX; stack: `first`, `second`,
/// `limit` float): calls slot 4 of `this` with `(first, second, 1)`, which returns a float;
/// the result is true unless `limit` is below it (`FCOMPP`, `TEST AH,1`: an unordered pair
/// counts as below).
pub fn convex_overlap_impl_check_overlap(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    limit: f32,
) -> bool {
    let distance = e.vcall(this.addr(), 4, &args![first, second, 1u32]).st0;
    let condition = fcom_ah(f64::from(x87_quiet(limit)), distance);
    condition & 0x01 == 0
}

// Translated from 00e22470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `IConvexOverlapImpl::ShapeBridge::getSupportingVertex` (Xbox PDB): a tail jump. Calls slot 8
/// of the object at `this + 4` with the words it was given after `this` (the stack arguments
/// stay where the caller put them and the callee pops them), and returns its result. Written
/// over the raw argument words because the callee's argument count is not known here.
pub fn shape_bridge_get_supporting_vertex(e: &mut Engine, words: &[u32]) -> Ret {
    let inner = e.mem.u32(words[0] + 4);
    e.vcall(inner, 8, &words[1..])
}

// Translated from 00e226b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::PrismShape<1>::getSupportingVertex` (Xbox PDB; `this` is ECX; stack: `direction`,
/// `out`): the prism at `this + 4` has three vertices (`+8`, `+0xc`, `+0x10`) and six
/// candidate points: each vertex position (lane 3 tagged `0x3f000000 | 2k`) and the position
/// plus `vertex[+0x20..] * vertex[+0x34]` (tag `0x3f000000 | (2k + 1)`). The candidate with the
/// largest dot product with the direction (a later one wins only when strictly greater) is
/// written to `out`.
pub fn hkgp_mesh_prism_shape_get_supporting_vertex(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let prism = e.mem.u32(this.addr() + 4);
    let direction = load_vec(e, direction.addr());
    let first = e.mem.u32(prism + 8);
    let mut start = load_vec(e, first + VERTEX_POSITION);
    start[3] = f32::from_bits(0x3f00_0000);
    store_vec(e, out.addr(), start);
    let mut best = sum_of_three(mul_ps(direction, start));
    for index in 1..6u32 {
        let vertex = e.mem.u32(prism + 8 + 4 * (index >> 1));
        let mut candidate = load_vec(e, vertex + VERTEX_POSITION);
        if index & 1 != 0 {
            let spread = load_vec(e, vertex + 0x30);
            let step = load_vec(e, vertex + 0x20);
            let offset = mul_ps([spread[1]; 4], step);
            candidate = add_ps(offset, candidate);
        }
        let tag = index | 0x3f00_0000;
        candidate[3] = f32::from_bits(tag);
        let value = sum_of_three(mul_ps(direction, candidate));
        let condition = fcom_ah(f64::from(best), f64::from(value));
        if !parity_even(condition & 0x05) {
            store_vec(e, out.addr(), candidate);
            best = value;
            e.mem.set_u32(out.addr() + 0xc, tag);
        }
    }
}

// Translated from 00e227f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::PrismShape<1>::getBoundingBox` (Xbox PDB; `this` is ECX; stack: `out`): the
/// bounding box (`00cbeb70`) of the six points of the prism at `this + 4` (see
/// [`hkgp_mesh_prism_shape_get_supporting_vertex`]; lane 3 of the points holds the tags
/// `0x3f000000 | 0..5`), written to `out`. Returns `out`.
pub fn hkgp_mesh_prism_shape_get_bounding_box(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let prism = e.mem.u32(this.addr() + 4);
    e.with_stack(0x60, |e, local| {
        for index in 0..6u32 {
            let vertex = e.mem.u32(prism + 8 + 4 * (index >> 1));
            let mut point = load_vec(e, vertex + VERTEX_POSITION);
            if index & 1 != 0 {
                let spread = load_vec(e, vertex + 0x30);
                let step = load_vec(e, vertex + 0x20);
                let offset = mul_ps([spread[1]; 4], step);
                point = add_ps(offset, point);
            }
            point[3] = f32::from_bits(index | 0x3f00_0000);
            store_vec(e, local.addr() + 0x10 * index, point);
        }
        e.call(CALC_AABB, &args![local, 6u32, out]);
    });
    out
}

// ---------------------------------------------------------------------------------------------
// Mesh methods that walk the triangle and vertex lists.
// ---------------------------------------------------------------------------------------------

/// The AABB tree query (`hkAabbTreeQueryUtils::queryAabb`, `00d1d570`, cdecl `(tree, box,
/// collector)`): calls the collector's `addLeaf` (slot 8) for each leaf whose box overlaps.
const QUERY_AABB: u32 = 0x00d1_d570;
/// `hkGeometryProcessing::checkPointInsideTriangle` (`00dea6b0`, cdecl `(point, a, b, c,
/// tolerance)`, result in `AL`).
const CHECK_POINT_INSIDE_TRIANGLE: u32 = 0x00de_a6b0;
/// `hkMatrix3::getDeterminant` (`00cd51c0`, thiscall: the matrix of three 16-byte columns,
/// one stack word `out`; returns a pointer to the vector with the result in lane 0).
const GET_DETERMINANT: u32 = 0x00cd_51c0;

// Translated from 00e12230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces a part number (`this` is ECX; stack: `from`, `to`): every triangle whose word at
/// `+0x34` equals `from` gets `to`.
pub fn fn_00e12230(e: &mut Engine, this: Ptr, from: u32, to: u32) {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        if e.mem.u32(triangle + TRIANGLE_PART) == from {
            e.mem.set_u32(triangle + TRIANGLE_PART, to);
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
}

// Translated from 00e121c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Casts a ray through the mesh's AABB tree (`this` is ECX; stack: `from`, `to`, `hit_out`;
/// returns a float): a collector object (vtable `010eade0`, the start at `+0x10`, `to - from`
/// at `+0x20`, no hit at `+0x30`, the fraction 1.0 at `+0x34`) is given to the ray query
/// `00d1d0b0` on the tree at `this + 0x40` with `from`, `to` and the fraction limit 1.0. When
/// `hit_out` is not null the collector's hit word is stored there. Returns the fraction left at
/// `+0x34` of the collector.
pub fn fn_00e121c0(e: &mut Engine, this: Ptr, from: Ptr, to: Ptr, hit_out: u32) -> f32 {
    let start = load_vec(e, from.addr());
    let end = load_vec(e, to.addr());
    e.with_stack(0x40, |e, collector| {
        let c = collector.addr();
        e.mem.set_u32(c, 0x010e_ade0);
        store_vec(e, c + 0x10, start);
        store_vec(e, c + 0x20, sub_ps(end, start));
        e.mem.set_u32(c + 0x30, 0);
        e.mem.set_f32(c + 0x34, 1.0);
        e.call(
            0x00d1_d0b0,
            &args![this.addr() + 0x40, from, to, collector, 1.0f32],
        );
        let fraction = x87_quiet(e.mem.f32(c + 0x34));
        if hit_out != 0 {
            let hit = e.mem.u32(c + 0x30);
            e.mem.set_u32(hit_out, hit);
        }
        fraction
    })
}

// Translated from 00e120e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queries the mesh's AABB tree with a shape (`this` is ECX; stack: `shape`, `tolerance`,
/// `flag`): a collector object (vtable `010eada8`; the tolerance, loaded and stored through the
/// x87 stack, at `+4`; the word at `this + 0x38` at `+8`; the shape at `+0xc`; a hit byte 0 at
/// `+0x10`; the flag byte at `+0x11`) and the shape's bounding box (slot `0x0c` of the shape,
/// into a stack buffer) go to `queryAabb` (`00d1d570`) on the tree at `this + 0x40`. Returns the
/// hit byte.
pub fn fn_00e120e0(e: &mut Engine, this: Ptr, shape: Ptr, tolerance: f32, flag: u8) -> u8 {
    let t = this.addr();
    e.with_stack(0x40, |e, frame| {
        let collector = frame.addr();
        let buffer = frame.addr() + 0x20;
        e.mem.set_u32(collector, 0x010e_ada8);
        e.mem
            .set_u32(collector + 4, x87_quiet_bits(tolerance.to_bits()));
        let word = e.mem.u32(t + 0x38);
        e.mem.set_u32(collector + 8, word);
        e.mem.set_u32(collector + 0xc, shape.addr());
        e.mem.set_u8(collector + 0x10, 0);
        e.mem.set_u8(collector + 0x11, flag);
        let boxed = e.vcall(shape.addr(), 0x0c, &args![buffer]).u32();
        e.call(QUERY_AABB, &args![t + 0x40, boxed, collector]);
        e.mem.u8(collector + 0x10)
    })
}

// Translated from 00e12260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::checkTriangleOverlap` (Xbox PDB; `this` is ECX; stack: three vertex positions `a`,
/// `b`, `c`): a collector object (vtable `010eadd4`, a hit word 0 at `+4`, the three vectors at
/// `+0x10`, `+0x20`, `+0x30`) and the bounding box of the three (`00cbeb70`) go to `queryAabb`
/// on the tree at `this + 0x40`. Returns the hit word.
pub fn mesh_check_triangle_overlap(e: &mut Engine, this: Ptr, a: Ptr, b: Ptr, c: Ptr) -> u32 {
    e.with_stack(0x60, |e, frame| {
        let aabb = frame.addr();
        let collector = frame.addr() + 0x20;
        e.mem.set_u32(collector, 0x010e_add4);
        e.mem.set_u32(collector + 4, 0);
        copy_vec(e, collector + 0x10, a.addr());
        copy_vec(e, collector + 0x20, b.addr());
        copy_vec(e, collector + 0x30, c.addr());
        e.call(CALC_AABB, &args![collector + 0x10, 3u32, aabb]);
        e.call(QUERY_AABB, &args![this.addr() + 0x40, aabb, collector]);
        e.mem.u32(collector + 4)
    })
}

// Translated from 00e122d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the edge of a triangle that runs from vertex `a` to vertex `b` (`this` is ECX; stack:
/// `out`, `a`, `b`): in list order, each triangle's edges are tried as 2 (vertex 2 to vertex 0),
/// 0 (0 to 1) and 1 (1 to 2). The first match writes (triangle, edge index) to `out`; with no
/// match `out` gets the zero pair of the function-local static at `0126ebe8` (see
/// [`static_zero_pair`]).
pub fn fn_00e122d0(e: &mut Engine, this: Ptr, out: Ptr, a: u32, b: u32) {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        for (start, end, index) in [(2u32, 0u32, 2u32), (0, 1, 0), (1, 2, 1)] {
            if e.mem.u32(triangle + 8 + 4 * start) == a && e.mem.u32(triangle + 8 + 4 * end) == b {
                e.mem.set_u32(out.addr(), triangle);
                e.mem.set_u32(out.addr() + 4, index);
                return;
            }
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    let (low, high) = static_zero_pair(e);
    e.mem.set_u32(out.addr(), low);
    e.mem.set_u32(out.addr() + 4, high);
}

// Translated from 00e12640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::fetchPositions` (Xbox PDB; `this` is ECX; stack: `array`, an `hkArray` of 16-byte
/// vectors): makes room for one more element per vertex (the larger of the new size and twice
/// the capacity, `00c908c0`), appends the position of every vertex and returns the number of
/// vertices.
pub fn mesh_fetch_positions(e: &mut Engine, this: Ptr, array: Ptr) -> i32 {
    let t = this.addr();
    let count = e.mem.i32(t + MESH_VERTEX_COUNT);
    let allocator = memory_allocator(e);
    let old_size = e.mem.u32(array.addr() + 4);
    let new_size = array_reserve_for_append(e, allocator, array.addr(), count, 0x10);
    e.mem.set_i32(array.addr() + 4, new_size);
    let mut destination = old_size
        .wrapping_mul(16)
        .wrapping_add(e.mem.u32(array.addr()));
    let mut vertex = e.mem.u32(t + MESH_VERTEX_HEAD);
    while vertex != 0 {
        copy_vec(e, destination, vertex + VERTEX_POSITION);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
        destination = destination.wrapping_add(0x10);
    }
    e.mem.i32(t + MESH_VERTEX_COUNT)
}

// Translated from 00e12890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Collects the mesh's triangles (`this` is ECX; stack: `array`, `reset`): empties the `hkArray`
/// of words, makes room for one per triangle (the count at `this + 0x30`; `00c908c0` with
/// exactly that many), pushes every triangle (setting its part word at `+0x34` to -1 first when
/// the low byte of `reset` is non-zero), then, with more than one, sorts the array with
/// `00e22a50(data, 0, size - 1, 0)` (the exe overwrites the low byte of its own `reset`
/// parameter with 0 and passes that word on as the last argument; callers pass it zero-extended).
pub fn fn_00e12890(e: &mut Engine, this: Ptr, array: Ptr, reset: u8) {
    let t = this.addr();
    let a = array.addr();
    e.mem.set_u32(a + 4, 0);
    let count = e.mem.i32(t + 0x30);
    let allocator = memory_allocator(e);
    let capacity = (e.mem.u32(a + 8) & 0x3fff_ffff) as i32;
    if capacity < count {
        e.call(ARRAY_RESERVE, &args![allocator, a, count, 4u32]);
    }
    let mut triangle = e.mem.u32(t + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        if reset != 0 {
            e.mem.set_u32(triangle + TRIANGLE_PART, 0xffff_ffff);
        }
        let size = e.mem.u32(a + 4);
        let data = e.mem.u32(a);
        e.mem.set_u32(data + size * 4, triangle);
        e.mem.set_u32(a + 4, size + 1);
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    let size = e.mem.i32(a + 4);
    if size > 1 {
        let data = e.mem.u32(a);
        e.call(0x00e2_2a50, &args![data, 0u32, size - 1, 0u32]);
    }
}

// Translated from 00e12920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::isClosedManifold` (Xbox PDB; `this` is ECX): true when every triangle has all
/// three neighbour links set (a link word whose bits above the low two are zero is unset).
pub fn mesh_is_closed_manifold(e: &mut Engine, this: Ptr) -> bool {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        for slot in 0..3u32 {
            if e.mem.u32(triangle + TRIANGLE_LINK_0 + 4 * slot) & 0xffff_fffc == 0 {
                return false;
            }
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    true
}

// Translated from 00e13660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Locates a point (`this` is ECX; stack: `point`, `out`, `found_out`; returns a float):
/// initialises a 0x54-byte record (`00e22ca0`), runs `00e13210` (not yet translated: `hkgpMesh::locate`)
/// on `this` with `point` and the record, copies the record's vector at `+0` to `out`, stores
/// whether its byte at `+0x44` is non-zero at `found_out` when that is not null, and returns the
/// float at `+0x40` of the record.
pub fn fn_00e13660(e: &mut Engine, this: Ptr, point: u32, out: Ptr, found_out: u32) -> f32 {
    e.with_stack(0x60, |e, record| {
        fn_00e22ca0(e, record);
        e.call(0x00e1_3210, &args![this, point, record]);
        let distance = x87_quiet(e.mem.f32(record.addr() + 0x40));
        if found_out != 0 {
            let found = e.mem.u8(record.addr() + 0x44) != 0;
            e.mem.set_u8(found_out, u8::from(found));
        }
        copy_vec(e, out.addr(), record.addr());
        distance
    })
}

// Translated from 00e136c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::getVolume` (Xbox PDB; `this` is ECX; returns a float): 0.0 unless the mesh is a
/// closed manifold (`00e12920`); then the sum over the triangles of the first lane of
/// `hkMatrix3::getDeterminant` (`00cd51c0`) of the three vertex positions, each added through
/// the x87 stack and rounded to a float, divided by the double at `01034208`.
pub fn mesh_get_volume(e: &mut Engine, this: Ptr) -> f32 {
    let mut sum_bits = 0u32;
    if mesh_is_closed_manifold(e, this) {
        let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
        while triangle != 0 {
            let next = e.with_stack(0x40, |e, frame| {
                let columns = frame.addr();
                let result = frame.addr() + 0x30;
                for slot in 0..3u32 {
                    let vertex = e.mem.u32(triangle + TRIANGLE_VERTEX_0 + 4 * slot);
                    copy_vec(e, columns + 0x10 * slot, vertex + VERTEX_POSITION);
                }
                let at = e.call(GET_DETERMINANT, &args![columns, result]).u32();
                let determinant = e.mem.u32(at);
                sum_bits = x87_store32(x87_add(x87_load32(determinant), x87_m32(sum_bits)));
                e.mem.u32(triangle + ELEMENT_NEXT)
            });
            triangle = next;
        }
        let divisor = e.mem.f64(0x0103_4208);
        sum_bits = x87_store32(x87_div(x87_load32(sum_bits), divisor));
    }
    x87_quiet(f32::from_bits(sum_bits))
}

// Translated from 00e13750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::rebuildConvexHull` (Xbox PDB; `this` is ECX): deletes the current hull
/// (`00e114c0`); with more than 3 vertices makes an `hkgpConvexHull` (a 0x14-byte block from the
/// router's allocator, its 16-bit size word set to 0x14, constructed by `00d76ca0`, stored at
/// `this + 0xe4`), builds it from the vertex positions (`00e12640` into a local array;
/// `00d74d40` makes the build configuration, its bytes `+9` = 0 and `+0xa` = 1; `00d771c0`
/// builds), and checks `00d74e10` (the dimension count). A hull that is not 3-dimensional is
/// deleted (slot 0 with flag 1) and the field cleared; the local array is freed either way.
/// Returns false only in that case; with 3 vertices or fewer it returns true without a hull.
pub fn mesh_rebuild_convex_hull(e: &mut Engine, this: Ptr) -> bool {
    let t = this.addr();
    fn_00e114c0(e, this);
    if e.mem.i32(t + MESH_VERTEX_COUNT) <= 3 {
        return true;
    }
    e.with_stack(0x20, |e, frame| {
        let array = frame;
        let config = frame.byte_add(0xc);
        e.call(0x00d7_4d40, &args![config]);
        e.mem.set_u8(config.addr() + 9, 0);
        e.mem.set_u8(config.addr() + 0xa, 1);
        let allocator = memory_allocator(e);
        let hull = e.vcall(allocator, 4, &args![0x14u32]).u32();
        e.mem.set_u16(hull + 4, 0x14);
        let hull = e.call(0x00d7_6ca0, &args![hull]).u32();
        e.mem.set_u32(t + MESH_CONVEX_HULL, hull);
        e.mem.set_u32(array.addr(), 0);
        e.mem.set_u32(array.addr() + 4, 0);
        e.mem.set_u32(array.addr() + 8, 0x8000_0000);
        mesh_fetch_positions(e, this, array);
        let size = e.mem.u32(array.addr() + 4);
        let data = e.mem.u32(array.addr());
        e.call(0x00d7_71c0, &args![hull, data, size, config]);
        let dimensions = e.call(0x00d7_4e10, &args![hull]).u32();
        let built = dimensions == 3;
        if !built {
            let hull = e.mem.u32(t + MESH_CONVEX_HULL);
            if hull != 0 {
                e.vcall(hull, 0, &args![1u32]);
            }
            e.mem.set_u32(t + MESH_CONVEX_HULL, 0);
        }
        let allocator = memory_allocator(e);
        let capacity = e.mem.u32(array.addr() + 8);
        e.mem.set_u32(array.addr() + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array.addr());
            let bytes = (capacity & 0x3fff_ffff) << 4;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        built
    })
}

// Translated from 00e13870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The determinant of three edge vectors at one corner (no register input; stack: `triangle`,
/// `index`; returns a float): 0.0 when the neighbour link `index` of the triangle is unset;
/// otherwise, with `p` the position of vertex `index`, the neighbour's vertex of the same link
/// index (`q`), the vertex `(index + 2) % 3` of the triangle (`r`) and the neighbour's vertex
/// `(link index + 2) % 3` (`s`), `hkMatrix3::getDeterminant` (`00cd51c0`) of the columns
/// `q - p`, `r - p`, `s - p`, lane 0 of the result.
pub fn fn_00e13870(e: &mut Engine, triangle: Ptr, index: u32) -> f32 {
    let t = triangle.addr();
    let link = e.mem.u32(t + TRIANGLE_LINK_0 + 4 * index);
    let neighbour_index = link & 3;
    let neighbour = link & 0xffff_fffc;
    if neighbour == 0 {
        return 0.0;
    }
    let own = e.mem.u32(t + 8 + 4 * index);
    let p = load_vec(e, own + VERTEX_POSITION);
    let other = e.mem.u32(neighbour + 8 + 4 * neighbour_index);
    let q = load_vec(e, other + VERTEX_POSITION);
    let previous = (0x12i32 >> ((2 * index) & 31)) as u32 & 3;
    let neighbour_previous = (0x12i32 >> ((2 * neighbour_index) & 31)) as u32 & 3;
    let own_previous = e.mem.u32(t + 8 + 4 * previous);
    let r = load_vec(e, own_previous + VERTEX_POSITION);
    let far = e.mem.u32(neighbour + 8 + 4 * neighbour_previous);
    let s = load_vec(e, far + VERTEX_POSITION);
    e.with_stack(0x40, |e, frame| {
        let columns = frame.addr();
        store_vec(e, columns, sub_ps(q, p));
        store_vec(e, columns + 0x10, sub_ps(r, p));
        store_vec(e, columns + 0x20, sub_ps(s, p));
        let at = e
            .call(GET_DETERMINANT, &args![columns, columns + 0x30])
            .u32();
        x87_quiet(e.mem.f32(at))
    })
}

// Translated from 00e13f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes a triangle from the mesh (`this` is ECX; stack: `triangle`): clears the back link in
/// each neighbour that points at it (the link's low two bits are the slot) and its own three
/// links, unlinks it from the triangle list (`this + 0x20` is the pool: head at `+0x0c`, count at
/// `+0x10`) and releases it to the pool (`00e22f90`).
pub fn fn_00e13f10(e: &mut Engine, this: Ptr, triangle: u32) {
    for slot in 0..3u32 {
        let link = e.mem.u32(triangle + TRIANGLE_LINK_0 + 4 * slot);
        if link & 0xffff_fffc != 0 {
            let neighbour = link & 0xffff_fffc;
            let neighbour_slot = link & 3;
            e.mem
                .set_u32(neighbour + TRIANGLE_LINK_0 + 4 * neighbour_slot, 0);
        }
        e.mem.set_u32(triangle + TRIANGLE_LINK_0 + 4 * slot, 0);
    }
    let next = e.mem.u32(triangle + ELEMENT_NEXT);
    let previous = e.mem.u32(triangle + ELEMENT_PREVIOUS);
    let pool = this.addr() + MESH_TRIANGLE_POOL;
    if next != 0 {
        e.mem.set_u32(next + ELEMENT_PREVIOUS, previous);
    }
    if previous != 0 {
        e.mem.set_u32(previous + ELEMENT_NEXT, next);
        let count = e.mem.u32(pool + POOL_COUNT);
        e.mem.set_u32(pool + POOL_COUNT, count.wrapping_sub(1));
    } else {
        let count = e.mem.u32(pool + POOL_COUNT);
        e.mem.set_u32(pool + POOL_COUNT, count.wrapping_sub(1));
        e.mem.set_u32(pool + POOL_HEAD, next);
    }
    pool_allocator_release_00e22f90(e, Ptr::new(pool), triangle);
}

// Translated from 00e14a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::removeOrphanVertices` (Xbox PDB; `this` is ECX): sets the float at `+0x30` of every
/// vertex to 0.0, sets it to 1.0 for the three vertices of every triangle, then unlinks and
/// releases (`00e22f90`) each vertex whose float compares equal to 0.0 (a NaN counts as not
/// equal). When any were removed the convex hull is deleted (`00e114c0`). Returns how many.
pub fn mesh_remove_orphan_vertices(e: &mut Engine, this: Ptr) -> u32 {
    let t = this.addr();
    let mut removed = 0u32;
    let mut vertex = e.mem.u32(t + MESH_VERTEX_HEAD);
    while vertex != 0 {
        e.mem.set_f32(vertex + 0x30, 0.0);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
    let mut triangle = e.mem.u32(t + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        for slot in 0..3u32 {
            let corner = e.mem.u32(triangle + TRIANGLE_VERTEX_0 + 4 * slot);
            e.mem.set_f32(corner + 0x30, 1.0);
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    let mut vertex = e.mem.u32(t + MESH_VERTEX_HEAD);
    if vertex == 0 {
        return removed;
    }
    while vertex != 0 {
        let current = vertex;
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
        // FCOM with 0.0 as ST0: TEST AH,0x44; JP skips unless the values are equal.
        let value = x87_m32(e.mem.u32(current + 0x30));
        let condition = fcom_ah(0.0, value);
        if !parity_even(condition & 0x44) {
            let next = e.mem.u32(current + ELEMENT_NEXT);
            let previous = e.mem.u32(current + ELEMENT_PREVIOUS);
            let pool = t + MESH_VERTEX_POOL;
            if next != 0 {
                e.mem.set_u32(next + ELEMENT_PREVIOUS, previous);
            }
            if previous != 0 {
                e.mem.set_u32(previous + ELEMENT_NEXT, next);
            } else {
                e.mem.set_u32(pool + POOL_HEAD, next);
            }
            let count = e.mem.u32(pool + POOL_COUNT);
            e.mem.set_u32(pool + POOL_COUNT, count.wrapping_sub(1));
            pool_allocator_release_00e22f90(e, Ptr::new(pool), current);
            removed += 1;
        }
    }
    if removed != 0 {
        fn_00e114c0(e, this);
    }
    removed
}

// Translated from 00e14fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes every triangle of a part (`this` is ECX; stack: `part`): each triangle whose word at
/// `+0x34` equals `part` goes through `00e13f10`; then the byte at `this + 0x101` is cleared.
pub fn fn_00e14fe0(e: &mut Engine, this: Ptr, part: u32) {
    let mut triangle = e.mem.u32(this.addr() + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        let next = e.mem.u32(triangle + ELEMENT_NEXT);
        if e.mem.u32(triangle + TRIANGLE_PART) == part {
            fn_00e13f10(e, this, triangle);
        }
        triangle = next;
    }
    e.mem.set_u8(this.addr() + MESH_DIRTY_FLAG, 0);
}

// Translated from 00e15020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::removePartFromClassification` (Xbox PDB; `this` is ECX; stack: `part`,
/// `transform`, `other`, `side`): for each triangle whose word at `+0x30` equals `part`, takes
/// its centroid (`(v1 + v0 + v2)` times the float at `0106c2e0`, as `00e11630` does), transforms
/// it (`hkVector4::setTransformedPos`), locates it in the mesh `other` (`00e22ca0` then
/// `00e13210`), and removes the triangle (`00e13f10`) when the signed byte at `+0x44` of the
/// location record equals the low byte of `side`. Clears the byte at `this + 0x101`.
pub fn mesh_remove_part_from_classification(
    e: &mut Engine,
    this: Ptr,
    part: u32,
    transform: Ptr,
    other: Ptr,
    side: u8,
) {
    let t = this.addr();
    let mut triangle = e.mem.u32(t + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        let next = e.mem.u32(triangle + ELEMENT_NEXT);
        if e.mem.u32(triangle + TRIANGLE_INDEX) == part {
            let v1 = e.mem.u32(triangle + TRIANGLE_VERTEX_1);
            let v0 = e.mem.u32(triangle + TRIANGLE_VERTEX_0);
            let v2 = e.mem.u32(triangle + TRIANGLE_VERTEX_2);
            let p1 = load_vec(e, v1 + VERTEX_POSITION);
            let p0 = load_vec(e, v0 + VERTEX_POSITION);
            let scale = e.mem.f32(THIRD);
            let p2 = load_vec(e, v2 + VERTEX_POSITION);
            let centroid = mul_ps(add_ps(p2, add_ps(p1, p0)), [scale; 4]);
            let matched = e.with_stack(0x70, |e, frame| {
                let point = frame.addr();
                let record = frame.addr() + 0x10;
                store_vec(e, point, centroid);
                e.call(SET_TRANSFORMED_POS, &args![point, transform, point]);
                fn_00e22ca0(e, Ptr::new(record));
                e.call(0x00e1_3210, &args![other, point, record]);
                let signed = i32::from(e.mem.u8(record + 0x44) as i8);
                signed == i32::from(side)
            });
            if matched {
                fn_00e13f10(e, this, triangle);
            }
        }
        triangle = next;
    }
    e.mem.set_u8(t + MESH_DIRTY_FLAG, 0);
}

// Translated from 00e157f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::reset` (Xbox PDB; `this` is ECX): deletes the convex hull (`00e114c0`), sets the
/// vector at `this + 0xf0` to the zero vector `01267e30`, releases the blocks of the vertex and
/// triangle pools (`00e234e0`) and clears the bytes at `this + 0x100` and `this + 0x101`.
pub fn mesh_reset(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    fn_00e114c0(e, this);
    copy_vec(e, t + 0xf0, ZERO_VECTOR);
    fn_00e234e0(e, Ptr::new(t + MESH_VERTEX_POOL));
    fn_00e234e0(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    e.mem.set_u8(t + MESH_PROBLEM_FLAG, 0);
    e.mem.set_u8(t + MESH_DIRTY_FLAG, 0);
}

/// Empties an `hkArray` inlined in an object (data, size, capacity word at `array`, `array + 4`,
/// `array + 8`): size 0, storage freed when owned (`element_bytes` per element of capacity,
/// allocator slot `0x10`), data 0, capacity word `0x80000000`. The router is asked first.
fn array_clear_and_deallocate(e: &mut Engine, array: u32, element_bytes: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    array_free_storage_with_capacity(e, allocator, array, capacity, element_bytes);
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

/// The storage release of [`array_clear_and_deallocate`] for a capacity word read earlier.
fn array_free_storage_with_capacity(
    e: &mut Engine,
    allocator: u32,
    array: u32,
    capacity: u32,
    element_bytes: u32,
) {
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        let bytes = (capacity & 0x3fff_ffff).wrapping_mul(element_bytes);
        e.vcall(allocator, 0x10, &args![data, bytes]);
    }
}

// Translated from 00e18a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::~hkgpMesh` (Xbox PDB; `this` is ECX): leaves vtable `010eb000`, resets the mesh
/// (`00e157f0`), empties the three `hkArray`s at `this + 0xd8` (words), `this + 0xcc` and
/// `this + 0xc0` (16-byte elements), leaves vtable `010d40a4` at `this + 0x40` and runs
/// `00d9be30` (the tree's clear) and `00d9c010` (its node manager's destructor, on
/// `this + 0x5c`), leaves vtable `010eaf50`, releases and destroys the pools at `this + 0x20` and
/// `this + 8` (`00e234e0`, `00e23860`) and leaves vtable `0102e388`.
pub fn hkgp_mesh_destructor(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t, 0x010e_b000);
    mesh_reset(e, this);
    array_clear_and_deallocate(e, t + 0xd8, 4);
    array_clear_and_deallocate(e, t + 0xcc, 16);
    array_clear_and_deallocate(e, t + 0xc0, 16);
    e.mem.set_u32(t + 0x40, 0x010d_40a4);
    e.call(0x00d9_be30, &args![t + 0x40]);
    e.call(0x00d9_c010, &args![t + 0x40 + 0x1c]);
    e.mem.set_u32(t, 0x010e_af50);
    fn_00e234e0(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e23860(e, Ptr::new(t + MESH_TRIANGLE_POOL));
    fn_00e234e0(e, Ptr::new(t + MESH_VERTEX_POOL));
    fn_00e23860(e, Ptr::new(t + MESH_VERTEX_POOL));
    e.mem.set_u32(t, 0x0102_e388);
}

// Translated from 00e1b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::hkgpMesh` (Xbox PDB; `this` is ECX): sets the 16-bit word at `+6` to 1, the
/// vertex and triangle pools and the three arrays to their empty state (capacity word
/// `0x80000000`), vtable `010eb000`, constructs the tree at `this + 0x40` (`00d9c110`), sets the
/// float at `this + 0x104` from `010d51c4`, the word at `this + 0x38` to `011bdee0`, clears the
/// convex hull pointer and sets `this + 0x108` to `2 * (f(c) * d1 * d2)` where `f` is the CRT
/// function at `00ec6040` (called with the double at `01021928`, result rounded to a float) and
/// `d1`, `d2` the doubles at `010290b0` and `0109cc78`; `this + 0x10c` becomes 0.0 and the
/// mesh is reset (`00e157f0`). Returns `this`.
pub fn hkgp_mesh_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u16(t + 6, 1);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, 0);
    e.mem.set_u32(t + 0x14, 0);
    e.mem.set_u32(t + 0x18, 0);
    e.mem.set_u32(t + 0x10, 0x8000_0000);
    e.mem.set_u32(t + 0x20, 0);
    e.mem.set_u32(t + 0x24, 0);
    e.mem.set_u32(t + 0x28, 0x8000_0000);
    e.mem.set_u32(t + 0x2c, 0);
    e.mem.set_u32(t + 0x30, 0);
    e.mem.set_u32(t, 0x010e_b000);
    e.call(0x00d9_c110, &args![t + 0x40]);
    let tolerance = e.mem.u32(0x010d_51c4);
    for offset in [0xc0u32, 0xcc, 0xd8] {
        e.mem.set_u32(t + offset, 0);
        e.mem.set_u32(t + offset + 4, 0);
        e.mem.set_u32(t + offset + 8, 0x8000_0000);
    }
    e.mem
        .set_u32(t + MESH_EDGE_TOLERANCE, x87_quiet_bits(tolerance));
    let argument = e.mem.f64(0x0102_1928);
    e.mem.set_u32(t + 0x38, 0x011b_dee0);
    e.mem.set_u32(t + MESH_CONVEX_HULL, 0);
    let root = e.call(0x00ec_6040, &args![argument]).st0;
    let root = x87_load32(x87_store32(root));
    let product = x87_mul(
        x87_mul(root, e.mem.f64(0x0102_90b0)),
        e.mem.f64(0x0109_cc78),
    );
    let doubled = x87_add(product, product);
    e.mem
        .set_u32(t + MESH_NORMAL_TOLERANCE, x87_store32(doubled));
    e.mem.set_u32(t + 0x10c, 0);
    mesh_reset(e, this);
    this
}

// Translated from 00e25ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the mesh's edge table (`this` is ECX; the table pointer goes to `this + 0x24c`): a
/// 16-byte object from the router's allocator (slot 4) holding an `hkArray` of 12-byte arrays,
/// reserved for 0x407 (1031) buckets (`00c908c0`) and set to that size with every bucket an
/// empty array (data 0, size 0, capacity word `0x80000000`); a failed allocation leaves the
/// pointer 0. The exe's code to destroy surplus buckets (size above 0x407) is kept; the size is
/// 0 at that point, so it never runs.
pub fn fn_00e25ba0(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    let allocator = memory_allocator(e);
    let table = e.vcall(allocator, 4, &args![0x10u32]).u32();
    if table == 0 {
        e.mem.set_u32(t + 0x24c, 0);
        return;
    }
    e.mem.set_u32(table, 0);
    e.mem.set_u32(table + 8, 0x8000_0000);
    e.mem.set_u32(table + 4, 0);
    e.mem.set_u32(table + 0xc, 0);
    let allocator = memory_allocator(e);
    let mut capacity = (e.mem.u32(table + 8) & 0x3fff_ffff) as i32;
    if capacity < 0x407 {
        capacity = capacity.wrapping_add(capacity);
        if capacity <= 0x407 {
            capacity = 0x407;
        }
        e.call(ARRAY_RESERVE, &args![allocator, table, capacity, 0xcu32]);
    }
    let size = e.mem.i32(table + 4);
    let data = e.mem.u32(table);
    let surplus = size - 0x407;
    let mut buckets = data.wrapping_add(0x3054);
    if surplus > 0 {
        // Unreachable: the size was set to 0 above.
        for _ in 0..surplus {
            let bucket_allocator = memory_allocator(e);
            let bucket_capacity = e.mem.u32(buckets + 8);
            e.mem.set_u32(buckets + 4, 0);
            array_free_storage_with_capacity(e, bucket_allocator, buckets, bucket_capacity, 12);
            e.mem.set_u32(buckets, 0);
            e.mem.set_u32(buckets + 8, 0x8000_0000);
            buckets = buckets.wrapping_add(12);
        }
    }
    let size = e.mem.i32(table + 4);
    let data = e.mem.u32(table);
    let mut slot = data.wrapping_add((size as u32).wrapping_mul(12));
    for _ in 0..(0x407 - size).max(0) {
        if slot != 0 {
            e.mem.set_u32(slot, 0);
            e.mem.set_u32(slot + 4, 0);
            e.mem.set_u32(slot + 8, 0x8000_0000);
        }
        slot = slot.wrapping_add(12);
    }
    e.mem.set_u32(table + 4, 0x407);
    let data = e.mem.u32(table);
    for bucket in 0..0x407u32 {
        e.mem.set_u32(data + bucket * 12 + 4, 0);
    }
    e.mem.set_u32(t + 0x24c, table);
}

// Translated from 00e22320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMeshInternals::RayCaster::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`; returns a
/// float): the ray (start at `this + 0x10`, direction at `this + 0x20`) against the plane of the
/// triangle at `node + 0x20` (its plane vector at `+0x20`). Only when the squared denominator
/// `d = plane . direction` is above the float at `0104da4c`, the fraction `t = (0 - (plane . start +
/// w)) / d` is above 0.0 and below the float at `this + 0x34`, and the hit point `start + t *
/// direction` is inside the triangle (`00dea6b0`, tolerance float from `010c8440`) are the
/// fraction and the triangle recorded at `this + 0x34` and `this + 0x30`. Returns the float at
/// `this + 0x34`.
pub fn hkgp_mesh_internals_ray_caster_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let t = this.addr();
    let triangle = e.mem.u32(node.addr() + 0x20);
    let epsilon = x87_quiet(e.mem.f32(0x0104_da4c));
    let plane = load_vec(e, triangle + TRIANGLE_PLANE);
    let direction = load_vec(e, t + 0x20);
    let denominator = sum_of_three(mul_ps(plane, direction));
    let squared = denominator * denominator;
    let condition = fcom_ah(f64::from(epsilon), f64::from(squared));
    if !parity_even(condition & 0x05) {
        let start = load_vec(e, t + 0x10);
        let along = mul_ps(plane, start);
        let upper = along[2] + plane[3];
        let lower = along[1] + along[0];
        let numerator = e.mem.f32(ZERO_LANES) - (upper + lower);
        let fraction = numerator / denominator;
        let hit = add_ps(mul_ps([fraction; 4], direction), start);
        let condition = fcom_ah(0.0, f64::from(fraction));
        if !parity_even(condition & 0x05) {
            let best = x87_quiet(e.mem.f32(t + 0x34));
            let condition = fcom_ah(f64::from(best), f64::from(fraction));
            if condition & 0x41 == 0 {
                let v0 = e.mem.u32(triangle + TRIANGLE_VERTEX_0);
                let v1 = e.mem.u32(triangle + TRIANGLE_VERTEX_1);
                let v2 = e.mem.u32(triangle + TRIANGLE_VERTEX_2);
                let tolerance = x87_quiet_bits(e.mem.u32(0x010c_8440));
                let inside = e.with_stack(0x10, |e, point| {
                    store_vec(e, point.addr(), hit);
                    e.call(
                        CHECK_POINT_INSIDE_TRIANGLE,
                        &args![
                            point,
                            v0 + VERTEX_POSITION,
                            v1 + VERTEX_POSITION,
                            v2 + VERTEX_POSITION,
                            tolerance
                        ],
                    )
                    .bool()
                });
                if inside {
                    e.mem.set_f32(t + 0x34, x87_quiet(fraction));
                    e.mem.set_u32(t + 0x30, triangle);
                }
            }
        }
    }
    x87_quiet(e.mem.f32(t + 0x34))
}

// Translated from 00e22050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::CollideShapeTriangle::addLeaf` (Xbox PDB; `this` is ECX; stack: `node`; returns a
/// float): tests whether the triangle at `node + 0x20` (plane at `+0x20`) comes within the
/// distance `-this[+4]` (a value below the double at `01017cf8` becomes the float at
/// `010959f4`) of the shape at `this + 0xc`. The shape's supporting vertex (slot 8) along the plane
/// vector gives a distance `plane . v + w`, which must not be below that value; the vertex along
/// the negated plane (the vector xor `010c44d0`) must give `plane . v + w` not below `0 - value`;
/// then the shape at `this + 8` (slot 4, given the shape at `this + 0xc`, a stack object with
/// vtable `010eadb4` and the triangle, and the byte at `this + 0x11`) must report a float below
/// `this[+4]`. In that case the byte at `this + 0x10` is set and 0.0 is returned; otherwise 1.0.
pub fn hkgp_mesh_collide_shape_triangle_add_leaf(e: &mut Engine, this: Ptr, node: Ptr) -> f32 {
    let t = this.addr();
    let leaf = e.mem.u32(node.addr() + 0x20);
    let sign_mask = load_vec(e, 0x010c_44d0);
    let negated_bits = x87_quiet_bits(e.mem.u32(t + 4)) ^ 0x8000_0000;
    let negated = f64::from(f32::from_bits(negated_bits));
    let plane = load_vec(e, leaf + TRIANGLE_PLANE);
    let rounded = f32::from_bits(x87_store32(negated));
    let mut value = f64::from(rounded);
    let condition = fcom_ah(value, e.mem.f64(0x0101_7cf8));
    if !parity_even(condition & 0x05) {
        value = f64::from(x87_quiet(e.mem.f32(0x0109_59f4)));
    }
    let value = f32::from_bits(x87_store32(value));
    let flipped: Vec4 = std::array::from_fn(|lane| {
        f32::from_bits(plane[lane].to_bits() ^ sign_mask[lane].to_bits())
    });
    let shape = e.mem.u32(t + 0xc);
    e.with_stack(0x80, |e, frame| {
        let f = frame.addr();
        let supporting = f + 0x20;
        let along = f + 0x40;
        let against = f + 0x50;
        store_vec(e, along, plane);
        store_vec(e, against, flipped);
        e.vcall(shape, 8, &args![along, supporting]);
        let vertex = load_vec(e, supporting);
        let plane_now = load_vec(e, leaf + TRIANGLE_PLANE);
        let products = mul_ps(plane_now, vertex);
        let distance = (products[2] + plane_now[3]) + (products[1] + products[0]);
        let condition = fcom_ah(f64::from(value), f64::from(distance));
        if condition & 0x41 == 0 {
            return 1.0;
        }
        e.vcall(shape, 8, &args![against, supporting]);
        let vertex = load_vec(e, supporting);
        let plane_now = load_vec(e, leaf + TRIANGLE_PLANE);
        let products = mul_ps(plane_now, vertex);
        let distance = (products[2] + plane_now[3]) + (products[1] + products[0]);
        let lower = e.mem.f32(ZERO_LANES) - value;
        let condition = fcom_ah(f64::from(lower), f64::from(distance));
        if !parity_even(condition & 0x05) {
            return 1.0;
        }
        let flag = u32::from(e.mem.u8(t + 0x11));
        let query = f + 0x18;
        e.mem.set_u32(query, 0x010e_adb4);
        e.mem.set_u32(query + 4, leaf);
        let other = e.mem.u32(t + 8);
        let answer = e.vcall(other, 4, &args![shape, query, flag]).f32();
        let limit = x87_quiet(e.mem.f32(t + 4));
        let condition = fcom_ah(f64::from(limit), f64::from(x87_quiet(answer)));
        if condition & 0x41 != 0 {
            return 1.0;
        }
        e.mem.set_u8(t + 0x10, 1);
        0.0
    })
}

// Translated from 00e22580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::PrismPolicy::addHit` (Xbox PDB; `this` is ECX; stack: `first`, `second`, two
/// tree nodes whose leaves (at `+0x20`) are prisms with three vertex pointers at `+8`, `+0xc`,
/// `+0x10`): counts the call (`this + 4`). Two prisms that share a vertex give true. Otherwise
/// the pair is counted (`this + 8`), the shapes of the two prisms (stack objects with vtable
/// `010eaea4` and the prism at `+4`) are given to slot 8 of the object at `this + 0xc` with the
/// limit 0.0, and when it reports overlap the float at `+0x3c` of all six vertices is set to 0.0.
/// Returns false in that case too.
pub fn hkgp_mesh_prism_policy_add_hit(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> bool {
    let t = this.addr();
    let prism_a = e.mem.u32(first.addr() + 0x20);
    let prism_b = e.mem.u32(second.addr() + 0x20);
    let calls = e.mem.u32(t + 4);
    e.mem.set_u32(t + 4, calls.wrapping_add(1));
    for slot_a in 0..3u32 {
        for slot_b in 0..3u32 {
            let vertex_a = e.mem.u32(prism_a + 8 + 4 * slot_a);
            if vertex_a == e.mem.u32(prism_b + 8 + 4 * slot_b) {
                return true;
            }
        }
    }
    let pairs = e.mem.u32(t + 8);
    e.mem.set_u32(t + 8, pairs.wrapping_add(1));
    let checker = e.mem.u32(t + 0xc);
    let overlap = e.with_stack(0x10, |e, frame| {
        let shape_b = frame.addr();
        let shape_a = frame.addr() + 8;
        e.mem.set_u32(shape_b, 0x010e_aea4);
        e.mem.set_u32(shape_a, 0x010e_aea4);
        e.mem.set_u32(shape_a + 4, prism_a);
        e.mem.set_u32(shape_b + 4, prism_b);
        e.vcall(checker, 8, &args![shape_a, shape_b, 0.0f32]).bool()
    });
    if overlap {
        for prism in [prism_a, prism_b] {
            for slot in 0..3u32 {
                let vertex = e.mem.u32(prism + 8 + 4 * slot);
                e.mem.set_f32(vertex + 0x3c, 0.0);
            }
        }
    }
    false
}

// Translated from 00e1d900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::appendFromConvexHull` (Xbox PDB; `this` is ECX; stack: `hull`): makes a geometry
/// record (two empty `hkArray`s, 0x18 bytes: data 0, size 0, capacity word `0x80000000` each)
/// from the hull (`00d76410` with 0 and the record), appends it to the mesh with the identity
/// transform `01268370` and part number -1 (`00e1ce10`, not yet translated) and destroys the
/// record (`00cce530`).
pub fn mesh_append_from_convex_hull(e: &mut Engine, this: Ptr, hull: Ptr) {
    e.with_stack(0x18, |e, geometry| {
        let g = geometry.addr();
        e.mem.set_u32(g + 8, 0x8000_0000);
        e.mem.set_u32(g + 0x14, 0x8000_0000);
        e.mem.set_u32(g, 0);
        e.mem.set_u32(g + 4, 0);
        e.mem.set_u32(g + 0xc, 0);
        e.mem.set_u32(g + 0x10, 0);
        e.call(0x00d7_6410, &args![hull, 0u32, geometry]);
        e.call(
            0x00e1_ce10,
            &args![this, geometry, IDENTITY_TRANSFORM, 0xffff_ffffu32],
        );
        e.call(0x00cc_e530, &args![geometry]);
    });
}

// ---------------------------------------------------------------------------------------------
// Loops over the mesh that need the locate query, the hull and the splitting of triangles.
// ---------------------------------------------------------------------------------------------

/// The CRT `sqrt` (`00ec6040`): the double in `ST0` is the leading `f64` argument, the result is
/// returned in `ST0`.
const CRT_SQRT: u32 = 0x00ec_6040;
/// A `float` the distance sum multiplies its projection with (`0104a494`).
const PROJECTION_SCALE: u32 = 0x0104_a494;

/// The next local point of the two loops of [`fn_00e14160`]: `((dir . p) * k) * dir + p` for a
/// position `p` (`k` the float at `0104a494`).
fn projected_point(direction: Vec4, position: Vec4, scale: f32) -> Vec4 {
    let along = sum_of_three(mul_ps(direction, position)) * scale;
    add_ps(mul_ps([along; 4], direction), position)
}

// Translated from 00e14160 (decompiled, FalloutNV.exe 1.4.0.525)
/// A sum of distances along a direction with an early stop (`this` is ECX; stack: `direction`,
/// `limit`; returns a float): for each vertex position `p`, then for each triangle's centroid
/// `p` (`(p1 + p0 + p2)` times the float at `0106c2e0`), it takes the point `((direction . p) * k) *
/// direction + p` (`k` the float at `0104a494`), locates it (`00e13660`, found-flag not
/// wanted) and adds the square root (CRT `00ec6040`, rounded to a float) of the distance it
/// returns to a float sum, added through the x87 stack. A sum that reaches `limit`
/// (`FCOMPP`, `TEST AH,0x41`, `JNP`: less than or equal, ordered) stops the vertex loop and
/// goes on to the triangles, where each addition that reaches the limit returns. The first
/// triangle's term is always added. Returns the sum.
pub fn fn_00e14160(e: &mut Engine, this: Ptr, direction: Ptr, limit: f32) -> f32 {
    let t = this.addr();
    let mut sum_bits = 0u32;
    let limit = f64::from(x87_quiet(limit));
    let scale = e.mem.f32(PROJECTION_SCALE);
    let mut vertex = e.mem.u32(t + MESH_VERTEX_HEAD);
    while vertex != 0 {
        let dir = load_vec(e, direction.addr());
        let position = load_vec(e, vertex + VERTEX_POSITION);
        let point = projected_point(dir, position, scale);
        add_located_distance(e, this, point, &mut sum_bits);
        let condition = fcom_ah(limit, x87_load32(sum_bits));
        if !parity_even(condition & 0x41) {
            break;
        }
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
    }
    let mut triangle = e.mem.u32(t + MESH_TRIANGLE_HEAD);
    while triangle != 0 {
        let v0 = e.mem.u32(triangle + TRIANGLE_VERTEX_0);
        let v1 = e.mem.u32(triangle + TRIANGLE_VERTEX_1);
        let v2 = e.mem.u32(triangle + TRIANGLE_VERTEX_2);
        let p0 = load_vec(e, v0 + VERTEX_POSITION);
        let p1 = load_vec(e, v1 + VERTEX_POSITION);
        let third = e.mem.f32(THIRD);
        let p2 = load_vec(e, v2 + VERTEX_POSITION);
        let centroid = mul_ps(add_ps(add_ps(p1, p0), p2), [third; 4]);
        let dir = load_vec(e, direction.addr());
        let scale = e.mem.f32(PROJECTION_SCALE);
        let point = projected_point(dir, centroid, scale);
        add_located_distance(e, this, point, &mut sum_bits);
        let condition = fcom_ah(limit, x87_load32(sum_bits));
        if !parity_even(condition & 0x41) {
            break;
        }
        triangle = e.mem.u32(triangle + ELEMENT_NEXT);
    }
    x87_quiet(f32::from_bits(sum_bits))
}

/// Locates `point` in the mesh (`00e13660`) and adds the square root of the returned distance
/// to the float whose bits are `sum_bits`, as the exe does through the x87 stack.
fn add_located_distance(e: &mut Engine, this: Ptr, point: Vec4, sum_bits: &mut u32) {
    e.with_stack(0x30, |e, frame| {
        store_vec(e, frame.addr(), point);
        let distance = fn_00e13660(e, this, frame.addr(), Ptr::new(frame.addr() + 0x10), 0);
        let root = e.call(CRT_SQRT, &args![f64::from(distance)]).st0;
        let root_bits = x87_store32(root);
        *sum_bits = x87_store32(x87_add(x87_load32(root_bits), x87_m32(*sum_bits)));
    });
}

// Translated from 00e13fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::normalize` (Xbox PDB; `this` is ECX; stack: `centre_out`, `extent_out`, `size`;
/// returns a float): `centre_out` and `extent_out` start as vectors of `0.0` (`01011d78`) and
/// `1.0` (`01012068`). With no vertices that is all. Otherwise the vertices' bounding box gives
/// the centre `k * (max + min)` (`k` the float at `0102295c`) and the extent `max - min`; the
/// largest of the first three extent lanes (a later lane replaces it unless it is below it) is
/// put in lanes 0 to 2. The mesh is translated by the centre vector (`00e11ca0`, as it is in
/// `centre_out`) and scaled (`00e11b00`) by the `RCPPS` estimate, with one Newton step, of
/// `(1 / size) * extent` (lane 3 taken as the float at `010c7170 + 12`); `extent_out` then holds
/// `(1 / size) * extent` and `centre_out` the centre times the float at `01012054`. A mesh
/// that had a convex hull gets it rebuilt (`00e13750`). Returns lane 0 of `extent_out`.
pub fn mesh_normalize(
    e: &mut Engine,
    this: Ptr,
    centre_out: Ptr,
    extent_out: Ptr,
    size: f32,
) -> f32 {
    let t = this.addr();
    let zero = e.mem.f32(0x0101_1d78);
    store_vec(e, centre_out.addr(), [zero; 4]);
    let one = e.mem.f32(0x0101_2068);
    store_vec(e, extent_out.addr(), [one; 4]);
    if e.mem.u32(t + MESH_VERTEX_COUNT) == 0 {
        return x87_quiet(e.mem.f32(extent_out.addr()));
    }
    let had_hull = e.mem.u32(t + MESH_CONVEX_HULL) != 0;
    let head = e.mem.u32(t + MESH_VERTEX_HEAD);
    let first = load_vec(e, head + VERTEX_POSITION);
    let mut minimum = first;
    let mut maximum = first;
    let mut vertex = head;
    while vertex != 0 {
        let position = load_vec(e, vertex + VERTEX_POSITION);
        vertex = e.mem.u32(vertex + ELEMENT_NEXT);
        minimum = min_ps(minimum, position);
        maximum = max_ps(maximum, position);
    }
    let sum = add_ps(maximum, minimum);
    store_vec(e, centre_out.addr(), sum);
    let half = e.mem.f32(0x0102_295c);
    store_vec(e, centre_out.addr(), mul_ps([half; 4], sum));
    store_vec(e, extent_out.addr(), sub_ps(maximum, minimum));
    // The largest of three lanes, through the x87 stack: FCOM of the later lane against the
    // earlier one; TEST AH,5; JP keeps the later lane unless it is below.
    let mut biggest = x87_quiet(e.mem.f32(extent_out.addr()));
    for lane in 1..3u32 {
        let later = x87_quiet(e.mem.f32(extent_out.addr() + 4 * lane));
        let condition = fcom_ah(f64::from(later), f64::from(biggest));
        if !parity_even(condition & 0x05) {
            // later < biggest: the earlier value stays.
        } else {
            biggest = later;
        }
    }
    for lane in 0..3u32 {
        e.mem.set_f32(extent_out.addr() + 4 * lane, biggest);
    }
    fn_00e11ca0(e, this, centre_out);
    let inverse = f32::from_bits(x87_store32(x87_div(1.0, f64::from(x87_quiet(size)))));
    let extent = load_vec(e, extent_out.addr());
    let scaled = mul_ps([inverse; 4], extent);
    store_vec(e, extent_out.addr(), scaled);
    let fourth = e.mem.f32(0x010c_7170 + 12);
    let divisor: Vec4 = [scaled[0], scaled[1], scaled[2], fourth];
    let estimate: Vec4 = std::array::from_fn(|lane| rcp_estimate(divisor[lane]));
    let refined: Vec4 = std::array::from_fn(|lane| {
        let product = (estimate[lane] * divisor[lane]) * estimate[lane];
        (estimate[lane] + estimate[lane]) - product
    });
    store_vec(e, extent_out.addr(), refined);
    mesh_apply_scale(e, this, extent_out);
    mesh_update_plane_equations(e, this);
    store_vec(e, extent_out.addr(), scaled);
    let minus_one = e.mem.f32(0x0101_2054);
    let centre = load_vec(e, centre_out.addr());
    store_vec(e, centre_out.addr(), mul_ps([minus_one; 4], centre));
    if had_hull {
        mesh_rebuild_convex_hull(e, this);
    }
    x87_quiet(e.mem.f32(extent_out.addr()))
}

/// The pushBack the exe inlines for an `hkArray` of 16-byte elements: the router first, room
/// through `00c90940` when the size equals the capacity, then the four words of the vector at
/// `source` are copied to the end and the size is incremented.
fn array_push_vec(e: &mut Engine, array: u32, source: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, array, 0x10u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    copy_vec(e, data + size * 16, source);
    e.mem.set_u32(array + 4, size + 1);
}

// Translated from 00e153f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `getLoopSurfaceArea` (Xbox PDB; no register input; stack: `loop`, an `hkArray` of 8-byte
/// elements (triangle, edge index); returns a float): 0.0 for an empty loop. Otherwise the
/// positions of the start vertex and the end vertex (`(9 >> 2 * index) & 3`) of every edge go to a
/// local array (reserved for twice the edge count, `00c908c0`), whose bounding box
/// (`00cbeb70`) gives `(max - min)`; the result is `z + (y + x)` of that vector. The local array's
/// storage is freed.
pub fn get_loop_surface_area(e: &mut Engine, edges: Ptr) -> f32 {
    let a = edges.addr();
    let count = e.mem.i32(a + 4);
    if count == 0 {
        return 0.0;
    }
    e.with_stack(0x40, |e, frame| {
        let array = frame.addr();
        let aabb = frame.addr() + 0x20;
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let wanted = count.wrapping_mul(2);
        let allocator = memory_allocator(e);
        let mut capacity = (e.mem.u32(array + 8) & 0x3fff_ffff) as i32;
        if capacity < wanted {
            capacity = capacity.wrapping_add(capacity);
            if wanted >= capacity {
                capacity = wanted;
            }
            e.call(ARRAY_RESERVE, &args![allocator, array, capacity, 0x10u32]);
        }
        for index in 0..e.mem.i32(a + 4) {
            let data = e.mem.u32(a);
            let triangle = e.mem.u32(data + 8 * index as u32);
            let edge = e.mem.u32(data + 8 * index as u32 + 4);
            let start = e.mem.u32(triangle + 8 + 4 * edge) + VERTEX_POSITION;
            array_push_vec(e, array, start);
            let next = (9i32 >> ((2 * edge) & 31)) as u32 & 3;
            let end = e.mem.u32(triangle + 8 + 4 * next) + VERTEX_POSITION;
            array_push_vec(e, array, end);
        }
        let positions = e.mem.u32(array);
        let size = e.mem.u32(array + 4);
        e.call(CALC_AABB, &args![positions, size, aabb]);
        let minimum = load_vec(e, aabb);
        let maximum = load_vec(e, aabb + 0x10);
        let extent = sub_ps(maximum, minimum);
        let result = sum_of_three(extent);
        let allocator = memory_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            let bytes = (capacity & 0x3fff_ffff) << 4;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
        x87_quiet(result)
    })
}

/// The pushBack the exe inlines for an `hkArray` of 8-byte elements (a pair of words): the
/// router first, room through `00c90940` with 8 when the size equals the capacity, then the pair
/// at the end unless that address is 0, and the size is incremented.
fn array_push_pair(e: &mut Engine, array: u32, first: u32, second: u32) {
    let allocator = memory_allocator(e);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        e.call(ARRAY_GROW, &args![allocator, array, 8u32]);
    }
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    let slot = data.wrapping_add(size.wrapping_mul(8));
    if slot != 0 {
        e.mem.set_u32(slot, first);
        e.mem.set_u32(slot + 4, second);
    }
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00e150e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips the triangles of a part (`this` is ECX; stack: `part`): for each triangle whose word at
/// `+0x30` equals `part`, the plane vector at `+0x20` is multiplied by the float at `01012054`
/// (splatted), its three (triangle, edge index) pairs go to a local `hkArray` of pairs and its
/// vertex pointers at `+8` and `+0xc` are swapped. Then each pair's link in its triangle and the
/// back link in the neighbour (if any) are cleared, `00e124d0` (not yet translated) is run on
/// the array, the byte at `this + 0x101` is cleared and the array's storage is freed.
pub fn fn_00e150e0(e: &mut Engine, this: Ptr, part: u32) {
    let t = this.addr();
    e.with_stack(0x10, |e, frame| {
        let array = frame.addr();
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        let mut triangle = e.mem.u32(t + MESH_TRIANGLE_HEAD);
        while triangle != 0 {
            if e.mem.u32(triangle + TRIANGLE_INDEX) == part {
                let factor = e.mem.f32(0x0101_2054);
                let plane = load_vec(e, triangle + TRIANGLE_PLANE);
                store_vec(e, triangle + TRIANGLE_PLANE, mul_ps([factor; 4], plane));
                for edge in 0..3u32 {
                    array_push_pair(e, array, triangle, edge);
                }
                let second = e.mem.u32(triangle + TRIANGLE_VERTEX_1);
                let first = e.mem.u32(triangle + TRIANGLE_VERTEX_0);
                e.mem.set_u32(triangle + TRIANGLE_VERTEX_0, second);
                e.mem.set_u32(triangle + TRIANGLE_VERTEX_1, first);
            }
            triangle = e.mem.u32(triangle + ELEMENT_NEXT);
        }
        let mut index = 0;
        while index < e.mem.i32(array + 4) {
            let data = e.mem.u32(array);
            let owner = e.mem.u32(data + 8 * index as u32);
            let edge = e.mem.u32(data + 8 * index as u32 + 4);
            let link = e.mem.u32(owner + TRIANGLE_LINK_0 + 4 * edge);
            if link & 0xffff_fffc != 0 {
                let neighbour = link & 0xffff_fffc;
                let slot = link & 3;
                e.mem.set_u32(neighbour + TRIANGLE_LINK_0 + 4 * slot, 0);
            }
            e.mem.set_u32(owner + TRIANGLE_LINK_0 + 4 * edge, 0);
            index += 1;
        }
        e.call(0x00e1_24d0, &args![this, array]);
        e.mem.set_u8(t + MESH_DIRTY_FLAG, 0);
        let allocator = memory_allocator(e);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            let bytes = (capacity & 0x3fff_ffff) << 3;
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

// Translated from 00e15220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkgpMesh::enumerateTriangleOverlaps` (Xbox PDB; `this` is ECX; stack: three positions `a`, `b`,
/// `c`, and `out`, an `hkArray` of words): a collector object (vtable `010eaf00`, an empty
/// `hkArray` at `+4`) and the bounding box of the three positions (`00cbeb70`) go to
/// `queryAabb` on the tree at `this + 0x40`. Every collected word is the address of a triangle;
/// the ones whose three vertex positions overlap the query triangle (`00deab10`, with the three
/// query positions and the three of the triangle) are pushed on `out` (growing it through
/// `00c90940`). The collector's array is freed.
pub fn mesh_enumerate_triangle_overlaps(
    e: &mut Engine,
    this: Ptr,
    a: Ptr,
    b: Ptr,
    c: Ptr,
    out: Ptr,
) {
    let t = this.addr();
    e.with_stack(0xc0, |e, frame| {
        let query = frame.addr();
        let candidate = frame.addr() + 0x30;
        let aabb = frame.addr() + 0x60;
        let collector = frame.addr() + 0x80;
        copy_vec(e, query, a.addr());
        copy_vec(e, query + 0x10, b.addr());
        copy_vec(e, query + 0x20, c.addr());
        e.mem.set_u32(collector, 0x010e_af00);
        e.mem.set_u32(collector + 4, 0);
        e.mem.set_u32(collector + 8, 0);
        e.mem.set_u32(collector + 0xc, 0x8000_0000);
        e.call(CALC_AABB, &args![query, 3u32, aabb]);
        e.call(QUERY_AABB, &args![t + 0x40, aabb, collector]);
        let mut index = 0;
        while index < e.mem.i32(collector + 8) {
            let data = e.mem.u32(collector + 4);
            let leaf = e.mem.u32(data + 4 * index as u32);
            for slot in 0..3u32 {
                let vertex = e.mem.u32(leaf + 8 + 4 * slot);
                copy_vec(e, candidate + 0x10 * slot, vertex + VERTEX_POSITION);
            }
            let hit = e.call(0x00de_ab10, &args![query, candidate]).u32();
            if hit != 0 {
                let leaf = e.mem.u32(e.mem.u32(collector + 4) + 4 * index as u32);
                array_push_word(e, out.addr(), leaf);
            }
            index += 1;
        }
        let allocator = memory_allocator(e);
        let capacity = e.mem.u32(collector + 0xc);
        e.mem.set_u32(collector + 8, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(collector + 4);
            let bytes = (capacity & 0x3fff_ffff).wrapping_mul(4);
            e.vcall(allocator, 0x10, &args![data, bytes]);
        }
    });
}

/// The vertex the three splitting functions build on a stack source before they copy it into
/// the vertex pool: words `+0x40` and `+0x44` are -1; the rest of the 0x48 bytes is whatever
/// the stack held (see the incomplete markers). Returns the vertex the pool made, with its
/// position set to `position` and its vector at `+0x30` to the zero vector.
fn new_vertex_with_position(e: &mut Engine, this: Ptr, position: Ptr) -> u32 {
    e.with_stack(0x48, |e, source| {
        e.mem.set_u32(source.addr() + 0x40, 0xffff_ffff);
        e.mem.set_u32(source.addr() + 0x44, 0xffff_ffff);
        // The words +0x10..+0x3f and the rest of the source vertex are uninitialised stack in
        // the exe (here zero), so the vector at +0x20 of the new vertex is unknown.
        let vertex = fn_00e24140(e, Ptr::new(this.addr() + MESH_VERTEX_POOL), source);
        copy_vec(e, vertex + 0x30, ZERO_VECTOR);
        copy_vec(e, vertex + VERTEX_POSITION, position.addr());
        vertex
    })
}

// Translated from 00e191a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits an edge at a new vertex (`this` is ECX; stack: `out`, `first`, `second`, `position`,
/// `flag`): makes a vertex at `position` (see [`new_vertex_with_position`]) and runs
/// `00e18ed0` (not yet translated) with `(out, first, second, vertex, flag)`. Returns `out`.
// Translation incomplete: 00e191a0 - the new vertex is copied from an uninitialised stack object, so its vector at +0x20 comes from stack garbage in the exe
pub fn fn_00e191a0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    first: u32,
    second: u32,
    position: Ptr,
    flag: u32,
) -> Ptr {
    let vertex = new_vertex_with_position(e, this, position);
    e.call(0x00e1_8ed0, &args![this, out, first, second, vertex, flag]);
    out
}

// Translated from 00e19200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits a triangle into three at a new vertex (`this` is ECX; stack: `triangle`, `position`
/// and one word that is not read): makes a vertex at `position` (see
/// [`new_vertex_with_position`]) and two copies of the triangle (`00e241a0`), then puts the
/// vertex at `+8` of the original, at `+0xc` of the first copy and at `+0x10` of the second.
/// A link is a triangle pointer with an edge index in its low two bits. Rewiring, in the exe's
/// order: the first copy takes the original's link 2 and, when that neighbour exists, its
/// slot (the index in the link) points at (first copy, 2); the second copy takes link 0, and
/// that neighbour's slot points at the second copy; the original's link 0 becomes (second
/// copy, 1) and the second copy's link 1 the original; the first copy's link 0 becomes (second
/// copy, 2) and the second copy's link 2 the first copy; the first copy's link 1 becomes
/// (original, 2) and the original's link 2 (first copy, 1). Returns the second copy.
// Translation incomplete: 00e19200 - the new vertex is copied from an uninitialised stack object, so its vector at +0x20 comes from stack garbage in the exe
pub fn fn_00e19200(e: &mut Engine, this: Ptr, triangle: u32, position: Ptr, _unused_3: u32) -> u32 {
    let vertex = new_vertex_with_position(e, this, position);
    let pool = Ptr::new(this.addr() + MESH_TRIANGLE_POOL);
    let first_copy = fn_00e241a0(e, pool, Ptr::new(triangle));
    let second_copy = fn_00e241a0(e, pool, Ptr::new(triangle));
    e.mem.set_u32(triangle + TRIANGLE_VERTEX_0, vertex);
    e.mem.set_u32(first_copy + TRIANGLE_VERTEX_1, vertex);
    e.mem.set_u32(second_copy + TRIANGLE_VERTEX_2, vertex);
    let link = e.mem.u32(triangle + 0x1c);
    let slot = link & 3;
    let neighbour = link & 0xffff_fffc;
    e.mem
        .set_u32(first_copy + 0x1c, neighbour.wrapping_add(slot));
    if neighbour != 0 {
        e.mem
            .set_u32(neighbour + 0x14 + 4 * slot, first_copy.wrapping_add(2));
    }
    let link = e.mem.u32(triangle + 0x14);
    let slot = link & 3;
    let neighbour = link & 0xffff_fffc;
    e.mem
        .set_u32(second_copy + 0x14, slot.wrapping_add(neighbour));
    if neighbour != 0 {
        e.mem.set_u32(neighbour + 0x14 + 4 * slot, second_copy);
    }
    e.mem.set_u32(triangle + 0x14, second_copy.wrapping_add(1));
    e.mem.set_u32(second_copy + 0x18, triangle);
    e.mem
        .set_u32(first_copy + 0x14, second_copy.wrapping_add(2));
    e.mem.set_u32(second_copy + 0x1c, first_copy);
    e.mem.set_u32(first_copy + 0x18, triangle.wrapping_add(2));
    e.mem.set_u32(triangle + 0x1c, first_copy.wrapping_add(1));
    second_copy
}

// Translated from 00e19cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a triangle on three vertices (`this` is ECX; stack: `a`, `b`, `c`, vertex pointers): a
/// triangle element copied (`00e241a0`) from an uninitialised stack object, then the word at
/// `+0x30` set to -1, its three links to 0, its vertices to `a`, `b`, `c` and its plane computed
/// by `setPlane` (checking on; the result is not used). Returns the triangle.
// Translation incomplete: 00e19cf0 - the new triangle is copied from an uninitialised stack object, so its words +0x34..+0x40 come from stack garbage in the exe
pub fn fn_00e19cf0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32) -> u32 {
    let triangle = e.with_stack(0x50, |e, source| {
        // The source is uninitialised stack in the exe (zero here).
        fn_00e241a0(e, Ptr::new(this.addr() + MESH_TRIANGLE_POOL), source)
    });
    e.mem.set_u32(triangle + TRIANGLE_INDEX, 0xffff_ffff);
    e.mem.set_u32(triangle + TRIANGLE_LINK_0, 0);
    e.mem.set_u32(triangle + TRIANGLE_LINK_0 + 4, 0);
    e.mem.set_u32(triangle + TRIANGLE_LINK_0 + 8, 0);
    e.mem.set_u32(triangle + TRIANGLE_VERTEX_2, c);
    e.mem.set_u32(triangle + TRIANGLE_VERTEX_1, b);
    e.mem.set_u32(triangle + TRIANGLE_VERTEX_0, a);
    mesh_set_plane(
        e,
        this,
        Ptr::new(a + VERTEX_POSITION),
        Ptr::new(b + VERTEX_POSITION),
        Ptr::new(c + VERTEX_POSITION),
        Ptr::new(triangle + TRIANGLE_PLANE),
        1,
    );
    triangle
}

// ---------------------------------------------------------------------------------------------
// Edge-table walking and an array copy.
// ---------------------------------------------------------------------------------------------

// Translated from 00e23a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns an `hkArray` of words (`this` is the array in ECX; stack: `source`, another array):
/// when the capacity is below the source's size, frees the storage if it is owned (slot `0x10`,
/// four bytes per element of capacity), asks for `4 * size` bytes (slot `0x0c`, through a stack
/// word that comes back as the granted size) and sets the capacity to the granted bytes
/// divided by 4 (rounding toward zero); then sets the size and copies the words. Returns `this`.
pub fn fn_00e23a60(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let t = this.addr();
    let s = source.addr();
    let allocator = memory_allocator(e);
    let capacity_word = e.mem.u32(t + 8);
    let capacity = capacity_word & 0x3fff_ffff;
    let size = e.mem.i32(s + 4);
    if (capacity as i32) < size {
        if (capacity_word as i32) >= 0 {
            let data = e.mem.u32(t);
            e.vcall(allocator, 0x10, &args![data, capacity.wrapping_mul(4)]);
        }
        e.with_stack(4, |e, request| {
            e.mem.set_u32(request.addr(), (size as u32).wrapping_mul(4));
            let storage = e.vcall(allocator, 0x0c, &args![request]).u32();
            e.mem.set_u32(t, storage);
            let granted = e.mem.i32(request.addr());
            // CDQ; AND EDX,3; ADD EAX,EDX; SAR EAX,2: a signed division by 4.
            let rounded = granted.wrapping_add((granted >> 31) & 3) >> 2;
            e.mem.set_i32(t + 8, rounded);
        });
    }
    let count = e.mem.i32(s + 4);
    e.mem.set_i32(t + 4, count);
    let from = e.mem.u32(s);
    let to = e.mem.u32(t);
    for word in 0..count.max(0) as u32 {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
    this
}

/// Follows the neighbour link of an edge (triangle, edge index): the link word of the triangle
/// at slot `index`; returns the neighbour triangle and its edge index, or `None` when the
/// neighbour is unset (bits above the low two clear).
fn follow_link(e: &Engine, triangle: u32, index: u32) -> Option<(u32, u32)> {
    let link = e.mem.u32(triangle + TRIANGLE_LINK_0 + 4 * index);
    if link & 0xffff_fffc == 0 {
        None
    } else {
        Some((link & 0xffff_fffc, link & 3))
    }
}

// Translated from 00e27ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the edge table values along an edge and its neighbour (`this` is ECX; stack: the edge
/// `triangle` and `index` by value, and `out`, two records of 0x20 bytes): the first byte of
/// each record starts at 0. For each of up to two edges, beginning with the one given, the
/// table value (`00e23900`) is looked up; when it is not 0 the `hkArray` it points to is copied
/// into the record's array at `+4` (`00e23a60`), the record's byte is set to 1 and `00e27450`
/// (not yet translated) is run on the mesh with the edge. The next edge is the neighbour
/// across the current one, and the walk stops at an unset neighbour.
pub fn fn_00e27ae0(e: &mut Engine, this: Ptr, triangle: u32, index: u32, out: Ptr) {
    let mut records = out.addr();
    e.mem.set_u8(records, 0);
    e.mem.set_u8(records + 0x20, 0);
    // The edge lives in the exe's own parameter words, whose address the callees are given.
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), triangle);
        e.mem.set_u32(pair.addr() + 4, index);
        for _ in 0..2 {
            let value = fn_00e23900(e, this, pair);
            if value != 0 {
                fn_00e23a60(e, Ptr::new(records + 4), Ptr::new(value));
                e.mem.set_u8(records, 1);
                e.call(0x00e2_7450, &args![this, pair]);
            }
            let current = e.mem.u32(pair.addr());
            let slot = e.mem.u32(pair.addr() + 4);
            match follow_link(e, current, slot) {
                Some(next) => {
                    e.mem.set_u32(pair.addr(), next.0);
                    e.mem.set_u32(pair.addr() + 4, next.1);
                }
                None => return,
            }
            records += 0x20;
        }
    });
}

// Translated from 00e25fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the records `00e27ae0` made (`this` is ECX; stack: the edge `triangle` and `index` by
/// value, and `records`, two records of 0x20 bytes): for each of up to two edges, beginning with
/// the one given, when the record's first byte is non-zero `00e25cd0` (not yet translated) is
/// run with `(this, edge, record + 4)`; the next edge is the neighbour across the current one,
/// and the walk stops at an unset neighbour.
pub fn fn_00e25fb0(e: &mut Engine, this: Ptr, triangle: u32, index: u32, records: Ptr) {
    let mut record = records.addr();
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), triangle);
        e.mem.set_u32(pair.addr() + 4, index);
        for _ in 0..2 {
            if e.mem.u8(record) != 0 {
                e.call(0x00e2_5cd0, &args![this, pair, record + 4]);
            }
            let current = e.mem.u32(pair.addr());
            let slot = e.mem.u32(pair.addr() + 4);
            match follow_link(e, current, slot) {
                Some(next) => {
                    e.mem.set_u32(pair.addr(), next.0);
                    e.mem.set_u32(pair.addr() + 4, next.1);
                }
                None => return,
            }
            record += 0x20;
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e2_4f70, deleting_destructor_00e24f70(Ptr, u8) -> Ptr),
        entry!(0x00d1_ba90, hkgp_mesh_prism_policy_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00d1_bab0, hkgp_mesh_internals_triangle_overlap_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e1_1480, hkgp_mesh_iconvex_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_2450, hk_gsk_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_2500, hkgp_mesh_iconvex_overlap_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_1e00, hkgp_mesh_line_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_1e30, hkgp_mesh_triangle_shape_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_2d50, hkgp_mesh_welding_policy_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_2d80, hkgp_mesh_collide_shape_triangle_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_2db0, hkgp_mesh_nearest_feature_policy_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e1_14a0, fn_00e114a0(Ptr) -> u32),
        entry!(0x00e1_14c0, fn_00e114c0(Ptr)),
        entry!(0x00e1_14f0, fn_00e114f0(Ptr, u32, u32, u32)),
        entry!(0x00e1_1500, fn_00e11500(Ptr, u32, u32)),
        entry!(0x00e1_1510, fn_00e11510(Ptr, u32, u32)),
        entry!(0x00e1_1520, fn_00e11520(Ptr, u32, u32)),
        entry!(0x00e1_1530, fn_00e11530(Ptr, Ptr)),
        entry!(0x00e1_1570, fn_00e11570(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00e1_1630, fn_00e11630(Ptr, Ptr)),
        entry!(0x00e1_1670, fn_00e11670(Ptr, Ptr) -> Ptr),
        entry!(0x00e1_1710, mesh_set_plane(Ptr, Ptr, Ptr, Ptr, Ptr, u8) -> u8),
        entry!(0x00e1_18e0, fn_00e118e0(Ptr, Ptr, Ptr, u8) -> u8),
        entry!(0x00e1_1910, fn_00e11910(Ptr, Ptr) -> bool),
        entry!(0x00e1_1960, fn_00e11960(Ptr) -> i32),
        entry!(0x00e1_1990, fn_00e11990(Ptr, Ptr, u32) -> u32),
        entry!(0x00e1_1a60, mesh_update_plane_equations(Ptr)),
        entry!(0x00e1_1a90, mesh_apply_transform(Ptr, Ptr)),
        entry!(0x00e1_1b00, mesh_apply_scale(Ptr, Ptr)),
        entry!(0x00e1_1b40, fn_00e11b40(Ptr, Ptr)),
        entry!(0x00e1_1ba0, fn_00e11ba0(Ptr, Ptr)),
        entry!(0x00e1_1ca0, fn_00e11ca0(Ptr, Ptr)),
        entry!(0x00e1_1d80, fn_00e11d80(Ptr, u32, f32)),
        entry!(0x00e1_1da0, fn_00e11da0(Ptr, u32)),
        entry!(
            0x00e2_2970,
            hk_array_raw_triangle_append(Ptr, u32, u32, i32)
        ),
        entry!(0x00e2_29f0, fn_00e229f0(Ptr, u32, u32, i32)),
        entry!(0x00e2_2640, fn_00e22640(Ptr, u32, u32, i32)),
        entry!(0x00e2_2c80, fn_00e22c80(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00e2_2ca0, fn_00e22ca0(Ptr) -> Ptr),
        entry!(0x00e2_2ec0, fn_00e22ec0(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_2f20, fn_00e22f20(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_3080, fn_00e23080(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_34e0, fn_00e234e0(Ptr)),
        entry!(0x00e2_39c0, fn_00e239c0(Ptr)),
        entry!(0x00e2_3a10, fn_00e23a10(Ptr)),
        entry!(0x00e2_3530, fn_00e23530(Ptr) -> u32),
        entry!(0x00e2_35d0, fn_00e235d0(Ptr) -> u32),
        entry!(0x00e2_3670, fn_00e23670(Ptr) -> u32),
        entry!(0x00e2_3710, fn_00e23710(Ptr) -> u32),
        entry!(
            0x00e2_37b0,
            hkgp_mesh_internals_simple_collector_add_leaf(Ptr, Ptr)
        ),
        entry!(0x00e2_3800, hkgp_mesh_internals_simple_collector_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_3860, fn_00e23860(Ptr)),
        entry!(0x00e2_3c70, fn_00e23c70(Ptr)),
        entry!(0x00e2_3cf0, fn_00e23cf0(Ptr)),
        entry!(0x00e2_3d70, fn_00e23d70(Ptr)),
        entry!(0x00e2_38e0, hkgp_abstract_mesh_reset_00e238e0(Ptr)),
        entry!(0x00e2_4a80, hkgp_abstract_mesh_reset_00e24a80(Ptr)),
        entry!(0x00e2_3b50, hkgp_abstract_mesh_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_4de0, hkgp_abstract_mesh_triangulator_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_3af0, fn_00e23af0(Ptr, u8) -> Ptr),
        entry!(0x00e2_4760, fn_00e24760(Ptr)),
        entry!(0x00e2_2f90, pool_allocator_release_00e22f90(Ptr, u32)),
        entry!(0x00e2_4e40, fn_00e24e40(Ptr, u32)),
        entry!(0x00e2_4140, fn_00e24140(Ptr, Ptr) -> u32),
        entry!(0x00e2_41a0, fn_00e241a0(Ptr, Ptr) -> u32),
        entry!(0x00e2_47c0, fn_00e247c0(Ptr, Ptr)),
        entry!(0x00e2_4850, fn_00e24850(Ptr, Ptr) -> u32),
        entry!(0x00e2_5f00, fn_00e25f00(Ptr, u8) -> Ptr),
        entry!(0x00e2_7400, fn_00e27400(Ptr)),
        entry!(0x00e2_7df0, hkgp_triangulator_scalar_deleting_destructor(Ptr, u8) -> Ptr),
        entry!(0x00e2_3900, fn_00e23900(Ptr, Ptr) -> u32),
        entry!(0x00e2_1ce0, hkgp_mesh_iconvex_shape_get_max_index(Ptr) -> u32),
        entry!(
            0x00e2_2520,
            hk_gsk_shape_convert_vertex_ids_to_vertices(Ptr, u32, u32, u32)
        ),
        entry!(0x00e2_1dd0, hkgp_mesh_line_shape_get_bounding_box(Ptr, Ptr)),
        entry!(
            0x00e2_1d40,
            hkgp_mesh_line_shape_get_supporting_vertex(Ptr, Ptr, Ptr)
        ),
        entry!(0x00e2_1e60, hkgp_mesh_welding_policy_add_leaf(Ptr, Ptr)),
        entry!(
            0x00e2_1ed0,
            hkgp_mesh_triangle_shape_get_supporting_vertex(Ptr, Ptr, Ptr)
        ),
        entry!(0x00e2_1fc0, hkgp_mesh_triangle_shape_get_bounding_box(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_21e0, hkgp_mesh_nearest_feature_policy_add_leaf(Ptr, Ptr) -> f32),
        entry!(0x00e2_2270, hkgp_mesh_internals_triangle_overlap_add_leaf(Ptr, Ptr) -> f32),
        entry!(0x00e2_2480, shape_bridge_convert_vertex_ids_to_vertices(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e2_24d0, convex_overlap_impl_check_overlap(Ptr, u32, u32, f32) -> bool),
        (0x00e2_2470, shape_bridge_get_supporting_vertex as AbiFn),
        entry!(
            0x00e2_26b0,
            hkgp_mesh_prism_shape_get_supporting_vertex(Ptr, Ptr, Ptr)
        ),
        entry!(0x00e2_27f0, hkgp_mesh_prism_shape_get_bounding_box(Ptr, Ptr) -> Ptr),
        entry!(0x00e1_2230, fn_00e12230(Ptr, u32, u32)),
        entry!(0x00e1_21c0, fn_00e121c0(Ptr, Ptr, Ptr, u32) -> f32),
        entry!(0x00e1_20e0, fn_00e120e0(Ptr, Ptr, f32, u8) -> u8),
        entry!(0x00e1_2260, mesh_check_triangle_overlap(Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00e1_22d0, fn_00e122d0(Ptr, Ptr, u32, u32)),
        entry!(0x00e1_2640, mesh_fetch_positions(Ptr, Ptr) -> i32),
        entry!(0x00e1_2890, fn_00e12890(Ptr, Ptr, u8)),
        entry!(0x00e1_2920, mesh_is_closed_manifold(Ptr) -> bool),
        entry!(0x00e1_3660, fn_00e13660(Ptr, u32, Ptr, u32) -> f32),
        entry!(0x00e1_36c0, mesh_get_volume(Ptr) -> f32),
        entry!(0x00e1_3750, mesh_rebuild_convex_hull(Ptr) -> bool),
        entry!(0x00e1_3870, fn_00e13870(Ptr, u32) -> f32),
        entry!(0x00e1_3f10, fn_00e13f10(Ptr, u32)),
        entry!(0x00e1_4a90, mesh_remove_orphan_vertices(Ptr) -> u32),
        entry!(0x00e1_4fe0, fn_00e14fe0(Ptr, u32)),
        entry!(
            0x00e1_5020,
            mesh_remove_part_from_classification(Ptr, u32, Ptr, Ptr, u8)
        ),
        entry!(0x00e1_57f0, mesh_reset(Ptr)),
        entry!(0x00e1_8a50, hkgp_mesh_destructor(Ptr)),
        entry!(0x00e1_b750, hkgp_mesh_constructor(Ptr) -> Ptr),
        entry!(0x00e2_5ba0, fn_00e25ba0(Ptr)),
        entry!(0x00e2_2320, hkgp_mesh_internals_ray_caster_add_leaf(Ptr, Ptr) -> f32),
        entry!(0x00e2_2050, hkgp_mesh_collide_shape_triangle_add_leaf(Ptr, Ptr) -> f32),
        entry!(0x00e2_2580, hkgp_mesh_prism_policy_add_hit(Ptr, Ptr, Ptr) -> bool),
        entry!(0x00e1_d900, mesh_append_from_convex_hull(Ptr, Ptr)),
        entry!(0x00e1_4160, fn_00e14160(Ptr, Ptr, f32) -> f32),
        entry!(0x00e1_3fa0, mesh_normalize(Ptr, Ptr, Ptr, f32) -> f32),
        entry!(0x00e1_53f0, get_loop_surface_area(Ptr) -> f32),
        entry!(0x00e1_50e0, fn_00e150e0(Ptr, u32)),
        entry!(
            0x00e1_5220,
            mesh_enumerate_triangle_overlaps(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00e1_91a0, fn_00e191a0(Ptr, Ptr, u32, u32, Ptr, u32) -> Ptr),
        entry!(0x00e1_9200, fn_00e19200(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00e1_9cf0, fn_00e19cf0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e2_3a60, fn_00e23a60(Ptr, Ptr) -> Ptr),
        entry!(0x00e2_7ae0, fn_00e27ae0(Ptr, u32, u32, Ptr)),
        entry!(0x00e2_5fb0, fn_00e25fb0(Ptr, u32, u32, Ptr)),
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
    fn deleting_destructor_00e24f70_frees_only_when_bit_zero_is_set() {
        check(0x00e2_4f70, 0x00e1_8a50);
    }

    // -----------------------------------------------------------------------------------------
    // Fixtures shared by the tests of the functions added below.
    // -----------------------------------------------------------------------------------------

    /// What the heap double logs: (slot, first word, second word).
    type HeapLog = Rc<RefCell<Vec<(u32, u32, u32)>>>;

    /// The argument words of every logged call at `address`.
    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(logged, _)| *logged == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn put_vec(e: &mut Engine, address: u32, value: [f32; 4]) {
        for (lane, word) in value.iter().enumerate() {
            e.mem.set_f32(address + 4 * lane as u32, *word);
        }
    }

    fn vec_at(e: &Engine, address: u32) -> [f32; 4] {
        std::array::from_fn(|lane| e.mem.f32(address + 4 * lane as u32))
    }

    /// `00c908c0` as a test double: gives the array `wanted` elements of storage, copying the
    /// old elements and freeing the old storage when the array owns it.
    fn array_reserve_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (array, wanted, element) = (a[1], a[2], a[3]);
        let size = e.mem.u32(array + 4);
        let old = e.mem.u32(array);
        let capacity = e.mem.u32(array + 8);
        let storage = e.mem.alloc(wanted * element);
        for byte in 0..size * element {
            let value = e.mem.u8(old + byte);
            e.mem.set_u8(storage + byte, value);
        }
        if (capacity as i32) >= 0 {
            e.mem.free(old);
        }
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 8, wanted);
        Ret::default()
    }

    /// `00c90940` as a test double: reserves twice the capacity (at least 4 elements).
    fn array_grow_double(e: &mut Engine, a: &[u32]) -> Ret {
        let (array, element) = (a[1], a[2]);
        let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
        let wanted = if capacity < 2 { 4 } else { capacity * 2 };
        array_reserve_double(e, &[a[0], array, wanted, element])
    }

    /// Installs the memory router with an allocator whose slot 4 allocates on the engine's
    /// heap, slot 8 frees a block `(pointer, size)` and slot `0x10` frees array storage
    /// `(pointer, bytes)`, and the two array-growth doubles. Every allocator call is logged.
    fn install_heap(e: &mut Engine) -> HeapLog {
        let log: HeapLog = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        e.mem.set_u32(vtable + 0x0c, 0x0900_000c);
        e.mem.set_u32(vtable + 0x10, 0x0900_0010);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let allocations = Rc::clone(&log);
        e.register_double(0x0900_0004, move |e, a| {
            allocations.borrow_mut().push((4, a[1], 0));
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let frees = Rc::clone(&log);
        e.register_double(0x0900_0008, move |e, a| {
            frees.borrow_mut().push((8, a[1], a[2]));
            e.mem.free(a[1]);
            Ret::default()
        });
        let array_allocations = Rc::clone(&log);
        e.register_double(0x0900_000c, move |e, a| {
            let bytes = e.mem.u32(a[1]);
            array_allocations.borrow_mut().push((0x0c, bytes, 0));
            Ret {
                eax: e.mem.alloc(bytes),
                ..Ret::default()
            }
        });
        let storage_frees = Rc::clone(&log);
        e.register_double(0x0900_0010, move |e, a| {
            storage_frees.borrow_mut().push((0x10, a[1], a[2]));
            e.mem.free(a[1]);
            Ret::default()
        });
        e.register(ARRAY_RESERVE, array_reserve_double);
        e.register(ARRAY_GROW, array_grow_double);
        log
    }

    /// Links `element` at the front of the list of the pool at `pool` (head at `+0x0c`, count at
    /// `+0x10`).
    fn link_front_for_test(e: &mut Engine, pool: u32, element: u32) {
        let head = e.mem.u32(pool + 0x0c);
        e.mem.set_u32(element, head);
        if head != 0 {
            e.mem.set_u32(head + 4, element);
        }
        let count = e.mem.u32(pool + 0x10);
        e.mem.set_u32(pool + 0x10, count + 1);
        e.mem.set_u32(pool + 0x0c, element);
    }

    /// A zeroed mesh object, large enough for every field the tests use.
    fn new_mesh(e: &mut Engine) -> u32 {
        e.mem.alloc(0x260)
    }

    /// A vertex with the position `position`, at the front of the mesh's vertex list.
    fn add_vertex(e: &mut Engine, mesh: u32, position: [f32; 4]) -> u32 {
        let vertex = e.mem.alloc(0x50);
        put_vec(e, vertex + 0x10, position);
        link_front_for_test(e, mesh + 0x08, vertex);
        vertex
    }

    /// A triangle on three vertices, at the front of the mesh's triangle list.
    fn add_triangle(e: &mut Engine, mesh: u32, a: u32, b: u32, c: u32) -> u32 {
        let triangle = e.mem.alloc(0x50);
        e.mem.set_u32(triangle + 8, a);
        e.mem.set_u32(triangle + 0xc, b);
        e.mem.set_u32(triangle + 0x10, c);
        link_front_for_test(e, mesh + 0x20, triangle);
        triangle
    }

    /// A function of the unit that takes `(this, flags)`, leaves `vtable` and frees through
    /// `operator delete` when bit 0 of `flags` is set.
    fn check_operator_delete(address: u32, vtable: u32) {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), vtable);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        e.mem.set_u32(this, 0);
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), vtable);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![this]]);
    }

    /// As [`check_operator_delete`] for the functions that free through the memory router's
    /// allocator with `size` bytes.
    fn check_router_free(address: u32, vtable: u32, size: u32) {
        let mut e = engine();
        let log = install_heap(&mut e);
        let this = e.mem.alloc(0x40);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this), vtable);
        assert!(log.borrow().is_empty());
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![(8, this, size)]);
    }

    #[test]
    fn hkgp_mesh_prism_policy_00d1ba90_frees_only_when_bit_zero_is_set() {
        check_operator_delete(0x00d1_ba90, 0x010c_d658);
    }

    #[test]
    fn hkgp_mesh_internals_triangle_overlap_00d1bab0_frees_only_when_bit_zero_is_set() {
        check_operator_delete(0x00d1_bab0, 0x010c_d664);
    }

    #[test]
    fn hkgp_mesh_iconvex_shape_00e11480_frees_only_when_bit_zero_is_set() {
        check_operator_delete(0x00e1_1480, 0x010e_acf4);
    }

    #[test]
    fn hk_gsk_shape_00e22450_frees_only_when_bit_zero_is_set() {
        check_operator_delete(0x00e2_2450, 0x010e_adfc);
    }

    #[test]
    fn hkgp_mesh_iconvex_overlap_00e22500_frees_only_when_bit_zero_is_set() {
        check_operator_delete(0x00e2_2500, 0x010e_ad08);
    }

    #[test]
    fn hkgp_mesh_line_shape_00e21e00_frees_through_the_router_when_bit_zero_is_set() {
        check_router_free(0x00e2_1e00, 0x010e_acf4, 0x30);
    }

    #[test]
    fn hkgp_mesh_triangle_shape_00e21e30_frees_through_the_router_when_bit_zero_is_set() {
        check_router_free(0x00e2_1e30, 0x010e_acf4, 8);
    }

    #[test]
    fn hkgp_mesh_welding_policy_00e22d50_frees_through_the_router_when_bit_zero_is_set() {
        check_router_free(0x00e2_2d50, 0x010c_d664, 0x30);
    }

    #[test]
    fn hkgp_mesh_collide_shape_triangle_00e22d80_frees_through_the_router_when_bit_zero_is_set() {
        check_router_free(0x00e2_2d80, 0x010c_d664, 0x14);
    }

    #[test]
    fn hkgp_mesh_nearest_feature_policy_00e22db0_frees_through_the_router_when_bit_zero_is_set() {
        check_router_free(0x00e2_2db0, 0x010c_d664, 0x60);
    }

    #[test]
    fn fn_00e114a0_returns_the_word_at_0xe4() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_u32(mesh + 0xe4, 0x1234_5678);
        assert_eq!(e.call(0x00e1_14a0, &args![mesh]).u32(), 0x1234_5678);
    }

    #[test]
    fn fn_00e114c0_deletes_the_convex_hull_and_clears_the_field() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0020);
        let hull = e.mem.alloc(0x10);
        e.mem.set_u32(hull, vtable);
        e.register(0x0900_0020, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        // Without a hull nothing is called.
        e.call(0x00e1_14c0, &args![mesh]);
        assert!(calls(&e, 0x0900_0020).is_empty());
        e.mem.set_u32(mesh + 0xe4, hull);
        e.call(0x00e1_14c0, &args![mesh]);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
    }

    /// The one-line callbacks set the byte at `+0x100` and nothing else.
    fn check_problem_flag(address: u32, words: usize) {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let mut arguments = vec![mesh, 7, 7, 7];
        arguments.truncate(1 + words);
        e.call(address, &arguments);
        assert_eq!(e.mem.u8(mesh + 0x100), 1);
        assert_eq!(e.mem.u8(mesh + 0x101), 0);
        assert_eq!(e.mem.u8(mesh + 0xff), 0);
    }

    #[test]
    fn fn_00e114f0_sets_the_problem_flag() {
        check_problem_flag(0x00e1_14f0, 3);
    }

    #[test]
    fn fn_00e11500_sets_the_problem_flag() {
        check_problem_flag(0x00e1_1500, 2);
    }

    #[test]
    fn fn_00e11510_sets_the_problem_flag() {
        check_problem_flag(0x00e1_1510, 2);
    }

    #[test]
    fn fn_00e11520_sets_the_problem_flag() {
        check_problem_flag(0x00e1_1520, 2);
    }

    #[test]
    fn fn_00e11530_gives_the_bounding_box_of_the_vertices() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        add_vertex(&mut e, mesh, [1.0, 5.0, -2.0, 1.0]);
        add_vertex(&mut e, mesh, [-3.0, 2.0, 4.0, 1.0]);
        add_vertex(&mut e, mesh, [0.0, 9.0, 0.5, 1.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00e1_1530, &args![mesh, out]);
        assert_eq!(vec_at(&e, out), [-3.0, 2.0, -2.0, 1.0]);
        assert_eq!(vec_at(&e, out + 0x10), [1.0, 9.0, 4.0, 1.0]);
    }

    #[test]
    #[should_panic]
    fn fn_00e11530_reads_the_head_position_before_testing_it() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let out = e.mem.alloc(0x20);
        e.call(0x00e1_1530, &args![mesh, out]);
    }

    #[test]
    fn fn_00e11570_finds_the_vertex_furthest_along_the_direction() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        add_vertex(&mut e, mesh, [1.0, 0.0, 0.0, 1.0]);
        add_vertex(&mut e, mesh, [3.0, 0.0, 0.0, 1.0]);
        add_vertex(&mut e, mesh, [2.0, 0.0, 0.0, 1.0]);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        // The list is walked newest first: 2.0 (index 0), 3.0 (index 1), 1.0 (index 2).
        let got = e.call(0x00e1_1570, &args![mesh, out, direction]).u32();
        assert_eq!(got, out);
        assert_eq!(vec_at(&e, out)[..3], [3.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0001);
    }

    #[test]
    fn fn_00e11570_keeps_the_head_copy_when_nothing_is_better() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        add_vertex(&mut e, mesh, [1.0, 0.0, 0.0, 1.0]);
        add_vertex(&mut e, mesh, [2.0, 0.0, 0.0, 1.0]);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [-1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        // Head is 2.0 (dot -2); the next vertex 1.0 (dot -1) is better.
        e.call(0x00e1_1570, &args![mesh, out, direction]);
        assert_eq!(vec_at(&e, out)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0001);
        // Now the head is the best: its own w lane stays, no index is stored.
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00e1_1570, &args![mesh, out, direction]);
        assert_eq!(vec_at(&e, out), [2.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn fn_00e11630_is_the_centroid_of_the_triangle() {
        let mut e = engine();
        e.mem.set_f32(0x0106_c2e0, 0.5);
        let mesh = new_mesh(&mut e);
        let a = add_vertex(&mut e, mesh, [1.0, 2.0, 3.0, 0.0]);
        let b = add_vertex(&mut e, mesh, [3.0, 4.0, 5.0, 0.0]);
        let c = add_vertex(&mut e, mesh, [5.0, 6.0, 7.0, 0.0]);
        let triangle = add_triangle(&mut e, mesh, a, b, c);
        let out = e.mem.alloc(0x10);
        e.call(0x00e1_1630, &args![out, triangle]);
        assert_eq!(vec_at(&e, out), [4.5, 6.0, 7.5, 0.0]);
    }

    #[test]
    fn fn_00e11670_averages_the_vertex_positions() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        add_vertex(&mut e, mesh, [1.0, 2.0, 3.0, 0.0]);
        add_vertex(&mut e, mesh, [3.0, 4.0, 5.0, 0.0]);
        let out = e.mem.alloc(0x10);
        let got = e.call(0x00e1_1670, &args![mesh, out]).u32();
        assert_eq!(got, out);
        assert_eq!(vec_at(&e, out), [2.0, 3.0, 4.0, 0.0]);
    }

    #[test]
    fn fn_00e11670_without_vertices_uses_the_hull_or_the_zero_vector() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let centre = e.mem.alloc(0x10);
        put_vec(&mut e, centre, [7.0, 8.0, 9.0, 1.0]);
        e.register_double(0x00d7_52d0, move |_, _| Ret {
            eax: centre,
            ..Ret::default()
        });
        let mesh = new_mesh(&mut e);
        put_vec(&mut e, 0x0126_7e30, [1.0, 2.0, 3.0, 4.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00e1_1670, &args![mesh, out]);
        assert_eq!(vec_at(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert!(calls(&e, 0x00d7_52d0).is_empty());
        let hull = e.mem.alloc(0x10);
        e.mem.set_u32(mesh + 0xe4, hull);
        e.call(0x00e1_1670, &args![mesh, out]);
        assert_eq!(vec_at(&e, out), [7.0, 8.0, 9.0, 1.0]);
        assert_eq!(calls(&e, 0x00d7_52d0), vec![vec![hull]]);
    }

    /// Three vertices of the right triangle in the xy plane, as pointers to their positions.
    fn plane_inputs(e: &mut Engine) -> (u32, u32, u32, u32) {
        let positions = e.mem.alloc(0x40);
        put_vec(e, positions, [0.0, 0.0, 0.0, 0.0]);
        put_vec(e, positions + 0x10, [1.0, 0.0, 0.0, 0.0]);
        put_vec(e, positions + 0x20, [0.0, 1.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        (positions, positions + 0x10, positions + 0x20, out)
    }

    #[test]
    fn mesh_set_plane_00e11710_gives_the_unit_normal_and_offset() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 1.0e-6);
        e.mem.set_f32(mesh + 0x108, 1.0e-3);
        let (a, b, c, out) = plane_inputs(&mut e);
        // Offset: the plane through (0, 0, 1) on a triangle lifted to z = 2 has offset -2.
        e.mem.set_f32(a + 8, 2.0);
        e.mem.set_f32(b + 8, 2.0);
        e.mem.set_f32(c + 8, 2.0);
        let got = e.call(0x00e1_1710, &args![mesh, a, b, c, out, 1u32]).u8();
        assert_eq!(got, 1);
        assert_eq!(vec_at(&e, out), [0.0, 0.0, 1.0, -2.0]);
        assert_eq!(e.mem.u8(mesh + 0x100), 0);
    }

    #[test]
    fn mesh_set_plane_00e11710_scales_the_normal_to_unit_length() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 1.0e-6);
        e.mem.set_f32(mesh + 0x108, 1.0e-3);
        let (a, b, c, out) = plane_inputs(&mut e);
        put_vec(&mut e, b, [4.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, c, [0.0, 2.0, 0.0, 0.0]);
        assert_eq!(
            e.call(0x00e1_1710, &args![mesh, a, b, c, out, 0u32]).u8(),
            1
        );
        assert_eq!(vec_at(&e, out), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn mesh_set_plane_00e11710_reports_a_degenerate_triangle_when_checking() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 1.0e-6);
        e.mem.set_f32(mesh + 0x108, 1.0e-3);
        let (a, b, c, out) = plane_inputs(&mut e);
        put_vec(&mut e, b, [0.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00e1_1710, &args![mesh, a, b, c, out, 1u32]).u8();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u8(mesh + 0x100), 1);
        // Without the check byte the same triangle is not reported (the result stays 1).
        e.mem.set_u8(mesh + 0x100, 0);
        let got = e.call(0x00e1_1710, &args![mesh, a, b, c, out, 0u32]).u8();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u8(mesh + 0x100), 0);
    }

    #[test]
    fn mesh_set_plane_00e11710_unordered_tolerances_never_report() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, f32::NAN);
        e.mem.set_f32(mesh + 0x108, f32::NAN);
        let (a, b, c, out) = plane_inputs(&mut e);
        put_vec(&mut e, b, [0.0, 0.0, 0.0, 0.0]);
        let got = e.call(0x00e1_1710, &args![mesh, a, b, c, out, 1u32]).u8();
        assert_eq!(got, 1);
        assert_eq!(e.mem.u8(mesh + 0x100), 0);
    }

    #[test]
    fn mesh_set_plane_00e11710_reports_a_short_normal() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 1.0e-12);
        e.mem.set_f32(mesh + 0x108, 1.0);
        let (a, b, c, out) = plane_inputs(&mut e);
        // Edges of length 0.5 pass the first tolerance, but the normal (0.25) is below 1.0.
        put_vec(&mut e, b, [0.5, 0.0, 0.0, 0.0]);
        put_vec(&mut e, c, [0.0, 0.5, 0.0, 0.0]);
        let got = e.call(0x00e1_1710, &args![mesh, a, b, c, out, 1u32]).u8();
        assert_eq!(got, 0);
        assert_eq!(e.mem.u8(mesh + 0x100), 1);
    }

    /// A triangle on the right-angle vertices of the xy plane; returns (mesh, triangle).
    fn plane_mesh(e: &mut Engine) -> (u32, u32) {
        let mesh = new_mesh(e);
        e.mem.set_f32(mesh + 0x104, 1.0e-6);
        e.mem.set_f32(mesh + 0x108, 1.0e-3);
        let a = add_vertex(e, mesh, [0.0, 0.0, 0.0, 0.0]);
        let b = add_vertex(e, mesh, [1.0, 0.0, 0.0, 0.0]);
        let c = add_vertex(e, mesh, [0.0, 1.0, 0.0, 0.0]);
        let triangle = add_triangle(e, mesh, a, b, c);
        (mesh, triangle)
    }

    #[test]
    fn fn_00e118e0_is_set_plane_on_the_triangle_vertices() {
        let mut e = engine();
        let (mesh, triangle) = plane_mesh(&mut e);
        let out = e.mem.alloc(0x10);
        let got = e.call(0x00e1_18e0, &args![mesh, triangle, out, 1u32]).u8();
        assert_eq!(got, 1);
        assert_eq!(vec_at(&e, out), [0.0, 0.0, 1.0, 0.0]);
    }

    #[test]
    fn fn_00e11910_stores_the_plane_only_when_it_succeeds() {
        let mut e = engine();
        let (mesh, triangle) = plane_mesh(&mut e);
        put_vec(&mut e, triangle + 0x20, [9.0, 9.0, 9.0, 9.0]);
        assert!(e.call(0x00e1_1910, &args![mesh, triangle]).bool());
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 1.0, 0.0]);
        // A degenerate triangle: the vertex of `b` moves onto `a`.
        let b = e.mem.u32(triangle + 0xc);
        put_vec(&mut e, b + 0x10, [0.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, triangle + 0x20, [9.0, 9.0, 9.0, 9.0]);
        assert!(!e.call(0x00e1_1910, &args![mesh, triangle]).bool());
        assert_eq!(vec_at(&e, triangle + 0x20), [9.0, 9.0, 9.0, 9.0]);
    }

    #[test]
    fn fn_00e11960_is_one_more_than_the_largest_triangle_word() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        assert_eq!(e.call(0x00e1_1960, &args![mesh]).i32(), 0);
        for value in [3i32, 7, -1] {
            let triangle = add_triangle(&mut e, mesh, 0, 0, 0);
            e.mem.set_i32(triangle + 0x30, value);
        }
        assert_eq!(e.call(0x00e1_1960, &args![mesh]).i32(), 8);
    }

    #[test]
    fn fn_00e11990_classifies_the_vertices_against_a_plane() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 0.25);
        let behind = add_vertex(&mut e, mesh, [-2.0, 0.0, 0.0, 1.0]);
        let near = add_vertex(&mut e, mesh, [1.25, 0.0, 0.0, 1.0]);
        let front = add_vertex(&mut e, mesh, [3.0, 0.0, 0.0, 1.0]);
        let plane = e.mem.alloc(0x10);
        put_vec(&mut e, plane, [1.0, 0.0, 0.0, -1.0]);
        let got = e.call(0x00e1_1990, &args![mesh, plane, 1u32]).u32();
        assert_eq!(got, 3);
        assert_eq!(e.mem.f32(behind + 0x34), -3.0);
        // The square of 0.25 is below the tolerance: the distance is taken as zero.
        assert_eq!(e.mem.f32(near + 0x34), 0.0);
        assert_eq!(e.mem.f32(front + 0x34), 2.0);
    }

    #[test]
    fn fn_00e11990_reports_one_side_at_a_time() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 0.25);
        add_vertex(&mut e, mesh, [3.0, 0.0, 0.0, 1.0]);
        let plane = e.mem.alloc(0x10);
        put_vec(&mut e, plane, [1.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.call(0x00e1_1990, &args![mesh, plane, 0u32]).u32(), 1);
        put_vec(&mut e, plane, [-1.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.call(0x00e1_1990, &args![mesh, plane, 0u32]).u32(), 2);
        // A NaN distance counts as neither side; a mesh without vertices gives 0.
        put_vec(&mut e, plane, [f32::NAN, 0.0, 0.0, 0.0]);
        assert_eq!(e.call(0x00e1_1990, &args![mesh, plane, 0u32]).u32(), 0);
        let empty = new_mesh(&mut e);
        assert_eq!(e.call(0x00e1_1990, &args![empty, plane, 0u32]).u32(), 0);
    }

    #[test]
    fn mesh_update_plane_equations_00e11a60_recomputes_every_plane() {
        let mut e = engine();
        let (mesh, triangle) = plane_mesh(&mut e);
        put_vec(&mut e, triangle + 0x20, [5.0, 5.0, 5.0, 5.0]);
        e.call(0x00e1_1a60, &args![mesh]);
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 1.0, 0.0]);
        // A degenerate triangle keeps the zero vector it was reset to.
        let b = e.mem.u32(triangle + 0xc);
        put_vec(&mut e, b + 0x10, [0.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, triangle + 0x20, [5.0, 5.0, 5.0, 5.0]);
        e.call(0x00e1_1a60, &args![mesh]);
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 0.0, 0.0]);
    }

    /// A convex hull whose slot 0 is a double that is logged at `0x0900_0020`.
    fn install_hull(e: &mut Engine, mesh: u32) -> u32 {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0020);
        let hull = e.mem.alloc(0x10);
        e.mem.set_u32(hull, vtable);
        e.register(0x0900_0020, |_, _| Ret::default());
        e.mem.set_u32(mesh + 0xe4, hull);
        hull
    }

    #[test]
    fn mesh_apply_transform_00e11a90_transforms_positions_and_updates_the_mesh() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.register(0x00c7_f640, |e, a| {
            let position = vec_at(e, a[2]);
            put_vec(
                e,
                a[0],
                [position[0] + 1.0, position[1], position[2], position[3]],
            );
            Ret::default()
        });
        let (mesh, triangle) = plane_mesh(&mut e);
        let hull = install_hull(&mut e, mesh);
        let transform = e.mem.alloc(0x40);
        let head = e.mem.u32(mesh + 0x14);
        e.call(0x00e1_1a90, &args![mesh, transform]);
        // The head vertex (0, 1, 0) moved one along x; the call was made for each vertex.
        assert_eq!(vec_at(&e, head + 0x10), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(calls(&e, 0x00c7_f640).len(), 3);
        assert_eq!(calls(&e, 0x00c7_f640)[0][..2], [head + 0x10, transform]);
        // The planes were recomputed (the triangle is (1,0,0),(2,0,0),(1,1,0)) and the hull deleted.
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
    }

    #[test]
    fn mesh_apply_scale_00e11b00_scales_positions_and_updates_the_mesh() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let (mesh, triangle) = plane_mesh(&mut e);
        let hull = install_hull(&mut e, mesh);
        let scale = e.mem.alloc(0x10);
        put_vec(&mut e, scale, [2.0, 3.0, 4.0, 1.0]);
        let head = e.mem.u32(mesh + 0x14);
        e.call(0x00e1_1b00, &args![mesh, scale]);
        assert_eq!(vec_at(&e, head + 0x10), [0.0, 3.0, 0.0, 0.0]);
        let second = e.mem.u32(head);
        assert_eq!(vec_at(&e, second + 0x10), [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
    }

    /// Records the sixteen words of the transform each `setTransformedPos` call is given.
    fn record_transforms(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00c7_f640, move |e, a| {
            log.borrow_mut()
                .push((0..16).map(|word| e.mem.u32(a[1] + 4 * word)).collect());
            Ret::default()
        });
        seen
    }

    /// Words 0 to 15 of the identity transform `01268370` get distinct values.
    fn put_identity_words(e: &mut Engine) {
        for word in 0..16 {
            e.mem.set_u32(0x0126_8370 + 4 * word, 0x3f80_0000 + word);
        }
    }

    #[test]
    fn fn_00e11b40_builds_a_transform_from_three_columns() {
        let mut e = engine();
        put_identity_words(&mut e);
        let seen = record_transforms(&mut e);
        let (mesh, _) = plane_mesh(&mut e);
        let rotation = e.mem.alloc(0x30);
        for word in 0..12 {
            e.mem.set_u32(rotation + 4 * word, 0x4000_0000 + word);
        }
        e.call(0x00e1_1b40, &args![mesh, rotation]);
        let expected: Vec<u32> = (0..12)
            .map(|word| 0x4000_0000 + word)
            .chain((12..16).map(|word| 0x3f80_0000 + word))
            .collect();
        assert_eq!(seen.borrow()[0], expected);
        assert_eq!(seen.borrow().len(), 3);
    }

    #[test]
    fn fn_00e11ba0_sets_the_rotation_of_the_identity_from_the_quaternion() {
        let mut e = engine();
        put_identity_words(&mut e);
        e.call_log = Some(vec![]);
        // The rotation setter marks the first word of the transform it fills.
        e.register(0x00cb_2d90, |e, a| {
            e.mem.set_u32(a[0], 0xdead_beef);
            Ret::default()
        });
        let seen = record_transforms(&mut e);
        let (mesh, _) = plane_mesh(&mut e);
        let quaternion = e.mem.alloc(0x10);
        e.call(0x00e1_1ba0, &args![mesh, quaternion]);
        let mut expected: Vec<u32> = (0..16).map(|word| 0x3f80_0000 + word).collect();
        expected[0] = 0xdead_beef;
        assert_eq!(seen.borrow()[0], expected);
        assert_eq!(calls(&e, 0x00cb_2d90).len(), 1);
        assert_eq!(calls(&e, 0x00cb_2d90)[0][1], quaternion);
    }

    #[test]
    fn fn_00e11ca0_builds_the_identity_rotation_with_a_translation() {
        let mut e = engine();
        put_identity_words(&mut e);
        let seen = record_transforms(&mut e);
        let (mesh, _) = plane_mesh(&mut e);
        let translation = e.mem.alloc(0x10);
        put_vec(&mut e, translation, [5.0, 6.0, 7.0, 8.0]);
        e.call(0x00e1_1ca0, &args![mesh, translation]);
        let mut expected: Vec<u32> = (0..12).map(|word| 0x3f80_0000 + word).collect();
        expected.extend([5.0f32, 6.0, 7.0, 8.0].map(f32::to_bits));
        assert_eq!(seen.borrow()[0], expected);
    }

    #[test]
    fn fn_00e11d80_stores_a_float_in_one_slot_of_every_vertex() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let first = add_vertex(&mut e, mesh, [0.0; 4]);
        let second = add_vertex(&mut e, mesh, [0.0; 4]);
        e.call(0x00e1_1d80, &args![mesh, 2u32, 2.5f32]);
        assert_eq!(e.mem.f32(first + 0x38), 2.5);
        assert_eq!(e.mem.f32(second + 0x38), 2.5);
        assert_eq!(e.mem.u32(first + 0x34), 0);
        // A signalling NaN is quieted by the load and store.
        e.call(0x00e1_1d80, &args![mesh, 0u32, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(first + 0x30), 0x7fc0_0001);
        // No vertices: nothing happens.
        let empty = new_mesh(&mut e);
        e.call(0x00e1_1d80, &args![empty, 0u32, 1.0f32]);
    }

    #[test]
    fn fn_00e11da0_stores_a_word_in_every_vertex() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let first = add_vertex(&mut e, mesh, [0.0; 4]);
        let second = add_vertex(&mut e, mesh, [0.0; 4]);
        e.call(0x00e1_1da0, &args![mesh, 0xabcd_u32]);
        assert_eq!(e.mem.u32(first + 0x44), 0xabcd);
        assert_eq!(e.mem.u32(second + 0x44), 0xabcd);
        assert_eq!(e.mem.u32(second + 0x40), 0);
    }

    /// An `hkArray` header (data, size, capacity word) over zeroed memory.
    fn new_array(e: &mut Engine) -> u32 {
        e.mem.alloc(0x10)
    }

    #[test]
    fn hk_array_raw_triangle_append_00e22970_grows_and_copies_the_elements() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let array = new_array(&mut e);
        let source = e.mem.alloc(0x30 * 5);
        for word in 0..(12 * 5) {
            e.mem.set_u32(source + 4 * word, 0x100 + word);
        }
        e.call(0x00e2_2970, &args![array, 0x1111u32, source, 2i32]);
        assert_eq!(e.mem.u32(array + 4), 2);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(data + 0x30), 0x100 + 12);
        assert_eq!(e.mem.u32(data + 0x30 + 4 * 11), 0x100 + 23);
        assert_eq!(calls(&e, ARRAY_RESERVE), vec![vec![0x1111, array, 2, 0x30]]);
        // Room for three more needs 5 elements, above twice the capacity of 2.
        e.call(0x00e2_2970, &args![array, 0x2222u32, source + 0x60, 3i32]);
        assert_eq!(e.mem.u32(array + 4), 5);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(data + 0x30 * 2), 0x100 + 24);
        assert_eq!(e.mem.u32(data + 0x30 * 4 + 44), 0x100 + 59);
        assert_eq!(calls(&e, ARRAY_RESERVE)[1], vec![0x2222, array, 5, 0x30]);
        // With enough capacity nothing is reserved; zero elements change nothing.
        e.call(0x00e2_2970, &args![array, 0x3333u32, source, 0i32]);
        assert_eq!(calls(&e, ARRAY_RESERVE).len(), 2);
        assert_eq!(e.mem.u32(array + 4), 5);
    }

    #[test]
    fn hk_array_raw_triangle_append_00e22970_doubles_the_capacity_when_that_is_enough() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let array = new_array(&mut e);
        let source = e.mem.alloc(0x30 * 8);
        e.call(0x00e2_2970, &args![array, 0u32, source, 4i32]);
        e.call(0x00e2_2970, &args![array, 0u32, source, 1i32]);
        // 4 + 1 elements fit in twice the capacity of 4.
        assert_eq!(calls(&e, ARRAY_RESERVE)[1], vec![0, array, 8, 0x30]);
    }

    #[test]
    fn fn_00e229f0_appends_words() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let array = new_array(&mut e);
        let source = e.mem.alloc(0x20);
        for word in 0..4 {
            e.mem.set_u32(source + 4 * word, 0x50 + word);
        }
        e.call(0x00e2_29f0, &args![array, 7u32, source, 3i32]);
        e.call(0x00e2_29f0, &args![array, 7u32, source + 12, 1i32]);
        assert_eq!(e.mem.u32(array + 4), 4);
        let data = e.mem.u32(array);
        let words: Vec<u32> = (0..4).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(words, vec![0x50, 0x51, 0x52, 0x53]);
        assert_eq!(
            calls(&e, ARRAY_RESERVE),
            vec![vec![7, array, 3, 4], vec![7, array, 6, 4]]
        );
    }

    #[test]
    fn fn_00e22640_appends_eight_byte_elements() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let array = new_array(&mut e);
        let source = e.mem.alloc(0x20);
        for word in 0..6 {
            e.mem.set_u32(source + 4 * word, 0x70 + word);
        }
        e.call(0x00e2_2640, &args![array, 9u32, source, 3i32]);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        let words: Vec<u32> = (0..6).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(words, vec![0x70, 0x71, 0x72, 0x73, 0x74, 0x75]);
        assert_eq!(calls(&e, ARRAY_RESERVE), vec![vec![9, array, 3, 8]]);
    }

    #[test]
    fn fn_00e22c80_fills_a_three_word_record() {
        let mut e = engine();
        let record = e.mem.alloc(0x10);
        let pair = e.mem.alloc(0x10);
        e.mem.set_u32(pair, 0xaaaa);
        e.mem.set_u32(pair + 4, 0xbbbb);
        let got = e.call(0x00e2_2c80, &args![record, 0x1234u32, pair]).u32();
        assert_eq!(got, record);
        assert_eq!(e.mem.u32(record), 0x1234);
        assert_eq!(e.mem.u32(record + 4), 0xaaaa);
        assert_eq!(e.mem.u32(record + 8), 0xbbbb);
        assert_eq!(e.mem.u32(record + 0xc), 0);
    }

    #[test]
    fn fn_00e22ca0_initialises_the_record_and_its_static_pair_once() {
        let mut e = engine();
        put_vec(&mut e, 0x0126_7e30, [1.0, 2.0, 3.0, 4.0]);
        e.mem.set_f32(0x0104_da48, 5.0);
        e.mem.set_u32(0x0126_ebe8, 0x77);
        e.mem.set_u32(0x0126_ebec, 0x88);
        let record = e.mem.alloc(0x60);
        e.mem.set_u8(record + 0x44, 9);
        let got = e.call(0x00e2_2ca0, &args![record]).u32();
        assert_eq!(got, record);
        for offset in [0, 0x10, 0x20] {
            assert_eq!(vec_at(&e, record + offset), [1.0, 2.0, 3.0, 4.0]);
        }
        assert_eq!(e.mem.f32(record + 0x40), 5.0);
        assert_eq!(e.mem.u8(record + 0x44), 0);
        assert_eq!(e.mem.u32(record + 0x48), 0);
        // The first run zeroed the static pair and set the guard bit.
        assert_eq!(e.mem.u32(record + 0x4c), 0);
        assert_eq!(e.mem.u32(record + 0x50), 0);
        assert_eq!(e.mem.u8(0x0126_ebf0) & 1, 1);
        // Later runs copy whatever the pair holds.
        e.mem.set_u32(0x0126_ebe8, 0x77);
        e.mem.set_u32(0x0126_ebec, 0x88);
        e.call(0x00e2_2ca0, &args![record]);
        assert_eq!(e.mem.u32(record + 0x4c), 0x77);
        assert_eq!(e.mem.u32(record + 0x50), 0x88);
    }

    #[test]
    fn fn_00e22ec0_copies_the_vertex_payload() {
        let mut e = engine();
        let source = e.mem.alloc(0x50);
        let this = e.mem.alloc(0x50);
        for word in 0..20 {
            e.mem.set_u32(source + 4 * word, 0x200 + word);
        }
        let got = e.call(0x00e2_2ec0, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this + 0x0c), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0x204);
        assert_eq!(e.mem.u32(this + 0x44), 0x211);
        assert_eq!(e.mem.u32(this + 0x48), 0);
    }

    #[test]
    fn fn_00e22f20_copies_the_triangle_payload() {
        let mut e = engine();
        let source = e.mem.alloc(0x50);
        let this = e.mem.alloc(0x50);
        for word in 0..20 {
            e.mem.set_u32(source + 4 * word, 0x300 + word);
        }
        let got = e.call(0x00e2_2f20, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this + 0x04), 0);
        assert_eq!(e.mem.u32(this + 0x08), 0x302);
        assert_eq!(e.mem.u32(this + 0x40), 0x310);
        assert_eq!(e.mem.u32(this + 0x44), 0);
    }

    #[test]
    fn fn_00e23080_copies_the_element_including_both_halfwords() {
        let mut e = engine();
        let source = e.mem.alloc(0x28);
        let this = e.mem.alloc(0x28);
        for word in 0..9 {
            e.mem.set_u32(source + 4 * word, 0x400 + word);
        }
        e.mem.set_u16(source + 0x20, 0xabcd);
        e.mem.set_u16(source + 0x22, 0x1234);
        e.mem.set_u16(this + 0x20, 0xffff);
        let got = e.call(0x00e2_3080, &args![this, source]).u32();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x402);
        assert_eq!(e.mem.u32(this + 0x1c), 0x407);
        assert_eq!(e.mem.u16(this + 0x20), 0xabcd);
        assert_eq!(e.mem.u16(this + 0x22), 0x1234);
        // With the destination starting at zero the result is the same.
        e.mem.set_u16(this + 0x20, 0);
        e.call(0x00e2_3080, &args![this, source]);
        assert_eq!(e.mem.u16(this + 0x20), 0xabcd);
    }

    /// Checks a block maker: the allocator is asked for `block_size` bytes, the 32 elements are
    /// chained from the first to the last, the block's free head and live count are set, and a
    /// second block goes first in the pool's array.
    fn check_new_block(address: u32, block_size: u32, stride: u32) {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let first = e.call(address, &args![pool]).u32();
        assert_eq!(*log.borrow(), vec![(4, block_size, 0)]);
        assert_eq!(e.mem.u32(pool + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(pool)), first);
        for index in 0..31 {
            assert_eq!(
                e.mem.u32(first + index * stride),
                first + (index + 1) * stride
            );
        }
        assert_eq!(e.mem.u32(first + 31 * stride), 0);
        assert_eq!(e.mem.u32(first + block_size - 8), first);
        assert_eq!(e.mem.u32(first + block_size - 4), 0);
        let second = e.call(address, &args![pool]).u32();
        assert_eq!(e.mem.u32(pool + 4), 2);
        let data = e.mem.u32(pool);
        assert_eq!(e.mem.u32(data), second);
        assert_eq!(e.mem.u32(data + 4), first);
    }

    #[test]
    fn fn_00e23530_makes_a_block_of_0x50_byte_elements() {
        check_new_block(0x00e2_3530, 0xa08, 0x50);
    }

    #[test]
    fn fn_00e235d0_makes_a_block_of_0x1c_byte_elements() {
        check_new_block(0x00e2_35d0, 0x388, 0x1c);
    }

    #[test]
    fn fn_00e23670_makes_a_block_of_0x14_byte_elements() {
        check_new_block(0x00e2_3670, 0x288, 0x14);
    }

    #[test]
    fn fn_00e23710_makes_a_block_of_0x24_byte_elements() {
        check_new_block(0x00e2_3710, 0x488, 0x24);
    }

    /// A pool (0x14 bytes) with `blocks` blocks made by the maker at `maker`.
    fn pool_with_blocks(e: &mut Engine, maker: u32, blocks: u32) -> u32 {
        let pool = e.mem.alloc(0x14);
        for _ in 0..blocks {
            e.call(maker, &args![pool]);
        }
        pool
    }

    /// Checks a block-releasing function (`00e234e0` and its siblings): every block goes back to
    /// the allocator and the block count, list head and live count are cleared.
    fn check_release_blocks(address: u32, maker: u32, block_size: u32) {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = pool_with_blocks(&mut e, maker, 2);
        let data = e.mem.u32(pool);
        let blocks = [e.mem.u32(data), e.mem.u32(data + 4)];
        e.mem.set_u32(pool + 0x0c, 0x1234);
        e.mem.set_u32(pool + 0x10, 5);
        log.borrow_mut().clear();
        e.call(address, &args![pool]);
        assert_eq!(
            *log.borrow(),
            vec![(8, blocks[0], block_size), (8, blocks[1], block_size)]
        );
        assert_eq!(e.mem.u32(pool + 4), 0);
        assert_eq!(e.mem.u32(pool + 0x0c), 0);
        assert_eq!(e.mem.u32(pool + 0x10), 0);
        // The array storage is kept.
        assert_ne!(e.mem.u32(pool), 0);
    }

    #[test]
    fn fn_00e234e0_releases_the_blocks_of_a_0xa08_pool() {
        check_release_blocks(0x00e2_34e0, 0x00e2_3530, 0xa08);
    }

    #[test]
    fn fn_00e239c0_releases_the_blocks_of_a_0x288_pool() {
        check_release_blocks(0x00e2_39c0, 0x00e2_3670, 0x288);
    }

    #[test]
    fn fn_00e23a10_releases_the_blocks_of_a_0x488_pool() {
        check_release_blocks(0x00e2_3a10, 0x00e2_3710, 0x488);
    }

    /// Checks a pool destructor: the blocks are freed, then the array's storage, and the array is
    /// left empty with the capacity word `0x80000000`.
    fn check_pool_destroy(address: u32, maker: u32, block_size: u32) {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = pool_with_blocks(&mut e, maker, 2);
        let data = e.mem.u32(pool);
        let capacity = e.mem.u32(pool + 8);
        let blocks = [e.mem.u32(data), e.mem.u32(data + 4)];
        log.borrow_mut().clear();
        e.call(address, &args![pool]);
        assert_eq!(
            *log.borrow(),
            vec![
                (8, blocks[0], block_size),
                (8, blocks[1], block_size),
                (0x10, data, capacity * 4)
            ]
        );
        assert_eq!(e.mem.u32(pool), 0);
        assert_eq!(e.mem.u32(pool + 4), 0);
        assert_eq!(e.mem.u32(pool + 8), 0x8000_0000);
        // Run again on the empty array: nothing is freed (the storage flag is set).
        log.borrow_mut().clear();
        e.call(address, &args![pool]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00e23860_destroys_a_0xa08_pool() {
        check_pool_destroy(0x00e2_3860, 0x00e2_3530, 0xa08);
    }

    #[test]
    fn fn_00e23c70_destroys_a_0x388_pool() {
        check_pool_destroy(0x00e2_3c70, 0x00e2_35d0, 0x388);
    }

    #[test]
    fn fn_00e23cf0_destroys_a_0x288_pool() {
        check_pool_destroy(0x00e2_3cf0, 0x00e2_3670, 0x288);
    }

    #[test]
    fn fn_00e23d70_destroys_a_0x488_pool() {
        check_pool_destroy(0x00e2_3d70, 0x00e2_3710, 0x488);
    }

    #[test]
    fn hkgp_abstract_mesh_reset_00e238e0_releases_both_pools() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        e.call(0x00e2_3530, &args![mesh + 0x08]);
        e.call(0x00e2_3530, &args![mesh + 0x20]);
        let vertex_block = e.mem.u32(e.mem.u32(mesh + 0x08));
        let triangle_block = e.mem.u32(e.mem.u32(mesh + 0x20));
        e.mem.set_u32(mesh + 0x14, 1);
        e.mem.set_u32(mesh + 0x2c, 2);
        log.borrow_mut().clear();
        e.call(0x00e2_38e0, &args![mesh]);
        assert_eq!(
            *log.borrow(),
            vec![(8, vertex_block, 0xa08), (8, triangle_block, 0xa08)]
        );
        assert_eq!(e.mem.u32(mesh + 0x14), 0);
        assert_eq!(e.mem.u32(mesh + 0x2c), 0);
    }

    #[test]
    fn hkgp_abstract_mesh_reset_00e24a80_releases_the_vertex_and_triangle_pools() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        e.call(0x00e2_3670, &args![mesh + 0x08]);
        e.call(0x00e2_3710, &args![mesh + 0x20]);
        let first_block = e.mem.u32(e.mem.u32(mesh + 0x08));
        let second_block = e.mem.u32(e.mem.u32(mesh + 0x20));
        log.borrow_mut().clear();
        e.call(0x00e2_4a80, &args![mesh]);
        assert_eq!(
            *log.borrow(),
            vec![(8, first_block, 0x288), (8, second_block, 0x488)]
        );
    }

    #[test]
    fn hkgp_abstract_mesh_scalar_deleting_destructor_00e23b50_destroys_both_pools() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let mesh = e.mem.alloc(0x40);
        e.mem.set_u16(mesh + 4, 0x40);
        let got = e.call(0x00e2_3b50, &args![mesh, 0u32]).u32();
        assert_eq!(got, mesh);
        assert_eq!(e.mem.u32(mesh), 0x0102_e388);
        // Each pool: the empty array's storage (data 0, 0 bytes) is "freed" and flagged empty.
        assert_eq!(*log.borrow(), vec![(0x10, 0, 0), (0x10, 0, 0)]);
        assert_eq!(e.mem.u32(mesh + 0x28), 0x8000_0000);
        assert_eq!(e.mem.u32(mesh + 0x10), 0x8000_0000);
        log.borrow_mut().clear();
        e.call(0x00e2_3b50, &args![mesh, 1u32]);
        assert_eq!(log.borrow().last(), Some(&(8, mesh, 0x40)));
    }

    #[test]
    fn hkgp_abstract_mesh_triangulator_00e24de0_destroys_both_pools() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let mesh = e.mem.alloc(0x40);
        e.mem.set_u16(mesh + 4, 0x40);
        let got = e.call(0x00e2_4de0, &args![mesh, 0u32]).u32();
        assert_eq!(got, mesh);
        assert_eq!(e.mem.u32(mesh), 0x0102_e388);
        assert_eq!(*log.borrow(), vec![(0x10, 0, 0), (0x10, 0, 0)]);
        log.borrow_mut().clear();
        e.call(0x00e2_4de0, &args![mesh, 1u32]);
        assert_eq!(log.borrow().last(), Some(&(8, mesh, 0x40)));
    }

    #[test]
    fn hkgp_triangulator_00e27df0_runs_its_cleanup_and_destroys_the_pools() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        e.register(0x00e2_75d0, |_, _| Ret::default());
        let object = e.mem.alloc(0x280);
        e.mem.set_u16(object + 4, 0x280);
        // The array of 8-byte elements at +0x40 owns storage of 4 elements.
        let storage = e.mem.alloc(0x20);
        e.mem.set_u32(object + 0x40, storage);
        e.mem.set_u32(object + 0x44, 3);
        e.mem.set_u32(object + 0x48, 4);
        let got = e.call(0x00e2_7df0, &args![object, 1u32]).u32();
        assert_eq!(got, object);
        assert_eq!(calls(&e, 0x00e2_75d0), vec![vec![object, 1]]);
        assert_eq!(e.mem.u32(object), 0x0102_e388);
        assert_eq!(e.mem.u32(object + 8), 0x0102_e388);
        assert_eq!(e.mem.u32(object + 0x40), 0);
        assert_eq!(e.mem.u32(object + 0x44), 0);
        assert_eq!(e.mem.u32(object + 0x48), 0x8000_0000);
        let frees = log.borrow().clone();
        assert!(frees.contains(&(0x10, storage, 32)));
        assert_eq!(frees.last(), Some(&(8, object, 0x280)));
    }

    #[test]
    fn hkgp_mesh_internals_simple_collector_add_leaf_00e237b0_pushes_the_leaf_word() {
        let mut e = engine();
        install_heap(&mut e);
        let collector = e.mem.alloc(0x20);
        let node = e.mem.alloc(0x40);
        for (index, leaf) in [0x11u32, 0x22, 0x33, 0x44, 0x55].into_iter().enumerate() {
            e.mem.set_u32(node + 0x20, leaf);
            e.call(0x00e2_37b0, &args![collector, node]);
            assert_eq!(e.mem.u32(collector + 8), index as u32 + 1);
        }
        let data = e.mem.u32(collector + 4);
        let words: Vec<u32> = (0..5).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(words, vec![0x11, 0x22, 0x33, 0x44, 0x55]);
    }

    #[test]
    fn hkgp_mesh_internals_simple_collector_destructor_00e23800_empties_the_array() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let collector = e.mem.alloc(0x20);
        let storage = e.mem.alloc(0x20);
        e.mem.set_u32(collector + 4, storage);
        e.mem.set_u32(collector + 8, 3);
        e.mem.set_u32(collector + 0xc, 6);
        let got = e.call(0x00e2_3800, &args![collector, 0u32]).u32();
        assert_eq!(got, collector);
        assert_eq!(*log.borrow(), vec![(0x10, storage, 24)]);
        assert_eq!(e.mem.u32(collector + 4), 0);
        assert_eq!(e.mem.u32(collector + 8), 0);
        assert_eq!(e.mem.u32(collector + 0xc), 0x8000_0000);
        assert_eq!(e.mem.u32(collector), 0x010c_d664);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        // The empty array does not own storage now: nothing more is freed, and bit 0 deletes.
        e.call(0x00e2_3800, &args![collector, 1u32]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![collector]]);
    }

    #[test]
    fn fn_00e23af0_empties_the_array_of_0x40_byte_elements() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x100);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, 3);
        e.mem.set_u32(array + 8, 4);
        let got = e.call(0x00e2_3af0, &args![array, 0u32]).u32();
        assert_eq!(got, array);
        assert_eq!(*log.borrow(), vec![(0x10, storage, 0x100)]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
        e.call(0x00e2_3af0, &args![array, 1u32]);
        assert_eq!(log.borrow().last(), Some(&(8, array, 0xc)));
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn fn_00e24760_empties_the_array_at_plus_4() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let object = e.mem.alloc(0x20);
        let storage = e.mem.alloc(0x20);
        e.mem.set_u32(object + 4, storage);
        e.mem.set_u32(object + 8, 2);
        e.mem.set_u32(object + 0xc, 8);
        e.call(0x00e2_4760, &args![object]);
        assert_eq!(*log.borrow(), vec![(0x10, storage, 32)]);
        assert_eq!(e.mem.u32(object + 4), 0);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(object + 0xc), 0x8000_0000);
        // Storage that is not owned (bit 31 of the capacity word) is not freed.
        e.mem.set_u32(object + 4, 0x1234);
        e.call(0x00e2_4760, &args![object]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(e.mem.u32(object + 4), 0);
    }

    #[test]
    fn fn_00e25f00_empties_the_nested_arrays() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let outer = e.mem.alloc(0x10);
        let entries = e.mem.alloc(0x20);
        let first_storage = e.mem.alloc(0x30);
        let second_storage = e.mem.alloc(0x30);
        e.mem.set_u32(outer, entries);
        e.mem.set_u32(outer + 4, 2);
        e.mem.set_u32(outer + 8, 2);
        // Entry 0 owns storage of two 12-byte elements, entry 1 does not own its storage.
        e.mem.set_u32(entries, first_storage);
        e.mem.set_u32(entries + 4, 1);
        e.mem.set_u32(entries + 8, 2);
        e.mem.set_u32(entries + 12, second_storage);
        e.mem.set_u32(entries + 16, 1);
        e.mem.set_u32(entries + 20, 0x8000_0002);
        let got = e.call(0x00e2_5f00, &args![outer, 0u32]).u32();
        assert_eq!(got, outer);
        assert_eq!(
            *log.borrow(),
            vec![(0x10, first_storage, 24), (0x10, entries, 24)]
        );
        assert_eq!(e.mem.u32(entries), 0);
        assert_eq!(e.mem.u32(entries + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(entries + 12), 0);
        assert_eq!(e.mem.u32(entries + 20), 0x8000_0000);
        assert_eq!(e.mem.u32(outer), 0);
        assert_eq!(e.mem.u32(outer + 4), 0);
        assert_eq!(e.mem.u32(outer + 8), 0x8000_0000);
        e.call(0x00e2_5f00, &args![outer, 1u32]);
        assert_eq!(log.borrow().last(), Some(&(8, outer, 0x10)));
    }

    #[test]
    fn fn_00e27400_destroys_the_object_at_plus_0x24c() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        // No object: nothing happens.
        e.call(0x00e2_7400, &args![mesh]);
        assert!(log.borrow().is_empty());
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(mesh + 0x24c, object);
        e.call(0x00e2_7400, &args![mesh]);
        assert_eq!(e.mem.u32(mesh + 0x24c), 0);
        // The destructor ran with flags 0 (an empty outer array: its storage is "freed" with 0
        // bytes), then the 16-byte block was returned.
        assert_eq!(*log.borrow(), vec![(0x10, 0, 0), (8, object, 0x10)]);
    }

    /// A pool with one block of `maker`'s kind, one element taken the way the pool functions do
    /// it (`pool_take_element`), as (pool, block).
    fn pool_with_one_block(e: &mut Engine, maker: u32) -> (u32, u32) {
        let pool = e.mem.alloc(0x14);
        let block = e.call(maker, &args![pool]).u32();
        (pool, block)
    }

    #[test]
    fn fn_00e24140_adds_a_copy_of_a_vertex_to_the_pool() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let source = e.mem.alloc(0x50);
        for word in 0..20 {
            e.mem.set_u32(source + 4 * word, 0x500 + word);
        }
        let first = e.call(0x00e2_4140, &args![pool, source]).u32();
        // The pool had no block: one of 0xa08 bytes was made; the element is its first.
        assert_eq!(*log.borrow(), vec![(4, 0xa08, 0)]);
        let block = e.mem.u32(e.mem.u32(pool));
        assert_eq!(first, block);
        assert_eq!(e.mem.u32(first + 0x10), 0x504);
        assert_eq!(e.mem.u32(first + 0x44), 0x511);
        assert_eq!(e.mem.u32(first + 4), 0);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u32(pool + 0x0c), first);
        assert_eq!(e.mem.u32(pool + 0x10), 1);
        assert_eq!(e.mem.u32(block + 0xa04), 1);
        assert_eq!(e.mem.u32(block + 0xa00), block + 0x50);
        // The second element is the next free one and goes in front.
        let second = e.call(0x00e2_4140, &args![pool, source]).u32();
        assert_eq!(second, block + 0x50);
        assert_eq!(e.mem.u32(second), first);
        assert_eq!(e.mem.u32(first + 4), second);
        assert_eq!(e.mem.u32(pool + 0x0c), second);
        assert_eq!(e.mem.u32(pool + 0x10), 2);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn fn_00e24140_makes_a_new_block_when_the_first_one_has_no_free_element() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let (pool, block) = pool_with_one_block(&mut e, 0x00e2_3530);
        let source = e.mem.alloc(0x50);
        // The first block's free head is empty.
        e.mem.set_u32(block + 0xa00, 0);
        let element = e.call(0x00e2_4140, &args![pool, source]).u32();
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(pool + 4), 2);
        let new_block = e.mem.u32(e.mem.u32(pool));
        assert_ne!(new_block, block);
        assert_eq!(element, new_block);
    }

    #[test]
    fn fn_00e241a0_adds_a_copy_of_a_triangle_to_the_pool() {
        let mut e = engine();
        install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let source = e.mem.alloc(0x50);
        for word in 0..20 {
            e.mem.set_u32(source + 4 * word, 0x600 + word);
        }
        let element = e.call(0x00e2_41a0, &args![pool, source]).u32();
        assert_eq!(e.mem.u32(element + 8), 0x602);
        assert_eq!(e.mem.u32(element + 0x40), 0x610);
        assert_eq!(e.mem.u32(element + 0x10 - 0x10 + 4), 0);
        assert_eq!(e.mem.u32(pool + 0x0c), element);
        assert_eq!(e.mem.u32(pool + 0x10), 1);
    }

    #[test]
    fn fn_00e247c0_adds_a_copy_of_a_0x14_byte_element_to_the_pool() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let source = e.mem.alloc(0x20);
        for word in 0..5 {
            e.mem.set_u32(source + 4 * word, 0x700 + word);
        }
        e.call(0x00e2_47c0, &args![pool, source]);
        assert_eq!(*log.borrow(), vec![(4, 0x288, 0)]);
        let element = e.mem.u32(pool + 0x0c);
        assert_eq!(e.mem.u32(element + 8), 0x702);
        assert_eq!(e.mem.u32(element + 0xc), 0x703);
        assert_eq!(e.mem.u32(element + 0x10), 0x704);
        assert_eq!(e.mem.u32(element + 4), 0);
        assert_eq!(e.mem.u32(pool + 0x10), 1);
        let block = e.mem.u32(e.mem.u32(pool));
        assert_eq!(e.mem.u32(block + 0x284), 1);
        assert_eq!(e.mem.u32(block + 0x280), block + 0x14);
    }

    #[test]
    fn fn_00e24850_adds_a_copy_of_a_0x24_byte_element_to_the_pool() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let source = e.mem.alloc(0x28);
        for word in 0..9 {
            e.mem.set_u32(source + 4 * word, 0x800 + word);
        }
        e.mem.set_u16(source + 0x20, 0x5a5a);
        e.mem.set_u16(source + 0x22, 0x3c3c);
        let element = e.call(0x00e2_4850, &args![pool, source]).u32();
        assert_eq!(*log.borrow(), vec![(4, 0x488, 0)]);
        assert_eq!(e.mem.u32(element + 8), 0x802);
        assert_eq!(e.mem.u16(element + 0x20), 0x5a5a);
        assert_eq!(e.mem.u16(element + 0x22), 0x3c3c);
        assert_eq!(e.mem.u32(pool + 0x0c), element);
        let block = e.mem.u32(e.mem.u32(pool));
        assert_eq!(e.mem.u32(block + 0x484), 1);
        assert_eq!(e.mem.u32(block + 0x480), block + 0x24);
    }

    #[test]
    fn pool_allocator_release_00e22f90_frees_the_block_with_its_last_element() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let source = e.mem.alloc(0x50);
        let first = e.call(0x00e2_4140, &args![pool, source]).u32();
        let second = e.call(0x00e2_4140, &args![pool, source]).u32();
        let block = e.mem.u32(e.mem.u32(pool));
        log.borrow_mut().clear();
        // A null element does nothing.
        e.call(0x00e2_2f90, &args![pool, 0u32]);
        assert_eq!(e.mem.u32(block + 0xa04), 2);
        e.call(0x00e2_2f90, &args![pool, first]);
        assert_eq!(e.mem.u32(block + 0xa04), 1);
        assert!(log.borrow().is_empty());
        e.call(0x00e2_2f90, &args![pool, second]);
        assert_eq!(*log.borrow(), vec![(8, block, 0xa08)]);
        assert_eq!(e.mem.u32(pool + 4), 0);
    }

    #[test]
    fn pool_allocator_release_00e22f90_moves_the_last_block_into_the_freed_slot() {
        let mut e = engine();
        install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let a = e.call(0x00e2_3530, &args![pool]).u32();
        let b = e.call(0x00e2_3530, &args![pool]).u32();
        // The array is [b, a]; empty block b by releasing its only live element.
        e.mem.set_u32(b + 0xa04, 1);
        e.call(0x00e2_2f90, &args![pool, b + 0x50]);
        assert_eq!(e.mem.u32(pool + 4), 1);
        assert_eq!(e.mem.u32(e.mem.u32(pool)), a);
    }

    /// Installs the reporter object at `01268360` whose slot `0x0c` returns `answer`, and the
    /// three string-buffer functions; returns the log of reporter calls.
    fn install_reporter(e: &mut Engine, answer: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let reports: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0030);
        let reporter = e.mem.alloc(0x10);
        e.mem.set_u32(reporter, vtable);
        e.mem.set_u32(0x0126_8360, reporter);
        let log = Rc::clone(&reports);
        e.register_double(0x0900_0030, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
        e.register(0x00cb_1600, |_, _| Ret::default());
        e.register(0x00cc_01c0, |_, _| Ret::default());
        e.register(0x00cc_00e0, |_, _| Ret::default());
        reports
    }

    #[test]
    fn pool_allocator_release_00e22f90_reports_an_element_outside_every_block() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let reports = install_reporter(&mut e, 0);
        let pool = e.mem.alloc(0x14);
        e.call(0x00e2_3530, &args![pool]);
        e.call(0x00e2_2f90, &args![pool, 0x1234_5678u32]);
        let reports = reports.borrow();
        assert_eq!(reports.len(), 1);
        let report = &reports[0];
        assert_eq!(report[1..3], [3, 0x9035_5365]);
        assert_eq!(report[4..], [0x010d_35f0, 0xaf]);
        let buffer = report[3];
        let init = calls(&e, 0x00cb_1600);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1..], [buffer, 0x200]);
        assert_eq!(buffer, init[0][0] + 0xc);
        assert_eq!(calls(&e, 0x00cc_01c0), vec![vec![init[0][0], 0x010d_365c]]);
        assert_eq!(calls(&e, 0x00cc_00e0), vec![vec![init[0][0]]]);
        // The pool is untouched.
        assert_eq!(e.mem.u32(pool + 4), 1);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00e23012")]
    fn pool_allocator_release_00e22f90_executes_int3_when_the_report_is_not_handled() {
        let mut e = engine();
        install_heap(&mut e);
        install_reporter(&mut e, 1);
        let pool = e.mem.alloc(0x14);
        e.call(0x00e2_2f90, &args![pool, 0x1234_5678u32]);
    }

    #[test]
    fn fn_00e24e40_empties_the_element_array_and_releases_the_element() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let pool = e.mem.alloc(0x14);
        let block = e.call(0x00e2_35d0, &args![pool]).u32();
        let element = block + 0x1c;
        let storage = e.mem.alloc(0x10);
        e.mem.set_u32(element, storage);
        e.mem.set_u32(element + 4, 2);
        e.mem.set_u32(element + 8, 4);
        e.mem.set_u32(block + 0x384, 2);
        log.borrow_mut().clear();
        e.call(0x00e2_4e40, &args![pool, element]);
        assert_eq!(*log.borrow(), vec![(0x10, storage, 16)]);
        assert_eq!(e.mem.u32(element), 0);
        assert_eq!(e.mem.u32(element + 4), 0);
        assert_eq!(e.mem.u32(element + 8), 0x8000_0000);
        assert_eq!(e.mem.u32(block + 0x384), 1);
        // The last live element frees the block.
        e.call(0x00e2_4e40, &args![pool, element]);
        assert_eq!(log.borrow().last(), Some(&(8, block, 0x388)));
        assert_eq!(e.mem.u32(pool + 4), 0);
        // A null element does nothing at all.
        let count = log.borrow().len();
        e.call(0x00e2_4e40, &args![pool, 0u32]);
        assert_eq!(log.borrow().len(), count);
    }

    #[test]
    #[should_panic(expected = "INT3 at 00e24f01")]
    fn fn_00e24e40_executes_int3_for_an_element_outside_every_block() {
        let mut e = engine();
        install_heap(&mut e);
        install_reporter(&mut e, 1);
        let pool = e.mem.alloc(0x14);
        let element = e.mem.alloc(0x20);
        e.call(0x00e2_4e40, &args![pool, element]);
    }

    /// The hash the edge table uses for the key (a, b), from the words at `+8` and `+0xc` of
    /// each vertex.
    fn edge_hash(e: &Engine, a: u32, b: u32) -> u32 {
        let first = e
            .mem
            .u32(a + 0xc)
            .wrapping_mul(0x0034_42a5)
            .wrapping_add(e.mem.u32(a + 8).wrapping_mul(0x2152_8000));
        let second = e
            .mem
            .u32(b + 0xc)
            .wrapping_mul(0x0019_58e9)
            .wrapping_sub(e.mem.u32(b + 8).wrapping_mul(0x538b_8000));
        first ^ second
    }

    #[test]
    fn fn_00e23900_finds_the_value_stored_for_an_edge() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let vertices: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x50)).collect();
        for (index, vertex) in vertices.iter().enumerate() {
            e.mem.set_u32(vertex + 8, 0x1000 + index as u32 * 0x77);
            e.mem.set_u32(vertex + 0xc, 0x2000 + index as u32 * 0x1313);
        }
        let triangle = add_triangle(&mut e, mesh, vertices[0], vertices[1], vertices[2]);
        let buckets = 5u32;
        let table = e.mem.alloc(0x10);
        let bucket_array = e.mem.alloc(12 * buckets);
        e.mem.set_u32(table, bucket_array);
        e.mem.set_u32(table + 4, buckets);
        e.mem.set_u32(mesh + 0x24c, table);
        // Edge 0 is (v0, v1), edge 2 is (v2, v0); only these two are stored.
        for (key, value) in [((0usize, 1usize), 0x77u32), ((2, 0), 0x99)] {
            let (a, b) = (vertices[key.0], vertices[key.1]);
            let bucket = bucket_array + 12 * (edge_hash(&e, a, b) % buckets);
            let entries = e.mem.alloc(24);
            let size = e.mem.u32(bucket + 4);
            // Entries of one bucket sit one after another; here each bucket has at most one.
            assert_eq!(size, 0);
            e.mem.set_u32(bucket, entries);
            e.mem.set_u32(bucket + 4, 1);
            e.mem.set_u32(entries, a);
            e.mem.set_u32(entries + 4, b);
            e.mem.set_u32(entries + 8, value);
        }
        let edge = e.mem.alloc(8);
        e.mem.set_u32(edge, triangle);
        for (index, expected) in [(0u32, 0x77u32), (1, 0), (2, 0x99)] {
            e.mem.set_u32(edge + 4, index);
            assert_eq!(e.call(0x00e2_3900, &args![mesh, edge]).u32(), expected);
        }
        // A null triangle and a missing table both give 0.
        e.mem.set_u32(edge, 0);
        assert_eq!(e.call(0x00e2_3900, &args![mesh, edge]).u32(), 0);
        e.mem.set_u32(edge, triangle);
        e.mem.set_u32(mesh + 0x24c, 0);
        assert_eq!(e.call(0x00e2_3900, &args![mesh, edge]).u32(), 0);
    }

    #[test]
    fn fn_00e23900_matches_both_vertices_of_the_key() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let vertices: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x50)).collect();
        let triangle = add_triangle(&mut e, mesh, vertices[0], vertices[1], vertices[2]);
        let table = e.mem.alloc(0x10);
        let bucket_array = e.mem.alloc(12);
        e.mem.set_u32(table, bucket_array);
        e.mem.set_u32(table + 4, 1);
        e.mem.set_u32(mesh + 0x24c, table);
        // One bucket with an entry whose second vertex differs, then the right one.
        let entries = e.mem.alloc(24);
        e.mem.set_u32(bucket_array, entries);
        e.mem.set_u32(bucket_array + 4, 2);
        e.mem.set_u32(entries, vertices[0]);
        e.mem.set_u32(entries + 4, vertices[2]);
        e.mem.set_u32(entries + 8, 0x11);
        e.mem.set_u32(entries + 12, vertices[0]);
        e.mem.set_u32(entries + 16, vertices[1]);
        e.mem.set_u32(entries + 20, 0x22);
        let edge = e.mem.alloc(8);
        e.mem.set_u32(edge, triangle);
        e.mem.set_u32(edge + 4, 0);
        assert_eq!(e.call(0x00e2_3900, &args![mesh, edge]).u32(), 0x22);
        // An empty bucket gives 0.
        e.mem.set_u32(bucket_array + 4, 0);
        assert_eq!(e.call(0x00e2_3900, &args![mesh, edge]).u32(), 0);
    }

    /// Checks one of the assertion-only functions: it reports with the given message words and
    /// does not stop when the report is handled (returns 0).
    fn check_assertion(
        address: u32,
        arguments: &[u32],
        text: u32,
        hash: u32,
        file: u32,
        line: u32,
    ) {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let reports = install_reporter(&mut e, 0);
        let result = e.call(address, arguments).u32();
        assert_eq!(result, 0);
        let reports = reports.borrow();
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0][1..3], [3, hash]);
        assert_eq!(reports[0][4..], [file, line]);
        let frame = calls(&e, 0x00cb_1600)[0][0];
        assert_eq!(reports[0][3], frame + 0xc);
        assert_eq!(calls(&e, 0x00cc_01c0), vec![vec![frame, text]]);
    }

    #[test]
    fn hkgp_mesh_iconvex_shape_get_max_index_00e21ce0_reports_and_returns_zero() {
        check_assertion(
            0x00e2_1ce0,
            &[0x1000],
            0x010e_ad80,
            0xcfa7_dfcc,
            0x010e_ad20,
            0x118,
        );
    }

    #[test]
    #[should_panic(expected = "INT3 at 00e21d2e")]
    fn hkgp_mesh_iconvex_shape_get_max_index_00e21ce0_executes_int3_when_unhandled() {
        let mut e = engine();
        install_reporter(&mut e, 1);
        e.call(0x00e2_1ce0, &args![0x1000u32]);
    }

    #[test]
    fn hk_gsk_shape_convert_vertex_ids_00e22520_reports() {
        check_assertion(
            0x00e2_2520,
            &[0x1000, 1, 2, 3],
            0x010e_ae6c,
            0x50cc_4da8,
            0x010e_ae18,
            0x2c,
        );
    }

    #[test]
    #[should_panic(expected = "INT3 at 00e2256b")]
    fn hk_gsk_shape_convert_vertex_ids_00e22520_executes_int3_when_unhandled() {
        let mut e = engine();
        install_reporter(&mut e, 1);
        e.call(0x00e2_2520, &args![0x1000u32, 1u32, 2u32, 3u32]);
    }

    #[test]
    fn hkgp_mesh_line_shape_get_bounding_box_00e21dd0_is_the_lane_wise_min_and_max() {
        let mut e = engine();
        let shape = e.mem.alloc(0x30);
        put_vec(&mut e, shape + 0x10, [1.0, 5.0, -2.0, 0.0]);
        put_vec(&mut e, shape + 0x20, [-3.0, 2.0, 4.0, 1.0]);
        let out = e.mem.alloc(0x20);
        e.call(0x00e2_1dd0, &args![shape, out]);
        assert_eq!(vec_at(&e, out), [-3.0, 2.0, -2.0, 0.0]);
        assert_eq!(vec_at(&e, out + 0x10), [1.0, 5.0, 4.0, 1.0]);
    }

    #[test]
    fn hkgp_mesh_line_shape_get_supporting_vertex_00e21d40_picks_the_larger_dot_product() {
        let mut e = engine();
        let shape = e.mem.alloc(0x30);
        put_vec(&mut e, shape + 0x10, [1.0, 0.0, 0.0, 7.0]);
        put_vec(&mut e, shape + 0x20, [3.0, 0.0, 0.0, 8.0]);
        let direction = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00e2_1d40, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out), [3.0, 0.0, 0.0, 8.0]);
        put_vec(&mut e, direction, [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00e2_1d40, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out), [1.0, 0.0, 0.0, 7.0]);
        // Equal dot products and unordered ones give the end point.
        put_vec(&mut e, direction, [0.0, 1.0, 0.0, 0.0]);
        e.call(0x00e2_1d40, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out), [3.0, 0.0, 0.0, 8.0]);
        put_vec(&mut e, direction, [f32::NAN, 0.0, 0.0, 0.0]);
        put_vec(&mut e, out, [0.0; 4]);
        e.call(0x00e2_1d40, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out)[0], 3.0);
    }

    #[test]
    fn hkgp_mesh_welding_policy_add_leaf_00e21e60_remembers_the_leaf_when_closer() {
        let mut e = engine();
        e.mem.set_f32(0x0101_6248, 0.5);
        let policy = e.mem.alloc(0x40);
        put_vec(&mut e, policy + 0x10, [1.0, 1.0, 2.0, 0.0]);
        let node = e.mem.alloc(0x30);
        put_vec(&mut e, node, [0.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, node + 0x10, [2.0, 2.0, 2.0, 0.0]);
        e.mem.set_u32(node + 0x20, 0x1234);
        // The squared distance from (1, 1, 1) to (1, 1, 2) is 1.
        e.mem.set_f32(policy + 0x20, 4.0);
        e.call(0x00e2_1e60, &args![policy, node]);
        assert_eq!(e.mem.u32(policy + 0x24), 0x1234);
        // Equal is not "below": the leaf is remembered again.
        e.mem.set_u32(policy + 0x24, 0);
        e.mem.set_f32(policy + 0x20, 1.0);
        e.call(0x00e2_1e60, &args![policy, node]);
        assert_eq!(e.mem.u32(policy + 0x24), 0x1234);
        // A smaller value at +0x20 leaves it alone.
        e.mem.set_u32(policy + 0x24, 0);
        e.mem.set_f32(policy + 0x20, 0.5);
        e.call(0x00e2_1e60, &args![policy, node]);
        assert_eq!(e.mem.u32(policy + 0x24), 0);
    }

    /// A triangle shape (`this + 4` is the triangle) on three vertices with the given positions.
    fn triangle_shape(e: &mut Engine, positions: [[f32; 4]; 3]) -> (u32, u32) {
        let mesh = new_mesh(e);
        let vertices = positions.map(|position| add_vertex(e, mesh, position));
        let triangle = add_triangle(e, mesh, vertices[0], vertices[1], vertices[2]);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 4, triangle);
        (shape, triangle)
    }

    #[test]
    fn hkgp_mesh_triangle_shape_get_supporting_vertex_00e21ed0_picks_the_extreme_vertex() {
        let mut e = engine();
        let direction = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        // (x of the three positions, direction x, expected vertex index)
        let cases = [
            ([1.0f32, 3.0, 2.0], 1.0f32, 1u32),
            ([1.0, 3.0, 2.0], -1.0, 0),
            ([1.0, 0.0, 5.0], 1.0, 2),
            ([0.0, 1.0, 5.0], 1.0, 2),
        ];
        for (xs, x, index) in cases {
            let positions = xs.map(|value| [value, 0.0, 0.0, 1.0]);
            let (shape, _) = triangle_shape(&mut e, positions);
            put_vec(&mut e, direction, [x, 0.0, 0.0, 0.0]);
            e.call(0x00e2_1ed0, &args![shape, direction, out]);
            assert_eq!(vec_at(&e, out)[..3], [xs[index as usize], 0.0, 0.0]);
            assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000 | index);
        }
    }

    #[test]
    fn hkgp_mesh_triangle_shape_get_supporting_vertex_00e21ed0_unordered_dots_pick_vertex_2() {
        let mut e = engine();
        let (shape, _) = triangle_shape(&mut e, [[1.0, 0.0, 0.0, 1.0]; 3]);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [f32::NAN, 0.0, 0.0, 0.0]);
        let out = e.mem.alloc(0x10);
        e.call(0x00e2_1ed0, &args![shape, direction, out]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0002);
    }

    /// Replaces `hkAabbUtil::calcAabb`: logs the words of every vector and writes the lane-wise
    /// minimum and maximum (four lanes) to `out`.
    fn install_calc_aabb(e: &mut Engine) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CALC_AABB, move |e, a| {
            let (vectors, count, out) = (a[0], a[1], a[2]);
            let words: Vec<u32> = (0..4 * count)
                .map(|word| e.mem.u32(vectors + 4 * word))
                .collect();
            log.borrow_mut().push(words.clone());
            let mut minimum = [f32::MAX; 4];
            let mut maximum = [f32::MIN; 4];
            for vector in words.chunks(4) {
                for lane in 0..4 {
                    let value = f32::from_bits(vector[lane]);
                    minimum[lane] = minimum[lane].min(value);
                    maximum[lane] = maximum[lane].max(value);
                }
            }
            put_vec(e, out, minimum);
            put_vec(e, out + 0x10, maximum);
            Ret::default()
        });
        seen
    }

    #[test]
    fn hkgp_mesh_triangle_shape_get_bounding_box_00e21fc0_boxes_the_three_positions() {
        let mut e = engine();
        let seen = install_calc_aabb(&mut e);
        let (shape, _) = triangle_shape(
            &mut e,
            [
                [1.0, 2.0, 3.0, 0.0],
                [-1.0, 5.0, 0.0, 0.0],
                [2.0, -4.0, 6.0, 0.0],
            ],
        );
        let out = e.mem.alloc(0x20);
        let got = e.call(0x00e2_1fc0, &args![shape, out]).u32();
        assert_eq!(got, out);
        assert_eq!(vec_at(&e, out)[..3], [-1.0, -4.0, 0.0]);
        assert_eq!(vec_at(&e, out + 0x10)[..3], [2.0, 5.0, 6.0]);
        let words = &seen.borrow()[0];
        assert_eq!(words.len(), 12);
        assert_eq!(words[0], 1.0f32.to_bits());
        assert_eq!(words[4], (-1.0f32).to_bits());
        assert_eq!(words[10], 6.0f32.to_bits());
    }

    #[test]
    fn hkgp_mesh_nearest_feature_policy_add_leaf_00e221e0_keeps_the_nearest_triangle() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        // The distance function answers with the first value of this list on each call and fills
        // its two vectors.
        let answers = Rc::new(RefCell::new(vec![3.0f32, 2.5, 2.5]));
        let list = Rc::clone(&answers);
        e.register_double(0x00de_9f10, move |e, a| {
            let value = list.borrow_mut().remove(0);
            put_vec(e, a[4], [value, 2.0, 3.0, 4.0]);
            put_vec(e, a[5], [5.0, 6.0, 7.0, 8.0]);
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        let (_, triangle) = triangle_shape(&mut e, [[0.0; 4]; 3]);
        let policy = e.mem.alloc(0x60);
        e.mem.set_f32(policy + 0x20, 4.0);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0x20, triangle);
        let got = e.call(0x00e2_21e0, &args![policy, node]).f32();
        assert_eq!(got, 3.0);
        assert_eq!(e.mem.f32(policy + 0x20), 3.0);
        assert_eq!(vec_at(&e, policy + 0x30), [3.0, 2.0, 3.0, 4.0]);
        assert_eq!(vec_at(&e, policy + 0x40), [5.0, 6.0, 7.0, 8.0]);
        assert_eq!(e.mem.u32(policy + 0x50), triangle);
        let call = &calls(&e, 0x00de_9f10)[0];
        assert_eq!(call[0], policy + 0x10);
        assert_eq!(call[1], e.mem.u32(triangle + 8) + 0x10);
        assert_eq!(call[2], e.mem.u32(triangle + 0xc) + 0x10);
        assert_eq!(call[3], e.mem.u32(triangle + 0x10) + 0x10);
        // A nearer one replaces it; an equal one does not.
        let got = e.call(0x00e2_21e0, &args![policy, node]).f32();
        assert_eq!(got, 2.5);
        e.mem.set_u32(policy + 0x50, 0);
        let got = e.call(0x00e2_21e0, &args![policy, node]).f32();
        assert_eq!(got, 2.5);
        assert_eq!(e.mem.u32(policy + 0x50), 0);
    }

    #[test]
    fn hkgp_mesh_internals_triangle_overlap_add_leaf_00e22270_records_a_hit() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let answers = Rc::new(RefCell::new(vec![0u32, 5, 0]));
        let list = Rc::clone(&answers);
        e.register_double(0x00de_ab10, move |_, _| Ret {
            eax: list.borrow_mut().remove(0),
            ..Ret::default()
        });
        let (_, triangle) = triangle_shape(
            &mut e,
            [
                [1.0, 2.0, 3.0, 4.0],
                [5.0, 6.0, 7.0, 8.0],
                [9.0, 10.0, 11.0, 12.0],
            ],
        );
        let overlap = e.mem.alloc(0x40);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0x20, triangle);
        assert_eq!(e.call(0x00e2_2270, &args![overlap, node]).f32(), 1.0);
        assert_eq!(e.mem.u32(overlap + 4), 0);
        assert_eq!(e.call(0x00e2_2270, &args![overlap, node]).f32(), 0.0);
        assert_eq!(e.mem.u32(overlap + 4), 1);
        // The hit stays recorded.
        assert_eq!(e.call(0x00e2_2270, &args![overlap, node]).f32(), 0.0);
        assert_eq!(e.mem.u32(overlap + 4), 1);
        assert_eq!(calls(&e, 0x00de_ab10).len(), 3);
        assert_eq!(calls(&e, 0x00de_ab10)[0][0], overlap + 0x10);
    }

    #[test]
    fn shape_bridge_convert_vertex_ids_00e22480_asks_for_a_unit_direction() {
        let mut e = engine();
        e.mem.set_f32(0x0101_2068, 1.0);
        e.mem.set_f32(0x0101_1d78, 0.0);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0040);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let seen: Rc<RefCell<Vec<(Vec<u32>, [f32; 4])>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0040, move |e, a| {
            log.borrow_mut().push((a.to_vec(), vec_at(e, a[1])));
            Ret {
                eax: 0x42,
                ..Ret::default()
            }
        });
        let got = e
            .call(0x00e2_2480, &args![object, 1u32, 2u32, 0x1234u32])
            .u32();
        assert_eq!(got, 0x42);
        let seen = seen.borrow();
        assert_eq!(seen[0].0[0], object);
        assert_eq!(seen[0].0[2], 0x1234);
        assert_eq!(seen[0].1, [1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn convex_overlap_impl_check_overlap_00e224d0_compares_the_distance_with_the_limit() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x0900_0050);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0050, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                st0: 2.5,
                ..Ret::default()
            }
        });
        for (limit, expected) in [(3.0f32, true), (2.5, true), (2.0, false), (f32::NAN, false)] {
            let got = e
                .call(0x00e2_24d0, &args![object, 7u32, 8u32, limit])
                .bool();
            assert_eq!(got, expected, "limit {limit}");
        }
        assert_eq!(seen.borrow()[0], vec![object, 7, 8, 1]);
    }

    #[test]
    fn shape_bridge_get_supporting_vertex_00e22470_forwards_to_slot_8_of_the_inner_object() {
        let mut e = engine();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0900_0060);
        let inner = e.mem.alloc(0x10);
        e.mem.set_u32(inner, vtable);
        let bridge = e.mem.alloc(0x10);
        e.mem.set_u32(bridge + 4, inner);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0060, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                eax: 0x5a,
                ..Ret::default()
            }
        });
        let got = e.call(0x00e2_2470, &args![bridge, 1u32, 2u32, 3u32]).u32();
        assert_eq!(got, 0x5a);
        assert_eq!(seen.borrow()[0], vec![inner, 1, 2, 3]);
    }

    /// A prism object (`this + 4`) on three vertices; each vertex has a position, a step
    /// vector at `+0x20` and the float at `+0x34` that scales it. Returns `this`.
    fn prism_shape(e: &mut Engine) -> u32 {
        let prism = e.mem.alloc(0x20);
        let vertices = [
            ([0.0f32, 0.0, 0.0, 9.0], [1.0f32, 0.0, 0.0, 0.0], 2.0f32),
            ([0.0, 1.0, 0.0, 9.0], [0.0, 0.0, 0.0, 0.0], 1.0),
            ([0.0, 0.0, 1.0, 9.0], [0.0, 0.0, 3.0, 0.0], 1.0),
        ];
        for (slot, (position, step, spread)) in vertices.into_iter().enumerate() {
            let vertex = e.mem.alloc(0x50);
            put_vec(e, vertex + 0x10, position);
            put_vec(e, vertex + 0x20, step);
            e.mem.set_f32(vertex + 0x34, spread);
            e.mem.set_u32(prism + 8 + 4 * slot as u32, vertex);
        }
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape + 4, prism);
        shape
    }

    #[test]
    fn hkgp_mesh_prism_shape_get_supporting_vertex_00e226b0_picks_the_extreme_of_six_points() {
        let mut e = engine();
        let shape = prism_shape(&mut e);
        let direction = e.mem.alloc(0x10);
        let out = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00e2_26b0, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out)[..3], [2.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0001);
        put_vec(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        e.call(0x00e2_26b0, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out)[..3], [0.0, 0.0, 4.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0005);
        // A direction no point is ahead in leaves the first point, tagged 0.
        put_vec(&mut e, direction, [-1.0, 0.0, 0.0, 0.0]);
        e.call(0x00e2_26b0, &args![shape, direction, out]);
        assert_eq!(vec_at(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000);
    }

    #[test]
    fn hkgp_mesh_prism_shape_get_bounding_box_00e227f0_boxes_the_six_points() {
        let mut e = engine();
        let seen = install_calc_aabb(&mut e);
        let shape = prism_shape(&mut e);
        let out = e.mem.alloc(0x20);
        let got = e.call(0x00e2_27f0, &args![shape, out]).u32();
        assert_eq!(got, out);
        let words = seen.borrow()[0].clone();
        assert_eq!(words.len(), 24);
        let tags: Vec<u32> = (0..6).map(|point| words[4 * point + 3]).collect();
        assert_eq!(
            tags,
            (0..6)
                .map(|index| 0x3f00_0000 | index)
                .collect::<Vec<u32>>()
        );
        // Points 1 and 5 are the positions pushed along the scaled steps: (2,0,0) and (0,0,4).
        assert_eq!(f32::from_bits(words[4]), 2.0);
        assert_eq!(f32::from_bits(words[20 + 2]), 4.0);
        assert_eq!(f32::from_bits(words[12 + 1]), 1.0);
        assert_eq!(vec_at(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(vec_at(&e, out + 0x10)[..3], [2.0, 1.0, 4.0]);
    }

    /// A mesh whose vertices and triangles are elements of real pool blocks (so that releasing
    /// them works): `vertices` positions and one triangle per entry of `corners` (indices into the
    /// vertices). Returns (mesh, vertices, triangles); the lists run newest first.
    fn pooled_mesh(
        e: &mut Engine,
        positions: &[[f32; 4]],
        corners: &[[usize; 3]],
    ) -> (u32, Vec<u32>, Vec<u32>) {
        let mesh = new_mesh(e);
        let source = e.mem.alloc(0x50);
        let vertices: Vec<u32> = positions
            .iter()
            .map(|position| {
                let vertex = e.call(0x00e2_4140, &args![mesh + 0x08, source]).u32();
                put_vec(e, vertex + 0x10, *position);
                vertex
            })
            .collect();
        let triangles: Vec<u32> = corners
            .iter()
            .map(|corner| {
                let triangle = e.call(0x00e2_41a0, &args![mesh + 0x20, source]).u32();
                for (slot, index) in corner.iter().enumerate() {
                    e.mem
                        .set_u32(triangle + 8 + 4 * slot as u32, vertices[*index]);
                }
                triangle
            })
            .collect();
        (mesh, vertices, triangles)
    }

    #[test]
    fn fn_00e12230_renames_a_part() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let triangles: Vec<u32> = [5u32, 7, 5]
            .iter()
            .map(|part| {
                let triangle = add_triangle(&mut e, mesh, 0, 0, 0);
                e.mem.set_u32(triangle + 0x34, *part);
                triangle
            })
            .collect();
        e.call(0x00e1_2230, &args![mesh, 5u32, 9u32]);
        let parts: Vec<u32> = triangles.iter().map(|t| e.mem.u32(t + 0x34)).collect();
        assert_eq!(parts, vec![9, 7, 9]);
    }

    #[test]
    fn fn_00e121c0_casts_a_ray_through_the_tree() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let seen: Rc<RefCell<Vec<(u32, [f32; 4], [f32; 4], u32, u32)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d1_d0b0, move |e, a| {
            let c = a[3];
            log.borrow_mut().push((
                e.mem.u32(c),
                vec_at(e, c + 0x10),
                vec_at(e, c + 0x20),
                e.mem.u32(c + 0x30),
                e.mem.u32(c + 0x34),
            ));
            e.mem.set_f32(c + 0x34, 0.25);
            e.mem.set_u32(c + 0x30, 0x1234);
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let from = e.mem.alloc(0x10);
        let to = e.mem.alloc(0x10);
        put_vec(&mut e, from, [1.0, 2.0, 3.0, 0.0]);
        put_vec(&mut e, to, [4.0, 6.0, 8.0, 0.0]);
        let hit = e.mem.alloc(8);
        let got = e.call(0x00e1_21c0, &args![mesh, from, to, hit]).f32();
        assert_eq!(got, 0.25);
        assert_eq!(e.mem.u32(hit), 0x1234);
        {
            let seen = seen.borrow();
            assert_eq!(seen[0].0, 0x010e_ade0);
            assert_eq!(seen[0].1, [1.0, 2.0, 3.0, 0.0]);
            assert_eq!(seen[0].2, [3.0, 4.0, 5.0, 0.0]);
            assert_eq!((seen[0].3, seen[0].4), (0, 1.0f32.to_bits()));
        }
        let call = &calls(&e, 0x00d1_d0b0)[0];
        assert_eq!(call[..3], [mesh + 0x40, from, to]);
        assert_eq!(call[4], 1.0f32.to_bits());
        // Without an output pointer nothing is written.
        let again = e.call(0x00e1_21c0, &args![mesh, from, to, 0u32]).f32();
        assert_eq!(again, 0.25);
    }

    #[test]
    fn fn_00e120e0_queries_the_tree_with_a_shape() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0070);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, vtable);
        // The shape's bounding box is written into the buffer it is given and returned.
        e.register(0x0900_0070, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let seen: Rc<RefCell<Vec<(u32, u32, u32, u32, u8, u8)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(QUERY_AABB, move |e, a| {
            let c = a[2];
            log.borrow_mut().push((
                e.mem.u32(c),
                e.mem.u32(c + 4),
                e.mem.u32(c + 8),
                e.mem.u32(c + 0xc),
                e.mem.u8(c + 0x10),
                e.mem.u8(c + 0x11),
            ));
            e.mem.set_u8(c + 0x10, 1);
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        e.mem.set_u32(mesh + 0x38, 0xabcd);
        let got = e
            .call(0x00e1_20e0, &args![mesh, shape, 0x7f80_0001u32, 1u32])
            .u8();
        assert_eq!(got, 1);
        let seen = seen.borrow();
        assert_eq!(seen[0], (0x010e_ada8, 0x7fc0_0001, 0xabcd, shape, 0, 1));
        let query = &calls(&e, QUERY_AABB)[0];
        assert_eq!(query[0], mesh + 0x40);
        assert_eq!(calls(&e, 0x0900_0070)[0][0], shape);
        assert_eq!(query[1], calls(&e, 0x0900_0070)[0][1]);
    }

    #[test]
    fn mesh_check_triangle_overlap_00e12260_queries_the_tree_with_a_triangle() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let boxes = install_calc_aabb(&mut e);
        let seen: Rc<RefCell<Vec<(u32, [f32; 4], [f32; 4], [f32; 4])>>> =
            Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(QUERY_AABB, move |e, a| {
            let c = a[2];
            log.borrow_mut().push((
                e.mem.u32(c),
                vec_at(e, c + 0x10),
                vec_at(e, c + 0x20),
                vec_at(e, c + 0x30),
            ));
            e.mem.set_u32(c + 4, 7);
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let points = e.mem.alloc(0x30);
        put_vec(&mut e, points, [1.0, 0.0, 0.0, 0.0]);
        put_vec(&mut e, points + 0x10, [0.0, 2.0, 0.0, 0.0]);
        put_vec(&mut e, points + 0x20, [0.0, 0.0, 3.0, 0.0]);
        let got = e
            .call(
                0x00e1_2260,
                &args![mesh, points, points + 0x10, points + 0x20],
            )
            .u32();
        assert_eq!(got, 7);
        let seen = seen.borrow();
        assert_eq!(seen[0].0, 0x010e_add4);
        assert_eq!(seen[0].1, [1.0, 0.0, 0.0, 0.0]);
        assert_eq!(seen[0].3, [0.0, 0.0, 3.0, 0.0]);
        assert_eq!(boxes.borrow()[0].len(), 12);
        assert_eq!(calls(&e, QUERY_AABB)[0][0], mesh + 0x40);
    }

    #[test]
    fn fn_00e122d0_finds_the_edge_between_two_vertices() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        let [a, b, c, d] = [0x100u32, 0x200, 0x300, 0x400];
        let triangle = add_triangle(&mut e, mesh, a, b, c);
        let out = e.mem.alloc(8);
        // (start, end, edge index): edge 0 runs 0 to 1, edge 1 runs 1 to 2, edge 2 runs 2 to 0.
        for (start, end, index) in [(a, b, 0u32), (b, c, 1), (c, a, 2)] {
            e.mem.set_u32(out, 0);
            e.call(0x00e1_22d0, &args![mesh, out, start, end]);
            assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (triangle, index));
        }
        // The first triangle in list order wins.
        let second = add_triangle(&mut e, mesh, a, b, d);
        e.call(0x00e1_22d0, &args![mesh, out, a, b]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (second, 0));
        // No match: the zero pair of the static (zeroed by the first run, then whatever it holds).
        e.mem.set_u32(0x0126_ebe8, 0x77);
        e.call(0x00e1_22d0, &args![mesh, out, c, d]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0, 0));
        e.mem.set_u32(0x0126_ebe8, 5);
        e.call(0x00e1_22d0, &args![mesh, out, c, d]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (5, 0));
    }

    #[test]
    fn mesh_fetch_positions_00e12640_appends_every_vertex_position() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        add_vertex(&mut e, mesh, [1.0, 2.0, 3.0, 4.0]);
        add_vertex(&mut e, mesh, [5.0, 6.0, 7.0, 8.0]);
        add_vertex(&mut e, mesh, [9.0, 10.0, 11.0, 12.0]);
        let array = new_array(&mut e);
        let got = e.call(0x00e1_2640, &args![mesh, array]).i32();
        assert_eq!(got, 3);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        // The list runs newest first.
        assert_eq!(vec_at(&e, data), [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(vec_at(&e, data + 0x20), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [3, 0x10]);
        // A second call needs 6 elements: twice the capacity of 3.
        e.call(0x00e1_2640, &args![mesh, array]);
        assert_eq!(calls(&e, ARRAY_RESERVE)[1][2..], [6, 0x10]);
        let data = e.mem.u32(array);
        assert_eq!(e.mem.u32(array + 4), 6);
        assert_eq!(vec_at(&e, data + 0x30), [9.0, 10.0, 11.0, 12.0]);
    }

    #[test]
    fn fn_00e12890_collects_and_sorts_the_triangles() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        e.register(0x00e2_2a50, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        let triangles: Vec<u32> = (0..3)
            .map(|_| {
                let triangle = add_triangle(&mut e, mesh, 0, 0, 0);
                e.mem.set_u32(triangle + 0x34, 4);
                triangle
            })
            .collect();
        e.mem.set_u32(mesh + 0x30, 3);
        let array = new_array(&mut e);
        e.mem.set_u32(array + 4, 99);
        e.call(0x00e1_2890, &args![mesh, array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 3);
        let data = e.mem.u32(array);
        let collected: Vec<u32> = (0..3).map(|index| e.mem.u32(data + 4 * index)).collect();
        assert_eq!(collected, vec![triangles[2], triangles[1], triangles[0]]);
        assert_eq!(e.mem.u32(triangles[0] + 0x34), 4);
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [3, 4]);
        assert_eq!(calls(&e, 0x00e2_2a50), vec![vec![data, 0, 2, 0]]);
        // With the reset byte the part words become -1; one triangle needs no sort.
        let single = new_mesh(&mut e);
        let only = add_triangle(&mut e, single, 0, 0, 0);
        e.mem.set_u32(only + 0x34, 4);
        e.mem.set_u32(single + 0x30, 1);
        let array = new_array(&mut e);
        e.call(0x00e1_2890, &args![single, array, 1u32]);
        assert_eq!(e.mem.u32(only + 0x34), 0xffff_ffff);
        assert_eq!(calls(&e, 0x00e2_2a50).len(), 1);
    }

    #[test]
    fn mesh_is_closed_manifold_00e12920_needs_every_link() {
        let mut e = engine();
        let mesh = new_mesh(&mut e);
        assert!(e.call(0x00e1_2920, &args![mesh]).bool());
        let triangle = add_triangle(&mut e, mesh, 0, 0, 0);
        assert!(!e.call(0x00e1_2920, &args![mesh]).bool());
        for slot in 0..3 {
            e.mem.set_u32(triangle + 0x14 + 4 * slot, 0x1234_0001);
        }
        assert!(e.call(0x00e1_2920, &args![mesh]).bool());
        // Only the low two bits set: the link is unset.
        e.mem.set_u32(triangle + 0x1c, 3);
        assert!(!e.call(0x00e1_2920, &args![mesh]).bool());
    }

    #[test]
    fn fn_00e13660_returns_the_distance_and_copies_the_location() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.register(0x00e1_3210, |e, a| {
            let record = a[2];
            put_vec(e, record, [1.0, 2.0, 3.0, 4.0]);
            e.mem.set_f32(record + 0x40, 0.5);
            e.mem.set_u8(record + 0x44, 1);
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let out = e.mem.alloc(0x10);
        let found = e.mem.alloc(8);
        let got = e
            .call(0x00e1_3660, &args![mesh, 0x1234u32, out, found])
            .f32();
        assert_eq!(got, 0.5);
        assert_eq!(vec_at(&e, out), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u8(found), 1);
        let call = &calls(&e, 0x00e1_3210)[0];
        assert_eq!(call[..2], [mesh, 0x1234]);
        // Without a flag pointer only the vector and the distance are returned.
        let got = e
            .call(0x00e1_3660, &args![mesh, 0x1234u32, out, 0u32])
            .f32();
        assert_eq!(got, 0.5);
    }

    #[test]
    fn mesh_get_volume_00e136c0_sums_the_determinants_and_divides() {
        let mut e = engine();
        e.mem.set_f64(0x0103_4208, 6.0);
        let results = Rc::new(RefCell::new(vec![6.0f32, 12.0]));
        let list = Rc::clone(&results);
        let columns: Rc<RefCell<Vec<Vec<f32>>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&columns);
        e.register_double(GET_DETERMINANT, move |e, a| {
            seen.borrow_mut()
                .push((0..12).map(|word| e.mem.f32(a[0] + 4 * word)).collect());
            let value = list.borrow_mut().remove(0);
            put_vec(e, a[1], [value, 0.0, 0.0, 0.0]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let mesh = new_mesh(&mut e);
        let a = add_vertex(&mut e, mesh, [1.0, 2.0, 3.0, 0.0]);
        let b = add_vertex(&mut e, mesh, [4.0, 5.0, 6.0, 0.0]);
        let c = add_vertex(&mut e, mesh, [7.0, 8.0, 9.0, 0.0]);
        for _ in 0..2 {
            let triangle = add_triangle(&mut e, mesh, a, b, c);
            for slot in 0..3 {
                e.mem.set_u32(triangle + 0x14 + 4 * slot, 0x1000);
            }
        }
        assert_eq!(e.call(0x00e1_36c0, &args![mesh]).f32(), 3.0);
        assert_eq!(columns.borrow()[0][..3], [1.0, 2.0, 3.0]);
        assert_eq!(columns.borrow()[0][8..11], [7.0, 8.0, 9.0]);
        // An open mesh gives 0.0 and does not call anything.
        let open = new_mesh(&mut e);
        add_triangle(&mut e, open, a, b, c);
        assert_eq!(e.call(0x00e1_36c0, &args![open]).f32(), 0.0);
        assert_eq!(columns.borrow().len(), 2);
    }

    /// Doubles for the convex hull functions `rebuildConvexHull` calls, with `dimensions` as the
    /// answer of `00d74e10`; returns the log of `build` calls (data, size, config bytes 9 and 10,
    /// and the first position).
    fn install_hull_functions(
        e: &mut Engine,
        dimensions: u32,
    ) -> Rc<RefCell<Vec<(u32, u32, u8, u8, [f32; 4])>>> {
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable, 0x0900_0020);
        e.register(0x0900_0020, |_, _| Ret::default());
        e.register(0x00d7_4d40, |_, _| Ret::default());
        e.register_double(0x00d7_6ca0, move |e, a| {
            e.mem.set_u32(a[0], vtable);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let builds: Rc<RefCell<Vec<(u32, u32, u8, u8, [f32; 4])>>> =
            Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&builds);
        e.register_double(0x00d7_71c0, move |e, a| {
            log.borrow_mut().push((
                a[1],
                a[2],
                e.mem.u8(a[3] + 9),
                e.mem.u8(a[3] + 0xa),
                vec_at(e, a[1]),
            ));
            Ret::default()
        });
        e.register_double(0x00d7_4e10, move |_, _| Ret {
            eax: dimensions,
            ..Ret::default()
        });
        builds
    }

    #[test]
    fn mesh_rebuild_convex_hull_00e13750_builds_a_hull_from_the_positions() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        let builds = install_hull_functions(&mut e, 3);
        let mesh = new_mesh(&mut e);
        for index in 0..4 {
            add_vertex(&mut e, mesh, [index as f32, 0.0, 0.0, 1.0]);
        }
        assert!(e.call(0x00e1_3750, &args![mesh]).bool());
        let hull = e.mem.u32(mesh + 0xe4);
        assert_ne!(hull, 0);
        assert_eq!(e.mem.u16(hull + 4), 0x14);
        let builds = builds.borrow();
        assert_eq!((builds[0].1, builds[0].2, builds[0].3), (4, 0, 1));
        // The positions run newest first.
        assert_eq!(builds[0].4, [3.0, 0.0, 0.0, 1.0]);
        // The hull block was allocated (0x14 bytes) and the positions array freed (4 x 16 bytes).
        let log = log.borrow();
        assert_eq!(log[0], (4, 0x14, 0));
        assert_eq!(log.last(), Some(&(0x10, builds[0].0, 64)));
        assert!(calls(&e, 0x0900_0020).is_empty());
    }

    #[test]
    fn mesh_rebuild_convex_hull_00e13750_deletes_a_hull_that_is_not_three_dimensional() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        install_hull_functions(&mut e, 2);
        let mesh = new_mesh(&mut e);
        for _ in 0..4 {
            add_vertex(&mut e, mesh, [0.0; 4]);
        }
        assert!(!e.call(0x00e1_3750, &args![mesh]).bool());
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
        assert_eq!(calls(&e, 0x0900_0020).len(), 1);
        assert_eq!(calls(&e, 0x0900_0020)[0][1], 1);
    }

    #[test]
    fn mesh_rebuild_convex_hull_00e13750_does_nothing_for_three_vertices_or_fewer() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        install_hull_functions(&mut e, 3);
        let mesh = new_mesh(&mut e);
        for _ in 0..3 {
            add_vertex(&mut e, mesh, [0.0; 4]);
        }
        // An old hull is still deleted first.
        let hull = install_hull(&mut e, mesh);
        assert!(e.call(0x00e1_3750, &args![mesh]).bool());
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00e13870_takes_the_determinant_of_the_edge_vectors() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let columns: Rc<RefCell<Vec<Vec<f32>>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&columns);
        e.register_double(GET_DETERMINANT, move |e, a| {
            seen.borrow_mut()
                .push((0..12).map(|word| e.mem.f32(a[0] + 4 * word)).collect());
            put_vec(e, a[1], [2.5, 0.0, 0.0, 0.0]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let mesh = new_mesh(&mut e);
        let own: Vec<u32> = (0..3)
            .map(|index| add_vertex(&mut e, mesh, [index as f32, 0.0, 0.0, 0.0]))
            .collect();
        let other: Vec<u32> = (0..3)
            .map(|index| add_vertex(&mut e, mesh, [0.0, 10.0 + index as f32, 0.0, 0.0]))
            .collect();
        let first = add_triangle(&mut e, mesh, own[0], own[1], own[2]);
        let second = add_triangle(&mut e, mesh, other[0], other[1], other[2]);
        // Link 0 of the first triangle is the second triangle's edge 1.
        e.mem.set_u32(first + 0x14, second | 1);
        let got = e.call(0x00e1_3870, &args![first, 0u32]).f32();
        assert_eq!(got, 2.5);
        // p = own[0]; q = other[1]; r = own[2] (vertex 2 precedes 0); s = other[0] (0 precedes 1).
        let columns = columns.borrow();
        assert_eq!(columns[0][..3], [0.0, 11.0, 0.0]);
        assert_eq!(columns[0][4..7], [2.0, 0.0, 0.0]);
        assert_eq!(columns[0][8..11], [0.0, 10.0, 0.0]);
        // An unset link gives 0.0 without a call.
        assert_eq!(e.call(0x00e1_3870, &args![first, 1u32]).f32(), 0.0);
        assert_eq!(calls(&e, GET_DETERMINANT).len(), 1);
    }

    #[test]
    fn fn_00e13f10_unlinks_and_releases_a_triangle() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let (mesh, _, triangles) =
            pooled_mesh(&mut e, &[[0.0; 4]; 3], &[[0, 1, 2], [0, 1, 2], [0, 1, 2]]);
        // The list runs [t2, t1, t0]; t1 is in the middle. Link t1's slot 1 to t0's slot 2 and t0's
        // slot 2 back to t1.
        let (t0, t1, t2) = (triangles[0], triangles[1], triangles[2]);
        e.mem.set_u32(t1 + 0x18, t0 | 2);
        e.mem.set_u32(t0 + 0x1c, t1 | 1);
        e.mem.set_u32(t1 + 0x14, 0x3);
        log.borrow_mut().clear();
        e.call(0x00e1_3f10, &args![mesh, t1]);
        assert_eq!(e.mem.u32(t0 + 0x1c), 0);
        assert_eq!(e.mem.u32(t1 + 0x14), 0);
        assert_eq!(e.mem.u32(t1 + 0x18), 0);
        assert_eq!(e.mem.u32(t2), t0);
        assert_eq!(e.mem.u32(t0 + 4), t2);
        assert_eq!(e.mem.u32(mesh + 0x30), 2);
        let block = e.mem.u32(e.mem.u32(mesh + 0x20));
        assert_eq!(e.mem.u32(block + 0xa04), 2);
        // The head: its successor loses its previous link and becomes the head.
        e.call(0x00e1_3f10, &args![mesh, t2]);
        assert_eq!(e.mem.u32(mesh + 0x2c), t0);
        assert_eq!(e.mem.u32(t0 + 4), 0);
        assert_eq!(e.mem.u32(mesh + 0x30), 1);
        // The last triangle empties the block, which goes back to the allocator.
        e.call(0x00e1_3f10, &args![mesh, t0]);
        assert_eq!(e.mem.u32(mesh + 0x2c), 0);
        assert_eq!(e.mem.u32(mesh + 0x30), 0);
        assert_eq!(*log.borrow(), vec![(8, block, 0xa08)]);
    }

    #[test]
    fn mesh_remove_orphan_vertices_00e14a90_removes_vertices_no_triangle_uses() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        // The orphan is added last, so it is the head of the list.
        let (mesh, vertices, _) = pooled_mesh(&mut e, &[[0.0; 4]; 4], &[[0, 1, 2]]);
        let hull = install_hull(&mut e, mesh);
        log.borrow_mut().clear();
        assert_eq!(e.call(0x00e1_4a90, &args![mesh]).u32(), 1);
        assert_eq!(e.mem.u32(mesh + 0x14), vertices[2]);
        assert_eq!(e.mem.u32(vertices[2] + 4), 0);
        assert_eq!(e.mem.u32(mesh + 0x18), 3);
        assert_eq!(e.mem.f32(vertices[0] + 0x30), 1.0);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
        // Nothing left to remove: no more deletions of the hull.
        install_hull(&mut e, mesh);
        let before = calls(&e, 0x0900_0020).len();
        assert_eq!(e.call(0x00e1_4a90, &args![mesh]).u32(), 0);
        assert_eq!(calls(&e, 0x0900_0020).len(), before);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn mesh_remove_orphan_vertices_00e14a90_unlinks_an_orphan_in_the_middle_and_at_the_tail() {
        let mut e = engine();
        install_heap(&mut e);
        // The list runs [v3, v2, v1, v0]; triangle (v3, v1, v0) leaves v2 (middle) orphaned, then
        // a second pass with a triangle on (v3, v1) and v0 as a corner leaves v0 (the tail)
        // orphaned once the triangle is gone.
        let (mesh, vertices, triangles) = pooled_mesh(&mut e, &[[0.0; 4]; 4], &[[3, 1, 0]]);
        assert_eq!(e.call(0x00e1_4a90, &args![mesh]).u32(), 1);
        assert_eq!(e.mem.u32(vertices[3]), vertices[1]);
        assert_eq!(e.mem.u32(vertices[1] + 4), vertices[3]);
        e.mem.set_u32(triangles[0] + 8, vertices[3]);
        e.mem.set_u32(triangles[0] + 0xc, vertices[3]);
        e.mem.set_u32(triangles[0] + 0x10, vertices[1]);
        assert_eq!(e.call(0x00e1_4a90, &args![mesh]).u32(), 1);
        assert_eq!(e.mem.u32(vertices[1]), 0);
        assert_eq!(e.mem.u32(mesh + 0x18), 2);
        // No triangles: no vertex is used and the whole list goes.
        let (mesh, _, _) = pooled_mesh(&mut e, &[[0.0; 4]; 2], &[]);
        assert_eq!(e.call(0x00e1_4a90, &args![mesh]).u32(), 2);
        assert_eq!(e.mem.u32(mesh + 0x14), 0);
        assert_eq!(e.mem.u32(mesh + 0x18), 0);
        // And an empty mesh returns 0 at once.
        let empty = new_mesh(&mut e);
        assert_eq!(e.call(0x00e1_4a90, &args![empty]).u32(), 0);
    }

    #[test]
    fn fn_00e14fe0_removes_the_triangles_of_a_part() {
        let mut e = engine();
        install_heap(&mut e);
        let (mesh, _, triangles) =
            pooled_mesh(&mut e, &[[0.0; 4]; 3], &[[0, 1, 2], [0, 1, 2], [0, 1, 2]]);
        for (triangle, part) in triangles.iter().zip([1u32, 2, 1]) {
            e.mem.set_u32(triangle + 0x34, part);
        }
        e.mem.set_u8(mesh + 0x101, 1);
        e.call(0x00e1_4fe0, &args![mesh, 1u32]);
        assert_eq!(e.mem.u32(mesh + 0x30), 1);
        assert_eq!(e.mem.u32(mesh + 0x2c), triangles[1]);
        assert_eq!(e.mem.u8(mesh + 0x101), 0);
    }

    #[test]
    fn mesh_remove_part_from_classification_00e15020_removes_the_triangles_on_one_side() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        e.mem.set_f32(0x0106_c2e0, 0.5);
        // Transforming moves the point's x by 100.
        e.register(0x00c7_f640, |e, a| {
            let point = vec_at(e, a[2]);
            put_vec(e, a[0], [point[0] + 100.0, point[1], point[2], point[3]]);
            Ret::default()
        });
        // The locate function answers with the side byte 1 for the first triangle and 0xff for the second.
        let sides = Rc::new(RefCell::new(vec![1u8, 0xff]));
        let list = Rc::clone(&sides);
        let points: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = Rc::clone(&points);
        e.register_double(0x00e1_3210, move |e, a| {
            seen.borrow_mut().push(vec_at(e, a[1]));
            let side = list.borrow_mut().remove(0);
            e.mem.set_u8(a[2] + 0x44, side);
            Ret::default()
        });
        let (mesh, vertices, triangles) = pooled_mesh(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 0.0],
                [2.0, 0.0, 0.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
            ],
            &[[0, 1, 2], [0, 1, 2], [0, 1, 2]],
        );
        let _ = vertices;
        // Triangles at +0x30: [t0 -> 5, t1 -> 9, t2 -> 5]; the list runs [t2, t1, t0].
        for (triangle, part) in triangles.iter().zip([5u32, 9, 5]) {
            e.mem.set_u32(triangle + 0x30, part);
        }
        e.mem.set_u8(mesh + 0x101, 1);
        let transform = e.mem.alloc(0x40);
        let other = new_mesh(&mut e);
        // Part 5, side 1: t2 is located first (side 1: removed), then t0 (0xff, which is -1 and is
        // not equal to the zero-extended 1: kept).
        e.call(0x00e1_5020, &args![mesh, 5u32, transform, other, 1u32]);
        assert_eq!(e.mem.u32(mesh + 0x30), 2);
        assert_eq!(e.mem.u32(mesh + 0x2c), triangles[1]);
        assert_eq!(e.mem.u8(mesh + 0x101), 0);
        // The point is the centroid sum times 0.5, moved by 100: (0 + 2 + 0) * 0.5 = 1 (+100).
        assert_eq!(points.borrow()[0][..3], [101.0, 1.0, 0.0]);
        let locate = &calls(&e, 0x00e1_3210)[0];
        assert_eq!(locate[0], other);
    }

    #[test]
    fn mesh_reset_00e157f0_clears_the_mesh() {
        let mut e = engine();
        let log = install_heap(&mut e);
        put_vec(&mut e, 0x0126_7e30, [1.0, 2.0, 3.0, 4.0]);
        let (mesh, _, _) = pooled_mesh(&mut e, &[[0.0; 4]; 2], &[[0, 1, 1]]);
        e.call_log = Some(vec![]);
        let hull = install_hull(&mut e, mesh);
        e.mem.set_u8(mesh + 0x100, 1);
        e.mem.set_u8(mesh + 0x101, 1);
        log.borrow_mut().clear();
        e.call(0x00e1_57f0, &args![mesh]);
        assert_eq!(calls(&e, 0x0900_0020), vec![vec![hull, 1]]);
        assert_eq!(vec_at(&e, mesh + 0xf0), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(mesh + 0x14), 0);
        assert_eq!(e.mem.u32(mesh + 0x2c), 0);
        assert_eq!(e.mem.u8(mesh + 0x100), 0);
        assert_eq!(e.mem.u8(mesh + 0x101), 0);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn hkgp_mesh_destructor_00e18a50_releases_everything() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        e.register(0x00d9_be30, |_, _| Ret::default());
        e.register(0x00d9_c010, |_, _| Ret::default());
        let (mesh, _, _) = pooled_mesh(&mut e, &[[0.0; 4]; 2], &[[0, 1, 1]]);
        // The three arrays own storage: words (capacity 4), 16-byte elements (2 and 3).
        let words = e.mem.alloc(0x10);
        let first = e.mem.alloc(0x20);
        let second = e.mem.alloc(0x30);
        for (offset, storage, capacity) in
            [(0xd8u32, words, 4u32), (0xcc, first, 2), (0xc0, second, 3)]
        {
            e.mem.set_u32(mesh + offset, storage);
            e.mem.set_u32(mesh + offset + 4, 1);
            e.mem.set_u32(mesh + offset + 8, capacity);
        }
        log.borrow_mut().clear();
        e.call(0x00e1_8a50, &args![mesh]);
        let frees: Vec<(u32, u32, u32)> = log
            .borrow()
            .iter()
            .filter(|entry| entry.0 == 0x10)
            .copied()
            .collect();
        assert_eq!(
            frees[..3],
            [(0x10, words, 16), (0x10, first, 32), (0x10, second, 48)]
        );
        for offset in [0xd8u32, 0xcc, 0xc0] {
            assert_eq!(e.mem.u32(mesh + offset), 0);
            assert_eq!(e.mem.u32(mesh + offset + 4), 0);
            assert_eq!(e.mem.u32(mesh + offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(mesh + 0x40), 0x010d_40a4);
        assert_eq!(calls(&e, 0x00d9_be30), vec![vec![mesh + 0x40]]);
        assert_eq!(calls(&e, 0x00d9_c010), vec![vec![mesh + 0x5c]]);
        assert_eq!(e.mem.u32(mesh), 0x0102_e388);
        assert_eq!(e.mem.u32(mesh + 0x08), 0);
        assert_eq!(e.mem.u32(mesh + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(mesh + 0x28), 0x8000_0000);
    }

    #[test]
    fn hkgp_mesh_constructor_00e1b750_sets_up_the_empty_mesh() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.register(0x00d9_c110, |_, _| Ret::default());
        e.register(0x00ec_6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        e.mem.set_u32(0x010d_51c4, 0x358637bd);
        e.mem.set_f64(0x0102_1928, 3.0);
        e.mem.set_f64(0x0102_90b0, 0.25);
        e.mem.set_f64(0x0109_cc78, 1.0e-3);
        let mesh = e.mem.alloc(0x260);
        // Garbage that the constructor must overwrite.
        e.mem.set_u32(mesh + 0xe4, 0x77);
        e.mem.set_u8(mesh + 0x100, 1);
        let got = e.call(0x00e1_b750, &args![mesh]).u32();
        assert_eq!(got, mesh);
        assert_eq!(e.mem.u16(mesh + 6), 1);
        assert_eq!(e.mem.u32(mesh), 0x010e_b000);
        assert_eq!(e.mem.u32(mesh + 0x10), 0x8000_0000);
        assert_eq!(e.mem.u32(mesh + 0x28), 0x8000_0000);
        for offset in [0xc0u32, 0xcc, 0xd8] {
            assert_eq!(e.mem.u32(mesh + offset + 8), 0x8000_0000);
        }
        assert_eq!(e.mem.u32(mesh + 0x104), 0x358637bd);
        assert_eq!(e.mem.u32(mesh + 0x38), 0x011b_dee0);
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
        assert_eq!(e.mem.u8(mesh + 0x100), 0);
        assert_eq!(calls(&e, 0x00d9_c110), vec![vec![mesh + 0x40]]);
        let root = f64::from(3.0f64.sqrt() as f32);
        let expected = (((root * 0.25) * 1.0e-3) * 2.0) as f32;
        assert_eq!(e.mem.f32(mesh + 0x108), expected);
        assert_eq!(e.mem.u32(mesh + 0x10c), 0);
    }

    #[test]
    fn fn_00e25ba0_makes_the_edge_table_of_1031_buckets() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        e.call(0x00e2_5ba0, &args![mesh]);
        let table = e.mem.u32(mesh + 0x24c);
        assert_ne!(table, 0);
        assert_eq!(log.borrow()[0], (4, 0x10, 0));
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][1..], [table, 0x407, 0xc]);
        assert_eq!(e.mem.u32(table + 4), 0x407);
        let data = e.mem.u32(table);
        for bucket in [0u32, 1, 0x406] {
            assert_eq!(e.mem.u32(data + bucket * 12), 0);
            assert_eq!(e.mem.u32(data + bucket * 12 + 4), 0);
            assert_eq!(e.mem.u32(data + bucket * 12 + 8), 0x8000_0000);
        }
    }

    #[test]
    fn fn_00e25ba0_leaves_the_pointer_zero_when_the_allocation_fails() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        e.register(0x0900_0004, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        e.mem.set_u32(mesh + 0x24c, 0x1234);
        e.call(0x00e2_5ba0, &args![mesh]);
        assert_eq!(e.mem.u32(mesh + 0x24c), 0);
        assert!(calls(&e, ARRAY_RESERVE).is_empty());
    }

    /// A triangle on the plane z = 0 with a plane vector (0, 0, 1) and offset 0, and a ray caster
    /// (start (0.25, 0.25, 1), direction (0, 0, -2), nearest fraction 1.0) with the constants and
    /// the point-in-triangle double in place. Returns (caster, node, triangle, log of point calls).
    fn ray_caster(
        e: &mut Engine,
        inside: bool,
    ) -> (u32, u32, u32, Rc<RefCell<Vec<(Vec<u32>, [f32; 4])>>>) {
        e.mem.set_f32(0x0104_da4c, 1.0e-20);
        e.mem.set_u32(0x010c_8440, 0x3a83_126f);
        let seen: Rc<RefCell<Vec<(Vec<u32>, [f32; 4])>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(CHECK_POINT_INSIDE_TRIANGLE, move |e, a| {
            log.borrow_mut().push((a.to_vec(), vec_at(e, a[0])));
            Ret {
                eax: u32::from(inside),
                ..Ret::default()
            }
        });
        let mesh = new_mesh(e);
        let a = add_vertex(e, mesh, [0.0, 0.0, 0.0, 0.0]);
        let b = add_vertex(e, mesh, [1.0, 0.0, 0.0, 0.0]);
        let c = add_vertex(e, mesh, [0.0, 1.0, 0.0, 0.0]);
        let triangle = add_triangle(e, mesh, a, b, c);
        put_vec(e, triangle + 0x20, [0.0, 0.0, 1.0, 0.0]);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0x20, triangle);
        let caster = e.mem.alloc(0x40);
        put_vec(e, caster + 0x10, [0.25, 0.25, 1.0, 0.0]);
        put_vec(e, caster + 0x20, [0.0, 0.0, -2.0, 0.0]);
        e.mem.set_f32(caster + 0x34, 1.0);
        (caster, node, triangle, seen)
    }

    #[test]
    fn hkgp_mesh_internals_ray_caster_add_leaf_00e22320_records_a_nearer_hit() {
        let mut e = engine();
        let (caster, node, triangle, seen) = ray_caster(&mut e, true);
        let got = e.call(0x00e2_2320, &args![caster, node]).f32();
        assert_eq!(got, 0.5);
        assert_eq!(e.mem.f32(caster + 0x34), 0.5);
        assert_eq!(e.mem.u32(caster + 0x30), triangle);
        let seen = seen.borrow();
        assert_eq!(seen[0].1[..3], [0.25, 0.25, 0.0]);
        assert_eq!(seen[0].0[4], 0x3a83_126f);
        let vertices: Vec<u32> = (0..3)
            .map(|slot| e.mem.u32(triangle + 8 + 4 * slot) + 0x10)
            .collect();
        assert_eq!(seen[0].0[1..4], vertices[..]);
    }

    #[test]
    fn hkgp_mesh_internals_ray_caster_add_leaf_00e22320_ignores_what_cannot_be_nearer() {
        // A hit that is not inside the triangle changes nothing.
        let mut e = engine();
        let (caster, node, _, seen) = ray_caster(&mut e, false);
        assert_eq!(e.call(0x00e2_2320, &args![caster, node]).f32(), 1.0);
        assert_eq!(e.mem.u32(caster + 0x30), 0);
        assert_eq!(seen.borrow().len(), 1);
        // The best hit is nearer than this one (0.5): no point test at all.
        e.mem.set_f32(caster + 0x34, 0.25);
        assert_eq!(e.call(0x00e2_2320, &args![caster, node]).f32(), 0.25);
        assert_eq!(seen.borrow().len(), 1);
        // A ray parallel to the plane (denominator 0), and one pointing away (fraction below 0).
        put_vec(&mut e, caster + 0x20, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(caster + 0x34, 1.0);
        assert_eq!(e.call(0x00e2_2320, &args![caster, node]).f32(), 1.0);
        put_vec(&mut e, caster + 0x20, [0.0, 0.0, 2.0, 0.0]);
        assert_eq!(e.call(0x00e2_2320, &args![caster, node]).f32(), 1.0);
        assert_eq!(seen.borrow().len(), 1);
    }

    /// Doubles for the shape (vtable `0x0900_0300`: slot 8 gives the vertices of `vertices` one
    /// call after the other, logging the directions) and the other shape (`0x0900_0400`, slot 4
    /// answers `answer`, logging its words); returns (this, node, directions, query words).
    fn collide_fixture(
        e: &mut Engine,
        vertices: Vec<[f32; 4]>,
        answer: f32,
    ) -> (
        u32,
        u32,
        Rc<RefCell<Vec<[f32; 4]>>>,
        Rc<RefCell<Vec<(Vec<u32>, u32, u32)>>>,
    ) {
        e.mem.set_f64(0x0101_7cf8, -1000.0);
        e.mem.set_f32(0x0109_59f4, -5.0);
        put_vec(e, 0x010c_44d0, [f32::from_bits(0x8000_0000); 4]);
        let shape_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(shape_vtable + 8, 0x0900_0300);
        let shape = e.mem.alloc(0x10);
        e.mem.set_u32(shape, shape_vtable);
        let other_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(other_vtable + 4, 0x0900_0400);
        let other = e.mem.alloc(0x10);
        e.mem.set_u32(other, other_vtable);
        let directions: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(Vec::new()));
        let queue = Rc::new(RefCell::new(vertices));
        let seen = Rc::clone(&directions);
        e.register_double(0x0900_0300, move |e, a| {
            seen.borrow_mut().push(vec_at(e, a[1]));
            let vertex = queue.borrow_mut().remove(0);
            put_vec(e, a[2], vertex);
            Ret::default()
        });
        let queries: Rc<RefCell<Vec<(Vec<u32>, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&queries);
        e.register_double(0x0900_0400, move |e, a| {
            log.borrow_mut()
                .push((a.to_vec(), e.mem.u32(a[2]), e.mem.u32(a[2] + 4)));
            Ret {
                st0: f64::from(answer),
                ..Ret::default()
            }
        });
        let mesh = new_mesh(e);
        let a = add_vertex(e, mesh, [0.0; 4]);
        let triangle = add_triangle(e, mesh, a, a, a);
        put_vec(e, triangle + 0x20, [0.0, 0.0, 1.0, 0.0]);
        let node = e.mem.alloc(0x30);
        e.mem.set_u32(node + 0x20, triangle);
        let this = e.mem.alloc(0x20);
        e.mem.set_f32(this + 4, 0.1);
        e.mem.set_u32(this + 8, other);
        e.mem.set_u32(this + 0xc, shape);
        e.mem.set_u8(this + 0x11, 1);
        (this, node, directions, queries)
    }

    #[test]
    fn hkgp_mesh_collide_shape_triangle_add_leaf_00e22050_flags_a_collision() {
        let mut e = engine();
        let (this, node, directions, queries) = collide_fixture(
            &mut e,
            vec![[0.0, 0.0, 2.0, 0.0], [0.0, 0.0, -2.0, 0.0]],
            0.05,
        );
        let got = e.call(0x00e2_2050, &args![this, node]).f32();
        assert_eq!(got, 0.0);
        assert_eq!(e.mem.u8(this + 0x10), 1);
        let directions = directions.borrow();
        assert_eq!(directions[0], [0.0, 0.0, 1.0, 0.0]);
        assert_eq!(directions[1][2], -1.0);
        let queries = queries.borrow();
        assert_eq!(queries[0].0[1], e.mem.u32(this + 0xc));
        assert_eq!(queries[0].0[3], 1);
        assert_eq!(queries[0].1, 0x010e_adb4);
        assert_eq!(queries[0].2, e.mem.u32(node + 0x20));
    }

    #[test]
    fn hkgp_mesh_collide_shape_triangle_add_leaf_00e22050_gives_1_when_the_shape_is_clear() {
        // Behind the plane by more than the margin (-0.1): the first test already passes the shape.
        let mut e = engine();
        let (this, node, directions, _) =
            collide_fixture(&mut e, vec![[0.0, 0.0, -1.0, 0.0]], 0.05);
        assert_eq!(e.call(0x00e2_2050, &args![this, node]).f32(), 1.0);
        assert_eq!(directions.borrow().len(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 0);
        // The vertex against the plane is too far in front: the second test passes it.
        let mut e = engine();
        let (this, node, directions, _) = collide_fixture(
            &mut e,
            vec![[0.0, 0.0, 2.0, 0.0], [0.0, 0.0, 5.0, 0.0]],
            0.05,
        );
        assert_eq!(e.call(0x00e2_2050, &args![this, node]).f32(), 1.0);
        assert_eq!(directions.borrow().len(), 2);
        // The other shape reports a float not below the limit: no collision.
        let mut e = engine();
        let (this, node, _, queries) = collide_fixture(
            &mut e,
            vec![[0.0, 0.0, 2.0, 0.0], [0.0, 0.0, -2.0, 0.0]],
            0.2,
        );
        assert_eq!(e.call(0x00e2_2050, &args![this, node]).f32(), 1.0);
        assert_eq!(queries.borrow().len(), 1);
        assert_eq!(e.mem.u8(this + 0x10), 0);
    }

    #[test]
    fn hkgp_mesh_collide_shape_triangle_add_leaf_00e22050_replaces_a_very_negative_margin() {
        // -[this+4] = -2000 is below -1000: the float at 010959f4 (-5) is used instead, so a vertex
        // 10 behind the plane clears the shape; the original margin would not have.
        let mut e = engine();
        let (this, node, _, _) = collide_fixture(&mut e, vec![[0.0, 0.0, -10.0, 0.0]], 0.05);
        e.mem.set_f32(this + 4, 2000.0);
        assert_eq!(e.call(0x00e2_2050, &args![this, node]).f32(), 1.0);
    }

    /// Two prisms on vertices given by pointers; returns the prisms' leaf nodes.
    fn prism_nodes(e: &mut Engine, first: [u32; 3], second: [u32; 3]) -> (u32, u32, u32, u32) {
        let make = |e: &mut Engine, vertices: [u32; 3]| {
            let prism = e.mem.alloc(0x20);
            for (slot, vertex) in vertices.iter().enumerate() {
                e.mem.set_u32(prism + 8 + 4 * slot as u32, *vertex);
            }
            let node = e.mem.alloc(0x30);
            e.mem.set_u32(node + 0x20, prism);
            (node, prism)
        };
        let (node_a, prism_a) = make(e, first);
        let (node_b, prism_b) = make(e, second);
        (node_a, prism_a, node_b, prism_b)
    }

    #[test]
    fn hkgp_mesh_prism_policy_add_hit_00e22580_returns_true_for_prisms_sharing_a_vertex() {
        let mut e = engine();
        let policy = e.mem.alloc(0x20);
        let (node_a, _, node_b, _) =
            prism_nodes(&mut e, [0x100, 0x200, 0x300], [0x400, 0x300, 0x500]);
        assert!(e.call(0x00e2_2580, &args![policy, node_a, node_b]).bool());
        assert_eq!(e.mem.u32(policy + 4), 1);
        assert_eq!(e.mem.u32(policy + 8), 0);
    }

    #[test]
    fn hkgp_mesh_prism_policy_add_hit_00e22580_asks_the_overlap_check_for_other_prisms() {
        let mut e = engine();
        let vertices: Vec<u32> = (0..6).map(|_| e.mem.alloc(0x50)).collect();
        let policy = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, 0x0900_0500);
        let checker = e.mem.alloc(0x10);
        e.mem.set_u32(checker, vtable);
        e.mem.set_u32(policy + 0xc, checker);
        let answers = Rc::new(RefCell::new(vec![0u32, 1]));
        let list = Rc::clone(&answers);
        let seen: Rc<RefCell<Vec<(Vec<u32>, u32, u32, u32, u32)>>> =
            Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x0900_0500, move |e, a| {
            log.borrow_mut().push((
                a.to_vec(),
                e.mem.u32(a[1]),
                e.mem.u32(a[1] + 4),
                e.mem.u32(a[2]),
                e.mem.u32(a[2] + 4),
            ));
            Ret {
                eax: list.borrow_mut().remove(0),
                ..Ret::default()
            }
        });
        let (node_a, prism_a, node_b, prism_b) = prism_nodes(
            &mut e,
            [vertices[0], vertices[1], vertices[2]],
            [vertices[3], vertices[4], vertices[5]],
        );
        for vertex in &vertices {
            e.mem.set_f32(vertex + 0x3c, 7.0);
        }
        // No overlap reported: the vertices are left alone.
        assert!(!e.call(0x00e2_2580, &args![policy, node_a, node_b]).bool());
        assert_eq!(e.mem.f32(vertices[0] + 0x3c), 7.0);
        assert_eq!(e.mem.u32(policy + 4), 1);
        assert_eq!(e.mem.u32(policy + 8), 1);
        // Overlap reported: the float at +0x3c of all six vertices becomes 0.0.
        assert!(!e.call(0x00e2_2580, &args![policy, node_a, node_b]).bool());
        for vertex in &vertices {
            assert_eq!(e.mem.f32(vertex + 0x3c), 0.0);
        }
        let seen = seen.borrow();
        assert_eq!(seen[0].0[0], checker);
        assert_eq!(seen[0].0[3], 0);
        assert_eq!((seen[0].1, seen[0].2), (0x010e_aea4, prism_a));
        assert_eq!((seen[0].3, seen[0].4), (0x010e_aea4, prism_b));
    }

    #[test]
    fn mesh_append_from_convex_hull_00e1d900_appends_the_hulls_geometry() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let seen: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00d7_6410, move |e, a| {
            log.borrow_mut()
                .push((0..6).map(|word| e.mem.u32(a[2] + 4 * word)).collect());
            Ret::default()
        });
        e.register(0x00e1_ce10, |_, _| Ret::default());
        e.register(0x00cc_e530, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        let hull = e.mem.alloc(0x20);
        e.call(0x00e1_d900, &args![mesh, hull]);
        assert_eq!(seen.borrow()[0], vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
        let first = &calls(&e, 0x00d7_6410)[0];
        assert_eq!((first[0], first[1]), (hull, 0));
        let append = &calls(&e, 0x00e1_ce10)[0];
        assert_eq!(
            (append[0], append[2], append[3]),
            (mesh, 0x0126_8370, 0xffff_ffff)
        );
        assert_eq!(calls(&e, 0x00cc_e530)[0][0], append[1]);
        assert_eq!(first[2], append[1]);
    }

    /// Doubles for the distance-sum loops: `00e13210` (the locate query) gives the distance
    /// `point.z` and logs the points; the CRT `sqrt` is real. Returns the log of points.
    fn install_locate_by_height(e: &mut Engine) -> Rc<RefCell<Vec<[f32; 4]>>> {
        let points: Rc<RefCell<Vec<[f32; 4]>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&points);
        e.register_double(0x00e1_3210, move |e, a| {
            let point = vec_at(e, a[1]);
            log.borrow_mut().push(point);
            e.mem.set_f32(a[2] + 0x40, point[2]);
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32)).sqrt(),
            ..Ret::default()
        });
        e.mem.set_f32(0x0104_a494, 0.5);
        e.mem.set_f32(0x0106_c2e0, 0.5);
        points
    }

    /// The float sum the exe builds: each root rounded to a float, added to the float sum in
    /// double precision and rounded again.
    fn x87_sum(terms: &[f64]) -> f32 {
        let mut sum = 0.0f32;
        for term in terms {
            let root = term.sqrt() as f32;
            sum = (f64::from(root) + f64::from(sum)) as f32;
        }
        sum
    }

    #[test]
    fn fn_00e14160_sums_the_distances_of_vertices_then_triangles() {
        let mut e = engine();
        let points = install_locate_by_height(&mut e);
        let mesh = new_mesh(&mut e);
        let a = add_vertex(&mut e, mesh, [0.0, 0.0, 2.0, 1.0]);
        let b = add_vertex(&mut e, mesh, [0.0, 0.0, 4.0, 1.0]);
        add_triangle(&mut e, mesh, a, b, b);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        let got = e
            .call(0x00e1_4160, &args![mesh, direction, 1000.0f32])
            .f32();
        // Vertex b (z 4): point z = 4 * 0.5 + 4 = 6; vertex a: 3; the triangle's centroid z is
        // (4 + 2 + 4) * 0.5 = 5, so its point has z 7.5.
        assert_eq!(got, x87_sum(&[6.0, 3.0, 7.5]));
        let points = points.borrow();
        assert_eq!(points.len(), 3);
        assert_eq!(points[0][..3], [0.0, 0.0, 6.0]);
        assert_eq!(points[2][..3], [0.0, 0.0, 7.5]);
    }

    #[test]
    fn fn_00e14160_stops_the_vertex_loop_at_the_limit_but_still_adds_one_triangle() {
        let mut e = engine();
        let points = install_locate_by_height(&mut e);
        let mesh = new_mesh(&mut e);
        let a = add_vertex(&mut e, mesh, [0.0, 0.0, 2.0, 1.0]);
        let b = add_vertex(&mut e, mesh, [0.0, 0.0, 4.0, 1.0]);
        add_triangle(&mut e, mesh, a, b, b);
        add_triangle(&mut e, mesh, a, b, b);
        let direction = e.mem.alloc(0x10);
        put_vec(&mut e, direction, [0.0, 0.0, 1.0, 0.0]);
        // The first vertex term is sqrt(6); a limit equal to it already stops the loops.
        let limit = 6.0f64.sqrt() as f32;
        let got = e.call(0x00e1_4160, &args![mesh, direction, limit]).f32();
        assert_eq!(got, x87_sum(&[6.0, 7.5]));
        assert_eq!(points.borrow().len(), 2);
        // An unordered limit never stops anything.
        points.borrow_mut().clear();
        e.call(0x00e1_4160, &args![mesh, direction, f32::NAN]);
        assert_eq!(points.borrow().len(), 4);
        // A mesh without vertices or triangles gives 0.
        let empty = new_mesh(&mut e);
        assert_eq!(
            e.call(0x00e1_4160, &args![empty, direction, 1.0f32]).f32(),
            0.0
        );
    }

    /// The doubles `normalize` needs: translation by the fourth column of the transform.
    fn install_translation_only(e: &mut Engine) {
        e.register(SET_TRANSFORMED_POS, |e, a| {
            let position = vec_at(e, a[2]);
            let shift = vec_at(e, a[1] + 0x30);
            put_vec(
                e,
                a[0],
                [
                    position[0] + shift[0],
                    position[1] + shift[1],
                    position[2] + shift[2],
                    position[3],
                ],
            );
            Ret::default()
        });
        e.mem.set_f32(0x0101_1d78, 0.0);
        e.mem.set_f32(0x0101_2068, 1.0);
        e.mem.set_f32(0x0102_295c, -0.5);
        e.mem.set_f32(0x010c_7170 + 12, 1.0);
        e.mem.set_f32(0x0101_2054, -1.0);
        // The identity transform's words are not needed: the translation column is replaced.
    }

    #[test]
    fn mesh_normalize_00e13fa0_centres_and_scales_the_mesh() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_translation_only(&mut e);
        let mesh = new_mesh(&mut e);
        let high = add_vertex(&mut e, mesh, [3.0, 2.0, 1.0, 1.0]);
        let low = add_vertex(&mut e, mesh, [-1.0, -2.0, -3.0, 1.0]);
        let hull = install_hull(&mut e, mesh);
        let centre = e.mem.alloc(0x10);
        let extent = e.mem.alloc(0x10);
        let got = e
            .call(0x00e1_3fa0, &args![mesh, centre, extent, 2.0f32])
            .f32();
        // The box is (-1,-2,-3) to (3,2,1): centre (1, 0, -1), extent 4 in every lane.
        assert_eq!(got, 2.0);
        assert_eq!(vec_at(&e, extent)[..3], [2.0, 2.0, 2.0]);
        assert_eq!(vec_at(&e, centre)[..3], [1.0, 0.0, -1.0]);
        let close = |got: f32, want: f32| assert!((got - want).abs() < 1.0e-6, "{got} != {want}");
        for (vertex, want) in [(high, [1.0f32, 1.0, 1.0]), (low, [-1.0, -1.0, -1.0])] {
            let position = vec_at(&e, vertex + 0x10);
            for lane in 0..3 {
                close(position[lane], want[lane]);
            }
        }
        // The hull was deleted by the translation and the mesh (3 or fewer vertices) has none now.
        assert_eq!(calls(&e, 0x0900_0020)[0], vec![hull, 1]);
        assert_eq!(e.mem.u32(mesh + 0xe4), 0);
    }

    #[test]
    fn mesh_normalize_00e13fa0_uses_the_largest_of_the_three_extents() {
        let mut e = engine();
        install_translation_only(&mut e);
        let centre = e.mem.alloc(0x10);
        let extent = e.mem.alloc(0x10);
        for (sizes, largest) in [
            ([1.0f32, 5.0, 3.0], 5.0f32),
            ([6.0, 5.0, 7.0], 7.0),
            ([8.0, 5.0, 7.0], 8.0),
        ] {
            let mesh = new_mesh(&mut e);
            add_vertex(&mut e, mesh, [0.0, 0.0, 0.0, 1.0]);
            add_vertex(&mut e, mesh, [sizes[0], sizes[1], sizes[2], 1.0]);
            let got = e
                .call(0x00e1_3fa0, &args![mesh, centre, extent, 1.0f32])
                .f32();
            assert_eq!(got, largest);
        }
    }

    #[test]
    fn mesh_normalize_00e13fa0_leaves_an_empty_mesh_alone() {
        let mut e = engine();
        install_translation_only(&mut e);
        let mesh = new_mesh(&mut e);
        let centre = e.mem.alloc(0x10);
        let extent = e.mem.alloc(0x10);
        put_vec(&mut e, centre, [9.0; 4]);
        let got = e
            .call(0x00e1_3fa0, &args![mesh, centre, extent, 3.0f32])
            .f32();
        assert_eq!(got, 1.0);
        assert_eq!(vec_at(&e, centre), [0.0; 4]);
        assert_eq!(vec_at(&e, extent), [1.0; 4]);
    }

    #[test]
    fn get_loop_surface_area_00e153f0_sums_the_extent_of_the_edge_positions() {
        let mut e = engine();
        let log = install_heap(&mut e);
        install_calc_aabb(&mut e);
        let mesh = new_mesh(&mut e);
        let a = add_vertex(&mut e, mesh, [0.0, 0.0, 0.0, 1.0]);
        let b = add_vertex(&mut e, mesh, [2.0, 0.0, 0.0, 1.0]);
        let c = add_vertex(&mut e, mesh, [0.0, 3.0, 0.0, 1.0]);
        let triangle = add_triangle(&mut e, mesh, a, b, c);
        let edges = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x20);
        e.mem.set_u32(edges, data);
        e.mem.set_u32(edges + 4, 2);
        e.mem.set_u32(edges + 8, 2);
        for (index, (owner, edge)) in [(triangle, 0u32), (triangle, 1)].into_iter().enumerate() {
            e.mem.set_u32(data + 8 * index as u32, owner);
            e.mem.set_u32(data + 8 * index as u32 + 4, edge);
        }
        e.call_log = Some(vec![]);
        let got = e.call(0x00e1_53f0, &args![edges]).f32();
        // Edge 0 runs a to b, edge 1 runs b to c: the box is (0,0,0) to (2,3,0), so 0 + (3 + 2).
        assert_eq!(got, 5.0);
        // The local array was reserved for four vectors and its storage freed (64 bytes).
        assert_eq!(calls(&e, ARRAY_RESERVE)[0][2..], [4, 0x10]);
        let frees = log.borrow().clone();
        assert_eq!(
            frees.last().map(|entry| (entry.0, entry.2)),
            Some((0x10, 64))
        );
        // An empty loop gives 0.0.
        e.mem.set_u32(edges + 4, 0);
        assert_eq!(e.call(0x00e1_53f0, &args![edges]).f32(), 0.0);
    }

    #[test]
    fn fn_00e150e0_flips_the_triangles_of_a_part() {
        let mut e = engine();
        let log = install_heap(&mut e);
        e.mem.set_f32(0x0101_2054, -1.0);
        let seen: Rc<RefCell<Vec<(u32, Vec<u32>, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen);
        e.register_double(0x00e1_24d0, move |e, a| {
            let array = a[1];
            let data = e.mem.u32(array);
            let words: Vec<u32> = (0..6).map(|word| e.mem.u32(data + 4 * word)).collect();
            record
                .borrow_mut()
                .push((e.mem.u32(array + 4), words, e.mem.u32(a[0] + 0x101), 0));
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let triangle_b = add_triangle(&mut e, mesh, 0x11, 0x22, 0x33);
        let triangle_a = add_triangle(&mut e, mesh, 0x44, 0x55, 0x66);
        e.mem.set_u32(triangle_a + 0x30, 5);
        e.mem.set_u32(triangle_b + 0x30, 9);
        put_vec(&mut e, triangle_a + 0x20, [1.0, 2.0, 3.0, 4.0]);
        put_vec(&mut e, triangle_b + 0x20, [7.0, 7.0, 7.0, 7.0]);
        e.mem.set_u32(triangle_a + 0x14, triangle_b | 1);
        e.mem.set_u32(triangle_a + 0x1c, triangle_b | 2);
        e.mem.set_u32(triangle_b + 0x18, triangle_a);
        e.mem.set_u32(triangle_b + 0x1c, triangle_a | 2);
        e.mem.set_u8(mesh + 0x101, 1);
        e.call(0x00e1_50e0, &args![mesh, 5u32]);
        assert_eq!(vec_at(&e, triangle_a + 0x20), [-1.0, -2.0, -3.0, -4.0]);
        assert_eq!(vec_at(&e, triangle_b + 0x20), [7.0; 4]);
        assert_eq!(e.mem.u32(triangle_a + 8), 0x55);
        assert_eq!(e.mem.u32(triangle_a + 0xc), 0x44);
        assert_eq!(e.mem.u32(triangle_b + 8), 0x11);
        for slot in 0..3 {
            assert_eq!(e.mem.u32(triangle_a + 0x14 + 4 * slot), 0);
        }
        // The neighbour's back links named by the low bits (slot 1 and slot 2) are cleared.
        assert_eq!(e.mem.u32(triangle_b + 0x18), 0);
        assert_eq!(e.mem.u32(triangle_b + 0x1c), 0);
        let seen = seen.borrow();
        assert_eq!(seen[0].0, 3);
        assert_eq!(seen[0].1, vec![triangle_a, 0, triangle_a, 1, triangle_a, 2]);
        assert_eq!(seen[0].2, 1);
        // The dirty byte is cleared after the call, and the array storage (3 pairs, capacity 4) is freed.
        assert_eq!(e.mem.u8(mesh + 0x101), 0);
        assert_eq!(
            log.borrow().last().map(|entry| (entry.0, entry.2)),
            Some((0x10, 32))
        );
    }

    #[test]
    fn mesh_enumerate_triangle_overlaps_00e15220_collects_the_triangles_that_overlap() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let log = install_heap(&mut e);
        install_calc_aabb(&mut e);
        let mesh = new_mesh(&mut e);
        let leaves: Vec<u32> = [1.0f32, 2.0, 3.0]
            .iter()
            .map(|x| {
                let a = add_vertex(&mut e, mesh, [*x, 0.0, 0.0, 1.0]);
                let b = add_vertex(&mut e, mesh, [*x, 1.0, 0.0, 1.0]);
                let c = add_vertex(&mut e, mesh, [*x, 0.0, 1.0, 1.0]);
                add_triangle(&mut e, mesh, a, b, c)
            })
            .collect();
        // The tree query hands all three triangles to the collector.
        let all = leaves.clone();
        let storage = Rc::new(RefCell::new(0u32));
        let kept = Rc::clone(&storage);
        e.register_double(QUERY_AABB, move |e, a| {
            let collector = a[2];
            let data = e.mem.alloc(16);
            for (index, leaf) in all.iter().enumerate() {
                e.mem.set_u32(data + 4 * index as u32, *leaf);
            }
            e.mem.set_u32(collector + 4, data);
            e.mem.set_u32(collector + 8, 3);
            e.mem.set_u32(collector + 0xc, 4);
            *kept.borrow_mut() = data;
            Ret::default()
        });
        // The overlap test says yes for the triangles at x = 1 and x = 3.
        e.register(0x00de_ab10, |e, a| Ret {
            eax: u32::from(e.mem.f32(a[1]) != 2.0),
            ..Ret::default()
        });
        let query = e.mem.alloc(0x30);
        put_vec(&mut e, query, [0.0, 0.0, 0.0, 1.0]);
        put_vec(&mut e, query + 0x10, [0.0, 1.0, 0.0, 1.0]);
        put_vec(&mut e, query + 0x20, [0.0, 0.0, 1.0, 1.0]);
        let out = new_array(&mut e);
        e.call(
            0x00e1_5220,
            &args![mesh, query, query + 0x10, query + 0x20, out],
        );
        assert_eq!(e.mem.u32(out + 4), 2);
        let data = e.mem.u32(out);
        assert_eq!(e.mem.u32(data), leaves[0]);
        assert_eq!(e.mem.u32(data + 4), leaves[2]);
        let query_call = &calls(&e, QUERY_AABB)[0];
        assert_eq!(query_call[0], mesh + 0x40);
        // The collector's storage (capacity 4 words) was freed.
        assert!(log.borrow().contains(&(0x10, *storage.borrow(), 16)));
        assert_eq!(calls(&e, 0x00de_ab10).len(), 3);
    }

    #[test]
    fn fn_00e191a0_splits_an_edge_at_a_new_vertex() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        e.register(0x00e1_8ed0, |_, _| Ret::default());
        put_vec(&mut e, 0x0126_7e30, [1.0, 2.0, 3.0, 4.0]);
        let mesh = new_mesh(&mut e);
        let out = e.mem.alloc(8);
        let position = e.mem.alloc(0x10);
        put_vec(&mut e, position, [7.0, 8.0, 9.0, 10.0]);
        let got = e
            .call(
                0x00e1_91a0,
                &args![mesh, out, 0x111u32, 0x222u32, position, 1u32],
            )
            .u32();
        assert_eq!(got, out);
        let vertex = e.mem.u32(mesh + 0x14);
        assert_eq!(vec_at(&e, vertex + 0x10), [7.0, 8.0, 9.0, 10.0]);
        assert_eq!(vec_at(&e, vertex + 0x30), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(vertex + 0x40), 0xffff_ffff);
        assert_eq!(e.mem.u32(vertex + 0x44), 0xffff_ffff);
        assert_eq!(
            calls(&e, 0x00e1_8ed0),
            vec![vec![mesh, out, 0x111, 0x222, vertex, 1]]
        );
    }

    #[test]
    fn fn_00e19200_splits_a_triangle_into_three() {
        let mut e = engine();
        install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        let source = e.mem.alloc(0x50);
        let triangle = e.call(0x00e2_41a0, &args![mesh + 0x20, source]).u32();
        let before = e.mem.u32(triangle + 0x14);
        assert_eq!(before, 0);
        let (north, west) = (e.mem.alloc(0x50), e.mem.alloc(0x50));
        // Link 0 is the neighbour `north` (its slot 1), link 2 the neighbour `west` (its slot 2).
        e.mem.set_u32(triangle + 0x14, north | 1);
        e.mem.set_u32(triangle + 0x1c, west | 2);
        let position = e.mem.alloc(0x10);
        put_vec(&mut e, position, [1.0, 2.0, 3.0, 4.0]);
        put_vec(&mut e, 0x0126_7e30, [5.0; 4]);
        let second = e
            .call(0x00e1_9200, &args![mesh, triangle, position, 0u32])
            .u32();
        let vertex = e.mem.u32(mesh + 0x14);
        assert_eq!(vec_at(&e, vertex + 0x10), [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(vec_at(&e, vertex + 0x30), [5.0; 4]);
        // The list runs newest first: second copy, first copy, original.
        let first = e.mem.u32(second);
        assert_eq!(e.mem.u32(first), triangle);
        assert_eq!(e.mem.u32(triangle + 8), vertex);
        assert_eq!(e.mem.u32(first + 0xc), vertex);
        assert_eq!(e.mem.u32(second + 0x10), vertex);
        assert_eq!(e.mem.u32(first + 0x1c), west | 2);
        assert_eq!(e.mem.u32(west + 0x14 + 8), first | 2);
        assert_eq!(e.mem.u32(second + 0x14), north | 1);
        assert_eq!(e.mem.u32(north + 0x14 + 4), second);
        assert_eq!(e.mem.u32(triangle + 0x14), second | 1);
        assert_eq!(e.mem.u32(second + 0x18), triangle);
        assert_eq!(e.mem.u32(first + 0x14), second | 2);
        assert_eq!(e.mem.u32(second + 0x1c), first);
        assert_eq!(e.mem.u32(first + 0x18), triangle | 2);
        assert_eq!(e.mem.u32(triangle + 0x1c), first | 1);
    }

    #[test]
    fn fn_00e19200_without_neighbours_only_links_the_three_triangles() {
        let mut e = engine();
        install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        let source = e.mem.alloc(0x50);
        let triangle = e.call(0x00e2_41a0, &args![mesh + 0x20, source]).u32();
        let position = e.mem.alloc(0x10);
        let second = e
            .call(0x00e1_9200, &args![mesh, triangle, position, 0u32])
            .u32();
        let first = e.mem.u32(second);
        assert_eq!(e.mem.u32(first + 0x1c), 0);
        assert_eq!(e.mem.u32(second + 0x14), 0);
        assert_eq!(e.mem.u32(triangle + 0x14), second | 1);
        assert_eq!(e.mem.u32(triangle + 0x1c), first | 1);
    }

    #[test]
    fn fn_00e19cf0_adds_a_triangle_with_its_plane() {
        let mut e = engine();
        install_heap(&mut e);
        let mesh = new_mesh(&mut e);
        e.mem.set_f32(mesh + 0x104, 1.0e-6);
        e.mem.set_f32(mesh + 0x108, 1.0e-3);
        let a = add_vertex(&mut e, mesh, [0.0, 0.0, 0.0, 0.0]);
        let b = add_vertex(&mut e, mesh, [1.0, 0.0, 0.0, 0.0]);
        let c = add_vertex(&mut e, mesh, [0.0, 1.0, 0.0, 0.0]);
        let triangle = e.call(0x00e1_9cf0, &args![mesh, a, b, c]).u32();
        assert_eq!(e.mem.u32(mesh + 0x2c), triangle);
        assert_eq!(e.mem.u32(mesh + 0x30), 1);
        assert_eq!(e.mem.u32(triangle + 0x30), 0xffff_ffff);
        for slot in 0..3 {
            assert_eq!(e.mem.u32(triangle + 0x14 + 4 * slot), 0);
        }
        assert_eq!(
            (
                e.mem.u32(triangle + 8),
                e.mem.u32(triangle + 0xc),
                e.mem.u32(triangle + 0x10)
            ),
            (a, b, c)
        );
        assert_eq!(vec_at(&e, triangle + 0x20), [0.0, 0.0, 1.0, 0.0]);
    }

    /// An `hkArray` of words with the given words, owning exact storage.
    fn words_array(e: &mut Engine, words: &[u32]) -> u32 {
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(4 * words.len().max(1) as u32);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(storage + 4 * index as u32, *word);
        }
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, words.len() as u32);
        e.mem.set_u32(array + 8, words.len() as u32);
        array
    }

    fn words_of(e: &Engine, array: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|index| e.mem.u32(data + 4 * index))
            .collect()
    }

    #[test]
    fn fn_00e23a60_copies_an_array_of_words_growing_the_destination() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let source = words_array(&mut e, &[0xa, 0xb, 0xc]);
        let destination = e.mem.alloc(0x10);
        let got = e.call(0x00e2_3a60, &args![destination, source]).u32();
        assert_eq!(got, destination);
        assert_eq!(words_of(&e, destination), vec![0xa, 0xb, 0xc]);
        assert_eq!(e.mem.u32(destination + 8), 3);
        assert_eq!(*log.borrow(), vec![(0x10, 0, 0), (0x0c, 12, 0)]);
    }

    #[test]
    fn fn_00e23a60_frees_owned_storage_and_uses_the_granted_size() {
        let mut e = engine();
        let log = install_heap(&mut e);
        // The allocator rounds requests up to 16 bytes and says so.
        e.register_double(0x0900_000c, |e, a| {
            let granted = (e.mem.u32(a[1]) + 15) & !15;
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: e.mem.alloc(granted),
                ..Ret::default()
            }
        });
        let source = words_array(&mut e, &[1, 2, 3]);
        let destination = words_array(&mut e, &[9, 9]);
        let old = e.mem.u32(destination);
        e.call(0x00e2_3a60, &args![destination, source]);
        assert_eq!(*log.borrow(), vec![(0x10, old, 8)]);
        assert_eq!(e.mem.u32(destination + 8), 4);
        assert_eq!(words_of(&e, destination), vec![1, 2, 3]);
    }

    #[test]
    fn fn_00e23a60_keeps_storage_that_is_big_enough_or_not_owned() {
        let mut e = engine();
        let log = install_heap(&mut e);
        let source = words_array(&mut e, &[7, 8]);
        let destination = words_array(&mut e, &[1, 2, 3, 4]);
        e.call(0x00e2_3a60, &args![destination, source]);
        assert!(log.borrow().is_empty());
        assert_eq!(words_of(&e, destination), vec![7, 8]);
        // Not owned and too small: nothing is freed, a block is asked for.
        let borrowed = e.mem.alloc(0x10);
        e.mem.set_u32(borrowed, 0x1234);
        e.mem.set_u32(borrowed + 8, 0x8000_0001);
        e.call(0x00e2_3a60, &args![borrowed, source]);
        assert_eq!(*log.borrow(), vec![(0x0c, 8, 0)]);
        assert_eq!(words_of(&e, borrowed), vec![7, 8]);
        assert_eq!(e.mem.u32(borrowed + 8), 2);
    }

    /// A mesh with an edge table of one bucket (all keys share it) holding `entries`, each
    /// (start vertex, end vertex, value).
    fn edge_table(e: &mut Engine, mesh: u32, entries: &[(u32, u32, u32)]) {
        let table = e.mem.alloc(0x10);
        let bucket = e.mem.alloc(12);
        let storage = e.mem.alloc(12 * entries.len().max(1) as u32);
        for (index, (start, end, value)) in entries.iter().enumerate() {
            let at = storage + 12 * index as u32;
            e.mem.set_u32(at, *start);
            e.mem.set_u32(at + 4, *end);
            e.mem.set_u32(at + 8, *value);
        }
        e.mem.set_u32(bucket, storage);
        e.mem.set_u32(bucket + 4, entries.len() as u32);
        e.mem.set_u32(table, bucket);
        e.mem.set_u32(table + 4, 1);
        e.mem.set_u32(mesh + 0x24c, table);
    }

    #[test]
    fn fn_00e27ae0_reads_the_table_values_of_an_edge_and_its_neighbour() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        let seen: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00e2_7450, move |e, a| {
            log.borrow_mut()
                .push((a[0], e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let vertices: Vec<u32> = (0..6).map(|_| e.mem.alloc(0x50)).collect();
        let first = add_triangle(&mut e, mesh, vertices[0], vertices[1], vertices[2]);
        let second = add_triangle(&mut e, mesh, vertices[3], vertices[4], vertices[5]);
        // Edge 0 of the first triangle borders edge 1 of the second.
        e.mem.set_u32(first + 0x14, second | 1);
        let ours = words_array(&mut e, &[10, 20]);
        let theirs = words_array(&mut e, &[30]);
        edge_table(
            &mut e,
            mesh,
            &[
                (vertices[0], vertices[1], ours),
                (vertices[4], vertices[5], theirs),
            ],
        );
        let out = e.mem.alloc(0x40);
        e.mem.set_u8(out, 9);
        e.mem.set_u8(out + 0x20, 9);
        e.call(0x00e2_7ae0, &args![mesh, first, 0u32, out]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(words_of(&e, out + 4), vec![10, 20]);
        assert_eq!(e.mem.u8(out + 0x20), 1);
        assert_eq!(words_of(&e, out + 0x24), vec![30]);
        assert_eq!(*seen.borrow(), vec![(mesh, first, 0), (mesh, second, 1)]);
    }

    #[test]
    fn fn_00e27ae0_stops_at_an_unset_neighbour_and_skips_missing_values() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        install_heap(&mut e);
        e.register(0x00e2_7450, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        let vertices: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x50)).collect();
        let triangle = add_triangle(&mut e, mesh, vertices[0], vertices[1], vertices[2]);
        edge_table(&mut e, mesh, &[]);
        let out = e.mem.alloc(0x40);
        e.mem.set_u8(out + 0x20, 5);
        e.call(0x00e2_7ae0, &args![mesh, triangle, 0u32, out]);
        // No value, no neighbour: both flags are 0, the second record is not even visited.
        assert_eq!(e.mem.u8(out), 0);
        assert_eq!(e.mem.u8(out + 0x20), 0);
        assert!(calls(&e, 0x00e2_7450).is_empty());
    }

    #[test]
    fn fn_00e25fb0_runs_the_apply_step_for_each_flagged_record() {
        let mut e = engine();
        let seen: Rc<RefCell<Vec<(Vec<u32>, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(0x00e2_5cd0, move |e, a| {
            log.borrow_mut()
                .push((a.to_vec(), e.mem.u32(a[1]), e.mem.u32(a[1] + 4)));
            Ret::default()
        });
        let mesh = new_mesh(&mut e);
        let first = add_triangle(&mut e, mesh, 0x100, 0x200, 0x300);
        let second = add_triangle(&mut e, mesh, 0x400, 0x500, 0x600);
        e.mem.set_u32(first + 0x1c, second | 2);
        let records = e.mem.alloc(0x40);
        e.mem.set_u8(records, 1);
        e.mem.set_u8(records + 0x20, 1);
        e.call(0x00e2_5fb0, &args![mesh, first, 2u32, records]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 2);
        assert_eq!((seen[0].0[0], seen[0].0[2]), (mesh, records + 4));
        assert_eq!((seen[0].1, seen[0].2), (first, 2));
        assert_eq!(
            (seen[1].0[2], seen[1].1, seen[1].2),
            (records + 0x24, second, 2)
        );
    }

    #[test]
    fn fn_00e25fb0_skips_unflagged_records_and_stops_at_an_unset_neighbour() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.register(0x00e2_5cd0, |_, _| Ret::default());
        let mesh = new_mesh(&mut e);
        let first = add_triangle(&mut e, mesh, 0x100, 0x200, 0x300);
        let second = add_triangle(&mut e, mesh, 0x400, 0x500, 0x600);
        e.mem.set_u32(first + 0x14, second | 1);
        let records = e.mem.alloc(0x40);
        e.mem.set_u8(records + 0x20, 1);
        // The first record is not flagged; the second triangle has no neighbour on edge 1.
        e.call(0x00e2_5fb0, &args![mesh, first, 0u32, records]);
        let logged = calls(&e, 0x00e2_5cd0);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][2], records + 0x24);
    }
}
