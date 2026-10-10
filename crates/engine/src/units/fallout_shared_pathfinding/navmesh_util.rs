//! `fallout shared/pathfinding/navmesh_util.cpp` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds the navmesh's spatial helpers: the search grid that lists,
//! per cell, the triangles touching it, the point-to-triangle queries that
//! use it, edge/segment intersection tests, and a few small value-type
//! helpers (`NiPoint2`, a two-point segment, a path request's copy
//! operators). Nearly all of them are methods of `NavMesh`
//! ([`crate::units::fallout_shared_pathfinding::navmesh::NavMesh`], `this`
//! is the mesh) or of its embedded search grid at `NavMesh + 0xA8`
//! ([`NavMeshGrid`], `this` is the grid).
//!
//! Translated so far (address order): everything up to and including
//! `00698620` (40 functions); the next session continues at `00698680`.
//!
//! Conventions in this file:
//! - Methods the engine map leaves unnamed are `fn_<address>`, named by what
//!   the body does in their doc comment.
//! - `006815c0` is the `NiPoint3` default constructor: its body only returns
//!   `this`, and it is called on stack locals that are written before they
//!   are read, so those calls (and the `_vector_constructor_iterator_`
//!   (`00401050`) over such locals) are left out. The compiler's
//!   exception-unwinding frames and stack-cookie checks are not translated.
//! - `float` stores round to `f32`; the x87 intermediate results are
//!   computed in `f64` (see the guide).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared_pathfinding::navmesh::{NavMesh, NavMeshGrid, NavMeshTriangle};

// ---------------------------------------------------------------------------
// Layouts and constants
// ---------------------------------------------------------------------------

/// Offset of the search grid (`MeshGrid`, a [`NavMeshGrid`]) inside a
/// `NavMesh`.
pub(crate) const MESH_GRID_OFFSET: u32 = 0xa8;
/// Offset of the `Triangles` array inside a `NavMesh`.
const MESH_TRIANGLES_OFFSET: u32 = 0x38;
/// Offset of the `ExtraEdgeInfo` array inside a `NavMesh`.
const MESH_EXTRA_EDGE_INFO_OFFSET: u32 = 0x48;
/// `NavMeshInfo::AdjacentNavMeshes` and `PreferredAdjacentNavMeshes`
/// (`BSSimpleArray<NavMeshInfo*>`, Xbox PDB) inside a `NavMeshInfo`.
const INFO_ADJACENT_NAV_MESHES: u32 = 0x24;
const INFO_PREFERRED_ADJACENT_NAV_MESHES: u32 = 0x34;
/// `EdgeExtraInfo::Portal.pOtherMesh` (Xbox PDB), the `NavMeshInfo*` of the
/// mesh on the other side of a portal edge.
const EDGE_INFO_OTHER_MESH: u32 = 0x04;

/// The `TES` instance pointer (`TES::GetNavMeshInfoMap` is called on it).
const TES_INSTANCE: u32 = 0x011d_ea10;
/// Type descriptors `__RTDynamicCast` casts between (`.?AVTESForm@@`,
/// `.?AVNavMesh@@`).
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_NAV_MESH: u32 = 0x0118_b6b4;
/// Virtual slot of `NavMesh` (byte offset) called with `1` before the extra
/// edge infos of a loaded mesh are repaired.
const NAV_MESH_SLOT_0XC8: u32 = 0xc8;

// Data the code reads from the exe.
/// `FLT_MAX` (`float`) and its negation.
const FLOAT_MAX: u32 = 0x0101_6970;
const NEGATIVE_FLOAT_MAX: u32 = 0x0101_5f5c;
/// `180.0f`, `64.0f` (used as the up and down tolerance of the triangle
/// height tests) and `0.5f`.
const HEIGHT_UP: u32 = 0x0106_b158;
const HEIGHT_DOWN: u32 = 0x0101_e6ec;
const HALF: u32 = 0x0101_6248;
/// `double` constants: `0.0`, `50.0`, `-180.0`, `64.0`, `40.0`, `180.0`.
const ZERO: u32 = 0x0101_2060;
const GROUND_MARGIN: u32 = 0x0101_e2c0;
const NEGATIVE_180: u32 = 0x0106_b160;
const SIXTY_FOUR: u32 = 0x0102_40c0;
const FORTY: u32 = 0x0103_5810;
const ONE_EIGHTY: u32 = 0x0104_ed58;
/// `0.5` (`double`), the numerator of the circumcenter's reciprocal.
const DOUBLE_HALF: u32 = 0x0101_1588;
/// Three `float`s: the vector the random-point search starts from (its `z`
/// is kept unchanged).
const RANDOM_POINT_BASE: u32 = 0x011f_426c;

// Functions of other units this one calls.
/// `BSSimpleArray::iSize` (`this`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// Address of element `index` of a `BSSimpleArray` of 4-byte elements.
const ARRAY_ELEMENT_4: u32 = 0x0087_7a30;
/// Address of element `index` of a `BSSimpleArray<unsigned short>`.
const ARRAY_ELEMENT_2: u32 = 0x0069_dec0;
/// `BSSimpleArray` emptiness test (`iSize == 0`).
const ARRAY_IS_EMPTY: u32 = 0x0076_b610;
/// Address of triangle `index` of the `Triangles` array (`this` is the array).
const TRIANGLES_ELEMENT: u32 = 0x0055_8dd0;
/// `BSSimpleArray::Remove(index, count)` (`this`, index, count).
const ARRAY_REMOVE: u32 = 0x006b_f8f0;
/// Resets the array to the given size (`this`, size, keep buffer) and
/// reserves room (`this`, count).
const ARRAY_SET_SIZE: u32 = 0x006f_2290;
const ARRAY_SET_RESERVED_SIZE: u32 = 0x0084_b5c0;
/// Appends a `u16` (read through the pointer) to a `BSSimpleArray<unsigned
/// short>`.
const ARRAY_ADD_WORD: u32 = 0x0069_1820;
/// Appends a `u32` (read through the pointer) to a `BSSimpleArray<unsigned
/// int>`.
const ARRAY_ADD_DWORD: u32 = 0x007c_b2e0;
/// Index of the first element equal to the key (`this`, key pointer, start,
/// comparator) and the removal of `count` elements from `index`
/// (`this`, index, count) of a `BSSimpleArray<unsigned short>`.
const ARRAY_FIND_WORD: u32 = 0x0069_b240;
const WORD_COMPARATOR: u32 = 0x0069_b2a0;
const WORD_ARRAY_REMOVE: u32 = 0x0069_add0;
/// `NiPointer` helpers: default constructor, constructor from a pointer
/// (cdecl: out, pointer; returns out), assignment, destructor, the held
/// pointer (`NiPointer::operator*`) and the word at `this`.
const POINTER_CONSTRUCT: u32 = 0x0042_fb00;
const POINTER_CONSTRUCT_FROM_CDECL: u32 = 0x0046_4fc0;
const POINTER_ASSIGN: u32 = 0x0042_f4c0;
const POINTER_DESTRUCT: u32 = 0x0042_fa40;
const POINTER_DEREFERENCE: u32 = 0x0045_8b50;
const POINTER_GET: u32 = 0x0055_9450;
/// Looks a form up by its id (cdecl) and `__RTDynamicCast` (cdecl).
const LOOKUP_FORM: u32 = 0x0048_39c0;
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TES::GetNavMeshInfoMap`.
const GET_NAV_MESH_INFO_MAP: u32 = 0x0045_af00;
/// Constructor and destructor of the `BSSimpleArray<NavMeshInfo*>` (vtable
/// `0106b1a4`) that collects the infos, and the map method that fills it.
const INFO_LIST_CONSTRUCTOR: u32 = 0x0069_aee0;
const INFO_LIST_DESTRUCTOR: u32 = 0x0069_af10;
const INFO_MAP_COLLECT: u32 = 0x006b_7910;
/// `NavMeshInfo` method: assigns `pNavMesh` to the `NiPointer<NavMesh>`
/// argument, true when it is non-null (`navmesh.cpp`).
const INFO_GET_NAV_MESH: u32 = 0x0069_0860;
/// `NavMesh::GetExtraEdgeInfo(index)`: the `EdgeExtraInfo` at `index` of the
/// extra info array.
const GET_EXTRA_EDGE_INFO: u32 = 0x0069_3d30;
/// `NavMesh::DeleteExtraEdgeInfo` (`this`, index).
const DELETE_EXTRA_EDGE_INFO: u32 = 0x0069_0e90;
/// `NavMesh::GetEdgeExtraInfo` (`this`, triangle, edge): null without info.
const GET_EDGE_EXTRA_INFO: u32 = 0x0068_f230;
/// `NavMesh::GetEdgeVertices` (`this`, out pair of vertex pointers,
/// triangle, edge).
const GET_EDGE_VERTICES: u32 = 0x0068_f040;
/// Appends an `EdgeExtraInfo` to the mesh's array, returns its index.
const ADD_EXTRA_EDGE_INFO: u32 = 0x0069_1020;
/// `EdgeExtraInfo` constructor (`this`).
const EDGE_INFO_CONSTRUCTOR: u32 = 0x0069_23c0;
/// `NavMesh::GetTriangleCount`, `NavMesh::GetTriangle(index)`,
/// `NavMeshTriangle::GetVertex(i)` and `NavMesh::GetCenter(out, triangle)`.
const TRIANGLE_COUNT: u32 = 0x0055_8200;
const GET_TRIANGLE: u32 = 0x0055_82f0;
const TRIANGLE_VERTEX: u32 = 0x0055_82d0;
const GET_CENTER: u32 = 0x0055_8220;
/// Address of vertex `index` of the mesh.
const VERTEX_POINTER: u32 = 0x0068_f0a0;
/// `NavMeshTriangle` methods: whether edge `edge` has extra info, whether
/// its neighbour slot is not `0xFFFF`, the neighbour slot, whether any edge
/// has extra info, a flag mask test, and the setters of a vertex, of an
/// edge's extra info slot and of an edge's plain neighbour.
const EDGE_HAS_INFO: u32 = 0x0068_f1d0;
const EDGE_HAS_NEIGHBOUR: u32 = 0x0068_f200;
const EDGE_SLOT: u32 = 0x0068_f2c0;
const ANY_EDGE_HAS_INFO: u32 = 0x0069_0e10;
const TRIANGLE_FLAGS_SET: u32 = 0x0069_1140;
const SET_VERTEX: u32 = 0x0069_0df0;
const SET_EDGE_INFO_SLOT: u32 = 0x0069_0e30;
const SET_EDGE_NEIGHBOUR: u32 = 0x0069_07d0;
/// Unpacks the six edge bits the triangle flags hold for an edge (`this`,
/// edge, out value, out flag, out flag) and stores them again (`this`, edge,
/// value, flag, flag).
const UNPACK_EDGE_BITS: u32 = 0x0069_1040;
const PACK_EDGE_BITS: u32 = 0x006a_d2f0;
/// Search grid methods in other files: `Clear`, setup from the mesh bounds
/// and triangle count (`this`, min, max, count), the cell list that holds a
/// point (`this`, point, out list pointer), the corner point of a cell
/// (`this`, out, x, y), the cell number/row/column of a point (`this`,
/// point, out row, out column).
const GRID_CLEAR: u32 = 0x0069_d2d0;
const GRID_SETUP: u32 = 0x0069_d800;
const GRID_CELL_LIST: u32 = 0x0069_d370;
const GRID_CELL_CORNER: u32 = 0x0069_d790;
const GRID_CELL_OF_POINT: u32 = 0x0069_12a0;
/// Forwarded to by `00696c00` (`this`, three words).
const GRID_FORWARDED: u32 = 0x0069_d4b0;
/// `floor` of a `float` (cdecl, result in ST0) and `fabs` of a `float`.
const FLOAT_FLOOR: u32 = 0x0040_6ce0;
const FLOAT_ABS: u32 = 0x0040_8840;
/// `NiPoint2(x, y)` constructor (`this`, x, y) and the 2D segment test of
/// `006ba010` (cdecl, four `NiPoint2*`).
const POINT2_CONSTRUCT: u32 = 0x0045_2dc0;
const SEGMENTS_INTERSECT_2D: u32 = 0x006b_a010;
/// `NiPoint3(x, y, z)` constructor (`this`, x, y, z).
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiPoint3` arithmetic: `this + other`, `this - other` (both: `this`,
/// out, other), `this * scalar` (`this`, out, scalar), `this += other`,
/// `this == other`, `this x other` (`this`, out, other), normalize in
/// place, squared length and squared length of x and y (result in ST0).
const POINT3_ADD: u32 = 0x0043_9e90;
const POINT3_SUBTRACT: u32 = 0x0043_9ef0;
const POINT3_SCALE: u32 = 0x0045_bb20;
const POINT3_ADD_ASSIGN: u32 = 0x0063_c8a0;
const POINT3_EQUALS: u32 = 0x0043_90c0;
const POINT3_CROSS: u32 = 0x004b_3800;
const POINT3_NORMALIZE: u32 = 0x004a_0c10;
const POINT3_SQUARED_LENGTH: u32 = 0x004a_7290;
const POINT3_SQUARED_LENGTH_2D: u32 = 0x0059_5c80;
/// `NiPoint3::UnitCross` (`this`, out, other).
const POINT3_UNIT_CROSS: u32 = 0x0053_d1a0;
/// Copy of a 16-byte structure (`this`, other): the base of the request
/// copy operators.
const COPY_STRUCT_16: u32 = 0x004a_3c90;
/// Height of the ground under a point in a cell (`this` is the cell, point,
/// out float).
const CELL_GROUND_HEIGHT: u32 = 0x0054_7250;
/// Random `float` in a range (cdecl: min, max) and the square root of a
/// `float` (cdecl, result in ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
const FLOAT_SQRT: u32 = 0x0040_19b0;
/// Ray test of a point against a triangle's three vertices (cdecl: three
/// vertex pointers, point, out float, two floats), the 2D test of two
/// segments (cdecl: four words and an out pointer) and the distance of a
/// point to a segment (cdecl: point, first end, second end; `float`).
const TRIANGLE_RAY_TEST: u32 = 0x006b_9cf0;
const SEGMENT_SEGMENT_TEST: u32 = 0x006b_9e90;
const SEGMENT_INTERSECTION_2D: u32 = 0x006b_a240;
const POINT_SEGMENT_DISTANCE: u32 = 0x006b_9940;
/// The `_vector_constructor_iterator_` (cdecl: array, element size, count,
/// constructor).
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x0040_1050;
/// Constructor of the 16-byte element of the two-element array `006985c0`
/// builds.
const ELEMENT_16_CONSTRUCTOR: u32 = 0x006d_5320;
/// `NiPoint3` default constructor (`this`; only returns it).
const POINT3_DEFAULT_CONSTRUCTOR: u32 = 0x0068_15c0;

/// "No triangle" in a 16-bit triangle index.
const NO_TRIANGLE: u16 = 0xffff;

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// `BSSimpleArray::iSize` (`0044ddc0`).
fn array_size<T>(e: &mut Engine, array: Ptr<T>) -> u32 {
    e.call(ARRAY_SIZE, &args![array]).u32()
}

/// `NavMesh::GetTriangle(index)` (`005582f0`); the index is a `u16`.
fn triangle_of(e: &mut Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
    e.call(GET_TRIANGLE, &args![mesh, index as u16]).ptr()
}

/// `NavMeshTriangle::GetVertex(corner)` (`005582d0`).
fn corner_vertex_index(e: &mut Engine, triangle: Ptr<NavMeshTriangle>, corner: u32) -> u16 {
    e.call(TRIANGLE_VERTEX, &args![triangle, corner]).u16()
}

/// Address of vertex `index` of the mesh (`0068f0a0`).
fn vertex_pointer(e: &mut Engine, mesh: Ptr<NavMesh>, index: u16) -> Ptr {
    e.call(VERTEX_POINTER, &args![mesh, index]).ptr()
}

/// Address of corner `corner` of triangle `triangle`.
fn corner_pointer(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    triangle: Ptr<NavMeshTriangle>,
    corner: u32,
) -> Ptr {
    let index = corner_vertex_index(e, triangle, corner);
    vertex_pointer(e, mesh, index)
}

/// The three words at `address` as three `float`s.
fn read_point(e: &Engine, address: u32) -> [f32; 3] {
    [
        e.mem.f32(address),
        e.mem.f32(address + 4),
        e.mem.f32(address + 8),
    ]
}

fn write_point(e: &mut Engine, address: u32, point: [f32; 3]) {
    e.mem.set_f32(address, point[0]);
    e.mem.set_f32(address + 4, point[1]);
    e.mem.set_f32(address + 8, point[2]);
}

/// The search grid embedded in the mesh.
fn mesh_grid(mesh: Ptr<NavMesh>) -> Ptr<NavMeshGrid> {
    mesh.byte_add(MESH_GRID_OFFSET).cast()
}

/// The `Triangles` array embedded in the mesh.
fn mesh_triangles(mesh: Ptr<NavMesh>) -> Ptr {
    mesh.byte_add(MESH_TRIANGLES_OFFSET)
}

/// The condition `FCOMP` leaves for `TEST AH, mask`: bit C0 (`st0 < operand`),
/// C2 (unordered) and C3 (equal) of the x87 status word as `0x01`, `0x04`
/// and `0x40`.
fn compare_flags(st0: f64, operand: f64) -> u8 {
    if st0.is_nan() || operand.is_nan() {
        0x45
    } else if st0 < operand {
        0x01
    } else if st0 == operand {
        0x40
    } else {
        0
    }
}

/// `TEST AH, mask` followed by `JP` (jump on even parity of the result).
fn parity_even(flags: u8, mask: u8) -> bool {
    (flags & mask).count_ones() % 2 == 0
}

/// Truncation of `value` as `FISTP qword` (rounding control: truncate) and
/// the low 32 bits of the result; an out-of-range value or a NaN gives the
/// "integer indefinite" `0x8000000000000000`.
fn float_to_integer_low(value: f32) -> u32 {
    let value = value as f64;
    if value.is_nan() || value.abs() >= 9.223_372_036_854_776e18 {
        0
    } else {
        value as i64 as u32
    }
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

// Translated from 006953b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FixPortalsFromNavMeshDeletion` (Xbox PDB; cdecl): the mesh
/// info `deleted_info` is going away. For every `NavMeshInfo` the info map
/// holds, every entry of its adjacent and preferred-adjacent lists that is
/// `deleted_info` is removed, and, when that info's navmesh is loaded (it is
/// looked up by its form id when `pNavMesh` is null), the navmesh's extra
/// edge infos whose portal leads to `deleted_info` are deleted too
/// (`NavMesh::DeleteExtraEdgeInfo`, after a virtual call on the mesh with `1`).
/// Does nothing for a null `deleted_info`.
pub fn nav_mesh_fix_portals_from_nav_mesh_deletion(e: &mut Engine, deleted_info: Ptr) {
    if deleted_info.is_null() {
        return;
    }
    e.with_stack(0x10, |e, list| {
        e.call(INFO_LIST_CONSTRUCTOR, &args![list]);
        let tes: u32 = e.global(TES_INSTANCE);
        let map = e.call(GET_NAV_MESH_INFO_MAP, &args![tes]).u32();
        e.call(INFO_MAP_COLLECT, &args![map, list]);
        let mut index = 0u32;
        while index < array_size(e, list) {
            let slot = e.call(ARRAY_ELEMENT_4, &args![list, index]).u32();
            let info = e.mem.u32(slot);
            let adjacent = Ptr::<()>::new(info + INFO_ADJACENT_NAV_MESHES);
            let preferred = Ptr::<()>::new(info + INFO_PREFERRED_ADJACENT_NAV_MESHES);
            let mut adjacent_count = array_size(e, adjacent) as i32;
            let mut preferred_count = array_size(e, preferred) as i32;
            let mut entry = 0i32;
            while entry < adjacent_count.wrapping_add(preferred_count) {
                let slot = if entry < adjacent_count {
                    e.call(ARRAY_ELEMENT_4, &args![adjacent, entry]).u32()
                } else {
                    e.call(
                        ARRAY_ELEMENT_4,
                        &args![preferred, entry.wrapping_sub(adjacent_count)],
                    )
                    .u32()
                };
                if e.mem.u32(slot) == deleted_info.addr() {
                    e.with_stack(4, |e, mesh_pointer| {
                        e.call(POINTER_CONSTRUCT, &args![mesh_pointer]);
                        let loaded = e
                            .call(
                                INFO_GET_NAV_MESH,
                                &args![Ptr::<()>::new(info), mesh_pointer],
                            )
                            .bool();
                        if !loaded {
                            let form_id = e.call(POINTER_GET, &args![Ptr::<()>::new(info)]).u32();
                            let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
                            let cast = e
                                .call(
                                    RT_DYNAMIC_CAST,
                                    &args![form, 0i32, TYPE_TES_FORM, TYPE_NAV_MESH, 0i32],
                                )
                                .u32();
                            e.with_stack(4, |e, temporary| {
                                let made = e
                                    .call(POINTER_CONSTRUCT_FROM_CDECL, &args![temporary, cast])
                                    .u32();
                                e.call(POINTER_ASSIGN, &args![mesh_pointer, made]);
                                e.call(POINTER_DESTRUCT, &args![temporary]);
                            });
                        }
                        if e.call(POINTER_DEREFERENCE, &args![mesh_pointer]).u32() != 0 {
                            let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                            e.vcall(mesh, NAV_MESH_SLOT_0XC8, &args![1u32]);
                            let mut extra = 0u32;
                            loop {
                                let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                let count = array_size(
                                    e,
                                    Ptr::<()>::new(mesh + MESH_EXTRA_EDGE_INFO_OFFSET),
                                );
                                if extra >= count {
                                    break;
                                }
                                let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                let edge_info = e
                                    .call(GET_EXTRA_EDGE_INFO, &args![mesh, extra & 0xffff])
                                    .u32();
                                if e.mem.u32(edge_info + EDGE_INFO_OTHER_MESH)
                                    == deleted_info.addr()
                                {
                                    let mesh = e.call(POINTER_GET, &args![mesh_pointer]).u32();
                                    e.call(DELETE_EXTRA_EDGE_INFO, &args![mesh, extra & 0xffff]);
                                    extra = extra.wrapping_sub(1);
                                }
                                extra = extra.wrapping_add(1);
                            }
                        }
                        if entry < adjacent_count {
                            e.call(ARRAY_REMOVE, &args![adjacent, entry, 1u32]);
                            adjacent_count -= 1;
                        } else {
                            e.call(
                                ARRAY_REMOVE,
                                &args![preferred, entry.wrapping_sub(adjacent_count), 1u32],
                            );
                            preferred_count -= 1;
                        }
                        entry -= 1;
                        e.call(POINTER_DESTRUCT, &args![mesh_pointer]);
                    });
                }
                entry += 1;
            }
            index += 1;
        }
        e.call(INFO_LIST_DESTRUCTOR, &args![list]);
    });
}

// Translated from 00695660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Flips triangle `triangle` of the mesh: vertices 0 and 1 are swapped, and
/// so are edges 0 and 1 (the plain neighbour or extra info slot, the extra
/// info flag and, when the triangle carries edge bits (`006957d0`), the six
/// packed edge bits of each edge).
pub fn fn_00695660(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16) {
    let tri = triangle_of(e, this, triangle as u32);
    let second_vertex = corner_vertex_index(e, tri, 1);
    let first_vertex = corner_vertex_index(e, tri, 0);
    e.call(SET_VERTEX, &args![tri, 1u32, first_vertex]);
    e.call(SET_VERTEX, &args![tri, 0u32, second_vertex]);
    let first_has_info = e.call(EDGE_HAS_INFO, &args![tri, 0u32]).bool();
    let second_has_info = e.call(EDGE_HAS_INFO, &args![tri, 1u32]).bool();
    let second_slot = e.call(EDGE_SLOT, &args![tri, 1u32]).u16();
    let first_slot = e.call(EDGE_SLOT, &args![tri, 0u32]).u16();
    if first_has_info {
        e.call(SET_EDGE_INFO_SLOT, &args![tri, 1u32, first_slot]);
    } else {
        e.call(SET_EDGE_NEIGHBOUR, &args![tri, 1u32, first_slot]);
    }
    if second_has_info {
        e.call(SET_EDGE_INFO_SLOT, &args![tri, 0u32, second_slot]);
    } else {
        e.call(SET_EDGE_NEIGHBOUR, &args![tri, 0u32, second_slot]);
    }
    if fn_006957d0(e, tri) {
        // Value word, then the two flag bytes, of edge 0 and edge 1.
        e.with_stack(16, |e, scratch| {
            let (value0, flag0_a, flag0_b) = (scratch, scratch.byte_add(4), scratch.byte_add(5));
            let (value1, flag1_a, flag1_b) = (
                scratch.byte_add(8),
                scratch.byte_add(12),
                scratch.byte_add(13),
            );
            e.call(
                UNPACK_EDGE_BITS,
                &args![tri, 0u32, value0, flag0_a, flag0_b],
            );
            e.call(
                UNPACK_EDGE_BITS,
                &args![tri, 1u32, value1, flag1_a, flag1_b],
            );
            let (v0, a0, b0) = (
                e.mem.u16(value0.addr()),
                e.mem.u8(flag0_a.addr()),
                e.mem.u8(flag0_b.addr()),
            );
            let (v1, a1, b1) = (
                e.mem.u16(value1.addr()),
                e.mem.u8(flag1_a.addr()),
                e.mem.u8(flag1_b.addr()),
            );
            e.call(PACK_EDGE_BITS, &args![tri, 0u32, v1, b1, a1]);
            e.call(PACK_EDGE_BITS, &args![tri, 1u32, v0, b0, a0]);
        });
    }
}

// Translated from 006957d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: whether any of the packed edge bits (flags
/// `0x0FFF0000`) is set.
pub fn fn_006957d0(e: &mut Engine, this: Ptr<NavMeshTriangle>) -> bool {
    e.get(this, NavMeshTriangle::TriangleFlags) & 0x0fff_0000 != 0
}

// Translated from 006957f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Bounding box of the mesh's vertices as seen through its triangles:
/// stores the smallest x, y, z at `min` and the largest at `max` (three
/// floats each). Does nothing when the mesh has no triangles.
pub fn fn_006957f0(e: &mut Engine, this: Ptr<NavMesh>, min: Ptr, max: Ptr) {
    if e.call(ARRAY_IS_EMPTY, &args![mesh_triangles(this)]).bool() {
        return;
    }
    let first = triangle_of(e, this, 0);
    let start = corner_pointer(e, this, first, 0);
    let mut high = read_point(e, start.addr());
    let mut low = high;
    let mut index = 0u32;
    while index < e.call(TRIANGLE_COUNT, &args![this]).u32() {
        let tri = triangle_of(e, this, index);
        for corner in 0..3 {
            let vertex = corner_pointer(e, this, tri, corner);
            let vertex = read_point(e, vertex.addr());
            for axis in 0..3 {
                if low[axis] > vertex[axis] {
                    low[axis] = vertex[axis];
                } else if high[axis] < vertex[axis] {
                    high[axis] = vertex[axis];
                }
            }
        }
        index += 1;
    }
    write_point(e, min.addr(), low);
    write_point(e, max.addr(), high);
}

// Translated from 006959b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rebuilds the mesh's search grid: clears it, and for a mesh with triangles
/// sizes it from the mesh bounds (`006957f0`, `0069d800`) and then lists
/// every triangle in every grid cell it touches. A triangle is listed in a
/// cell when
/// - all its vertices fall in one cell, or
/// - the cell is within the triangle's cell rectangle and one of the
///   vertices' cells, or one vertex lies inside the cell (the second
///   vertex is tested against `Min.y <= third vertex y`, as the game does),
///   or the ray test `00697730` hits one of the cell's four corners, or one
///   of the cell's four sides crosses one of the triangle's three edges
///   (`00696540`).
///
/// The per-triangle minimum and maximum x and y the code also computes are
/// never used and are left out; so is the "listed" flag it keeps.
pub fn fn_006959b0(e: &mut Engine, this: Ptr<NavMesh>) {
    let triangle_count = e.call(TRIANGLE_COUNT, &args![this]).u32() as i32;
    let grid = mesh_grid(this);
    e.call(GRID_CLEAR, &args![grid]);
    if triangle_count == 0 {
        return;
    }
    // 0: bounds min, 12: bounds max, 24: the four cell corners (12 bytes
    // each), 72: a corner read from the grid, 84: the height out parameter,
    // 96: the four points of the two segments (cell side, triangle edge).
    e.with_stack(144, |e, scratch| {
        let bounds_min = scratch;
        let bounds_max = scratch.byte_add(12);
        let corner_slots = [24u32, 36, 48, 60].map(|offset| scratch.byte_add(offset));
        let corner_out = scratch.byte_add(72);
        let height_out = scratch.byte_add(84);
        let segment_slots = [96u32, 108, 120, 132].map(|offset| scratch.byte_add(offset));

        fn_006957f0(e, this, bounds_min, bounds_max);
        e.call(
            GRID_SETUP,
            &args![grid, bounds_min, bounds_max, triangle_count],
        );
        let size = e.call(POINTER_GET, &args![grid]).u32();
        let column_length = e.get(grid, NavMeshGrid::fColumnSectionLen);
        let row_length = e.get(grid, NavMeshGrid::fRowSectionLen);
        let min_x = e.mem.f32(bounds_min.addr());
        let min_y = e.mem.f32(bounds_min.addr() + 4);
        let ray_limit: f32 = e.global(FLOAT_MAX);
        let ray_height: f32 = e.global(HEIGHT_UP);

        for index in 0..triangle_count {
            let triangle = index as u16 as u32;
            let tri = triangle_of(e, this, index as u32);
            let mut vertices = [[0f32; 3]; 3];
            let mut cells = [[0u32; 2]; 3];
            for corner in 0..3usize {
                let vertex = corner_pointer(e, this, tri, corner as u32);
                let vertex = read_point(e, vertex.addr());
                vertices[corner] = vertex;
                let column_position =
                    ((vertex[0] as f64 - min_x as f64) / column_length as f64) as f32;
                let floored = e.call(FLOAT_FLOOR, &args![column_position]).f32();
                cells[corner][0] = float_to_integer_low(floored);
                let row_position = ((vertex[1] as f64 - min_y as f64) / row_length as f64) as f32;
                let floored = e.call(FLOAT_FLOOR, &args![row_position]).f32();
                cells[corner][1] = float_to_integer_low(floored);
                if cells[corner][0] >= size {
                    cells[corner][0] = size.wrapping_sub(1);
                }
                if cells[corner][1] >= size {
                    cells[corner][1] = size.wrapping_sub(1);
                }
            }
            let (mut cell_min_x, mut cell_max_x) = (cells[0][0], cells[0][0]);
            let (mut cell_min_y, mut cell_max_y) = (cells[0][1], cells[0][1]);
            for cell in &cells[1..] {
                cell_min_x = cell_min_x.min(cell[0]);
                cell_max_x = cell_max_x.max(cell[0]);
                cell_min_y = cell_min_y.min(cell[1]);
                cell_max_y = cell_max_y.max(cell[1]);
            }
            if cell_min_x == cell_max_x && cell_min_y == cell_max_y {
                fn_00696500(e, grid, triangle, cell_min_x, cell_min_y);
                continue;
            }
            let mut cell_x = cell_min_x;
            while cell_x <= cell_max_x {
                let mut cell_y = cell_min_y;
                while cell_y <= cell_max_y {
                    let vertex_cell = cells.iter().any(|c| c[0] == cell_x && c[1] == cell_y);
                    if vertex_cell {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    // The cell's corners: (x, y), (x + 1, y), (x, y + 1),
                    // (x + 1, y + 1).
                    let mut corners = [[0f32; 3]; 4];
                    for (n, (dx, dy)) in [(0u32, 0u32), (1, 0), (0, 1), (1, 1)]
                        .into_iter()
                        .enumerate()
                    {
                        let corner = e
                            .call(
                                GRID_CELL_CORNER,
                                &args![
                                    grid,
                                    corner_out,
                                    cell_x.wrapping_add(dx),
                                    cell_y.wrapping_add(dy)
                                ],
                            )
                            .u32();
                        corners[n] = read_point(e, corner);
                        write_point(e, corner_slots[n].addr(), corners[n]);
                    }
                    let (low, high) = (corners[0], corners[3]);
                    let (v0, v1, v2) = (vertices[0], vertices[1], vertices[2]);
                    let inside = |v: [f32; 3], y: f32| {
                        v[0] <= high[0] && v[1] <= high[1] && low[0] <= v[0] && low[1] <= y
                    };
                    // The second vertex is tested against its own x and y
                    // but the third vertex's y for the low side.
                    let accepted = inside(v0, v0[1]) || inside(v1, v2[1]) || inside(v2, v2[1]);
                    if accepted {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    let mut hit = false;
                    for slot in corner_slots {
                        if fn_00697730(
                            e,
                            this,
                            triangle as u16,
                            slot,
                            height_out,
                            ray_limit,
                            ray_height,
                        ) {
                            hit = true;
                            break;
                        }
                    }
                    if hit {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                        cell_y += 1;
                        continue;
                    }
                    // Cell sides in the order bottom, right, top, left;
                    // triangle edges 0-1, 1-2, 2-0.
                    let sides = [(0usize, 1usize), (1, 3), (3, 2), (2, 0)];
                    let edges = [(0usize, 1usize), (1, 2), (2, 0)];
                    let mut crossing = false;
                    'sides: for (from, to) in sides {
                        write_point(e, segment_slots[0].addr(), corners[from]);
                        write_point(e, segment_slots[1].addr(), corners[to]);
                        for (edge_from, edge_to) in edges {
                            write_point(e, segment_slots[2].addr(), vertices[edge_from]);
                            write_point(e, segment_slots[3].addr(), vertices[edge_to]);
                            if fn_00696540(
                                e,
                                segment_slots[0],
                                segment_slots[1],
                                segment_slots[2],
                                segment_slots[3],
                            ) {
                                crossing = true;
                                break 'sides;
                            }
                        }
                    }
                    if crossing {
                        fn_00696500(e, grid, triangle, cell_x, cell_y);
                    }
                    cell_y += 1;
                }
                cell_x += 1;
            }
        }
    });
}

// Translated from 00696500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: appends `triangle` (as a `u16`) to the triangle
/// list of cell `(x, y)`, cell number `y * iGridSize + x`.
pub fn fn_00696500(e: &mut Engine, this: Ptr<NavMeshGrid>, triangle: u32, x: u32, y: u32) {
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cell_number = y.wrapping_mul(size).wrapping_add(x);
    let cells = e.get(this, NavMeshGrid::GridData);
    let cell = cells.addr().wrapping_add(cell_number << 4);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), triangle);
        e.call(ARRAY_ADD_WORD, &args![Ptr::<()>::new(cell), slot]);
    });
}

