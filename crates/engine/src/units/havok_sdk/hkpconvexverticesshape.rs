//! `Havok SDK/hkpconvexverticesshape.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkpConvexVerticesShape` (Xbox PDB, 0x70 bytes) keeps its vertices as `m_rotatedVertices`
//! (+0x40, an `hkArray` of `FourVectors`: 0x30 bytes holding the x, y and z of four vertices as
//! three four-lane vectors at +0, +0x10 and +0x20), `m_numVertices` (+0x4c), the
//! `m_convexPiece` (+0x50, a `hkpCompressedMeshShape::ConvexPiece`) and `m_planeEquations`
//! (+0x54, an `hkArray` of `hkVector4`). Vertex `i` is lane `i & 3` of the `FourVectors` number
//! `i >> 2`. Copies that go through x87 (`FLD`/`FSTP`) quiet signalling NaNs
//! ([`x87_quiet`]); the SSE ones are bit for bit. x87 code compares the way its `FNSTSW`/`TEST
//! AH` pairs read the flags (NaN included). `RSQRTSS` is the host CPU's estimate
//! (`rsqrt_estimate`), as elsewhere in the crate. The profiling records `castRayImpl` writes use
//! the host clock for `RDTSC`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::tesobjectrefr::rsqrt_estimate;

/// `m_radius` (+0x10, Xbox PDB), `m_aabbCenter` (+0x30), `m_rotatedVertices` (+0x40: data +0x40,
/// size +0x44), `m_numVertices` (+0x4c), `m_convexPiece` (+0x50) and `m_planeEquations` (+0x54:
/// data +0x54, size +0x58, capacity word +0x5c).
const RADIUS: u32 = 0x10;
const AABB_CENTRE: u32 = 0x30;
const ROTATED_VERTICES: u32 = 0x40;
const ROTATED_VERTICES_SIZE: u32 = 0x44;
const NUM_VERTICES: u32 = 0x4c;
const CONVEX_PIECE: u32 = 0x50;
const PLANE_EQUATIONS: u32 = 0x54;
/// `hkpCompressedMeshShape::ConvexPiece` (Xbox PDB): `m_faceVertices` data at +0x1c and
/// `m_faceOffsets` data and size at +0x28 and +0x2c.
const PIECE_FACE_VERTICES: u32 = 0x1c;
const PIECE_FACE_OFFSETS: u32 = 0x28;
const PIECE_FACE_OFFSETS_SIZE: u32 = 0x2c;
/// The size of `hkpConvexVerticesShape::FourVectors`.
const FOUR_VECTORS: u32 = 0x30;

/// The memory router lookup (`00c85750`, no arguments); its field `+0x10` is the allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl `(allocator, array, element size)`).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `_ftol2_sse` (`00ec62c0`): the float in ST0 truncated to an integer.
const FTOL: u32 = 0x00ec_62c0;
/// The call `getAabbImpl` forwards to (`00d26230`, cdecl `(transform, a, b, margin, out)`).
const AABB_OF_SEGMENT: u32 = 0x00d2_6230;
/// `TlsGetValue` import slot and the TLS index of the profiling record stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const PROFILE_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcConvexVert"` and `"Et"`.
const PROFILE_BEGIN_CONVEX: u32 = 0x010d_0fb0;
const PROFILE_END: u32 = 0x010c_41f8;

/// `float` constants: 0.5, 0.0, 3.0 (the `RSQRTSS` refinement), -1.0 and the vector constant of
/// four 0.0.
const HALF: u32 = 0x0101_6248;
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const MINUS_ONE: u32 = 0x0101_2054;
const ZEROS: u32 = 0x010c_7160;
/// `getSupportingVertexImpl`: the function-local static of four copies of the float at
/// `0104e148` (its guard byte is `01268fb0`, the copies `01268fa0`), the vector `(0.5, 1.5, 2.5,
/// 3.5)` the lane numbers start from and the vector of four 4.0 they advance by.
const BEST_INITIAL_SOURCE: u32 = 0x0104_e148;
const BEST_INITIAL: u32 = 0x0126_8fa0;
const BEST_INITIAL_GUARD: u32 = 0x0126_8fb0;
const LANE_NUMBERS: u32 = 0x010d_9fe0;
const LANE_STEP: u32 = 0x010d_0fa0;

// `hkpShapeRayCastInput` (Xbox PDB): `m_from` +0x00, `m_to` +0x10.
const RAY_FROM: u32 = 0x00;
const RAY_TO: u32 = 0x10;
// `hkpShapeRayCastOutput` (Xbox PDB): `m_normal` +0x00, `m_hitFraction` +0x10, `m_shapeKeys`
// +0x20, `m_shapeKeyIndex` +0x40.
const OUT_NORMAL: u32 = 0x00;
const OUT_HIT_FRACTION: u32 = 0x10;
const OUT_SHAPE_KEYS: u32 = 0x20;
const OUT_SHAPE_KEY_INDEX: u32 = 0x40;

