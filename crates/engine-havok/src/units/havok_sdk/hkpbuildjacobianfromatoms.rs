//! `Havok SDK/hkpbuildjacobianfromatoms.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The solver's Jacobian builders. Each function appends fixed-size records to the Jacobian
//! stream behind a cursor (a pointer to the write position, advanced in place). The Xbox PDB has
//! no record of the Havok classes, so the field names below are unconfirmed; the code addresses
//! the constraint "info" block (`this` of the atom builders: `+0x20` the weight (inverse time
//! step), `+0x24` the slope, `+0x30`/`+0x34` the two bodies, `+0x50` the breaking limit, `+0x60`
//! the constraint id, `+0x68` the pointer to the list of broken constraints) and the body's
//! solver data (`+0x20` angular velocity, `+0x30` the inverse inertia, `+0x40` the centre of
//! mass, `+0x50`/`+0x60`/`+0x70` the rotation rows). Functions that take values in registers
//! list them first, in register order (EAX, ECX, EDX, ESI, EDI), then the stack words.

use super::hkpboxmotion::{add, cross, mul, splat, sub};
use super::hkpsphereshape::{
    copy_vec, inverse_length, load_bits, load_vec, shufps, store_bits, store_vec, sum3, unpckhps,
    Vec4, SIGN_MASK,
};
#[allow(unused_imports)]
use crate::prelude::*;

/// The `double` 1.1920929e-07 (`FLT_EPSILON`) the builders add to the first word of a record and
/// to the effective mass.
const EPSILON: u32 = 0x010c_9300;
/// The `double` 3.4028234663852886e38 (`FLT_MAX`) the limit is compared with.
const MAX_FLOAT: u32 = 0x0104_e138;
/// Three unit vectors, the axes of the linear builders (16 bytes each).
const AXIS_TABLE: u32 = 0x0126_8370;
/// `InterlockedExchangeAdd` import slot (stdcall: the counter, the addend).
const INTERLOCKED_EXCHANGE_ADD: u32 = 0x00fd_f1ec;
/// Unnamed (thiscall, two stack words): `this` receives the vector made of the body's
/// angular term; the words are the body's rotation rows (`+0x50`) and angular velocity (`+0x20`).
const ROTATE_ANGULAR: u32 = 0x004b_4ab0;
/// `hkSimpleContactConstraintDataAddALittleSurfaceViscosity` (cdecl, one word).
const ADD_SURFACE_VISCOSITY: u32 = 0x00d7_2090;
/// `hkSimpleContactConstraintDataBuildJacobian` (cdecl: atom, info copy, flag, cursor).
const BUILD_CONTACT_JACOBIAN: u32 = 0x00d7_2190;
/// `hkSolverBuildJacobianFromAtomsNotContact` (cdecl: atom, id, info copy, cursor).
const BUILD_NOT_CONTACT_JACOBIAN: u32 = 0x00d6_f460;
/// `hk1dLinearBilateralConstraintUserTauBuildJacobian` (cdecl: input block, two words).
const LINEAR_BILATERAL_USER_TAU: u32 = 0x00dc_f880;
/// `hk1dLinearLimitBuildJacobian` (cdecl: input block, two words).
const LINEAR_LIMIT: u32 = 0x00dc_fa30;
/// `hk1dAngularFrictionBuildJacobian` (cdecl: input block, info, cursor).
const ANGULAR_FRICTION: u32 = 0x00dc_efb0;

/// The 16-byte write position of the first float of a record: adds `FLT_EPSILON` through the
/// x87 stack (the old word is what the stream held before, converted to a double).
fn add_epsilon(e: &mut Engine, record: u32) {
    let epsilon: f64 = e.global(EPSILON);
    let old = x87_quiet(e.mem.f32(record));
    e.mem.set_f32(record, (f64::from(old) + epsilon) as f32);
}

/// `(c.x * r0 + c.y * r1) + c.z * r2`: the vector `c` carried through three rows (the code adds
/// the `y` and `x` parts first; the sum is commutative, so the order is not observable).
fn rotate_rows(c: Vec4, rows: [Vec4; 3]) -> Vec4 {
    add(
        add(mul(splat(c, 0), rows[0]), mul(splat(c, 1), rows[1])),
        mul(splat(c, 2), rows[2]),
    )
}

/// The three rotation rows `+0x50`, `+0x60`, `+0x70` of a body's solver data.
fn body_rows(e: &Engine, body: u32) -> [Vec4; 3] {
    [
        load_vec(e, body + 0x50),
        load_vec(e, body + 0x60),
        load_vec(e, body + 0x70),
    ]
}

/// The effective-mass sum of the axis builders: lanes `x`, `y`, `z` of `acc` and lane `w` of
/// `mass`, added as `(y + w) + (x + z)` (`UNPCKHPS`, `SHUFPS`, two pair-wise adds).
fn pair_sum(acc: Vec4, mass: Vec4) -> f32 {
    let unpacked = unpckhps(acc, mass);
    let merged = shufps(acc, unpacked, 0xc4);
    let swapped = shufps(merged, merged, 0x4e);
    let pairs = add(swapped, merged);
    let shifted = shufps(pairs, pairs, 0xb1);
    add(shifted, pairs)[0]
}

/// Negates all four lanes (`XORPS` with the sign mask at `010c44d0`).
fn negate(e: &Engine, v: Vec4) -> Vec4 {
    let mask = load_bits(e, SIGN_MASK);
    std::array::from_fn(|lane| f32::from_bits(v[lane].to_bits() ^ mask[lane]))
}

/// One axis record of the linear builders (the common part of `fn_00d6c630` and
/// `fn_00d6c970`): the axis (16 bytes) at `+0`, the two angular terms at `+0x10` and `+0x20`,
/// the effective mass at `+0x2c`, `slope / mass` at `+0x1c` and `(b - a) . axis * weight` at
/// `+0xc`.
fn axis_record(e: &mut Engine, info: u32, a: u32, b: u32, record: u32, axis: u32) {
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let slope = x87_quiet(e.mem.f32(info + 0x24));
    let weight = x87_quiet(e.mem.f32(info + 0x20));
    let axis_vector = load_vec(e, axis);
    store_vec(e, record, axis_vector);
    let from_a = sub(load_vec(e, a), load_vec(e, body_a + 0x40));
    let from_b = sub(load_vec(e, b), load_vec(e, body_b + 0x40));
    let axis_vector = load_vec(e, axis);
    let torque_a = cross(from_a, axis_vector);
    let torque_b = cross(axis_vector, from_b);
    let rows_a = body_rows(e, body_a);
    let rows_b = body_rows(e, body_b);
    let angular_a = rotate_rows(torque_a, rows_a);
    store_vec(e, record + 0x10, angular_a);
    let angular_b = rotate_rows(torque_b, rows_b);
    store_vec(e, record + 0x20, angular_b);
    let inverse_a = load_vec(e, body_a + 0x30);
    let inverse_b = load_vec(e, body_b + 0x30);
    let terms = add(
        mul(mul(angular_b, angular_b), inverse_b),
        mul(mul(angular_a, angular_a), inverse_a),
    );
    let sum = pair_sum(terms, add(inverse_a, inverse_b));
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    e.mem.set_f32(record + 0x2c, total);
    e.mem
        .set_f32(record + 0x1c, (f64::from(slope) / f64::from(total)) as f32);
    let distance = sum3(mul(sub(load_vec(e, b), load_vec(e, a)), axis_vector));
    e.mem.set_f32(
        record + 0xc,
        (f64::from(distance) * f64::from(weight)) as f32,
    );
}

/// The angular pair of the angular builders: `vector` carried through body A's rows into
/// `record + 0`, the negated vector through body B's rows into `record + 0x10`. Returns the sum
/// of the squared terms times the inverse inertias, `z + (y + x)`.
fn angular_pair(e: &mut Engine, info: u32, vector: u32, record: u32) -> f32 {
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let v = load_vec(e, vector);
    let negated = negate(e, v);
    let rows_a = body_rows(e, body_a);
    let rows_b = body_rows(e, body_b);
    let angular_a = rotate_rows(v, rows_a);
    store_vec(e, record, angular_a);
    let angular_b = rotate_rows(negated, rows_b);
    store_vec(e, record + 0x10, angular_b);
    let terms = add(
        mul(mul(angular_a, angular_a), load_vec(e, body_a + 0x30)),
        mul(mul(angular_b, angular_b), load_vec(e, body_b + 0x30)),
    );
    sum3(terms)
}

// Translated from 00d6c5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromViscousSurfaceModifier` (engine map): `source` (EAX) is the 0x80-byte info
/// block; the stack words are the atom and the cursor. Backs the cursor up by 0x10, copies the
/// info block to the stack, walks the atom chain (`+0x14`, while the type word is at least 0x17)
/// and adds a little surface viscosity to the contact data behind the node (`node + 0x10`), then
/// builds the Jacobian of the next atom: a contact atom (type 0x16) through
/// `hkSimpleContactConstraintDataBuildJacobian` (with the flag 1; the word the exe pushes has
/// unset upper bytes the callee never reads), any other through
/// `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`.
pub fn build_jacobian_from_viscous_surface_modifier(
    e: &mut Engine,
    source: Ptr,
    atom: Ptr,
    cursor: Ptr,
) {
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    e.with_stack(0x80, |e, copy| {
        for word in 0..0x20 {
            let value = e.mem.u32(source.addr() + 4 * word);
            e.mem.set_u32(copy.addr() + 4 * word, value);
        }
        let id = u32::from(e.mem.u16(atom.addr() + 0x12));
        let mut node = atom.addr();
        loop {
            node = e.mem.u32(node + 0x14);
            if e.mem.u16(node) < 0x17 {
                break;
            }
        }
        e.call(ADD_SURFACE_VISCOSITY, &args![node + 0x10]);
        let next = e.mem.u32(atom.addr() + 0x14);
        if e.mem.u16(next) == 0x16 {
            e.call(BUILD_CONTACT_JACOBIAN, &args![next, copy, 1u32, cursor]);
        } else {
            e.call(BUILD_NOT_CONTACT_JACOBIAN, &args![next, id, copy, cursor]);
        }
    });
}

// Translated from 00d6c630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three linear-limit records (no engine-map name). `info` (EDI) is the info block; the stack
/// words are the two world points `a` and `b`, the cursor, the limit and a scale. Makes each
/// body's angular term through the unnamed rotate function, forms the position error
/// `(b - a) + (rb x ob) * h - (ra x oa) * h` with `h = (1 / weight) * scale`, and when the
/// list at `info + 0x68` exists and `info + 0x50` minus the squared error is negative appends
/// the constraint id at `info + 0x60` to it (an `InterlockedExchangeAdd` slot, at most 0x80
/// entries). Then each axis gets a record from `axis_record`: the `+0xc` word is replaced by
/// the error component times the weight, the first float gets `FLT_EPSILON` added and the tag
/// byte is 5 when `limit` equals `FLT_MAX` and 6 otherwise (then `+0x2c` holds the limit).
pub fn fn_00d6c630(e: &mut Engine, info: Ptr, a: Ptr, b: Ptr, cursor: Ptr, limit: f32, scale: f32) {
    let info = info.addr();
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let mut record = e.mem.u32(cursor.addr());
    let weight_reciprocal = 1.0f64 / f64::from(e.mem.f32(info + 0x20));
    let step = (weight_reciprocal * f64::from(scale)) as f32;
    let a_point = load_vec(e, a.addr());
    let b_point = load_vec(e, b.addr());
    let from_a = sub(a_point, load_vec(e, body_a + 0x40));
    let from_b = sub(b_point, load_vec(e, body_b + 0x40));
    let angular_a = rotated_angular(e, body_a);
    let angular_b = rotated_angular(e, body_b);
    let steps = [step; 4];
    let moved = add(mul(cross(from_b, angular_b), steps), sub(b_point, a_point));
    let error = sub(moved, mul(cross(from_a, angular_a), steps));
    let list = e.mem.u32(info + 0x68);
    if list != 0 {
        let squared = sum3(mul(error, error));
        let margin = e.mem.f32(info + 0x50) - squared;
        if margin.to_bits() >> 31 != 0 {
            let old = e.call(INTERLOCKED_EXCHANGE_ADD, &args![list, 1u32]).u32();
            if old < 0x80 {
                let id = e.mem.u32(info + 0x60);
                e.mem.set_u32(list + 4 + old * 4, id);
            }
        }
    }
    let max_float: f64 = e.global(MAX_FLOAT);
    for axis in 0..3u32 {
        axis_record(
            e,
            info,
            a.addr(),
            b.addr(),
            record,
            AXIS_TABLE + 0x10 * axis,
        );
        let scaled = (f64::from(error[axis as usize]) * f64::from(e.mem.f32(info + 0x20))) as f32;
        e.mem.set_f32(record + 0xc, scaled);
        add_epsilon(e, record);
        if f64::from(limit) == max_float {
            e.mem.set_u8(record, 5);
        } else {
            e.mem.set_u8(record, 6);
            e.mem.set_f32(record + 0x2c, x87_quiet(limit));
        }
        record += 0x30;
    }
    e.mem.set_u32(cursor.addr(), record);
}

/// The unnamed rotate function (`004b4ab0`) on a body's rotation rows and angular velocity.
fn rotated_angular(e: &mut Engine, body: u32) -> Vec4 {
    e.with_stack(0x10, |e, out| {
        e.call(ROTATE_ANGULAR, &args![out, body + 0x50, body + 0x20]);
        load_vec(e, out.addr())
    })
}

// Translated from 00d6c970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three linear records without an error term (no engine-map name; cdecl: the two world points,
/// the info block, the output record): for each axis of the table at `01268370` the same
/// `axis_record` as `fn_00d6c630` (axis, angular terms, effective mass, `slope / mass`,
/// `(b - a) . axis * weight`), records 0x30 bytes apart, with no tag byte.
pub fn fn_00d6c970(e: &mut Engine, a: Ptr, b: Ptr, info: Ptr, record: Ptr) {
    for axis in 0..3u32 {
        axis_record(
            e,
            info.addr(),
            a.addr(),
            b.addr(),
            record.addr() + 0x30 * axis,
            AXIS_TABLE + 0x10 * axis,
        );
    }
}

// Translated from 00d6cb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record (no engine-map name; cdecl: the input block `vector`, the info block and
/// the cursor). The input block holds a vector at `+0` and floats at `+0x10`, `+0x14`, `+0x18`
/// and `+0x1c` (the last is `k`). Writes `vector` carried through body A's rows at `+0` and
/// `-vector` through body B's at `+0x10`; `+0xc` is `k / (mass + FLT_EPSILON)`; with
/// `m = -info[2]`: `+0x1c` is `input[6] * m`, `+0x20` is `input[4] * m * k`, `+0x24` is
/// `m * input[5] * k` and `+0x28` is `k`; the first float gets `FLT_EPSILON` added and the tag
/// byte 0xd. The cursor moves on by 0x30.
pub fn fn_00d6cb30(e: &mut Engine, input: Ptr, info: Ptr, cursor: Ptr) {
    let (input, info) = (input.addr(), info.addr());
    let k = x87_quiet(e.mem.f32(input + 0x1c));
    let record = e.mem.u32(cursor.addr());
    let sum = angular_pair(e, info, input, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    e.mem
        .set_f32(record + 0xc, (f64::from(k) / f64::from(total)) as f32);
    let m = -x87_quiet(e.mem.f32(info + 8));
    let fourth = x87_quiet(e.mem.f32(input + 0x10));
    let fifth = x87_quiet(e.mem.f32(input + 0x14));
    let sixth = e.mem.f32(input + 0x18);
    e.mem
        .set_f32(record + 0x1c, (f64::from(sixth) * f64::from(m)) as f32);
    e.mem.set_u32(cursor.addr(), record + 0x30);
    let first = (f64::from(fourth) * f64::from(m)) * f64::from(k);
    e.mem.set_f32(record + 0x20, first as f32);
    let second = (f64::from(m) * f64::from(fifth)) * f64::from(k);
    e.mem.set_f32(record + 0x24, second as f32);
    e.mem.set_f32(record + 0x28, k);
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xd);
}

// Translated from 00d6cc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular-velocity-motor begin step with registers (no engine-map name): `info` (ECX), the
/// output `record` (EDX), `vector` (ESI) and the stack word `out`. Same body as
/// `hk1d_angular_velocity_motor_begin_jacobian`: the angular pair at `record + 0`/`+0x10`, then
/// `1 / (sum + FLT_EPSILON)` rounded to `float` at `record + 0xc` and at `*out`.
pub fn fn_00d6cc80(e: &mut Engine, info: Ptr, record: Ptr, vector: Ptr, out: Ptr) {
    angular_motor_begin(e, info.addr(), vector.addr(), record.addr(), out.addr());
}