// Translated from 00696540 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl: whether the 2D segments `a`-`b` and `c`-`d` intersect. Each point
/// (three floats; only x and y are used) is copied into a `NiPoint2` and the
/// four go to `006ba010`.
pub fn fn_00696540(e: &mut Engine, a: Ptr, b: Ptr, c: Ptr, d: Ptr) -> bool {
    e.with_stack(32, |e, base| {
        // The copies are built d first, then c, b and a.
        let slots = [
            base.byte_add(24), // a
            base.byte_add(16), // b
            base.byte_add(8),  // c
            base,              // d
        ];
        for (point, slot) in [(d, slots[3]), (c, slots[2]), (b, slots[1]), (a, slots[0])] {
            let x = e.mem.f32(point.addr());
            let y = e.mem.f32(point.addr() + 4);
            e.call(POINT2_CONSTRUCT, &args![slot, x, y]);
        }
        e.call(
            SEGMENTS_INTERSECT_2D,
            &args![slots[0], slots[1], slots[2], slots[3]],
        )
        .bool()
    })
}

// Translated from 006965d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lists triangles `first` to `last` (inclusive) in the search grid: for
/// each, finds the smallest and largest x and y of its vertices, converts
/// both to grid rows and columns (`006912a0`) and appends the triangle to
/// every cell of that rectangle. Returns true.
pub fn fn_006965d0(e: &mut Engine, this: Ptr<NavMesh>, first: u16, last: u16) -> bool {
    let grid = mesh_grid(this);
    let mut triangle = first as u32;
    while triangle <= last as u32 {
        let tri = triangle_of(e, this, triangle);
        let (low, high) = triangle_xy_bounds(e, this, tri);
        e.with_stack(48, |e, scratch| {
            let low_point = scratch;
            let high_point = scratch.byte_add(12);
            write_point(e, low_point.addr(), low);
            write_point(e, high_point.addr(), high);
            let (row_min, column_min) = (scratch.byte_add(24), scratch.byte_add(28));
            let (row_max, column_max) = (scratch.byte_add(32), scratch.byte_add(36));
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, low_point, row_min, column_min],
            );
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, high_point, row_max, column_max],
            );
            let (row_min, column_min) = (e.mem.u32(row_min.addr()), e.mem.u32(column_min.addr()));
            let (row_max, column_max) = (e.mem.u32(row_max.addr()), e.mem.u32(column_max.addr()));
            let mut row = row_min;
            while row <= row_max {
                let mut column = column_min;
                while column <= column_max {
                    let cell = fn_006967c0(e, grid, column, row);
                    if !cell.is_null() {
                        e.with_stack(4, |e, slot| {
                            e.mem.set_u16(slot.addr(), triangle as u16);
                            e.call(ARRAY_ADD_WORD, &args![cell, slot]);
                        });
                    }
                    column = column.wrapping_add(1);
                }
                row = row.wrapping_add(1);
            }
        });
        triangle += 1;
    }
    true
}

/// Smallest and largest x and y of a triangle's three vertices (z is 0). The
/// minimum starts at `FLT_MAX` and the maximum at `-FLT_MAX`; each vertex
/// widens them with a strict comparison (`NiPoint3(x, y, 0)` constructor
/// `00416870`).
fn triangle_xy_bounds(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    tri: Ptr<NavMeshTriangle>,
) -> ([f32; 3], [f32; 3]) {
    let flt_max: f32 = e.global(FLOAT_MAX);
    let negative_flt_max: f32 = e.global(NEGATIVE_FLOAT_MAX);
    let (low, high) = e.with_stack(24, |e, scratch| {
        let high_slot = scratch.byte_add(12);
        e.call(POINT3_CONSTRUCT, &args![scratch, flt_max, flt_max, 0.0f32]);
        e.call(
            POINT3_CONSTRUCT,
            &args![high_slot, negative_flt_max, negative_flt_max, 0.0f32],
        );
        for corner in 0..3 {
            let vertex = corner_pointer(e, mesh, tri, corner);
            let vertex = read_point(e, vertex.addr());
            let low = read_point(e, scratch.addr());
            let high = read_point(e, high_slot.addr());
            if low[0] > vertex[0] {
                e.mem.set_f32(scratch.addr(), vertex[0]);
            }
            if high[0] < vertex[0] {
                e.mem.set_f32(high_slot.addr(), vertex[0]);
            }
            if low[1] > vertex[1] {
                e.mem.set_f32(scratch.addr() + 4, vertex[1]);
            }
            if high[1] < vertex[1] {
                e.mem.set_f32(high_slot.addr() + 4, vertex[1]);
            }
        }
        (
            read_point(e, scratch.addr()),
            read_point(e, high_slot.addr()),
        )
    });
    (low, high)
}