type Vec4 = [f32; 4];

fn load_vec(e: &Engine, addr: u32) -> Vec4 {
    std::array::from_fn(|lane| e.mem.f32(addr + 4 * lane as u32))
}

fn store_vec(e: &mut Engine, addr: u32, v: Vec4) {
    for (lane, value) in v.into_iter().enumerate() {
        e.mem.set_f32(addr + 4 * lane as u32, value);
    }
}

/// A 16-byte copy (`MOVAPS` load and store), bit for bit.
fn copy_vec(e: &mut Engine, from: u32, to: u32) {
    for lane in 0..4 {
        let word = e.mem.u32(from + 4 * lane);
        e.mem.set_u32(to + 4 * lane, word);
    }
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a profiling record (`label`, time stamp) to the thread's record stream (`TlsGetValue`
/// of the index at `0126835c`) while its cursor (`+4`) is below its limit (`+0xC`); a record is 12
/// bytes.
fn profile_record(e: &mut Engine, label: u32) {
    let slot: u32 = e.global(PROFILE_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![slot]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero guard:
/// `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == zero`.
fn inverse_length(e: &Engine, sum: f32) -> f32 {
    let zero: f32 = e.global(ZERO);
    let three: f32 = e.global(THREE);
    let half: f32 = e.global(HALF);
    let estimate = rsqrt_estimate(sum);
    let step = three - (sum * estimate) * estimate;
    let value = (half * estimate) * step;
    if sum == zero {
        0.0
    } else {
        value
    }
}

/// The address of the `FourVectors` holding vertex `index` and the lane inside it.
fn vertex_slot(e: &Engine, this: u32, index: u32) -> (u32, u32) {
    let data = e.mem.u32(this + ROTATED_VERTICES);
    (
        data.wrapping_add((index >> 2).wrapping_mul(FOUR_VECTORS)),
        index & 3,
    )
}

/// Vertex `index` as `x87` copies read it: `x`, `y` and `z` through `FLD`/`FSTP`.
fn vertex_through_x87(e: &Engine, this: u32, index: u32) -> [f32; 3] {
    let (block, lane) = vertex_slot(e, this, index);
    std::array::from_fn(|axis| x87_quiet(e.mem.f32(block + 0x10 * axis as u32 + 4 * lane)))
}

// Translated from 00d58d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getNumCollisionSpheresImpl` (Xbox PDB): `m_numVertices`.
pub fn hkp_convex_vertices_shape_get_num_collision_spheres_impl(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + NUM_VERTICES)
}

// Translated from 00d58d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getCentreImpl` (Xbox PDB): copies `m_aabbCenter`.
pub fn hkp_convex_vertices_shape_get_centre_impl(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_vec(e, this.addr() + AABB_CENTRE, out.addr());
}

// Translated from 00d58d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getFirstVertex` (Xbox PDB): the first vertex (lane 0 of the first
/// `FourVectors`) with `w` = 0.0, copied through x87.
pub fn hkp_convex_vertices_shape_get_first_vertex(e: &mut Engine, this: Ptr, out: Ptr) {
    let vertex = vertex_through_x87(e, this.addr(), 0);
    store_vec(e, out.addr(), [vertex[0], vertex[1], vertex[2], 0.0]);
}

// Translated from 00d58d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getCollisionSpheresImpl` (Xbox PDB): writes one sphere per vertex to
/// `out`: the vertex's x, y and z with `m_radius` in `w`, all copied bit for bit. Nothing for a
/// count of zero or less.
pub fn hkp_convex_vertices_shape_get_collision_spheres_impl(e: &mut Engine, this: Ptr, out: Ptr) {
    let me = this.addr();
    let count = e.mem.i32(me + NUM_VERTICES);
    let radius_bits = e.mem.u32(me + RADIUS);
    let mut sphere = out.addr();
    for index in 0..count.max(0) as u32 {
        let (block, lane) = vertex_slot(e, me, index);
        for axis in 0..3 {
            let word = e.mem.u32(block + 0x10 * axis + 4 * lane);
            e.mem.set_u32(sphere + 4 * axis, word);
        }
        e.mem.set_u32(sphere + 0xc, radius_bits);
        sphere = sphere.wrapping_add(0x10);
    }
}

// Translated from 00d58ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getSupportingVertexImpl` (Xbox PDB): finds the vertex with the
/// greatest dot product with `direction` and writes it to `out`, its `w` lane holding the vertex
/// index as the bits `0x3f000000 | index`.
///
/// Each `FourVectors` gives four dot products `(y * dy + x * dx) + z * dz`; per lane the greatest
/// so far (strictly greater wins, NaN never does) is kept with its lane number as a float
/// (`0.5, 1.5, ...`, advancing by 4.0 per block), starting from four copies of the function's
/// static initial value (a `float` read from `0104e148` once, guarded by bit 0 of `01268fb0`).
/// The lanes are then reduced pairwise with the `FCOMPP` tests (the second lane of a pair wins
/// ties) and the winner's number is truncated by `_ftol2` (`00ec62c0`).
pub fn hkp_convex_vertices_shape_get_supporting_vertex_impl(
    e: &mut Engine,
    this: Ptr,
    direction: Ptr,
    out: Ptr,
) {
    let me = this.addr();
    if e.mem.u8(BEST_INITIAL_GUARD) & 1 == 0 {
        let initial = x87_quiet(e.global::<f32>(BEST_INITIAL_SOURCE));
        let guard = e.mem.u32(BEST_INITIAL_GUARD);
        e.mem.set_u32(BEST_INITIAL_GUARD, guard | 1);
        for lane in 0..4 {
            e.mem.set_f32(BEST_INITIAL + 4 * lane, initial);
        }
    }
    let data = e.mem.u32(me + ROTATED_VERTICES);
    let blocks = e.mem.i32(me + ROTATED_VERTICES_SIZE).wrapping_mul(4);
    let direction = load_vec(e, direction.addr());
    let mut best = load_vec(e, BEST_INITIAL);
    let mut lane_numbers = load_vec(e, LANE_NUMBERS);
    let lane_step = load_vec(e, LANE_STEP);
    let mut best_numbers: Vec4 = [0.0; 4];
    if blocks > 0 {
        let block_count = (((blocks - 1) as u32) >> 2) + 1;
        for block in 0..block_count {
            let base = data.wrapping_add(block.wrapping_mul(FOUR_VECTORS));
            let x = load_vec(e, base);
            let y = load_vec(e, base + 0x10);
            let z = load_vec(e, base + 0x20);
            let dots: Vec4 = std::array::from_fn(|lane| {
                (y[lane] * direction[1] + x[lane] * direction[0]) + z[lane] * direction[2]
            });
            // `CMPLTPS best, dot`: strictly greater wins, NaN never does.
            let greater: [bool; 4] = std::array::from_fn(|lane| best[lane] < dots[lane]);
            best = std::array::from_fn(|lane| {
                if greater[lane] {
                    dots[lane]
                } else {
                    best[lane]
                }
            });
            best_numbers = std::array::from_fn(|lane| {
                if greater[lane] {
                    lane_numbers[lane]
                } else {
                    best_numbers[lane]
                }
            });
            lane_numbers = std::array::from_fn(|lane| lane_numbers[lane] + lane_step[lane]);
        }
    }
    // The first pair: lane 1 unless it is strictly below lane 0 (`FCOMPP`, `JP`).
    let first = if best[1] < best[0] { 0 } else { 1 };
    let second = if best[3] < best[2] { 2 } else { 3 };
    let winner = if best[second] < best[first] {
        first
    } else {
        second
    };
    let number = e.call(FTOL, &args![f64::from(best_numbers[winner])]).i32() as u32;
    let (block, lane) = vertex_slot(e, me, number);
    let vertex = std::array::from_fn::<f32, 3, _>(|axis| {
        x87_quiet(e.mem.f32(block + 0x10 * axis as u32 + 4 * lane))
    });
    store_vec(e, out.addr(), [vertex[0], vertex[1], vertex[2], 0.0]);
    e.mem.set_u32(out.addr() + 0xc, number | 0x3f00_0000);
}

// Translated from 00d59060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::convertVertexIdsToVerticesImpl` (Xbox PDB): for each of `count`
/// 16-bit vertex ids writes the vertex (x, y, z copied through x87) to `out` with the bits
/// `0x3f000000 | id` in `w`. Nothing for a count of zero or less.
pub fn hkp_convex_vertices_shape_convert_vertex_ids_to_vertices_impl(
    e: &mut Engine,
    this: Ptr,
    ids: Ptr,
    count: i32,
    out: Ptr,
) {
    let mut id_address = ids.addr();
    let mut out_address = out.addr();
    for _ in 0..count.max(0) {
        let id = u32::from(e.mem.u16(id_address));
        let vertex = vertex_through_x87(e, this.addr(), id);
        store_vec(
            e,
            out_address,
            [vertex[0], vertex[1], vertex[2], f32::from_bits(0)],
        );
        e.mem.set_u32(out_address + 0xc, id | 0x3f00_0000);
        id_address = id_address.wrapping_add(2);
        out_address = out_address.wrapping_add(0x10);
    }
}

// Translated from 00d591d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::getAabbImpl` (Xbox PDB): forwards to `00d26230` with the transform,
/// the addresses `this + 0x20` and `this + 0x30` (`m_aabbHalfExtents` and `m_aabbCenter`), the
/// margin `m_radius + tolerance` (rounded to `float`) and `out`.
pub fn hkp_convex_vertices_shape_get_aabb_impl(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    tolerance: f32,
    out: Ptr,
) {
    let radius = e.mem.f32(this.addr() + RADIUS);
    let margin = (f64::from(radius) + f64::from(tolerance)) as f32;
    e.call(
        AABB_OF_SEGMENT,
        &args![
            transform,
            this.addr() + 0x20,
            this.addr() + 0x30,
            margin,
            out
        ],
    );
}

/// Builds `m_planeEquations` from the convex piece's faces (`castRayImpl`, first time): for each
/// face but the last, the three vertex indices at `faceVertices + faceOffsets[i]` give the
/// plane through them: the normal `(v2 - v0) x (v1 - v0)` normalised with the refined `RSQRTSS`
/// (`+0.0` for a zero normal) and `w = 0.0 - normal . v0`; planes are appended, growing the
/// array (`00c90940`) when it is full.
fn build_plane_equations(e: &mut Engine, this: u32, piece: u32) {
    let face_vertices = e.mem.u32(piece + PIECE_FACE_VERTICES);
    let mut offsets = e.mem.u32(piece + PIECE_FACE_OFFSETS);
    let face_count = e.mem.i32(piece + PIECE_FACE_OFFSETS_SIZE).wrapping_sub(1);
    let array = this + PLANE_EQUATIONS;
    for _ in 0..face_count.max(0) {
        let face = face_vertices.wrapping_add(u32::from(e.mem.u16(offsets)));
        let corners: [[f32; 3]; 3] = std::array::from_fn(|corner| {
            let index = u32::from(e.mem.u8(face + corner as u32));
            vertex_through_x87(e, this, index)
        });
        let [v0, v1, v2] = corners;
        let a: [f32; 3] = std::array::from_fn(|axis| v1[axis] - v0[axis]);
        let b: [f32; 3] = std::array::from_fn(|axis| v2[axis] - v0[axis]);
        let normal = [
            b[2] * a[1] - b[1] * a[2],
            b[0] * a[2] - b[2] * a[0],
            b[1] * a[0] - b[0] * a[1],
        ];
        let squared = normal[2] * normal[2] + (normal[1] * normal[1] + normal[0] * normal[0]);
        let inverse = inverse_length(e, squared);
        let plane: [f32; 3] = normal.map(|lane| inverse * lane);
        let along = plane[2] * v0[2] + (plane[1] * v0[1] + plane[0] * v0[0]);
        let zero: f32 = e.global(ZEROS);
        let distance = x87_quiet(zero - along);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        if e.mem.u32(array + 4) == e.mem.u32(array + 8) & 0x3fff_ffff {
            let allocator = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 0x10u32]);
        }
        offsets = offsets.wrapping_add(2);
        let size = e.mem.u32(array + 4);
        let slot = e.mem.u32(array).wrapping_add(size.wrapping_mul(0x10));
        store_vec(e, slot, [plane[0], plane[1], plane[2], distance]);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
    }
}

