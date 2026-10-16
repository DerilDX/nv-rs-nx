//! `Havok SDK/run_00e2b070` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! These functions belong to a convex-hull growing routine over two convex
//! shapes (the next unit, `run_00df9840`, drives it); the executable has no
//! names for them and no Xbox PDB entry, so the routines are named by address
//! and the fields below come from this code only.
//!
//! The polytope object (`this` of `00e2d0d0`, `00e2d1b0`, `00e2d440`,
//! `00e2d550`) holds: +0x08 and +0x0c counts, +0x10 the number of faces in
//! use, +0x20 and +0x50 vertex records of 0x40 bytes (the vector at +0, a link
//! at +0x30), +0xde0 a stack of up to 100 recycled face pointers (count at
//! +0x0c) and +0xf70 the faces, 0x50 bytes each: the normal (+0x00), three
//! edge slots of 0x10 bytes (+0x10, +0x20, +0x30), a state word (+0x44) and
//! the plane distance (+0x40). SSE code computes lane by lane in `f32`; x87
//! sums stay unrounded until the `FSTP float`, compares read the status word
//! the way the `TEST AH` pairs do (NaN included).
//!
//! Register conventions: `00e2da40` takes the shape in `ECX`, the matrix in
//! `ESI` and the direction in `EDI` (stack: the second shape and the record,
//! the caller pops them); here those registers are the first three
//! arguments, in that order.

use crate::units::fallout_shared::tesobjectrefr::rsqrt_estimate;

#[allow(unused_imports)]
use crate::prelude::*;

type Vec4 = [f32; 4];

/// Float constants: 0.0, 3.0 and 0.5 (the `RSQRTSS` refinement), the largest
/// value (`0x7f7fffee`) and the most negative one (`0xff7fffee`).
const ZERO: u32 = 0x0101_1d78;
const THREE: u32 = 0x0101_7718;
const HALF: u32 = 0x0101_6248;
const MAX_FLOAT: u32 = 0x0104_da48;
const MIN_FLOAT: u32 = 0x0104_e148;
/// Four sign-bit masks (`0x80000000`).
const SIGN_MASK: u32 = 0x010c_44d0;
/// `double` constants: -1e-10 (as a float widened), 0.9 and 0.99, and 0.5.
const NEG_TINY: u32 = 0x010e_b190;
const NINE_TENTHS: u32 = 0x0106_b9e8;
const SHRINK: u32 = 0x0106_d508;
const HALF_DOUBLE: u32 = 0x0101_1588;
/// The `float` 0.001 the shrinking step of `00e2db70` stops at.
const SMALL_STEP: u32 = 0x0101_7d00;
/// The table of three direction vectors `00e2db70` starts from.
const START_DIRECTIONS: u32 = 0x0126_8370;
/// The table `00e2d440` builds the first four faces from (twelve entries of
/// three words).
const FACE_TABLE: u32 = 0x010e_d0f0;

/// Callees outside this unit, and the functions of this unit that call each
/// other.
const RESET_STATES: u32 = 0x00e6_7fd0;
const FACE_SIDE_TEST: u32 = 0x00e6_3af0;
const NORMALIZE_FACE: u32 = 0x00e2_cfe0;
const SUPPORT_POINT: u32 = 0x00e2_da40;
const WALK_FACES: u32 = 0x00e2_d0d0;

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