// Translated from 006967c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: the address of the triangle list of cell
/// `(x, y)`: `GridData + (y * iGridSize + x) * 16`.
pub fn fn_006967c0(e: &mut Engine, this: Ptr<NavMeshGrid>, x: u32, y: u32) -> Ptr {
    let size = e.get(this, NavMeshGrid::iGridSize);
    let cell_number = y.wrapping_mul(size).wrapping_add(x);
    let cells = e.get(this, NavMeshGrid::GridData);
    Ptr::new((cell_number << 4).wrapping_add(cells.addr()))
}

// Translated from 006967f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `006965d0`: removes triangles `first` to `last` (inclusive)
/// from every grid cell of their rectangle (the triangle is looked up in
/// the cell's list with `0069b240` and removed with `0069add0`). Returns
/// true.
pub fn fn_006967f0(e: &mut Engine, this: Ptr<NavMesh>, first: u16, last: u16) -> bool {
    let grid = mesh_grid(this);
    let mut triangle = first as u32;
    while triangle <= last as u32 {
        let tri = triangle_of(e, this, triangle);
        let (low, high) = triangle_xy_bounds(e, this, tri);
        e.with_stack(48, |e, scratch| {
            let low_point = scratch;
            let high_point = scratch.byte_add(12);
            write_point(e, low_point.addr(), low);
            write_point(e, high_point.addr(), high);
            let (row_min, column_min) = (scratch.byte_add(24), scratch.byte_add(28));
            let (row_max, column_max) = (scratch.byte_add(32), scratch.byte_add(36));
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, low_point, row_min, column_min],
            );
            e.call(
                GRID_CELL_OF_POINT,
                &args![grid, high_point, row_max, column_max],
            );
            let (row_min, column_min) = (e.mem.u32(row_min.addr()), e.mem.u32(column_min.addr()));
            let (row_max, column_max) = (e.mem.u32(row_max.addr()), e.mem.u32(column_max.addr()));
            let mut row = row_min;
            while row <= row_max {
                let mut column = column_min;
                while column <= column_max {
                    let cell = fn_006967c0(e, grid, column, row);
                    e.with_stack(4, |e, key| {
                        e.mem.set_u32(key.addr(), triangle);
                        let index = e
                            .call(ARRAY_FIND_WORD, &args![cell, key, 0u32, WORD_COMPARATOR])
                            .u32();
                        e.call(WORD_ARRAY_REMOVE, &args![cell, index, 1u32]);
                    });
                    column = column.wrapping_add(1);
                }
                row = row.wrapping_add(1);
            }
        });
        triangle += 1;
    }
    true
}

// Translated from 006969e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the search grid's bounds to `min` and `max` (`00696a10` on the
/// embedded grid). Returns true.
pub fn fn_006969e0(e: &mut Engine, this: Ptr<NavMesh>, min: Ptr, max: Ptr) -> bool {
    fn_00696a10(e, mesh_grid(this), min, max);
    true
}

// Translated from 00696a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshGrid` method: copies `GridBoundsMin` to the three floats at `min`
/// and `GridBoundsMax` to those at `max`.
pub fn fn_00696a10(e: &mut Engine, this: Ptr<NavMeshGrid>, min: Ptr, max: Ptr) {
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x0c + 4 * word);
        e.mem.set_u32(min.addr() + 4 * word, value);
    }
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x18 + 4 * word);
        e.mem.set_u32(max.addr() + 4 * word, value);
    }
}

// Translated from 00696a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The triangle under the point at `point` (x, y, z floats): looks up the
/// grid cell's triangle list and returns the index of the triangle whose
/// surface is nearest in height (`00697810`, with `callback`, an optional
/// object whose virtual slot 0 is told about each hit), or `0xFFFF`.
/// Triangles with flag `0x20` are skipped; when the point is not more than
/// 50 above the ground of the mesh's parent cell (`00547250`) heights below
/// `-180` are accepted too, otherwise not; heights above `64` are never
/// accepted. A triangle within `40` returns at once, otherwise the nearest
/// wins.
pub fn fn_00696a50(e: &mut Engine, this: Ptr<NavMesh>, point: Ptr, callback: Ptr) -> u16 {
    e.with_stack(12, |e, scratch| {
        let list_slot = scratch;
        let ground_slot = scratch.byte_add(4);
        let height_slot = scratch.byte_add(8);
        let grid = mesh_grid(this);
        e.call(GRID_CELL_LIST, &args![grid, point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            return NO_TRIANGLE;
        }
        let count = array_size(e, Ptr::<()>::new(list));
        let mut best_height: f32 = e.global(FLOAT_MAX);
        let ground: f32 = e.global(NEGATIVE_FLOAT_MAX);
        e.mem.set_f32(ground_slot.addr(), ground);
        let cell = e.get(this, NavMesh::pParentCell);
        if !cell.is_null() {
            e.call(CELL_GROUND_HEIGHT, &args![cell, point, ground_slot]);
        }
        let margin: f64 = e.global(GROUND_MARGIN);
        let threshold = (e.mem.f32(ground_slot.addr()) as f64 + margin) as f32;
        let point_z = e.mem.f32(point.addr() + 8);
        // `FCOMPP` then `JP`: the point is below the threshold.
        let near_ground = point_z < threshold;
        let mut best = NO_TRIANGLE;
        let mut index = 0u32;
        while index < count {
            let triangle = e
                .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                .ptr::<()>();
            let triangle = e.mem.u16(triangle.addr());
            index += 1;
            let tri = e
                .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
                .ptr::<NavMeshTriangle>();
            if e.call(TRIANGLE_FLAGS_SET, &args![tri, 0x20u32]).bool() {
                continue;
            }
            if !fn_00697810(e, this, triangle, point, height_slot, callback) {
                continue;
            }
            let height = e.mem.f32(height_slot.addr());
            if !near_ground {
                let below: f64 = e.global(NEGATIVE_180);
                if (height as f64) < below {
                    continue;
                }
            }
            let above: f64 = e.global(SIXTY_FOUR);
            if height as f64 > above {
                continue;
            }
            let height = e.call(FLOAT_ABS, &args![height]).f32();
            e.mem.set_f32(height_slot.addr(), height);
            if best_height.partial_cmp(&height) != Some(std::cmp::Ordering::Greater) {
                continue;
            }
            let close: f64 = e.global(FORTY);
            if (height as f64) < close {
                return triangle;
            }
            best = triangle;
            best_height = height;
        }
        best
    })
}

// Translated from 00696c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards three words to `0069d4b0` on the mesh's search grid.
pub fn fn_00696c00(e: &mut Engine, this: Ptr<NavMesh>, first: u32, second: u32, third: u32) {
    e.call(
        GRID_FORWARDED,
        &args![mesh_grid(this), first, second, third],
    );
}

// Translated from 00696c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x54-byte structure: `00696c60` (the first 0x44
/// bytes), then the word at +0x50. Returns `this`.
pub fn fn_00696c30(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    fn_00696c60(e, this, other);
    let value = e.mem.u32(other.addr() + 0x50);
    e.mem.set_u32(this.addr() + 0x50, value);
    this
}

// Translated from 00696c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x44-byte structure: `00696ca0` (the first 0x20
/// bytes), then the eight words at +0x20 and the word at +0x40. Returns
/// `this`.
pub fn fn_00696c60(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    fn_00696ca0(e, this, other);
    for word in 0..8 {
        let value = e.mem.u32(other.addr() + 0x20 + 4 * word);
        e.mem.set_u32(this.addr() + 0x20 + 4 * word, value);
    }
    let value = e.mem.u32(other.addr() + 0x40);
    e.mem.set_u32(this.addr() + 0x40, value);
    this
}

// Translated from 00696ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy assignment of the 0x20-byte structure: the 16-byte part (`004a3c90`),
/// then the four words at +0x10 (the first is copied as a `float`).
/// Returns `this`.
pub fn fn_00696ca0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    e.call(COPY_STRUCT_16, &args![this, other]);
    let value = e.mem.f32(other.addr() + 0x10);
    e.mem.set_f32(this.addr() + 0x10, value);
    for word in 1..4 {
        let value = e.mem.u32(other.addr() + 0x10 + 4 * word);
        e.mem.set_u32(this.addr() + 0x10 + 4 * word, value);
    }
    this
}

/// State of `NavMesh::FindClosestTriangleForLocation`'s scan.
struct ClosestScan {
    /// Nearest triangle that has a height at the point (`0xFFFF`: none) and
    /// its absolute height difference.
    by_height: u16,
    height_distance: f32,
    /// Triangle whose center is nearest to the point and the squared
    /// distance.
    by_center: u16,
    center_distance: f32,
}

impl ClosestScan {
    fn new(e: &Engine) -> ClosestScan {
        ClosestScan {
            by_height: NO_TRIANGLE,
            height_distance: e.global(FLOAT_MAX),
            by_center: NO_TRIANGLE,
            center_distance: e.global(FLOAT_MAX),
        }
    }
}

/// One candidate triangle of `FindClosestTriangleForLocation`: the ray test
/// `00697730` (tolerances 180 and 64) gives a height difference; a triangle
/// within 40 ends the search (returns true after storing the answer); a
/// nearer one becomes `by_height`. Without a height, the squared distance
/// from the triangle's center (`GetCenter`, `-`, squared length) can become
/// `by_center`.
fn closest_candidate(
    e: &mut Engine,
    mesh: Ptr<NavMesh>,
    scan: &mut ClosestScan,
    triangle: u16,
    point: Ptr,
    out_triangle: Ptr,
    out_found: Ptr,
) -> bool {
    let up: f32 = e.global(HEIGHT_UP);
    let down: f32 = e.global(HEIGHT_DOWN);
    e.with_stack(32, |e, scratch| {
        let height_slot = scratch;
        let center_difference = scratch.byte_add(4);
        let center_slot = scratch.byte_add(16);
        if fn_00697730(e, mesh, triangle, point, height_slot, up, down) {
            let height = e.mem.f32(height_slot.addr());
            let height = e.call(FLOAT_ABS, &args![height]).f32();
            e.mem.set_f32(height_slot.addr(), height);
            if scan.height_distance > height {
                let close: f64 = e.global(FORTY);
                if (height as f64) < close {
                    e.mem.set_u16(out_triangle.addr(), triangle);
                    e.mem.set_u8(out_found.addr(), 1);
                    return true;
                }
                scan.by_height = triangle;
                scan.height_distance = height;
            }
        } else {
            let center = e
                .call(GET_CENTER, &args![mesh, center_slot, triangle as u32])
                .ptr::<()>();
            e.call(POINT3_SUBTRACT, &args![center, center_difference, point]);
            let distance = e
                .call(POINT3_SQUARED_LENGTH, &args![center_difference])
                .f32();
            if scan.center_distance > distance {
                scan.by_center = triangle;
                scan.center_distance = distance;
            }
        }
        false
    })
}