// Translated from 00d592a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConvexVerticesShape::castRayImpl` (Xbox PDB): casts the ray `input`
/// (`hkpShapeRayCastInput`: `m_from`, `m_to`) against the convex hull and updates `output`
/// (`hkpShapeRayCastOutput`) when it enters the hull before `output.m_hitFraction`. Writes the
/// hit flag (`hkBool`) to `result` and returns `result`; a profiling record `"TtrcConvexVert"` is
/// written first and `"Et"` last.
///
/// The first time (no plane equations yet, a convex piece present) the planes are built from the
/// piece (`build_plane_equations`). Then, going through the planes from the last to the first, the
/// ray's signed distances `d0` (start) and `d1` (end) to a plane `(n, w)` are `(n_y y + n_x x) +
/// (n_z z + w)`. Both in front (`>= 0`) is a miss; both behind is no constraint; start in front
/// and end behind enters at `t = d0 / (d0 - d1)`, which raises the entry fraction when it is not
/// below it and keeps that plane as the normal; start behind and end in front exits at the same
/// `t`, lowering the exit fraction (initially `m_hitFraction`) when it is not above it. After
/// such a crossing an exit below the entry is a miss. A hit needs an entry fraction that is not
/// below zero (it starts at -1.0, NaN counts as a hit); it is stored as the hit fraction with the
/// plane as the normal (`w` lane included) and the shape key slot `m_shapeKeyIndex` set to -1.
pub fn hkp_convex_vertices_shape_cast_ray_impl(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    input: Ptr,
    output: Ptr,
) -> Ptr {
    profile_record(e, PROFILE_BEGIN_CONVEX);
    let hit = cast_ray_against_hull(e, this.addr(), input.addr(), output.addr());
    profile_record(e, PROFILE_END);
    e.mem.set_u8(result.addr(), hit as u8);
    result
}