fn angular_motor_begin(e: &mut Engine, info: u32, vector: u32, record: u32, out: u32) {
    let sum = angular_pair(e, info, vector, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    let inverse = (1.0f64 / f64::from(total)) as f32;
    e.mem.set_f32(record + 0xc, inverse);
    e.mem.set_f32(out, inverse);
}

// Translated from 00d6cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dAngularVelocityMotorBeginJacobian` (engine map; cdecl: the axis vector, the info block,
/// the output record and a float output): the angular pair at `record + 0`/`+0x10`, then
/// `1 / (sum + FLT_EPSILON)` rounded to `float` at `record + 0xc` and at `*out`.
pub fn hk1d_angular_velocity_motor_begin_jacobian(
    e: &mut Engine,
    vector: Ptr,
    info: Ptr,
    record: Ptr,
    out: Ptr,
) {
    angular_motor_begin(e, info.addr(), vector.addr(), record.addr(), out.addr());
}

// Translated from 00d6ce70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hk1dAngularVelocityMotorCommitJacobian` (engine map; cdecl: the motor block, the motor
/// state, the begin record and the cursor): `begin + 0x1c = state[2] * block[0]`; at the cursor
/// a new record with the tag 0xf and `FLT_EPSILON` added to its first float, `+0x20 = block[2] *
/// state[1]`, `+0x24 = block[3] * state[1]`, `+0x28`, `+0x2c` and `+0x30` the floats
/// `block[1]`, `block[4]` and `block[5]` (copied through the x87 stack). The cursor moves on
/// by 0x40. The first word of the new record keeps the stream's earlier bytes apart from the
/// tag.
pub fn hk1d_angular_velocity_motor_commit_jacobian(
    e: &mut Engine,
    block: Ptr,
    state: Ptr,
    begin: Ptr,
    cursor: Ptr,
) {
    let (block, state) = (block.addr(), state.addr());
    let product = f64::from(e.mem.f32(state + 8)) * f64::from(e.mem.f32(block));
    e.mem.set_f32(begin.addr() + 0x1c, product as f32);
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xf);
    e.mem.set_u32(cursor.addr(), record + 0x40);
    let scale = f64::from(e.mem.f32(state + 4));
    e.mem.set_f32(
        record + 0x20,
        (f64::from(e.mem.f32(block + 8)) * scale) as f32,
    );
    e.mem.set_f32(
        record + 0x24,
        (f64::from(e.mem.f32(block + 0xc)) * scale) as f32,
    );
    for (to, from) in [(0x28, 4), (0x2c, 0x10), (0x30, 0x14)] {
        let value = x87_quiet(e.mem.f32(block + from));
        e.mem.set_f32(record + to, value);
    }
}

/// Builds the input block of a one-dimensional linear builder on the stack and calls `callee`
/// with it and the two stack words (`fn_00d6d220`, `fn_00d6d280`). The block: `+0` the vector at
/// `other + 0x30`, `+0x10` the vector at `bodies + 0x30`, `+0x20` the vector at
/// `bodies + (atom.byte[2] << 4)`, `+0x30` and `+0x34` the floats at `atom + 4` and `atom + 8`
/// (copied through the x87 stack).
fn linear_input_block(
    e: &mut Engine,
    atom: u32,
    bodies: u32,
    other: u32,
    callee: u32,
    tail: [u32; 2],
) {
    e.with_stack(0x40, |e, block| {
        let block = block.addr();
        let first = load_vec(e, other + 0x30);
        store_vec(e, block, first);
        let second = load_vec(e, bodies + 0x30);
        store_vec(e, block + 0x10, second);
        let axis = u32::from(e.mem.u8(atom + 2)) << 4;
        let third = load_vec(e, bodies + axis);
        store_vec(e, block + 0x20, third);
        let tau = x87_quiet(e.mem.f32(atom + 4));
        e.mem.set_f32(block + 0x30, tau);
        let limit = x87_quiet(e.mem.f32(atom + 8));
        e.mem.set_f32(block + 0x34, limit);
        e.call(callee, &args![block, tail[0], tail[1]]);
    });
}

// Translated from 00d6d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// A user-tau bilateral linear builder (no engine-map name). Registers: the atom (EAX) and the
/// body pair (ECX); then the stack words `a`, `other` and `b`. Builds the 0x38-byte input block
/// (see `linear_input_block`) and calls `hk1dLinearBilateralConstraintUserTauBuildJacobian`
/// with the block, `a` and `b`.
pub fn fn_00d6d220(e: &mut Engine, atom: Ptr, bodies: Ptr, a: u32, other: Ptr, b: u32) {
    linear_input_block(
        e,
        atom.addr(),
        bodies.addr(),
        other.addr(),
        LINEAR_BILATERAL_USER_TAU,
        [a, b],
    );
}

// Translated from 00d6d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A linear-limit builder (no engine-map name), the same input block as `fn_00d6d220`.
/// Registers: the atom (EAX) and the body pair (ECX); then the stack words `a`, `other` and
/// `b`. Calls `hk1dLinearLimitBuildJacobian` with the block, `a` and `b`.
pub fn fn_00d6d280(e: &mut Engine, atom: Ptr, bodies: Ptr, a: u32, other: Ptr, b: u32) {
    linear_input_block(
        e,
        atom.addr(),
        bodies.addr(),
        other.addr(),
        LINEAR_LIMIT,
        [a, b],
    );
}

// Translated from 00d6d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular friction record (no engine-map name). Registers: the atom (ECX), the cursor
/// pointer (EDX) and the info block (ESI); stack word: the rows base. When `atom.byte[2]` is
/// not 0 and the float at `atom + 8` is not 0 (a NaN counts as not 0), calls
/// `hk1dAngularFrictionBuildJacobian` with a stack block `{rows + (atom.byte[3] << 4), info
/// [0x64], friction, atom.byte[4]}`, the info block and the cursor pointer. Otherwise appends a
/// record: the byte `atom.byte[4]` at `+4`, `FLT_EPSILON` added to the first float and the tag
/// 3; the cursor moves on by 0x10.
pub fn fn_00d6d990(e: &mut Engine, atom: Ptr, cursor: Ptr, info: Ptr, rows: u32) {
    let atom = atom.addr();
    if e.mem.u8(atom + 2) != 0 && 0.0 != f64::from(e.mem.f32(atom + 8)) {
        e.with_stack(0x10, |e, block| {
            let base = rows.wrapping_add(u32::from(e.mem.u8(atom + 3)) << 4);
            e.mem.set_u32(block.addr(), base);
            let friction = x87_quiet(e.mem.f32(atom + 8));
            e.mem.set_f32(block.addr() + 8, friction);
            let group = u32::from(e.mem.u8(atom + 4));
            e.mem.set_u32(block.addr() + 0xc, group);
            let word = e.mem.u32(info.addr() + 0x64);
            e.mem.set_u32(block.addr() + 4, word);
            e.call(ANGULAR_FRICTION, &args![block, info, cursor]);
        });
        return;
    }
    let record = e.mem.u32(cursor.addr());
    let group = e.mem.u8(atom + 4);
    e.mem.set_u8(record + 4, group);
    add_epsilon(e, record);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

/// The one-constraint-header record of `fn_00d6f2b0` and `fn_00d6ea30`: the first float gets
/// `FLT_EPSILON` added, `info.word[0x70]` (low 16 bits) goes to `+6`, `info.word[0x6c]` (low 16)
/// to `+4`, `info.word[0x60]` to `+0xc`, `value` to `+8`, `flag` to `+1` and the tag 1 to `+0`.
fn header_record(e: &mut Engine, info: u32, cursor: Ptr, value: f32, flag: u8) {
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    let high = e.mem.u32(info + 0x70) as u16;
    e.mem.set_u16(record + 6, high);
    let id = e.mem.u32(info + 0x60);
    e.mem.set_u32(record + 0xc, id);
    let low = e.mem.u32(info + 0x6c) as u16;
    e.mem.set_u16(record + 4, low);
    e.mem.set_u32(record + 8, value.to_bits());
    e.mem.set_u8(record, 1);
    e.mem.set_u8(record + 1, flag);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

// Translated from 00d6f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A header record (no engine-map name; cdecl: the info block, the cursor pointer, a float and
/// a byte). See `header_record`. The float word is copied as bits.
pub fn fn_00d6f2b0(e: &mut Engine, info: Ptr, cursor: Ptr, value: f32, flag: u8) {
    header_record(e, info.addr(), cursor, value, flag);
}

// Translated from 00d6ea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same header record as `fn_00d6f2b0`, with the info block (ECX) and the cursor pointer
/// (EDX) in registers and the float and the byte on the stack.
pub fn fn_00d6ea30(e: &mut Engine, info: Ptr, cursor: Ptr, value: f32, flag: u8) {
    header_record(e, info.addr(), cursor, value, flag);
}

/// `hkMath_atan2fApproximation` (engine map; cdecl: two floats, the result in ST0).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;
/// The CRT square root (`00ec6040`, argument in ST0).
const X87_SQRT: u32 = 0x00ec_6040;
/// `hkCalcMotorData` (engine map; cdecl: the motor, an input block, an output block).
const CALC_MOTOR_DATA: u32 = 0x00dc_fff0;
/// The `float` 1.1920929e-07 the axis builders compare a squared length with.
const SMALL_FLOAT: u32 = 0x0104_da4c;
/// Two byte tables indexed by an axis number (the offsets of the other two axes in a row of
/// 16-byte vectors).
const NEXT_AXIS_TABLE: u32 = 0x010d_33c1;
const THIRD_AXIS_TABLE: u32 = 0x010d_33c2;
/// The `double`s -1.0, 0.5, pi and 2 pi (the exe's truncated values) the angle code uses.
const MINUS_ONE: u32 = 0x0101_a6b0;
const HALF: u32 = 0x0101_1588;
const PI: u32 = 0x0101_ff40;
const TWO_PI: u32 = 0x0101_ff48;
/// The `double` 1e-16 below which a summed axis is taken as zero (`fn_00d6d450`).
const TINY: u32 = 0x010d_33d0;
/// The `double` the angle limit is scaled by in `fn_00d6d720`.
const LIMIT_SCALE: u32 = 0x0103_c7c8;
/// `float` tables indexed by a byte of the atom: the multiplier (`+4` per entry from
/// `010d33e0`), the offset (`010d33d8`) and the axis sign (`010d33e4` going down).
const ANGLE_SCALE_TABLE: u32 = 0x010d_33e0;
const ANGLE_OFFSET_TABLE: u32 = 0x010d_33d8;
const AXIS_SIGN_TABLE: u32 = 0x010d_33e4;

/// Moves `angle` (a `float`) by whole turns of 2 pi until it lies within pi of `limit`, the way
/// the two x87 loops do: each step is added or subtracted in double precision and stored as a
/// `float`; a comparison with NaN is false.
fn wrap_angle(e: &Engine, angle: f32, limit: f32) -> f32 {
    let pi: f64 = e.global(PI);
    let two_pi: f64 = e.global(TWO_PI);
    let limit = f64::from(limit);
    let mut angle = angle;
    if pi < limit - f64::from(angle) {
        loop {
            angle = (f64::from(angle) + two_pi) as f32;
            let again = limit - f64::from(angle) > pi;
            if !again {
                break;
            }
        }
    }
    if f64::from(angle) - limit > pi {
        loop {
            angle = (f64::from(angle) - two_pi) as f32;
            let again = f64::from(angle) - limit > pi;
            if !again {
                break;
            }
        }
    }
    angle
}

// Translated from 00d6d2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular-limit record (no engine-map name; cdecl). Registers: the atom (ECX) and the
/// output record (ESI); stack words: the info block, two bases of rows of 16-byte vectors.
/// Returns 1 when the atom's byte `+2` is 0 and writes nothing. Otherwise (`axis = atom.byte[3]`)
/// the record gets row `axis` of the first base at `+0`, the atom's floats `+4` and `+8` at
/// `+0x10` and `+0x14`, the angle between the two bases about that axis
/// (`hkMath_atan2fApproximation(dot, cross . row)`, with the other two axes taken from the
/// byte tables at `010d33c1` and `010d33c2`) at `+0x18`, moved by whole turns to within pi of
/// `-info[0x64][1]` (the limit), and `info.float[0x40] * atom.float[3]` at `+0x1c`. Returns 0.
/// (The turn loops run forever on an infinite angle, as in the exe.)
pub fn fn_00d6d2e0(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let (atom, record, info) = (atom.addr(), record.addr(), info.addr());
    if e.mem.u8(atom + 2) == 0 {
        return 1;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let first = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis));
    let second = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis));
    let product =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0xc)));
    e.mem.set_f32(record + 0x1c, product as f32);
    let axis_row = axis << 4;
    let row = load_vec(e, rows_a.addr() + axis_row);
    store_vec(e, record, row);
    let other_a = load_vec(e, rows_a.addr() + first);
    let tau = x87_quiet(e.mem.f32(atom + 4));
    e.mem.set_f32(record + 0x10, tau);
    let other_b = load_vec(e, rows_b.addr() + second);
    let axis_b = load_vec(e, rows_b.addr() + axis_row);
    let limit_value = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x14, limit_value);
    let dot = sum3(mul(other_b, other_a));
    let turned = sum3(mul(cross(other_b, axis_b), other_a));
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(dot).to_bits(), x87_quiet(turned).to_bits()],
        )
        .f32();
    let limits = e.mem.u32(info + 0x64);
    let scale: f64 = e.global(MINUS_ONE);
    let limit = (f64::from(x87_quiet(e.mem.f32(limits + 4))) * scale) as f32;
    let wrapped = wrap_angle(e, angle, limit);
    e.mem.set_f32(record + 0x18, wrapped);
    0
}

// Translated from 00d6d450 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record about the sum of two rows (no engine-map name; cdecl). Registers: the
/// first base (EDX), the atom (ESI) and the output record (EDI); stack words: the info block
/// and the second base. With `axis = atom.byte[3]` and `other = atom.byte[4]`, `n` is the
/// row `axis` of the second base added to that of the first, normalised when its length is
/// above `1e-16` (and then `half` is half the length; otherwise `n` is the second base's row
/// itself and `half` is 0). The record gets row `axis` of the first base at `+0` (replaced
/// by `n` at the end), the atom's floats `+8` and `+0xc` at `+0x10` and `+0x14`, the angle
/// `hkMath_atan2fApproximation(a . t, a . (t x n))` with `t = n x b` (`a` and `b` the row
/// `other` of the two bases) at `+0x18` and `half * half * info.float[0x40] *
/// atom.float[4]` at `+0x1c`. Returns 0.
pub fn fn_00d6d450(
    e: &mut Engine,
    base_a: Ptr,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    base_b: Ptr,
) -> u32 {
    let (base_a, atom, record, info, base_b) = (
        base_a.addr(),
        atom.addr(),
        record.addr(),
        info.addr(),
        base_b.addr(),
    );
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    let first = load_vec(e, base_a + axis_row);
    store_vec(e, record, first);
    let tau = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x10, tau);
    let limit_value = x87_quiet(e.mem.f32(atom + 0xc));
    e.mem.set_f32(record + 0x14, limit_value);
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    let other_row = u32::from(e.mem.u8(atom + 4)) << 4;
    let axis_b = load_vec(e, base_b + axis_row);
    let axis_a = load_vec(e, base_a + axis_row);
    let sum = add(axis_a, axis_b);
    let length = sum3(mul(sum, sum)).sqrt();
    let tiny: f64 = e.global(TINY);
    let (half, normal) = if f64::from(x87_quiet(length)) > tiny {
        let half_constant: f64 = e.global(HALF);
        let half = (half_constant * f64::from(length)) as f32;
        let inverse = (1.0f64 / f64::from(length)) as f32;
        (half, mul([inverse; 4], sum))
    } else {
        (0.0f32, axis_b)
    };
    let other_b = load_vec(e, base_b + other_row);
    let tangent = cross(normal, other_b);
    let binormal = cross(tangent, normal);
    let other_a = load_vec(e, base_a + other_row);
    let along = sum3(mul(other_a, tangent));
    let across = sum3(mul(other_a, binormal));
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(along).to_bits(), x87_quiet(across).to_bits()],
        )
        .f32();
    e.mem.set_f32(record + 0x18, angle);
    store_vec(e, record, normal);
    let weight =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0x10)));
    let half = f64::from(x87_quiet(half));
    e.mem
        .set_f32(record + 0x1c, ((weight * half) * half) as f32);
    0
}

/// The common part of `fn_00d6d5d0` and `fn_00d6d720`: the axes `a` (row `atom.byte[3]` of the
/// first base) and `b` (row `atom.byte[4]` of the second); returns `None` (the functions return
/// 1) when the squared length of `a x b` is below `FLT_EPSILON`, else writes the record and
/// returns the angle written at `+0x18`: `atan2(|a x b|, a . b)` scaled and offset by the
/// tables at `010d33e0` and `010d33d8` (index `atom.byte[5]`), with `(a x b) / |a x b|` times
/// the axis sign (`010d33e4`, going down) at `+0`, the atom's floats `+8` and `+0xc` at
/// `+0x10` and `+0x14` and `info.float[0x40] * atom.float[4]` at `+0x1c`.
fn axis_angle_record(
    e: &mut Engine,
    atom: u32,
    record: u32,
    info: u32,
    rows_a: u32,
    rows_b: u32,
) -> Option<f32> {
    let a_row = (u32::from(e.mem.u8(atom + 3)) << 4).wrapping_add(rows_a);
    let b_row = (u32::from(e.mem.u8(atom + 4)) << 4).wrapping_add(rows_b);
    let a = load_vec(e, a_row);
    let b = load_vec(e, b_row);
    let turned = cross(a, b);
    let squared = sum3(mul(turned, turned));
    let small: f32 = e.global(SMALL_FLOAT);
    if x87_quiet(squared) < x87_quiet(small) {
        return None;
    }
    let index = u32::from(e.mem.u8(atom + 5));
    let dot = sum3(mul(b, a));
    let length = e
        .call(X87_SQRT, &args![f64::from(x87_quiet(squared))])
        .f32();
    let angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(length).to_bits(), x87_quiet(dot).to_bits()],
        )
        .f64();
    let scale = f64::from(e.global::<f32>(ANGLE_SCALE_TABLE + 4 * index));
    let offset = f64::from(e.global::<f32>(ANGLE_OFFSET_TABLE + 4 * index));
    let value = (angle * scale + offset) as f32;
    let sign = x87_quiet(e.global::<f32>(AXIS_SIGN_TABLE.wrapping_sub(4 * index)));
    let weight =
        f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(x87_quiet(e.mem.f32(atom + 0x10)));
    e.mem.set_f32(record + 0x1c, weight as f32);
    let inverse = (1.0f64 / f64::from(x87_quiet(length))) as f32;
    e.mem.set_f32(record + 0x18, x87_quiet(value));
    let scaled = mul(mul([inverse; 4], turned), [sign; 4]);
    store_vec(e, record, scaled);
    let tau = x87_quiet(e.mem.f32(atom + 8));
    e.mem.set_f32(record + 0x10, tau);
    let limit_value = x87_quiet(e.mem.f32(atom + 0xc));
    e.mem.set_f32(record + 0x14, limit_value);
    Some(value)
}