// Translated from 00696cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindClosestTriangleForLocation` (Xbox PDB): the triangle for
/// the point at `point` (three floats). The candidates are the triangles of
/// the grid cell holding the point (without flag `0x20`), or all triangles
/// when the grid has no list for the point. The stored triangle is the
/// nearest one with a height at the point (stored `1` at `out_found`),
/// otherwise the one with the nearest center (stored `0`), otherwise
/// `0xFFFF` (stored `0`, result false).
pub fn nav_mesh_find_closest_triangle_for_location(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    out_triangle: Ptr,
    out_found: Ptr,
) -> bool {
    let mut scan = ClosestScan::new(e);
    e.with_stack(4, |e, list_slot| {
        e.call(GRID_CELL_LIST, &args![mesh_grid(this), point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            let count = array_size(e, mesh_triangles(this));
            let mut index = 0u32;
            while index < count {
                if closest_candidate(
                    e,
                    this,
                    &mut scan,
                    index as u16,
                    point,
                    out_triangle,
                    out_found,
                ) {
                    return true;
                }
                index += 1;
            }
        } else {
            let count = array_size(e, Ptr::<()>::new(list));
            let mut index = 0u32;
            while index < count {
                let slot = e
                    .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                    .u32();
                let triangle = e.mem.u16(slot);
                index += 1;
                let tri = e
                    .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
                    .ptr::<NavMeshTriangle>();
                if e.call(TRIANGLE_FLAGS_SET, &args![tri, 0x20u32]).bool() {
                    continue;
                }
                if closest_candidate(e, this, &mut scan, triangle, point, out_triangle, out_found) {
                    return true;
                }
            }
        }
        if scan.by_height != NO_TRIANGLE {
            e.mem.set_u16(out_triangle.addr(), scan.by_height);
            e.mem.set_u8(out_found.addr(), 1);
            return true;
        }
        if scan.by_center == NO_TRIANGLE {
            e.mem.set_u16(out_triangle.addr(), NO_TRIANGLE);
            e.mem.set_u8(out_found.addr(), 0);
            return false;
        }
        e.mem.set_u16(out_triangle.addr(), scan.by_center);
        e.mem.set_u8(out_found.addr(), 0);
        true
    })
}

// Translated from 00697010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scans the mesh's edges without neighbours and measures each midpoint's
/// squared 2D and 3D distance from `point`. An edge qualifies when its
/// midpoint's z lies strictly between `point.z - 64` and `point.z + 180`
/// (swapped when `flip` is set), or when its 3D squared distance is at most
/// `radius² - dz²`. The executable compares the planar distance with the previous
/// best 3D distance, then stores the new 3D distance; consequently, later
/// edges can replace earlier ones even when farther away. On success stores
/// the triangle at `out_triangle` (`u16`) and edge at `out_edge`.
pub fn fn_00697010(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    radius: f32,
    out_triangle: Ptr,
    out_edge: Ptr,
    flip: bool,
) -> bool {
    let mut best_triangle = 0u16;
    let mut best_edge = -1i32;
    let mut best: f32 = e.global(FLOAT_MAX);
    let radius_squared = (radius as f64 * radius as f64) as f32;
    let point_z = e.mem.f32(point.addr() + 8) as f64;
    let sixty_four: f64 = e.global(SIXTY_FOUR);
    let one_eighty: f64 = e.global(ONE_EIGHTY);
    let (mut low, mut high) = ((point_z - sixty_four) as f32, (point_z + one_eighty) as f32);
    if flip {
        low = (point_z - one_eighty) as f32;
        high = (point_z + sixty_four) as f32;
    }
    let half: f32 = e.global(HALF);
    let count = array_size(e, mesh_triangles(this));
    // 0: the edge's vertex pair, 8: the vertex sum, 20: the midpoint,
    // 32: the midpoint minus the point.
    e.with_stack(48, |e, scratch| {
        let pair = scratch;
        let sum = scratch.byte_add(8);
        let midpoint = scratch.byte_add(20);
        let difference = scratch.byte_add(32);
        let mut triangle = 0u32;
        while triangle < count {
            let tri = triangle_of(e, this, triangle);
            let mut edge = 0i32;
            while edge < 3 {
                if e.call(EDGE_HAS_NEIGHBOUR, &args![tri, edge]).bool() {
                    edge += 1;
                    continue;
                }
                e.call(GET_EDGE_VERTICES, &args![this, pair, triangle as u16, edge]);
                let first = e.mem.u32(pair.addr());
                let second = e.mem.u32(pair.addr() + 4);
                let total = e.call(POINT3_ADD, &args![first, sum, second]).ptr::<()>();
                e.call(POINT3_SCALE, &args![total, midpoint, half]);
                e.call(POINT3_SUBTRACT, &args![midpoint, difference, point]);
                let planar = e.call(POINT3_SQUARED_LENGTH_2D, &args![difference]).f32();
                let vertical = e.mem.f32(difference.addr() + 8);
                let vertical_squared = (vertical as f64 * vertical as f64) as f32;
                let total_squared = (planar as f64 + vertical_squared as f64) as f32;
                let midpoint_z = e.mem.f32(midpoint.addr() + 8);
                if best > planar
                    && ((high > midpoint_z && low < midpoint_z)
                        || total_squared as f64 <= radius_squared as f64 - vertical_squared as f64)
                {
                    best_triangle = triangle as u16;
                    best_edge = edge;
                    best = total_squared;
                }
                edge += 1;
            }
            triangle += 1;
        }
    });
    if best_edge == -1 {
        return false;
    }
    e.mem.set_u16(out_triangle.addr(), best_triangle);
    e.mem.set_i32(out_edge.addr(), best_edge);
    true
}

// Translated from 006971e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The unit normal of triangle `triangle`: `UnitCross` (`0053d1a0`) of the
/// edges `v1 - v0` and `v2 - v0` into the `NiPoint3` at `result`. Returns
/// `result`.
pub fn fn_006971e0(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr, triangle: u16) -> Ptr {
    let tri = triangle_of(e, this, triangle as u32);
    e.with_stack(24, |e, scratch| {
        let first_edge = scratch;
        let second_edge = scratch.byte_add(12);
        let first = corner_pointer(e, this, tri, 0);
        let second = corner_pointer(e, this, tri, 1);
        e.call(POINT3_SUBTRACT, &args![second, first_edge, first]);
        let first = corner_pointer(e, this, tri, 0);
        let third = corner_pointer(e, this, tri, 2);
        e.call(POINT3_SUBTRACT, &args![third, second_edge, first]);
        e.call(POINT3_UNIT_CROSS, &args![first_edge, result, second_edge]);
    });
    result
}

// Translated from 00697290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The circumcenter of triangle `triangle` in x and y, with the largest z of
/// its three vertices, stored at `result`. Returns `result`. (With the
/// edge vectors `a` and `b` from vertex 0, the center is `v0 + 0.5 *
/// (b.y |a|² - a.y |b|², a.x |b|² - b.x |a|²) / (a.x b.y - b.x a.y)`.)
pub fn fn_00697290(e: &mut Engine, this: Ptr<NavMesh>, result: Ptr, triangle: u16) -> Ptr {
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let tri = triangle_of(e, this, triangle as u32);
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let [v0, v1, v2] = vertices;
    let (ax, ay) = (
        (v1[0] as f64 - v0[0] as f64) as f32,
        (v1[1] as f64 - v0[1] as f64) as f32,
    );
    let (bx, by) = (
        (v2[0] as f64 - v0[0] as f64) as f32,
        (v2[1] as f64 - v0[1] as f64) as f32,
    );
    let a_squared = (ax as f64 * ax as f64 + ay as f64 * ay as f64) as f32;
    let b_squared = (bx as f64 * bx as f64 + by as f64 * by as f64) as f32;
    let determinant = (ax as f64 * by as f64 - bx as f64 * ay as f64) as f32;
    let reciprocal_half = {
        let half: f64 = e.global(DOUBLE_HALF);
        (half / determinant as f64) as f32
    };
    let mut z = v2[2];
    if v1[2] <= v0[2] && v2[2] <= v0[2] {
        z = v0[2];
    } else if v2[2] <= v1[2] {
        z = v1[2];
    }
    let y = (((ax as f64 * b_squared as f64 - bx as f64 * a_squared as f64)
        * reciprocal_half as f64)
        + v0[1] as f64) as f32;
    let x = (v0[0] as f64
        - ((ay as f64 * b_squared as f64 - by as f64 * a_squared as f64) * reciprocal_half as f64))
        as f32;
    e.with_stack(12, |e, center| {
        e.call(POINT3_CONSTRUCT, &args![center, x, y, z]);
        let value = read_point(e, center.addr());
        write_point(e, result.addr(), value);
    });
    result
}

// Translated from 00697440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The circumcenter of triangle `triangle` in x and y (as `00697290`), with
/// z = 0, stored at the three floats at `result`.
pub fn fn_00697440(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, result: Ptr) {
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let tri = triangle_of(e, this, triangle as u32);
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let [v0, v1, v2] = vertices;
    let (ax, ay) = (
        (v1[0] as f64 - v0[0] as f64) as f32,
        (v1[1] as f64 - v0[1] as f64) as f32,
    );
    let (bx, by) = (
        (v2[0] as f64 - v0[0] as f64) as f32,
        (v2[1] as f64 - v0[1] as f64) as f32,
    );
    let a_squared = (ax as f64 * ax as f64 + ay as f64 * ay as f64) as f32;
    let b_squared = (bx as f64 * bx as f64 + by as f64 * by as f64) as f32;
    let determinant = (ax as f64 * by as f64 - bx as f64 * ay as f64) as f32;
    let half: f64 = e.global(DOUBLE_HALF);
    let reciprocal_half = (half / determinant as f64) as f32;
    let x = (v0[0] as f64
        - ((ay as f64 * b_squared as f64 - by as f64 * a_squared as f64) * reciprocal_half as f64))
        as f32;
    let y = (((ax as f64 * b_squared as f64 - bx as f64 * a_squared as f64)
        * reciprocal_half as f64)
        + v0[1] as f64) as f32;
    e.mem.set_f32(result.addr(), x);
    e.mem.set_f32(result.addr() + 4, y);
    e.mem.set_f32(result.addr() + 8, 0.0);
}

// Translated from 00697590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindPortalTriangles` (Xbox PDB): fills the word array `result`
/// (cleared and reserved for 100 entries) with the index of every triangle
/// that has an edge whose extra info leads to the `NavMeshInfo` `info`
/// (each triangle once). True when the array is not empty.
pub fn nav_mesh_find_portal_triangles(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    info: Ptr,
    result: Ptr,
) -> bool {
    e.call(ARRAY_SET_SIZE, &args![result, 0u32, 1u32]);
    e.call(ARRAY_SET_RESERVED_SIZE, &args![result, 100u32]);
    let mut triangle = 0u32;
    while triangle < array_size(e, mesh_triangles(this)) {
        let tri = triangle_of(e, this, triangle);
        if e.call(ANY_EDGE_HAS_INFO, &args![tri]).bool() {
            let mut edge = 0u32;
            while (edge as i32) < 3 {
                if e.call(EDGE_HAS_INFO, &args![tri, edge]).bool() {
                    let slot = e.call(EDGE_SLOT, &args![tri, edge]).u16();
                    let edge_info = e.call(GET_EXTRA_EDGE_INFO, &args![this, slot]).u32();
                    if e.mem.u32(edge_info + EDGE_INFO_OTHER_MESH) == info.addr() {
                        e.with_stack(4, |e, entry| {
                            e.mem.set_u32(entry.addr(), triangle);
                            e.call(ARRAY_ADD_DWORD, &args![result, entry]);
                        });
                        break;
                    }
                }
                edge += 1;
            }
        }
        triangle += 1;
    }
    array_size(e, result) != 0
}

// Translated from 00697670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `00696a50` finds a triangle for `point` (`second` is passed on as
/// its callback).
pub fn fn_00697670(e: &mut Engine, this: Ptr<NavMesh>, point: Ptr, second: Ptr) -> bool {
    fn_00696a50(e, this, point, second) != NO_TRIANGLE
}

// Translated from 006976a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `EdgeExtraInfo` of edge `edge` of triangle `triangle`, created when
/// the edge has none (a new info is appended to the mesh and the edge's
/// slot and flag are set to it), then reset to "no other mesh" (`pOtherMesh`
/// 0, triangle `0xFFFF`). Returns the info.
pub fn fn_006976a0(e: &mut Engine, this: Ptr<NavMesh>, triangle: u16, edge: i32) -> Ptr {
    let mut info = e
        .call(GET_EDGE_EXTRA_INFO, &args![this, triangle, edge])
        .ptr::<()>();
    if info.is_null() {
        let index = e.with_stack(12, |e, fresh| {
            e.call(EDGE_INFO_CONSTRUCTOR, &args![fresh]);
            e.call(ADD_EXTRA_EDGE_INFO, &args![this, fresh]).u16()
        });
        let tri = e
            .call(TRIANGLES_ELEMENT, &args![mesh_triangles(this), triangle])
            .ptr::<NavMeshTriangle>();
        e.call(SET_EDGE_INFO_SLOT, &args![tri, edge, index]);
        info = e
            .call(GET_EDGE_EXTRA_INFO, &args![this, triangle, edge])
            .ptr();
    }
    e.mem.set_u32(info.addr() + EDGE_INFO_OTHER_MESH, 0);
    e.mem.set_u16(info.addr() + 8, 0xffff);
    info
}

// Translated from 00697730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The ray test of a point against triangle `triangle`: gathers the
/// triangle's three vertices (`NiPoint3` copies) and calls `006b9cf0`
/// (cdecl) with them, `point`, the out float `height` and the two limits.
/// Returns its result.
pub fn fn_00697730(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    height: Ptr,
    first_limit: f32,
    second_limit: f32,
) -> bool {
    let tri = triangle_of(e, this, triangle as u32);
    e.with_stack(36, |e, scratch| {
        for corner in 0..3u32 {
            let vertex = corner_pointer(e, this, tri, corner);
            let value = read_point(e, vertex.addr());
            write_point(e, scratch.addr() + 12 * corner, value);
        }
        e.call(
            TRIANGLE_RAY_TEST,
            &args![
                scratch,
                scratch.byte_add(12),
                scratch.byte_add(24),
                point,
                height,
                first_limit,
                second_limit
            ],
        )
        .bool()
    })
}

// Translated from 00697810 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00697730` with both limits `FLT_MAX`. When `callback` is not null
/// and the ray hits, `callback`'s virtual slot 0 is called with `point`,
/// the mesh, the triangle and the hit point (`point` with its z raised by
/// the height) and its result is returned; the plain hit result otherwise.
pub fn fn_00697810(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    height: Ptr,
    callback: Ptr,
) -> bool {
    let limit: f32 = e.global(FLOAT_MAX);
    if callback.is_null() {
        return fn_00697730(e, this, triangle, point, height, limit, limit);
    }
    let hit = fn_00697730(e, this, triangle, point, height, limit, limit);
    if !hit {
        return false;
    }
    let mut hit_point = read_point(e, point.addr());
    hit_point[2] = (hit_point[2] as f64 + e.mem.f32(height.addr()) as f64) as f32;
    e.with_stack(12, |e, slot| {
        write_point(e, slot.addr(), hit_point);
        e.vcall(
            callback.addr(),
            0,
            &args![point, this, triangle as u32, slot],
        )
        .bool()
    })
}

// Translated from 00697980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::ComputeTriangleZForLocation` (Xbox PDB): whether the point at
/// `point` (x, y, z) lies over triangle `triangle` (barycentric weights all
/// not negative, a flat triangle never) and within the vertical tolerances:
/// the triangle's height at the point is stored at `height_out`, and the
/// point may be at most `above` over and `below` under it.
pub fn nav_mesh_compute_triangle_z_for_location(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    point: Ptr,
    above: f32,
    below: f32,
    height_out: Ptr,
) -> bool {
    let tri = triangle_of(e, this, triangle as u32);
    let a = corner_pointer(e, this, tri, 0);
    let b = corner_pointer(e, this, tri, 1);
    let c = corner_pointer(e, this, tri, 2);
    let (a, b, c) = (
        read_point(e, a.addr()),
        read_point(e, b.addr()),
        read_point(e, c.addr()),
    );
    let p = read_point(e, point.addr());
    let sub = |x: f32, y: f32| x as f64 - y as f64;
    let determinant =
        ((sub(b[0], a[0]) * sub(c[1], a[1])) - (sub(c[0], a[0]) * sub(b[1], a[1]))) as f32;
    let zero: f64 = e.global(ZERO);
    if determinant as f64 == zero {
        return false;
    }
    let b_x = (b[0] as f64 - p[0] as f64) as f32;
    let c_x = (c[0] as f64 - p[0] as f64) as f32;
    let b_y = (b[1] as f64 - p[1] as f64) as f32;
    let c_y = (c[1] as f64 - p[1] as f64) as f32;
    let reciprocal = (1.0 / determinant as f64) as f32;
    let weight_a =
        (((b_x as f64 * c_y as f64) - (c_x as f64 * b_y as f64)) * reciprocal as f64) as f32;
    if (weight_a as f64) < zero {
        return false;
    }
    let a_x = (a[0] as f64 - p[0] as f64) as f32;
    let a_y = (a[1] as f64 - p[1] as f64) as f32;
    let weight_b =
        (((c_x as f64 * a_y as f64) - (a_x as f64 * c_y as f64)) * reciprocal as f64) as f32;
    if (weight_b as f64) < zero {
        return false;
    }
    let weight_c =
        (((a_x as f64 * b_y as f64) - (b_x as f64 * a_y as f64)) * reciprocal as f64) as f32;
    if (weight_c as f64) < zero {
        return false;
    }
    let height = (weight_a as f64 * a[2] as f64
        + weight_b as f64 * b[2] as f64
        + weight_c as f64 * c[2] as f64) as f32;
    e.mem.set_f32(height_out.addr(), height);
    let difference = (p[2] as f64 - height as f64) as f32;
    // `above < difference` fails; `difference < -below` fails.
    if above < difference {
        return false;
    }
    if (difference as f64) < -(below as f64) {
        return false;
    }
    true
}

// Translated from 00697b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the point at `point` lies over any triangle of its grid cell
/// (`ComputeTriangleZForLocation` with `above` and `below`; the first
/// triangle that does stores its height at `height_out`). False when the
/// grid has no list for the point.
pub fn fn_00697b70(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    above: f32,
    below: f32,
    height_out: Ptr,
) -> bool {
    e.with_stack(4, |e, list_slot| {
        e.call(GRID_CELL_LIST, &args![mesh_grid(this), point, list_slot]);
        let list = e.mem.u32(list_slot.addr());
        if list == 0 {
            return false;
        }
        let count = array_size(e, Ptr::<()>::new(list));
        let mut index = 0u32;
        while index < count {
            let slot = e
                .call(ARRAY_ELEMENT_2, &args![Ptr::<()>::new(list), index])
                .u32();
            let triangle = e.mem.u16(slot);
            if nav_mesh_compute_triangle_z_for_location(
                e, this, triangle, point, above, below, height_out,
            ) {
                return true;
            }
            index += 1;
        }
        false
    })
}

// Translated from 00697c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random walkable point near `point`: up to 8 times draws a point in a
/// disc of radius `radius` (x uniform in `[-r, r]`, y uniform in the chord at
/// that x, z from the static vector `011f426c`) and keeps the first whose
/// position (added to `point`) is over a triangle (`00697670`); then adds
/// `point` to it, pushes it away from nearby border edges (`00697d40`, 64
/// units) and stores it at `result`. False (nothing stored) if all 8 draws
/// miss.
pub fn fn_00697c10(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    point: Ptr,
    radius: f32,
    result: Ptr,
) -> bool {
    e.with_stack(48, |e, scratch| {
        // 0: the candidate offset, 12: candidate + point, 24: the pushed
        // point.
        let offset = scratch;
        let moved = scratch.byte_add(12);
        let pushed = scratch.byte_add(24);
        for word in 0..3 {
            let value = e.mem.u32(RANDOM_POINT_BASE + 4 * word);
            e.mem.set_u32(offset.addr() + 4 * word, value);
        }
        let mut found = false;
        let mut attempt = 0u32;
        while attempt < 8 && !found {
            let x = e.call(RANDOM_FLOAT, &args![-radius, radius]).f32();
            let remaining = ((radius as f64 * radius as f64) - (x as f64 * x as f64)) as f32;
            let half_chord = e.call(FLOAT_SQRT, &args![remaining]).f32();
            let y = e.call(RANDOM_FLOAT, &args![-half_chord, half_chord]).f32();
            e.mem.set_f32(offset.addr(), x);
            e.mem.set_f32(offset.addr() + 4, y);
            let candidate = e.call(POINT3_ADD, &args![offset, moved, point]).ptr::<()>();
            found = fn_00697670(e, this, candidate, Ptr::NULL);
            attempt += 1;
        }
        if !found {
            return false;
        }
        e.call(POINT3_ADD_ASSIGN, &args![offset, point]);
        let limit: f32 = e.global(HEIGHT_DOWN);
        let adjusted = fn_00697d40(e, this, pushed, offset, limit);
        let value = read_point(e, adjusted.addr());
        write_point(e, result.addr(), value);
        true
    })
}

// Translated from 00697d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pushes the point at `position` away from the border edges of the mesh
/// near it: finds the triangle under the position (`00696a50`); for each
/// of its edges that has no neighbour, and for each border edge of its
/// neighbours, whose distance to the position (`006b9940`) is less than
/// `distance`, moves the result by the triangle's unit normal crossed with
/// the edge vector, normalized and scaled by `distance - edge distance`.
/// The result (the position when no triangle is found) is stored at
/// `result`; returns `result`.
pub fn fn_00697d40(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    position: Ptr,
    distance: f32,
) -> Ptr {
    let mut moved = read_point(e, position.addr());
    let found = fn_00696a50(e, this, position, Ptr::NULL);
    if found != NO_TRIANGLE {
        // 0: moved point, 12: normal, 24: edge vector, 36: cross product,
        // 48: scaled push, 64: the edge's vertex pair.
        e.with_stack(80, |e, scratch| {
            let normal = scratch.byte_add(12);
            let edge_vector = scratch.byte_add(24);
            let cross = scratch.byte_add(36);
            let push = scratch.byte_add(48);
            let pair = scratch.byte_add(64);
            let mut neighbours = [0u16; 3];
            let mut neighbour_count = 0usize;
            let tri = triangle_of(e, this, found as u32);
            let apply = |e: &mut Engine, triangle: u16, edge: i32, moved: &mut [f32; 3]| {
                e.call(GET_EDGE_VERTICES, &args![this, pair, triangle, edge]);
                let first = e.mem.u32(pair.addr());
                let second = e.mem.u32(pair.addr() + 4);
                let edge_distance = e
                    .call(POINT_SEGMENT_DISTANCE, &args![position, first, second])
                    .f32();
                if distance > edge_distance {
                    fn_006971e0(e, this, normal, triangle);
                    let difference = e
                        .call(POINT3_SUBTRACT, &args![first, edge_vector, second])
                        .ptr::<()>();
                    e.call(POINT3_CROSS, &args![normal, cross, difference]);
                    e.call(POINT3_NORMALIZE, &args![cross]);
                    let strength = (distance as f64 - edge_distance as f64) as f32;
                    let scaled = e
                        .call(POINT3_SCALE, &args![cross, push, strength])
                        .ptr::<()>();
                    write_point(e, scratch.addr(), *moved);
                    e.call(POINT3_ADD_ASSIGN, &args![scratch, scaled]);
                    *moved = read_point(e, scratch.addr());
                }
            };
            for edge in 0..3i32 {
                let mut neighbour = NO_TRIANGLE;
                if !e.call(EDGE_HAS_INFO, &args![tri, edge]).bool() {
                    neighbour = e.call(EDGE_SLOT, &args![tri, edge]).u16();
                }
                if neighbour != NO_TRIANGLE {
                    neighbours[neighbour_count] = neighbour;
                    neighbour_count += 1;
                } else {
                    apply(e, found, edge, &mut moved);
                }
            }
            for &neighbour in &neighbours[..neighbour_count] {
                let neighbour_tri = triangle_of(e, this, neighbour as u32);
                for edge in 0..3i32 {
                    if !e.call(EDGE_HAS_INFO, &args![neighbour_tri, edge]).bool()
                        && e.call(EDGE_SLOT, &args![neighbour_tri, edge]).u16() == NO_TRIANGLE
                    {
                        apply(e, neighbour, edge, &mut moved);
                    }
                }
            }
        });
    }
    write_point(e, result.addr(), moved);
    result
}

// Translated from 00698030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMeshTriangle` method: the four packed bits of edge `edge` (bits
/// `16 + 6 * edge` upward of the flags).
pub fn fn_00698030(e: &mut Engine, this: Ptr<NavMeshTriangle>, edge: u16) -> u32 {
    let shift = (edge as u32).wrapping_mul(6).wrapping_add(0x10);
    (e.get(this, NavMeshTriangle::TriangleFlags) >> (shift & 31)) & 0xf
}

// Translated from 00698060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first edge of triangle `triangle`, other than `skip`, for which the
/// 2D segment test `006b9e90` of the edge's two vertices (x and y only)
/// with the three words `first`, `second` and `third` succeeds; -1 if none.
pub fn fn_00698060(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    skip: i32,
    first: u32,
    second: u32,
    third: u32,
) -> i32 {
    e.with_stack(24, |e, points| {
        for corner in 0..3u32 {
            let tri = triangle_of(e, this, triangle as u32);
            let vertex = corner_pointer(e, this, tri, corner);
            let x = e.mem.u32(vertex.addr());
            let y = e.mem.u32(vertex.addr() + 4);
            e.mem.set_u32(points.addr() + 8 * corner, x);
            e.mem.set_u32(points.addr() + 8 * corner + 4, y);
        }
        for edge in 0..3i32 {
            if edge == skip {
                continue;
            }
            let next = if edge + 1 == 3 { 0 } else { edge + 1 };
            let hit = e
                .call(
                    SEGMENT_SEGMENT_TEST,
                    &args![
                        points.byte_add(8 * edge as u32),
                        points.byte_add(8 * next as u32),
                        first,
                        second,
                        third
                    ],
                )
                .bool();
            if hit {
                return edge;
            }
        }
        -1
    })
}

// Translated from 00698160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NavMesh::FindInterectingEdge2` (Xbox PDB; the typo is the game's): the
/// first edge of triangle `triangle`, other than `skip` and other than a
/// degenerate (zero length) one, that the 2D segment test `006ba240`
/// (`first`, `point`) crosses. Writes the 20-byte result at `result`:
/// `{3, edge, x, y, z}` with the intersection point, or `{0, -1, ..}`
/// without a crossing (the three floats are not written then: the game
/// leaves uninitialized stack there). Returns `result`.
pub fn nav_mesh_find_interecting_edge2(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    result: Ptr,
    triangle: u16,
    skip: i32,
    first: u32,
    point: Ptr,
) -> Ptr {
    e.with_stack(48, |e, scratch| {
        let points = scratch;
        let intersection = scratch.byte_add(36);
        for corner in 0..3u32 {
            let tri = triangle_of(e, this, triangle as u32);
            let vertex = corner_pointer(e, this, tri, corner);
            let value = read_point(e, vertex.addr());
            write_point(e, points.addr() + 12 * corner, value);
        }
        for edge in 0..3i32 {
            if edge == skip {
                continue;
            }
            let next = if edge + 1 == 3 { 0 } else { edge + 1 };
            let from = points.byte_add(12 * edge as u32);
            let to = points.byte_add(12 * next as u32);
            if e.call(POINT3_EQUALS, &args![from, to]).bool() {
                continue;
            }
            let hit = e
                .call(
                    SEGMENT_INTERSECTION_2D,
                    &args![from, to, first, point, intersection],
                )
                .bool();
            if hit {
                e.mem.set_u32(result.addr(), 3);
                e.mem.set_i32(result.addr() + 4, edge);
                for word in 0..3 {
                    let value = e.mem.u32(intersection.addr() + 4 * word);
                    e.mem.set_u32(result.addr() + 8 + 4 * word, value);
                }
                return;
            }
        }
        e.mem.set_u32(result.addr(), 0);
        e.mem.set_i32(result.addr() + 4, -1);
    });
    result
}

// Translated from 00698300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 20-byte intersection result: default-constructs the
/// `NiPoint3` at +8 (`006815c0`). Returns `this`.
pub fn fn_00698300(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(POINT3_DEFAULT_CONSTRUCTOR, &args![this.byte_add(8)]);
    this
}

// Translated from 00698320 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first edge of triangle `triangle`, other than `skip`, that the
/// segment from `first` to `second` (x and y of two points) properly
/// crosses: the triangle edge's end points are on opposite sides (or
/// touching, but the first one off) of the segment's line and the segment's
/// end points are on opposite sides (or touching, but the second one off)
/// of the edge's line. -1 if there is none.
pub fn fn_00698320(
    e: &mut Engine,
    this: Ptr<NavMesh>,
    triangle: u16,
    skip: i32,
    first: Ptr,
    second: Ptr,
) -> i32 {
    let tri = triangle_of(e, this, triangle as u32);
    let mut vertices = [[0f32; 3]; 3];
    for (corner, slot) in vertices.iter_mut().enumerate() {
        let vertex = corner_pointer(e, this, tri, corner as u32);
        *slot = read_point(e, vertex.addr());
    }
    let p = read_point(e, first.addr());
    let q = read_point(e, second.addr());
    let zero: f64 = e.global(ZERO);
    for edge in 0..3i32 {
        if edge == skip {
            continue;
        }
        let next = if edge + 1 == 3 { 0 } else { edge + 1 };
        let a = vertices[edge as usize];
        let b = vertices[next as usize];
        let cross = |u: [f32; 3], v: [f32; 3], w: [f32; 3]| -> f32 {
            // (u.x - w.x) * (v.y - w.y) - (u.y - w.y) * (v.x - w.x)
            let first = (u[0] as f64 - w[0] as f64) * (v[1] as f64 - w[1] as f64);
            let second = (u[1] as f64 - w[1] as f64) * (v[0] as f64 - w[0] as f64);
            (first - second) as f32
        };
        let q_side = cross(a, b, q);
        let p_side = cross(a, b, p);
        let a_side = cross(p, q, a);
        let b_side = cross(p, q, b);
        // `JP` after `TEST AH, 0x41` on `st0 <=> 0.0`: taken (edge rejected)
        // when the value is greater than zero, or unordered.
        let rejected = |value: f64| {
            let flags = compare_flags(value, zero);
            parity_even(flags, 0x41)
        };
        if rejected(q_side as f64 * p_side as f64) || rejected(a_side as f64 * b_side as f64) {
            continue;
        }
        if q_side as f64 == zero || b_side as f64 == zero {
            continue;
        }
        return edge;
    }
    -1
}

// Translated from 00698580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a two-point segment: copies the three floats at `first`
/// to `this + 0` and those at `second` to `this + 0xC`. Returns `this`.
pub fn fn_00698580(e: &mut Engine, this: Ptr, first: Ptr, second: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(first.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 4 * word, value);
    }
    for word in 0..3 {
        let value = e.mem.u32(second.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 12 + 4 * word, value);
    }
    this
}