fn zip_with(a: Vec4, b: Vec4, f: impl Fn(f32, f32) -> f32) -> Vec4 {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// `z + (y + x)`: the `SHUFPS`/`ADDSS` sum of the first three lanes.
fn sum3(v: Vec4) -> f32 {
    v[2] + (v[1] + v[0])
}

/// `SHUFPS v, v, 0xc9`: the lanes y, z, x, w.
fn yzx(v: Vec4) -> Vec4 {
    [v[1], v[2], v[0], v[3]]
}

/// `SHUFPS v, v, 0xd2`: the lanes z, x, y, w.
fn zxy(v: Vec4) -> Vec4 {
    [v[2], v[0], v[1], v[3]]
}

/// `first.yzx * second.zxy - first.zxy * second.yzx`, the cross product of
/// the SSE sequence (`MULPS`, `MULPS`, `SUBPS`), lane by lane.
fn cross(first: Vec4, second: Vec4) -> Vec4 {
    let plus = zip_with(yzx(first), zxy(second), |a, b| a * b);
    let minus = zip_with(zxy(first), yzx(second), |a, b| a * b);
    zip_with(plus, minus, |a, b| a - b)
}

/// The `RSQRTSS` estimate refined once, with the `CMPEQSS`/`ANDNPS` zero
/// guard: `(half * r) * (three - (sum * r) * r)`, `+0.0` when `sum == 0`.
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

/// `vector` with every lane's sign bit flipped (`XORPS` with the sign mask).
fn negated(e: &Engine, vector: Vec4) -> Vec4 {
    std::array::from_fn(|lane| {
        let mask = e.mem.u32(SIGN_MASK + 4 * lane as u32);
        f32::from_bits(vector[lane].to_bits() ^ mask)
    })
}

/// `FLD float; FCHS; FSTP float`: the negation of a value that went through
/// the x87 stack (a signalling NaN comes back quiet).
fn x87_negated(value: f32) -> f32 {
    f32::from_bits(x87_quiet_bits(value.to_bits()) ^ 0x8000_0000)
}

// Translated from 00e2cfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up the plane of a face (`this` in `ECX`, a face of the polytope):
/// the normal is `(v0 - v1) x (v1 - v2)` of the vectors the three slot
/// pointers at +0x10, +0x20 and +0x30 point to (the SSE cross product, lane
/// by lane), normalised with the refined `RSQRTSS` (zero stays zero) and
/// stored at +0x00; the plane distance, the sum `z + (y + x)` of the unit
/// normal times `v0`, goes to +0x40.
pub fn fn_00e2cfe0(e: &mut Engine, face: Ptr) {
    let v1 = load_vec(e, e.mem.u32(face.addr() + 0x20));
    let v0_address = e.mem.u32(face.addr() + 0x10);
    let v0 = load_vec(e, v0_address);
    let v2 = load_vec(e, e.mem.u32(face.addr() + 0x30));
    let first = zip_with(v0, v1, |a, b| a - b);
    let second = zip_with(v1, v2, |a, b| a - b);
    let normal = cross(first, second);
    let sum = sum3(zip_with(normal, normal, |a, b| a * b));
    let scale = inverse_length(e, sum);
    let unit: Vec4 = normal.map(|lane| scale * lane);
    store_vec(e, face.addr(), unit);
    let v0_again = load_vec(e, v0_address);
    let distance = sum3(zip_with(unit, v0_again, |a, b| a * b));
    e.mem.set_f32(face.addr() + 0x40, x87_quiet(distance));
}

// Translated from 00e2d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the faces around `face` (`this` in `ECX` is the polytope): for each
/// of its three edge slots, whose pointer at +0x18, +0x28, +0x38 leads to the
/// record of the face across it (`record + 0xc`), a neighbour with state 0
/// is classified by `00e63af0` (`point_test` and the polytope's first two
/// words are its arguments) and the state saved: a result of 1 recurses into
/// that neighbour (a result of 2 from the recursion ends this walk), a result
/// of 4 ends the walk with 2; a neighbour in state 2 (just classified or from
/// before) is linked: the record's first word points to a structure whose
/// +0x30 gets the record, `*count` goes up by one and `*last` is set to the
/// record. Afterwards the face's plane distance (+0x40) is set to the largest
/// float and, while the stack of processed faces (+0xde0, count at +0x0c)
/// has fewer than 100 entries, the face is pushed on it. Returns the walk's
/// result, 2 when the stack is full.
pub fn fn_00e2d0d0(
    e: &mut Engine,
    this: Ptr,
    face: Ptr,
    point_test: u32,
    count: Ptr,
    last: Ptr,
) -> i32 {
    let mut result = 0i32;
    let mut slot_pointer = face.addr() + 0x18;
    let mut index = 0;
    loop {
        let record = e.mem.u32(slot_pointer);
        let neighbour = e.mem.u32(record + 0xc);
        let mut state = e.mem.u32(neighbour + 0x44);
        if state == 0 {
            state = e
                .call(
                    FACE_SIDE_TEST,
                    &args![neighbour, point_test, this, this.addr() + 4],
                )
                .u32();
            e.mem.set_u32(neighbour + 0x44, state);
            if state == 1 {
                result = e
                    .call(WALK_FACES, &args![this, neighbour, point_test, count, last])
                    .u32() as i32;
                if result == 2 {
                    break;
                }
            } else if state == 4 {
                result = 2;
                break;
            }
        }
        if state == 2 {
            let target = e.mem.u32(record);
            e.mem.set_u32(target + 0x30, record);
            let counted = e.mem.u32(count.addr());
            e.mem.set_u32(count.addr(), counted.wrapping_add(1));
            e.mem.set_u32(last.addr(), record);
        }
        slot_pointer = slot_pointer.wrapping_add(0x10);
        index += 1;
        if index >= 3 {
            break;
        }
    }
    // The plane distance is reset to the largest float (`FLD`/`FSTP`).
    let largest = x87_quiet(e.global::<f32>(MAX_FLOAT));
    e.mem.set_f32(face.addr() + 0x40, largest);
    let depth = e.mem.i32(this.addr() + 0xc);
    if depth >= 100 {
        return 2;
    }
    e.mem
        .set_u32(this.addr() + 0xde0 + 4 * depth as u32, face.addr());
    e.mem.set_i32(this.addr() + 0xc, depth + 1);
    result
}

// Translated from 00e2d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the faces that close the hole left when the faces of a visible
/// region were removed (`this` in `ECX` is the polytope): `count` faces are
/// taken, first from the stack of recycled faces (+0xde0, count at +0x0c),
/// then new ones from the end of the face array (+0xf70 + 0x50 * the number
/// in use at +0x10, which grows). The edges of the hole form a linked list
/// starting at `edge_list`: an edge record holds its first vertex pointer
/// (+0x00) and a slot (+0x04) leading to the pointer of its second vertex,
/// whose record's +0x30 gives the next edge (and is cleared). Face `i`,
/// built on the edge `i`, joins that edge's two vertices and `point`; the
/// edge slot, the face's three slots (back pointers, links to the neighbour
/// face `i + 1`, wrapping to the first) and its plane (the unit normal of
/// `(vb - va) x (va - point)` from the SSE cross product with the refined
/// `RSQRTSS`, the distance `z + (y + x)` of the normal times `vb`) are
/// filled in. The call returns 2 early when the list ends too soon or a face
/// has a zero normal. Afterwards the states of all faces (+0x44) and the
/// first +0x08 vertex records (+0x50, 0x40 apart) are cleared; returns 2
/// when the last edge reached is not `edge_list` (the hole did not close),
/// else 0.
pub fn fn_00e2d1b0(e: &mut Engine, this: Ptr, count: i32, edge_list: Ptr, point: Ptr) -> u32 {
    let t = this.addr();
    // A scratch array of face pointers on the stack (up to 100).
    e.with_stack(0x200, |e, scratch| {
        let local = scratch.addr();
        let mut top = e.mem.i32(t + 0xc).wrapping_sub(1);
        let mut filled = 0u32;
        let mut remaining = count;
        if count != 0 {
            let mut slot = t
                .wrapping_add(0xde0)
                .wrapping_add((top as u32).wrapping_mul(4));
            loop {
                if top < 0 {
                    break;
                }
                let face = e.mem.u32(slot);
                let depth = e.mem.i32(t + 0xc);
                e.mem.set_i32(t + 0xc, depth.wrapping_sub(1));
                e.mem.set_u32(local + 4 * filled, face);
                remaining = remaining.wrapping_sub(1);
                filled += 1;
                top = top.wrapping_sub(1);
                slot = slot.wrapping_sub(4);
                if remaining == 0 {
                    break;
                }
            }
        }
        if remaining > 0 {
            let in_use = e.mem.i32(t + 0x10);
            let mut offset = (in_use as u32).wrapping_mul(0x50);
            for k in 0..remaining as u32 {
                e.mem.set_u32(
                    local + 4 * (filled + k),
                    offset.wrapping_add(t).wrapping_add(0xf70),
                );
                offset = offset.wrapping_add(0x50);
            }
            e.mem.set_i32(t + 0x10, in_use.wrapping_add(remaining));
        }

        let mut edge = edge_list.addr();
        let mut index = 0i32;
        if count > 0 {
            loop {
                if edge == 0 {
                    return 2;
                }
                let face = e.mem.u32(local + 4 * index as u32);
                let next_index = index + 1;
                let next_face = if next_index < count {
                    e.mem.u32(local + 4 * next_index as u32)
                } else {
                    e.mem.u32(local)
                };
                e.mem.set_u32(face + 0x1c, face);
                e.mem.set_u32(face + 0x2c, face);
                e.mem.set_u32(face + 0x3c, face);
                let second_slot = face + 0x20;
                e.mem.set_u32(face + 0x14, second_slot);
                let third_slot = face + 0x30;
                e.mem.set_u32(face + 0x24, third_slot);
                let first_slot = face + 0x10;
                e.mem.set_u32(face + 0x34, first_slot);
                let second_pointer = e.mem.u32(e.mem.u32(edge + 4));
                let first_pointer = e.mem.u32(edge);
                e.mem.set_u32(first_slot, second_pointer);
                e.mem.set_u32(third_slot, point.addr());
                e.mem.set_u32(face + 0x20, first_pointer);
                e.mem.set_u32(edge + 8, first_slot);
                e.mem.set_u32(face + 0x18, edge);
                e.mem.set_u32(next_face + 0x28, third_slot);
                e.mem.set_u32(face + 0x38, next_face + 0x20);

                let second = load_vec(e, second_pointer);
                let first = load_vec(e, first_pointer);
                let apex = load_vec(e, point.addr());
                let along = zip_with(second, first, |a, b| a - b);
                let back = zip_with(first, apex, |a, b| a - b);
                let normal = cross(along, back);
                let sum = sum3(zip_with(normal, normal, |a, b| a * b));
                if sum == 0.0 {
                    return 2;
                }
                let scale = inverse_length(e, sum);
                let unit: Vec4 = normal.map(|lane| scale * lane);
                let distance = sum3(zip_with(second, unit, |a, b| a * b));
                e.mem.set_f32(face + 0x40, x87_quiet(distance));
                store_vec(e, face, unit);
                let next_slot = e.mem.u32(edge + 4);
                let vertex = e.mem.u32(next_slot);
                edge = e.mem.u32(vertex + 0x30);
                e.mem.set_u32(vertex + 0x30, 0);
                index = next_index;
                if index >= count {
                    break;
                }
            }
        }
        for face in 0..e.mem.i32(t + 0x10).max(0) as u32 {
            e.mem.set_u32(t + 0xfb4 + 0x50 * face, 0);
        }
        for vertex in 0..e.mem.i32(t + 8).max(0) as u32 {
            e.mem.set_u32(t + 0x50 + 0x40 * vertex, 0);
        }
        if edge_list.addr() != edge {
            2
        } else {
            0
        }
    })
}

// Translated from 00e2d440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets up the first four faces of the polytope (`this` in `ECX`): the face
/// count (+0x10) becomes 4 and each of the four faces (+0xf70, 0x50 bytes
/// apart) gets its slot links and, from the twelve-entry table at
/// `010ed0f0`, the vertex pointers of its three slots (`entry[0] + this +
/// 0x20`, and `entry[2] + entry[1] + this + 0xf80` for the pointer in +8);
/// state 0. The last face (+0x1060) is given its plane (`00e2cfe0`); if the
/// first two vertices (+0x20 and +0x60) satisfy `(v20 - v60) . normal > 0`
/// for that normal, the vertex pointers of three slots are redirected to the
/// vertices at +0x60 and +0x20 (+0x1020, +0xfd0, +0xf80 get +0x60; +0x1070,
/// +0xfe0, +0xfa0 get +0x20). Then all four faces get their planes.
pub fn fn_00e2d440(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    e.mem.set_u32(t + 0x10, 4);
    let mut table = FACE_TABLE;
    let end = t + 0x10b0;
    let mut face = t + 0xf70;
    while face != end {
        e.mem.set_u32(face + 0x14, face + 0x20);
        e.mem.set_u32(face + 0x24, face + 0x30);
        let mut slot = face + 0x10;
        let limit = face + 0x40;
        e.mem.set_u32(face + 0x44, 0);
        e.mem.set_u32(face + 0x34, slot);
        while slot < limit {
            e.mem.set_u32(slot + 0xc, face);
            let first = e.mem.u32(table);
            e.mem
                .set_u32(slot, first.wrapping_add(t).wrapping_add(0x20));
            let third = e.mem.u32(table + 8).wrapping_add(e.mem.u32(table + 4));
            slot += 0x10;
            e.mem
                .set_u32(slot - 8, third.wrapping_add(t).wrapping_add(0xf80));
            table = table.wrapping_add(0xc);
        }
        face += 0x50;
    }
    let last_face = t + 0x1060;
    e.call(NORMALIZE_FACE, &args![last_face]);
    let v60 = t + 0x60;
    let v20 = t + 0x20;
    let difference = zip_with(load_vec(e, v20), load_vec(e, v60), |a, b| a - b);
    let normal = load_vec(e, last_face);
    let side = sum3(zip_with(difference, normal, |a, b| a * b));
    if side > 0.0 {
        e.mem.set_u32(t + 0x1020, v60);
        e.mem.set_u32(t + 0xfd0, v60);
        e.mem.set_u32(t + 0xf80, v60);
        e.mem.set_u32(last_face + 0x10, v20);
        e.mem.set_u32(t + 0xfe0, v20);
        e.mem.set_u32(t + 0xfa0, v20);
    }
    for k in 0..4 {
        e.call(NORMALIZE_FACE, &args![t + 0xf70 + 0x50 * k]);
    }
}

// Translated from 00e2d550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hill-climbs over the faces of the polytope (`this` in `ECX`) from `face`:
/// resets the per-face states (`00e67fd0`), then repeatedly marks the
/// current face as state 3 and looks through its three slots, skipping the
/// one it was entered by. For slot `k` with vertices `p` (its pointer at
/// +0x00) and `q` (the pointer in the slot at +0x04), `d = q - p`, `f = (d x
/// normal) . p`, the slot counts as a way out when `-1e-10 * d.d > f * |f|`
/// (the constant is a `double`, the product stays unrounded) and the starting
/// face's plane distance (+0x40) is not above 0 or 0.9 times it is at most the
/// neighbour's. The walk moves to the neighbour across the slot (`slot + 8`
/// leads to its record, `+0xc` to the face), ending after that move when
/// the neighbour already has state 3. Returns the face it stopped at.
pub fn fn_00e2d550(e: &mut Engine, this: Ptr, face: Ptr) -> Ptr {
    e.call(RESET_STATES, &args![this]);
    let tiny: f64 = e.global(NEG_TINY);
    let nine_tenths: f64 = e.global(NINE_TENTHS);
    let mut current = face.addr();
    let mut excluded = 0u32;
    let mut done = false;
    loop {
        if done {
            return Ptr::new(current);
        }
        e.mem.set_u32(current + 0x44, 3);
        let mut slot_index = 0u32;
        let mut slot = current + 0x10;
        while slot_index < 3 {
            if slot != excluded {
                let p = load_vec(e, e.mem.u32(slot));
                let q_address = e.mem.u32(e.mem.u32(slot + 4));
                let q = load_vec(e, q_address);
                let d = zip_with(q, p, |a, b| a - b);
                let d_squared = sum3(zip_with(d, d, |a, b| a * b));
                let normal = load_vec(e, current);
                let crossed = cross(d, normal);
                let f = sum3(zip_with(crossed, p, |a, b| a * b));
                let bound = f as f64 * f.abs() as f64;
                let threshold = (d_squared as f64 * tiny) as f32;
                if threshold as f64 > bound {
                    let neighbour = e.mem.u32(e.mem.u32(slot + 8) + 0xc);
                    let own = e.mem.f32(face.addr() + 0x40);
                    let theirs = e.mem.f32(neighbour + 0x40);
                    let positive = 0.0 < own;
                    let nearer = own as f64 * nine_tenths > theirs as f64;
                    if !positive || !nearer {
                        excluded = e.mem.u32(current + 0x18 + 0x10 * slot_index);
                        done = e.mem.u32(neighbour + 0x44) == 3;
                        current = neighbour;
                        break;
                    }
                }
            }
            slot_index += 1;
            slot += 0x10;
        }
        if slot_index == 3 {
            return Ptr::new(current);
        }
    }
}

// Translated from 00e2da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Support point of the pair of shapes along a direction (custom
/// convention): `shape_a` is `ECX`, `matrix` `ESI` (four column vectors: the
/// rotation and the translation of `shape_b` relative to `shape_a`) and
/// `direction` `EDI`; the stack arguments are `shape_b` and `record`.
///
/// Asks `shape_a` (virtual call, slot +0x38) for its vertex along `-direction`
/// and `shape_b` for its vertex along the direction turned by the transpose
/// of the rotation, moves that vertex by the transform (`(y * c1 + x * c0) +
/// (z * c2 + c3)` per lane) and takes `s = (va - vb') . direction` (sum `z +
/// (y + x)`). If the record's float at +0x2c is below `s` (ordered), the
/// record is updated: +0x00 the vertex of `shape_a`, +0x10 the vertex of
/// `shape_b` (before the transform), +0x20 the direction, +0x2c `s`.
pub fn fn_00e2da40(
    e: &mut Engine,
    shape_a: Ptr,
    matrix: Ptr,
    direction: Ptr,
    shape_b: Ptr,
    record: Ptr,
) {
    e.with_stack(0x40, |e, frame| {
        let flipped = frame.addr();
        let rotated = frame.addr() + 0x10;
        let vertex_b = frame.addr() + 0x20;
        let vertex_a = frame.addr() + 0x30;
        let direction_now = load_vec(e, direction.addr());
        let backwards = negated(e, direction_now);
        store_vec(e, flipped, backwards);
        e.vcall(shape_a.addr(), 0x38, &args![flipped, vertex_a]);

        let c0 = load_vec(e, matrix.addr());
        let c1 = load_vec(e, matrix.addr() + 0x10);
        let c2 = load_vec(e, matrix.addr() + 0x20);
        let d = load_vec(e, direction.addr());
        // The transposed rows: the x, y, z of each column, with lane 3 taken
        // from c2's y and w.
        let row_x = [c0[0], c1[0], c2[0], c2[1]];
        let row_y = [c0[1], c1[1], c2[1], c2[3]];
        let row_z = [c0[2], c1[2], c2[2], c2[3]];
        let turned: Vec4 = std::array::from_fn(|lane| {
            (d[1] * row_y[lane] + d[0] * row_x[lane]) + d[2] * row_z[lane]
        });
        store_vec(e, rotated, turned);
        e.vcall(shape_b.addr(), 0x38, &args![rotated, vertex_b]);

        let b = load_vec(e, vertex_b);
        let c0 = load_vec(e, matrix.addr());
        let c1 = load_vec(e, matrix.addr() + 0x10);
        let c2 = load_vec(e, matrix.addr() + 0x20);
        let c3 = load_vec(e, matrix.addr() + 0x30);
        let direction_after = load_vec(e, direction.addr());
        let moved: Vec4 = std::array::from_fn(|lane| {
            (b[1] * c1[lane] + b[0] * c0[lane]) + (b[2] * c2[lane] + c3[lane])
        });
        let a = load_vec(e, vertex_a);
        let gap = zip_with(a, moved, |x, y| x - y);
        let score = sum3(zip_with(gap, direction_after, |x, y| x * y));
        // `FCOMP`: updates only when the record's float is below the score.
        if e.mem.f32(record.addr() + 0x2c) < score {
            store_vec(e, record.addr() + 0x20, direction_after);
            store_vec(e, record.addr(), a);
            store_vec(e, record.addr() + 0x10, b);
            e.mem.set_f32(record.addr() + 0x2c, x87_quiet(score));
        }
    });
}

// Translated from 00e2db70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refines the pair of closest support points of two shapes (cdecl, six
/// stack arguments: the shapes `shape_a` and `shape_b`, the `matrix` of
/// `00e2da40`, and the outputs `out_a`, `out_b` and `out_c`).
///
/// A record of `00e2da40` starts with the float +0x2c at the lowest value
/// (`0104e148`). The three directions of the table at `01268370` and their
/// negations seed it (two calls each). Then, with a step `s` that starts at
/// 1.0, every round shrinks it (`s = s * 0.99`, the `double` at `0106d508`),
/// takes the record's direction `d`, builds a vector `u` perpendicular to
/// it (two lanes picked from the smallest absolute components, one negated),
/// the unit `un = u / |u|` (refined `RSQRTSS`), the cross product `d x un`,
/// and the four directions `d + s un`, `d - s un`, `s (d x un) + d` and
/// `d - s (d x un)`, each scaled by the same normalising factor (that of
/// the first); each goes through `00e2da40`. The step is then halved and the
/// loop goes on while it exceeds 0.001. Finally `out_a` gets the record's
/// +0x00 vector, `out_b` its +0x10 vector, and `out_c` the direction (+0x00,
/// lane 3 being the score), the +0x00 vector at +0x10 and the score at +0x20.
pub fn fn_00e2db70(
    e: &mut Engine,
    shape_a: Ptr,
    shape_b: Ptr,
    matrix: Ptr,
    out_a: Ptr,
    out_b: Ptr,
    out_c: Ptr,
) {
    e.with_stack(0x100, |e, frame| {
        let base = frame.addr();
        let record = Ptr::<()>::new(base + 0x40);
        let start = x87_quiet(e.global::<f32>(MIN_FLOAT));
        e.mem.set_f32(base + 0x6c, start);
        for k in 0..3u32 {
            copy_vec(e, START_DIRECTIONS + 0x10 * k, base + 0x20);
            e.call(
                SUPPORT_POINT,
                &args![shape_a, matrix, base + 0x20, shape_b, record],
            );
            let flipped = negated(e, load_vec(e, base + 0x20));
            store_vec(e, base + 0x20, flipped);
            e.call(
                SUPPORT_POINT,
                &args![shape_a, matrix, base + 0x20, shape_b, record],
            );
        }

        let shrink: f64 = e.global(SHRINK);
        let half: f64 = e.global(HALF_DOUBLE);
        let mut step = 1.0f32;
        loop {
            step = (step as f64 * shrink) as f32;
            let d = load_vec(e, base + 0x60);
            let abs_x = d[0].abs();
            let abs_y = d[1].abs();
            let abs_z = d[2].abs();
            let mut first = 1usize;
            let mut second = 2usize;
            let mut third = 0usize;
            let mut smallest = abs_x;
            if abs_x > abs_y {
                smallest = abs_y;
                first = 0;
                third = 1;
            }
            if smallest > abs_z {
                second = third;
            }
            let mut u = [0.0f32; 4];
            u[first] = x87_quiet(d[second]);
            u[second] = x87_negated(d[first]);
            let u_sum = sum3(zip_with(u, u, |a, b| a * b));
            let u_scale = inverse_length(e, u_sum);
            let unit_u: Vec4 = u.map(|lane| u_scale * lane);
            let scaled_u: Vec4 = unit_u.map(|lane| step * lane);
            let crossed = cross(d, unit_u);
            let scaled_cross: Vec4 = crossed.map(|lane| lane * step);
            let plus = zip_with(d, scaled_u, |a, b| a + b);
            let plus_sum = sum3(zip_with(plus, plus, |a, b| a * b));
            let scale = inverse_length(e, plus_sum);
            let directions: [Vec4; 4] = [
                plus.map(|lane| scale * lane),
                zip_with(d, scaled_u, |a, b| a - b).map(|lane| lane * scale),
                zip_with(scaled_cross, d, |a, b| a + b).map(|lane| lane * scale),
                zip_with(d, scaled_cross, |a, b| a - b).map(|lane| lane * scale),
            ];
            let slots = [0xb0u32, 0x90, 0xa0, 0xc0];
            for (direction, slot) in directions.iter().zip(slots) {
                store_vec(e, base + slot, *direction);
            }
            for slot in slots {
                e.call(
                    SUPPORT_POINT,
                    &args![shape_a, matrix, base + slot, shape_b, record],
                );
            }
            step = (step as f64 * half) as f32;
            // `FCOM`: the loop goes on only while the step is above the limit.
            let limit: f32 = e.global(SMALL_STEP);
            let go_on = step > limit;
            if !go_on {
                break;
            }
        }
        copy_vec(e, base + 0x40, out_a.addr());
        copy_vec(e, base + 0x50, out_b.addr());
        let best = x87_quiet(e.mem.f32(base + 0x6c));
        e.mem.set_f32(out_c.addr() + 0x20, best);
        copy_vec(e, base + 0x60, out_c.addr());
        copy_vec(e, base + 0x40, out_c.addr() + 0x10);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e2cfe0, fn_00e2cfe0(Ptr)),
        entry!(0x00e2d0d0, fn_00e2d0d0(Ptr, Ptr, u32, Ptr, Ptr) -> i32),
        entry!(0x00e2d1b0, fn_00e2d1b0(Ptr, i32, Ptr, Ptr) -> u32),
        entry!(0x00e2d440, fn_00e2d440(Ptr)),
        entry!(0x00e2d550, fn_00e2d550(Ptr, Ptr) -> Ptr),
        entry!(0x00e2da40, fn_00e2da40(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00e2db70, fn_00e2db70(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(ZERO, 0.0f32);
        e.set_global(THREE, 3.0f32);
        e.set_global(HALF, 0.5f32);
        e.set_global(MAX_FLOAT, f32::from_bits(0x7f7f_ffee));
        e.set_global(MIN_FLOAT, f32::from_bits(0xff7f_ffee));
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e.set_global(NEG_TINY, -1e-10f32 as f64);
        e.set_global(NINE_TENTHS, 0.9f32 as f64);
        e.set_global(SHRINK, 0.99f32 as f64);
        e.set_global(HALF_DOUBLE, 0.5f64);
        e.set_global(SMALL_STEP, 0.001f32);
        e
    }

    fn vector_at(e: &mut Engine, v: Vec4) -> u32 {
        let address = e.mem.alloc(16);
        store_vec(e, address, v);
        address
    }

    fn assert_close(actual: &[f32], expected: &[f32]) {
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-4, "{actual:?} vs {expected:?}");
        }
    }

    /// A face whose three slots point at the given vertices.
    fn face_on(e: &mut Engine, vertices: [Vec4; 3]) -> u32 {
        let face = e.mem.alloc(0x50);
        for (k, v) in vertices.into_iter().enumerate() {
            let at = vector_at(e, v);
            e.mem.set_u32(face + 0x10 + 0x10 * k as u32, at);
        }
        face
    }

    #[test]
    fn face_plane_is_the_unit_normal_and_its_distance() {
        let mut e = engine();
        let face = face_on(
            &mut e,
            [
                [1.0, 0.0, 5.0, 0.0],
                [0.0, 0.0, 3.0, 0.0],
                [0.0, 1.0, 3.0, 0.0],
            ],
        );
        e.call(NORMALIZE_FACE, &args![face]);
        let root = 5.0f32.sqrt();
        assert_close(&load_vec(&e, face)[..3], &[2.0 / root, 0.0, -1.0 / root]);
        assert_close(&[e.mem.f32(face + 0x40)], &[-3.0 / root]);
    }

    #[test]
    fn face_plane_of_coincident_vertices_is_zero() {
        let mut e = engine();
        let same = [1.0, 2.0, 3.0, 0.0];
        let face = face_on(&mut e, [same, same, same]);
        e.call(NORMALIZE_FACE, &args![face]);
        assert_eq!(load_vec(&e, face), [0.0; 4]);
        assert_eq!(e.mem.f32(face + 0x40), 0.0);
    }

    /// A face with three records leading to three neighbour faces with the
    /// given states; returns (face, records, neighbours).
    fn ring(e: &mut Engine, states: [u32; 3]) -> (u32, [u32; 3], [u32; 3]) {
        let face = e.mem.alloc(0x50);
        let mut records = [0; 3];
        let mut neighbours = [0; 3];
        for k in 0..3 {
            let neighbour = e.mem.alloc(0x50);
            e.mem.set_u32(neighbour + 0x44, states[k]);
            let target = e.mem.alloc(0x40);
            let record = e.mem.alloc(0x10);
            e.mem.set_u32(record, target);
            e.mem.set_u32(record + 0xc, neighbour);
            e.mem.set_u32(face + 0x18 + 0x10 * k as u32, record);
            records[k] = record;
            neighbours[k] = neighbour;
        }
        (face, records, neighbours)
    }

    #[test]
    fn walk_links_neighbours_that_are_in_state_two() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, records, _) = ring(&mut e, [2, 5, 2]);
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(counter), 2);
        assert_eq!(e.mem.u32(counter + 4), records[2]);
        for k in [0usize, 2] {
            let target = e.mem.u32(records[k]);
            assert_eq!(e.mem.u32(target + 0x30), records[k]);
        }
        let untouched = e.mem.u32(records[1]);
        assert_eq!(e.mem.u32(untouched + 0x30), 0);
        // The face is reset and pushed on the stack of processed faces.
        assert_eq!(e.mem.f32(face + 0x40), f32::from_bits(0x7f7f_ffee));
        assert_eq!(e.mem.u32(this + 0xc), 1);
        assert_eq!(e.mem.u32(this + 0xde0), face);
    }

    #[test]
    fn walk_classifies_fresh_neighbours_and_stops_on_a_four() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, records, neighbours) = ring(&mut e, [0, 0, 0]);
        let results = [3u32, 2, 4];
        let seen: std::rc::Rc<std::cell::RefCell<Vec<Vec<u32>>>> = Default::default();
        let log = seen.clone();
        e.register_double(FACE_SIDE_TEST, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            let index = log.borrow().len() - 1;
            Ret {
                eax: results[index],
                ..Ret::default()
            }
        });
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 2);
        assert_eq!(seen.borrow()[0], vec![neighbours[0], 7, this, this + 4]);
        assert_eq!(seen.borrow().len(), 3);
        // The result is saved in each neighbour's state.
        assert_eq!(e.mem.u32(neighbours[0] + 0x44), 3);
        assert_eq!(e.mem.u32(neighbours[1] + 0x44), 2);
        assert_eq!(e.mem.u32(neighbours[2] + 0x44), 4);
        // The state-2 neighbour was linked; the one that answered 4 was not.
        assert_eq!(e.mem.u32(counter), 1);
        assert_eq!(e.mem.u32(counter + 4), records[1]);
    }

    #[test]
    fn walk_recurses_into_a_neighbour_that_answers_one() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        let (face, _, neighbours) = ring(&mut e, [0, 5, 5]);
        // The neighbour's own neighbours are all finished (state 5).
        let (inner, _, _) = ring(&mut e, [5, 5, 5]);
        // Make the first record's neighbour the inner ring's face.
        let record = e.mem.u32(face + 0x18);
        e.mem.set_u32(record + 0xc, inner);
        e.register_double(FACE_SIDE_TEST, move |_, a| {
            assert_eq!(a[0], inner);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(inner + 0x44), 1);
        // The inner face was walked first and pushed first.
        assert_eq!(e.mem.u32(this + 0xc), 2);
        assert_eq!(e.mem.u32(this + 0xde0), inner);
        assert_eq!(e.mem.u32(this + 0xde4), face);
        let _ = neighbours;
    }

    #[test]
    fn walk_returns_two_when_the_face_stack_is_full() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        e.mem.set_u32(this + 0xc, 100);
        let (face, _, _) = ring(&mut e, [5, 5, 5]);
        let counter = e.mem.alloc(8);
        let result = e
            .call(WALK_FACES, &args![this, face, 7u32, counter, counter + 4])
            .u32() as i32;
        assert_eq!(result, 2);
        assert_eq!(e.mem.u32(this + 0xc), 100);
        assert_eq!(e.mem.f32(face + 0x40), f32::from_bits(0x7f7f_ffee));
    }

    /// A polytope with three vertex records and a closed list of two edges
    /// A->B and B->C around the apex `point`; returns (this, edge0, point).
    fn hole(e: &mut Engine, apex: Vec4) -> (u32, u32, u32, [u32; 3]) {
        let this = e.mem.alloc(0x1200);
        e.mem.set_u32(this + 8, 2);
        let a = e.mem.alloc(0x40);
        let b = e.mem.alloc(0x40);
        let c = e.mem.alloc(0x40);
        store_vec(e, a, [0.0, 0.0, 0.0, 0.0]);
        store_vec(e, b, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, c, [0.0, 1.0, 0.0, 0.0]);
        let mut edges = [0; 2];
        for (k, (from, to)) in [(a, b), (b, c)].into_iter().enumerate() {
            let edge = e.mem.alloc(0x10);
            let cell = e.mem.alloc(4);
            e.mem.set_u32(edge, from);
            e.mem.set_u32(edge + 4, cell);
            e.mem.set_u32(cell, to);
            edges[k] = edge;
        }
        e.mem.set_u32(b + 0x30, edges[1]);
        e.mem.set_u32(c + 0x30, edges[0]);
        let point = vector_at(e, apex);
        (this, edges[0], point, [a, b, c])
    }

    #[test]
    fn closing_a_hole_builds_a_ring_of_faces() {
        let mut e = engine();
        let (this, edge, point, [a, b, c]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 0);
        let face0 = this + 0xf70;
        let face1 = this + 0xfc0;
        assert_eq!(e.mem.u32(this + 0x10), 2);
        // Face 0 sits on the edge A->B, face 1 on B->C.
        assert_eq!(e.mem.u32(face0 + 0x10), b);
        assert_eq!(e.mem.u32(face0 + 0x20), a);
        assert_eq!(e.mem.u32(face0 + 0x30), point);
        assert_eq!(e.mem.u32(face0 + 0x14), face0 + 0x20);
        assert_eq!(e.mem.u32(face0 + 0x24), face0 + 0x30);
        assert_eq!(e.mem.u32(face0 + 0x34), face0 + 0x10);
        assert_eq!(e.mem.u32(face0 + 0x1c), face0);
        assert_eq!(e.mem.u32(face0 + 0x18), edge);
        assert_eq!(e.mem.u32(edge + 8), face0 + 0x10);
        assert_eq!(e.mem.u32(face1 + 0x10), c);
        assert_eq!(e.mem.u32(face1 + 0x20), b);
        // Neighbour links, the last face wrapping to the first.
        assert_eq!(e.mem.u32(face0 + 0x38), face1 + 0x20);
        assert_eq!(e.mem.u32(face1 + 0x28), face0 + 0x30);
        assert_eq!(e.mem.u32(face1 + 0x38), face0 + 0x20);
        assert_eq!(e.mem.u32(face0 + 0x28), face1 + 0x30);
        // The planes and the cleared links.
        assert_close(&load_vec(&e, face0)[..3], &[0.0, 1.0, 0.0]);
        assert_close(&[e.mem.f32(face0 + 0x40)], &[0.0]);
        let third = 1.0 / 3.0f32.sqrt();
        assert_close(&load_vec(&e, face1)[..3], &[-third, -third, -third]);
        assert_close(&[e.mem.f32(face1 + 0x40)], &[-third]);
        assert_eq!(e.mem.u32(b + 0x30), 0);
        assert_eq!(e.mem.u32(c + 0x30), 0);
    }

    #[test]
    fn closing_a_hole_takes_recycled_faces_first() {
        let mut e = engine();
        let (this, edge, point, [_, b, _]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let recycled = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(this + 0xde0, recycled);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        // One face came off the stack, one is new.
        assert_eq!(e.mem.u32(this + 0x10), 1);
        assert_eq!(e.mem.u32(recycled + 0x10), b);
        assert_eq!(e.mem.u32(edge + 8), recycled + 0x10);
    }

    #[test]
    fn closing_a_hole_stops_at_a_degenerate_face() {
        let mut e = engine();
        // The apex coincides with the first vertex: a zero normal.
        let (this, edge, point, _) = hole(&mut e, [0.0, 0.0, 0.0, 0.0]);
        let result = e.call(0x00e2_d1b0, &args![this, 2i32, edge, point]).u32();
        assert_eq!(result, 2);
        // The edge list running out ends the call with 2 as well.
        let result = e.call(0x00e2_d1b0, &args![this, 1i32, 0u32, point]).u32();
        assert_eq!(result, 2);
    }

    #[test]
    fn closing_a_hole_reports_an_open_ring() {
        let mut e = engine();
        let (this, edge, point, [_, _, c]) = hole(&mut e, [0.0, 0.0, 1.0, 0.0]);
        // Only the first edge is used: the walk ends on edge 1, not edge 0.
        let result = e.call(0x00e2_d1b0, &args![this, 1i32, edge, point]).u32();
        assert_eq!(result, 2);
        let _ = c;
    }

    #[test]
    fn first_faces_are_built_from_the_table() {
        let mut e = engine();
        for j in 0..12u32 {
            e.mem.set_u32(FACE_TABLE + 12 * j, 0x40 * (j % 4));
            e.mem.set_u32(FACE_TABLE + 12 * j + 4, 0x10 * j);
            e.mem.set_u32(FACE_TABLE + 12 * j + 8, 0x20);
        }
        let this = e.mem.alloc(0x1200);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, 1.0, 0.0]);
        store_vec(&mut e, this + 0x60, [0.0; 4]);
        let faces: std::rc::Rc<std::cell::RefCell<Vec<u32>>> = Default::default();
        let log = faces.clone();
        let last = this + 0x1060;
        e.register_double(NORMALIZE_FACE, move |e, a| {
            log.borrow_mut().push(a[0]);
            if a[0] == last {
                store_vec(e, a[0], [0.0, 0.0, 1.0, 0.0]);
            }
            Ret::default()
        });
        e.call(0x00e2_d440, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), 4);
        // Face 1 (+0xfc0), slot 2: entry 5 of the table.
        let face = this + 0xfc0;
        assert_eq!(e.mem.u32(face + 0x20), this + 0x20);
        assert_eq!(e.mem.u32(face + 0x28), 0x20 + 0x10 * 4 + this + 0xf80);
        assert_eq!(e.mem.u32(face + 0x2c), face);
        assert_eq!(e.mem.u32(face + 0x14), face + 0x20);
        assert_eq!(e.mem.u32(face + 0x24), face + 0x30);
        assert_eq!(e.mem.u32(face + 0x34), face + 0x10);
        assert_eq!(e.mem.u32(face + 0x44), 0);
        // The last face's normal faces the first vertex: three slots are redirected.
        assert_eq!(e.mem.u32(this + 0x1020), this + 0x60);
        assert_eq!(e.mem.u32(this + 0xfd0), this + 0x60);
        assert_eq!(e.mem.u32(this + 0xf80), this + 0x60);
        assert_eq!(e.mem.u32(last + 0x10), this + 0x20);
        assert_eq!(e.mem.u32(this + 0xfe0), this + 0x20);
        assert_eq!(e.mem.u32(this + 0xfa0), this + 0x20);
        assert_eq!(
            *faces.borrow(),
            vec![
                last,
                this + 0xf70,
                this + 0xfc0,
                this + 0x1010,
                this + 0x1060
            ]
        );
    }

    #[test]
    fn first_faces_keep_their_vertices_when_the_normal_points_away() {
        let mut e = engine();
        let this = e.mem.alloc(0x1200);
        store_vec(&mut e, this + 0x20, [0.0, 0.0, -1.0, 0.0]);
        store_vec(&mut e, this + 0x60, [0.0; 4]);
        let last = this + 0x1060;
        e.register_double(NORMALIZE_FACE, move |e, a| {
            if a[0] == last {
                store_vec(e, a[0], [0.0, 0.0, 1.0, 0.0]);
            }
            Ret::default()
        });
        e.call(0x00e2_d440, &args![this]);
        assert_eq!(e.mem.u32(this + 0x1020), e.mem.u32(this + 0x1020));
        assert_ne!(e.mem.u32(this + 0x1020), this + 0x60);
        assert_ne!(e.mem.u32(this + 0xfd0), this + 0x60);
    }
}