// Translated from 00d6d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An angular record from two axes (no engine-map name; cdecl). Registers: the atom (ESI) and
/// the output record (EDI); stack words: the info block and the two bases. See
/// `axis_angle_record`; returns 1 for parallel axes and otherwise 0.
pub fn fn_00d6d5d0(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let written = axis_angle_record(
        e,
        atom.addr(),
        record.addr(),
        info.addr(),
        rows_a.addr(),
        rows_b.addr(),
    );
    u32::from(written.is_none())
}

// Translated from 00d6d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same record as `fn_00d6d5d0` followed by a limit update (no engine-map name; cdecl; the
/// same registers and stack words). Returns 1 for parallel axes, 0 when `atom.byte[6] <= 1`
/// (nothing more), else, with `limits = info.word[0x19]` and `slot = atom.byte[6]`: with the
/// angle `v` just written, `a = limits[slot]`, `c = -limits[1]`, `m = info.float[3]`, the new
/// bound `M = max(0.5 * m, a * K)` (`K` the double at `0103c7c8`; NaN picks `0.5 * m`) and the
/// differences `x = v - a` and `y = c - x`; `limits[slot]` becomes `a - M` (but at least
/// `FLT_EPSILON`) and the float at `+0x18` of the record `x + M`, both reduced by `y` /
/// raised by it when `0.5 * (0.5 * m + 2 M) < -y` and `c` is not 0.
pub fn fn_00d6d720(
    e: &mut Engine,
    atom: Ptr,
    record: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
) -> u32 {
    let (atom, record, info) = (atom.addr(), record.addr(), info.addr());
    let Some(angle) = axis_angle_record(e, atom, record, info, rows_a.addr(), rows_b.addr()) else {
        return 1;
    };
    let slot = u32::from(e.mem.u8(atom + 6));
    if slot <= 1 {
        return 0;
    }
    let limits = e.mem.u32(info + 0x64);
    let a = x87_quiet(e.mem.f32(limits + slot));
    let c = -x87_quiet(e.mem.f32(limits + 4));
    let x = (f64::from(angle) - f64::from(a)) as f32;
    let y = (f64::from(c) - f64::from(x)) as f32;
    let half_constant: f64 = e.global(HALF);
    let m = x87_quiet(e.mem.f32(info + 0xc));
    let half_m = (half_constant * f64::from(m)) as f32;
    let scale: f64 = e.global(LIMIT_SCALE);
    let a_scaled = (f64::from(a) * scale) as f32;
    let bound = if half_m < a_scaled { a_scaled } else { half_m };
    let mut low = (f64::from(a) - f64::from(bound)) as f32;
    let mut high = (f64::from(x) + f64::from(bound)) as f32;
    let sum = f64::from(half_m) + (f64::from(bound) + f64::from(bound));
    let blend = (half_constant * sum) as f32;
    if blend < -y && f64::from(c) != 0.0 {
        low = (f64::from(low) - f64::from(y)) as f32;
        high = (f64::from(y) + f64::from(high)) as f32;
    }
    let small: f32 = e.global(SMALL_FLOAT);
    low = if x87_quiet(small) < low {
        low
    } else {
        x87_quiet(small)
    };
    e.mem.set_f32(limits + slot, low);
    e.mem.set_f32(record + 0x18, high);
    0
}

/// Asks `hkCalcMotorData` for the six floats of a motor: the input block is `{inverse, info,
/// limits[0], limits[1], remaining, passed}` (the limits as raw words) and the output has six
/// words, of which the code reads `1..=5`.
fn calc_motor_data(e: &mut Engine, motor: u32, block: [u32; 6]) -> [f32; 6] {
    e.with_stack(0x18, |e, input| {
        for (index, word) in block.into_iter().enumerate() {
            e.mem.set_u32(input.addr() + 4 * index as u32, word);
        }
        e.with_stack(0x18, |e, output| {
            e.call(CALC_MOTOR_DATA, &args![motor, input, output]);
            std::array::from_fn(|word| e.mem.f32(output.addr() + 4 * word as u32))
        })
    })
}

/// Completes the 0x40-byte tag-15 record at the cursor from the motor data: `begin + 0x1c`
/// becomes `info.float[2] * out[3]`; the record gets `FLT_EPSILON` added to its first float,
/// the tag 15, `out[2]` and `out[3]` times `info.float[1]` at `+0x20` and `+0x24` and `out[1]`,
/// `out[4]` and `out[5]` at `+0x28`, `+0x2c` and `+0x30`; the cursor moves on by 0x40.
fn commit_motor_record(e: &mut Engine, info: u32, cursor: Ptr, begin: u32, out: [f32; 6]) {
    let weight = f64::from(x87_quiet(e.mem.f32(info + 8))) * f64::from(out[3]);
    e.mem.set_f32(begin + 0x1c, weight as f32);
    let record = e.mem.u32(cursor.addr());
    add_epsilon(e, record);
    e.mem.set_u8(record, 0xf);
    let tau = f64::from(x87_quiet(e.mem.f32(info + 4)));
    e.mem
        .set_f32(record + 0x20, (f64::from(out[2]) * tau) as f32);
    e.mem
        .set_f32(record + 0x24, (f64::from(out[3]) * tau) as f32);
    e.mem.set_u32(cursor.addr(), record + 0x40);
    e.mem.set_f32(record + 0x28, x87_quiet(out[1]));
    e.mem.set_f32(record + 0x2c, x87_quiet(out[4]));
    e.mem.set_f32(record + 0x30, x87_quiet(out[5]));
}

/// The motor atoms' placeholder record: `{tag 3, byte[4] = marker}` and the cursor on by 0x10.
fn placeholder_record(e: &mut Engine, cursor: Ptr, marker: u8) {
    let record = e.mem.u32(cursor.addr());
    e.mem.set_u8(record + 4, marker);
    add_epsilon(e, record);
    e.mem.set_u8(record, 3);
    e.mem.set_u32(cursor.addr(), record + 0x10);
}

// Translated from 00d6da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromAngMotorAtom` (engine map; cdecl). Registers: the info block (ESI) and
/// the atom (EDI); stack words: two bases of rows of 16-byte vectors and the cursor pointer.
/// `flag` and `value` are the byte and the float at `info.word[0x19] + atom.short[2]` and
/// `+ atom.short[3]`. Without a motor (`atom.word[4]`) or with the atom's byte `+2` 0 it
/// appends the placeholder record (`{tag 3, byte[4] = 1}`). Otherwise it takes the angle of the
/// two bases about row `atom.byte[3]` (`hkMath_atan2fApproximation(a . b, a . (b x c))`),
/// moved by whole turns to within pi of `-info.float[atom.short[4] + 4]` when that offset is
/// not 0 (`wrap_angle`), stores it in `value` when `flag` is 0, makes the angular pair and its
/// inverse effective mass in the record at the cursor, asks `hkCalcMotorData` (the motor,
/// `{1 / mass, info, limits[0], limits[1], atom.float[3] - value, value - angle}`) and
/// completes the 0x40-byte record from the answer (`commit_motor_record`); `value` and `flag`
/// are set to `atom.float[3]` and 1 when the two floats differ or `flag` is 0.
pub fn build_jacobian_from_ang_motor_atom(
    e: &mut Engine,
    info: Ptr,
    atom: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
    cursor: Ptr,
) {
    let (info, atom) = (info.addr(), atom.addr());
    let limits = e.mem.u32(info + 0x64);
    let flag = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let value = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    let motor = e.mem.u32(atom + 0x10);
    if motor == 0 || e.mem.u8(atom + 2) == 0 {
        placeholder_record(e, cursor, 1);
        return;
    }
    let axis = u32::from(e.mem.u8(atom + 3));
    let first = u32::from(e.mem.u8(NEXT_AXIS_TABLE + axis)) + rows_a.addr();
    let second = u32::from(e.mem.u8(THIRD_AXIS_TABLE + axis)) + rows_b.addr();
    let a = load_vec(e, first);
    let b = load_vec(e, second);
    let dot = sum3(mul(a, b));
    let c = load_vec(e, (axis << 4) + rows_b.addr());
    let turned = sum3(mul(cross(b, c), a));
    let mut angle = e
        .call(
            ATAN2_APPROXIMATION,
            &args![x87_quiet(dot).to_bits(), x87_quiet(turned).to_bits()],
        )
        .f32();
    let offset = e.mem.u16(atom + 8);
    if offset != 0 {
        let at = limits.wrapping_add(offset as i16 as i32 as u32);
        let scale: f64 = e.global(MINUS_ONE);
        let bound = (f64::from(x87_quiet(e.mem.f32(at + 4))) * scale) as f32;
        angle = wrap_angle(e, angle, bound);
    }
    if e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(angle));
    }
    let record = e.mem.u32(cursor.addr());
    let axis_vector = (axis << 4) + rows_a.addr();
    let inverse = angular_inverse(e, info, axis_vector, record);
    let bounds = [e.mem.u32(limits), e.mem.u32(limits + 4)];
    let target = x87_quiet(e.mem.f32(atom + 0xc));
    let current = x87_quiet(e.mem.f32(value));
    let remaining = (f64::from(target) - f64::from(current)) as f32;
    let current = x87_quiet(e.mem.f32(value));
    let passed = (f64::from(current) - f64::from(angle)) as f32;
    let block = [
        inverse.to_bits(),
        info,
        bounds[0],
        bounds[1],
        remaining.to_bits(),
        passed.to_bits(),
    ];
    let out = calc_motor_data(e, motor, block);
    commit_motor_record(e, info, cursor, record, out);
    let now = e.mem.f32(value);
    let wanted = e.mem.f32(atom + 0xc);
    // `FUCOMPP`: only equal, ordered values keep the update away.
    if now != wanted || e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(wanted));
        e.mem.set_u8(flag, 1);
    }
}

/// The angular pair of `vector` into `record`, then `1 / (sum + FLT_EPSILON)` rounded to a
/// `float` stored at `record + 0xc` and returned (the begin step of the motor builders).
fn angular_inverse(e: &mut Engine, info: u32, vector: u32, record: u32) -> f32 {
    let sum = angular_pair(e, info, vector, record);
    let epsilon: f64 = e.global(EPSILON);
    let total = (f64::from(sum) + epsilon) as f32;
    let inverse = (1.0f64 / f64::from(x87_quiet(total))) as f32;
    e.mem.set_f32(record + 0xc, inverse);
    inverse
}

/// `hkMatrix3::setMulMat3Mat3` (engine map; cdecl: the result matrix, two matrices).
const SET_MUL_MAT3_MAT3: u32 = 0x00cd_5530;

/// The unit axis of a ragdoll cone limit: `sum` times `1 / |sum|` when the length of its first
/// three lanes is above `1e-16`, else `fallback`.
fn limit_axis(e: &Engine, sum: Vec4, fallback: Vec4) -> Vec4 {
    let length = x87_quiet(sum3(mul(sum, sum)).sqrt());
    let tiny: f64 = e.global(TINY);
    if f64::from(length) > tiny {
        let inverse = (1.0f64 / f64::from(length)) as f32;
        mul([inverse; 4], sum)
    } else {
        fallback
    }
}

/// `hkMath_atan2fApproximation(t . v, (t x axis) . v)` with `t = axis x w`, the angle of the
/// vector `v` about `axis` measured from `w`.
fn angle_about(e: &mut Engine, axis: Vec4, w: Vec4, v: Vec4) -> f32 {
    let tangent = cross(axis, w);
    let binormal = cross(tangent, axis);
    let along = sum3(mul(tangent, v));
    let across = sum3(mul(binormal, v));
    e.call(
        ATAN2_APPROXIMATION,
        &args![x87_quiet(along).to_bits(), x87_quiet(across).to_bits()],
    )
    .f32()
}

// Translated from 00d6dd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromRagdollMotorAtom` (engine map; cdecl). Registers: the atom (EAX) and the
/// info block (EDI); stack words: two bases of rows of 16-byte vectors and the cursor pointer.
/// With the atom's byte `+2` 0 it appends the placeholder record (`{tag 3, byte[4] = 3}`) and
/// stops. Otherwise it multiplies the info block's matrix (`info.word[0xf]`) and the atom's
/// matrix (`+0x10`) with `hkMatrix3::setMulMat3Mat3` and, for the three motors at `atom + 0x40`
/// (axis `p` = 0, 1, 2 and `q` = 1, 0, 0): a motor of 0 appends the placeholder record
/// (`{tag 3, byte[4] = 1}`); otherwise the angle about the axis of the rows `p` of the two
/// bases (`limit_axis`, `angle_about` with the rows `q`) is the first angle and, about the
/// axis of the product's row `p` added to the second base's, the second angle. The first
/// angle goes to `value[i]` when `flag[i]` is 0 (`flag` and `value` as for
/// `build_jacobian_from_ang_motor_atom`, one byte and one float further per motor). The
/// angular pair of the second axis and `hkCalcMotorData` (limits `info.word[0x19][2 i]`,
/// `[2 i + 1]`, the second angle minus `value[i]`, `value[i]` minus the first angle) make the
/// 0x40-byte record (`commit_motor_record`); `value[i]` is replaced by the second angle when
/// they differ.
pub fn build_jacobian_from_ragdoll_motor_atom(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    rows_a: Ptr,
    rows_b: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let limits = e.mem.u32(info + 0x64);
    let flags = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let values = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    if e.mem.u8(atom + 2) == 0 {
        placeholder_record(e, cursor, 3);
        return;
    }
    let matrix_word = e.mem.u32(info + 0x3c);
    e.with_stack(0x30, |e, product| {
        e.call(SET_MUL_MAT3_MAT3, &args![product, matrix_word, atom + 0x10]);
        for motor_index in 0..3u32 {
            let axis = motor_index;
            let other = [1u32, 0, 0][motor_index as usize];
            let motor = e.mem.u32(atom + 0x40 + 4 * motor_index);
            if motor == 0 {
                placeholder_record(e, cursor, 1);
                continue;
            }
            let flag = flags + motor_index;
            let value = values + 4 * motor_index;
            let axis_row = axis << 4;
            let other_row = other << 4;
            let (rows_a, rows_b) = (rows_a.addr(), rows_b.addr());
            let b_axis = load_vec(e, rows_b + axis_row);
            let a_axis = load_vec(e, rows_a + axis_row);
            let first_axis = limit_axis(e, add(a_axis, b_axis), b_axis);
            let b_other = load_vec(e, rows_b + other_row);
            let a_other = load_vec(e, rows_a + other_row);
            let first_angle = angle_about(e, first_axis, b_other, a_other);
            let product_axis = load_vec(e, product.addr() + axis_row);
            let second_axis = limit_axis(e, add(product_axis, b_axis), b_axis);
            let product_other = load_vec(e, product.addr() + other_row);
            let second_angle = angle_about(e, second_axis, b_other, product_other);
            if e.mem.u8(flag) == 0 {
                e.mem.set_f32(value, x87_quiet(first_angle));
                e.mem.set_u8(flag, 1);
            }
            let record = e.mem.u32(cursor.addr());
            let inverse = e.with_stack(0x10, |e, vector| {
                store_vec(e, vector.addr(), second_axis);
                angular_inverse(e, info, vector.addr(), record)
            });
            let bounds = [
                e.mem.u32(limits + 8 * motor_index),
                e.mem.u32(limits + 8 * motor_index + 4),
            ];
            let current = x87_quiet(e.mem.f32(value));
            let remaining = (f64::from(second_angle) - f64::from(current)) as f32;
            let current = x87_quiet(e.mem.f32(value));
            let passed = (f64::from(current) - f64::from(first_angle)) as f32;
            let block = [
                inverse.to_bits(),
                info,
                bounds[0],
                bounds[1],
                remaining.to_bits(),
                passed.to_bits(),
            ];
            let out = calc_motor_data(e, motor, block);
            commit_motor_record(e, info, cursor, record, out);
            if second_angle != e.mem.f32(value) {
                e.mem.set_f32(value, x87_quiet(second_angle));
            }
        }
    });
}

/// `hk1dLinearFrictionBuildJacobian` (engine map; cdecl: an input block, the info block, the
/// cursor pointer).
const LINEAR_FRICTION: u32 = 0x00dc_f100;
/// `hkAddVelocityBuildJacobian` (engine map; cdecl: a vector, a flag, the cursor pointer).
const ADD_VELOCITY: u32 = 0x00dc_edd0;
/// Unnamed (cdecl: two vectors, the cursor pointer): appends a tag-25 record of the two vectors.
const PUSH_VECTOR_PAIR: u32 = 0x00dc_ed90;
/// `hkSetCenterOfMassBuildJacobian` (engine map; cdecl: two matrices, the cursor pointer).
const SET_CENTER_OF_MASS: u32 = 0x00dc_ee10;
/// Unnamed (thiscall, one stack word, `RET 4`): copies a body's solver data like
/// `fn_00d71f90`, but the vectors at `+0x10..+0x80` as 16-byte rows (bytes `1..4` and the word
/// at `+0xc` stay as they were).
const COPY_BODY_DATA: u32 = 0x00d7_2030;
/// `hk1dLinearVelocityMotorBeginJacobian` and `hk1dLinearVelocityMotorCommitJacobian` (engine
/// map; cdecl).
const LINEAR_VELOCITY_BEGIN: u32 = 0x00dd_0260;
const LINEAR_VELOCITY_COMMIT: u32 = 0x00dd_03c0;
/// `hkVector4::setRotatedDir` (thiscall: the result, a matrix, a vector), `hkVector4::setMul3`
/// (the same body), `hkMatrix3::setCrossSkewSymmetric` (thiscall: the matrix, a vector),
/// `hkMatrix3::setMulInverse` (thiscall: the result, two matrices) and the unnamed row scaler
/// (thiscall: the matrix, a vector whose first lane scales every row).
const SET_ROTATED_DIR: u32 = 0x00c7_f700;
const SET_MUL3: u32 = 0x00c7_f750;
const SET_CROSS_SKEW_SYMMETRIC: u32 = 0x00cd_4cc0;
const SET_MUL_INVERSE: u32 = 0x00cd_5610;
const SCALE_ROWS: u32 = 0x00cd_4c70;
/// The `float` (-1.0) the center-of-mass changer scales the rows with.
const MINUS_ONE_FLOAT: u32 = 0x0101_2054;
/// Sign bits in the first three lanes only.
const SIGN_MASK_XYZ: u32 = 0x010c_4c00;