// Translated from 006985c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the object holding two 16-byte elements at +4: runs
/// `_vector_constructor_iterator_` over them with the element constructor
/// `006d5320`. Returns `this`.
pub fn fn_006985c0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![this.byte_add(4), 0x10u32, 2u32, ELEMENT_16_CONSTRUCTOR],
    );
    this
}

// Translated from 006985f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` copy: copies the two floats at `other` to `this`. Returns
/// `this`.
pub fn fn_006985f0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let x = e.mem.f32(other.addr());
    let y = e.mem.f32(other.addr() + 4);
    e.mem.set_f32(this.addr(), x);
    e.mem.set_f32(this.addr() + 4, y);
    this
}

// Translated from 00698620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPoint2` product of `this` and `other` as complex numbers conjugated:
/// `result = (this.x * other.x + this.y * other.y,
/// this.y * other.x - this.x * other.y)`. Returns `result`.
pub fn fn_00698620(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let (ax, ay) = (e.mem.f32(this.addr()), e.mem.f32(this.addr() + 4));
    let (bx, by) = (e.mem.f32(other.addr()), e.mem.f32(other.addr() + 4));
    let x = (ax as f64 * bx as f64 + ay as f64 * by as f64) as f32;
    let y = (ay as f64 * bx as f64 - ax as f64 * by as f64) as f32;
    e.with_stack(8, |e, product| {
        e.mem.set_f32(product.addr(), x);
        e.mem.set_f32(product.addr() + 4, y);
        fn_006985f0(e, result, product);
    });
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006953b0, nav_mesh_fix_portals_from_nav_mesh_deletion(Ptr)),
        entry!(0x00695660, fn_00695660(Ptr<NavMesh>, u16)),
        entry!(0x006957d0, fn_006957d0(Ptr<NavMeshTriangle>) -> bool),
        entry!(0x006957f0, fn_006957f0(Ptr<NavMesh>, Ptr, Ptr)),
        entry!(0x006959b0, fn_006959b0(Ptr<NavMesh>)),
        entry!(0x00696500, fn_00696500(Ptr<NavMeshGrid>, u32, u32, u32)),
        entry!(0x00696540, fn_00696540(Ptr, Ptr, Ptr, Ptr) -> bool),
        entry!(0x006965d0, fn_006965d0(Ptr<NavMesh>, u16, u16) -> bool),
        entry!(0x006967c0, fn_006967c0(Ptr<NavMeshGrid>, u32, u32) -> Ptr),
        entry!(0x006967f0, fn_006967f0(Ptr<NavMesh>, u16, u16) -> bool),
        entry!(0x006969e0, fn_006969e0(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x00696a10, fn_00696a10(Ptr<NavMeshGrid>, Ptr, Ptr)),
        entry!(0x00696a50, fn_00696a50(Ptr<NavMesh>, Ptr, Ptr) -> u16),
        entry!(0x00696c00, fn_00696c00(Ptr<NavMesh>, u32, u32, u32)),
        entry!(0x00696c30, fn_00696c30(Ptr, Ptr) -> Ptr),
        entry!(0x00696c60, fn_00696c60(Ptr, Ptr) -> Ptr),
        entry!(0x00696ca0, fn_00696ca0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00696cf0,
            nav_mesh_find_closest_triangle_for_location(Ptr<NavMesh>, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00697010,
            fn_00697010(Ptr<NavMesh>, Ptr, f32, Ptr, Ptr, bool) -> bool
        ),
        entry!(0x006971e0, fn_006971e0(Ptr<NavMesh>, Ptr, u16) -> Ptr),
        entry!(0x00697290, fn_00697290(Ptr<NavMesh>, Ptr, u16) -> Ptr),
        entry!(0x00697440, fn_00697440(Ptr<NavMesh>, u16, Ptr)),
        entry!(
            0x00697590,
            nav_mesh_find_portal_triangles(Ptr<NavMesh>, Ptr, Ptr) -> bool
        ),
        entry!(0x00697670, fn_00697670(Ptr<NavMesh>, Ptr, Ptr) -> bool),
        entry!(0x006976a0, fn_006976a0(Ptr<NavMesh>, u16, i32) -> Ptr),
        entry!(
            0x00697730,
            fn_00697730(Ptr<NavMesh>, u16, Ptr, Ptr, f32, f32) -> bool
        ),
        entry!(
            0x00697810,
            fn_00697810(Ptr<NavMesh>, u16, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(
            0x00697980,
            nav_mesh_compute_triangle_z_for_location(Ptr<NavMesh>, u16, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(
            0x00697b70,
            fn_00697b70(Ptr<NavMesh>, Ptr, f32, f32, Ptr) -> bool
        ),
        entry!(0x00697c10, fn_00697c10(Ptr<NavMesh>, Ptr, f32, Ptr) -> bool),
        entry!(0x00697d40, fn_00697d40(Ptr<NavMesh>, Ptr, Ptr, f32) -> Ptr),
        entry!(0x00698030, fn_00698030(Ptr<NavMeshTriangle>, u16) -> u32),
        entry!(
            0x00698060,
            fn_00698060(Ptr<NavMesh>, u16, i32, u32, u32, u32) -> i32
        ),
        entry!(
            0x00698160,
            nav_mesh_find_interecting_edge2(Ptr<NavMesh>, Ptr, u16, i32, u32, Ptr) -> Ptr
        ),
        entry!(0x00698300, fn_00698300(Ptr) -> Ptr),
        entry!(
            0x00698320,
            fn_00698320(Ptr<NavMesh>, u16, i32, Ptr, Ptr) -> i32
        ),
        entry!(0x00698580, fn_00698580(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x006985c0, fn_006985c0(Ptr) -> Ptr),
        entry!(0x006985f0, fn_006985f0(Ptr, Ptr) -> Ptr),
        entry!(0x00698620, fn_00698620(Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn ret_float(value: f32) -> Ret {
        Ret {
            st0: value as f64,
            ..Ret::default()
        }
    }

    fn arg_f32(word: u32) -> f32 {
        f32::from_bits(word)
    }

    /// A double that records its arguments and returns `result`.
    fn recorder(e: &mut Engine, address: u32, result: u32) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(address, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(result)
        });
        calls
    }

    fn read3(e: &Engine, address: u32) -> [f32; 3] {
        read_point(e, address)
    }

    /// An engine with the exe data this file reads mapped and set, and
    /// doubles for the small getters and the `NiPoint3` arithmetic that
    /// behave like the game's (on the real memory layout).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [
            FLOAT_MAX,
            NEGATIVE_FLOAT_MAX,
            HEIGHT_UP,
            HEIGHT_DOWN,
            HALF,
            ZERO,
            GROUND_MARGIN,
            NEGATIVE_180,
            SIXTY_FOUR,
            FORTY,
            ONE_EIGHTY,
            DOUBLE_HALF,
            RANDOM_POINT_BASE,
            TES_INSTANCE,
        ] {
            e.map(address & !0xfff, 0x1000);
        }
        e.set_global(FLOAT_MAX, f32::MAX);
        e.set_global(NEGATIVE_FLOAT_MAX, -f32::MAX);
        e.set_global(HEIGHT_UP, 180.0f32);
        e.set_global(HEIGHT_DOWN, 64.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(ZERO, 0.0f64);
        e.set_global(GROUND_MARGIN, 50.0f64);
        e.set_global(NEGATIVE_180, -180.0f64);
        e.set_global(SIXTY_FOUR, 64.0f64);
        e.set_global(FORTY, 40.0f64);
        e.set_global(ONE_EIGHTY, 180.0f64);
        e.set_global(DOUBLE_HALF, 0.5f64);

        // BSSimpleArray (buffer +4, size +8) and its element getters.
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(
            ARRAY_IS_EMPTY,
            |e, a| ret((e.mem.u32(a[0] + 8) == 0) as u32),
        );
        e.register(ARRAY_ELEMENT_4, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        e.register(ARRAY_ELEMENT_2, |e, a| ret(e.mem.u32(a[0] + 4) + 2 * a[1]));
        e.register(TRIANGLES_ELEMENT, |e, a| {
            ret(e.mem.u32(a[0] + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        // NavMesh getters: triangles at +0x38, vertices at +0x28.
        e.register(TRIANGLE_COUNT, |e, a| ret(e.mem.u32(a[0] + 0x38 + 8)));
        e.register(GET_TRIANGLE, |e, a| {
            ret(e.mem.u32(a[0] + 0x38 + 4) + (a[1] & 0xffff) * 16)
        });
        e.register(VERTEX_POINTER, |e, a| {
            ret(e.mem.u32(a[0] + 0x28 + 4) + (a[1] & 0xffff) * 12)
        });
        // NavMeshTriangle getters and setters.
        e.register(TRIANGLE_VERTEX, |e, a| {
            ret(e.mem.u16(a[0] + 2 * a[1]) as u32)
        });
        e.register(EDGE_HAS_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 12) >> (a[1] & 31)) & 1)
        });
        e.register(EDGE_HAS_NEIGHBOUR, |e, a| {
            ret((e.mem.u16(a[0] + 6 + 2 * a[1]) != 0xffff) as u32)
        });
        e.register(EDGE_SLOT, |e, a| ret(e.mem.u16(a[0] + 6 + 2 * a[1]) as u32));
        e.register(ANY_EDGE_HAS_INFO, |e, a| {
            ret((e.mem.u32(a[0] + 12) & 7 != 0) as u32)
        });
        e.register(TRIANGLE_FLAGS_SET, |e, a| {
            ret((e.mem.u32(a[0] + 12) & a[1] != 0) as u32)
        });
        e.register(SET_VERTEX, |e, a| {
            e.mem.set_u16(a[0] + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(SET_EDGE_INFO_SLOT, |e, a| {
            let flags = e.mem.u32(a[0] + 12);
            e.mem.set_u32(a[0] + 12, flags | 1 << (a[1] & 31));
            e.mem.set_u16(a[0] + 6 + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(SET_EDGE_NEIGHBOUR, |e, a| {
            let flags = e.mem.u32(a[0] + 12);
            e.mem.set_u32(a[0] + 12, flags & !(1 << (a[1] & 31)));
            e.mem.set_u16(a[0] + 6 + 2 * a[1], a[2] as u16);
            Ret::default()
        });
        e.register(FLOAT_ABS, |_, a| ret_float(arg_f32(a[0]).abs()));
        e.register(FLOAT_FLOOR, |_, a| ret_float(arg_f32(a[0]).floor()));
        // NiPoint3 and NiPoint2 arithmetic.
        e.register(POINT3_CONSTRUCT, |e, a| {
            write_point(e, a[0], [arg_f32(a[1]), arg_f32(a[2]), arg_f32(a[3])]);
            ret(a[0])
        });
        e.register(POINT2_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            e.mem.set_u32(a[0] + 4, a[2]);
            ret(a[0])
        });
        e.register(POINT3_ADD, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[1])
        });
        e.register(POINT3_SUBTRACT, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], [p[0] - q[0], p[1] - q[1], p[2] - q[2]]);
            ret(a[1])
        });
        e.register(POINT3_SCALE, |e, a| {
            let (p, s) = (read3(e, a[0]), arg_f32(a[2]));
            write_point(e, a[1], [p[0] * s, p[1] * s, p[2] * s]);
            ret(a[1])
        });
        e.register(POINT3_ADD_ASSIGN, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[1]));
            write_point(e, a[0], [p[0] + q[0], p[1] + q[1], p[2] + q[2]]);
            ret(a[0])
        });
        e.register(POINT3_CROSS, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            write_point(e, a[1], cross3(p, q));
            ret(a[1])
        });
        e.register(POINT3_UNIT_CROSS, |e, a| {
            let (p, q) = (read3(e, a[0]), read3(e, a[2]));
            let c = cross3(p, q);
            let length = (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt();
            write_point(e, a[1], [c[0] / length, c[1] / length, c[2] / length]);
            ret(a[1])
        });
        e.register(POINT3_NORMALIZE, |e, a| {
            let p = read3(e, a[0]);
            let length = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
            write_point(e, a[0], [p[0] / length, p[1] / length, p[2] / length]);
            Ret::default()
        });
        e.register(POINT3_SQUARED_LENGTH, |e, a| {
            let p = read3(e, a[0]);
            ret_float(p[0] * p[0] + p[1] * p[1] + p[2] * p[2])
        });
        e.register(POINT3_SQUARED_LENGTH_2D, |e, a| {
            let p = read3(e, a[0]);
            ret_float(p[0] * p[0] + p[1] * p[1])
        });
        e.register(POINT3_EQUALS, |e, a| {
            ret((read3(e, a[0]) == read3(e, a[1])) as u32)
        });
        e
    }

    fn cross3(p: [f32; 3], q: [f32; 3]) -> [f32; 3] {
        [
            p[1] * q[2] - p[2] * q[1],
            p[2] * q[0] - p[0] * q[2],
            p[0] * q[1] - p[1] * q[0],
        ]
    }

    fn put_array(e: &mut Engine, array: Ptr, data: &[u8], count: u32) {
        let buffer = e.mem.alloc(data.len().max(8) as u32);
        e.mem.write(buffer, data);
        e.mem.set_u32(array.addr() + 4, buffer);
        e.mem.set_u32(array.addr() + 8, count);
    }

    fn words(values: &[u32]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    fn words16(values: &[u16]) -> Vec<u8> {
        values.iter().flat_map(|v| v.to_le_bytes()).collect()
    }

    type TriangleSpec = ([u16; 3], [u16; 3], u32);

    /// A mesh with the given vertices and `(vertices, neighbours, flags)`
    /// triangles.
    fn mesh_with(
        e: &mut Engine,
        vertices: &[[f32; 3]],
        triangles: &[TriangleSpec],
    ) -> Ptr<NavMesh> {
        let mesh: Ptr<NavMesh> = e.new_object();
        let vertex_bytes: Vec<u8> = vertices
            .iter()
            .flat_map(|v| v.iter().flat_map(|c| c.to_le_bytes()))
            .collect();
        put_array(
            e,
            mesh.at(NavMesh::Vertices).cast(),
            &vertex_bytes,
            vertices.len() as u32,
        );
        let mut triangle_bytes = Vec::new();
        for (v, n, f) in triangles {
            for word in v.iter().chain(n.iter()) {
                triangle_bytes.extend_from_slice(&word.to_le_bytes());
            }
            triangle_bytes.extend_from_slice(&f.to_le_bytes());
        }
        put_array(
            e,
            mesh.at(NavMesh::Triangles).cast(),
            &triangle_bytes,
            triangles.len() as u32,
        );
        mesh
    }

    fn triangle_ptr(e: &Engine, mesh: Ptr<NavMesh>, index: u32) -> Ptr<NavMeshTriangle> {
        Ptr::new(e.mem.u32(mesh.addr() + 0x38 + 4) + index * 16)
    }

    fn point(e: &mut Engine, p: [f32; 3]) -> Ptr {
        let address = e.mem.alloc(12);
        write_point(e, address, p);
        Ptr::new(address)
    }

    fn array_words(e: &Engine, array: u32) -> Vec<u32> {
        let buffer = e.mem.u32(array + 4);
        (0..e.mem.u32(array + 8))
            .map(|i| e.mem.u32(buffer + 4 * i))
            .collect()
    }

    // --- 006953b0 ----------------------------------------------------

    #[test]
    fn fix_portals_removes_the_deleted_info_everywhere() {
        let mut e = engine();
        e.map(TES_INSTANCE & !0xfff, 0x1000);
        e.set_global(TES_INSTANCE, 0x7000u32);
        let deleted = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        let other_two = e.mem.alloc(0x60);
        let info = e.mem.alloc(0x60);
        let bare = e.mem.alloc(0x60);
        let bare_two = e.mem.alloc(0x60);
        // `info` is adjacent to `deleted` twice over and has a loaded
        // navmesh whose extra edge infos lead to `deleted`, `other`,
        // `deleted`.
        let mesh = e.mem.alloc(0x108);
        let vtable = e.mem.alloc(0xd0);
        e.mem.set_u32(vtable + 0xc8, 0x0500_0000);
        e.mem.set_u32(mesh, vtable);
        let infos: Vec<u8> = [deleted, other, deleted]
            .iter()
            .flat_map(|target| words(&[0, *target, 0]))
            .collect();
        put_array(&mut e, Ptr::new(mesh + 0x48), &infos, 3);
        e.mem.set_u32(info + 0x54, mesh);
        put_array(&mut e, Ptr::new(info + 0x24), &words(&[deleted, other]), 2);
        put_array(
            &mut e,
            Ptr::new(info + 0x34),
            &words(&[other_two, deleted]),
            2,
        );
        // `bare` is not adjacent to anything that matters; a second one has
        // no navmesh loaded and is found through its form id.
        put_array(&mut e, Ptr::new(bare + 0x24), &words(&[other]), 1);
        let unloaded = bare_two;
        e.mem.set_u32(unloaded, 0x77);
        put_array(&mut e, Ptr::new(unloaded + 0x24), &words(&[deleted]), 1);
        let unloaded_mesh = e.mem.alloc(0x108);
        e.mem.set_u32(unloaded_mesh, vtable);
        put_array(
            &mut e,
            Ptr::new(unloaded_mesh + 0x48),
            &words(&[0, deleted, 0]),
            1,
        );

        let listed = [info, bare, unloaded];
        e.register(INFO_LIST_CONSTRUCTOR, |_, _| Ret::default());
        e.register(INFO_LIST_DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_NAV_MESH_INFO_MAP, |_, a| {
            assert_eq!(a, [0x7000]);
            ret(0x1234)
        });
        let buffer = e.mem.alloc(16);
        for (i, info) in listed.iter().enumerate() {
            e.mem.set_u32(buffer + 4 * i as u32, *info);
        }
        let map_calls = Rc::new(RefCell::new(Vec::new()));
        let log = map_calls.clone();
        e.register_double(INFO_MAP_COLLECT, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            e.mem.set_u32(a[1] + 4, buffer);
            e.mem.set_u32(a[1] + 8, 3);
            Ret::default()
        });
        // NiPointer<NavMesh> stand-ins: the pointer is the word at +0.
        e.register(POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(POINTER_DESTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(POINTER_DEREFERENCE, |e, a| ret(e.mem.u32(a[0])));
        e.register(INFO_GET_NAV_MESH, |e, a| {
            let mesh = e.mem.u32(a[0] + 0x54);
            e.mem.set_u32(a[1], mesh);
            ret((mesh != 0) as u32)
        });
        e.register(POINTER_CONSTRUCT_FROM_CDECL, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let lookups = recorder(&mut e, LOOKUP_FORM, 0x4000);
        let casts = recorder(&mut e, RT_DYNAMIC_CAST, unloaded_mesh);
        let slot_calls = recorder(&mut e, 0x0500_0000, 0);
        // Removal from a pointer array shifts the later entries down.
        e.register(ARRAY_REMOVE, |e, a| {
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            for i in a[1]..size - 1 {
                let next = e.mem.u32(buffer + 4 * (i + 1));
                e.mem.set_u32(buffer + 4 * i, next);
            }
            e.mem.set_u32(a[0] + 8, size - 1);
            Ret::default()
        });
        e.register(GET_EXTRA_EDGE_INFO, |e, a| {
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * a[1])
        });
        // Deleting an extra edge info moves the last one into the gap.
        let deleted_indexes = Rc::new(RefCell::new(Vec::new()));
        let log = deleted_indexes.clone();
        e.register_double(DELETE_EXTRA_EDGE_INFO, move |e, a| {
            log.borrow_mut().push((a[0], a[1]));
            let buffer = e.mem.u32(a[0] + 0x48 + 4);
            let size = e.mem.u32(a[0] + 0x48 + 8);
            for word in 0..3 {
                let last = e.mem.u32(buffer + 12 * (size - 1) + 4 * word);
                e.mem.set_u32(buffer + 12 * a[1] + 4 * word, last);
            }
            e.mem.set_u32(a[0] + 0x48 + 8, size - 1);
            Ret::default()
        });

        nav_mesh_fix_portals_from_nav_mesh_deletion(&mut e, Ptr::new(deleted));
        assert_eq!(map_calls.borrow().len(), 1);
        assert_eq!(map_calls.borrow()[0][0], 0x1234);
        // Both lists of `info` lost `deleted`; the others are untouched.
        assert_eq!(array_words(&e, info + 0x24), vec![other]);
        assert_eq!(array_words(&e, info + 0x34), vec![other_two]);
        assert_eq!(array_words(&e, bare + 0x24), vec![other]);
        assert_eq!(array_words(&e, unloaded + 0x24), Vec::<u32>::new());
        // `info`'s mesh: the extra infos to `deleted` were deleted (the
        // second one moved into the first slot and was checked again).
        assert_eq!(
            *deleted_indexes.borrow(),
            vec![(mesh, 0), (mesh, 0), (unloaded_mesh, 0)]
        );
        // One extra info is left, the one leading to `other`.
        assert_eq!(e.mem.u32(mesh + 0x48 + 8), 1);
        let buffer = e.mem.u32(mesh + 0x48 + 4);
        assert_eq!(e.mem.u32(buffer + 4), other);
        // The unloaded info's mesh was found through its form id.
        assert_eq!(*lookups.borrow(), vec![vec![0x77]]);
        assert_eq!(
            casts.borrow()[0],
            vec![0x4000, 0, TYPE_TES_FORM, TYPE_NAV_MESH, 0]
        );
        // The virtual call (slot 0xC8, argument 1) ran for both meshes.
        // (once per matching entry: `info` matches twice).
        assert_eq!(
            *slot_calls.borrow(),
            vec![vec![mesh, 1], vec![mesh, 1], vec![unloaded_mesh, 1]]
        );

        // A null info does nothing at all.
        e.call_log = Some(vec![]);
        nav_mesh_fix_portals_from_nav_mesh_deletion(&mut e, Ptr::NULL);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 0);
    }

    // --- 00695660 and 006957d0 -----------------------------------------

    fn triangle_words(e: &Engine, tri: Ptr<NavMeshTriangle>) -> ([u16; 3], [u16; 3], u32) {
        let a = tri.addr();
        (
            [e.mem.u16(a), e.mem.u16(a + 2), e.mem.u16(a + 4)],
            [e.mem.u16(a + 6), e.mem.u16(a + 8), e.mem.u16(a + 10)],
            e.mem.u32(a + 12),
        )
    }

    #[test]
    fn flip_swaps_vertices_and_the_first_two_edges() {
        let mut e = engine();
        // Edge 0 is a plain neighbour (5), edge 1 has extra info (slot 7).
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0b010)]);
        fn_00695660(&mut e, mesh, 0);
        let tri = triangle_ptr(&e, mesh, 0);
        // Edge 0 now has the info (flag set, slot 7); edge 1 the neighbour.
        assert_eq!(triangle_words(&e, tri), ([1, 0, 2], [7, 5, 9], 0b001));
    }

    #[test]
    fn flip_swaps_the_packed_edge_bits_when_present() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0x0001_0000)]);
        // Edge 0 unpacks to (3, 1, 0), edge 1 to (9, 0, 1).
        e.register(UNPACK_EDGE_BITS, |e, a| {
            let (value, flag_a, flag_b) = if a[1] == 0 { (3, 1, 0) } else { (9, 0, 1) };
            e.mem.set_u16(a[2], value);
            e.mem.set_u8(a[3], flag_a);
            e.mem.set_u8(a[4], flag_b);
            Ret::default()
        });
        let packs = recorder(&mut e, PACK_EDGE_BITS, 0);
        fn_00695660(&mut e, mesh, 0);
        let tri = triangle_ptr(&e, mesh, 0).addr();
        assert_eq!(
            *packs.borrow(),
            vec![vec![tri, 0, 9, 1, 0], vec![tri, 1, 3, 0, 1]]
        );

        // Without edge bits the pack function is not called.
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [5, 7, 9], 0)]);
        packs.borrow_mut().clear();
        fn_00695660(&mut e, mesh, 0);
        assert!(packs.borrow().is_empty());
    }

    #[test]
    fn packed_edge_bits_test() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 1, 2], [0; 3], 0x0001_0000),
                ([0, 1, 2], [0; 3], 0x0000_ffff),
                ([0, 1, 2], [0; 3], 0xf000_0000),
                ([0, 1, 2], [0; 3], 0x0800_0000),
            ],
        );
        let results: Vec<bool> = (0..4)
            .map(|i| {
                let tri = triangle_ptr(&e, mesh, i);
                fn_006957d0(&mut e, tri)
            })
            .collect();
        assert_eq!(results, vec![true, false, false, true]);
    }

    // --- 006957f0 -----------------------------------------------------

    #[test]
    fn bounds_cover_the_vertices_of_the_triangles() {
        let mut e = engine();
        // Vertex 4 belongs to no triangle and does not count.
        let mesh = mesh_with(
            &mut e,
            &[
                [1.0, 2.0, 3.0],
                [-4.0, 5.0, 6.0],
                [7.0, -8.0, 9.0],
                [0.0, 0.0, -10.0],
                [100.0, 100.0, 100.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([1, 2, 3], [0xffff; 3], 0)],
        );
        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        fn_006957f0(&mut e, mesh, min, max);
        assert_eq!(read_point(&e, min.addr()), [-4.0, -8.0, -10.0]);
        assert_eq!(read_point(&e, max.addr()), [7.0, 5.0, 9.0]);

        // A mesh without triangles leaves the outputs alone.
        let empty = mesh_with(&mut e, &[], &[]);
        let min = point(&mut e, [1.0, 2.0, 3.0]);
        let max = point(&mut e, [4.0, 5.0, 6.0]);
        fn_006957f0(&mut e, empty, min, max);
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);
    }

    // --- grid helpers ---------------------------------------------------

    const GRID_SIZE: u32 = 3;

    /// Sets up the grid of `mesh` like the game's setup would for this
    /// test: 3 by 3 cells of 10 by 10 from the origin, each an (empty)
    /// 16-byte array.
    fn set_up_grid(e: &mut Engine, mesh: Ptr<NavMesh>) -> Ptr<NavMeshGrid> {
        let grid = mesh_grid(mesh);
        e.set(grid, NavMeshGrid::iGridSize, GRID_SIZE);
        e.set(grid, NavMeshGrid::fColumnSectionLen, 10.0f32);
        e.set(grid, NavMeshGrid::fRowSectionLen, 10.0f32);
        let cells = e.mem.alloc(GRID_SIZE * GRID_SIZE * 16);
        e.set(grid, NavMeshGrid::GridData, Ptr::new(cells));
        grid
    }

    /// Records the `(cell number, word)` of every `ArrayAdd` on a grid cell.
    fn record_cell_adds(e: &mut Engine, grid: Ptr<NavMeshGrid>) -> Rc<RefCell<Vec<(u32, u16)>>> {
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        let adds = Rc::new(RefCell::new(Vec::new()));
        let log = adds.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            log.borrow_mut()
                .push(((a[0] - cells) / 16, e.mem.u16(a[1])));
            Ret::default()
        });
        adds
    }

    /// The row/column of a point in the test grid (10 by 10 cells from the
    /// origin, clamped), as `006912a0` computes them.
    fn register_cell_of_point(e: &mut Engine) {
        e.register(GRID_CELL_OF_POINT, |e, a| {
            let (x, y) = (e.mem.f32(a[1]), e.mem.f32(a[1] + 4));
            let column = ((x / 10.0).floor() as i32).clamp(0, 2) as u32;
            let row = ((y / 10.0).floor() as i32).clamp(0, 2) as u32;
            e.mem.set_u32(a[2], row);
            e.mem.set_u32(a[3], column);
            Ret::default()
        });
    }

    fn inside_triangle(p: [f32; 2], a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> bool {
        let side = |u: [f32; 2], v: [f32; 2]| {
            (v[0] - u[0]) * (p[1] - u[1]) - (v[1] - u[1]) * (p[0] - u[0])
        };
        let (s1, s2, s3) = (side(a, b), side(b, c), side(c, a));
        (s1 >= 0.0 && s2 >= 0.0 && s3 >= 0.0) || (s1 <= 0.0 && s2 <= 0.0 && s3 <= 0.0)
    }

    fn segments_cross(a: [f32; 2], b: [f32; 2], c: [f32; 2], d: [f32; 2]) -> bool {
        let orient = |p: [f32; 2], q: [f32; 2], r: [f32; 2]| {
            (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
        };
        orient(a, b, c) * orient(a, b, d) < 0.0 && orient(c, d, a) * orient(c, d, b) < 0.0
    }

    // --- 006959b0 ------------------------------------------------------

    #[test]
    fn grid_lists_each_triangle_in_the_cells_it_touches() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                // 0: inside cell (1, 1) only.
                [12.0, 12.0, 0.0],
                [14.0, 12.0, 0.0],
                [12.0, 14.0, 0.0],
                // 1: a large triangle over most of the grid.
                [1.0, 1.0, 0.0],
                [25.0, 1.0, 0.0],
                [1.0, 25.0, 0.0],
                // 2: a thin sliver along the bottom row.
                [1.0, 1.0, 0.0],
                [29.0, 1.5, 0.0],
                [1.0, 2.0, 0.0],
                // 3: at the far corner; 30 is clamped to the last cell.
                [30.0, 30.0, 0.0],
                [29.5, 29.5, 0.0],
                [29.0, 30.0, 0.0],
            ],
            &[
                ([0, 1, 2], [0xffff; 3], 0),
                ([3, 4, 5], [0xffff; 3], 0),
                ([6, 7, 8], [0xffff; 3], 0),
                ([9, 10, 11], [0xffff; 3], 0),
            ],
        );
        let grid = mesh_grid(mesh);
        let clears = recorder(&mut e, GRID_CLEAR, 0);
        let setups = Rc::new(RefCell::new(Vec::new()));
        let log = setups.clone();
        e.register_double(GRID_SETUP, move |e, a| {
            log.borrow_mut()
                .push((a.to_vec(), read_point(e, a[1]), read_point(e, a[2])));
            let grid = Ptr::<NavMeshGrid>::new(a[0]);
            e.set(grid, NavMeshGrid::iGridSize, GRID_SIZE);
            e.set(grid, NavMeshGrid::fColumnSectionLen, 10.0f32);
            e.set(grid, NavMeshGrid::fRowSectionLen, 10.0f32);
            let cells = e.mem.alloc(GRID_SIZE * GRID_SIZE * 16);
            e.set(grid, NavMeshGrid::GridData, Ptr::new(cells));
            Ret::default()
        });
        e.register(GRID_CELL_CORNER, |e, a| {
            write_point(e, a[1], [a[2] as f32 * 10.0, a[3] as f32 * 10.0, 0.0]);
            ret(a[1])
        });
        e.register(TRIANGLE_RAY_TEST, |e, a| {
            let corner = |p: u32| [e.mem.f32(p), e.mem.f32(p + 4)];
            let hit = inside_triangle(corner(a[3]), corner(a[0]), corner(a[1]), corner(a[2]));
            e.mem.set_f32(a[4], 0.0);
            ret(hit as u32)
        });
        e.register(SEGMENTS_INTERSECT_2D, |e, a| {
            let p = |i: usize| [e.mem.f32(a[i]), e.mem.f32(a[i] + 4)];
            ret(segments_cross(p(0), p(1), p(2), p(3)) as u32)
        });
        // The cells are read after the setup double allocated them: record
        // them lazily.
        let adds = Rc::new(RefCell::new(Vec::new()));
        let log = adds.clone();
        e.register_double(ARRAY_ADD_WORD, move |e, a| {
            let cells = e.mem.u32(grid.addr() + 0x24);
            log.borrow_mut()
                .push(((a[0] - cells) / 16, e.mem.u16(a[1])));
            Ret::default()
        });

        fn_006959b0(&mut e, mesh);
        assert_eq!(*clears.borrow(), vec![vec![grid.addr()]]);
        let setups = setups.borrow();
        assert_eq!(setups.len(), 1);
        // The grid was sized from the bounds of the mesh and 4 triangles.
        assert_eq!(setups[0].0[3], 4);
        assert_eq!(setups[0].1, [1.0, 1.0, 0.0]);
        assert_eq!(setups[0].2, [30.0, 30.0, 0.0]);

        let cells_of = |triangle: u16| -> Vec<u32> {
            let mut cells: Vec<u32> = adds
                .borrow()
                .iter()
                .filter(|(_, t)| *t == triangle)
                .map(|(cell, _)| *cell)
                .collect();
            cells.sort();
            cells
        };
        assert_eq!(cells_of(0), vec![4]);
        // The large triangle: cell (1, 2) is the only one of its rectangle
        // it misses. Cells (2, 1) and (2, 2) are listed because the game
        // tests the second vertex's x range against the third vertex's y
        // for the low side of the cell, so the vertex at (25, 1) accepts the
        // whole right column.
        assert_eq!(cells_of(1), vec![0, 1, 2, 3, 4, 5, 6, 8]);
        // The sliver reaches the middle cell only through the edge crossing.
        assert_eq!(cells_of(2), vec![0, 1, 2]);
        assert_eq!(cells_of(3), vec![8]);
    }

    #[test]
    fn grid_of_an_empty_mesh_is_only_cleared() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let clears = recorder(&mut e, GRID_CLEAR, 0);
        let setups = recorder(&mut e, GRID_SETUP, 0);
        fn_006959b0(&mut e, mesh);
        assert_eq!(clears.borrow().len(), 1);
        assert!(setups.borrow().is_empty());
    }

    // --- 00696500, 00696540, 006967c0 -----------------------------------

    #[test]
    fn cell_add_appends_to_the_cell_of_the_coordinates() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = set_up_grid(&mut e, mesh);
        let adds = record_cell_adds(&mut e, grid);
        // Cell (2, 1) is number 1 * 3 + 2.
        fn_00696500(&mut e, grid, 0x1234, 2, 1);
        assert_eq!(*adds.borrow(), vec![(5, 0x1234)]);
    }

    #[test]
    fn segment_test_copies_the_points_to_2d_points_in_order() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENTS_INTERSECT_2D, move |e, a| {
            let points: Vec<[f32; 2]> = a
                .iter()
                .map(|p| [e.mem.f32(*p), e.mem.f32(*p + 4)])
                .collect();
            log.borrow_mut().push(points);
            ret(1)
        });
        let a = point(&mut e, [1.0, 2.0, 9.0]);
        let b = point(&mut e, [3.0, 4.0, 9.0]);
        let c = point(&mut e, [5.0, 6.0, 9.0]);
        let d = point(&mut e, [7.0, 8.0, 9.0]);
        assert!(fn_00696540(&mut e, a, b, c, d));
        assert_eq!(
            *seen.borrow(),
            vec![vec![[1.0, 2.0], [3.0, 4.0], [5.0, 6.0], [7.0, 8.0]]]
        );
        e.register(SEGMENTS_INTERSECT_2D, |_, _| ret(0));
        assert!(!fn_00696540(&mut e, a, b, c, d));
    }

    #[test]
    fn cell_address_is_row_major() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = set_up_grid(&mut e, mesh);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        assert_eq!(
            fn_006967c0(&mut e, grid, 1, 2).addr(),
            cells + (2 * 3 + 1) * 16
        );
        assert_eq!(fn_006967c0(&mut e, grid, 0, 0).addr(), cells);
    }

    // --- 006965d0 and 006967f0 ---------------------------------------------

    fn two_triangle_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[
                // A large triangle: x from 1 to 25, y from 1 to 25.
                [1.0, 1.0, 0.0],
                [25.0, 1.0, 0.0],
                [1.0, 25.0, 0.0],
                // A small one in the middle cell.
                [12.0, 12.0, 0.0],
                [14.0, 12.0, 0.0],
                [12.0, 14.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn range_add_lists_the_triangle_rectangles() {
        let mut e = engine();
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        register_cell_of_point(&mut e);
        let adds = record_cell_adds(&mut e, grid);
        assert!(fn_006965d0(&mut e, mesh, 0, 1));
        let adds = adds.borrow();
        // Triangle 0 covers rows 0 to 2 and columns 0 to 2, row by row;
        // triangle 1 only the middle cell.
        let first: Vec<(u32, u16)> = (0..9).map(|cell| (cell, 0)).collect();
        assert_eq!(adds[..9], first[..]);
        assert_eq!(adds[9..], [(4, 1)]);

        // A range of one triangle.
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        let adds = record_cell_adds(&mut e, grid);
        assert!(fn_006965d0(&mut e, mesh, 1, 1));
        assert_eq!(*adds.borrow(), vec![(4, 1)]);
    }

    #[test]
    fn range_remove_deletes_the_triangle_from_each_cell() {
        let mut e = engine();
        let mesh = two_triangle_mesh(&mut e);
        let grid = set_up_grid(&mut e, mesh);
        let cells = e.get(grid, NavMeshGrid::GridData).addr();
        register_cell_of_point(&mut e);
        let finds = Rc::new(RefCell::new(Vec::new()));
        let log = finds.clone();
        e.register_double(ARRAY_FIND_WORD, move |e, a| {
            log.borrow_mut().push((a[0], e.mem.u32(a[1]), a[2], a[3]));
            ret(7)
        });
        let removes = recorder(&mut e, WORD_ARRAY_REMOVE, 0);
        assert!(fn_006967f0(&mut e, mesh, 1, 1));
        assert_eq!(
            *finds.borrow(),
            vec![(cells + 4 * 16, 1, 0, WORD_COMPARATOR)]
        );
        assert_eq!(*removes.borrow(), vec![vec![cells + 4 * 16, 7, 1]]);

        finds.borrow_mut().clear();
        removes.borrow_mut().clear();
        assert!(fn_006967f0(&mut e, mesh, 0, 0));
        assert_eq!(finds.borrow().len(), 9);
        assert_eq!(removes.borrow().len(), 9);
        // Triangle 0, cells in row-major order.
        assert_eq!(finds.borrow()[0], (cells, 0, 0, WORD_COMPARATOR));
        assert_eq!(finds.borrow()[8], (cells + 8 * 16, 0, 0, WORD_COMPARATOR));
    }

    // --- 006969e0, 00696a10 -----------------------------------------------

    #[test]
    fn grid_bounds_are_copied_out() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let grid = mesh_grid(mesh);
        write_point(&mut e, grid.addr() + 0x0c, [1.0, 2.0, 3.0]);
        write_point(&mut e, grid.addr() + 0x18, [4.0, 5.0, 6.0]);
        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        fn_00696a10(&mut e, grid, min, max);
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);

        let min = point(&mut e, [0.0; 3]);
        let max = point(&mut e, [0.0; 3]);
        assert!(fn_006969e0(&mut e, mesh, min, max));
        assert_eq!(read_point(&e, min.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, max.addr()), [4.0, 5.0, 6.0]);
    }

    // --- triangle lookup by point ----------------------------------------

    /// A mesh of `markers.len()` triangles; triangle `i` has the vertices
    /// `3i`, `3i + 1`, `3i + 2`, the first at `(markers[i], 0, 0)` and the
    /// others at the origin, so a test double of the ray test can tell the
    /// triangles apart by the first vertex's x. `flags` are the triangle
    /// flags (default 0).
    fn marked_mesh(e: &mut Engine, markers: &[f32], flags: &[(usize, u32)]) -> Ptr<NavMesh> {
        let vertices: Vec<[f32; 3]> = markers
            .iter()
            .flat_map(|m| [[*m, 0.0, 0.0], [0.0; 3], [0.0; 3]])
            .collect();
        let triangles: Vec<TriangleSpec> = (0..markers.len() as u16)
            .map(|i| {
                let flag = flags
                    .iter()
                    .find(|(t, _)| *t == i as usize)
                    .map_or(0, |f| f.1);
                ([3 * i, 3 * i + 1, 3 * i + 2], [0xffff; 3], flag)
            })
            .collect();
        mesh_with(e, &vertices, &triangles)
    }

    /// The ray test answers by the first vertex's x: `(marker, Some(height))`
    /// hits with that height, `(marker, None)` misses.
    fn register_ray_table(e: &mut Engine, table: &[(f32, Option<f32>)]) -> Calls {
        let table = table.to_vec();
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(TRIANGLE_RAY_TEST, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let marker = e.mem.f32(a[0]);
            match table
                .iter()
                .find(|(m, _)| *m == marker)
                .and_then(|(_, h)| *h)
            {
                Some(height) => {
                    e.mem.set_f32(a[4], height);
                    ret(1)
                }
                None => ret(0),
            }
        });
        calls
    }

    /// Makes the grid answer with a cell list holding `triangles`, or with
    /// no list.
    fn register_cell_list(e: &mut Engine, triangles: Option<&[u16]>) {
        let list = triangles.map(|triangles| {
            let list = e.mem.alloc(16);
            put_array(
                e,
                Ptr::new(list),
                &words16(triangles),
                triangles.len() as u32,
            );
            list
        });
        e.register_double(GRID_CELL_LIST, move |e, a| {
            e.mem.set_u32(a[2], list.unwrap_or(0));
            Ret::default()
        });
    }

    #[test]
    fn triangle_for_point_picks_the_nearest_height() {
        let mut e = engine();
        let mesh = marked_mesh(
            &mut e,
            &[100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0],
            &[(0, 0x20)],
        );
        register_ray_table(
            &mut e,
            &[
                (100.0, Some(10.0)),
                (200.0, Some(50.0)),
                (300.0, Some(-60.0)),
                (400.0, Some(30.0)),
                (500.0, Some(70.0)),
                (600.0, Some(-200.0)),
                (700.0, None),
            ],
        );
        let at = point(&mut e, [0.0; 3]);
        let find = |e: &mut Engine, list: Option<&[u16]>| {
            register_cell_list(e, list);
            fn_00696a50(e, mesh, at, Ptr::NULL)
        };
        // Triangle 0 is flagged and ignored; 1 at distance 50 beats 2 at 60.
        assert_eq!(find(&mut e, Some(&[0, 1, 2])), 1);
        // A triangle within 40 ends the search at once.
        assert_eq!(find(&mut e, Some(&[1, 3, 2])), 3);
        // More than 64 above is never accepted.
        assert_eq!(find(&mut e, Some(&[4])), 0xffff);
        // No hit, no list.
        assert_eq!(find(&mut e, Some(&[6])), 0xffff);
        assert_eq!(find(&mut e, None), 0xffff);

        // Far below the point (-200): rejected unless the point is not
        // more than 50 above the ground of the parent cell.
        assert_eq!(find(&mut e, Some(&[5])), 0xffff);
        let cell = 0x9000u32;
        e.set(mesh, NavMesh::pParentCell, Ptr::new(cell));
        let ground = Rc::new(RefCell::new(-1000.0f32));
        let value = ground.clone();
        e.register_double(CELL_GROUND_HEIGHT, move |e, a| {
            assert_eq!(a[0], 0x9000);
            e.mem.set_f32(a[2], *value.borrow());
            Ret::default()
        });
        assert_eq!(find(&mut e, Some(&[5])), 0xffff);
        *ground.borrow_mut() = 1000.0;
        assert_eq!(find(&mut e, Some(&[5])), 5);
    }

    #[test]
    fn point_has_a_triangle_when_one_is_found() {
        let mut e = engine();
        let mesh = marked_mesh(&mut e, &[100.0], &[]);
        register_ray_table(&mut e, &[(100.0, Some(5.0))]);
        let at = point(&mut e, [0.0; 3]);
        register_cell_list(&mut e, Some(&[0]));
        assert!(fn_00697670(&mut e, mesh, at, Ptr::NULL));
        register_cell_list(&mut e, None);
        assert!(!fn_00697670(&mut e, mesh, at, Ptr::NULL));
    }

    #[test]
    fn grid_call_is_forwarded() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[]);
        let calls = recorder(&mut e, GRID_FORWARDED, 0);
        fn_00696c00(&mut e, mesh, 1, 2, 3);
        assert_eq!(*calls.borrow(), vec![vec![mesh_grid(mesh).addr(), 1, 2, 3]]);
    }

    // --- copy operators ---------------------------------------------------

    fn filled(e: &mut Engine, size: u32) -> Ptr {
        let block = e.mem.alloc(size);
        for i in 0..size {
            e.mem.set_u8(block + i, (i + 1) as u8);
        }
        Ptr::new(block)
    }

    #[test]
    fn copy_operators_copy_their_parts() {
        let mut e = engine();
        e.register(COPY_STRUCT_16, |e, a| {
            for i in 0..16 {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            ret(a[0])
        });
        let source = filled(&mut e, 0x58);

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696ca0(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x20),
            e.mem.bytes(source.addr(), 0x20)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x20, 0x38)
            .iter()
            .all(|b| *b == 0));

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696c60(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x44),
            e.mem.bytes(source.addr(), 0x44)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x44, 0x14)
            .iter()
            .all(|b| *b == 0));

        let target = Ptr::new(e.mem.alloc(0x58));
        assert_eq!(fn_00696c30(&mut e, target, source), target);
        assert_eq!(
            e.mem.bytes(target.addr(), 0x44),
            e.mem.bytes(source.addr(), 0x44)
        );
        assert!(e
            .mem
            .bytes(target.addr() + 0x44, 0xc)
            .iter()
            .all(|b| *b == 0));
        assert_eq!(
            e.mem.u32(target.addr() + 0x50),
            e.mem.u32(source.addr() + 0x50)
        );
        assert_eq!(e.mem.u32(target.addr() + 0x54), 0);
    }

    // --- 00696cf0 ------------------------------------------------------------

    /// The center of triangle `i` is `(10 i, 0, 0)`.
    fn register_centers(e: &mut Engine) {
        e.register(GET_CENTER, |e, a| {
            write_point(e, a[1], [10.0 * a[2] as f32, 0.0, 0.0]);
            ret(a[1])
        });
    }

    #[test]
    fn closest_triangle_prefers_a_height_then_the_nearest_center() {
        let mut e = engine();
        register_centers(&mut e);
        let at = point(&mut e, [0.0; 3]);
        let out_triangle = Ptr::new(e.mem.alloc(4));
        let out_found = Ptr::new(e.mem.alloc(4));
        let run = |e: &mut Engine,
                   mesh: Ptr<NavMesh>,
                   table: &[(f32, Option<f32>)],
                   list: Option<&[u16]>| {
            register_ray_table(e, table);
            register_cell_list(e, list);
            let result =
                nav_mesh_find_closest_triangle_for_location(e, mesh, at, out_triangle, out_found);
            (
                result,
                e.mem.u16(out_triangle.addr()),
                e.mem.u8(out_found.addr()),
            )
        };
        let mesh = marked_mesh(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0], &[]);
        // Without a cell list every triangle is scanned: 2 and 3 have a
        // height (50, 45), the nearer one wins although neither is within 40.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, Some(50.0)),
            (4.0, Some(-45.0)),
            (5.0, None),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 3, 1));
        // A height difference within 40 returns at once.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, Some(50.0)),
            (4.0, Some(30.0)),
            (5.0, Some(1.0)),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 3, 1));
        // Nothing has a height: the nearest center, triangle 0.
        let table = [
            (1.0, None),
            (2.0, None),
            (3.0, None),
            (4.0, None),
            (5.0, None),
        ];
        assert_eq!(run(&mut e, mesh, &table, None), (true, 0, 0));
        // With a list, the flagged triangle is skipped: only triangle 2 and
        // 4 are candidates and the center of 2 is nearer.
        let flagged = marked_mesh(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0], &[(2, 0x20)]);
        assert_eq!(run(&mut e, flagged, &table, Some(&[2, 4])), (true, 4, 0));
        // An empty mesh has nothing.
        let empty = marked_mesh(&mut e, &[], &[]);
        assert_eq!(run(&mut e, empty, &[], None), (false, 0xffff, 0));
    }

    // --- 00697010 -----------------------------------------------------------

    /// `GetEdgeVertices`: the addresses of the two vertices of an edge.
    fn register_edge_vertices(e: &mut Engine) {
        e.register(GET_EDGE_VERTICES, |e, a| {
            let triangle = e.mem.u32(a[0] + 0x38 + 4) + (a[2] & 0xffff) * 16;
            let vertices = e.mem.u32(a[0] + 0x28 + 4);
            let first = e.mem.u16(triangle + 2 * a[3]) as u32;
            let second = e.mem.u16(triangle + 2 * ((a[3] + 1) % 3)) as u32;
            e.mem.set_u32(a[1], vertices + 12 * first);
            e.mem.set_u32(a[1] + 4, vertices + 12 * second);
            ret(a[1])
        });
    }

    fn square_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        // Two triangles A B C and B C D of a square of side 10; the edge
        // B-C is shared.
        mesh_with(
            e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [10.0, 10.0, 0.0],
            ],
            &[
                ([0, 1, 2], [0xffff, 1, 0xffff], 0),
                ([1, 3, 2], [0xffff, 0xffff, 0], 0),
            ],
        )
    }

    #[test]
    fn border_edge_search_finds_the_nearest_edge_without_neighbour() {
        let mut e = engine();
        register_edge_vertices(&mut e);
        let mesh = square_mesh(&mut e);
        let out_triangle = Ptr::new(e.mem.alloc(4));
        let out_edge = Ptr::new(e.mem.alloc(4));
        let near = point(&mut e, [6.0, 1.0, 0.0]);
        assert!(fn_00697010(
            &mut e,
            mesh,
            near,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
        // The bottom edge A-B of triangle 0 (midpoint (5, 0)).
        assert_eq!(e.mem.u16(out_triangle.addr()), 0);
        assert_eq!(e.mem.i32(out_edge.addr()), 0);

        // The right edge of triangle 1 (B-D, edge 0, midpoint (10, 5)).
        let right = point(&mut e, [9.0, 5.0, 0.0]);
        assert!(fn_00697010(
            &mut e,
            mesh,
            right,
            1.0,
            out_triangle,
            out_edge,
            true
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 1);
        assert_eq!(e.mem.i32(out_edge.addr()), 0);

        // Above the height range, the large radius admits several edges.
        // The executable compares planar distance with the previous full
        // distance, so the later triangle 1 edge 1 replaces triangle 0 edge 0.
        let high = point(&mut e, [6.0, 1.0, 500.0]);
        e.mem.set_u16(out_triangle.addr(), 0x4444);
        assert!(!fn_00697010(
            &mut e,
            mesh,
            high,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 0x4444);
        assert!(fn_00697010(
            &mut e,
            mesh,
            high,
            1000.0,
            out_triangle,
            out_edge,
            false
        ));
        assert_eq!(e.mem.u16(out_triangle.addr()), 1);
        assert_eq!(e.mem.i32(out_edge.addr()), 1);

        // A mesh whose edges all have neighbours has no border edge.
        let closed = mesh_with(
            &mut e,
            &[[0.0; 3], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            &[([0, 1, 2], [0, 0, 0], 0)],
        );
        assert!(!fn_00697010(
            &mut e,
            closed,
            near,
            1.0,
            out_triangle,
            out_edge,
            false
        ));
    }

    // --- 006971e0, 00697290, 00697440 ----------------------------------------

    #[test]
    fn triangle_normal_is_the_unit_cross_of_two_edges() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[[1.0, 1.0, 5.0], [3.0, 1.0, 5.0], [1.0, 4.0, 5.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        );
        let out = point(&mut e, [9.0; 3]);
        assert_eq!(fn_006971e0(&mut e, mesh, out, 0), out);
        assert_eq!(read_point(&e, out.addr()), [0.0, 0.0, 1.0]);
    }

    fn circumcenter_mesh(e: &mut Engine, z: [f32; 3]) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[[0.0, 0.0, z[0]], [4.0, 0.0, z[1]], [0.0, 4.0, z[2]]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn circumcenter_has_the_highest_z() {
        let mut e = engine();
        let out = point(&mut e, [9.0; 3]);
        for (z, expected) in [
            ([1.0, 5.0, 3.0], 5.0),
            ([7.0, 2.0, 3.0], 7.0),
            ([1.0, 9.0, 9.0], 9.0),
            ([1.0, 2.0, 3.0], 3.0),
        ] {
            let mesh = circumcenter_mesh(&mut e, z);
            assert_eq!(fn_00697290(&mut e, mesh, out, 0), out);
            assert_eq!(read_point(&e, out.addr()), [2.0, 2.0, expected]);
        }
    }

    #[test]
    fn circumcenter_in_the_plane_has_zero_z() {
        let mut e = engine();
        let mesh = circumcenter_mesh(&mut e, [1.0, 2.0, 3.0]);
        let out = point(&mut e, [9.0; 3]);
        fn_00697440(&mut e, mesh, 0, out);
        assert_eq!(read_point(&e, out.addr()), [2.0, 2.0, 0.0]);
    }

    // --- 00697590 ---------------------------------------------------------

    #[test]
    fn portal_triangles_list_the_triangles_leading_to_the_info() {
        let mut e = engine();
        let info = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        // Triangle 0: edge 1 (slot 0) and edge 2 (slot 1) both lead to
        // `info`; triangle 1 has no extra info; triangle 2: edge 0 (slot 2)
        // leads to `other`; triangle 3: edge 2 (slot 1) leads to `info`.
        let mesh = mesh_with(
            &mut e,
            &[],
            &[
                ([0, 1, 2], [0xffff, 0, 1], 0b110),
                ([0, 1, 2], [0xffff; 3], 0),
                ([0, 1, 2], [2, 0xffff, 0xffff], 0b001),
                ([0, 1, 2], [0xffff, 0xffff, 1], 0b100),
            ],
        );
        let infos: Vec<u8> = [info, info, other]
            .iter()
            .flat_map(|target| words(&[0, *target, 0]))
            .collect();
        put_array(&mut e, Ptr::new(mesh.addr() + 0x48), &infos, 3);
        e.register(GET_EXTRA_EDGE_INFO, |e, a| {
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * (a[1] & 0xffff))
        });
        let sizes = recorder(&mut e, ARRAY_SET_SIZE, 0);
        let reserves = recorder(&mut e, ARRAY_SET_RESERVED_SIZE, 0);
        e.register(ARRAY_ADD_DWORD, |e, a| {
            let value = e.mem.u32(a[1]);
            let buffer = e.mem.u32(a[0] + 4);
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(buffer + 4 * size, value);
            e.mem.set_u32(a[0] + 8, size + 1);
            Ret::default()
        });
        let result = e.mem.alloc(16);
        let buffer = e.mem.alloc(64);
        e.mem.set_u32(result + 4, buffer);
        assert!(nav_mesh_find_portal_triangles(
            &mut e,
            mesh,
            Ptr::new(info),
            Ptr::new(result)
        ));
        assert_eq!(array_words(&e, result), vec![0, 3]);
        assert_eq!(*sizes.borrow(), vec![vec![result, 0, 1]]);
        assert_eq!(*reserves.borrow(), vec![vec![result, 100]]);

        // Another info matches nothing.
        e.mem.set_u32(result + 8, 0);
        assert!(!nav_mesh_find_portal_triangles(
            &mut e,
            mesh,
            Ptr::new(0x1234),
            Ptr::new(result)
        ));
    }

    // --- 006976a0 ---------------------------------------------------------

    #[test]
    fn edge_info_is_created_on_demand_and_reset() {
        let mut e = engine();
        e.register(GET_EDGE_EXTRA_INFO, |e, a| {
            let triangle = e.mem.u32(a[0] + 0x38 + 4) + (a[1] & 0xffff) * 16;
            if (e.mem.u32(triangle + 12) >> a[2]) & 1 == 0 {
                return ret(0);
            }
            let slot = e.mem.u16(triangle + 6 + 2 * a[2]) as u32;
            ret(e.mem.u32(a[0] + 0x48 + 4) + 12 * slot)
        });
        let constructs = recorder(&mut e, EDGE_INFO_CONSTRUCTOR, 0);
        e.register(ADD_EXTRA_EDGE_INFO, |e, a| {
            let size = e.mem.u32(a[0] + 0x48 + 8);
            e.mem.set_u32(a[0] + 0x48 + 8, size + 1);
            ret(size)
        });
        // Edge 1 of triangle 0 has no info; the mesh has room for 4.
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [0xffff; 3], 0)]);
        let buffer = e.mem.alloc(48);
        e.mem.set_u32(mesh.addr() + 0x48 + 4, buffer);
        e.mem.set_u32(buffer + 4, 0x5555);
        let info = fn_006976a0(&mut e, mesh, 0, 1);
        assert_eq!(constructs.borrow().len(), 1);
        // The new info is number 0: the edge now points at it.
        let tri = triangle_ptr(&e, mesh, 0);
        assert_eq!(
            triangle_words(&e, tri),
            ([0, 1, 2], [0xffff, 0, 0xffff], 0b010)
        );
        assert_eq!(info.addr(), buffer);
        // Reset to "no other mesh".
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(e.mem.u16(buffer + 8), 0xffff);

        // An existing info is only reset.
        e.mem.set_u32(buffer + 4, 0x7777);
        e.mem.set_u16(buffer + 8, 3);
        let again = fn_006976a0(&mut e, mesh, 0, 1);
        assert_eq!(again, info);
        assert_eq!(constructs.borrow().len(), 1);
        assert_eq!(e.mem.u32(buffer + 4), 0);
        assert_eq!(e.mem.u16(buffer + 8), 0xffff);
    }

    // --- 00697730, 00697810 -------------------------------------------------

    #[test]
    fn ray_test_passes_copies_of_the_three_vertices() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[[9.0; 3], [1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
            &[([1, 2, 3], [0xffff; 3], 0)],
        );
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(TRIANGLE_RAY_TEST, move |e, a| {
            log.borrow_mut().push((
                [
                    read_point(e, a[0]),
                    read_point(e, a[1]),
                    read_point(e, a[2]),
                ],
                a[3],
                a[4],
                (arg_f32(a[5]), arg_f32(a[6])),
            ));
            ret(1)
        });
        let at = point(&mut e, [0.0; 3]);
        let out = Ptr::new(e.mem.alloc(4));
        assert!(fn_00697730(&mut e, mesh, 0, at, out, 3.5, 4.5));
        assert_eq!(
            *seen.borrow(),
            vec![(
                [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
                at.addr(),
                out.addr(),
                (3.5, 4.5)
            )]
        );
        e.register(TRIANGLE_RAY_TEST, |_, _| ret(0));
        assert!(!fn_00697730(&mut e, mesh, 0, at, out, 3.5, 4.5));
    }

    #[test]
    fn ray_test_with_a_callback_reports_the_hit_point() {
        let mut e = engine();
        let mesh = marked_mesh(&mut e, &[100.0], &[]);
        let table_calls = register_ray_table(&mut e, &[(100.0, Some(5.0))]);
        let at = point(&mut e, [1.0, 2.0, 10.0]);
        let out = Ptr::new(e.mem.alloc(4));
        // Without a callback the limits are FLT_MAX.
        assert!(fn_00697810(&mut e, mesh, 0, at, out, Ptr::NULL));
        let call = table_calls.borrow()[0].clone();
        assert_eq!((arg_f32(call[5]), arg_f32(call[6])), (f32::MAX, f32::MAX));

        // With a callback its virtual slot 0 gets the point, the mesh, the
        // triangle and the point raised by the height, and its answer counts.
        let callback = e.mem.alloc(8);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(callback, vtable);
        e.mem.set_u32(vtable, 0x0500_0000);
        let hits = Rc::new(RefCell::new(Vec::new()));
        let log = hits.clone();
        e.register_double(0x0500_0000, move |e, a| {
            log.borrow_mut().push((a.to_vec(), read_point(e, a[4])));
            ret(0)
        });
        assert!(!fn_00697810(&mut e, mesh, 0, at, out, Ptr::new(callback)));
        assert_eq!(
            *hits.borrow(),
            vec![(
                vec![callback, at.addr(), mesh.addr(), 0, hits.borrow()[0].0[4]],
                [1.0, 2.0, 15.0]
            )]
        );

        // A miss never calls the callback.
        let miss = marked_mesh(&mut e, &[200.0], &[]);
        assert!(!fn_00697810(&mut e, miss, 0, at, out, Ptr::new(callback)));
        assert_eq!(hits.borrow().len(), 1);
    }

    // --- 00697980 ---------------------------------------------------------

    #[test]
    fn height_over_a_triangle() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 10.0],
                // A flat (degenerate) triangle.
                [0.0, 0.0, 0.0],
                [5.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        );
        let out = Ptr::new(e.mem.alloc(4));
        let check = |e: &mut Engine, triangle: u16, p: [f32; 3], above: f32, below: f32| {
            let p = point(e, p);
            nav_mesh_compute_triangle_z_for_location(e, mesh, triangle, p, above, below, out)
        };
        // Over (2, 2) the surface is at z = 2 (the third vertex has z = 10
        // and weight 0.2).
        assert!(check(&mut e, 0, [2.0, 2.0, 3.0], 5.0, 5.0));
        assert!((e.mem.f32(out.addr()) - 2.0).abs() < 1e-5);
        // 1 above the surface is within 0.5? No.
        assert!(!check(&mut e, 0, [2.0, 2.0, 3.0], 0.5, 5.0));
        // 8 under the surface is within 10 under, not 5.
        assert!(check(&mut e, 0, [2.0, 2.0, -6.0], 5.0, 10.0));
        assert!(!check(&mut e, 0, [2.0, 2.0, -6.0], 5.0, 5.0));
        // Outside the triangle (each weight can be the negative one).
        assert!(!check(&mut e, 0, [-1.0, 2.0, 0.0], 5.0, 5.0));
        assert!(!check(&mut e, 0, [2.0, -1.0, 0.0], 5.0, 5.0));
        assert!(!check(&mut e, 0, [8.0, 8.0, 0.0], 5.0, 5.0));
        // On the border counts.
        assert!(check(&mut e, 0, [5.0, 0.0, 0.0], 5.0, 5.0));
        // A flat triangle never.
        assert!(!check(&mut e, 1, [5.0, 0.0, 0.0], 5.0, 5.0));
    }

    #[test]
    fn height_search_in_the_cell_list() {
        let mut e = engine();
        let mesh = mesh_with(
            &mut e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [20.0, 20.0, 0.0],
                [30.0, 20.0, 0.0],
                [20.0, 30.0, 0.0],
            ],
            &[([0, 1, 2], [0xffff; 3], 0), ([3, 4, 5], [0xffff; 3], 0)],
        );
        let out = Ptr::new(e.mem.alloc(4));
        let over_second = point(&mut e, [22.0, 22.0, 1.0]);
        register_cell_list(&mut e, Some(&[0, 1]));
        assert!(fn_00697b70(&mut e, mesh, over_second, 5.0, 5.0, out));
        assert_eq!(e.mem.f32(out.addr()), 0.0);
        let nowhere = point(&mut e, [50.0, 50.0, 1.0]);
        assert!(!fn_00697b70(&mut e, mesh, nowhere, 5.0, 5.0, out));
        register_cell_list(&mut e, None);
        assert!(!fn_00697b70(&mut e, mesh, over_second, 5.0, 5.0, out));
    }

    // --- 00697c10, 00697d40 -----------------------------------------------

    /// One triangle `(0,0,0) (10,0,0) (0,10,0)` and a second one so the
    /// first can have neighbours on all sides.
    fn neighbour_mesh(e: &mut Engine, neighbours: [u16; 3]) -> Ptr<NavMesh> {
        mesh_with(
            e,
            &[
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [0.0, 10.0, 0.0],
                [10.0, 10.0, 0.0],
            ],
            &[([0, 1, 2], neighbours, 0), ([1, 3, 2], [0, 0, 0], 0)],
        )
    }

    fn register_geometry(e: &mut Engine) {
        register_edge_vertices(e);
        // Distance of a point to a 2D segment.
        e.register(POINT_SEGMENT_DISTANCE, |e, a| {
            let p = read_point(e, a[0]);
            let (u, v) = (read_point(e, a[1]), read_point(e, a[2]));
            let (dx, dy) = (v[0] - u[0], v[1] - u[1]);
            let t =
                (((p[0] - u[0]) * dx + (p[1] - u[1]) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            let (cx, cy) = (u[0] + t * dx, u[1] + t * dy);
            ret_float(((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt())
        });
        // Any triangle is hit by any point; the triangle's height is 0.
        e.register(TRIANGLE_RAY_TEST, |e, a| {
            e.mem.set_f32(a[4], 0.0);
            ret(1)
        });
    }

    #[test]
    fn position_is_pushed_away_from_border_edges() {
        let mut e = engine();
        register_geometry(&mut e);
        register_cell_list(&mut e, Some(&[0]));
        // Triangle 0 (edges 0, 1, 2) has a neighbour only on edge 1.
        let mesh = neighbour_mesh(&mut e, [0xffff, 1, 0xffff]);
        let position = point(&mut e, [5.0, 1.0, 0.0]);
        let result = point(&mut e, [9.0; 3]);
        assert_eq!(fn_00697d40(&mut e, mesh, result, position, 3.0), result);
        // Edge 0 (the bottom, distance 1 < 3) pushes by 2 along the normal
        // (0, 0, 1) crossed with the edge vector (-10, 0, 0), normalized:
        // (0, -1, 0). Edge 2 (distance 5) is too far.
        assert_eq!(read_point(&e, result.addr()), [5.0, -1.0, 0.0]);
        // Neighbour 1 (edges 0, 1, 2 of the second triangle): its edge 1
        // and 2 have no neighbour: edge 1 (3 to 2) is the far side.
        // The position is left alone otherwise.
        assert_eq!(read_point(&e, position.addr()), [5.0, 1.0, 0.0]);

        // Without a triangle under the position nothing moves.
        register_cell_list(&mut e, None);
        let result = point(&mut e, [9.0; 3]);
        fn_00697d40(&mut e, mesh, result, position, 3.0);
        assert_eq!(read_point(&e, result.addr()), [5.0, 1.0, 0.0]);
    }

    #[test]
    fn random_point_is_kept_when_it_lies_over_the_mesh() {
        let mut e = engine();
        register_geometry(&mut e);
        // Both triangles are closed: nothing to push away from.
        let mesh = neighbour_mesh(&mut e, [1, 1, 1]);
        e.register(FLOAT_SQRT, |_, a| ret_float(arg_f32(a[0]).sqrt()));
        let draws = Rc::new(RefCell::new(Vec::new()));
        let log = draws.clone();
        let mut next = 0;
        let values = [3.0f32, 4.0];
        e.register_double(RANDOM_FLOAT, move |_, a| {
            log.borrow_mut().push((arg_f32(a[0]), arg_f32(a[1])));
            next += 1;
            ret_float(values[(next - 1) % 2])
        });
        // The base vector keeps its z.
        write_point(&mut e, RANDOM_POINT_BASE, [0.0, 0.0, 7.0]);
        let origin = point(&mut e, [1.0, 1.0, 0.0]);
        let result = point(&mut e, [9.0; 3]);
        register_cell_list(&mut e, Some(&[0]));
        assert!(fn_00697c10(&mut e, mesh, origin, 5.0, result));
        // x in [-5, 5], then y in [-3, 3] (the half chord at x = 3 is
        // sqrt(25 - 9) = 4: the second range is (-4, 4)).
        assert_eq!(*draws.borrow(), vec![(-5.0, 5.0), (-4.0, 4.0)]);
        // Offset (3, 4, 7) plus the origin.
        assert_eq!(read_point(&e, result.addr()), [4.0, 5.0, 7.0]);

        // If no draw lands over a triangle, 8 attempts are made.
        register_cell_list(&mut e, None);
        draws.borrow_mut().clear();
        let result = point(&mut e, [9.0; 3]);
        assert!(!fn_00697c10(&mut e, mesh, origin, 5.0, result));
        assert_eq!(draws.borrow().len(), 16);
        assert_eq!(read_point(&e, result.addr()), [9.0; 3]);
    }

    // --- 00698030 -----------------------------------------------------------

    #[test]
    fn packed_edge_value_is_four_bits_per_edge() {
        let mut e = engine();
        let mesh = mesh_with(&mut e, &[], &[([0, 1, 2], [0xffff; 3], 0x0a35_0000 | 0x2f)]);
        let tri = triangle_ptr(&e, mesh, 0);
        // Edge n's bits start at bit 16 + 6 n.
        assert_eq!(fn_00698030(&mut e, tri, 0), 0x5);
        assert_eq!(fn_00698030(&mut e, tri, 1), (0x0a35_002f >> 22) & 0xf);
        assert_eq!(fn_00698030(&mut e, tri, 2), 0);
    }

    // --- 00698060, 00698160, 00698300, 00698320 ---------------------------------

    fn crossing_mesh(e: &mut Engine) -> Ptr<NavMesh> {
        // (0,0) (10,0) (0,10): edge 0 along the bottom, edge 1 the
        // hypotenuse, edge 2 up the left.
        mesh_with(
            e,
            &[[0.0, 0.0, 1.0], [10.0, 0.0, 2.0], [0.0, 10.0, 3.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        )
    }

    #[test]
    fn edge_segment_test_returns_the_first_edge_that_hits() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENT_SEGMENT_TEST, move |e, a| {
            log.borrow_mut().push((
                [
                    e.mem.f32(a[0]),
                    e.mem.f32(a[0] + 4),
                    e.mem.f32(a[1]),
                    e.mem.f32(a[1] + 4),
                ],
                a[2..].to_vec(),
            ));
            // Hits the segment (10, 0) - (0, 10) only.
            ret((e.mem.f32(a[0]) == 10.0) as u32)
        });
        assert_eq!(fn_00698060(&mut e, mesh, 0, -1, 7, 8, 9), 1);
        assert_eq!(
            *seen.borrow(),
            vec![
                ([0.0, 0.0, 10.0, 0.0], vec![7, 8, 9]),
                ([10.0, 0.0, 0.0, 10.0], vec![7, 8, 9]),
            ]
        );
        // The skipped edge is not tested; no other edge hits.
        seen.borrow_mut().clear();
        assert_eq!(fn_00698060(&mut e, mesh, 0, 1, 7, 8, 9), -1);
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(seen.borrow()[1].0, [0.0, 10.0, 0.0, 0.0]);
    }

    #[test]
    fn intersecting_edge_reports_type_edge_and_point() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(SEGMENT_INTERSECTION_2D, move |e, a| {
            log.borrow_mut()
                .push((read_point(e, a[0]), read_point(e, a[1]), a[2], a[3]));
            if e.mem.f32(a[0]) == 10.0 {
                write_point(e, a[4], [5.0, 5.0, 0.0]);
                return ret(1);
            }
            ret(0)
        });
        let query = point(&mut e, [20.0, 20.0, 0.0]);
        let result = Ptr::new(e.mem.alloc(20));
        let returned = nav_mesh_find_interecting_edge2(&mut e, mesh, result, 0, -1, 0xabc, query);
        assert_eq!(returned, result);
        assert_eq!(e.mem.u32(result.addr()), 3);
        assert_eq!(e.mem.i32(result.addr() + 4), 1);
        assert_eq!(read_point(&e, result.addr() + 8), [5.0, 5.0, 0.0]);
        assert_eq!(
            seen.borrow()[1],
            ([10.0, 0.0, 2.0], [0.0, 10.0, 3.0], 0xabc, query.addr())
        );

        // Without a crossing: type 0, edge -1.
        e.register(SEGMENT_INTERSECTION_2D, |_, _| ret(0));
        let result = Ptr::new(e.mem.alloc(20));
        nav_mesh_find_interecting_edge2(&mut e, mesh, result, 0, -1, 0xabc, query);
        assert_eq!(e.mem.u32(result.addr()), 0);
        assert_eq!(e.mem.i32(result.addr() + 4), -1);

        // The zero-length edge 0 is never tested, and neither is the skipped edge.
        let degenerate = mesh_with(
            &mut e,
            &[[1.0; 3], [1.0; 3], [5.0, 5.0, 5.0]],
            &[([0, 1, 2], [0xffff; 3], 0)],
        );
        let calls = recorder(&mut e, SEGMENT_INTERSECTION_2D, 0);
        nav_mesh_find_interecting_edge2(&mut e, degenerate, result, 0, 2, 0, query);
        assert_eq!(calls.borrow().len(), 1);
        calls.borrow_mut().clear();
        nav_mesh_find_interecting_edge2(&mut e, degenerate, result, 0, -1, 0, query);
        assert_eq!(calls.borrow().len(), 2);
    }

    #[test]
    fn intersection_result_constructor_builds_the_point() {
        let mut e = engine();
        let calls = recorder(&mut e, POINT3_DEFAULT_CONSTRUCTOR, 0);
        let result = Ptr::new(e.mem.alloc(20));
        assert_eq!(fn_00698300(&mut e, result), result);
        assert_eq!(*calls.borrow(), vec![vec![result.addr() + 8]]);
    }

    #[test]
    fn segment_crossing_edge() {
        let mut e = engine();
        let mesh = crossing_mesh(&mut e);
        let segment = |e: &mut Engine, from: [f32; 3], to: [f32; 3], skip: i32| {
            let (from, to) = (point(e, from), point(e, to));
            fn_00698320(e, mesh, 0, skip, from, to)
        };
        // Straight through the bottom edge.
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 3.0, 0.0], -1), 0);
        // The skipped edge is passed over, nothing else is crossed (the end
        // point inside the triangle lies off the hypotenuse and the left
        // edge).
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 3.0, 0.0], 0), -1);
        // Through the hypotenuse.
        assert_eq!(segment(&mut e, [4.0, 4.0, 0.0], [8.0, 8.0, 0.0], -1), 1);
        // Through the left edge.
        assert_eq!(segment(&mut e, [-3.0, 4.0, 0.0], [3.0, 4.0, 0.0], -1), 2);
        // Ending exactly on an edge is not a crossing of that edge (the
        // second end point lies on its line).
        assert_eq!(segment(&mut e, [5.0, -5.0, 0.0], [5.0, 0.0, 0.0], 1), -1);
        // A segment on both sides of the hypotenuse's line but beyond the
        // triangle's end: the edge's end points are on the same side.
        assert_eq!(segment(&mut e, [20.0, 0.0, 0.0], [0.0, 20.0, 0.0], 0), -1);
        // Collinear segments never count.
        assert_eq!(segment(&mut e, [-5.0, 0.0, 0.0], [5.0, 0.0, 0.0], 1), -1);
    }

    // --- NiPoint2 and segment helpers ------------------------------------------

    #[test]
    fn segment_constructor_copies_both_points() {
        let mut e = engine();
        let first = point(&mut e, [1.0, 2.0, 3.0]);
        let second = point(&mut e, [4.0, 5.0, 6.0]);
        let target = Ptr::new(e.mem.alloc(24));
        assert_eq!(fn_00698580(&mut e, target, first, second), target);
        assert_eq!(read_point(&e, target.addr()), [1.0, 2.0, 3.0]);
        assert_eq!(read_point(&e, target.addr() + 12), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn two_element_array_is_vector_constructed() {
        let mut e = engine();
        let calls = recorder(&mut e, VECTOR_CONSTRUCTOR_ITERATOR, 0);
        let target = Ptr::new(e.mem.alloc(40));
        assert_eq!(fn_006985c0(&mut e, target), target);
        assert_eq!(
            *calls.borrow(),
            vec![vec![target.addr() + 4, 16, 2, ELEMENT_16_CONSTRUCTOR]]
        );
    }

    #[test]
    fn point2_copy_and_product() {
        let mut e = engine();
        let source = point(&mut e, [1.5, -2.5, 99.0]);
        let target = point(&mut e, [0.0, 0.0, 7.0]);
        assert_eq!(fn_006985f0(&mut e, target, source), target);
        // Only x and y are copied.
        assert_eq!(read_point(&e, target.addr()), [1.5, -2.5, 7.0]);

        let a = point(&mut e, [1.0, 2.0, 0.0]);
        let b = point(&mut e, [3.0, 4.0, 0.0]);
        let result = point(&mut e, [0.0, 0.0, 7.0]);
        assert_eq!(fn_00698620(&mut e, a, result, b), result);
        // (1 * 3 + 2 * 4, 2 * 3 - 1 * 4)
        assert_eq!(read_point(&e, result.addr()), [11.0, 2.0, 7.0]);

        // Five calls compared with FalloutNV.exe 1.4.0.525 through nv-call;
        // compare the result words bit for bit (source vectors/results are
        // in the translator audit, not repository artifacts).
        let vectors = [
            ([1.5, -2.5], [3.0, 4.0], [0xc0b0_0000, 0xc158_0000]),
            ([0.0, 0.0], [2.0, 3.0], [0x0000_0000, 0x0000_0000]),
            ([-10.0, 20.0], [0.25, -0.5], [0xc148_0000, 0x0000_0000]),
            ([0.1, 0.2], [0.3, 0.4], [0x3de1_47af, 0x3ca3_d70b]),
            (
                [10_000.125, -10_000.25],
                [0.000_125, -0.000_25],
                [0x4070_0148, 0x3fa0_0000],
            ),
        ];
        for (left, right, expected_bits) in vectors {
            let a = point(&mut e, [left[0], left[1], 0.0]);
            let b = point(&mut e, [right[0], right[1], 0.0]);
            let result = point(&mut e, [0.0; 3]);
            fn_00698620(&mut e, a, result, b);
            assert_eq!(
                [
                    e.mem.f32(result.addr()).to_bits(),
                    e.mem.f32(result.addr() + 4).to_bits()
                ],
                expected_bits
            );
        }
    }
}
