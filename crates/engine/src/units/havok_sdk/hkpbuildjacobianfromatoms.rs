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
    load_bits, load_vec, shufps, store_vec, sum3, unpckhps, Vec4, SIGN_MASK,
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