// Translated from 00d6e280 (decompiled, FalloutNV.exe 1.4.0.525)
/// A linear friction record (no engine-map name; cdecl). Registers: the atom (ECX), the info
/// block (EDX), the cursor pointer (ESI) and a base of 16-byte rows (EDI); stack word: the
/// bodies block. When the atom's byte `+2` is not 0 and its float `+4` is above 0 it calls
/// `hk1dLinearFrictionBuildJacobian` with a stack block `{bodies.vector[0x30], rows[atom.byte[3]],
/// atom.float[1], info.word[0x19]}`, the info block and the cursor pointer; otherwise it
/// appends the placeholder record (`{tag 3, byte[4] = 1}`).
pub fn fn_00d6e280(e: &mut Engine, atom: Ptr, info: Ptr, cursor: Ptr, rows: Ptr, bodies: Ptr) {
    let (atom, info) = (atom.addr(), info.addr());
    if e.mem.u8(atom + 2) != 0 && 0.0 < f64::from(e.mem.f32(atom + 4)) {
        let friction = x87_quiet(e.mem.f32(atom + 4));
        let body_vector = load_vec(e, bodies.addr() + 0x30);
        let row = (u32::from(e.mem.u8(atom + 3)) << 4).wrapping_add(rows.addr());
        let limits = e.mem.u32(info + 0x64);
        e.with_stack(0x30, |e, block| {
            let block_address = block.addr();
            store_vec(e, block_address, body_vector);
            let row_vector = load_vec(e, row);
            store_vec(e, block_address + 0x10, row_vector);
            e.mem.set_f32(block_address + 0x20, friction);
            e.mem.set_u32(block_address + 0x24, limits);
            e.call(LINEAR_FRICTION, &args![block, info, cursor]);
        });
        return;
    }
    placeholder_record(e, cursor, 1);
}

/// The 0x40-byte tag-11 record of `fn_00d6e300` and `fn_00d6e6b0` (a length constraint between
/// two points with an offset and a second weight: no engine-map name, so the identity is
/// unconfirmed). `points` are `p0` (the first body's arm), `p1` (the second body's arm), `p2`
/// and `p3` (the far ends). With `d1 = p2 - p0` and `d2 = p1 - p3` the unit directions `n1`
/// of `d1` and `n2` of `d2` (the first x axis, the second its negative, when the squared
/// length is not above `FLT_EPSILON`; the rsqrt-based `inverse_length` otherwise) fill `+0`
/// and `+0x10` (`scale * n2`); `+0x20` and `+0x30` are the arms' cross products rotated into the
/// two bodies, `+0x2c` is `info.float[9] / (mass terms)`, `+0xc` is
/// `((scale * |d2| + |d1|) - offset) * info.float[2]`. The first float gets `FLT_EPSILON` added,
/// the tag 11 is set and the cursor moves on by 0x40.
fn spring_record(
    e: &mut Engine,
    info: u32,
    cursor: Ptr,
    points: [Vec4; 4],
    offset: f32,
    scale: f32,
) {
    let [p0, p1, p2, p3] = points;
    let first_difference = sub(p2, p0);
    let second_difference = sub(p1, p3);
    let record = e.mem.u32(cursor.addr());
    let small = x87_quiet(e.global::<f32>(SMALL_FLOAT));
    let axis_x = load_vec(e, AXIS_TABLE);
    let first_squared = sum3(mul(first_difference, first_difference));
    let body_a = e.mem.u32(info + 0x30);
    let body_b = e.mem.u32(info + 0x34);
    let first = if small < first_squared {
        mul([inverse_length(e, first_squared); 4], first_difference)
    } else {
        axis_x
    };
    let second_squared = sum3(mul(second_difference, second_difference));
    let second = if small < second_squared {
        mul([inverse_length(e, second_squared); 4], second_difference)
    } else {
        negate(e, axis_x)
    };
    let info_weight = x87_quiet(e.mem.f32(info + 8));
    let info_numerator = x87_quiet(e.mem.f32(info + 0x24));
    let scaled = mul([scale; 4], second);
    store_vec(e, record + 0x10, scaled);
    store_vec(e, record, first);
    let arm_a = sub(p0, load_vec(e, body_a + 0x40));
    let arm_b = sub(p1, load_vec(e, body_b + 0x40));
    let turn_a = cross(arm_a, first);
    let turn_b = cross(scaled, arm_b);
    let rows_a = body_rows(e, body_a);
    let angular_a = rotate_rows(turn_a, rows_a);
    store_vec(e, record + 0x20, angular_a);
    let rows_b = body_rows(e, body_b);
    let angular_b = rotate_rows(turn_b, rows_b);
    store_vec(e, record + 0x30, angular_b);
    let inverse_a = load_vec(e, body_a + 0x30);
    let inverse_b = load_vec(e, body_b + 0x30);
    let terms = add(
        mul(mul(angular_a, angular_a), inverse_a),
        mul(mul(angular_b, angular_b), inverse_b),
    );
    let inertia = sum3(terms);
    let scale_squared = (f64::from(scale) * f64::from(scale)) as f32;
    let mass_a = e.mem.f32(body_a + 0x3c);
    let mass_b = e.mem.f32(body_b + 0x3c);
    let epsilon: f64 = e.global(EPSILON);
    let mass_term =
        ((f64::from(scale_squared) * f64::from(mass_b) + f64::from(mass_a)) + epsilon) as f32;
    let denominator = (f64::from(mass_term) + f64::from(inertia)) as f32;
    let first_length = sum3(mul(first_difference, first_difference)).sqrt();
    let second_length = sum3(mul(second_difference, second_difference)).sqrt();
    e.mem.set_f32(
        record + 0x2c,
        (f64::from(info_numerator) / f64::from(denominator)) as f32,
    );
    let reach = (f64::from(scale) * f64::from(second_length) + f64::from(first_length)) as f32;
    let excess = (f64::from(reach) - f64::from(offset)) as f32;
    e.mem.set_f32(
        record + 0xc,
        (f64::from(excess) * f64::from(info_weight)) as f32,
    );
    let next = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), next.wrapping_add(0x40));
    add_epsilon(e, next);
    e.mem.set_u8(next, 0xb);
}

// Translated from 00d6e300 (decompiled, FalloutNV.exe 1.4.0.525)
/// A tag-11 length record from one input block (no engine-map name; cdecl). Register: the
/// input block (EDI): the points at `+0`, `+0x10`, `+0x20`, `+0x30` and the floats `+0x40`
/// (the offset) and `+0x44` (the scale); stack words: the info block and the cursor pointer.
/// See `spring_record`.
pub fn fn_00d6e300(e: &mut Engine, block: Ptr, info: Ptr, cursor: Ptr) {
    let block = block.addr();
    let points = [
        load_vec(e, block),
        load_vec(e, block + 0x10),
        load_vec(e, block + 0x20),
        load_vec(e, block + 0x30),
    ];
    let offset = x87_quiet(e.mem.f32(block + 0x40));
    let scale = x87_quiet(e.mem.f32(block + 0x44));
    spring_record(e, info.addr(), cursor, points, offset, scale);
}

// Translated from 00d6e6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same tag-11 length record from three input blocks (no engine-map name; cdecl).
/// Registers: `first` (EAX), `second` (ECX) and the info block (EDI); stack words: `third` and
/// the cursor pointer. The points are `second[0x30]`, `third[0x30]`, `first[0x10]` and
/// `first[0x20]`; the offset and the scale are the floats `first[0x30]` and `first[0x34]`.
pub fn fn_00d6e6b0(e: &mut Engine, first: Ptr, second: Ptr, info: Ptr, third: Ptr, cursor: Ptr) {
    let (first, second, third) = (first.addr(), second.addr(), third.addr());
    let offset = x87_quiet(e.mem.f32(first + 0x30));
    let points = [
        load_vec(e, second + 0x30),
        load_vec(e, third + 0x30),
        load_vec(e, first + 0x10),
        load_vec(e, first + 0x20),
    ];
    let scale = x87_quiet(e.mem.f32(first + 0x34));
    spring_record(e, info.addr(), cursor, points, offset, scale);
}

// Translated from 00d6ea70 (decompiled, FalloutNV.exe 1.4.0.525)
/// A modifier that lowers the contact points' values (no engine-map name; cdecl). Register:
/// the info block (EDX); stack words: the atom and the cursor pointer. Backs the cursor up by
/// 0x10, copies the info block to the stack with `float[9]`, `float[0x10]` and `float[0x11]`
/// multiplied by the atom's `float[8]`, walks the atom chain (`+0x14`, while the type is at least
/// 0x17) to a node with `count = word[2]`, `skip = word[3]` and `stride = byte[0xa]` and, for
/// each of the `count` entries (`entry j` has its float at `node + 0x4c + 0x20 j`), takes
/// `u = atom.float[9] * info.float[1] * info.float[0] / atom.float[8] + entry` and
/// lowers the float at `node + 0x30 + (skip << 5) + stride j + 0x10` to `u` when `u` is smaller.
/// Then builds the Jacobian of the next atom: a contact atom (type 0x16) through
/// `hkSimpleContactConstraintDataBuildJacobian` (the flag is `count & 0xff00 | 1`), any other
/// through `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`.
pub fn fn_00d6ea70(e: &mut Engine, info: Ptr, atom: Ptr, cursor: Ptr) {
    let (info, atom) = (info.addr(), atom.addr());
    let position = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
    let scale = x87_quiet(e.mem.f32(atom + 0x20));
    let id = u32::from(e.mem.u16(atom + 0x12));
    let numerator = x87_quiet(e.mem.f32(info + 0x24));
    e.with_stack(0x80, |e, copy| {
        let copy = copy.addr();
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        e.mem.set_f32(
            copy + 0x24,
            (f64::from(numerator) * f64::from(scale)) as f32,
        );
        let first = f64::from(x87_quiet(e.mem.f32(info + 0x40))) * f64::from(scale);
        e.mem.set_f32(copy + 0x40, first as f32);
        let second = f64::from(scale) * f64::from(x87_quiet(e.mem.f32(info + 0x44)));
        e.mem.set_f32(copy + 0x44, second as f32);
        let mut node = atom;
        loop {
            node = e.mem.u32(node + 0x14);
            if e.mem.u16(node) < 0x17 {
                break;
            }
        }
        let count = u32::from(e.mem.u16(node + 4));
        let skip = u32::from(e.mem.u16(node + 6));
        let stride = u32::from(e.mem.u8(node + 0xa));
        let targets = node.wrapping_add(0x30).wrapping_add(skip << 5);
        for entry in 0..count {
            let value = x87_quiet(e.mem.f32(node + 0x4c + 0x20 * entry));
            let atom_numerator = f64::from(x87_quiet(e.mem.f32(atom + 0x24)));
            let first_factor = f64::from(x87_quiet(e.mem.f32(info + 4)));
            let second_factor = f64::from(x87_quiet(e.mem.f32(info)));
            let divisor = f64::from(x87_quiet(e.mem.f32(atom + 0x20)));
            let quotient = (((atom_numerator * first_factor) * second_factor) / divisor) as f32;
            let sum = f64::from(quotient) + f64::from(value);
            let target = targets
                .wrapping_add(stride.wrapping_mul(entry))
                .wrapping_add(0x10);
            if f64::from(x87_quiet(e.mem.f32(target))) > sum {
                e.mem.set_f32(target, sum as f32);
            }
        }
        let next = e.mem.u32(atom + 0x14);
        if e.mem.u16(next) == 0x16 {
            let flag = (count & 0xff00) | 1;
            e.call(BUILD_CONTACT_JACOBIAN, &args![next, copy, flag, cursor]);
        } else {
            e.call(BUILD_NOT_CONTACT_JACOBIAN, &args![next, id, copy, cursor]);
        }
    });
}

// Translated from 00d6ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A modifier that scales the bodies' vectors around one Jacobian (no engine-map name; cdecl).
/// Register: the atom (EAX); stack words: the info block and the cursor pointer. Copies the
/// info block and, with `fn_00d71f90`, both bodies' solver data to the stack; the copy's body
/// pointers (`+0x30`, `+0x34`) point to those copies, whose vectors at `+0x30` are multiplied
/// by the atom's vectors at `+0x20` and `+0x30`. Appends the tag-25 record of the two scaled
/// vectors (`00dced90`), then the Jacobian of the next atom built from the info copy (type
/// 0x16: `hkSimpleContactConstraintDataBuildJacobian` with the flag `id & 0xff00 | 1`; any
/// other: `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`), then the
/// tag-25 record of the bodies' own vectors at `+0x30`.
pub fn fn_00d6ecb0(e: &mut Engine, atom: Ptr, info: Ptr, cursor: Ptr) {
    let (atom, info) = (atom.addr(), info.addr());
    // The exe's locals in one block at its own offsets: the second body copy at `+0x30`, the
    // first at `+0xb0` and the info copy at `+0x130`.
    e.with_stack(0x1b0, |e, frame| {
        let frame = frame.addr();
        let (copy_b, copy_a, info_copy) = (frame + 0x30, frame + 0xb0, frame + 0x130);
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(info_copy + 4 * word, value);
        }
        let first_scale = load_vec(e, atom + 0x20);
        let next = e.mem.u32(atom + 0x14);
        let second_scale = load_vec(e, atom + 0x30);
        let id = u32::from(e.mem.u16(atom + 0x12));
        let body_a = e.mem.u32(info + 0x30);
        fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
        let body_b = e.mem.u32(info + 0x34);
        fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
        let scaled_a = mul(load_vec(e, copy_a + 0x30), first_scale);
        store_vec(e, copy_a + 0x30, scaled_a);
        e.mem.set_u32(info_copy + 0x30, copy_a);
        let scaled_b = mul(load_vec(e, copy_b + 0x30), second_scale);
        store_vec(e, copy_b + 0x30, scaled_b);
        e.mem.set_u32(info_copy + 0x34, copy_b);
        e.call(
            PUSH_VECTOR_PAIR,
            &args![copy_a + 0x30, copy_b + 0x30, cursor],
        );
        if e.mem.u16(next) == 0x16 {
            let flag = (id & 0xffff_ff00) | 1;
            e.call(
                BUILD_CONTACT_JACOBIAN,
                &args![next, info_copy, flag, cursor],
            );
        } else {
            e.call(
                BUILD_NOT_CONTACT_JACOBIAN,
                &args![next, id, info_copy, cursor],
            );
        }
        let body_a = e.mem.u32(info + 0x30);
        let body_b = e.mem.u32(info + 0x34);
        e.call(
            PUSH_VECTOR_PAIR,
            &args![body_a + 0x30, body_b + 0x30, cursor],
        );
    });
}

// Translated from 00d71f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a body's solver data (no engine-map name; thiscall, one stack word, `RET 4`): `this`
/// gets the byte at `+0`, the words at `+4` and `+0x10..+0x50`, the float at `+8` (through
/// the x87 stack, so a signalling NaN is quieted) and the 16-byte rows at `+0x50`, `+0x60`
/// and `+0x70` of `source`; bytes `1..4` and the word at `+0xc` stay as they were. Returns
/// `this`.
pub fn fn_00d71f90(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let (to, from) = (this.addr(), source.addr());
    let byte = e.mem.u8(from);
    e.mem.set_u8(to, byte);
    let word = e.mem.u32(from + 4);
    e.mem.set_u32(to + 4, word);
    let value = x87_quiet(e.mem.f32(from + 8));
    e.mem.set_f32(to + 8, value);
    for offset in (0x10..0x50).step_by(4) {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(to + offset, word);
    }
    for offset in [0x50, 0x60, 0x70] {
        copy_vec(e, from + offset, to + offset);
    }
    this
}

// Translated from 00d6f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromMovingSurfaceModifier` (engine map; cdecl). Register: the atom (EAX); stack
/// words: the info block and the cursor pointer. Copies the info block to the stack and
/// replaces the last record (the cursor backs up by 0x10) by a header record whose value word is
/// the address after the next atom's entries (`node + 0x30 + (word[3] << 5)`) and whose flag
/// is the byte `node[0xa]`. A local copy of body B (when `info.word[0x1b]` is 0, else of body
/// A; `00d72030`) replaces that body in the original info block, which keeps the stack
/// pointer; the atom's vector at `+0x20` (negated in its first three lanes for B) loses its
/// part along the node's vector at `+0x40` and is negated, `y`; the velocity record for `y`
/// (`hkAddVelocityBuildJacobian`), the copy's vector at `+0x10` plus `y`, the node's Jacobian
/// (`hkSimpleContactConstraintDataBuildJacobian`, flag `info.word[0x18] & 0xffffff00`, from the
/// info copy) and the velocity record for `-y` follow.
pub fn build_jacobian_from_moving_surface_modifier(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let velocity = load_vec(e, atom + 0x20);
    e.with_stack(0x80, |e, copy| {
        let copy = copy.addr();
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(copy + 4 * word, value);
        }
        let node = e.mem.u32(atom + 0x14);
        let position = e.mem.u32(cursor.addr());
        e.mem.set_u32(cursor.addr(), position.wrapping_sub(0x10));
        let skip = u32::from(e.mem.u16(node + 6));
        let flag_byte = e.mem.u8(node + 0xa);
        let targets = skip.wrapping_shl(5).wrapping_add(0x30).wrapping_add(node);
        let identifier = e.mem.u32(info + 0x60);
        header_record(e, info, cursor, f32::from_bits(targets), flag_byte);
        let second_body = e.mem.u32(info + 0x6c) == 0;
        e.with_stack(0x80, |e, buffer| {
            let buffer_address = buffer.addr();
            let mut direction = velocity;
            if second_body {
                let body = e.mem.u32(info + 0x34);
                e.call(COPY_BODY_DATA, &args![buffer, body]);
                e.mem.set_u32(info + 0x34, buffer_address);
                let mask = load_bits(e, SIGN_MASK_XYZ);
                direction = std::array::from_fn(|lane| {
                    f32::from_bits(velocity[lane].to_bits() ^ mask[lane])
                });
            } else {
                let body = e.mem.u32(info + 0x30);
                e.call(COPY_BODY_DATA, &args![buffer, body]);
                e.mem.set_u32(info + 0x30, buffer_address);
            }
            let normal = load_vec(e, node + 0x40);
            let along = sum3(mul(normal, direction));
            let rest = sub(direction, mul([along; 4], normal));
            let negated = negate(e, rest);
            e.with_stack(0x10, |e, vector| {
                store_vec(e, vector.addr(), negated);
                e.call(ADD_VELOCITY, &args![vector, u32::from(second_body), cursor]);
                let body_vector = load_vec(e, buffer_address + 0x10);
                let pushed = load_vec(e, vector.addr());
                store_vec(e, buffer_address + 0x10, add(body_vector, pushed));
                let flag = identifier & 0xffff_ff00;
                e.call(BUILD_CONTACT_JACOBIAN, &args![node, copy, flag, cursor]);
                let pushed = load_vec(e, vector.addr());
                let opposite = negate(e, pushed);
                store_vec(e, vector.addr(), opposite);
                e.call(ADD_VELOCITY, &args![vector, u32::from(second_body), cursor]);
            });
        });
    });
}