/// The body of `castRayImpl`.
#[allow(clippy::neg_cmp_op_on_partial_ord)] // the exe's flag tests, NaN included
fn cast_ray_against_hull(e: &mut Engine, me: u32, input: u32, out: u32) -> bool {
    if e.mem.u32(me + PLANE_EQUATIONS + 4) == 0 {
        let piece = e.mem.u32(me + CONVEX_PIECE);
        if piece != 0 {
            build_plane_equations(e, me, piece);
        }
    }
    let mut entry = x87_quiet(e.global::<f32>(MINUS_ONE));
    let mut exit = x87_quiet(e.mem.f32(out + OUT_HIT_FRACTION));
    let mut normal: Vec4 = [0.0; 4];
    let from = load_vec(e, input + RAY_FROM);
    let to = load_vec(e, input + RAY_TO);
    // The signed distance of `point` to `plane`: `(n_y y + n_x x) + (n_z z + w)`.
    let distance = |plane: Vec4, point: Vec4| -> f32 {
        (plane[1] * point[1] + plane[0] * point[0]) + (plane[2] * point[2] + plane[3])
    };
    let planes_size = e.mem.i32(me + PLANE_EQUATIONS + 4);
    let mut index = planes_size.wrapping_sub(1);
    if index < 0 {
        return false;
    }
    while index >= 0 {
        let data = e.mem.u32(me + PLANE_EQUATIONS);
        let plane = load_vec(e, data.wrapping_add((index as u32) << 4));
        let start = distance(plane, from);
        let end = distance(plane, to);
        // `FCOM` against 0.0 leaves C0 set for "below" and for unordered.
        let start_behind = start < 0.0 || start.is_nan();
        let end_behind = end < 0.0 || end.is_nan();
        if start_behind {
            if !end_behind {
                // Leaving through this plane.
                let crossing = (f64::from(start) / (f64::from(start) - f64::from(end))) as f32;
                // `JZ` skips the update when the crossing is above the exit (ordered).
                if !(crossing > exit) {
                    exit = crossing;
                }
                if exit < entry {
                    return false;
                }
            }
        } else if end_behind {
            // Entering through this plane.
            let crossing = (f64::from(start) / (f64::from(start) - f64::from(end))) as f32;
            if crossing >= entry {
                entry = crossing;
                normal = plane;
            }
            if exit < entry {
                return false;
            }
        } else {
            return false;
        }
        index -= 1;
    }
    // `FCOM` of the entry against 0.0, `JP`: a hit unless the entry is below zero.
    if entry < 0.0 {
        return false;
    }
    let key_index = e.mem.u32(out + OUT_SHAPE_KEY_INDEX);
    e.mem.set_f32(out + OUT_HIT_FRACTION, entry);
    for (lane, value) in normal.into_iter().enumerate() {
        e.mem.set_f32(out + OUT_NORMAL + 4 * lane as u32, value);
    }
    e.mem.set_u32(
        out.wrapping_add(key_index.wrapping_mul(4))
            .wrapping_add(OUT_SHAPE_KEYS),
        0xffff_ffff,
    );
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d58d10,
            hkp_convex_vertices_shape_get_num_collision_spheres_impl(Ptr) -> u32
        ),
        entry!(
            0x00d58d20,
            hkp_convex_vertices_shape_get_centre_impl(Ptr, Ptr)
        ),
        entry!(
            0x00d58d60,
            hkp_convex_vertices_shape_get_first_vertex(Ptr, Ptr)
        ),
        entry!(
            0x00d58d80,
            hkp_convex_vertices_shape_get_collision_spheres_impl(Ptr, Ptr)
        ),
        entry!(
            0x00d58ee0,
            hkp_convex_vertices_shape_get_supporting_vertex_impl(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d59060,
            hkp_convex_vertices_shape_convert_vertex_ids_to_vertices_impl(Ptr, Ptr, i32, Ptr)
        ),
        entry!(
            0x00d591d0,
            hkp_convex_vertices_shape_get_aabb_impl(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00d592a0,
            hkp_convex_vertices_shape_cast_ray_impl(Ptr, Ptr, Ptr, Ptr) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e.set_global(HALF, 0.5f32);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(BEST_INITIAL_SOURCE, f32::from_bits(0xff7f_ffee));
        store_vec(&mut e, LANE_NUMBERS, [0.5, 1.5, 2.5, 3.5]);
        store_vec(&mut e, LANE_STEP, [4.0; 4]);
        e
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// A hull of six vertices in two `FourVectors`: (0,0,0), (1,0,0), (0,1,0), (0,0,1) and
    /// (2,3,4), (-1,-2,-3), the unused lanes zero; `m_radius` 0.25.
    fn hull(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x60);
        store_vec(e, data, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, data + 0x10, [0.0, 0.0, 1.0, 0.0]);
        store_vec(e, data + 0x20, [0.0, 0.0, 0.0, 1.0]);
        store_vec(e, data + 0x30, [2.0, -1.0, 0.0, 0.0]);
        store_vec(e, data + 0x40, [3.0, -2.0, 0.0, 0.0]);
        store_vec(e, data + 0x50, [4.0, -3.0, 0.0, 0.0]);
        e.mem.set_u32(this + ROTATED_VERTICES, data);
        e.mem.set_u32(this + ROTATED_VERTICES_SIZE, 2);
        e.mem.set_u32(this + NUM_VERTICES, 6);
        e.mem.set_f32(this + RADIUS, 0.25);
        store_vec(e, this + AABB_CENTRE, [1.0, 2.0, 3.0, 4.0]);
        Ptr::new(this)
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(called, _)| *called == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn assert_near(actual: Vec4, expected: Vec4, tolerance: f32) {
        for lane in 0..4 {
            assert!(
                (actual[lane] - expected[lane]).abs() <= tolerance,
                "{actual:?} != {expected:?}"
            );
        }
    }

    #[test]
    fn num_collision_spheres_is_the_vertex_count() {
        let mut e = engine();
        let this = hull(&mut e);
        assert_eq!(e.call(0x00d58d10, &args![this]).u32(), 6);
    }

    #[test]
    fn centre_is_the_aabb_centre() {
        let mut e = engine();
        let this = hull(&mut e);
        let out = e.mem.alloc(16);
        e.call(0x00d58d20, &args![this, out]);
        assert_eq!(load_vec(&e, out), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn first_vertex_is_quieted_and_has_a_zero_w() {
        let mut e = engine();
        let this = hull(&mut e);
        let data = e.mem.u32(this.addr() + ROTATED_VERTICES);
        e.mem.set_u32(data, 0x7f80_0001);
        e.mem.set_f32(data + 0x10, 5.0);
        e.mem.set_f32(data + 0x20, 6.0);
        let out = e.mem.alloc(16);
        e.mem.set_f32(out + 0xc, 9.0);
        e.call(0x00d58d60, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0x7fc0_0001);
        assert_eq!(e.mem.f32(out + 4), 5.0);
        assert_eq!(e.mem.f32(out + 8), 6.0);
        assert_eq!(e.mem.u32(out + 0xc), 0);
    }

    #[test]
    fn collision_spheres_are_the_vertices_with_the_radius() {
        let mut e = engine();
        let this = hull(&mut e);
        let out = e.mem.alloc(0x60);
        e.call(0x00d58d80, &args![this, out]);
        let expected = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [2.0, 3.0, 4.0],
            [-1.0, -2.0, -3.0],
        ];
        for (index, vertex) in expected.iter().enumerate() {
            let sphere = load_vec(&e, out + 0x10 * index as u32);
            assert_eq!(sphere, [vertex[0], vertex[1], vertex[2], 0.25]);
        }
        // No vertices, nothing written.
        e.mem.set_u32(this.addr() + NUM_VERTICES, 0);
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d58d80, &args![this, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn supporting_vertex_is_the_furthest_along_the_direction() {
        let mut e = engine();
        e.register_double(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            ret(value as i32 as u32)
        });
        let this = hull(&mut e);
        let out = e.mem.alloc(16);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 1.0, 1.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0004);
        // The static of four initial values was filled in once.
        assert_eq!(e.mem.u8(BEST_INITIAL_GUARD) & 1, 1);
        assert_eq!(load_vec(&e, BEST_INITIAL), [f32::from_bits(0xff7f_ffee); 4]);
        store_vec(&mut e, direction, [-1.0, -1.0, -1.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [-1.0, -2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0005);
    }

    #[test]
    fn supporting_vertex_without_vertices_is_the_first_one() {
        let mut e = engine();
        e.register_double(FTOL, |_, _| ret(0));
        let this = hull(&mut e);
        e.mem.set_u32(this.addr() + ROTATED_VERTICES_SIZE, 0);
        let out = e.mem.alloc(16);
        let direction = e.mem.alloc(16);
        store_vec(&mut e, direction, [1.0, 0.0, 0.0, 0.0]);
        e.call(0x00d58ee0, &args![this, direction, out]);
        assert_eq!(load_vec(&e, out)[..3], [0.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0000);
    }

    #[test]
    fn vertex_ids_expand_to_vertices() {
        let mut e = engine();
        let this = hull(&mut e);
        let ids = e.mem.alloc(8);
        e.mem.set_u16(ids, 4);
        e.mem.set_u16(ids + 2, 5);
        e.mem.set_u16(ids + 4, 1);
        let out = e.mem.alloc(0x30);
        e.call(0x00d59060, &args![this, ids, 3i32, out]);
        assert_eq!(load_vec(&e, out)[..3], [2.0, 3.0, 4.0]);
        assert_eq!(e.mem.u32(out + 0xc), 0x3f00_0004);
        assert_eq!(load_vec(&e, out + 0x10)[..3], [-1.0, -2.0, -3.0]);
        assert_eq!(e.mem.u32(out + 0x1c), 0x3f00_0005);
        assert_eq!(load_vec(&e, out + 0x20)[..3], [1.0, 0.0, 0.0]);
        assert_eq!(e.mem.u32(out + 0x2c), 0x3f00_0001);
        let untouched = e.mem.alloc(0x10);
        e.call(0x00d59060, &args![this, ids, 0i32, untouched]);
        e.call(0x00d59060, &args![this, ids, -1i32, untouched]);
        assert_eq!(e.mem.u32(untouched + 0xc), 0);
    }

    #[test]
    fn aabb_forwards_the_margin_and_the_box_vectors() {
        let mut e = engine();
        let this = hull(&mut e);
        e.register_double(AABB_OF_SEGMENT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00d591d0, &args![this, 0x1111u32, 0.5f32, 0x2222u32]);
        assert_eq!(
            calls_to(&e, AABB_OF_SEGMENT),
            vec![vec![
                0x1111,
                this.addr() + 0x20,
                this.addr() + 0x30,
                0.75f32.to_bits(),
                0x2222
            ]]
        );
    }

    // ---- castRayImpl

    /// A profile stream with room for two records; the TLS lookup returns it.
    fn profile_stream(e: &mut Engine) -> u32 {
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x38);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, a| {
            assert_eq!(a[0], 3);
            ret(stream)
        });
        stream
    }

    fn assert_profile_records(e: &Engine, stream: u32) {
        assert_eq!(e.mem.u32(stream + 0x20), PROFILE_BEGIN_CONVEX);
        assert_eq!(e.mem.u32(stream + 0x2c), PROFILE_END);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x38);
    }

    struct Cast {
        hit: bool,
        output: u32,
        stream: u32,
    }

    /// Casts `from` -> `to` at `this` with `hit_fraction` already in the output.
    fn cast(e: &mut Engine, this: Ptr, from: Vec4, to: Vec4, hit_fraction: f32) -> Cast {
        let stream = profile_stream(e);
        let input = e.mem.alloc(0x30);
        store_vec(e, input, from);
        store_vec(e, input + 0x10, to);
        let output = e.mem.alloc(0x50);
        e.mem.set_f32(output + 0x10, hit_fraction);
        e.mem.set_u32(output + 0x40, 2);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let returned = e
            .call(0x00d592a0, &args![this, result, input, output])
            .u32();
        assert_eq!(returned, result);
        Cast {
            hit: e.mem.u8(result) != 0,
            output,
            stream,
        }
    }

    /// The hull with the six planes of the cube of half width 1 around the origin.
    fn cube(e: &mut Engine) -> Ptr {
        let this = hull(e);
        let planes = e.mem.alloc(0x60);
        let normals = [
            [1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, -1.0],
        ];
        for (index, normal) in normals.iter().enumerate() {
            store_vec(
                e,
                planes + 0x10 * index as u32,
                [normal[0], normal[1], normal[2], -1.0],
            );
        }
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS, planes);
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS + 4, 6);
        e.mem.set_u32(this.addr() + PLANE_EQUATIONS + 8, 6);
        this
    }

    #[test]
    fn cast_ray_enters_through_the_nearest_face() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            1.0,
        );
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 1.0 / 3.0).abs() < 1e-6);
        assert_eq!(load_vec(&e, c.output), [1.0, 0.0, 0.0, -1.0]);
        assert_eq!(e.mem.u32(c.output + 0x20 + 2 * 4), 0xffff_ffff);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_in_front_of_a_face_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 3.0, 0.0, 0.0],
            [3.0, -3.0, 0.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 1.0);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_behind_a_nearer_hit_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            0.2,
        );
        assert!(!c.hit);
        assert_eq!(e.mem.f32(c.output + 0x10), 0.2);
    }

    #[test]
    fn cast_ray_starting_inside_has_no_entry_and_misses() {
        let mut e = engine();
        let this = cube(&mut e);
        let c = cast(
            &mut e,
            this,
            [0.0, 0.0, 0.0, 0.0],
            [0.5, 0.0, 0.0, 0.0],
            1.0,
        );
        assert!(!c.hit);
    }

    #[test]
    fn cast_ray_without_planes_or_piece_misses() {
        let mut e = engine();
        let this = hull(&mut e);
        let c = cast(
            &mut e,
            this,
            [3.0, 0.1, 0.2, 0.0],
            [-3.0, 0.1, 0.2, 0.0],
            1.0,
        );
        assert!(!c.hit);
        assert_profile_records(&e, c.stream);
    }

    #[test]
    fn cast_ray_builds_the_plane_equations_from_the_convex_piece() {
        let mut e = engine();
        let this = hull(&mut e);
        // Two faces (the offsets array has three entries, the last is not a face): vertices
        // (0,1,2) and (0,2,3) at byte offsets 0 and 3.
        let piece = e.mem.alloc(0x40);
        let faces = e.mem.alloc(8);
        for (index, vertex) in [0u8, 1, 2, 0, 2, 3].into_iter().enumerate() {
            e.mem.set_u8(faces + index as u32, vertex);
        }
        let offsets = e.mem.alloc(8);
        e.mem.set_u16(offsets, 0);
        e.mem.set_u16(offsets + 2, 3);
        e.mem.set_u16(offsets + 4, 6);
        e.mem.set_u32(piece + PIECE_FACE_VERTICES, faces);
        e.mem.set_u32(piece + PIECE_FACE_OFFSETS, offsets);
        e.mem.set_u32(piece + PIECE_FACE_OFFSETS_SIZE, 3);
        e.mem.set_u32(this.addr() + CONVEX_PIECE, piece);
        // The router's allocator and the array growth: capacity 0, so the first plane grows it.
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, 0xa110c);
        e.register_double(MEMORY_ROUTER, move |_, _| ret(router));
        e.register_double(ARRAY_RESERVE_MORE, |e, a| {
            assert_eq!(a.len(), 3);
            let storage = e.mem.alloc(0x80);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        let c = cast(
            &mut e,
            this,
            [-0.5, -0.5, 2.0, 0.0],
            [-0.5, -0.5, -2.0, 0.0],
            1.0,
        );
        let array = this.addr() + PLANE_EQUATIONS;
        assert_eq!(e.mem.u32(array + 4), 2);
        let data = e.mem.u32(array);
        assert_near(load_vec(&e, data), [0.0, 0.0, 1.0, 0.0], 1e-6);
        assert_near(load_vec(&e, data + 0x10), [1.0, 0.0, 0.0, 0.0], 1e-6);
        let reserve_calls = calls_to(&e, ARRAY_RESERVE_MORE);
        assert_eq!(reserve_calls, vec![vec![0xa110c, array, 0x10]]);
        // The ray enters through the first plane (z = 0) halfway.
        assert!(c.hit);
        assert!((e.mem.f32(c.output + 0x10) - 0.5).abs() < 1e-6);
        assert_near(load_vec(&e, c.output), [0.0, 0.0, 1.0, 0.0], 1e-6);
        // A second cast does not build the planes again.
        let again = cast(
            &mut e,
            this,
            [-0.5, -0.5, 2.0, 0.0],
            [-0.5, -0.5, -2.0, 0.0],
            1.0,
        );
        assert!(again.hit);
        assert_eq!(e.mem.u32(array + 4), 2);
        assert!(calls_to(&e, ARRAY_RESERVE_MORE).is_empty());
    }

    #[test]
    fn profile_record_is_skipped_when_the_stream_is_full() {
        let mut e = engine();
        let stream = e.mem.alloc(0x40);
        e.mem.set_u32(stream + 4, stream + 0x20);
        e.mem.set_u32(stream + 0xc, stream + 0x20);
        e.set_global(PROFILE_SLOT, 3u32);
        e.register_double(TLS_GET_VALUE, move |_, _| ret(stream));
        profile_record(&mut e, 0x1234);
        assert_eq!(e.mem.u32(stream + 4), stream + 0x20);
        assert_eq!(e.mem.u32(stream + 0x20), 0);
    }
}