// Translated from 00d6f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromLinMotorAtom` (engine map; cdecl). Register: the atom (ESI); stack words:
/// the info block, two bases (`first`, `second`) of 16-byte rows and the cursor pointer.
/// `flag` and `value` are the byte and the float at `info.word[0x19] + atom.short[2]` and
/// `+ atom.short[3]`. Without a motor (`atom.word[3]`) or with the atom's byte `+2` 0 it appends
/// the placeholder record (`{tag 3, byte[4] = 1}`). Otherwise `value` becomes `atom.float[2]`
/// when `flag` is 0, `hk1dLinearVelocityMotorBeginJacobian` makes the begin record from the
/// row `atom.byte[3]` of `second` and `first[0x30]`, `hkCalcMotorData` (the motor, `{begin,
/// info, limits[0], limits[1], atom.float[2] - value, value - (first[0x30] - second[0x30]) .
/// row}`) and `hk1dLinearVelocityMotorCommitJacobian` complete it; `value` and `flag` are
/// set to `atom.float[2]` and 1 when `atom.float[2]` differs from `value` or `flag` is 0.
pub fn build_jacobian_from_lin_motor_atom(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    first: Ptr,
    second: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    let (first, second) = (first.addr(), second.addr());
    let limits = e.mem.u32(info + 0x64);
    let flag = limits.wrapping_add(e.mem.u16(atom + 4) as i16 as i32 as u32);
    let value = limits.wrapping_add(e.mem.u16(atom + 6) as i16 as i32 as u32);
    let motor = e.mem.u32(atom + 0xc);
    if e.mem.u8(atom + 2) == 0 || motor == 0 {
        placeholder_record(e, cursor, 1);
        return;
    }
    if e.mem.u8(flag) == 0 {
        let target = x87_quiet(e.mem.f32(atom + 8));
        e.mem.set_f32(value, target);
    }
    let record = e.mem.u32(cursor.addr());
    let axis_row = u32::from(e.mem.u8(atom + 3)) << 4;
    e.with_stack(0x18, |e, block| {
        let block_address = block.addr();
        // The block's words 2 and 3 are zero before the call; the exe leaves the others unset.
        e.mem.set_u32(block_address + 8, 0);
        e.mem.set_u32(block_address + 0xc, 0);
        e.call(
            LINEAR_VELOCITY_BEGIN,
            &args![second + axis_row, first + 0x30, info, record, block],
        );
        e.mem.set_u32(block_address + 4, info);
        let target = x87_quiet(e.mem.f32(atom + 8));
        let current = x87_quiet(e.mem.f32(value));
        let remaining = (f64::from(target) - f64::from(current)) as f32;
        let bounds = [e.mem.u32(limits), e.mem.u32(limits + 4)];
        e.mem.set_u32(block_address + 8, bounds[0]);
        e.mem.set_f32(block_address + 0x10, remaining);
        let current = x87_quiet(e.mem.f32(value));
        e.mem.set_u32(block_address + 0xc, bounds[1]);
        let relative = sub(load_vec(e, first + 0x30), load_vec(e, second + 0x30));
        let axis = load_vec(e, second + axis_row);
        let along = sum3(mul(relative, axis));
        let passed = (f64::from(current) - f64::from(along)) as f32;
        e.mem.set_f32(block_address + 0x14, passed);
        e.with_stack(0x18, |e, out| {
            e.call(CALC_MOTOR_DATA, &args![motor, block, out]);
            e.call(LINEAR_VELOCITY_COMMIT, &args![out, info, record, cursor]);
        });
    });
    let now = e.mem.f32(value);
    let wanted = e.mem.f32(atom + 8);
    if wanted != now || e.mem.u8(flag) == 0 {
        e.mem.set_f32(value, x87_quiet(wanted));
        e.mem.set_u8(flag, 1);
    }
}

// Translated from 00d6edf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `buildJacobianFromCenterOfMassChangerModifier` (engine map; cdecl). Stack words: the atom,
/// the info block and the cursor pointer. Copies the info block and, with `fn_00d71f90`, both
/// bodies' solver data to the stack; the copy's body pointers (`+0x30`, `+0x34`) point to those
/// copies. The atom's vectors at `+0x20` and `+0x30` are turned by the matrices `info.word[0xe]`
/// and `info.word[0xf]` (`hkVector4::setRotatedDir`) and added to the body copies' vectors at
/// `+0x40`. The skew-symmetric matrices of the two turned vectors
/// (`hkMatrix3::setCrossSkewSymmetric`) are multiplied by the inverse of each body copy's
/// rotation rows at `+0x50` (`hkMatrix3::setMulInverse`) into `m1` and `m2`. Copies of `m1` and
/// `m2` with the rows scaled by the `float` at `01012054` (-1.0) are applied to each body
/// copy's vector at `+0x20` (`hkVector4::setMul3`) and added to its vector at `+0x10`. The
/// center-of-mass record of the scaled copies (`hkSetCenterOfMassBuildJacobian`), the Jacobian
/// of the next atom built from the info copy (type 0x16:
/// `hkSimpleContactConstraintDataBuildJacobian` with the flag `id & 0xff00 | 1`; any other:
/// `hkSolverBuildJacobianFromAtomsNotContact` with the id at `atom + 0x12`) and the
/// center-of-mass record of `m1` and `m2` follow.
pub fn build_jacobian_from_center_of_mass_changer_modifier(
    e: &mut Engine,
    atom: Ptr,
    info: Ptr,
    cursor: Ptr,
) {
    let (atom, info) = (atom.addr(), info.addr());
    // One block for all of the exe's locals, at the exe's own offsets (the block start is the
    // stack pointer of the body): the id at `+0xc`, the scale vector at `+0x10`, `m2` at `+0x20`,
    // the turned vectors at `+0x50` and `+0x60`, `m1` at `+0x70`, a scratch matrix at `+0xa0`,
    // the scaled copies at `+0xd0` and `+0x110`, the products at `+0x100` and `+0x140`, the two
    // body copies at `+0x150` and `+0x1d0` and the info copy at `+0x250`.
    e.with_stack(0x2d0, |e, frame| {
        let frame = frame.addr();
        let (turned_a, turned_b) = (frame + 0x60, frame + 0x50);
        let (m1, m2, scratch) = (frame + 0x70, frame + 0x20, frame + 0xa0);
        let (scaled_1, scaled_2, scale) = (frame + 0xd0, frame + 0x110, frame + 0x10);
        let (product_1, product_2) = (frame + 0x140, frame + 0x100);
        let (copy_a, copy_b, info_copy) = (frame + 0x150, frame + 0x1d0, frame + 0x250);
        for word in 0..0x20 {
            let value = e.mem.u32(info + 4 * word);
            e.mem.set_u32(info_copy + 4 * word, value);
        }
        let id = u32::from(e.mem.u16(atom + 0x12));
        let next = e.mem.u32(atom + 0x14);
        let rotation_a = e.mem.u32(info + 0x38);
        e.call(SET_ROTATED_DIR, &args![turned_a, rotation_a, atom + 0x20]);
        let rotation_b = e.mem.u32(info + 0x3c);
        e.call(SET_ROTATED_DIR, &args![turned_b, rotation_b, atom + 0x30]);
        let body_a = e.mem.u32(info + 0x30);
        fn_00d71f90(e, Ptr::new(copy_a), Ptr::new(body_a));
        let body_b = e.mem.u32(info + 0x34);
        fn_00d71f90(e, Ptr::new(copy_b), Ptr::new(body_b));
        let position_a = add(load_vec(e, copy_a + 0x40), load_vec(e, turned_a));
        store_vec(e, copy_a + 0x40, position_a);
        let position_b = add(load_vec(e, copy_b + 0x40), load_vec(e, turned_b));
        e.mem.set_u32(info_copy + 0x34, copy_b);
        store_vec(e, copy_b + 0x40, position_b);
        e.mem.set_u32(info_copy + 0x30, copy_a);
        e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m1, turned_a]);
        e.call(SET_CROSS_SKEW_SYMMETRIC, &args![m2, turned_b]);
        for (matrix, body_copy) in [(m1, copy_a), (m2, copy_b)] {
            for row in 0..3 {
                copy_vec(e, matrix + 0x10 * row, scratch + 0x10 * row);
            }
            e.call(SET_MUL_INVERSE, &args![matrix, scratch, body_copy + 0x50]);
        }
        let minus_one = e.global::<u32>(MINUS_ONE_FLOAT);
        for (matrix, scaled) in [(m1, scaled_1), (m2, scaled_2)] {
            for row in 0..3 {
                copy_vec(e, matrix + 0x10 * row, scaled + 0x10 * row);
            }
            store_bits(e, scale, [minus_one, 0, 0, 0]);
            e.call(SCALE_ROWS, &args![scaled, scale]);
        }
        e.call(SET_MUL3, &args![product_1, scaled_1, copy_a + 0x20]);
        e.call(SET_MUL3, &args![product_2, scaled_2, copy_b + 0x20]);
        let linear_a = add(load_vec(e, copy_a + 0x10), load_vec(e, product_1));
        store_vec(e, copy_a + 0x10, linear_a);
        let linear_b = add(load_vec(e, copy_b + 0x10), load_vec(e, product_2));
        store_vec(e, copy_b + 0x10, linear_b);
        e.call(SET_CENTER_OF_MASS, &args![scaled_1, scaled_2, cursor]);
        if e.mem.u16(next) == 0x16 {
            let flag = (id & 0xffff_ff00) | 1;
            e.call(
                BUILD_CONTACT_JACOBIAN,
                &args![next, info_copy, flag, cursor],
            );
        } else {
            e.call(
                BUILD_NOT_CONTACT_JACOBIAN,
                &args![next, id, info_copy, cursor],
            );
        }
        e.call(SET_CENTER_OF_MASS, &args![m1, m2, cursor]);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00d6_c5a0,
            build_jacobian_from_viscous_surface_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_c630, fn_00d6c630(Ptr, Ptr, Ptr, Ptr, f32, f32)),
        entry!(0x00d6_c970, fn_00d6c970(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_cb30, fn_00d6cb30(Ptr, Ptr, Ptr)),
        entry!(0x00d6_cc80, fn_00d6cc80(Ptr, Ptr, Ptr, Ptr)),
        entry!(
            0x00d6_cd70,
            hk1d_angular_velocity_motor_begin_jacobian(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_ce70,
            hk1d_angular_velocity_motor_commit_jacobian(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_d220, fn_00d6d220(Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x00d6_d280, fn_00d6d280(Ptr, Ptr, u32, Ptr, u32)),
        entry!(0x00d6_d990, fn_00d6d990(Ptr, Ptr, Ptr, u32)),
        entry!(0x00d6_ea30, fn_00d6ea30(Ptr, Ptr, f32, u8)),
        entry!(0x00d6_f2b0, fn_00d6f2b0(Ptr, Ptr, f32, u8)),
        entry!(0x00d6_d2e0, fn_00d6d2e0(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d450, fn_00d6d450(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d5d0, fn_00d6d5d0(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(0x00d6_d720, fn_00d6d720(Ptr, Ptr, Ptr, Ptr, Ptr) -> u32),
        entry!(
            0x00d6_da00,
            build_jacobian_from_ang_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_dd60,
            build_jacobian_from_ragdoll_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d6_e280, fn_00d6e280(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_e300, fn_00d6e300(Ptr, Ptr, Ptr)),
        entry!(0x00d6_e6b0, fn_00d6e6b0(Ptr, Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d6_ea70, fn_00d6ea70(Ptr, Ptr, Ptr)),
        entry!(0x00d6_ecb0, fn_00d6ecb0(Ptr, Ptr, Ptr)),
        entry!(
            0x00d6_edf0,
            build_jacobian_from_center_of_mass_changer_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_f0c0,
            build_jacobian_from_moving_surface_modifier(Ptr, Ptr, Ptr)
        ),
        entry!(
            0x00d6_f300,
            build_jacobian_from_lin_motor_atom(Ptr, Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x00d7_1f90, fn_00d71f90(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range, `FLT_EPSILON` and `FLT_MAX` as doubles, the three
    /// axes and the all-lanes sign mask.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e.set_global(MAX_FLOAT, f64::from(f32::MAX));
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
        }
        e
    }

    /// A body: inverse inertia at `+0x30`, centre of mass at `+0x40`, identity rotation rows.
    fn body(e: &mut Engine, position: Vec4, inverse: Vec4, angular: Vec4) -> u32 {
        let body = e.mem.alloc(0x80);
        store_vec(e, body + 0x20, angular);
        store_vec(e, body + 0x30, inverse);
        store_vec(e, body + 0x40, position);
        store_vec(e, body + 0x50, [1.0, 0.0, 0.0, 0.0]);
        store_vec(e, body + 0x60, [0.0, 1.0, 0.0, 0.0]);
        store_vec(e, body + 0x70, [0.0, 0.0, 1.0, 0.0]);
        body
    }

    /// An info block with weight 2, slope 5 and two bodies (inverse inertias 1 and 2, w lanes
    /// 0.5; B turning about z when `spin` is set).
    fn info(e: &mut Engine, spin: bool) -> u32 {
        let first = body(e, [0.0; 4], [1.0, 1.0, 1.0, 0.5], [0.0; 4]);
        let angular = if spin { [0.0, 0.0, 1.0, 0.0] } else { [0.0; 4] };
        let second = body(e, [0.0; 4], [2.0, 2.0, 2.0, 0.5], angular);
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x20, 2.0);
        e.mem.set_f32(info + 0x24, 5.0);
        e.mem.set_u32(info + 0x30, first);
        e.mem.set_u32(info + 0x34, second);
        info
    }

    fn point(e: &mut Engine, value: Vec4) -> u32 {
        let address = e.mem.alloc(0x10);
        store_vec(e, address, value);
        address
    }

    fn cell(e: &mut Engine, value: u32) -> u32 {
        let address = e.mem.alloc(8);
        e.mem.set_u32(address, value);
        address
    }

    #[test]
    fn plain_axis_records_carry_the_axis_the_angular_terms_and_the_mass() {
        let mut e = engine();
        let info = info(&mut e, false);
        let a = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = point(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let record = e.mem.alloc(0x90);
        fn_00d6c970(
            &mut e,
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(info),
            Ptr::new(record),
        );
        let first = record;
        assert_eq!(e.mem.f32(first + 0xc), 4.0);
        assert_eq!(e.mem.u32(first + 0x2c), 0x3f80_0001);
        assert_eq!(
            e.mem.f32(first + 0x1c),
            (5.0f64 / f64::from(f32::from_bits(0x3f80_0001))) as f32
        );
        let second = record + 0x30;
        assert_eq!(load_vec(&e, second + 0x10)[..3], [0.0, 0.0, 1.0]);
        assert_eq!(load_vec(&e, second + 0x20)[..3], [0.0, 0.0, -3.0]);
        assert_eq!(e.mem.f32(second + 0x2c), 20.0);
        assert_eq!(e.mem.f32(second + 0x1c), 0.25);
        assert_eq!(e.mem.f32(second + 0xc), 0.0);
        let third = record + 0x60;
        assert_eq!(load_vec(&e, third + 0x10)[..3], [0.0, -1.0, 0.0]);
        assert_eq!(load_vec(&e, third + 0x20)[..3], [0.0, 3.0, 0.0]);
    }

    fn rotate_double(e: &mut Engine) {
        e.register_double(ROTATE_ANGULAR, |e, a| {
            let vector = load_vec(e, a[2]);
            store_vec(e, a[0], vector);
            Ret::default()
        });
    }

    fn counter_double(e: &mut Engine) {
        e.register_double(INTERLOCKED_EXCHANGE_ADD, |e, a| {
            let old = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], old + a[1]);
            Ret {
                eax: old,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn limit_records_add_the_rotation_error_and_tag_the_limit() {
        let mut e = engine();
        rotate_double(&mut e);
        counter_double(&mut e);
        let info = info(&mut e, true);
        e.mem.set_f32(info + 0x50, 1.0);
        e.mem.set_u32(info + 0x60, 0x1234);
        let list = e.mem.alloc(0x20);
        e.mem.set_u32(info + 0x68, list);
        let a = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = point(&mut e, [3.0, 0.0, 0.0, 0.0]);
        let record = e.mem.alloc(0x90);
        let cursor = cell(&mut e, record);
        fn_00d6c630(
            &mut e,
            Ptr::new(info),
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(cursor),
            f32::MAX,
            4.0,
        );
        assert_eq!(e.mem.u32(cursor), record + 0x90);
        assert_eq!(e.mem.u32(list), 1);
        assert_eq!(e.mem.u32(list + 4), 0x1234);
        assert_eq!(e.mem.f32(record + 0xc), 4.0);
        assert_eq!(e.mem.f32(record + 0x3c), -12.0);
        assert_eq!(e.mem.f32(record + 0x6c), 0.0);
        assert_eq!(e.mem.u32(record), 0x3f80_0005);
        assert_eq!(e.mem.u32(record + 0x30), 0x3400_0005);
        assert_eq!(e.mem.u8(record + 0x60), 5);
        e.mem.set_f32(info + 0x50, 100.0);
        e.mem.set_u32(cursor, record);
        fn_00d6c630(
            &mut e,
            Ptr::new(info),
            Ptr::new(a),
            Ptr::new(b),
            Ptr::new(cursor),
            7.5,
            4.0,
        );
        assert_eq!(e.mem.u32(list), 1);
        assert_eq!(e.mem.u8(record), 6);
        assert_eq!(e.mem.f32(record + 0x2c), 7.5);
        assert_eq!(e.mem.f32(record + 0x5c), 7.5);
    }

    #[test]
    fn the_angular_record_scales_the_motor_terms() {
        let mut e = engine();
        let info = info(&mut e, false);
        e.mem.set_f32(info + 8, 1.5);
        let input = e.mem.alloc(0x40);
        store_vec(&mut e, input, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_f32(input + 0x10, 2.0);
        e.mem.set_f32(input + 0x14, 3.0);
        e.mem.set_f32(input + 0x18, 4.0);
        e.mem.set_f32(input + 0x1c, 0.5);
        let record = e.mem.alloc(0x40);
        let cursor = cell(&mut e, record);
        fn_00d6cb30(&mut e, Ptr::new(input), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(e.mem.u32(cursor), record + 0x30);
        assert_eq!(e.mem.u32(record), 0x3f80_000d);
        assert_eq!(load_vec(&e, record + 0x10)[..3], [-1.0, 0.0, 0.0]);
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        assert_eq!(e.mem.f32(record + 0xc), (0.5f64 / f64::from(total)) as f32);
        assert_eq!(e.mem.f32(record + 0x1c), -6.0);
        assert_eq!(e.mem.f32(record + 0x20), -1.5);
        assert_eq!(e.mem.f32(record + 0x24), -2.25);
        assert_eq!(e.mem.f32(record + 0x28), 0.5);
    }

    #[test]
    fn both_angular_motor_begin_functions_store_the_inverse_mass() {
        let mut e = engine();
        let info = info(&mut e, false);
        let vector = point(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        let inverse = (1.0f64 / f64::from(total)) as f32;
        for (index, registers) in [true, false].into_iter().enumerate() {
            let record = e.mem.alloc(0x20);
            let out = e.mem.alloc(8);
            if registers {
                fn_00d6cc80(
                    &mut e,
                    Ptr::new(info),
                    Ptr::new(record),
                    Ptr::new(vector),
                    Ptr::new(out),
                );
            } else {
                hk1d_angular_velocity_motor_begin_jacobian(
                    &mut e,
                    Ptr::new(vector),
                    Ptr::new(info),
                    Ptr::new(record),
                    Ptr::new(out),
                );
            }
            assert_eq!(
                load_vec(&e, record)[..3],
                [1.0, 0.0, 0.0],
                "variant {index}"
            );
            assert_eq!(load_vec(&e, record + 0x10)[..3], [-1.0, 0.0, 0.0]);
            assert_eq!(e.mem.f32(record + 0xc), inverse);
            assert_eq!(e.mem.f32(out), inverse);
        }
    }

    #[test]
    fn the_commit_record_copies_the_motor_terms() {
        let mut e = engine();
        let block = e.mem.alloc(0x40);
        for (index, value) in [2.0f32, 7.0, 3.0, 4.0, 8.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(block + 4 * index as u32, value);
        }
        let state = e.mem.alloc(0x10);
        e.mem.set_f32(state + 4, 0.5);
        e.mem.set_f32(state + 8, 10.0);
        let begin = e.mem.alloc(0x40);
        let record = e.mem.alloc(0x40);
        e.mem.set_f32(record, 1.0);
        let cursor = cell(&mut e, record);
        hk1d_angular_velocity_motor_commit_jacobian(
            &mut e,
            Ptr::new(block),
            Ptr::new(state),
            Ptr::new(begin),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.f32(begin + 0x1c), 20.0);
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u32(record), 0x3f80_000f);
        let tail: Vec<f32> = [0x20, 0x24, 0x28, 0x2c, 0x30]
            .into_iter()
            .map(|offset| e.mem.f32(record + offset))
            .collect();
        assert_eq!(tail, vec![1.5, 2.0, 7.0, 8.0, 9.0]);
    }

    #[test]
    fn the_linear_input_blocks_gather_the_vectors_and_floats() {
        for (callee, builder) in [
            (
                LINEAR_BILATERAL_USER_TAU,
                fn_00d6d220 as fn(&mut Engine, Ptr, Ptr, u32, Ptr, u32),
            ),
            (LINEAR_LIMIT, fn_00d6d280),
        ] {
            let mut e = engine();
            let log: Log = Rc::default();
            let sink = Rc::clone(&log);
            e.register_double(callee, move |e, a| {
                let mut words = a[1..3].to_vec();
                words.extend((0..14).map(|n| e.mem.u32(a[0] + 4 * n)));
                sink.borrow_mut().push((callee, words));
                Ret::default()
            });
            let bodies = e.mem.alloc(0x80);
            store_vec(&mut e, bodies + 0x30, [1.0, 2.0, 3.0, 4.0]);
            store_vec(&mut e, bodies + 0x20, [9.0, 9.5, 9.25, 9.75]);
            let other = e.mem.alloc(0x40);
            store_vec(&mut e, other + 0x30, [5.0, 6.0, 7.0, 8.0]);
            let atom = e.mem.alloc(0x10);
            e.mem.set_u8(atom + 2, 2);
            e.mem.set_f32(atom + 4, 0.25);
            e.mem.set_f32(atom + 8, 0.75);
            builder(
                &mut e,
                Ptr::new(atom),
                Ptr::new(bodies),
                0x111,
                Ptr::new(other),
                0x222,
            );
            let floats = [
                5.0f32, 6.0, 7.0, 8.0, 1.0, 2.0, 3.0, 4.0, 9.0, 9.5, 9.25, 9.75, 0.25, 0.75,
            ];
            let mut expected = vec![0x111, 0x222];
            expected.extend(floats.iter().map(|value| value.to_bits()));
            assert_eq!(*log.borrow(), vec![(callee, expected)]);
        }
    }

    #[test]
    fn friction_calls_the_builder_for_a_nonzero_value_and_otherwise_writes_a_record() {
        let mut e = engine();
        let log: Log = Rc::default();
        let sink = Rc::clone(&log);
        e.register_double(ANGULAR_FRICTION, move |e, a| {
            let mut words = a[1..3].to_vec();
            words.extend((0..4).map(|n| e.mem.u32(a[0] + 4 * n)));
            sink.borrow_mut().push((a[0], words));
            Ret::default()
        });
        let atom = e.mem.alloc(0x10);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_u8(atom + 4, 6);
        e.mem.set_f32(atom + 8, 0.3);
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, 0xabcd);
        let record = e.mem.alloc(0x20);
        let cursor = cell(&mut e, record);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        {
            let calls = log.borrow();
            assert_eq!(calls.len(), 1);
            assert_eq!(
                calls[0].1,
                vec![info, cursor, 0x4000_0010, 0xabcd, 0.3f32.to_bits(), 6]
            );
        }
        assert_eq!(e.mem.u32(cursor), record);
        e.mem.set_u32(atom + 8, 0x7fa0_0000);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow()[1].1[4], 0x7fe0_0000);
        e.mem.set_f32(atom + 8, 0.0);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u32(record), 0x3400_0003);
        assert_eq!(e.mem.u8(record + 4), 6);
        e.mem.set_u8(atom + 2, 0);
        e.mem.set_f32(atom + 8, 1.0);
        fn_00d6d990(
            &mut e,
            Ptr::new(atom),
            Ptr::new(cursor),
            Ptr::new(info),
            0x4000_0000,
        );
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(e.mem.u32(cursor), record + 0x20);
    }

    #[test]
    fn both_header_records_hold_the_ids_the_value_and_the_flag() {
        let mut e = engine();
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x60, 0xaa);
        e.mem.set_u32(info + 0x6c, 0x1111_2222);
        e.mem.set_u32(info + 0x70, 0x3333_4444);
        for plain in [true, false] {
            let record = e.mem.alloc(0x20);
            e.mem.set_f32(record, 2.0);
            let cursor = cell(&mut e, record);
            if plain {
                fn_00d6f2b0(&mut e, Ptr::new(info), Ptr::new(cursor), 1.5, 7);
            } else {
                fn_00d6ea30(&mut e, Ptr::new(info), Ptr::new(cursor), 1.5, 7);
            }
            assert_eq!(e.mem.u32(cursor), record + 0x10);
            assert_eq!(e.mem.u8(record), 1);
            assert_eq!(e.mem.u8(record + 1), 7);
            assert_eq!(e.mem.u16(record + 4), 0x2222);
            assert_eq!(e.mem.u16(record + 6), 0x4444);
            assert_eq!(e.mem.u32(record + 8), 1.5f32.to_bits());
            assert_eq!(e.mem.u32(record + 0xc), 0xaa);
        }
    }

    #[test]
    fn the_viscosity_modifier_builds_the_next_atom_from_a_copy_of_the_info_block() {
        for contact in [false, true] {
            let mut e = engine();
            let log: Log = Rc::default();
            let source = e.mem.alloc(0x80);
            for word in 0..0x20 {
                e.mem.set_u32(source + 4 * word, 0x100 + word);
            }
            for (callee, tag) in [
                (ADD_SURFACE_VISCOSITY, 1u32),
                (BUILD_CONTACT_JACOBIAN, 2),
                (BUILD_NOT_CONTACT_JACOBIAN, 3),
            ] {
                let sink = Rc::clone(&log);
                e.register_double(callee, move |e, a| {
                    let mut words = vec![tag];
                    words.extend_from_slice(&a[..4.min(a.len())]);
                    if tag != 1 {
                        let copy = if tag == 2 { a[1] } else { a[2] };
                        words.push((0..0x20).all(|n| e.mem.u32(copy + 4 * n) == 0x100 + n) as u32);
                    }
                    sink.borrow_mut().push((callee, words));
                    Ret::default()
                });
            }
            let atom = e.mem.alloc(0x20);
            let first = e.mem.alloc(0x40);
            let second = e.mem.alloc(0x40);
            e.mem.set_u16(atom + 0x12, 0x77);
            e.mem.set_u32(atom + 0x14, first);
            e.mem.set_u16(first, if contact { 0x16 } else { 0x30 });
            e.mem.set_u32(first + 0x14, second);
            e.mem.set_u16(second, 5);
            let cursor = cell(&mut e, 0x5000);
            build_jacobian_from_viscous_surface_modifier(
                &mut e,
                Ptr::new(source),
                Ptr::new(atom),
                Ptr::new(cursor),
            );
            assert_eq!(e.mem.u32(cursor), 0x4ff0);
            let calls = log.borrow();
            if contact {
                assert_eq!(calls[0], (ADD_SURFACE_VISCOSITY, vec![1, first + 0x10]));
                assert_eq!(calls[1].0, BUILD_CONTACT_JACOBIAN);
                assert_eq!(calls[1].1[1], first);
                assert_eq!((calls[1].1[3], calls[1].1[4]), (1, cursor));
                assert_eq!(calls[1].1[5], 1);
            } else {
                assert_eq!(calls[0], (ADD_SURFACE_VISCOSITY, vec![1, second + 0x10]));
                assert_eq!(calls[1].0, BUILD_NOT_CONTACT_JACOBIAN);
                assert_eq!((calls[1].1[1], calls[1].1[2]), (first, 0x77));
                assert_eq!(calls[1].1[4], cursor);
                assert_eq!(calls[1].1[5], 1);
            }
        }
    }
}

#[cfg(test)]
mod continuation_tests {
    use super::super::hkpsphereshape::{HALF_F32, THREE_F32, ZERO_F32};
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the exe's data range and the constants the translated functions read.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.set_global(EPSILON, f64::from(f32::EPSILON));
        e.set_global(SMALL_FLOAT, f32::EPSILON);
        e.set_global(ZERO_F32, 0.0f32);
        e.set_global(THREE_F32, 3.0f32);
        e.set_global(HALF_F32, 0.5f32);
        e.set_global(PI, f64::from_bits(0x4009_21fb_6000_0000));
        e.set_global(TWO_PI, f64::from_bits(0x4019_21fb_6000_0000));
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(TINY, 1e-16f64);
        e.set_global(LIMIT_SCALE, 0.25f64);
        e.set_global(MINUS_ONE_FLOAT, -1.0f32);
        for lane in 0..4 {
            e.mem.set_u32(SIGN_MASK + 4 * lane, 0x8000_0000);
            e.mem.set_u32(
                SIGN_MASK_XYZ + 4 * lane,
                if lane < 3 { 0x8000_0000 } else { 0 },
            );
        }
        for axis in 0..3u32 {
            let mut vector = [0.0f32; 4];
            vector[axis as usize] = 1.0;
            store_vec(&mut e, AXIS_TABLE + 0x10 * axis, vector);
        }
        for (index, offset) in [0x10u8, 0x20, 0x00, 0x10].into_iter().enumerate() {
            e.mem.set_u8(NEXT_AXIS_TABLE + index as u32, offset);
        }
        // Index 1 of the angle tables: offset 0.5, sign -1.0 and scale 2.0.
        e.mem.set_f32(ANGLE_OFFSET_TABLE + 4, 0.5);
        e.mem.set_f32(AXIS_SIGN_TABLE - 4, -1.0);
        e.mem.set_f32(ANGLE_SCALE_TABLE + 4, 2.0);
        e
    }

    fn cell(e: &mut Engine, value: u32) -> u32 {
        let address = e.mem.alloc(8);
        e.mem.set_u32(address, value);
        address
    }

    /// Three rows of 16 bytes: the identity.
    fn identity(e: &mut Engine) -> u32 {
        let rows = e.mem.alloc(0x30);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, rows + 0x10 * axis, row);
        }
        rows
    }

    /// A body: inverse inertia (`w` 0.5) at `+0x30`, position at `+0x40`, identity rotation.
    fn body(e: &mut Engine, inverse: [f32; 3], position: Vec4) -> u32 {
        let body = e.mem.alloc(0x80);
        store_vec(e, body + 0x30, [inverse[0], inverse[1], inverse[2], 0.5]);
        store_vec(e, body + 0x40, position);
        for axis in 0..3u32 {
            let mut row = [0.0f32; 4];
            row[axis as usize] = 1.0;
            store_vec(e, body + 0x50 + 0x10 * axis, row);
        }
        body
    }

    /// An info block with weight 2, slope 5, motor factors 0.5 (`+4`) and 1.5 (`+8`) and two
    /// bodies (inverse inertias 1 and 2).
    fn info(e: &mut Engine) -> u32 {
        let first = body(e, [1.0; 3], [0.0; 4]);
        let second = body(e, [2.0; 3], [0.0; 4]);
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 4, 0.5);
        e.mem.set_f32(info + 8, 1.5);
        e.mem.set_f32(info + 0x20, 2.0);
        e.mem.set_f32(info + 0x24, 5.0);
        e.mem.set_u32(info + 0x30, first);
        e.mem.set_u32(info + 0x34, second);
        info
    }

    /// Logs the argument words of every call to `address`.
    fn spy(e: &mut Engine, address: u32, log: &Log) {
        let sink = Rc::clone(log);
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    /// The argument words of the calls to `address`, in order.
    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// `hkMath_atan2fApproximation` returning `angle` and logging its two words.
    fn atan2_double(e: &mut Engine, log: &Log, angle: f64) {
        let sink = Rc::clone(log);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            sink.borrow_mut().push((ATAN2_APPROXIMATION, a.to_vec()));
            Ret {
                st0: angle,
                ..Ret::default()
            }
        });
    }

    /// `hkCalcMotorData` writing `out` and logging the input block's six words.
    fn motor_double(e: &mut Engine, log: &Log, out: [f32; 6]) {
        let sink = Rc::clone(log);
        e.register_double(CALC_MOTOR_DATA, move |e, a| {
            let mut words = vec![a[0]];
            words.extend((0..6).map(|n| e.mem.u32(a[1] + 4 * n)));
            sink.borrow_mut().push((CALC_MOTOR_DATA, words));
            for (index, value) in out.into_iter().enumerate() {
                e.mem.set_f32(a[2] + 4 * index as u32, value);
            }
            Ret::default()
        });
    }

    fn near(actual: f32, expected: f32) -> bool {
        (actual - expected).abs() < 1e-5
    }

    #[test]
    fn the_angular_limit_record_wraps_the_angle_into_the_limit() {
        let two_pi = f64::from_bits(0x4019_21fb_6000_0000);
        for (angle, expected) in [(0.5f64, 0.5f32), (5.0, (5.0 - two_pi) as f32)] {
            let mut e = engine();
            let log = Log::default();
            atan2_double(&mut e, &log, angle);
            let info = e.mem.alloc(0x80);
            e.mem.set_f32(info + 0x40, 2.0);
            let limits = e.mem.alloc(0x20);
            e.mem.set_f32(limits + 4, 1.0);
            e.mem.set_u32(info + 0x64, limits);
            let atom = e.mem.alloc(0x20);
            e.mem.set_u8(atom + 2, 1);
            e.mem.set_f32(atom + 4, 0.25);
            e.mem.set_f32(atom + 8, 0.75);
            e.mem.set_f32(atom + 0xc, 3.0);
            let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
            let record = e.mem.alloc(0x40);
            let result = fn_00d6d2e0(
                &mut e,
                Ptr::new(atom),
                Ptr::new(record),
                Ptr::new(info),
                Ptr::new(rows_a),
                Ptr::new(rows_b),
            );
            assert_eq!(result, 0);
            assert_eq!(
                calls(&log, ATAN2_APPROXIMATION),
                vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
            );
            assert_eq!(load_vec(&e, record)[..3], [1.0, 0.0, 0.0]);
            assert_eq!(e.mem.f32(record + 0x10), 0.25);
            assert_eq!(e.mem.f32(record + 0x14), 0.75);
            assert_eq!(e.mem.f32(record + 0x18), expected);
            assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        }
    }

    #[test]
    fn the_angular_limit_record_is_skipped_without_the_limit_byte() {
        let mut e = engine();
        let atom = e.mem.alloc(0x20);
        let (info, record) = (e.mem.alloc(0x80), e.mem.alloc(0x40));
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let result = fn_00d6d2e0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(record + 0x1c), 0);
    }

    #[test]
    fn the_summed_axis_record_normalises_the_sum_or_keeps_the_second_axis() {
        for (second_axis, normal, half_weight) in [
            ([1.0f32, 0.0, 0.0], [1.0f32, 0.0, 0.0], 8.0f32),
            ([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 0.0),
        ] {
            let mut e = engine();
            let log = Log::default();
            atan2_double(&mut e, &log, 0.5);
            let info = e.mem.alloc(0x80);
            e.mem.set_f32(info + 0x40, 2.0);
            let atom = e.mem.alloc(0x20);
            e.mem.set_u8(atom + 3, 0);
            e.mem.set_u8(atom + 4, 1);
            e.mem.set_f32(atom + 8, 0.25);
            e.mem.set_f32(atom + 0xc, 0.75);
            e.mem.set_f32(atom + 0x10, 4.0);
            let (base_a, base_b) = (identity(&mut e), identity(&mut e));
            store_vec(
                &mut e,
                base_b,
                [second_axis[0], second_axis[1], second_axis[2], 0.0],
            );
            let record = e.mem.alloc(0x40);
            let result = fn_00d6d450(
                &mut e,
                Ptr::new(base_a),
                Ptr::new(atom),
                Ptr::new(record),
                Ptr::new(info),
                Ptr::new(base_b),
            );
            assert_eq!(result, 0);
            assert_eq!(
                calls(&log, ATAN2_APPROXIMATION),
                vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
            );
            assert_eq!(load_vec(&e, record)[..3], normal);
            assert_eq!(e.mem.f32(record + 0x10), 0.25);
            assert_eq!(e.mem.f32(record + 0x14), 0.75);
            assert_eq!(e.mem.f32(record + 0x18), 0.5);
            assert_eq!(e.mem.f32(record + 0x1c), half_weight);
        }
    }

    /// The doubles and data of the two axis-angle records: the CRT square root, the angle
    /// 2.0 and two bases whose axes `atom.byte[3]` / `atom.byte[4]` are x and y (or both x).
    fn angle_record_setup(parallel: bool) -> (Engine, Log, [u32; 5]) {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 2.0);
        e.register_double(X87_SQRT, |_, a| {
            let mut next = 0;
            Ret {
                st0: f64::take(a, &mut next).sqrt(),
                ..Ret::default()
            }
        });
        let info = e.mem.alloc(0x80);
        e.mem.set_f32(info + 0x40, 2.0);
        e.mem.set_f32(info + 0xc, 0.5);
        let limits = e.mem.alloc(0x20);
        e.mem.set_f32(limits + 4, -3.0);
        e.mem.set_f32(limits + 8, 4.0);
        e.mem.set_u32(info + 0x64, limits);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u8(atom + 3, 0);
        e.mem.set_u8(atom + 4, if parallel { 0 } else { 1 });
        e.mem.set_u8(atom + 5, 1);
        e.mem.set_f32(atom + 8, 0.25);
        e.mem.set_f32(atom + 0xc, 0.75);
        e.mem.set_f32(atom + 0x10, 4.0);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x40);
        (e, log, [atom, record, info, rows_a, rows_b])
    }

    #[test]
    fn the_two_axis_record_scales_the_angle_and_signs_the_axis() {
        let (mut e, log, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        let result = fn_00d6d5d0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 0);
        assert_eq!(
            calls(&log, ATAN2_APPROXIMATION),
            vec![vec![1.0f32.to_bits(), 0.0f32.to_bits()]]
        );
        assert_eq!(load_vec(&e, record)[..3], [0.0, 0.0, -1.0]);
        assert_eq!(e.mem.f32(record + 0x10), 0.25);
        assert_eq!(e.mem.f32(record + 0x14), 0.75);
        assert_eq!(e.mem.f32(record + 0x18), 4.5);
        assert_eq!(e.mem.f32(record + 0x1c), 8.0);
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(true);
        let result = fn_00d6d5d0(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 1);
        assert_eq!(e.mem.u32(record + 0x18), 0);
    }

    #[test]
    fn the_limit_variant_lowers_the_bound_by_the_angle_overshoot() {
        // slot 0: nothing more than the record.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        let result = fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(result, 0);
        assert_eq!(e.mem.f32(record + 0x18), 4.5);
        let limits = e.mem.u32(info + 0x64);
        assert_eq!(e.mem.f32(limits + 8), 4.0);
        // slot 8, limit 3 (the record's angle stays below the blend): bound 3 and -1.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        e.mem.set_u8(atom + 6, 8);
        let limits = e.mem.u32(info + 0x64);
        fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        // a = 4, x = 4.5 - 4 = 0.5, c = 3, y = 2.5, M = max(0.25, 1) = 1.
        assert_eq!(e.mem.f32(limits + 8), 3.0);
        assert_eq!(e.mem.f32(record + 0x18), 1.5);
        // limit -3 -> c = -(-3) = 3 above; with limits[1] = 5, c = -5, y = -5.5: the blend
        // 0.5 * (0.25 + 2) = 1.125 is below 5.5, so both bounds move by y.
        let (mut e, _, [atom, record, info, rows_a, rows_b]) = angle_record_setup(false);
        e.mem.set_u8(atom + 6, 8);
        let limits = e.mem.u32(info + 0x64);
        e.mem.set_f32(limits + 4, 5.0);
        fn_00d6d720(
            &mut e,
            Ptr::new(atom),
            Ptr::new(record),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
        );
        assert_eq!(e.mem.f32(limits + 8), 8.5);
        assert_eq!(e.mem.f32(record + 0x18), -4.0);
    }

    /// An angular motor atom: the flag byte at `limits + 0x10`, the value at `limits + 0x14`,
    /// target 3.0, a motor.
    fn motor_atom(e: &mut Engine) -> u32 {
        let atom = e.mem.alloc(0x60);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 0);
        e.mem.set_u16(atom + 4, 0x10);
        e.mem.set_u16(atom + 6, 0x14);
        e.mem.set_f32(atom + 0xc, 3.0);
        e.mem.set_u32(atom + 0x10, 0xabc);
        atom
    }

    #[test]
    fn the_angular_motor_atom_builds_the_motor_record_from_the_motor_data() {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 0.75);
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        let info = info(&mut e);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits, 0x1111);
        e.mem.set_u32(limits + 4, 0x2222);
        e.mem.set_u32(info + 0x64, limits);
        let atom = motor_atom(&mut e);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ang_motor_atom(
            &mut e,
            Ptr::new(info),
            Ptr::new(atom),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(
            calls(&log, ATAN2_APPROXIMATION),
            vec![vec![0.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        let total = (3.0f64 + f64::from(f32::EPSILON)) as f32;
        let inverse = (1.0f64 / f64::from(total)) as f32;
        // remaining = 3.0 - 0.75, passed = 0.75 - 0.75.
        assert_eq!(
            calls(&log, CALC_MOTOR_DATA),
            vec![vec![
                0xabc,
                inverse.to_bits(),
                info,
                0x1111,
                0x2222,
                2.25f32.to_bits(),
                0.0f32.to_bits()
            ]]
        );
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u8(record), 0xf);
        assert_eq!(e.mem.f32(record + 0xc), inverse);
        assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        assert_eq!(e.mem.f32(record + 0x20), 1.0);
        assert_eq!(e.mem.f32(record + 0x24), 2.0);
        assert_eq!(e.mem.f32(record + 0x28), 7.0);
        assert_eq!(e.mem.f32(record + 0x2c), 8.0);
        assert_eq!(e.mem.f32(record + 0x30), 9.0);
        assert_eq!(e.mem.f32(limits + 0x14), 3.0);
        assert_eq!(e.mem.u8(limits + 0x10), 1);
    }

    #[test]
    fn the_angular_motor_atom_without_a_motor_appends_the_placeholder() {
        let mut e = engine();
        let info = info(&mut e);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(info + 0x64, limits);
        let atom = motor_atom(&mut e);
        e.mem.set_u32(atom + 0x10, 0);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ang_motor_atom(
            &mut e,
            Ptr::new(info),
            Ptr::new(atom),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
    }

    #[test]
    fn the_ragdoll_motor_atom_builds_a_record_per_motor() {
        let mut e = engine();
        let log = Log::default();
        atan2_double(&mut e, &log, 0.5);
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        // The product matrix is the identity.
        e.register_double(SET_MUL_MAT3_MAT3, |e, a| {
            for axis in 0..3u32 {
                let mut row = [0.0f32; 4];
                row[axis as usize] = 1.0;
                store_vec(e, a[0] + 0x10 * axis, row);
            }
            Ret::default()
        });
        let info = info(&mut e);
        let limits = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, limits);
        let atom = e.mem.alloc(0x60);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u16(atom + 4, 0x20);
        e.mem.set_u16(atom + 6, 0x30);
        e.mem.set_u32(atom + 0x40, 0xaaa);
        e.mem.set_u32(atom + 0x48, 0xbbb);
        let (rows_a, rows_b) = (identity(&mut e), identity(&mut e));
        let record = e.mem.alloc(0x100);
        let cursor = cell(&mut e, record);
        build_jacobian_from_ragdoll_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x90);
        assert_eq!(e.mem.u8(record), 0xf);
        assert_eq!(e.mem.u8(record + 0x40), 3);
        assert_eq!(e.mem.u8(record + 0x44), 1);
        assert_eq!(e.mem.u8(record + 0x50), 0xf);
        let motors: Vec<u32> = calls(&log, CALC_MOTOR_DATA)
            .iter()
            .map(|words| words[0])
            .collect();
        assert_eq!(motors, vec![0xaaa, 0xbbb]);
        assert_eq!(calls(&log, ATAN2_APPROXIMATION).len(), 4);
        // flag and value of the motors 0 and 2.
        assert_eq!(e.mem.u8(limits + 0x20), 1);
        assert_eq!(e.mem.u8(limits + 0x22), 1);
        assert_eq!(e.mem.f32(limits + 0x30), 0.5);
        assert_eq!(e.mem.f32(limits + 0x38), 0.5);
        assert_eq!(e.mem.f32(record + 0x1c), 6.0);
        // Without the atom's byte `+2` only the placeholder with the marker 3 is made.
        e.mem.set_u8(atom + 2, 0);
        e.mem.set_u32(cursor, record);
        build_jacobian_from_ragdoll_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(rows_a),
            Ptr::new(rows_b),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 3);
    }

    #[test]
    fn the_friction_atom_passes_a_block_or_appends_the_placeholder() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(LINEAR_FRICTION, move |e, a| {
            let mut words = vec![a[1], a[2]];
            words.extend((0..10).map(|n| e.mem.u32(a[0] + 4 * n)));
            sink.borrow_mut().push((LINEAR_FRICTION, words));
            Ret::default()
        });
        let bodies = e.mem.alloc(0x40);
        store_vec(&mut e, bodies + 0x30, [1.0, 2.0, 3.0, 4.0]);
        let rows = e.mem.alloc(0x40);
        store_vec(&mut e, rows + 0x10, [5.0, 6.0, 7.0, 8.0]);
        let info = e.mem.alloc(0x80);
        e.mem.set_u32(info + 0x64, 0x7777);
        let atom = e.mem.alloc(0x10);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_f32(atom + 4, 0.5);
        let record = e.mem.alloc(0x20);
        let cursor = cell(&mut e, record);
        fn_00d6e280(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
            Ptr::new(rows),
            Ptr::new(bodies),
        );
        let expected: Vec<u32> = [info, cursor]
            .into_iter()
            .chain([1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 0.5].map(f32::to_bits))
            .chain([0x7777])
            .collect();
        assert_eq!(calls(&log, LINEAR_FRICTION), vec![expected]);
        assert_eq!(e.mem.u32(cursor), record);
        // A friction of 0 is not above 0: the placeholder.
        e.mem.set_f32(atom + 4, 0.0);
        fn_00d6e280(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
            Ptr::new(rows),
            Ptr::new(bodies),
        );
        assert_eq!(calls(&log, LINEAR_FRICTION).len(), 1);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
    }

    /// The points `p0..p3` of the length records: the arm of the first body (0, 1, 0), the
    /// origin, and the far ends (3, 1, 0) and (0, -4, 0). The bodies sit at the origin with
    /// the inverse masses 0.5, and the offset and the scale are 1 and 2.
    fn spring_points() -> [Vec4; 4] {
        [
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [3.0, 1.0, 0.0, 0.0],
            [0.0, -4.0, 0.0, 0.0],
        ]
    }

    #[test]
    fn the_length_record_holds_the_directions_arms_and_the_stretch() {
        let mut e = engine();
        let points = spring_points();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        for (index, point) in points.into_iter().enumerate() {
            store_vec(&mut e, block + 0x10 * index as u32, point);
        }
        e.mem.set_f32(block + 0x40, 1.0);
        e.mem.set_f32(block + 0x44, 2.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(e.mem.u32(cursor), record + 0x40);
        assert_eq!(e.mem.u8(record), 0xb);
        let first = load_vec(&e, record);
        assert!(near(first[1], 0.0) && near(first[2], 0.0));
        let second = load_vec(&e, record + 0x10);
        assert!(near(second[0], 0.0) && near(second[1], 2.0) && near(second[2], 0.0));
        let arm = load_vec(&e, record + 0x20);
        assert!(near(arm[0], 0.0) && near(arm[1], 0.0) && near(arm[2], -1.0));
        assert_eq!(load_vec(&e, record + 0x30)[..3], [0.0, 0.0, 0.0]);
        // mass terms 2 * 2 * 0.5 + 0.5 + FLT_EPSILON, inertia 1: 5 / 3.5.
        assert!(near(e.mem.f32(record + 0x2c), 5.0 / 3.5));
        // ((2 * 4 + 3) - 1) * 1.5.
        assert_eq!(e.mem.f32(record + 0xc), 15.0);
    }

    #[test]
    fn the_length_record_of_three_blocks_matches_the_one_block_form() {
        let mut e = engine();
        let points = spring_points();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        for (index, point) in points.into_iter().enumerate() {
            store_vec(&mut e, block + 0x10 * index as u32, point);
        }
        e.mem.set_f32(block + 0x40, 1.0);
        e.mem.set_f32(block + 0x44, 2.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        let expected: Vec<u32> = (0..0x10).map(|n| e.mem.u32(record + 4 * n)).collect();
        let first = e.mem.alloc(0x40);
        store_vec(&mut e, first + 0x10, points[2]);
        store_vec(&mut e, first + 0x20, points[3]);
        e.mem.set_f32(first + 0x30, 1.0);
        e.mem.set_f32(first + 0x34, 2.0);
        let second = e.mem.alloc(0x40);
        store_vec(&mut e, second + 0x30, points[0]);
        let third = e.mem.alloc(0x40);
        store_vec(&mut e, third + 0x30, points[1]);
        let other = e.mem.alloc(0x80);
        let other_cursor = cell(&mut e, other);
        fn_00d6e6b0(
            &mut e,
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(info),
            Ptr::new(third),
            Ptr::new(other_cursor),
        );
        let actual: Vec<u32> = (0..0x10).map(|n| e.mem.u32(other + 4 * n)).collect();
        assert_eq!(actual, expected);
        assert_eq!(e.mem.u32(other_cursor), other + 0x40);
    }

    #[test]
    fn the_length_record_falls_back_to_the_x_axis_for_coincident_points() {
        let mut e = engine();
        let info = info(&mut e);
        let block = e.mem.alloc(0x50);
        e.mem.set_f32(block + 0x44, 3.0);
        let record = e.mem.alloc(0x80);
        let cursor = cell(&mut e, record);
        fn_00d6e300(&mut e, Ptr::new(block), Ptr::new(info), Ptr::new(cursor));
        assert_eq!(load_vec(&e, record + 0x10)[..3], [-3.0, 0.0, 0.0]);
        assert_eq!(load_vec(&e, record)[1..3], [0.0, 0.0]);
        assert_eq!(e.mem.f32(record + 0xc), 0.0);
    }

    /// An atom chain for the modifiers: the atom, with the next node of type `kind`.
    fn modifier_atom(e: &mut Engine, kind: u16) -> (u32, u32) {
        let node = e.mem.alloc(0x100);
        e.mem.set_u16(node, kind);
        let atom = e.mem.alloc(0x80);
        e.mem.set_u16(atom + 0x12, 0x1234);
        e.mem.set_u32(atom + 0x14, node);
        (atom, node)
    }

    #[test]
    fn the_lowering_modifier_scales_the_info_copy_and_caps_the_node_entries() {
        for (kind, contact) in [(0x16u16, true), (0x10, false)] {
            let mut e = engine();
            let log = Log::default();
            for callee in [BUILD_CONTACT_JACOBIAN, BUILD_NOT_CONTACT_JACOBIAN] {
                let sink = Rc::clone(&log);
                e.register_double(callee, move |e, a| {
                    let copy = if callee == BUILD_CONTACT_JACOBIAN {
                        a[1]
                    } else {
                        a[2]
                    };
                    let mut words = a.to_vec();
                    words.extend([0x24, 0x40, 0x44].map(|offset| e.mem.u32(copy + offset)));
                    sink.borrow_mut().push((callee, words));
                    Ret::default()
                });
            }
            let info = e.mem.alloc(0x80);
            for (offset, value) in [
                (0, 2.0f32),
                (4, 3.0),
                (0x24, 5.0),
                (0x40, 7.0),
                (0x44, 11.0),
            ] {
                e.mem.set_f32(info + offset, value);
            }
            let (atom, node) = modifier_atom(&mut e, kind);
            e.mem.set_f32(atom + 0x20, 0.5);
            e.mem.set_f32(atom + 0x24, 0.25);
            e.mem.set_u16(node + 4, 2);
            e.mem.set_u16(node + 6, 1);
            e.mem.set_u8(node + 0xa, 0x20);
            e.mem.set_f32(node + 0x4c, 1.0);
            e.mem.set_f32(node + 0x6c, 10.0);
            e.mem.set_f32(node + 0x60, 100.0);
            e.mem.set_f32(node + 0x80, 5.0);
            let record = e.mem.alloc(0x40);
            let cursor = cell(&mut e, record + 0x10);
            fn_00d6ea70(&mut e, Ptr::new(info), Ptr::new(atom), Ptr::new(cursor));
            assert_eq!(e.mem.u32(cursor), record);
            // 0.25 * 3 * 2 / 0.5 = 3: the entries 3 + 1 and 3 + 10.
            assert_eq!(e.mem.f32(node + 0x60), 4.0);
            assert_eq!(e.mem.f32(node + 0x80), 5.0);
            let scaled = [2.5f32, 3.5, 5.5].map(f32::to_bits);
            if contact {
                let [call] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
                assert_eq!([call[0], call[2], call[3]], [node, 1, cursor]);
                assert_eq!(call[4..], scaled);
                assert!(calls(&log, BUILD_NOT_CONTACT_JACOBIAN).is_empty());
            } else {
                let [call] = calls(&log, BUILD_NOT_CONTACT_JACOBIAN).try_into().unwrap();
                assert_eq!([call[0], call[1], call[3]], [node, 0x1234, cursor]);
                assert_eq!(call[4..], scaled);
            }
        }
    }

    #[test]
    fn the_scaling_modifier_wraps_the_jacobian_between_two_vector_records() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(PUSH_VECTOR_PAIR, move |e, a| {
            let mut words = a.to_vec();
            for pointer in &a[..2] {
                words.extend((0..4).map(|n| e.mem.u32(pointer + 4 * n)));
            }
            sink.borrow_mut().push((PUSH_VECTOR_PAIR, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BUILD_CONTACT_JACOBIAN, move |e, a| {
            let mut words = a.to_vec();
            words.extend([e.mem.u32(a[1] + 0x30), e.mem.u32(a[1] + 0x34)]);
            sink.borrow_mut().push((BUILD_CONTACT_JACOBIAN, words));
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        let body_a = e.mem.alloc(0x80);
        let body_b = e.mem.alloc(0x80);
        store_vec(&mut e, body_a + 0x30, [1.0, 2.0, 3.0, 4.0]);
        store_vec(&mut e, body_b + 0x30, [5.0, 6.0, 7.0, 8.0]);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        let (atom, node) = modifier_atom(&mut e, 0x16);
        e.mem.set_u16(atom + 0x12, 0x0207);
        store_vec(&mut e, atom + 0x20, [2.0; 4]);
        store_vec(&mut e, atom + 0x30, [3.0; 4]);
        let cursor = cell(&mut e, 0x5000);
        fn_00d6ecb0(&mut e, Ptr::new(atom), Ptr::new(info), Ptr::new(cursor));
        let pairs = calls(&log, PUSH_VECTOR_PAIR);
        assert_eq!(pairs.len(), 2);
        let words = |values: [f32; 8]| values.map(f32::to_bits).to_vec();
        assert_eq!(pairs[0][2], cursor);
        assert_eq!(
            pairs[0][3..],
            words([2.0, 4.0, 6.0, 8.0, 15.0, 18.0, 21.0, 24.0])[..]
        );
        assert_eq!(pairs[1][..3], [body_a + 0x30, body_b + 0x30, cursor]);
        assert_eq!(
            pairs[1][3..],
            words([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])[..]
        );
        let [contact] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
        assert_eq!([contact[0], contact[2], contact[3]], [node, 0x0201, cursor]);
        assert_eq!(contact[4], pairs[0][0] - 0x30);
        assert_eq!(contact[5], pairs[0][1] - 0x30);
        assert_eq!(load_vec(&e, body_a + 0x30), [1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn the_body_copy_leaves_the_flag_bytes_and_quiets_the_float() {
        let mut e = engine();
        let source = e.mem.alloc(0x80);
        let target = e.mem.alloc(0x80);
        for word in 0..0x20 {
            e.mem.set_u32(source + 4 * word, 0x1000_0000 + word);
            e.mem.set_u32(target + 4 * word, 0xaaaa_aaaa);
        }
        e.mem.set_u32(source + 8, 0x7f80_0001);
        let result = fn_00d71f90(&mut e, Ptr::new(target), Ptr::new(source));
        assert_eq!(result.addr(), target);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(e.mem.u32(target + 4), 0x1000_0001);
        assert_eq!(e.mem.u32(target + 8), 0x7fc0_0001);
        assert_eq!(e.mem.u32(target) >> 8, 0xaa_aaaa);
        assert_eq!(e.mem.u32(target + 0xc), 0xaaaa_aaaa);
        for word in 4..0x20 {
            assert_eq!(e.mem.u32(target + 4 * word), 0x1000_0000 + word, "{word}");
        }
    }

    #[test]
    fn the_moving_surface_modifier_replaces_one_body_and_brackets_the_jacobian() {
        for second_body in [true, false] {
            let mut e = engine();
            let log = Log::default();
            e.register_double(COPY_BODY_DATA, |e, a| {
                for word in 0..0x20 {
                    let value = e.mem.u32(a[1] + 4 * word);
                    e.mem.set_u32(a[0] + 4 * word, value);
                }
                Ret::default()
            });
            let sink = Rc::clone(&log);
            e.register_double(ADD_VELOCITY, move |e, a| {
                let mut words = a.to_vec();
                words.extend((0..4).map(|n| e.mem.u32(a[0] + 4 * n)));
                sink.borrow_mut().push((ADD_VELOCITY, words));
                Ret::default()
            });
            let sink = Rc::clone(&log);
            e.register_double(BUILD_CONTACT_JACOBIAN, move |e, a| {
                let mut words = a.to_vec();
                words.extend([e.mem.u32(a[1] + 0x30), e.mem.u32(a[1] + 0x34)]);
                sink.borrow_mut().push((BUILD_CONTACT_JACOBIAN, words));
                Ret::default()
            });
            let info = e.mem.alloc(0x80);
            let body_a = e.mem.alloc(0x80);
            let body_b = e.mem.alloc(0x80);
            store_vec(&mut e, body_a + 0x10, [10.0; 4]);
            store_vec(&mut e, body_b + 0x10, [20.0; 4]);
            e.mem.set_u32(info + 0x30, body_a);
            e.mem.set_u32(info + 0x34, body_b);
            e.mem.set_u32(info + 0x60, 0x1234_5678);
            e.mem.set_u32(info + 0x6c, u32::from(!second_body));
            let (atom, node) = modifier_atom(&mut e, 0x16);
            store_vec(&mut e, atom + 0x20, [1.0, 2.0, 3.0, 4.0]);
            e.mem.set_u16(node + 6, 1);
            e.mem.set_u8(node + 0xa, 3);
            store_vec(&mut e, node + 0x40, [0.0, 1.0, 0.0, 0.0]);
            let record = e.mem.alloc(0x40);
            let cursor = cell(&mut e, record + 0x10);
            build_jacobian_from_moving_surface_modifier(
                &mut e,
                Ptr::new(atom),
                Ptr::new(info),
                Ptr::new(cursor),
            );
            // The header record replaces the last record.
            assert_eq!(e.mem.u32(cursor), record + 0x10);
            assert_eq!(e.mem.u8(record), 1);
            assert_eq!(e.mem.u8(record + 1), 3);
            assert_eq!(e.mem.u32(record + 8), node + 0x50);
            assert_eq!(e.mem.u32(record + 0xc), 0x1234_5678);
            let (kept, replaced) = if second_body {
                (info + 0x30, info + 0x34)
            } else {
                (info + 0x34, info + 0x30)
            };
            assert_eq!(e.mem.u32(kept), if second_body { body_a } else { body_b });
            let buffer = e.mem.u32(replaced);
            assert_ne!(buffer, body_a);
            assert_ne!(buffer, body_b);
            // Direction: x, y, z negated for the second body; its part along y is removed.
            let sign = if second_body { -1.0 } else { 1.0 };
            let rest = [sign * 1.0, 0.0, sign * 3.0, 4.0];
            let negated = rest.map(|lane| -lane);
            let velocities = calls(&log, ADD_VELOCITY);
            assert_eq!(velocities.len(), 2);
            let flag = u32::from(second_body);
            assert_eq!(velocities[0][1], flag);
            assert_eq!(velocities[0][3..], negated.map(f32::to_bits));
            assert_eq!(velocities[1][1], flag);
            assert_eq!(velocities[1][3..], rest.map(f32::to_bits));
            let original = if second_body { 20.0 } else { 10.0 };
            let moved = load_vec(&e, buffer + 0x10);
            assert_eq!(moved, negated.map(|lane| lane + original));
            let [contact] = calls(&log, BUILD_CONTACT_JACOBIAN).try_into().unwrap();
            assert_eq!(
                [contact[0], contact[2], contact[3]],
                [node, 0x1234_5600, cursor]
            );
            // The copy handed on still has both original bodies.
            assert_eq!(contact[4..], [body_a, body_b]);
        }
    }

    #[test]
    fn the_linear_motor_atom_builds_the_begin_block_and_commits() {
        let mut e = engine();
        let log = Log::default();
        let sink = Rc::clone(&log);
        e.register_double(LINEAR_VELOCITY_BEGIN, move |e, a| {
            sink.borrow_mut().push((LINEAR_VELOCITY_BEGIN, a.to_vec()));
            let seen = [e.mem.u32(a[4] + 8), e.mem.u32(a[4] + 0xc)];
            sink.borrow_mut().push((0, seen.to_vec()));
            e.mem.set_f32(a[4], 0.125);
            Ret::default()
        });
        motor_double(&mut e, &log, [0.0, 7.0, 2.0, 4.0, 8.0, 9.0]);
        spy(&mut e, LINEAR_VELOCITY_COMMIT, &log);
        let info = e.mem.alloc(0x80);
        let limits = e.mem.alloc(0x40);
        e.mem.set_u32(limits, 0x1111);
        e.mem.set_u32(limits + 4, 0x2222);
        e.mem.set_u32(info + 0x64, limits);
        let first = e.mem.alloc(0x40);
        store_vec(&mut e, first + 0x30, [4.0, 0.0, 0.0, 0.0]);
        let second = e.mem.alloc(0x40);
        store_vec(&mut e, second + 0x30, [1.0, 0.0, 0.0, 0.0]);
        store_vec(&mut e, second + 0x10, [1.0, 0.0, 0.0, 0.0]);
        let atom = e.mem.alloc(0x20);
        e.mem.set_u8(atom + 2, 1);
        e.mem.set_u8(atom + 3, 1);
        e.mem.set_u16(atom + 4, 0x10);
        e.mem.set_u16(atom + 6, 0x14);
        e.mem.set_f32(atom + 8, 2.5);
        e.mem.set_u32(atom + 0xc, 0xbeef);
        let record = e.mem.alloc(0x40);
        let cursor = cell(&mut e, record);
        build_jacobian_from_lin_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(cursor),
        );
        let [begin] = calls(&log, LINEAR_VELOCITY_BEGIN).try_into().unwrap();
        assert_eq!(begin[..4], [second + 0x10, first + 0x30, info, record]);
        // The begin function sees zeros in the block's words 2 and 3.
        assert_eq!(calls(&log, 0), vec![vec![0, 0]]);
        // remaining = 2.5 - 2.5, passed = 2.5 - (4 - 1) . x.
        let [motor] = calls(&log, CALC_MOTOR_DATA).try_into().unwrap();
        assert_eq!(motor[0], 0xbeef);
        assert_eq!(motor[1], 0.125f32.to_bits());
        assert_eq!(
            motor[2..],
            [info, 0x1111, 0x2222, 0.0f32.to_bits(), (-0.5f32).to_bits()]
        );
        let [commit] = calls(&log, LINEAR_VELOCITY_COMMIT).try_into().unwrap();
        assert_eq!(commit[1..], [info, record, cursor]);
        assert_eq!(e.mem.f32(limits + 0x14), 2.5);
        assert_eq!(e.mem.u8(limits + 0x10), 1);
        // No motor: the placeholder.
        e.mem.set_u32(atom + 0xc, 0);
        build_jacobian_from_lin_motor_atom(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(first),
            Ptr::new(second),
            Ptr::new(cursor),
        );
        assert_eq!(e.mem.u8(record), 3);
        assert_eq!(e.mem.u8(record + 4), 1);
        assert_eq!(e.mem.u32(cursor), record + 0x10);
    }

    #[test]
    fn the_center_of_mass_changer_moves_the_body_copies_and_brackets_the_jacobian() {
        let mut e = engine();
        let log = Log::default();
        e.register_double(SET_ROTATED_DIR, |e, a| {
            copy_vec(e, a[2], a[0]);
            Ret::default()
        });
        e.register_double(SET_CROSS_SKEW_SYMMETRIC, |e, a| {
            let source = load_vec(e, a[1]);
            for row in 0..3 {
                store_vec(e, a[0] + 0x10 * row, [source[row as usize]; 4]);
            }
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_MUL_INVERSE, move |e, a| {
            sink.borrow_mut().push((SET_MUL_INVERSE, a.to_vec()));
            for row in 0..3 {
                copy_vec(e, a[1] + 0x10 * row, a[0] + 0x10 * row);
            }
            Ret::default()
        });
        e.register_double(SCALE_ROWS, |e, a| {
            let factor = e.mem.f32(a[1]);
            for row in 0..3 {
                let scaled = load_vec(e, a[0] + 0x10 * row).map(|lane| lane * factor);
                store_vec(e, a[0] + 0x10 * row, scaled);
            }
            Ret::default()
        });
        e.register_double(SET_MUL3, |e, a| {
            copy_vec(e, a[1], a[0]);
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(SET_CENTER_OF_MASS, move |e, a| {
            let mut words = a.to_vec();
            words.extend(a[..2].iter().map(|matrix| e.mem.u32(*matrix)));
            sink.borrow_mut().push((SET_CENTER_OF_MASS, words));
            Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(BUILD_NOT_CONTACT_JACOBIAN, move |e, a| {
            let mut words = a.to_vec();
            for buffer in [e.mem.u32(a[2] + 0x30), e.mem.u32(a[2] + 0x34)] {
                words.push(buffer);
                for offset in [0x10, 0x40] {
                    words.extend((0..4).map(|n| e.mem.u32(buffer + offset + 4 * n)));
                }
            }
            sink.borrow_mut().push((BUILD_NOT_CONTACT_JACOBIAN, words));
            Ret::default()
        });
        let info = e.mem.alloc(0x80);
        let body_a = e.mem.alloc(0x80);
        let body_b = e.mem.alloc(0x80);
        store_vec(&mut e, body_a + 0x10, [10.0; 4]);
        store_vec(&mut e, body_b + 0x10, [20.0; 4]);
        e.mem.set_u32(info + 0x30, body_a);
        e.mem.set_u32(info + 0x34, body_b);
        e.mem.set_u32(info + 0x38, 0x1111);
        e.mem.set_u32(info + 0x3c, 0x2222);
        let (atom, node) = modifier_atom(&mut e, 0x10);
        store_vec(&mut e, atom + 0x20, [1.0, 2.0, 3.0, 0.0]);
        store_vec(&mut e, atom + 0x30, [4.0, 5.0, 6.0, 0.0]);
        let cursor = cell(&mut e, 0x6000);
        build_jacobian_from_center_of_mass_changer_modifier(
            &mut e,
            Ptr::new(atom),
            Ptr::new(info),
            Ptr::new(cursor),
        );
        let order: Vec<u32> = log.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(
            order,
            vec![
                SET_MUL_INVERSE,
                SET_MUL_INVERSE,
                SET_CENTER_OF_MASS,
                BUILD_NOT_CONTACT_JACOBIAN,
                SET_CENTER_OF_MASS
            ]
        );
        // setMulInverse: the skew matrices and the body copies' rotation rows.
        let inverses = calls(&log, SET_MUL_INVERSE);
        let [not_contact] = calls(&log, BUILD_NOT_CONTACT_JACOBIAN).try_into().unwrap();
        assert_eq!(
            [not_contact[0], not_contact[1], not_contact[3]],
            [node, 0x1234, cursor]
        );
        // The words after the four arguments: the copy's body, its vectors at +0x10 and +0x40.
        let (buffer_a, buffer_b) = (not_contact[4], not_contact[13]);
        assert_eq!(inverses[0][2], buffer_a + 0x50);
        assert_eq!(inverses[1][2], buffer_b + 0x50);
        // Linear velocities: 10 and 20 plus the negated first rows; positions: 0 plus the
        // turned vectors.
        let words = |values: [f32; 4]| values.map(f32::to_bits).to_vec();
        assert_eq!(not_contact[5..9], words([9.0; 4])[..]);
        assert_eq!(not_contact[9..13], words([1.0, 2.0, 3.0, 0.0])[..]);
        assert_eq!(not_contact[14..18], words([16.0; 4])[..]);
        assert_eq!(not_contact[18..], words([4.0, 5.0, 6.0, 0.0])[..]);
        // The record of the negated matrices (first floats -1 and -4), then the record of the
        // products with the inverse rotations (1 and 4).
        let records = calls(&log, SET_CENTER_OF_MASS);
        assert_eq!(records[0][2], cursor);
        assert_eq!(records[0][3..], [(-1.0f32).to_bits(), (-4.0f32).to_bits()]);
        assert_eq!(records[1][..3], [inverses[0][0], inverses[1][0], cursor]);
        assert_eq!(records[1][3..], [1.0f32.to_bits(), 4.0f32.to_bits()]);
    }
}
